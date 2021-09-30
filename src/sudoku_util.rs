use std::{thread, time};
use std::time::Duration;
use crate::sudoku_process::SudokuCell;
use crate::sudoku_process::SudokuCellType;
use owo_colors::OwoColorize;
use crate::sudoku_possibles::SudokuPossibles;

pub fn set_guess(board: &mut [[SudokuCell; 9]; 9], input: u8, x: usize, y: usize) {
  if input <= 9 {
    if board[x][y].cell_type == SudokuCellType::Empty {
      board[x][y].value = input;
      board[x][y].cell_type = SudokuCellType::Guess;
    }
  }
}

pub fn set_val(board: &mut [[SudokuCell; 9]; 9], input: u8, x: usize, y: usize) {
  if input <= 9 {
    if board[x][y].cell_type == SudokuCellType::Empty {
      board[x][y].value = input;
      board[x][y].cell_type = SudokuCellType::Fixed;
      board[x][y].possibles = [false; 9];
    }
  }
}

pub fn print_possible_item(input: [[bool; 9]; 9]) {
  for i in 0..9 {
    print!("{}: ", i);
    for n in 0..9 {
      if input[i][n] {
        print!("{} ", (n + 1))
      }
    }
    println!();
  }
}

pub fn print_possibles(input: SudokuPossibles) {
  println!("rows");
  print_possible_item(input.row);
  println!("cols");
  print_possible_item(input.row);
  println!("quarters");
  print_possible_item(input.quarter);
}

pub fn print_full_board(board: [[SudokuCell; 9]; 9]) {
  print!("{esc}c", esc = 27 as char);
  for row in 0..9 {
    if row == 0 || row == 3 || row == 6 {
      print_full_line_hard();
    } else {
      print_full_line();
    }
    for part in 0..3 {
      for col in 0..9 {
        if col == 0 || col == 3 || col == 6 {
          print!("||")
        } else {
          print!("|")
        }
        print_full_cell(board[row][col], part);
      }
      println!("||");
    }
  }
  print_full_line_hard();
}


pub fn print_full_board_info(board: [[SudokuCell; 9]; 9], n: isize, d: Duration, data: String) {
  print_full_board(board);
  print_inf(n, d, data);
}

fn print_full_cell(cell: SudokuCell, part: u8) {
  if cell.cell_type == SudokuCellType::Fixed || cell.cell_type == SudokuCellType::Guess {
    if part == 0 || part == 2 {
      print!("     ")
    } else {
      if cell.cell_type == SudokuCellType::Fixed {
        print!("  {}  ", cell.value.purple())
      } else {
        print!("  {}  ", cell.value.red())
      }
    }
  } else {
    let init: usize = (part * 3) as usize;
    let mut a: String = String::from(" ");
    if cell.possibles[init] {
      a = (init + 1).to_string();
    }
    let mut b: String = String::from(" ");
    if cell.possibles[init + 1] {
      b = (init + 2).to_string();
    }
    let mut c: String = String::from(" ");
    if cell.possibles[init + 2] {
      c = (init + 3).to_string();
    }
    let string_formatted = format!("{} {} {}", a, b, c);
    //let string_formatted = format!("     ");

    print!("{}", string_formatted.cyan())
  }
}

fn print_full_line() {
  println!("++-----+-----+-----++-----+-----+-----++-----+-----+-----++")
}

fn print_full_line_hard() {
  println!("++=====+=====+=====++=====+=====+=====++=====+=====+=====++")
}

pub fn calculate_quarter(x: usize, y: usize) -> usize {
  let a = x / 3;
  let b = y / 3;
  return a * 3 + b;
}

pub fn sleep_time() {
  let time = time::Duration::from_millis(700);
  thread::sleep(time);
}

pub fn print_inf(n: isize, d: Duration, data: String) {
  println!("step: {}, duration_total: {} - {}", n, d.as_millis(), data)
}