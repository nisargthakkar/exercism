use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {

    let mut input_clone = input.clone();
    let mut output: BTreeMap<char, i32> = BTreeMap::new();
    for (score, character_array) in input_clone.iter_mut() {
        for character in character_array {
            output.insert(character.to_lowercase().next().unwrap(), *score);
        }
    }

    output
}