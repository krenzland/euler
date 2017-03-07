use std::collections::HashSet;
use std::cmp::max;
use ntheory::{digits, digits_to_num};

#[inline]
fn is_pandigital(digits: &[u64]) -> bool {
    if digits.len() != 9 {
        return false;
    }
    let digits: HashSet<u64> = digits.to_owned().into_iter().collect();
    !digits.contains(&0) && digits.len() == 9
}

fn max_pandigital() -> u64 {
    let mut max_pan = 0;
    for n in 5000..10001 {
        let mut pan_digs: Vec<u64> = digits(n);
        for factor in 2..1000 {
            let n = factor * n;
            let digs = digits(n);
            pan_digs.extend(digs.into_iter());
            if pan_digs.len() < 9 {
                continue;
            }
            if is_pandigital(&pan_digs) {
                let pan_digs: Vec<u64> = pan_digs;
                max_pan = max(max_pan, digits_to_num(&pan_digs));
            }
            break;
        }
    }
    max_pan
}

pub fn main() {
    println!("{}", max_pandigital());
}

#[test]
fn test_is_pandigital() {
    assert!(is_pandigital(&digits(129384576)));
    assert!(is_pandigital(&digits(627384591)));
    assert!(!is_pandigital(&digits(929384576)));
}

#[test]
fn test() {
    assert_eq!(max_pandigital(), 932718654);
}
