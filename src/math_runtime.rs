use crate::parallel::parallel_map;

pub fn parallel_prime_flags(values: Vec<u64>, workers: usize) -> Vec<bool> {
    parallel_map(values, workers, |n| {
        if n < 2 { return false; }
        if n == 2 { return true; }
        if n % 2 == 0 { return false; }
        let mut d = 3;
        while d <= n / d { if n % d == 0 { return false; } d += 2; }
        true
    })
}

pub fn parallel_sum_squares(values: Vec<f64>, workers: usize) -> f64 {
    parallel_map(values, workers, |x| x * x).into_iter().sum()
}
