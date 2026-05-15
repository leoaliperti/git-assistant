use std::env;
use std::process::Command;

fn main() {
    // --- 0. LEGGIAMO GLI ARGOMENTI EXTRA ---
    // Raccogliamo tutto quello che l'utente scrive dopo "git-assistant"
    // .skip(1) serve a ignorare il nome del programma stesso
    let extra_args: Vec<String> = env::args().skip(1).collect();

    // --- 1. PRENDIAMO IL DIFF ---
    let git_diff = Command::new("git")
        .args(["diff", "--cached"])
        .output()
        .expect("Errore: Impossibile eseguire git diff");

    let diff_text = String::from_utf8_lossy(&git_diff.stdout);
    if diff_text.trim().is_empty() {
        println!("⚠️ Nulla in stage. Usa 'git add' prima.");
        return;
    }

    // --- 2. CHIEDIAMO AD APFEL (PROMPT BLINDATO) ---
    let prompt = format!(
        "You are a strict code analyzer. Read the git diff and output a single-line git commit message using the Conventional Commits format.

CRITICAL RULES:
1. DO NOT copy the examples provided below.
2. If the diff only removes comments or changes whitespace, use 'style:' or 'docs:'.
3. Output ONLY the raw commit message, nothing else.

EXAMPLES OF FORMAT (DO NOT COPY THESE):
style: remove obsolete code
fix: resolve crash on startup
docs: update readme instructions

DIFF TO ANALYZE:
{}",
        diff_text
    );

    println!("🤖 Generazione messaggio in corso...");
    let apfel_output = Command::new("apfel")
        .arg(&prompt)
        .output()
        .expect("Errore: apfel non trovato");

    let commit_message = String::from_utf8_lossy(&apfel_output.stdout).trim().to_string();
    
    if commit_message.is_empty() || commit_message.contains("apple.com") {
        println!("❌ L'AI ha dato una risposta non valida. Riprova.");
        return;
    }

    println!("✅ Messaggio creato: {}", commit_message);

    // --- 3. ESEGUIAMO IL COMMIT ---
    println!("📦 Eseguo il commit...");
    let commit_status = Command::new("git")
        .args(["commit", "-m", &commit_message])
        .status()
        .expect("Errore durante il commit");

    if commit_status.success() {
        // --- 4. ESEGUIAMO IL PUSH CON I PARAMETRI EXTRA ---
        println!("🚀 Invio al server (push)...");
        
        let mut push_cmd = Command::new("git");
        push_cmd.arg("push");

        // Se hai scritto parametri come --force, li aggiungiamo al comando push
        if !extra_args.is_empty() {
            println!("   🔧 Parametri aggiuntivi rilevati: {}", extra_args.join(" "));
            push_cmd.args(&extra_args);
        }

        let push_status = push_cmd.status()
            .expect("Errore durante l'esecuzione del push");

        if push_status.success() {
            println!("✨ Tutto fatto! Codice online.");
        } else {
            println!("❌ Il push ha restituito un errore (forse devi fare un git pull prima?).");
        }
    }
}