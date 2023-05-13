mod cryptocol;

use cryptocol::big_uint::*;
use cryptocol::primitive::*;

fn main()
{
    type BI = BigUInt::<u8, 16>;
    println!("_ = {}\n0 = {}\nA = {}\na = {}", '_' as u32, '0' as u32, 'A' as u32, 'a' as u32);
    let a = BI::from_string("1111111111111111").unwrap();
    let b = BI::from_string("2").unwrap();
    let c = a << 1;
    println!("{} = {}", c.to_string_with_radix(2), c.to_string());
}
