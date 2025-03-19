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
                    text.chars().filter(|&c| c == '\n').count() + 1
                }
            }
            RopeNode::Internal { left, right, .. } => {
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

    fn text_range(&self, start: usize, end: usize) -> String {
        match self {
            RopeNode::Leaf { text } => {
                if start >= text.len() || end > text.len() || start > end {
                    String::new()
                } else {
                    text[start..end].to_string()
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

#[derive(Debug)]
pub struct Rope {
    root: Option<Rc<RefCell<RopeNode>>>,
}

impl Rope {
    // Helper function to split text into lines with newlines at the start
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
        let mut rope = Rope { root: None };
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

    // Returns the total number of characters in the Rope
    pub fn char_size(&self) -> usize {
        self.root.as_ref().map_or(0, |root| root.borrow().char_size())
    }

    // Returns the number of lines in the Rope
    pub fn lines(&self) -> usize {
        self.root.as_ref().map_or(0, |root| root.borrow().lines())
    }

    // Builds a balanced binary tree from a vector of lines
    // The tree is balanced by splitting the lines at the middle
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
    
    // Inserts text at the specified character index
    pub fn insert(&mut self, index: usize, text: &str) {
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
            Rope::insert_recursive(root, index, text)
        };

        self.root = Some(new_root);
        let new_lines = self.lines();
        if old_lines != new_lines {
            self.rebalance();
        }
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
        let mut nodes = Vec::new();
        let mut current_line = String::new();

        for c in combined.chars() {
            if c == '\n' {
                if !current_line.is_empty() {
                    nodes.push(Rc::new(RefCell::new(RopeNode::Leaf { text: current_line })));
                    current_line = String::new();
                }
                nodes.push(Rc::new(RefCell::new(RopeNode::Leaf { text: "\n".to_string() })));
            } else {
                current_line.push(c);
            }
        }

        // Add the last line if it's not empty
        if !current_line.is_empty() {
            nodes.push(Rc::new(RefCell::new(RopeNode::Leaf { text: current_line })));
        }

        // If no nodes were added, add an empty leaf
        if nodes.is_empty() {
            nodes.push(Rc::new(RefCell::new(RopeNode::Leaf { text: String::new() })));
        }

        // Build and return the balanced tree
        Rope::build_balanced_tree_from_nodes(nodes)
    }

    // Recursively inserts text at the specified index
    // If the insertion involves newlines, the tree is rebalanced
    fn insert_recursive(root: Rc<RefCell<RopeNode>>, index: usize, text: &str) -> Rc<RefCell<RopeNode>> {
        match &*root.borrow() {
            RopeNode::Leaf { text: leaf_text } => {
                // If the text contains newlines, we need to split it into lines
                if text.contains('\n') {
                    let mut combined = String::with_capacity(leaf_text.len() + text.len());
                    combined.push_str(&leaf_text[..index]);
                    combined.push_str(text);
                    combined.push_str(&leaf_text[index..]);
                    
                    // Split into lines and create a balanced tree
                    let lines = Rope::split_text_into_lines(&combined);
                    Rope::build_balanced_tree(&lines)
                } else {
                    // If no newlines, just insert the text into the current leaf
                    let mut new_text = String::with_capacity(leaf_text.len() + text.len());
                    new_text.push_str(&leaf_text[..index]);
                    new_text.push_str(text);
                    new_text.push_str(&leaf_text[index..]);
                    Rc::new(RefCell::new(RopeNode::Leaf { text: new_text }))
                }
            }
            RopeNode::Internal { left_size, left, right } => {
                if index <= *left_size {
                    let new_left = Rope::insert_recursive(left.clone(), index, text);
                    Rc::new(RefCell::new(RopeNode::Internal {
                        left_size: left_size + text.chars().count(),
                        left: new_left,
                        right: right.clone(),
                    }))
                } else {
                    let new_right = Rope::insert_recursive(right.clone(), index - left_size, text);
                    Rc::new(RefCell::new(RopeNode::Internal {
                        left_size: *left_size,
                        left: left.clone(),
                        right: new_right,
                    }))
                }
            }
        }
    }

    // Debug helper to print the tree structure
    pub fn print_structure(&self) {
        self.root.as_ref().map(|root| root.borrow().print_structure(0));
    }    

    // Returns the number of leaf nodes in the tree
    pub fn leaf_count(&self) -> usize {
        self.root.as_ref().map_or(0, |root| root.borrow().leaf_count())
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
            Rope::remove_recursive(root, start, end)
        };

        self.root = Some(new_root);
        let new_lines = self.lines();
        if old_lines != new_lines {
            self.rebalance();
        }
    }

    // Recursively removes a range of characters from the text
    // Returns a new tree with the specified range removed
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
                    let new_left = Rope::remove_recursive(left.clone(), start, end);
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
                    let new_right = Rope::remove_recursive(right.clone(), start - left_size, end - left_size);
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
                    let new_left = Rope::remove_recursive(left.clone(), start, *left_size);
                    let new_right = Rope::remove_recursive(right.clone(), 0, end - left_size);
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
}
