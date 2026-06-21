use pnet::datalink::{self, Channel::Ethernet};
use tokio::sync::mpsc::Sender;
use crate::processor::process_packet;

pub async fn start_sniffer(target_ip: String, tx: Sender<crate::models::AuditEntry>) {
    let interfaces = datalink::interfaces();

    println!("NetSentinel: Scanning available network interfaces...");
    for iface in &interfaces {
        println!("Found: {} | IPs: {:?} | Up: {}", iface.name, iface.ips, iface.is_up());
    }

    let interface = interfaces
        .into_iter()
        .find(|i| !i.is_loopback() && !i.ips.is_empty())
        .expect("No suitable network interface found. Please ensure Npcap is installed and you are running as Administrator.");

    println!("NetSentinel: Using interface: {}", interface.name);
    println!("NetSentinel: Listening for traffic to/from: {}", target_ip);

    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unsupported channel type - only Ethernet is supported"),
        Err(e) => panic!("Failed to open network channel: {}. Make sure you have Npcap installed and are running with Administrator privileges.", e),
    };

    println!("NetSentinel: Sniffing started successfully.");

    loop {
        match rx.next() {
            Ok(packet) => {
                if let Some(entry) = process_packet(packet, &target_ip) {
                    if let Err(e) = tx.send(entry).await {
                        eprintln!("Failed to send audit entry: {}", e);
                    }
                }
            }
            Err(e) => eprintln!("Packet capture error: {}", e),
        }
    }
}