use std::io;

fn main() {
    let mut a = String::new();
    let a_weight = 2.0;
    io::stdin().read_line(&mut a).unwrap();

    let mut b = String::new();
    let b_weight = 3.0;
    io::stdin().read_line(&mut b).unwrap();

    let mut c = String::new();
    let c_weight = 5.0;
    io::stdin().read_line(&mut c).unwrap();

    let A = a.trim().parse::<f64>().expect("Failed to convert to float");
    let B = b.trim().parse::<f64>().expect("Failed to convert to float");
    let C = c.trim().parse::<f64>().expect("Failed to convert to float");

    let result = (A * a_weight + B * b_weight + C * c_weight) / 10.0;
    println!("MEDIA = {:.1}", result);
}
