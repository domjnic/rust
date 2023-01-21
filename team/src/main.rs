use clearscreen::ClearScreen;
use rand::Rng;
use std::{collections::HashMap, io};

fn main() {
    let mut team = HashMap::new();

    ClearScreen::default().clear();

    team.insert("John", 1);
    team.insert("Kathy", 2);

    println!("This is the team: {:?}", team);

    println!("\n\nWhat is your name?");

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    name = name.trim().to_string();

    let number = rand::thread_rng().gen_range(1..1001);

    team.insert(&name, number);

    println!("\n\n{:?}", team)
}
