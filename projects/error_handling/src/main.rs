use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{self, read_to_string, Read};
use std::io::ErrorKind;

// Returning Box<dyn Error> instead of io::Error allows us to return any type of error through the main function
fn main() -> Result<(), Box<dyn Error>> {
    // Basic panic! call
    // panic!("crash and burn!");

    // let v = vec![1, 2, 3];
    // v[99];

    // Try to open a file or creates it if not found
    //let greeting_file_result = File::open("hello.txt");;

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(file) => file,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         _ => {
    //             panic!("Problem opening the file: {error:?}");
    //         }
    //     },
    // };

    // Another way to achieve the same thing without match and cleaner
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });

    // With unwrap, calls panic! automatically if Result<> is an error
    let greeting_file = File::open("hello.txt").unwrap();
    // expect does the same, but you can pass it a specific error message
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");

    // This will not work if main would return ()
    // This works only if the method it's inside return Result<> or Option<>
    let greeting_file = fs::read_to_string("hello.txt")?;

    Ok(())
}

// Reads the username from a file
// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");
//
//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//
//     let mut username = String::new();
//
//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }
// Sam function as above using the ? operator
// fn read_username_from_file() -> Result<String, io::Error> {
//     // let mut username_file = File::open("hello.txt")?;
//     // let mut username = String::new();
//     // username_file.read_to_string(&mut username)?;
//     // Even shorter
//     let mut username = String::new();
//     File::open("hello.txt")?.read_to_string(&mut username)?;
//     Ok(username)
// }
// Using fs::read_to_string makes it even shorter again
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn last_car_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
