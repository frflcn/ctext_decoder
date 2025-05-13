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