use std::io;

fn main(){
        let mut input_1 = String::new();
        let mut input_2 = String::new();
        let mut input_3 = String::new();

        io::stdin().read_line(&mut input_1).unwrap();
        io::stdin().read_line(&mut input_2).unwrap();
        io::stdin().read_line(&mut input_3).unwrap();

        let cat1 = input_1.trim();
        let cat2 = input_2.trim();
        let cat3 = input_3.trim();

        let animal = if cat1 == "vertebrado" {
            if cat2 == "ave" {
                if cat3 == "carnivoro" { "aguia" } else { "pomba" }
            } else {
                if cat3 == "onivoro" { "homem" } else { "vaca" }
            }
        } else {
            if cat2 == "inseto" {
                if cat3 == "hematofago" { "pulga" } else { "lagarta" }
            } else {
                if cat3 == "hematofago" { "sanguessuga" } else { "minhoca" }
            }
        };

        println!("{}", animal);
}
