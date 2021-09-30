use crate::sudoku_gui::{print_compare_board_clean, print_full_board_clean};
use crate::sudoku_types::{SudokuBoard, SudokuOptions};
use crate::resolve::{hidden_tuple_resolve, intersection_removal_resolve, last_remaining, naked_single_resolve, naked_tuple_resolve, pinned};

#[derive(Debug)]
pub(crate) enum StepEnum {
  Pinned,
  LastRemain,
  NakedSingle,
  Naked,
  Hidden,
  IntersectionRemove,
  XWing,
  Swordfish,
}

pub(crate) fn do_step(mut board: SudokuBoard, opt: SudokuOptions, step_type: StepEnum) -> SudokuBoard {
  let mut repeat_step = true;
  while repeat_step {
    let (aux_board, hits) = match step_type {
      StepEnum::Pinned => pinned(board),
      StepEnum::LastRemain => last_remaining(board),
      StepEnum::NakedSingle => naked_single_resolve(board),
      StepEnum::Naked => naked_tuple_resolve(board),
      StepEnum::Hidden => hidden_tuple_resolve(board),
      StepEnum::IntersectionRemove => intersection_removal_resolve(board),
      StepEnum::XWing => pinned(board),
      StepEnum::Swordfish => pinned(board)
    };
    print!("{esc}c", esc = 27 as char);
    println!("{:?}", step_type);
    print_compare_board_clean(aux_board, board,opt);
    board = aux_board;
    repeat_step = hits > 0;
  }
  return board
}