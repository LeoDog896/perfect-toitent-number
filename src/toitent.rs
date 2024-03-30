//! Adapted from https://github.com/phillip-h/reikna/blob/master/src/totient.rs

use primes::PrimeSet;
use primes::Sieve;

// helper function to do the totient calculation
fn totient_calc(n: u64, sieve: &mut Sieve) -> u64 {
    let mut totient = n as f64;
    let mut factors = sieve.prime_factors(n);
    factors.dedup();
    for factor in factors {
        totient *= 1.0 - (1.0 / factor as f64);
    }

    totient as u64
}

/// Calculate the value of Euler's totient function for `n`.
///
/// This function uses Euler's product formula to compute the
/// totient function. This involves factoring `n`, meaning that
/// the computation of the totient function can take a long time
/// if `n` is a large prime or has a large prime factor.
///
/// # Examples
///
/// ```
/// assert_eq!(totient(17), 16);
/// assert_eq!(totient(36), 12);
/// ```
pub fn totient(n: u64, sieve: &mut Sieve) -> u64 {
    if n <= 2 {
        return 1;
    }

    if sieve.is_prime(n) {
        return n - 1;
    }

    totient_calc(n, sieve)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_totient() {
        let mut sieve = Sieve::new();

        assert_eq!(totient(0, &mut sieve), 1);
        assert_eq!(totient(1, &mut sieve), 1);
        assert_eq!(totient(2, &mut sieve), 1);
        assert_eq!(totient(3, &mut sieve), 2);
        assert_eq!(totient(4, &mut sieve), 2);
        assert_eq!(totient(5, &mut sieve), 4);
        assert_eq!(totient(6, &mut sieve), 2);
        assert_eq!(totient(7, &mut sieve), 6);
        assert_eq!(totient(8, &mut sieve), 4);
        assert_eq!(totient(9, &mut sieve), 6);
        assert_eq!(totient(99, &mut sieve), 60);
        assert_eq!(totient(10_809_483_705_896, &mut sieve), 5_404_726_850_224);
    }
}