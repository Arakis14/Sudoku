use std::fs;

pub fn read_text_file(path: &str) -> String {
  let mut file_content = fs::read_to_string(path)
    .expect("Something went wrong reading the file");
  //println!("{}", file_content);
  remove_whitespace(&mut file_content);
  file_content
}

fn remove_whitespace(s: &mut String) {
  s.retain(|c| !c.is_whitespace());
}