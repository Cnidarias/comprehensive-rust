pub mod ex_5_6;
pub mod ex_6_7;

fn main() {
    let n = 20;
    println!("Exercise 5.6: fib({n}) = {}", ex_5_6::fib(n));

    let n = 3;
    println!(
        "Exercise 6.7: collatz_length({n}) = {}",
        ex_6_7::collatz_length(n)
    );
}
