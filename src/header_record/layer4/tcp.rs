use error;
use std::io::SeekFrom;
use types;
use utils::{DecodeableWithSize, Decodeable, u8_to_bool_array};

#[derive(Debug, Clone, Default)]
pub struct TcpPacket {
    pub src_port: u16,
    pub dst_port: u16,
    pub sequence_number: u32,
    pub ack_number: u32,
    pub data_offset: u8,
    pub ns: bool,
    pub cwr: bool,
    pub ece: bool,
    pub urg: bool,
    pub ack: bool,
    pub psh: bool,
    pub rst: bool,
    pub syn: bool,
    pub fin: bool,
    pub window_size: u16,
    pub checksum: u16,
    pub urg_pointer: u16,
    //options are skipped
    pub data: String,
}

impl DecodeableWithSize for TcpPacket {
    #[inline]
    fn read_and_decode_with_size(bytes: i64, stream: &mut types::ReadSeeker) -> Result<Self, error::Error> {
        let src_port = u16::read_and_decode(stream)?;
        let dst_port = u16::read_and_decode(stream)?;

        let sequence_number = u32::read_and_decode(stream)?;
        let ack_number = u32::read_and_decode(stream)?;

        let octet12 = u8::read_and_decode(stream)?;
        let data_offset: u8 = octet12 >> 4;
        let ns = octet12 % 2 == 1;
        let octet13 = u8_to_bool_array(u8::read_and_decode(stream)?);

        let window_size = u16::read_and_decode(stream)?;
        let checksum = u16::read_and_decode(stream)?;
        let urg_pointer = u16::read_and_decode(stream)?;

        let options_len = (data_offset - 5) * 4;
        stream.seek(SeekFrom::Current(options_len as i64))?;

        let data_len = bytes - (data_offset * 4) as i64;

        let data = String::read_and_decode_with_size(data_len, stream)?;
        let packet = TcpPacket {
            src_port,
            dst_port,
            sequence_number,
            ack_number,
            data_offset,
            ns,
            cwr: octet13[0],
            ece: octet13[1],
            urg: octet13[2],
            ack: octet13[3],
            psh: octet13[4],
            rst: octet13[5],
            syn: octet13[6],
            fin: octet13[7],
            window_size,
            checksum,
            urg_pointer,
            data,
        };
        Ok(packet)
    }
}
