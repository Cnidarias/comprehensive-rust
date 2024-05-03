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
