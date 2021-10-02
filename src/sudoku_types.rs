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

#[derive(PartialEq, Eq, Hash)]
pub struct PairTuple {
  pub x: usize,
  pub y: usize
}

impl SudokuCell {
  pub(crate) fn is_filled(&self) -> bool {
    !self.is_empty()
  }
  pub(crate) fn is_empty(&self) -> bool {
    self.cell_type == SudokuCellType::Empty
  }
}