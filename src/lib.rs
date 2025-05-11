use std::slice::Windows;
use std::str::{Chars, FromStr};
//use encoding_rs::{WINDOWS_1252, EUC_JP, EUC_KR, GBK, SHIFT_JIS, ISO_8859_2, ISO_8859_3, ISO_8859_4, ISO_8859_5, ISO_8859_6, ISO_8859_7, ISO_8859_8, ISO_8859_9};
use encoding_rs::Encoding;
use encoding_rs as enc;

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

#[derive(PartialEq)]
enum Charset {
    Ascii,
    JisX0201L,
    JisX0201R,
    Iso8859_1,
    Iso8859_2,
    Iso8859_3,
    Iso8859_4,
    Iso8859_5,
    Iso8859_6,
    Iso8859_7,
    Iso8859_8,
    Iso8859_9,
    GB2312,
    KSC5601,
    JisX0208,

}

impl Charset {
    // fn is_non_ascii_yet_ascii_compatible(charset: Charset) -> bool{
    //     match charset {
    //         Charset::Ascii | Charset::JisX0201L | Charset::JisX0201R => {
    //             return false;
    //         }
    //         _ => {
    //             return true;
    //         }
    //     }
    // }
    fn is_non_ascii_yet_ascii_compatible(&self) -> bool {
        match self {
            Charset::Ascii | Charset::JisX0201L | Charset::JisX0201R => {
                return false;
            }
            _ => {
                return true;
            }
        }
    }

    fn is_natural_left(&self) -> bool {
        match self {
            Charset::Ascii | Charset::JisX0201L => {
                return true;
            }
            _ => {
                return false;
            }

        }
    }

    fn is_natural_right(&self) -> bool {
        match self {
            Charset::Ascii | Charset::JisX0201L => {
                return false;
            }
            _ => {
                return true;
            }

        }
    }

    fn match_charset(charset: &Charset) -> &'static enc::Encoding {
        let encoding;
        match charset {
            Charset::Ascii => {
                encoding = enc::WINDOWS_1252;
            }
            Charset::JisX0201L => {
                encoding = enc::SHIFT_JIS;
            }
            Charset::JisX0201R => {
                encoding = enc::SHIFT_JIS;
            }
            Charset::Iso8859_1 => {
                encoding = enc::WINDOWS_1252;
            }
            Charset::Iso8859_2 => {
                encoding = enc::ISO_8859_2;
            }
            Charset::Iso8859_3 => {
                encoding = enc::ISO_8859_3;
            }
            Charset::Iso8859_4 => {
                encoding = enc::ISO_8859_4;
            }
            Charset::Iso8859_5 => {
                encoding = enc::ISO_8859_5;
            }
            Charset::Iso8859_6 => {
                encoding = enc::ISO_8859_6;
            }
            Charset::Iso8859_7 => {
                encoding = enc::ISO_8859_7;
            }
            Charset::Iso8859_8 => {
                encoding = enc::ISO_8859_8;
            }
            Charset::Iso8859_9 => {
                encoding = enc::WINDOWS_1254;
            }
            Charset::KSC5601 => {
                encoding = enc::EUC_KR;
            }
            Charset::JisX0208 => {
                encoding = enc::EUC_JP;
            }
            Charset::GB2312 => {
                encoding = enc::GBK;
            }
        }
        return encoding;
    }
    fn get_encoding(&self) -> &'static Encoding {
        return Charset::match_charset(&self);
    }
}


struct DecodeBlock {

    is_utf8:  bool,
    is_single_decoder: bool,


    flip_right: bool,
    flip_left: bool,
    right_charset: Charset,
    left_charset: Charset,

}

impl DecodeBlock {
    fn new() -> DecodeBlock{
        DecodeBlock {

            is_utf8: false,

            is_single_decoder: true,

            flip_left: false,
            flip_right: false,
            left_charset: Charset::Ascii,
            right_charset: Charset::Iso8859_1,
    

        }
    }

    fn decode_single_decoder(encoding: &'static enc::Encoding, bytes: &[u8], string: &mut String) {
        let output = encoding.decode_without_bom_handling(bytes);
        //TODO: Check if replacement characters were added
        string.push_str(output.0.as_ref());
    }

