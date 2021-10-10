// ToDo: Hidden single, pair, triples, quad
// En una fila, columna o caja hay un conjunto de numeros que solo aparecen n veces, siendo n el numero de repeticiones
// para que el sistema funcione tienen que estar todos o al menos uno.
// p. e. 1, 5, 6 - 5, 6 9 - 3, 6  aunque el 5 no aparezca en las tres ocasiones
// si ni el 5 ni el 6 estan en ninguna otra casilla del conjunto podemos borrar el resto de numeros de las celdas en las que aparecen
// El orden de probabilidad es 1, 2, 3, 4
// En el caso de Single tambien se conoce como pinned y simplemente setea


use crate::board_util::{get_candidates_from_cell, get_inverse_range_from_pos};
use crate::sudoku_types::{SudokuBoard, SudokuCell};

fn hidden(mut board: SudokuBoard) -> (SudokuBoard, isize) {
  let mut hits = 0;
  return (board, hits);
}
