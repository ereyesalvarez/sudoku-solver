use std::io::{self, BufRead, Write};
use std::io::Error;
use std::io::ErrorKind;

use crate::sudoku_process::create_board;
use crate::sudoku_process::SudokuCell;
use crate::sudoku_util::set_val;

pub fn _read_board() -> [[SudokuCell; 9]; 9] {
  let mut board = create_board();
  let stdin = io::stdin();
  let mut read_lines = 0;
  while read_lines != 9 {
    let _ = io::stdout().flush();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    process_input_line(&mut board, line, read_lines);
    read_lines += 1;
  }
  check_valid_sudoku(board).unwrap();
  return board;
}

pub fn process_input_line(arr: &mut [[SudokuCell; 9]; 9], input: String, row: usize) {
  let split = input.split(" ");
  let mut col = 0;
  for s in split {
    if s == "x" || s == "" {} else {
      let number: u8 = s.parse().expect("Not a number!");
      set_val(arr, number, row, col);
    }
    col += 1;
  }
}

pub fn check_valid_sudoku(arr: [[SudokuCell; 9]; 9]) -> Result<(), io::Error> {
  // Comprueba que en las row solo se encuentre un valor de cada numero.
  for i in 0..9 {
    let row = arr[i];
    let mut find = [false; 9];
    for j in 0..9 {
      let val = row[j];
      match check_valid_number(val.value, &mut find) {
        Ok(_) => {}
        Err(error) => { panic!("row number repeated in x: {}, y {}, {:?}", i, j, error) }
      }
    }
  }
  for i in 0..9 {
    let mut find = [false; 9];
    for j in 0..9 {
      let val = arr[j][i];
      match check_valid_number(val.value, &mut find) {
        Ok(_) => {}
        Err(error) => { panic!("col number repeated in x: {}, y {}, {:?}", i, j, error) }
      }
    }
  }
  for x in 0..3 {
    for y in 0..3 {
      let mut find = [false; 9];
      for n in 0..3 {
        for n2 in 0..3 {
          let aux_x = x * 3 + n;
          let aux_y = y * 3 + n2;
          let val = arr[aux_x][aux_y];
          match check_valid_number(val.value, &mut find) {
            Ok(_) => {}
            Err(error) => { panic!("quadrant number repeated in x: {}, y {}, {:?}", aux_x, aux_y, error) }
          }
        }
      }
    }
  }
  Ok(())
}

fn check_valid_number(value: u8, find: &mut [bool; 9]) -> Result<(), io::Error> {
  if value != 0 {
    if value > 9 {
      return Err(Error::new(ErrorKind::Other, "number not valid > 9"));
    }
    let position: usize = (value - 1).into();
    if find[position] {
      return Err(Error::new(ErrorKind::Other, "Numero repetido..."));
    }
    find[position] = true;
  }
  Ok(())
}