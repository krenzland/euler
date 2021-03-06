use bit_vec::BitVec;
use std::collections::HashSet;

pub fn digits_base(x: u64, base: u64) -> Vec<u64> {
    let mut digits: Vec<u64> = Vec::new();
    let mut r = x;
    while r > 0 {
        let digit = r % base;
        r /= base;
        digits.push(digit);
    }
    digits.into_iter().rev().collect()
}

pub fn digits_to_num(digits: &[u64]) -> u64 {
    let mut num = 0;
    for &dig in digits.iter() {
        num *= 10;
        num += dig;
    }
    num
}

pub fn digits(x: u64) -> Vec<u64> {
    digits_base(x, 10)
}

pub fn divisors(x: u32) -> Vec<u32> {
    let upper_bound = x / 2 + 1;
    let mut divisors: Vec<u32> = Vec::new();
    for i in 1..upper_bound {
        if x % i == 0 {
            divisors.push(i);
        }
    }
    divisors
}

pub fn primes(max: usize) -> BitVec {
    let mut primes = BitVec::from_elem(max, true);

    primes.set(0, false);
    primes.set(1, false);

    let upper_bound = ((max as f64).sqrt() as usize) + 1;
    for i in 2..upper_bound {
        if primes[i] {
            let mut cur = i * i;
            while cur < max {
                primes.set(cur, false);
                cur += i;
            }
        }
    }

    primes
}

pub fn digit_sum_str(x: &str) -> u32 {
    let mut sum = 0;
    for c in x.chars() {
        let digit = c.to_digit(10).unwrap();
        sum += digit;
    }
    sum
}

pub fn is_palindrome<T: Eq>(list: &[T]) -> bool {
    for (a, b) in list.iter().rev().zip(list) {
        if a != b {
            return false;
        }
    }
    true
}

pub fn is_pandigital(digits: &[u64]) -> bool {
    if digits.len() != 9 {
        return false;
    }
    let digits: HashSet<u64> = digits.to_owned().into_iter().collect();
    !digits.contains(&0) && digits.len() == 9
}

#[test]
fn test_divisors() {
    assert_eq!(divisors(28), vec![1, 2, 4, 7, 14]);
}

#[test]
fn test_primes() {
    let primes_is = primes(100);
    let primes_should = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61,
                             67, 71, 73, 79, 83, 89, 97];
    let composites = vec![4, 8, 12, 14, 15, 16, 84, 90];
    for prime in primes_should {
        assert!(primes_is[prime]);
    }
    for composite in composites {
        assert!(!primes_is[composite]);
    }
}

#[test]
fn test_digit_sum_str() {
    assert_eq!(digit_sum_str("1"), 1);
    assert_eq!(digit_sum_str("17"), 8);
    assert_eq!(digit_sum_str("18"), 9);
    assert_eq!(digit_sum_str("391"), 13);
    assert_eq!(digit_sum_str("101"), 2);
}

#[test]
fn test_digits() {
    assert_eq!(digits(10), vec![1, 0]);
    assert_eq!(digits(132), vec![1, 3, 2]);
    assert_eq!(digits(983), vec![9, 8, 3]);
}
#[test]
fn test_digits_to_num() {
    for i in 123..322 {
        let digits = digits(i as u64);
        assert!(i == digits_to_num(&digits));
    }
}

#[test]
fn test_is_pandigital() {
    assert!(is_pandigital(&digits(129384576)));
    assert!(is_pandigital(&digits(627384591)));
    assert!(!is_pandigital(&digits(929384576)));
}
