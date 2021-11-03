use super::telegram::Telegram;
use crate::{packet_from_yamls, telegram::TelegramBuilder};

packet_from_yamls! {
    "packet-data/etcs5.yml",
}
pub trait Packet {
    fn decode(data: &Telegram, start: u16, len: u16) -> Self;
    fn encode(&self, builder: &mut TelegramBuilder);
}
