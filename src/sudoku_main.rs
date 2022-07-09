pub struct SudokuRoot {
  pub map: Vec<u8>,
} 

pub fn text_to_map(sudoku: &mut SudokuRoot, contents: &String) {
  for c in contents.chars() {
    sudoku.map.push(c as u8);
  }
}