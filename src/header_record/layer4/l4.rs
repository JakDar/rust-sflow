use header_record::layer4::icmp::IcmpPacket;
use header_record::layer4::tcp::TcpPacket;
use header_record::layer4::udp::UdpPacket;

#[derive(Debug, Clone)]
pub enum Layer4Packet {
    Icmp(IcmpPacket),
    UDP(UdpPacket), //TODO - add udp :x
    TCP(TcpPacket),
    Unknown,
}
