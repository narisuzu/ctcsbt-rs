use core::{fmt::Debug, panic};

use crate::value_type::ValType;

type RawSegment = u32;
type BitIndex = u16; //2^16 = 65536
/// the amount of segments
const SEGMENT_LEN: u16 = 1024 / RawSegment::BITS as u16;

pub type Telegram = [RawSegment; SEGMENT_LEN as usize];

trait ValueTrait<T, N> {
    fn get_val(&self, from: BitIndex, to: BitIndex) -> T;
    fn get_bit(&self, at: BitIndex) -> N;
    fn set_val(&mut self, at: BitIndex, val: T);
}

impl<T> ValueTrait<T, bool> for Telegram
where
    T: ValType + TryFrom<BitIndex> + TryFrom<RawSegment>,
    <T as TryFrom<RawSegment>>::Error: Debug,
    <T as TryFrom<BitIndex>>::Error: Debug,
{
    fn get_val(&self, from: BitIndex, to: BitIndex) -> T {
        if from > to || (to - from) as u32 > T::BITS {
            panic!("range overflow")
        }
        let (first_segment_pos, last_segment_pos) = (from / SEGMENT_LEN, (to - 1) / SEGMENT_LEN);
        let (relative_start, relative_end) = (from % SEGMENT_LEN, (to - 1) % SEGMENT_LEN);
        let k = last_segment_pos - first_segment_pos;
        let mut sum = T::default();
        let mut offset = 0;

        for i in 0..=k {
            let mut segment = self[(last_segment_pos - i) as usize];
            if i == k {
                segment = (segment << relative_start) >> relative_start;
            }
            if i == 0 {
                segment >>= SEGMENT_LEN - relative_end - 1;
            }
            //根據所劃定的寬度決定類型
            let expanded: T = segment.try_into().unwrap();
            sum |= expanded << offset;
            offset += if i == 0 {
                relative_end + 1
            } else {
                SEGMENT_LEN
            } as u32;
        }
        sum
    }

    fn set_val(&mut self, at: BitIndex, val: T) {
        let (segmant_pos, relative_pos) = (at / SEGMENT_LEN, at % SEGMENT_LEN);
        T::BITS / SEGMENT_LEN as u32;
        todo!()
    }

    fn get_bit(&self, at: BitIndex) -> bool {
        let (segment_pos, relative_pos) = (at / SEGMENT_LEN, at % SEGMENT_LEN);
        self[segment_pos as usize] & (1 << SEGMENT_LEN - relative_pos - 1) != 0
    }
}

pub struct TelegramBuilder {
    buf: Telegram,
    len: u16,
}

impl TelegramBuilder {
    fn new() -> Self {
        TelegramBuilder {
            buf: Telegram::default(),
            len: 0,
        }
    }

    fn build(&self) -> Telegram {
        self.buf
    }

    fn write<T: ValType>(&mut self, data: T, len: u8) -> Result<(), &'static str> {
        let data = data << (T::BITS - len as u32);
        self.buf;
        Ok(())
    }
}
