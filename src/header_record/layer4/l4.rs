use header_record::layer4::icmp::IcmpPacket;
use header_record::layer4::tcp::TcpPacket;

#[derive(Debug, Clone)]
pub enum Layer4Packet {
    Icmp(IcmpPacket),
    UDP(), //TODO - add udp :x
    TCP(TcpPacket),
    Unknown,
}
