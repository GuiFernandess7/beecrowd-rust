use std::io;

const PI: f64 = 3.14159;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let r = input.trim().parse::<f64>().unwrap();
    let result = (4.0 / 3.0) * PI * r.powi(3);
    println!("VOLUME = {:.3}", result);
}
