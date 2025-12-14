use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).unwrap();

    let x1y1: Vec<&str> = input.split_whitespace().collect();
    let x1 = x1y1[0]
        .trim()
        .parse::<f64>()
        .expect("error converting to float");
    let y1 = x1y1[1]
        .trim()
        .parse::<f64>()
        .expect("error converting to float");

    let x2y2: Vec<&str> = input2.split_whitespace().collect();
    let x2 = x2y2[0]
        .trim()
        .parse::<f64>()
        .expect("error converting to float");
    let y2 = x2y2[1]
        .trim()
        .parse::<f64>()
        .expect("error converting to float");

    let diff1 = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
    println!("{:.4}", diff1);
}
