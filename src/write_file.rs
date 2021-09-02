use std::fs;
// TODO: figure out how this should actually work
pub fn write_new_file (name: &String, directory: &String, content: &Vec<String>) -> std::io::Result<()> {
  let new_file = fs::File::create(name)?;
  for line in content {
    // new_file::write_all(line);
  }
  Ok(())
}