use super::GB_2312;
use super::KS_C5601;
use super::JIS_X0208;
use super::JIS_X0201;
use super::{ISO_8859_1, ISO_8859_2, ISO_8859_3, ISO_8859_4, ISO_8859_5, ISO_8859_6, ISO_8859_7, ISO_8859_8, ISO_8859_9};

const SINGLE_BYTE_CHARSETS: [[u32; 256]; 10] = [JIS_X0201, ISO_8859_1, ISO_8859_2, ISO_8859_3, ISO_8859_4, ISO_8859_5, ISO_8859_6, ISO_8859_7, ISO_8859_8, ISO_8859_9];
const DOUBLE_BYTE_CHARSETS: [[[u32; 94]; 94]; 3] = [JIS_X0208, GB_2312, KS_C5601];

#[test]
fn test_gbk() {
    assert_eq!(GB_2312[15][64], 25644)
}

#[test]
fn test_ks() {
    assert_eq!(KS_C5601[9][23], 12376)
}

#[test]
fn test_jis_x0208() {
    assert_eq!(JIS_X0208[20][8], 31232)
}

//NOTE: Uses full width instead of half width characters
#[test]
fn test_jis_x0201() {
    assert_eq!(JIS_X0201[92], 165);
    assert_eq!(JIS_X0201[126], 8254);
    assert_eq!(JIS_X0201[170], 12455);
}

#[test]
fn test_iso_8859_1() {
    assert_eq!(ISO_8859_1[32], 32);
    assert_eq!(ISO_8859_1[89], 89);
    assert_eq!(ISO_8859_1[111], 111);
    assert_eq!(ISO_8859_1[92], 92);
    assert_eq!(ISO_8859_1[126], 126);
    assert_eq!(ISO_8859_1[179], 179);
    assert_eq!(ISO_8859_1[234], 234);
    assert_eq!(ISO_8859_1[187], 187);
}

#[test]
fn test_iso_8859_2() {
    assert_eq!(ISO_8859_2[32], 32);
    assert_eq!(ISO_8859_2[89], 89);
    assert_eq!(ISO_8859_2[111], 111);
    assert_eq!(ISO_8859_2[92], 92);
    assert_eq!(ISO_8859_2[126], 126);
    assert_eq!(ISO_8859_2[179], 322);
    assert_eq!(ISO_8859_2[234], 281);
}

#[test]
fn test_iso_8859_3() {
    assert_eq!(ISO_8859_3[32], 32);
    assert_eq!(ISO_8859_3[89], 89);
    assert_eq!(ISO_8859_3[111], 111);
    assert_eq!(ISO_8859_3[92], 92);
    assert_eq!(ISO_8859_3[126], 126);
    assert_eq!(ISO_8859_3[179], 179);
    assert_eq!(ISO_8859_3[234], 234);
    assert_eq!(ISO_8859_3[187], 287);
}

#[test]
fn test_iso_8859_4() {
    assert_eq!(ISO_8859_4[32], 32);
    assert_eq!(ISO_8859_4[89], 89);
    assert_eq!(ISO_8859_4[111], 111);
    assert_eq!(ISO_8859_4[92], 92);
    assert_eq!(ISO_8859_4[126], 126);
    assert_eq!(ISO_8859_4[179], 343);
    assert_eq!(ISO_8859_4[234], 281);
}

#[test]
fn test_iso_8859_5() {
    assert_eq!(ISO_8859_5[32], 32);
    assert_eq!(ISO_8859_5[89], 89);
    assert_eq!(ISO_8859_5[111], 111);
    assert_eq!(ISO_8859_5[92], 92);
    assert_eq!(ISO_8859_5[126], 126);
    assert_eq!(ISO_8859_5[179], 1043);
    assert_eq!(ISO_8859_5[234], 1098);
}

#[test]
fn test_iso_8859_6() {
    assert_eq!(ISO_8859_6[32], 32);
    assert_eq!(ISO_8859_6[89], 89);
    assert_eq!(ISO_8859_6[111], 111);
    assert_eq!(ISO_8859_6[92], 92);
    assert_eq!(ISO_8859_6[126], 126);
    assert_eq!(ISO_8859_6[179], 65533);
    assert_eq!(ISO_8859_6[234], 1610);
    assert_eq!(ISO_8859_6[198], 1574);
}

#[test]
fn test_iso_8859_7() {
    assert_eq!(ISO_8859_7[32], 32);
    assert_eq!(ISO_8859_7[89], 89);
    assert_eq!(ISO_8859_7[111], 111);
    assert_eq!(ISO_8859_7[92], 92);
    assert_eq!(ISO_8859_7[126], 126);
    assert_eq!(ISO_8859_7[179], 179);
    assert_eq!(ISO_8859_7[234], 954);
    assert_eq!(ISO_8859_7[198], 918);
}

#[test]
fn test_iso_8859_8() {
    assert_eq!(ISO_8859_8[32], 32);
    assert_eq!(ISO_8859_8[89], 89);
    assert_eq!(ISO_8859_8[111], 111);
    assert_eq!(ISO_8859_8[92], 92);
    assert_eq!(ISO_8859_8[126], 126);
    assert_eq!(ISO_8859_8[179], 179);
    assert_eq!(ISO_8859_8[234], 1498);
    assert_eq!(ISO_8859_8[198], 65533);
}


// Discrepency between wikipedia and https://www.rfc-editor.org/rfc/rfc1345.txt at codepoints 234, 236, 239, 175
#[test]
fn test_iso_8859_9() {
    assert_eq!(ISO_8859_9[32], 32);
    assert_eq!(ISO_8859_9[89], 89);
    assert_eq!(ISO_8859_9[111], 111);
    assert_eq!(ISO_8859_9[92], 92);
    assert_eq!(ISO_8859_9[126], 126);
    assert_eq!(ISO_8859_9[179], 179);
    //assert_eq!(ISO_8859_9[234], 234); 
    assert_eq!(ISO_8859_9[198], 198);
    assert_eq!(ISO_8859_9[221], 304);
}

#[test]
fn test_from_u32_unchecked_is_safe() {
    for charset in SINGLE_BYTE_CHARSETS {
        for chr in charset {
            unsafe { let _ = std::char::from_u32_unchecked(chr); }
        }
    }
    for charset in DOUBLE_BYTE_CHARSETS {
        for page in charset {
            for chr in page {
                unsafe { let _ = std::char::from_u32_unchecked(chr); }
            }
        }
    }
}