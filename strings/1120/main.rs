use std::io;

fn main(){
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error reading variable");
        let mut inputs: Vec<&str> = input.split_whitespace().collect();


        let digit = inputs[0].trim().as_bytes()[0] as char;
        let values: Vec<char> = inputs[1].trim().chars().collect();

        if digit == '0' && values[0] == '0' && values.len() == 1 {
            break
        }

        let mut output = String::new();
        for value in values {
            if value != digit {
                output.push(value);  
            }
        }

        let output = output.trim_start_matches('0');

        if output.is_empty() {
            println!("0");
        } else {
            println!("{}", output);
        }
    }
}