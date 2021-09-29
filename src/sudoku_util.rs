use crate::sudoku_process::SudokuCell;
use crate::sudoku_process::SudokuCellType;
use owo_colors::OwoColorize;

pub fn print(arr: [[SudokuCell; 9]; 9]){
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

