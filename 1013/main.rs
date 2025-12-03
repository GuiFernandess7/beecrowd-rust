use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let values: Vec<&str> = input.trim().split_whitespace().collect();
    let a = values[0].parse::<f64>().expect("Error converting to float");
    let b = values[1].parse::<f64>().expect("Error converting to float");
    let c = values[2].parse::<f64>().expect("Error converting to float");

    let abs = (a - b).abs();
    let biggestAB = (a + b + abs) / 2.0;
    let biggest = (biggestAB + c + (biggestAB - c).abs()) / 2.0;

    println!("{} eh o maior", biggest);
}
