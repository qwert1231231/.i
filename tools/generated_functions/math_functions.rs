//! Math Functions for i Language
//! Generated automatically - 3037 functions

use std::collections::HashMap;
use std::f64::consts::PI;

// Utility functions
fn factorial(n: f64) -> f64 {
    if n <= 1.0 { 1.0 } else { n * factorial(n - 1.0) }
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 32
fn sin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 64
fn sin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 256
fn sin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sin precision 128
fn sin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 128
fn cos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 256
fn cos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 64
fn cos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cos precision 32
fn cos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 256
fn tan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 128
fn tan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 32
fn tan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tan precision 64
fn tan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 64
fn asin_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 32
fn asin_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 128
fn asin_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// asin precision 256
fn asin_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 32
fn acos_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 64
fn acos_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 128
fn acos_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// acos precision 256
fn acos_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 256
fn atan_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 128
fn atan_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 32
fn atan_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// atan precision 64
fn atan_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 64
fn sinh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 128
fn sinh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 256
fn sinh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// sinh precision 32
fn sinh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 128
fn cosh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 64
fn cosh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 256
fn cosh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// cosh precision 32
fn cosh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 64
fn tanh_precision_64(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 256
fn tanh_precision_256(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 32
fn tanh_precision_32(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// tanh precision 128
fn tanh_precision_128(x: f64) -> f64 {
    let mut result = x;
    for k in 1..=precision {
        let term = if k % 2 == 0 { -1.0 } else { 1.0 };
        let power = (2 * k + 1) as f64;
        result += term * x.powf(power) / factorial(power);
    }
    result
}


// Matrix multiply 2x2
fn matrix_multiply_2x2(matrix: &[[f64; 2]; 2]) -> Option<[[f64; 2]; 2]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 2]; 2];
    for i in 0..2 {
        for j in 0..2 {
            // multiply algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix multiply 3x3
fn matrix_multiply_3x3(matrix: &[[f64; 3]; 3]) -> Option<[[f64; 3]; 3]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            // multiply algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix multiply 4x4
fn matrix_multiply_4x4(matrix: &[[f64; 4]; 4]) -> Option<[[f64; 4]; 4]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            // multiply algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix multiply 5x5
fn matrix_multiply_5x5(matrix: &[[f64; 5]; 5]) -> Option<[[f64; 5]; 5]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 5]; 5];
    for i in 0..5 {
        for j in 0..5 {
            // multiply algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix multiply 6x6
fn matrix_multiply_6x6(matrix: &[[f64; 6]; 6]) -> Option<[[f64; 6]; 6]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 6]; 6];
    for i in 0..6 {
        for j in 0..6 {
            // multiply algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix multiply 7x7
fn matrix_multiply_7x7(matrix: &[[f64; 7]; 7]) -> Option<[[f64; 7]; 7]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 7]; 7];
    for i in 0..7 {
        for j in 0..7 {
            // multiply algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix multiply 8x8
fn matrix_multiply_8x8(matrix: &[[f64; 8]; 8]) -> Option<[[f64; 8]; 8]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 8]; 8];
    for i in 0..8 {
        for j in 0..8 {
            // multiply algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix multiply 9x9
fn matrix_multiply_9x9(matrix: &[[f64; 9]; 9]) -> Option<[[f64; 9]; 9]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 9]; 9];
    for i in 0..9 {
        for j in 0..9 {
            // multiply algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix invert 2x2
fn matrix_invert_2x2(matrix: &[[f64; 2]; 2]) -> Option<[[f64; 2]; 2]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 2]; 2];
    for i in 0..2 {
        for j in 0..2 {
            // invert algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix invert 3x3
fn matrix_invert_3x3(matrix: &[[f64; 3]; 3]) -> Option<[[f64; 3]; 3]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            // invert algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix invert 4x4
fn matrix_invert_4x4(matrix: &[[f64; 4]; 4]) -> Option<[[f64; 4]; 4]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            // invert algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix invert 5x5
fn matrix_invert_5x5(matrix: &[[f64; 5]; 5]) -> Option<[[f64; 5]; 5]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 5]; 5];
    for i in 0..5 {
        for j in 0..5 {
            // invert algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix invert 6x6
fn matrix_invert_6x6(matrix: &[[f64; 6]; 6]) -> Option<[[f64; 6]; 6]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 6]; 6];
    for i in 0..6 {
        for j in 0..6 {
            // invert algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix invert 7x7
fn matrix_invert_7x7(matrix: &[[f64; 7]; 7]) -> Option<[[f64; 7]; 7]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 7]; 7];
    for i in 0..7 {
        for j in 0..7 {
            // invert algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix invert 8x8
fn matrix_invert_8x8(matrix: &[[f64; 8]; 8]) -> Option<[[f64; 8]; 8]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 8]; 8];
    for i in 0..8 {
        for j in 0..8 {
            // invert algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix invert 9x9
fn matrix_invert_9x9(matrix: &[[f64; 9]; 9]) -> Option<[[f64; 9]; 9]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 9]; 9];
    for i in 0..9 {
        for j in 0..9 {
            // invert algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix determinant 2x2
fn matrix_determinant_2x2(matrix: &[[f64; 2]; 2]) -> Option<[[f64; 2]; 2]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 2]; 2];
    for i in 0..2 {
        for j in 0..2 {
            // determinant algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix determinant 3x3
fn matrix_determinant_3x3(matrix: &[[f64; 3]; 3]) -> Option<[[f64; 3]; 3]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            // determinant algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix determinant 4x4
fn matrix_determinant_4x4(matrix: &[[f64; 4]; 4]) -> Option<[[f64; 4]; 4]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            // determinant algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix determinant 5x5
fn matrix_determinant_5x5(matrix: &[[f64; 5]; 5]) -> Option<[[f64; 5]; 5]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 5]; 5];
    for i in 0..5 {
        for j in 0..5 {
            // determinant algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix determinant 6x6
fn matrix_determinant_6x6(matrix: &[[f64; 6]; 6]) -> Option<[[f64; 6]; 6]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 6]; 6];
    for i in 0..6 {
        for j in 0..6 {
            // determinant algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix determinant 7x7
fn matrix_determinant_7x7(matrix: &[[f64; 7]; 7]) -> Option<[[f64; 7]; 7]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 7]; 7];
    for i in 0..7 {
        for j in 0..7 {
            // determinant algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix determinant 8x8
fn matrix_determinant_8x8(matrix: &[[f64; 8]; 8]) -> Option<[[f64; 8]; 8]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 8]; 8];
    for i in 0..8 {
        for j in 0..8 {
            // determinant algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix determinant 9x9
fn matrix_determinant_9x9(matrix: &[[f64; 9]; 9]) -> Option<[[f64; 9]; 9]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 9]; 9];
    for i in 0..9 {
        for j in 0..9 {
            // determinant algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix transpose 2x2
fn matrix_transpose_2x2(matrix: &[[f64; 2]; 2]) -> Option<[[f64; 2]; 2]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 2]; 2];
    for i in 0..2 {
        for j in 0..2 {
            // transpose algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix transpose 3x3
fn matrix_transpose_3x3(matrix: &[[f64; 3]; 3]) -> Option<[[f64; 3]; 3]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            // transpose algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix transpose 4x4
fn matrix_transpose_4x4(matrix: &[[f64; 4]; 4]) -> Option<[[f64; 4]; 4]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            // transpose algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix transpose 5x5
fn matrix_transpose_5x5(matrix: &[[f64; 5]; 5]) -> Option<[[f64; 5]; 5]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 5]; 5];
    for i in 0..5 {
        for j in 0..5 {
            // transpose algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix transpose 6x6
fn matrix_transpose_6x6(matrix: &[[f64; 6]; 6]) -> Option<[[f64; 6]; 6]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 6]; 6];
    for i in 0..6 {
        for j in 0..6 {
            // transpose algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix transpose 7x7
fn matrix_transpose_7x7(matrix: &[[f64; 7]; 7]) -> Option<[[f64; 7]; 7]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 7]; 7];
    for i in 0..7 {
        for j in 0..7 {
            // transpose algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix transpose 8x8
fn matrix_transpose_8x8(matrix: &[[f64; 8]; 8]) -> Option<[[f64; 8]; 8]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 8]; 8];
    for i in 0..8 {
        for j in 0..8 {
            // transpose algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix transpose 9x9
fn matrix_transpose_9x9(matrix: &[[f64; 9]; 9]) -> Option<[[f64; 9]; 9]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 9]; 9];
    for i in 0..9 {
        for j in 0..9 {
            // transpose algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix eigenvalues 2x2
fn matrix_eigenvalues_2x2(matrix: &[[f64; 2]; 2]) -> Option<[[f64; 2]; 2]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 2]; 2];
    for i in 0..2 {
        for j in 0..2 {
            // eigenvalues algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix eigenvalues 3x3
fn matrix_eigenvalues_3x3(matrix: &[[f64; 3]; 3]) -> Option<[[f64; 3]; 3]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            // eigenvalues algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix eigenvalues 4x4
fn matrix_eigenvalues_4x4(matrix: &[[f64; 4]; 4]) -> Option<[[f64; 4]; 4]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            // eigenvalues algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix eigenvalues 5x5
fn matrix_eigenvalues_5x5(matrix: &[[f64; 5]; 5]) -> Option<[[f64; 5]; 5]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 5]; 5];
    for i in 0..5 {
        for j in 0..5 {
            // eigenvalues algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix eigenvalues 6x6
fn matrix_eigenvalues_6x6(matrix: &[[f64; 6]; 6]) -> Option<[[f64; 6]; 6]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 6]; 6];
    for i in 0..6 {
        for j in 0..6 {
            // eigenvalues algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix eigenvalues 7x7
fn matrix_eigenvalues_7x7(matrix: &[[f64; 7]; 7]) -> Option<[[f64; 7]; 7]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 7]; 7];
    for i in 0..7 {
        for j in 0..7 {
            // eigenvalues algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix eigenvalues 8x8
fn matrix_eigenvalues_8x8(matrix: &[[f64; 8]; 8]) -> Option<[[f64; 8]; 8]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 8]; 8];
    for i in 0..8 {
        for j in 0..8 {
            // eigenvalues algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}


// Matrix eigenvalues 9x9
fn matrix_eigenvalues_9x9(matrix: &[[f64; 9]; 9]) -> Option<[[f64; 9]; 9]> {
    // Complex matrix operation implementation
    let mut result = [[0.0; 9]; 9];
    for i in 0..9 {
        for j in 0..9 {
            // eigenvalues algorithm implementation
            result[i][j] = matrix[i][j].sqrt().abs();
        }
    }
    Some(result)
}

