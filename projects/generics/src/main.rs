use std::fmt::Display;
use generics::{NewsArticle, SocialPost, Summary};

struct Point<T>{
    x: T,
    y: T,
}

impl<T> Point<T>{
    fn x(&self) -> &T{
        &self.x
    }
}

impl Point<f32>{
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct PointMix<T, U> {
    x: T,
    y: U,
}

impl<T, U> PointMix<T, U>{
    fn mixup<V, W>(self, other: PointMix<V, W>) -> PointMix<T, W> {
        PointMix{ x: self.x, y: other.y }
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self{
        Self{x, y}
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // first elision rule
    fn level(&self) -> i32 {
        3
    }

    // third elision rule
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    // let mut largest = &number_list[0];
    // for number in &number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    //let mixed = Point { x: 2, y: 5.0}; // Needs Point<T, U> to work
    println!("integer.x = {}, float.x = {}", integer.x(), float.x());
    println!("distance from origin = {}", float.distance_from_origin());

    let p1 = PointMix { x: 5, y: 10.4 };
    let p2 = PointMix { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };
    println!("1 new post: {}", post.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    // Is gonna use the default implementation of summarize
    println!("New article available! {}", article.summarize());

    let my_pair = Pair::new(5, 10);
    my_pair.cmp_display();

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");

        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");

        result = longest(string1.as_str(), string2.as_str());
    }
    // Doesn't work since string2 is out of scope even if it is string1 that is returned
    //println!("The longest string is {}", result);

    lifetime_test();

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");

        result = first(string1.as_str(), string2.as_str());
    }
    // Works here because the lifetime is only on the first parameter, so the second can get out of scope
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    let s: &'static str = "I have a static lifetime.";
}

// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];
//     for number in list {
//         if number > largest {
//             largest = number;
//         }
//     }
//     largest
// }

// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];
//     for char in list {
//         if char > largest {
//             largest = char;
//         }
//     }
//     largest
// }

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for char in list {
        if char > largest {
            largest = char;
        }
    }
    largest
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Function using a mix of lifetime and generic and trait
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display {
    println!("Announcement! {}", ann);
    if x.len() > y.len() { x } else { y }
}

// Only returns first parameter so no need of lifetime on the second
fn first<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

fn lifetime_test() {
    println!("Lifetime test function called!");

    // let string1 = String::from("abcd");
    // let string2 = String::from("xyz");
    // let result1 = longest(string1.as_str(), string2.as_str());
    // println!("The longest string is {}", result1);

    // let string1 = String::from("abcd");
    // {
    //     let string2 = String::from("xyz");
    //     let result1 = longest(string1.as_str(), string2.as_str());
    //     println!("The longest string is {}", result1);
    // }

    // let result1 = "a";
    // let string1 = String::from("abcd");
    // {
    //     let string2 = String::from("xyz");
    //     let result1 = longest(string1.as_str(), string2.as_str());
    //     println!("The longest string is {}", result1);
    // }
    // // Because of shadowing in the inner scope, the value was not modifies here
    // println!("The longest string is {}", result1);

    let string1 = String::from("abcd");
    let result1;
    {
        let string2 = String::from("xyz");
        result1 = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result1);
    }
    // Here, this doesn't work because string2 does not live long enough
    //println!("The longest string is {}", result1);

    println!("Lifetime test function ended");
}

// This was requiring lifetime before, but with some pattern recognition, the compiler let you not put it
// fn first_word<'a>(str: &'a str) -> &'a str {
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
