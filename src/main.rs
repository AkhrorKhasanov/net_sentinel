mod models;
mod logger;
mod processor;
mod sniffer;

use clap::Parser;
use tokio::sync::mpsc;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    ip: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    let (tx, rx) = mpsc::channel(100);

    println!("NetSentinel monitoring {}...", args.ip);

    tokio::spawn(logger::start_logger(rx));

    sniffer::start_sniffer(args.ip, tx).await;
}