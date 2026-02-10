use std::io;

fn main(){
    let mut count = String::new();
    io::stdin().read_line(&mut count).expect("error reading variable");
    let count_value = count.trim().parse::<i64>().expect("error converting to integer");
    
    let mut vec_result: Vec<i64> = Vec::new();
    for i in 0..count_value {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error reading variable");
        let values: Vec<i64> = input.split_whitespace().map(|s| s.parse().expect("cannot convert to integer")).collect();
        
        let mut r = values[0];
        let mut v = values[1];
        
        while v != 0 {
            let rest = r % v;
            r = v;
            v = rest;
        }
        vec_result.push(r);
    }
    
    for v in vec_result {
        println!("{}", v);
    }
}
