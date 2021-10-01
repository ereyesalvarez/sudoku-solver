use std::time;
use std::time::Duration;

use crate::{sudoku_mock, sudoku_resolve};
use crate::sudoku_gui::{print_and_wait, print_full_board_info, print_intro};
use crate::sudoku_input::check_valid_sudoku;
use crate::sudoku_process::SudokuCell;
use crate::sudoku_resolve::{is_finish};

pub enum SudokuStep {
  ClearBoard,
  ClearByTuple,
  ResolveDirect,
  ResolveInfer,
  ClearByQuarterConstrain,
}

pub fn play() {
  print_intro();
  let mut board = sudoku_mock::fake();
  let mut steps = 0;
  let mut d = Duration::from_secs(0);
  d += do_complete_step(&mut board, steps, d, SudokuStep::ClearBoard);
  loop {
    let remaining = is_finish(board);
    check_valid_sudoku(board).unwrap();
    if remaining == 0 {
      print_full_board_info(board, steps, d, format!("WINNNNNN!!!"));
      break;
    }
    d = play_step(&mut board, steps, d);
    steps += 1;
  }
}

fn play_step(board: &mut [[SudokuCell; 9]; 9], step: isize, mut d: Duration) -> Duration {
  d += do_complete_step(board, step, d, SudokuStep::ResolveDirect);
  d += do_complete_step(board, step, d, SudokuStep::ResolveInfer);
  d += do_complete_step(board, step, d, SudokuStep::ClearBoard);
  d += do_complete_step(board, step, d, SudokuStep::ClearByTuple);
  d += do_complete_step(board, step, d, SudokuStep::ClearByQuarterConstrain);
  return d;
}

fn do_complete_step(board: &mut [[SudokuCell; 9]; 9], n: isize, d: Duration, n_step: SudokuStep) -> Duration {
  let i: u8;
  match n_step {
    SudokuStep::ClearBoard => i = 1,
    SudokuStep::ClearByTuple => i = 2,
    SudokuStep::ResolveDirect => i = 3,
    SudokuStep::ResolveInfer => i = 4,
    SudokuStep::ClearByQuarterConstrain => i = 5,

  }
  let d1 = do_step(board, i);
  let remaining = is_finish(*board);
  // print_and_wait(*board, n as isize, d + d1, remaining, i);
  return d1;
}

fn do_step(board: &mut [[SudokuCell; 9]; 9], n_step: u8) -> Duration {
  let now = time::Instant::now();
  match n_step {
    1 => sudoku_resolve::clear_board(board),
    2 => sudoku_resolve::resolve_direct(board),
    3 => sudoku_resolve::resolve_infer(board),
    4 => sudoku_resolve::clean_by_tuples(board),
    5 => sudoku_resolve::clean_by_quarter_constrain(board),
    _ => println!("Not implemented"),
  }
  return now.elapsed();
}