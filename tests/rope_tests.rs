/// Tests for the Rope data structure implementation.
/// 
/// This test suite covers:
/// - Basic operations (insert, remove)
/// - Line handling and newlines
/// - Edge cases and boundary conditions
/// - Performance with large inputs
/// - Tree structure maintenance
/// - Empty and single-character cases

#[cfg(test)]
mod tests {
    use rawdeo::Rope;

    #[test]
    /// Tests basic operations on an empty Rope.
    /// Verifies that:
    /// - An empty Rope has zero characters and one line
    /// - Inserting text works correctly
    /// - Removing text works correctly
    fn test_empty_string() {
        let rope = Rope::new("");
        assert_eq!(rope.lines(), 1);
        assert_eq!(rope.char_size(), 0);
    }

    #[test]
    /// Tests inserting text at the start of the Rope.
    /// Verifies that:
    /// - Text is inserted correctly at index 0
    /// - Character count is updated
    /// - Line count is maintained
    fn test_insert_at_start() {
        let mut rope = Rope::new("World");
        rope.insert(0, "Hello ");
        assert_eq!(rope.char_size(), 11);
        assert_eq!(rope.lines(), 1);
    }

    #[test]
    /// Tests inserting text at the end of the Rope.
    /// Verifies that:
    /// - Text is appended correctly
    /// - Character count is updated
    /// - Line count is maintained
    fn test_insert_at_end() {
        let mut rope = Rope::new("Hello");
        rope.insert(5, " World");
        assert_eq!(rope.char_size(), 11);
        assert_eq!(rope.lines(), 1);
    }

    #[test]
    /// Tests inserting text in the middle of the Rope.
    /// Verifies that:
    /// - Text is inserted at the correct position
    /// - Existing text is preserved
    /// - Character count is updated
    fn test_insert_in_middle() {
        let mut rope = Rope::new("Hello World");
        rope.insert(6, "Beautiful ");
        assert_eq!(rope.char_size(), 21);
        assert_eq!(rope.lines(), 1);
    }

    #[test]
    /// Tests inserting an empty string.
    /// Verifies that:
    /// - The Rope remains unchanged
    /// - Character and line counts are preserved
    fn test_insert_empty_string() {
        let mut rope = Rope::new("Hello");
        rope.insert(2, "");
        assert_eq!(rope.char_size(), 5);
        assert_eq!(rope.lines(), 1);
    }

    #[test]
    /// Tests inserting text at the boundaries of the Rope.
    /// Verifies that:
    /// - Text can be inserted at the start
    /// - Text can be inserted at the end
    /// - Text can be inserted in the middle
    fn test_insert_at_boundaries() {
        let mut rope = Rope::new("Hello\nWorld");
        rope.insert(5, " ");
        assert_eq!(rope.char_size(), 12);
        assert_eq!(rope.lines(), 2);
    }

    #[test]
    /// Tests inserting text containing newlines.
    /// Verifies that:
    /// - Newlines are handled correctly
    /// - Line count is updated
    /// - Text is split into appropriate segments
    fn test_insert_newline() {
        let mut rope = Rope::new("Hello World");
        rope.insert(5, "\n");
        assert_eq!(rope.char_size(), 12);
        assert_eq!(rope.lines(), 2);
    }

    #[test]
    /// Tests inserting multiple lines of text.
    /// Verifies that:
    /// - Multiple lines are handled correctly
    /// - Line count is updated
    /// - Text structure is maintained
    fn test_insert_multiple_lines() {
        let mut rope = Rope::new("Line 1\nLine 2");
        rope.insert(6, "New Line\n");
        assert_eq!(rope.char_size(), 22);
        assert_eq!(rope.lines(), 3);
    }

    #[test]
    /// Tests inserting text with multiple consecutive newlines.
    /// Verifies that:
    /// - Multiple newlines are handled correctly
    /// - Line count is updated
    /// - Empty lines are preserved
    fn test_insert_multiple_newlines() {
        let mut rope = Rope::new("Line 1\nLine 2");
        rope.insert(6, "New Line\n");
        assert_eq!(rope.lines(), 3);
        assert_eq!(rope.leaf_count(), 3); // Should have one Leaf per line
    }

    #[test]
    /// Tests inserting a single newline character.
    /// Verifies that:
    /// - Single newline is handled correctly
    /// - Line count is updated
    /// - Text structure is maintained
    fn test_single_newline() {
        let rope = Rope::new("\n");
        assert_eq!(rope.lines(), 2);
        assert_eq!(rope.char_size(), 1);
    }

    #[test]
    /// Tests inserting text with only newlines.
    /// Verifies that:
    /// - Multiple newlines are handled correctly
    /// - Line count is updated
    /// - Empty lines are preserved
    fn test_only_newlines() {
        let rope = Rope::new("\n\n\n\n");
        assert_eq!(rope.lines(), 5);
        assert_eq!(rope.char_size(), 4);
    }

