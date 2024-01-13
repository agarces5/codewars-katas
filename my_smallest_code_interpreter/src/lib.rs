pub fn brain_luck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let mut output = vec![];
    let mut data = [0u8; 4096]; // Instructions says that a few thousand should suffice

    // Store position of the current character within 'code'
    let (mut code_index, mut data_pointer) = (0, 0);
    // Open and close position of matching braces
    let mut brackets = vec![];
    let mut iter_input = input.iter();
    // Get vec with brackets
    {
        let a = code
            .char_indices()
            .filter(|(_i, ch)| ch == &'[' || ch == &']');
        let mut aux = vec![];
        for (i, ch) in a {
            match ch {
                '[' => {
                    aux.push(i);
                }
                ']' => {
                    brackets.push((aux.pop().unwrap(), i));
                }
                _ => todo!(), // Never reaches
            }
        }
    }
    while code_index < code.len() {
        match code.chars().nth(code_index).unwrap() {
            '>' => {
                data_pointer += 1;
            }
            '<' => {
                data_pointer -= 1;
            }
            '+' => data[data_pointer] = data[data_pointer].overflowing_add(1).0,
            '-' => data[data_pointer] = data[data_pointer].overflowing_sub(1).0,
            '.' => output.push(data[data_pointer]),
            ',' => {
                if let Some(ch) = iter_input.next() {
                    data[data_pointer] = ch.to_owned();
                } else {
                    break;
                }
            }
            '[' => {
                if data[data_pointer] == 0 {
                    code_index = brackets
                        .iter()
                        .find(|(open, _close)| open == &code_index)
                        .unwrap_or(&(0, 0))
                        .1;
                }
            }
            ']' => {
                if data[data_pointer] != 0 {
                    code_index = brackets
                        .iter()
                        .find(|(_open, close)| close == &code_index)
                        .unwrap_or(&(0, 0))
                        .0;
                }
            }
            _ => todo!(),
        }
        code_index += 1;
    }
    output
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
