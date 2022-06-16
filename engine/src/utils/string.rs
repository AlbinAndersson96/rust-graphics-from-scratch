use std::error::Error;

pub fn string_to_utf16_u16_vec(string: &String) -> Vec<u16> {
    let mut string_new: Vec<u16> = string.encode_utf16().collect();
    string_new.push(0);

    string_new
}

pub fn utf16_u16_vec_to_string(u16_vec: &Vec<u16>) -> Result<String, Box<dyn Error>> {

    let s_res = String::from_utf16(&u16_vec)?;

    Ok(s_res.trim_matches(char::from(0)).to_owned())
}