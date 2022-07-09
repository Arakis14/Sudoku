mod file_reader;
mod map_builder;
mod sudoku_main;
use crate::sudoku_main::SudokuRoot;
use crate::sudoku_main::text_to_map;

fn print_vec (vec: Vec<u8>) {
    for i in &vec {
        println!("{}", i);
    }
}

fn main() {
    let file_path = "/home/wiktor/Documents/workspace/Sudoku/src/sudoku.txt";
    let contents = file_reader::read_text_file(file_path);
    let v = map_builder::map_builder();
    let mut sudoku_instance = SudokuRoot {
        map: v,
    };
    text_to_map(&mut sudoku_instance, &contents);

    let is_empty_1 = sudoku_instance.map.is_empty();
    println!("{}", is_empty_1);

    print_vec(sudoku_instance.map);
}
