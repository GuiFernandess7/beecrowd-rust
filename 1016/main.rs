use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let distance = input
        .trim()
        .parse::<f64>()
        .expect("error converting to float");
    let result = distance * 2.0;
    println!("{} minutos", result);
}
