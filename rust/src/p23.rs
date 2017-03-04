use ntheory::divisors;

fn is_abundant(num: u32) -> bool {
    let div = divisors(num);
    let div_sum: u32 = div.iter().sum();
    div_sum > num
}

fn is_abundant_sum(num: u32, abundant: &[u32]) -> bool {
    for a in abundant {
        for b in abundant {
            if a + b == num {
                return true;
            }
        }
    }
    false
}

pub fn main() {
    let mut abundant: Vec<u32> = Vec::new();
    let mut not_writeable = 0;
    let upper_bound = 28123;
    for i in 1..upper_bound + 1 {
        if is_abundant(i) {
            abundant.push(i);
        }
        // Note: A number can be abundant and writeable as abundant-sum at the same time!
        if !is_abundant_sum(i, &abundant) {
           not_writeable += i; 
        }
    }
    println!("{}", not_writeable);
}

#[test]
fn test_is_abundant() {
    assert!(is_abundant(12));
    assert!(!is_abundant(28));
}
