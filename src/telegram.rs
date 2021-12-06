use core::mem::size_of;
use core::ops::{BitOrAssign, Shl};
use core::{fmt::Debug, panic};

type RawSegment = u32;
type BitIndex = u16; //2^16 = 65536
/// the amount of segments
const SEGMENT_LEN: u16 = 1024 / RawSegment::BITS as u16;

pub type Telegram = [RawSegment; SEGMENT_LEN as usize];

const fn bits<T: Sized>() -> BitIndex {
    size_of::<T>() as BitIndex * 8
}

trait ValueTrait<T: Sized> {
    fn get_val(&self, from: BitIndex, to: BitIndex) -> Result<T, &'static str>;
    fn get_bit(&self, at: BitIndex) -> bool;
    fn set_val(&mut self, at: BitIndex, val: T);
}

impl<T> ValueTrait<T> for Telegram
where
    T: BitOrAssign + Default + Copy + Shl<BitIndex, Output = T> + TryFrom<RawSegment>,
    <T as TryFrom<RawSegment>>::Error: Debug,
{
    fn get_val(&self, from: BitIndex, to: BitIndex) -> Result<T, &'static str> {
        if from > to || to - from > bits::<T>() {
            return Err("Range Overflow");
        }
        let (first_segment_pos, last_segment_pos) = (from / SEGMENT_LEN, (to - 1) / SEGMENT_LEN);
        let (relative_start, relative_end) = (from % SEGMENT_LEN, (to - 1) % SEGMENT_LEN);
        let k = last_segment_pos - first_segment_pos;
        let mut sum = T::default();
        let mut offset = relative_end + 1;
        let mut is_last = true;
        for i in 0..=k {
            let mut segment = self[(last_segment_pos - i) as usize];
            if i == k {
                segment = (segment << relative_start) >> relative_start;
            }
            if is_last {
                is_last = false;
                segment >>= SEGMENT_LEN - offset;
                sum = segment.try_into().unwrap();
            } else {
                let expanded: T = segment.try_into().unwrap();
                sum |= expanded << offset;
                offset += SEGMENT_LEN;
            }
        }
        Ok(sum)
    }

    fn set_val(&mut self, at: BitIndex, val: T) {
        let (segmant_pos, relative_pos) = (at / SEGMENT_LEN, at % SEGMENT_LEN);
        bits::<T>() / SEGMENT_LEN + 1;
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

    fn write<T: Sized>(&mut self, data: T, len: u8) -> Result<(), &'static str> {
        //  let data = data << (bits::<T>() - len as BitIndex);
        self.buf;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Telegram;
    use crate::telegram::ValueTrait;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_get_val() {
        let mut rng = thread_rng();
        let tg: Telegram = rng.gen();

        let iter = [(10, 17), (20, 34), (68, 120), (330, 440)];

        println!("Telegram:");
        for (i, j) in tg.into_iter().enumerate() {
            println!("{:4}: {:032b}", i * 32, j);
        }

        for (x, y) in iter {
            let re_8: Result<u8, _> = tg.get_val(x, y);
            let re_16: Result<u16, _> = tg.get_val(x, y);
            let re_32: Result<u32, _> = tg.get_val(x, y);
            let re_64: Result<u64, _> = tg.get_val(x, y);
            let re_128: Result<u128, _> = tg.get_val(x, y);

            if let Ok(k) = re_8 {
                println!("8b:[{},{}) {:08b}", x, y, k)
            }
            if let Ok(k) = re_16 {
                println!("16b:[{},{}) {:016b}", x, y, k)
            }
            if let Ok(k) = re_32 {
                println!("32b:[{},{}) {:032b}", x, y, k)
            }
            if let Ok(k) = re_64 {
                println!("64b:[{},{}) {:064b}", x, y, k)
            }
            if let Ok(k) = re_128 {
                println!("128b:[{},{}) {:0128b}", x, y, k)
            }
        }
    }
}
