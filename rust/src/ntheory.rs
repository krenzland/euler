use bit_vec::BitVec;

pub fn digits_base(x: u64, base: u64) -> Vec<u64> {
    let mut digits: Vec<u64> = Vec::new();
    let mut r = x;
    while r > 0 {
        let digit = r % base;
        r /= base;
        digits.push(digit);
    }
    digits
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
