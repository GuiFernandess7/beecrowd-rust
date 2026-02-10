use std::io;

fn main(){
    let mut counter = 0;
    for i in 0..5 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error reading variable");
        let value = input.trim().parse::<i64>().expect("error converting to integer");
        
        if value % 2 == 0 {
            counter += 1;
        }
    }
    println!("{} valores pares", counter);
}