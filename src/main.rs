#![allow(missing_docs)]
#![allow(missing_doc_code_examples)]

mod tests;
use tests::*;

fn main()
{
   test_main_BigUInt();
//    test_main_Unions();
    // test_main_SmallUInt();
//     Test();
//     find_maximum();
}

/*
fn t_1024()
{
    define_utypes_with!(u128);
    let a = u1024::random();
    println!("{} 비트짜리 난수: {}", 1024, a);
    let b = u1024::from(1_u128);
    println!("{} 비트짜리 1: {}", 1024, b);
    let c = a + b;
    println!("{} + {} = {}", a, b, c);
}

fn t_2048()
{
    define_utypes_with!(u128);
    let a = u2048::random();
    println!("{} 비트짜리 난수: {}", 2048, a);
    let b = u2048::from(1_u128);
    println!("{} 비트짜리 1: {}", 2048, b);
    let c = a + b;
    println!("{} + {} = {}", a, b, c);
}

fn t_4096()
{
    define_utypes_with!(u128);
    let a = u4096::random();
    println!("{} 비트짜리 난수: {}", 4096, a);
    let b = u4096::from(1_u128);
    println!("{} 비트짜리 1: {}", 4096, b);
    let c = a + b;
    println!("{} + {} = {}", a, b, c);
}



fn func<T: Uint + Add<Output = T>>(lhs: T, rhs: T) -> T
{
    lhs + rhs
}
fn func2<T: Uint>(lhs: T, rhs: T) -> T
{
    lhs.wrapping_add(rhs)
}

fn main()
{



    let a = 100;
    let b = a % -3;
    let c = "123456789012".parse::<u256>().unwrap();
    let e = c.to_string_with_radix_and_stride(10, 4);
    let d: u128 = c.into_u128();
    println!("a = {}, b = {}, c = {}, e = {}", a, b, c, e);
    let a = "123_4566".parse::<u256>().unwrap();
    println!("a = {}", a);
    let ss = UShort { byte: [101, 100] };
    unsafe { println!("ss.short = {}", ss.ushort ); }
    println!("{}", (25700_u16 + 25800_u16));

    // a: u16 === (a_high, a_low) == (100_u8, 101u8) == 25701_u16
    let a_high = 100_u8;
    let a_low = 101_u8;
    // b: u16 === (b_high, b_low) == (100_u8, 200u8) == 51300_u16
    let b_high = 100_u8;
    let b_low = 200_u8;
    // c: u16 === (c_high, c_low)
    let c_high: u8;
    let c_low: u8;
    let mut carry: bool;
    // (100_u8, 101_u8) + (100_u8, 200_u8) == 25701_u16 + 25800_u16 == 51501_u16
    (c_high, c_low, carry) = add_long(a_high, a_low, b_high, b_low);
    println!("{}-{}, {}", c_high, c_low, carry);
    assert_eq!(c_high, 201);
    assert_eq!(c_low, 45);
    assert_eq!(carry, false);

    let d_high: u128;
    let d_low: u128;
    let e = BigUInt::<u128, 2>::from_array(&[6789012345678919134, 12345678901234569124]);
    println!("big = {}", e);
    (d_high, d_low, carry) = add_long(12345678901234567890_u128, 6789012345678912345_u128, 1234_u128, 6789_u128);
    println!("{}-{}, {}", d_high, d_low, carry);
    assert_eq!(d_high, 12345678901234569124);
    assert_eq!(d_low, 6789012345678919134);
    assert_eq!(carry, false);
}

fn add_long<T: Uint>(lhs_high: T, lhs_low: T, rhs_high: T, rhs_low: T) -> (T, T, bool)
{
    let mut carry = false;
    let mut sum_high: T;
    let mut sum_low: T;
    (sum_low, carry) = lhs_low.carrying_add(rhs_low, carry);
    (sum_high, carry) = lhs_high.carrying_add(rhs_high, carry);
    (sum_high, sum_low, carry)
}

fn main()
{
    let a = func(50_u128, 4_u128);
    println!("50 + 4 = {}", a);
    assert_eq!(a, 54_u128);

    let b = func2(u8::MAX, u8::MAX);
    println!("{} * 15_u64 = {}", u128::MAX, b);
    assert_eq!(b, 254_u8);
    
    // u256::new();
    // let a = 100_u8;
    // let b = 100_u8;
    // let c = func(a, b);
    // let d = func(c, 57);
    // println!("a + b = {}", c);
    // println!("c + 57 = {}", d);
    // assert_eq!(c, 200_u8);
    // assert_eq!(d, 1_u8);
    
    let mut a = u256::from_string_with_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let b = u256::from_string_with_radix("11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000", 2).unwrap();
    let d = u256::max();
    let c = !a | a;
    println!("c = {}", c.to_string_with_radix(2));
    assert_eq!(c, u256::max());

    // let mut sum = u1024::new();
    // sum.set_max();
    // println!("sum = {}", sum);

    // let mut a = u256::from_string("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
    // println!("{}", a);
    // a >>= 2;
    // println!("a = {}\n{}", a, a.is_underflow());
    // assert_eq!(a.is_underflow(), true);
}
*/