fn main(){
    for i in 1..101 {
        if i % 2 != 0 {
            continue
        }
        println!("{}", i);
    }
}