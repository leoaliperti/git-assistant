use std::env;
use std::process::{self, Command};

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

/// A lightweight error wrapper that carries a user-facing message.
/// Using a newtype keeps the dependency footprint at zero (no `anyhow`/`thiserror`
/// needed), while still giving us `?`-based propagation throughout the codebase.
#[derive(Debug)]
struct AppError(String);

impl AppError {
    fn new(msg: impl Into<String>) -> Self {
        Self(msg.into())
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Allow any type that implements std::error::Error to be wrapped automatically,
// so we can use `?` on things like io::Error without manual conversion.
impl<E: std::error::Error> From<E> for AppError {
    fn from(e: E) -> Self {
        Self(e.to_string())
    }
}

type Result<T> = std::result::Result<T, AppError>;

// ---------------------------------------------------------------------------
// CLI argument handling
// ---------------------------------------------------------------------------

/// Everything the program needs from the command line.
struct Args {
    /// Flags forwarded verbatim to `git push` (e.g. `--force`, `origin main`).
    push_args: Vec<String>,
}

impl Args {
    fn parse() -> Self {
        // Skip argv[0] (the binary name). Everything else is a push argument.
        let push_args: Vec<String> = env::args().skip(1).collect();
        Self { push_args }
    }
}

// ---------------------------------------------------------------------------
// Git helpers
// ---------------------------------------------------------------------------

/// Returns the staged diff, or `None` when the staging area is empty.
fn get_staged_diff() -> Result<Option<String>> {
    let output = Command::new("git")
        .args(["diff", "--cached"])
        .output()
        .map_err(|e| AppError::new(format!("Could not run `git diff`: {e}")))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::new(format!("`git diff` failed: {stderr}")));
    }

    let diff = String::from_utf8_lossy(&output.stdout).into_owned();
    if diff.trim().is_empty() {
        Ok(None)
    } else {
        Ok(Some(diff))
    }
}

/// Runs `git commit -m <message>` and returns whether it succeeded.
fn git_commit(message: &str) -> Result<bool> {
    let status = Command::new("git")
        .args(["commit", "-m", message])
        .status()
        .map_err(|e| AppError::new(format!("Could not run `git commit`: {e}")))?;

    Ok(status.success())
}

/// Runs `git push [extra_args…]` and returns whether it succeeded.
fn git_push(extra_args: &[String]) -> Result<bool> {
    let status = Command::new("git")
        .arg("push")
        .args(extra_args)
        .status()
        .map_err(|e| AppError::new(format!("Could not run `git push`: {e}")))?;

    Ok(status.success())
}

// ---------------------------------------------------------------------------
// Apple Intelligence (apfel) integration
// ---------------------------------------------------------------------------

const SYSTEM_PROMPT: &str =
    "You are a strict Git Commit tool. Output EXACTLY ONE single line. \
     Format: <type>: <description>. NO LISTS. NO EXPLANATIONS. NO MULTIPLE OPTIONS.";

/// Calls `apfel` with the staged diff and returns the first non-empty line of
/// its response, which is the generated commit message.
///
/// The flags `--permissive -q -s` are **required** to bypass Apple's on-device
/// content guardrails and must not be changed.
fn generate_commit_message(diff: &str) -> Result<Option<String>> {
    let user_prompt = format!(
        "Write exactly ONE single commit message for the following diff:\n\n{diff}"
    );

    let output = Command::new("apfel")
        .arg("--permissive")
        .arg("-q")
        .arg("-s")
        .arg(SYSTEM_PROMPT)
        .arg(&user_prompt)
        .output()
        .map_err(|e| {
            AppError::new(format!(
                "Could not run `apfel` — is it installed and on your PATH? ({e})"
            ))
        })?;

    // apfel writes its response to stdout; stderr may carry debug noise.
    let raw = String::from_utf8_lossy(&output.stdout).into_owned();

    // Take only the first non-empty line to strip any preamble/trailing text.
    let message = raw
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or("")
        .trim()
        .to_string();

    // An empty message or a safety-filter redirect URL means the model bailed.
    if message.is_empty() || message.contains("apple.com") {
        return Ok(None);
    }

    Ok(Some(message))
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn run(args: Args) -> Result<()> {
    // 1. Check for staged changes.
    let diff = match get_staged_diff()? {
        Some(d) => d,
        None => {
            println!("⚠️  Nothing staged. Run `git add` first.");
            return Ok(());
        }
    };

    // 2. Generate commit message via Apple Intelligence.
    println!("🤖 Consulting Apple Intelligence…");
    let commit_message = match generate_commit_message(&diff)? {
        Some(msg) => msg,
        None => {
            return Err(AppError::new(
                "❌ Apple Intelligence triggered a safety filter or returned an empty response. \
                 Please try again.",
            ));
        }
    };

    println!("✅ Generated commit message:\n   > {commit_message}");

    // 3. Commit with the generated message.
    println!("📝 Committing…");
    if !git_commit(&commit_message)? {
        return Err(AppError::new(
            "❌ `git commit` failed. Check the output above for details.",
        ));
    }

    // 4. Push to remote.
    println!("🚀 Pushing to remote…");
    if git_push(&args.push_args)? {
        println!("✨ Push successful!");
    } else {
        return Err(AppError::new(
            "❌ `git push` failed. \
             You may need to run `git pull --rebase` first, or set an upstream branch.",
        ));
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    if let Err(e) = run(args) {
        eprintln!("{e}");
        process::exit(1);
    }
}