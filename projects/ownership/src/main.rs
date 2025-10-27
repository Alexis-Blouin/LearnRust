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

    let s = String::from("hello world");
    let word = first_word(&s.as_str()); // or &s[..]
    println!("The first word is {word}");
    let second = second_word(&s[..]);
    println!("The second word is {second}");
    let single = String::from("single word");
    let empty = second_word(&single[2..8]);
    println!("The second word is '{empty}'");
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

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut space_index = 0;
    let mut space_found = false;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if space_found {
                return &s[space_index + 1..i];
            } else {
                space_index = i;
                space_found = true;
            }
        }
    }

    if space_found {
        &s[space_index + 1..]
    } else {
        &s[0..0] // return an empty string slice if there's no second word
    }
}
