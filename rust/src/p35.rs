use ntheory::{digits, primes};

// Takes a set of digits and calculates all possible cyclic permutated numbers.
fn digit_permutations(digs: &[u64]) -> Vec<u64> {
    let digs: Vec<u64> = digs.to_owned().into_iter().rev().collect();
    let mut result = Vec::new();
    for start in 0..digs.len() {
        let mut num = 0;
        let mut power = 1;
        for i in 0..digs.len() {
            num += power * digs[(start + i) % digs.len()];
            power *= 10;
        }
        result.push(num);
    }
    result
}

fn circular_count(max: usize) -> usize {
    let primes = primes(max);
    (1..max).filter(|&i| primes[i]) // Small optimisation.
            .filter(|&i| {
                let digits: Vec<u64> = digits(i as u64);
                let permutations = digit_permutations(&digits);
                permutations.iter().all(|&x| primes[x as usize])
            })
    .count()
}

pub fn main() {
    println!("{}", circular_count(1000000));
}

#[test]
fn test_permutations() {
    let digits = digits(197);
    let mut perm_should = vec![197, 719, 971];
    let mut perm_is = digit_permutations(&digits);
    perm_should.sort();
    perm_is.sort();
    assert_eq!(perm_should, perm_is);
}

#[test]
fn test() {
    assert_eq!(circular_count(100), 13);
    assert_eq!(circular_count(1000000), 55);
}
