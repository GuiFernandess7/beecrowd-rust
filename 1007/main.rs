use std::io;

fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();

    let mut b = String::new();
    io::stdin().read_line(&mut b).unwrap();

    let mut c = String::new();
    io::stdin().read_line(&mut c).unwrap();

    let mut d = String::new();
    io::stdin().read_line(&mut d).unwrap();

    let A = a.trim().parse::<f64>().expect("Failed to convert to float");
    let B = b.trim().parse::<f64>().expect("Failed to convert to float");
    let C = c.trim().parse::<f64>().expect("Failed to convert to float");
    let D = d.trim().parse::<f64>().expect("Failed to convert to float");

    let result = (A * B - C * D);
    println!("DIFERENCA = {}", result);
}
