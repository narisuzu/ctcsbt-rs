use core::{
    fmt::Debug,
    ops::{AddAssign, Shl},
    panic,
};

type RawSegment = u32;
const SEGMENT_LEN: u8 = (1024 / RawSegment::BITS) as u8;
pub type Telegram = [RawSegment; SEGMENT_LEN as usize];

trait ValueTrait<T> {
    fn value_of(&self, from: u8, to: u8) -> T;
}

impl<T> ValueTrait<T> for Telegram
where
    T: TryFrom<u8> + TryFrom<RawSegment> + AddAssign + Default + Copy + Shl<Output = T>,
    <T as TryFrom<RawSegment>>::Error: Debug,
    <T as TryFrom<u8>>::Error: Debug,
{
    fn value_of(&self, from: u8, to: u8) -> T {
        // the width support max to 128
        if from > to || to - from > 128 {
            panic!("range overflow")
        }
        let (first_segment_pos, last_segment_pos) = (from / SEGMENT_LEN, (to - 1) / SEGMENT_LEN);
        let (relative_start, relative_end) = (from % SEGMENT_LEN, (to - 1) % SEGMENT_LEN);
        // println!("rs: {}, re: {}", relative_start, relative_end);
        let k = last_segment_pos - first_segment_pos + 1;
        // println!("k: {}", k);
        let mut sum = T::default();
        let mut offset = 0;

        for i in 0..k {
            let mut segment = self[(last_segment_pos - i) as usize];
            if i == k-1 {
                segment = (segment << relative_start) >> relative_start;
            }
            if i == 0 {
                segment >>= SEGMENT_LEN - relative_end - 1;
            }
            //根據所劃定的寬度決定類型
            let expanded: T = segment.try_into().unwrap();
            sum += expanded << offset.try_into().unwrap();
            offset += if i == 0 { relative_end + 1 } else { SEGMENT_LEN };
        }
        sum
    }
}
