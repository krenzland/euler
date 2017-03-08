use ntheory::digits;
use std::collections::HashSet;
use std::iter::FromIterator;

// Does num*(1..) contain the same digits for all factors up to order?
fn is_perm_multip(num: u64, order: u64) -> bool {
    let num_digs: HashSet<u64> = HashSet::from_iter(digits(num));
    for factor in 1..(order + 1) {
        let new_num = num * factor;
        let new_digs: HashSet<u64> = HashSet::from_iter(digits(new_num));
        let inters = num_digs.intersection(&new_digs);
        if num_digs.len() != new_digs.len() || inters.count() != num_digs.len() {
            return false;
        }
    }
    true
}

fn max_perm_multip() -> u64 {
    (1..1000000).find(|&x| is_perm_multip(x, 6)).expect("No multiple found")
}

pub fn main() {
    println!("{}", max_perm_multip());
}

#[test]
fn test() {
    assert_eq!(max_perm_multip(), 142857);
}

#[test]
fn test_is_perm_multip() {
    assert!(is_perm_multip(125874, 2));
    assert!(!is_perm_multip(12, 2));
    assert!(!is_perm_multip(125874, 3));
    assert!(is_perm_multip(142857, 6));
    assert!(!is_perm_multip(142857, 7));
}
