use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::Packet;
use crate::models::AuditEntry;
use chrono::Local;

pub fn process_packet(packet: &[u8], target_ip: &str) -> Option<AuditEntry> {
    let ipv4 = Ipv4Packet::new(packet)?;
    
    // if ipv4.get_source().to_string() != target_ip && ipv4.get_destination().to_string() != target_ip {
    //     return None;
    // }

    if src_ip != target_ip && dst_ip != target_ip {
    return None;
}

    if let Some(tcp) = TcpPacket::new(ipv4.payload()) {
        return Some(AuditEntry {
            timestamp: Local::now().to_rfc3339(),
            src_ip: ipv4.get_source().to_string(),
            dst_ip: ipv4.get_destination().to_string(),
            dst_port: tcp.get_destination(),
            protocol: "TCP".to_string(),
            flags: format!("{:?}", tcp.get_flags()),
            packet_size: packet.len(),
            ttl: ipv4.get_ttl(),
            window_size: tcp.get_window(),
        });
    }
    None
}
