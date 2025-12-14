use std::io;

fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();

    let mut b = String::new();
    io::stdin().read_line(&mut b).unwrap();

    let num_a = a.trim().parse::<f64>().expect("Failed to convert to float");
    let num_b = b.trim().parse::<f64>().expect("Failed to convert to float");

    let result = (num_a * 3.5 + num_b * 7.5) / 11.0;
    println!("MEDIA = {:.5}", result);
}
