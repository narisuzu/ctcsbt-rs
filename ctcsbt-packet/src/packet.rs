
pub trait Packet {
    fn decode(data: &[u32; 32], start: u16, len: u16) -> Self;
    fn encode(&self);
}
