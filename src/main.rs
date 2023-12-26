use crate::services::character_service::CharacterService;

mod models;
mod services;
mod traits;
mod utils;

fn main() {
    println!("Loading data...");
    let population = CharacterService::populate_world(10);
    let length = &population.len();

    // print all characters...
    for (i, char) in population.iter().enumerate() {
        println!("{} : {:?}",i+1 , char);
    }

    println!("The population size is: {}", &length);

    println!("Initiating world...");

    println!("End.");
}
