use std::io;
use std::os::unix::io::AsRawFd;
use libc::{tcgetattr, tcsetattr, termios, TCSAFLUSH, ECHO, ICANON};

static mut ORIGINAL_TERMIOS: Option<termios> = None;

pub fn enable_raw_mode() -> io::Result<()> {
    let stdin_fd = io::stdin().as_raw_fd();

    let original_termios: termios = unsafe {
        let mut termios = std::mem::zeroed();
        if tcgetattr(stdin_fd, &mut termios) !=0 {
            return Err(io::Error::last_os_error());
        }
        termios
    };

    unsafe {
        ORIGINAL_TERMIOS = Some(original_termios);
    }
    
    let mut raw_termios = original_termios;
    raw_termios.c_lflag &= !(ECHO | ICANON );

    if unsafe { tcsetattr(stdin_fd, TCSAFLUSH, &raw_termios) } != 0 {
        return Err(io::Error::last_os_error());
    }

    Ok(())
}

pub fn disable_raw_mode() {
    let stdin_fd = io::stdin().as_raw_fd();

    unsafe {
        if let Some(original) = ORIGINAL_TERMIOS {
            tcsetattr(stdin_fd, TCSAFLUSH, &original);
        }
    }
}
