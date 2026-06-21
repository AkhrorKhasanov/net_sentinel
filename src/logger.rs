use std::fs::OpenOptions;
use std::io::Write;
use crate::models::AuditEntry;

pub async fn start_logger(mut rx: tokio::sync::mpsc::Receiver<AuditEntry>) {
    let mut file = OpenOptions::new()
    .create(true)
    .append(true)
    .open("logs/audit.jsonl")
    .expect("Could not open log file");

    while let Some(entry) = rx.recv().await {
        if let Ok(json) = serde_json::to_string(&entry) {
            let _ = writeln!(file, "{}", json);
        }
    }
}