use std::io;

fn main() {
    let mut employee = String::new();
    io::stdin().read_line(&mut employee).unwrap();

    let mut work_hours = String::new();
    io::stdin().read_line(&mut work_hours).unwrap();

    let mut amount_received = String::new();
    io::stdin().read_line(&mut amount_received).unwrap();

    let employee_num = employee
        .trim()
        .parse::<i32>()
        .expect("Failed to convert to integer");
    let work_hours_num = work_hours
        .trim()
        .parse::<f64>()
        .expect("Failed to convert to integer");
    let amount_received_nums = amount_received
        .trim()
        .parse::<f64>()
        .expect("Failed to convert to float");

    let salary = work_hours_num * amount_received_nums;
    println!("NUMBER = {}", employee_num);
    println!("SALARY = {:.2}", salary);
}
