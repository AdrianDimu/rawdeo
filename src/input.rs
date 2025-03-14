use std::io::{self, Read, Write};

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
            let mut seq = [0; 2];
            if stdin.lock().read_exact(&mut seq).is_ok() {
                match seq {
                    [b'[', b'A'] => Key::ArrowUp,
                    [b'[', b'B'] => Key::ArrowDown,
                    [b'[', b'C'] => Key::ArrowRight,
                    [b'[', b'D'] => Key::ArrowLeft,
                    _ => Key::Escape,
                }
            } else {
                Key::Escape
            }
        }
        32..=126 => Key::Char(buffer[0] as char),
        _=> Key::Unknown,
    }
}

pub fn handle_keypress(key: Key) {
    match key {
        Key::ArrowUp => println!("Arrow Up"),
        Key::ArrowDown => println!("Arrow Down"),
        Key::ArrowRight => println!("Arrow Right"),
        Key::ArrowLeft => println!("Arrow Left"),
        Key::Enter => println!("Enter"),
        Key::Backspace => println!("Backspace"),
        Key::Tab => println!("Tab"),
        Key::Escape => println!("Escape"),
        Key::Space => println!("Space"),
        Key::Char(c) => print!("You pressed: {}\r\n", c),
        _ => {}
    }
    io::stdout().flush().unwrap();
}