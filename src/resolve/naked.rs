use crate::board_util::{contains_candidates_in_list, get_box_range_from_pos, get_candidates_from_cell, get_range_of_box};
use crate::sudoku_types::{SudokuBoard, SudokuCell, SudokuCellType};

pub(super) fn naked_single(mut board: SudokuBoard) -> (SudokuBoard, isize) {
    let mut hits: isize = 0;
    for x in 0..9 {
        for y in 0..9 {
            let cell = board.get_cell(x, y);
            if cell.is_empty() {
                let candidates = get_candidates_from_cell(cell);
                if candidates.len() == 1 {
                    board.set_guess(*candidates.get(0).unwrap(), x, y);
                    hits += 1;
                }
            }
        }
    }
    return (board, hits);
}

pub(super) fn naked_pair(mut board: SudokuBoard) -> (SudokuBoard, isize) {
    let mut hits = 0;
    for a in 1..10 {
        for b in a..10 {
            let  (aux_board, aux_hit) = naked_tuple_check(board, vec![a, b]);
            board = aux_board;
            hits += aux_hit;
        }
    }
    (board, hits)
}

pub(super) fn naked_tuple_check(mut board: SudokuBoard, candidates: Vec<usize>) -> (SudokuBoard, isize) {
    let mut hits = 0;
    // Check if contained in row
    // Check if contained in col
    // Check if contained in box
    let len = candidates.len();
    for axis_a in 0..9 {
        let mut occurrences_row: u8 = 0;
        let mut occurrences_col: u8 = 0;
        for axis_b in 0..9 {
            if contains_candidates_in_list(board.get_cell(axis_a, axis_b), &candidates) {
                occurrences_row += 1;
            }
            if contains_candidates_in_list(board.get_cell(axis_b, axis_a), &candidates) {
                occurrences_col += 1;
            }
            if occurrences_row == len as u8 {
                let  (aux_board, aux_hit) = clean_candidates_line_naked(board, &candidates, axis_a, true);
                board = aux_board;
                hits += aux_hit;
            }
            if occurrences_col == len as u8 {
                let (aux_board, aux_hit) = clean_candidates_line_naked(board, &candidates, axis_a, false);
                board = aux_board;
                hits += aux_hit;
            }
        }
    }
    (board, hits)
}

fn clean_candidates_line_naked(mut board: SudokuBoard, candidates: &Vec<usize>, axis: usize, clean_row: bool) -> (SudokuBoard, isize) {
    let mut hits = 0;
    let mut delete = false;
    for axis_b in 0..9 {
        let x = if clean_row {
            axis
        } else {
            axis_b
        };
        let y = if clean_row {
            axis_b
        } else {
            axis
        };
        if contains_candidates_in_list(board.get_cell(x, y), &candidates) {
            for n in candidates {
                if board.board[x][y].candidates[n - 1] {
                    hits += 1;
                    board.board[x][y].candidates[n - 1] = false;
                }
            }
        }
    }
    (board, hits)
}

fn clean_candidates_box_naked(mut board: SudokuBoard, base_x: usize, base_y: usize) -> (SudokuBoard, isize) {
    let mut hits = 0;
    let box_range = get_box_range_from_pos(base_x, base_y);
    for x in box_range.0 {
        for y in box_range.1 {
            // ToDo: Check if need to repeat
            let candidates: Vec<usize> = get_candidates_from_cell(board.get_cell(base_x, base_y));
            let is_match = contains_candidates_in_list(board.get_cell(x, y), &candidates);
            if !is_match {
                // tenemos que eliminar todos lo que no estan presentes
                let to_delete_candidates = get_candidates_from_cell(board.get_cell(x, y));
                for n in to_delete_candidates {
                    if board.board[x][y].candidates[n - 1] {
                        hits += 1;
                    }
                    board.board[x][y].candidates[n - 1] = false;
                }
            }
        }
    }
    return (board, hits);
}


// ToDo: End single
// ToDo: naked pair, triples and Quads
// En una unidad se encuentra un numero
// En una columna, fila o caja se encuentran para las veces que aparece conjuntos de n tuplas
// No tienen por que aparecer, pero no puede haber ninguno mas
