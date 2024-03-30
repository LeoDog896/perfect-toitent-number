mod toitent;

use primes::Sieve;
use toitent::totient;

fn is_perfect_toitent(n: u64, sieve: &mut Sieve) -> bool {
    let mut t = n;
    let mut sum: u64 = 0;
    while t > 1 {
        t = totient(t, sieve);
        if t >= n {
            return false;
        }
        sum += t;
    }

    sum == n
}

fn main() {
    let mut prime_sieve = Sieve::new();

    for n in 2..u64::MAX {
        if is_perfect_toitent(n, &mut prime_sieve) {
            println!("{}", n);
        }
    }
}
