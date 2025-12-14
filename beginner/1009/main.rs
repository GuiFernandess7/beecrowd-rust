use std::io;

fn main() {
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    let mut salary = String::new();
    io::stdin().read_line(&mut salary).unwrap();

    let mut money_by_month = String::new();
    io::stdin().read_line(&mut money_by_month).unwrap();

    let salary_num = salary
        .trim()
        .parse::<f64>()
        .expect("Failed to convert to float");
    let money_by_month_value = money_by_month
        .trim()
        .parse::<f64>()
        .expect("Failed to convert to float");

    let result = salary_num + money_by_month_value * 0.15;
    println!("TOTAL = R$ {:.2}", result)
}
