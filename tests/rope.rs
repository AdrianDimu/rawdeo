use rawdeo::rope::{Rope, SplitStrategy};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_rope() {
        let rope = Rope::from_string("Hello, world!", SplitStrategy::LineBased);
        assert_eq!(rope.len(), 13);
        assert_eq!(rope.debug_string(), "Leaf: \"Hello, world!\"\n");
    }

    #[test]
    fn test_insert_into_leaf() {
        let mut rope = Rope::from_string("Hello world!", SplitStrategy::LineBased);
        rope.insert(6, "amazing ");

        let expected_output = "Leaf: \"Hello amazing world!\"\n";
        assert_eq!(rope.debug_string(), expected_output);
    }

    #[test]
    fn test_delete_from_leaf() {
        let mut rope = Rope::from_string("Hello amazing world!", SplitStrategy::LineBased);
        rope.delete(6, 14);

        let expected_output = "Leaf: \"Hello world!\"\n";
        assert_eq!(rope.debug_string(), expected_output);
    }

    #[test]
    fn test_get_char() {
        let rope = Rope::from_string("Hello, world!", SplitStrategy::LineBased);
        assert_eq!(rope.get_char(7), Some('w'));
    }

    #[test]
    fn test_split_at() {
        let mut rope = Rope::from_string("Hello, world!", SplitStrategy::LineBased);
        let right_part = rope.split_at(7);

        let expected_left = "Leaf: \"Hello, \"\n";
        let expected_right = "Leaf: \"world!\"\n";

        assert_eq!(rope.debug_string(), expected_left);
        assert_eq!(right_part.debug_string(), expected_right);
    }

    #[test]
    fn test_merge() {
        let mut rope1 = Rope::from_string("Hello, ", SplitStrategy::LineBased);
        let rope2 = Rope::from_string("world!", SplitStrategy::LineBased);
        rope1.merge(rope2);

        let expected_output = "Internal (left_size = 7):\n  Leaf: \"Hello, \"\n  Leaf: \"world!\"\n";
        assert_eq!(rope1.debug_string(), expected_output);
    }


    #[test]
    fn test_insert_and_delete_inside_leaf() {
        let mut rope = Rope::from_string("Hello world!", SplitStrategy::LineBased);
        rope.insert(6, "amazing ");
        rope.delete(6, 14); // Remove "amazing "

        let expected_output = "Leaf: \"Hello world!\"\n";
        assert_eq!(rope.debug_string(), expected_output);
    }

    #[test]
    fn test_insert_creates_new_leaves() {
        let mut rope = Rope::from_string("Hello world!", SplitStrategy::LineBased);
        rope.insert(6, "\nThis is Rust!\n");

        let expected_output = "Internal (left_size = 7):\n  Leaf: \"Hello \"\n  Leaf: \"\nThis is Rust!\nworld!\"\n";
        assert_eq!(rope.debug_string(), expected_output);
    }

    #[test]
    fn test_insert_and_delete_entire_leaf() {
        let mut rope = Rope::from_string("Hello\nRust!\nWorld!", SplitStrategy::LineBased);
        rope.insert(6, "\nAmazing ");
        rope.delete(6, 15); // Remove "Amazing "

        let expected_output = "Internal (left_size = 6):\n  Leaf: \"Hello\n\"\n  Leaf: \"Rust!\nWorld!\"\n";
        assert_eq!(rope.debug_string(), expected_output);
    }

    #[test]
    fn test_insert_fixed_size_splitting() {
        let mut rope = Rope::from_string("Hello world!", SplitStrategy::FixedSize(10));
        rope.insert(6, " amazing"); // Causes split due to max 10 chars

        let expected_output = "Internal (left_size = 10):\n  Leaf: \"Hello \"\n  Leaf: \"amazing world!\"\n";
        assert_eq!(rope.debug_string(), expected_output);
    }

    #[test]
    fn test_delete_across_internal_nodes() {
        let mut rope = Rope::from_string("Hello\nRust!\nWorld!", SplitStrategy::LineBased);
        rope.insert(6, "\nNew Line!");
        rope.delete(6, 16); // Remove the newly inserted "New Line!"

        let expected_output = "Internal (left_size = 6):\n  Leaf: \"Hello\n\"\n  Leaf: \"Rust!\nWorld!\"\n";
        assert_eq!(rope.debug_string(), expected_output);
    }

    #[test]
    fn test_insert_delete_mixed_operations() {
        let mut rope = Rope::from_string("Hello, world!", SplitStrategy::LineBased);
        rope.insert(5, " wonderful");
        rope.insert(13, "\nNew Line!\n");
        rope.delete(5, 16); // Delete " wonderful"
        rope.insert(0, "Start: ");
        rope.delete(0, 7); // Delete "Start: "

        let expected_output = "Internal (left_size = 13):\n  Leaf: \"Hello, world!\"\n  Leaf: \"\nNew Line!\n\"\n";
        assert_eq!(rope.debug_string(), expected_output);
    }

}
