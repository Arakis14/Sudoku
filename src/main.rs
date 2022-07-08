mod sudoku_main;
use crate::sudoku_main::SudokuRoot;

fn main() {
    let mut v: Vec<u16> = Vec::new();
    v.push(1);
    v.push(2);
    let sudoku_instance = SudokuRoot {
        map: v,
    };
}
