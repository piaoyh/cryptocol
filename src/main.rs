mod number;

use number::*;
use std::mem::size_of;
use number::ULong;

type T = u8;
const N: usize = 12;
const NN: usize = N - 1;

type AI = BigUInt::<T, N>;
type BI = BigUInt::<T, NN>;

fn main()
{
    let mut num = ULonger::new();
    num.ulonger = 0b_0111_1111__1111_1111__1111_1111__1111_1111__1111_1111__1111_1111__1111_1111__1111_1111__1111_1111__1111_1111__1111_1111;
    let num_txt = "0111_1111__1111_1111__1111_1111__1111_1111__1111_1111__1111_1111__1111_1111__1111_1111__1111_1111__1111_1111__1111_1111";
    let a = AI::from_string_with_radix(&num_txt, 2).unwrap();
    let b = BI::from_biguint(&a);
    let c = BI::from_string_with_radix(&num_txt, 2).unwrap();
    unsafe { println!("n = {:?}", num.byte); }
    unsafe { println!("n = {}", num.ulonger); }
    println!("b = {:?}", b);
    unsafe { println!("{} : {} - {}", num.ulonger, num.ulonger / 10, num.ulonger % 10); }
    let (q, r) = b.divide_fully(BI::from_uint(10));
    println!("{} : {} - {}", b, q, r);
    let (q, r) = a.divide_fully(AI::from_uint(10));
    println!("{} : {} - {}", a, q, r);
}
