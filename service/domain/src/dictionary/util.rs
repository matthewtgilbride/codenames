use std::collections::HashSet;

use log::debug;

use crate::dictionary::model::DictionaryType;
use crate::StdResult;

pub fn get_dictionary_words(_dictionary_type: DictionaryType) -> StdResult<HashSet<String>> {
    debug!("call: util.get_dictionary_words");
    // todo: pass dict type to select from a set of available dictionaries
    Ok(std::str::from_utf8(include_bytes!("default.txt"))?
        .split("\n")
        .into_iter()
        .map(|s| s.to_string())
        .collect())
}
