#[derive(Debug, Clone)]
pub enum Layer7Packet {
    Http(),
    Unknown,
}
