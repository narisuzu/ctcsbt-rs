use super::packet::Packet;
use super::packet_from_yamls;

packet_from_yamls! {
    "ctcsbt-packet/data/etcs16.yml",
    "ctcsbt-packet/data/etcs5.yml",
}
