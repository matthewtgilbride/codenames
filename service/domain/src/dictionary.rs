use std::collections::HashSet;

use dyn_clone::DynClone;
use log::debug;

use crate::{ServiceResult, StdResult};

#[derive(Clone)]
pub struct DictionaryService {
    words: HashSet<String>,
    generator: Box<dyn WordGenerator>,
}

impl DictionaryService {
    pub fn new(generator: Box<dyn WordGenerator>) -> StdResult<DictionaryService> {
        debug!("call: dictionary.Service::new");
        let words = get_dictionary_words(DictionaryType::Default)?;
        Ok(DictionaryService { words, generator })
    }

    pub fn new_word_set(&self) -> ServiceResult<[String; 25]> {
        self.generator
            .random_set(self.words.iter().cloned().collect())
    }

    pub fn new_word_pair(&self) -> ServiceResult<(String, String)> {
        debug!("call: dictionary.Service.new_word_pair)");
        self.generator
            .random_pair(self.words.iter().cloned().collect())
    }
}

pub trait WordGenerator: DynClone + Send + Sync {
    fn random_set(&self, dictionary: HashSet<String>) -> ServiceResult<[String; 25]>;
    fn random_pair(&self, dictionary: HashSet<String>) -> ServiceResult<(String, String)>;
}

dyn_clone::clone_trait_object!(WordGenerator);

pub fn get_dictionary_words(_dictionary_type: DictionaryType) -> StdResult<HashSet<String>> {
    debug!("call: util.get_dictionary_words");
    // todo: pass dict type to select from a set of available dictionaries
    Ok(std::str::from_utf8(include_bytes!("default.txt"))?
        .split("\n")
        .into_iter()
        .map(|s| s.to_string())
        .collect())
}

pub const MINIMUM_DICTIONARY_SIZE: usize = 25;

pub enum DictionaryType {
    Default,
}
