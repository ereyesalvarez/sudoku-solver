/*
only two possible cells for a value in each of two different rows,
and these candidates lie also in the same columns,
then all other candidates for this value in the columns can be eliminated.
*/
// ToDo:
/*
en dos filas o columnas tiene que haber solo dos ocurrencias de un candidato.
Cada uno de los pares de un candidato debe estar conectado a uno de la otra fila o columna.
Se puede conectar por fila, columna o caja.
Incluso se podrian conectar dos cajas por filas

Ejemplo de conexion por caja ( se eliminaria el candidato de la fila dos.
- - x | - - x | - - -
- - - | x x x | x - x
- x - | - x - | - - -

Ejemplo de conexion de caja por file

- - - | - - - | - - -
x - - | - - - | - - -
- - - | - - - | - - -

Posibilidades:


    Starting from 2 rows and eliminating in 2 columns (classic)
    Starting from 2 columns and eliminating in 2 rows (classic)
    Starting from 2 boxes and eliminating in 2 rows (intersection)
    Starting from 2 boxes and eliminating in 2 columns (intersection)
    Starting from 2 rows and eliminating in 2 boxes (pointing pair)
    Starting from 2 columns and eliminating in 2 boxes (pointing pair)




    Classic X-Wing
    Classic X-Wing
    Same effect as line/box reduction
    Same effect as line/box reduction
    Same effect as pointing pairs
    Same effect as pointing pairs


*/