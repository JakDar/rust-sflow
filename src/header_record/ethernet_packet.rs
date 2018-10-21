use header_record::ipv4_packet::Ipv4Packet;
use utils::DecodeableWithSize;
use types::ReadSeeker;
use error::Error;

#[derive(Debug, Clone)]
pub struct SampledEthernetPacket {
    pub destination: [u8; 6],
    pub source: [u8; 6],

    pub type_: u16, // 2 byte - 0x0800 for ipv4

    pub packet: Ipv4Packet,
}

impl DecodeableWithSize for SampledEthernetPacket {
    fn read_and_decode_with_size(bytes: i64, stream: &mut ReadSeeker) -> Result<Self, Error> where Self: Sized {
        let s: SampledEthernetPacket = SampledEthernetPacket {
            destination: ::utils::Decodeable::read_and_decode(stream)?,
            source: ::utils::Decodeable::read_and_decode(stream)?,
            type_: ::utils::Decodeable::read_and_decode(stream)?,
            packet: ::utils::DecodeableWithSize::read_and_decode_with_size(bytes - 14i64, stream)?,

        };
        Ok(s)
    }
}


// todo - add enough support for IPV6 to skip ipv6 packets



