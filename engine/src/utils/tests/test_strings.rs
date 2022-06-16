#[cfg(test)]
mod test_strings {
    use crate::utils::string::string_to_utf16_u16_vec;
    use crate::utils::string::utf16_u16_vec_to_string;

    #[test]
    fn test_string_to_utf16_u16_vec_and_back() {
        let original_string = "8BIcOjxgMTKpaOoWgJv7".to_owned();

        let s = string_to_utf16_u16_vec(&original_string);

        let s_new = utf16_u16_vec_to_string(&s);
        assert!(s_new.is_ok());

        assert_eq!(original_string, s_new.unwrap());
    }
}
