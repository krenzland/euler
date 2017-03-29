use std::clone::Clone;
use std::cmp::PartialOrd;
use std::iter::FromIterator;
use ntheory::digits_to_num;

// Finds the next lexiographic permutation.
fn next_permutation<T: PartialOrd + Clone>(permutation: &[T]) -> Option<Vec<T>> {
    let mut k = -1;
    // k: maximal i s.t. permutation[i] < permutation[i+1]
    for (idx, (a, b)) in permutation.iter().zip(permutation.iter().skip(1)).enumerate() {
        if a < b {
            k = idx as isize;
        }
    }
    if k == -1 {
        return None;
    }

    let ak = &permutation[k as usize];
    let mut l = -1;
    // l: maximal i s.t. permutation[k] < permutation[i]
    for (idx, x) in permutation.iter().enumerate().skip(k as usize) {
        if ak < x {
            l = idx as isize;
        }
    }
    if l == -1 {
        return None;
    }

    let (k, l) = (k as usize, l as usize);
    let mut permutation = permutation.to_vec();
    permutation[..].swap(l, k);

    // Reverse sequence from k + 1 to end
    (&mut permutation[k + 1..]).reverse();

    Some(permutation)
}

fn p24() -> u64 {
    let mut perm = Vec::from_iter(0..10);

    // perm already is the first permutation!
    for _ in 1..1000000 {
        perm = next_permutation(perm.as_slice()).expect("No permutation found.");
    }
    digits_to_num(perm.as_slice())
}

pub fn main() {
    println!("{:?}", p24());
}

#[test]
fn test() {
    assert_eq!(p24(), 2783915460);
}

#[test]
fn test_next_permutation() {
    let mut perm = Vec::from_iter(0..3);
    let mut perms_is: Vec<u64> = vec![digits_to_num(perm.as_slice())];
    let perms_should: Vec<u64> = vec![12, 21, 102, 120, 201, 210];

    while let Some(p) = next_permutation(perm.as_slice()) {
        perms_is.push(digits_to_num(p.as_slice()));
        perm = p;
    }

    assert_eq!(perms_is, perms_should);
}
