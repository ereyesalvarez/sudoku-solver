use std::io;
use std::io::{BufRead, Write};
use std::prelude::v1::String;
use std::time::Duration;

use owo_colors::OwoColorize;
use crate::board_print::{print_board, print_compare_board};

use crate::sudoku_io::{process_input_line, wait_press};
use crate::sudoku_types::{SudokuBoard, SudokuCell, SudokuCellType, SudokuOptions};
use crate::sudoku_validate::check_valid_sudoku;

pub fn read_line_from_command_line(stdin: &io::Stdin, flush: bool) -> String {
  if flush {
    let _ = io::stdout().flush();
  }
  return stdin.lock().lines().next().unwrap().unwrap();
}

pub fn _read_board() -> SudokuBoard {
  let mut board = SudokuBoard::new();
  let stdin = io::stdin();
  let mut read_lines = 0;
  while read_lines != 9 {
    let line = read_line_from_command_line(&stdin, true);
    process_input_line(&mut board, line, read_lines);
    read_lines += 1;
  }
  let (valid, invalid_x, invalid_y) = check_valid_sudoku(board);
  if !valid {
    panic!("Invalid sudoku on pos x: {}, y: {}", invalid_x, invalid_y);
  }
  return board;
}

pub(crate) fn print_load_instructions() {
  println!("{}", "Introduce los numeros separados por espacios".green());
  println!("{}", "Conjuntos de dos o más espacios  provocaran error.".green());
  println!("{}", "Para indicar un numero no conocido indicar con x.".green());
  println!("{}", "Pulsa enter para saltar de linea, se esperan 9 lineas.".green());
}

pub(crate) fn print_intro() {
  println!("{}", "Bienvenido al sudoku resolutor");
}

pub(crate) fn ask_for_options() -> SudokuOptions {
  let opt = SudokuOptions {
    print_steps: true,
    print_candidates: true,
    load_example: false,
    wait_press: true,
  };
  println!("A continuación se le preguntará por las opciones de juego. (enter dara un valor por defecto)");
  println!("¿Desea cargar un sudoku de ejemplo de los ficheros de ejemplo y/n?");
  return opt;
}


pub fn print_full_board_clean_and_info(board: SudokuBoard, opt: SudokuOptions, iteration: isize, remaining: isize) {
  print!("{esc}c", esc = 27 as char);
  print_board(board, opt);
  print_info_line(iteration, remaining);
  if opt.wait_press {
    wait_press();
  }
}

pub fn print_full_board_clean(board: SudokuBoard, opt: SudokuOptions) {
  print!("{esc}c", esc = 27 as char);
  print_board(board, opt);
  if opt.wait_press {
    wait_press();
  }
}

pub fn print_compare_board_clean(board: SudokuBoard, prev: SudokuBoard, opt: SudokuOptions) {
  print_compare_board(board, prev, opt);
  if opt.wait_press {
    wait_press();
  }
}


fn print_info_line(iteration: isize, remaining: isize) {
  println!("iteration: {}, remaining cells {}", iteration, remaining);
}