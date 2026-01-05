use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let number = input.trim().parse::<f64>().expect("error converting to float");

    if number <= 25.0 && number >= 0.0 {
        println!("Intervalo [0,25]");
    } else if number > 25.0 && number <= 50.0 {
        println!("Intervalo (25,50]");
    }  else if number > 50.0 && number <= 75.0 {
        println!("Intervalo (50,75]");
    } else if number > 75.0 && number <= 100.0 {
        println!("Intervalo (75,100]");
    } else if number > 100.0 || number < 0.0 {
        println!("Fora de intervalo");
    }
}