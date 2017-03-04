use std::collections::HashSet;
use num::rational::Rational;

fn digits(x: u32) -> Vec<u32> {
    let mut digits: Vec<u32> = Vec::new();
    let mut r = x;
    while r > 0 {
        let digit = r % 10;
        r /= 10;
        digits.push(digit);
    }
    digits
}

fn is_curious_frac(nom: u32, denom: u32) -> bool {
    let nom_digs: HashSet<_> = digits(nom).into_iter().collect();
    let denom_digs: HashSet<_> = digits(denom).into_iter().collect();
    let mut intersec = nom_digs.intersection(&denom_digs);
    // Contains 0 -> divisible by 10, |intersec| = 0 -> nothing to cancel
    // |intersec| = 2 -> trivial
    if intersec.clone().count() != 1 || intersec.any(|&x| x == 0) {
        return false;
    }
    let nom_dig = nom_digs.iter().find(|x| !denom_digs.contains(x));
    let denom_dig = denom_digs.iter().find(|x| !nom_digs.contains(x));
    if let (Some(&nom_dig), Some(&denom_dig)) = (nom_dig, denom_dig) {
        let digit_cancel = (nom_dig as f32) / (denom_dig as f32);
        let proper_cancel = (nom as f32) / (denom as f32);
        let eps = 10e-6;
        f32::abs(digit_cancel - proper_cancel) < eps
    } else {
        false
    }
}

pub fn p33() -> u32 {
    let mut prod = Rational::new(1, 1);
    for nom in 10..100 {
        for denom in 10..100 {
            if (nom as f32) / (denom as f32) > 1.0 {
                continue;
            }
            if is_curious_frac(nom, denom) {
                let ratio = Rational::new(nom as isize, denom as isize);
                prod = prod * ratio;
            }
        }
    }
    *prod.denom() as u32
}

pub fn main() {
    println!("{}", p33());
}

#[test]
fn test_is_curious_frac() {
    assert!(is_curious_frac(49, 98));
    assert!(is_curious_frac(16, 64));
    assert!(is_curious_frac(19, 95));
    assert!(is_curious_frac(49, 98));
    assert!(!is_curious_frac(49, 89));
    assert!(!is_curious_frac(30, 50));
    assert!(!is_curious_frac(12, 36));
}

#[test]
fn test() {
    assert_eq!(p33(), 100);
}
