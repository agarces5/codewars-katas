// Welcome.

// In this kata you are required to, given a string, replace every letter with its position in the alphabet.

// If anything in the text isn't a letter, ignore it and don't return it.

// "a" = 1, "b" = 2, etc.
// Example

// alphabet_position("The sunset sets at twelve o' clock.")

// Should return "20 8 5 19 21 14 19 5 20 19 5 20 19 1 20 20 23 5 12 22 5 15 3 12 15 3 11" ( as a string )

#![allow(dead_code)]
use std::collections::HashMap;

fn alphabet_position(text: &str) -> String {
    let mut dictionary: HashMap<char, usize> = HashMap::new();
    ('a'..='z').enumerate().for_each(|(index, c)| {
        dictionary.entry(c).or_insert(index + 1);
    });

    text.to_lowercase()
        .chars()
        .filter_map(|letter| {
            dictionary
                .get(&letter) // Get the value contained in the key
                .map(|number| number.to_string()) // Map the Option to contain a String
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn returns_expected() {
    assert_eq!(
        alphabet_position("The sunset sets at twelve o' clock."),
        "20 8 5 19 21 14 19 5 20 19 5 20 19 1 20 20 23 5 12 22 5 15 3 12 15 3 11".to_string()
    );
    assert_eq!(
        alphabet_position("The narwhal bacons at midnight."),
        "20 8 5 14 1 18 23 8 1 12 2 1 3 15 14 19 1 20 13 9 4 14 9 7 8 20".to_string()
    );
}
