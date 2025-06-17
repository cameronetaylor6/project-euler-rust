pub fn problem3() -> i64 {
    const MAX: i64 = 600851475143;
    let mut i = (MAX as f64).sqrt().floor() as i64;
    while i > 0 {
        if MAX % i == 0 && is_prime(i) {
            return i;
        }
        i -= 1;
    }
    0
}

pub fn is_prime(n: i64) -> bool {
    let sqrt = (n as f64).sqrt().floor() as i64;
    for i in 2..sqrt {
        if n % i == 0 {
            return false;
        }
    }
    true
}
