pub fn brain_luck(code: &str, input: Vec<u8>) -> Vec<u8> {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_tests() {
        // Echo until byte 255 encountered
        assert_eq!(
            String::from_utf8(brain_luck(",+[-.,+]", ez_vec("Codewars", 255))).unwrap(),
            "Codewars"
        );
        // Echo until byte 0 encountered
        assert_eq!(
            String::from_utf8(brain_luck(",[.[-],]", ez_vec("Codewars", 0))).unwrap(),
            "Codewars"
        );
        // Multiply two numbers
        assert_eq!(
            brain_luck(",>,<[>[->+>+<<]>>[-<<+>>]<<<-]>>.", vec![8, 9]),
            vec![72]
        );
    }

    // Takes a static string and a terminating byte and returns an owned Vec<u8> for convenience
    // Without it, character-based tests are a pain
    fn ez_vec(s: &str, i: u8) -> Vec<u8> {
        let mut v = s.to_string().into_bytes();
        v.push(i);
        v
    }
}
