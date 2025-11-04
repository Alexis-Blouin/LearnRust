fn main() {
    // Creates empty Vector with i32 type values
    let v: Vec<i32> = Vec::new();
    // Creates Vector with i32 type values inferred from the values (vec! is a macro)
    let v = vec![1, 2, 3];
    // No need to hard type, because we put data inside so Rust infers again
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // let does_not_exists = &v[100]; // Will panic because there is no 101st element
    let does_not_exist = v.get(100); // Will return None
    match does_not_exist {
        Some(elem) => println!("The 101st element is {elem}"),
        None => println!("There is no 101st element."),
    }

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0]; // Immutable borrow
    println!("The first element is {first}");

    // This works if the print is placed before, but first will be not working after.
    v.push(6); // Mutable borrow

    // Doesn't work if print is here because there is a risk of memory being re-allocated so first would point nowhere
    //println!("The first element is {first}");

    let v = vec![100, 32, 57];

    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{i}");
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for item in &row {
        match item {
            SpreadsheetCell::Int(i) => println!("i32: {i}"),
            SpreadsheetCell::Float(i) => println!("f64: {i}"),
            SpreadsheetCell::Text(i) => println!("String: {i}"),
        }
    }
}
