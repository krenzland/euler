pub fn digits(x: u32) -> Vec<u32> {
    let mut digits: Vec<u32> = Vec::new();
    let mut r = x;
    while r > 0 {
        let digit = r % 10;
        r /= 10;
        digits.push(digit);
    }
    digits
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

#[test]
fn test_divisors() {
    assert_eq!(divisors(28), vec![1, 2, 4, 7, 14]);
}
