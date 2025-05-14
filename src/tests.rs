use crate::{decode_with_replacement};
use std::str::FromStr;


#[test]
fn test_latin1_default() {
    let decode_result = decode_with_replacement(&vec![97, 98, 32, 67, 68, 46, 192, 233, 167, 92, 126]);
    assert_eq!(decode_result.text, String::from_str("ab CD.Àé§\\~").unwrap());
    assert_eq!(decode_result.replacement_added, false);
}

#[test]
fn test_latin1_specified() {
    let decode_result = decode_with_replacement(&vec![27, 45, 65, 97, 98, 32, 67, 68, 46, 192, 233, 167, 92, 126]);
    assert_eq!(decode_result.text, String::from_str("ab CD.Àé§\\~").unwrap());
    assert_eq!(decode_result.replacement_added, false);
}

#[test]
fn test_x0201_full_width() {
    let decode_result = decode_with_replacement(&vec![27, 41, 73, 27, 40, 74, 97, 98, 32, 67, 68, 46, 192, 167, 92, 126]);
    assert_eq!(decode_result.text, String::from_str("ab CD.タァ¥‾").unwrap());
    assert_eq!(decode_result.replacement_added, false);
}

#[test]
fn test_iso_8859_1() {
    
}



