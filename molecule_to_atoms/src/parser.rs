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
