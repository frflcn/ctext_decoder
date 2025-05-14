const REPLACEMENT_CHARACTER: u32 = 65533;
use std::char;
use crate::charsets;


pub trait Encoding {
    fn decode(&self, bytes: &[u8], string: &mut String) -> bool;
}

pub struct SingleByteEncoding {
    charset: [u32; 256]
}

impl Encoding for SingleByteEncoding {
    fn decode(&self, bytes: &[u8], string: &mut String) -> bool {
        let mut added_replacement_character = false;
        for byte in bytes {
            let next_decoded_byte = self.charset[*byte as usize];
            if next_decoded_byte == REPLACEMENT_CHARACTER {
                added_replacement_character = true;
                string.push(char::REPLACEMENT_CHARACTER);
            }
            else {
                string.push(unsafe {char::from_u32_unchecked(next_decoded_byte)});
            }
        }
        return added_replacement_character;
    }
}



pub struct DoubleByteEncoding {
    charset: [[u32; 94]; 94]
}

impl Encoding for DoubleByteEncoding {
    fn decode(&self, bytes: &[u8], string: &mut String) -> bool {
        let mut added_replacement_character = false;
        let mut byte_index = 0;
        let mut first_byte;
        let mut second_byte;
        let mut next_unicode;
        while byte_index < bytes.len() {
            first_byte = bytes[byte_index] as usize;
            if first_byte < 128 {
                next_unicode = charsets::ISO_8859_1[first_byte];
                string.push(unsafe { char::from_u32_unchecked(next_unicode) });
                byte_index += 1;
            }
            else if first_byte >= 255 || first_byte <= 160 {
                byte_index += 2;
                string.push(char::REPLACEMENT_CHARACTER);
                added_replacement_character = true;
            }
            else {
                byte_index += 1;
                if byte_index >= bytes.len() {
                    string.push(char::REPLACEMENT_CHARACTER);
                    added_replacement_character = true;
                    break;
                }
                second_byte = bytes[byte_index] as usize;
                next_unicode = self.charset[first_byte][second_byte];
                if next_unicode == REPLACEMENT_CHARACTER {
                    added_replacement_character = true;
                }
                string.push(unsafe {char::from_u32_unchecked(next_unicode)});
                byte_index += 1;
            }
        }
        
        return added_replacement_character;
    }
}

pub struct UTF8Encoding;

impl Encoding for UTF8Encoding {
    fn decode(&self, bytes: &[u8], string: &mut String) -> bool {
        let decoded_string = String::from_utf8_lossy(bytes);
        let added_replacement_character = decoded_string.contains(char::REPLACEMENT_CHARACTER);
        string.push_str(&decoded_string);
        return added_replacement_character;
    }
}

pub static UTF8: UTF8Encoding = UTF8Encoding;

pub static GB_2312: DoubleByteEncoding = DoubleByteEncoding {
    charset: charsets::GB_2312,
};

pub static KS_C5601: DoubleByteEncoding = DoubleByteEncoding {
    charset: charsets::KS_C5601,
};

pub static JIS_X0208: DoubleByteEncoding = DoubleByteEncoding {
    charset: charsets::JIS_X0208,
};

pub static JIS_X0201: SingleByteEncoding = SingleByteEncoding {
    charset: charsets::JIS_X0201,
};

pub static ISO_8859_1: SingleByteEncoding = SingleByteEncoding {
    charset: charsets::ISO_8859_1,
};

pub static ISO_8859_2: SingleByteEncoding = SingleByteEncoding {
    charset: charsets::ISO_8859_2,
};

pub static ISO_8859_3: SingleByteEncoding = SingleByteEncoding {
    charset: charsets::ISO_8859_3,
};

pub static ISO_8859_4: SingleByteEncoding = SingleByteEncoding {
    charset: charsets::ISO_8859_4,
};

pub static ISO_8859_5: SingleByteEncoding = SingleByteEncoding {
    charset: charsets::ISO_8859_5,
};

pub static ISO_8859_6: SingleByteEncoding = SingleByteEncoding {
    charset: charsets::ISO_8859_6,
};

pub static ISO_8859_7: SingleByteEncoding = SingleByteEncoding {
    charset: charsets::ISO_8859_7,
};

pub static ISO_8859_8: SingleByteEncoding = SingleByteEncoding {
    charset: charsets::ISO_8859_8,
};

pub static ISO_8859_9: SingleByteEncoding = SingleByteEncoding {
    charset: charsets::ISO_8859_9,
};
