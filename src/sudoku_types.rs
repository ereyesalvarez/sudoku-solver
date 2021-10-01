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