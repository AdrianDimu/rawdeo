mod rope;

use rope::Rope;

fn main () {
    let rope = Rope::new("Hello, World!");
    println!("Total lines: {}", rope.lines());
}