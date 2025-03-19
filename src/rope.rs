/// A Rope data structure implementation for efficient text manipulation.
/// 
/// The Rope is implemented as a binary tree where:
/// - Leaf nodes contain actual text segments
/// - Internal nodes maintain metadata about their subtrees
/// - Each line (except the first) starts with a newline character
/// - The tree is kept balanced for optimal performance
/// 
/// Key features:
/// - Efficient insert and delete operations: O(log n)
/// - Line-aware text manipulation
/// - Memory efficient for large texts
/// - Automatic tree balancing

use std::rc::Rc;
use std::cell::RefCell;
use crate::undo::{UndoStack, UndoAction};

/// A node in the Rope data structure.
/// Can be either an Internal node (with left and right children) or a Leaf node (containing text).
#[derive(Debug)]
enum RopeNode {
    /// Internal node that maintains tree structure and metadata
    Internal {
        /// Left child node
        left: Rc<RefCell<RopeNode>>,
        /// Right child node
        right: Rc<RefCell<RopeNode>>,
        /// Number of characters in the left subtree
        left_size: usize,
    },
    /// Leaf node containing actual text content
    Leaf {
        /// The text segment stored in this leaf
        text: String,
    },
}

impl RopeNode {
    /// Returns the total number of characters in this node and its subtrees.
    /// For leaf nodes, this is the length of the text.
    /// For internal nodes, this is the sum of characters in both subtrees.
    fn char_size(&self) -> usize {
        match self {
            RopeNode::Leaf { text } => text.chars().count(),
            RopeNode::Internal { left: _, right, left_size } => {
                left_size + right.borrow().char_size()
            }
        }
    }

    /// Counts the number of lines in the text by counting newlines.
    /// Each newline character represents a line break, and we add 1 for
    /// the first line that doesn't have a newline at the start.
    fn lines(&self) -> usize {
        match self {
            RopeNode::Leaf { text } => {
                if text.is_empty() {
                    1
                } else {
                    text.chars().filter(|&c| c == '\n').count() + 1
                }
            }
            RopeNode::Internal { left, right, .. } => {
                left.borrow().lines() + right.borrow().lines() - 1
            }
        }
    }

    /// Debug helper to print the tree structure with indentation
    /// showing the hierarchy of nodes.
    fn print_structure(&self, depth: usize) {
        let indent = "-".repeat(depth);
        match self {
            RopeNode::Leaf { text } => {
                println!("{}Leaf: {:?}", indent, text);
            }
            RopeNode::Internal { left, right, left_size } => {
                println!("{}Internal: left_size = {}", indent, left_size);
                left.borrow().print_structure(depth + 1);
                right.borrow().print_structure(depth + 1);
            }
        }
    }

    /// Counts the total number of leaf nodes in the tree.
    /// Used for debugging and verifying tree structure.
    fn leaf_count(&self) -> usize {
        match self {
            RopeNode::Leaf { .. } => 1,
            RopeNode::Internal { left, right, .. } => {
                left.borrow().leaf_count() + right.borrow().leaf_count()
            }
        }
    }

    /// Returns the complete text content of this node and its subtrees.
    /// For internal nodes, concatenates text from left and right children.
    fn text(&self) -> String {
        match self {
            RopeNode::Leaf { text } => text.clone(),
            RopeNode::Internal { left, right, .. } => {
                let mut result = String::new();
                result.push_str(&left.borrow().text());
                result.push_str(&right.borrow().text());
                result
            }
        }
    }

    /// Returns a substring of the text content within the specified range.
    /// 
    /// # Arguments
    /// * `start` - Starting index (inclusive)
    /// * `end` - Ending index (exclusive)
    /// 
    /// # Returns
    /// The text content within the specified range
    fn text_range(&self, start: usize, end: usize) -> String {
        match self {
            RopeNode::Leaf { text } => {
                if start >= text.chars().count() || end > text.chars().count() || start > end {
                    String::new()
                } else {
                    // Convert byte indices to char indices
                    let mut char_indices = text.char_indices();
                    let start_byte = char_indices
                        .clone()
                        .nth(start)
                        .map_or(text.len(), |(i, _)| i);
                    let end_byte = char_indices
                        .nth(end - 1)
                        .map_or(text.len(), |(i, c)| i + c.len_utf8());
                    text[start_byte..end_byte].to_string()
                }
            }
            RopeNode::Internal { left, right, left_size } => {
                if end <= *left_size {
                    left.borrow().text_range(start, end)
                } else if start >= *left_size {
                    right.borrow().text_range(start - left_size, end - left_size)
                } else {
                    let mut result = String::new();
                    result.push_str(&left.borrow().text_range(start, *left_size));
                    result.push_str(&right.borrow().text_range(0, end - left_size));
                    result
                }
            }
        }
    }
}

