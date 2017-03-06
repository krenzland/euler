// https://projecteuler.net/problem=65

use num::bigint::BigInt;
use num::FromPrimitive;
use ntheory::digit_sum_str;

fn sum_of_nth_nominator(num_conv: usize) -> u32 {
    let even = (1..num_conv / 2).map(|i| i * 2);
    let mut series: Vec<isize> = vec![1];
    for e in even {
        let e = e as isize;
        series.extend(&[e, 1, 1]);
    }
    series.resize(num_conv - 1, 0);

    // The nominators follow this formula:
    // h_0 = a_0
    // h_1 = a_1 * a_0 + 1
    // h_n = a_n * h_{n_1} + h_{n-2}
    let mut nominators = vec![BigInt::from_isize(2).unwrap(),
                              BigInt::from_isize(2 * series[0] + 1).unwrap()];
    for (i, &coeff) in series.iter().skip(1).enumerate() {
        let i = i + 2;
        let coeff = BigInt::from_isize(coeff).unwrap();
        let nom = coeff * nominators[i - 1].clone() + nominators[i - 2].clone();
        nominators.push(nom);
    }
    let digits = nominators[num_conv - 1].to_str_radix(10);
    digit_sum_str(&digits)
}

pub fn main() {
    let num_conv = 100;
    let sum = sum_of_nth_nominator(num_conv);
    println!("{}", sum);
}


#[test]
fn test() {
    assert_eq!(sum_of_nth_nominator(10), 17);
    assert_eq!(sum_of_nth_nominator(100), 272);

}
