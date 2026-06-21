use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::Packet;
use tokio::sync::mpsc::Sender;
use crate::processor::process_packet;

pub fn start_sniffer(target_ip: String, tx: Sender<crate::models::AuditEntry>) {
    let interfaces = datalink::interfaces();
    
    let interface = interfaces
        .into_iter()
        .find(|i| i.name == "eth0")
        .expect("eth0 interfeysi topilmadi! Iltimos, interfeys nomini tekshiring.");

    println!("NetSentinel: {} interfeysi orqali tinglash boshlandi...", interface.name);

    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Faqat Ethernet kanallari qo'llab-quvvatlanadi"),
        Err(e) => panic!("Kanalni ochishda xatolik: {}", e),
    };

    println!("NetSentinel: Paketlar kutilmoqda...");

    loop {
        match rx.next() {
            Ok(packet) => {

                if let Some(entry) = process_packet(packet, &target_ip) {
                    let _ = tx.blocking_send(entry);
                }
            }
            Err(e) => {
                eprintln!("Xatolik: {}", e);
            }
        }
    }
}