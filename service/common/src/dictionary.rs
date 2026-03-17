use std::{collections::HashSet, convert::TryInto};

use async_trait::async_trait;
use dyn_clone::DynClone;
use log::debug;
use rand::{seq::SliceRandom, thread_rng};

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

    pub async fn new_word_set(&self) -> ServiceResult<[String; 25]> {
        self.generator
            .random_set(self.words.iter().cloned().collect())
            .await
    }

    pub async fn new_word_pair(&self) -> ServiceResult<(String, String)> {
        debug!("call: dictionary.Service.new_word_pair)");
        self.generator
            .random_pair(self.words.iter().cloned().collect())
            .await
    }
}

#[async_trait]
pub trait WordGenerator: DynClone + Send + Sync {
    async fn random_set(&self, dictionary: HashSet<String>) -> ServiceResult<[String; 25]>;
    async fn random_pair(&self, dictionary: HashSet<String>) -> ServiceResult<(String, String)>;
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

#[derive(Clone)]
pub struct WordGeneratorRand;

#[async_trait]
impl WordGenerator for WordGeneratorRand {
    async fn random_set(&self, dictionary: HashSet<String>) -> ServiceResult<[String; 25]> {
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

    async fn random_pair(&self, dictionary: HashSet<String>) -> ServiceResult<(String, String)> {
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

        Ok((first, second))
    }
}
