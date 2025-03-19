use std::rc::Rc;
use crate::rope::Rope;

/// Represents a single action that can be undone
#[derive(Debug)]
pub enum UndoAction {
    /// An insertion action that can be undone by removing the inserted text
    Insert {
        /// The index where text was inserted
        index: usize,
        /// The text that was inserted
        text: String,
    },
    /// A deletion action that can be undone by reinserting the deleted text
    Delete {
        /// The index where text was deleted
        index: usize,
        /// The text that was deleted
        text: String,
    },
}

/// Manages the undo stack for a Rope
#[derive(Debug)]
pub struct UndoStack {
    /// The stack of actions that can be undone
    actions: Vec<UndoAction>,
}

impl UndoStack {
    /// Creates a new empty undo stack
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    /// Adds a new action to the undo stack
    pub fn push(&mut self, action: UndoAction) {
        self.actions.push(action);
    }

    /// Removes and returns the last action from the stack
    pub fn pop(&mut self) -> Option<UndoAction> {
        self.actions.pop()
    }

    /// Returns true if there are actions that can be undone
    pub fn can_undo(&self) -> bool {
        !self.actions.is_empty()
    }

    /// Applies the given action to the rope
    pub fn apply_action(action: UndoAction, rope: &mut Rope) {
        match action {
            UndoAction::Insert { index, text } => {
                // To undo an insert, we need to remove the inserted text
                rope.remove_without_undo(index, index + text.len());
            }
            UndoAction::Delete { index, text } => {
                // To undo a delete, we need to reinsert the deleted text
                rope.insert_without_undo(index, &text);
            }
        }
    }
} 