const MAX_FIBONACCI: i32 = 4_000_000;

pub fn problem2() -> i32 {
    sum_of_even_fibonacci(MAX_FIBONACCI)
}

fn sum_of_even_fibonacci(limit: i32) -> i32 {
    let mut sum = 0;
    let mut prev = 1;
    let mut curr = 2;

    while curr < limit {
        if curr % 2 == 0 {
            sum += curr;
        }
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    sum
}
