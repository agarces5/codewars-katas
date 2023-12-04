mod errors;
mod parser;

use crate::errors::ParseError;
use crate::parser::has_valid_brackets;
use crate::parser::rewrite_molecule;

<<<<<<< HEAD
use std::collections::HashMap;
=======
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

// Input As2{Be4C5[BCo3(CO2)3]2}4Cu5 -> [("As", 2), ("B", 8), ("Be", 16), ("C", 44), ("Co", 24), ("Cu", 5), ("O", 48)]
// Input: PuK5Ra[(Ra6)] -> [("K", 5), ("Pu", 1), ("Ra", 7)]

use std::{collections::HashMap, fmt::Display};

use thiserror::Error;
>>>>>>> refs/remotes/origin/main

pub type Atom = (String, usize);
pub type Molecule = Vec<Atom>;

pub fn parse_molecule(s: &str) -> Result<Molecule, ParseError> {
<<<<<<< HEAD
    has_valid_brackets(s)?;
    let molecule = rewrite_molecule(&s);
    let mut dictionary: HashMap<String, usize> = HashMap::new();
=======
    println!("Input: {}", s);
    let mut dictionary = HashMap::from([('H', 0usize)]);
    let mut result: Vec<Atom> = vec![];
    has_valid_brackets(s)?;
    let molecule = &s.replace("Mg", "X");
    let molecule = &molecule.replace("As", "A");
    let molecule = &molecule.replace("Be", "D");
    let molecule = &molecule.replace("Co", "W");
    let molecule = &molecule.replace("Cu", "Z");
    let molecule = &molecule.replace("Fe", "F");
    let molecule = &molecule.replace("Mo", "M");
    let molecule = &molecule.replace("Pd", "T");
    let molecule = rewrite_molecule(&molecule);
>>>>>>> refs/remotes/origin/main
    for (i, c) in molecule.char_indices() {
        println!("Char: {}\tDictionary:{:?}", c, dictionary);
        if c.is_ascii_uppercase() {
            let mut el = String::from(c);
            if let Some(next_char) = molecule.chars().nth(i + 1) {
<<<<<<< HEAD
                if next_char.is_lowercase() {
                    el.push(next_char);
                } else if next_char.is_ascii_digit() {
                    let mut next_number = String::from(next_char);
=======
                if next_char.is_ascii_digit() {
                    let mut next_number = String::new();
                    next_number.push(next_char);
>>>>>>> refs/remotes/origin/main
                    if let Some(digit_at_2) = molecule.chars().nth(i + 2) {
                        if digit_at_2.is_ascii_digit() {
                            next_number.push(digit_at_2);
                        }
                    }
                    let next_digit = next_number
                        .parse()
                        .expect("Failed to convert String to number");
                    dictionary
<<<<<<< HEAD
                        .entry(c.to_string())
=======
                        .entry(c)
>>>>>>> refs/remotes/origin/main
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
<<<<<<< HEAD

    let result: Vec<Atom> = dictionary.into_iter().collect();
=======
    let new_dic: HashMap<String, usize> = dictionary
        .clone()
        .iter()
        .map(|(k, &v)| match *k {
            'A' => ("As".to_string(), v),
            'D' => ("Be".to_string(), v),
            'W' => ("Co".to_string(), v),
            'Z' => ("Cu".to_string(), v),
            'X' => ("Mg".to_string(), v),
            'F' => ("Fe".to_string(), v),
            'M' => ("Mo".to_string(), v),
            'T' => ("Pd".to_string(), v),
            _ => (k.to_string(), v),
        })
        .collect();
    for (k, v) in new_dic {
        if v > 0 {
            result.push((k, v))
        }
    }
    println!("Result: {:?}", result);
>>>>>>> refs/remotes/origin/main
    if result.is_empty() {
        return Err(ParseError::Invalid);
    }
    Ok(result)
}
<<<<<<< HEAD
=======

pub fn rewrite_molecule(molecule: &str) -> String {
    let mut rewrite_molecule = no_parenthesis(molecule);
    while rewrite_molecule.find(|c| c == '(').is_some() {
        rewrite_molecule = no_parenthesis(&rewrite_molecule);
    }
    while rewrite_molecule.find(|c| c == '[').is_some() {
        rewrite_molecule = no_brackets(&rewrite_molecule);
    }
    while rewrite_molecule.find(|c| c == '{').is_some() {
        rewrite_molecule = no_braces(&rewrite_molecule);
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
>>>>>>> refs/remotes/origin/main

pub fn no_braces(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        if let '{' = c {
            let (start, end) = s.split_once('}').unwrap();
            let (first, inside_brackets) = start.split_once('{').unwrap();
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
