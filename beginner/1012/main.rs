use std::io;

const PI: f64 = 3.14159;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let values = input.trim();
    let parts: Vec<&str> = values.split_whitespace().collect();

    let a = parts[0].parse::<f64>().expect("Error parsing to float");
    let b = parts[1].parse::<f64>().expect("Error parsing to float");
    let c = parts[2].parse::<f64>().expect("Error parsing to float");

    let triangle = (a * c) / 2.0;
    let circle = PI * c * c;
    let trapezium = (a + b) * c / 2.0;
    let square = b * b;
    let rectangle = a * b;

    println!("TRIANGULO: {:.3}", triangle);
    println!("CIRCULO: {:.3}", circle);
    println!("TRAPEZIO: {:.3}", trapezium);
    println!("QUADRADO: {:.3}", square);
    println!("RETANGULO: {:.3}", rectangle);
}
