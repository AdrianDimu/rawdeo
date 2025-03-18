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
            RopeNode::Internal { left, right, left_size } => {
                left_size + right.borrow().char_size()
            }
        }
    }

    fn lines(&self) -> usize {
        match self {
            RopeNode::Leaf { .. } => 1,
            RopeNode::Internal { left, right, .. } => {
                left.borrow().lines() + right.borrow().lines()
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
        let lines: Vec<&str> = if text.is_empty() {
         vec![""]   
        } else {
            text.lines().collect()
        };

        let root = if lines.len() == 1 {
            Rc::new(RefCell::new(RopeNode::Leaf { 
                text: lines[0].to_string(), 
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

    fn build_balanced_tree(lines: &[&str]) -> Rc<RefCell<RopeNode>> {
        if lines.len() == 1 {
            Rc::new(RefCell::new(RopeNode::Leaf {
                text: lines[0].to_string(),
            }))
        } else {
            let mid = lines.len() / 2;
            let left = Rope::build_balanced_tree(&lines[..mid]);
            let right = Rope::build_balanced_tree(&lines[mid..]);

            Rc::new(RefCell::new(RopeNode::Internal { 
                left: left.clone(), 
                right: right.clone(), 
                left_size: left.borrow().char_size(), 
            }))
        }
    }
}