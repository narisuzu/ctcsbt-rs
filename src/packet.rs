use super::telegram::Telegram;
use crate::{packet_from_yamls, telegram::TelegramBuilder};

packet_from_yamls! {
    "packet-data/etcs5.yml",
}
pub trait Packet {
    fn decode(data: &Telegram, start: u16, len: u16) -> Self;
    fn encode(&self, builder: &mut TelegramBuilder);
}

mod tests {
    #[test]
    fn test_packet_from_yaml() {
        use super::*;
        let packet = Etcs5 {
            nid_packet: 0x0,
            q_dir: 0x0,
            l_packet: 0x0,
            q_scale: 0x0,
            d_link: 0x0,
            q_newcountry: 0x0,
            nid_c: 0x0,
            nid_bg: 0x0,
            q_linkorientation: 0x0,
            q_linkreaction: 0x0,
            q_locacc: 0x0,
            n_iter: 0x0,
            d_link_k: List {
                data: [0x0; 31],
                len: 0x0,
            },
        };
    }
}
