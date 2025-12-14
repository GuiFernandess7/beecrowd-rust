use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let values: Vec<i64> = vec![100, 50, 20, 10, 5, 2, 1];
    let mut input_value = input
        .trim()
        .parse::<i64>()
        .expect("Error converting to integer");

    println!("{}", input_value);
    let mut remaining = input_value;
    for curr_value in values {
        let mut notes = remaining / curr_value;
        remaining = remaining % curr_value;
        let note_str = format!("{},00", curr_value);
        println!("{} nota(s) de R$ {}", notes, note_str);
    }
}