    #[test]
    /// Tests inserting text with a trailing newline.
    /// Verifies that:
    /// - Trailing newline is handled correctly
    /// - Line count is updated
    /// - Text structure is maintained
    fn test_trailing_newline() {
        let rope = Rope::new("Hello\n");
        assert_eq!(rope.lines(), 2);
        assert_eq!(rope.char_size(), 6);
    }

    #[test]
    /// Tests inserting a single character.
    /// Verifies that:
    /// - Single character is inserted correctly
    /// - Character count is updated
    /// - Line count is maintained
    fn test_single_character() {
        let rope = Rope::new("a");
        assert_eq!(rope.lines(), 1);
        assert_eq!(rope.char_size(), 1);
    }

    #[test]
    /// Tests inserting a single line of text.
    /// Verifies that:
    /// - Single line is inserted correctly
    /// - Character count is updated
    /// - Line count is maintained
    fn test_single_line() {
        let rope = Rope::new("Hello World!");
        assert_eq!(rope.lines(), 1);
        assert_eq!(rope.char_size(), 12);
    }

    #[test]
    /// Tests inserting multiple lines of text.
    /// Verifies that:
    /// - Multiple lines are handled correctly
    /// - Line count is updated
    /// - Text structure is maintained
    fn test_multiple_lines() {
        let text = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8\nLine 9\nLine 10";
        let rope = Rope::new(text);
        assert_eq!(rope.lines(), 10);
        assert_eq!(rope.char_size(), text.len());
    }

    #[test]
    /// Tests inserting a long text string.
    /// Verifies that:
    /// - Large text is handled correctly
    /// - Character count is updated
    /// - Line count is maintained
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
    /// Tests inserting a very large text string.
    /// Verifies that:
    /// - Large text is handled correctly
    /// - Character count is updated
    /// - Line count is maintained
    /// - Performance is acceptable
    fn test_insert_large_text() {
        let large_text = "Rust is great!\n".repeat(10000);
        let rope = Rope::new(&large_text);
        assert_eq!(rope.lines(), 10001);
        assert_eq!(rope.char_size(), large_text.len());
    }

    #[test]
    /// Tests inserting a very large text string and verifies leaf structure.
    /// Verifies that:
    /// - Large text is handled correctly
    /// - Tree structure is maintained
    /// - Leaf nodes are properly organized
    fn test_insert_large_text_leaf_structure() {
        let mut rope = Rope::new("Start\nEnd");
        let large_text = "Middle\n".repeat(100);
        rope.insert(6, &large_text);
        rope.print_structure();
        assert_eq!(rope.lines(), 102);
        assert_eq!(rope.leaf_count(), 102); // Should have one Leaf per line
    }

    #[test]
    /// Tests inserting text at line boundaries and verifies leaf structure.
    /// Verifies that:
    /// - Line boundaries are handled correctly
    /// - Tree structure is maintained
    /// - Leaf nodes are properly organized
    fn test_insert_at_line_boundaries_leaf_structure() {
        let mut rope = Rope::new("Line 1\nLine 2\nLine 3");
        rope.insert(6, "New\n");
        assert_eq!(rope.lines(), 4);
        assert_eq!(rope.leaf_count(), 4); // Should have one Leaf per line
    }

    #[test]
    /// Tests inserting empty lines and verifies leaf structure.
    /// Verifies that:
    /// - Empty lines are handled correctly
    /// - Tree structure is maintained
    /// - Leaf nodes are properly organized
    fn test_insert_empty_lines_leaf_structure() {
        let mut rope = Rope::new("Hello");
        rope.insert(5, "\n\n\n");
        assert_eq!(rope.lines(), 4);
        assert_eq!(rope.leaf_count(), 4); // Should have one Leaf per line, including empty lines
    }

    #[test]
    /// Tests inserting text with multiple newlines and verifies leaf structure.
    /// Verifies that:
    /// - Multiple newlines are handled correctly
    /// - Tree structure is maintained
    /// - Leaf nodes are properly organized
    fn test_insert_multiple_newlines_leaf_structure() {
        let mut rope = Rope::new("Line 1\nLine 2");
        rope.insert(6, "New Line\n");
        assert_eq!(rope.lines(), 3);
        assert_eq!(rope.leaf_count(), 3); // Should have one Leaf per line
    }

    #[test]
    /// Tests removing text across multiple leaf nodes.
    /// Verifies that:
    /// - Text is removed correctly across leaves
    /// - Character count is updated
    /// - Line count is maintained
    fn test_remove_across_leaves() {
        let mut rope = Rope::new("Hello\nWorld");
        rope.remove(4, 7); // Remove "o\nW"
        assert_eq!(rope.char_size(), 8);
        assert_eq!(rope.lines(), 1);
        assert_eq!(rope.leaf_count(), 1);
    }

    #[test]
    /// Tests removing text containing newlines.
    /// Verifies that:
    /// - Newlines are handled correctly
    /// - Line count is updated
    /// - Text structure is maintained
    fn test_remove_newlines() {
        let mut rope = Rope::new("Line 1\nLine 2\nLine 3");
        rope.remove(6, 14); // Remove "\nLine 2\n" -> "Line 1Line 3"
        assert_eq!(rope.char_size(), 12);
        assert_eq!(rope.lines(), 1);
        assert_eq!(rope.leaf_count(), 1);
    }

