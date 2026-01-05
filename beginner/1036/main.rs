use std::io;

fn main(){
    let mut input_row = String::new();
    io::stdin().read_line(&mut input_row).unwrap();

    let mut values: Vec<&str> = input_row.split_whitespace().collect();
    let a = values[0].trim().parse::<f64>().expect("error converting to float");
    let b = values[1].trim().parse::<f64>().expect("error converting to float");
    let c = values[2].trim().parse::<f64>().expect("error converting to float");

    let delta = b.powi(2) - 4.0 * a * c;

    if delta < 0.0 || a == 0.0 {
        println!("Impossivel calcular");
        return;
    }

    let sqrt_delta = delta.sqrt();

    let x1 = (-b + sqrt_delta) / (2.0 * a);
    let x2 = (-b - sqrt_delta) / (2.0 * a);

    println!("R1 = {:.5}", x1);
    println!("R2 = {:.5}", x2);
}
