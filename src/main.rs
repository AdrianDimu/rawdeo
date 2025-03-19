mod rope;
mod undo;

use rope::Rope;
use rawdeo::Editor;

fn main() -> std::io::Result<()> {
    let mut editor = Editor::new()?;
    editor.run()?;
    Ok(())
}