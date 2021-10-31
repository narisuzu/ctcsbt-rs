
pub trait Packet {
    fn decode(data: u16, start: u16, len: u16) -> Self;
}
