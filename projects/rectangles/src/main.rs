#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    // let width1 = 30;
    // let height1 = 50;

    // println!(
    //     "The area of the rectangle is {} square units.",
    //     area(width1, height1)
    // );

    // let rect1 = (30, 50);

    // println!(
    //     "The area of the rectangle is {} square units.",
    //     area(rect1)
    // );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:#?}");
    dbg!(&rect1); // need to pass a reference cause taking ownership
    //let rect1 = dbg!(rect1); // this works too, but dbg! takes ownership

    // println!(
    //     "The area of the rectangle is {} square units.",
    //     area(&rect1)
    // );

    println!(
        "The area of the rectangle is {} square units.",
        rect1.area()
    );

    println!("The rectangle has a nonzero width: {}", rect1.width());

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
    println!("Square is {sq:#?}");
    println!("Can rect1 hold square? {}", rect1.can_hold(&sq));
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
