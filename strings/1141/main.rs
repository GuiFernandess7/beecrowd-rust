use std::io;
use std::collections::HashMap;

fn main(){
    let mut map: HashMap<&str, i64> = HashMap::new();
    //let mut seq = 0;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error reading variable.");
        let mut curr_string = input.trim().to_string();

        if let Ok(count) = input.parse::<i32>() {
            counter = Some(count);

            

            for i in 0..counter {
                map.insert(curr_string, counter)
            }


        }
    }

    println!("{}", seq);
}