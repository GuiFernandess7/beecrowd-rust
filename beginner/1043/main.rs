use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error reading variable");
    let values: Vec<f64> = input.split_whitespace().map(|s| s.parse().expect("not a float")).collect();
    
    let a = values[0];
    let b = values[1];
    let c = values[2];
    
    let rule1 = a + b > c;
    let rule2 = a + c > b;
    let rule3 = b + c > a;
    
    if rule1 == true && rule2 == true && rule3 == true {
        let p = a + c + b;
        println!("Perimetro = {:.1}", p);
    } else {
        let area = (a + b) * c / 2.0;
        println!("Area = {:.1}", area);
    }
}