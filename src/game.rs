use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub fn guess() {
    // mut 는 mutable 가변의 / mut 가 없이 선언 시 immutable 값이 됨.
    // .expect 를 통해 try catch 기능의 catch 기능을 함.
    println!("Guess number!");
    let secret_number = rand::thread_rng().gen_range(1, 11);
    // println!("The secret number is: {}", secret_number);
    println!("Please insert your guess");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("failed to read line");
    let guessed: u32 = guess
        .trim()
        .parse()
        .expect("faild to change number");
    println!("you guessed: {}", guess);
    match guessed.cmp(&secret_number) {
        Ordering::Less => println!("Too Small!"),
        Ordering::Greater => println!("Too Big!"),
        Ordering::Equal => println!("You win!"),
    };
    println!("secret number is: {}", secret_number);
    
    // string 에서 {}를 넣고 뒤에 변수를 넣어주면 그 값을 반환함.
    // y = 12; 는 에러를 반환시킴 mut 를 붙이지 않았기 때문에 immutable 이기 떄문.
    // let x = 5;
    // let y = 10;
    // println!("x={} and y={}", x, y);
}

