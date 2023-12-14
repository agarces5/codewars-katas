pub fn zeros(n: u64) -> u64 {
    if n == 0 {
        0
    } else {
        // (1..=((n as f64).log(5.0).floor() as u32)).fold(0, |acc, k| acc + (n / 5u64.pow(k)))
        n / 5 + zeros(n / 5)
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(zeros(0), 0);
        assert_eq!(zeros(6), 1);
        assert_eq!(zeros(14), 2);
        assert_eq!(zeros(30), 7);
        assert_eq!(zeros(1000), 249);
        assert_eq!(zeros(100000), 24999);
        assert_eq!(zeros(1000000000), 249999998);
    }

    #[test]
    fn benchmarks() {
        let start = std::time::Instant::now();
        zeros(u64::MAX);
        let elapsed = start.elapsed();
        println!("Elapsed: {elapsed:?}");
    }
}
