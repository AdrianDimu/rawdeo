use std::io::{self, Write};

pub struct TextBuffer {
    pub lines: Vec<String>,
    pub cursor_x: usize,
    pub cursor_y: usize,
}

impl TextBuffer {
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()],
            cursor_x: 0,
            cursor_y: 0,
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

    pub fn render(&self) {
        print!("\x1b[2J\x1b[H");
        for line in &self.lines {
            println!("{}", line);
        }
        print!("\x1b[{};{}H", self.cursor_y + 1, self.cursor_x + 1);
        io::stdout().flush().unwrap();
    }
}