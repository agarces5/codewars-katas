use std::collections::HashMap;

use crate::{errors::ParseError, Dictionary};

fn has_valid_brackets(s: &str) -> Result<HashMap<char, i32>, ParseError> {
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
    Ok(brackets)
}
pub fn rewrite_molecule(molecule: &str) -> Result<String, ParseError> {
    let mut brackets: Vec<(char, i32)> = has_valid_brackets(molecule)?.into_iter().collect();
    let mut rw_molecule = molecule.to_string();
    brackets.retain(|(k, _)| matches!(k, '(' | '[' | '{'));
    brackets.sort_by(|v1, v2| v2.1.cmp(&v1.1));

    let mut my_iter = brackets.into_iter().cycle().peekable();
    let mut counter = 0;
    while let Some(mut my_vec) = my_iter.next() {
        while my_vec.1 >= my_iter.peek().unwrap().1 {
            rw_molecule = clean_symbols(my_vec.0, &rw_molecule);
            my_vec.1 -= 1;
        }
        if my_vec.1 <= 0 {
            counter += 1;
        }
        if counter == 3 {
            break;
        }
    }

    Ok(rw_molecule)
}

fn clean_symbols(open_symbol: char, s: &str) -> String {
    let mut result = String::new();
    let close_symbol = match open_symbol {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        _ => '0',
    };
    if !s.contains(close_symbol) || !s.contains(open_symbol) {
        return s.to_string();
    } // revisar

    let (start, right_side) = s.split_once(close_symbol).unwrap();
    let (left_side, inside) = start.rsplit_once(open_symbol).unwrap();

    let mut num = String::new();

    for c in right_side.chars() {
        if c.is_ascii_digit() {
            num.push(c)
        } else {
            break;
        }
    }

    if !num.is_empty() {
        let (_num, end) = right_side.split_at(num.len());
        result.push_str(left_side);
        result.push_str(&inside.repeat(num.parse().unwrap()));
        result.push_str(end);
    } else {
        result.push_str(left_side);
        result.push_str(inside);
        result.push_str(right_side);
    }

    if result.is_empty() {
        result = s.to_string();
    }
    result
}

fn parse_quantity(quantity: &str) -> usize {
    if quantity.is_empty() {
        1
    } else {
        quantity.parse::<usize>().unwrap()
    }
}
fn check_element(element: &str) -> Result<(), ParseError> {
    if element.chars().next().is_some_and(|c| c.is_uppercase()) {
        Ok(())
    } else {
        Err(ParseError::Invalid)
    }
}
pub fn push_into_dictionary(
    dictionary: &mut Dictionary,
    element: &str,
    quantity: &str,
) -> Result<(), ParseError> {
    check_element(element)?;
    let quantity = parse_quantity(quantity);
    dictionary
        .entry(element.to_owned())
        .and_modify(|e| *e += quantity)
        .or_insert(quantity);
    Ok(())
}
