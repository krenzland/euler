use ntheory::is_palindrome;
use num::bigint::BigInt;
use num::FromPrimitive;
use std::str::FromStr;

fn is_lychrel(num: u64) -> bool {
    let mut num = BigInt::from_u64(num).unwrap();
    for _ in 0..50 {
        let digs_rev: String = num.to_str_radix(10).chars().rev().collect();
        let num_rev = BigInt::from_str(&digs_rev).unwrap();
        num = num + num_rev;
        let digits = num.to_str_radix(10);
        if is_palindrome(digits.as_bytes()) {
            return false;
        }

    }
    true
}

fn count_lychrel(max: u64) -> usize {
    (1..max).filter(|&x| is_lychrel(x)).count()
}

pub fn main() {
    println!("{}", count_lychrel(10000));
}

#[test]
fn test_is_lychrel() {
    assert!(!is_lychrel(56));
    assert!(is_lychrel(196));
}

#[test]
fn test() {
    assert_eq!(count_lychrel(10000), 249);
}
