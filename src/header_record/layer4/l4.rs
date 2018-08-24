use header_record::layer4::icmp::IcmpPacket;

#[derive(Debug, Clone)]
pub enum Layer4Packet {
    Icmp(IcmpPacket),
    UDP(),
    TCP(),
    Unknown,
}
