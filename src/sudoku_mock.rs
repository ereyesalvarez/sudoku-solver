use crate::sudoku_util::set_val;

pub fn fake() -> [[SudokuCell; 9]; 9] {
    let mut board = create_board();
    // input, x, y,
    process_input_line(&mut board, "9 x x x 8 2 5", 0);
    process_input_line(&mut board, "1 2 x 5 4 7 6 8 9", 1);
    process_input_line(&mut board, "5 4 x x x 6 2 x x", 2);

    process_input_line(&mut board, "7 8 x 6 1 5 x x x", 3);
    process_input_line(&mut board, "x x 4 9 2 8 7 x x", 4);
    process_input_line(&mut board, "x x 5 x x 4 1 2 8", 5);
    
    process_input_line(&mut board, "8 3 x x 7 x 4 6 5", 6);
    process_input_line(&mut board, "x x 9 x x 3 8 1 x", 7);
    process_input_line(&mut board, "4 5 7 x x 1 9 3 x", 8);
    return board;
}



    