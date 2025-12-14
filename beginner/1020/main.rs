use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let total_days = input
        .trim()
        .parse::<i64>()
        .expect("Error converting to integer");

    let years = total_days / 365;
    let rem_days = total_days % 365;

    let months = rem_days / 30;
    let days = rem_days % 30;

    println!("{} ano(s)", years);
    println!("{} mes(es)", months);
    println!("{} dia(s)", days);
}
