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
        assert_eq!(rope.lines(), 2);
        assert_eq!(rope.char_size(), 1);
    }

    #[test]
    fn test_trailing_newline() {
        let rope = Rope::new("Hello\n");
        assert_eq!(rope.lines(), 2);
        assert_eq!(rope.char_size(), 6);
    }

    #[test]
    fn test_only_newlines() {
        let rope = Rope::new("\n\n\n\n");
        assert_eq!(rope.lines(), 5);
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

    #[test]
    fn test_insert_at_start() {
        let mut rope = Rope::new("World");
        rope.insert(0, "Hello ");
        assert_eq!(rope.char_size(), 11);
        assert_eq!(rope.lines(), 1);
    }

    #[test]
    fn test_insert_at_end() {
        let mut rope = Rope::new("Hello");
        rope.insert(5, " World");
        assert_eq!(rope.char_size(), 11);
        assert_eq!(rope.lines(), 1);
    }

    #[test]
    fn test_insert_in_middle() {
        let mut rope = Rope::new("Hello World");
        rope.insert(6, "Beautiful ");
        assert_eq!(rope.char_size(), 21);
        assert_eq!(rope.lines(), 1);
    }

    #[test]
    fn test_insert_newline() {
        let mut rope = Rope::new("Hello World");
        rope.insert(5, "\n");
        assert_eq!(rope.char_size(), 12);
        assert_eq!(rope.lines(), 2);
    }

    #[test]
    fn test_insert_multiple_lines() {
        let mut rope = Rope::new("Line 1\nLine 2");
        rope.insert(6, "New Line\n");
        assert_eq!(rope.char_size(), 22);
        assert_eq!(rope.lines(), 3);
    }

    #[test]
    fn test_insert_empty_string() {
        let mut rope = Rope::new("Hello");
        rope.insert(2, "");
        assert_eq!(rope.char_size(), 5);
        assert_eq!(rope.lines(), 1);
    }

    #[test]
    fn test_insert_at_boundaries() {
        let mut rope = Rope::new("Hello\nWorld");
        rope.insert(5, " ");
        assert_eq!(rope.char_size(), 12);
        assert_eq!(rope.lines(), 2);
    }

    #[test]
    fn test_insert_large_text() {
        let mut rope = Rope::new("Start\nEnd");
        let large_text = "Middle\n".repeat(100);
        rope.insert(6, &large_text);
        assert_eq!(rope.lines(), 102);
        assert_eq!(rope.char_size(), 6 + large_text.len() + 3);
    }

    #[test]
    fn test_insert_newline_leaf_structure() {
        let mut rope = Rope::new("Hello World");
        rope.insert(5, "\n");
        assert_eq!(rope.lines(), 2);
        assert_eq!(rope.leaf_count(), 2); // Should have one Leaf per line
    }

    #[test]
    fn test_insert_multiple_newlines_leaf_structure() {
        let mut rope = Rope::new("Line 1\nLine 2");
        rope.insert(6, "New Line\n");
        assert_eq!(rope.lines(), 3);
        assert_eq!(rope.leaf_count(), 3); // Should have one Leaf per line
    }

    #[test]
    fn test_insert_large_text_leaf_structure() {
        let mut rope = Rope::new("Start\nEnd");
        let large_text = "Middle\n".repeat(100);
        rope.insert(6, &large_text);
        rope.print_structure();
        assert_eq!(rope.lines(), 102);
        assert_eq!(rope.leaf_count(), 102); // Should have one Leaf per line
    }

    #[test]
    fn test_insert_empty_lines_leaf_structure() {
        let mut rope = Rope::new("Hello");
        rope.insert(5, "\n\n\n");
        assert_eq!(rope.lines(), 4);
        assert_eq!(rope.leaf_count(), 4); // Should have one Leaf per line, including empty lines
    }

    #[test]
    fn test_insert_at_line_boundaries_leaf_structure() {
        let mut rope = Rope::new("Line 1\nLine 2\nLine 3");
        rope.insert(6, "New\n");
        assert_eq!(rope.lines(), 4);
        assert_eq!(rope.leaf_count(), 4); // Should have one Leaf per line
    }

    #[test]
    fn test_remove_single_leaf() {
        let mut rope = Rope::new("Hello World");
        rope.remove(5, 7); // Remove " W"
        assert_eq!(rope.char_size(), 9);
        assert_eq!(rope.lines(), 1);
        assert_eq!(rope.leaf_count(), 1);
    }

    #[test]
    fn test_remove_across_leaves() {
        let mut rope = Rope::new("Hello\nWorld");
        rope.remove(4, 7); // Remove "o\nW"
        assert_eq!(rope.char_size(), 8);
        assert_eq!(rope.lines(), 1);
        assert_eq!(rope.leaf_count(), 1);
    }

    #[test]
    fn test_remove_newlines() {
        let mut rope = Rope::new("Line 1\nLine 2\nLine 3");
        rope.remove(6, 14); // Remove "\nLine 2\n" -> "Line 1Line 3"
        assert_eq!(rope.char_size(), 12);
        assert_eq!(rope.lines(), 1);
        assert_eq!(rope.leaf_count(), 1);
    }

    #[test]
    fn test_remove_empty_range() {
        let mut rope = Rope::new("Hello World");
        rope.remove(5, 5); // Remove nothing
        assert_eq!(rope.char_size(), 11);
        assert_eq!(rope.lines(), 1);
        assert_eq!(rope.leaf_count(), 1);
    }

    #[test]
    fn test_remove_full_range() {
        let mut rope = Rope::new("Hello World");
        rope.remove(0, 11); // Remove everything
        assert_eq!(rope.char_size(), 0);
        assert_eq!(rope.lines(), 1);
        assert_eq!(rope.leaf_count(), 1);
    }

    #[test]
    fn test_remove_at_start() {
        let mut rope = Rope::new("Hello World");
        rope.remove(0, 6); // Remove "Hello "
        assert_eq!(rope.char_size(), 5);
        assert_eq!(rope.lines(), 1);
        assert_eq!(rope.leaf_count(), 1);
    }

    #[test]
    fn test_remove_at_end() {
        let mut rope = Rope::new("Hello World");
        rope.remove(6, 11); // Remove "World"
        assert_eq!(rope.char_size(), 6);
        assert_eq!(rope.lines(), 1);
        assert_eq!(rope.leaf_count(), 1);
    }

    #[test]
    fn test_remove_multiple_newlines() {
        let mut rope = Rope::new("Line 1\n\n\nLine 4");
        rope.remove(6, 9); // Remove "\n\n\n"
        assert_eq!(rope.char_size(), 12);
        assert_eq!(rope.lines(), 1);
        assert_eq!(rope.leaf_count(), 1);
    }

    #[test]
    fn test_remove_and_insert() {
        let mut rope = Rope::new("Hello World");
        rope.remove(5, 7); // Remove " W"
        rope.insert(5, " "); // Insert " " back
        assert_eq!(rope.char_size(), 10);
        assert_eq!(rope.lines(), 1);
        assert_eq!(rope.leaf_count(), 1);
    }

    #[test]
    fn test_remove_empty_rope() {
        let mut rope = Rope::new("");
        rope.remove(0, 0); // Remove nothing from empty rope
        assert_eq!(rope.char_size(), 0);
        assert_eq!(rope.lines(), 1);
        assert_eq!(rope.leaf_count(), 1);
    }

    #[test]
    fn test_remove_large_text() {
        let text = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8";
        let mut rope = Rope::new(text);
        rope.remove(12, 36); // Remove "2\nLine 3\nLine 4\nLine 5\nL" -> "Line 1\nLine ine 6\nLine 7\nLine "
        assert_eq!(rope.char_size(), 31);
        assert_eq!(rope.lines(), 4);
        assert_eq!(rope.leaf_count(), 4);
    }
}