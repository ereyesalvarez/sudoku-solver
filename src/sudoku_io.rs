use std::{fs, io};
use std::io::BufRead;
use crate::sudoku_types::{SudokuBoard, SudokuCell};

use std::path::Path;

pub fn read_board_from_file(path: String) -> SudokuBoard {
  println!("going to read file");
  if !Path::new(path.as_str()).exists() {
    println!("file {} not exist", path);
  } else {
    println!("file {} exist", path);
  }
  let contents = fs::read_to_string(path)
    .expect("Something went wrong reading the file");
  return read_board_from_string(contents);
}

pub fn read_board_from_string(input: String) -> SudokuBoard {
  let mut board = SudokuBoard::new();
  let split = input.split("\n");
  let mut row = 0;
  for s in split {
    process_input_line(&mut board, String::from(s), row);
    row += 1;
  }
  return board;
}

pub fn process_input_line(board: &mut SudokuBoard, input: String, row: usize) {
  let split = input.split(" ");
  let mut col = 0;
  for s in split {
    if s == "x" || s == "" {} else {
      println!("{}", s);
      let number: usize = s.parse().expect("Not a number!");
      board.set_fixed(number, row, col);
    }
    col += 1;
  }
}

pub fn wait_press(){
  let stdin = io::stdin();
  stdin.lock().lines().next().unwrap().unwrap();
}