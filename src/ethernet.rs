use ipaddress::{IPAddress, decode_ipv4, decode_ipv6};
use std::io::SeekFrom;

add_decoder! {
#[derive(Debug, Clone, Default)]
pub struct EthernetPacket{
    pub source: [u8;6],
    pub destination : [u8;6],

    pub type_ : u16, // 2 byte - 0x0800 for ipv4

    pub packet:Ipv4Packet,
}
}

#[derive(Debug, Clone, Default)]
pub struct IpContent {}

// todo - ipv6?
#[derive(Debug, Clone, Default)]
pub struct Ipv4Packet {
    pub version_and_ihl: u8,
    pub tos_and_ecn: u8,
    pub total_length: u16,
    pub row1: u32,
    pub row2: u32,
    pub source_addr: IPAddress,
    pub dst_addr: IPAddress,
    //todo - handle non-empty options
    pub content: IpContent,
}

impl ::utils::Decodeable for Ipv4Packet {
    fn read_and_decode(stream: &mut ::types::ReadSeeker) -> ::std::result::Result<Ipv4Packet, ::error::Error> {
        let s: Ipv4Packet = Ipv4Packet {
            version_and_ihl: ::utils::Decodeable::read_and_decode(stream)?,
            tos_and_ecn: ::utils::Decodeable::read_and_decode(stream)?,
            total_length: ::utils::Decodeable::read_and_decode(stream)?,
            row1: ::utils::Decodeable::read_and_decode(stream)?,
            row2: ::utils::Decodeable::read_and_decode(stream)?,
            source_addr: decode_ipv4(stream)?,
            dst_addr: decode_ipv4(stream)?,
            content: IpContent {},
        };
        stream.seek(SeekFrom::Current((s.total_length - 20) /*(todo*/ as i64))?;

        Ok(s)
    }
}




