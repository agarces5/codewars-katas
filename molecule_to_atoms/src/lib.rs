mod errors;
mod parser;

use crate::errors::ParseError;
use crate::parser::has_valid_brackets;
use crate::parser::rewrite_molecule;

use std::collections::HashMap;

pub type Atom = (String, usize);
pub type Molecule = Vec<Atom>;

pub fn parse_molecule(s: &str) -> Result<Molecule, ParseError> {
    has_valid_brackets(s)?;
    let molecule = rewrite_molecule(&s);
    let mut dictionary: HashMap<String, usize> = HashMap::new();
    for (i, c) in molecule.char_indices() {
        if c.is_ascii_uppercase() {
            let mut el = String::from(c);
            if let Some(next_char) = molecule.chars().nth(i + 1) {
                if next_char.is_lowercase() {
                    el.push(next_char);
                } else if next_char.is_ascii_digit() {
                    let mut next_number = String::from(next_char);
                    if let Some(digit_at_2) = molecule.chars().nth(i + 2) {
                        if digit_at_2.is_ascii_digit() {
                            next_number.push(digit_at_2);
                        }
                    }
                    let next_digit = next_number
                        .parse()
                        .expect("Failed to convert String to number");
                    dictionary
                        .entry(c.to_string())
                        .and_modify(|e| *e += next_digit)
                        .or_insert(next_digit);
                } else {
                    dictionary
                        .entry(c.to_string())
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                }
            } else {
                dictionary
                    .entry(c.to_string())
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }
    }

    let result: Vec<Atom> = dictionary.into_iter().collect();
    if result.is_empty() {
        return Err(ParseError::Invalid);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::rewrite_molecule;

    use super::{parse_molecule, Molecule};

    macro_rules! assert_parse {
        ($formula:expr, $expected:expr, $name:ident) => {
            #[test]
            fn $name() {
                super::assert_parse($formula, &$expected, "");
            }
        };
    }

    mod molecules {
        assert_parse!("H", [("H", 1)], hydrogen);
        assert_parse!("O2", [("O", 2)], oxygen);
        assert_parse!("H2O", [("H", 2), ("O", 1)], water);
        assert_parse!(
            "Mg(OH)2",
            [("Mg", 1), ("O", 2), ("H", 2)],
            magnesium_hydroxide
        );
        assert_parse!(
            "K4[ON(SO3)2]2",
            [("K", 4), ("O", 14), ("N", 2), ("S", 4)],
            fremys_salt
        );
    }

    #[test]
    fn errors() {
        assert_fail("pie", "Not a valid molecule");
        assert_fail("Mg(OH", "Mismatched parenthesis");
        assert_fail("Mg(OH}2", "Mismatched parenthesis");
    }

    fn assert_fail(formula: &str, msg: &str) {
        let result = parse_molecule(formula);
        assert!(
            result.is_err(),
            "expected {} {:?} to fail, got {:?}",
            msg,
            formula,
            result.unwrap()
        );
    }

    fn assert_parse(formula: &str, expected: &[(&str, usize)], _mst: &str) {
        println!("{}", rewrite_molecule(formula));
        let mut expected = expected
            .into_iter()
            .map(|&(name, usize)| (name.to_owned(), usize))
            .collect::<Molecule>();
        let result = parse_molecule(formula);
        assert!(
            result.is_ok(),
            "expected {:?} to pass, got {:?}",
            formula,
            result
        );
        let mut actual = result.unwrap();
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }
}
