use std::collections::HashSet;
use std::convert::TryInto;

use log::debug;
use wasmcloud_actor_extras as extras;

use codenames_domain::dictionary::WordGenerator;
use codenames_domain::dictionary::MINIMUM_DICTIONARY_SIZE;
use codenames_domain::{ServiceError, ServiceResult};

#[derive(Clone)]
pub struct WordGeneratorWasmCloud;

impl WordGeneratorWasmCloud {
    fn random_list(&self, dictionary: &HashSet<String>, size: usize) -> ServiceResult<Vec<String>> {
        debug!("call: dictionary.WordGenerator.random_list");
        if dictionary.len() < (MINIMUM_DICTIONARY_SIZE + 1) {
            return Err("dictionary must have at least 26 words".into());
        }

        let as_vector: Vec<String> = dictionary.into_iter().cloned().collect();

        let mut chosen_indices: HashSet<usize> = HashSet::new();
        while chosen_indices.len() < size {
            chosen_indices.insert(
                (extras::default()
                    .request_random(0, dictionary.len() as u32)
                    .map_err(|_| ServiceError::Unknown("could not get random number".into()))?)
                    as usize,
            );
        }

        Ok(chosen_indices
            .into_iter()
            .map(|i| as_vector[i].clone())
            .collect())
    }
}

impl WordGenerator for WordGeneratorWasmCloud {
    fn random_set(&self, dictionary: HashSet<String>) -> ServiceResult<[String; 25]> {
        let words = self.random_list(&dictionary, 25)?;
        Ok(words.try_into().unwrap())
    }

    fn random_pair(&self, dictionary: HashSet<String>) -> ServiceResult<(String, String)> {
        debug!("call: dictionary.WordGenerator.random_pair");
        let result = self.random_list(&dictionary, 2)?;
        debug!(
            "dictionary.WordGenerator.random_pair: got random list from generator: {}",
            result.len()
        );
        if result.len() == 2 {
            return Ok((result[0].clone(), result[1].clone()));
        }
        return Err(ServiceError::Unknown(
            "error generating random words.  expected vec of length 2 but got something else"
                .to_string(),
        ));
    }
}
