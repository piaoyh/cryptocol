use std::time::SystemTime;
use std::fmt::{ Display, Debug };
use std::ops::*;
use std::convert::*;
use std::mem::size_of;

mod number;
use std::str::FromStr;
use Cryptocol::number::{Uint, UShort, BigUInt, ULonger};
use rand_distr::num_traits::PrimInt; //{u256, BigInteger, HugeInteger};

fn main()
{
    //    57896044618658097711785492504343953926307055644800578124155540853313808954190 == (170141183460469231731687303715884105727_u128, 12345678901234567890123456789012345678_u128)
    // - 308778904632843187796189293356501087608549893209439890708590319850715068122315 == (226854911280625642308916404954512140970_u128, 56789012345678912345678901234567890123_u128)
    //--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
    //   19298681539552699237261830834781317975046994857318776714373108680289488156697 == ( 56713727820156410577229101238628035241_u128, 69134691246913480235802358023580235801_u128)

    define_utypes_with_u128!();
    let a = u256::from_array(&[12345678901234567890123456789012345678_u128, 170141183460469231731687303715884105727_u128]);
    let b = u256::from_array(&[56789012345678912345678901234567890123_u128, 226854911280625642308916404954512140970_u128]);
    let c = a - b;
    println!("a = {}, {:?}", a, a);
    println!("b = {}, {:?}", b, b);
    println!("c = {}, {:?}", c, c);

    UInt_carrying_add___main();
    UInt_wrapping_add___main();
    UInt_overflowing_add___main();
    UInt_checked_add___main();
    UInt_unchecked_add___main();
    UInt_saturating_add___main();
    UInt_borrowing_sub___main();
    /*
    BigUInt_wrapping_add___main();
    BigUInt_wrapping_add_assign___main();
    */
}


/***** UInt *****/

fn UInt_carrying_add___main()
{
    // a_u16: u16 === (a_high_u8, a_low_u8) == (100_u8, 101_u8) == 25701_u16
    let a_high_u8 = 100_u8;
    let a_low_u8 = 101_u8;
    // b_u16: u16 === (b_high_u8, b_low_u8) == (100_u8, 200_u8) == 25800_u16
    let b_high_u8 = 100_u8;
    let b_low_u8 = 200_u8;
    // c: u16 === (c_high, c_low)
    let c_high_u8: u8;
    let c_low_u8: u8;
    let mut carry: bool;
    // (100_u8, 101_u8) + (100_u8, 200_u8) == 25701_u16 + 25800_u16 == 51501_u16
    //   25701_u16 == (100_u8, 101_u8)
    // + 25800_u16 == (100_u8, 200_u8)
    // -------------------------------
    //   51501_u16 == (201_u8,  45_u8)
    (c_high_u8, c_low_u8, carry) = UInt_carrying_add___func(a_high_u8, a_low_u8, b_high_u8, b_low_u8);
    println!("{}-{}, {}", c_high_u8, c_low_u8, carry);
    assert_eq!(c_high_u8, 201);
    assert_eq!(c_low_u8, 45);
    assert_eq!(carry, false);

    let d_high_u8: u8;
    let d_low_u8: u8;
    //  (201_u8,  45_u8) + (100_u8, 200_u8) == 25701_u16 + 25800_u16 == 51501_u16
    //   25701_u16 == (100_u8, 101_u8)
    // + 25800_u16 == (100_u8, 200_u8)
    // -------------------------------
    //   11765_u16 == ( 45_u8, 245_u8)
    (d_high_u8, d_low_u8, carry) = UInt_carrying_add___func(c_high_u8, c_low_u8, b_high_u8, b_low_u8);
    println!("{}-{}, {}", d_high_u8, d_low_u8, carry);
    assert_eq!(d_high_u8, 45_u8);
    assert_eq!(d_low_u8, 245_u8);
    assert_eq!(carry, true);

    let a_high_u128: u128;
    let a_low_u128: u128;
    //  4201016837757989640311993609423984479246482890531986660185 == (12345678901234567890_u128, 6789012345678912345_u128)
    //+                 419908440780438063913804265570801972943493 == (                1234_u128,                6789_u128)
    //---------------------------------------------------------------------------------------------------------------------
    //  4201016837757990060220434389862048393050748461333959603678 == (12345678901234569124_u128, 6789012345678919134_u128)
    let (a_high_u128, a_low_u128, carry) = UInt_carrying_add___func(12345678901234567890_u128, 6789012345678912345_u128, 1234_u128, 6789_u128);
    println!("{}-{}, {}", a_high_u128, a_low_u128, carry);
    assert_eq!(a_high_u128, 12345678901234569124_u128);
    assert_eq!(a_low_u128, 6789012345678919134_u128);
    assert_eq!(carry, false);

    let b_high_u128: u128;
    let b_low_u128: u128;
    //  308778904632843187796189293356501087608549893209439890708590319850715068122315 == (226854911280625642308916404954512140970_u128, 56789012345678912345678901234567890123_u128)
    //+  57896044618658097711785492504343953926307055644800578124155540853313808954190 == (170141183460469231731687303715884105727_u128, 12345678901234567890123456789012345678_u128)
    //--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
    //   19298681539552699237261830834781317975046994857318776714373108680289488156697 == ( 56713727820156410577229101238628035241_u128, 69134691246913480235802358023580235801_u128)
    let (b_high_u128, b_low_u128, carry) = UInt_carrying_add___func(226854911280625642308916404954512140970_u128, 56789012345678912345678901234567890123_u128, 170141183460469231731687303715884105727_u128, 12345678901234567890123456789012345678_u128);
    println!("{}-{}, {}", b_high_u128, b_low_u128, carry);
    assert_eq!(b_high_u128, 56713727820156410577229101238628035241_u128);
    assert_eq!(b_low_u128, 69134691246913480235802358023580235801_u128);
    assert_eq!(carry, true);
}

