pub mod large_number;
pub mod prime;

#[cfg(test)]
mod test {
    use super::prime;

    #[test]
    fn test_is_prime() {
        assert_eq!(prime::is_prime(1), false);
        assert_eq!(prime::is_prime(2), true);
        assert_eq!(prime::is_prime(3), true);
        assert_eq!(prime::is_prime(4), false);
        assert_eq!(prime::is_prime(5), true);
        assert_eq!(prime::is_prime(6), false);
        assert_eq!(prime::is_prime(7), true);
        assert_eq!(prime::is_prime(8), false);
        assert_eq!(prime::is_prime(9), false);
        assert_eq!(prime::is_prime(10), false);
        assert_eq!(prime::is_prime(11), true);
        assert_eq!(prime::is_prime(12), false);
        assert_eq!(prime::is_prime(13), true);
        assert_eq!(prime::is_prime(14), false);
        assert_eq!(prime::is_prime(15), false);
        assert_eq!(prime::is_prime(16), false);
        assert_eq!(prime::is_prime(17), true);
        assert_eq!(prime::is_prime(18), false);
        assert_eq!(prime::is_prime(19), true);
        assert_eq!(prime::is_prime(20), false);
        assert_eq!(prime::is_prime(21), false);
        assert_eq!(prime::is_prime(22), false);
        assert_eq!(prime::is_prime(23), true);
        assert_eq!(prime::is_prime(24), false);
        assert_eq!(prime::is_prime(25), false);
        assert_eq!(prime::is_prime(26), false);
        assert_eq!(prime::is_prime(27), false);
        assert_eq!(prime::is_prime(28), false);
        assert_eq!(prime::is_prime(29), true);
        assert_eq!(prime::is_prime(30), false);
        assert_eq!(prime::is_prime(31), true);
        assert_eq!(prime::is_prime(32), false);
        assert_eq!(prime::is_prime(33), false);
        assert_eq!(prime::is_prime(34), false);
        assert_eq!(prime::is_prime(35), false);
        assert_eq!(prime::is_prime(36), false);
        assert_eq!(prime::is_prime(37), true);
    }
}
