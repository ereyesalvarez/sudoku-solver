use owo_colors::OwoColorize;

pub enum SudokuStep {
    ClearBoard,
    ClearByTuple,
    ResolveDirect,
    ResolveInfer
}

pub fn print_intro(){
    println!("{}","Bienvenido al sudoku resolutor");
    println!("{}","Introduce los numeros separados por espacios".green());
    println!("{}","Conjuntos de dos o más espacios  provocaran error.".green());
    println!("{}","Para indicar un numero no conocido indicar con x.".green());
    println!("{}","Pulsa enter para saltar de linea, se esperan 9 lineas.".green());
}