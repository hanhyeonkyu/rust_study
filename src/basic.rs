

pub fn variables() {
    // mutable && immutable diff
    // this is immutable
    let x = 5;
    // x = 3;
    // this is mutable
    // let mut y = 1;
    let y = 4;
    println!("x = {} && y = {}", x, y);

    // immutable cal
    let z = 5;
    let z = z + 1;
    let z = z * 2;
    println!("z is {}", z);

    // 1st case is str, 2nd case is number. so mutable value can't mutable data type.
    let spaces = "          ";
    let spaces = spaces.len();
    // let mut spaces = "           ";
    // let mut spaces = spaces.len();
    println!("spaces length is {}", spaces);

}