mod terminal;
mod input;
mod buffer;

use ctrlc;
use terminal::{enable_raw_mode, disable_raw_mode};
use input::{read_key, Key};
use buffer::TextBuffer;

fn main() {
    enable_raw_mode().expect("Failed to enable raw mode");

    ctrlc::set_handler(move || {
        disable_raw_mode();
        println!("\nRestoring terminal settings... Exiting.");
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    print!("\x1b[2J\x1b[H");

    let mut buffer = TextBuffer::new();
    println!("Raw mode enabled! Start typing... (Ctrl+C to exit)");

    loop {
        buffer.render();
        let key = read_key();
        match key {
            Key::Char(c) => buffer.insert_char(c),
            Key::Space => buffer.insert_char(' '),
            Key::Tab => buffer.insert_char('\t'),
            Key::Enter => buffer.insert_new_line(),
            Key::Backspace => buffer.delete_char(),
            Key::ArrowLeft | Key::ArrowRight | Key::ArrowUp | Key::ArrowDown => buffer.move_cursor(key), 
            _ => {}
        }
    }
}
