//! Contains functionality for working with numbers.

/// Returns `true` if the given number is prime, `false` otherwise.
///
/// Might block for a while if n is large.
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

/// Returns the next prime number after the given number.
pub fn next_prime(n: u64) -> u64 {
    let mut n = n + 1;
    while !is_prime(n) {
        n += 1;
    }
    n
}

/// Returns false if the given number is prime, true otherwise.
pub fn not_prime(n: u64) -> bool {
    !is_prime(n)
}

/// Returns the successor of the given number.
///
/// # Examples
///
/// ```
/// use glennib_thelib::succ;
/// let n = 41;
/// assert_eq!(succ(n), 42);
/// ```
pub fn succ(number: u64) -> u64 {
    number + 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_prime_works() {
        let expectations = [
            (1, false),
            (2, true),
            (3, true),
            (4, false),
            (5, true),
            (6, false),
            (7, true),
            (8, false),
            (9, false),
            (10, false),
            (11, true),
            (12, false),
            (13, true),
        ];
        for (n, expected) in expectations {
            assert_eq!(is_prime(n), expected);
        }
    }

    #[test]
    fn next_prime_works() {
        let expectations = [
            (1, 2),
            (2, 3),
            (3, 5),
            (4, 5),
            (5, 7),
            (6, 7),
            (7, 11),
            (8, 11),
            (9, 11),
            (10, 11),
            (11, 13),
            (12, 13),
            (13, 17),
        ];
        for (n, expected) in expectations {
            assert_eq!(next_prime(n), expected);
        }
    }
}
