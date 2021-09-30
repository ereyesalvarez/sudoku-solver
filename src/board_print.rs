use owo_colors::OwoColorize;

use crate::sudoku_types::{SudokuBoard, SudokuCell, SudokuCellType, SudokuOptions};

pub(crate) fn print_board(board: SudokuBoard, opt: SudokuOptions) {
  print_full_board(board, board, false, opt);
}

pub(crate) fn print_compare_board(board: SudokuBoard, prev: SudokuBoard, opt: SudokuOptions) {
  print_full_board(board, prev, true, opt);
}

fn print_full_cell(cell: SudokuCell, prev: SudokuCell, part: u8, comparing: bool, opt: SudokuOptions) {
  if cell.is_filled() {
    print_filled_cell(cell, prev, part, comparing);
  } else {
    if opt.print_candidates {
      print_candidate_from_cell(cell, prev, part, comparing);
    }
  }
}

fn print_filled_cell(cell: SudokuCell, prev: SudokuCell, part: u8, comparing: bool) {
  if part == 0 || part == 2 {
    print!("     ")
  } else {
    if cell.cell_type == SudokuCellType::Fixed {
      print!("  {}  ", cell.value.white())
    } else {
      if prev.cell_type == SudokuCellType::Guess {
        print!("  {}  ", cell.value.yellow())
      } else if comparing {
        print!("  {}  ", cell.value.green())
      }
    }
  }
}

fn print_candidate_from_cell(cell: SudokuCell, prev: SudokuCell, part: u8, comparing: bool) {
  let init: usize = (part * 3) as usize;
  print_value_from_candidate_as_string(cell, prev, init, comparing);
  print!(" ");
  print_value_from_candidate_as_string(cell, prev, init + 1, comparing);
  print!(" ");
  print_value_from_candidate_as_string(cell, prev, init + 2, comparing);
}

fn print_value_from_candidate_as_string(cell: SudokuCell, prev: SudokuCell, n: usize, comparing: bool) {
  if cell.candidates[n] {
    print!("{}", (n + 1).cyan())
  } else if comparing && prev.candidates[n] {
    print!("{}", (n + 1).red())
  } else {
    print!(" ")
  }
}

fn print_full_board(board: SudokuBoard, prev: SudokuBoard, comparing: bool, opt: SudokuOptions) {
  for row in 0..9 {
    if row == 0 || row == 3 || row == 6 {
      print_board_line(true, comparing);
    } else {
      print_board_line(false, comparing);
    }
    for part in 0..3 {
      for col in 0..9 {
        if col == 0 || col == 3 || col == 6 {
          print!("||")
        } else {
          print!("|")
        }
        if comparing {
          print_full_cell(board.get_cell(row, col), prev.get_cell(row, col), part, comparing, opt);
        } else {
          print_full_cell(board.get_cell(row, col), board.get_cell(row, col), part, comparing, opt);
        }
      }
      println!("||");
    }
  }
  print_board_line(true, comparing);
}

fn print_board_line(hard: bool, comparing: bool) {
  if hard {
    let out = "++=====+=====+=====++=====+=====+=====++=====+=====+=====++";
    if comparing {
      println!("{}", out.green());
    } else {
      println!("{}", out);
    }
  } else {
    let out = "++-----+-----+-----++-----+-----+-----++-----+-----+-----++";
    println!("{}", out);
  }
}
