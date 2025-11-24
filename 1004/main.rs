use std::io;

fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();

    let mut b = String::new();
    io::stdin().read_line(&mut b).unwrap();

    let num_a = a.trim().parse::<i32>().expect("Failed to convert to float");
    let num_b = b.trim().parse::<i32>().expect("Failed to convert to float");

    let PROD = num_a * num_b;
    println!("PROD = {}", PROD);
}
