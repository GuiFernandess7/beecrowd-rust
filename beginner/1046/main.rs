use std::io;

fn main(){
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("error reading input");
    let mut hours: Vec<i32> = inp.split_whitespace().map(|s| s.parse().expect("error converting to int32")).collect();
    
    let start = hours[0];
    let end = hours[1];
    let mut range = 1;

    if start == end {
        range = 24;
    }
    else if start > end {
        range = (24 - start) + end;
    } 
    else {
        range = end - start; 
    } 

    println!("O JOGO DUROU {} HORA(S)", range);
}