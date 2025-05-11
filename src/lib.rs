use std::slice::Windows;
use std::str::{Chars, FromStr};
//use encoding_rs::{WINDOWS_1252, EUC_JP, EUC_KR, GBK, SHIFT_JIS, ISO_8859_2, ISO_8859_3, ISO_8859_4, ISO_8859_5, ISO_8859_6, ISO_8859_7, ISO_8859_8, ISO_8859_9};
use encoding_rs::Encoding;
use encoding_rs as enc;
use crate::charset::Charset;
use crate::decode_block::DecodeBlock;

mod charset;
mod decode_block;

//Start Escape Sequence
const ESC: u8 = 27;


//First byte of Escape Sequence
const GL_94: u8 = 40;
const GR_94: u8 = 41;
const GR_96: u8 = 45;
const G_94N: u8 = 36;
const SWITCH_ENCODING: u8 = 37;

const GL_94N: u8 = 40;
const GR_94N: u8 = 41;


const ASCII: u8 = 66;
const JIS_X0201_R: u8 = 73;
const JIS_X0201_L: u8 = 74;

const ISO_8859_1: u8 = 65;
const ISO_8859_2: u8 = 66;
const ISO_8859_3: u8 = 67;
const ISO_8859_4: u8 = 68;
const ISO_8859_5: u8 = 76;
const ISO_8859_6: u8 = 71;
const ISO_8859_7: u8 = 70;
const ISO_8859_8: u8 = 72;
const ISO_8859_9: u8 = 77;

const GB2312: u8 = 65;
const JIS_X0208: u8 = 66;
const KS_C5601: u8 = 67;

const UTF8_STANDARD_RETURN: u8 = 71;
const STANDARD_RETURN: u8 = 64;

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


struct Decoder {
    start: u64,
    end: u64,

    is_utf8: bool,
    is_single_encoder: bool,
    single_decoder: encoding_rs::Decoder,


    flip_right: bool,
    flip_left: bool,

    right_decoder: encoding_rs::Decoder,
    left_decoder: encoding_rs::Decoder,
}

impl Decoder {
    pub fn decode(bytes: Vec<u8>) -> String {
        
        for byte_index in 0..bytes.len() {

        }
        "".to_string()
    }
}







pub fn decode(bytes: &Vec<u8>) -> String {
    //println!("Hello from dec");
    println!("BYTES: {:?}", bytes);
    let mut decode_block = DecodeBlock::new();
    let mut decoded_string = String::with_capacity(150);

    let mut start_index: usize = 0;

    
    let mut byte_index: usize = 0;
    while byte_index < bytes.len() {
        if bytes[byte_index] == ESC {
            // println!("BYTE INDEX {}", byte_index);
            // println!("BYTES {:?}", &bytes[start..byte_index]);
            decode_block.decode(&bytes[start_index..byte_index], &mut decoded_string);

            if bytes.len() - byte_index <= 2 {
                return decoded_string;
            }
            // return decoded_string;
            byte_index += 1;
            match bytes[byte_index] {
                GL_94 => {
                    byte_index += 1;
                    decode_block.assign_left(match_94(bytes[byte_index]));
                }
                GR_94 => {
                    byte_index += 1;
                    decode_block.assign_right(match_94(bytes[byte_index]));
                }

                GR_96 => {
                    byte_index += 1;
                    decode_block.assign_right(match_96(bytes[byte_index]));
                }
                G_94N => {
                    if bytes.len() - byte_index <= 2 {
                        return decoded_string;
                    }
                    byte_index += 1;
                    match bytes[byte_index] {
                        GL_94N => {
                            byte_index += 1;
                            decode_block.assign_left(match_94n(bytes[byte_index]));
                        }
                        GR_94N => {
                            byte_index += 1;
                            decode_block.assign_right(match_94n(bytes[byte_index]));
                        }
                        _ => {
                            panic!("Unrecognized GR or GL Specifier for 94N");
                        }
                    }
                }   
                SWITCH_ENCODING => {
                    byte_index += 1;
                    match bytes[byte_index] {
                        UTF8_STANDARD_RETURN => {
                            decode_block.is_utf8 = true;
                        }
                        STANDARD_RETURN => {
                            decode_block.is_utf8 = false;
                        }
                        _ => {
                            panic!("Unrecognized switch encoding specifier");
                        }
                    }
                }
                _ => {
                    panic!("Unrecognized specifier after ESC");
                }

            }
            start_index = byte_index + 1;
        }
        byte_index += 1;
    }
    decode_block.decode(&bytes[start_index..], &mut decoded_string);
    decoded_string
}

fn match_96(byte: u8) -> Charset {
    match byte {
        ISO_8859_1 => {
            Charset::Iso8859_1
        }
        ISO_8859_2 => {
            Charset::Iso8859_2
        }
        ISO_8859_3 => {
            Charset::Iso8859_3
        }
        ISO_8859_4 => {
            Charset::Iso8859_4
        }
        ISO_8859_5 => {
            Charset::Iso8859_5
        }
        ISO_8859_6 => {
            Charset::Iso8859_6
        }
        ISO_8859_7 => {
            Charset::Iso8859_7
        }
        ISO_8859_8 => {
            Charset::Iso8859_8
        }
        ISO_8859_9 => {
            Charset::Iso8859_9
        }
        _ => {panic!("Unrecognized 96 character charset");}
    }
}

fn match_94n (byte: u8) -> Charset {
    match byte {
        GB2312 => {
            Charset::GB2312
        }
        JIS_X0208 => {
            Charset::JisX0208
        }
        KS_C5601 => {
            Charset::KSC5601
        }
        _ => {
            panic!("Unrecognized 94N character charset");
        }
    }
}

fn match_94(byte: u8) -> Charset {
    match byte {
        ASCII => {
            Charset::Ascii
        }
        JIS_X0201_L => {
            Charset::JisX0201L
        }
        JIS_X0201_R => {
            Charset::JisX0201R
        }
        _ => {
            panic!("Unrecognized 94 character charset");
        }
    }
}
