use super::*;
use super::sudoku_process::*;
use super::sudoku_util::*;

#[test]
fn create_board_test() {
    let board: [[SudokuCell; 9]; 9] = create_board();
    for row in board {
        for val in row {
            assert_eq!(val.value, 0, "values don't match");
            let same_type = val.cell_type == SudokuCellType::Empty;
            assert!(same_type, "type don't match");
        }
    }
}

#[test]
fn set_val_test() {
    let mut board: [[SudokuCell; 9]; 9] = create_board();
    set_val(&mut board, 5, 4, 3);
    let mut index = 0;
    for row in board {
        if index == 4 {
            let val = row[3];
            assert_eq!(val.value, 5, "values don't match");
            let same_type = val.cell_type == SudokuCellType::Fixed;
            assert!(same_type, "type don't match");
        } else{
            for val in row {
                assert_eq!(val.value, 0, "values don't match");
                let same_type = val.cell_type == SudokuCellType::Empty;
                assert!(same_type, "type don't match");
            }
        }
        index += 1;
    }
}

#[test]
fn set_guess_test() {
    let mut board: [[SudokuCell; 9]; 9] = create_board();
    set_guess(&mut board, 6, 4, 3);
    let mut index = 0;
    for row in board {
        if index == 4 {
            let val = row[3];
            assert_eq!(val.value, 6, "values don't match");
            let same_type = val.cell_type == SudokuCellType::Guess;
            assert!(same_type, "type don't match");
        } else{
            for val in row {
                assert_eq!(val.value, 0, "values don't match");
                let same_type = val.cell_type == SudokuCellType::Empty;
                assert!(same_type, "type don't match");
            }
        }
        index += 1;
    }
}

#[test]
fn calculate_quarter_test() {
    assert_calculate_quarter(0, 0, 0);
    assert_calculate_quarter(1, 1, 0);
    assert_calculate_quarter(2, 2, 0);
    assert_calculate_quarter(3, 0, 3);
    assert_calculate_quarter(8, 8, 8);
    assert_calculate_quarter(5, 8, 5);
}

fn assert_calculate_quarter(x: usize, y: usize, expected: usize){
    let result = calculate_quarter(x, y);
    assert_eq!(result, expected, "values don't match");
}