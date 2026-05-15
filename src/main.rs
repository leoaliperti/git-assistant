use std::env;
use std::process::Command;

fn main() {
    let extra_args: Vec<String> = env::args().skip(1).collect();

    // 1. PRENDIAMO IL DIFF
    let git_diff = Command::new("git")
        .args(["diff", "--cached"])
        .output()
        .expect("Errore: Impossibile eseguire git diff");

    let diff_text = String::from_utf8_lossy(&git_diff.stdout);
    if diff_text.trim().is_empty() {
        println!("⚠️ Nulla in stage. Usa 'git add' prima.");
        return;
    }

    println!("🤖 Consultazione di Apple Intelligence in corso...");

    // 2. IL SYSTEM PROMPT (Passato con il flag -s)
    let system_prompt = "You are a Git Commit generator. Your ONLY purpose is to read code diffs and output a single, raw commit message using Conventional Commits (feat:, fix:, docs:, style:, chore:, refactor:). Do not wrap the message in quotes. Do not add explanations.";

    // 3. CHIAMATA OTTIMIZZATA AD APFEL
    // Usiamo --permissive per abbassare i guardrail di Apple
    // Usiamo -q per la modalità silenziosa
    // Usiamo -s per impostare il ruolo del modello
    let apfel_output = Command::new("apfel")
        .arg("--permissive")
        .arg("-q")
        .arg("-s")
        .arg(system_prompt)
        .arg(&diff_text.to_string()) // Passiamo il diff come input principale
        .output()
        .expect("Errore: apfel non trovato");

    let commit_message = String::from_utf8_lossy(&apfel_output.stdout).trim().to_string();
    
    if commit_message.is_empty() || commit_message.contains("apple.com") {
        println!("❌ L'AI ha attivato i filtri di sicurezza. Riprova o fai un commit manuale.");
        return;
    }

    println!("✅ Messaggio generato:\n> {}", commit_message);

    // 4. ESECUZIONE DEL COMMIT
    let commit_status = Command::new("git")
        .args(["commit", "-m", &commit_message])
        .status()
        .expect("Errore durante il commit");

    if commit_status.success() {
        // 5. PUSH CON I PARAMETRI
        println!("🚀 Invio al server...");
        
        let mut push_cmd = Command::new("git");
        push_cmd.arg("push");

        if !extra_args.is_empty() {
            push_cmd.args(&extra_args);
        }

        let push_status = push_cmd.status().expect("Errore nel push");

        if push_status.success() {
            println!("✨ Push completato con successo!");
        } else {
            println!("❌ Fallito (forse devi fare 'git pull' o impostare l'upstream).");
        }
    }
}