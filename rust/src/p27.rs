use ntheory::primes;
use bit_vec::BitVec;

fn max_prime(a: i32, b: i32, primes: &BitVec) -> u32 {
    let mut n = 0;
    loop {
        let eval = n * n + a * n + b;
        if eval < 0 || !primes[eval as usize] {
            return n as u32;
        }
        n += 1;
    }
}

fn best_prime_coeff() -> (i32, i32) {
    let maximum = 100000;
    let primes = primes(maximum);

    let mut best_a = 0;
    let mut best_b = 0;
    let mut max_n = 0;

    // For n=0 the quadratic results in b => b has to be prime.
    let possible_bs: Vec<i32> = (3..1001).filter(|&x| primes[(x as i32).abs() as usize]).collect();
    for a in (-999)..1000 {
        for &b in &possible_bs {
            let cur_n = max_prime(a, b, &primes);
            if max_n < cur_n {
                best_a = a;
                best_b = b;
                max_n = cur_n;
            }
        }
    }
    println!("{}", possible_bs.len());
    (best_a, best_b)
}

pub fn main() {
    let (a, b) = best_prime_coeff();
    println!("{}", a * b);
}

#[test]
fn test() {
    let (a, b) = best_prime_coeff();
    assert_eq!(a * b, -59231);
}

#[test]
fn test_max_prime() {
    let primes = primes(10000);
    assert_eq!(max_prime(1, 41, &primes), 40);
    assert_eq!(max_prime(-79, 1601, &primes), 80);
}
