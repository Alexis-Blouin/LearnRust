fn main() {
    let c_temp = 32.0;
    let f_temp = c_to_f(c_temp);
    println!("{:.1}c is {:.1}f", c_temp, f_temp);

    let f_temp = 99.0;
    let c_temp = f_to_c(f_temp);
    println!("{:.1}f is {:.1}c", f_temp, c_temp);
}

fn c_to_f(c: f32) -> f32 {
    c * 9.0 / 5.0 + 32.0
}

fn f_to_c(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}
