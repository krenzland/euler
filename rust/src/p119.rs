use ntheory::digits;

// Note: does not work for arbitrarily sized nth without tweaking
// upper_sum and upper_exp!
fn power_sum(nth: usize) -> u64 {
    // Idea: Calculate digit_sum^exp and then check if number sat. condition
    // instead of trying every digit.
    let upper_sum: u64 = 150;
    let upper_exp: u32 = 9;
    let mut result: Vec<u64> = Vec::new();
    for sum in 1..upper_sum {
        for exp in 1..upper_exp {
            let n: u64 = sum.pow(exp);
            if n < 10 {
                continue;
            }
            let digits = digits(n);
            let digit_sum = digits.iter().sum();
            if sum == digit_sum {
                result.push(n);
            }
        }
    }
    result.sort();
    result[nth - 1]
}

pub fn main() {
    println!("{:?}", power_sum(30));
}

#[test]
fn test() {
    assert_eq!(power_sum(2), 512);
    assert_eq!(power_sum(10), 614656);
    assert_eq!(power_sum(30), 248155780267521);
}
