// Copyright 2023 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(missing_docs)]
#![allow(missing_doc_code_examples)]

use std::time::SystemTime;
use std::fmt::{ Display, Debug };
use std::ops::*;
use std::convert::*;
use std::mem::size_of;

use std::str::FromStr;
use Cryptocol::number::*;
use rand_distr::num_traits::PrimInt; //{u256, BigInteger, HugeInteger};


pub fn test_main_UInt()
{
    UInt_carrying_add___main();
    UInt_wrapping_add___main();
    UInt_overflowing_add___main();
    UInt_checked_add___main();
    UInt_unchecked_add___main();
    UInt_saturating_add___main();

    UInt_borrowing_sub___main();
    UInt_wrapping_sub___main();
    UInt_overflowing_sub___main();
    UInt_checked_sub___main();
    UInt_unchecked_sub___main();
    UInt_saturating_sub___main();
    UInt_abs_diff___main();

    UInt_wrapping_mul___main();
    UInt_overflowing_mul___main();
    UInt_checked_mul___main();
    UInt_unchecked_mul___main();
    UInt_saturating_mul___main();

    UInt_wrapping_div___main();
    UInt_overflowing_div___main();
    UInt_checked_div___main();
    UInt_saturating_div___main();

    UInt_wrapping_rem___main();
    UInt_overflowing_rem___main();
    UInt_checked_rem___main();

    UInt_pow___main();
    UInt_wrapping_pow___main();
}

fn UInt_carrying_add___main()
{
    println!("UInt_carrying_add___main()");
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
    println!("UInt_wrapping_add___main()");
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
    println!("--------------------------------------");
}

fn UInt_wrapping_add___func<T: Uint>(lhs: T, rhs: T) -> T
{
    lhs.wrapping_add(rhs)
}

fn UInt_overflowing_add___main()
{

    println!("UInt_overflowing_add___main()");
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
    println!("--------------------------------------");
}

fn UInt_overflowing_add___func<T: Uint>(lhs: T, rhs: T) -> (T, bool)
{
    lhs.overflowing_add(rhs)
}

fn UInt_checked_add___main()
{
    println!("UInt_checked_add___main()");
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
    println!("--------------------------------------");
}

fn UInt_checked_add___func<T: Uint>(lhs: T, rhs: T) -> Option<T>
{
    lhs.checked_add(rhs)
}

fn UInt_unchecked_add___main()
{
    println!("UInt_unchecked_add___main()");
    let a_u8 = UInt_unchecked_add___func(u8::MAX - 55_u8, 55_u8);
    println!("{} + 55 = {}", u8::MAX - 55_u8, a_u8);
    assert_eq!(a_u8, u8::MAX);

    // It will panic
    // let b_u8 = UInt_unchecked_add___func(a_u8, 1_u8);

    let a_u16 = UInt_unchecked_add___func(u16::MAX - 55_u16, 55_u16);
    println!("{} + 55 = {}", u16::MAX - 55_u16, a_u16);
    assert_eq!(a_u16, u16::MAX);

    // It will panic
    // let b_u16 = UInt_unchecked_add___func(a_u16, 1_u16);

    let a_u32 = UInt_unchecked_add___func(u32::MAX - 55_u32, 55_u32);
    println!("{} + 55 = {}", u32::MAX - 55_u32, a_u32);
    assert_eq!(a_u32, u32::MAX);

    // It will panic
    // let b_u32 = UInt_unchecked_add___func(a_u32, 1_u32);

    let a_u64 = UInt_unchecked_add___func(u64::MAX - 55_u64, 55_u64);
    println!("{} + 55 = {}", u64::MAX - 55_u64, a_u64);
    assert_eq!(a_u64, u64::MAX);

    // It will panic
    // let b_u64 = UInt_unchecked_add___func(a_u64, 1_u64);

    let a_u128 = UInt_unchecked_add___func(u128::MAX - 55_u128, 55_u128);
    println!("{} + 55 = {}", u128::MAX - 55_u128, a_u128);
    assert_eq!(a_u128, u128::MAX);

    // It will panic
    // let b_u128 = UInt_unchecked_add___func(a_u128, 1_u128);

    let a_usize = UInt_unchecked_add___func(usize::MAX - 55_usize, 55_usize);
    println!("{} + 55 = {}", usize::MAX - 55_usize, a_usize);
    assert_eq!(a_usize, usize::MAX);

    // It will panic
    // let b_usize = UInt_unchecked_add___func(a_usize, 1_usize);
    println!("--------------------------------------");
}

fn UInt_unchecked_add___func<T: Uint>(lhs: T, rhs: T) -> T
{
    lhs.unchecked_add(rhs)
}

fn UInt_saturating_add___main()
{
    println!("UInt_saturating_add___main()");
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
    println!("--------------------------------------");
}

fn UInt_saturating_add___func<T: Uint>(lhs: T, rhs: T) -> T
{
    lhs.saturating_add(rhs)
}

