const MAX: i64 = 600851475143;

pub fn problem3() -> i64 {
    let sqrt = sqrt_floor(MAX);
    let mut candidate_factor = sqrt;

    while candidate_factor > 0 {
        if is_factor(MAX, candidate_factor) && is_prime(candidate_factor) {
            return candidate_factor;
        }
        candidate_factor -= 1;
    }
    0
}

fn is_factor(number: i64, potential_factor: i64) -> bool {
    number % potential_factor == 0
}

pub fn is_prime(n: i64) -> bool {
    let sqrt = sqrt_floor(n);
    for i in 2..=sqrt {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn sqrt_floor(n: i64) -> i64 {
    (n as f64).sqrt().floor() as i64
}
