use std::io::{self, Write};
use crate::input::Key;

pub struct TextBuffer {
    pub lines: Vec<String>,
    pub cursor_x: usize,
    pub cursor_y: usize,
    render_cache: Vec<String>,
}

impl TextBuffer {
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()],
            cursor_x: 0,
            cursor_y: 0,
            render_cache: vec![String::new()],
        }
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
        } 
    }

    pub fn insert_new_line(&mut self) {
        let current_line = self.lines[self.cursor_y].split_off(self.cursor_x);
        self.cursor_y += 1;
        self.cursor_x = 0;
        self.lines.insert(self.cursor_y, current_line);
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
                }
            }
            Key::ArrowDown => {
                if self.cursor_y < self.lines.len() - 1 {
                    self.cursor_y += 1;
                    self.cursor_x = self.cursor_x.min(self.lines[self.cursor_y].len());
                }
            }
            _ => {}
        }
    }

    pub fn render(&mut self) {
        print!("\x1b[?25l");

        for (i, line) in self.lines.iter().enumerate() {
            if i >= self.render_cache.len() || self.render_cache[i] != *line {
                print!("\x1b[{};1H\x1b[K{}", i + 1, line);
            }
        }

        if self.lines.len() < self.render_cache.len() {
            for i in self.lines.len()..self.render_cache.len() {
                print!("\x1b[{};1H\x1b[K", i + 1);
            }
        }

        self.render_cache = self.lines.clone();

        print!("\x1b[{};{}H", self.cursor_y + 1, self.cursor_x + 1);
        print!("\x1b[?25h");

        io::stdout().flush().unwrap();
    }
}