    fn decode(&mut self, bytes: &[u8], string: &mut String) {
        if self.is_utf8 {
            return string.push_str(String::from_utf8_lossy(bytes).as_ref());
        }
        else if self.is_single_decoder {
            let encoding;
            match self.right_charset {
                Charset::JisX0201R => {
                    encoding = enc::SHIFT_JIS;
                }
                Charset::Iso8859_1 => {
                    encoding = enc::WINDOWS_1252;
                }
                Charset::Iso8859_2 => {
                    encoding = enc::ISO_8859_2;
                }
                Charset::Iso8859_3 => {
                    encoding = enc::ISO_8859_3;
                }
                Charset::Iso8859_4 => {
                    encoding = enc::ISO_8859_4;
                }
                Charset::Iso8859_5 => {
                    encoding = enc::ISO_8859_5;
                }
                Charset::Iso8859_6 => {
                    encoding = enc::ISO_8859_6;
                }
                Charset::Iso8859_7 => {
                    encoding = enc::ISO_8859_7;
                }
                Charset::Iso8859_8 => {
                    encoding = enc::ISO_8859_8;
                }
                Charset::Iso8859_9 => {
                    encoding = enc::WINDOWS_1254;
                }
                Charset::KSC5601 => {
                    encoding = enc::EUC_KR;
                }
                Charset::JisX0208 => {
                    encoding = enc::EUC_JP;
                }
                Charset::GB2312 => {
                    encoding = enc::GBK;
                }
                _ => {
                    unreachable!("Ascii or Jisx0201L in the right graphic and single decoder is true")
                }
            }
            return DecodeBlock::decode_single_decoder(encoding, bytes, string);
        }
        else {
            let mut flipped_bytes: Vec<u8> = Vec::with_capacity(bytes.len()); 
            let flip_right_operator = if self.flip_right { 128u8 } else { 0u8 };
            let flip_left_operator = if self.flip_right { 128u8 } else { 0u8 };
            let right_encoding = self.right_charset.get_encoding();
            let left_encoding = self.left_charset.get_encoding();
            let mut byte_index = 0;
            let mut start_index = 0;
            //let mut is_right = bytes[byte_index] & 128u8 == 128u8;
            //byte_index += 1;
            let mut output;
            while byte_index < bytes.len() {
                while bytes[byte_index] & 128u8 == 128u8 {
                    flipped_bytes[byte_index] = bytes[byte_index] ^ flip_right_operator;
                    byte_index += 1;
                }
                output = right_encoding.decode_without_bom_handling(&flipped_bytes[start_index..byte_index]);

                //TODO: Check if replacement characters were added
                string.push_str(output.0.as_ref());
                start_index = byte_index;

                while bytes[byte_index] & 128u8 == 0u8 {
                    flipped_bytes[byte_index] = bytes[byte_index] ^ flip_left_operator;
                    byte_index += 1;
                }
                output = left_encoding.decode_without_bom_handling(&flipped_bytes[start_index..byte_index]);

                //TODO: Check if replacement characters were added
                string.push_str(output.0.as_ref());
                start_index = byte_index;
            }
        }
    }

    fn assign_right(&mut self, charset: Charset) {
        if charset.is_natural_right() {
            self.flip_right = false;
            if charset.is_non_ascii_yet_ascii_compatible() {
                self.is_single_decoder = self.left_charset == Charset::Ascii;
            }
            else if charset == Charset::JisX0201R {
                self.is_single_decoder = self.left_charset == Charset::JisX0201L;
            }
        }
        else {
            self.flip_right = true;
            self.is_single_decoder = false;
        }
        self.right_charset = charset;
    }

    fn assign_left(&mut self, charset: Charset) {
        match charset {
            Charset::Ascii => {
                self.flip_left = false;
                if self.right_charset.is_non_ascii_yet_ascii_compatible() {
                    self.is_single_decoder = true;
                }
            }
            Charset::JisX0201L => {
                self.flip_left = false;
                self.is_single_decoder = self.right_charset == Charset::JisX0201R;
            }
            _ => {
                self.flip_left = true;
                self.is_single_decoder = false;
            }
        }
        self.left_charset = charset;
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
