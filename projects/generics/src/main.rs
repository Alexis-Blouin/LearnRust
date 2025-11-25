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
