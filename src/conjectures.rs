//! Computational experiments for major mathematical conjectures.
//!
//! These routines are verification/search algorithms. They do NOT constitute
//! proofs or disproofs of open conjectures.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GoldbachPair {
    pub even: u64,
    pub p: u64,
    pub q: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TwinPrimePair {
    pub p: u64,
    pub q: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CollatzResult {
    pub start: u64,
    pub steps: u64,
    pub peak: u64,
    pub reached_one: bool,
}

pub fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let mut d = 3u64;
    while d <= n / d {
        if n % d == 0 { return false; }
        d += 2;
    }
    true
}

/// Strong Goldbach experiment: find a prime pair for every even n in range.
pub fn goldbach_test(max_even: u64) -> Vec<GoldbachPair> {
    let mut out = Vec::new();
    for n in (4..=max_even).filter(|n| n % 2 == 0) {
        let mut found = false;
        for p in 2..=n / 2 {
            if is_prime(p) && is_prime(n - p) {
                out.push(GoldbachPair { even: n, p, q: n - p });
                found = true;
                break;
            }
        }
        if !found { break; }
    }
    out
}

/// Search for twin-prime pairs p,p+2 through the supplied bound.
pub fn twin_primes(max_n: u64) -> Vec<TwinPrimePair> {
    (3..=max_n.saturating_sub(2))
        .filter(|&p| is_prime(p) && is_prime(p + 2))
        .map(|p| TwinPrimePair { p, q: p + 2 })
        .collect()
}

/// Compute a Collatz trajectory until 1, overflow, or a configurable limit.
pub fn collatz(start: u64, max_steps: u64) -> CollatzResult {
    if start == 0 { return CollatzResult { start, steps: 0, peak: 0, reached_one: false }; }
    let mut n = start;
    let mut peak = n;
    let mut steps = 0;
    while n != 1 && steps < max_steps {
        n = if n % 2 == 0 { n / 2 } else { n.saturating_mul(3).saturating_add(1) };
        peak = peak.max(n);
        steps += 1;
    }
    CollatzResult { start, steps, peak, reached_one: n == 1 }
}

pub fn collatz_test(max_start: u64, max_steps: u64) -> Vec<CollatzResult> {
    (1..=max_start).map(|n| collatz(n, max_steps)).collect()
}

/// Legendre experiment: search between n² and (n+1)² for a prime.
pub fn legendre_test(max_n: u64) -> Vec<(u64, Option<u64>)> {
    (1..=max_n).map(|n| {
        let lo = n.saturating_mul(n);
        let hi = (n + 1).saturating_mul(n + 1);
        let p = ((lo + 1)..hi).find(|&x| is_prime(x));
        (n, p)
    }).collect()
}

/// Polignac experiment: for an even gap k, search for consecutive primes
/// separated by k. This is a finite computational test of the pattern.
pub fn polignac_test(gap: u64, max_n: u64) -> Vec<(u64, u64)> {
    if gap == 0 || gap % 2 != 0 { return Vec::new(); }
    let mut primes = Vec::new();
    for n in 2..=max_n { if is_prime(n) { primes.push(n); } }
    primes.windows(2)
        .filter_map(|w| if w[1] - w[0] == gap { Some((w[0], w[1])) } else { None })
        .collect()
}

/// Bertrand-style experiment: every n>1 has a prime p with n<p<2n.
pub fn bertrand_test(max_n: u64) -> Vec<(u64, Option<u64>)> {
    (2..=max_n).map(|n| (n, ((n + 1)..(2 * n)).find(|&x| is_prime(x)))).collect()
}

/// Computational Riemann-Hypothesis probe using zeta zeros supplied by the
/// caller. A full proof cannot be reduced to finite numerical checking.
pub fn riemann_critical_line_probe(imaginary_parts: &[f64], real_parts: &[f64], tolerance: f64) -> bool {
    imaginary_parts.len() == real_parts.len()
        && real_parts.iter().all(|&re| (re - 0.5).abs() <= tolerance)
}

/// A compact registry for the conjectures currently implemented by LambdaGen.
pub fn conjecture_registry() -> &'static [&'static str] {
    &[
        "Goldbach",
        "Twin Prime",
        "Collatz",
        "Legendre",
        "Polignac",
        "Bertrand (theorem verification)",
        "Riemann Hypothesis (finite critical-line probe)",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn goldbach_through_100() {
        assert_eq!(goldbach_test(100).len(), 49);
    }

    #[test]
    fn twin_primes_include_3_5() {
        assert!(twin_primes(10).contains(&TwinPrimePair { p: 3, q: 5 }));
    }

    #[test]
    fn collatz_13_reaches_one() {
        let r = collatz(13, 1000);
        assert!(r.reached_one);
        assert_eq!(r.steps, 9);
    }

    #[test]
    fn legendre_has_a_prime_through_20() {
        assert!(legendre_test(20).iter().all(|(_, p)| p.is_some()));
    }

    #[test]
    fn polignac_gap_two_has_pairs() {
        assert!(!polignac_test(2, 100).is_empty());
    }

    #[test]
    fn bertrand_holds_in_small_range() {
        assert!(bertrand_test(100).iter().all(|(_, p)| p.is_some()));
    }
}
