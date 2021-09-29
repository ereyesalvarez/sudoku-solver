use crate::sudoku_process::SudokuCell;
use crate::sudoku_process::SudokuCellType;
use owo_colors::OwoColorize;

pub fn print(arr: [[SudokuCell; 9]; 9]){
  print_beauty(arr);
}

pub fn print_simple(arr: [[SudokuCell; 9]; 9]){
  for row in arr{
      for val in row{
        if val.cell_type == SudokuCellType::Empty {
          print!("_ ");
        }
        if val.cell_type == SudokuCellType::Fixed {
          print!("{} ", val.value.green());
        }
        if val.cell_type == SudokuCellType::Guess {
          print!("{} ", val.value.red());
        }
      }
      println!("");
    }
}

pub fn print_beauty(arr: [[SudokuCell; 9]; 9]){
  for x in 0..9 {
    if x == 0 || x == 3 || x == 6{
      println!("----------------------");
    }
    for y in 0..9 {
      if y == 0 || y == 3 || y == 6{
        print!("|")
      }
      let val = arr[x][y];
      if val.cell_type == SudokuCellType::Empty {
        print!("  ");
      }
      if val.cell_type == SudokuCellType::Fixed {
        print!("{} ", val.value.green());
      }
      if val.cell_type == SudokuCellType::Guess {
        print!("{} ", val.value.red());
      }
      if y == 8 {
        print!("|")
      }
    }
    println!("");
  }
  println!("----------------------");
}


pub fn set_guess (board: &mut [[SudokuCell; 9]; 9], input: u8, x :usize, y :usize, ){
  if input <= 9 {
    if board[x][y].cell_type == SudokuCellType::Empty {
      board[x][y].value = input;
      board[x][y].cell_type = SudokuCellType::Guess;
    }
  }
}

pub fn set_val (board: &mut [[SudokuCell; 9]; 9], input: u8, x :usize, y :usize, ){
  if input <= 9 {
    if board[x][y].cell_type == SudokuCellType::Empty {
      board[x][y].value = input;
      board[x][y].cell_type = SudokuCellType::Fixed;
    }
  }
}

