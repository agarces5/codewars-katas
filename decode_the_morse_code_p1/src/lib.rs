use std::collections::HashMap;

fn decode_morse(encoded: &str) -> String {
    let dictionary: HashMap<String, String> = HashMap::new(); // This should be a dictionary with morse -> ASCII translation.
    encoded
        .trim()
        .split("   ") // First, split morse code in separated words
        .map(|word| {
            // For each word in morse, return the equivalent in ASCII
            word.split_ascii_whitespace() // Now, split by whitespace to take each ASCII char
                .fold(String::new(), |acc, word| {
                    format!(
                        "{}{}",
                        acc,
                        dictionary.get(&word.to_owned()).unwrap_or(&"".to_string())
                    )
                }) // Translate morse to  ASCII
        })
        .collect::<Vec<String>>()
        .join(" ")
        .to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hey_jude() {
        assert_eq!(decode_morse(".... . -.--   .--- ..- -.. ."), "HEY JUDE");
    }
}
