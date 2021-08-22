pub trait Telegram {
    fn bits<T>(&self, pos: usize, len: usize) -> T;
}

impl Telegram for [bool; 1024] {
    fn bits(&self, pos: usize, len: usize) -> u8 {
        let slice = &self[pos .. pos+len];
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tg() {
        let test = vec![3u8, 3u8];

        for i in 0..test.len()*u8::BITS as usize {
            println!("{}", if test.bit(i) {1} else {0})
        }
    }
}
