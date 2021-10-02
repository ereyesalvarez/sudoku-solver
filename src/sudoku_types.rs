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

  /// Is the candidate as number (1..9) present
  pub(crate) fn is_candidate_present(&self, val :usize) -> bool{
    self.possibles[val -1]
  }

  /// Is the candidate as position (0..8) present
  pub(crate) fn is_candidate_present_pos(&self, val :usize) -> bool{
    self.possibles[val]
  }

  /// Clean all candidates except the entered (val is position);
  pub(crate) fn candidates_clean_except(&mut self, val :usize) {
    self.possibles = [false; 9];
    self.possibles[val] = true;
  }

  /// Clean all candidates except the entered (val_n are position);
  pub(crate) fn candidates_clean_except_two(&mut self, val :usize, val2: usize) {
    self.possibles = [false; 9];
    self.possibles[val] = true;
    self.possibles[val2] = true;
  }

  /// Clean all candidates except the entered (val_n are position);
  pub(crate) fn candidates_clean_except_three(&mut self, val :usize, val2 :usize, val3: usize) {
    self.possibles = [false; 9];
    self.possibles[val] = true;
    self.possibles[val2] = true;
    self.possibles[val3] = true;
  }
}