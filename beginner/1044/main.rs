use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error reading string");
    let values: Vec<f64> = input.split_whitespace().map(|s| s.parse().expect("error converting to float")).collect();

    let x = values[0];
    let y = values[1];

    if x % y == 0.0 || y % x == 0.0 {
        println!("Sao Multiplos");
    } else {
        println!("Nao sao Multiplos");
    }
}