use std::fs;
// TODO: figure out how this should actually work
// TODO: figure out how this will work in a CLI so can have a dynamic root-dir/relative-dir
pub fn write_new_file (name: String, content: String) -> std::io::Result<()> {
  fs::write(name, content)?;
  Ok(())
}