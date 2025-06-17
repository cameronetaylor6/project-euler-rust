const START_CANDIDATE: i64 = 2520;
const DIVISOR_RANGE: std::ops::RangeInclusive<i64> = 2..=20;

pub(crate) fn problem5() -> i64 {
    let mut candidate = START_CANDIDATE;
    loop {
        if is_divisible_by_all(candidate) {
            return candidate;
        }
        candidate += 20;
    }
}

fn is_divisible_by_all(candidate: i64) -> bool {
    for i in DIVISOR_RANGE.rev() {
        if candidate % i != 0 {
            return false;
        }
    }
    true
}
