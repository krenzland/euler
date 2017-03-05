use ntheory::digits;

fn curious_sum(upper_bound: u64) -> u64 {
    let mut facts: Vec<u64> = vec![1];
    let mut fact = 1;
    for i in 1..10 {
        fact *= i;
        facts.push(fact);
    }
    let facts = facts;

    let sum = (3..upper_bound)
        .filter(|&num| {
            let digs = digits(num);
            let fac_sum = digs.iter().map(|&x| facts[x as usize]).sum();
            num == fac_sum
        })
        .sum();
    sum
}

pub fn main() {
    println!("{}", curious_sum(10e4 as u64));
}

#[test]
fn test() {
    assert_eq!(curious_sum(10e2 as u64), 145);
    assert_eq!(curious_sum(10e3 as u64), 145);
    assert_eq!(curious_sum(10e4 as u64), 40730);
}
