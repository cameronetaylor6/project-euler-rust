pub fn problem2() -> i32 {
    let mut sum = 0;
    let mut i = 0;
    let mut j = 1;
    let mut fib = fibonacci(i, j);
    while fib < 4_000_000 {
        if fib % 2 == 0 {
            sum += fib;
        }
        i = j;
        j = fib;
        fib = fibonacci(i, j);
    }
    sum
}

fn fibonacci(i: i32, j: i32) -> i32 {
    i + j
}
