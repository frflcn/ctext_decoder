use std::char::REPLACEMENT_CHARACTER;


use crate::charset::Charset;
use crate::decode_block::DecodeBlock;

mod charset;
mod decode_block;
mod charsets;
mod encoding;

#[cfg(test)]
mod tests;

//Start Escape Sequence
const ESC: u8 = 27;

//Bidi
const CSI: u8 = 0x9B;
const LEFT_TO_RIGHT: u8 = 0x31;
const RIGHT_TO_LEFT: u8 = 0x32;
const END_BIDI: u8 = 0x5D;

const LRE_UNICODE: char = char::from_u32(0x202A).unwrap();
const RLE_UNICODE: char = char::from_u32(0x202B).unwrap();
const PDF_UNICODE: char = char::from_u32(0x202C).unwrap();

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

pub struct DecodeWithReplacementResult {
    pub text: String,
    pub replacement_added: bool
}



pub fn decode_with_replacement(bytes: &Vec<u8>) -> DecodeWithReplacementResult {

    let mut decode_block = DecodeBlock::new();
    let mut decoded_string = String::with_capacity(bytes.len() * 4);

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

            if decode_block.decode(&bytes[start_index..byte_index], &mut decoded_string) {
                replacement_char_added = true;
            }

            if bytes.len() - byte_index <= 2 {
                decoded_string.shrink_to_fit();
                return DecodeWithReplacementResult {
                    text: decoded_string,
                    replacement_added: replacement_char_added
                };
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
                        decoded_string.shrink_to_fit();
                        return DecodeWithReplacementResult {
                            text: decoded_string,
                            replacement_added: replacement_char_added
                        };
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
        else if bytes[byte_index] == CSI {
            if decode_block.decode(&bytes[start_index..byte_index], &mut decoded_string) {
                replacement_char_added = true;
            }
            byte_index += 1;
            if byte_index >= bytes.len() {
                decoded_string.shrink_to_fit();
                return DecodeWithReplacementResult {
                    text: decoded_string,
                    replacement_added: replacement_char_added
                };
            }
            match bytes[byte_index] {
                LEFT_TO_RIGHT => {
                    byte_index += 1;
                    if byte_index >= bytes.len(){
                        decoded_string.shrink_to_fit();
                        return DecodeWithReplacementResult {
                            text: decoded_string,
                            replacement_added: replacement_char_added
                        };
                    }
                    else if bytes[byte_index] == END_BIDI {
                        decoded_string.push(LRE_UNICODE)
                    }
                    else {
                        decoded_string.push(REPLACEMENT_CHARACTER);
                        replacement_char_added = true;
                    }
                }
                RIGHT_TO_LEFT => {
                    byte_index += 1;
                    if byte_index >= bytes.len(){
                        decoded_string.shrink_to_fit();
                        return DecodeWithReplacementResult {
                            text: decoded_string,
                            replacement_added: replacement_char_added
                        };
                    }
                    else if bytes[byte_index] == END_BIDI {
                        decoded_string.push(RLE_UNICODE)
                    }
                    else {
                        decoded_string.push(REPLACEMENT_CHARACTER);
                        replacement_char_added = true;
                    }
                }
                END_BIDI => {
                    decoded_string.push(PDF_UNICODE)
                }
                _ => {
                    decoded_string.push(REPLACEMENT_CHARACTER);
                    replacement_char_added = true;
                }
            }
        }
        byte_index += 1;
    }
    if decode_block.decode(&bytes[start_index..], &mut decoded_string) {
        replacement_char_added = true;
    }
    decoded_string.shrink_to_fit();
    return DecodeWithReplacementResult {
        text: decoded_string,
        replacement_added: replacement_char_added
    };
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
            Err("No matching 94 character Charset")
        }
    }
}
