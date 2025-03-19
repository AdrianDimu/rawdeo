mod rope;
mod undo;

use rope::Rope;

fn main() {
    let mut rope = Rope::new("Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8\nLine 9\nLine 10\n\n\n");
    println!("Initial state:");
    println!("Total lines: {}", rope.lines());
    println!("Total chr: {}", rope.char_size());
    rope.print_structure();

    // Demonstrate undo functionality
    println!("\nInserting text...");
    rope.insert(5, "Hello ");
    println!("After insert:");
    println!("Total lines: {}", rope.lines());
    println!("Total chr: {}", rope.char_size());
    rope.print_structure();

    println!("\nRemoving text...");
    rope.remove(0, 6);
    println!("After remove:");
    println!("Total lines: {}", rope.lines());
    println!("Total chr: {}", rope.char_size());
    rope.print_structure();

    println!("\nUndoing last action...");
    rope.undo();
    println!("After undo:");
    println!("Total lines: {}", rope.lines());
    println!("Total chr: {}", rope.char_size());
    rope.print_structure();

    println!("\nUndoing insert...");
    rope.undo();
    println!("After undo:");
    println!("Total lines: {}", rope.lines());
    println!("Total chr: {}", rope.char_size());
    rope.print_structure();
}