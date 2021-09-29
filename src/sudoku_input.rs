use std::io::{self, BufRead, Write};
use std::io::ErrorKind;
use std::io::Error;
use owo_colors::OwoColorize;
use crate::sudoku_process::SudokuCell;
use crate::sudoku_process::SudokuCellType;
use crate::sudoku_process::create_board;

pub fn startup() -> [[SudokuCell; 9]; 9] {
  println!("{}","Bienvenido al sudoku resolutor");
  println!("{}","Introduce los numeros separados por espacios".green());
  println!("{}","Conjuntos de dos o más espacios  provocaran error.".green());
  println!("{}","Para indicar un numero no conocido indicar con x.".green());
  println!("{}","Pulsa enter para saltar de linea, se esperan 9 lineas.".green());
  let mut board = create_board();
  let stdin = io::stdin(); // We get `Stdin` here.
  let mut read_lines = 0;
  while read_lines != 9 {
    print!("{}: ", read_lines);
    let _ = io::stdout().flush();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    process_input_line(&mut board, line, read_lines);
    read_lines += 1;
  }
  check_valid_sudoku(board).unwrap();
  return board;
}

fn process_input_line(arr: &mut [[SudokuCell; 9]; 9], input: String, row :usize){
  let split = input.split(" ");
  let mut col = 0;
  for s in split {
    if s == "x" || s == "" {
    } 
    else {
      let number: u8 = s.parse().expect("Not a number!");
      if number <= 9 {
        arr[row][col].value = number;
        arr[row][col].cell_type = SudokuCellType::Fixed;
      }
    }
    col += 1;
  }
}

fn check_valid_sudoku(arr: [[SudokuCell; 9]; 9]) -> Result<(), io::Error>{
  println!("valid row");
  for row in arr{
    let mut find = [false; 9];
    for val in row{
      check_valid_numer(val.value, &mut find).unwrap();
    }
  }
  println!("valid col");
  for i in 0..9{
    let mut find = [false; 9];
    for j in 0..9{
      let val = arr[j][i];
      check_valid_numer(val.value, &mut find).unwrap();
    }
  }
  // ToDo: Check if quarters are valid
  println!("valid quarter");
  for x in 0..3{
    for y in 0..3{
      let mut find = [false; 9];
      for n in 0..3 {
        for n2 in 0..3 {
          let aux_x = x * 3 + n;
          let aux_y = y * 3 + n2;
          print!("quarter {}, {}", aux_x, aux_y);
          let val = arr[aux_x][aux_y];
          check_valid_numer(val.value, &mut find).unwrap();
          }
        }
      }
      println!("");
    }
  Ok(())
}

fn check_valid_numer (value: u8, find:  &mut [bool; 9])  -> Result<(), io::Error>{
  if value != 0 {
    if value > 9 {
      println!("{}", "number not valid > 9".red());
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