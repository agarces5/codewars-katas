pub mod errors;

use crate::errors::ParseError;

use std::collections::HashMap;

pub type Atom = (String, usize);
pub type Molecule = Vec<Atom>;

pub fn parse_molecule(s: &str) -> Result<Molecule, ParseError> {
    let rev_input = s.chars().rev();
    let mut accumulated_brackets: Vec<(char, usize)> = Vec::with_capacity(8);
    let mut element = String::with_capacity(4);
    let mut num = String::with_capacity(4);
    let mut atoms: HashMap<String, usize> = HashMap::with_capacity(8);

    for c in rev_input {
        match c {
            '(' | '[' | '{' => match accumulated_brackets.pop() {
                Some(bracket) if get_opening_bracket(bracket.0) == c => continue,
                _ => return Err(ParseError::Mismatch),
            },
            ')' | ']' | '}' => {
                accumulated_brackets.push((c, num.parse().unwrap_or(1)));
                num.clear();
            }
            'A'..='Z' => {
                element.insert(0, c);
                atoms
                    .entry(element.clone())
                    .and_modify(|v| *v += calc_num(&accumulated_brackets, &num)) // Suma lo que habia a lo que se calcula
                    .or_insert(calc_num(&accumulated_brackets, &num));
                element.clear();
                num.clear();
            }
            'a'..='z' => element.insert(0, c),
            '0'..='9' => num.insert(0, c),
            _ => return Err(ParseError::Invalid),
        }
    }
    if atoms.is_empty() {
        Err(ParseError::Invalid)
    } else {
        Ok(atoms.into_iter().collect())
    }
}

/// Multiplica el acumulado en los brackets por el numero que tenga el elemento al lado si lo tiene y si no por 1
fn calc_num(acc_brackets: &[(char, usize)], num: &str) -> usize {
    acc_brackets
        .iter()
        .map(|(_bracket, number)| number)
        .product::<usize>()
        * num.parse().unwrap_or(1)
}

/// Devuelve el caracter de apertura correspondiente o el mismo si no hace match
fn get_opening_bracket(close_bracket: char) -> char {
    match close_bracket {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        _ => close_bracket,
    }
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
