use pnet::datalink::{self, Channel::Ethernet};
use tokio::sync::mpsc::Sender;
use crate::processor::process_packet;

pub async fn start_sniffer(target_ip: String, tx: Sender<crate::models::AuditEntry>) {
    let interfaces = datalink::interfaces();
    
    let interface = interfaces
        .into_iter()
        .find(|i| !i.ips.is_empty() && i.is_up())
        .expect("No active network interface with an IP address found. Please connect to a network.");

    println!("NetSentinel: Using interface: {}", interface.name);
    println!("NetSentinel: Listening for traffic to/from: {}", target_ip);

    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unsupported channel type"),
        Err(e) => panic!("Failed to open network channel: {}", e),
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                if let Some(entry) = process_packet(packet, &target_ip) {
                    let _ = tx.send(entry).await;
                }
            }
            Err(e) => eprintln!("Packet capture error: {}", e),
        }
    }
}