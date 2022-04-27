use std::{collections::HashSet, convert::TryInto};

use async_trait::async_trait;
use codenames_domain::{
    dictionary::{WordGenerator, MINIMUM_DICTIONARY_SIZE},
    ServiceError, ServiceResult,
};
use wasmcloud_interface_numbergen::random_in_range;

use crate::{log_stuff, to_service_error};

#[derive(Clone)]
pub struct WordGeneratorWasmCloud;

impl WordGeneratorWasmCloud {
    async fn random_list(
        &self,
        dictionary: &HashSet<String>,
        size: usize,
    ) -> ServiceResult<Vec<String>> {
        log_stuff(String::from("call: dictionary.WordGenerator.random_list")).await?;
        if dictionary.len() < (MINIMUM_DICTIONARY_SIZE + 1) {
            return Err("dictionary must have at least 26 words".into());
        }

        let as_vector: Vec<String> = dictionary.into_iter().cloned().collect();

        let mut chosen_indices: HashSet<usize> = HashSet::new();
        while chosen_indices.len() < size {
            let rand = random_in_range(0, dictionary.len() as u32)
                .await
                .map_err(|e| to_service_error(e))?;
            chosen_indices.insert(rand as usize);
        }
        Ok(chosen_indices
            .into_iter()
            .map(|i| as_vector[i].clone())
            .collect())
    }
}

#[async_trait]
impl WordGenerator for WordGeneratorWasmCloud {
    async fn random_set(&self, dictionary: HashSet<String>) -> ServiceResult<[String; 25]> {
        let words = self.random_list(&dictionary, 25).await?;
        Ok(words.try_into().unwrap())
    }

    async fn random_pair(&self, dictionary: HashSet<String>) -> ServiceResult<(String, String)> {
        log_stuff(String::from("call: dictionary.WordGenerator.random_pair")).await?;
        let result = self.random_list(&dictionary, 2).await?;
        log_stuff(format!(
            "dictionary.WordGenerator.random_pair: got random list from generator: {}",
            result.len()
        ))
        .await?;
        if result.len() == 2 {
            return Ok((result[0].clone(), result[1].clone()));
        }
        return Err(ServiceError::Unknown(
            "error generating random words.  expected vec of length 2 but got something else"
                .to_string(),
        ));
    }
}
