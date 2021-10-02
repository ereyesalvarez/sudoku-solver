use crate::sudoku_input::process_input_line;
use crate::sudoku_io::read_sudoku_from_string;
use crate::sudoku_io::read_sudoku_from_file;
use crate::sudoku_process::create_board;
use crate::sudoku_types::SudokuCell;
use rand::Rng;
pub fn fake() -> [[SudokuCell; 9]; 9] {
  let input = String::from("sudoku_examples/sample03.txt");
  return read_sudoku_from_file(input);
}

pub fn fake_random() -> [[SudokuCell; 9]; 9] {
  let mut rng = rand::thread_rng();
  let sample =  rng.gen_range(0..5);
  return match sample {
    0 => fake1(),
    1 => fake2(),
    2 => fake3(),
    3 => fake4(),
    4 => fake5(),
    _ => fake1()
  }
}

fn fake1() -> [[SudokuCell; 9]; 9] {
  let input = String::from("sudoku_examples/sample01.txt");
  return read_sudoku_from_file(input);
}
fn fake2() -> [[SudokuCell; 9]; 9] {
  let input = String::from("sudoku_examples/sample02.txt");
  return read_sudoku_from_file(input);
}
fn fake3() -> [[SudokuCell; 9]; 9] {
  let input = String::from("sudoku_examples/sample03.txt");
  return read_sudoku_from_file(input);
}
fn fake4() -> [[SudokuCell; 9]; 9] {
  let input = String::from("sudoku_examples/sample04.txt");
  return read_sudoku_from_file(input);
}
fn fake5() -> [[SudokuCell; 9]; 9] {
  let input = String::from("sudoku_examples/sample05.txt");
  return read_sudoku_from_file(input);
}