use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct AuditEntry {
    pub timestamp: String,
    pub src_ip: String,
    pub dst_ip: String,
    pub dst_port: u16,
    pub ptorocol: String,
    pub flags: String,
    pub packet_size: usize,
    pub ttl: u8,
    pub window_size: u16,
}