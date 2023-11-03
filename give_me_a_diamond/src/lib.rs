// Jamie is a programmer, and James' girlfriend. She likes diamonds, and wants a diamond string from James. Since James doesn't know how to make this happen, he needs your help.
// Task

// You need to return a string that looks like a diamond shape when printed on the screen, using asterisk (*) characters. Trailing spaces should be removed, and every line must be terminated with a newline character (\n).

// Return null/nil/None/... if the input is an even number or negative, as it is not possible to print a diamond of even or negative size.
// Examples

// A size 3 diamond:

//  *
// ***
//  *

// ...which would appear as a string of " *\n***\n *\n"

// A size 5 diamond:

//   *
//  ***
// *****
//  ***
//   *

// ...that is:

// "  *\n ***\n*****\n ***\n  *\n"

pub fn print(n: i32) -> Option<String> {
    match n % 2 {
        1 => {
            let n = n as usize;
            let diamond = (1..=n)
                .chain((1..n).rev())
                .step_by(2)
                .map(|i| format!("{}{}\n", " ".repeat((n - i) / 2), "*".repeat(i)))
                .collect();
            Some(diamond)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(print(3), Some(" *\n***\n *\n".to_string()));
    }
    #[test]
    fn test2() {
        assert_eq!(print(5), Some("  *\n ***\n*****\n ***\n  *\n".to_string()));
    }
    #[test]
    fn test3() {
        assert_eq!(print(-3), None);
    }
    #[test]
    fn test4() {
        assert_eq!(print(2), None);
    }
    #[test]
    fn test5() {
        assert_eq!(print(0), None);
    }
    #[test]
    fn test6() {
        assert_eq!(print(1), Some("*\n".to_string()));
    }
}
