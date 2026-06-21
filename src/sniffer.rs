use pnet::datalink::{self, Channel::Ethernet};
use tokio::sync::mpsc::Sender;
use crate::processor::process_packet;

pub fn start_sniffer(target_ip: String, tx: Sender<crate::models::AuditEntry>) {
    let interace = datalink::interfaces().into_iter().find(|i| i.is_up()).expect("Interface not found");
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx), 
        _ => panic!("Error opening channel"),
    };

    loop {
        if let Ok(packet) = rx.next() {
            if let Some(entry) = process_packet(packet, &target_ip) {
                let _ = tx.blocking_send(entry);
            }
        }
    }
}