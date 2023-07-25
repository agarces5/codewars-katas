// Welcome. In this kata, you are asked to square every digit of a number and concatenate them.

// For example, if we run 9119 through the function, 811181 will come out, because 9^2 is 81 and 1^2 is 1. (81-1-1-81)

// Example #2: An input of 765 will/should return 493625 because 7^2 is 49, 6^2 is 36, and 5^2 is 25. (49-36-25)

// Note: The function accepts an integer and returns an integer.

// Happy Coding!

fn square_digits(num: u64) -> u64 {
    num.to_string()
        .chars()
        .fold(String::new(), |mut acc, char| {
            acc.push_str(&char.to_digit(10).unwrap().pow(2).to_string());
            acc
        })
        .parse()
        .expect("Failed to parse String")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_digits() {
        assert_eq!(square_digits(9119), 811181, "\nFailed with num 9119");
        assert_eq!(square_digits(765), 493625, "\nFailed with num 765");
        assert_eq!(square_digits(0), 0, "\nFailed with number 0");
        assert_eq!(square_digits(1), 1, "\nFailed with number 1");
        assert_eq!(square_digits(2), 4, "\nFailed with number 2");
    }
}
