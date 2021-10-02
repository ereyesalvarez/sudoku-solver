use std::fs;
use crate::sudoku_input::process_input_line;
use crate::sudoku_process::create_board;
use crate::sudoku_types::SudokuCell;

use std::path::Path;

pub fn read_sudoku_from_file(path: String) -> [[SudokuCell; 9]; 9] {
  println!("going to read file");
  if !Path::new(path.as_str()).exists() {
    println!("file {} not exist", path);
  } else {
    println!("file {} exist", path);
  }
  let contents = fs::read_to_string(path)
    .expect("Something went wrong reading the file");
  println!("{}", contents);
  return read_sudoku_from_string(contents);
}

pub fn read_sudoku_from_string(input: String) -> [[SudokuCell; 9]; 9] {
  let mut board = create_board();
  let split = input.split("\n");
  let mut col = 0;
  for s in split {
    process_input_line(&mut board, String::from(s), col);
    col += 1;
  }
  return board;
}

