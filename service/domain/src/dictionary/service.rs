use std::collections::HashSet;

use crate::dictionary::model::DictionaryType;
use crate::dictionary::util::get_dictionary_words;
use crate::StdResult;

pub struct Service {
    words: HashSet<String>,
    generator: Box<dyn WordGenerator + Send + Sync>,
}

impl Service {
    pub fn new(generator: Box<dyn WordGenerator + Send + Sync>) -> StdResult<Service> {
        let words = get_dictionary_words(DictionaryType::Default)?;
        Ok(Service { words, generator })
    }

    pub fn new_word_set(&self) -> StdResult<[String; 25]> {
        self.generator
            .random_set(self.words.iter().cloned().collect())
    }

    pub fn new_word_pair(&self) -> StdResult<(String, String)> {
        self.generator
            .random_pair(self.words.iter().cloned().collect())
    }
}

pub trait WordGenerator {
    fn random_set(&self, dictionary: HashSet<String>) -> StdResult<[String; 25]>;
    fn random_pair(&self, dictionary: HashSet<String>) -> StdResult<(String, String)>;
}
