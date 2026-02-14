use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error reading input");
    let times = input.trim().parse::<i32>().expect("error converting to integer");
    let mut combinations: Vec<i32> = Vec::new();

    for _ in 0..times {
        let mut password = String::new();
        io::stdin().read_line(&mut password).expect("error reading input");
        let mut combination = 1;
        
        for ch in password.trim().chars()  {
            let l_upper = ch.to_ascii_uppercase();

            if ['A', 'E', 'I', 'O', 'S'].contains(&l_upper) {
                combination *= 3;
            } else {
                combination *= 2;
            }
        }
        combinations.push(combination);
    }

    for c in combinations {
        println!("{}", c);
    }

}
