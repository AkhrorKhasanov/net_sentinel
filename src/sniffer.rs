use pnet::datalink::{self, Channel::Ethernet};
use tokio::sync::mpsc::Sender;
use crate::processor::process_packet;

pub async fn start_sniffer(target_ip: String, tx: Sender<crate::models::AuditEntry>) {
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(|i| i.is_up() && !i.is_loopback() && !i.ips.is_empty())
        .expect("No active network interface found! Please check your internet connection.");

    println!("NetSentinel: Started sniffing on interface: {}...", interface.name);

    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Only Ethernet channels are supported"),
        Err(e) => panic!("Error opening network channel: {}", e),
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                if let Some(entry) = process_packet(packet, &target_ip) {
                    if let Err(e) = tx.send(entry).await {
                        eprintln!("Failed to send audit entry to logger: {}", e);
                    }
                }
            }
            Err(e) => eprintln!("Error receiving packet: {}", e),
        }
    }
}