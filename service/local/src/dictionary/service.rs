use std::collections::HashSet;
use std::convert::TryInto;

use rand::seq::SliceRandom;
use rand::thread_rng;

use codenames_domain::dictionary::model::MINIMUM_DICTIONARY_SIZE;
use codenames_domain::dictionary::service::WordGenerator;
use codenames_domain::StdResult;

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
