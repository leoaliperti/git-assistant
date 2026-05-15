use std::process::Command;

fn main() {
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


    // --- 2. CHIEDIAMO AD APFEL (PROMPT POTENZIATO) ---
    let prompt = format!(
        "You are a strict code analyzer. Your ONLY job is to read a git diff and output a single-line git commit message using the Conventional Commits format.

CRITICAL RULES:
1. NEVER write 'Here is your commit', 'Commit:', or 'I generated'.
2. DO NOT explain what you did.
3. OUTPUT ONLY THE RAW COMMIT MESSAGE.

EXAMPLES OF CORRECT OUTPUT:
feat: add automatic push logic to main.rs
fix: correct error handling for git diff command
chore: update dependencies

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
        // --- 4. ESEGUIAMO IL PUSH ---
        println!("🚀 Invio al server (push)...");
        let push_status = Command::new("git")
            .arg("push")
            .status()
            .expect("Errore durante il push");

        if push_status.success() {
            println!("✨ Tutto fatto! Codice online.");
        }
    }
}