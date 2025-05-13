use std::char::REPLACEMENT_CHARACTER;


use crate::charset::Charset;
use crate::decode_block::DecodeBlock;
pub use crate::encoder::Encoder;

mod charset;
mod decode_block;
mod encoder;

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







pub fn decode(bytes: &Vec<u8>) -> (String, bool) {

    let mut decode_block = DecodeBlock::new();
    let mut decoded_string = String::with_capacity(150);

    let mut start_index: usize = 0;
    let mut replacement_char_added = false;
    fn assign_right(result: Result<Charset, &'static str>, decoded_string: &mut String, decode_block: &mut DecodeBlock, replacement_char_added: &mut bool) {
        match result {
            Ok(charset) => {
                decode_block.assign_right(charset);
            },
            Err(_) => {
                decoded_string.push(REPLACEMENT_CHARACTER);
                *replacement_char_added = true;
            }
        }
    }
    fn assign_left(result: Result<Charset, &'static str>, decoded_string: &mut String, decode_block: &mut DecodeBlock, replacement_char_added: &mut bool) {
        match result {
            Ok(charset) => {
                decode_block.assign_left(charset);
            },
            Err(_) => {
                decoded_string.push(REPLACEMENT_CHARACTER);
                *replacement_char_added = true;
            }
        }
    }
    

    
    let mut byte_index: usize = 0;
    while byte_index < bytes.len() {
        if bytes[byte_index] == ESC {

            decode_block.decode(&bytes[start_index..byte_index], &mut decoded_string);

            if bytes.len() - byte_index <= 2 {
                return (decoded_string, replacement_char_added);
            }

            byte_index += 1;
            match bytes[byte_index] {
                GL_94 => {
                    byte_index += 1;
                    let result = match_94(bytes[byte_index]);
                    assign_left(result, &mut decoded_string, &mut decode_block, &mut replacement_char_added);
                }
                GR_94 => {
                    byte_index += 1;
                    let result = match_94(bytes[byte_index]);
                    assign_right(result, &mut decoded_string, &mut decode_block, &mut replacement_char_added);
                }

                GR_96 => {
                    byte_index += 1;
                    let result = match_96(bytes[byte_index]);
                    assign_right(result, &mut decoded_string, &mut decode_block, &mut replacement_char_added); 
                    
                }
                G_94N => {
                    if bytes.len() - byte_index <= 2 {
                        return (decoded_string, replacement_char_added);
                    }
                    byte_index += 1;
                    match bytes[byte_index] {
                        GL_94N => {
                            byte_index += 1;
                            let result =match_94n(bytes[byte_index]);
                            assign_left(result, &mut decoded_string, &mut decode_block, &mut replacement_char_added);
                        }
                        GR_94N => {
                            byte_index += 1;
                            let result = match_94n(bytes[byte_index]);
                            assign_right(result, &mut decoded_string, &mut decode_block, &mut replacement_char_added);
                        }
                        _ => {
                            decoded_string.push(REPLACEMENT_CHARACTER);
                            replacement_char_added = true;
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
                            decoded_string.push(REPLACEMENT_CHARACTER);
                            replacement_char_added = true;
                        }
                    }
                }
                _ => {
                    decoded_string.push(REPLACEMENT_CHARACTER);
                    replacement_char_added = true;
                }

            }
            start_index = byte_index + 1;
        }
        byte_index += 1;
    }
    decode_block.decode(&bytes[start_index..], &mut decoded_string);
    
    return (decoded_string, replacement_char_added);
}

fn match_96(byte: u8) -> Result<Charset, &'static str> {
    match byte {
        ISO_8859_1 => {
            Ok(Charset::Iso8859_1)
        }
        ISO_8859_2 => {
            Ok(Charset::Iso8859_2)
        }
        ISO_8859_3 => {
            Ok(Charset::Iso8859_3)
        }
        ISO_8859_4 => {
            Ok(Charset::Iso8859_4)
        }
        ISO_8859_5 => {
            Ok(Charset::Iso8859_5)
        }
        ISO_8859_6 => {
            Ok(Charset::Iso8859_6)
        }
        ISO_8859_7 => {
            Ok(Charset::Iso8859_7)
        }
        ISO_8859_8 => {
            Ok(Charset::Iso8859_8)
        }
        ISO_8859_9 => {
            Ok(Charset::Iso8859_9)
        }
        _ => {
            Err("No matching 96 character Charset")
        }
    }
}

fn match_94n (byte: u8) -> Result<Charset, &'static str> {
    match byte {
        GB2312 => {
            Ok(Charset::GB2312)
        }
        JIS_X0208 => {
            Ok(Charset::JisX0208)
        }
        KS_C5601 => {
            Ok(Charset::KSC5601)
        }
        _ => {
            //panic!("Unrecognized 94N character charset");
            Err("No matching 94N character Charset")
        }
        
    }
}

fn match_94(byte: u8) -> Result<Charset, &'static str> {
    match byte {
        ASCII => {
            Ok(Charset::Ascii)
        }
        JIS_X0201_L => {
            Ok(Charset::JisX0201L)
        }
        JIS_X0201_R => {
            Ok(Charset::JisX0201R)
        }
        _ => {
            //panic!("Unrecognized 94 character charset");
            Err("No matching 94 character Charset")
        }
    }
}
