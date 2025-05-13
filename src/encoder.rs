
use std::str::FromStr;

pub struct Encoder {

}

impl Encoder {

    pub fn encode(string: String) -> Vec<u8> {
        Vec::new()
    }

    pub fn hello() -> String {
        String::from_str("Hello From Ctext_decoder").unwrap()
    }
}