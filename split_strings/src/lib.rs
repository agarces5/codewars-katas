// Complete the solution so that it splits the string into pairs of two characters. If the string contains an odd number of characters then it should replace the missing second character of the final pair with an underscore ('_').

// Examples:

// * 'abc' =>  ['ab', 'c_']
// * 'abcdef' => ['ab', 'cd', 'ef']

fn solution(s: &str) -> Vec<String> {
    if s.is_empty() {
        return vec![];
    }
    let s = if s.len() % 2 == 1 {
        format!("{}_", s)
    } else {
        s.to_string()
    };
    let x: Vec<char> = s.chars().collect();
    let x = x.chunks(2);
    x.fold(vec![], |mut acc: Vec<String>, c| {
        acc.push(c.to_owned().iter().collect());
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solution("abcdef"), ["ab", "cd", "ef"]);
        assert_eq!(solution("abcdefg"), ["ab", "cd", "ef", "g_"]);
        assert_eq!(solution(""), [] as [&str; 0]);
    }
}
