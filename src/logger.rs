use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use crate::models::AuditEntry;

pub async fn start_logger(mut rx: tokio::sync::mpsc::Receiver<AuditEntry>) {
    let log_path = "logs/audit.jsonl";
    
    // Ensure the log directory exists
    if let Some(parent) = std::path::Path::new(log_path).parent() {
        if !parent.exists() {
            create_dir_all(parent).expect("Failed to create log directory");
        }
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        .expect("Failed to open log file");

    while let Some(entry) = rx.recv().await {
        if let Ok(json) = serde_json::to_string(&entry) {
            if let Err(e) = writeln!(file, "{}", json) {
                eprintln!("Failed to write to log file: {}", e);
            }
        }
    }
}