use std::io;
use std::io::BufRead;
use std::time::Duration;

use owo_colors::OwoColorize;

use crate::sudoku_process::{SudokuCell, SudokuCellType};

pub fn print_and_wait(board: [[SudokuCell; 9]; 9], n: isize, d: Duration, remaining: i32, n_step: u8) {
  let mut s = format!("{}", remaining);
  match n_step {
    1 => s = format!("{} clear board", remaining),
    2 => s = format!("{} sudoku_resolve direct", remaining),
    3 => s = format!("{} sudoku_resolve infer", remaining),
    4 => s = format!("{} clean by tuple", remaining),
    5 => s = format!("{} clean by Q Contrain", remaining),
    _ => println!("Not implemented"),
  }
  if n_step != 0 {
    print_full_board_info(board, n, d, s);
    println!("waiting");
    let stdin = io::stdin();
    stdin.lock().lines().next().unwrap().unwrap();
  }
}

pub fn print_full_board_info(board: [[SudokuCell; 9]; 9], n: isize, d: Duration, data: String) {
  print!("{esc}c", esc = 27 as char);
  print_full_board(board);
  print_inf(n, d, data);
}

pub fn print_intro() {
  println!("{}", "Bienvenido al sudoku resolutor");
  println!("{}", "Introduce los numeros separados por espacios".green());
  println!("{}", "Conjuntos de dos o más espacios  provocaran error.".green());
  println!("{}", "Para indicar un numero no conocido indicar con x.".green());
  println!("{}", "Pulsa enter para saltar de linea, se esperan 9 lineas.".green());
}

fn print_inf(n: isize, d: Duration, data: String) {
  println!("step: {}, process_duration: {} - {}", n, d.as_millis(), data)
}

fn print_full_board(board: [[SudokuCell; 9]; 9]) {
  for row in 0..9 {
    if row == 0 || row == 3 || row == 6 {
      print_board_line(true);
    } else {
      print_board_line(false);
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
  print_board_line(true);
}

fn print_full_cell(cell: SudokuCell, part: u8) {
  let print_options = true;
  if cell.cell_type == SudokuCellType::Fixed || cell.cell_type == SudokuCellType::Guess {
    if part == 0 || part == 2 {
      print!("     ")
    } else {
      if cell.cell_type == SudokuCellType::Fixed {
        print!("  {}  ", cell.value.purple())
      } else {
        print!("  {}  ", cell.value.yellow())
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
    if print_options {
      let string_formatted = format!("{} {} {}", a, b, c);
      print!("{}", string_formatted.cyan())
    } else {
      let string_formatted = format!("     ");
      print!("{}", string_formatted.cyan())
    }
  }
}

fn print_board_line(hard: bool) {
  if hard {
    println!("++=====+=====+=====++=====+=====+=====++=====+=====+=====++");
  } else {
    println!("++-----+-----+-----++-----+-----+-----++-----+-----+-----++");
  }
}

