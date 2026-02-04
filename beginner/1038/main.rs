use std::io;
use std::collections::HashMap;

fn main(){
    let mut items: HashMap<i32, f64> = HashMap::new();
    items.insert(1, 4.0);
    items.insert(2, 4.5);
    items.insert(3, 5.0);
    items.insert(4, 2.0);
    items.insert(5, 1.5);

    let mut input_values = String::new();
    io::stdin().read_line(&mut input_values).expect("error reading variable");

    let mut values: Vec<&str> = input_values.split_whitespace().collect();
    let code = values[0].trim().parse::<i32>().expect("error converting to integer");
    let amount = values[1].trim().parse::<f64>().expect("error converting to integer");

    if let Some(price) = items.get(&code) {
        println!("Total: R$ {:.2}", *price * amount);
    }
}
