use std::io::SeekFrom;

use error;
use types;
use utils::{Decodeable, DecodeableWithSize};

#[derive(Debug, Clone, Default)]
pub struct UdpPacket {
    pub src_port: u16,
    pub dst_port: u16,
    pub length: u16,
    pub checksum: u16,
}

impl DecodeableWithSize for UdpPacket {
    #[inline]
    fn read_and_decode_with_size(bytes: i64, stream: &mut types::ReadSeeker) -> Result<Self, error::Error> {
        let src_port = u16::read_and_decode(stream)?;
        let dst_port = u16::read_and_decode(stream)?;

        let length= u16::read_and_decode(stream)?; // in bytes
        let checksum= u16::read_and_decode(stream)?;

        let bytes_left= bytes.min(length as i64) - 8i64; //todo - probably bytes- 8 would work
        stream.seek(SeekFrom::Current(bytes_left))?;



        let packet = UdpPacket{
            src_port,
            dst_port,
            length,
            checksum,
        };

        Ok(packet)
    }
}
