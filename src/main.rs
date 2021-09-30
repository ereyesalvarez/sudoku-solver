use std::{io, time};
use std::io::BufRead;
use std::time::Duration;
use crate::sudoku_input::check_valid_sudoku;
use crate::sudoku_possibles::reset_possibles;
use crate::sudoku_process::{SudokuCell, SudokuCellType};
use crate::sudoku_game::{SudokuStep};
use crate::sudoku_resolve::is_finis;
use crate::sudoku_util::{print_full_board, print_full_board_info, print_possibles};

mod sudoku_util;
mod sudoku_input;
mod sudoku_process;
mod sudoku_game;
mod sudoku_mock;
mod sudoku_resolve;
mod sudoku_possibles;
mod sudoku_resolve_util;

fn main() {
  let max_iterations = 2;
  let mut idle_iterations = 0;
  let mut last_remain = 1000;
  sudoku_game::print_intro();
  let mut board = sudoku_mock::fake();
  print!("{esc}c", esc = 27 as char);
  let mut steps = 0;
  let mut d = Duration::from_secs(0);
  d += do_complete_step(&mut board, steps, d, SudokuStep::ClearBoard);
  let mut remaining = 99;
  loop {
    remaining = is_finis(board);
    check_valid_sudoku(board).unwrap();
    if remaining == 0 {
      print!("{esc}c", esc = 27 as char);
      print_full_board_info(board, steps, d, format!("WINNNNNN!!!"));
      break;
    }
    if last_remain == remaining {
      idle_iterations += 1;
      if idle_iterations >= max_iterations {
        //print_full_board_info(board, steps, d, format!("LOST!!!"));
        break;
      }
    } else {
      idle_iterations = 0;
    }
    last_remain = remaining;
    d += do_complete_step(&mut board, steps, d, SudokuStep::ClearByTuple);
    d += do_complete_step(&mut board, steps, d, SudokuStep::ResolveDirect);
    d += do_complete_step(&mut board, steps, d, SudokuStep::ClearBoard);
    d += do_complete_step(&mut board, steps, d, SudokuStep::ResolveInfer);
    d += do_complete_step(&mut board, steps, d, SudokuStep::ClearBoard);
    steps += 1;
  }
}

fn do_complete_step(board: &mut [[SudokuCell; 9]; 9], n: isize, d: Duration, n_step: SudokuStep) -> Duration {
  let mut i :u8 = 0;
  match n_step {
    SudokuStep::ClearBoard => i = 1,
    SudokuStep::ClearByTuple => i = 2,
    SudokuStep::ResolveDirect => i = 3,
    SudokuStep::ResolveInfer => i = 4
  }
  let d1 = do_step(board, i);
  let remaining = is_finis(*board);
  print_and_sleep(*board, n as isize, d + d1, remaining, i );
  return d1;
}

fn print_and_sleep(board: [[SudokuCell; 9]; 9], n: isize, d: Duration, remaining: i32, n_step: u8) {
  let mut s = format!("{}", remaining);
  match n_step {
    1 => s = format!("{} clear board", remaining),
    2 => s = format!("{} resolve direct", remaining),
    3 => s = format!("{} resolve infer", remaining),
    4 => s = format!("{} clean by tuple", remaining),
    _ => println!("Not implemented"),
  }
  if n_step != 0 {
    print_full_board_info(board, n, d, s);
    println!("waiting");
    let stdin = io::stdin();
    stdin.lock().lines().next().unwrap().unwrap();
    //sudoku_util::sleep_time();
  }
}

fn do_step(board: &mut [[SudokuCell; 9]; 9], n_step: u8) -> Duration {
  let now = time::Instant::now();
  match n_step {
    1 => sudoku_resolve::clear_board(board),
    2 => sudoku_resolve::resolve_direct(board),
    3 => sudoku_resolve::resolve_infer(board),
    4 => sudoku_resolve::clean_by_tuples(board),
    _ => println!("Not implemented"),
  }
  return now.elapsed();
}

#[cfg(test)]
mod sudoku_test;
