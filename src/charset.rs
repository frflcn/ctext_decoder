use crate::encoding::Encoding;
use crate::encoding::{GB_2312, JIS_X0201, JIS_X0208, KS_C5601};
use crate::encoding::{ISO_8859_1, ISO_8859_2, ISO_8859_3};
use crate::encoding::{ISO_8859_4, ISO_8859_5, ISO_8859_6};
use crate::encoding::{ISO_8859_7, ISO_8859_8, ISO_8859_9};
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

    fn match_charset(charset: &Charset) -> Box<&dyn Encoding> {
        let encoding: Box<&dyn Encoding>;
        match charset {
            Charset::Ascii => {
                encoding = Box::new(&ISO_8859_1);
            }
            Charset::JisX0201L => {
                encoding = Box::new(&JIS_X0201);
            }
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
        }
        return encoding;
    }
    pub fn get_encoding(&self) -> Box<&dyn Encoding> {
        return Charset::match_charset(&self);
    }
}