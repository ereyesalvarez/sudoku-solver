use crate::sudoku_gui::{print_compare_board_clean, print_full_board_clean};
use crate::sudoku_types::{SudokuBoard, SudokuOptions};
use crate::resolve::{last_remaining, naked, pinned};

pub(crate) enum StepEnum {
  Pinned,
  LastRemain,
  Naked,
  Hidden,
  IntersectionRemove,
  XWing,
  Swordfish,
}

pub(crate) fn do_step(mut board: SudokuBoard, opt: SudokuOptions, step_type: StepEnum) -> SudokuBoard {
  let mut cycles = 0;
  let mut repeat_step = true;
  while repeat_step {
    let (aux_board, hits) = match step_type {
      StepEnum::Pinned => pinned(board),
      StepEnum::LastRemain => last_remaining(board),
      StepEnum::Naked => naked(board),
      StepEnum::Hidden => naked(board),
      StepEnum::IntersectionRemove => naked(board),
      StepEnum::XWing => naked(board),
      StepEnum::Swordfish => naked(board)
    };
    print_compare_board_clean(aux_board, board,opt);
    board = aux_board;
    repeat_step = hits > 0;
    cycles += 1;
    if(cycles > 4){
      repeat_step = true;
    }
  }
  return board
}