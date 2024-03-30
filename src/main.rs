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
    let mut handles = vec![];
    let max_threads = num_cpus::get();
    for i in 0..max_threads {
        let handle = std::thread::spawn(move || {
            let mut sieve = Sieve::new();
            for n in (2..u64::MAX).skip(i).step_by(max_threads) {
                // println!("Thread {}: {}", i, n);
                if is_perfect_toitent(n, &mut sieve) {
                    println!("{}", n);
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
