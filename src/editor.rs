use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode},
    style::{Color, Print, SetForegroundColor},
};
use std::{
    io::{self, Write},
    time::Duration,
};

use crate::rope::Rope;

/// Represents the current mode of the editor
#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Insert,
    Macro,
    Command(String), // Holds the current command buffer
}

/// The main editor struct that manages the UI and state
pub struct Editor {
    /// The text content
    rope: Rope,
    /// Current cursor position (x, y)
    cursor: (u16, u16),
    /// Current mode
    mode: Mode,
    /// Terminal size
    size: (u16, u16),
    /// Whether the editor is running
    running: bool,
}

impl Editor {
    /// Creates a new editor instance
    pub fn new() -> io::Result<Self> {
        let size = crossterm::terminal::size()?;
        Ok(Self {
            rope: Rope::new(""),
            cursor: (0, 0),
            mode: Mode::Insert,
            size,
            running: true,
        })
    }

    /// Runs the editor
    pub fn run(&mut self) -> io::Result<()> {
        // Set up terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;

        // Main event loop
        while self.running {
            self.draw(&mut stdout)?;

            // Poll for events with a timeout
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    self.handle_key_event(key)?;
                }
            }
        }

        // Clean up terminal
        execute!(stdout, LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    /// Handles key events
    fn handle_key_event(&mut self, key: KeyEvent) -> io::Result<()> {
        // Handle Control + Space in any mode
        if key.code == KeyCode::Char(' ') && key.modifiers.contains(KeyModifiers::CONTROL) {
            self.mode = Mode::Macro;
            return Ok(());
        }

        match self.mode.clone() {
            Mode::Insert => self.handle_insert_mode(key),
            Mode::Macro => self.handle_macro_mode(key),
            Mode::Command(cmd_buffer) => {
                let mut new_buffer = cmd_buffer;
                match key.code {
                    KeyCode::Enter => {
                        // Process command
                        if new_buffer == "q" {
                            self.running = false;
                        }
                        // Clear command buffer
                        self.mode = Mode::Command(String::new());
                    }
                    KeyCode::Char(c) if key.modifiers.is_empty() => {
                        new_buffer.push(c);
                        self.mode = Mode::Command(new_buffer);
                    }
                    KeyCode::Backspace => {
                        new_buffer.pop();
                        self.mode = Mode::Command(new_buffer);
                    }
                    _ => {}
                }
                Ok(())
            }
        }
    }

    /// Handles key events in Insert mode
    fn handle_insert_mode(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.code {
            KeyCode::Char(c) => {
                // Insert character at cursor position
                let pos = self.get_absolute_position();
                self.rope.insert(pos, &c.to_string());
                self.cursor.0 += 1;
            }
            KeyCode::Tab => {
                // Insert actual tab character
                let pos = self.get_absolute_position();
                self.rope.insert(pos, "\t");
                self.cursor.0 += 4; // Move cursor by tab width
            }
            KeyCode::Enter => {
                // Insert newline at cursor position
                let pos = self.get_absolute_position();
                self.rope.insert(pos, "\n");
                self.cursor.0 = 0;
                self.cursor.1 += 1;
            }
            KeyCode::Backspace => {
                let pos = self.get_absolute_position();
                if pos > 0 {
                    // Get the character at the position before cursor
                    let text = self.rope.to_string();
                    let prev_char = text.chars().nth(pos - 1);
                    
                    // Remove the character
                    self.rope.remove(pos - 1, pos);

                    // Update cursor position
                    if prev_char == Some('\n') {
                        // If we deleted a newline, move cursor to end of previous line
                        if self.cursor.1 > 0 {
                            self.cursor.1 -= 1;
                            let line = self.get_line(self.cursor.1);
                            self.cursor.0 = self.get_visual_line_length(&line);
                        }
                    } else if prev_char == Some('\t') {
                        self.cursor.0 = self.cursor.0.saturating_sub(4);
                    } else if self.cursor.0 > 0 {
                        self.cursor.0 -= 1;
                    }
                }
            }
            KeyCode::Left => {
                if self.cursor.0 > 0 {
                    // Check if we're moving past a tab
                    let line = self.get_line(self.cursor.1);
                    let visual_pos = self.cursor.0 as usize;
                    let actual_pos = self.get_actual_position_from_visual(&line, visual_pos);
                    if actual_pos > 0 && line.chars().nth(actual_pos - 1) == Some('\t') {
                        self.cursor.0 = self.cursor.0.saturating_sub(4);
                    } else {
                        self.cursor.0 -= 1;
                    }
                } else if self.cursor.1 > 0 {
                    // Move to end of previous line
                    self.cursor.1 -= 1;
                    let line = self.get_line(self.cursor.1);
                    self.cursor.0 = self.get_visual_line_length(&line);
                }
            }
            KeyCode::Right => {
                let line = self.get_line(self.cursor.1);
                let visual_length = self.get_visual_line_length(&line);
                if self.cursor.0 < visual_length {
                    // Check if we're moving past a tab
                    let visual_pos = self.cursor.0 as usize;
                    let actual_pos = self.get_actual_position_from_visual(&line, visual_pos);
                    if actual_pos < line.len() && line.chars().nth(actual_pos) == Some('\t') {
                        self.cursor.0 += 4;
                    } else {
                        self.cursor.0 += 1;
                    }
                } else {
                    // Move to start of next line if it exists
                    let text = self.rope.to_string();
                    let lines: Vec<&str> = text.lines().collect();
                    if self.cursor.1 + 1 < lines.len() as u16 {
                        self.cursor.0 = 0;
                        self.cursor.1 += 1;
                    }
                }
            }
            KeyCode::Up => {
                if self.cursor.1 > 0 {
                    self.cursor.1 -= 1;
                    // Adjust x position if current line is shorter
                    let line = self.get_line(self.cursor.1);
                    let visual_length = self.get_visual_line_length(&line);
                    self.cursor.0 = self.cursor.0.min(visual_length);
                }
            }
            KeyCode::Down => {
                let text = self.rope.to_string();
                let lines: Vec<&str> = text.lines().collect();
                if self.cursor.1 + 1 < lines.len() as u16 {
                    self.cursor.1 += 1;
                    // Adjust x position if current line is shorter
                    let line = self.get_line(self.cursor.1);
                    let visual_length = self.get_visual_line_length(&line);
                    self.cursor.0 = self.cursor.0.min(visual_length);
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Gets the absolute position in the text based on cursor coordinates
    fn get_absolute_position(&self) -> usize {
        let text = self.rope.to_string();
        let mut pos = 0;
        let mut current_line = 0;

        for c in text.chars() {
            if current_line == self.cursor.1 {
                if pos >= text.len() || (pos - self.get_line_start_position(current_line)) as u16 == self.cursor.0 {
                    break;
                }
            }
            pos += 1;
            if c == '\n' {
                current_line += 1;
            }
        }
        pos
    }

    /// Gets the start position of a line in absolute position
    fn get_line_start_position(&self, line_number: u16) -> usize {
        let text = self.rope.to_string();
        let mut pos = 0;
        let mut current_line = 0;

        for c in text.chars() {
            if current_line == line_number {
                break;
            }
            pos += 1;
            if c == '\n' {
                current_line += 1;
            }
        }
        pos
    }

    /// Gets the content of a specific line
    fn get_line(&self, line_number: u16) -> String {
        let text = self.rope.to_string();
        text.lines().nth(line_number as usize).unwrap_or("").to_string()
    }

    /// Handles key events in Macro mode
    fn handle_macro_mode(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.code {
            KeyCode::Char('i') => {
                self.mode = Mode::Insert;
            }
            KeyCode::Char('`') => {
                self.mode = Mode::Command(String::new());
            }
            _ => {}
        }
        Ok(())
    }

    /// Gets the visual length of a line (counting tabs as 4 spaces)
    fn get_visual_line_length(&self, line: &str) -> u16 {
        let mut length = 0;
        for c in line.chars() {
            length += if c == '\t' { 4 } else { 1 };
        }
        length
    }

    /// Converts a visual position to an actual position in the string
    fn get_actual_position_from_visual(&self, line: &str, visual_pos: usize) -> usize {
        let mut actual_pos = 0;
        let mut visual_current = 0;
        
        for (i, c) in line.chars().enumerate() {
            if visual_current >= visual_pos {
                break;
            }
            visual_current += if c == '\t' { 4 } else { 1 };
            actual_pos = i + 1;
        }
        
        actual_pos
    }

    /// Draws the editor UI
    fn draw(&self, stdout: &mut io::Stdout) -> io::Result<()> {
        // Clear screen
        execute!(stdout, Clear(ClearType::All))?;

        // Calculate margin width for line numbers (minimum 2 spaces)
        let text = self.rope.to_string();
        let lines: Vec<&str> = text.lines().collect();
        let total_lines = lines.len();
        let margin_width = (total_lines + 1).to_string().len().max(2);

        // Draw status line
        let mode_str = match &self.mode {
            Mode::Insert => "INSERT",
            Mode::Macro => "MACRO",
            Mode::Command(cmd) => if cmd.is_empty() { "COMMAND" } else { cmd },
        };
        execute!(
            stdout,
            MoveTo(0, 0),
            SetForegroundColor(Color::White),
            Print(format!(" {} | {}:{}", mode_str, self.cursor.0, self.cursor.1))
        )?;

        // Draw text content with line numbers
        let mut y = 1;
        let content_start_x = (margin_width + 1) as u16; // +1 for space after number

        // Draw existing lines
        for (i, line) in text.lines().enumerate() {
            if y >= self.size.1 {
                break;
            }
            // Draw line number
            execute!(
                stdout,
                MoveTo(0, y),
                SetForegroundColor(Color::DarkGrey),
                Print(format!("{:>width$} ", i + 1, width = margin_width))
            )?;

            // Draw line content
            let mut x = content_start_x;
            execute!(stdout, SetForegroundColor(Color::White))?;
            for c in line.chars() {
                if x >= self.size.0 {
                    break;
                }
                if c == '\t' {
                    // Draw tab as 4 spaces
                    execute!(stdout, MoveTo(x, y), Print("    "))?;
                    x += 4;
                } else {
                    execute!(stdout, MoveTo(x, y), Print(c))?;
                    x += 1;
                }
            }
            y += 1;
        }

        // Draw ~ for empty lines
        while y < self.size.1 {
            execute!(
                stdout,
                MoveTo(0, y),
                SetForegroundColor(Color::DarkGrey),
                Print(format!("{:>width$} ~", "", width = margin_width))
            )?;
            y += 1;
        }

        // Position cursor (adjusted for margin)
        execute!(
            stdout,
            MoveTo(content_start_x + self.cursor.0, self.cursor.1 + 1)
        )?;

        stdout.flush()?;
        Ok(())
    }
} 