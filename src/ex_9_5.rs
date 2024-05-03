// Calculate the magnitude of a vector by summing the squares of its coordinates
// and taking the square root. Use the `sqrt()` method to calculate the square
// root, like `v.sqrt()`.

use std::f64;

fn magnitude(vec: &[f64; 3]) -> f64 {
    let mut maginute_squared = 0.0;
    for v in vec {
        maginute_squared += v * v;
    }
    maginute_squared.sqrt()
}

// Normalize a vector by calculating its magnitude and dividing all of its
// coordinates by that magnitude.

fn normalize(vec: &mut [f64; 3]) {
    let magnitude = magnitude(vec);
    for v in vec {
        *v /= magnitude;
    }
}

pub fn print_exercise() {
    println!("Exercise");
    println!("----- 9.5 -----");

    println!(
        "Magnitude of a unit vector: {}",
        magnitude(&[0.0, 1.0, 0.0])
    );

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));

    println!("---------------");
    println!();
}

#[test]
fn test_magnite_of_unit_vector() {
    let unit_vec = [0.0, 1.0, 0.0];
    assert_eq!(magnitude(&unit_vec), 1.0);
}

#[test]
fn test_magnite_of_know_vector() {
    let unit_vec = [1.0, 4.0, 8.0];
    assert_eq!(magnitude(&unit_vec), 9.0);
}

#[test]
fn test_normalization_longer_vec() {
    // ensure that our vector is first longer than 1
    let mut unit_vec = [1.0, 4.0, 8.0];
    assert!(magnitude(&unit_vec) > 1.0);
    // then ensure that it has length 1 after normalization
    //
    normalize(&mut unit_vec);
    assert_eq!(magnitude(&unit_vec), 1.0);
}

#[test]
fn test_normalization_shorter_vec() {
    // ensure that our vector is first longer than 1
    let mut unit_vec = [0.0, 0.0, 0.1];
    assert!(magnitude(&unit_vec) < 1.0);
    // then ensure that it has length 1 after normalization
    //
    normalize(&mut unit_vec);
    assert_eq!(magnitude(&unit_vec), 1.0);
}
