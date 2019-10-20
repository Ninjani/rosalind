use std::collections::HashSet;
use std::hash::Hash;

use num::BigUint;

/// Permutation $^nC_r$
pub fn ncr(n: u64, r: u64) -> BigUint {
    let r = r.min(n - r);
    if r == 0 {
        return BigUint::from(1u64);
    }
    let numerator: BigUint = ((n - r + 1)..=n).product();
    let denominator: BigUint = (1..=r).product();
    numerator / denominator
}

pub fn factorial(n: usize) -> BigUint {
    (1..=n).product()
}

/// max/min argmax/argmin for floats
pub trait Comparable: Sized {
    /// max value in slice
    fn max(array: &[Self]) -> Self;
    /// min value in slice
    fn min(array: &[Self]) -> Self;
    /// (index of max value, max value) in slice
    fn argmax_max(array: &[Self]) -> (usize, Self);
    /// (index of min value, min value) in slice
    fn argmin_min(array: &[Self]) -> (usize, Self);
}

impl Comparable for f64 {
    fn max(array: &[Self]) -> Self {
        array.to_vec().into_iter().fold(::std::f64::NAN, f64::max)
    }
    fn min(array: &[Self]) -> Self {
        array.to_vec().into_iter().fold(0., f64::min)
    }
    fn argmax_max(array: &[Self]) -> (usize, Self) {
        let mut max_index = 0;
        let mut max_value = ::std::f64::MIN;
        for (i, a) in array.to_vec().into_iter().enumerate() {
            if a > max_value {
                max_index = i;
                max_value = a;
            }
        }
        (max_index, max_value)
    }
    fn argmin_min(array: &[Self]) -> (usize, Self) {
        let mut min_index = 0;
        let mut min_value = ::std::f64::MAX;
        for (i, a) in array.to_vec().into_iter().enumerate() {
            if a < min_value {
                min_index = i;
                min_value = a;
            }
        }
        (min_index, min_value)
    }
}

impl Comparable for f32 {
    fn max(array: &[Self]) -> Self {
        array.to_vec().into_iter().fold(::std::f32::NAN, f32::max)
    }
    fn min(array: &[Self]) -> Self {
        array.to_vec().into_iter().fold(0., f32::min)
    }
    fn argmax_max(array: &[Self]) -> (usize, Self) {
        let mut max_index = 0;
        let mut max_value = ::std::f32::MIN;
        for (i, a) in array.to_vec().into_iter().enumerate() {
            if a > max_value {
                max_index = i;
                max_value = a;
            }
        }
        (max_index, max_value)
    }
    fn argmin_min(array: &[Self]) -> (usize, Self) {
        let mut min_index = 0;
        let mut min_value = ::std::f32::MAX;
        for (i, a) in array.to_vec().into_iter().enumerate() {
            if a < min_value {
                min_index = i;
                min_value = a;
            }
        }
        (min_index, min_value)
    }
}

/// pop a set (random ordering)
pub fn set_pop<T: Hash + Eq + Clone>(set: &mut HashSet<T>) -> Option<T> {
    let next_value = set.iter().next().cloned();
    match next_value {
        Some(x) => set.take(&x),
        None => None,
    }
}
