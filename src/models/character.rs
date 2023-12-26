use rand::{Rng, thread_rng};
use crate::models::enums::Gender;
use crate::models::enums::Gender::Female;
use crate::traits::create_random::CreateRandom;
use crate::utils::loader::{get_female_names, get_male_names, get_surnames};

#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub age: u32,
    pub gender: Gender,
}

//associated functions
impl CreateRandom<Character> for Character  {
    fn create_random() -> Character {
        let males = get_male_names();
        let females = get_female_names();
        let surnames = get_surnames();
        let mut rng = thread_rng();

        let male_or_female = if rng.gen_bool(0.5) { Gender::Male } else { Female };
        let chosen_name = match male_or_female {
            Female => females.get(rng.gen_range(0..females.len())),
            _ => males.get(rng.gen_range(0..males.len())),
        };

        let chosen_name = chosen_name.expect("Name not found.");
        let chosen_surname = surnames.get(rng.gen_range(0..surnames.len())).expect("No surname found.");

        Character {
            age: rng.gen_range(0..=80),
            gender: male_or_female,
            name: format!("{} {}", chosen_name, chosen_surname)
        }
    }
}