fn UInt_carrying_add___func<T: Uint>(lhs_high: T, lhs_low: T, rhs_high: T, rhs_low: T) -> (T, T, bool)
{
    let mut carry = false;
    let mut sum_high: T;
    let mut sum_low: T;
    (sum_low, carry) = lhs_low.carrying_add(rhs_low, carry);
    (sum_high, carry) = lhs_high.carrying_add(rhs_high, carry);
    (sum_high, sum_low, carry)
}


fn UInt_wrapping_add___main()
{
    let a_u8 = UInt_wrapping_add___func(u8::MAX - 55_u8, 55_u8);
    println!("{} + 55 = {}", u8::MAX - 55_u8, a_u8);
    assert_eq!(a_u8, u8::MAX);

    let b_u8 = UInt_wrapping_add___func(a_u8, 1_u8);
    println!("{} + 1 = {}", a_u8, b_u8);
    assert_eq!(b_u8, 0_u8);

    let a_u16 = UInt_wrapping_add___func(u16::MAX - 55_u16, 55_u16);
    println!("{} + 55 = {}", u16::MAX - 55_u16, a_u16);
    assert_eq!(a_u16, u16::MAX);

    let b_u16 = UInt_wrapping_add___func(a_u16, 1_u16);
    println!("{} + 1 = {}", a_u16, b_u16);
    assert_eq!(b_u16, 0_u16);

    let a_u32 = UInt_wrapping_add___func(u32::MAX - 55_u32, 55_u32);
    println!("{} + 55 = {}", u32::MAX - 55_u32, a_u32);
    assert_eq!(a_u32, u32::MAX);

    let b_u32 = UInt_wrapping_add___func(a_u32, 1_u32);
    println!("{} + 1 = {}", a_u32, b_u32);
    assert_eq!(b_u32, 0_u32);

    let a_u64 = UInt_wrapping_add___func(u64::MAX - 55_u64, 55_u64);
    println!("{} + 55 = {}", u64::MAX - 55_u64, a_u64);
    assert_eq!(a_u64, u64::MAX);

    let b_u64 = UInt_wrapping_add___func(a_u64, 1_u64);
    println!("{} + 1 = {}", a_u64, b_u64);
    assert_eq!(b_u64, 0_u64);

    let a_u128 = UInt_wrapping_add___func(u128::MAX - 55_u128, 55_u128);
    println!("{} + 55 = {}", u128::MAX - 55_u128, a_u128);
    assert_eq!(a_u128, u128::MAX);

    let b_u128 = UInt_wrapping_add___func(a_u128, 1_u128);
    println!("{} + 1 = {}",a_u128, b_u128);
    assert_eq!(b_u128, 0_u128);

    let a_usize = UInt_wrapping_add___func(usize::MAX - 55_usize, 55_usize);
    println!("{} + 55 = {}", usize::MAX - 55_usize, a_usize);
    assert_eq!(a_usize, usize::MAX);

    let b_usize = UInt_wrapping_add___func(a_usize, 1_usize);
    println!("{} + 1 = {}", a_usize, b_usize);
    assert_eq!(b_usize, 0_usize);
}

