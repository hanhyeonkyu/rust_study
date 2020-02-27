pub fn functypes() {
    
}

pub fn datatypes() {
    // when you assign data, you can declare type and parse type and treat exception
    let guess: u32 = "42".parse().expect("not a number!");
    println!("guess is {}", guess);
    let (x, y) = (2.5, 3.3);
    println!("x is {} && y is {}", x, y);

    // can use tuple type!
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (p, q, r) = tup;
    let fivehundred = tup.0;
    println!("p is {} && q is {} && r is {} && {}", p, q, r, fivehundred);

    // array type!
    let a = [1,2,3,4,5];
    let b: [i32; 5] = [1,2,3,4,5];
    let c = [3; 5];
    println!("{} && {} && {}", a[1], b[2], c[4]);
}

pub fn mutvsimmut() {
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