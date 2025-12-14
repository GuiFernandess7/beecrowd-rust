use std::io;

fn main() {
    let mut total_all = 0.0;
    let mut count = 2;

    loop {
        if count == 0 {
            break;
        }
        let mut values_to_pay = String::new();
        io::stdin().read_line(&mut values_to_pay).unwrap();

        let values = values_to_pay.trim();
        let parts: Vec<&str> = values.split_whitespace().collect();

        let value_A = parts[1].parse::<f64>().expect("Erro parsing value A");
        let value_B = parts[2].parse::<f64>().expect("Erro parsing value B");

        let total_p = value_A * value_B;
        total_all += total_p;
        count -= 1
    }
    println!("VALOR A PAGAR: R$ {:.2}", total_all);
}
