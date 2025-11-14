use std::collections::HashMap;

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

    let mut s = String::new();

    let data = "initial contents";
    println!("{data}");

    let s = data.to_string();
    // Also works directly on the literal
    let s = "initial contents".to_string();
    let s = String::from("initial contents");
    println!("{s}");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1: {s1}, s2: {s2}");

    let mut s = String::from("lo");
    s.push('s');
    println!("{s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // After this, s1 is no longer available as we took ownership of it
    let s3 = s1 + &s2;
    //let s3 = &s1 + &s2; // And this is not working because add(self, s: &str) -> Sting
    println!("{s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // Pretty unwieldy with the + operator
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{s}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // Better and no loose of ownership
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    let h1 = String::from("hi");
    //let h = hi[0]; // Not working
    let h = h1.chars().nth(0).unwrap(); // unwrap() returns the contained Some value
    println!("{}", h);

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    //let s = &hello[0..1]; // Would panic because it's slicing part of a character
    println!("{}", s); // Prints Зд because each char is 2 bytes here

    for c in "Здравствуйте".chars(){
        println!("{c}");
    }
    // Prints the raw bytes
    for b in "Зд".bytes() {
        println!("{b}");
    }

    let mut scores = HashMap::new();
    // Can do this if we don't have direct inserts after
    // let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Red"), 25);
    println!("scores = {:?}", scores);

    let team_name = String::from("Blue");
    // copied allows to not get a reference and unwrap gets the value or 0 if no score
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{team_name} score is {score}");

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("map = {:?}", map);
    // These fail because String doesn't have the Copy trait
    // println!("{field_name}");
    // println!("{field_value}");

    // Overwrites original value of 'Blue'
    scores.insert(String::from("Blue"), 25);
    println!("scores = {:?}", scores);

    // Checks if value exists before inserting so no overwrite of blue
    scores.entry(String::from("Blue")).or_insert(65);
    scores.entry(String::from("Orange")).or_insert(90);
    println!("scores = {:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1; // Dereferences so modifies the value in map
    }
    println!("map = {:?}", map);
}
