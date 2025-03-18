use rawdeo::rope::{Rope, SplitStrategy};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_leaf_line_based() {
        let rope = Rope::from_string("Hello\nWorld!\nRust!", SplitStrategy::LineBased);
        let (left, right) = rope.split_leaf("Hello\nWorld!\nRust!", 6);

        assert_eq!(left, "Hello\n");
        assert_eq!(right, "World!\nRust!");
    }

    #[test]
    fn test_split_leaf_fixed_size() {
        let rope = Rope::from_string("Hello wonderful world!", SplitStrategy::FixedSize(10));
        let (left, right) = rope.split_leaf("Hello wonderful world!", 10);

        assert_eq!(left, "Hello "); // Split at nearest space before 10
        assert_eq!(right, "wonderful world!");
    }

    #[test]
    fn test_insert_line_based_creates_internal_node() {
        let mut rope = Rope::from_string("Hello world!", SplitStrategy::LineBased);
        rope.insert(6, "\nThis is Rust!\n");

        let expected_output = "Internal (left_size = 7):\n  Leaf: \"Hello \"\n  Leaf: \"\nThis is Rust!\nworld!\"\n";
        assert_eq!(rope.debug_string(), expected_output);
    }

    #[test]
    fn test_insert_fixed_size_creates_internal_node() {
        let mut rope = Rope::from_string("Hello world!", SplitStrategy::FixedSize(10));
        rope.insert(6, " amazing");

        let expected_output = "Internal (left_size = 10):\n  Leaf: \"Hello \"\n  Leaf: \"amazing world!\"\n";
        assert_eq!(rope.debug_string(), expected_output);
    }

    #[test]
    fn test_delete_after_split() {
        let mut rope = Rope::from_string("Hello\nRust!\nWorld!", SplitStrategy::LineBased);
        rope.insert(6, "\nNew Line!");
        rope.delete(6, 16);

        let expected_output = "Internal (left_size = 6):\n  Leaf: \"Hello\n\"\n  Leaf: \"Rust!\nWorld!\"\n";
        assert_eq!(rope.debug_string(), expected_output);
    }
}
