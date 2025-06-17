const UPPER_LIMIT: i32 = 1_000;

fn is_multiple_of(num: i32, divisors: &[i32]) -> bool {
    divisors.iter().any(|&divisor| num % divisor == 0)
}

pub(crate) fn problem1() -> i32 {
    let mut sum = 0;
    for num in 0..UPPER_LIMIT {
        if is_multiple_of(num, &[3, 5]) {
            sum += num;
        }
    }
    sum
}
