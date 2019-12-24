fn main() {
    let n = 9;
    let fib = calculate_fib(n);

    println!("The {} number the fib squence is {}", n, fib);
}

fn calculate_fib(n: u32) -> u32 {
    // 0,  1,  1,  2,  3,  5,  8,  13,  21,  ...

    let mut count = 2;
    let mut first = 0;
    let mut second = 1;
    let mut current: u32;

    while count < n {
        current = first + second;
        first = second;
        second = current;
        count += 1;
    }
    second
}
