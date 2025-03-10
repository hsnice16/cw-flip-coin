#[cfg(test)]
mod tests {
    use crate::helper::is_prime;

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(3), true);
    }
}
