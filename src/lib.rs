//! # DW11 — Largest Value
//!
//! ```text
//! Largest: 45
//! ```
//! Do not edit `src/main.rs`; the bugs are in the library files.

/// Returns the largest value in `values`. `values` must not be empty.
///
/// ```
/// use dw11::largest;
///
/// assert_eq!(largest(&[12, 45, 7, 33, 21]), 45);
/// ```
///
/// The largest value can be the last one:
///
/// ```
/// use dw11::largest;
///
/// assert_eq!(largest(&[3, 8, 20]), 20);
/// ```
///
/// Every value can be negative:
///
/// ```
/// use dw11::largest;
///
/// assert_eq!(largest(&[-5, -2, -9]), -2);
/// ```
pub fn largest(values: &[i32]) -> i32 {
    let mut biggest = 0;
    for i in 1..values.len() - 1 {
        if values[i] > biggest {
            biggest = values[i];
        }
    }
    biggest
}
