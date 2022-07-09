use std::fs;

pub fn read_text_file(path: &str) -> String {
  let file_content = fs::read_to_string(path)
    .expect("Something went wrong reading the file");
  //println!("{}", file_content);
  file_content
}