use std::io;

fn main(){
    let mut animals: Vec<String> = Vec::new();

    for i in 0..3 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error reading variable");
        animals.push(input.trim().to_string());
    }

    let specie = animals[1].to_string();
    let style = animals[2].to_string();
    let mut animal = "";
    
    if style == "carnivoro" {
        animal = "aguia";
    } 
    
    if style == "hematofago" && specie != "inseto"{
        animal = "sanguessuga";
    } else if style == "hematofago" && specie == "inseto" {
        animal = "pulga";
    }
    
    if style == "herbivoro" && specie != "mamifero" {
        animal = "vaca";
    } else if style == "herbivoro" && specie == "mamifero" {
        animal = "lagarta";
    }
    
    if style == "onivoro" {
        if specie == "ave" {
            animal = "pomba";
        } else if specie == "mamifero" {
            animal = "homem";
        } else if specie == "anelideo" {
            animal = "minhoca";
        }
    }
    
    
    println!("{}", animal);
}
