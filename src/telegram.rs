pub trait Telegram {
    fn value_of(&self, pos: usize, len: usize) -> TelegramValue;
}

impl Telegram for [u32; 32] {
    fn value_of(&self, pos: usize, len: usize) -> TelegramValue {
        let slice = &self[pos .. pos+len];
        todo!()
    }

}

// 報文值類型
enum TelegramValue {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128)
}

