mod file_reader;
mod map_builder;
mod sudoku_main;
use crate::sudoku_main::SudokuRoot;

fn main() {
    let file_path = "/home/wiktor/Documents/workspace/Sudoku/src/sudoku.txt";
    file_reader::read_text_file(file_path);
    let v = map_builder::map_builder();
    let sudoku_instance = SudokuRoot {
        map: v,
    };
}
