use std::io;

fn main(){
    let mut km = String::new();
    io::stdin().read_line(&mut km).unwrap();
    
    let mut l = String::new();
    io::stdin().read_line(&mut l).unwrap();
    
    let X = km.trim().parse::<f64>().expect("error converting to integer");
    let Y = l.trim().parse::<f64>().expect("error converting to float");
    
    let result = X / Y;
    println!("{:.3} km/l", result);
}