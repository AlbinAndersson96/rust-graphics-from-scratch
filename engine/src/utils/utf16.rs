#![allow(unused_macros)]
#![allow(unused_imports)]

macro_rules! to_utf16_vec {
    ($a:expr) => {
        {
            let mut vec = $a.encode_utf16().collect::<Vec<u16>>();
            vec.push(0);
            vec
        }
    }
} pub(crate) use to_utf16_vec;

macro_rules! to_utf16_string {
    ($a:expr) => {
        match String::from_utf16(&$a) {
            Ok(s) => s.trim_matches(char::from(0)).to_owned(),
            Err(e) => {
                eprintln!("Problem parsing String from UTF16, returning blank String: {}", e);
                String::new()
            }
        }
    }
} pub(crate) use to_utf16_string;

#[cfg(test)]
mod tests {
    use crate::utils::utf16::*;

    #[test]
    fn test_utf16_vec_macros() {
        let original_string = "8BIcOjxgMTKpaOoWgJv7";
        let utf16 = to_utf16_vec!(original_string);
        let s_new = to_utf16_string!(&utf16);
        assert_eq!(original_string, s_new);
    }
}