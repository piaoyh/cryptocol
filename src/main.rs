mod number;

use number::*;

type T = u16;
const N: usize = 16;
type BI = BigUInt::<T, N>;
type AI = BigUInt::<usize, N>;

fn main()
{
    let a = BI::from_string_with_radix("123456789", 10).unwrap();
    let b = BI::from_string_with_radix("2", 10).unwrap();
    let c = a / b;
    println!("a = {}", a);
    println!("b = {}", b);
    println!("c = {}", c);
    println!("a << 1 = {}",  a << 1);
}
