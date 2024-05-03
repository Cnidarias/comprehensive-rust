fn fib(n: u32) -> u32 {
    if n < 2 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

pub fn print_exercise() {
    println!("Exercise");
    println!("----- 5.6 -----");

    let n = 20;
    println!("fib({n}) = {}", fib(n));

    println!("---------------");
    println!();
}

#[test]
fn test_fibonacci_seq_33() {
    let n = 33;
    let res = 3524578;
    assert_eq!(fib(n), res);
}
