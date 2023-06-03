mod number;

use number::*;
use std::mem::size_of;

type T = u16;
const N: usize = 16;
const M: usize = size_of::<T>() * N;
type BI = BigUInt::<T, N>;
type AI = BigUInt::<u8, M>;

fn main()
{
    let a = AI::from_string("123456789123456789123456789123456789123456789123456789").unwrap();
    let b = BI::from_biguint(&a);
    println!("a = {}", a);
    println!("b = {}", b);
}
