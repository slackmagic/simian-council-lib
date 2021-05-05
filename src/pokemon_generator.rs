use crate::resources;
use rand::prelude::*;

pub fn generate() -> String {
    let mut rng = rand::thread_rng();
    let pokemon_data = String::from_utf8_lossy(resources::EN_POKEMON_LIST_FILE);
    let list: Vec<String> = serde_json::from_str(&pokemon_data).unwrap();

    let random_index = rng.gen_range(1, list.len());
    list[random_index].to_string()
}

pub fn generate_with_adjective() -> String {
    let mut rng = rand::thread_rng();
    let mut dictionary: Vec<String> = Vec::new();

    for line in String::from_utf8_lossy(resources::EN_ADJECTIVES_LIST_FILE).lines() {
        dictionary.push(line.to_string())
    }

    let random_index = rng.gen_range(1, dictionary.len());
    dictionary[random_index].to_string() + " " + &generate()
}
