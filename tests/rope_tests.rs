#[cfg(test)]
mod tests {
    use rawdeo::Rope;

    #[test]
    fn test_single_line() {
        let rope = Rope::new("Hello World!");
        assert_eq!(rope.lines(), 1);
        assert_eq!(rope.char_size(), 12);
    }

    #[test]
    fn test_multiple_lines() {
        let text = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8\nLine 9\nLine 10";
        let rope = Rope::new(text);
        assert_eq!(rope.lines(), 10);
        assert_eq!(rope.char_size(), text.len());
    }

    #[test]
    fn test_empty_string() {
        let rope = Rope::new("");
        assert_eq!(rope.lines(), 1);
        assert_eq!(rope.char_size(), 0);
    }

    #[test]
    fn test_single_newline() {
        let rope = Rope::new("\n");
        assert_eq!(rope.lines(), 2); // A single newline creates two lines
        assert_eq!(rope.char_size(), 1);
    }

    #[test]
    fn test_trailing_newline() {
        let rope = Rope::new("Hello\n");
        assert_eq!(rope.lines(), 2); // "Hello" is one line, and the trailing "\n" creates another
        assert_eq!(rope.char_size(), 6);
    }

    #[test]
    fn test_only_newlines() {
        let rope = Rope::new("\n\n\n\n");
        assert_eq!(rope.lines(), 5); // 4 newlines = 5 lines
        assert_eq!(rope.char_size(), 4);
    }

    #[test]
    fn test_long_text() {
        let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n\
                    Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.\n\
                    Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.\n\
                    Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.\n\
                    Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
        let rope = Rope::new(text);
        let expected_lines = text.lines().count();
        assert_eq!(rope.lines(), expected_lines);
        assert_eq!(rope.char_size(), text.len());
    }

    #[test]
    fn test_single_character() {
        let rope = Rope::new("a");
        assert_eq!(rope.lines(), 1);
        assert_eq!(rope.char_size(), 1);
    }

    #[test]
    fn test_large_input() {
        let large_text = "Rust is great!\n".repeat(10000);
        let rope = Rope::new(&large_text);
        assert_eq!(rope.lines(), 10001);
        assert_eq!(rope.char_size(), large_text.len());
    }
}