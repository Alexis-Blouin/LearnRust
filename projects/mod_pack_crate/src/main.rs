use crate::garden::vegetables::Asparagus;

mod garden;

fn main() {
    let marty = Asparagus { name: String::from("Marty") };
    println!("My vegetable: {marty:?}");
}
