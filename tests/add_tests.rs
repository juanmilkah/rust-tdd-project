#[cfg(test)]
mod tests {
    use rust_tdd_dev::add;

    #[test]
    fn add_two_positives() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn add_two_negatives() {
        assert_eq!(add(-1, -2), -3);
    }

    #[test]
    fn add_mixed_numbers() {
        assert_eq!(add(-1, 2), 1);
    }
}
