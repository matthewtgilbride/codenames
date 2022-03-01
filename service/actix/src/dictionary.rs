use std::{collections::HashSet, convert::TryInto};

use async_trait::async_trait;
use codenames_domain::{
    dictionary::{WordGenerator, MINIMUM_DICTIONARY_SIZE},
    ServiceResult,
};
use rand::{seq::SliceRandom, thread_rng};

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

        return Ok((first, second));
    }
}
