mod map_builder;
mod sudoku_main;
use crate::sudoku_main::SudokuRoot;

fn main() {
    let v = map_builder::map_builder();
    let sudoku_instance = SudokuRoot {
        map: v,
    };
}
