#[derive(Copy, Clone)]
pub struct SudokuCell {
  pub value: u8,
  pub cell_type: SudokuCellType,
  pub possibles: [bool; 9],
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SudokuCellType {
  Empty,
  Fixed,
  Guess,
}

pub fn create_board() -> [[SudokuCell; 9]; 9] {
  let cell = SudokuCell {
    value: 0,
    cell_type: SudokuCellType::Empty,
    possibles: [true; 9],
  };
  return [[cell; 9]; 9];
}