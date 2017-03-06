use ntheory::digit_sum_str;
use num::bigint::BigInt;
use num::FromPrimitive;
use std::cmp::max;

fn maximum_digit_sum(a_max: u64, b_max: u64) -> u32 {
    let mut max_sum = 0;
    for a in 2..(a_max + 1) {
        let a = BigInt::from_u64(a).unwrap();
        let mut cur = BigInt::from_u64(1).unwrap();
        let mut b = 1;
        while b < b_max {
            cur = &a * cur;
            b += 1;
            let digit_sum = digit_sum_str(&cur.to_str_radix(10));
            max_sum = max(max_sum, digit_sum);
        }
    }
    max_sum
}

pub fn main() {
    println!("{}", maximum_digit_sum(99, 99));
}

#[test]
fn test() {
    assert_eq!(maximum_digit_sum(99, 99), 972);
}
