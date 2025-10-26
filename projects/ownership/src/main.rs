fn main() {
    let s = String::from("hello world");
    take_ownership(&s);
    println!("{s}");

    let i = 5;
    make_copy(i);
    println!("{i}");

    let s1 = give_ownership();
    println!("{s1}");
    let s2 = String::from("hello");
    let s3 = take_and_gives_back(s2);
    println!("{s3}");

    let new_string = String::from("hello, world!");
    let (s2, len) = calculate_length(new_string);
    println!("The length of '{}' is {}.", s2, len);
    let len = calculate_length_ref(&s2);
    println!("The length of '{}' is {}.", s2, len);

    let mut s = String::from("lololo");
    change(&mut s);
    println!("{s}");
}

fn take_ownership(some_string: &String) {
    println!("{}", some_string);
}

fn make_copy(some_interger: i32) {
    println!("{}", some_interger);
}

fn give_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn take_and_gives_back(a_string: String) -> String {
    println!("{}", a_string);
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(" lalala");
}
