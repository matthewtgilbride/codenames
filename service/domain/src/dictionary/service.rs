use std::collections::HashSet;

use dyn_clone::DynClone;
use log::debug;

use crate::{ServiceResult, StdResult};
use crate::dictionary::model::DictionaryType;
use crate::dictionary::util::get_dictionary_words;

#[derive(Clone)]
pub struct Service {
    words: HashSet<String>,
    generator: Box<dyn WordGenerator>,
}

impl Service {
    pub fn new(generator: Box<dyn WordGenerator>) -> StdResult<Service> {
        debug!("call: dictionary.Service::new");
        let words = get_dictionary_words(DictionaryType::Default)?;
        Ok(Service { words, generator })
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
