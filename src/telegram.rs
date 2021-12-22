use core::mem::size_of;
use core::ops::{BitOrAssign, Range, Shl};
use core::{fmt::Debug, panic};

const TELEGRAM_SIZE: usize = 1024;
type RawSegment = u32;
type BitIndex = u16; //2^16 = 65536
/// the amount of segments
const SEGMENT_BITS: BitIndex = RawSegment::BITS as BitIndex;
pub type Telegram = [RawSegment; TELEGRAM_SIZE / SEGMENT_BITS as usize];

const fn bits<T: Sized>() -> usize {
    size_of::<T>() * 8
}

trait ValueTrait<T: Sized> {
    fn get_val(&self, span: Range<BitIndex>) -> Result<T, &'static str>;
    fn get_bit(&self, at: BitIndex) -> bool;
    fn set_val(&mut self, at: BitIndex, val: T);
}

impl<T> ValueTrait<T> for Telegram
where
    T: BitOrAssign + Default + Copy + Shl<BitIndex, Output = T> + TryFrom<RawSegment>,
    <T as TryFrom<RawSegment>>::Error: Debug,
{
    fn get_val(&self, span: Range<BitIndex>) -> Result<T, &'static str> {
        if span.is_empty() || span.len() > bits::<T>() {
            return Err("Range Overflow");
        }
        let Range { start, end } = span;
        let (first_segment_pos, relative_start) = (start / SEGMENT_BITS, start % SEGMENT_BITS);
        let (last_segment_pos, relative_end) = ((end - 1) / SEGMENT_BITS, (end - 1) % SEGMENT_BITS);
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
                segment >>= SEGMENT_BITS - offset;
                sum = segment.try_into().unwrap();
            } else {
                let expanded: T = segment.try_into().unwrap();
                sum |= expanded << offset;
                offset += SEGMENT_BITS;
            }
        }
        Ok(sum)
    }

    fn set_val(&mut self, at: BitIndex, val: T) {
        let (segmant_pos, relative_pos) = (at / SEGMENT_BITS, at % SEGMENT_BITS);
        //bits::<T>() / SEGMENT_BITS + 1;
        todo!()
    }

    fn get_bit(&self, at: BitIndex) -> bool {
        let (segment_pos, relative_pos) = (at / SEGMENT_BITS, at % SEGMENT_BITS);
        self[segment_pos as usize] & (1 << SEGMENT_BITS - relative_pos - 1) != 0
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
    use crate::telegram::ValueTrait;
    #[test]
    fn test_get_val() {
        let tg = [
            0b01110110001001110000110010111010,
            0b10011001000111100110111101010010,
            0b00100111111000101101010001100000,
            0b00010001111101100011001101011101,
            0b01010000001001100101101010001011,
            0b10011011000111010000100111001010,
            0b01101110001010010101010100011101,
            0b10011101100111000100100010010011,
            0b00110111000111110110011011001110,
            0b11101010011011111101100100010000,
            0b10111111001011001010111001011110,
            0b10010010100111100010011111001001,
            0b11011111101111111010001111011001,
            0b11001010010011100010000001000000,
            0b00000101110101111100110001100000,
            0b00001001000100110101000101000000,
            0b11110000110010010111010000111101,
            0b00111100000111000100101111011011,
            0b11100010100110110101110111010011,
            0b11000101110100100000101110110010,
            0b01010001001011010011111110110101,
            0b10100010011011011100011111101011,
            0b00101011111001011001100110000101,
            0b11011000110000110011010111110010,
            0b10001011000001011000101010100100,
            0b10001011001101011110111110011010,
            0b00010100011100110000000001000110,
            0b11001000101011010101101011011100,
            0b00110011001001110010000001101000,
            0b01110110100101111010110101001100,
            0b00100101100010110010001100111100,
            0b01101110011110110001110111100001,
        ];

        let test_set = [10..17, 20..34, 68..120, 330..440];

        let re_8: Result<u8, _> = tg.get_val(test_set[0].clone());
        let re_16: Result<u16, _> = tg.get_val(test_set[0].clone());
        assert_eq!(re_8.unwrap(), 0b1001110u8);
        assert_eq!(re_16.unwrap(), 0b1001110u16);
    }
}
