pub struct DictionaryEntry {
    pub value: String,
    pub word: String,
}

impl DictionaryEntry {
    pub fn new(value: String, word: String) -> DictionaryEntry {
        DictionaryEntry {
            value: value,
            word: word,
        }
    }
}