fn UInt_wrapping_add___func<T: Uint>(lhs: T, rhs: T) -> T
{
    lhs.wrapping_add(rhs)
}

fn UInt_overflowing_add___main()
{
    let a_u8 = UInt_overflowing_add___func(u8::MAX - 55_u8, 55_u8);
    println!("{} + 55 = {}\nOverflow = {}", u8::MAX - 55_u8, a_u8.0, a_u8.1);
    assert_eq!(a_u8.0, u8::MAX);
    assert_eq!(a_u8.1, false);
 
    let b_u8 = UInt_overflowing_add___func(a_u8.0, 1_u8);
    println!("{} + 1 = {}\nOverflow = {}", a_u8.0, b_u8.0, b_u8.1);
    assert_eq!(b_u8.0, 0_u8);
    assert_eq!(b_u8.1, true);

    let a_u16 = UInt_overflowing_add___func(u16::MAX - 55_u16, 55_u16);
    println!("{} + 55 = {}\nOverflow = {}", u16::MAX - 55_u16, a_u16.0, a_u16.1);
    assert_eq!(a_u16.0, u16::MAX);
    assert_eq!(a_u16.1, false);
 
    let b_u16 = UInt_overflowing_add___func(a_u16.0, 1_u16);
    println!("{} + 1 = {}\nOverflow = {}", a_u16.0, b_u16.0, b_u16.1);
    assert_eq!(b_u16.0, 0_u16);
    assert_eq!(b_u16.1, true);

    let a_u32 = UInt_overflowing_add___func(u32::MAX - 55_u32, 55_u32);
    println!("{} + 55 = {}\nOverflow = {}", u32::MAX - 55_u32, a_u32.0, a_u32.1);
    assert_eq!(a_u32.0, u32::MAX);
    assert_eq!(a_u32.1, false);
 
    let b_u32 = UInt_overflowing_add___func(a_u32.0, 1_u32);
    println!("{} + 1 = {}\nOverflow = {}", a_u32.0, b_u32.0, b_u32.1);
    assert_eq!(b_u32.0, 0_u32);
    assert_eq!(b_u32.1, true);

    let a_u64 = UInt_overflowing_add___func(u64::MAX - 55_u64, 55_u64);
    println!("{} + 55 = {}\nOverflow = {}", u64::MAX - 55_u64, a_u64.0, a_u64.1);
    assert_eq!(a_u64.0, u64::MAX);
    assert_eq!(a_u64.1, false);
 
    let b_u64 = UInt_overflowing_add___func(a_u64.0, 1_u64);
    println!("{} + 1 = {}\nOverflow = {}", a_u64.0, b_u64.0, b_u64.1);
    assert_eq!(b_u64.0, 0_u64);
    assert_eq!(b_u64.1, true);

    let a_u128 = UInt_overflowing_add___func(u128::MAX - 55_u128, 55_u128);
    println!("{} + 55 = {}\nOverflow = {}", u128::MAX - 55_u128, a_u128.0, a_u128.1);
    assert_eq!(a_u128.0, u128::MAX);
    assert_eq!(a_u128.1, false);
 
    let b_u128 = UInt_overflowing_add___func(a_u128.0, 1_u128);
    println!("{} + 1 = {}\nOverflow = {}", a_u128.0, b_u128.0, b_u128.1);
    assert_eq!(b_u128.0, 0_u128);
    assert_eq!(b_u128.1, true);

    let a_usize = UInt_overflowing_add___func(usize::MAX - 55_usize, 55_usize);
    println!("{} + 55 = {}\nOverflow = {}", usize::MAX - 55_usize, a_usize.0, a_usize.1);
    assert_eq!(a_usize.0, usize::MAX);
    assert_eq!(a_usize.1, false);
 
    let b_usize = UInt_overflowing_add___func(a_usize.0, 1_usize);
    println!("{} + 1 = {}\nOverflow = {}", a_usize.0, b_usize.0, b_usize.1);
    assert_eq!(b_usize.0, 0_usize);
    assert_eq!(b_usize.1, true);
}

