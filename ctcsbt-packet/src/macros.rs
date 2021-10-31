
#[macro_export]
macro_rules! packet_from_yamls {
    ($($path: literal,)*) => {
        use ctcsbt_macro::packet_from_yaml;
        use ctcsbt_macro::Packet;

        $(
            packet_from_yaml!($path);
        )*
    }
}