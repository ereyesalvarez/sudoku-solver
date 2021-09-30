use std::time;
use std::time::Duration;
use crate::sudoku_possibles::reset_possibles;
use crate::sudoku_resolve::is_finis;
use crate::sudoku_util::{print_full_board, print_full_board_info, print_possibles};

mod sudoku_util;
mod sudoku_input;
mod sudoku_process;
mod sudoku_game;
mod sudoku_mock;
mod sudoku_resolve;
mod sudoku_possibles;

fn main() {
  sudoku_game::print_intro();
  let mut board = sudoku_mock::fake();
  let mut possibles = reset_possibles();
  print!("{esc}c", esc = 27 as char);
  let mut steps = 0;
  let mut d = Duration::from_secs(0);
  print_full_board(board);
  sudoku_util::sleep_time();
  let mut remaining = 999;
  loop {
    let now = time::Instant::now();
    sudoku_resolve::clear_possibles(&mut board, &mut possibles);
    d += now.elapsed();
    print_full_board_info(board, steps, d, format!("{} clear", remaining));
    sudoku_util::sleep_time();
    let now = time::Instant::now();
    sudoku_resolve::resolve_direct(&mut board);
    d += now.elapsed();
    print_full_board_info(board, steps, d, format!("{} direct", remaining));
    sudoku_util::sleep_time();
    let now = time::Instant::now();
    sudoku_resolve::resolve_infer(&mut board);
    d += now.elapsed();
    sudoku_util::sleep_time();
    print_full_board_info(board, steps, d, format!("{} infer", remaining));
    sudoku_util::sleep_time();
    remaining = is_finis(board);
    if remaining == 0 {
      print!("{esc}c", esc = 27 as char);
      println!("-------WINNNNNNNNNNN-----");
      break;
    }
    sudoku_util::sleep_time();
    steps += 1;
  }
}

#[cfg(test)]
mod sudoku_test;