fn UInt_overflowing_add___func<T: Uint>(lhs: T, rhs: T) -> (T, bool)
{
    lhs.overflowing_add(rhs)
}

fn UInt_checked_add___main()
{
    let a_u8 = UInt_checked_add___func(u8::MAX - 55_u8, 55_u8);
    match a_u8
    {
        Some(a) => {
                println!("{} + 55 = {}", u8::MAX - 55_u8, a);
                assert_eq!(a, u8::MAX);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(a_u8, None);
            },
    }
 
    let b_u8 = UInt_checked_add___func(a_u8.unwrap(), 1_u8);
    match b_u8
    {
        Some(b) => {
                println!("{} + 1 = {}", a_u8.unwrap(), b);
                assert_eq!(b, 0_u8);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(b_u8, None);
            },
    }

    let a_u16 = UInt_checked_add___func(u16::MAX - 55_u16, 55_u16);
    match a_u16
    {
        Some(a) => {
                println!("{} + 55 = {}", u16::MAX - 55_u16, a);
                assert_eq!(a, u16::MAX);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(a_u16, None);
            },
    }

    let b_u16 = UInt_checked_add___func(a_u16.unwrap(), 1_u16);
    match b_u16
    {
        Some(b) => {
                println!("{} + 1 = {}", a_u16.unwrap(), b);
                assert_eq!(b, 0_u16);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(b_u16, None);
            },
    }

    let a_u32 = UInt_checked_add___func(u32::MAX - 55_u32, 55_u32);
    match a_u32
    {
        Some(a) => {
                println!("{} + 55 = {}", u32::MAX - 55_u32, a);
                assert_eq!(a, u32::MAX);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(a_u32, None);
            },
    }

    let b_u32 = UInt_checked_add___func(a_u32.unwrap(), 1_u32);
    match b_u32
    {
        Some(b) => {
                println!("{} + 1 = {}", a_u32.unwrap(), b);
                assert_eq!(b, 0_u32);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(b_u32, None);
            },
    }

    let a_u64 = UInt_checked_add___func(u64::MAX - 55_u64, 55_u64);
    match a_u64
    {
        Some(a) => {
                println!("{} + 55 = {}", u64::MAX - 55_u64, a);
                assert_eq!(a, u64::MAX);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(a_u64, None);
            },
    }

    let b_u64 = UInt_checked_add___func(a_u64.unwrap(), 1_u64);
    match b_u64
    {
        Some(b) => {
                println!("{} + 1 = {}", a_u64.unwrap(), b);
                assert_eq!(b, 0_u64);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(b_u64, None);
            },
    }

    let a_u128 = UInt_checked_add___func(u128::MAX - 55_u128, 55_u128);
    match a_u128
    {
        Some(a) => {
                println!("{} + 55 = {}", u128::MAX - 55_u128, a);
                assert_eq!(a, u128::MAX);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(a_u128, None);
            },
    }

    let b_u128 = UInt_checked_add___func(a_u128.unwrap(), 1_u128);
    match b_u128
    {
        Some(b) => {
                println!("{} + 1 = {}", a_u128.unwrap(), b);
                assert_eq!(b, 0_u128);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(b_u128, None);
            },
    }

    let a_usize = UInt_checked_add___func(usize::MAX - 55_usize, 55_usize);
    match a_usize
    {
        Some(a) => {
                println!("{} + 55 = {}", usize::MAX - 55_usize, a);
                assert_eq!(a, usize::MAX);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(a_usize, None);
            },
    }

    let b_usize = UInt_checked_add___func(a_usize.unwrap(), 1_usize);
    match b_usize
    {
        Some(b) => {
                println!("{} + 1 = {}", a_usize.unwrap(), b);
                assert_eq!(b, 0_usize);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(b_usize, None);
            },
    }
}

