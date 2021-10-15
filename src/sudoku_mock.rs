use crate::sudoku_io::read_board_from_file;

use crate::sudoku_types::{SudokuBoard};

pub fn fake() -> SudokuBoard {
  let input = String::from("sudoku_examples/sample10.txt");
  return read_board_from_file(input);
}