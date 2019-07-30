use lazy_static::lazy_static;

use super::*;

const INPUT_BUFFER: [u8; 84] = [
    0x15, 0x00, // inputFlags
    0x00, 0x00, // pad2octetsA
    0x09, 0x04, 0x00, 0x00, // keyboardLayout
    0x04, 0x00, 0x00, 0x00, // keyboardType
    0x00, 0x00, 0x00, 0x00, // keyboardSubType
    0x0c, 0x00, 0x00, 0x00, // keyboardFunctionKey
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, // imeFileName
];

lazy_static! {
    pub static ref INPUT: Input = Input {
        input_flags: InputFlags::SCANCODES | InputFlags::UNICODE | InputFlags::MOUSEX,
        keyboard_layout: 0x409,
        keyboard_type: Some(KeyboardType::IbmEnhanced),
        keyboard_subtype: 0,
        keyboard_function_key: 12,
        keyboard_ime_filename: String::new(),
    };
}

#[test]
fn from_buffer_correctly_parses_input_capset() {
    assert_eq!(*INPUT, Input::from_buffer(INPUT_BUFFER.as_ref()).unwrap());
}

#[test]
fn to_buffer_correctly_serializes_input_capset() {
    let mut buffer = Vec::new();

    INPUT.to_buffer(&mut buffer).unwrap();

    assert_eq!(buffer, INPUT_BUFFER.as_ref());
}

#[test]
fn buffer_length_is_correct_for_input_capset() {
    assert_eq!(INPUT_BUFFER.len(), INPUT.buffer_length());
}
