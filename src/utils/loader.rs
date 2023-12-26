use lazy_static::lazy_static;
use crate::models::enums::Gender;
use crate::services::file_service::FileService;

lazy_static! {
    static ref MALE_NAMES: Vec<String> = FileService::load_names(Gender::Male);
    static ref FEMALE_NAMES: Vec<String> = FileService::load_names(Gender::Female);
    static ref SURNAMES: Vec<String> = FileService::load_surnames();
}

pub fn get_male_names() -> &'static Vec<String> { &MALE_NAMES }
pub fn get_female_names() -> &'static Vec<String> { &FEMALE_NAMES }
pub fn get_surnames() -> &'static Vec<String> { &SURNAMES }