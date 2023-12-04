// For a given chemical formula represented by a string, count the number of atoms of each element contained in the molecule and return an object (associative array in PHP, Dictionary<string, int> in C#, Map<String,Integer> in Java).

// For example:

// parse_molecule("H2O");           // water
// // Ok([("H", 2), ("O", 1)])

// parse_molecule("Mg(OH)2");       // magnesium hydroxide
// // Ok([("Mg", 1), ("O", 2), ("H", 2)]

// parse_molecule("K4[ON(SO3)2]2"); // Fremy's salt
// // Ok([("K", 4), ("O", 14),("N", 2),("S", 4)])

// parse_molecule("pie")
// // Err(ParseError)

// As you can see, some formulas have brackets in them. The index outside the brackets tells you that you have to multiply count of each atom inside the bracket on this index. For example, in Fe(NO3)2 you have one iron atom, two nitrogen atoms and six oxygen atoms.

// Note that brackets may be round, square or curly and can also be nested. Index after the braces is optional.

use std::{collections::HashMap, fmt::Display};

use thiserror::Error;

pub type Atom = (String, usize);
pub type Molecule = Vec<Atom>;

#[derive(Error, Debug)]
pub enum ParseError {
    Mismatch,
    Invalid,
    // variants
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Mismatch => write!(f, "Mismatched parenthesis"),
            ParseError::Invalid => write!(f, "Not a valid molecule"),
        }
    }
}

pub fn has_valid_brackets(s: &str) -> Result<(), ParseError> {
    let mut brackets = HashMap::from([('{', 0), ('}', 0), ('[', 0), (']', 0), ('(', 0), (')', 0)]);
    s.chars().for_each(|c| {
        match c {
            '{' => {
                brackets.entry(c).and_modify(|b| *b += 1);
            }
            '}' => {
                brackets.entry(c).and_modify(|b| *b += 1);
            }
            '[' => {
                brackets.entry(c).and_modify(|b| *b += 1);
            }
            ']' => {
                brackets.entry(c).and_modify(|b| *b += 1);
            }
            '(' => {
                brackets.entry(c).and_modify(|b| *b += 1);
            }
            ')' => {
                brackets.entry(c).and_modify(|b| *b += 1);
            }
            _ => (),
        };
    });
    if brackets.get(&'{').unwrap() - brackets.get(&'}').unwrap() != 0 {
        return Err(ParseError::Mismatch);
    }
    if brackets.get(&'[').unwrap() - brackets.get(&']').unwrap() != 0 {
        return Err(ParseError::Mismatch);
    }
    if brackets.get(&'(').unwrap() - brackets.get(&')').unwrap() != 0 {
        return Err(ParseError::Mismatch);
    }
    Ok(())
}

pub fn parse_molecule(s: &str) -> Result<Molecule, ParseError> {
    let mut dictionary = HashMap::from([('H', 0usize), ('O', 0), ('X', 0), ('K', 0), ('S', 0)]);
    let mut result: Vec<Atom> = vec![];
    has_valid_brackets(s)?;
    let molecule = rewrite_molecule(&s.replace("Mg", "X"));
    for (i, c) in molecule.char_indices() {
        if c.is_ascii_uppercase() {
            if let Some(next_char) = molecule.chars().nth(i + 1) {
                if next_char.is_ascii_digit() {
                    dictionary
                        .entry(c)
                        .and_modify(|e| *e += next_char.to_digit(10).unwrap() as usize)
                        .or_insert(1);
                } else {
                    dictionary.entry(c).and_modify(|e| *e += 1).or_insert(1);
                }
            } else {
                dictionary.entry(c).and_modify(|e| *e += 1).or_insert(1);
            }
        }
    }
    let new_dic: HashMap<String, usize> = dictionary
        .clone()
        .iter()
        .map(|(k, &v)| {
            if *k == 'X' {
                ("Mg".to_string(), v)
            } else {
                (k.to_string(), v)
            }
        })
        .collect();
    for (k, v) in new_dic {
        if v > 0 {
            result.push((k, v))
        }
    }
    println!("Result: {:?}", result);
    if result.is_empty() {
        return Err(ParseError::Invalid);
    }
    Ok(result)
}
pub fn rewrite_molecule(molecule: &str) -> String {
    let mut rewrite_molecule = no_parenthesis(molecule);
    while rewrite_molecule.find(|c| c == '(').is_some() {
        rewrite_molecule = no_parenthesis(&rewrite_molecule);
    }
    while rewrite_molecule.find(|c| c == '[').is_some() {
        rewrite_molecule = no_brackets(&rewrite_molecule);
    }
    rewrite_molecule
}

pub fn no_parenthesis(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        if let '(' = c {
            let (start, end) = s.split_once(')').unwrap();
            let (first, inside_brackets) = start.split_once('(').unwrap();
            if end.starts_with(char::is_numeric) {
                let (num, end) = end.split_at(1);
                result.push_str(first);
                result.push_str(&inside_brackets.repeat(num.parse().unwrap()));
                result.push_str(end);
            } else {
                result.push_str(first);
                result.push_str(inside_brackets);
                result.push_str(end);
            }
        }
    }
    if result.is_empty() {
        result = s.to_string();
    }
    result
}
pub fn no_brackets(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        if let '[' = c {
            let (start, end) = s.split_once(']').unwrap();
            let (first, inside_brackets) = start.split_once('[').unwrap();
            if end.starts_with(char::is_numeric) {
                let (num, end) = end.split_at(1);
                result.push_str(first);
                result.push_str(&inside_brackets.repeat(num.parse().unwrap()));
                result.push_str(end);
            } else {
                result.push_str(first);
                result.push_str(inside_brackets);
                result.push_str(end);
            }
        }
    }
    if result.is_empty() {
        result = s.to_string();
    }
    result
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
