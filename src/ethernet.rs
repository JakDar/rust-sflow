use ipaddress::{decode_ipv4, IPAddress};
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
    pub version: u8,
    pub header_length: u8,
    pub tos: u8,
    pub ecn: u8,
    pub total_length: u16,
    pub row1: u32,
    pub ttl: u8,
    pub protocol: u8,
    pub header_checksum: u16,
    pub source_addr: IPAddress,
    pub dst_addr: IPAddress,
    pub options_were_present: bool,

    pub content: IpContent,
}

impl ::utils::Decodeable for Ipv4Packet {
    fn read_and_decode(stream: &mut ::types::ReadSeeker) -> ::std::result::Result<Ipv4Packet, ::error::Error> {
        let version_and_ihl: u8 = ::utils::Decodeable::read_and_decode(stream)?;


        let version = version_and_ihl >> 4;
        let header_length = version_and_ihl % (1 << 4);

        let tos_and_ecn: u8 = ::utils::Decodeable::read_and_decode(stream)?;
        let total_length: u16 = ::utils::Decodeable::read_and_decode(stream)?;
        let row1: u32 = ::utils::Decodeable::read_and_decode(stream)?;
        let ttl: u8 = ::utils::Decodeable::read_and_decode(stream)?;
        let protocol: u8 = ::utils::Decodeable::read_and_decode(stream)?;
        let header_checksum: u16 = ::utils::Decodeable::read_and_decode(stream)?;
        let source_addr = decode_ipv4(stream)?;
        let dst_addr = decode_ipv4(stream)?;

        let options_were_present = header_length > 5;

        if options_were_present {
            let option_len_in_bytes = 4 * (header_length - 5);
            stream.seek(SeekFrom::Current(option_len_in_bytes as i64))?;
        }


        let packet: Ipv4Packet = Ipv4Packet {
            version,
            header_length,
            tos: tos_and_ecn >> 1,
            ecn: tos_and_ecn % 2,
            total_length,
            row1,
            ttl,
            protocol,
            header_checksum,
            source_addr,
            dst_addr,
            options_were_present: false,
            content: IpContent {},
        };
        stream.seek(SeekFrom::Current((packet.total_length - packet.header_length as u16 * 4u16) /*(todo*/ as i64))?;

        Ok(packet)
    }
}




