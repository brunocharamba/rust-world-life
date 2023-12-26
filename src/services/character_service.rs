use crate::models::character::Character;
use crate::traits::create_random::CreateRandom;

pub struct CharacterService {}

impl CharacterService {
    pub fn populate_world(npc_number: u32) -> Vec<Character> {
        let mut population: Vec<Character> = Vec::new();
        for _ in 0..npc_number {
            population.push(Character::create_random());
        }

        population
    }
}

