// A Rope data structure implementation for efficient text manipulation
// The Rope is a binary tree where:
// - Leaf nodes contain actual text segments
// - Internal nodes maintain the structure and track the size of their left subtree
// - Newlines are always at the start of each line (except the first line)

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum RopeNode {
    Internal {
        left: Rc<RefCell<RopeNode>>,
        right: Rc<RefCell<RopeNode>>,
        left_size: usize,  // Size of the left subtree in characters
    },
    Leaf {
        text: String,      // The actual text content
    },
}

impl RopeNode {
    // Returns the total number of characters in this node and its subtrees
    fn char_size(&self) -> usize {
        match self {
            RopeNode::Leaf { text } => text.len(),
            RopeNode::Internal { left: _, right, left_size } => {
                left_size + right.borrow().char_size()
            }
        }
    }

    // Counts the number of lines in the text by counting newlines
    // Each newline character represents a line break, 
    //and we add 1 for the first line that doesn't have a newline at the start
    fn lines(&self) -> usize {
        match self {
            RopeNode::Leaf { text } => {
                if text.is_empty() {
                    1
                } else {
                    // Count newlines and add 1 for the first line
                    text.chars().filter(|&c| c == '\n').count() + 1
                }
            }
            RopeNode::Internal { left, right, .. } => {
                // Combine line counts from both children, subtracting 1 to avoid double-counting
                left.borrow().lines() + right.borrow().lines() - 1
            }
        }
    }

    // Debug helper to print the tree structure
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

    // Counts the number of leaf nodes in the tree
    fn leaf_count(&self) -> usize {
        match self {
            RopeNode::Leaf { .. } => 1,
            RopeNode::Internal { left, right, .. } => {
                left.borrow().leaf_count() + right.borrow().leaf_count()
            }
        }
    }
}

#[derive(Debug)]
pub struct Rope {
    root: Rc<RefCell<RopeNode>>,
}

impl Rope {
    // Helper function to split text into lines with newlines at the start
    fn split_text_into_lines(text: &str) -> Vec<String> {
        if text.is_empty() {
            return vec![String::new()];
        }

        let mut lines: Vec<String> = Vec::new();
        
        // Split on newlines, keeping the newlines with the lines
        let mut parts = text.split_inclusive('\n');
        
        // Handle the first line specially
        if let Some(first) = parts.next() {
            if first.starts_with('\n') {
                lines.push(String::new()); // Empty line before newline
            }
            lines.push(first.to_string());
        }

        // Add remaining lines
        lines.extend(parts.map(|s| s.to_string()));

        // If we have no lines, add an empty line
        if lines.is_empty() {
            lines.push(String::new());
        }

        lines
    }

    // Helper function to build a balanced tree from a vector of nodes
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

    // Creates a new Rope from a string
    // The text is split into lines with newlines at the start of each line (except the first)
    pub fn new(text: &str) -> Self {
        if text.is_empty() {
            return Rope {
                root: Rc::new(RefCell::new(RopeNode::Leaf { text: String::new() }))
            };
        }

        let lines = Rope::split_text_into_lines(text);
        let root = if lines.len() == 1 {
            Rc::new(RefCell::new(RopeNode::Leaf { text: lines[0].clone() }))
        } else {
            Rope::build_balanced_tree(&lines)
        };

        Rope { root }
    }

    // Returns the total number of characters in the Rope
    pub fn char_size(&self) -> usize {
        self.root.borrow().char_size()
    }

    // Returns the number of lines in the Rope
    pub fn lines(&self) -> usize {
        self.root.borrow().lines()
    }

    // Builds a balanced binary tree from a vector of lines
    // The tree is balanced by splitting the lines at the middle
    fn build_balanced_tree(lines: &[String]) -> Rc<RefCell<RopeNode>> {
        if lines.is_empty() {
            return Rc::new(RefCell::new(RopeNode::Leaf { text: String::new() }));
        }
        if lines.len() == 1 {
            return Rc::new(RefCell::new(RopeNode::Leaf {
                text: lines[0].clone(),
            }));
        } 

        let mid = lines.len() / 2;
        let left = Rope::build_balanced_tree(&lines[..mid]);
        let right = Rope::build_balanced_tree(&lines[mid..]);

        Rc::new(RefCell::new(RopeNode::Internal { 
            left: left.clone(), 
            right: right.clone(), 
            left_size: left.borrow().char_size(), 
        }))
    }
    
    // Inserts text at the specified character index
    pub fn insert(&mut self, index: usize, text: &str) {
        let root = self.root.clone();
        self.root = Rope::insert_recursive(root, index, text);            
    }

    // Updates the tree structure after an insertion that involves newlines
    // This function rebuilds the tree to maintain balance
    fn update_balanced_tree(text: &str, index: usize, insert_text: &str) -> Rc<RefCell<RopeNode>> {
        // Combine the text before and after the insertion point
        let mut combined = String::with_capacity(text.len() + insert_text.len());
        combined.push_str(&text[..index]);
        combined.push_str(insert_text);
        combined.push_str(&text[index..]);

        // Split into lines and create leaf nodes
        let lines = Rope::split_text_into_lines(&combined);
        let nodes: Vec<Rc<RefCell<RopeNode>>> = lines.into_iter()
            .map(|line| Rc::new(RefCell::new(RopeNode::Leaf { text: line })))
            .collect();

        // Build and return the balanced tree
        Rope::build_balanced_tree_from_nodes(nodes)
    }

