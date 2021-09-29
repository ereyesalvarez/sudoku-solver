mod sudoku_util;
mod sudoku_input;
mod sudoku_process;

fn main() {
  let mut board = sudoku_input::startup();
  print!("{esc}c", esc = 27 as char);
  sudoku_util::print(board);
}


#[cfg(test)]
mod sudoku_test;
