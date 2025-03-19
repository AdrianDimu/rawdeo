use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum RopeNode {
    Internal {
        left: Rc<RefCell<RopeNode>>,
        right: Rc<RefCell<RopeNode>>,
        left_size: usize,
    },
    Leaf {
        text: String,
    },
}

impl RopeNode {
    fn char_size(&self) -> usize {
        match self {
            RopeNode::Leaf { text } => text.len(),
            RopeNode::Internal { left: _, right, left_size } => {
                left_size + right.borrow().char_size()
            }
        }
    }

    // Count actual newlines in the text, regardless of how the rope is structured
    fn lines(&self) -> usize {
        match self {
            RopeNode::Leaf { text } => {
                // Count newlines in the text, adding 1 for the final line
                text.chars().filter(|&c| c == '\n').count() + 1
            }
            RopeNode::Internal { left, right, .. } => {
                // For internal nodes, combine the line counts from both children
                // Subtract 1 from the right count to avoid double-counting the line break
                left.borrow().lines() + right.borrow().lines() - 1
            }
        }
    }

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
}

#[derive(Debug)]
pub struct Rope {
    root: Rc<RefCell<RopeNode>>,
}

impl Rope {
    pub fn new(text: &str) -> Self {
        let mut lines: Vec<String> = if text.is_empty() {
         vec![String::new()]   
        } else {
            text.split_inclusive('\n').map(|s| s.to_string()).collect()
        };

        if text.ends_with('\n') {
            lines.push(String::new())
        }

        let root = if lines.len() == 1 {
            Rc::new(RefCell::new(RopeNode::Leaf { 
                text: lines[0].clone(), 
            }))
        } else {
            Rope::build_balanced_tree(&lines)
        };

        Rope { root }
    }

    pub fn char_size(&self) -> usize {
            self.root.borrow().char_size()
        }

    pub fn lines(&self) -> usize {
        self.root.borrow().lines()
    }

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

    pub fn print_structure(&self) {
        self.root.borrow().print_structure(0);
    }    
}
