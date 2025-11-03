use rand::Rng;
use crate::garden::vegetables::Asparagus;

// use std::cmp::Ordering;
// use std::io;
// Nested path of above
// use std::{cmp::Ordering, io};

mod garden;

fn main() {
    let marty = Asparagus { name: String::from("Marty") };
    println!("My vegetable: {marty:?}");

    let secret_number = rand::rng().random_range(1..101);
}