fn UInt_checked_add___func<T: Uint>(lhs: T, rhs: T) -> Option<T>
{
    lhs.checked_add(rhs)
}

fn UInt_unchecked_add___main()
{
    let a_u8 = UInt_uchecked_add___func(u8::MAX - 55_u8, 55_u8);
    println!("{} + 55 = {}", u8::MAX - 55_u8, a_u8);
    assert_eq!(a_u8, u8::MAX);

    // It will panic
    // let b_u8 = UInt_uchecked_add___func(a_u8, 1_u8);

    let a_u16 = UInt_uchecked_add___func(u16::MAX - 55_u16, 55_u16);
    println!("{} + 55 = {}", u16::MAX - 55_u16, a_u16);
    assert_eq!(a_u16, u16::MAX);

    // It will panic
    // let b_u16 = UInt_uchecked_add___func(a_u16, 1_u16);

    let a_u32 = UInt_uchecked_add___func(u32::MAX - 55_u32, 55_u32);
    println!("{} + 55 = {}", u32::MAX - 55_u32, a_u32);
    assert_eq!(a_u32, u32::MAX);

    // It will panic
    // let b_u32 = UInt_uchecked_add___func(a_u32, 1_u32);

    let a_u64 = UInt_uchecked_add___func(u64::MAX - 55_u64, 55_u64);
    println!("{} + 55 = {}", u64::MAX - 55_u64, a_u64);
    assert_eq!(a_u64, u64::MAX);

    // It will panic
    // let b_u64 = UInt_uchecked_add___func(a_u64, 1_u64);

    let a_u128 = UInt_uchecked_add___func(u128::MAX - 55_u128, 55_u128);
    println!("{} + 55 = {}", u128::MAX - 55_u128, a_u128);
    assert_eq!(a_u128, u128::MAX);

    // It will panic
    // let b_u128 = UInt_uchecked_add___func(a_u128, 1_u128);

    let a_usize = UInt_uchecked_add___func(usize::MAX - 55_usize, 55_usize);
    println!("{} + 55 = {}", usize::MAX - 55_usize, a_usize);
    assert_eq!(a_usize, usize::MAX);

    // It will panic
    // let b_usize = UInt_uchecked_add___func(a_usize, 1_usize);
}

fn UInt_uchecked_add___func<T: Uint>(lhs: T, rhs: T) -> T
{
    lhs.unchecked_add(rhs)
}

