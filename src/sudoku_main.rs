pub struct SudokuRoot {
  pub map: Vec<u32>,
} 

pub fn text_to_map(sudoku: &mut SudokuRoot, contents: &String) {
  const RADIX: u32 = 10;
  let size = contents.len();

  match size {
    81 => println!("Size is correct."),
    _ => panic!("Size incorrect: {} . Should be 81.", size) 
  }
  for c in contents.chars() {
    let c_int = c.to_digit(RADIX).unwrap();
    sudoku.map.push(c_int);
  }
}