    // Recursively inserts text at the specified index
    // If the insertion involves newlines, the tree is rebalanced
    fn insert_recursive(root: Rc<RefCell<RopeNode>>, index: usize, text: &str) -> Rc<RefCell<RopeNode>> {
        match &*root.borrow() {
            RopeNode::Leaf { text: leaf_text } => {
                if text.contains('\n') || leaf_text.contains('\n') {
                    // If either text contains newlines, rebalance the tree
                    Rope::update_balanced_tree(leaf_text, index, text)
                } else {
                    // No newlines, just insert the text
                    let mut new_text = leaf_text.clone();
                    new_text.insert_str(index, text);
                    Rc::new(RefCell::new(RopeNode::Leaf { text: new_text }))
                }
            }
            RopeNode::Internal { left, right, left_size } => {
                if index <= *left_size {
                    // Insert in the left subtree
                    let new_left = Rope::insert_recursive(left.clone(), index, text);
                    let new_left_size = new_left.borrow().char_size();
                    Rc::new(RefCell::new(RopeNode::Internal {
                        left: new_left,
                        right: right.clone(),
                        left_size: new_left_size,
                    }))
                } else {
                    // Insert in the right subtree
                    let new_right = Rope::insert_recursive(right.clone(), index - left_size, text);
                    Rc::new(RefCell::new(RopeNode::Internal {
                        left: left.clone(),
                        right: new_right,
                        left_size: *left_size,
                    }))
                }
            }
        }
    }

    // Debug helper to print the tree structure
    pub fn print_structure(&self) {
        self.root.borrow().print_structure(0);
    }    

    // Returns the number of leaf nodes in the tree
    pub fn leaf_count(&self) -> usize {
        self.root.borrow().leaf_count()
    }

    // Helper function to join text from two nodes, handling newlines correctly
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

    // Removes a range of characters from the text
    // start: inclusive start index
    // end: exclusive end index
    pub fn remove(&mut self, start: usize, end: usize) {
        if start == end {
            return;
        }

        let root = self.root.clone();
        let new_root = Rope::remove_recursive(root, start, end);
        
        // Collect text and rebuild the tree
        let mut text = String::new();
        let mut stack = vec![new_root];
        while let Some(node) = stack.pop() {
            match &*node.borrow() {
                RopeNode::Leaf { text: leaf_text } => {
                    text.push_str(leaf_text);
                }
                RopeNode::Internal { left, right, .. } => {
                    stack.push(right.clone());
                    stack.push(left.clone());
                }
            }
        }

        // Create a new rope from the collected text
        self.root = Rope::new(&text).root;
    }

    // Recursively removes a range of characters from the text
    // Returns a new tree with the specified range removed
    fn remove_recursive(root: Rc<RefCell<RopeNode>>, start: usize, end: usize) -> Rc<RefCell<RopeNode>> {
        if start == end {
            return root;
        }

        match &*root.borrow() {
            RopeNode::Leaf { text } => {
                // For leaf nodes, create a new string with the range removed
                let mut new_text = String::with_capacity(text.len() - (end - start));
                new_text.push_str(&text[..start]);
                new_text.push_str(&text[end..]);

                // If the resulting text is empty and we're not the only node,
                // we should return an empty node
                if new_text.is_empty() {
                    return Rc::new(RefCell::new(RopeNode::Leaf { text: String::new() }));
                }

                // Create a new leaf node with the modified text
                Rc::new(RefCell::new(RopeNode::Leaf { text: new_text }))
            }
            RopeNode::Internal { left, right, left_size } => {
                if end <= *left_size {
                    // Range is entirely in the left subtree
                    let new_left = Rope::remove_recursive(left.clone(), start, end);
                    let new_left_size = new_left.borrow().char_size();
                    Rc::new(RefCell::new(RopeNode::Internal {
                        left: new_left,
                        right: right.clone(),
                        left_size: new_left_size,
                    }))
                } else if start >= *left_size {
                    // Range is entirely in the right subtree
                    let new_right = Rope::remove_recursive(right.clone(), start - left_size, end - left_size);
                    Rc::new(RefCell::new(RopeNode::Internal {
                        left: left.clone(),
                        right: new_right,
                        left_size: *left_size,
                    }))
                } else {
                    // Range spans both subtrees
                    // Remove the end portion from the left subtree
                    let new_left = Rope::remove_recursive(left.clone(), start, *left_size);
                    // Remove the start portion from the right subtree
                    let new_right = Rope::remove_recursive(right.clone(), 0, end - left_size);
                    
                    // If either subtree is empty after removal, return the other one
                    if new_left.borrow().char_size() == 0 {
                        return new_right;
                    }
                    if new_right.borrow().char_size() == 0 {
                        return new_left;
                    }

                    // Try to get text from both nodes
                    let mut combined_text = None;
                    {
                        let left_node = new_left.borrow();
                        let right_node = new_right.borrow();
                        if let (RopeNode::Leaf { text: left_text }, RopeNode::Leaf { text: right_text }) = (&*left_node, &*right_node) {
                            combined_text = Some(Rope::join_text(left_text, right_text));
                        }
                    }

                    if let Some(combined) = combined_text {
                        // Create a new leaf node with the combined text
                        Rc::new(RefCell::new(RopeNode::Leaf { text: combined }))
                    } else {
                        // Not both leaves, combine as internal node
                        let new_left_size = new_left.borrow().char_size();
                        Rc::new(RefCell::new(RopeNode::Internal {
                            left: new_left,
                            right: new_right,
                            left_size: new_left_size,
                        }))
                    }
                }
            }
        }
    }
}