fn UInt_saturating_add___main()
{
    let a_u8 = UInt_saturating_add___func(u8::MAX - 55_u8, 55_u8);
    println!("{} + 55 = {}", u8::MAX - 55_u8, a_u8);
    assert_eq!(a_u8, u8::MAX);

    let b_u8 = UInt_saturating_add___func(a_u8, 55_u8);
    println!("{} + 55 = {}", a_u8, b_u8);
    assert_eq!(b_u8, u8::MAX);

    let a_u16 = UInt_saturating_add___func(u16::MAX - 55_u16, 55_u16);
    println!("{} + 55 = {}", u16::MAX - 55_u16, a_u16);
    assert_eq!(a_u16, u16::MAX);

    let b_u16 = UInt_saturating_add___func(a_u16, 55_u16);
    println!("{} + 55 = {}", a_u16, b_u16);
    assert_eq!(b_u16, u16::MAX);

    let a_u32 = UInt_saturating_add___func(u32::MAX - 55_u32, 55_u32);
    println!("{} + 55 = {}", u32::MAX - 55_u32, a_u32);
    assert_eq!(a_u32, u32::MAX);

    let b_u32 = UInt_saturating_add___func(a_u32, 55_u32);
    println!("{} + 55 = {}", a_u32, b_u32);
    assert_eq!(b_u32, u32::MAX);

    let a_u64 = UInt_saturating_add___func(u64::MAX - 55_u64, 55_u64);
    println!("{} + 55 = {}", u64::MAX - 55_u64, a_u64);
    assert_eq!(a_u64, u64::MAX);

    let b_u64 = UInt_saturating_add___func(a_u64, 55_u64);
    println!("{} + 55 = {}", a_u64, b_u64);
    assert_eq!(b_u64, u64::MAX);

    let a_u128 = UInt_saturating_add___func(u128::MAX - 55_u128, 55_u128);
    println!("{} + 55 = {}", u128::MAX - 55_u128, a_u128);
    assert_eq!(a_u128, u128::MAX);

    let b_u128 = UInt_saturating_add___func(a_u128, 55_u128);
    println!("{} + 55 = {}",a_u128, b_u128);
    assert_eq!(b_u128, u128::MAX);

    let a_usize = UInt_saturating_add___func(usize::MAX - 55_usize, 55_usize);
    println!("{} + 55 = {}", usize::MAX - 55_usize, a_usize);
    assert_eq!(a_usize, usize::MAX);

    let b_usize = UInt_saturating_add___func(a_usize, 55_usize);
    println!("{} + 55 = {}", a_usize, b_usize);
    assert_eq!(b_usize, usize::MAX);
}

fn UInt_saturating_add___func<T: Uint>(lhs: T, rhs: T) -> T
{
    lhs.saturating_add(rhs)
}

fn UInt_borrowing_sub___main()
{
    // a_u16: u16 === (a_high_u8, a_low_u8) == (100_u8, 200_u8) == 25800_u16
    let a_high_u8 = 100_u8;
    let a_low_u8 = 200_u8;
    // b_u16: u16 === (b_high_u8, b_low_u8) == (100_u8, 101_u8) == 25701_u16
    let b_high_u8 = 100_u8;
    let b_low_u8 = 101_u8;
    // c_u16: u16 === (c_high_u8, c_low_u8)
    let c_high_u8: u8;
    let c_low_u8: u8;
    let mut borrow: bool;
    // (100_u8, 200_u8) - (100_u8, 101_u8) == 25800_u16 - 25701_u16 == 99_u16
    //   25800_u16 == (100_u8, 200_u8)
    // - 25701_u16 == (100_u8, 101_u8)
    // -------------------------------
    //      99_u16 == (  0_u8,  99_u8)
    (c_high_u8, c_low_u8, borrow) = UInt_borrowing_sub___func(a_high_u8, a_low_u8, b_high_u8, b_low_u8);
    println!("{}-{}, {}", c_high_u8, c_low_u8, borrow);
    assert_eq!(c_high_u8, 0_u8);
    assert_eq!(c_low_u8, 99_u8);
    assert_eq!(borrow, false);

    let d_high_u8: u8;
    let d_low_u8: u8;
    //  (  0_u8,  99_u8) - (100_u8, 101_u8) == 99_u16 - 25701_u16 == 51501_u16
    //      99_u16 == (  0_u8,  99_u8)
    // - 25701_u16 == (100_u8, 101_u8)
    // -------------------------------
    //   39934_u16 == (155_u8, 254_u8)
    (d_high_u8, d_low_u8, borrow) = UInt_borrowing_sub___func(c_high_u8, c_low_u8, b_high_u8, b_low_u8);
    println!("{}-{}, {}", d_high_u8, d_low_u8, borrow);
    assert_eq!(d_high_u8, 155_u8);
    assert_eq!(d_low_u8, 254_u8);
    assert_eq!(borrow, true);

    let a_high_u128: u128;
    let a_low_u128: u128;
    //   4201016837757989640311993609423984479246482890531986660185 == (12345678901234567890_u128, 6789012345678912345_u128)
    // -                 419908440780438063913804265570801972943493 == (                1234_u128,                6789_u128)
    // ---------------------------------------------------------------------------------------------------------------------
    //   4201016837757989220403552828985920565442217319730013716692 == (12345678901234566656_u128, 6789012345678905556_u128)
    (a_high_u128, a_low_u128, borrow) = UInt_borrowing_sub___func(12345678901234567890_u128, 6789012345678912345_u128, 1234_u128, 6789_u128);
    println!("{}-{}, {}", a_high_u128, a_low_u128, borrow);
    assert_eq!(a_high_u128, 12345678901234566656_u128);
    assert_eq!(a_low_u128, 6789012345678905556_u128);
    assert_eq!(borrow, false);

    let b_high_u128: u128;
    let b_low_u128: u128;
    //    57896044618658097711785492504343953926307055644800578124155540853313808954190 == (170141183460469231731687303715884105727_u128,  12345678901234567890123456789012345678_u128)
    // - 308778904632843187796189293356501087608549893209439890708590319850715068122315 == (226854911280625642308916404954512140970_u128,  56789012345678912345678901234567890123_u128)
    // --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
    //   328077586172395887033451124191282405584107085763563507612853141042164389031555 == (283568639100782052886145506193140176212_u128, 295839033476494119007819162986212667011_u128)
    (b_high_u128, b_low_u128, borrow) = UInt_borrowing_sub___func(170141183460469231731687303715884105727_u128, 12345678901234567890123456789012345678_u128, 226854911280625642308916404954512140970_u128, 56789012345678912345678901234567890123_u128);
    println!("{}-{}, {}", b_high_u128, b_low_u128, borrow);
    assert_eq!(b_high_u128, 283568639100782052886145506193140176212_u128);
    assert_eq!(b_low_u128, 295839033476494119007819162986212667011_u128);
    assert_eq!(borrow, true);
}

