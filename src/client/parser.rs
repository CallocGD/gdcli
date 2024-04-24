use std::collections::HashMap;
use itertools::Itertools;

/* Turns an HTTP Data Response into a string with a delimiter */
pub fn dictify<'a>( string: &'a str, delimiter: &'a str) -> HashMap<&'a str, &'a str> {
    string.split(delimiter).tuples().collect()
}

// pub (crate) fn dictify(string:&String, delimiter:&str) -> HashMap<&str, &str>{
//     string.split(delimiter).tuples().map(|(key, val)| (key, val)).collect()
// }


/// gets a key and converts it to a unsigned long long
pub fn key_for_u64(dict:&HashMap<&str, &str>, key:&str) -> u64 {
    dict.get(key).unwrap_or(&"0").parse::<u64>().unwrap_or(0)
}

pub fn key_for_i64(dict:&HashMap<&str, &str>, key:&str) -> i64 {
    dict.get(key).unwrap_or(&"0").parse::<i64>().unwrap_or(0)
}

pub fn key_for_u32(dict:&HashMap<&str, &str>, key:&str) -> u32 {
    dict.get(key).unwrap_or(&"0").parse::<u32>().unwrap_or(0)
}

pub fn key_for_u8(dict:&HashMap<&str, &str>, key:&str) -> u8 {
    dict.get(key).unwrap_or(&"0").parse::<u8>().unwrap_or(0)
}

pub fn key_for_u16(dict:&HashMap<&str, &str>, key:&str) -> u16 {
    dict.get(key).unwrap_or(&"0").parse::<u16>().unwrap_or(0)
}

pub fn key_for_str(dict:&HashMap<&str, &str>, key:&str) -> String {
    dict.get(key).unwrap_or(&"").to_string()
}

pub fn key_for_bool(dict:&HashMap<&str, &str>, key:&str) -> bool {
    dict.get(key).unwrap_or(&"0").starts_with("1")
}


// trades in a key for the value of x
// pub fn key_for_x<T:FromStr>(dict:&HashMap<&str, &str>, key:&str) -> Result<T, T::Err> {
//     FromStr::from_str(dict.get(key).unwrap_or(&"0"))
// }

