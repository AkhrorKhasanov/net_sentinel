use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::ethernet::{EthernetPacket, EtherTypes};
use pnet::packet::Packet;
use crate::models::AuditEntry;
use chrono::Local;

pub fn process_packet(packet: &[u8], target_ip: &str) -> Option<AuditEntry> {
    let eth = EthernetPacket::new(packet)?;

    if eth.get_ethertype() != EtherTypes::Ipv4 {
        return None;
    }

    let ipv4 = Ipv4Packet::new(eth.payload())?;
    
    let src_ip = ipv4.get_source().to_string();
    let dst_ip = ipv4.get_destination().to_string();

    if src_ip != target_ip && dst_ip != target_ip {
        return None;
    }

    if let Some(tcp) = TcpPacket::new(ipv4.payload()) {
        let dst_port = tcp.get_destination();
        
        if dst_port != 80 && dst_port != 443 {
            return None;
        }

        return Some(AuditEntry {
            timestamp: Local::now().to_rfc3339(),
            src_ip,
            dst_ip,
            dst_port,
            protocol: "TCP".to_string(),
            flags: format!("{:?}", tcp.get_flags()),
            packet_size: packet.len(),
            ttl: ipv4.get_ttl(),
            window_size: tcp.get_window(),
        });
    }
    None
}