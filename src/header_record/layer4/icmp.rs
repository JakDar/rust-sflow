use error;
// Std Lib Imports
use std::io::SeekFrom;
use types;
use utils::{DecodeableWithSize, Decodeable};

#[derive(Debug, Clone, Default)]
pub struct IcmpPacket {
    pub icmp_type: u8,
    pub icmp_code: u8,
    pub header_checksum: u16,
}

impl DecodeableWithSize for IcmpPacket {
    #[inline]
    fn read_and_decode_with_size(bytes: i64, stream: &mut types::ReadSeeker) -> Result<Self, error::Error> {
        let packet = IcmpPacket {
            icmp_type: u8::read_and_decode(stream)?,
            icmp_code: u8::read_and_decode(stream)?,
            header_checksum: u16::read_and_decode(stream)?,
        };
        stream.seek(SeekFrom::Current(bytes - 4i64))?; // todo - 2 here, but should be 4 and trailer should be seeked above - no idea why lengths don't match
        Ok(packet)
    }
}
