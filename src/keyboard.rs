use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use crate::Color;

static mut KEYBOARD: Keyboard<layouts::Us104Key, ScancodeSet1> = 
    Keyboard::new(ScancodeSet1::new(), layouts::Us104Key, HandleControl::Ignore);

pub fn handle_scancode(scancode: u8) {
    let keyboard = unsafe { &mut KEYBOARD };

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!(Color::LightGreen, Color::Black, "{}", character),
                DecodedKey::RawKey(key) => print!(Color::LightGreen, Color::Black, "{:?}", key),
            }
        }
    }
}
