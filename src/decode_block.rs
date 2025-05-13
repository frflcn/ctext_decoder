use crate::Charset;
use encoding_rs as enc;

pub struct DecodeBlock {

    pub is_utf8:  bool,
    is_single_decoder: bool,


    flip_right: bool,
    flip_left: bool,
    right_charset: Charset,
    left_charset: Charset,

}

impl DecodeBlock {
    pub fn new() -> DecodeBlock{
        DecodeBlock {

            is_utf8: false,

            is_single_decoder: true,

            flip_left: false,
            flip_right: false,
            left_charset: Charset::Ascii,
            right_charset: Charset::Iso8859_1,
    

        }
    }

    fn decode_single_decoder(encoding: &'static enc::Encoding, bytes: &[u8], string: &mut String) -> bool {
        let (decoded_string, replacement_character_added) = encoding.decode_without_bom_handling(bytes);
        //TODO: Check if replacement characters were added
        string.push_str(&decoded_string);
        replacement_character_added
    }

    pub fn decode(&mut self, bytes: &[u8], string: &mut String) -> bool {
        if self.is_utf8 {
            let (decoded_string, replacement_character_added) = enc::UTF_8.decode_without_bom_handling(bytes);
            string.push_str(&decoded_string);
            return replacement_character_added;
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
            let mut replacement_character_added = false;
            let mut flipped_bytes: Vec<u8> = Vec::with_capacity(bytes.len()); 
            let flip_right_operator = if self.flip_right { 128u8 } else { 0u8 };
            let flip_left_operator = if self.flip_right { 128u8 } else { 0u8 };
            let right_encoding = self.right_charset.get_encoding();
            let left_encoding = self.left_charset.get_encoding();
            let mut byte_index = 0;
            let mut start_index = 0;
            let (mut decoded_string, mut replacement_character_added_temp);

            while byte_index < bytes.len() {
                while bytes[byte_index] & 128u8 == 128u8 {
                    flipped_bytes[byte_index] = bytes[byte_index] ^ flip_right_operator;
                    byte_index += 1;
                }

                (decoded_string, replacement_character_added_temp) = right_encoding.decode_without_bom_handling(&flipped_bytes[start_index..byte_index]);
                replacement_character_added = if replacement_character_added_temp { true } else { replacement_character_added };

                string.push_str(&decoded_string);
                start_index = byte_index;


                
                while bytes[byte_index] & 128u8 == 0u8 {
                    flipped_bytes[byte_index] = bytes[byte_index] ^ flip_left_operator;
                    byte_index += 1;
                }

                (decoded_string, replacement_character_added_temp) = left_encoding.decode_without_bom_handling(&flipped_bytes[start_index..byte_index]);
                replacement_character_added = if replacement_character_added_temp { true } else { replacement_character_added };

                string.push_str(&decoded_string);
                start_index = byte_index;
            };
            return replacement_character_added;
        }
    }

    pub fn assign_right(&mut self, charset: Charset) {
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

    pub fn assign_left(&mut self, charset: Charset) {
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