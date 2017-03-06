use ntheory::digits_base;

fn is_palindrome<T: Eq>(list: &[T]) -> bool {
    for (a, b) in list.iter().rev().zip(list) {
        if a != b {
            return false;
        }
    }
    true
}

fn is_double_palindrome(x: u64) -> bool {
    let digits_10 = digits_base(x, 10);
    let digits_2 = digits_base(x, 2);
    is_palindrome(&digits_10) && is_palindrome(&digits_2)
}

fn palindromic_sum(max: u64) -> u64 {
    (1..(max + 1)).filter(|&x| is_double_palindrome(x)).sum()
}

pub fn main() {
    println!("{}", palindromic_sum(1000000));
}

#[test]
fn test_is_double_palindrome() {
    assert!(is_double_palindrome(585));
    assert!(!is_double_palindrome(586));
    assert!(!is_double_palindrome(868));
}

#[test]
fn test() {
    assert_eq!(palindromic_sum(1000000), 872187);
}
