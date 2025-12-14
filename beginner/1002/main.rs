use std::io;

fn main() {
    const N: f64 = 3.14159;

    let mut r = String::new();
    io::stdin().read_line(&mut r).unwrap();

    let num_r = r.trim().parse::<f64>().expect("Failed to convert to float");

    let a: f64 = N * num_r.powi(2);
    println!("A={:.4}", a);
}
