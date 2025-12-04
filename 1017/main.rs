use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).unwrap();

    let hours = input
        .trim()
        .parse::<f64>()
        .expect("error converting to float");
    let avg_speed_kmh = input2
        .trim()
        .parse::<f64>()
        .expect("error converting to float");

    let distance = hours * avg_speed_kmh;
    let fuel = distance / 12.0;
    println!("{:.3}", fuel);
}
