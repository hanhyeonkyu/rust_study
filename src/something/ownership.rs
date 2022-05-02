pub fn scopes() {
    basicscope();
    scopemoving();
    borrowing();
}

fn borrowing() {
    let s1 = String::from("hello!@");
    let len = calculate_length(&s1);
    println!("length of {} is {}", s1, len);
}
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn scopemoving() {
    let s1 = gives_ownership();         // gives_ownership moves its return value into s1
    let s2 = String::from("hello");     // s2 comes into scope
    let s3 = takes_and_gives_back(s2);  // s2 is moved into takes_and_gives_back, which also moves its return value into s3
    println!("s1 : {} s3 : {}", s1, s3);
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was moved, so nothing happens. s1 goes out of scope and is dropped.
fn gives_ownership() -> String {             // gives_ownership will move its return value into the function that calls it
    let some_string = String::from("hello"); // some_string comes into scope
    some_string                              // some_string is returned and moves out to the calling function
}
// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string  // a_string is returned and moves out to the calling function
}

fn basicscope() {
    // mutable && str.push_str() function used
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    // scopes
    let s1 = String::from("hello");
    let s2 = s1;
    // Auxilary saved in String::from() and then it have already moved to s2!
    // println!("{}, world!", s1);
    // but you can clone!
    let s1 = s2.clone();
    println!("{}, world!", s1);
    println!("{}, world!", s2);
    // it's possible way easily
    let p = 5;
    let q = p;
    println!("p = {} && q = {}", p, q);
}