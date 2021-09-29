mod sudoku_util;
mod sudoku_input;
fn main() {
  let mut state = [[0u8; 9]; 9];
  sudoku_input::startup(&mut state);
  print!("{esc}c", esc = 27 as char);
  sudoku_util::print(state);
}
