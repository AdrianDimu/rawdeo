mod rope;

use rope::Rope;

fn main () {
    let rope = Rope::new("Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8\nLine 9\nLine 10\n\n\n");
    println!("Total lines: {}", rope.lines());
    println!("Total chr: {}", rope.char_size());
    rope.print_structure();
}