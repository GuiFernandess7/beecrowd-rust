use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading variable");

    let input = input.trim();

    // LOOP CHARACTERS
    let mut output = String::from("");
    for (_i, c) in input.chars().enumerate() { 
        let ascii_next = (c as u8 + 3) as char;
        output.push(ascii_next);
    }

    // REVERSE STRING
    let mut reversed: String = output.chars().rev().collect();
    let mut half = String::from("");

    let len = reversed.len();
    let middle_index = len / 2;
    let mut slice: String = reversed.chars().take(middle_index).collect();

    for c in reversed.chars().skip(middle_index){
        let ascii_prev = (c as u8 - 1) as char;
        slice.push(ascii_prev);
    }

    println!("{}", slice);
    use std::io;
}

// fn main() {
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("Error reading variable");
//     let input = input.trim();

//     // SHIFT +3
//     let mut bytes: Vec<u8> = input.bytes().map(|b| b + 3).collect();

//     // REVERSE
//     bytes.reverse();

//     // METADADE
//     let mid = bytes.len() / 2;

//     // SHIFT -1 na segunda metade
//     for b in &mut bytes[mid..] {
//         *b -= 1;
//     }

//     // CONVERT PARA STRING
//     let result = String::from_utf8_lossy(&bytes);
//     println!("{}", result);
// }
