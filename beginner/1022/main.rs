use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let values: Vec<&str> = input.split_whitespace().collect();
    let a = values[0].trim().parse::<i64>().expect("error converting to integer");
    let b = values[1].trim().parse::<i64>().expect("error converting to integer");
    let c = values[2].trim().parse::<i64>().expect("error converting to integer");
    let d = values[3].trim().parse::<i64>().expect("error converting to integer");
    
    let b_greater_than_c = b > c;
    let d_greater_than_a = d > a;
    let sum_greater_than = (c + d) > (a + b);
    let positives = (c > 0) && (d > 0);
    let a_even = a % 2 == 0;
    
    if b_greater_than_c && d_greater_than_a && sum_greater_than && positives && a_even{
        println!("Valores aceitos");
    } else {
        println!("Valores nao aceitos");
    }
}