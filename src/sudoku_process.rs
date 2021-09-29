#[derive(Copy, Clone)]
pub struct SudokuCell {
  pub value: u8,
  pub cell_type: SudokuCellType
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SudokuCellType {
  Empty,
  Fixed,
  Guess
}

pub fn create_board () -> [[SudokuCell; 9]; 9]{
  let cell = SudokuCell {
    value: 0,
    cell_type: SudokuCellType::Empty
  };
  return [[cell; 9]; 9]
} 