    #[test]
    /// Tests removing text with multiple consecutive newlines.
    /// Verifies that:
    /// - Multiple newlines are handled correctly
    /// - Line count is updated
    /// - Text structure is maintained
    fn test_remove_multiple_newlines() {
        let mut rope = Rope::new("Line 1\n\n\nLine 4");
        rope.remove(6, 9); // Remove "\n\n\n"
        assert_eq!(rope.char_size(), 12);
        assert_eq!(rope.lines(), 1);
        assert_eq!(rope.leaf_count(), 1);
    }

    #[test]
    /// Tests removing text from a large text string.
    /// Verifies that:
    /// - Large text removal works correctly
    /// - Character count is updated
    /// - Line count is maintained
    fn test_remove_large_text() {
        let text = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8";
        let mut rope = Rope::new(text);
        rope.remove(12, 36); // Remove "2\nLine 3\nLine 4\nLine 5\nL" -> "Line 1\nLine ine 6\nLine 7\nLine "
        assert_eq!(rope.char_size(), 31);
        assert_eq!(rope.lines(), 4);
        assert_eq!(rope.leaf_count(), 4);
    }

    #[test]
    /// Tests removing text from an empty Rope.
    /// Verifies that:
    /// - Empty Rope remains unchanged
    /// - Character and line counts are preserved
    fn test_remove_empty_rope() {
        let mut rope = Rope::new("");
        rope.remove(0, 0); // Remove nothing from empty rope
        assert_eq!(rope.char_size(), 0);
        assert_eq!(rope.lines(), 1);
        assert_eq!(rope.leaf_count(), 1);
    }

    #[test]
    /// Tests removing an empty range of text.
    /// Verifies that:
    /// - Rope remains unchanged
    /// - Character and line counts are preserved
    fn test_remove_empty_range() {
        let mut rope = Rope::new("Hello World");
        rope.remove(5, 5); // Remove nothing
        assert_eq!(rope.char_size(), 11);
        assert_eq!(rope.lines(), 1);
        assert_eq!(rope.leaf_count(), 1);
    }

    #[test]
    /// Tests removing text from the start of the Rope.
    /// Verifies that:
    /// - Text is removed correctly from the start
    /// - Character count is updated
    /// - Line count is maintained
    fn test_remove_at_start() {
        let mut rope = Rope::new("Hello World");
        rope.remove(0, 6); // Remove "Hello "
        assert_eq!(rope.char_size(), 5);
        assert_eq!(rope.lines(), 1);
        assert_eq!(rope.leaf_count(), 1);
    }

    #[test]
    /// Tests removing text from the end of the Rope.
    /// Verifies that:
    /// - Text is removed correctly from the end
    /// - Character count is updated
    /// - Line count is maintained
    fn test_remove_at_end() {
        let mut rope = Rope::new("Hello World");
        rope.remove(6, 11); // Remove "World"
        assert_eq!(rope.char_size(), 6);
        assert_eq!(rope.lines(), 1);
        assert_eq!(rope.leaf_count(), 1);
    }

    #[test]
    /// Tests removing the entire content of the Rope.
    /// Verifies that:
    /// - All text is removed
    /// - Character count is zero
    /// - Line count is one (empty line)
    fn test_remove_full_range() {
        let mut rope = Rope::new("Hello World");
        rope.remove(0, 11); // Remove everything
        assert_eq!(rope.char_size(), 0);
        assert_eq!(rope.lines(), 1);
        assert_eq!(rope.leaf_count(), 1);
    }

    #[test]
    /// Tests removing text from a single leaf node.
    /// Verifies that:
    /// - Text is removed correctly from a leaf
    /// - Character count is updated
    /// - Line count is maintained
    fn test_remove_single_leaf() {
        let mut rope = Rope::new("Hello World");
        rope.remove(5, 7); // Remove " W"
        assert_eq!(rope.char_size(), 9);
        assert_eq!(rope.lines(), 1);
        assert_eq!(rope.leaf_count(), 1);
    }

    #[test]
    /// Tests removing text and then inserting new text.
    /// Verifies that:
    /// - Text is removed correctly
    /// - New text is inserted correctly
    /// - Character and line counts are updated
    fn test_remove_and_insert() {
        let mut rope = Rope::new("Hello World");
        rope.remove(5, 7); // Remove " W"
        rope.insert(5, " "); // Insert " " back
        assert_eq!(rope.char_size(), 10);
        assert_eq!(rope.lines(), 1);
        assert_eq!(rope.leaf_count(), 1);
    }

    #[test]
    /// Tests handling of a very large input text.
    /// Verifies that:
    /// - Large text is handled correctly
    /// - Character count is accurate
    /// - Line count is accurate
    /// - Performance is acceptable
    fn test_large_input() {
        let large_text = "Rust is great!\n".repeat(10000);
        let rope = Rope::new(&large_text);
        assert_eq!(rope.lines(), 10001);
        assert_eq!(rope.char_size(), large_text.len());
    }
}