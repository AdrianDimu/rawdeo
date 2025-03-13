mod terminal;

use std::io::{self, Read, Write};
use ctrlc;
use terminal::{enable_raw_mode, disable_raw_mode};

fn main() {
    enable_raw_mode().expect("Failed to enable raw mode");

    ctrlc::set_handler(move || {
        disable_raw_mode();
        println!("Restoring terminal settings... Exiting.");
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    println!("Raw mode enabled! Press 'q' to exit.");

    let mut buffer = [0; 1];
    let stdin = io::stdin();

    loop {
        stdin.lock().read_exact(&mut buffer).unwrap();
        let key = buffer[0];

        if key == b'q' {
            break;
        }

        print!("You pressed: {}\r\n", key as char);
        io::stdout().flush().unwrap();
    }

    disable_raw_mode();
    println!("Restoring terminal settings... Exiting.");
}
