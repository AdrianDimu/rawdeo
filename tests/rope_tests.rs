#[cfg(test)]
mod tests {
    use rawdeo::Rope;

    #[test]
    fn test_single_line() {
        let rope = Rope::new("Hello World!");
        assert_eq!(rope.lines(), 1);
    }

    #[test]
    fn test_multiple_lines() {
        let text = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8\nLine 9\nLine 10";
        let rope = Rope::new(text);
        assert_eq!(rope.lines(), 10);
    }

    #[test]
    fn test_empty_string() {
        let rope = Rope::new("");
        assert_eq!(rope.lines(), 1);
    }    
}