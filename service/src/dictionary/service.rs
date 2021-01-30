use std::collections::HashSet;
use std::convert::TryInto;

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::dictionary::model::{DictionaryType, MINIMUM_DICTIONARY_SIZE};
use crate::dictionary::util::get_dictionary_words;
use crate::model::StdResult;

pub struct Service {
    words: HashSet<String>,
    generator: Box<dyn WordGenerator>,
}

impl Service {
    pub fn new(generator: Box<dyn WordGenerator>) -> StdResult<Service> {
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

pub struct WordGeneratorRand;

impl WordGenerator for WordGeneratorRand {
    fn random_set(&self, dictionary: HashSet<String>) -> StdResult<[String; 25]> {
        if dictionary.len() < (MINIMUM_DICTIONARY_SIZE + 1) {
            return Err("dictionary must have at least 26 words".into());
        }

        let as_vector: Vec<String> = dictionary.into_iter().collect();

        let random_subset: Vec<String> = as_vector
            .choose_multiple(&mut thread_rng(), 25)
            .cloned()
            .collect();

        Ok(random_subset.try_into().unwrap())
    }

    fn random_pair(&self, dictionary: HashSet<String>) -> StdResult<(String, String)> {
        if dictionary.len() < (MINIMUM_DICTIONARY_SIZE + 1) {
            return Err("dictionary must have at least 26 words".into());
        }

        let as_vector: Vec<String> = dictionary.into_iter().collect();

        let random_subset: Vec<String> = as_vector
            .choose_multiple(&mut thread_rng(), 2)
            .cloned()
            .collect();

        let first = random_subset.get(0).unwrap().clone();
        let second = random_subset.get(1).unwrap().clone();

        return Ok((first, second));
    }
}
