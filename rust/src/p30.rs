use ntheory::digits;

fn curious_sum(upper_bound: u64) -> u64 {
    let powers: Vec<u64> = (0..10).map(|x| x * x * x * x * x).collect();

    (2..upper_bound)
        .filter(|&num| {
            let digs = digits(num);
            let fac_sum = digs.iter().map(|&x| powers[x as usize]).sum();
            num == fac_sum
        })
        .sum()
}

pub fn main() {
    // Simple upper bound: 10^n > n*9^5
    // => n > 5.5
    println!("{}", curious_sum(10e6 as u64));
}

#[test]
fn test() {
    assert_eq!(curious_sum(10e6 as u64), 443839);
}
