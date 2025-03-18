mod rope;

use rope::Rope;

fn main () {
    let rope = Rope::new("\n\n\n\n");
    println!("Total lines: {}", rope.lines());
    rope.print_structure();
}