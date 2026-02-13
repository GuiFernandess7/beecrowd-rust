use std::io;

fn main(){
    let mut day_1 = String::new();
    io::stdin().read_line(&mut day_1).expect("error reading value");

    let mut hour_1 = String::new();
    io::stdin().read_line(&mut hour_1).expect("error reading value");

    let mut day_2 = String::new();
    io::stdin().read_line(&mut day_2).expect("error reading value");

    let mut hour_2 = String::new();
    io::stdin().read_line(&mut hour_2).expect("error reading value");

    let hours_1: Vec<i32> = hour_1.split(":").map(|s| s.trim()).map(|s| s.parse::<i32>().unwrap()).collect();
    let day_1_n: i32 = day_1.split_whitespace().nth(1).unwrap().trim().parse().expect("error converting to integer");
    let secs_1 = (day_1_n * 24 * 60 * 60) + (hours_1[0] * 60 * 60) + (hours_1[1] * 60) + (hours_1[2]);

    let hours_2: Vec<i32> = hour_2.split(":").map(|s| s.trim()).map(|s| s.parse::<i32>().unwrap()).collect();
    let day_2_n: i32 = day_2.split_whitespace().nth(1).unwrap().trim().parse().expect("error converting to integer");
    let secs_2 = (day_2_n * 24 * 60 * 60) + (hours_2[0] * 60 * 60) + (hours_2[1] * 60) + (hours_2[2]);
    
    let secs_total = secs_2 - secs_1;
    let dias = secs_total / 86400;
    
    let mut resto = secs_total % 86400;
    let horas = resto / 3600;

    resto = resto % 3600;
    let minutos = resto / 60;
    let segundos = resto % 60;

    println!("{} dia(s)", dias);
    println!("{} hora(s)", horas);
    println!("{} minuto(s)", minutos);
    println!("{} segundo(s)", segundos);
}