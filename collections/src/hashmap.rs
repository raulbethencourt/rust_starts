use std::collections::HashMap;

/// Creates a new hash map with the default size.
pub fn set_hashmap(color: String, number: i32) -> HashMap<String, i32> {
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(color, number);

    scores
}