fn UInt_borrowing_sub___main()
{
    println!("UInt_borrowing_sub___main()");
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
    println!("--------------------------------------");
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

fn UInt_wrapping_sub___main()
{
    println!("UInt_wrapping_sub___main()");
    let a_u8 = UInt_wrapping_sub___func(55_u8, 55_u8);
    println!("55 - 55 = {}", a_u8);
    assert_eq!(a_u8, 0_u8);

    let b_u8 = UInt_wrapping_sub___func(a_u8, 1_u8);
    println!("{} - 1 = {}", a_u8, b_u8);
    assert_eq!(b_u8, u8::MAX);

    let a_u16 = UInt_wrapping_sub___func(55_u16, 55_u16);
    println!("55 - 55 = {}", a_u16);
    assert_eq!(a_u16, 0_u16);

    let b_u16 = UInt_wrapping_sub___func(a_u16, 1_u16);
    println!("{} - 1 = {}", a_u16, b_u16);
    assert_eq!(b_u16, u16::MAX);

    let a_u32 = UInt_wrapping_sub___func(55_u32, 55_u32);
    println!("55 - 55 = {}", a_u32);
    assert_eq!(a_u32, 0_u32);

    let b_u32 = UInt_wrapping_sub___func(a_u32, 1_u32);
    println!("{} - 1 = {}", a_u32, b_u32);
    assert_eq!(b_u32, u32::MAX);

    let a_u64 = UInt_wrapping_sub___func(55_u64, 55_u64);
    println!("55 - 55 = {}", a_u64);
    assert_eq!(a_u64, 0_u64);

    let b_u64 = UInt_wrapping_sub___func(a_u64, 1_u64);
    println!("{} - 1 = {}", a_u64, b_u64);
    assert_eq!(b_u64, u64::MAX);

    let a_u128 = UInt_wrapping_sub___func(55_u128, 55_u128);
    println!("55 - 55 = {}", a_u128);
    assert_eq!(a_u128, 0_u128);

    let b_u128 = UInt_wrapping_sub___func(a_u128, 1_u128);
    println!("{} - 1 = {}",a_u128, b_u128);
    assert_eq!(b_u128, u128::MAX);

    let a_usize = UInt_wrapping_sub___func(55_usize, 55_usize);
    println!("55 - 55 = {}", a_usize);
    assert_eq!(a_usize, 0_usize);

    let b_usize = UInt_wrapping_sub___func(a_usize, 1_usize);
    println!("{} - 1 = {}", a_usize, b_usize);
    assert_eq!(b_usize, usize::MAX);
    println!("--------------------------------------");
}

fn UInt_wrapping_sub___func<T: Uint>(lhs: T, rhs: T) -> T
{
    lhs.wrapping_sub(rhs)
}

fn UInt_overflowing_sub___main()
{
    println!("UInt_overflowing_sub___main()");
    let a_u8 = UInt_overflowing_sub___func(55_u8, 55_u8);
    println!("55 - 55 = {}\nUnderflow = {}", a_u8.0, a_u8.1);
    assert_eq!(a_u8.0, 0_u8);
    assert_eq!(a_u8.1, false);
 
    let b_u8 = UInt_overflowing_sub___func(a_u8.0, 1_u8);
    println!("{} - 1 = {}\nUnderflow = {}", a_u8.0, b_u8.0, b_u8.1);
    assert_eq!(b_u8.0, u8::MAX);
    assert_eq!(b_u8.1, true);

    let a_u16 = UInt_overflowing_sub___func(55_u16, 55_u16);
    println!("55 - 55 = {}\nUnderflow = {}", a_u16.0, a_u16.1);
    assert_eq!(a_u16.0, 0_u16);
    assert_eq!(a_u16.1, false);
 
    let b_u16 = UInt_overflowing_sub___func(a_u16.0, 1_u16);
    println!("{} - 1 = {}\nUnderflow = {}", a_u16.0, b_u16.0, b_u16.1);
    assert_eq!(b_u16.0, u16::MAX);
    assert_eq!(b_u16.1, true);

    let a_u32 = UInt_overflowing_sub___func(55_u32, 55_u32);
    println!("55 - 55 = {}\nUnderflow = {}", a_u32.0, a_u32.1);
    assert_eq!(a_u32.0, 0_u32);
    assert_eq!(a_u32.1, false);
 
    let b_u32 = UInt_overflowing_sub___func(a_u32.0, 1_u32);
    println!("{} - 1 = {}\nUnderflow = {}", a_u32.0, b_u32.0, b_u32.1);
    assert_eq!(b_u32.0, u32::MAX);
    assert_eq!(b_u32.1, true);

    let a_u64 = UInt_overflowing_sub___func(55_u64, 55_u64);
    println!("55 - 55 = {}\nUnderflow = {}", a_u64.0, a_u64.1);
    assert_eq!(a_u64.0, 0_u64);
    assert_eq!(a_u64.1, false);
 
    let b_u64 = UInt_overflowing_sub___func(a_u64.0, 1_u64);
    println!("{} - 1 = {}\nUnderflow = {}", a_u64.0, b_u64.0, b_u64.1);
    assert_eq!(b_u64.0, u64::MAX);
    assert_eq!(b_u64.1, true);

    let a_u128 = UInt_overflowing_sub___func(55_u128, 55_u128);
    println!("55 - 55 = {}\nUnderflow = {}", a_u128.0, a_u128.1);
    assert_eq!(a_u128.0, 0_u128);
    assert_eq!(a_u128.1, false);
 
    let b_u128 = UInt_overflowing_sub___func(a_u128.0, 1_u128);
    println!("{} - 1 = {}\nUnderflow = {}", a_u128.0, b_u128.0, b_u128.1);
    assert_eq!(b_u128.0, u128::MAX);
    assert_eq!(b_u128.1, true);

    let a_usize = UInt_overflowing_sub___func(55_usize, 55_usize);
    println!("55 - 55 = {}\nUnderflow = {}", a_usize.0, a_usize.1);
    assert_eq!(a_usize.0, 0_usize);
    assert_eq!(a_usize.1, false);
 
    let b_usize = UInt_overflowing_sub___func(a_usize.0, 1_usize);
    println!("{} - 1 = {}\nUnderflow = {}", a_usize.0, b_usize.0, b_usize.1);
    assert_eq!(b_usize.0, usize::MAX);
    assert_eq!(b_usize.1, true);
    println!("--------------------------------------");
}

fn UInt_overflowing_sub___func<T: Uint>(lhs: T, rhs: T) -> (T, bool)
{
    lhs.overflowing_sub(rhs)
}

fn UInt_checked_sub___main()
{
    println!("UInt_checked_sub___main()");
    let a_u8 = UInt_checked_sub___func(55_u8, 55_u8);
    match a_u8
    {
        Some(a) => {
                println!("55 - 55 = {}", a);
                assert_eq!(a, 0_u8);
            },
        None => {
                println!("Underflow happened.");
                assert_eq!(a_u8, None);
            },
    }
 
    let b_u8 = UInt_checked_sub___func(a_u8.unwrap(), 1_u8);
    match b_u8
    {
        Some(b) => {
                println!("{} - 1 = {}", a_u8.unwrap(), b);
                assert_eq!(b, u8::MAX);
            },
        None => {
                println!("Underflow happened.");
                assert_eq!(b_u8, None);
            },
    }

    let a_u16 = UInt_checked_sub___func(55_u16, 55_u16);
    match a_u16
    {
        Some(a) => {
                println!("55 - 55 = {}", a);
                assert_eq!(a, 0_u16);
            },
        None => {
                println!("Underflow happened.");
                assert_eq!(a_u16, None);
            },
    }

    let b_u16 = UInt_checked_sub___func(a_u16.unwrap(), 1_u16);
    match b_u16
    {
        Some(b) => {
                println!("{} - 1 = {}", a_u16.unwrap(), b);
                assert_eq!(b, u16::MAX);
            },
        None => {
                println!("Underflow happened.");
                assert_eq!(b_u16, None);
            },
    }

    let a_u32 = UInt_checked_sub___func(55_u32, 55_u32);
    match a_u32
    {
        Some(a) => {
                println!("55 - 55 = {}", a);
                assert_eq!(a, 0_u32);
            },
        None => {
                println!("Underflow happened.");
                assert_eq!(a_u32, None);
            },
    }

    let b_u32 = UInt_checked_sub___func(a_u32.unwrap(), 1_u32);
    match b_u32
    {
        Some(b) => {
                println!("{} - 1 = {}", a_u32.unwrap(), b);
                assert_eq!(b, u32::MAX);
            },
        None => {
                println!("Underflow happened.");
                assert_eq!(b_u32, None);
            },
    }

    let a_u64 = UInt_checked_sub___func(55_u64, 55_u64);
    match a_u64
    {
        Some(a) => {
                println!("55 - 55 = {}", a);
                assert_eq!(a, 0_u64);
            },
        None => {
                println!("Underflow happened.");
                assert_eq!(a_u64, None);
            },
    }

    let b_u64 = UInt_checked_sub___func(a_u64.unwrap(), 1_u64);
    match b_u64
    {
        Some(b) => {
                println!("{} + 1 = {}", a_u64.unwrap(), b);
                assert_eq!(b, u64::MAX);
            },
        None => {
                println!("Underflow happened.");
                assert_eq!(b_u64, None);
            },
    }

    let a_u128 = UInt_checked_sub___func(55_u128, 55_u128);
    match a_u128
    {
        Some(a) => {
                println!("55 - 55 = {}", a);
                assert_eq!(a, 0_u128);
            },
        None => {
                println!("Underflow happened.");
                assert_eq!(a_u128, None);
            },
    }

    let b_u128 = UInt_checked_sub___func(a_u128.unwrap(), 1_u128);
    match b_u128
    {
        Some(b) => {
                println!("{} - 1 = {}", a_u128.unwrap(), b);
                assert_eq!(b, u128::MAX);
            },
        None => {
                println!("Underflow happened.");
                assert_eq!(b_u128, None);
            },
    }

    let a_usize = UInt_checked_sub___func(55_usize, 55_usize);
    match a_usize
    {
        Some(a) => {
                println!("55 - 55 = {}", a);
                assert_eq!(a, 0_usize);
            },
        None => {
                println!("Underflow happened.");
                assert_eq!(a_usize, None);
            },
    }

    let b_usize = UInt_checked_sub___func(a_usize.unwrap(), 1_usize);
    match b_usize
    {
        Some(b) => {
                println!("{} - 1 = {}", a_usize.unwrap(), b);
                assert_eq!(b, usize::MAX);
            },
        None => {
                println!("Underflow happened.");
                assert_eq!(b_usize, None);
            },
    }
    println!("--------------------------------------");
}

fn UInt_checked_sub___func<T: Uint>(lhs: T, rhs: T) -> Option<T>
{
    lhs.checked_sub(rhs)
}

fn UInt_unchecked_sub___main()
{
    println!("UInt_unchecked_sub___main()");
    let a_u8 = UInt_unchecked_sub___func(55_u8, 55_u8);
    println!("55 - 55 = {}", a_u8);
    assert_eq!(a_u8, 0_u8);

    // It will panic
    // let b_u8 = UInt_unchecked_sub___func(a_u8, 1_u8);

    let a_u16 = UInt_unchecked_sub___func(55_u16, 55_u16);
    println!("55 - 55 = {}", a_u16);
    assert_eq!(a_u16, 0_u16);

    // It will panic
    // let b_u16 = UInt_unchecked_sub___func(a_u16, 1_u16);

    let a_u32 = UInt_unchecked_sub___func(55_u32, 55_u32);
    println!("55 - 55 = {}", a_u32);
    assert_eq!(a_u32, 0_u32);

    // It will panic
    // let b_u32 = UInt_unchecked_sub___func(a_u32, 1_u32);

    let a_u64 = UInt_unchecked_sub___func(55_u64, 55_u64);
    println!("55 - 55 = {}", a_u64);
    assert_eq!(a_u64, 0_u64);

    // It will panic
    // let b_u64 = UInt_unchecked_sub___func(a_u64, 1_u64);

    let a_u128 = UInt_unchecked_sub___func(55_u128, 55_u128);
    println!("55 - 55 = {}", a_u128);
    assert_eq!(a_u128, 0_u128);

    // It will panic
    // let b_u128 = UInt_unchecked_sub___func(a_u128, 1_u128);

    let a_usize = UInt_unchecked_sub___func(55_usize, 55_usize);
    println!("55 - 55 = {}", a_usize);
    assert_eq!(a_usize, 0_usize);

    // It will panic
    // let b_usize = UInt_unchecked_sub___func(a_usize, 1_usize);
    println!("--------------------------------------");
}

fn UInt_unchecked_sub___func<T: Uint>(lhs: T, rhs: T) -> T
{
    lhs.unchecked_sub(rhs)
}

fn UInt_saturating_sub___main()
{
    println!("UInt_saturating_sub___main()");
    let a_u8 = UInt_saturating_sub___func(55_u8, 50_u8);
    println!("55 - 50 = {}", a_u8);
    assert_eq!(a_u8, 5_u8);

    let b_u8 = UInt_saturating_sub___func(a_u8, 55_u8);
    println!("5 - 55 = {}", b_u8);
    assert_eq!(b_u8, 0_u8);

    let a_u16 = UInt_saturating_sub___func(55_u16, 50_u16);
    println!("55 - 50 = {}", a_u16);
    assert_eq!(a_u16, 5_u16);

    let b_u16 = UInt_saturating_sub___func(a_u16, 55_u16);
    println!("5 - 55 = {}", b_u16);
    assert_eq!(b_u16, 0_u16);

    let a_u32 = UInt_saturating_sub___func(55_u32, 50_u32);
    println!("55 - 50 = {}", a_u32);
    assert_eq!(a_u32, 5_u32);

    let b_u32 = UInt_saturating_sub___func(a_u32, 55_u32);
    println!("{} - 55 = {}", a_u32, b_u32);
    assert_eq!(b_u32, 0_u32);

    let a_u64 = UInt_saturating_sub___func(55_u64, 50_u64);
    println!("55 - 50 = {}", a_u64);
    assert_eq!(a_u64, 5_u64);

    let b_u64 = UInt_saturating_sub___func(a_u64, 55_u64);
    println!("{} - 55 = {}", a_u64, b_u64);
    assert_eq!(b_u64, 0_u64);

    let a_u128 = UInt_saturating_sub___func(55_u128, 50_u128);
    println!("55 - 50 = {}", a_u128);
    assert_eq!(a_u128, 5_u128);

    let b_u128 = UInt_saturating_sub___func(a_u128, 55_u128);
    println!("{} - 55 = {}", a_u128, b_u128);
    assert_eq!(b_u128, 0_u128);

    let a_usize = UInt_saturating_sub___func(55_usize, 50_usize);
    println!("55 - 50 = {}", a_usize);
    assert_eq!(a_usize, 5_usize);

    let b_usize = UInt_saturating_sub___func(a_usize, 55_usize);
    println!("{} - 55 = {}", a_usize, b_usize);
    assert_eq!(b_usize, 0_usize);
    println!("--------------------------------------");
}

fn UInt_saturating_sub___func<T: Uint>(lhs: T, rhs: T) -> T
{
    lhs.saturating_sub(rhs)
}


fn UInt_abs_diff___main()
{
    println!("UInt_abs_diff___main()");
    let a_u8 = UInt_abs_diff___func(55_u8, 50_u8);
    println!("55 <-> 50 = {}", a_u8);
    assert_eq!(a_u8, 5_u8);

    let b_u8 = UInt_abs_diff___func(50_u8, 55_u8);
    println!("50 <-> 55 = {}", b_u8);
    assert_eq!(b_u8, 5_u8);

    let a_u16 = UInt_abs_diff___func(5050_u16, 5000_u16);
    println!("5050 <-> 5000 = {}", a_u16);
    assert_eq!(a_u16, 50_u16);

    let b_u16 = UInt_abs_diff___func(5000_u16, 5050_u16);
    println!("5000 <-> 5050 = {}", b_u16);
    assert_eq!(b_u16, 50_u16);

    let a_u32 = UInt_abs_diff___func(500500_u32, 500000_u32);
    println!("500500 <-> 500000 = {}", a_u32);
    assert_eq!(a_u32, 500_u32);

    let b_u32 = UInt_abs_diff___func(500000_u32, 500500_u32);
    println!("500000 <-> 500500 = {}", b_u32);
    assert_eq!(b_u32, 500_u32);

    let a_u64 = UInt_abs_diff___func(5000050000_u64, 5000000000_u64);
    println!("5000050000 <-> 5000000000 = {}", a_u64);
    assert_eq!(a_u64, 50000_u64);

    let b_u64 = UInt_abs_diff___func(5000000000_u64, 5000050000_u64);
    println!("5000000000 <-> 5000050000 = {}", b_u64);
    assert_eq!(b_u64, 50000_u64);

    let a_u128 = UInt_abs_diff___func(500000000500000000_u128, 500000000000000000_u128);
    println!("500000000500000000 <-> 500000000000000000 = {}", a_u128);
    assert_eq!(a_u128, 500000000_u128);

    let b_u128 = UInt_abs_diff___func(500000000000000000_u128, 500000000500000000_u128);
    println!("500000000000000000 <-> 500000000500000000 = {}", b_u128);
    assert_eq!(b_u128, 500000000_u128);

    let a_usize = UInt_abs_diff___func(5000050000_usize, 5000000000_usize);
    println!("5000050000 <-> 5000000000 = {}", a_usize);
    assert_eq!(a_usize, 50000_usize);

    let b_usize = UInt_abs_diff___func(5000000000_usize, 5000050000_usize);
    println!("5000000000 <-> 5000050000 = {}", b_usize);
    assert_eq!(b_usize, 50000_usize);
    println!("--------------------------------------");
}

fn UInt_abs_diff___func<T: Uint>(lhs: T, rhs: T) -> T
{
    lhs.abs_diff(rhs)
}

fn UInt_wrapping_mul___main()
{
    println!("UInt_wrapping_mul___main()");
    let a_u8 = UInt_wrapping_mul___func(u8::MAX / 3, 2_u8);
    println!("{} * 2 = {}", u8::MAX / 3, a_u8);
    assert_eq!(a_u8, 170_u8);

    let b_u8 = UInt_wrapping_mul___func(a_u8, 2_u8);
    println!("{} * 2 = {}", a_u8, b_u8);
    assert_eq!(b_u8, 84_u8);

    let a_u16 = UInt_wrapping_mul___func(u16::MAX / 3, 2_u16);
    println!("{} * 2 = {}", u16::MAX / 3, a_u16);
    assert_eq!(a_u16, 43690_u16);

    let b_u16 = UInt_wrapping_mul___func(a_u16, 2_u16);
    println!("{} * 2 = {}", a_u16, b_u16);
    assert_eq!(b_u16, 21844_u16);

    let a_u32 = UInt_wrapping_mul___func(u32::MAX / 3, 2_u32);
    println!("{} * 2 = {}", u32::MAX / 3, a_u32);
    assert_eq!(a_u32, 2863311530_u32);

    let b_u32 = UInt_wrapping_mul___func(a_u32, 2_u32);
    println!("{} * 2 = {}", a_u32, b_u32);
    assert_eq!(b_u32, 1431655764_u32);

    let a_u64 = UInt_wrapping_mul___func(u64::MAX / 3, 2_u64);
    println!("{} * 2 = {}", u64::MAX / 3, a_u64);
    assert_eq!(a_u64, 12297829382473034410_u64);

    let b_u64 = UInt_wrapping_mul___func(a_u64, 2_u64);
    println!("{} * 2 = {}", a_u64, b_u64);
    assert_eq!(b_u64, 6148914691236517204_u64);

    let a_u128 = UInt_wrapping_mul___func(u128::MAX / 3, 2_u128);
    println!("{} * 2 = {}", u128::MAX / 3, a_u128);
    assert_eq!(a_u128,226854911280625642308916404954512140970_u128);

    let b_u128 = UInt_wrapping_mul___func(a_u128, 2_u128);
    println!("{} * 2 = {}", a_u128, b_u128);
    assert_eq!(b_u128, 113427455640312821154458202477256070484_u128);

    let a_usize = UInt_wrapping_mul___func(usize::MAX / 3, 2_usize);
    println!("{} * 2 = {}", usize::MAX / 3, a_usize);
    assert_eq!(a_usize, 12297829382473034410_usize);

    let b_usize = UInt_wrapping_mul___func(a_usize, 2_usize);
    println!("{} * 2 = {}", a_usize, b_usize);
    assert_eq!(b_usize, 6148914691236517204_usize);
    println!("--------------------------------------");
}

fn UInt_wrapping_mul___func<T: Uint>(lhs: T, rhs: T) -> T
{
    lhs.wrapping_mul(rhs)
}

fn UInt_overflowing_mul___main()
{
    println!("UInt_overflowing_mul___main()");
    let a_u8 = UInt_overflowing_mul___func(u8::MAX / 3, 2_u8);
    println!("{} * 2 = {}\nOverflow = {}", u8::MAX / 3, a_u8.0, a_u8.1);
    assert_eq!(a_u8.0, 170_u8);
    assert_eq!(a_u8.1, false);
 
    let b_u8 = UInt_overflowing_mul___func(a_u8.0, 2_u8);
    println!("{} * 2 = {}\nOverflow = {}", a_u8.0, b_u8.0, b_u8.1);
    assert_eq!(b_u8.0, 84_u8);
    assert_eq!(b_u8.1, true);

    let a_u16 = UInt_overflowing_mul___func(u16::MAX / 3, 2_u16);
    println!("{} * 2 = {}\nOverflow = {}", u16::MAX / 3, a_u16.0, a_u16.1);
    assert_eq!(a_u16.0, 43690_u16);
    assert_eq!(a_u16.1, false);
 
    let b_u16 = UInt_overflowing_mul___func(a_u16.0, 2_u16);
    println!("{} * 2 = {}\nOverflow = {}", a_u16.0, b_u16.0, b_u16.1);
    assert_eq!(b_u16.0, 21844_u16);
    assert_eq!(b_u16.1, true);

    let a_u32 = UInt_overflowing_mul___func(u32::MAX / 3, 2_u32);
    println!("{} * 2 = {}\nOverflow = {}", u32::MAX / 3, a_u32.0, a_u32.1);
    assert_eq!(a_u32.0, 2863311530_u32);
    assert_eq!(a_u32.1, false);
 
    let b_u32 = UInt_overflowing_mul___func(a_u32.0, 2_u32);
    println!("{} * 2 = {}\nOverflow = {}", a_u32.0, b_u32.0, b_u32.1);
    assert_eq!(b_u32.0, 1431655764_u32);
    assert_eq!(b_u32.1, true);

    let a_u64 = UInt_overflowing_mul___func(u64::MAX / 3, 2_u64);
    println!("{} * 2 = {}\nOverflow = {}", u64::MAX / 3, a_u64.0, a_u64.1);
    assert_eq!(a_u64.0, 12297829382473034410_u64);
    assert_eq!(a_u64.1, false);
 
    let b_u64 = UInt_overflowing_mul___func(a_u64.0, 2_u64);
    println!("{} * 2 = {}\nOverflow = {}", a_u64.0, b_u64.0, b_u64.1);
    assert_eq!(b_u64.0, 6148914691236517204_u64);
    assert_eq!(b_u64.1, true);

    let a_u128 = UInt_overflowing_mul___func(u128::MAX / 3, 2_u128);
    println!("{} * 2 = {}\nOverflow = {}", u128::MAX / 3, a_u128.0, a_u128.1);
    assert_eq!(a_u128.0, 226854911280625642308916404954512140970_u128);
    assert_eq!(a_u128.1, false);
 
    let b_u128 = UInt_overflowing_mul___func(a_u128.0, 2_u128);
    println!("{} * 2 = {}\nOverflow = {}", a_u128.0, b_u128.0, b_u128.1);
    assert_eq!(b_u128.0, 113427455640312821154458202477256070484_u128);
    assert_eq!(b_u128.1, true);

    let a_usize = UInt_overflowing_mul___func(usize::MAX / 3, 2_usize);
    println!("{} * 2 = {}\nOverflow = {}", usize::MAX / 3, a_usize.0, a_usize.1);
    assert_eq!(a_usize.0, 12297829382473034410_usize);
    assert_eq!(a_usize.1, false);
 
    let b_usize = UInt_overflowing_mul___func(a_usize.0, 2_usize);
    println!("{} * 2 = {}\nOverflow = {}", a_usize.0, b_usize.0, b_usize.1);
    assert_eq!(b_usize.0, 6148914691236517204_usize);
    assert_eq!(b_usize.1, true);
    println!("--------------------------------------");
}

fn UInt_overflowing_mul___func<T: Uint>(lhs: T, rhs: T) -> (T, bool)
{
    lhs.overflowing_mul(rhs)
}

fn UInt_checked_mul___main()
{
    println!("UInt_checked_mul___main()");
    let a_u8 = UInt_checked_mul___func(u8::MAX / 3, 2_u8);
    match a_u8
    {
        Some(a) => {
                println!("{} * 2 = {}", u8::MAX / 3, a_u8.unwrap());
                assert_eq!(a, 170_u8);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(a_u8, None);
            },
    }

    let b_u8 = UInt_checked_mul___func(a_u8.unwrap(), 2_u8);
    match b_u8
    {
        Some(b) => {
                println!("{} * 2 = {}", a_u8.unwrap(), b_u8.unwrap());
                assert_eq!(b, 84_u8);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(b_u8, None);
            },
    }

    let a_u16 = UInt_checked_mul___func(u16::MAX / 3, 2_u16);
    match a_u16
    {
        Some(a) => {
                println!("{} * 2 = {}", u16::MAX / 3, a_u16.unwrap());
                assert_eq!(a, 43690_u16);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(a_u16, None);
            },
    }

    let b_u16 = UInt_checked_mul___func(a_u16.unwrap(), 2_u16);
    match b_u16
    {
        Some(b) => {
                println!("{} * 2 = {}", a_u16.unwrap(), b);
                assert_eq!(b, 21844_u16);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(b_u16, None);
            },
    }

    let a_u32 = UInt_checked_mul___func(u32::MAX / 3, 2_u32);
    match a_u32
    {
        Some(a) => {
                println!("{} * 2 = {}", u32::MAX / 3, a_u32.unwrap());
                assert_eq!(a, 2863311530_u32);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(a_u32, None);
            },
    }

    let b_u32 = UInt_checked_mul___func(a_u32.unwrap(), 2_u32);
    match b_u32
    {
        Some(b) => {
                println!("{} * 2 = {}", a_u32.unwrap(), b);
                assert_eq!(b, 1431655764_u32);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(b_u32, None);
            },
    }

    let a_u64 = UInt_checked_mul___func(u64::MAX / 3, 2_u64);
    match a_u64
    {
        Some(a) => {
                println!("{} * 2 = {}", u64::MAX / 3, a_u64.unwrap());
                assert_eq!(a, 12297829382473034410_u64);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(a_u64, None);
            },
    }

    let b_u64 = UInt_checked_mul___func(a_u64.unwrap(), 2_u64);
    match b_u64
    {
        Some(b) => {
                println!("{} * 2 = {}", a_u64.unwrap(), b);
                assert_eq!(b, 6148914691236517204_u64);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(b_u64, None);
            },
    }

    let a_u128 = UInt_checked_mul___func(u128::MAX / 3, 2_u128);
    match a_u128
    {
        Some(a) => {
                println!("{} * 2 = {}", u128::MAX / 3, a_u128.unwrap());
                assert_eq!(a, 226854911280625642308916404954512140970_u128);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(a_u128, None);
            },
    }

    let b_u128 = UInt_checked_mul___func(a_u128.unwrap(), 2_u128);
    match b_u128
    {
        Some(b) => {
                println!("{} * 2 = {}", a_u128.unwrap(), b);
                assert_eq!(b, 113427455640312821154458202477256070484_u128);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(b_u128, None);
            },
    }

    let a_usize = UInt_checked_mul___func(usize::MAX / 3, 2_usize);
    match a_usize
    {
        Some(a) => {
                println!("{} * 2 = {}", usize::MAX / 3, a_usize.unwrap());
                assert_eq!(a, 12297829382473034410_usize);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(a_usize, None);
            },
    }

    let b_usize = UInt_checked_mul___func(a_usize.unwrap(), 2_usize);
    match b_usize
    {
        Some(b) => {
                println!("{} * 2 = {}", a_usize.unwrap(), b);
                assert_eq!(b, 6148914691236517204_usize);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(b_usize, None);
            },
    }
    println!("--------------------------------------");
}

fn UInt_checked_mul___func<T: Uint>(lhs: T, rhs: T) -> Option<T>
{
    lhs.checked_mul(rhs)
}

fn UInt_unchecked_mul___main()
{
    println!("UInt_unchecked_add___main()");
    let a_u8 = UInt_unchecked_mul___func(u8::MAX / 3, 2_u8);
    println!("{} * 2 = {}", u8::MAX / 3, a_u8);
    assert_eq!(a_u8, 170_u8);

    // It will panic
    // let b_u8 = UInt_unchecked_mul___func(a_u8, 2_u8);

    let a_u16 = UInt_unchecked_mul___func(u16::MAX / 3, 2_u16);
    println!("{} * 2 = {}", u16::MAX / 3, a_u16);
    assert_eq!(a_u16, 43690_u16);

    // It will panic
    // let b_u16 = UInt_unchecked_mul___func(a_u16, 2_u16);

    let a_u32 = UInt_unchecked_mul___func(u32::MAX / 3, 2_u32);
    println!("{} * 2 = {}", u32::MAX / 3, a_u32);
    assert_eq!(a_u32, 2863311530_u32);

    // It will panic
    // let b_u32 = UInt_unchecked_mul___func(a_u32, 2_u32);

    let a_u64 = UInt_unchecked_mul___func(u64::MAX / 3, 2_u64);
    println!("{} * 2 = {}", u64::MAX / 3, a_u64);
    assert_eq!(a_u64, 12297829382473034410_u64);

    // It will panic
    // let b_u64 = UInt_unchecked_mul___func(a_u64, 2_u64);

    let a_u128 = UInt_unchecked_mul___func(u128::MAX / 3, 2_u128);
    println!("{} * 2 = {}", u128::MAX / 3, a_u128);
    assert_eq!(a_u128, 226854911280625642308916404954512140970_u128);

    // It will panic
    // let b_u128 = UInt_unchecked_mul___func(a_u128, 2_u128);

    let a_usize = UInt_unchecked_mul___func(usize::MAX / 3, 2_usize);
    println!("{} * 2 = {}", usize::MAX / 3, a_usize);
    assert_eq!(a_usize, 12297829382473034410_usize);

    // It will panic
    // let b_usize = UInt_unchecked_mul___func(a_usize, 2_usize);
    println!("--------------------------------------");
}

fn UInt_unchecked_mul___func<T: Uint>(lhs: T, rhs: T) -> T
{
    lhs.unchecked_mul(rhs)
}

fn UInt_saturating_mul___main()
{
    println!("UInt_saturating_mul___main()");
    let a_u8 = UInt_saturating_mul___func(u8::MAX / 3, 2_u8);
    println!("{} * 2 = {}", u8::MAX / 3, a_u8);
    assert_eq!(a_u8, 170_u8);

    let b_u8 = UInt_saturating_mul___func(a_u8, 2_u8);
    println!("{} * 2 = {}", a_u8, b_u8);
    assert_eq!(b_u8, u8::MAX);

    let a_u16 = UInt_saturating_mul___func(u16::MAX / 3, 2_u16);
    println!("{} * 2 = {}", u16::MAX / 3, a_u16);
    assert_eq!(a_u16, 43690_u16);

    let b_u16 = UInt_saturating_mul___func(a_u16, 2_u16);
    println!("{} * 2 = {}", a_u16, b_u16);
    assert_eq!(b_u16, u16::MAX);

    let a_u32 = UInt_saturating_mul___func(u32::MAX / 3, 2_u32);
    println!("{} * 2 = {}", u32::MAX / 3, a_u32);
    assert_eq!(a_u32, 2863311530_u32);

    let b_u32 = UInt_saturating_mul___func(a_u32, 2_u32);
    println!("{} * 2 = {}", a_u32, b_u32);
    assert_eq!(b_u32, u32::MAX);

    let a_u64 = UInt_saturating_mul___func(u64::MAX / 3, 2_u64);
    println!("{} * 2 = {}", u64::MAX / 3, a_u64);
    assert_eq!(a_u64, 12297829382473034410_u64);

    let b_u64 = UInt_saturating_mul___func(a_u64, 2_u64);
    println!("{} * 2 = {}", a_u64, b_u64);
    assert_eq!(b_u64, u64::MAX);

    let a_u128 = UInt_saturating_mul___func(u128::MAX / 3, 2_u128);
    println!("{} * 2 = {}", u128::MAX / 3, a_u128);
    assert_eq!(a_u128, 226854911280625642308916404954512140970_u128);

    let b_u128 = UInt_saturating_mul___func(a_u128, 2_u128);
    println!("{} * 2 = {}",a_u128, b_u128);
    assert_eq!(b_u128, u128::MAX);

    let a_usize = UInt_saturating_mul___func(usize::MAX / 3, 2_usize);
    println!("{} * 2 = {}", usize::MAX / 3, a_usize);
    assert_eq!(a_usize, 12297829382473034410_usize);

    let b_usize = UInt_saturating_mul___func(a_usize, 2_usize);
    println!("{} * 2 = {}", a_usize, b_usize);
    assert_eq!(b_usize, usize::MAX);
    println!("--------------------------------------");
}

fn UInt_saturating_mul___func<T: Uint>(lhs: T, rhs: T) -> T
{
    lhs.saturating_mul(rhs)
}

fn UInt_wrapping_div___main()
{
    println!("UInt_wrapping_div___main()");
    let a_u8 = UInt_wrapping_div___func(u8::MAX / 3, 2_u8);
    println!("{} / 2 = {}", u8::MAX / 3, a_u8);
    assert_eq!(a_u8, 42_u8);

    let a_u16 = UInt_wrapping_div___func(u16::MAX / 3, 2_u16);
    println!("{} / 2 = {}", u16::MAX / 3, a_u16);
    assert_eq!(a_u16, 10922_u16);

    let a_u32 = UInt_wrapping_div___func(u32::MAX / 3, 2_u32);
    println!("{} / 2 = {}", u32::MAX / 3, a_u32);
    assert_eq!(a_u32, 715827882_u32);

    let a_u64 = UInt_wrapping_div___func(u64::MAX / 3, 2_u64);
    println!("{} / 2 = {}", u64::MAX / 3, a_u64);
    assert_eq!(a_u64, 3074457345618258602_u64);

    let a_u128 = UInt_wrapping_div___func(u128::MAX / 3, 2_u128);
    println!("{} / 2 = {}", u128::MAX / 3, a_u128);
    assert_eq!(a_u128, 56713727820156410577229101238628035242_u128);

    let a_usize = UInt_wrapping_div___func(usize::MAX / 3, 2_usize);
    println!("{} / 2 = {}", usize::MAX / 3, a_usize);
    assert_eq!(a_usize, 3074457345618258602_usize);

    // It will panic.
    // let a_panic = UInt_wrapping_div___func(usize::MAX / 3, 0_usize);
    println!("--------------------------------------");
}

fn UInt_wrapping_div___func<T: Uint>(lhs: T, rhs: T) -> T
{
    lhs.wrapping_div(rhs)
}

fn UInt_overflowing_div___main()
{
    println!("UInt_overflowing_div___main()");
    let a_u8 = UInt_overflowing_div___func(u8::MAX / 3, 2_u8);
    println!("{} / 2 = {}\nOverflow = {}", u8::MAX / 3, a_u8.0, a_u8.1);
    assert_eq!(a_u8.0, 42_u8);
    assert_eq!(a_u8.1, false);

    let a_u16 = UInt_overflowing_div___func(u16::MAX / 3, 2_u16);
    println!("{} / 2 = {}\nOverflow = {}", u16::MAX / 3, a_u16.0, a_u16.1);
    assert_eq!(a_u16.0, 10922_u16);
    assert_eq!(a_u16.1, false);
 
    let a_u32 = UInt_overflowing_div___func(u32::MAX / 3, 2_u32);
    println!("{} / 2 = {}\nOverflow = {}", u32::MAX / 3, a_u32.0, a_u32.1);
    assert_eq!(a_u32.0, 715827882_u32);
    assert_eq!(a_u32.1, false);
 
    let a_u64 = UInt_overflowing_div___func(u64::MAX / 3, 2_u64);
    println!("{} / 2 = {}\nOverflow = {}", u64::MAX / 3, a_u64.0, a_u64.1);
    assert_eq!(a_u64.0, 3074457345618258602_u64);
    assert_eq!(a_u64.1, false);
 
    let a_u128 = UInt_overflowing_div___func(u128::MAX / 3, 2_u128);
    println!("{} / 2 = {}\nOverflow = {}", u128::MAX / 3, a_u128.0, a_u128.1);
    assert_eq!(a_u128.0, 56713727820156410577229101238628035242_u128);
    assert_eq!(a_u128.1, false);
 
    let a_usize = UInt_overflowing_div___func(usize::MAX / 3, 2_usize);
    println!("{} / 2 = {}\nOverflow = {}", usize::MAX / 3, a_usize.0, a_usize.1);
    assert_eq!(a_usize.0, 3074457345618258602_usize);
    assert_eq!(a_usize.1, false);
 
    // It will panic.
    // let a_panic = UInt_overflowing_div___func(a_usize.0, 0_usize);
    println!("--------------------------------------");
}

fn UInt_overflowing_div___func<T: Uint>(lhs: T, rhs: T) -> (T, bool)
{
    lhs.overflowing_div(rhs)
}

fn UInt_checked_div___main()
{
    println!("UInt_checked_div___main()");
    let a_u8 = UInt_checked_div___func(u8::MAX / 3, 2_u8);
    match a_u8
    {
        Some(a) => {
                println!("{} / 2 = {}", u8::MAX / 3, a);
                assert_eq!(a, 42_u8);
            },
        None => {
                println!("Divided by zero.");
                assert_eq!(a_u8, None);
            },
    }

    let b_u8 = UInt_checked_div___func(u8::MAX / 3, 0_u8);
    match b_u8
    {
        Some(b) => { println!("{} / 2 = {}", u8::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_u8, None);
            },
    }

    let a_u16 = UInt_checked_div___func(u16::MAX / 3, 2_u16);
    match a_u16
    {
        Some(a) => {
                println!("{} / 2 = {}", u16::MAX / 3, a);
                assert_eq!(a, 10922_u16);
            },
        None => {
                println!("Divided by zero.");
                assert_eq!(a_u16, None);
            },
    }

    let b_u16 = UInt_checked_div___func(u16::MAX / 3, 0_u16);
    match b_u16
    {
        Some(b) => { println!("{} / 2 = {}", u16::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_u16, None);
            },
    }

    let a_u32 = UInt_checked_div___func(u32::MAX / 3, 2_u32);
    match a_u32
    {
        Some(a) => {
                println!("{} / 2 = {}", u32::MAX / 3, a);
                assert_eq!(a, 715827882_u32);
            },
        None => {
                println!("Divided by zero.");
                assert_eq!(a_u32, None);
            },
    }

    let b_u32 = UInt_checked_div___func(u32::MAX / 3, 0_u32);
    match b_u32
    {
        Some(b) => { println!("{} / 2 = {}", u32::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_u32, None);
            },
    }

    let a_u64 = UInt_checked_div___func(u64::MAX / 3, 2_u64);
    match a_u64
    {
        Some(a) => {
                println!("{} / 2 = {}", u64::MAX / 3, a);
                assert_eq!(a, 3074457345618258602_u64);
            },
        None => {
                println!("Divided by zero.");
                assert_eq!(a_u64, None);
            },
    }

    let b_u64 = UInt_checked_div___func(u64::MAX / 3, 0_u64);
    match b_u64
    {
        Some(b) => { println!("{} / 2 = {}", u64::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_u64, None);
            },
    }

    let a_u128 = UInt_checked_div___func(u128::MAX / 3, 2_u128);
    match a_u128
    {
        Some(a) => {
                println!("{} / 2 = {}", u128::MAX / 3, a);
                assert_eq!(a, 56713727820156410577229101238628035242_u128);
            },
        None => {
                println!("Divided by zero.");
                assert_eq!(a_u128, None);
            },
    }

    let b_u128 = UInt_checked_div___func(u128::MAX / 3, 0_u128);
    match b_u128
    {
        Some(b) => { println!("{} / 2 = {}", u128::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_u128, None);
            },
    }

    let a_usize = UInt_checked_div___func(usize::MAX / 3, 2_usize);
    match a_usize
    {
        Some(a) => {
                println!("{} / 2 = {}", usize::MAX / 3, a);
                assert_eq!(a, 3074457345618258602_usize);
            },
        None => {
                println!("Divided by zero.");
                assert_eq!(a_usize, None);
            },
    }

    let b_usize = UInt_checked_div___func(usize::MAX / 3, 0_usize);
    match b_usize
    {
        Some(b) => { println!("{} / 2 = {}", usize::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_usize, None);
            },
    }
    println!("--------------------------------------");
}

fn UInt_checked_div___func<T: Uint>(lhs: T, rhs: T) -> Option<T>
{
    lhs.checked_div(rhs)
}

fn UInt_saturating_div___main()
{
    println!("UInt_saturating_div___main()");
    let a_u8 = UInt_saturating_div___func(u8::MAX / 3, 2_u8);
    println!("{} / 2 = {}", u8::MAX / 3, a_u8);
    assert_eq!(a_u8, 42_u8);

    let a_u16 = UInt_saturating_div___func(u16::MAX / 3, 2_u16);
    println!("{} / 2 = {}", u16::MAX / 3, a_u16);
    assert_eq!(a_u16, 10922_u16);

    let a_u32 = UInt_saturating_div___func(u32::MAX / 3, 2_u32);
    println!("{} / 2 = {}", u32::MAX / 3, a_u32);
    assert_eq!(a_u32, 715827882_u32);

    let a_u64 = UInt_saturating_div___func(u64::MAX / 3, 2_u64);
    println!("{} / 2 = {}", u64::MAX / 3, a_u64);
    assert_eq!(a_u64, 3074457345618258602_u64);

    let a_u128 = UInt_saturating_div___func(u128::MAX / 3, 2_u128);
    println!("{} / 2 = {}", u128::MAX / 3, a_u128);
    assert_eq!(a_u128, 56713727820156410577229101238628035242_u128);

    let a_usize = UInt_saturating_div___func(usize::MAX / 3, 2_usize);
    println!("{} / 2 = {}", usize::MAX / 3, a_usize);
    assert_eq!(a_usize, 3074457345618258602_usize);

    // It will panic.
    // let a_panic = UInt_saturating_div___func(usize::MAX / 3, 0_usize);
    println!("--------------------------------------");
}

fn UInt_saturating_div___func<T: Uint>(lhs: T, rhs: T) -> T
{
    lhs.saturating_div(rhs)
}

fn UInt_wrapping_rem___main()
{
    println!("UInt_wrapping_rem___main()");
    let a_u8 = UInt_wrapping_rem___func(u8::MAX / 3, 3_u8);
    println!("{} % 3 = {}", u8::MAX / 3, a_u8);
    assert_eq!(a_u8, 1_u8);

    let a_u16 = UInt_wrapping_rem___func(u16::MAX / 3, 3_u16);
    println!("{} % 3 = {}", u16::MAX / 3, a_u16);
    assert_eq!(a_u16, 2_u16);

    let a_u32 = UInt_wrapping_rem___func(u32::MAX / 3, 3_u32);
    println!("{} % 3 = {}", u32::MAX / 3, a_u32);
    assert_eq!(a_u32, 1_u32);

    let a_u64 = UInt_wrapping_rem___func(u64::MAX / 3, 3_u64);
    println!("{} % 3 = {}", u64::MAX / 3, a_u64);
    assert_eq!(a_u64, 2_u64);

    let a_u128 = UInt_wrapping_rem___func(u128::MAX / 3, 3_u128);
    println!("{} % 3 = {}", u128::MAX / 3, a_u128);
    assert_eq!(a_u128, 1_u128);

    let a_usize = UInt_wrapping_rem___func(usize::MAX / 3, 3_usize);
    println!("{} % 3 = {}", usize::MAX / 3, a_usize);
    assert_eq!(a_usize, 2_usize);

    // It will panic.
    // let a_panic = UInt_wrapping_rem___func(usize::MAX / 3, 0_usize);
    println!("--------------------------------------");
}

fn UInt_wrapping_rem___func<T: Uint>(lhs: T, rhs: T) -> T
{
    lhs.wrapping_rem(rhs)
}

fn UInt_overflowing_rem___main()
{
    println!("UInt_overflowing_rem___main()");
    let a_u8 = UInt_overflowing_rem___func(u8::MAX / 3, 3_u8);
    println!("{} % 3 = {}\nOverflow = {}", u8::MAX / 3, a_u8.0, a_u8.1);
    assert_eq!(a_u8.0, 1_u8);
    assert_eq!(a_u8.1, false);

    let a_u16 = UInt_overflowing_rem___func(u16::MAX / 3, 3_u16);
    println!("{} % 3 = {}\nOverflow = {}", u16::MAX / 3, a_u16.0, a_u16.1);
    assert_eq!(a_u16.0, 2_u16);
    assert_eq!(a_u16.1, false);
 
    let a_u32 = UInt_overflowing_rem___func(u32::MAX / 3, 3_u32);
    println!("{} % 3 = {}\nOverflow = {}", u32::MAX / 3, a_u32.0, a_u32.1);
    assert_eq!(a_u32.0, 1_u32);
    assert_eq!(a_u32.1, false);
 
    let a_u64 = UInt_overflowing_rem___func(u64::MAX / 3, 3_u64);
    println!("{} % 3 = {}\nOverflow = {}", u64::MAX / 3, a_u64.0, a_u64.1);
    assert_eq!(a_u64.0, 2_u64);
    assert_eq!(a_u64.1, false);
 
    let a_u128 = UInt_overflowing_rem___func(u128::MAX / 3, 3_u128);
    println!("{} % 3 = {}\nOverflow = {}", u128::MAX / 3, a_u128.0, a_u128.1);
    assert_eq!(a_u128.0, 1_u128);
    assert_eq!(a_u128.1, false);
 
    let a_usize = UInt_overflowing_rem___func(usize::MAX / 3, 3_usize);
    println!("{} % 3 = {}\nOverflow = {}", usize::MAX / 3, a_usize.0, a_usize.1);
    assert_eq!(a_usize.0, 2_usize);
    assert_eq!(a_usize.1, false);
 
    // It will panic.
    // let a_panic = UInt_overflowing_rem___func(a_usize.0, 0_usize);
    println!("--------------------------------------");
}

fn UInt_overflowing_rem___func<T: Uint>(lhs: T, rhs: T) -> (T, bool)
{
    lhs.overflowing_rem(rhs)
}

fn UInt_checked_rem___main()
{
    println!("UInt_checked_rem___main()");
    let a_u8 = UInt_checked_rem___func(u8::MAX / 3, 3_u8);
    match a_u8
    {
        Some(a) => {
                println!("{} % 3 = {}", u8::MAX / 3, a);
                assert_eq!(a, 1_u8);
            },
        None => {
                println!("Divided by zero.");
                assert_eq!(a_u8, None);
            },
    }

    let b_u8 = UInt_checked_rem___func(u8::MAX / 3, 0_u8);
    match b_u8
    {
        Some(b) => { println!("{} % 3 = {}", u8::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_u8, None);
            },
    }

    let a_u16 = UInt_checked_rem___func(u16::MAX / 3, 3_u16);
    match a_u16
    {
        Some(a) => {
                println!("{} % 3 = {}", u16::MAX / 3, a);
                assert_eq!(a, 2_u16);
            },
        None => {
                println!("Divided by zero.");
                assert_eq!(a_u16, None);
            },
    }

    let b_u16 = UInt_checked_rem___func(u16::MAX / 3, 0_u16);
    match b_u16
    {
        Some(b) => { println!("{} % 3 = {}", u16::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_u16, None);
            },
    }

    let a_u32 = UInt_checked_rem___func(u32::MAX / 3, 3_u32);
    match a_u32
    {
        Some(a) => {
                println!("{} % 3 = {}", u32::MAX / 3, a);
                assert_eq!(a, 1_u32);
            },
        None => {
                println!("Divided by zero.");
                assert_eq!(a_u32, None);
            },
    }

    let b_u32 = UInt_checked_rem___func(u32::MAX / 3, 0_u32);
    match b_u32
    {
        Some(b) => { println!("{} % 3 = {}", u32::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_u32, None);
            },
    }

    let a_u64 = UInt_checked_rem___func(u64::MAX / 3, 3_u64);
    match a_u64
    {
        Some(a) => {
                println!("{} % 3 = {}", u64::MAX / 3, a);
                assert_eq!(a, 2_u64);
            },
        None => {
                println!("Divided by zero.");
                assert_eq!(a_u64, None);
            },
    }

    let b_u64 = UInt_checked_rem___func(u64::MAX / 3, 0_u64);
    match b_u64
    {
        Some(b) => { println!("{} % 3 = {}", u64::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_u64, None);
            },
    }

    let a_u128 = UInt_checked_rem___func(u128::MAX / 3, 3_u128);
    match a_u128
    {
        Some(a) => {
                println!("{} % 3 = {}", u128::MAX / 3, a);
                assert_eq!(a, 1_u128);
            },
        None => {
                println!("Divided by zero.");
                assert_eq!(a_u128, None);
            },
    }

    let b_u128 = UInt_checked_rem___func(u128::MAX / 3, 0_u128);
    match b_u128
    {
        Some(b) => { println!("{} % 3 = {}", u128::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_u128, None);
            },
    }

    let a_usize = UInt_checked_rem___func(usize::MAX / 3, 3_usize);
    match a_usize
    {
        Some(a) => {
                println!("{} % 3 = {}", usize::MAX / 3, a);
                assert_eq!(a, 2_usize);
            },
        None => {
                println!("Divided by zero.");
                assert_eq!(a_usize, None);
            },
    }

    let b_usize = UInt_checked_rem___func(usize::MAX / 3, 0_usize);
    match b_usize
    {
        Some(b) => { println!("{} % 3 = {}", usize::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_usize, None);
            },
    }
    println!("--------------------------------------");
}

fn UInt_checked_rem___func<T: Uint>(lhs: T, rhs: T) -> Option<T>
{
    lhs.checked_rem(rhs)
}

fn UInt_pow___main()
{
    println!("UInt_pow___main()");
    let a_u8 = UInt_pow___func(3_u8, 5_u32);
    println!("3 ** 5 = {}", a_u8);
    assert_eq!(a_u8, 243_u8);
    // It will panic.
    // println!("3 ** 6 = {}", UInt_pow___func(3_u8, 6_u32));
    
    let a_u16 = UInt_pow___func(9_u16, 5_u32);
    println!("9 ** 5 = {}", a_u16);
    assert_eq!(a_u16, 59049_u16);
    // It will panic.
    // println!("9 ** 6 = {}", UInt_pow___func(9_u16, 6_u32));

    let a_u32 = UInt_pow___func(81_u32, 5_u32);
    println!("81 ** 5 = {}", a_u32);
    assert_eq!(a_u32, 3486784401_u32);
    // It will panic.
    // println!("81 ** 6 = {}", UInt_pow___func(81_u32, 6_u32));

    let a_u64 = UInt_pow___func(6561_u64, 5_u32);
    println!("6561 ** 5 = {}", a_u64);
    assert_eq!(a_u64, 12157665459056928801_u64);
    // It will panic.
    // println!("6561 ** 6 = {}", UInt_pow___func(6561_u64, 6_u32));

    let a_u128 = UInt_pow___func(43046721_u128, 5_u32);
    println!("43046721 ** 5 = {}", a_u128);
    assert_eq!(a_u128, 147808829414345923316083210206383297601_u128);
    // It will panic.
    // println!("43046721 ** 6 = {}", UInt_pow___func(43046721_u64, 6_u32));

    let a_usize = UInt_pow___func(6561_usize, 5_u32);
    println!("6561 ** 5 = {}", a_usize);
    assert_eq!(a_usize, 12157665459056928801_usize);
    // It will panic.
    // println!("6561 ** 6 = {}", UInt_pow___func(6561_usize, 6_u32));

    let a_ushort = ShortUnion::new_with(9);
    let b_ushort = UInt_pow___func(a_ushort, 5_u32);
    println!("9 ** 5 = {}", b_ushort);
    assert_eq!(b_ushort.get(), 59049_u16);
    // It will panic.
    // println!("9 ** 5 = {}", UInt_pow___func(a_ushort, 6_u32));

    let a_uint = IntUnion::new_with(81);
    let b_uint = UInt_pow___func(a_uint, 5_u32);
    println!("81 ** 5 = {}", b_uint);
    assert_eq!(b_uint.get(), 3486784401_u32);
    // It will panic.
    // println!("81 ** 6 = {}", UInt_pow___func(a_uint, 6_u32));

    let a_ulong = LongUnion::new_with(6561);
    let b_ulong = UInt_pow___func(a_ulong, 5_u32);
    println!("6561 ** 5 = {}", b_ulong);
    assert_eq!(b_ulong.get(), 12157665459056928801_u64);
    // It will panic.
    // println!("6561 ** 6 = {}", UInt_pow___func(a_ulong, 6_u32));

    let a_ulonger = LongerUnion::new_with(43046721);
    let b_ulonger = UInt_pow___func(a_ulonger, 5_u32);
    println!("43046721 ** 5 = {}", b_ulonger);
    assert_eq!(b_ulonger.get(), 147808829414345923316083210206383297601_u128);
    // It will panic.
    // println!("43046721 ** 6 = {}", UInt_pow___func(a_ulonger, 6_u32));

    let a_size = SizeUnion::new_with(6561);
    let b_size = UInt_pow___func(a_size, 5_u32);
    println!("6561 ** 5 = {}",b_size);
    assert_eq!(b_size.get(), 12157665459056928801_usize);
    // It will panic.
    // println!("6561 ** 6 = {}", UInt_pow___func(a_size, 6_u32));
    println!("--------------------------------------");
}

fn UInt_pow___func<T: Uint>(base: T, exp: u32) -> T
{
    base.pow(exp)
}


fn UInt_wrapping_pow___main()
{
    println!("UInt_wrapping_pow___func()");
    let a_u8 = UInt_wrapping_pow___func(3_u8, 5_u32);
    println!("3 ** 5 = {}", a_u8);
    assert_eq!(a_u8, 243_u8);

    let b_u8 = UInt_wrapping_pow___func(3_u8, 6_u32);
    println!("3 ** 6 = {}", b_u8);
    assert_eq!(b_u8, 217_u8);
    
    let a_u16 = UInt_wrapping_pow___func(9_u16, 5_u32);
    println!("9 ** 5 = {}", a_u16);
    assert_eq!(a_u16, 59049_u16);

    let b_u16 = UInt_wrapping_pow___func(9_u16, 6_u32);
    println!("9 ** 6 = {}", b_u16);
    assert_eq!(b_u16, 7153_u16);

    let a_u32 = UInt_wrapping_pow___func(81_u32, 5_u32);
    println!("81 ** 5 = {}", a_u32);
    assert_eq!(a_u32, 3486784401_u32);

    let b_u32 = UInt_wrapping_pow___func(81_u32, 6_u32);
    println!("81 ** 6 = {}", b_u32);
    assert_eq!(b_u32, 3256662241_u32);

    let a_u64 = UInt_wrapping_pow___func(6561_u64, 5_u32);
    println!("6561 ** 5 = {}", a_u64);
    assert_eq!(a_u64, 12157665459056928801_u64);

    let b_u64 = UInt_wrapping_pow___func(6561_u64, 6_u32);
    println!("6561 ** 6 = {}", b_u64);
    assert_eq!(b_u64, 2721702152408675777_u64);

    let a_u128 = UInt_wrapping_pow___func(43046721_u128, 5_u32);
    println!("43046721 ** 5 = {}", a_u128);
    assert_eq!(a_u128, 147808829414345923316083210206383297601_u128);

    let b_u128 = UInt_wrapping_pow___func(43046721_u128, 6_u32);
    println!("43046721 ** 6 = {}", b_u128);
    assert_eq!(b_u128, 333574137813082321045752866839264852865_u128);

    let a_usize = UInt_wrapping_pow___func(6561_usize, 5_u32);
    println!("6561 ** 5 = {}", a_usize);
    assert_eq!(a_usize, 12157665459056928801_usize);

    let b_usize = UInt_wrapping_pow___func(6561_usize, 6_u32);
    println!("6561 ** 6 = {}", b_usize);
    assert_eq!(b_usize, 2721702152408675777_usize);

    let a_ushort = ShortUnion::new_with(9);
    let b_ushort = UInt_wrapping_pow___func(a_ushort, 5_u32);
    println!("9 ** 5 = {}", b_ushort);
    assert_eq!(b_ushort.get(), 59049_u16);

    let c_ushort = UInt_wrapping_pow___func(a_ushort, 6_u32);
    println!("9 ** 6 = {}", c_ushort);
    assert_eq!(c_ushort.get(), 7153_u16);

    let a_uint = IntUnion::new_with(81);
    let b_uint = UInt_wrapping_pow___func(a_uint, 5_u32);
    println!("81 ** 5 = {}", b_uint);
    assert_eq!(b_uint.get(), 3486784401_u32);

    let c_uint = UInt_wrapping_pow___func(a_uint, 6_u32);
    println!("81 ** 6 = {}", c_uint);
    assert_eq!(c_uint.get(), 3256662241_u32);

    let a_ulong = LongUnion::new_with(6561);
    let b_ulong = UInt_wrapping_pow___func(a_ulong, 5_u32);
    println!("6561 ** 5 = {}", b_ulong);
    assert_eq!(b_ulong.get(), 12157665459056928801_u64);

    let c_ulong = UInt_wrapping_pow___func(a_ulong, 6_u32);
    println!("6561 ** 6 = {}", c_ulong);
    assert_eq!(c_ulong.get(), 2721702152408675777_u64);

    let a_ulonger = LongerUnion::new_with(43046721);
    let b_ulonger = UInt_wrapping_pow___func(a_ulonger, 5_u32);
    println!("43046721 ** 5 = {}", b_ulonger);
    assert_eq!(b_ulonger.get(), 147808829414345923316083210206383297601_u128);

    let c_ulonger = UInt_wrapping_pow___func(a_ulonger, 6_u32);
    println!("43046721 ** 6 = {}", c_ulonger);
    assert_eq!(c_ulonger.get(), 333574137813082321045752866839264852865_u128);

    let a_size = SizeUnion::new_with(6561);
    let b_size = UInt_wrapping_pow___func(a_size, 5_u32);
    println!("6561 ** 5 = {}", b_size);
    assert_eq!(b_size.get(), 12157665459056928801_usize);

    let c_size = UInt_wrapping_pow___func(a_size, 6_u32);
    println!("6561 ** 6 = {}", c_size);
    assert_eq!(c_size.get(), 2721702152408675777_usize);
    println!("--------------------------------------");
}

fn UInt_wrapping_pow___func<T: Uint>(base: T, exp: u32) -> T
{
    base.wrapping_pow(exp)
}

/*
fn UInt_overflowing_powv___main()
{
    println!("UInt_overflowing_powv___main()");
    let a_u8 = UInt_overflowing_pow___func(u8::MAX / 3, 2_u8);
    println!("{} / 2 = {}\nOverflow = {}", u8::MAX / 3, a_u8.0, a_u8.1);
    assert_eq!(a_u8.0, 42_u8);
    assert_eq!(a_u8.1, false);

    let a_u16 = UInt_overflowing_pow___func(u16::MAX / 3, 2_u16);
    println!("{} / 2 = {}\nOverflow = {}", u16::MAX / 3, a_u16.0, a_u16.1);
    assert_eq!(a_u16.0, 10922_u16);
    assert_eq!(a_u16.1, false);
 
    let a_u32 = UInt_overflowing_pow___func(u32::MAX / 3, 2_u32);
    println!("{} / 2 = {}\nOverflow = {}", u32::MAX / 3, a_u32.0, a_u32.1);
    assert_eq!(a_u32.0, 715827882_u32);
    assert_eq!(a_u32.1, false);
 
    let a_u64 = UInt_overflowing_pow___func(u64::MAX / 3, 2_u64);
    println!("{} / 2 = {}\nOverflow = {}", u64::MAX / 3, a_u64.0, a_u64.1);
    assert_eq!(a_u64.0, 3074457345618258602_u64);
    assert_eq!(a_u64.1, false);
 
    let a_u128 = UInt_overflowing_pow___func(u128::MAX / 3, 2_u128);
    println!("{} / 2 = {}\nOverflow = {}", u128::MAX / 3, a_u128.0, a_u128.1);
    assert_eq!(a_u128.0, 56713727820156410577229101238628035242_u128);
    assert_eq!(a_u128.1, false);
 
    let a_usize = UInt_overflowing_pow___func(usize::MAX / 3, 2_usize);
    println!("{} / 2 = {}\nOverflow = {}", usize::MAX / 3, a_usize.0, a_usize.1);
    assert_eq!(a_usize.0, 3074457345618258602_usize);
    assert_eq!(a_usize.1, false);
 
    // It will panic.
    // let a_panic = UInt_overflowing_pow___func(a_usize.0, 0_usize);
    println!("--------------------------------------");
}

fn UInt_overflowing_pow___func<T: Uint>(base: T, exp: u32) -> (T, bool)
{
    base.overflowing_pow(exp)
}

fn UInt_checked_pow___main()
{
    println!("UInt_checked_div___main()");
    let a_u8 = UInt_checked_pow___main(u8::MAX / 3, 2_u8);
    match a_u8
    {
        Some(a) => {
                println!("{} / 2 = {}", u8::MAX / 3, a);
                assert_eq!(a, 42_u8);
            },
        None => {
                println!("Divided by zero.");
                assert_eq!(a_u8, None);
            },
    }

    let b_u8 = UInt_checked_pow___func(u8::MAX / 3, 0_u8);
    match b_u8
    {
        Some(b) => { println!("{} / 2 = {}", u8::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_u8, None);
            },
    }

    let a_u16 = UInt_checked_pow___func(u16::MAX / 3, 2_u16);
    match a_u16
    {
        Some(a) => {
                println!("{} / 2 = {}", u16::MAX / 3, a);
                assert_eq!(a, 10922_u16);
            },
        None => {
                println!("Divided by zero.");
                assert_eq!(a_u16, None);
            },
    }

    let b_u16 = UInt_checked_pow___func(u16::MAX / 3, 0_u32);
    match b_u16
    {
        Some(b) => { println!("{} / 2 = {}", u16::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_u16, None);
            },
    }

    let a_u32 = UInt_checked_pow___func(u32::MAX / 3, 2_u32);
    match a_u32
    {
        Some(a) => {
                println!("{} / 2 = {}", u32::MAX / 3, a);
                assert_eq!(a, 715827882_u32);
            },
        None => {
                println!("Divided by zero.");
                assert_eq!(a_u32, None);
            },
    }

    let b_u32 = UInt_checked_pow___func(u32::MAX / 3, 0_u32);
    match b_u32
    {
        Some(b) => { println!("{} / 2 = {}", u32::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_u32, None);
            },
    }

    let a_u64 = UInt_checked_pow___func(u64::MAX / 3, 2_u64);
    match a_u64
    {
        Some(a) => {
                println!("{} / 2 = {}", u64::MAX / 3, a);
                assert_eq!(a, 3074457345618258602_u64);
            },
        None => {
                println!("Divided by zero.");
                assert_eq!(a_u64, None);
            },
    }

    let b_u64 = UInt_checked_pow___func(u64::MAX / 3, 0_u64);
    match b_u64
    {
        Some(b) => { println!("{} / 2 = {}", u64::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_u64, None);
            },
    }

    let a_u128 = UInt_checked_pow___func(u128::MAX / 3, 2_u128);
    match a_u128
    {
        Some(a) => {
                println!("{} / 2 = {}", u128::MAX / 3, a);
                assert_eq!(a, 56713727820156410577229101238628035242_u128);
            },
        None => {
                println!("Divided by zero.");
                assert_eq!(a_u128, None);
            },
    }

    let b_u128 = UInt_checked_pow___func(u128::MAX / 3, 0_u128);
    match b_u128
    {
        Some(b) => { println!("{} / 2 = {}", u128::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_u128, None);
            },
    }

    let a_usize = UInt_checked_pow___func(usize::MAX / 3, 2_usize);
    match a_usize
    {
        Some(a) => {
                println!("{} / 2 = {}", usize::MAX / 3, a);
                assert_eq!(a, 3074457345618258602_usize);
            },
        None => {
                println!("Divided by zero.");
                assert_eq!(a_usize, None);
            },
    }

    let b_usize = UInt_checked_pow___func(usize::MAX / 3, 0_usize);
    match b_usize
    {
        Some(b) => { println!("{} / 2 = {}", usize::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_usize, None);
            },
    }
    println!("--------------------------------------");
}

fn UInt_checked_pow___func<T: Uint>(base: T, exp: u32) -> Option<T>
{
    base.checked_pow(exp)
}

fn UInt_saturating_pow___main()
{
    println!("UInt_saturating_pow___main()");
    let a_u8 = UInt_saturating_pow___func(u8::MAX / 3, 2_u8);
    println!("{} / 2 = {}", u8::MAX / 3, a_u8);
    assert_eq!(a_u8, 42_u8);

    let a_u16 = UInt_saturating_pow___func(u16::MAX / 3, 2_u16);
    println!("{} / 2 = {}", u16::MAX / 3, a_u16);
    assert_eq!(a_u16, 10922_u16);

    let a_u32 = UInt_saturating_pow___func(u32::MAX / 3, 2_u32);
    println!("{} / 2 = {}", u32::MAX / 3, a_u32);
    assert_eq!(a_u32, 715827882_u32);

    let a_u64 = UInt_saturating_pow___func(u64::MAX / 3, 2_u64);
    println!("{} / 2 = {}", u64::MAX / 3, a_u64);
    assert_eq!(a_u64, 3074457345618258602_u64);

    let a_u128 = UInt_saturating_pow___func(u128::MAX / 3, 2_u128);
    println!("{} / 2 = {}", u128::MAX / 3, a_u128);
    assert_eq!(a_u128, 56713727820156410577229101238628035242_u128);

    let a_usize = UInt_saturating_pow___func(usize::MAX / 3, 2_usize);
    println!("{} / 2 = {}", usize::MAX / 3, a_usize);
    assert_eq!(a_usize, 3074457345618258602_usize);

    // It will panic.
    // let a_panic = UInt_saturating_pow___func(usize::MAX / 3, 0_usize);
    println!("--------------------------------------");
}

fn UInt_saturating_pow___func<T: Uint>(lhs: T, rhs: T) -> T
{
    lhs.saturating_pow(rhs)
}
*/