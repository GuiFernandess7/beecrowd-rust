use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut input_value = input.trim().parse::<i64>().unwrap();
    let hours = input_value / 3600;
    let mut remaining_seconds = input_value % 3600;

    let minutes = remaining_seconds / 60;
    let seconds = remaining_seconds % 60;
    println!("{}:{}:{}", hours, minutes, seconds);
}
