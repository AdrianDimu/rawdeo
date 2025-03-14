use std::io::{self, Write};
use crate::input::Key;

pub struct TextBuffer {
    pub lines: Vec<String>,
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub scroll_y: usize,
    pub screen_height: usize,
    render_cache: Vec<String>,
}

impl TextBuffer {
    pub fn new(screen_height: usize) -> Self {
        Self {
            lines: vec![String::new()],
            cursor_x: 0,
            cursor_y: 0,
            scroll_y: 0,
            screen_height,
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

            if self.cursor_y < self.scroll_y {
                self.scroll_y = self.cursor_y;
            }
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

        for (i, line_index) in (self.scroll_y..self.scroll_y + self.screen_height)
            .enumerate()
            .take(self.lines.len() - self.scroll_y) 
        {
            let line = &self.lines[line_index];

            print!("\x1b[{};1H\x1b[K{:>width$} | {}", i + 1, line_index + 1, line, width = new_max_digits);
           
        }

        if self.lines.len() < self.render_cache.len() {
            for i in self.lines.len()..self.render_cache.len() {
                print!("\x1b[{};1H\x1b[K", i + 1);
            }
        }

        if self.lines.len() < self.render_cache.len() {
            for i in self.lines.len()..self.render_cache.len() {
                print!("\x1b[{};1H\x1b[K", i + 1);
            }
        }

        self.render_cache = self.lines.clone();

        let cursor_offset = new_max_digits + 3;
        let cursor_screen_y = self.cursor_y - self.scroll_y;
        print!("\x1b[{};{}H", cursor_screen_y + 1, self.cursor_x + cursor_offset + 1);
        print!("\x1b[?25h");

        io::stdout().flush().unwrap();
    }
}