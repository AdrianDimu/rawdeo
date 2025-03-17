use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum RopeNode {
    Leaf(String),
    Internal {
        left: Rc<RefCell<Rope>>,
        right: Rc<RefCell<Rope>>,
        left_size: usize,
    },
}

impl Clone for RopeNode {
    fn clone(&self) -> Self {
        match self {
            RopeNode::Leaf(text) => RopeNode::Leaf(text.clone()),
            RopeNode::Internal { left, right, left_size } => RopeNode::Internal {
                left: Rc::new(RefCell::new(left.borrow().clone())),
                right: Rc::new(RefCell::new(right.borrow().clone())),
                left_size: *left_size,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SplitStrategy {
    LineBased,
    FixedSize(usize),
}

#[derive(Debug, Clone)]
pub struct Rope {
    root: Option<RopeNode>,
    split_strategy: SplitStrategy,
}

impl Rope {
    pub fn new(strategy: SplitStrategy) -> Self {
        Rope { root: None, split_strategy: strategy }
    }

    pub fn from_string(text: &str, strategy: SplitStrategy) -> Self {
        let mut rope = Rope::new(strategy);

        if text.contains('\n') || text.len() > 512 {
            let (left_part, right_part) = rope.split_leaf(text, text.len() / 2);

            rope.root = Some(RopeNode::Internal { 
                left: Rc::new(RefCell::new(Rope::from_string(&left_part, strategy))), 
                right: Rc::new(RefCell::new(Rope::from_string(&right_part, strategy))), 
                left_size: left_part.len(),
             });
        } else {
            rope.root = Some(RopeNode::Leaf(text.to_string()));
        }
        rope
    }

    pub fn insert(&mut self, index: usize, text: &str) {
        match &mut self.root {
            Some(RopeNode::Leaf(existing_text)) => {
                if index > existing_text.len() {
                    panic!("Index out of bounds");
                }

                let new_text = format!(
                    "{}{}{}",
                    &existing_text[..index], text, &existing_text[index..]
                );

                self.root = Some(RopeNode::Leaf(new_text));
            }
            Some(RopeNode::Internal {left, right, left_size }) => {
                if index < *left_size {
                    left.borrow_mut().insert(index, text);
                } else {
                    right.borrow_mut().insert(index - *left_size, text);
                }
            }
            None => {
                self.root = Some(RopeNode::Leaf(text.to_string()));
            }
        }
    }

    pub fn delete(&mut self, start: usize, end: usize) {
        match &mut self.root {
            Some(RopeNode::Leaf(existing_text)) => {
                if start >= existing_text.len() || end > existing_text.len() || start > end {
                    panic!("Invalid delete range")
                }

                let new_text = format!(
                    "{}{}",
                    &existing_text[..start],
                    &existing_text[end..]
                );

                self.root = Some(RopeNode::Leaf(new_text));
            }
            Some(RopeNode::Internal { left, right, left_size }) => {
                if end < *left_size {
                    left.borrow_mut().delete(start, end);
                } else if start >= *left_size {
                    right.borrow_mut().delete(start - *left_size, end - *left_size);
                } else {
                    left.borrow_mut().delete(start, *left_size);
                    right.borrow_mut().delete(0, end - *left_size);
                }
            }
            None => {}
        }
    }

    pub fn get_char(&self, index: usize) -> Option<char> {
        match &self.root {
            Some(RopeNode::Leaf(text)) => text.chars().nth(index),
            Some(RopeNode::Internal { left, right, left_size }) => {
                if index < *left_size {
                    left.borrow().get_char(index)
                } else {
                    right.borrow().get_char(index - left_size)
                }
            }
            None => None,
        }
    }

    pub fn get_line(&self, line_number: usize) -> Option<String> {
        let mut current_line = 0;
        let mut result = String::new();

        self.traverse_lines(line_number, &mut current_line, &mut result);
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }

    fn traverse_lines(&self, target_line: usize, current_line: &mut usize, result: &mut String) {
        if let Some(node) = &self.root {
            match node {
                RopeNode::Leaf(text) => {
                    for line in text.lines() {
                        if *current_line == target_line {
                            result.push_str(line);
                            return;
                        }
                        *current_line += 1;
                    }
                }
                RopeNode::Internal { left, right, .. } => {
                    left.borrow().traverse_lines(target_line, current_line, result);
                    right.borrow().traverse_lines(target_line, current_line, result);
                }
            }
        }
    }

    fn split_leaf(&self, text: &str, index: usize) -> (String, String) {
        match self.split_strategy {
            SplitStrategy::LineBased => {
                let split_index = match text[..index].rfind('\n') {
                    Some(pos) => pos +1,
                    None => index,
                };

                (
                    text[..split_index].to_string(),
                    text[split_index..].to_string(),
                )
            }
            SplitStrategy::FixedSize(max_size) => {
                let split_index = if index > max_size {
                    match text[..max_size].rfind(' ') {
                        Some(pos) => pos +1,
                        None => max_size,
                    }
                } else {
                    index
                };

                (
                    text[..split_index].to_string(),
                    text[split_index..].to_string(),
                )
            }
        }
    }

    pub fn split_at(&mut self, index: usize) -> Rope {
        match &mut self.root.take() {
            Some(RopeNode::Leaf(text)) => {
                let (left_part, right_part) = self.split_leaf(&text, index);

                self.root = Some(RopeNode::Leaf(left_part));
                Rope {
                    root: Some(RopeNode::Leaf(right_part)),
                    split_strategy: self.split_strategy,
                }
            }
            Some(RopeNode::Internal { left, right, left_size }) => {
                if index < *left_size {
                    let new_right = left.borrow_mut().split_at(index);
                    let mut new_rope = Rope::new(self.split_strategy);
                    new_rope.root = Some(RopeNode::Internal {
                        left: Rc::new(RefCell::new(new_right)),
                        right: right.clone(),
                        left_size: index,
                    });
                    new_rope
                } else {
                    right.borrow_mut().split_at(index - *left_size)
                }
            }
            None => Rope::new(self.split_strategy),
        }
    }

    pub fn merge(&mut self, other: Rope) {
        let left_size = self.len();

        let new_left = Rc::new(RefCell::new(self.clone()));
        let new_right = Rc::new(RefCell::new(other));

        self.root = Some(RopeNode::Internal {
            left: new_left,
            right: new_right,
            left_size,
        });
    }

    pub fn len(&self) -> usize {
        match &self.root {
            Some(RopeNode::Leaf(text)) => text.len(),
            Some(RopeNode::Internal { left, right, .. }) => left.borrow().len() + right.borrow().len(),
            None => 0,
        }
    }

    pub fn debug_string(&self) -> String {
        fn traverse(node: &Option<RopeNode>, depth: usize) -> String {
            match node {
                Some(RopeNode::Leaf(text)) => format!("{}Leaf: \"{}\"\n", "  ".repeat(depth), text),
                Some(RopeNode::Internal { left, right, left_size }) => {
                    let left_str = traverse(&left.borrow().root, depth + 1);
                    let right_str = traverse(&right.borrow().root, depth + 1);
                    format!(
                        "{}Internal (left_size = {}):\n{}{}",
                        "  ".repeat(depth),
                        left_size,
                        left_str,
                        right_str
                    )
                }
                None => format!("{}(Empty)\n", "  ".repeat(depth)),
            }
        }
        traverse(&self.root, 0)
    }
}