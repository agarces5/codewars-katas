#![allow(dead_code)]
struct Sudoku {
    data: Vec<Vec<u32>>,
}

impl Sudoku {
    fn valid_square(&self, sum: u32, sqrt: usize) -> bool {
        // Constructs an sqrt(N)xN vector
        let sum_vector: Vec<Vec<u32>> = self
            .data
            .iter()
            .map(|v| {
                (0..v.len()).fold((0..sqrt).map(|_| 0).collect(), |mut acc: Vec<u32>, i| {
                    acc[i / sqrt] += v[i]; // Sums each `sqrt` range in a row. N=3; Index 0+1+2 -> 0, 3+4+5->1, 6+7+8->2
                    acc
                })
            })
            .collect();

        (0..sqrt)
            .fold(vec![], |mut acc, j| {
                acc.push((0..sum_vector.len()).fold(
                    (0..sqrt).map(|_| 0).collect(),
                    |mut acc: Vec<u32>, i| {
                        acc[i / sqrt] += sum_vector[i][j];
                        acc
                    },
                ));
                acc
            })
            .iter()
            .flat_map(|v| v.to_owned())
            .all(|s| s == sum)
    }

    fn is_valid(&self) -> bool {
        let n = self.data.len();
        let sum = (1..=n as u32).sum();
        let sqrt = (n as f32).sqrt() as usize;

        if let Some(v) = self.data.first() {
            self.data.len() == v.len()
                && (v.len() as f32).sqrt().fract() <= f32::EPSILON
                && self.data.iter().all(|v| v.iter().sum::<u32>() == sum)
                && (0..n)
                    .map(|i| (0..n).map(move |j| self.data[j][i]).collect())
                    .all(|v: Vec<u32>| v.iter().sum::<u32>() == sum)
                && self.valid_square(sum, sqrt)
        } else {
            false
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_sudoku() {
        let good_sudoku_1 = Sudoku {
            data: vec![
                vec![7, 8, 4, 1, 5, 9, 3, 2, 6],
                vec![5, 3, 9, 6, 7, 2, 8, 4, 1],
                vec![6, 1, 2, 4, 3, 8, 7, 5, 9],
                vec![9, 2, 8, 7, 1, 5, 4, 6, 3],
                vec![3, 5, 7, 8, 4, 6, 1, 9, 2],
                vec![4, 6, 1, 9, 2, 3, 5, 8, 7],
                vec![8, 7, 6, 3, 9, 4, 2, 1, 5],
                vec![2, 4, 3, 5, 6, 1, 9, 7, 8],
                vec![1, 9, 5, 2, 8, 7, 6, 3, 4],
            ],
        };

        let good_sudoku_2 = Sudoku {
            data: vec![
                vec![1, 4, 2, 3],
                vec![3, 2, 4, 1],
                vec![4, 1, 3, 2],
                vec![2, 3, 1, 4],
            ],
        };
        assert!(good_sudoku_1.is_valid());
        assert!(good_sudoku_2.is_valid());
    }

    #[test]
    fn bad_sudoku() {
        let bad_sudoku_1 = Sudoku {
            data: vec![
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            ],
        };

        let bad_sudoku_2 = Sudoku {
            data: vec![
                vec![1, 2, 3, 4, 5],
                vec![1, 2, 3, 4],
                vec![1, 2, 3, 4],
                vec![1],
            ],
        };
        assert!(!bad_sudoku_1.is_valid());
        assert!(!bad_sudoku_2.is_valid());
    }
}
