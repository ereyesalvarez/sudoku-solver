use std::io::{self, BufRead, Write};
use std::io::ErrorKind;
use std::io::Error;
use owo_colors::OwoColorize;


pub fn startup(arr: &mut [[u8; 9]; 9]){
  println!("{}","Bienvenido al sudoku resolutor");
  println!("{}","Introduce los numeros separados por espacios".green());
  println!("{}","Conjuntos de dos o más espacios  provocaran error.".green());
  println!("{}","Para indicar un numero no conocido indicar con x.".green());
  println!("{}","Pulsa enter para saltar de linea, se esperan 9 lineas.".green());
  let stdin = io::stdin(); // We get `Stdin` here.
  let mut read_lines = 0;
  while read_lines != 9 {
    print!("{}: ", read_lines);
    let _ = io::stdout().flush();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    process_input_line(arr, line, read_lines);
    read_lines += 1;
  }
  check_valid_sudoku(*arr).unwrap();
}

fn process_input_line(arr: &mut [[u8; 9]; 9], input: String, row :usize){
  let split = input.split(" ");
  let mut col = 0;
  for s in split {
    if s == "x" || s == "" {
    } 
    else {
      let number: u8 = s.parse().expect("Not a number!");
      if number <= 9 {
        arr[row][col] = number;
      }
    }
    col += 1;
  }
}

fn check_valid_sudoku(arr: [[u8; 9]; 9]) -> Result<(), io::Error>{
  for row in arr{
    let mut find = [false; 9];
    // ToDo: Check row
    for val in row{
      if val != 0 {
        if val > 9 {
          println!("{}", "number not valid > 9".red());
          return Err(Error::new(ErrorKind::Other, "number not valid > 9"));
        }
        let position: usize = (val - 1).into();
        if find[position] {
          return Err(Error::new(ErrorKind::Other, "Numero repetido..."));
        }
        find[position] = true
      }
    }
  }
  for i in 0..9{
    let mut find = [false; 9];
    for j in 0..9{
      let val = arr[j][i];
      if val != 0 {
        if val > 9 {
          println!("{}", "number not valid > 9".red());
          return Err(Error::new(ErrorKind::Other, "number not valid > 9"));
        }
        let position: usize = (val - 1).into();
        if find[position] {
          return Err(Error::new(ErrorKind::Other, "Numero repetido..."));
        }
        find[position] = true
      }
    }
  }
  Ok(())
}