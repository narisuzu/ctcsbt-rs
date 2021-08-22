mod telegram;
mod packets;

pub trait Packet {
    fn decode(data: u16, start: u16, len: u16) -> Self;
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