/// A Rope data structure for efficient text manipulation.
/// Maintains text as a balanced tree of text segments for optimal
/// insert and delete operations.
#[derive(Debug)]
pub struct Rope {
    /// The root node of the rope tree. None represents an empty rope.
    root: Option<Rc<RefCell<RopeNode>>>,
    /// The undo stack for tracking changes
    undo_stack: UndoStack,
}

impl Rope {
    /// Splits input text into lines, creating leaf nodes.
    /// Each line (except the first) starts with its newline character.
    /// 
    /// # Arguments
    /// * `text` - The input text to split into lines
    /// 
    /// # Returns
    /// A vector of RopeNodes, each containing a line of text
    fn split_text_into_lines(text: &str) -> Vec<Rc<RefCell<RopeNode>>> {
        if text.is_empty() {
            return vec![Rc::new(RefCell::new(RopeNode::Leaf { text: String::new() }))];
        }

        let mut lines = Vec::new();
        let mut current_line = String::new();
        let mut chars = text.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '\n' {
                // Add the current line with the newline
                current_line.push(c);
                lines.push(Rc::new(RefCell::new(RopeNode::Leaf { text: current_line.clone() })));
                current_line.clear();

                // If this is the last character, add an empty line
                if chars.peek().is_none() {
                    lines.push(Rc::new(RefCell::new(RopeNode::Leaf { text: String::new() })));
                }
            } else {
                current_line.push(c);
                // If this is the last character and it's not a newline, add the line
                if chars.peek().is_none() {
                    lines.push(Rc::new(RefCell::new(RopeNode::Leaf { text: current_line.clone() })));
                }
            }
        }

        // If we have no lines, add an empty line
        if lines.is_empty() {
            lines.push(Rc::new(RefCell::new(RopeNode::Leaf { text: String::new() })));
        }

