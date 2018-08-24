use header_record::ipv4_packet::Ipv4Packet;

add_decoder! {
#[derive(Debug, Clone)]
pub struct EthernetPacket{
    pub source: [u8;6],
    pub destination : [u8;6],

    pub type_ : u16, // 2 byte - 0x0800 for ipv4

    pub packet:Ipv4Packet,
}
}

// todo - add enough support for IPV6 to skip ipv6 packets



