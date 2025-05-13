use crate::decode;
use std::str::FromStr;

use encoding_rs as  enc;

#[test]
fn test_latin1_default() {
    let (decoded_string, replacement_characters_added) = decode(&vec![97, 98, 32, 67, 68, 46, 192, 233, 167, 92, 126]);
    assert_eq!(decoded_string, String::from_str("ab CD.Àé§\\~").unwrap());
    assert_eq!(replacement_characters_added, false);
}

#[test]
fn test_latin1_specified() {
    let (decoded_string, replacement_characters_added) = decode(&vec![27, 45, 65, 97, 98, 32, 67, 68, 46, 192, 233, 167, 92, 126]);
    assert_eq!(decoded_string, String::from_str("ab CD.Àé§\\~").unwrap());
    assert_eq!(replacement_characters_added, false);
}

#[test]
fn test_x0201() {
    let (decoded_string, replacement_characters_added) = decode(&vec![27, 41, 73, 27, 40, 74, 97, 98, 32, 67, 68, 46, 192, 167, 92, 126]);
    assert_eq!(decoded_string, String::from_str("ab CD.ﾀｧ¥‾").unwrap());
    assert_eq!(replacement_characters_added, false);
}