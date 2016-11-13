//! Aufgabe 2.1: Primzahltest

fn is_prime(number: usize) -> bool {
    if number <= 1 {
        false
    } else if number <= 3 {
        true
    } else if number % 2 == 0
           || number % 3 == 0 {
        false
    } else {
        let mut i = 5;
        let mut w = 2;

        while i * i <= number {
            if number % i == 0 {
                return false
            }

            i += w;
            w = 6 - w;
        }

        true
    }
}

#[test]
fn small_primes() {
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(is_prime(5));
    assert!(is_prime(7));
}

#[test]
fn small_composites() {
    assert!(!is_prime(1));
    assert!(!is_prime(4));
    assert!(!is_prime(6));
    assert!(!is_prime(8));
    assert!(!is_prime(9));
}

#[test]
fn large_primes() {
    assert!(is_prime(1_300_769));
    assert!(is_prime(1_300_297));
    assert!(is_prime(7_367_287));
}

#[test]
fn large_composites() {
    assert!(!is_prime(908_209));
    assert!(!is_prime(3_073_009));
    assert!(!is_prime(4_897_369));
}
