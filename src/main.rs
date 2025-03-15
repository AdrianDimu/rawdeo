mod terminal;
mod input;
mod buffer;

use ctrlc;
use terminal::{enable_raw_mode, disable_raw_mode};
use terminal_size::{Height, Width, terminal_size};
use input::read_key;
use buffer::TextBuffer;

fn main() {
    enable_raw_mode().expect("Failed to enable raw mode");

    ctrlc::set_handler(move || {
        disable_raw_mode();
        println!("\nRestoring terminal settings... Exiting.");
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    print!("\x1b[2J\x1b[H");

    let (_, Height(h)) = terminal_size().unwrap_or((Width(80), Height(24)));
    let mut buffer = TextBuffer::new(h as usize -2);

    println!("Raw mode enabled! Start typing... (Ctrl+C to exit)");

    loop {
        buffer.render();
        let key = read_key();
        buffer.handle_keypress(key);
    }
}
