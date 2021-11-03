#![no_std]

mod list;
mod macros;
mod packet;
mod telegram;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
