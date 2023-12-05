pub mod errors;
pub mod parser;

use crate::errors::ParseError;
use crate::parser::*;

use std::collections::HashMap;

pub type Atom = (String, usize);
pub type Molecule = Vec<Atom>;
pub type Dictionary = HashMap<String, usize>;

pub fn parse_molecule(s: &str) -> Result<Molecule, ParseError> {
    let molecule = rewrite_molecule(s)?;
    let mut dictionary: Dictionary = HashMap::new();

    let mut el = String::new();
    let mut num = String::new();
    for (i, c) in molecule.char_indices() {
        if c.is_ascii_alphabetic() {
            el.push(c);
        } else {
            num.push(c);
        }
        match molecule.chars().nth(i + 1) {
            Some(c) => {
                if c.is_uppercase() {
                    push_into_dictionary(&mut dictionary, &el, &num)?;
                    el = String::new();
                    num = String::new();
                }
            }
            None => push_into_dictionary(&mut dictionary, &el, &num)?,
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
        let mut expected = expected
            .iter()
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
