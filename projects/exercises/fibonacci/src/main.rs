use std::io;

fn main() {
    let mut n = String::new();

    println!("Please input the nth Fibonacci number you want.");
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: i32 = match n.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("You did not enter a valid number, closing program...");
            return;
        }
    };

    let num = fibonacci(n);
    println!("The {n}nth Fibonacci number is {num}");
}

fn fibonacci(n: i32) -> i32 {
    let mut a;
    let mut b = 1;
    let mut next = b;
    for _count in 1..(n + 1) {
        a = b;
        b = next;
        next = a + b;
    }
    b
}
