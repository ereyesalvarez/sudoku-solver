
pub fn print(arr: [[u8; 9]; 9]) {
    for row in arr{
      for val in row{
        if val == 0{
          print!("_ ");
        } else {
          print!("{} ", val);
        }
      }
      println!("");
    }
  }