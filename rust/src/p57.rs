use num::rational::BigRational;
use num::bigint::BigInt;
use num::FromPrimitive;

fn square_root_convergents(num_conv: u32) -> u32 {
    let mut convergent = BigRational::new(BigInt::from_u32(1).unwrap(), BigInt::from_u32(1).unwrap()); 
    let mut num_lt_denom = 0;
    for _ in 0..num_conv {
        let num = convergent.numer() + BigInt::from_u32(2).unwrap() * convergent.denom();
        let denom = convergent.numer() + convergent.denom();
        let ndig_num = num.to_str_radix(10).len();
        let ndig_denom = denom.to_str_radix(10).len();
        convergent = BigRational::new(num, denom);
        if ndig_num > ndig_denom {
           num_lt_denom += 1;
        }
    }
    num_lt_denom
}

pub fn main() {
    println!("{}", square_root_convergents(1000));
}

#[test]
fn test() {
    assert_eq!(square_root_convergents(8),1);
    assert_eq!(square_root_convergents(1000),153);
}
