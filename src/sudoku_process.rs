use crate::sudoku_types::{SudokuCell, SudokuCellType};

pub fn create_board() -> [[SudokuCell; 9]; 9] {
  let cell = SudokuCell {
    value: 0,
    cell_type: SudokuCellType::Empty,
    possibles: [true; 9],
  };
  return [[cell; 9]; 9];
}