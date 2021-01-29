use std::collections::HashSet;

use crate::dictionary::model::DictionaryType;
use crate::model::StandardResult;

pub fn get_dictionary_words(_dictionary_type: DictionaryType) -> StandardResult<HashSet<String>> {
    // todo: pass dict type to select from a set of available dictionaries
    Ok(std::str::from_utf8(include_bytes!("default.txt"))?
        .split("\n")
        .into_iter()
        .map(|s| s.to_string())
        .collect())
}
