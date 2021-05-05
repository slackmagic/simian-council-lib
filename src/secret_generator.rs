pub mod dictionary_entry;
use crate::resources;
use crate::secret_generator::dictionary_entry::*;
use crate::utils::file_interactor::*;
use rand::prelude::*;
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn generate(num: u8, start_enthro_level: u8, end_enthro_level: u8) -> String {
    let mut rng = rand::thread_rng();
    let mut secret: String = String::new();

    //Slice dictionary
    let dictionary = get_dictionary();
    validate_range(start_enthro_level, end_enthro_level);
    let start_range = get_range(start_enthro_level, dictionary.len());
    let end_range = get_range(end_enthro_level, dictionary.len());
    let filtered_dictionary = &dictionary[start_range..end_range];

    //Create secret
    for _ in 0..num {
        let random_index = rng.gen_range(1, filtered_dictionary.len());
        secret += &filtered_dictionary[random_index].word;
        secret += " ";
    }

    secret
}

fn validate_range(start_enthro_level: u8, end_enthro_level: u8) {
    if start_enthro_level > end_enthro_level {
        panic!(
            "Start point must be lesser than endpoint : {:?} > {:?}",
            start_enthro_level, end_enthro_level
        );
    }
}

fn get_range(mut enthropy_level: u8, max_size: usize) -> usize {
    if enthropy_level > 100 {
        enthropy_level = 100;
    }
    let percent: f32 = (enthropy_level as f32) / 100.0;
    (max_size as f32 * percent) as usize
}

fn get_dictionary() -> Box<Vec<DictionaryEntry>> {
    let mut dictionary: Vec<DictionaryEntry> = Vec::new();
    for line in String::from_utf8_lossy(resources::EN_WORD_LIST_FILE).lines() {
        let entry: Vec<&str> = line.split(' ').collect();

        dictionary.push(DictionaryEntry::new(
            entry[0].to_string(),
            entry[1].to_string(),
        ));
    }

    Box::new(dictionary)
}

fn get_50k_dictionary() -> Box<Vec<DictionaryEntry>> {
    let mut dictionary: Vec<DictionaryEntry> = Vec::new();
    let path = Path::new("./resources/word/en_50k.txt");

    for line in BufReader::new(get_file(&path)).lines() {
        match line {
            Err(why) => panic!("couldn't read line: {}", why.description()),
            Ok(line) => {
                let entry: Vec<&str> = line.split(' ').collect();

                dictionary.push(DictionaryEntry::new(
                    entry[1].to_string(),
                    entry[0].to_string(),
                ));
            }
        }
    }

    Box::new(dictionary)
}
