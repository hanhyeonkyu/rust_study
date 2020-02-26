use std::io;

fn main() {
    variables()
}

fn variables() {
    // mut 는 mutable 가변의 / mut 가 없이 선언 시 immutable 값이 됨.
    // .expect 를 통해 try catch 기능의 catch 기능을 함.
    println!("Guess number!");
    println!("Please insert your guess");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("failed to read line");
    println!("you guessed: {}", guess);
    // string 에서 {}를 넣고 뒤에 변수를 넣어주면 그 값을 반환함.
    let x = 5;
    let y = 10;
    // y = 12; 는 에러를 반환시킴 mut 를 붙이지 않았기 때문에 immutable 이기 떄문.
    println!("x={} and y={}", x, y);
}
