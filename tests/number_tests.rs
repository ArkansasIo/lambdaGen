use lambdagen::number::{is_even, is_odd, is_prime};

#[test]
fn even_numbers_are_detected() {
    assert!(is_even(2));
    assert!(is_even(100));
    assert!(!is_even(7));
}

#[test]
fn odd_numbers_are_detected() {
    assert!(is_odd(3));
    assert!(is_odd(99));
    assert!(!is_odd(8));
}

#[test]
fn primes_are_detected() {
    assert!(!is_prime(-1));
    assert!(!is_prime(0));
    assert!(!is_prime(1));
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(is_prime(97));
    assert!(!is_prime(100));
}
