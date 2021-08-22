// use ctcsbt_macro::*;
// use crate::telegram;
// use crate::Packet;

// #[derive(Packet)]
// struct Packet21 {
//     #[var(len = 8)]
//     nid_packet: u8,
//     #[var(len = 2)]
//     q_dir: u8,
//     #[var(len = 13)]
//     l_packet: u16,
//     #[var(len = 2)]
//     q_scale: u8,

//     // PART 2
//     #[var(len = 15)]
//     d_gradient: u16,
//     #[var(len = 1)]
//     q_gdir: u8,
//     #[var(len = 8)]
//     g_a: u8,

//     // PART 3
//     #[var(len = 5)]
//     n_iter: u8,
// }