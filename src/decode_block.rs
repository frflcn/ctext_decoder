use crate::Charset;
use crate::encoding::{Encoding, GB_2312, JIS_X0201, JIS_X0208, KS_C5601, UTF8};
use crate::encoding::{ISO_8859_1, ISO_8859_2, ISO_8859_3};
use crate::encoding::{ISO_8859_4, ISO_8859_5, ISO_8859_6};
use crate::encoding::{ISO_8859_7, ISO_8859_8, ISO_8859_9};

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

    fn decode_single_decoder(encoding: Box<&dyn Encoding>, bytes: &[u8], string: &mut String) -> bool {
        encoding.decode(bytes, string)
    }

    pub fn decode(&mut self, bytes: &[u8], string: &mut String) -> bool {
        if self.is_utf8 {
            return UTF8.decode(bytes, string);
        }
        else if self.is_single_decoder {
            let encoding: Box<&dyn Encoding>;
            match self.right_charset {
                Charset::JisX0201R => {
                    encoding = Box::new(&JIS_X0201);
                }
                Charset::Iso8859_1 => {
                    encoding = Box::new(&ISO_8859_1);
                }
                Charset::Iso8859_2 => {
                    encoding = Box::new(&ISO_8859_2);
                }
                Charset::Iso8859_3 => {
                    encoding = Box::new(&ISO_8859_3);
                }
                Charset::Iso8859_4 => {
                    encoding = Box::new(&ISO_8859_4);
                }
                Charset::Iso8859_5 => {
                    encoding = Box::new(&ISO_8859_5);
                }
                Charset::Iso8859_6 => {
                    encoding = Box::new(&ISO_8859_6);
                }
                Charset::Iso8859_7 => {
                    encoding = Box::new(&ISO_8859_7);
                }
                Charset::Iso8859_8 => {
                    encoding = Box::new(&ISO_8859_8);
                }
                Charset::Iso8859_9 => {
                    encoding = Box::new(&ISO_8859_9);
                }
                Charset::KSC5601 => {
                    encoding = Box::new(&KS_C5601);
                }
                Charset::JisX0208 => {
                    encoding = Box::new(&JIS_X0208);
                }
                Charset::GB2312 => {
                    encoding = Box::new(&GB_2312);
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
            let mut replacement_character_added_temp;

            while byte_index < bytes.len() {
                while bytes[byte_index] & 128u8 == 128u8 {
                    flipped_bytes[byte_index] = bytes[byte_index] ^ flip_right_operator;
                    byte_index += 1;
                }

                replacement_character_added_temp = right_encoding.decode(&flipped_bytes[start_index..byte_index], string);
                replacement_character_added = if replacement_character_added_temp { true } else { replacement_character_added };

   
                start_index = byte_index;


                
                while bytes[byte_index] & 128u8 == 0u8 {
                    flipped_bytes[byte_index] = bytes[byte_index] ^ flip_left_operator;
                    byte_index += 1;
                }

                replacement_character_added_temp = left_encoding.decode(&flipped_bytes[start_index..byte_index], string);
                replacement_character_added = if replacement_character_added_temp { true } else { replacement_character_added };

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