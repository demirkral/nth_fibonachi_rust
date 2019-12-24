fn main() {
    let n = 93;
    let fib = calculate_fib(n);

    println!("The {} number the fib squence is {}", n, fib);
}

fn calculate_fib(mut n: usize) -> usize {   
    // 0,  1,  1,  2,  3,  5,  8,  13,  21,  ...

    n -= 1;
    let mut first = 0;
    let mut second = 1;
    let mut current: usize;

    while n > 0 {
        current = first + second;
        first = second;
        second = current;
        n -= 1;
    }
    second
}