pub mod config;
pub mod utils;

#[macro_export]
macro_rules! hashmap_populate {
    ($( $key: expr => $val: expr ),*) => {{
        let mut map = std::collections::HashMap::new();
        $( map.insert($key, $val); )*
        map
    }}
}
