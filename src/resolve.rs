use crate::sudoku_types::SudokuBoard;

mod naked;
mod x_wing;
mod swordfish;
mod hidden;
mod pinned;
mod last_remain;
mod intersection_removal;

/// Check if some cell only have one candidate
pub fn pinned(mut board: SudokuBoard) -> (SudokuBoard, isize) {
  return pinned::pinned(board);
}

/// Check if some line or box have any number that only can be in one cell
pub fn last_remaining(mut board: SudokuBoard) -> (SudokuBoard, isize) {
  return last_remain::last_remaining(board);
}

pub fn naked_single_resolve(board: SudokuBoard) -> (SudokuBoard, isize) {
  return naked::naked_single(board);
}

pub fn naked_tuple_resolve(board: SudokuBoard) -> (SudokuBoard, isize) {
  return naked::naked_tuple(board);
}

pub fn intersection_removal_resolve(board: SudokuBoard) -> (SudokuBoard, isize) {
  return intersection_removal::intersection_removal(board);
}

pub fn hidden_tuple_resolve(board: SudokuBoard) -> (SudokuBoard, isize) {
  return hidden::hidden_tuple(board);
}

