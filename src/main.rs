mod file_reader;
mod map_builder;
mod sudoku_main;
use crate::sudoku_main::SudokuRoot;
use crate::sudoku_main::text_to_map;

fn print_vec (vec: Vec<u32>) {
    let mut counter = 1;
    for i in &vec {
        if counter % 9 == 0 {
            println!("{} ", i);
            counter += 1;
        } 
        else {
            print!("{} ", i);
            counter += 1;
        }
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

    print_vec(sudoku_instance.map);
}
