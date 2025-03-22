use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode},
    style::{Color, Print, SetForegroundColor, SetBackgroundColor, ResetColor, SetAttribute, Attribute},
};
use std::{
    io::{self, Write},
    time::{Duration, Instant},
};

use crate::rope::Rope;

/// Represents an operation that can be undone
#[derive(Debug, Clone)]
enum Operation {
    Insert {
        position: usize,
        text: String,
    }, 
    Remove {
        position: usize,
        text: String,
    },
}

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
    /// Operation history for undo
    history: Vec<Operation>,
    /// Redo stack for redoing undone operations
    redo_stack: Vec<Operation>,
    /// Scroll offset (how many lines we've scrolled down)
    scroll_offset: u16,
    /// Selection start position (absolute position in text)
    selection_start: Option<usize>,
    /// Selection end position (absolute position in text)
    selection_end: Option<usize>,
    /// Last time the cursor blinked
    last_blink: Instant,
    /// Whether the cursor is currently visible
    cursor_visible: bool,
    /// Clipboard buffer for copy/paste operations
    clipboard: String,
    /// Debug message to display
    debug_message: Option<String>,
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
            history: Vec::new(),
            redo_stack: Vec::new(),
            scroll_offset: 0,
            selection_start: None,
            selection_end: None,
            last_blink: Instant::now(),
            cursor_visible: true,
            clipboard: String::new(),
            debug_message: None,
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
            // Update cursor blink state
            let now = Instant::now();
            if now.duration_since(self.last_blink) >= Duration::from_millis(530) {
                self.cursor_visible = !self.cursor_visible;
                self.last_blink = now;
            }

            self.draw(&mut stdout)?;

            // Poll for events with a shorter timeout to maintain smooth cursor blinking
            if event::poll(Duration::from_millis(50))? {
                if let Event::Key(key) = event::read()? {
                    // Reset cursor visibility on any key press
                    self.cursor_visible = true;
                    self.last_blink = Instant::now();
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
        // Add debug logging for selection state
        if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
            eprintln!("DEBUG: Current selection - start: {}, end: {}", start, end);
        }

        // Clear redo stack when new operations are performed
        self.redo_stack.clear();

        // Check if we're in selection mode (Shift is pressed)
        let is_selection_mode = key.modifiers.contains(KeyModifiers::SHIFT);

        // Get selection range if it exists (with bounds checking)
        let selection = if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
            let text_len = self.rope.char_size();
            eprintln!("DEBUG: Text length: {}", text_len);
            
            // Order the selection points
            let (start, end) = if start <= end {
                (start, end)
            } else {
                (end, start)
            };
            
            // Then apply bounds checking
            let safe_start = start.min(text_len);
            let safe_end = end.min(text_len);
            eprintln!("DEBUG: Safe selection bounds - start: {}, end: {}", safe_start, safe_end);
            
            Some((safe_start, safe_end))
        } else {
            None
        };

        match key.code {
            KeyCode::Char(c) => {
                if let Some((start, end)) = selection {
                    // Remove the selected text
                    let selected_text = self.rope.text_range(start, end);
                    self.rope.remove(start, end);
                    self.history.push(Operation::Remove {
                        position: start,
                        text: selected_text,
                    });

                    // Insert the new character at the start position
                    self.rope.insert(start, &c.to_string());
                    self.history.push(Operation::Insert {
                        position: start,
                        text: c.to_string(),
                    });

                    // Move cursor to after the inserted character
                    let (x, y) = self.get_cursor_position(start + 1);
                    self.cursor = (x, y);
                } else {
                    let pos = self.get_absolute_position();
                    self.rope.insert(pos, &c.to_string());
                    self.history.push(Operation::Insert {
                        position: pos,
                        text: c.to_string(),
                    });
                    self.cursor.0 += 1;
                }
                // Clear selection after inserting
                self.selection_start = None;
                self.selection_end = None;
            }
            KeyCode::Tab => {
                if let Some((start, end)) = selection {
                    // Remove the selected text
                    let selected_text = self.rope.text_range(start, end);
                    self.rope.remove(start, end);
                    self.history.push(Operation::Remove {
                        position: start,
                        text: selected_text,
                    });

                    // Insert spaces at the start position
                    self.rope.insert(start, "    ");
                    self.history.push(Operation::Insert {
                        position: start,
                        text: "    ".to_string(),
                    });

                    // Move cursor to after the inserted spaces
                    let (x, y) = self.get_cursor_position(start + 4);
                    self.cursor = (x, y);
                } else {
                    let pos = self.get_absolute_position();
                    self.rope.insert(pos, "    ");
                    self.history.push(Operation::Insert {
                        position: pos,
                        text: "    ".to_string(),
                    });
                    self.cursor.0 += 4;
                }
                // Clear selection after inserting
                self.selection_start = None;
                self.selection_end = None;
            }
            KeyCode::Enter => {
                if let Some((start, end)) = selection {
                    // Remove the selected text
                    let selected_text = self.rope.text_range(start, end);
                    self.rope.remove(start, end);
                    self.history.push(Operation::Remove {
                        position: start,
                        text: selected_text,
                    });

                    // Insert newline at the start position
                    self.rope.insert(start, "\n");
                    self.history.push(Operation::Insert {
                        position: start,
                        text: "\n".to_string(),
                    });

                    // Move cursor to the start of the new line
                    let (x, y) = self.get_cursor_position(start + 1);
                    self.cursor = (x, y);
                } else {
                    let pos = self.get_absolute_position();
                    self.rope.insert(pos, "\n");
                    self.history.push(Operation::Insert {
                        position: pos,
                        text: "\n".to_string(),
                    });
                    self.cursor.0 = 0;
                    self.cursor.1 += 1;
                }
                // Clear selection after inserting
                self.selection_start = None;
                self.selection_end = None;
            }
            KeyCode::Backspace => {
                if let Some((start, end)) = selection {
                    // Ensure we don't try to remove beyond the text bounds
                    let text_size = self.rope.char_size();
                    eprintln!("DEBUG: Backspace with selection - text size: {}", text_size);
                    
                    // No need to reorder here since it's already ordered in the selection handling above
                    eprintln!("DEBUG: Safe removal bounds - start: {}, end: {}", start, end);
                    
                    if start < end {
                        // Log the text being removed
                        let selected_text = self.rope.text_range(start, end);
                        eprintln!("DEBUG: Removing text: {:?}", selected_text);
                        
                        self.rope.remove(start, end);
                        self.history.push(Operation::Remove {
                            position: start,
                            text: selected_text,
                        });

                        // Move cursor to the start of the selection
                        let (x, y) = self.get_cursor_position(start);
                        eprintln!("DEBUG: New cursor position - x: {}, y: {}", x, y);
                        self.cursor = (x, y);
                    }
                    
                    // Clear selection after removing
                    self.selection_start = None;
                    self.selection_end = None;
                } else {
                    let pos = self.get_absolute_position();
                    if pos > 0 {
                        let text = self.rope.to_string();
                        let prev_char = text.chars().nth(pos - 1);
                        
                        if let Some(c) = prev_char {
                            self.history.push(Operation::Remove {
                                position: pos - 1,
                                text: c.to_string(),
                            });
                        }
                        
                        self.rope.remove(pos - 1, pos);

                        if prev_char == Some('\n') {
                            if self.cursor.1 > 0 {
                                self.cursor.1 -= 1;
                                let line = self.get_line(self.cursor.1);
                                self.cursor.0 = line.len() as u16;
                            }
                        } else if self.cursor.0 > 0 {
                            self.cursor.0 -= 1;
                        }
                    }
                }
            }
            KeyCode::Left => {
                // Update selection before moving cursor
                if is_selection_mode {
                    if self.selection_start.is_none() {
                        let current_pos = self.get_absolute_position();
                        self.selection_start = Some(current_pos);
                        self.selection_end = Some(current_pos);
                    }
                } else {
                    self.selection_start = None;
                    self.selection_end = None;
                }

                // Move cursor
                if self.cursor.0 > 0 {
                    self.cursor.0 -= 1;
                } else if self.cursor.1 > 0 {
                    self.cursor.1 -= 1;
                    let line = self.get_line(self.cursor.1);
                    self.cursor.0 = line.len() as u16;
                }

                // Update selection after moving
                if is_selection_mode {
                    self.selection_end = Some(self.get_absolute_position());
                }
            }
            KeyCode::Right => {
                // Store current position before moving
                let current_pos = self.get_absolute_position();
                
                // Update selection before moving cursor
                if is_selection_mode {
                    if self.selection_start.is_none() {
                        // Start new selection from current position
                        self.selection_start = Some(current_pos);
                        self.selection_end = Some(current_pos);
                    }
                } else {
                    self.selection_start = None;
                    self.selection_end = None;
                }

                // Move cursor
                let line = self.get_line(self.cursor.1);
                if self.cursor.0 < line.len() as u16 {
                    self.cursor.0 += 1;
                } else {
                    let text = self.rope.to_string();
                    let lines: Vec<&str> = text.lines().collect();
                    if self.cursor.1 + 1 < lines.len() as u16 {
                        self.cursor.0 = 0;
                        self.cursor.1 += 1;
                    }
                }

                // Update selection after moving
                if is_selection_mode {
                    self.selection_end = Some(self.get_absolute_position());
                }
            }
            KeyCode::Up => {
                if is_selection_mode && self.selection_start.is_none() {
                    self.selection_start = Some(self.get_absolute_position());
                }

                if self.cursor.1 > 0 {
                    self.cursor.1 -= 1;
                    let line = self.get_line(self.cursor.1);
                    self.cursor.0 = self.cursor.0.min(line.len() as u16);
                }

                if is_selection_mode {
                    self.update_selection_end();
                } else {
                    self.selection_start = None;
                    self.selection_end = None;
                }
            }
            KeyCode::Down => {
                let current_pos = self.get_absolute_position();
                
                if is_selection_mode {
                    if self.selection_start.is_none() {
                        self.selection_start = Some(current_pos);
                        self.selection_end = Some(current_pos);
                    }
                } else {
                    self.selection_start = None;
                    self.selection_end = None;
                }

                let text = self.rope.to_string();
                let lines: Vec<&str> = text.lines().collect();
                if self.cursor.1 + 1 < lines.len() as u16 {
                    self.cursor.1 += 1;
                    let line = self.get_line(self.cursor.1);
                    self.cursor.0 = self.cursor.0.min(line.len() as u16);
                }

                if is_selection_mode {
                    self.selection_end = Some(self.get_absolute_position());
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Gets the absolute position in the text based on cursor coordinates
    fn get_absolute_position(&self) -> usize {
        let text = self.rope.to_string();
        if text.is_empty() {
            return 0;
        }

        let mut current_line = 0;
        let mut line_start = 0;

        // Handle case where cursor is beyond text bounds
        if self.cursor.1 as usize >= text.lines().count() {
            return text.len();
        }

        for (i, c) in text.chars().enumerate() {
            if current_line == self.cursor.1 {
                // If we're at the target line
                let line_offset = (i - line_start) as u16;
                if line_offset == self.cursor.0 {
                    return i;
                }
            }
            if c == '\n' {
                if current_line == self.cursor.1 {
                    // If we hit a newline on our target line, return current position
                    return i;
                }
                current_line += 1;
                line_start = i + 1;
            }
        }

        // If we're at the last line
        if current_line == self.cursor.1 {
            return text.len();
        }

        // Fallback: return a safe position
        text.len()
    }

    /// Gets the start position of a line in absolute position
    fn get_line_start_position(&self, line_number: u16) -> usize {
        let text = self.rope.to_string();
        if text.is_empty() {
            return 0;
        }

        let mut current_line = 0;
        let mut last_pos = 0;

        // Handle case where requested line is beyond text bounds
        if line_number as usize >= text.lines().count() {
            return text.len();
        }

        for (i, c) in text.chars().enumerate() {
            if current_line == line_number {
                return i;
            }
            if c == '\n' {
                current_line += 1;
                last_pos = i + 1;
            }
        }

        // If we're looking for a line beyond what we found
        last_pos.min(text.len())
    }

    /// Gets the content of a specific line
    fn get_line(&self, line_number: u16) -> String {
        let text = self.rope.to_string();
        text.lines().nth(line_number as usize).unwrap_or("").to_string()
    }

    /// Handles key events in Macro mode
    fn handle_macro_mode(&mut self, key: KeyEvent) -> io::Result<()> {
        // Check if we're in selection mode (Shift is pressed)
        let is_selection_mode = key.modifiers.contains(KeyModifiers::SHIFT);

        // Start selection if shift is pressed and we don't have a selection yet
        if is_selection_mode && self.selection_start.is_none() {
            let current_pos = self.get_absolute_position();
            self.selection_start = Some(current_pos);
            self.selection_end = Some(current_pos);
            self.set_debug(format!("Started selection at pos {}", current_pos));
        }

        match key.code {
            KeyCode::Char('i') => {
                self.mode = Mode::Insert;
            }
            KeyCode::Char('`') => {
                self.mode = Mode::Command(String::new());
            }
            KeyCode::Char('u') => {
                self.undo()?;
            }
            KeyCode::Char('r') => {
                self.redo()?;
            }
            // Vim-like movement keys
            KeyCode::Char('h') | KeyCode::Left => {
                if self.cursor.0 > 0 {
                    self.cursor.0 -= 1;
                } else if self.cursor.1 > 0 {
                    self.cursor.1 -= 1;
                    let line = self.get_line(self.cursor.1);
                    self.cursor.0 = line.len() as u16;
                }
                if is_selection_mode {
                    self.selection_end = Some(self.get_absolute_position());
                    self.set_debug(format!("Selection: {} to {}", 
                        self.selection_start.unwrap_or(0), 
                        self.selection_end.unwrap_or(0)));
                } else {
                    self.selection_start = None;
                    self.selection_end = None;
                }
            }
            KeyCode::Char('l') | KeyCode::Right => {
                let line = self.get_line(self.cursor.1);
                if self.cursor.0 < line.len() as u16 {
                    self.cursor.0 += 1;
                } else {
                    let text = self.rope.to_string();
                    let lines: Vec<&str> = text.lines().collect();
                    if self.cursor.1 + 1 < lines.len() as u16 {
                        self.cursor.0 = 0;
                        self.cursor.1 += 1;
                    }
                }
                if is_selection_mode {
                    self.selection_end = Some(self.get_absolute_position());
                    self.set_debug(format!("Selection: {} to {}", 
                        self.selection_start.unwrap_or(0), 
                        self.selection_end.unwrap_or(0)));
                } else {
                    self.selection_start = None;
                    self.selection_end = None;
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if self.cursor.1 > 0 {
                    self.cursor.1 -= 1;
                    let line = self.get_line(self.cursor.1);
                    self.cursor.0 = self.cursor.0.min(line.len() as u16);
                }
                if is_selection_mode {
                    self.selection_end = Some(self.get_absolute_position());
                    self.set_debug(format!("Selection: {} to {}", 
                        self.selection_start.unwrap_or(0), 
                        self.selection_end.unwrap_or(0)));
                } else {
                    self.selection_start = None;
                    self.selection_end = None;
                }
            }
            KeyCode::Char('j') | KeyCode::Down => {
                let text = self.rope.to_string();
                let lines: Vec<&str> = text.lines().collect();
                if self.cursor.1 + 1 < lines.len() as u16 {
                    self.cursor.1 += 1;
                    let line = self.get_line(self.cursor.1);
                    self.cursor.0 = self.cursor.0.min(line.len() as u16);
                }
                if is_selection_mode {
                    self.selection_end = Some(self.get_absolute_position());
                    self.set_debug(format!("Selection: {} to {}", 
                        self.selection_start.unwrap_or(0), 
                        self.selection_end.unwrap_or(0)));
                } else {
                    self.selection_start = None;
                    self.selection_end = None;
                }
            }
            // Word movement
            KeyCode::Char('w') => {
                let text = self.rope.to_string();
                let pos = self.get_absolute_position();
                let mut chars = text[pos..].chars();
                let mut word_end = pos;
                
                // Skip current word
                while let Some(c) = chars.next() {
                    word_end += 1;
                    if c.is_whitespace() {
                        break;
                    }
                }
                
                // Skip whitespace
                while let Some(c) = chars.next() {
                    word_end += 1;
                    if !c.is_whitespace() {
                        break;
                    }
                }
                
                let (x, y) = self.get_cursor_position(word_end);
                self.cursor = (x, y);
                
                if is_selection_mode {
                    self.selection_end = Some(word_end);
                }
            }
            KeyCode::Char('b') => {
                let text = self.rope.to_string();
                let pos = self.get_absolute_position();
                if pos > 0 {
                    let mut word_start = pos - 1;
                    
                    // Skip back over whitespace
                    while word_start > 0 && text.chars().nth(word_start).map_or(false, |c| c.is_whitespace()) {
                        word_start -= 1;
                    }
                    
                    // Skip back over word
                    while word_start > 0 && text.chars().nth(word_start - 1).map_or(false, |c| !c.is_whitespace()) {
                        word_start -= 1;
                    }
                    
                    let (x, y) = self.get_cursor_position(word_start);
                    self.cursor = (x, y);
                    
                    if is_selection_mode {
                        self.selection_end = Some(word_start);
                    }
                }
            }
            // Copy selection
            KeyCode::Char('c') => {
                if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
                    let text_len = self.rope.char_size();
                    
                    // Order the selection points
                    let (start, end) = if start <= end {
                        (start, end)
                    } else {
                        (end, start)
                    };
                    
                    // Apply bounds checking
                    let safe_start = start.min(text_len);
                    let safe_end = end.min(text_len);
                    
                    // Copy the selected text to clipboard
                    if safe_start < safe_end {
                        self.clipboard = self.rope.text_range(safe_start, safe_end);
                        self.set_debug(format!("Copied {} chars: {:?}", 
                            self.clipboard.len(),
                            if self.clipboard.len() > 20 {
                                format!("{}...", &self.clipboard[..20])
                            } else {
                                self.clipboard.clone()
                            }
                        ));
                    }

                    // Don't clear selection after copying
                    // This allows for multiple operations on the same selection
                } else {
                    self.set_debug("No selection to copy".to_string());
                }
            }
            // Cut selection
            KeyCode::Char('x') => {
                if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
                    let text_len = self.rope.char_size();
                    
                    // Order the selection points
                    let (start, end) = if start <= end {
                        (start, end)
                    } else {
                        (end, start)
                    };
                    
                    // Apply bounds checking
                    let safe_start = start.min(text_len);
                    let safe_end = end.min(text_len);
                    
                    // Cut (copy and remove) the selected text
                    if safe_start < safe_end {
                        // First copy to clipboard
                        self.clipboard = self.rope.text_range(safe_start, safe_end);
                        
                        // Then remove the text
                        self.rope.remove(safe_start, safe_end);
                        self.history.push(Operation::Remove {
                            position: safe_start,
                            text: self.clipboard.clone(),
                        });

                        // Move cursor to the start of where the cut text was
                        let (x, y) = self.get_cursor_position(safe_start);
                        self.cursor = (x, y);

                        self.set_debug(format!("Cut {} chars: {:?}", 
                            self.clipboard.len(),
                            if self.clipboard.len() > 20 {
                                format!("{}...", &self.clipboard[..20])
                            } else {
                                self.clipboard.clone()
                            }
                        ));
                    }

                    // Clear selection after cutting
                    self.selection_start = None;
                    self.selection_end = None;
                } else {
                    self.set_debug("No selection to cut".to_string());
                }
            }
            // Paste clipboard content
            KeyCode::Char('v') => {
                if !self.clipboard.is_empty() {
                    let pos = self.get_absolute_position();
                    
                    // If there's a selection, remove it first
                    if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
                        let text_len = self.rope.char_size();
                        let (start, end) = if start <= end {
                            (start, end)
                        } else {
                            (end, start)
                        };
                        let safe_start = start.min(text_len);
                        let safe_end = end.min(text_len);
                        
                        if safe_start < safe_end {
                            let removed_text = self.rope.text_range(safe_start, safe_end);
                            self.rope.remove(safe_start, safe_end);
                            self.history.push(Operation::Remove {
                                position: safe_start,
                                text: removed_text,
                            });
                        }
                    }
                    
                    // Insert clipboard content
                    self.rope.insert(pos, &self.clipboard);
                    self.history.push(Operation::Insert {
                        position: pos,
                        text: self.clipboard.clone(),
                    });
                    
                    // Move cursor to end of pasted text
                    let new_pos = pos + self.clipboard.len();
                    let (x, y) = self.get_cursor_position(new_pos);
                    self.cursor = (x, y);
                    
                    // Clear selection
                    self.selection_start = None;
                    self.selection_end = None;
                    
                    self.set_debug(format!("Pasted {} chars", self.clipboard.len()));
                } else {
                    self.set_debug("Clipboard is empty".to_string());
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Undo the last operation
    fn undo(&mut self) -> io::Result<()> {
        if let Some(operation) = self.history.pop() {
            // Store the operation in redo stack
            self.redo_stack.push(operation.clone());

            match operation {
                Operation::Insert { position, text } => {
                    // Remove the inserted text
                    self.rope.remove(position, position + text.len());
                    
                    // Update cursor position
                    let (x, y) = self.get_cursor_position(position);
                    self.cursor = (x, y);
                }
                Operation::Remove { position, text } => {
                    // Re-insert the removed text
                    self.rope.insert(position, &text);
                    
                    // Update cursor position
                    let (x, y) = self.get_cursor_position(position + text.len());
                    self.cursor = (x, y);
                }
            }
        }
        Ok(())
    }

    /// Redo the last undone operation
    fn redo(&mut self) -> io::Result<()> {
        if let Some(operation) = self.redo_stack.pop() {
            // Store the operation back in history
            self.history.push(operation.clone());

            match operation {
                Operation::Insert { position, text } => {
                    // Re-insert the text
                    self.rope.insert(position, &text);
                    
                    // Update cursor position
                    let (x, y) = self.get_cursor_position(position + text.len());
                    self.cursor = (x, y);
                }
                Operation::Remove { position, text } => {
                    // Remove the text again
                    self.rope.remove(position, position + text.len());
                    
                    // Update cursor position
                    let (x, y) = self.get_cursor_position(position);
                    self.cursor = (x, y);
                }
            }
        }
        Ok(())
    }

    /// Gets cursor coordinates (x, y) for a given absolute position
    fn get_cursor_position(&self, position: usize) -> (u16, u16) {
        let text = self.rope.to_string();
        let mut current_pos = 0;
        let mut x = 0;
        let mut y = 0;

        for c in text.chars() {
            if current_pos == position {
                break;
            }
            if c == '\n' {
                y += 1;
                x = 0;
            } else {
                x += 1;
            }
            current_pos += 1;
        }

        (x, y)
    }

    /// Gets the visual length of a line
    fn get_visual_line_length(&self, line: &str) -> u16 {
        line.chars().count() as u16
    }

    /// Converts a visual position to an actual position in the string
    fn get_actual_position_from_visual(&self, line: &str, visual_pos: usize) -> usize {
        visual_pos.min(line.chars().count())
    }

    /// Updates scroll position based on cursor
    fn update_scroll(&mut self) {
        let visible_lines = self.size.1 - 1; // -1 for status line
        
        // Scroll up if cursor is above visible area
        if self.cursor.1 < self.scroll_offset {
            self.scroll_offset = self.cursor.1;
        }
        
        // Scroll down if cursor is below visible area
        if self.cursor.1 >= self.scroll_offset + visible_lines {
            self.scroll_offset = self.cursor.1 - visible_lines + 1;
        }
    }

    /// Updates selection based on current cursor position
    fn update_selection_end(&mut self) {
        if let Some(start) = self.selection_start {
            let text_len = self.rope.char_size();
            eprintln!("DEBUG: update_selection_end - text_len: {}, start: {}", text_len, start);
            
            // Ensure start position is within bounds
            let safe_start = start.min(text_len);
            
            // Get current position with bounds checking
            let current = self.get_absolute_position();
            let safe_current = current.min(text_len);
            
            eprintln!("DEBUG: update_selection_end - safe_start: {}, safe_current: {}", safe_start, safe_current);
            
            // Update selection with safe values
            self.selection_start = Some(safe_start);
            self.selection_end = Some(safe_current);
        }
    }

    /// Sets a debug message to be displayed
    fn set_debug(&mut self, msg: String) {
        self.debug_message = Some(msg);
    }

    /// Draws the editor UI
    fn draw(&mut self, stdout: &mut io::Stdout) -> io::Result<()> {
        self.update_scroll();
        execute!(stdout, Clear(ClearType::All))?;

        let text = self.rope.to_string();
        let lines: Vec<&str> = text.lines().collect();
        let total_lines = lines.len();
        let margin_width = (total_lines + 1).to_string().len().max(2);

        // Draw status line with selection info and debug message
        let mode_str = match &self.mode {
            Mode::Insert => "INSERT",
            Mode::Macro => "MACRO",
            Mode::Command(cmd) => if cmd.is_empty() { "COMMAND" } else { cmd },
        };
        let selection_info = if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
            let len = if start > end {
                start - end
            } else {
                end - start
            };
            format!(" [SEL:{}]", len)
        } else {
            String::new()
        };
        let clipboard_info = if !self.clipboard.is_empty() {
            format!(" [CLIP:{}]", self.clipboard.len())
        } else {
            String::new()
        };
        let debug_info = if let Some(msg) = &self.debug_message {
            format!(" | {}", msg)
        } else {
            String::new()
        };
        execute!(
            stdout,
            MoveTo(0, 0),
            SetForegroundColor(Color::White),
            Print(format!(
                " {}{}{} | {}:{} [{}/{}]{}",
                mode_str, selection_info, clipboard_info, 
                self.cursor.0, self.cursor.1, 
                self.scroll_offset + 1, total_lines,
                debug_info
            ))
        )?;

        let mut y = 1;
        let content_start_x = (margin_width + 1) as u16;
        let visible_lines = self.size.1 - 1;

        // Get selection range (ordered)
        let (sel_start, sel_end) = if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
            if start <= end {
                (start, end)
            } else {
                (end, start)
            }
        } else {
            (0, 0)
        };

        // Draw visible lines
        for line_num in self.scroll_offset..self.scroll_offset + visible_lines {
            if y >= self.size.1 {
                break;
            }

            // Check if this line should show the cursor
            let is_cursor_line = line_num == self.cursor.1;

            if let Some(line) = lines.get(line_num as usize) {
                // Draw line number
                execute!(
                    stdout,
                    MoveTo(0, y),
                    SetForegroundColor(Color::DarkGrey),
                    ResetColor,
                    Print(format!("{:>width$} ", line_num + 1, width = margin_width))
                )?;

                // Draw line content with selection highlighting
                let mut x = content_start_x;
                let line_start = self.get_line_start_position(line_num);
                
                for (i, c) in line.chars().enumerate() {
                    let pos = line_start + i;
                    let is_selected = self.selection_start.is_some() && 
                                    pos >= sel_start && 
                                    pos < sel_end;
                    let is_cursor_pos = is_cursor_line && 
                                      x == content_start_x + self.cursor.0;
                    
                    if is_cursor_pos && self.cursor_visible {
                        execute!(
                            stdout,
                            MoveTo(x, y),
                            SetAttribute(Attribute::Reverse),
                            Print(c),
                            SetAttribute(Attribute::Reset)
                        )?;
                    } else if is_selected {
                        execute!(
                            stdout,
                            MoveTo(x, y),
                            SetBackgroundColor(Color::Yellow),
                            SetForegroundColor(Color::Black),
                            Print(c),
                            ResetColor
                        )?;
                    } else {
                        execute!(
                            stdout,
                            MoveTo(x, y),
                            SetForegroundColor(Color::White),
                            Print(c)
                        )?;
                    }
                    x += 1;
                }

                // Draw cursor at end of line if needed
                if is_cursor_line && 
                   self.cursor.0 as usize == line.len() && 
                   self.cursor_visible {
                    execute!(
                        stdout,
                        MoveTo(content_start_x + self.cursor.0, y),
                        SetAttribute(Attribute::Reverse),
                        Print(" "),
                        SetAttribute(Attribute::Reset)
                    )?;
                }
            } else {
                // Draw empty line marker
                execute!(
                    stdout,
                    MoveTo(0, y),
                    SetForegroundColor(Color::DarkGrey),
                    Print(format!("{:>width$} ~", "", width = margin_width))
                )?;

                // Draw cursor on empty line if needed
                if is_cursor_line && self.cursor_visible {
                    execute!(
                        stdout,
                        MoveTo(content_start_x + self.cursor.0, y),
                        SetAttribute(Attribute::Reverse),
                        Print(" "),
                        SetAttribute(Attribute::Reset)
                    )?;
                }
            }
            y += 1;
        }

        stdout.flush()?;
        Ok(())
    }
} 