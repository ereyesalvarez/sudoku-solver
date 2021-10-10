use crate::resolve::naked::naked_single;
use crate::sudoku_types::SudokuBoard;

mod naked;
mod x_wing;
mod swordfish;
mod hidden;
mod pinned;
mod last_remain;

/// Check if some cell only have one candidate
pub fn pinned(mut board: SudokuBoard) -> (SudokuBoard, isize) {
  return pinned::pinned(board);
}
/// Check if some line or box have any number that only can be in one cell
pub fn last_remaining(mut board: SudokuBoard) -> (SudokuBoard, isize) {
  return last_remain::last_remaining(board);
}


pub fn naked(mut board: SudokuBoard) -> (SudokuBoard, isize) {
  return naked_single(board);
}



