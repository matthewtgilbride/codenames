use std::collections::HashSet;

pub enum DictionaryType {
    Default,
}

pub fn get_dictionary(
    _dictionary_type: DictionaryType,
) -> Result<HashSet<String>, std::str::Utf8Error> {
    // todo: pass dict type to select from a set of available dictionaries
    Ok(
        std::str::from_utf8(include_bytes!("../resources/dictionary/default.txt",))?
            .split("\n")
            .into_iter()
            .map(|s| s.to_string())
            .collect(),
    )
}
