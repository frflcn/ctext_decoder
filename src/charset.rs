use encoding_rs as enc;
use encoding_rs::Encoding;
#[derive(PartialEq)]
pub enum Charset {
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

    pub fn is_non_ascii_yet_ascii_compatible(&self) -> bool {
        match self {
            Charset::Ascii | Charset::JisX0201L | Charset::JisX0201R => {
                return false;
            }
            _ => {
                return true;
            }
        }
    }

    pub fn is_natural_right(&self) -> bool {
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
    pub fn get_encoding(&self) -> &'static Encoding {
        return Charset::match_charset(&self);
    }
}