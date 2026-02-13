use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error reading string");
    let values: Vec<f64> = input.split_whitespace().map(|s| s.parse().expect("error converting to float")).collect();
    
    let x = values[0];
    let y = values[1];

    if x > 0.0 && y > 0.0 {
        println!("Q1");
    } else if x > 0.0 && y < 0.0 {
        println!("Q4");
    } else if x < 0.0 && y > 0.0 {
        println!("Q2");
    } else if x < 0.0 && y < 0.0 {
        println!("Q3");
    } else if x == 0.0 && y == 0.0 {
        println!("Origem");
    } else if x == 0.0 && y != 0.0 {
        println!("Eixo Y");
    } else if x != 0.0 && y == 0.0 {
        println!("Eixo X");
    }
}