use std::io::{self, Write};
use crate::{input::Key, terminal::disable_raw_mode};

pub struct TextBuffer {
    pub lines: Vec<String>,
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub scroll_y: usize,
    pub screen_height: usize,
    pub mode: Mode,
    pub command_input: String,
    render_cache: Vec<String>,
}

pub enum Mode {
    Normal,
    Insert,
    Command,
}

impl TextBuffer {
    pub fn new(screen_height: usize) -> Self {
        Self {
            lines: vec![String::new()],
            cursor_x: 0,
            cursor_y: 0,
            scroll_y: 0,
            screen_height: screen_height -2,
            mode: Mode::Normal,
            command_input: String::new(),
            render_cache: vec![String::new()],
        }
    }

    pub fn handle_keypress(&mut self, key: Key) {
        match self.mode {
            Mode::Insert => self.handle_insert_mode(key),
            Mode::Normal => self.handle_normal_mode(key),
            Mode::Command => self.handle_command_mode(key),
        }
    }

    fn handle_insert_mode(&mut self, key: Key) {
        match key {
            Key::Char(c) => self.insert_char(c),
            Key::Space => self.insert_char(' '),
            Key::Tab => self.insert_char('\t'),
            Key::Enter => self.insert_new_line(),
            Key::Backspace => self.delete_char(),
            Key::ArrowLeft | Key::ArrowRight | Key::ArrowUp | Key::ArrowDown => self.move_cursor(key),
            Key::OptionSpace => self.mode = Mode::Normal,
            _ => {}
        }
    }

    fn handle_normal_mode(&mut self, key: Key) {
        match key {
            Key::Char('i') => self.mode = Mode::Insert,
            Key::Char(':') => {
                self.mode = Mode::Command;
                self.command_input.clear();
            }
            Key::OptionSpace => self.mode = Mode::Insert,
            Key::ArrowLeft | Key::ArrowRight | Key::ArrowUp | Key::ArrowDown => self.move_cursor(key),
            Key::Space => self.insert_char(' '),
            Key::Tab => self.insert_char('\t'),
            Key::Enter => self.insert_new_line(),
            Key::Backspace => self.delete_char(),
            _ => {}
        }
    }

    fn handle_command_mode(&mut self, key: Key) {
        match key {
            Key::Char(c) => self.command_input.push(c),
            Key::Backspace => {
                self.command_input.pop();
            }
            Key::Enter => {
                self.execute_command();
            }
            Key::OptionSpace => self.mode = Mode::Normal,
            _ => {}
        }
    }

    fn execute_command(&mut self) {
        print!("\x1b[2;1H\x1b[K");
        println!("executed: {}", self.command_input);
        io::stdout().flush().unwrap();

        match self.command_input.as_str() {
            "q!" => {
                print!("\x1b[2J\x1b[H");
                disable_raw_mode();
                std::process::exit(0);
            }
            _ => {}
        }

        self.command_input.clear();
        self.mode = Mode::Normal;
    }

    pub fn insert_char(&mut self, c: char) {
        if c == '\t' {
            for _ in 0..4 {
                self.lines[self.cursor_y].insert(self.cursor_x, ' ');
                self.cursor_x += 1;
            }
        } else if c == ' ' || c.is_ascii_graphic() {
            if self.cursor_x > self.lines[self.cursor_y].len() {
                self.cursor_x = self.lines[self.cursor_y].len();
            }
            self.lines[self.cursor_y].insert(self.cursor_x, c);
            self.cursor_x += 1;
        }
    }

    pub fn delete_char(&mut self) {
        if self.cursor_x > 0 {
            self.lines[self.cursor_y].remove(self.cursor_x -1);
            self.cursor_x -= 1;
        } else if self.cursor_y > 0 {
            let prev_line = self.lines.remove(self.cursor_y);
            self.cursor_y -= 1;
            self.cursor_x = self.lines[self.cursor_y].len();
            self.lines[self.cursor_y].push_str(&prev_line);

            if self.cursor_y < self.scroll_y {
                self.scroll_y = self.cursor_y;
            }
        } 

        self.cursor_x = self.cursor_x.min(self.lines[self.cursor_y].len());

        if self.cursor_y == 0 {
            self.scroll_y = 0;
        }
    }

    pub fn insert_new_line(&mut self) {
        let current_line = self.lines[self.cursor_y].split_off(self.cursor_x);
        self.cursor_y += 1;
        self.cursor_x = 0;
        self.lines.insert(self.cursor_y, current_line);

        if self.cursor_y >= self.scroll_y + self.screen_height {
            self.scroll_y += 1;
        }
    }

    pub fn move_cursor(&mut self, direction: Key) {
        match direction {
            Key::ArrowLeft => {
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                } else if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                    self.cursor_x = self.lines[self.cursor_y].len();
                }
            }
            Key::ArrowRight => {
                if self.cursor_x < self.lines[self.cursor_y].len() {
                    self.cursor_x += 1;
                } else if self.cursor_y < self.lines.len() - 1 {
                    self.cursor_y += 1;
                    self.cursor_x = 0;
                }
            }
            Key::ArrowUp => {
                if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                    self.cursor_x = self.cursor_x.min(self.lines[self.cursor_y].len());
                    if self.cursor_y < self.scroll_y {
                        self.scroll_y -= 1;
                    }
                }
            }
            Key::ArrowDown => {
                if self.cursor_y < self.lines.len() - 1 {
                    self.cursor_y += 1;
                    self.cursor_x = self.cursor_x.min(self.lines[self.cursor_y].len());
                    if self.cursor_y >= self.scroll_y + self.screen_height {
                        self.scroll_y += 1;
                    }
                }
            }
            _ => {}
        }
    }

    pub fn render(&mut self) {
        print!("\x1b[?25l");

        let new_max_digits = self.lines.len().to_string().len();
        let old_max_digits = self.render_cache.len().to_string().len();

        if new_max_digits != old_max_digits {
            print!("\x1b[2J\x1b[H");
        }

        let mode_display = match self.mode {
            Mode::Normal => "-- NORMAL --",
            Mode::Insert => "-- INSERT --",
            Mode::Command => "-- COMMAND --",
        };
        print!("\x1b[1;1H\x1b[K{}", mode_display);

        print!("\x1b[2;1H\x1b[K:{}", self.command_input);

        let mut last_rendered_line = 0;
        for (i, line_index) in (self.scroll_y..self.scroll_y + self.screen_height)
            .enumerate()
            .take(self.lines.len() - self.scroll_y) 
        {
            let line = &self.lines[line_index];

            print!("\x1b[{};1H\x1b[K{:>width$} | {}", i + 3, line_index + 1, line, width = new_max_digits);
            last_rendered_line = i + 3;
           
        }

        for i in last_rendered_line + 1..self.screen_height + 3 {
            print!("\x1b[{};1H\x1b[K", i);
        }

        self.render_cache = self.lines.clone();

        let cursor_offset = new_max_digits + 3;
        let cursor_screen_y = self.cursor_y.saturating_sub(self.scroll_y) + 2;
        print!("\x1b[{};{}H", cursor_screen_y + 1, self.cursor_x + cursor_offset + 1);
        print!("\x1b[?25h");

        io::stdout().flush().unwrap();
    }
}