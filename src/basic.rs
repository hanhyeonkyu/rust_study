pub fn controlflow() {
    iffunc(5);
    ifletfunc(true);
    loopfunc(10);
    whilefunc();
    forloop();
    forloop2();
}
fn forloop2() {
    for number in (1..4).rev() {
        println!("forloop2 {}", number);
    }
    println!("forloop2 over");
}
fn forloop() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("forloop {}", element);
    }
    println!("forloop over");
}
fn whilefunc() {
    let mut number = 3;
    while number != 0 {
        println!("whileloop{}", number);
        number -= 1;
    }
    println!("whileloop over");
}
fn loopfunc(cnt: i32) {
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == cnt {
            break count * 2;
        }
    };
    println!("{}", result);
}
fn ifletfunc(std: bool){
    let iflet = if std {
        5
    } else {
        6
    };
    println!("{}", iflet);
}
fn iffunc(x: i32) {
    if x > 10 {
        println!("upper ten");
    } else if x == 5 {
        println!("it is five");
    } else if x < 5 {
        println!("lower five");
    } else {
        println!("else");
    }
}

pub fn functypes() {
    func0();
    func1(4);
    func2();
    let x = func3(3);
    println!("arrow type func3 is {}", x);
}
fn func0() {
    println!("no parameter func")
}
fn func1(x: i32) {
    println!("parameter type func is {}", x)
}
fn func2() {
    // when you declare inside you can change immut value
    let y = {
        let x = 3;
        x + 1
    };
    println!("declare type func2 is {}", y)
}
fn func3(x: i32) -> i32 {
    // write what you want to return
    x + 7
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