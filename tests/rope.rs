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
}
