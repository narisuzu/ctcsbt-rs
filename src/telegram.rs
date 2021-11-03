use core::{
    fmt::Debug,
    ops::{AddAssign, Shl},
    panic,
};

type RawSegment = u32;
type BitIndex = u16; //2^16 = 65536
/// the amount of segments
const SEGMENT_LEN: u16 = 1024 / RawSegment::BITS as u16;
pub type Telegram = [RawSegment; SEGMENT_LEN as usize];

trait ValueTrait<T, N> {
    fn get_range_val(&self, from: BitIndex, to: BitIndex) -> T;
    fn get_bit(&self, at: BitIndex) -> N;
}

impl<T> ValueTrait<T, bool> for Telegram
where
    T: TryFrom<BitIndex> + TryFrom<RawSegment> + AddAssign + Default + Copy + Shl<Output = T>,
    <T as TryFrom<RawSegment>>::Error: Debug,
    <T as TryFrom<BitIndex>>::Error: Debug,
{
    fn get_range_val(&self, from: BitIndex, to: BitIndex) -> T {
        if from > to || to - from > 128 {
            panic!("range overflow")
        }
        let (first_segment_pos, last_segment_pos) = (from / SEGMENT_LEN, (to - 1) / SEGMENT_LEN);
        let (relative_start, relative_end) = (from % SEGMENT_LEN, (to - 1) % SEGMENT_LEN);
        let k = last_segment_pos - first_segment_pos + 1;
        let mut sum = T::default();
        let mut offset = 0;

        for i in 0..k {
            let mut segment = self[(last_segment_pos - i) as usize];
            if i == k - 1 {
                segment = (segment << relative_start) >> relative_start;
            }
            if i == 0 {
                segment >>= SEGMENT_LEN - relative_end - 1;
            }
            //根據所劃定的寬度決定類型
            let expanded: T = segment.try_into().unwrap();
            sum += expanded << offset.try_into().unwrap();
            offset += if i == 0 {
                relative_end + 1
            } else {
                SEGMENT_LEN
            };
        }
        sum
    }

    fn get_bit(&self, at: BitIndex) -> bool {
        let (segment_pos, relative_pos) = (at / SEGMENT_LEN, at % SEGMENT_LEN);
        self[segment_pos as usize] & (1 << SEGMENT_LEN - relative_pos - 1) != 0
    }
}

pub struct TelegramBuilder {
    data: Telegram,
    len: u16,
}

impl TelegramBuilder {
    fn new() -> Self {
        TelegramBuilder {
            data: Telegram::default(),
            len: 0,
        }
    }

    fn build(&self) -> Telegram {
        self.data
    }

    fn write<T>(&mut self, data: T) -> Result<(), &'static str> {
        Ok(())
    }
}
