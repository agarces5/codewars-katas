#[cfg(test)]
mod tests {
    use std::vec;

    use molecule_to_atoms::parse_molecule;

    #[test]
    fn test_product_empty() {
        let v: Vec<(char, usize)> = vec![];
        let product: usize = v.iter().map(|(_c, v)| v).product();
        assert_eq!(1, product);
    }
    #[test]
    fn test_hard() {
        let s = "[{Nb}20]7{[{Sm}]25}19";
        let mut result = parse_molecule(s).unwrap();
        result.sort();
        let correct = vec![("Nb".to_string(), 140), ("Sm".to_string(), 475)];
        assert_eq!(format!("{:?}", correct), format!("{:?}", result));
    }
    #[test]
    fn test_rewrite_hard() {
        let s = "[{Nb}20]7{[{Sm}]25}19";
        let mut result = parse_molecule(s).unwrap();
        result.sort();
        let correct = vec![("Nb", 140), ("Sm", 475)];
        assert_eq!(format!("{:?}", correct), format!("{:?}", result));
    }
    #[test]
    fn test_weird_mol() {
        let s = "As2{Be4C5[BCo3(CO2)3]2}4Cu5";
        let mut result = parse_molecule(s).unwrap();
        result.sort();
        let correct = vec![
            ("As", 2),
            ("B", 8),
            ("Be", 16),
            ("C", 44),
            ("Co", 24),
            ("Cu", 5),
            ("O", 48),
        ];
        assert_eq!(format!("{:?}", correct), format!("{:?}", result));
    }
}
