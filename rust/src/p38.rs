use std::cmp::max;
use ntheory::{digits, digits_to_num, is_pandigital};

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
fn test() {
    assert_eq!(max_pandigital(), 932718654);
}
