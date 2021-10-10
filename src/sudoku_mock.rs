use rand::Rng;
use crate::sudoku_io::read_board_from_file;

use crate::sudoku_types::{SudokuBoard, SudokuCell};

pub fn fake() -> SudokuBoard {
  let input = String::from("sudoku_examples/sample01.txt");
  return read_board_from_file(input);
}