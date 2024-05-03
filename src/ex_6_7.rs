fn collatz_length(mut n: i32) -> u32 {
    assert!(n > 0, "n must be greater than 0");

    let mut len = 1;
    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        len += 1;
    }

    len
}

pub fn print_exercise() {
    println!("Exercise");
    println!("----- 6.7 -----");

    let n = 3;
    println!("collatz_length({n}) = {}", collatz_length(n));

    println!("---------------");
    println!();
}

#[test]
fn test_collatz_length_with_3() {
    assert_eq!(collatz_length(3), 8);
}

#[test]
fn test_collatz_length_with_11() {
    assert_eq!(collatz_length(11), 15);
}

#[test]
#[should_panic(expected = "n must be greater than 0")]
fn test_collatz_length_with_0() {
    collatz_length(0);
}
