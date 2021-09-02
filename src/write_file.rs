use std::fs;
pub fn write_new_file (name: String, content: String) -> std::io::Result<()> {
  fs::write(name, content)?;
  Ok(())
}