use crate::sudoku_types::SudokuBoard;

pub fn compare(a: SudokuBoard, b: SudokuBoard) -> SudokuBoard{
  let mut out = SudokuBoard::new();
  for x in 0..9 {
    for y in 0..9 {
      if !a.board[x][y].is_filled() {
        if b.board[x][y].is_filled() {
          out.board[x][y].set_cell_value(b.get_cell_value(x, y), true)
        } else {
          for n in 0..9 {
            if a.board[x][y].candidates[n] && !b.board[x][y].candidates[n] {
              out.board[x][y].candidates_clean_except(n);
            }
          }
        }
      } else {
        out.board[x][y].candidates_remove_all()
      }
    }
  }

  return out;
}