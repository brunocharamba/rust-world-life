use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::models::enums::Gender;
use crate::utils::constants::{FILE_PATH_FEMALE_NAMES, FILE_PATH_MALE_NAMES, FILE_PATH_SURNAMES};

pub struct FileService {}

impl FileService {
    pub fn load_names(gender: Gender) -> Vec<String> {
        let file_path = match gender {
            Gender::Female => FILE_PATH_FEMALE_NAMES,
            _ => FILE_PATH_MALE_NAMES,
        };

        let file = File::open(&file_path).expect("File Not Found!");
        let buffer = BufReader::new(file);

        let lines = buffer.lines()
            .map(|line| line.expect("Cannot parse the line from file.")).collect();

        lines
    }

    pub fn load_surnames() -> Vec<String> {
        let file_path = FILE_PATH_SURNAMES;

        let file = File::open(&file_path).expect("File Not Found!");
        let buffer = BufReader::new(file);

        let lines = buffer.lines()
            .map(|line| line.expect("Cannot parse the line from file.")).collect();

        lines
    }
}