        lines
    }

    /// Builds a balanced binary tree from a vector of nodes.
    /// Used when rebuilding parts of the tree after modifications.
    /// 
    /// # Arguments
    /// * `nodes` - Vector of RopeNodes to build into a tree
    /// 
    /// # Returns
    /// The root node of the balanced tree
    fn build_balanced_tree_from_nodes(mut nodes: Vec<Rc<RefCell<RopeNode>>>) -> Rc<RefCell<RopeNode>> {
        // If we have no nodes (empty document), add an empty leaf
        if nodes.is_empty() {
            return Rc::new(RefCell::new(RopeNode::Leaf { text: String::new() }));
        }

        // Build a balanced tree by combining nodes pairwise
        while nodes.len() > 1 {
            let mut new_nodes = Vec::new();
            let mut i = 0;
            while i < nodes.len() {
                if i + 1 < nodes.len() {
                    let left = nodes[i].clone();
                    let right = nodes[i + 1].clone();
                    let left_size = left.borrow().char_size();
                    new_nodes.push(Rc::new(RefCell::new(RopeNode::Internal {
                        left_size,
                        left,
                        right,
                    })));
                    i += 2;
                } else {
                    new_nodes.push(nodes[i].clone());
                    i += 1;
                }
            }
            nodes = new_nodes;
        }

        nodes.pop().unwrap_or_else(|| Rc::new(RefCell::new(RopeNode::Leaf { text: String::new() })))
    }

    /// Creates a new Rope from input text.
    /// The text is split into lines and organized into a balanced tree.
    /// 
    /// # Arguments
    /// * `text` - The input text to create the rope from
    /// 
    /// # Returns
    /// A new Rope instance containing the text
    pub fn new(text: &str) -> Self {
        let mut rope = Rope { 
            root: None,
            undo_stack: UndoStack::new(),
        };
        if !text.is_empty() {
            let nodes = Rope::split_text_into_lines(text);
            rope.root = Some(Rope::build_balanced_tree(&nodes));
        } else {
            rope.root = Some(Rc::new(RefCell::new(RopeNode::Leaf {
                text: String::new(),
            })));
        }
        rope
    }

    /// Returns the total number of characters in the Rope.
    /// 
    /// # Returns
    /// The total character count, or 0 for an empty rope
    pub fn char_size(&self) -> usize {
        self.root.as_ref().map_or(0, |root| root.borrow().char_size())
    }

    /// Returns the number of lines in the Rope.
    /// A line is defined as text ending with a newline,
    /// plus one for the last line if it doesn't end with a newline.
    /// 
    /// # Returns
    /// The total line count, or 0 for an empty rope
    pub fn lines(&self) -> usize {
        self.root.as_ref().map_or(0, |root| root.borrow().lines())
    }

    /// Builds a balanced binary tree from a slice of nodes.
    /// Used during tree rebalancing operations.
    /// 
    /// # Arguments
    /// * `nodes` - Slice of RopeNodes to build into a tree
    /// 
    /// # Returns
    /// The root node of the balanced tree
    fn build_balanced_tree(nodes: &[Rc<RefCell<RopeNode>>]) -> Rc<RefCell<RopeNode>> {
        if nodes.is_empty() {
            return Rc::new(RefCell::new(RopeNode::Leaf { text: String::new() }));
        }
        if nodes.len() == 1 {
            return nodes[0].clone();
        }

        let mid = nodes.len() / 2;
        let left = Rope::build_balanced_tree(&nodes[..mid]);
        let right = Rope::build_balanced_tree(&nodes[mid..]);
        let left_size = left.borrow().char_size();

        Rc::new(RefCell::new(RopeNode::Internal {
            left_size,
            left,
            right,
        }))
    }
    
    /// Inserts text at the specified character index without recording an undo action.
    /// This is used internally by the undo system.
    /// 
    /// # Arguments
    /// * `index` - The character position to insert at
    /// * `text` - The text to insert
    pub(crate) fn insert_without_undo(&mut self, index: usize, text: &str) {
        if text.is_empty() {
            return;
        }

        let old_lines = self.lines();
        let root = self.root.take().unwrap_or_else(|| {
            Rc::new(RefCell::new(RopeNode::Leaf {
                text: String::new(),
            }))
        });

        let new_root = if text.contains('\n') {
            // Get the text before and after the insertion point
            let before = root.borrow().text_range(0, index);
            let after = root.borrow().text_range(index, root.borrow().char_size());

            // Combine the text with the inserted text
            let mut combined = before;
            combined.push_str(text);
            combined.push_str(&after);

            // Split into lines and create a new tree
            let mut nodes = Rope::split_text_into_lines(&combined);
            if nodes.len() == 1 {
                nodes.pop().unwrap()
            } else {
                let mut current = nodes.pop().unwrap();
                while let Some(node) = nodes.pop() {
                    let left_size = node.borrow().char_size();
                    current = Rc::new(RefCell::new(RopeNode::Internal {
                        left_size,
                        left: node,
                        right: current,
                    }));
                }
                current
            }
        } else {
            Self::insert_recursive(root, index, text)
        };

        self.root = Some(new_root);
        let new_lines = self.lines();
        if old_lines != new_lines {
            self.rebalance();
        }
    }

    /// Recursively inserts text at the specified index.
    /// Handles both simple insertions and those involving newlines.
    /// 
    /// # Arguments
    /// * `root` - The root node to insert into
    /// * `index` - The insertion point
    /// * `text` - The text to insert
    /// 
    /// # Returns
    /// The new root node after insertion
    fn insert_recursive(root: Rc<RefCell<RopeNode>>, index: usize, text: &str) -> Rc<RefCell<RopeNode>> {
        match &*root.borrow() {
            RopeNode::Leaf { text: leaf_text } => {
                // If the text contains newlines, we need to split it into lines
                if text.contains('\n') {
                    let mut combined = String::with_capacity(leaf_text.len() + text.len());
                    // Convert character index to byte index for the leaf text
                    let byte_index = leaf_text.char_indices()
                        .nth(index)
                        .map_or(leaf_text.len(), |(i, _)| i);
                    combined.push_str(&leaf_text[..byte_index]);
                    combined.push_str(text);
                    combined.push_str(&leaf_text[byte_index..]);
                    
                    // Split into lines and create a balanced tree
                    let lines = Rope::split_text_into_lines(&combined);
                    Rope::build_balanced_tree(&lines)
                } else {
                    // If no newlines, just insert the text into the current leaf
                    let mut new_text = String::with_capacity(leaf_text.len() + text.len());
                    // Convert character index to byte index for the leaf text
                    let byte_index = leaf_text.char_indices()
                        .nth(index)
                        .map_or(leaf_text.len(), |(i, _)| i);
                    new_text.push_str(&leaf_text[..byte_index]);
                    new_text.push_str(text);
                    new_text.push_str(&leaf_text[byte_index..]);
                    Rc::new(RefCell::new(RopeNode::Leaf { text: new_text }))
                }
            }
            RopeNode::Internal { left_size, left, right } => {
                if index <= *left_size {
                    let new_left = Self::insert_recursive(left.clone(), index, text);
                    Rc::new(RefCell::new(RopeNode::Internal {
                        left_size: left_size + text.chars().count(),
                        left: new_left,
                        right: right.clone(),
                    }))
                } else {
                    let new_right = Self::insert_recursive(right.clone(), index - left_size, text);
                    Rc::new(RefCell::new(RopeNode::Internal {
                        left_size: *left_size,
                        left: left.clone(),
                        right: new_right,
                    }))
                }
            }
        }
    }

    /// Removes a range of characters from the text without recording an undo action.
    /// This is used internally by the undo system.
    /// 
    /// # Arguments
    /// * `start` - Starting index (inclusive)
    /// * `end` - Ending index (exclusive)
    pub(crate) fn remove_without_undo(&mut self, start: usize, end: usize) {
        if start >= end {
            return;
        }

        let old_lines = self.lines();
        let root = self.root.take().unwrap_or_else(|| {
            Rc::new(RefCell::new(RopeNode::Leaf {
                text: String::new(),
            }))
        });

        let new_root = if root.borrow().text_range(start, end).contains('\n') {
            // Get the text before and after the removal range
            let before = root.borrow().text_range(0, start);
            let after = root.borrow().text_range(end, root.borrow().char_size());

            // Combine the text
            let mut combined = before;
            combined.push_str(&after);

            // Split into lines and create a new tree
            let mut nodes = Rope::split_text_into_lines(&combined);
            if nodes.len() == 1 {
                nodes.pop().unwrap()
            } else {
                let mut current = nodes.pop().unwrap();
                while let Some(node) = nodes.pop() {
                    let left_size = node.borrow().char_size();
                    current = Rc::new(RefCell::new(RopeNode::Internal {
                        left_size,
                        left: node,
                        right: current,
                    }));
                }
                current
            }
        } else {
            Self::remove_recursive(root, start, end)
        };

        self.root = Some(new_root);
        let new_lines = self.lines();
        if old_lines != new_lines {
            self.rebalance();
        }
    }

    /// Recursively removes a range of characters from the text.
    /// Handles both simple removals and those involving newlines.
    /// 
    /// # Arguments
    /// * `root` - The root node to remove from
    /// * `start` - Starting index (inclusive)
    /// * `end` - Ending index (exclusive)
    /// 
    /// # Returns
    /// The new root node after removal
    fn remove_recursive(root: Rc<RefCell<RopeNode>>, start: usize, end: usize) -> Rc<RefCell<RopeNode>> {
        match &*root.borrow() {
            RopeNode::Leaf { text } => {
                let mut new_text = text[..start].to_string();
                new_text.push_str(&text[end..]);
                if new_text.is_empty() {
                    Rc::new(RefCell::new(RopeNode::Leaf { text: String::new() }))
                } else {
                    // Split the text into lines and create a new node for each line
                    let nodes = Rope::split_text_into_lines(&new_text);
                    Rope::build_balanced_tree(&nodes)
                }
            }
            RopeNode::Internal { left_size, left, right } => {
                if end <= *left_size {
                    let new_left = Self::remove_recursive(left.clone(), start, end);
                    if new_left.borrow().char_size() == 0 {
                        right.clone()
                    } else {
                        Rc::new(RefCell::new(RopeNode::Internal {
                            left_size: left_size - (end - start),
                            left: new_left,
                            right: right.clone(),
                        }))
                    }
                } else if start >= *left_size {
                    let new_right = Self::remove_recursive(right.clone(), start - left_size, end - left_size);
                    if new_right.borrow().char_size() == 0 {
                        left.clone()
                    } else {
                        Rc::new(RefCell::new(RopeNode::Internal {
                            left_size: *left_size,
                            left: left.clone(),
                            right: new_right,
                        }))
                    }
                } else {
                    let new_left = Self::remove_recursive(left.clone(), start, *left_size);
                    let new_right = Self::remove_recursive(right.clone(), 0, end - left_size);
                    if new_left.borrow().char_size() == 0 {
                        new_right
                    } else if new_right.borrow().char_size() == 0 {
                        new_left
                    } else {
                        // Get the text from both nodes and combine them
                        let left_text = new_left.borrow().text_range(0, new_left.borrow().char_size());
                        let right_text = new_right.borrow().text_range(0, new_right.borrow().char_size());
                        let mut combined = left_text;
                        combined.push_str(&right_text);

                        // Split into lines and create a new tree
                        let nodes = Rope::split_text_into_lines(&combined);
                        Rope::build_balanced_tree(&nodes)
                    }
                }
            }
        }
    }

    /// Inserts text at the specified character index.
    /// If the text contains newlines, the tree is rebuilt to maintain
    /// the line-based structure.
    /// 
    /// # Arguments
    /// * `index` - The character position to insert at
    /// * `text` - The text to insert
    pub fn insert(&mut self, index: usize, text: &str) {
        if text.is_empty() {
            return;
        }

        // Record the action for undo
        self.undo_stack.push(UndoAction::Insert {
            index,
            text: text.to_string(),
        });

        self.insert_without_undo(index, text);
    }

    /// Removes a range of characters from the text.
    /// If the range contains newlines, rebuilds the affected portion
    /// of the tree.
    /// 
    /// # Arguments
    /// * `start` - Starting index (inclusive)
    /// * `end` - Ending index (exclusive)
    pub fn remove(&mut self, start: usize, end: usize) {
        if start >= end {
            return;
        }

        // Get the text that will be deleted before removing it
        let deleted_text = self.text_range(start, end);

        // Record the action for undo
        self.undo_stack.push(UndoAction::Delete {
            index: start,
            text: deleted_text,
        });

        self.remove_without_undo(start, end);
    }

    /// Debug helper to print the entire tree structure.
    /// Useful for visualizing the rope's internal organization.
    pub fn print_structure(&self) {
        self.root.as_ref().map(|root| root.borrow().print_structure(0));
    }    

    /// Returns the number of leaf nodes in the tree.
    /// Used for debugging and verifying tree structure.
    /// 
    /// # Returns
    /// The total number of leaf nodes
    pub fn leaf_count(&self) -> usize {
        self.root.as_ref().map_or(0, |root| root.borrow().leaf_count())
    }

    /// Helper function to join text from two nodes.
    /// Handles special cases with newlines at the boundary.
    /// 
    /// # Arguments
    /// * `left` - The left text to join
    /// * `right` - The right text to join
    /// 
    /// # Returns
    /// The combined text with proper newline handling
    fn join_text(left: &str, right: &str) -> String {
        // If either text is empty, return the other one
        if left.is_empty() {
            return right.to_string();
        }
        if right.is_empty() {
            return left.to_string();
        }

        let mut result = String::with_capacity(left.len() + right.len());
        
        // If both sides have a newline at the boundary, remove one of them
        if left.ends_with('\n') && right.starts_with('\n') {
            result.push_str(&left[..left.len()-1]);
            result.push_str(right);
        } else {
            result.push_str(left);
            result.push_str(right);
        }
        
        result
    }

    /// Rebalances the entire tree by collecting all text and
    /// rebuilding with optimal balance.
    /// Called after operations that may unbalance the tree.
    fn rebalance(&mut self) {
        // Collect all text from the rope
        let mut text = String::new();
        let mut stack = vec![self.root.clone()];
        while let Some(Some(node)) = stack.pop() {
            match &*node.borrow() {
                RopeNode::Leaf { text: leaf_text } => {
                    text.push_str(&leaf_text);
                }
                RopeNode::Internal { left, right, .. } => {
                    stack.push(Some(right.clone()));
                    stack.push(Some(left.clone()));
                }
            }
        }

        // Split into lines and create a balanced tree
        let nodes = Rope::split_text_into_lines(&text);
        self.root = Some(Rope::build_balanced_tree(&nodes));
    }

    /// Returns the text content within the specified range.
    /// 
    /// # Arguments
    /// * `start` - Starting index (inclusive)
    /// * `end` - Ending index (exclusive)
    /// 
    /// # Returns
    /// The text content within the specified range
    pub fn text_range(&self, start: usize, end: usize) -> String {
        self.root.as_ref().map_or(String::new(), |root| root.borrow().text_range(start, end))
    }

    /// Undoes the last action performed on this rope.
    /// 
    /// # Returns
    /// true if an action was undone, false if there were no actions to undo
    pub fn undo(&mut self) -> bool {
        if let Some(action) = self.undo_stack.pop() {
            UndoStack::apply_action(action, self);
            true
        } else {
            false
        }
    }

    /// Returns true if there are actions that can be undone.
    /// 
    /// # Returns
    /// true if there are actions in the undo stack
    pub fn can_undo(&self) -> bool {
        self.undo_stack.can_undo()
    }
}
