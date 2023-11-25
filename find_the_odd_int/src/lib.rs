// Given an array of integers, find the one that appears an odd number of times.

// There will always be only one integer that appears an odd number of times.
// Examples

// [7] should return 7, because it occurs 1 time (which is odd).
// [0] should return 0, because it occurs 1 time (which is odd).
// [1,1,2] should return 2, because it occurs 1 time (which is odd).
// [0,1,0,1,0] should return 0, because it occurs 3 times (which is odd).
// [1,2,2,3,3,3,4,3,3,3,2,2,1] should return 4, because it appears 1 time (which is odd).

use std::collections::HashMap;

pub fn find_odd(arr: &[i32]) -> i32 {
    let numbers = arr.iter().fold(HashMap::new(), |mut acc, n| {
        acc.entry(n).and_modify(|i| *i += 1).or_insert(1);
        acc
    });
    let (&&result, _i) = numbers.iter().find(|(_, &value)| value % 2 == 1).unwrap();
    result
}

#[cfg(test)]
mod tests {
    use super::find_odd;

    #[test]
    fn basic_tests() {
        assert_eq!(
            find_odd(&vec![
                20, 1, -1, 2, -2, 3, 3, 5, 5, 1, 2, 4, 20, 4, -1, -2, 5
            ]),
            5
        );
        assert_eq!(find_odd(&vec![1, 1, 2, -2, 5, 2, 4, 4, -1, -2, 5]), -1);
        assert_eq!(find_odd(&vec![20, 1, 1, 2, 2, 3, 3, 5, 5, 4, 20, 4, 5]), 5);
        assert_eq!(find_odd(&vec![10]), 10);
        assert_eq!(find_odd(&vec![1, 1, 1, 1, 1, 1, 10, 1, 1, 1, 1]), 10);
        assert_eq!(find_odd(&vec![5, 4, 3, 2, 1, 5, 4, 3, 2, 10, 10]), 1);
    }
}
