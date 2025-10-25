fn main() {
    let x = two();
    print_labeled_measurement(x, 'h');
}

fn print_labeled_measurement(unit: i32, unit_label: char) {
    println!("The measurement is: {unit}{unit_label}");
}

fn two() -> i32 {
    let x = 1 + 1;
    x
}