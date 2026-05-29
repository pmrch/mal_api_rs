#[macro_export]
macro_rules! my_hash_map {
    ( $( $key:expr => $val:expr ),* $(,)? ) => {{
        let mut map = std::collections::HashMap::new();
        $(
            // Every iteration of the macro creates an INDEPENDENT owned value
            let format_val = compact_str::format_compact!("{}", $val);
            map.insert($key, format_val);
        )*
        map
    }};
}
