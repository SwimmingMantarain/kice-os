use crate::{print, Color};
use pc_keyboard::{layouts, DecodedKey, HandleControl, KeyCode, Keyboard, ScancodeSet1};


static mut KEYBOARD: Keyboard<layouts::Us104Key, ScancodeSet1> = Keyboard::new(
    ScancodeSet1::new(),
    layouts::Us104Key,
    HandleControl::Ignore,
);

pub fn handle_scancode(scancode: u8) {
    let keyboard = unsafe { &mut KEYBOARD };

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => {
                    if character.is_control() {
                        match character {
                            '\x08' | '\t' | '\n' => {
                                print!(Color::LightGreen, Color::Black, "{}", character);
                            }

                            _ => {
                                // Ignore all other control characters
                            },
                        }
                    } else {
                        print!(Color::LightGreen, Color::Black, "{}", character);
                    }
                }
                DecodedKey::RawKey(key) => {
                    // Instead of printing every raw key (like LeftArrow, F1, etc.),
                    // only handle the exceptions: Backspace, Tab, and Return.
                    match key {
                        KeyCode::Backspace | KeyCode::Tab | KeyCode::Return => {
                            // Map the raw key to the corresponding control character.
                            let character = match key {
                                KeyCode::Backspace => '\x08',
                                KeyCode::Tab => '\t',
                                KeyCode::Return => '\r',
                                _ => unreachable!(),
                            };
                            print!(Color::LightGreen, Color::Black, "{}", character);
                        }
                        _ => {
                            // Ignore other raw keys
                        }
                    }
                }
            }
        }
    }
}
