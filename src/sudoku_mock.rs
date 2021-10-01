use crate::sudoku_input::process_input_line;
use crate::sudoku_process::create_board;
use crate::sudoku_types::SudokuCell;

pub fn fake() -> [[SudokuCell; 9]; 9] {
  return fake5();
}
fn _fake1() -> [[SudokuCell; 9]; 9] {
  let input = "\
9 x 1 x 7 x x x x \n\
7 x x 9 1 x 4 x x \n\
8 6 x 4 x 5 7 1 x \n\
x x 9 x 5 7 1 8 x \n\
x x 8 2 x 9 5 x x \n\
5 x x 1 8 x x x 2 \n\
x x 7 x x 4 x x x \n\
x 9 3 7 x x x x 4 \n\
1 5 x 3 x x 2 7 x \n\
";
  return process_text(String::from(input));
}

fn _fake2() -> [[SudokuCell; 9]; 9] {
  let mut board = create_board();
  // input, x, y,
  process_input_line(&mut board, String::from("9 x x x 8 2 5"), 0);
  process_input_line(&mut board, String::from("1 2 x 5 4 7 6 8 9"), 1);
  process_input_line(&mut board, String::from("5 4 x x x 6 2 x x"), 2);

  process_input_line(&mut board, String::from("7 8 x 6 1 5 x x x"), 3);
  process_input_line(&mut board, String::from("x x 4 9 2 8 7 x x"), 4);
  process_input_line(&mut board, String::from("x x 5 x x 4 1 2 8"), 5);

  process_input_line(&mut board, String::from("8 3 x x 7 x 4 6 5"), 6);
  process_input_line(&mut board, String::from("x x 9 x x 3 8 1 x"), 7);
  process_input_line(&mut board, String::from("4 5 7 x x 1 9 3 x"), 8);
  return board;
}

fn _fake3() -> [[SudokuCell; 9]; 9] {
  let input = "\
2 x x 7 1 x 9 4 x \n\
x x x x 3 x x 2 x \n\
6 9 x x x 8 x x 1 \n\
x x 3 x x x 2 x x \n\
1 x x 5 x 3 x x 7 \n\
x x 2 x x x 8 x x \n\
7 x x 1 x x x 8 4 \n\
x 1 x x 5 x x x x \n\
x 2 4 x 6 7 x x 5 \n\
";
  return process_text(String::from(input));
}


fn _fake4() -> [[SudokuCell; 9]; 9] {
  let input = "\
x 7 8 5 x x 9 x x \n\
1 6 x 7 9 x x x x \n\
2 x x x x 3 x 6 x \n\
x x 6 x x x x x x \n\
8 x 4 6 x 5 1 x 7 \n\
x x x x x x 8 x x \n\
x 8 x 1 x x x x 9 \n\
x x x x 5 9 x 1 2 \n\
x x 1 x x 4 6 8 x \n\
";
  return process_text(String::from(input));
}

fn fake5() -> [[SudokuCell; 9]; 9] {
  let input = "\
x x 7 4 x x x x x \n\
1 x x x x x x x 7 \n\
x x x 9 x x x 8 x \n\
x x x x x x x 9 8 \n\
x 4 x x 6 5 x 3 x \n\
3 x 8 x 2 x x x x \n\
4 x x x x x x x x \n\
2 x x 5 x x 3 x x \n\
x x 1 7 x 3 x 5 x \n\
";
  return process_text(String::from(input));
}

fn process_text(input: String) -> [[SudokuCell; 9]; 9] {
  let mut board = create_board();
  let split = input.split("\n");
  let mut col = 0;
  for s in split {
    process_input_line(&mut board, String::from(s), col);
    col += 1;
  }
  return board;
}



    