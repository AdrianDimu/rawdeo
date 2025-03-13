mod terminal;
mod input;

use ctrlc;
use terminal::{enable_raw_mode, disable_raw_mode};
use input::{read_key, handle_keypress, Key};

fn main() {
    enable_raw_mode().expect("Failed to enable raw mode");

    ctrlc::set_handler(move || {
        disable_raw_mode();
        println!("Restoring terminal settings... Exiting.");
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    println!("Raw mode enabled! Press 'q' to exit.");

    loop {
        let key = read_key();
        if let Key::Quit = key {
            break;
        }
        handle_keypress(key);
    }

    disable_raw_mode();
    println!("Restoring terminal settings... Exiting.");
}
