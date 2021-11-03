#[macro_export]
macro_rules! packet_from_yamls {
    ($($path: literal,)*) => {
        use ctcsbt_macro::packet_from_yaml;
        use crate::list::List;

        $(packet_from_yaml!($path);)*
    }
}
