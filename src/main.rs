fn main() {
    let nth_fibonacci_number = 4;
    let result = fibonacci(nth_fibonacci_number);

    println!("The {nth_fibonacci_number}º Fibonacci number is: {result}");
}

fn fibonacci(number: u64) -> u64 {
    if number == 0 {
        return 0;
    } else if number == 1 {
        return 1;
    } else {
        return fibonacci(number - 1) + fibonacci(number - 2)
    }
}