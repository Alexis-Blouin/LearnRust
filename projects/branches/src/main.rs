fn main() {
    let number = 5;

    if number == 3 {
        println!("The number is three!");
    } else {
        println!("The number is not three.");
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
