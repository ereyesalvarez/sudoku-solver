use std::fmt;

use crate::board_util::get_box_range_from_pos;

#[derive(Copy, Clone)]
pub struct SudokuBoard {
  pub board: [[SudokuCell; 9]; 9],
}

#[derive(Copy, Clone)]
pub struct SudokuCell {
  pub value: u8,
  pub cell_type: SudokuCellType,
  pub candidates: [bool; 9],
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SudokuCellType {
  Empty,
  Fixed,
  Guess,
}

#[derive(PartialEq, Eq, Hash)]
pub struct Position {
  pub x: usize,
  pub y: usize,
}

#[derive(Copy, Clone)]
pub struct SudokuOptions {
  pub print_steps: bool,
  pub print_candidates: bool,
  pub load_example: bool,
  pub wait_press: bool,
}

impl fmt::Display for SudokuBoard {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut s = String::new();
    for row in self.board {
      for col in row {
        s += format!(" {} ", col.value).as_str()
      }
      s += "\n"
    }
    write!(f, "{}", s)
  }
}

impl SudokuBoard {
  pub(crate) fn new() -> SudokuBoard {
    let cell = SudokuCell {
      value: 0,
      cell_type: SudokuCellType::Empty,
      candidates: [true; 9],
    };
    return SudokuBoard { board: [[cell; 9]; 9] };
  }

  pub(crate) fn get_cell_value(&self, x: usize, y: usize) -> usize {
    self.board[x][y].value as usize
  }

  pub(crate) fn set_fixed(&mut self, num: usize, x: usize, y: usize) {
    self.board[x][y].set_cell_value(num, false);
    self.clean_by_pos_and_value(x, y, num);
  }

  pub(crate) fn set_guess(&mut self, num: usize, x: usize, y: usize) {
    self.board[x][y].set_cell_value(num, true);
    self.clean_by_pos_and_value(x, y, num);
  }

  pub(crate) fn clean_by_pos_and_value(&mut self, x: usize, y: usize, num: usize) {
    for axis in 0..9 {
      // col
      if axis != x {
        self.board[axis][y].candidates_remove(num)
      }
      // row
      if axis != y {
        self.board[x][axis].candidates_remove(num)
      }
      // box
      let (x_range, y_range) = get_box_range_from_pos(x, y);
      for row in x_range {
        for col in y_range {
          if row != x && col != y {
            self.board[row][col].candidates_remove(num)
          }
        }
      }
    }
  }

  pub(crate) fn get_cell(&self, x: usize, y: usize) -> SudokuCell {
    self.board[x][y]
  }

}

impl SudokuCell {
  pub(crate) fn is_filled(&self) -> bool {
    !self.is_empty()
  }

  pub(crate) fn is_empty(&self) -> bool {
    self.cell_type == SudokuCellType::Empty
  }


  pub(crate) fn set_cell_value(&mut self, num: usize, is_guess: bool) {
    if num <= 9 {
      if self.is_empty() {
        self.value = num as u8;
        if is_guess {
          self.cell_type = SudokuCellType::Guess;
        } else {
          self.cell_type = SudokuCellType::Fixed;
        }
        self.candidates = [false; 9];
      }
    }
  }

  pub(crate) fn is_candidate_present(&self, num: usize) -> bool {
    self.is_candidate_present_pos(num - 1)
  }

  pub(crate) fn is_candidate_present_pos(&self, pos: usize) -> bool {
    self.candidates[pos]
  }


  pub(crate) fn candidates_remove_all(&mut self) {
    self.candidates = [false; 9];
  }

  pub(crate) fn candidates_remove(&mut self, num: usize) {
    if self.is_empty() {
      self.candidates_remove_pos(num - 1)
    }
  }

  pub(crate) fn candidates_remove_pos(&mut self, pos: usize) {
    self.candidates[pos] = false;
  }

  pub(crate) fn candidates_clean_except(&mut self, num: usize) {
    self.candidates_clean_except(num - 1)
  }

  pub(crate) fn candidates_clean_except_pos(&mut self, pos: usize) {
    self.candidates = [false; 9];
    self.candidates[pos] = true;
  }

  pub(crate) fn candidates_clean_except_two_pos(&mut self, pos: usize, pos2: usize) {
    self.candidates = [false; 9];
    self.candidates[pos] = true;
    self.candidates[pos2] = true;
  }

  pub(crate) fn candidates_clean_except_two(&mut self, num: usize, num2: usize) {
    self.candidates_clean_except_two_pos(num - 1, num2 - 1)
  }

  pub(crate) fn candidates_clean_except_three_pos(&mut self, pos: usize, pos2: usize, pos3: usize) {
    self.candidates = [false; 9];
    self.candidates[pos] = true;
    self.candidates[pos2] = true;
    self.candidates[pos3] = true;
  }
}
