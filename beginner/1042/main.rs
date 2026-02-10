use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error reading variable");
    let values_original: Vec<&str> = input.split_whitespace().collect();
    let mut values: Vec<i64> = values_original.iter().map(|s| s.parse().expect("not a integer")).collect();
    values.sort();

    for v in values {
        println!("{}", v);
    }
    println!("");
    
    for v in values_original {
        println!("{}", v);
    }
}
