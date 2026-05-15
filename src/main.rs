use std::process::Command;

fn main() {
    // 1. Eseguiamo il comando per vedere quali file sono pronti per il commit
    let git_diff = Command::new("git")
        .args(["diff", "--cached"])
        .output()
        .expect("Errore critico: Impossibile eseguire il comando git.");

    let diff_text = String::from_utf8_lossy(&git_diff.stdout);

    // Se non c'è nulla in stage (nessun file aggiunto con git add), ci fermiamo
    if diff_text.trim().is_empty() {
        println!("⚠️ Nessun file in stage. Usa 'git add <file>' prima di lanciare l'assistente.");
        return;
    }

    // 2. Prepariamo le istruzioni per Apfel (Il "Prompt")
    let prompt = format!(
        "Sei uno sviluppatore esperto. Scrivi un messaggio di commit breve e descrittivo per questo diff di git. Usa la convenzione 'Conventional Commits' (es. feat:, fix:, docs:). Rispondi SOLO con il messaggio, senza intro o spiegazioni aggiuntive.\n\nEcco il diff:\n{}",
        diff_text
    );

    println!("🤖 Lettura del codice e consultazione di Apple Intelligence in corso...");

    // 3. Chiamiamo la CLI di apfel dal nostro programma Rust
    let apfel_output = Command::new("apfel")
        .arg(&prompt)
        .output()
        .expect("Errore: Impossibile trovare 'apfel'. Sicuro di averlo installato con brew?");

    // 4. Estraiamo la risposta
    let commit_message = String::from_utf8_lossy(&apfel_output.stdout);
    
    println!("\n✅ Messaggio suggerito:\n");
    println!("{}", commit_message.trim());
}