use crate::sudoku_util::set_val;
use crate::sudoku_input::process_input_line;
use crate::sudoku_process::create_board;
use crate::sudoku_process::SudokuCell;

pub fn fake() -> [[SudokuCell; 9]; 9] {
    let mut board = create_board();
    // input, x, y,
    process_input_line(&mut board, String::from("9 x x x 8 2 5"), 0);
    process_input_line(&mut board, String::from("1 2 x 5 4 7 6 8 9"), 1);
    process_input_line(&mut board, String::from("5 4 x x x 6 2 x x"), 2);

    process_input_line(&mut board, String::from("7 8 x 6 1 5 x x x"), 3);
    process_input_line(&mut board, String::from("x x 4 9 2 8 7 x x"), 4);
    process_input_line(&mut board, String::from("x x 5 x x 4 1 2 8"), 5);
    
    process_input_line(&mut board, String::from("8 3 x x 7 x 4 6 5"), 6);
    process_input_line(&mut board, String::from("x x 9 x x 3 8 1 x"), 7);
    process_input_line(&mut board, String::from("4 5 7 x x 1 9 3 x"), 8);
    return board;
}



    