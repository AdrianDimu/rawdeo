use std::io::{self, Read};

pub enum Key {
    Char(char),
    Tab,
    Escape,
    Space,
    Enter,
    Backspace,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    OptionSpace,
    Unknown,
}

pub fn read_key() -> Key {
    let mut buffer = [0; 1];
    let stdin = io::stdin();

    if stdin.lock().read_exact(&mut buffer).is_err() {
        return Key::Unknown;
    }

    match buffer[0] {
        b'\t' => Key::Tab,
        b' ' => Key::Space,
        b'\n' => Key::Enter,
        b'\x7f' => Key::Backspace,
        b'\x1b' => {
            let mut seq = [0, 2];
            if stdin.lock().read_exact(&mut seq[0..1]).is_ok() {
                if seq[0] == b'[' {
                    if stdin.lock().read_exact(&mut seq[1..2]).is_ok() {
                        return match seq {
                            [b'[', b'A'] => Key::ArrowUp,
                            [b'[', b'B'] => Key::ArrowDown,
                            [b'[', b'C'] => Key::ArrowRight,
                            [b'[', b'D'] => Key::ArrowLeft,
                            _ => Key::Escape,
                        }; 
                    }
                }
                Key::Escape
            } else {
                Key::Escape
            }
        }
        b'\xC2' => {
            let mut seq = [0; 1];
            if stdin.lock().read_exact(&mut seq).is_ok() && seq[0] == b'\xA0' {
               return Key::OptionSpace
            }
            Key::Unknown
        }
        32..=126 => Key::Char(buffer[0] as char),
        _=> Key::Unknown,
    }
}
