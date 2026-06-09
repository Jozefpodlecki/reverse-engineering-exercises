use iced_x86::DecoderOptions;

pub fn assert_ice(result: &[u8], expected: iced_x86::Code) {
    let mut decoder = iced_x86::Decoder::new(64, result, DecoderOptions::NONE);
    assert_eq!(decoder.decode().code(), expected);
}