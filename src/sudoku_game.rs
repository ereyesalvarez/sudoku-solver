use std::time;
use std::time::Duration;

use crate::sudoku_mock;
use crate::solve_steps::{do_step, StepEnum};
use crate::sudoku_gui::{ask_for_options, print_full_board_clean, print_full_board_clean_and_info, print_intro};
use crate::sudoku_mock::fake;
use crate::sudoku_types::{SudokuBoard, SudokuCell, SudokuOptions};
use crate::sudoku_validate::{check_valid_sudoku, get_remaining_cells};

pub fn game() {
    let opt = start_game();
    let mut board = load_board(opt);
    print_full_board_clean(board, opt);
    let mut iteration = 0;
    let mut repeat_step = true;
    while repeat_step {
        board = make_iterate(board, opt);
        iteration += 1;
        if iteration > 10 {
            repeat_step = false;
        }
    }
    let remaining = get_remaining_cells(board);
    print_full_board_clean_and_info(board, opt, iteration, remaining);
    let (valid, invalid_x, invalid_y) = check_valid_sudoku(board);
    if !valid {
        panic!("Invalid sudoku on pos x: {}, y: {}", invalid_x, invalid_y);
    }
    if remaining == 0 {
        return;
    }
}

pub fn make_iterate(mut board: SudokuBoard, opt: SudokuOptions) -> SudokuBoard {
    let board = do_step(board, opt, StepEnum::Pinned);
    let board = do_step(board, opt, StepEnum::LastRemain);
    let board = do_step(board, opt, StepEnum::NakedSingle);
    let board = do_step(board, opt, StepEnum::Naked);
    return board;
}

pub fn start_game() -> SudokuOptions {
    print_intro();
    return ask_for_options();
}

pub fn load_board(opt: SudokuOptions) -> SudokuBoard {
    return fake();
}

