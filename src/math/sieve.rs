//! Prime sieves and related arithmetic precomputations.

// ===== Prime generation =====

/// Returns a primality table for integers in `[0, limit]`.
///
/// `is_prime[x] == true` iff `x` is prime.
pub fn eratosthenes_is_prime(limit: usize) -> Vec<bool> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    if limit >= 1 {
        is_prime[1] = false;
    }
    if limit < 2 {
        return is_prime;
    }

    let mut i = 2;
    while i <= limit / i {
        if is_prime[i] {
            let mut j = i * i;
            while j <= limit {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    is_prime
}

/// Returns all primes in `[2, limit]` using the Eratosthenes sieve.
pub fn eratosthenes_primes(limit: usize) -> Vec<usize> {
    let is_prime = eratosthenes_is_prime(limit);
    (2..=limit).filter(|&x| is_prime[x]).collect()
}

/// Linear sieve in `O(limit)`.
///
/// Returns `(primes, spf)` where:
/// - `primes` is the increasing list of primes in `[2, limit]`
/// - `spf[x]` is the smallest prime factor of `x` (and `spf[1] = 1`)
pub fn linear_sieve(limit: usize) -> (Vec<usize>, Vec<usize>) {
    let mut primes = Vec::new();
    let mut spf = vec![0; limit + 1];
    if limit >= 1 {
        spf[1] = 1;
    }

    for x in 2..=limit {
        if spf[x] == 0 {
            spf[x] = x;
            primes.push(x);
        }
        for &p in &primes {
            if p > spf[x] || x > limit / p {
                break;
            }
            spf[x * p] = p;
        }
    }

    (primes, spf)
}

// ===== Factorization helpers =====

/// Factorizes `n` using a smallest-prime-factor table from `linear_sieve`.
///
/// Returns `[(prime, exponent), ...]` sorted by prime.
///
/// # Panics
///
/// Panics if `n == 0` or `n >= spf.len()`.
pub fn factorize_with_spf(mut n: usize, spf: &[usize]) -> Vec<(usize, usize)> {
    assert!(n > 0 && n < spf.len());
    if n == 1 {
        return vec![];
    }

    let mut factors = Vec::new();
    while n > 1 {
        let p = spf[n];
        assert!(p >= 2);
        let mut exp = 0;
        while n % p == 0 {
            n /= p;
            exp += 1;
        }
        factors.push((p, exp));
    }
    factors
}

// ===== Arithmetic-function tables =====

/// Computes Euler's totient for all integers in `[0, limit]`.
///
/// `phi[x]` equals the count of integers in `[1, x]` coprime with `x`.
pub fn euler_totient_table(limit: usize) -> Vec<usize> {
    let (_, spf) = linear_sieve(limit);
    let mut phi = vec![0; limit + 1];
    if limit >= 1 {
        phi[1] = 1;
    }

    for x in 2..=limit {
        let p = spf[x];
        let m = x / p;
        phi[x] = if m % p == 0 {
            phi[m] * p
        } else {
            phi[m] * (p - 1)
        };
    }
    phi
}

/// Computes Mobius values for all integers in `[0, limit]`.
///
/// `mu[x]` is:
/// - `0` if `x` is divisible by a square prime,
/// - `1` if `x` has an even number of distinct prime factors,
/// - `-1` if `x` has an odd number of distinct prime factors.
pub fn mobius_table(limit: usize) -> Vec<i8> {
    let (_, spf) = linear_sieve(limit);
    let mut mu = vec![0_i8; limit + 1];
    if limit >= 1 {
        mu[1] = 1;
    }

    for x in 2..=limit {
        let p = spf[x];
        let m = x / p;
        mu[x] = if m % p == 0 { 0 } else { -mu[m] };
    }
    mu
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_eratosthenes_primes() {
        assert_eq!(eratosthenes_primes(1), vec![]);
        assert_eq!(eratosthenes_primes(30), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn test_linear_sieve_and_factorization() {
        let (primes, spf) = linear_sieve(100);
        assert_eq!(&primes[..10], &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);

        assert_eq!(factorize_with_spf(1, &spf), vec![]);
        assert_eq!(factorize_with_spf(84, &spf), vec![(2, 2), (3, 1), (7, 1)]);
        assert_eq!(factorize_with_spf(97, &spf), vec![(97, 1)]);
    }

    #[test]
    fn test_totient_table() {
        let phi = euler_totient_table(10);
        assert_eq!(phi, vec![0, 1, 1, 2, 2, 4, 2, 6, 4, 6, 4]);
    }

    #[test]
    fn test_mobius_table() {
        let mu = mobius_table(10);
        assert_eq!(mu, vec![0, 1, -1, -1, 0, -1, 1, -1, 0, 0, 1]);
    }
}