use std::io;
use std::collections::HashMap;

fn main(){
    let mut count = String::new();
    io::stdin().read_line(&mut count).expect("error reading input");
    let count_i = count.trim().parse::<i32>().expect("error reading integer");

    let mut total_count = 0;
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert(String::from("C"), 0);
    map.insert(String::from("R"), 0);
    map.insert(String::from("S"), 0);

    for _ in 0..count_i { 
        let mut test = String::new();
        io::stdin().read_line(&mut test).expect("error reading input");
        let values: Vec<&str> = test.split_whitespace().collect();
        let quantity = values[0].trim().parse::<i32>().expect("error converting to integer");
        map.entry(values[1].to_string()).and_modify(|q| *q += quantity); // Precisa converter para de &str para String, para manter na memória e atribuir ao hashmap.
        total_count += quantity;
    }

    let coelhos = map.get("C").unwrap_or(&0);
    let ratos = map.get("R").unwrap_or(&0);
    let sapos = map.get("S").unwrap_or(&0);
    let total_f = total_count as f64;

    println!("Total: {} cobaias", total_count);
    println!("Total de coelhos: {}", coelhos);
    println!("Total de ratos: {}", ratos);
    println!("Total de sapos: {}", sapos);

    println!("Percentual de coelhos: {:.2} %", (*coelhos as f64 / total_f) * 100.0); // Precisa desrefernciar pois "coelhos" não é um número, mas sim um Option<i32> (referencia).
    println!("Percentual de ratos: {:.2} %", (*ratos as f64 / total_f) * 100.0);
    println!("Percentual de sapos: {:.2} %", (*sapos as f64 / total_f) * 100.0);
}