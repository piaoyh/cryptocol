mod number;

use number::*;
use std::mem::size_of;
use number::ULong;
//use number::big_uint::u1024;

type T = u8;
const N: usize = 12;
const NN: usize = N - 1;

type AI = BigUInt::<T, N>;
type BI = BigUInt::<T, NN>;

fn main()
{
    let a = u1024::new();
}
