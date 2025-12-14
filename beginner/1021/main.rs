use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut value: i32 = (input.trim().parse::<f64>().unwrap() * 100.0).round() as i32;
    let notes = [10000, 5000, 2000, 1000, 500, 200];
    let coins = [100, 50, 25, 10, 5, 1];

    println!("NOTAS:");
    for note  in notes {
        let qty = value / note;
        value %= note;
        println!("{} nota(s) de R$ {:.2}", qty, note as f64 / 100.0);
    }

    println!("MOEDAS:");
    for coin in coins {
        let qty = value / coin;
        value %= coin;
        println!("{} moeda(s) de R$ {:.2}", qty, coin as f64 / 100.0);
    }
}
