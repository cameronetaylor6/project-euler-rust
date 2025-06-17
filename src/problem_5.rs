const MIN: i64 = 2520;

pub(crate) fn problem5() -> i64 {
    let mut candidate = MIN;
    loop {
        if is_evenly_divisible(candidate) {
            return candidate;
        }
        candidate += 20;
    }
}

fn is_evenly_divisible(candidate: i64) -> bool {
    for i in (2..=20).rev() {
        if candidate % i != 0 {
            return false;
        }
    }
    true
}