fn UInt_borrowing_sub___func<T: Uint>(lhs_high: T, lhs_low: T, rhs_high: T, rhs_low: T) -> (T, T, bool)
{
    let mut borrow = false;
    let mut sum_high: T;
    let mut sum_low: T;
    (sum_low, borrow) = lhs_low.borrowing_sub(rhs_low, borrow);
    (sum_high, borrow) = lhs_high.borrowing_sub(rhs_high, borrow);
    (sum_high, sum_low, borrow)
}



/***** BigUInt *****/

fn BigUInt_wrapping_add___main()
{
    define_utypes_with!(u128);

    let a = u512::max() - u512::from(1_u128);

    println!("{} + 1 = {}", a, a.wrapping_add(u512::from(1_u128)));
    assert_eq!(a.wrapping_add(u512::from(1_u128)), u512::max());

    println!("{} + 2 = {}", a, a.wrapping_add(u512::from(2_u128)));
    assert_eq!(a.wrapping_add(u512::from(2_u128)), u512::zero());

    println!("{} + 3 = {}", a, a.wrapping_add(u512::from(3_u128)));
    assert_eq!(a.wrapping_add(u512::from(3_u128)), u512::one());
}

fn BigUInt_wrapping_add_assign___main()
{
    define_utypes_with!(u128);
    let mut a = u256::max() - u256::from(1_u128);
    println!("Originally,\ta = {}", a);

    a.wrapping_add_assign(u256::from(1_u128));
    println!("After a += 1,\ta = {}", a);
    assert_eq!(a, u256::from_str("347376267711948586270712955026063723559809953996921692118372752023739388919807").unwrap());

    a.wrapping_add_assign(u256::from(1_u128));
    println!("After a += 1,\ta = {}", a);
    assert_eq!(a, u256::zero());

    a.wrapping_add_assign(u256::from(1_u128));
    println!("After a += 1,\ta = {}", a);
    assert_eq!(a, u256::one());
}




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


/*
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