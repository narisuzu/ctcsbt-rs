use core::{convert::From, fmt::Debug, panic};

type RawSegment = u32;
const SEGMENT_LEN: u8 = (1024 / RawSegment::BITS) as u8;

#[derive(Debug)]
pub struct Telegram {
    data: [RawSegment; SEGMENT_LEN as usize],
    len: u16,
}

fn range(from: u8, to: u8) -> (u8, u8) {
    // 最大支援到 128 位，因此to不得大於 128
    if from > to || to > 128 {
        panic!("range overflow")
    }

    (from, to)
}

impl Telegram {
    fn value_of<T>(&self, from: u8, to: u8) -> T
    where
        T: TryFrom<RawSegment>,
        <T as TryFrom<RawSegment>>::Error: Debug,
    {
        let (from, to) = range(from, to);
        let (first_pos, last_pos) = (from / SEGMENT_LEN, to / SEGMENT_LEN);
        for i in first_pos..=last_pos {
            let segment = self.data[i as usize];
            // 使用 mask 純化所求區間
            let purified = segment & segment_mask(from, to);
            //根據所劃定的寬度決定類型
            let expanded: T = purified.try_into().unwrap();
        }
        todo!()
    }
}

/// generate the mask of a u32
fn segment_mask(from: u8, to: u8) -> RawSegment {
    if from > to || to - from > SEGMENT_LEN {
        panic!("range overflow");
    }
    ((1 << (SEGMENT_LEN - from)) - 1) - ((1 << (SEGMENT_LEN - to)) - 1)
}
