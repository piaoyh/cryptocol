// Copyright 2023, 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


#![allow(missing_docs)]
#![allow(missing_doc_code_examples)]
#[allow(non_camel_case_types)]

pub fn main()
{
    small_uint_add_main();
    small_uint_sub_main();
    small_uint_mul_main();
    small_uint_div_main();
    small_uint_rem_main();
    small_uint_pow_main();
    small_uint_root_main();
    small_uint_prime_main();
}

fn small_uint_add_main()
{
    small_uint_carrying_add();
    small_uint_wrapping_add();
    small_uint_overflowing_add();
    small_uint_checked_add();
    small_uint_unchecked_add();
    small_uint_saturating_add();
    small_uint_modular_add();
}

fn small_uint_carrying_add()
{
    println!("small_uint_carrying_add()");
    use cryptocol::number::SmallUInt;
    // Example for u8
    // a_u16: u16 === (a_high_u8, a_low_u8) == (100_u8, 101_u8) == 25701_u16
    let a_high_u8 = 100_u8;
    let a_low_u8 = 101_u8;
    // b_u16: u16 === (b_high_u8, b_low_u8) == (100_u8, 200_u8) == 25800_u16
    let b_high_u8 = 100_u8;
    let b_low_u8 = 200_u8;

    // (100_u8, 101_u8) + (100_u8, 200_u8) == 25701_u16 + 25800_u16 == 51501_u16
    //   25701_u16 == (100_u8, 101_u8)
    // + 25800_u16 == (100_u8, 200_u8)
    // -------------------------------
    //   51501_u16 == (201_u8,  45_u8)

    // c: u16 === (c_high, c_low)
    let (c_low_u8, c_high_u8, carry) = small_uint_carrying_add_func(a_low_u8, a_high_u8, b_low_u8, b_high_u8);
    println!("{}-{}, {}", c_high_u8, c_low_u8, carry);
    assert_eq!(c_high_u8, 201);
    assert_eq!(c_low_u8, 45);
    assert_eq!(carry, false);

    //  (201_u8,  45_u8) + (100_u8, 200_u8) == 25701_u16 + 25800_u16 == 51501_u16
    //   25701_u16 == (100_u8, 101_u8)
    // + 25800_u16 == (100_u8, 200_u8)
    // -------------------------------
    //   11765_u16 == ( 45_u8, 245_u8)

    // d: u16 === (d_high_u8, d_low_u8)
    let (d_low_u8, d_high_u8, carry) = small_uint_carrying_add_func(c_low_u8, c_high_u8, b_low_u8, b_high_u8);
    println!("{}-{}, {}", d_high_u8, d_low_u8, carry);
    assert_eq!(d_high_u8, 45_u8);
    assert_eq!(d_low_u8, 245_u8);
    assert_eq!(carry, true);

    // Example for u16
    // a_u32: u32 === (a_high_u16, a_low_u16) == (10000_u16, 10100_u16) == 655370100_u32
    let a_high_u16 = 10000_u16;
    let a_low_u16 = 10100_u16;
    // b_u32: u32 === (b_high_u16, b_low_u16) == (10000_u16, 20000_u16) == 3276830000_u32
    let b_high_u16 = 50000_u16;
    let b_low_u16 = 30000_u16;

    // (10000_u16, 10100_u16) + (50000_u16, 30000_u16) == 655370100_u32 + 3276830000_u32 == 3932200100_u32
    //    655370100_u32 == (10000_u16, 10100_u16)
    // + 3276830000_u32 == (50000_u16, 30000_u16)
    // ------------------------------------------
    //   3932200100_u32 == (60000_u16, 40100_u16)

    // c: u32 === (c_high_u16, c_low_u16)
    let (c_low_u16, c_high_u16, carry) = small_uint_carrying_add_func(a_low_u16, a_high_u16, b_low_u16, b_high_u16);
    println!("{}-{}, {}", c_high_u16, c_low_u16, carry);
    assert_eq!(c_high_u16, 60000_u16);
    assert_eq!(c_low_u16, 40100_u16);
    assert_eq!(carry, false);

    // (10000_u16, 10100_u16) + (50000_u16, 30000_u16) == 3932200100_u32 + 3276830000_u32 == 51501_u16
    //   3932200100_u32 == (60000_u16, 40100_u16)
    // + 3276830000_u32 == (50000_u16, 30000_u16)
    // ------------------------------------------
    //   2914062804_u32 == (44465_u16,  4564_u16)

    // d: u32 === (d_high_u16, d_low_u16)
    let (d_low_u16, d_high_u16, carry) = small_uint_carrying_add_func(c_low_u16, c_high_u16, b_low_u16, b_high_u16);
    println!("{}-{}, {}", d_high_u16, d_low_u16, carry);
    assert_eq!(d_high_u16, 44465_u16);
    assert_eq!(d_low_u16, 4564_u16);
    assert_eq!(carry, true);

    // Example for u128
    //  4201016837757989640311993609423984479246482890531986660185 == (12345678901234567890_u128, 6789012345678912345_u128)
    //+                 419908440780438063913804265570801972943493 == (                1234_u128,                6789_u128)
    //---------------------------------------------------------------------------------------------------------------------
    //  4201016837757990060220434389862048393050748461333959603678 == (12345678901234569124_u128, 6789012345678919134_u128)

    // a: u256 === (a_high_u128, a_low_u128)
    let (a_low_u128, a_high_u128, carry) = small_uint_carrying_add_func(6789012345678912345_u128, 12345678901234567890_u128, 6789_u128, 1234_u128);
    println!("{}-{}, {}", a_high_u128, a_low_u128, carry);
    assert_eq!(a_high_u128, 12345678901234569124_u128);
    assert_eq!(a_low_u128, 6789012345678919134_u128);
    assert_eq!(carry, false);

    //  308778904632843187796189293356501087608549893209439890708590319850715068122315 == (226854911280625642308916404954512140970_u128, 56789012345678912345678901234567890123_u128)
    //+  57896044618658097711785492504343953926307055644800578124155540853313808954190 == (170141183460469231731687303715884105727_u128, 12345678901234567890123456789012345678_u128)
    //--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
    //   19298681539552699237261830834781317975046994857318776714373108680289488156697 == ( 56713727820156410577229101238628035241_u128, 69134691246913480235802358023580235801_u128)

    // b: u256 === (b_high_u128, b_low_u128)
    let (b_low_u128, b_high_u128, carry) = small_uint_carrying_add_func(56789012345678912345678901234567890123_u128, 226854911280625642308916404954512140970_u128, 12345678901234567890123456789012345678_u128, 170141183460469231731687303715884105727_u128);
    println!("{}-{}, {}", b_high_u128, b_low_u128, carry);
    assert_eq!(b_high_u128, 56713727820156410577229101238628035241_u128);
    assert_eq!(b_low_u128, 69134691246913480235802358023580235801_u128);
    assert_eq!(carry, true);

    // Example for ShortUnion for Little Endian
    // a_intunion: IntUnion === (a_high_shortunion, a_low_shortunion) == (10000_u16, 10100_u16) == 25701_u16
    let a_high_shortunion = 10000_u16.into_shortunion();
    let a_low_shortunion = 10100_u16.into_shortunion();
    // b_intunion: IntUnion === (b_high_shortunion, b_low_shortunion) == (10000_u16, 20000_u16) == 25800_u16
    let b_high_shortunion = 50000_u16.into_shortunion();
    let b_low_shortunion = 30000_u16.into_shortunion();

    // (10000_u16, 10100_u16) + (50000_u16, 30000_u16) == 655370100_u32 + 655380000_u32 == 51501_u16
    //    655370100_u32 == (10000_u16, 10100_u16)
    // + 3276830000_u32 == (50000_u16, 30000_u16)
    // ------------------------------------------
    //   3932200100_u32 == (60000_u16, 40100_u16)

    // c: u32 === (c_high_shortunion, c_low_shortunion)
    let (c_low_shortunion, c_high_shortunion, carry) = small_uint_carrying_add_func(a_low_shortunion, a_high_shortunion, b_low_shortunion, b_high_shortunion);
    println!("{}-{}, {}", c_high_shortunion, c_low_shortunion, carry);
    assert_eq!(c_high_shortunion.get(), 60000_u16);
    assert_eq!(c_low_shortunion.get(), 40100_u16);
    assert_eq!(carry, false);

    // (10000_u16, 10100_u16) + (50000_u16, 30000_u16) == 3932200100_u32 + 3276830000_u32 == 51501_u16
    //   3932200100_u32 == (60000_u16, 40100_u16)
    // + 3276830000_u32 == (50000_u16, 30000_u16)
    // ------------------------------------------
    //   2914062804_u32 == (44465_u16,  4564_u16)

    // d: u32 === (d_high_shortunion, d_low_shortunion)
    let (d_low_shortunion, d_high_shortunion, carry) = small_uint_carrying_add_func(c_low_shortunion, c_high_shortunion, b_low_shortunion, b_high_shortunion);
    println!("{}-{}, {}", d_high_shortunion, d_low_shortunion, carry);
    assert_eq!(d_high_shortunion.get(), 44465_u16);
    assert_eq!(d_low_shortunion.get(), 4564_u16);
    assert_eq!(carry, true);

    // Example for LongerUnion for Little Endian
    //  4201016837757989640311993609423984479246482890531986660185 == (12345678901234567890_u128, 6789012345678912345_u128)
    //+                 419908440780438063913804265570801972943493 == (                1234_u128,                6789_u128)
    //---------------------------------------------------------------------------------------------------------------------
    //  4201016837757990060220434389862048393050748461333959603678 == (12345678901234569124_u128, 6789012345678919134_u128)

    // a: u256 === (a_high_longerunion, a_low_longerunion)
    let (a_low_longerunion, a_high_longerunion, carry) = small_uint_carrying_add_func(6789012345678912345_u128.into_longerunion(), 12345678901234567890_u128.into_longerunion(), 6789_u128.into_longerunion(), 1234_u128.into_longerunion());
    println!("{}-{}, {}", a_high_longerunion, a_low_longerunion, carry);
    assert_eq!(a_high_longerunion.get(), 12345678901234569124_u128);
    assert_eq!(a_low_longerunion.get(), 6789012345678919134_u128);
    assert_eq!(carry, false);

    //  308778904632843187796189293356501087608549893209439890708590319850715068122315 == (226854911280625642308916404954512140970_u128, 56789012345678912345678901234567890123_u128)
    //+  57896044618658097711785492504343953926307055644800578124155540853313808954190 == (170141183460469231731687303715884105727_u128, 12345678901234567890123456789012345678_u128)
    //--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
    //   19298681539552699237261830834781317975046994857318776714373108680289488156697 == ( 56713727820156410577229101238628035241_u128, 69134691246913480235802358023580235801_u128)

    // b: u256 === (b_high_longerunion, b_low_longerunion)
    let (b_low_longerunion, b_high_longerunion, carry) = small_uint_carrying_add_func(56789012345678912345678901234567890123_u128.into_longerunion(), 226854911280625642308916404954512140970_u128.into_longerunion(), 12345678901234567890123456789012345678_u128.into_longerunion(), 170141183460469231731687303715884105727_u128.into_longerunion());
    println!("{}-{}, {}", b_high_longerunion, b_low_longerunion, carry);
    assert_eq!(b_high_longerunion.get(), 56713727820156410577229101238628035241_u128);
    assert_eq!(b_low_longerunion.get(), 69134691246913480235802358023580235801_u128);
    assert_eq!(carry, true);
    println!("--------------------------------------");
}

fn small_uint_carrying_add_func<T: cryptocol::number::SmallUInt>(lhs_low: T, lhs_high: T, rhs_low: T, rhs_high: T) -> (T, T, bool)
{
    let mut carry = false;
    let sum_high: T;
    let sum_low: T;
    (sum_low, carry) = lhs_low.carrying_add(rhs_low, carry);
    (sum_high, carry) = lhs_high.carrying_add(rhs_high, carry);
    (sum_low, sum_high, carry)
}

fn small_uint_wrapping_add()
{
    println!("small_uint_wrapping_add()");
    use cryptocol::number::{ SmallUInt, ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };
    // Example for u8
    let a_u8 = small_uint_wrapping_add_func(u8::MAX - 55_u8, 55_u8);
    println!("{} + 55 = {}", u8::MAX - 55_u8, a_u8);
    assert_eq!(a_u8, u8::MAX);

    let b_u8 = small_uint_wrapping_add_func(a_u8, 1_u8);
    println!("{} + 1 = {}", a_u8, b_u8);
    assert_eq!(b_u8, 0_u8);

    // Example for u16
    let a_u16 = small_uint_wrapping_add_func(u16::MAX - 55_u16, 55_u16);
    println!("{} + 55 = {}", u16::MAX - 55_u16, a_u16);
    assert_eq!(a_u16, u16::MAX);

    let b_u16 = small_uint_wrapping_add_func(a_u16, 1_u16);
    println!("{} + 1 = {}", a_u16, b_u16);
    assert_eq!(b_u16, 0_u16);

    // Example for u32
    let a_u32 = small_uint_wrapping_add_func(u32::MAX - 55_u32, 55_u32);
    println!("{} + 55 = {}", u32::MAX - 55_u32, a_u32);
    assert_eq!(a_u32, u32::MAX);

    let b_u32 = small_uint_wrapping_add_func(a_u32, 1_u32);
    println!("{} + 1 = {}", a_u32, b_u32);
    assert_eq!(b_u32, 0_u32);

    // Example for u64
    let a_u64 = small_uint_wrapping_add_func(u64::MAX - 55_u64, 55_u64);
    println!("{} + 55 = {}", u64::MAX - 55_u64, a_u64);
    assert_eq!(a_u64, u64::MAX);

    let b_u64 = small_uint_wrapping_add_func(a_u64, 1_u64);
    println!("{} + 1 = {}", a_u64, b_u64);
    assert_eq!(b_u64, 0_u64);

    // Example for u128
    let a_u128 = small_uint_wrapping_add_func(u128::MAX - 55_u128, 55_u128);
    println!("{} + 55 = {}", u128::MAX - 55_u128, a_u128);
    assert_eq!(a_u128, u128::MAX);

    let b_u128 = small_uint_wrapping_add_func(a_u128, 1_u128);
    println!("{} + 1 = {}",a_u128, b_u128);
    assert_eq!(b_u128, 0_u128);

    // Example for usize
    let a_usize = small_uint_wrapping_add_func(usize::MAX - 55_usize, 55_usize);
    println!("{} + 55 = {}", usize::MAX - 55_usize, a_usize);
    assert_eq!(a_usize, usize::MAX);

    let b_usize = small_uint_wrapping_add_func(a_usize, 1_usize);
    println!("{} + 1 = {}", a_usize, b_usize);
    assert_eq!(b_usize, 0_usize);

    // Example for ShortUnion
    let a_shortunion = ShortUnion::new_with(u16::MAX - 55_u16);
    let b_shortunion = ShortUnion::new_with(55);
    let c_shortunion = small_uint_wrapping_add_func(a_shortunion, b_shortunion);
    println!("{} + {} = {}", a_shortunion, b_shortunion, c_shortunion);
    assert_eq!(c_shortunion.get(), u16::MAX);

    let d_shortunion = small_uint_wrapping_add_func(c_shortunion, 1_u16.into_shortunion());
    println!("{} + 1 = {}", a_shortunion, d_shortunion);
    assert_eq!(d_shortunion.get(), 0_u16);

    // Example for IntUnion
    let a_intunion = IntUnion::new_with(u32::MAX - 55_u32);
    let b_intunion = IntUnion::new_with(55);
    let c_intunion = small_uint_wrapping_add_func(a_intunion, b_intunion);
    println!("{} + {} = {}", a_intunion, b_intunion, c_intunion);
    assert_eq!(c_intunion.get(), u32::MAX);

    let d_intunion = small_uint_wrapping_add_func(c_intunion, 1_u32.into_intunion());
    println!("{} + 1 = {}", a_intunion, d_intunion);
    assert_eq!(d_intunion.get(), 0_u32);

    // Example for LongUnion
    let a_longunion = LongUnion::new_with(u64::MAX - 55_u64);
    let b_longunion = LongUnion::new_with(55);
    let c_longunion = small_uint_wrapping_add_func(a_longunion, b_longunion);
    println!("{} + {} = {}", a_longunion, b_longunion, c_longunion);
    assert_eq!(c_longunion.get(), u64::MAX);

    let d_longunion = small_uint_wrapping_add_func(c_longunion, 1_u32.into_longunion());
    println!("{} + 1 = {}", a_intunion, d_longunion);
    assert_eq!(d_longunion.get(), 0_u64);

    // Example for LongerUnion
    let a_longerunion = LongerUnion::new_with(u128::MAX - 55_u128);
    let b_longerunion = LongerUnion::new_with(55);
    let c_longerunion = small_uint_wrapping_add_func(a_longerunion, b_longerunion);
    println!("{} + {} = {}", a_longerunion, b_longerunion, c_longerunion);
    assert_eq!(c_longerunion.get(), u128::MAX);

    let d_longerunion = small_uint_wrapping_add_func(c_longerunion, 1_u128.into_longerunion());
    println!("{} + 1 = {}", a_intunion, d_longunion);
    assert_eq!(d_longerunion.get(), 0_u128);

    // Example for SizeUnion
    let a_sizeunion = SizeUnion::new_with(usize::MAX - 55_usize);
    let b_sizeunion = SizeUnion::new_with(55);
    let c_sizeunion = small_uint_wrapping_add_func(a_sizeunion, b_sizeunion);
    println!("{} + {} = {}", a_sizeunion, b_sizeunion, c_sizeunion);
    assert_eq!(c_sizeunion.get(), usize::MAX);

    let d_sizeunion = small_uint_wrapping_add_func(c_sizeunion, 1_usize.into_sizeunion());
    println!("{} + 1 = {}", a_sizeunion, d_sizeunion);
    assert_eq!(d_sizeunion.get(), 0_usize);
    println!("--------------------------------------");
}

fn small_uint_wrapping_add_func<T: cryptocol::number::SmallUInt>(lhs: T, rhs: T) -> T
{
    lhs.wrapping_add(rhs)
}

fn small_uint_overflowing_add()
{
    println!("small_uint_overflowing_add()");
    use cryptocol::number::{ SmallUInt, ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };
    // Example for u8
    let a_u8 = small_uint_overflowing_add_func(u8::MAX - 55_u8, 55_u8);
    println!("{} + 55 = {}\nOverflow = {}", u8::MAX - 55_u8, a_u8.0, a_u8.1);
    assert_eq!(a_u8.0, u8::MAX);
    assert_eq!(a_u8.1, false);
 
    let b_u8 = small_uint_overflowing_add_func(a_u8.0, 1_u8);
    println!("{} + 1 = {}\nOverflow = {}", a_u8.0, b_u8.0, b_u8.1);
    assert_eq!(b_u8.0, 0_u8);
    assert_eq!(b_u8.1, true);

    // Example for u16
    let a_u16 = small_uint_overflowing_add_func(u16::MAX - 55_u16, 55_u16);
    println!("{} + 55 = {}\nOverflow = {}", u16::MAX - 55_u16, a_u16.0, a_u16.1);
    assert_eq!(a_u16.0, u16::MAX);
    assert_eq!(a_u16.1, false);
 
    let b_u16 = small_uint_overflowing_add_func(a_u16.0, 1_u16);
    println!("{} + 1 = {}\nOverflow = {}", a_u16.0, b_u16.0, b_u16.1);
    assert_eq!(b_u16.0, 0_u16);
    assert_eq!(b_u16.1, true);

    // Example for u32
    let a_u32 = small_uint_overflowing_add_func(u32::MAX - 55_u32, 55_u32);
    println!("{} + 55 = {}\nOverflow = {}", u32::MAX - 55_u32, a_u32.0, a_u32.1);
    assert_eq!(a_u32.0, u32::MAX);
    assert_eq!(a_u32.1, false);
 
    let b_u32 = small_uint_overflowing_add_func(a_u32.0, 1_u32);
    println!("{} + 1 = {}\nOverflow = {}", a_u32.0, b_u32.0, b_u32.1);
    assert_eq!(b_u32.0, 0_u32);
    assert_eq!(b_u32.1, true);

    // Example for u64
    let a_u64 = small_uint_overflowing_add_func(u64::MAX - 55_u64, 55_u64);
    println!("{} + 55 = {}\nOverflow = {}", u64::MAX - 55_u64, a_u64.0, a_u64.1);
    assert_eq!(a_u64.0, u64::MAX);
    assert_eq!(a_u64.1, false);
 
    let b_u64 = small_uint_overflowing_add_func(a_u64.0, 1_u64);
    println!("{} + 1 = {}\nOverflow = {}", a_u64.0, b_u64.0, b_u64.1);
    assert_eq!(b_u64.0, 0_u64);
    assert_eq!(b_u64.1, true);

    // Example for u128
    let a_u128 = small_uint_overflowing_add_func(u128::MAX - 55_u128, 55_u128);
    println!("{} + 55 = {}\nOverflow = {}", u128::MAX - 55_u128, a_u128.0, a_u128.1);
    assert_eq!(a_u128.0, u128::MAX);
    assert_eq!(a_u128.1, false);
 
    let b_u128 = small_uint_overflowing_add_func(a_u128.0, 1_u128);
    println!("{} + 1 = {}\nOverflow = {}", a_u128.0, b_u128.0, b_u128.1);
    assert_eq!(b_u128.0, 0_u128);
    assert_eq!(b_u128.1, true);

    // Example for usize
    let a_usize = small_uint_overflowing_add_func(usize::MAX - 55_usize, 55_usize);
    println!("{} + 55 = {}\nOverflow = {}", usize::MAX - 55_usize, a_usize.0, a_usize.1);
    assert_eq!(a_usize.0, usize::MAX);
    assert_eq!(a_usize.1, false);
 
    let b_usize = small_uint_overflowing_add_func(a_usize.0, 1_usize);
    println!("{} + 1 = {}\nOverflow = {}", a_usize.0, b_usize.0, b_usize.1);
    assert_eq!(b_usize.0, 0_usize);
    assert_eq!(b_usize.1, true);

    // Example for ShortUnion
    let a_shortunion = (u16::MAX - 55_u16).into_shortunion();
    let (b_shortunion, mut overflow) = small_uint_overflowing_add_func(a_shortunion, 55_u16.into_shortunion());
    println!("{} + 55 = {}\nOverflow = {}", a_shortunion, b_shortunion, overflow);
    assert_eq!(b_shortunion.into_u16(), u16::MAX);
    assert_eq!(overflow, false);
 
    let c_shortunion: ShortUnion;
    (c_shortunion, overflow) = small_uint_overflowing_add_func(b_shortunion, 1_u16.into_shortunion());
    println!("{} + 1 = {}\nOverflow = {}", b_shortunion, c_shortunion, overflow);
    assert_eq!(c_shortunion.into_u16(), 0_u16);
    assert_eq!(overflow, true);

    // Example for IntUnion
    let a_intunion = (u32::MAX - 55_u32).into_intunion();
    let (b_intunion, mut overflow) = small_uint_overflowing_add_func(a_intunion, 55_u32.into_intunion());
    println!("{} + 55 = {}\nOverflow = {}", a_intunion, b_intunion, overflow);
    assert_eq!(b_intunion.into_u32(), u32::MAX);
    assert_eq!(overflow, false);
 
    let c_intunion: IntUnion;
    (c_intunion, overflow) = small_uint_overflowing_add_func(b_intunion, 1_u32.into_intunion());
    println!("{} + 1 = {}\nOverflow = {}", b_intunion, c_intunion, overflow);
    assert_eq!(c_intunion.into_u32(), 0_u32);
    assert_eq!(overflow, true);

    // Example for LongUnion
    let a_longunion = (u64::MAX - 55_u64).into_longunion();
    let (b_longunion, mut overflow) = small_uint_overflowing_add_func(a_longunion, 55_u64.into_longunion());
    println!("{} + 55 = {}\nOverflow = {}", a_longunion, b_longunion, overflow);
    assert_eq!(b_longunion.into_u64(), u64::MAX);
    assert_eq!(overflow, false);
 
    let c_longunion: LongUnion;
    (c_longunion, overflow) = small_uint_overflowing_add_func(b_longunion, 1_u64.into_longunion());
    println!("{} + 1 = {}\nOverflow = {}", b_longunion, c_longunion, overflow);
    assert_eq!(c_longunion.into_u64(), 0_u64);
    assert_eq!(overflow, true);

    // Example for LongerUnion
    let a_longerunion = (u128::MAX - 55_u128).into_longerunion();
    let (b_longerunion, mut overflow) = small_uint_overflowing_add_func(a_longerunion, 55_u128.into_longerunion());
    println!("{} + 55 = {}\nOverflow = {}", a_longerunion, b_longerunion, overflow);
    assert_eq!(b_longerunion.into_u128(), u128::MAX);
    assert_eq!(overflow, false);
 
    let c_longerunion: LongerUnion;
    (c_longerunion, overflow) = small_uint_overflowing_add_func(b_longerunion, 1_u128.into_longerunion());
    println!("{} + 1 = {}\nOverflow = {}", b_longerunion, c_longerunion, overflow);
    assert_eq!(c_longerunion.into_u128(), 0_u128);
    assert_eq!(overflow, true);

    // Example for SizeUnion
    let a_sizeunion = (usize::MAX - 55_usize).into_sizeunion();
    let (b_sizeunion, mut overflow) = small_uint_overflowing_add_func(a_sizeunion, 55_usize.into_sizeunion());
    println!("{} + 55 = {}\nOverflow = {}", a_sizeunion, b_sizeunion, overflow);
    assert_eq!(b_sizeunion.into_usize(), usize::MAX);
    assert_eq!(overflow, false);
 
    let c_sizeunion: SizeUnion;
    (c_sizeunion, overflow) = small_uint_overflowing_add_func(b_sizeunion, 1_usize.into_sizeunion());
    println!("{} + 1 = {}\nOverflow = {}", b_sizeunion, c_sizeunion, overflow);
    assert_eq!(c_sizeunion.into_usize(), 0_usize);
    assert_eq!(overflow, true);
    println!("--------------------------------------");
}

fn small_uint_overflowing_add_func<T: cryptocol::number::SmallUInt>(lhs: T, rhs: T) -> (T, bool)
{
    lhs.overflowing_add(rhs)
}

fn small_uint_checked_add()
{
    println!("small_uint_checked_add()");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = small_uint_checked_add_func(u8::MAX - 55_u8, 55_u8);
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
 
    let b_u8 = small_uint_checked_add_func(a_u8.unwrap(), 1_u8);
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

    // Example for u16
    let a_u16 = small_uint_checked_add_func(u16::MAX - 55_u16, 55_u16);
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

    let b_u16 = small_uint_checked_add_func(a_u16.unwrap(), 1_u16);
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

    // Example for u32
    let a_u32 = small_uint_checked_add_func(u32::MAX - 55_u32, 55_u32);
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

    let b_u32 = small_uint_checked_add_func(a_u32.unwrap(), 1_u32);
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

    // Example for u64
    let a_u64 = small_uint_checked_add_func(u64::MAX - 55_u64, 55_u64);
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

    let b_u64 = small_uint_checked_add_func(a_u64.unwrap(), 1_u64);
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

    // Example for u128
    let a_u128 = small_uint_checked_add_func(u128::MAX - 55_u128, 55_u128);
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

    let b_u128 = small_uint_checked_add_func(a_u128.unwrap(), 1_u128);
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

    // Example for usize
    let a_usize = small_uint_checked_add_func(usize::MAX - 55_usize, 55_usize);
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

    let b_usize = small_uint_checked_add_func(a_usize.unwrap(), 1_usize);
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

    // Example for ShortUnion
    let a_shortunion = (u16::MAX - 55_u16).into_shortunion();
    let b_shortunion = 55_u16.into_shortunion();
    let c_shortunion = small_uint_checked_add_func(a_shortunion, b_shortunion);
    match c_shortunion
    {
        Some(c) => {
                println!("{} + 55 = {}", a_shortunion, c);
                assert_eq!(c.get(), u16::MAX);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(c_shortunion, None);
            },
    }

    let d_shortunion = small_uint_checked_add_func(c_shortunion.unwrap(), 1_u16.into_shortunion());
    match d_shortunion
    {
        Some(d) => {
                println!("{} + 1 = {}", a_shortunion, d);
                assert_eq!(d.get(), 0_u16);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(d_shortunion, None);
            },
    }

    // Example for IntUnion
    let a_intunion = (u32::MAX - 55_u32).into_intunion();
    let b_intunion = 55_u32.into_intunion();
    let c_intunion = small_uint_checked_add_func(a_intunion, b_intunion);
    match c_intunion
    {
        Some(c) => {
                println!("{} + 55 = {}", a_intunion, c);
                assert_eq!(c.get(), u32::MAX);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(c_intunion, None);
            },
    }

    let d_intunion = small_uint_checked_add_func(c_intunion.unwrap(), 1_u32.into_intunion());
    match d_intunion
    {
        Some(d) => {
                println!("{} + 1 = {}", a_intunion, d);
                assert_eq!(d.get(), 0_u32);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(d_shortunion, None);
            },
    }

    // Example for LongUnion
    let a_longunion = (u64::MAX - 55_u64).into_longunion();
    let b_longunion = 55_u64.into_longunion();
    let c_longunion = small_uint_checked_add_func(a_longunion, b_longunion);
    match c_longunion
    {
        Some(c) => {
                println!("{} + 55 = {}", a_longunion, c);
                assert_eq!(c.get(), u64::MAX);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(c_intunion, None);
            },
    }

    let d_longunion = small_uint_checked_add_func(c_longunion.unwrap(), 1_u64.into_longunion());
    match d_longunion
    {
        Some(d) => {
                println!("{} + 1 = {}", a_longunion, d);
                assert_eq!(d.get(), 0_u64);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(d_longunion, None);
            },
    }

    // Example for LongerUnion
    let a_longerunion = (u128::MAX - 55_u128).into_longerunion();
    let b_longerunion = 55_u128.into_longerunion();
    let c_longerunion = small_uint_checked_add_func(a_longerunion, b_longerunion);
    match c_longerunion
    {
        Some(c) => {
                println!("{} + 55 = {}", a_longerunion, c);
                assert_eq!(c.get(), u128::MAX);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(c_intunion, None);
            },
    }

    let d_longerunion = small_uint_checked_add_func(c_longerunion.unwrap(), 1_u128.into_longerunion());
    match d_longerunion
    {
        Some(d) => {
                println!("{} + 1 = {}", a_longerunion, d);
                assert_eq!(d.get(), 0_u128);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(d_longerunion, None);
            },
    }

    // Example for SizeUnion
    let a_sizeunion = (usize::MAX - 55_usize).into_sizeunion();
    let b_sizeunion = 55_usize.into_sizeunion();
    let c_sizeunion = small_uint_checked_add_func(a_sizeunion, b_sizeunion);
    match c_sizeunion
    {
        Some(c) => {
                println!("{} + 55 = {}", a_sizeunion, c);
                assert_eq!(c.get(), usize::MAX);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(c_sizeunion, None);
            },
    }

    let d_sizeunion = small_uint_checked_add_func(c_sizeunion.unwrap(), 1_usize.into_sizeunion());
    match d_sizeunion
    {
        Some(d) => {
                println!("{} + 1 = {}", a_sizeunion, d);
                assert_eq!(d.get(), 0_usize);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(d_sizeunion, None);
            },
    }
    println!("--------------------------------------");
}

fn small_uint_checked_add_func<T: cryptocol::number::SmallUInt>(lhs: T, rhs: T) -> Option<T>
{
    lhs.checked_add(rhs)
}

fn small_uint_unchecked_add()
{
    println!("small_uint_unchecked_add()");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = small_uint_unchecked_add_func(u8::MAX - 55_u8, 55_u8);
    println!("{} + 55 = {}", u8::MAX - 55_u8, a_u8);
    assert_eq!(a_u8, u8::MAX);

    // It will panic
    // let b_u8 = small_uint_unchecked_add_func(a_u8, 1_u8);

    // Example for u16
    let a_u16 = small_uint_unchecked_add_func(u16::MAX - 55_u16, 55_u16);
    println!("{} + 55 = {}", u16::MAX - 55_u16, a_u16);
    assert_eq!(a_u16, u16::MAX);

    // It will panic
    // let b_u16 = small_uint_unchecked_add_func(a_u16, 1_u16);

    // Example for u32
    let a_u32 = small_uint_unchecked_add_func(u32::MAX - 55_u32, 55_u32);
    println!("{} + 55 = {}", u32::MAX - 55_u32, a_u32);
    assert_eq!(a_u32, u32::MAX);

    // It will panic
    // let b_u32 = small_uint_unchecked_add_func(a_u32, 1_u32);

    // Example for u64
    let a_u64 = small_uint_unchecked_add_func(u64::MAX - 55_u64, 55_u64);
    println!("{} + 55 = {}", u64::MAX - 55_u64, a_u64);
    assert_eq!(a_u64, u64::MAX);

    // It will panic
    // let b_u64 = small_uint_unchecked_add_func(a_u64, 1_u64);

    // Example for u128
    let a_u128 = small_uint_unchecked_add_func(u128::MAX - 55_u128, 55_u128);
    println!("{} + 55 = {}", u128::MAX - 55_u128, a_u128);
    assert_eq!(a_u128, u128::MAX);

    // It will panic
    // let b_u128 = small_uint_unchecked_add_func(a_u128, 1_u128);

    // Example for usize
    let a_usize = small_uint_unchecked_add_func(usize::MAX - 55_usize, 55_usize);
    println!("{} + 55 = {}", usize::MAX - 55_usize, a_usize);
    assert_eq!(a_usize, usize::MAX);

    // It will panic
    // let b_usize = small_uint_unchecked_add_func(a_usize, 1_usize);

    // Example for ShortUnion
    let a_shortunion = (u16::MAX - 55_u16).into_shortunion();
    let b_shortunion = 55_u16.into_shortunion();
    let c_shortunion = small_uint_unchecked_add_func(a_shortunion, b_shortunion);
    println!("{} + 55 = {}", a_shortunion, c_shortunion);
    assert_eq!(c_shortunion.get(), u16::MAX);

    // It will panic
    // let d_shortunion = small_uint_unchecked_add_func(c_shortunion, 1_u16);

    // Example for IntUnion
    let a_intunion = (u32::MAX - 55_u32).into_intunion();
    let b_intunion = 55_u32.into_intunion();
    let c_intunion = small_uint_unchecked_add_func(a_intunion, b_intunion);
    println!("{} + 55 = {}", a_intunion, c_intunion);
    assert_eq!(c_intunion.get(), u32::MAX);

    // It will panic
    // let d_intunion = small_uint_unchecked_add_func(c_intunion, 1_u32);

    // Example for LongUnion
    let a_longunion = (u64::MAX - 55_u64).into_longunion();
    let b_longunion = 55_u64.into_longunion();
    let c_longunion = small_uint_unchecked_add_func(a_longunion, b_longunion);
    println!("{} + 55 = {}", a_longunion, c_longunion);
    assert_eq!(c_longunion.get(), u64::MAX);

    // It will panic
    // let d_longunion = small_uint_unchecked_add_func(c_longunion, 1_u64);

    // Example for LongerUnion
    let a_longerunion = (u128::MAX - 55_u128).into_longerunion();
    let b_longerunion = 55_u128.into_longerunion();
    let c_longerunion = small_uint_unchecked_add_func(a_longerunion, b_longerunion);
    println!("{} + 55 = {}", a_longerunion, c_longerunion);
    assert_eq!(c_longerunion.get(), u128::MAX);

    // It will panic
    // let d_longerunion = small_uint_unchecked_add_func(c_longerunion, 1_u128);

    // Example for SizeUnion
    let a_sizeunion = (usize::MAX - 55_usize).into_sizeunion();
    let b_sizeunion = 55_usize.into_sizeunion();
    let c_sizeunion = small_uint_unchecked_add_func(a_sizeunion, b_sizeunion);
    println!("{} + 55 = {}", a_sizeunion, c_sizeunion);
    assert_eq!(c_sizeunion.get(), usize::MAX);

    // It will panic
    // let d_sizeunion = small_uint_unchecked_add_func(c_sizeunion, 1_usize);
    println!("--------------------------------------");
}

fn small_uint_unchecked_add_func<T: cryptocol::number::SmallUInt>(lhs: T, rhs: T) -> T
{
    lhs.unchecked_add(rhs)
}

fn small_uint_saturating_add()
{
    println!("small_uint_saturating_add()");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = small_uint_saturating_add_func(u8::MAX - 55_u8, 55_u8);
    println!("{} + 55 = {}", u8::MAX - 55_u8, a_u8);
    assert_eq!(a_u8, u8::MAX);

    let b_u8 = small_uint_saturating_add_func(a_u8, 55_u8);
    println!("{} + 55 = {}", a_u8, b_u8);
    assert_eq!(b_u8, u8::MAX);

    // Example for u16
    let a_u16 = small_uint_saturating_add_func(u16::MAX - 55_u16, 55_u16);
    println!("{} + 55 = {}", u16::MAX - 55_u16, a_u16);
    assert_eq!(a_u16, u16::MAX);

    let b_u16 = small_uint_saturating_add_func(a_u16, 55_u16);
    println!("{} + 55 = {}", a_u16, b_u16);
    assert_eq!(b_u16, u16::MAX);

    // Example for u32
    let a_u32 = small_uint_saturating_add_func(u32::MAX - 55_u32, 55_u32);
    println!("{} + 55 = {}", u32::MAX - 55_u32, a_u32);
    assert_eq!(a_u32, u32::MAX);

    let b_u32 = small_uint_saturating_add_func(a_u32, 55_u32);
    println!("{} + 55 = {}", a_u32, b_u32);
    assert_eq!(b_u32, u32::MAX);

    // Example for u64
    let a_u64 = small_uint_saturating_add_func(u64::MAX - 55_u64, 55_u64);
    println!("{} + 55 = {}", u64::MAX - 55_u64, a_u64);
    assert_eq!(a_u64, u64::MAX);

    let b_u64 = small_uint_saturating_add_func(a_u64, 55_u64);
    println!("{} + 55 = {}", a_u64, b_u64);
    assert_eq!(b_u64, u64::MAX);

    // Example for u128
    let a_u128 = small_uint_saturating_add_func(u128::MAX - 55_u128, 55_u128);
    println!("{} + 55 = {}", u128::MAX - 55_u128, a_u128);
    assert_eq!(a_u128, u128::MAX);

    let b_u128 = small_uint_saturating_add_func(a_u128, 55_u128);
    println!("{} + 55 = {}",a_u128, b_u128);
    assert_eq!(b_u128, u128::MAX);

    // Example for usize
    let a_usize = small_uint_saturating_add_func(usize::MAX - 55_usize, 55_usize);
    println!("{} + 55 = {}", usize::MAX - 55_usize, a_usize);
    assert_eq!(a_usize, usize::MAX);

    let b_usize = small_uint_saturating_add_func(a_usize, 55_usize);
    println!("{} + 55 = {}", a_usize, b_usize);
    assert_eq!(b_usize, usize::MAX);

    // Example for SortUnion
    let a_shortunion = (u16::MAX - 55_u16).into_shortunion();
    let b_shortunion = 55_u16.into_shortunion();
    let c_shortunion = small_uint_saturating_add_func(a_shortunion, b_shortunion);
    println!("{} + 55 = {}", a_shortunion, c_shortunion);
    assert_eq!(c_shortunion.get(), u16::MAX);

    let d_shortunion = small_uint_saturating_add_func(c_shortunion, b_shortunion);
    println!("{} + 55 = {}", c_shortunion, d_shortunion);
    assert_eq!(d_shortunion.get(), u16::MAX);

    // Example for IntUnion
    let a_intunion = (u32::MAX - 55_u32).into_intunion();
    let b_intunion = 55_u32.into_intunion();
    let c_intunion = small_uint_saturating_add_func(a_intunion, b_intunion);
    println!("{} + 55 = {}", a_intunion, c_intunion);
    assert_eq!(c_intunion.get(), u32::MAX);

    let d_intunion = small_uint_saturating_add_func(c_intunion, b_intunion);
    println!("{} + 55 = {}", c_intunion, d_intunion);
    assert_eq!(d_intunion.get(), u32::MAX);

    // Example for LongUnion
    let a_longunion = (u64::MAX - 55_u64).into_longunion();
    let b_longunion = 55_u64.into_longunion();
    let c_longunion = small_uint_saturating_add_func(a_longunion, b_longunion);
    println!("{} + 55 = {}", a_longunion, c_longunion);
    assert_eq!(c_longunion.get(), u64::MAX);

    let d_longunion = small_uint_saturating_add_func(c_longunion, b_longunion);
    println!("{} + 55 = {}", c_longunion, d_longunion);
    assert_eq!(d_longunion.get(), u64::MAX);

    // Example for LongerUnion
    let a_longerunion = (u128::MAX - 55_u128).into_longerunion();
    let b_longerunion = 55_u128.into_longerunion();
    let c_longerunion = small_uint_saturating_add_func(a_longerunion, b_longerunion);
    println!("{} + 55 = {}", a_longerunion, c_longerunion);
    assert_eq!(c_longerunion.get(), u128::MAX);

    let d_longerunion = small_uint_saturating_add_func(c_longerunion, b_longerunion);
    println!("{} + 55 = {}", c_longerunion, d_longerunion);
    assert_eq!(d_longerunion.get(), u128::MAX);

    // Example for SizeUnion
    let a_sizeunion = (usize::MAX - 55_usize).into_sizeunion();
    let b_sizeunion = 55_usize.into_sizeunion();
    let c_sizeunion = small_uint_saturating_add_func(a_sizeunion, b_sizeunion);
    println!("{} + 55 = {}", a_sizeunion, c_sizeunion);
    assert_eq!(c_sizeunion.get(), usize::MAX);

    let d_sizeunion = small_uint_saturating_add_func(c_sizeunion, b_sizeunion);
    println!("{} + 55 = {}", c_sizeunion, d_sizeunion);
    assert_eq!(d_sizeunion.get(), usize::MAX);
    println!("--------------------------------------");
}

fn small_uint_saturating_add_func<T: cryptocol::number::SmallUInt>(lhs: T, rhs: T) -> T
{
    lhs.saturating_add(rhs)
}

fn small_uint_modular_add()
{
    println!("small_uint_modular_add()");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = small_uint_modular_add_func(60_u8, 15, 100);
    println!("60 + 55 = {} (mod 100)", a_u8);
    assert_eq!(a_u8, 75);

    let b_u8 = small_uint_modular_add_func(a_u8, 55, 100);
    println!("{} + 55 = {} (mod 100)", a_u8, b_u8);
    assert_eq!(b_u8, 30);

    // Example for u16
    let a_u16 = small_uint_modular_add_func(6000_u16, 1500, 1_0000);
    println!("6000 + 1500 = {} (mod 1_0000)", a_u16);
    assert_eq!(a_u16, 7500);

    let b_u16 = small_uint_modular_add_func(a_u16, 5500, 1_0000);
    println!("{} + 55 = {} (mod 1_0000)", a_u16, b_u16);
    assert_eq!(b_u16, 3000);

    // Example for u32
    let a_u32 = small_uint_modular_add_func(6_0000_0000_u32, 1_5000_0000, 10_0000_0000);
    println!("6_0000_0000 + 1_5000_0000 = {} (mod 10_0000_0000)", a_u32);
    assert_eq!(a_u32, 7_5000_0000);

    let b_u32 = small_uint_modular_add_func(a_u32, 5_5000_0000, 10_0000_0000);
    println!("{} + 5_5000_0000 = {} (mod 10_0000_0000)", a_u32, b_u32);
    assert_eq!(b_u32, 3_0000_0000);

    // Example for u64
    let a_u64 = small_uint_modular_add_func(6_0000_0000_0000_0000_u64, 1_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    println!("6_0000_0000_0000_0000 + 1_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_u64);
    assert_eq!(a_u64, 7_5000_0000_0000_0000);

    let b_u64 = small_uint_modular_add_func(a_u64, 5_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    println!("{} + 5_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_u64, b_u64);
    assert_eq!(b_u64, 3_0000_0000_0000_0000);

    // Example for u128
    let a_u128 = small_uint_modular_add_func(6_0000_0000_0000_0000_0000_0000_0000_0000_u128, 1_5000_0000_0000_0000_0000_0000_0000_0000, 10_0000_0000_0000_0000_0000_0000_0000_0000);
    println!("6_0000_0000_0000_0000_0000_0000_0000_0000 + 1_5000_0000_0000_0000_0000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000_0000_0000_0000_0000)", a_u128);
    assert_eq!(a_u128, 7_5000_0000_0000_0000_0000_0000_0000_0000);

    let b_u128 = small_uint_modular_add_func(a_u128, 5_5000_0000_0000_0000_0000_0000_0000_0000, 10_0000_0000_0000_0000_0000_0000_0000_0000);
    println!("{} + 5_5000_0000_0000_0000_0000_0000_0000_0000 = {}",a_u128, b_u128);
    assert_eq!(b_u128, 3_0000_0000_0000_0000_0000_0000_0000_0000);

    // Example for usize
    let a_usize = small_uint_modular_add_func(6_0000_0000_0000_0000_usize, 1_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    println!("6_0000_0000_0000_0000 + 1_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_usize);
    assert_eq!(a_usize, 7_5000_0000_0000_0000);

    let b_usize = small_uint_modular_add_func(a_usize, 5_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    println!("{} + 5_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_usize, b_usize);
    assert_eq!(b_usize, 3_0000_0000_0000_0000);

    // Example for ShortUnion
    let a_shortunion = small_uint_modular_add_func(6000_u16.into_shortunion(), 1500_u16.into_shortunion(), 1_0000_u16.into_shortunion());
    println!("6000 + 1500 = {} (mod 1_0000)", a_shortunion);
    assert_eq!(a_shortunion.get(), 7500_u16);

    let b_shortunion = small_uint_modular_add_func(a_shortunion.into_shortunion(), 5500_u16.into_shortunion(), 1_0000_u16.into_shortunion());
    println!("{} + 55 = {} (mod 1_0000)", a_shortunion, b_shortunion);
    assert_eq!(b_shortunion.get(), 3000_u16);

    // Example for IntUnion
    let a_intunion = small_uint_modular_add_func(6_0000_0000_u32.into_intunion(), 1_5000_0000_u32.into_intunion(), 10_0000_0000_u32.into_intunion());
    println!("6_0000_0000 + 1_5000_0000 = {} (mod 10_0000_0000)", a_intunion);
    assert_eq!(a_intunion.get(), 7_5000_0000_u32);

    let b_intunion = small_uint_modular_add_func(a_intunion, 5_5000_0000_u32.into_intunion(), 10_0000_0000_u32.into_intunion());
    println!("{} + 5_5000_0000 = {} (mod 10_0000_0000)", a_intunion, b_intunion);
    assert_eq!(b_intunion.get(), 3_0000_0000_u32);

    // Example for LongUnion
    let a_longunion = small_uint_modular_add_func(6_0000_0000_0000_0000_u64.into_longunion(), 1_5000_0000_0000_0000_u64.into_longunion(), 10_0000_0000_0000_0000_u64.into_longunion());
    println!("6_0000_0000_0000_0000 + 1_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_longunion);
    assert_eq!(a_longunion.get(), 7_5000_0000_0000_0000);

    let b_longunion = small_uint_modular_add_func(a_longunion, 5_5000_0000_0000_0000_u64.into_longunion(), 10_0000_0000_0000_0000_u64.into_longunion());
    println!("{} + 5_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_longunion, b_longunion);
    assert_eq!(b_longunion, 3_0000_0000_0000_0000_u64.into_longunion());

    // Example for LongerUnion
    let a_longerunion = small_uint_modular_add_func(6_0000_0000_0000_0000_0000_0000_0000_0000_u128.into_longerunion(), 1_5000_0000_0000_0000_0000_0000_0000_0000_u128.into_longerunion(), 10_0000_0000_0000_0000_0000_0000_0000_0000_u128.into_longerunion());
    println!("6_0000_0000_0000_0000_0000_0000_0000_0000 + 1_5000_0000_0000_0000_0000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000_0000_0000_0000_0000)", a_longerunion);
    assert_eq!(a_longerunion.get(), 7_5000_0000_0000_0000_0000_0000_0000_0000_u128);

    let b_longerunion = small_uint_modular_add_func(a_longerunion, 5_5000_0000_0000_0000_0000_0000_0000_0000_u128.into_longerunion(), 10_0000_0000_0000_0000_0000_0000_0000_0000_u128.into_longerunion());
    println!("{} + 5_5000_0000_0000_0000_0000_0000_0000_0000 = {}",a_longerunion, b_longerunion);
    assert_eq!(b_longerunion.get(), 3_0000_0000_0000_0000_0000_0000_0000_0000_u128);

    // Example for SizeUnion
    let a_sizeunion = small_uint_modular_add_func(6_0000_0000_0000_0000_usize.into_sizeunion(), 1_5000_0000_0000_0000_usize.into_sizeunion(), 10_0000_0000_0000_0000_usize.into_sizeunion());
    println!("6_0000_0000_0000_0000 + 1_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_sizeunion);
    assert_eq!(a_sizeunion.get(), 7_5000_0000_0000_0000_usize);

    let b_sizeunion = small_uint_modular_add_func(a_sizeunion, 5_5000_0000_0000_0000_usize.into_sizeunion(), 10_0000_0000_0000_0000_usize.into_sizeunion());
    println!("{} + 5_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_sizeunion, b_sizeunion);
    assert_eq!(b_sizeunion.get(), 3_0000_0000_0000_0000_usize);
    println!("--------------------------------------");
}

fn small_uint_modular_add_func<T: cryptocol::number::SmallUInt>(lhs: T, rhs: T, modulo: T) -> T
{
    lhs.modular_add(rhs, modulo)
}

fn small_uint_sub_main()
{
    small_uint_borrowing_sub();
    small_uint_wrapping_sub();
    small_uint_overflowing_sub();
    small_uint_checked_sub();
    small_uint_unchecked_sub();
    small_uint_saturating_sub();
    small_uint_abs_diff();
    small_uint_modular_sub();
}

fn small_uint_borrowing_sub()
{
    println!("small_uint_borrowing_sub()");
    use cryptocol::number::SmallUInt;
    // Example for u8
    // a_u16: u16 === (a_high_u8, a_low_u8) == (100_u8, 200_u8) == 25800_u16
    let a_high_u8 = 100_u8;
    let a_low_u8 = 200_u8;
    // b_u16: u16 === (b_high_u8, b_low_u8) == (100_u8, 101_u8) == 25701_u16
    let b_high_u8 = 100_u8;
    let b_low_u8 = 101_u8;

    // (100_u8, 200_u8) - (100_u8, 101_u8) == 25800_u16 - 25701_u16 == 99_u16
    //   25800_u16 == (100_u8, 200_u8)
    // - 25701_u16 == (100_u8, 101_u8)
    // -------------------------------
    //      99_u16 == (  0_u8,  99_u8)

    // c_u16: u16 === (c_high_u8, c_low_u8)
    let (c_low_u8, c_high_u8, borrow) = small_uint_borrowing_sub_func(a_low_u8, a_high_u8, b_low_u8, b_high_u8);
    println!("{}-{}, {}", c_high_u8, c_low_u8, borrow);
    assert_eq!(c_high_u8, 0_u8);
    assert_eq!(c_low_u8, 99_u8);
    assert_eq!(borrow, false);

    //  (  0_u8,  99_u8) - (100_u8, 101_u8) == 99_u16 - 25701_u16 == 51501_u16
    //      99_u16 == (  0_u8,  99_u8)
    // - 25701_u16 == (100_u8, 101_u8)
    // -------------------------------
    //   39934_u16 == (155_u8, 254_u8)

    // d_u16: u16 === (d_high_u8, d_low_u8)
    let (d_low_u8, d_high_u8, borrow) = small_uint_borrowing_sub_func(c_low_u8, c_high_u8, b_low_u8, b_high_u8);
    println!("{}-{}, {}", d_high_u8, d_low_u8, borrow);
    assert_eq!(d_high_u8, 155_u8);
    assert_eq!(d_low_u8, 254_u8);
    assert_eq!(borrow, true);

    // Example for u16
    // a_u32: u32 === (a_high_u16, a_low_u16) == (50000_u16, 30000_u16) == 3276830000_u32
    let a_high_u16 = 50000_u16;
    let a_low_u16 = 30000_u16;
    // b_u32: u32 === (b_high_u16, b_low_u16) == (10000_u16, 10100_u16) == 655370100_u32
    let b_high_u16 = 10000_u16;
    let b_low_u16 = 10100_u16;

    // (50000_u16, 30000_u16) - (10000_u16, 10100_u16) == 3276830000_u32 - 655370100_u32 == 99_u16
    //   3276830000_u32 == (50000_u16, 30000_u16)
    // -  655370100_u32 == (10000_u16, 10100_u16)
    // ------------------------------------------
    //   2621459900_u32 == (40000_u16, 19900_u16)

    // c: u32 === (c_high_u16, c_low_u16)
    let (c_low_u16, c_high_u16, borrow) = small_uint_borrowing_sub_func(a_low_u16, a_high_u16, b_low_u16, b_high_u16);
    println!("{}-{}, {}", c_high_u16, c_low_u16, borrow);
    assert_eq!(c_high_u16, 40000_u16);
    assert_eq!(c_low_u16, 19900_u16);
    assert_eq!(borrow, false);

    // (10000_u16, 10100_u16) - (50000_u16, 30000_u16) == 655370100_u32 - 3276830000_u32 == 51501_u16
    //    655370100_u32 == (10000_u16, 10100_u16)
    // - 3276830000_u32 == (50000_u16, 30000_u16)
    // ------------------------------------------
    //   1673507396_u32 == (25535_u16, 45636_u16)

    // d: u32 === (d_high_u16, d_low_u16)
    let (d_low_u16, d_high_u16, carry) = small_uint_borrowing_sub_func(b_low_u16, b_high_u16, a_low_u16, a_high_u16);
    println!("{}-{}, {}", d_high_u16, d_low_u16, carry);
    println!("{}", 655370100_u32 - 3276830000_u32);
    assert_eq!(d_high_u16, 25535_u16);
    assert_eq!(d_low_u16, 45636_u16);
    assert_eq!(carry, true);

    // Example for u128
    //   4201016837757989640311993609423984479246482890531986660185 == (12345678901234567890_u128, 6789012345678912345_u128)
    // -                 419908440780438063913804265570801972943493 == (                1234_u128,                6789_u128)
    // ---------------------------------------------------------------------------------------------------------------------
    //   4201016837757989220403552828985920565442217319730013716692 == (12345678901234566656_u128, 6789012345678905556_u128)

    // a_u256: u256 === (a_high_u128, a_low_u1288)
    let (a_low_u128, a_high_u128, borrow) = small_uint_borrowing_sub_func(6789012345678912345_u128, 12345678901234567890_u128, 6789_u128, 1234_u128);
    println!("{}-{}, {}", a_high_u128, a_low_u128, borrow);
    assert_eq!(a_high_u128, 12345678901234566656_u128);
    assert_eq!(a_low_u128, 6789012345678905556_u128);
    assert_eq!(borrow, false);

    //    57896044618658097711785492504343953926307055644800578124155540853313808954190 == (170141183460469231731687303715884105727_u128,  12345678901234567890123456789012345678_u128)
    // - 308778904632843187796189293356501087608549893209439890708590319850715068122315 == (226854911280625642308916404954512140970_u128,  56789012345678912345678901234567890123_u128)
    // --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
    //   328077586172395887033451124191282405584107085763563507612853141042164389031555 == (283568639100782052886145506193140176212_u128, 295839033476494119007819162986212667011_u128)

    // b_u256: u256 === (b_high_u128, b_low_u128)
    let (b_low_u128, b_high_u128, borrow) = small_uint_borrowing_sub_func(12345678901234567890123456789012345678_u128, 170141183460469231731687303715884105727_u128, 56789012345678912345678901234567890123_u128, 226854911280625642308916404954512140970_u128);
    println!("{}-{}, {}", b_high_u128, b_low_u128, borrow);
    assert_eq!(b_high_u128, 283568639100782052886145506193140176212_u128);
    assert_eq!(b_low_u128, 295839033476494119007819162986212667011_u128);
    assert_eq!(borrow, true);

    // Example for IntUnion
    // a_u32: u32 === (a_high_u16, a_low_u16) == (50000_u16, 30000_u16) == 3276830000_u32
    let a_high_shortunion = 50000_u16.into_shortunion();
    let a_low_shortunion = 30000_u16.into_shortunion();
    // b_u32: u32 === (b_high_u16, b_low_u16) == (10000_u16, 10100_u16) == 655370100_u32
    let b_high_shortunion = 10000_u16.into_shortunion();
    let b_low_shortunion = 10100_u16.into_shortunion();

    // (50000_u16, 30000_u16) - (10000_u16, 10100_u16) == 3276830000_u32 - 655370100_u32 == 99_u16
    //   3276830000_u32 == (50000_u16, 30000_u16)
    // -  655370100_u32 == (10000_u16, 10100_u16)
    // ------------------------------------------
    //   2621459900_u32 == (40000_u16, 19900_u16)

    // c: u32 === (c_high_intunion, c_low_intunion)
    let (c_low_shortunion, c_high_shortunion, borrow) = small_uint_borrowing_sub_func(a_low_shortunion, a_high_shortunion, b_low_shortunion, b_high_shortunion);
    println!("{}-{}, {}", c_high_shortunion, c_low_shortunion, borrow);
    assert_eq!(c_high_shortunion.get(), 40000_u16);
    assert_eq!(c_low_shortunion.get(), 19900_u16);
    assert_eq!(borrow, false);

    // (10000_u16, 10100_u16) - (50000_u16, 30000_u16) == 655370100_u32 - 3276830000_u32 == 51501_u16
    //    655370100_u32 == (10000_u16, 10100_u16)
    // - 3276830000_u32 == (50000_u16, 30000_u16)
    // ------------------------------------------
    //   1673507396_u32 == (25535_u16, 45636_u16)

    // d: u32 === (d_high_u16, d_low_shortunion)
    let (d_low_shortunion, d_high_shortunion, carry) = small_uint_borrowing_sub_func(b_low_shortunion, b_high_shortunion, a_low_shortunion, a_high_shortunion);
    println!("{}-{}, {}", d_low_shortunion, d_low_shortunion, carry);
    println!("{}", 655370100_u32 - 3276830000_u32);
    assert_eq!(d_high_shortunion.get(), 25535_u16);
    assert_eq!(d_low_shortunion.get(), 45636_u16);
    assert_eq!(carry, true);

    // Example for LongerUnion
    //   4201016837757989640311993609423984479246482890531986660185 == (12345678901234567890_u128, 6789012345678912345_u128)
    // -                 419908440780438063913804265570801972943493 == (                1234_u128,                6789_u128)
    // ---------------------------------------------------------------------------------------------------------------------
    //   4201016837757989220403552828985920565442217319730013716692 == (12345678901234566656_u128, 6789012345678905556_u128)

    // a_u256: u256 === (a_high_longerunion, a_low_longerunion)
    let (a_low_longerunion, a_high_longerunion, borrow) = small_uint_borrowing_sub_func(6789012345678912345_u128.into_longerunion(), 12345678901234567890_u128.into_longerunion(), 6789_u128.into_longerunion(), 1234_u128.into_longerunion());
    println!("{}-{}, {}", a_low_longerunion, a_high_longerunion, borrow);
    assert_eq!(a_high_longerunion.get(), 12345678901234566656_u128);
    assert_eq!(a_low_longerunion.get(), 6789012345678905556_u128);
    assert_eq!(borrow, false);

    //    57896044618658097711785492504343953926307055644800578124155540853313808954190 == (170141183460469231731687303715884105727_u128,  12345678901234567890123456789012345678_u128)
    // - 308778904632843187796189293356501087608549893209439890708590319850715068122315 == (226854911280625642308916404954512140970_u128,  56789012345678912345678901234567890123_u128)
    // --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
    //   328077586172395887033451124191282405584107085763563507612853141042164389031555 == (283568639100782052886145506193140176212_u128, 295839033476494119007819162986212667011_u128)

    // b_u256: u256 === (b_high_longerunion, b_low_longerunion)
    let (b_low_longerunion, b_high_longerunion, borrow) = small_uint_borrowing_sub_func(12345678901234567890123456789012345678_u128.into_longerunion(), 170141183460469231731687303715884105727_u128.into_longerunion(), 56789012345678912345678901234567890123_u128.into_longerunion(), 226854911280625642308916404954512140970_u128.into_longerunion());
    println!("{}-{}, {}", b_high_longerunion, b_low_longerunion, borrow);
    assert_eq!(b_high_longerunion.get(), 283568639100782052886145506193140176212_u128);
    assert_eq!(b_low_longerunion.get(), 295839033476494119007819162986212667011_u128);
    assert_eq!(borrow, true);
    println!("--------------------------------------");
}

fn small_uint_borrowing_sub_func<T: cryptocol::number::SmallUInt>(lhs_low: T, lhs_high: T, rhs_low: T, rhs_high: T) -> (T, T, bool)
{
    let mut borrow = false;
    let dif_high: T;
    let dif_low: T;
    (dif_low, borrow) = lhs_low.borrowing_sub(rhs_low, borrow);
    (dif_high, borrow) = lhs_high.borrowing_sub(rhs_high, borrow);
    (dif_low, dif_high, borrow)
}

fn small_uint_wrapping_sub()
{
    println!("small_uint_wrapping_sub()");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = small_uint_wrapping_sub_func(55_u8, 55_u8);
    println!("55 - 55 = {}", a_u8);
    assert_eq!(a_u8, 0_u8);

    let b_u8 = small_uint_wrapping_sub_func(a_u8, 1_u8);
    println!("{} - 1 = {}", a_u8, b_u8);
    assert_eq!(b_u8, u8::MAX);

    // Example for u16
    let a_u16 = small_uint_wrapping_sub_func(55_u16, 55_u16);
    println!("55 - 55 = {}", a_u16);
    assert_eq!(a_u16, 0_u16);

    let b_u16 = small_uint_wrapping_sub_func(a_u16, 1_u16);
    println!("{} - 1 = {}", a_u16, b_u16);
    assert_eq!(b_u16, u16::MAX);

    // Example for u32
    let a_u32 = small_uint_wrapping_sub_func(55_u32, 55_u32);
    println!("55 - 55 = {}", a_u32);
    assert_eq!(a_u32, 0_u32);

    let b_u32 = small_uint_wrapping_sub_func(a_u32, 1_u32);
    println!("{} - 1 = {}", a_u32, b_u32);
    assert_eq!(b_u32, u32::MAX);

    // Example for u64
    let a_u64 = small_uint_wrapping_sub_func(55_u64, 55_u64);
    println!("55 - 55 = {}", a_u64);
    assert_eq!(a_u64, 0_u64);

    let b_u64 = small_uint_wrapping_sub_func(a_u64, 1_u64);
    println!("{} - 1 = {}", a_u64, b_u64);
    assert_eq!(b_u64, u64::MAX);

    // Example for u128
    let a_u128 = small_uint_wrapping_sub_func(55_u128, 55_u128);
    println!("55 - 55 = {}", a_u128);
    assert_eq!(a_u128, 0_u128);

    let b_u128 = small_uint_wrapping_sub_func(a_u128, 1_u128);
    println!("{} - 1 = {}",a_u128, b_u128);
    assert_eq!(b_u128, u128::MAX);

    // Example for usize
    let a_usize = small_uint_wrapping_sub_func(55_usize, 55_usize);
    println!("55 - 55 = {}", a_usize);
    assert_eq!(a_usize, 0_usize);

    let b_usize = small_uint_wrapping_sub_func(a_usize, 1_usize);
    println!("{} - 1 = {}", a_usize, b_usize);
    assert_eq!(b_usize, usize::MAX);

    // Example for ShortUnion
    let a_shortunion = small_uint_wrapping_sub_func(55_u16.into_shortunion(), 55_u16.into_shortunion());
    println!("55 - 55 = {}", a_shortunion);
    assert_eq!(a_shortunion.get(), 0_u16);

    let b_shortunion = small_uint_wrapping_sub_func(a_shortunion, 1_u16.into_shortunion());
    println!("{} - 1 = {}", a_shortunion, b_shortunion);
    assert_eq!(b_shortunion.get(), u16::MAX);

    // Example for IntUnion
    let a_intunion = small_uint_wrapping_sub_func(55_u32.into_intunion(), 55_u32.into_intunion());
    println!("55 - 55 = {}", a_intunion);
    assert_eq!(a_intunion.get(), 0_u32);

    let b_intunion = small_uint_wrapping_sub_func(a_intunion, 1_u32.into_intunion());
    println!("{} - 1 = {}", a_intunion, b_intunion);
    assert_eq!(b_intunion.get(), u32::MAX);

    // Example for LongUnion
    let a_longunion = small_uint_wrapping_sub_func(55_u64.into_longunion(), 55_u64.into_longunion());
    println!("55 - 55 = {}", a_longunion);
    assert_eq!(a_longunion.get(), 0_u64);

    let b_longunion = small_uint_wrapping_sub_func(a_longunion, 1_u64.into_longunion());
    println!("{} - 1 = {}", a_longunion, b_longunion);
    assert_eq!(b_longunion.get(), u64::MAX);

    // Example for LongerUnion
    let a_longerunion = small_uint_wrapping_sub_func(55_u128.into_longerunion(), 55_u128.into_longerunion());
    println!("55 - 55 = {}", a_longerunion);
    assert_eq!(a_longerunion.get(), 0_u128);

    let b_longerunion = small_uint_wrapping_sub_func(a_longerunion, 1_u128.into_longerunion());
    println!("{} - 1 = {}", a_longerunion, b_longerunion);
    assert_eq!(b_longerunion.get(), u128::MAX);

    // Example for SizeUnion
    let a_sizeunion = small_uint_wrapping_sub_func(55_usize.into_sizeunion(), 55_usize.into_sizeunion());
    println!("55 - 55 = {}", a_sizeunion);
    assert_eq!(a_sizeunion.get(), 0_usize);

    let b_sizeunion = small_uint_wrapping_sub_func(a_sizeunion, 1_usize.into_sizeunion());
    println!("{} - 1 = {}", a_sizeunion, b_sizeunion);
    assert_eq!(b_sizeunion.get(), usize::MAX);
    println!("--------------------------------------");
}

fn small_uint_wrapping_sub_func<T: cryptocol::number::SmallUInt>(lhs: T, rhs: T) -> T
{
    lhs.wrapping_sub(rhs)
}

fn small_uint_overflowing_sub()
{
    println!("small_uint_overflowing_sub()");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = small_uint_overflowing_sub_func(55_u8, 55_u8);
    println!("55 - 55 = {}\nUnderflow = {}", a_u8.0, a_u8.1);
    assert_eq!(a_u8.0, 0_u8);
    assert_eq!(a_u8.1, false);
 
    let b_u8 = small_uint_overflowing_sub_func(a_u8.0, 1_u8);
    println!("{} - 1 = {}\nUnderflow = {}", a_u8.0, b_u8.0, b_u8.1);
    assert_eq!(b_u8.0, u8::MAX);
    assert_eq!(b_u8.1, true);

    // Example for u16
    let a_u16 = small_uint_overflowing_sub_func(55_u16, 55_u16);
    println!("55 - 55 = {}\nUnderflow = {}", a_u16.0, a_u16.1);
    assert_eq!(a_u16.0, 0_u16);
    assert_eq!(a_u16.1, false);
 
    let b_u16 = small_uint_overflowing_sub_func(a_u16.0, 1_u16);
    println!("{} - 1 = {}\nUnderflow = {}", a_u16.0, b_u16.0, b_u16.1);
    assert_eq!(b_u16.0, u16::MAX);
    assert_eq!(b_u16.1, true);

    // Example for u32
    let a_u32 = small_uint_overflowing_sub_func(55_u32, 55_u32);
    println!("55 - 55 = {}\nUnderflow = {}", a_u32.0, a_u32.1);
    assert_eq!(a_u32.0, 0_u32);
    assert_eq!(a_u32.1, false);
 
    let b_u32 = small_uint_overflowing_sub_func(a_u32.0, 1_u32);
    println!("{} - 1 = {}\nUnderflow = {}", a_u32.0, b_u32.0, b_u32.1);
    assert_eq!(b_u32.0, u32::MAX);
    assert_eq!(b_u32.1, true);

    // Example for u64
    let a_u64 = small_uint_overflowing_sub_func(55_u64, 55_u64);
    println!("55 - 55 = {}\nUnderflow = {}", a_u64.0, a_u64.1);
    assert_eq!(a_u64.0, 0_u64);
    assert_eq!(a_u64.1, false);
 
    let b_u64 = small_uint_overflowing_sub_func(a_u64.0, 1_u64);
    println!("{} - 1 = {}\nUnderflow = {}", a_u64.0, b_u64.0, b_u64.1);
    assert_eq!(b_u64.0, u64::MAX);
    assert_eq!(b_u64.1, true);

    // Example for u128
    let a_u128 = small_uint_overflowing_sub_func(55_u128, 55_u128);
    println!("55 - 55 = {}\nUnderflow = {}", a_u128.0, a_u128.1);
    assert_eq!(a_u128.0, 0_u128);
    assert_eq!(a_u128.1, false);
 
    let b_u128 = small_uint_overflowing_sub_func(a_u128.0, 1_u128);
    println!("{} - 1 = {}\nUnderflow = {}", a_u128.0, b_u128.0, b_u128.1);
    assert_eq!(b_u128.0, u128::MAX);
    assert_eq!(b_u128.1, true);

    // Example for usize
    let a_usize = small_uint_overflowing_sub_func(55_usize, 55_usize);
    println!("55 - 55 = {}\nUnderflow = {}", a_usize.0, a_usize.1);
    assert_eq!(a_usize.0, 0_usize);
    assert_eq!(a_usize.1, false);
 
    let b_usize = small_uint_overflowing_sub_func(a_usize.0, 1_usize);
    println!("{} - 1 = {}\nUnderflow = {}", a_usize.0, b_usize.0, b_usize.1);
    assert_eq!(b_usize.0, usize::MAX);
    assert_eq!(b_usize.1, true);

    // Example for ShortUnion
    let (a_shortunion, overflow) = small_uint_overflowing_sub_func(55_u16.into_shortunion(), 55_u16.into_shortunion());
    println!("55 - 55 = {}\nUnderflow = {}", a_shortunion, overflow);
    assert_eq!(a_shortunion.get(), 0_u16);
    assert_eq!(overflow, false);
 
    let (b_shortunion, overflow) = small_uint_overflowing_sub_func(a_shortunion, 1_u16.into_shortunion());
    println!("{} - 1 = {}\nUnderflow = {}", b_shortunion, b_shortunion, overflow);
    assert_eq!(b_shortunion.get(), u16::MAX);
    assert_eq!(overflow, true);

    // Example for IntUnion
    let (a_intunion, overflow) = small_uint_overflowing_sub_func(55_u32.into_intunion(), 55_u32.into_intunion());
    println!("55 - 55 = {}\nUnderflow = {}", a_intunion, overflow);
    assert_eq!(a_intunion.get(), 0_u32);
    assert_eq!(overflow, false);
 
    let (b_intunion, overflow) = small_uint_overflowing_sub_func(a_intunion, 1_u32.into_intunion());
    println!("{} - 1 = {}\nUnderflow = {}", a_intunion, b_intunion, overflow);
    assert_eq!(b_intunion.get(), u32::MAX);
    assert_eq!(overflow, true);

    // Example for LongUnion
    let (a_longunion, overflow) = small_uint_overflowing_sub_func(55_u64.into_longunion(), 55_u64.into_longunion());
    println!("55 - 55 = {}\nUnderflow = {}", a_longunion, overflow);
    assert_eq!(a_longunion.get(), 0_u64);
    assert_eq!(overflow, false);
 
    let (b_longunion, overflow) = small_uint_overflowing_sub_func(a_longunion, 1_u64.into_longunion());
    println!("{} - 1 = {}\nUnderflow = {}", a_longunion, b_longunion, overflow);
    assert_eq!(b_longunion.get(), u64::MAX);
    assert_eq!(overflow, true);

    // Example for LongerUnion
    let (a_longerunion, overflow) = small_uint_overflowing_sub_func(55_u128.into_longerunion(), 55_u128.into_longerunion());
    println!("55 - 55 = {}\nUnderflow = {}", a_longerunion, a_longerunion);
    assert_eq!(a_longerunion.get(), 0_u128);
    assert_eq!(overflow, false);
 
    let (b_longerunion, overflow) = small_uint_overflowing_sub_func(a_longerunion, 1_u128.into_longerunion());
    println!("{} - 1 = {}\nUnderflow = {}", a_longerunion, b_longerunion, overflow);
    assert_eq!(b_longerunion.get(), u128::MAX);
    assert_eq!(overflow, true);

    // Example for SizeUnion
    let (a_sizeunion, overflow) = small_uint_overflowing_sub_func(55_usize.into_sizeunion(), 55_usize.into_sizeunion());
    println!("55 - 55 = {}\nUnderflow = {}", a_sizeunion, overflow);
    assert_eq!(a_sizeunion.get(), 0_usize);
    assert_eq!(overflow, false);
 
    let (b_sizeunion, overflow) = small_uint_overflowing_sub_func(a_sizeunion, 1_usize.into_sizeunion());
    println!("{} - 1 = {}\nUnderflow = {}", a_sizeunion, b_sizeunion, overflow);
    assert_eq!(b_sizeunion.get(), usize::MAX);
    assert_eq!(overflow, true);
    println!("--------------------------------------");
}

fn small_uint_overflowing_sub_func<T: cryptocol::number::SmallUInt>(lhs: T, rhs: T) -> (T, bool)
{
    lhs.overflowing_sub(rhs)
}

fn small_uint_checked_sub()
{
    println!("small_uint_checked_sub()");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = small_uint_checked_sub_func(55_u8, 55_u8);
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
 
    let b_u8 = small_uint_checked_sub_func(a_u8.unwrap(), 1_u8);
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

    // Example for u16
    let a_u16 = small_uint_checked_sub_func(55_u16, 55_u16);
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

    let b_u16 = small_uint_checked_sub_func(a_u16.unwrap(), 1_u16);
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

    // Example for u32
    let a_u32 = small_uint_checked_sub_func(55_u32, 55_u32);
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

    let b_u32 = small_uint_checked_sub_func(a_u32.unwrap(), 1_u32);
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

    // Example for u64
    let a_u64 = small_uint_checked_sub_func(55_u64, 55_u64);
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

    let b_u64 = small_uint_checked_sub_func(a_u64.unwrap(), 1_u64);
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

    // Example for u128
    let a_u128 = small_uint_checked_sub_func(55_u128, 55_u128);
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

    let b_u128 = small_uint_checked_sub_func(a_u128.unwrap(), 1_u128);
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

    // Example for usize
    let a_usize = small_uint_checked_sub_func(55_usize, 55_usize);
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

    let b_usize = small_uint_checked_sub_func(a_usize.unwrap(), 1_usize);
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

    // Example for ShortUnion
    let a_shortunion = small_uint_checked_sub_func(55_u16.into_shortunion(), 55_u16.into_shortunion());
    match a_shortunion
    {
        Some(a) => {
                println!("55 - 55 = {}", a);
                assert_eq!(a.get(), 0_u16);
            },
        None => {
                println!("Underflow happened.");
                assert_eq!(a_shortunion, None);
            },
    }

    let b_shortunion = small_uint_checked_sub_func(a_shortunion.unwrap(), 1_u16.into_shortunion());
    match b_shortunion
    {
        Some(b) => {
                println!("{} - 1 = {}", a_shortunion.unwrap(), b);
                assert_eq!(b.get(), u16::MAX);
            },
        None => {
                println!("Underflow happened.");
                assert_eq!(b_shortunion, None);
            },
    }

    // Example for IntUnion
    let a_intunion = small_uint_checked_sub_func(55_u32.into_intunion(), 55_u32.into_intunion());
    match a_intunion
    {
        Some(a) => {
                println!("55 - 55 = {}", a);
                assert_eq!(a.get(), 0_u32);
            },
        None => {
                println!("Underflow happened.");
                assert_eq!(a_intunion, None);
            },
    }

    let b_intunion = small_uint_checked_sub_func(a_intunion.unwrap(), 1_u32.into_intunion());
    match b_intunion
    {
        Some(b) => {
                println!("{} - 1 = {}", a_u32.unwrap(), b);
                assert_eq!(b.get(), u32::MAX);
            },
        None => {
                println!("Underflow happened.");
                assert_eq!(b_intunion, None);
            },
    }

    // Example for LongUnion
    let a_longunion = small_uint_checked_sub_func(55_u64.into_longunion(), 55_u64.into_longunion());
    match a_longunion
    {
        Some(a) => {
                println!("55 - 55 = {}", a);
                assert_eq!(a.get(), 0_u64);
            },
        None => {
                println!("Underflow happened.");
                assert_eq!(a_longunion, None);
            },
    }

    let b_longunion = small_uint_checked_sub_func(a_longunion.unwrap(), 1_u64.into_longunion());
    match b_longunion
    {
        Some(b) => {
                println!("{} + 1 = {}", a_longunion.unwrap(), b);
                assert_eq!(b.get(), u64::MAX);
            },
        None => {
                println!("Underflow happened.");
                assert_eq!(b_longunion, None);
            },
    }

    // Example for LongerUnion
    let a_longerunion = small_uint_checked_sub_func(55_u128.into_longerunion(), 55_u128.into_longerunion());
    match a_longerunion
    {
        Some(a) => {
                println!("55 - 55 = {}", a);
                assert_eq!(a.get(), 0_u128);
            },
        None => {
                println!("Underflow happened.");
                assert_eq!(a_longerunion, None);
            },
    }

    let b_longerunion = small_uint_checked_sub_func(a_longerunion.unwrap(), 1_u128.into_longerunion());
    match b_longerunion
    {
        Some(b) => {
                println!("{} - 1 = {}", a_longerunion.unwrap(), b);
                assert_eq!(b.get(), u128::MAX);
            },
        None => {
                println!("Underflow happened.");
                assert_eq!(b_longerunion, None);
            },
    }

    // Example for SizeUnion
    let a_sizeunion = small_uint_checked_sub_func(55_usize.into_sizeunion(), 55_usize.into_sizeunion());
    match a_sizeunion
    {
        Some(a) => {
                println!("55 - 55 = {}", a);
                assert_eq!(a.get(), 0_usize);
            },
        None => {
                println!("Underflow happened.");
                assert_eq!(a_sizeunion, None);
            },
    }

    let b_sizeunion = small_uint_checked_sub_func(a_sizeunion.unwrap(), 1_usize.into_sizeunion());
    match b_sizeunion
    {
        Some(b) => {
                println!("{} - 1 = {}", a_sizeunion.unwrap(), b);
                assert_eq!(b.get(), usize::MAX);
            },
        None => {
                println!("Underflow happened.");
                assert_eq!(b_sizeunion, None);
            },
    }
    println!("--------------------------------------");
}

fn small_uint_checked_sub_func<T: cryptocol::number::SmallUInt>(lhs: T, rhs: T) -> Option<T>
{
    lhs.checked_sub(rhs)
}

fn small_uint_unchecked_sub()
{
    println!("small_uint_unchecked_sub()");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = small_uint_unchecked_sub_func(55_u8, 55_u8);
    println!("55 - 55 = {}", a_u8);
    assert_eq!(a_u8, 0_u8);

    // It will panic
    // let b_u8 = small_uint_unchecked_sub_func(a_u8, 1_u8);

    // Example for u16
    let a_u16 = small_uint_unchecked_sub_func(55_u16, 55_u16);
    println!("55 - 55 = {}", a_u16);
    assert_eq!(a_u16, 0_u16);

    // It will panic
    // let b_u16 = small_uint_unchecked_sub_func(a_u16, 1_u16);

    // Example for u32
    let a_u32 = small_uint_unchecked_sub_func(55_u32, 55_u32);
    println!("55 - 55 = {}", a_u32);
    assert_eq!(a_u32, 0_u32);

    // It will panic
    // let b_u32 = small_uint_unchecked_sub_func(a_u32, 1_u32);

    // Example for u64
    let a_u64 = small_uint_unchecked_sub_func(55_u64, 55_u64);
    println!("55 - 55 = {}", a_u64);
    assert_eq!(a_u64, 0_u64);

    // It will panic
    // let b_u64 = small_uint_unchecked_sub_func(a_u64, 1_u64);

    // Example for u128
    let a_u128 = small_uint_unchecked_sub_func(55_u128, 55_u128);
    println!("55 - 55 = {}", a_u128);
    assert_eq!(a_u128, 0_u128);

    // It will panic
    // let b_u128 = small_uint_unchecked_sub_func(a_u128, 1_u128);

    // Example for usize
    let a_usize = small_uint_unchecked_sub_func(55_usize, 55_usize);
    println!("55 - 55 = {}", a_usize);
    assert_eq!(a_usize, 0_usize);

    // It will panic
    // let b_usize = small_uint_unchecked_sub_func(a_usize, 1_usize);

    // Example for ShortUnion
    let a_ushortunion = small_uint_unchecked_sub_func(55_u16.into_shortunion(), 55_u16.into_shortunion());
    println!("55 - 55 = {}", a_ushortunion);
    assert_eq!(a_ushortunion.get(), 0_u16);

    // It will panic
    // let b_ushortunion = small_uint_unchecked_sub_func(a_ushortunion, 1_u16);

    // Example for IntUnion
    let a_intunion = small_uint_unchecked_sub_func(55_u32.into_intunion(), 55_u32.into_intunion());
    println!("55 - 55 = {}", a_intunion);
    assert_eq!(a_intunion.get(), 0_u32);

    // It will panic
    // let b_intunion = small_uint_unchecked_sub_func(a_intunion, 1_u32.into_intunion());

    // Example for LongUnion
    let a_longunion = small_uint_unchecked_sub_func(55_u64.into_longunion(), 55_u64.into_longunion());
    println!("55 - 55 = {}", a_longunion);
    assert_eq!(a_longunion.get(), 0_u64);

    // It will panic
    // let b_u64 = small_uint_unchecked_sub_func(a_longunion, 1_u64);

    // Example for LongerUnion
    let a_longerunion = small_uint_unchecked_sub_func(55_u128.into_longerunion(), 55_u128.into_longerunion());
    println!("55 - 55 = {}", a_longerunion);
    assert_eq!(a_longerunion.get(), 0_u128);

    // It will panic
    // let b_longerunion = small_uint_unchecked_sub_func(a_longerunion, 1_u128.into_longerunion());

    // Example for SizeUnion
    let a_sizeunion = small_uint_unchecked_sub_func(55_usize.into_sizeunion(), 55_usize.into_sizeunion());
    println!("55 - 55 = {}", a_usize);
    assert_eq!(a_sizeunion.get(), 0_usize);

    // It will panic
    // let b_sizeunion = small_uint_unchecked_sub_func(a_sizeunion, 1_usize.into_sizeunion());
    println!("--------------------------------------");
}

fn small_uint_unchecked_sub_func<T: cryptocol::number::SmallUInt>(lhs: T, rhs: T) -> T
{
    lhs.unchecked_sub(rhs)
}

fn small_uint_saturating_sub()
{
    println!("small_uint_saturating_sub()");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = small_uint_saturating_sub_func(55_u8, 50_u8);
    println!("55 - 50 = {}", a_u8);
    assert_eq!(a_u8, 5_u8);

    let b_u8 = small_uint_saturating_sub_func(a_u8, 55_u8);
    println!("5 - 55 = {}", b_u8);
    assert_eq!(b_u8, 0_u8);

    // Example for u16
    let a_u16 = small_uint_saturating_sub_func(55_u16, 50_u16);
    println!("55 - 50 = {}", a_u16);
    assert_eq!(a_u16, 5_u16);

    let b_u16 = small_uint_saturating_sub_func(a_u16, 55_u16);
    println!("5 - 55 = {}", b_u16);
    assert_eq!(b_u16, 0_u16);

    // Example for u32
    let a_u32 = small_uint_saturating_sub_func(55_u32, 50_u32);
    println!("55 - 50 = {}", a_u32);
    assert_eq!(a_u32, 5_u32);

    let b_u32 = small_uint_saturating_sub_func(a_u32, 55_u32);
    println!("{} - 55 = {}", a_u32, b_u32);
    assert_eq!(b_u32, 0_u32);

    // Example for u64
    let a_u64 = small_uint_saturating_sub_func(55_u64, 50_u64);
    println!("55 - 50 = {}", a_u64);
    assert_eq!(a_u64, 5_u64);

    let b_u64 = small_uint_saturating_sub_func(a_u64, 55_u64);
    println!("{} - 55 = {}", a_u64, b_u64);
    assert_eq!(b_u64, 0_u64);

    // Example for u128
    let a_u128 = small_uint_saturating_sub_func(55_u128, 50_u128);
    println!("55 - 50 = {}", a_u128);
    assert_eq!(a_u128, 5_u128);

    let b_u128 = small_uint_saturating_sub_func(a_u128, 55_u128);
    println!("{} - 55 = {}", a_u128, b_u128);
    assert_eq!(b_u128, 0_u128);

    // Example for usize
    let a_usize = small_uint_saturating_sub_func(55_usize, 50_usize);
    println!("55 - 50 = {}", a_usize);
    assert_eq!(a_usize, 5_usize);

    let b_usize = small_uint_saturating_sub_func(a_usize, 55_usize);
    println!("{} - 55 = {}", a_usize, b_usize);
    assert_eq!(b_usize, 0_usize);

    // Example for ShortUnion
    let a_shortunion = small_uint_saturating_sub_func(55_u16.into_shortunion(), 50_u16.into_shortunion());
    println!("55 - 50 = {}", a_shortunion);
    assert_eq!(a_shortunion.get(), 5_u16);

    let b_u16 = small_uint_saturating_sub_func(a_shortunion, 55_u16.into_shortunion());
    println!("5 - 55 = {}", b_u16);
    assert_eq!(b_u16.get(), 0_u16);

    // Example for IntUnion
    let a_intunion = small_uint_saturating_sub_func(55_u32.into_intunion(), 50_u32.into_intunion());
    println!("55 - 50 = {}", a_intunion);
    assert_eq!(a_intunion.get(), 5_u32);

    let b_intunion = small_uint_saturating_sub_func(a_intunion, 55_u32.into_intunion());
    println!("{} - 55 = {}", a_u32, b_intunion);
    assert_eq!(b_intunion.get(), 0_u32);

    // Example for LongUnion
    let a_longunion = small_uint_saturating_sub_func(55_u64.into_longunion(), 50_u64.into_longunion());
    println!("55 - 50 = {}", a_longunion);
    assert_eq!(a_longunion.get(), 5_u64);

    let b_longunion = small_uint_saturating_sub_func(a_longunion, 55_u64.into_longunion());
    println!("{} - 55 = {}", a_longunion, b_longunion);
    assert_eq!(b_longunion.get(), 0_u64);

    // Example for LongerUnion
    let a_longerunion = small_uint_saturating_sub_func(55_u128.into_longerunion(), 50_u128.into_longerunion());
    println!("55 - 50 = {}", a_u128);
    assert_eq!(a_longerunion.get(), 5_u128);

    let b_longerunion = small_uint_saturating_sub_func(a_longerunion, 55_u128.into_longerunion());
    println!("{} - 55 = {}", a_longerunion, b_longerunion);
    assert_eq!(b_longerunion.get(), 0_u128);

    // Example for SizeUnion
    let a_sizeunion = small_uint_saturating_sub_func(55_usize.into_sizeunion(), 50_usize.into_sizeunion());
    println!("55 - 50 = {}", a_sizeunion);
    assert_eq!(a_sizeunion.get(), 5_usize);

    let b_sizeunion = small_uint_saturating_sub_func(a_sizeunion, 55_usize.into_sizeunion());
    println!("{} - 55 = {}", a_sizeunion, b_sizeunion);
    assert_eq!(b_sizeunion.get(), 0_usize);
    println!("--------------------------------------");
}

fn small_uint_saturating_sub_func<T: cryptocol::number::SmallUInt>(lhs: T, rhs: T) -> T
{
    lhs.saturating_sub(rhs)
}

fn small_uint_abs_diff()
{
    println!("small_uint_abs_diff()");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = small_uint_abs_diff_func(55_u8, 50_u8);
    println!("55 <-> 50 = {}", a_u8);
    assert_eq!(a_u8, 5_u8);

    let b_u8 = small_uint_abs_diff_func(50_u8, 55_u8);
    println!("50 <-> 55 = {}", b_u8);
    assert_eq!(b_u8, 5_u8);

    // Example for u16
    let a_u16 = small_uint_abs_diff_func(5050_u16, 5000_u16);
    println!("5050 <-> 5000 = {}", a_u16);
    assert_eq!(a_u16, 50_u16);

    let b_u16 = small_uint_abs_diff_func(5000_u16, 5050_u16);
    println!("5000 <-> 5050 = {}", b_u16);
    assert_eq!(b_u16, 50_u16);

    // Example for u32
    let a_u32 = small_uint_abs_diff_func(500500_u32, 500000_u32);
    println!("500500 <-> 500000 = {}", a_u32);
    assert_eq!(a_u32, 500_u32);

    let b_u32 = small_uint_abs_diff_func(500000_u32, 500500_u32);
    println!("500000 <-> 500500 = {}", b_u32);
    assert_eq!(b_u32, 500_u32);

    // Example for u64
    let a_u64 = small_uint_abs_diff_func(5000050000_u64, 5000000000_u64);
    println!("5000050000 <-> 5000000000 = {}", a_u64);
    assert_eq!(a_u64, 50000_u64);

    let b_u64 = small_uint_abs_diff_func(5000000000_u64, 5000050000_u64);
    println!("5000000000 <-> 5000050000 = {}", b_u64);
    assert_eq!(b_u64, 50000_u64);

    // Example for u128
    let a_u128 = small_uint_abs_diff_func(500000000500000000_u128, 500000000000000000_u128);
    println!("500000000500000000 <-> 500000000000000000 = {}", a_u128);
    assert_eq!(a_u128, 500000000_u128);

    let b_u128 = small_uint_abs_diff_func(500000000000000000_u128, 500000000500000000_u128);
    println!("500000000000000000 <-> 500000000500000000 = {}", b_u128);
    assert_eq!(b_u128, 500000000_u128);

    // Example for usize
    let a_usize = small_uint_abs_diff_func(5000050000_usize, 5000000000_usize);
    println!("5000050000 <-> 5000000000 = {}", a_usize);
    assert_eq!(a_usize, 50000_usize);

    let b_usize = small_uint_abs_diff_func(5000000000_usize, 5000050000_usize);
    println!("5000000000 <-> 5000050000 = {}", b_usize);
    assert_eq!(b_usize, 50000_usize);

    // Example for ShortUnion
    let a_shortunion = small_uint_abs_diff_func(5050_u16.into_shortunion(), 5000_u16.into_shortunion());
    println!("5050 <-> 5000 = {}", a_shortunion);
    assert_eq!(a_shortunion.get(), 50_u16);

    let b_shortunion = small_uint_abs_diff_func(5000_u16.into_shortunion(), 5050_u16.into_shortunion());
    println!("5000 <-> 5050 = {}", b_shortunion);
    assert_eq!(b_shortunion.get(), 50_u16);

    // Example for IntUnion
    let a_intunion = small_uint_abs_diff_func(500500_u32.into_intunion(), 500000_u32.into_intunion());
    println!("500500 <-> 500000 = {}", a_intunion);
    assert_eq!(a_intunion.get(), 500_u32);

    let b_intunion = small_uint_abs_diff_func(500000_u32.into_intunion(), 500500_u32.into_intunion());
    println!("500000 <-> 500500 = {}", b_intunion);
    assert_eq!(b_intunion.get(), 500_u32);

    // Example for LongUnion
    let a_longunion = small_uint_abs_diff_func(5000050000_u64.into_longunion(), 5000000000_u64.into_longunion());
    println!("5000050000 <-> 5000000000 = {}", a_longunion);
    assert_eq!(a_longunion.get(), 50000_u64);

    let b_longunion = small_uint_abs_diff_func(5000000000_u64.into_longunion(), 5000050000_u64.into_longunion());
    println!("5000000000 <-> 5000050000 = {}", b_longunion);
    assert_eq!(b_longunion.get(), 50000_u64);

    // Example for LongerUnion
    let a_longerunion = small_uint_abs_diff_func(500000000500000000_u128.into_longerunion(), 500000000000000000_u128.into_longerunion());
    println!("500000000500000000 <-> 500000000000000000 = {}", a_longerunion);
    assert_eq!(a_longerunion.get(), 500000000_u128);

    let b_longerunion = small_uint_abs_diff_func(500000000000000000_u128.into_longerunion(), 500000000500000000_u128.into_longerunion());
    println!("500000000000000000 <-> 500000000500000000 = {}", b_longerunion);
    assert_eq!(b_longerunion.get(), 500000000_u128);

    // Example for SizeUnion
    let a_sizeunion = small_uint_abs_diff_func(5000050000_usize.into_sizeunion(), 5000000000_usize.into_sizeunion());
    println!("5000050000 <-> 5000000000 = {}", a_sizeunion);
    assert_eq!(a_sizeunion.get(), 50000_usize);

    let b_sizeunion = small_uint_abs_diff_func(5000000000_usize.into_sizeunion(), 5000050000_usize.into_sizeunion());
    println!("5000000000 <-> 5000050000 = {}", b_sizeunion);
    assert_eq!(b_sizeunion.get(), 50000_usize);
    println!("--------------------------------------");
}

fn small_uint_abs_diff_func<T: cryptocol::number::SmallUInt>(lhs: T, rhs: T) -> T
{
    lhs.abs_diff(rhs)
}

fn small_uint_modular_sub()
{
    println!("small_uint_modular_sub()");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = small_uint_modular_sub_func(60_u8, 55, 100);
    println!("60 - 55 = {} (mod 100)", a_u8);
    assert_eq!(a_u8, 5);

    let b_u8 = small_uint_modular_sub_func(a_u8, 15, 100);
    println!("{} - 15 = {} (mod 100)", a_u8, b_u8);
    assert_eq!(b_u8, 90);

    // Example for u16
    let a_u16 = small_uint_modular_sub_func(6000_u16, 5500, 1_0000);
    println!("6000 - 5500 = {} (mod 1_0000)", a_u16);
    assert_eq!(a_u16, 500);

    let b_u16 = small_uint_modular_sub_func(a_u16, 1500, 1_0000);
    println!("{} - 1500 = {} (mod 1_0000)", a_u16, b_u16);
    assert_eq!(b_u16, 9000);

    // Example for u32
    let a_u32 = small_uint_modular_sub_func(6_0000_0000_u32, 5_5000_0000, 10_0000_0000);
    println!("6_0000_0000 - 5_5000_0000 = {} (mod 10_0000_0000)", a_u32);
    assert_eq!(a_u32, 5000_0000);

    let b_u32 = small_uint_modular_sub_func(a_u32, 1_5000_0000, 10_0000_0000);
    println!("{} - 1_5000_0000 = {} (mod 10_0000_0000)", a_u32, b_u32);
    assert_eq!(b_u32, 9_0000_0000);

    // Example for u64
    let a_u64 = small_uint_modular_sub_func(6_0000_0000_0000_0000_u64, 5_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    println!("6_0000_0000_0000_0000 - 5_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_u64);
    assert_eq!(a_u64, 5000_0000_0000_0000);

    let b_u64 = small_uint_modular_sub_func(a_u64, 1_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    println!("{} - 1_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_u64, b_u64);
    assert_eq!(b_u64, 9_0000_0000_0000_0000);

    // Example for u128
    let a_u128 = small_uint_modular_sub_func(6_0000_0000_0000_0000_0000_0000_0000_0000_u128, 5_5000_0000_0000_0000_0000_0000_0000_0000, 10_0000_0000_0000_0000_0000_0000_0000_0000);
    println!("6_0000_0000_0000_0000_0000_0000_0000_0000 - 5_5000_0000_0000_0000_0000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000_0000_0000_0000_0000)", a_u128);
    assert_eq!(a_u128, 5000_0000_0000_0000_0000_0000_0000_0000);

    let b_u128 = small_uint_modular_sub_func(a_u128, 1_5000_0000_0000_0000_0000_0000_0000_0000, 10_0000_0000_0000_0000_0000_0000_0000_0000);
    println!("{} - 1_5000_0000_0000_0000_0000_0000_0000_0000 = {}",a_u128, b_u128);
    assert_eq!(b_u128, 9_0000_0000_0000_0000_0000_0000_0000_0000);

    // Example for usize
    let a_usize = small_uint_modular_sub_func(6_0000_0000_0000_0000_usize, 5_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    println!("6_0000_0000_0000_0000 - 5_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_usize);
    assert_eq!(a_usize, 5000_0000_0000_0000);

    let b_usize = small_uint_modular_sub_func(a_usize, 1_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    println!("{} - 1_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_usize, b_usize);
    assert_eq!(b_usize, 9_0000_0000_0000_0000);

    // Example for ShortUnion
    let a_shortunion = small_uint_modular_sub_func(6000_u16.into_shortunion(), 5500_u16.into_shortunion(), 1_0000_u16.into_shortunion());
    println!("6000 - 5500 = {} (mod 1_0000)", a_shortunion);
    assert_eq!(a_shortunion.get(), 500);

    let b_shortunion = small_uint_modular_sub_func(a_shortunion, 1500_u16.into_shortunion(), 1_0000_u16.into_shortunion());
    println!("{} - 1500 = {} (mod 1_0000)", a_shortunion, b_shortunion);
    assert_eq!(b_shortunion.get(), 9000);

    // Example for IntUnion
    let a_intunion = small_uint_modular_sub_func(6_0000_0000_u32.into_intunion(), 5_5000_0000_u32.into_intunion(), 10_0000_0000_u32.into_intunion());
    println!("6_0000_0000 - 5_5000_0000 = {} (mod 10_0000_0000)", a_intunion);
    assert_eq!(a_intunion.get(), 5000_0000);

    let b_intunion = small_uint_modular_sub_func(a_intunion, 1_5000_0000_u32.into_intunion(), 10_0000_0000_u32.into_intunion());
    println!("{} - 1_5000_0000 = {} (mod 10_0000_0000)", a_intunion, b_intunion);
    assert_eq!(b_intunion.get(), 9_0000_0000);

    // Example for LongUnion
    let a_longunion = small_uint_modular_sub_func(6_0000_0000_0000_0000_u64.into_longunion(), 5_5000_0000_0000_0000_u64.into_longunion(), 10_0000_0000_0000_0000_u64.into_longunion());
    println!("6_0000_0000_0000_0000 - 5_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_longunion);
    assert_eq!(a_longunion.get(), 5000_0000_0000_0000);

    let b_longunion = small_uint_modular_sub_func(a_longunion, 1_5000_0000_0000_0000_u64.into_longunion(), 10_0000_0000_0000_0000_u64.into_longunion());
    println!("{} - 1_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_longunion, b_longunion);
    assert_eq!(b_longunion.get(), 9_0000_0000_0000_0000);

    // Example for LongerUnion
    let a_longerunion = small_uint_modular_sub_func(6_0000_0000_0000_0000_0000_0000_0000_0000_u128.into_longerunion(), 5_5000_0000_0000_0000_0000_0000_0000_0000_u128.into_longerunion(), 10_0000_0000_0000_0000_0000_0000_0000_0000_u128.into_longerunion());
    println!("6_0000_0000_0000_0000_0000_0000_0000_0000 - 5_5000_0000_0000_0000_0000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000_0000_0000_0000_0000)", a_longerunion);
    assert_eq!(a_longerunion.get(), 5000_0000_0000_0000_0000_0000_0000_0000);

    let b_longerunion = small_uint_modular_sub_func(a_longerunion, 1_5000_0000_0000_0000_0000_0000_0000_0000_u128.into_longerunion(), 10_0000_0000_0000_0000_0000_0000_0000_0000_u128.into_longerunion());
    println!("{} - 1_5000_0000_0000_0000_0000_0000_0000_0000 = {}", a_longerunion, b_longerunion);
    assert_eq!(b_longerunion.get(), 9_0000_0000_0000_0000_0000_0000_0000_0000);

    // Example for SizeUnion
    let a_sizeunion = small_uint_modular_sub_func(6_0000_0000_0000_0000_usize.into_sizeunion(), 5_5000_0000_0000_0000_usize.into_sizeunion(), 10_0000_0000_0000_0000_usize.into_sizeunion());
    println!("6_0000_0000_0000_0000 - 5_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_sizeunion);
    assert_eq!(a_sizeunion.get(), 5000_0000_0000_0000);

    let b_sizeunion = small_uint_modular_sub_func(a_sizeunion, 1_5000_0000_0000_0000_usize.into_sizeunion(), 10_0000_0000_0000_0000_usize.into_sizeunion());
    println!("{} - 1_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_sizeunion, b_sizeunion);
    assert_eq!(b_sizeunion.get(), 9_0000_0000_0000_0000);
    println!("--------------------------------------");
}

fn small_uint_modular_sub_func<T: cryptocol::number::SmallUInt>(lhs: T, rhs: T, modulo: T) -> T
{
    lhs.modular_sub(rhs, modulo)
}

fn small_uint_mul_main()
{
    small_uint_carrying_mul();
    small_uint_widening_mul();
    small_uint_wrapping_mul();
    small_uint_overflowing_mul();
    small_uint_checked_mul();
    small_uint_unchecked_mul();
    small_uint_saturating_mul();
    small_uint_modular_mul();
}
fn small_uint_carrying_mul()
{
    println!("small_uint_carrying_mul()");
    use cryptocol::number::SmallUInt;
    use cryptocol::number::IntUnion;
    use cryptocol::number::LongUnion;
    // Example for u8
    // a_u16: u16 === (a_high_u8, a_low_u8) == (100_u8, 101_u8) == 25701_u16
    let a_high_u8 = 100_u8;
    let a_low_u8 = 101_u8;
    // b_u16: u16 === (b_high_u8, b_low_u8) == (100_u8, 200_u8) == 25800_u16
    let b_high_u8 = 100_u8;
    let b_low_u8 = 200_u8;

    // (100_u8, 101_u8) X (100_u8, 200_u8) == 25701_u16 X 25800_u16 == 663085800_u32
    //
    //                  (100_u8, 101_u8) == 25701_u16
    // X                (100_u8, 200_u8) == 25800_u16
    // ---------------------------------
    //                  ( 78_u8, 232_u8)
    //          ( 78_u8,  32_u8)
    //          ( 39_u8, 116_u8)
    // + (39_u8,  16_u8)
    // ---------------------------------
    //   (39_u8, 133_u8, 226_u8, 232_u8) == 663085800_u32
    let (c_lower_u8, c_low_u8, c_high_u8, c_higher_u8 ) = small_uint_carrying_mul_func(a_low_u8, a_high_u8, b_low_u8, b_high_u8);
    println!("{}-{}-{}-{}", c_higher_u8, c_high_u8, c_low_u8, c_lower_u8);
    assert_eq!(c_higher_u8, 39);
    assert_eq!(c_high_u8, 133);
    assert_eq!(c_low_u8, 226);
    assert_eq!(c_lower_u8, 232);

    let a = IntUnion::new_with_ubytes([a_low_u8, a_high_u8, 0, 0]);
    let b = IntUnion::new_with_ubytes([b_low_u8, b_high_u8, 0, 0]);
    let c = a * b;
    println!("{} * {} = {}", a.get(), b.get(), c.get());
    assert_eq!(c_higher_u8, c.get_ubyte_(3));
    assert_eq!(c_high_u8, c.get_ubyte_(2));
    assert_eq!(c_low_u8, c.get_ubyte_(1));
    assert_eq!(c_lower_u8, c.get_ubyte_(0));

    // Example for u16 for Little Endian
    // a_u32: u32 === (a_high_u16, a_low_u16) == (10000_u16, 10100_u16) == 257010000_u32
    let a_high_u16 = 10000_u16;
    let a_low_u16 = 10100_u16;
    // b_u32: u32 === (b_high_u16, b_low_u16) == (10000_u16, 20000_u16) == 258000000_u32
    let b_high_u16 = 10000_u16;
    let b_low_u16 = 20000_u16;
    
    // (10000_u16, 10100_u16) X (10000_u16, 20000_u16) == 257010000_u32 X 258000000_u32 == 66308580000000000_u32
    //
    //                        (10000_u16, 10100_u16) == 655370100_u32
    // X                      (10000_u16, 20000_u16) == 655380000_u32
    // ---------------------------------------------
    //                       (  3082_u16, 18048_u16)
    //            (  3051_u16, 49664_u16)
    //            (  1541_u16,  9024_u16)
    // + (1525_u16, 57600_u16)
    // ---------------------------------
    //   (1525_u16, 62192_u16, 61770_u16, 18048_u16) == 429516456138000000_u64
    let (c_lower_u16, c_low_u16, c_high_u16, c_higher_u16 ) = small_uint_carrying_mul_func(a_low_u16, a_high_u16, b_low_u16, b_high_u16);
    println!("{}-{}-{}-{}", c_higher_u16, c_high_u16, c_low_u16, c_lower_u16);
    assert_eq!(c_higher_u16, 1525);
    assert_eq!(c_high_u16, 62192);
    assert_eq!(c_low_u16, 61770);
    assert_eq!(c_lower_u16, 18048);

    let a = LongUnion::new_with_ushorts([a_low_u16, a_high_u16, 0, 0]);
    let b = LongUnion::new_with_ushorts([b_low_u16, b_high_u16, 0, 0]);
    let c = a * b;
    println!("{} * {} = {}", a.get(), b.get(), c.get());
    assert_eq!(c_higher_u16, c.get_ushort_(3));
    assert_eq!(c_high_u16, c.get_ushort_(2));
    assert_eq!(c_low_u16, c.get_ushort_(1));
    assert_eq!(c_lower_u16, c.get_ushort_(0));

    // Example for ShortUnion for Little Endian
    // a_u64: u32 === (a_high_u16, a_low_u16) == (10000_u16, 10100_u16) == 257010000_u32
    let a_high_shortunion = 10000_u16.into_shortunion();
    let a_low_shortunion = 10100_u16.into_shortunion();
    // b_u32: u32 === (b_high_u16, b_low_u16) == (10000_u16, 20000_u16) == 258000000_u32
    let b_high_shortunion = 10000_u16.into_shortunion();
    let b_low_shortunion = 20000_u16.into_shortunion();
    
    // (10000_u16, 10100_u16) X (10000_u16, 20000_u16) == 257010000_u32 X 258000000_u32 == 66308580000000000_u32
    //
    //                        (10000_u16, 10100_u16) == 655370100_u32
    // X                      (10000_u16, 20000_u16) == 655380000_u32
    // ---------------------------------------------
    //                       (  3082_u16, 18048_u16)
    //            (  3051_u16, 49664_u16)
    //            (  1541_u16,  9024_u16)
    // + (1525_u16, 57600_u16)
    // ---------------------------------
    //   (1525_u16, 62192_u16, 61770_u16, 18048_u16) == 429516456138000000_u64
    let (c_lower_shortunion, c_low_shortunion, c_high_shortunion, c_higher_shortunion ) = small_uint_carrying_mul_func(a_low_shortunion, a_high_shortunion, b_low_shortunion, b_high_shortunion);
    println!("{}-{}-{}-{}", c_higher_shortunion, c_high_shortunion, c_low_shortunion, c_lower_shortunion);
    assert_eq!(c_higher_shortunion.get(), 1525);
    assert_eq!(c_high_shortunion.get(), 62192);
    assert_eq!(c_low_shortunion.get(), 61770);
    assert_eq!(c_lower_shortunion.get(), 18048);

    let a = LongUnion::new_with_ushorts([a_low_shortunion.get(), a_high_shortunion.get(), 0, 0]);
    let b = LongUnion::new_with_ushorts([b_low_shortunion.get(), b_high_shortunion.get(), 0, 0]);
    let c = a * b;
    println!("{} * {} = {}", a.get(), b.get(), c.get());
    assert_eq!(c_higher_shortunion.get(), c.get_ushort_(3));
    assert_eq!(c_high_shortunion.get(), c.get_ushort_(2));
    assert_eq!(c_low_shortunion.get(), c.get_ushort_(1));
    assert_eq!(c_lower_shortunion.get(), c.get_ushort_(0));
    println!("--------------------------------------");
}

fn small_uint_carrying_mul_func<T: cryptocol::number::SmallUInt>(lhs_low: T, lhs_high: T, rhs_low: T, rhs_high: T) -> (T, T, T, T)
{
    let (c_low, c_high ) = rhs_low.carrying_mul(lhs_low, T::zero());
    let (d_low, d_high ) = rhs_low.carrying_mul(lhs_high, c_high);
    let (mut e_low, e_high ) = rhs_high.carrying_mul(lhs_low, T::zero());
    let (mut f_low, mut f_high ) = rhs_high.carrying_mul(lhs_high, e_high);

    let mut overflow: bool;
    (e_low, overflow) = e_low.overflowing_add(d_low);
    if overflow
        { (f_low, overflow) = f_low.overflowing_add(T::one()); }
    if overflow
        { f_high = f_high.wrapping_add(T::one()); }

    (f_low, overflow) = f_low.overflowing_add(d_high);
    if overflow
        { f_high = f_high.wrapping_add(T::one()); }
    (c_low, e_low, f_low, f_high)
}

fn small_uint_widening_mul()
{
    println!("small_uint_widening_mul()");
    use cryptocol::number::SmallUInt;
    use cryptocol::number::IntUnion;
    use cryptocol::number::LongUnion;
    // Example for u8
    // a_u16: u16 === (a_high_u8, a_low_u8) == (100_u8, 101_u8) == 25701_u16
    let a_high_u8 = 100_u8;
    let a_low_u8 = 101_u8;
    // b_u16: u16 === (b_high_u8, b_low_u8) == (100_u8, 200_u8) == 25800_u16
    let b_high_u8 = 100_u8;
    let b_low_u8 = 200_u8;

    // (100_u8, 101_u8) X (100_u8, 200_u8) == 25701_u16 X 25800_u16 == 663085800_u32
    //
    //                  (100_u8, 101_u8) == 25701_u16
    // X                (100_u8, 200_u8) == 25800_u16
    // ---------------------------------
    //                  ( 78_u8, 232_u8)
    //          ( 78_u8,  32_u8)
    //          ( 39_u8, 116_u8)
    // + (39_u8,  16_u8)
    // ---------------------------------
    //   (39_u8, 133_u8, 226_u8, 232_u8) == 663085800_u32
    let (c_lower_u8, c_low_u8, c_high_u8, c_higher_u8 ) = small_uint_widening_mul_func(a_low_u8, a_high_u8, b_low_u8, b_high_u8);
    println!("{}-{}-{}-{}", c_higher_u8, c_high_u8, c_low_u8, c_lower_u8);
    assert_eq!(c_higher_u8, 39);
    assert_eq!(c_high_u8, 133);
    assert_eq!(c_low_u8, 226);
    assert_eq!(c_lower_u8, 232);

    let a = IntUnion::new_with_ubytes([a_low_u8, a_high_u8, 0, 0]);
    let b = IntUnion::new_with_ubytes([b_low_u8, b_high_u8, 0, 0]);
    let c = a * b;
    println!("{} * {} = {}", a.get(), b.get(), c.get());
    assert_eq!(c_higher_u8, c.get_ubyte_(3));
    assert_eq!(c_high_u8, c.get_ubyte_(2));
    assert_eq!(c_low_u8, c.get_ubyte_(1));
    assert_eq!(c_lower_u8, c.get_ubyte_(0));

    // Example for u16
    // a_u32: u32 === (a_high_u16, a_low_u16) == (10000_u16, 10100_u16) == 257010000_u32
    let a_high_u16 = 10000_u16;
    let a_low_u16 = 10100_u16;
    // b_u32: u32 === (b_high_u16, b_low_u16) == (10000_u16, 20000_u16) == 258000000_u32
    let b_high_u16 = 10000_u16;
    let b_low_u16 = 20000_u16;
    
    // (10000_u16, 10100_u16) X (10000_u16, 20000_u16) == 257010000_u32 X 258000000_u32 == 66308580000000000_u32
    //
    //                        (10000_u16, 10100_u16) == 655370100_u32
    // X                      (10000_u16, 20000_u16) == 655380000_u32
    // ---------------------------------------------
    //                       (  3082_u16, 18048_u16)
    //            (  3051_u16, 49664_u16)
    //            (  1541_u16,  9024_u16)
    // + (1525_u16, 57600_u16)
    // ---------------------------------
    //   (1525_u16, 62192_u16, 61770_u16, 18048_u16) == 429516456138000000_u64
    let (c_lower_u16, c_low_u16, c_high_u16, c_higher_u16 ) = small_uint_widening_mul_func(a_low_u16, a_high_u16, b_low_u16, b_high_u16);
    println!("{}-{}-{}-{}", c_higher_u16, c_high_u16, c_low_u16, c_lower_u16);
    assert_eq!(c_higher_u16, 1525);
    assert_eq!(c_high_u16, 62192);
    assert_eq!(c_low_u16, 61770);
    assert_eq!(c_lower_u16, 18048);

    let a = LongUnion::new_with_ushorts([a_low_u16, a_high_u16, 0, 0]);
    let b = LongUnion::new_with_ushorts([b_low_u16, b_high_u16, 0, 0]);
    let c = a * b;
    println!("{} * {} = {}", a.get(), b.get(), c.get());
    assert_eq!(c_higher_u16, c.get_ushort_(3));
    assert_eq!(c_high_u16, c.get_ushort_(2));
    assert_eq!(c_low_u16, c.get_ushort_(1));
    assert_eq!(c_lower_u16, c.get_ushort_(0));

    // Example for ShortUnion for Little Endian
    // a_u64: u32 === (a_high_u16, a_low_u16) == (10000_u16, 10100_u16) == 257010000_u32
    let a_high_shortunion = 10000_u16.into_shortunion();
    let a_low_shortunion = 10100_u16.into_shortunion();
    // b_u32: u32 === (b_high_u16, b_low_u16) == (10000_u16, 20000_u16) == 258000000_u32
    let b_high_shortunion = 10000_u16.into_shortunion();
    let b_low_shortunion = 20000_u16.into_shortunion();
    
    // (10000_u16, 10100_u16) X (10000_u16, 20000_u16) == 257010000_u32 X 258000000_u32 == 66308580000000000_u32
    //
    //                        (10000_u16, 10100_u16) == 655370100_u32
    // X                      (10000_u16, 20000_u16) == 655380000_u32
    // ---------------------------------------------
    //                       (  3082_u16, 18048_u16)
    //            (  3051_u16, 49664_u16)
    //            (  1541_u16,  9024_u16)
    // + (1525_u16, 57600_u16)
    // ---------------------------------
    //   (1525_u16, 62192_u16, 61770_u16, 18048_u16) == 429516456138000000_u64
    let (c_lower_shortunion, c_low_shortunion, c_high_shortunion, c_higher_shortunion ) = small_uint_widening_mul_func(a_low_shortunion, a_high_shortunion, b_low_shortunion, b_high_shortunion);
    println!("{}-{}-{}-{}", c_higher_shortunion, c_high_shortunion, c_low_shortunion, c_lower_shortunion);
    assert_eq!(c_higher_shortunion.get(), 1525);
    assert_eq!(c_high_shortunion.get(), 62192);
    assert_eq!(c_low_shortunion.get(), 61770);
    assert_eq!(c_lower_shortunion.get(), 18048);

    let a = LongUnion::new_with_ushorts([a_low_shortunion.get(), a_high_shortunion.get(), 0, 0]);
    let b = LongUnion::new_with_ushorts([b_low_shortunion.get(), b_high_shortunion.get(), 0, 0]);
    let c = a * b;
    println!("{} * {} = {}", a.get(), b.get(), c.get());
    assert_eq!(c_higher_shortunion.get(), c.get_ushort_(3));
    assert_eq!(c_high_shortunion.get(), c.get_ushort_(2));
    assert_eq!(c_low_shortunion.get(), c.get_ushort_(1));
    assert_eq!(c_lower_shortunion.get(), c.get_ushort_(0));
    println!("--------------------------------------");
}

fn small_uint_widening_mul_func<T: cryptocol::number::SmallUInt>(lhs_low: T, lhs_high: T, rhs_low: T, rhs_high: T) -> (T, T, T, T)
{
    let (c_low, c_high ) = rhs_low.widening_mul(lhs_low);
    let (d_low, d_high ) = rhs_low.widening_mul(lhs_high);
    let (mut e_low, e_high ) = rhs_high.widening_mul(lhs_low);
    let (mut f_low, mut f_high ) = rhs_high.widening_mul(lhs_high);

    let mut overflow: bool;
    (e_low, overflow) = e_low.overflowing_add(d_low);
    if overflow
        { (f_low, overflow) = f_low.overflowing_add(T::one()); }
    if overflow
        { f_high = f_high.wrapping_add(T::one()); }
    (e_low, overflow) = e_low.overflowing_add(c_high);
    if overflow
        { (f_low, overflow) = f_low.overflowing_add(T::one()); }
    if overflow
        { f_high = f_high.wrapping_add(T::one()); }

    (f_low, overflow) = f_low.overflowing_add(d_high);
    if overflow
        { f_high = f_high.wrapping_add(T::one()); }
    (f_low, overflow) = f_low.overflowing_add(e_high);
    if overflow
        { f_high = f_high.wrapping_add(T::one()); }
    (c_low, e_low, f_low, f_high)
}

fn small_uint_wrapping_mul()
{
    println!("small_uint_wrapping_mul()");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = small_uint_wrapping_mul_func(u8::MAX / 3, 2_u8);
    println!("{} * 2 = {}", u8::MAX / 3, a_u8);
    assert_eq!(a_u8, 170_u8);

    let b_u8 = small_uint_wrapping_mul_func(a_u8, 2_u8);
    println!("{} * 2 = {}", a_u8, b_u8);
    assert_eq!(b_u8, 84_u8);

    // Example for u16
    let a_u16 = small_uint_wrapping_mul_func(u16::MAX / 3, 2_u16);
    println!("{} * 2 = {}", u16::MAX / 3, a_u16);
    assert_eq!(a_u16, 43690_u16);

    let b_u16 = small_uint_wrapping_mul_func(a_u16, 2_u16);
    println!("{} * 2 = {}", a_u16, b_u16);
    assert_eq!(b_u16, 21844_u16);

    // Example for u32
    let a_u32 = small_uint_wrapping_mul_func(u32::MAX / 3, 2_u32);
    println!("{} * 2 = {}", u32::MAX / 3, a_u32);
    assert_eq!(a_u32, 2863311530_u32);

    let b_u32 = small_uint_wrapping_mul_func(a_u32, 2_u32);
    println!("{} * 2 = {}", a_u32, b_u32);
    assert_eq!(b_u32, 1431655764_u32);

    // Example for u64
    let a_u64 = small_uint_wrapping_mul_func(u64::MAX / 3, 2_u64);
    println!("{} * 2 = {}", u64::MAX / 3, a_u64);
    assert_eq!(a_u64, 12297829382473034410_u64);

    let b_u64 = small_uint_wrapping_mul_func(a_u64, 2_u64);
    println!("{} * 2 = {}", a_u64, b_u64);
    assert_eq!(b_u64, 6148914691236517204_u64);

    // Example for u128
    let a_u128 = small_uint_wrapping_mul_func(u128::MAX / 3, 2_u128);
    println!("{} * 2 = {}", u128::MAX / 3, a_u128);
    assert_eq!(a_u128,226854911280625642308916404954512140970_u128);

    let b_u128 = small_uint_wrapping_mul_func(a_u128, 2_u128);
    println!("{} * 2 = {}", a_u128, b_u128);
    assert_eq!(b_u128, 113427455640312821154458202477256070484_u128);

    // Example for usize
    let a_usize = small_uint_wrapping_mul_func(usize::MAX / 3, 2_usize);
    println!("{} * 2 = {}", usize::MAX / 3, a_usize);
    assert_eq!(a_usize, 12297829382473034410_usize);

    let b_usize = small_uint_wrapping_mul_func(a_usize, 2_usize);
    println!("{} * 2 = {}", a_usize, b_usize);
    assert_eq!(b_usize, 6148914691236517204_usize);

    // Example for ShortUnion
    let a_shortunion = small_uint_wrapping_mul_func((u16::MAX / 3).into_shortunion(), 2_u16.into_shortunion());
    println!("{} * 2 = {}", (u16::MAX / 3).into_shortunion(), a_shortunion);
    assert_eq!(a_shortunion.get(), 43690_u16);

    let b_shortunion = small_uint_wrapping_mul_func(a_shortunion, 2_u16.into_shortunion());
    println!("{} * 2 = {}", a_shortunion, b_shortunion);
    assert_eq!(b_shortunion.get(), 21844_u16);

    // Example for IntUnion
    let a_intunion = small_uint_wrapping_mul_func((u32::MAX / 3).into_intunion(), 2_u32.into_intunion());
    println!("{} * 2 = {}", u32::MAX / 3, a_intunion);
    assert_eq!(a_intunion.get(), 2863311530_u32);

    let b_intunion = small_uint_wrapping_mul_func(a_intunion, 2_u32.into_intunion());
    println!("{} * 2 = {}", a_intunion, b_intunion);
    assert_eq!(b_intunion.get(), 1431655764_u32);

    // Example for LongUnion
    let a_longunion: cryptocol::number::LongUnion = small_uint_wrapping_mul_func((u64::MAX / 3).into_longunion(), 2_u64.into_longunion());
    println!("{} * 2 = {}", (u64::MAX / 3).into_longunion(), a_longunion);
    assert_eq!(a_longunion.get(), 12297829382473034410_u64);

    let b_longunion = small_uint_wrapping_mul_func(a_longunion, 2_u64.into_longunion());
    println!("{} * 2 = {}", a_longunion, b_longunion);
    assert_eq!(b_longunion.get(), 6148914691236517204_u64);

    // Example for LongerUnion
    let a_longerunion = small_uint_wrapping_mul_func((u128::MAX / 3).into_longerunion(), 2_u128.into_longerunion());
    println!("{} * 2 = {}", u128::MAX / 3, a_longerunion);
    assert_eq!(a_longerunion.get(), 226854911280625642308916404954512140970_u128);

    let b_longerunion = small_uint_wrapping_mul_func(a_longerunion, 2_u128.into_longerunion());
    println!("{} * 2 = {}", a_longerunion, b_longerunion);
    assert_eq!(b_longerunion.get(), 113427455640312821154458202477256070484_u128);

    // Example for SizeUnion
    let a_sizeunion = small_uint_wrapping_mul_func((usize::MAX / 3).into_sizeunion(), 2_usize.into_sizeunion());
    println!("{} * 2 = {}", (usize::MAX / 3).into_sizeunion(), a_sizeunion);
    assert_eq!(a_sizeunion.get(), 12297829382473034410_usize);

    let b_sizeunion = small_uint_wrapping_mul_func(a_sizeunion, 2_usize.into_sizeunion());
    println!("{} * 2 = {}", a_sizeunion, b_sizeunion);
    assert_eq!(b_sizeunion.get(), 6148914691236517204_usize);
    println!("--------------------------------------");
}

fn small_uint_wrapping_mul_func<T: cryptocol::number::SmallUInt>(lhs: T, rhs: T) -> T
{
    lhs.wrapping_mul(rhs)
}

fn small_uint_overflowing_mul()
{
    println!("small_uint_overflowing_mul()");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = small_uint_overflowing_mul_func(u8::MAX / 3, 2_u8);
    println!("{} * 2 = {}\nOverflow = {}", u8::MAX / 3, a_u8.0, a_u8.1);
    assert_eq!(a_u8.0, 170_u8);
    assert_eq!(a_u8.1, false);
 
    let b_u8 = small_uint_overflowing_mul_func(a_u8.0, 2_u8);
    println!("{} * 2 = {}\nOverflow = {}", a_u8.0, b_u8.0, b_u8.1);
    assert_eq!(b_u8.0, 84_u8);
    assert_eq!(b_u8.1, true);

    // Example for u16
    let a_u16 = small_uint_overflowing_mul_func(u16::MAX / 3, 2_u16);
    println!("{} * 2 = {}\nOverflow = {}", u16::MAX / 3, a_u16.0, a_u16.1);
    assert_eq!(a_u16.0, 43690_u16);
    assert_eq!(a_u16.1, false);
 
    let b_u16 = small_uint_overflowing_mul_func(a_u16.0, 2_u16);
    println!("{} * 2 = {}\nOverflow = {}", a_u16.0, b_u16.0, b_u16.1);
    assert_eq!(b_u16.0, 21844_u16);
    assert_eq!(b_u16.1, true);

    // Example for u32
    let a_u32 = small_uint_overflowing_mul_func(u32::MAX / 3, 2_u32);
    println!("{} * 2 = {}\nOverflow = {}", u32::MAX / 3, a_u32.0, a_u32.1);
    assert_eq!(a_u32.0, 2863311530_u32);
    assert_eq!(a_u32.1, false);
 
    let b_u32 = small_uint_overflowing_mul_func(a_u32.0, 2_u32);
    println!("{} * 2 = {}\nOverflow = {}", a_u32.0, b_u32.0, b_u32.1);
    assert_eq!(b_u32.0, 1431655764_u32);
    assert_eq!(b_u32.1, true);

    // Example for u64
    let a_u64 = small_uint_overflowing_mul_func(u64::MAX / 3, 2_u64);
    println!("{} * 2 = {}\nOverflow = {}", u64::MAX / 3, a_u64.0, a_u64.1);
    assert_eq!(a_u64.0, 12297829382473034410_u64);
    assert_eq!(a_u64.1, false);
 
    let b_u64 = small_uint_overflowing_mul_func(a_u64.0, 2_u64);
    println!("{} * 2 = {}\nOverflow = {}", a_u64.0, b_u64.0, b_u64.1);
    assert_eq!(b_u64.0, 6148914691236517204_u64);
    assert_eq!(b_u64.1, true);

    // Example for u128
    let a_u128 = small_uint_overflowing_mul_func(u128::MAX / 3, 2_u128);
    println!("{} * 2 = {}\nOverflow = {}", u128::MAX / 3, a_u128.0, a_u128.1);
    assert_eq!(a_u128.0, 226854911280625642308916404954512140970_u128);
    assert_eq!(a_u128.1, false);
 
    let b_u128 = small_uint_overflowing_mul_func(a_u128.0, 2_u128);
    println!("{} * 2 = {}\nOverflow = {}", a_u128.0, b_u128.0, b_u128.1);
    assert_eq!(b_u128.0, 113427455640312821154458202477256070484_u128);
    assert_eq!(b_u128.1, true);

    // Example for usize
    let a_usize = small_uint_overflowing_mul_func(usize::MAX / 3, 2_usize);
    println!("{} * 2 = {}\nOverflow = {}", usize::MAX / 3, a_usize.0, a_usize.1);
    assert_eq!(a_usize.0, 12297829382473034410_usize);
    assert_eq!(a_usize.1, false);
 
    let b_usize = small_uint_overflowing_mul_func(a_usize.0, 2_usize);
    println!("{} * 2 = {}\nOverflow = {}", a_usize.0, b_usize.0, b_usize.1);
    assert_eq!(b_usize.0, 6148914691236517204_usize);
    assert_eq!(b_usize.1, true);

    // Example for ShortUnion
    let (a_shortunion, overflow) = small_uint_overflowing_mul_func((u16::MAX / 3).into_shortunion(), 2_u16.into_shortunion());
    println!("{} * 2 = {}\nOverflow = {}", (u16::MAX / 3).into_shortunion(), a_shortunion, overflow);
    assert_eq!(a_shortunion.get(), 43690_u16);
    assert_eq!(overflow, false);
 
    let (b_shortunion, overflow) = small_uint_overflowing_mul_func(a_shortunion, 2_u16.into_shortunion());
    println!("{} * 2 = {}\nOverflow = {}", a_shortunion, b_shortunion, overflow);
    assert_eq!(b_shortunion.get(), 21844_u16);
    assert_eq!(overflow, true);

    // Example for IntUnion
    let (a_intunion, overflow) = small_uint_overflowing_mul_func((u32::MAX / 3).into_intunion(), 2_u32.into_intunion());
    println!("{} * 2 = {}\nOverflow = {}", (u32::MAX / 3).into_intunion(), a_intunion, overflow);
    assert_eq!(a_intunion.get(), 2863311530_u32);
    assert_eq!(overflow, false);
 
    let (b_intunion, overflow) = small_uint_overflowing_mul_func(a_intunion, 2_u32.into_intunion());
    println!("{} * 2 = {}\nOverflow = {}", a_intunion, b_intunion, overflow);
    assert_eq!(b_intunion.get(), 1431655764_u32);
    assert_eq!(overflow, true);

    // Example for LongUnion
    let (a_longunion, overflow) = small_uint_overflowing_mul_func((u64::MAX / 3).into_longunion(), 2_u64.into_longunion());
    println!("{} * 2 = {}\nOverflow = {}", u64::MAX / 3, a_longunion, a_u64.1);
    assert_eq!(a_longunion.get(), 12297829382473034410_u64);
    assert_eq!(overflow, false);
 
    let (b_longunion, overflow) = small_uint_overflowing_mul_func(a_longunion, 2_u64.into_longunion());
    println!("{} * 2 = {}\nOverflow = {}", a_longunion, b_longunion, overflow);
    assert_eq!(b_longunion.get(), 6148914691236517204_u64);
    assert_eq!(overflow, true);

    // Example for LongerUnion
    let (a_longerunion, overflow) = small_uint_overflowing_mul_func((u128::MAX / 3).into_longerunion(), 2_u128.into_longerunion());
    println!("{} * 2 = {}\nOverflow = {}", (u128::MAX / 3).into_longerunion(), a_longerunion, overflow);
    assert_eq!(a_longerunion.get(), 226854911280625642308916404954512140970_u128);
    assert_eq!(overflow, false);
 
    let (b_longerunion, overflow)= small_uint_overflowing_mul_func(a_longerunion, 2_u128.into_longerunion());
    println!("{} * 2 = {}\nOverflow = {}", a_longerunion, b_longerunion, overflow);
    assert_eq!(b_longerunion.get(), 113427455640312821154458202477256070484_u128);
    assert_eq!(overflow, true);

    // Example for SizeUnion
    let (a_sizeunion, overflow) = small_uint_overflowing_mul_func((usize::MAX / 3).into_sizeunion(), 2_usize.into_sizeunion());
    println!("{} * 2 = {}\nOverflow = {}", usize::MAX / 3, a_sizeunion, overflow);
    assert_eq!(a_sizeunion.get(), 12297829382473034410_usize);
    assert_eq!(overflow, false);
 
    let (b_sizeunion, overflow) = small_uint_overflowing_mul_func(a_sizeunion, 2_usize.into_sizeunion());
    println!("{} * 2 = {}\nOverflow = {}", a_sizeunion, b_sizeunion, overflow);
    assert_eq!(b_sizeunion.get(), 6148914691236517204_usize);
    assert_eq!(overflow, true);
    println!("--------------------------------------");
}

fn small_uint_overflowing_mul_func<T: cryptocol::number::SmallUInt>(lhs: T, rhs: T) -> (T, bool)
{
    lhs.overflowing_mul(rhs)
}

fn small_uint_checked_mul()
{
    println!("small_uint_checked_mul()");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = small_uint_checked_mul_func(u8::MAX / 3, 2_u8);
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

    let b_u8 = small_uint_checked_mul_func(a_u8.unwrap(), 2_u8);
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

    // Example for u16
    let a_u16 = small_uint_checked_mul_func(u16::MAX / 3, 2_u16);
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

    let b_u16 = small_uint_checked_mul_func(a_u16.unwrap(), 2_u16);
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

    // Example for u32
    let a_u32 = small_uint_checked_mul_func(u32::MAX / 3, 2_u32);
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

    let b_u32 = small_uint_checked_mul_func(a_u32.unwrap(), 2_u32);
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

    // Example for u64
    let a_u64 = small_uint_checked_mul_func(u64::MAX / 3, 2_u64);
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

    let b_u64 = small_uint_checked_mul_func(a_u64.unwrap(), 2_u64);
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

    // Example for u128
    let a_u128 = small_uint_checked_mul_func(u128::MAX / 3, 2_u128);
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

    let b_u128 = small_uint_checked_mul_func(a_u128.unwrap(), 2_u128);
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

    // Example for usize
    let a_usize = small_uint_checked_mul_func(usize::MAX / 3, 2_usize);
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

    let b_usize = small_uint_checked_mul_func(a_usize.unwrap(), 2_usize);
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

    // Example for ShortUnion
    let a_shortunion = small_uint_checked_mul_func((u16::MAX / 3).into_shortunion(), 2_u16.into_shortunion());
    match a_shortunion
    {
        Some(a) => {
                println!("{} * 2 = {}", (u16::MAX / 3).into_shortunion(), a);
                assert_eq!(a.get(), 43690_u16);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(a_shortunion, None);
            },
    }

    let b_shortunion = small_uint_checked_mul_func(a_shortunion.unwrap(), 2_u16.into_shortunion());
    match b_shortunion
    {
        Some(b) => {
                println!("{} * 2 = {}", a_shortunion.unwrap(), b);
                assert_eq!(b.get(), 21844_u16);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(b_shortunion, None);
            },
    }

    // Example for IntUnion
    let a_intunion = small_uint_checked_mul_func((u32::MAX / 3).into_intunion(), 2_u32.into_intunion());
    match a_intunion
    {
        Some(a) => {
                println!("{} * 2 = {}", (u32::MAX / 3).into_intunion(), a);
                assert_eq!(a.get(), 2863311530_u32);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(a_intunion, None);
            },
    }

    let b_intunion = small_uint_checked_mul_func(a_intunion.unwrap(), 2_u32.into_intunion());
    match b_intunion
    {
        Some(b) => {
                println!("{} * 2 = {}", a_intunion.unwrap(), b);
                assert_eq!(b.get(), 1431655764_u32);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(b_intunion, None);
            },
    }

    // Example for LongUnion
    let a_longunion = small_uint_checked_mul_func((u64::MAX / 3).into_longunion(), 2_u64.into_longunion());
    match a_longunion
    {
        Some(a) => {
                println!("{} * 2 = {}", (u64::MAX / 3).into_longunion(), a);
                assert_eq!(a.get(), 12297829382473034410_u64);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(a_longunion, None);
            },
    }

    let b_longunion = small_uint_checked_mul_func(a_longunion.unwrap(), 2_u64.into_longunion());
    match b_longunion
    {
        Some(b) => {
                println!("{} * 2 = {}", a_longunion.unwrap(), b);
                assert_eq!(b.get(), 6148914691236517204_u64);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(b_longunion, None);
            },
    }

    // Example for LongerUnion
    let a_longerunion = small_uint_checked_mul_func((u128::MAX / 3).into_longerunion(), 2_u128.into_longerunion());
    match a_longerunion
    {
        Some(a) => {
                println!("{} * 2 = {}", (u128::MAX / 3).into_longerunion(), a);
                assert_eq!(a.get(), 226854911280625642308916404954512140970_u128);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(a_longerunion, None);
            },
    }

    let b_longerunion = small_uint_checked_mul_func(a_longerunion.unwrap(), 2_u128.into_longerunion());
    match b_longerunion
    {
        Some(b) => {
                println!("{} * 2 = {}", a_longerunion.unwrap(), b);
                assert_eq!(b.get(), 113427455640312821154458202477256070484_u128);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(b_longerunion, None);
            },
    }

    // Example for SizeUnion
    let a_sizeunion = small_uint_checked_mul_func((usize::MAX / 3).into_sizeunion(), 2_usize.into_sizeunion());
    match a_sizeunion
    {
        Some(a) => {
                println!("{} * 2 = {}", (usize::MAX / 3).into_sizeunion(), a.into_sizeunion());
                assert_eq!(a.get(), 12297829382473034410_usize);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(a_sizeunion, None);
            },
    }

    let b_sizeunion = small_uint_checked_mul_func(a_sizeunion.unwrap(), 2_usize.into_sizeunion());
    match b_sizeunion
    {
        Some(b) => {
                println!("{} * 2 = {}", a_sizeunion.unwrap(), b);
                assert_eq!(b.get(), 6148914691236517204_usize);
            },
        None => {
                println!("Overflow happened.");
                assert_eq!(b_sizeunion, None);
            },
    }
    println!("--------------------------------------");
}

fn small_uint_checked_mul_func<T: cryptocol::number::SmallUInt>(lhs: T, rhs: T) -> Option<T>
{
    lhs.checked_mul(rhs)
}

fn small_uint_unchecked_mul()
{
    println!("small_uint_unchecked_add()");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = small_uint_unchecked_mul_func(u8::MAX / 3, 2_u8);
    println!("{} * 2 = {}", u8::MAX / 3, a_u8);
    assert_eq!(a_u8, 170_u8);

    // It will panic
    // let b_u8 = small_uint_unchecked_mul_func(a_u8, 2_u8);

    // Example for u16
    let a_u16 = small_uint_unchecked_mul_func(u16::MAX / 3, 2_u16);
    println!("{} * 2 = {}", u16::MAX / 3, a_u16);
    assert_eq!(a_u16, 43690_u16);

    // It will panic
    // let b_u16 = small_uint_unchecked_mul_func(a_u16, 2_u16);

    // Example for u32
    let a_u32 = small_uint_unchecked_mul_func(u32::MAX / 3, 2_u32);
    println!("{} * 2 = {}", u32::MAX / 3, a_u32);
    assert_eq!(a_u32, 2863311530_u32);

    // It will panic
    // let b_u32 = small_uint_unchecked_mul_func(a_u32, 2_u32);

    // Example for u64
    let a_u64 = small_uint_unchecked_mul_func(u64::MAX / 3, 2_u64);
    println!("{} * 2 = {}", u64::MAX / 3, a_u64);
    assert_eq!(a_u64, 12297829382473034410_u64);

    // It will panic
    // let b_u64 = small_uint_unchecked_mul_func(a_u64, 2_u64);

    // Example for u128
    let a_u128 = small_uint_unchecked_mul_func(u128::MAX / 3, 2_u128);
    println!("{} * 2 = {}", u128::MAX / 3, a_u128);
    assert_eq!(a_u128, 226854911280625642308916404954512140970_u128);

    // It will panic
    // let b_u128 = small_uint_unchecked_mul_func(a_u128, 2_u128);

    // Example for usize
    let a_usize = small_uint_unchecked_mul_func(usize::MAX / 3, 2_usize);
    println!("{} * 2 = {}", usize::MAX / 3, a_usize);
    assert_eq!(a_usize, 12297829382473034410_usize);

    // It will panic
    // let b_usize = small_uint_unchecked_mul_func(a_usize, 2_usize);

    // Example for ShortUnion
    let a_shortunion = small_uint_unchecked_mul_func((u16::MAX / 3).into_shortunion(), 2_u16.into_shortunion());
    println!("{} * 2 = {}", (u16::MAX / 3).into_shortunion(), a_shortunion);
    assert_eq!(a_shortunion.get(), 43690_u16);


    // It will panic
    // let b_shortunion = small_uint_unchecked_mul_func(a_shortunion, 2_u16.into_shortunion());

    // Example for IntUnion
    let a_intunion = small_uint_unchecked_mul_func((u32::MAX / 3).into_intunion(), 2_u32.into_intunion());
    println!("{} * 2 = {}", (u32::MAX / 3).into_intunion(), a_intunion);
    assert_eq!(a_intunion.get(), 2863311530_u32);

    // It will panic
    // let b_intunion = small_uint_unchecked_mul_func(a_intunion, 2_u32.into_intunion());

    // Example for LongUnion
    let a_longunion = small_uint_unchecked_mul_func((u64::MAX / 3).into_longunion(), 2_u64.into_longunion());
    println!("{} * 2 = {}", (u64::MAX / 3).into_longunion(), a_longunion);
    assert_eq!(a_longunion.get(), 12297829382473034410_u64);

    // It will panic
    // let b_longunion = small_uint_unchecked_mul_func(a_longunion, 2_u64.into_longunion());

    // Example for LongerUnion
    let a_longerunion = small_uint_unchecked_mul_func((u128::MAX / 3).into_longerunion(), 2_u128.into_longerunion());
    println!("{} * 2 = {}", (u128::MAX / 3).into_longerunion(), a_longerunion);
    assert_eq!(a_longerunion.get(), 226854911280625642308916404954512140970_u128);

    // It will panic
    // let b_longerunion = small_uint_unchecked_mul_func(a_longerunion, 2_u128.into_longerunion());

    // Example for SizeUnion
    let a_sizeunion = small_uint_unchecked_mul_func((usize::MAX / 3).into_sizeunion(), 2_usize.into_sizeunion());
    println!("{} * 2 = {}", (usize::MAX / 3).into_sizeunion(), a_sizeunion);
    assert_eq!(a_sizeunion.get(), 12297829382473034410_usize);

    // It will panic
    // let b_sizeunion = small_uint_unchecked_mul_func(a_sizeunion, 2_usize.into_sizeunion());
    println!("--------------------------------------");
}

fn small_uint_unchecked_mul_func<T: cryptocol::number::SmallUInt>(lhs: T, rhs: T) -> T
{
    lhs.unchecked_mul(rhs)
}

fn small_uint_saturating_mul()
{
    println!("small_uint_saturating_mul()");
    // Example for u8
    let a_u8 = small_uint_saturating_mul_func(u8::MAX / 3, 2_u8);
    println!("{} * 2 = {}", u8::MAX / 3, a_u8);
    assert_eq!(a_u8, 170_u8);

    let b_u8 = small_uint_saturating_mul_func(a_u8, 2_u8);
    println!("{} * 2 = {}", a_u8, b_u8);
    assert_eq!(b_u8, u8::MAX);

    // Example for u16
    let a_u16 = small_uint_saturating_mul_func(u16::MAX / 3, 2_u16);
    println!("{} * 2 = {}", u16::MAX / 3, a_u16);
    assert_eq!(a_u16, 43690_u16);

    let b_u16 = small_uint_saturating_mul_func(a_u16, 2_u16);
    println!("{} * 2 = {}", a_u16, b_u16);
    assert_eq!(b_u16, u16::MAX);

    // Example for u32
    let a_u32 = small_uint_saturating_mul_func(u32::MAX / 3, 2_u32);
    println!("{} * 2 = {}", u32::MAX / 3, a_u32);
    assert_eq!(a_u32, 2863311530_u32);

    let b_u32 = small_uint_saturating_mul_func(a_u32, 2_u32);
    println!("{} * 2 = {}", a_u32, b_u32);
    assert_eq!(b_u32, u32::MAX);

    // Example for u64
    let a_u64 = small_uint_saturating_mul_func(u64::MAX / 3, 2_u64);
    println!("{} * 2 = {}", u64::MAX / 3, a_u64);
    assert_eq!(a_u64, 12297829382473034410_u64);

    let b_u64 = small_uint_saturating_mul_func(a_u64, 2_u64);
    println!("{} * 2 = {}", a_u64, b_u64);
    assert_eq!(b_u64, u64::MAX);

    // Example for u128
    let a_u128 = small_uint_saturating_mul_func(u128::MAX / 3, 2_u128);
    println!("{} * 2 = {}", u128::MAX / 3, a_u128);
    assert_eq!(a_u128, 226854911280625642308916404954512140970_u128);

    let b_u128 = small_uint_saturating_mul_func(a_u128, 2_u128);
    println!("{} * 2 = {}",a_u128, b_u128);
    assert_eq!(b_u128, u128::MAX);

    // Example for usize
    let a_usize = small_uint_saturating_mul_func(usize::MAX / 3, 2_usize);
    println!("{} * 2 = {}", usize::MAX / 3, a_usize);
    assert_eq!(a_usize, 12297829382473034410_usize);

    let b_usize = small_uint_saturating_mul_func(a_usize, 2_usize);
    println!("{} * 2 = {}", a_usize, b_usize);
    assert_eq!(b_usize, usize::MAX);
    println!("--------------------------------------");
}

fn small_uint_saturating_mul_func<T: cryptocol::number::SmallUInt>(lhs: T, rhs: T) -> T
{
    lhs.saturating_mul(rhs)
}

fn small_uint_modular_mul()
{
    println!("small_uint_modular_mul()");
    // Example for u8
    let a_u8 = 90_u8;
    let b_u8 = small_uint_modular_mul_func(a_u8, 2, 200);
    println!("{} * 2 = {} (mod 200)", a_u8, b_u8);
    assert_eq!(b_u8, 180_u8);

    let c_u8 = small_uint_modular_mul_func(b_u8, 2, 200);
    println!("{} * 2 = {} (mod 200)", b_u8, c_u8);
    assert_eq!(c_u8, 160_u8);

    // Example for u16
    let a_u16 = 9000_u16;
    let b_u16 = small_uint_modular_mul_func(a_u16, 2, 20000);
    println!("{} * 2 = {}", a_u16, b_u16);
    assert_eq!(b_u16, 18000_u16);

    let c_u16 = small_uint_modular_mul_func(b_u16, 2, 20000);
    println!("{} * 2 = {}", b_u16, c_u16);
    assert_eq!(c_u16, 16000_u16);

    // Example for u32
    let a_u32 = 9000000_u32;
    let b_u32 = small_uint_modular_mul_func(a_u32, 2, 20000000);
    println!("{} * 2 = {}", a_u32, b_u32);
    assert_eq!(b_u32, 18000000_u32);

    let c_u32 = small_uint_modular_mul_func(b_u32, 2, 20000000);
    println!("{} * 2 = {}", b_u32, c_u32);
    assert_eq!(c_u32, 16000000_u32);

    // Example for u64
    let a_u64 = 900000000000_u64;
    let b_u64 = small_uint_modular_mul_func(a_u64, 2, 2000000000000);
    println!("{} * 2 = {}", a_u64, b_u64);
    assert_eq!(b_u64, 1800000000000_u64);

    let c_u64 = small_uint_modular_mul_func(b_u64, 2, 2000000000000);
    println!("{} * 2 = {}", b_u64, c_u64);
    assert_eq!(c_u64, 1600000000000_u64);

    // Example for u128
    let a_u128 = 90000000000000000000000_u128;
    let b_u128 = small_uint_modular_mul_func(a_u128, 2, 200000000000000000000000);
    println!("{} * 2 = {}", a_u128, b_u128);
    assert_eq!(b_u128, 180000000000000000000000_u128);

    let c_u128 = small_uint_modular_mul_func(b_u128, 2, 200000000000000000000000);
    println!("{} * 2 = {}", b_u128, c_u128);
    assert_eq!(c_u128, 160000000000000000000000_u128);

    // Example for usize
    let a_usize = 900000000000_usize;
    let b_usize = small_uint_modular_mul_func(a_usize, 2, 2000000000000);
    println!("{} * 2 = {}", a_usize, b_usize);
    assert_eq!(b_usize, 1800000000000_usize);

    let c_usize = small_uint_modular_mul_func(b_usize, 2, 2000000000000);
    println!("{} * 2 = {}", b_u64, c_u64);
    assert_eq!(c_usize, 1600000000000_usize);
    println!("--------------------------------------");
}

fn small_uint_modular_mul_func<T: cryptocol::number::SmallUInt>(lhs: T, rhs: T, modulo: T) -> T
{
    lhs.modular_mul(rhs, modulo)
}

fn small_uint_div_main()
{
    small_uint_wrapping_div();
    small_uint_overflowing_div();
    small_uint_checked_div();
    small_uint_unchecked_div();
    small_uint_saturating_div();
}

fn small_uint_wrapping_div()
{
    println!("small_uint_wrapping_div()");
    // Example for u8
    let a_u8 = small_uint_wrapping_div_func(u8::MAX / 3, 2_u8);
    println!("{} / 2 = {}", u8::MAX / 3, a_u8);
    assert_eq!(a_u8, 42_u8);

    // Example for u16
    let a_u16 = small_uint_wrapping_div_func(u16::MAX / 3, 2_u16);
    println!("{} / 2 = {}", u16::MAX / 3, a_u16);
    assert_eq!(a_u16, 10922_u16);

    // Example for u32
    let a_u32 = small_uint_wrapping_div_func(u32::MAX / 3, 2_u32);
    println!("{} / 2 = {}", u32::MAX / 3, a_u32);
    assert_eq!(a_u32, 715827882_u32);

    // Example for u64
    let a_u64 = small_uint_wrapping_div_func(u64::MAX / 3, 2_u64);
    println!("{} / 2 = {}", u64::MAX / 3, a_u64);
    assert_eq!(a_u64, 3074457345618258602_u64);

    // Example for u128
    let a_u128 = small_uint_wrapping_div_func(u128::MAX / 3, 2_u128);
    println!("{} / 2 = {}", u128::MAX / 3, a_u128);
    assert_eq!(a_u128, 56713727820156410577229101238628035242_u128);

    // Example for usize
    let a_usize = small_uint_wrapping_div_func(usize::MAX / 3, 2_usize);
    println!("{} / 2 = {}", usize::MAX / 3, a_usize);
    assert_eq!(a_usize, 3074457345618258602_usize);

    // It will panic.
    // let a_panic = small_uint_wrapping_div_func(usize::MAX / 3, 0_usize);
    println!("--------------------------------------");
}

fn small_uint_wrapping_div_func<T: cryptocol::number::SmallUInt>(lhs: T, rhs: T) -> T
{
    lhs.wrapping_div(rhs)
}

fn small_uint_overflowing_div()
{
    println!("small_uint_overflowing_div()");
    // Example for u8
    let a_u8 = small_uint_overflowing_div_func(u8::MAX / 3, 2_u8);
    println!("{} / 2 = {}\nOverflow = {}", u8::MAX / 3, a_u8.0, a_u8.1);
    assert_eq!(a_u8.0, 42_u8);
    assert_eq!(a_u8.1, false);

    // Example for u16
    let a_u16 = small_uint_overflowing_div_func(u16::MAX / 3, 2_u16);
    println!("{} / 2 = {}\nOverflow = {}", u16::MAX / 3, a_u16.0, a_u16.1);
    assert_eq!(a_u16.0, 10922_u16);
    assert_eq!(a_u16.1, false);
 
    // Example for u32
    let a_u32 = small_uint_overflowing_div_func(u32::MAX / 3, 2_u32);
    println!("{} / 2 = {}\nOverflow = {}", u32::MAX / 3, a_u32.0, a_u32.1);
    assert_eq!(a_u32.0, 715827882_u32);
    assert_eq!(a_u32.1, false);
 
    // Example for u64
    let a_u64 = small_uint_overflowing_div_func(u64::MAX / 3, 2_u64);
    println!("{} / 2 = {}\nOverflow = {}", u64::MAX / 3, a_u64.0, a_u64.1);
    assert_eq!(a_u64.0, 3074457345618258602_u64);
    assert_eq!(a_u64.1, false);
 
    // Example for u128
    let a_u128 = small_uint_overflowing_div_func(u128::MAX / 3, 2_u128);
    println!("{} / 2 = {}\nOverflow = {}", u128::MAX / 3, a_u128.0, a_u128.1);
    assert_eq!(a_u128.0, 56713727820156410577229101238628035242_u128);
    assert_eq!(a_u128.1, false);
 
    // Example for usize
    let a_usize = small_uint_overflowing_div_func(usize::MAX / 3, 2_usize);
    println!("{} / 2 = {}\nOverflow = {}", usize::MAX / 3, a_usize.0, a_usize.1);
    assert_eq!(a_usize.0, 3074457345618258602_usize);
    assert_eq!(a_usize.1, false);
 
    // It will panic.
    // let a_panic = small_uint_overflowing_div_func(a_usize.0, 0_usize);
    println!("--------------------------------------");
}

fn small_uint_overflowing_div_func<T: cryptocol::number::SmallUInt>(lhs: T, rhs: T) -> (T, bool)
{
    lhs.overflowing_div(rhs)
}

fn small_uint_checked_div()
{
    println!("small_uint_checked_div()");
    // Example for u8
    let a_u8 = small_uint_checked_div_func(u8::MAX / 3, 2_u8);
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

    let b_u8 = small_uint_checked_div_func(u8::MAX / 3, 0_u8);
    match b_u8
    {
        Some(b) => { println!("{} / 2 = {}", u8::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_u8, None);
            },
    }

    // Example for u16
    let a_u16 = small_uint_checked_div_func(u16::MAX / 3, 2_u16);
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

    let b_u16 = small_uint_checked_div_func(u16::MAX / 3, 0_u16);
    match b_u16
    {
        Some(b) => { println!("{} / 2 = {}", u16::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_u16, None);
            },
    }

    // Example for u32
    let a_u32 = small_uint_checked_div_func(u32::MAX / 3, 2_u32);
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

    let b_u32 = small_uint_checked_div_func(u32::MAX / 3, 0_u32);
    match b_u32
    {
        Some(b) => { println!("{} / 2 = {}", u32::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_u32, None);
            },
    }

    // Example for u64
    let a_u64 = small_uint_checked_div_func(u64::MAX / 3, 2_u64);
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

    let b_u64 = small_uint_checked_div_func(u64::MAX / 3, 0_u64);
    match b_u64
    {
        Some(b) => { println!("{} / 2 = {}", u64::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_u64, None);
            },
    }

    // Example for u128
    let a_u128 = small_uint_checked_div_func(u128::MAX / 3, 2_u128);
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

    let b_u128 = small_uint_checked_div_func(u128::MAX / 3, 0_u128);
    match b_u128
    {
        Some(b) => { println!("{} / 2 = {}", u128::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_u128, None);
            },
    }

    // Example for usize
    let a_usize = small_uint_checked_div_func(usize::MAX / 3, 2_usize);
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

    let b_usize = small_uint_checked_div_func(usize::MAX / 3, 0_usize);
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

fn small_uint_checked_div_func<T: cryptocol::number::SmallUInt>(lhs: T, rhs: T) -> Option<T>
{
    lhs.checked_div(rhs)
}

fn small_uint_unchecked_div()
{
    println!("small_uint_unchecked_div()");
    // Example for u8
    let a_u8 = small_uint_unchecked_div_func(u8::MAX / 3, 2_u8);
    println!("{} / 2 = {}", u8::MAX / 3, a_u8);
    assert_eq!(a_u8, 42_u8);
    // It will panic.
    // let b_u8 = small_uint_unchecked_div_func(u8::MAX / 3, 0_u8);

    // Example for u16
    let a_u16 = small_uint_unchecked_div_func(u16::MAX / 3, 2_u16);
    println!("{} / 2 = {}", u16::MAX / 3, a_u16);
    assert_eq!(a_u16, 10922_u16);
    // It will panic.
    // let b_u16 = small_uint_unchecked_div_func(u16::MAX / 3, 0_u16);

    // Example for u32
    let a_u32 = small_uint_unchecked_div_func(u32::MAX / 3, 2_u32);
    println!("{} / 2 = {}", u32::MAX / 3, a_u32);
    assert_eq!(a_u32, 715827882_u32);
    // It will panic.
    // let b_u32 = small_uint_unchecked_div_func(u32::MAX / 3, 0_u32);

    // Example for u64
    let a_u64 = small_uint_unchecked_div_func(u64::MAX / 3, 2_u64);
    println!("{} / 2 = {}", u64::MAX / 3, a_u64);
    assert_eq!(a_u64, 3074457345618258602_u64);
    // It will panic.
    // let b_u64 = small_uint_unchecked_div_func(u64::MAX / 3, 0_u64);

    // Example for u128
    let a_u128 = small_uint_unchecked_div_func(u128::MAX / 3, 2_u128);
    println!("{} / 2 = {}", u128::MAX / 3, a_u128);
    assert_eq!(a_u128, 56713727820156410577229101238628035242_u128);
    // It will panic.
    // let b_u128 = small_uint_unchecked_div_func(u128::MAX / 3, 0_u128);

    // Example for usize
    let a_usize = small_uint_unchecked_div_func(usize::MAX / 3, 2_usize);
    println!("{} / 2 = {}", usize::MAX / 3, a_usize);
    // It will panic.
    // let b_usize = small_uint_unchecked_div_func(usize::MAX / 3, 0_usize);
    println!("--------------------------------------");
}

fn small_uint_unchecked_div_func<T: cryptocol::number::SmallUInt>(lhs: T, rhs: T) -> T
{
    lhs.unchecked_div(rhs)
}

fn small_uint_saturating_div()
{
    println!("small_uint_saturating_div()");
    // Example for u8
    let a_u8 = small_uint_saturating_div_func(u8::MAX / 3, 2_u8);
    println!("{} / 2 = {}", u8::MAX / 3, a_u8);
    assert_eq!(a_u8, 42_u8);

    // Example for u16
    let a_u16 = small_uint_saturating_div_func(u16::MAX / 3, 2_u16);
    println!("{} / 2 = {}", u16::MAX / 3, a_u16);
    assert_eq!(a_u16, 10922_u16);

    // Example for u32
    let a_u32 = small_uint_saturating_div_func(u32::MAX / 3, 2_u32);
    println!("{} / 2 = {}", u32::MAX / 3, a_u32);
    assert_eq!(a_u32, 715827882_u32);

    // Example for u64
    let a_u64 = small_uint_saturating_div_func(u64::MAX / 3, 2_u64);
    println!("{} / 2 = {}", u64::MAX / 3, a_u64);
    assert_eq!(a_u64, 3074457345618258602_u64);

    // Example for u128
    let a_u128 = small_uint_saturating_div_func(u128::MAX / 3, 2_u128);
    println!("{} / 2 = {}", u128::MAX / 3, a_u128);
    assert_eq!(a_u128, 56713727820156410577229101238628035242_u128);

    // Example for usize
    let a_usize = small_uint_saturating_div_func(usize::MAX / 3, 2_usize);
    println!("{} / 2 = {}", usize::MAX / 3, a_usize);
    assert_eq!(a_usize, 3074457345618258602_usize);

    // It will panic.
    // let a_panic = small_uint_saturating_div_func(usize::MAX / 3, 0_usize);
    println!("--------------------------------------");
}

fn small_uint_saturating_div_func<T: cryptocol::number::SmallUInt>(lhs: T, rhs: T) -> T
{
    lhs.saturating_div(rhs)
}

fn small_uint_rem_main()
{
    small_uint_wrapping_rem();
    small_uint_overflowing_rem();
    small_uint_checked_rem();
    small_uint_unchecked_rem();
}

fn small_uint_wrapping_rem()
{
    println!("small_uint_wrapping_rem()");
    // Example for u8
    let a_u8 = small_uint_wrapping_rem_func(u8::MAX / 3, 3_u8);
    println!("{} % 3 = {}", u8::MAX / 3, a_u8);
    assert_eq!(a_u8, 1_u8);

    // Example for u16
    let a_u16 = small_uint_wrapping_rem_func(u16::MAX / 3, 3_u16);
    println!("{} % 3 = {}", u16::MAX / 3, a_u16);
    assert_eq!(a_u16, 2_u16);

    // Example for u32
    let a_u32 = small_uint_wrapping_rem_func(u32::MAX / 3, 3_u32);
    println!("{} % 3 = {}", u32::MAX / 3, a_u32);
    assert_eq!(a_u32, 1_u32);

    // Example for u64
    let a_u64 = small_uint_wrapping_rem_func(u64::MAX / 3, 3_u64);
    println!("{} % 3 = {}", u64::MAX / 3, a_u64);
    assert_eq!(a_u64, 2_u64);

    // Example for u128
    let a_u128 = small_uint_wrapping_rem_func(u128::MAX / 3, 3_u128);
    println!("{} % 3 = {}", u128::MAX / 3, a_u128);
    assert_eq!(a_u128, 1_u128);

    // Example for usize
    let a_usize = small_uint_wrapping_rem_func(usize::MAX / 3, 3_usize);
    println!("{} % 3 = {}", usize::MAX / 3, a_usize);
    assert_eq!(a_usize, 2_usize);

    // It will panic.
    // let a_panic = small_uint_wrapping_rem_func(usize::MAX / 3, 0_usize);
    println!("--------------------------------------");
}

fn small_uint_wrapping_rem_func<T: cryptocol::number::SmallUInt>(lhs: T, rhs: T) -> T
{
    lhs.wrapping_rem(rhs)
}

fn small_uint_overflowing_rem()
{
    println!("small_uint_overflowing_rem()");
    // Example for u8
    let a_u8 = small_uint_overflowing_rem_func(u8::MAX / 3, 3_u8);
    println!("{} % 3 = {}\nOverflow = {}", u8::MAX / 3, a_u8.0, a_u8.1);
    assert_eq!(a_u8.0, 1_u8);
    assert_eq!(a_u8.1, false);

    // Example for u16
    let a_u16 = small_uint_overflowing_rem_func(u16::MAX / 3, 3_u16);
    println!("{} % 3 = {}\nOverflow = {}", u16::MAX / 3, a_u16.0, a_u16.1);
    assert_eq!(a_u16.0, 2_u16);
    assert_eq!(a_u16.1, false);
 
    // Example for u32
    let a_u32 = small_uint_overflowing_rem_func(u32::MAX / 3, 3_u32);
    println!("{} % 3 = {}\nOverflow = {}", u32::MAX / 3, a_u32.0, a_u32.1);
    assert_eq!(a_u32.0, 1_u32);
    assert_eq!(a_u32.1, false);
 
    // Example for u64
    let a_u64 = small_uint_overflowing_rem_func(u64::MAX / 3, 3_u64);
    println!("{} % 3 = {}\nOverflow = {}", u64::MAX / 3, a_u64.0, a_u64.1);
    assert_eq!(a_u64.0, 2_u64);
    assert_eq!(a_u64.1, false);
 
    // Example for u128
    let a_u128 = small_uint_overflowing_rem_func(u128::MAX / 3, 3_u128);
    println!("{} % 3 = {}\nOverflow = {}", u128::MAX / 3, a_u128.0, a_u128.1);
    assert_eq!(a_u128.0, 1_u128);
    assert_eq!(a_u128.1, false);
 
    // Example for usize
    let a_usize = small_uint_overflowing_rem_func(usize::MAX / 3, 3_usize);
    println!("{} % 3 = {}\nOverflow = {}", usize::MAX / 3, a_usize.0, a_usize.1);
    assert_eq!(a_usize.0, 2_usize);
    assert_eq!(a_usize.1, false);
 
    // It will panic.
    // let a_panic = small_uint_overflowing_rem_func(a_usize.0, 0_usize);
    println!("--------------------------------------");
}

fn small_uint_overflowing_rem_func<T: cryptocol::number::SmallUInt>(lhs: T, rhs: T) -> (T, bool)
{
    lhs.overflowing_rem(rhs)
}

fn small_uint_checked_rem()
{
    println!("small_uint_checked_rem()");
    // Example for u8
    let a_u8 = small_uint_checked_rem_func(u8::MAX / 3, 3_u8);
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

    let b_u8 = small_uint_checked_rem_func(u8::MAX / 3, 0_u8);
    match b_u8
    {
        Some(b) => { println!("{} % 3 = {}", u8::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_u8, None);
            },
    }

    // Example for u16
    let a_u16 = small_uint_checked_rem_func(u16::MAX / 3, 3_u16);
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

    let b_u16 = small_uint_checked_rem_func(u16::MAX / 3, 0_u16);
    match b_u16
    {
        Some(b) => { println!("{} % 3 = {}", u16::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_u16, None);
            },
    }

    // Example for u32
    let a_u32 = small_uint_checked_rem_func(u32::MAX / 3, 3_u32);
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

    let b_u32 = small_uint_checked_rem_func(u32::MAX / 3, 0_u32);
    match b_u32
    {
        Some(b) => { println!("{} % 3 = {}", u32::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_u32, None);
            },
    }

    // Example for u64
    let a_u64 = small_uint_checked_rem_func(u64::MAX / 3, 3_u64);
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

    let b_u64 = small_uint_checked_rem_func(u64::MAX / 3, 0_u64);
    match b_u64
    {
        Some(b) => { println!("{} % 3 = {}", u64::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_u64, None);
            },
    }

    // Example for u128
    let a_u128 = small_uint_checked_rem_func(u128::MAX / 3, 3_u128);
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

    let b_u128 = small_uint_checked_rem_func(u128::MAX / 3, 0_u128);
    match b_u128
    {
        Some(b) => { println!("{} % 3 = {}", u128::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_u128, None);
            },
    }

    // Example for usize
    let a_usize = small_uint_checked_rem_func(usize::MAX / 3, 3_usize);
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

    let b_usize = small_uint_checked_rem_func(usize::MAX / 3, 0_usize);
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

fn small_uint_checked_rem_func<T: cryptocol::number::SmallUInt>(lhs: T, rhs: T) -> Option<T>
{
    lhs.checked_rem(rhs)
}

fn small_uint_unchecked_rem()
{
    println!("small_uint_checked_rem()");
    // Example for u8
    let a_u8 = small_uint_unchecked_rem_func(u8::MAX / 3, 3_u8);
    println!("{} % 3 = {}", u8::MAX / 3, a_u8);
    assert_eq!(a_u8, 1_u8);

    // It will panic.
    // let b_u8 = small_uint_unchecked_rem_func(u8::MAX / 3, 0_u8);

    // Example for u16
    let a_u16 = small_uint_unchecked_rem_func(u16::MAX / 3, 3_u16);
    println!("{} % 3 = {}", u16::MAX / 3, a_u16);
    assert_eq!(a_u16, 2_u16);

    // It will panic.
    // let b_u16 = small_uint_unchecked_rem_func(u16::MAX / 3, 0_u16);

    // Example for u32
    let a_u32 = small_uint_unchecked_rem_func(u32::MAX / 3, 3_u32);
    println!("{} % 3 = {}", u32::MAX / 3, a_u32);
    assert_eq!(a_u32, 1_u32);
    // It will panic.
    // let b_u32 = small_uint_unchecked_rem_func(u32::MAX / 3, 0_u32);

    // Example for u64
    let a_u64 = small_uint_unchecked_rem_func(u64::MAX / 3, 3_u64);
    println!("{} % 3 = {}", u64::MAX / 3, a_u64);
    assert_eq!(a_u64, 2_u64);

    // It will panic.
    // let b_u64 = small_uint_unchecked_rem_func(u64::MAX / 3, 0_u64);

    // Example for u128
    let a_u128 = small_uint_unchecked_rem_func(u128::MAX / 3, 3_u128);
    println!("{} % 3 = {}", u128::MAX / 3, a_u128);
    assert_eq!(a_u128, 1_u128);

    // It will panic.
    // let b_u128 = small_uint_unchecked_rem_func(u128::MAX / 3, 0_u128);

    // Example for u128
    let a_usize = small_uint_unchecked_rem_func(usize::MAX / 3, 3_usize);
    println!("{} % 3 = {}", usize::MAX / 3, a_usize);
    assert_eq!(a_usize, 2_usize);

    // It will panic.
    // let b_usize = small_uint_unchecked_rem_func(usize::MAX / 3, 0_usize);
    println!("--------------------------------------");
}

fn small_uint_unchecked_rem_func<T: cryptocol::number::SmallUInt>(lhs: T, rhs: T) -> T
{
    lhs.unchecked_rem(rhs)
}

fn small_uint_pow_main()
{
    small_uint_pow();
    small_uint_wrapping_pow();
    small_uint_overflowing_pow();
    small_uint_checked_pow();
    small_uint_unchecked_pow();
    small_uint_saturating_pow();
    small_uint_modular_pow();
}

fn small_uint_pow()
{
    println!("small_uint_pow()");
    use cryptocol::number::*;

    // Example for u8
    let a_u8 = small_uint_pow_func(3_u8, 5_u32);
    println!("3 ** 5 = {}", a_u8);
    assert_eq!(a_u8, 243_u8);
    // It will panic.
    // println!("3 ** 6 = {}", small_uint_pow_func(3_u8, 6_u32));

    // Example for u16
    let a_u16 = small_uint_pow_func(9_u16, 5_u32);
    println!("9 ** 5 = {}", a_u16);
    assert_eq!(a_u16, 59049_u16);
    // It will panic.
    // println!("9 ** 6 = {}", small_uint_pow_func(9_u16, 6_u32));

    // Example for u32
    let a_u32 = small_uint_pow_func(81_u32, 5_u32);
    println!("81 ** 5 = {}", a_u32);
    assert_eq!(a_u32, 3486784401_u32);
    // It will panic.
    // println!("81 ** 6 = {}", small_uint_pow_func(81_u32, 6_u32));

    // Example for u64
    let a_u64 = small_uint_pow_func(6561_u64, 5_u32);
    println!("6561 ** 5 = {}", a_u64);
    assert_eq!(a_u64, 12157665459056928801_u64);
    // It will panic.
    // println!("6561 ** 6 = {}", small_uint_pow_func(6561_u64, 6_u32));

    // Example for u128
    let a_u128 = small_uint_pow_func(43046721_u128, 5_u32);
    println!("43046721 ** 5 = {}", a_u128);
    assert_eq!(a_u128, 147808829414345923316083210206383297601_u128);
    // It will panic.
    // println!("43046721 ** 6 = {}", small_uint_pow_func(43046721_u64, 6_u32));

    // Example for usize
    let a_usize = small_uint_pow_func(6561_usize, 5_u32);
    println!("6561 ** 5 = {}", a_usize);
    assert_eq!(a_usize, 12157665459056928801_usize);
    // It will panic.
    // println!("6561 ** 6 = {}", small_uint_pow_func(6561_usize, 6_u32));

    // Example for ShortUnion
    let a_ushort = ShortUnion::new_with(9);
    let b_ushort = small_uint_pow_func(a_ushort, 5_u32);
    println!("9 ** 5 = {}", b_ushort);
    assert_eq!(b_ushort.get(), 59049_u16);
    // It will panic.
    // println!("9 ** 5 = {}", small_uint_pow_func(a_ushort, 6_u32));

    // Example for IntUnion
    let a_uint = IntUnion::new_with(81);
    let b_uint = small_uint_pow_func(a_uint, 5_u32);
    println!("81 ** 5 = {}", b_uint);
    assert_eq!(b_uint.get(), 3486784401_u32);
    // It will panic.
    // println!("81 ** 6 = {}", small_uint_pow_func(a_uint, 6_u32));

    // Example for LongUnion
    let a_ulong = LongUnion::new_with(6561);
    let b_ulong = small_uint_pow_func(a_ulong, 5_u32);
    println!("6561 ** 5 = {}", b_ulong);
    assert_eq!(b_ulong.get(), 12157665459056928801_u64);
    // It will panic.
    // println!("6561 ** 6 = {}", small_uint_pow_func(a_ulong, 6_u32));

    // Example for LongerUnion
    let a_ulonger = LongerUnion::new_with(43046721);
    let b_ulonger = small_uint_pow_func(a_ulonger, 5_u32);
    println!("43046721 ** 5 = {}", b_ulonger);
    assert_eq!(b_ulonger.get(), 147808829414345923316083210206383297601_u128);
    // It will panic.
    // println!("43046721 ** 6 = {}", small_uint_pow_func(a_ulonger, 6_u32));

    // Example for SizeUnion
    let a_size = SizeUnion::new_with(6561);
    let b_size = small_uint_pow_func(a_size, 5_u32);
    println!("6561 ** 5 = {}", b_size);
    assert_eq!(b_size.get(), 12157665459056928801_usize);
    // It will panic.
    // println!("6561 ** 6 = {}", small_uint_pow_func(a_size, 6_u32));
    println!("--------------------------------------");
}

fn small_uint_pow_func<T: cryptocol::number::SmallUInt>(base: T, exp: u32) -> T
{
    base.pow(exp)
}

fn small_uint_wrapping_pow()
{
    println!("small_uint_wrapping_pow()");
    use cryptocol::number::*;

    let a_u8 = small_uint_wrapping_pow_func(3_u8, 5_u32);
    println!("3 ** 5 = {}", a_u8);
    assert_eq!(a_u8, 243_u8);

    let b_u8 = small_uint_wrapping_pow_func(3_u8, 6_u32);
    println!("3 ** 6 = {}", b_u8);
    assert_eq!(b_u8, 217_u8);
    
    let a_u16 = small_uint_wrapping_pow_func(9_u16, 5_u32);
    println!("9 ** 5 = {}", a_u16);
    assert_eq!(a_u16, 59049_u16);

    let b_u16 = small_uint_wrapping_pow_func(9_u16, 6_u32);
    println!("9 ** 6 = {}", b_u16);
    assert_eq!(b_u16, 7153_u16);

    let a_u32 = small_uint_wrapping_pow_func(81_u32, 5_u32);
    println!("81 ** 5 = {}", a_u32);
    assert_eq!(a_u32, 3486784401_u32);

    let b_u32 = small_uint_wrapping_pow_func(81_u32, 6_u32);
    println!("81 ** 6 = {}", b_u32);
    assert_eq!(b_u32, 3256662241_u32);

    let a_u64 = small_uint_wrapping_pow_func(6561_u64, 5_u32);
    println!("6561 ** 5 = {}", a_u64);
    assert_eq!(a_u64, 12157665459056928801_u64);

    let b_u64 = small_uint_wrapping_pow_func(6561_u64, 6_u32);
    println!("6561 ** 6 = {}", b_u64);
    assert_eq!(b_u64, 2721702152408675777_u64);

    let a_u128 = small_uint_wrapping_pow_func(43046721_u128, 5_u32);
    println!("43046721 ** 5 = {}", a_u128);
    assert_eq!(a_u128, 147808829414345923316083210206383297601_u128);

    let b_u128 = small_uint_wrapping_pow_func(43046721_u128, 6_u32);
    println!("43046721 ** 6 = {}", b_u128);
    assert_eq!(b_u128, 333574137813082321045752866839264852865_u128);

    let a_usize = small_uint_wrapping_pow_func(6561_usize, 5_u32);
    println!("6561 ** 5 = {}", a_usize);
    assert_eq!(a_usize, 12157665459056928801_usize);

    let b_usize = small_uint_wrapping_pow_func(6561_usize, 6_u32);
    println!("6561 ** 6 = {}", b_usize);
    assert_eq!(b_usize, 2721702152408675777_usize);

    let a_ushort = ShortUnion::new_with(9);
    let b_ushort = small_uint_wrapping_pow_func(a_ushort, 5_u32);
    println!("9 ** 5 = {}", b_ushort);
    assert_eq!(b_ushort.get(), 59049_u16);

    let c_ushort = small_uint_wrapping_pow_func(a_ushort, 6_u32);
    println!("9 ** 6 = {}", c_ushort);
    assert_eq!(c_ushort.get(), 7153_u16);

    let a_uint = IntUnion::new_with(81);
    let b_uint = small_uint_wrapping_pow_func(a_uint, 5_u32);
    println!("81 ** 5 = {}", b_uint);
    assert_eq!(b_uint.get(), 3486784401_u32);

    let c_uint = small_uint_wrapping_pow_func(a_uint, 6_u32);
    println!("81 ** 6 = {}", c_uint);
    assert_eq!(c_uint.get(), 3256662241_u32);

    let a_ulong = LongUnion::new_with(6561);
    let b_ulong = small_uint_wrapping_pow_func(a_ulong, 5_u32);
    println!("6561 ** 5 = {}", b_ulong);
    assert_eq!(b_ulong.get(), 12157665459056928801_u64);

    let c_ulong = small_uint_wrapping_pow_func(a_ulong, 6_u32);
    println!("6561 ** 6 = {}", c_ulong);
    assert_eq!(c_ulong.get(), 2721702152408675777_u64);

    let a_ulonger = LongerUnion::new_with(43046721_u128);
    let b_ulonger = small_uint_wrapping_pow_func(a_ulonger, 5_u32);
    println!("43046721 ** 5 = {}", b_ulonger);
    assert_eq!(b_ulonger.get(), 147808829414345923316083210206383297601_u128);

    let c_ulonger = small_uint_wrapping_pow_func(a_ulonger, 6_u32);
    println!("43046721 ** 6 = {}", c_ulonger);
    assert_eq!(c_ulonger.get(), 333574137813082321045752866839264852865_u128);

    let a_size = SizeUnion::new_with(6561);
    let b_size = small_uint_wrapping_pow_func(a_size, 5_u32);
    println!("6561 ** 5 = {}", b_size);
    assert_eq!(b_size.get(), 12157665459056928801_usize);

    let c_size = small_uint_wrapping_pow_func(a_size, 6_u32);
    println!("6561 ** 6 = {}", c_size);
    assert_eq!(c_size.get(), 2721702152408675777_usize);
    println!("--------------------------------------");
}

fn small_uint_wrapping_pow_func<T: cryptocol::number::SmallUInt>(base: T, exp: u32) -> T
{
    base.wrapping_pow(exp)
}

fn small_uint_overflowing_pow()
{
    println!("small_uint_overflowing_pow()");
    let (a_u8, overflow) = small_uint_overflowing_pow_func(6_u8, 3);
    println!("{} ** 3 = {}\nOverflow = {}", 6, a_u8, overflow);
    assert_eq!(a_u8, 216_u8);
    assert_eq!(overflow, false);

    let (a_u16, overflow) = small_uint_overflowing_pow_func(12_u16, 5);
    println!("{} ** 4 = {}\nOverflow = {}", 12_u16, a_u16, overflow);
    assert_eq!(a_u16, 52224_u16);
    assert_eq!(overflow, true);
 
    let a_u32 = small_uint_overflowing_pow_func(u32::MAX / 3, 2_u32);
    println!("{} / 2 = {}\nOverflow = {}", u32::MAX / 3, a_u32.0, a_u32.1);
    assert_eq!(a_u32.0, 715827882_u32);
    assert_eq!(a_u32.1, false);
 
    let a_u64 = small_uint_overflowing_pow_func(u64::MAX / 3, 2_u32);
    println!("{} / 2 = {}\nOverflow = {}", u64::MAX / 3, a_u64.0, a_u64.1);
    assert_eq!(a_u64.0, 3074457345618258602_u64);
    assert_eq!(a_u64.1, false);
 
    let a_u128 = small_uint_overflowing_pow_func(u128::MAX / 3, 2_u32);
    println!("{} / 2 = {}\nOverflow = {}", u128::MAX / 3, a_u128.0, a_u128.1);
    assert_eq!(a_u128.0, 56713727820156410577229101238628035242_u128);
    assert_eq!(a_u128.1, false);
 
    let a_usize = small_uint_overflowing_pow_func(usize::MAX / 3, 2_u32);
    println!("{} / 2 = {}\nOverflow = {}", usize::MAX / 3, a_usize.0, a_usize.1);
    assert_eq!(a_usize.0, 3074457345618258602_usize);
    assert_eq!(a_usize.1, false);
 
    // It will panic.
    // let a_panic = small_uint_overflowing_pow_func(a_usize.0, 0_usize);
    println!("--------------------------------------");
}

fn small_uint_overflowing_pow_func<T: cryptocol::number::SmallUInt>(base: T, exp: u32) -> (T, bool)
{
    base.overflowing_pow(exp)
}

fn small_uint_checked_pow()
{
    println!("small_uint_checked_pow()");
    let a_u8 = small_uint_checked_pow_func(u8::MAX / 3, 2_u32);
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

    let b_u8 = small_uint_checked_pow_func(u8::MAX / 3, 0_u32);
    match b_u8
    {
        Some(b) => { println!("{} / 2 = {}", u8::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_u8, None);
            },
    }

    let a_u16 = small_uint_checked_pow_func(u16::MAX / 3, 2_u32);
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

    let b_u16 = small_uint_checked_pow_func(u16::MAX / 3, 0_u32);
    match b_u16
    {
        Some(b) => { println!("{} / 2 = {}", u16::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_u16, None);
            },
    }

    let a_u32 = small_uint_checked_pow_func(u32::MAX / 3, 2_u32);
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

    let b_u32 = small_uint_checked_pow_func(u32::MAX / 3, 0_u32);
    match b_u32
    {
        Some(b) => { println!("{} / 2 = {}", u32::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_u32, None);
            },
    }

    let a_u64 = small_uint_checked_pow_func(u64::MAX / 3, 2_u32);
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

    let b_u64 = small_uint_checked_pow_func(u64::MAX / 3, 0_u32);
    match b_u64
    {
        Some(b) => { println!("{} / 2 = {}", u64::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_u64, None);
            },
    }

    let a_u128 = small_uint_checked_pow_func(u128::MAX / 3, 2_u32);
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

    let b_u128 = small_uint_checked_pow_func(u128::MAX / 3, 0_u32);
    match b_u128
    {
        Some(b) => { println!("{} / 2 = {}", u128::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_u128, None);
            },
    }

    let a_usize = small_uint_checked_pow_func(usize::MAX / 3, 2_u32);
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

    let b_usize = small_uint_checked_pow_func(usize::MAX / 3, 0_u32);
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

fn small_uint_checked_pow_func<T: cryptocol::number::SmallUInt>(base: T, exp: u32) -> Option<T>
{
    base.checked_pow(exp)
}

fn small_uint_unchecked_pow()
{
    println!("small_uint_unchecked_pow()");
    let a_u8 = small_uint_unchecked_pow_func(u8::MAX / 3, 2_u32);
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

    let b_u8 = small_uint_unchecked_pow_func(u8::MAX / 3, 0_u32);
    match b_u8
    {
        Some(b) => { println!("{} / 2 = {}", u8::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_u8, None);
            },
    }

    let a_u16 = small_uint_unchecked_pow_func(u16::MAX / 3, 2_u32);
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

    let b_u16 = small_uint_unchecked_pow_func(u16::MAX / 3, 0_u32);
    match b_u16
    {
        Some(b) => { println!("{} / 2 = {}", u16::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_u16, None);
            },
    }

    let a_u32 = small_uint_unchecked_pow_func(u32::MAX / 3, 2_u32);
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

    let b_u32 = small_uint_unchecked_pow_func(u32::MAX / 3, 0_u32);
    match b_u32
    {
        Some(b) => { println!("{} / 2 = {}", u32::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_u32, None);
            },
    }

    let a_u64 = small_uint_unchecked_pow_func(u64::MAX / 3, 2_u32);
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

    let b_u64 = small_uint_unchecked_pow_func(u64::MAX / 3, 0_u32);
    match b_u64
    {
        Some(b) => { println!("{} / 2 = {}", u64::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_u64, None);
            },
    }

    let a_u128 = small_uint_unchecked_pow_func(u128::MAX / 3, 2_u32);
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

    let b_u128 = small_uint_unchecked_pow_func(u128::MAX / 3, 0_u32);
    match b_u128
    {
        Some(b) => { println!("{} / 2 = {}", u128::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_u128, None);
            },
    }

    let a_usize = small_uint_unchecked_pow_func(usize::MAX / 3, 2_u32);
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

    let b_usize = small_uint_unchecked_pow_func(usize::MAX / 3, 0_u32);
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

fn small_uint_unchecked_pow_func<T: cryptocol::number::SmallUInt>(base: T, exp: u32) -> Option<T>
{
    base.checked_pow(exp)
}

fn small_uint_saturating_pow()
{
    println!("small_uint_saturating_pow()");
    let a_u8 = small_uint_saturating_pow_func(u8::MAX / 3, 2_u32);
    println!("{} / 2 = {}", u8::MAX / 3, a_u8);
    assert_eq!(a_u8, 42_u8);

    let a_u16 = small_uint_saturating_pow_func(u16::MAX / 3, 2_u32);
    println!("{} / 2 = {}", u16::MAX / 3, a_u16);
    assert_eq!(a_u16, 10922_u16);

    let a_u32 = small_uint_saturating_pow_func(u32::MAX / 3, 2_u32);
    println!("{} / 2 = {}", u32::MAX / 3, a_u32);
    assert_eq!(a_u32, 715827882_u32);

    let a_u64 = small_uint_saturating_pow_func(u64::MAX / 3, 2_u32);
    println!("{} / 2 = {}", u64::MAX / 3, a_u64);
    assert_eq!(a_u64, 3074457345618258602_u64);

    let a_u128 = small_uint_saturating_pow_func(u128::MAX / 3, 2_u32);
    println!("{} / 2 = {}", u128::MAX / 3, a_u128);
    assert_eq!(a_u128, 56713727820156410577229101238628035242_u128);

    let a_usize = small_uint_saturating_pow_func(usize::MAX / 3, 2_u32);
    println!("{} / 2 = {}", usize::MAX / 3, a_usize);
    assert_eq!(a_usize, 3074457345618258602_usize);

    // It will panic.
    // let a_panic = small_uint_saturating_pow_func(usize::MAX / 3, 0_usize);
    println!("--------------------------------------");
}

fn small_uint_saturating_pow_func<T: cryptocol::number::SmallUInt>(lhs: T, rhs: u32) -> T
{
    lhs.saturating_pow(rhs)
}

fn small_uint_modular_pow()
{
    println!("small_uint_modular_pow");
    use cryptocol::number::SmallUInt;
    let a = 2_u128;
    let d = 4776913109852041418248056622882488318_u128;
    let modulo = 4776913109852041418248056622882488319_u128;
    let res = a.modular_pow(d, modulo);
    println!("res = {}", res);
    assert_eq!(res, 8);
    println!("--------------------------------------");
}

fn small_uint_root_main()
{
    small_uint_sqrt();
    small_uint_root();
}

fn small_uint_sqrt()
{
    use cryptocol::number::SmallUInt;
    let a = 100000000000000_u64;
    let b = a.sqrt();
    let c = b.pow(2);
    let d = (a + 1).sqrt();
    let e = (b + 1).pow(2);
    println!("sqrt({}) = {}\t{} ** 2 = {}", a, b, b, c);
    println!("sqrt({}) = {}\t{} ** 2 = {}", a + 1, d, b + 1, e);
}

fn small_uint_root()
{
    use cryptocol::number::SmallUInt;
    let a = 100000000000000_u64;
    let b = a.root(3);
    let c = b.pow(3);
    let d = (a + 1).root(3);
    let e = (b + 1).pow(3);
    println!("cbrt({}) = {}\t\t{} ** 3 = {}", a, b, b, c);
    println!("cbrt({}) = {}\t\t{} ** 3 = {}", a + 1, d, b + 1, e);
}


fn small_uint_prime_main()
{
    // small_uint_is_random_main();
    small_uint_is_prime_using_miller_rabin_main();
    // small_uint_random_prime_using_Miller_Rabin_main();
}

fn small_uint_is_prime_using_miller_rabin_main()
{
    println!("small_uint_is_prime_using_miller_rabin_main");
    use cryptocol::number::*;

    let num = 4776913109852041418248056622882488319_u128;//125469875632546987525485265478911_u128;
    let b_prime = num.is_prime_using_miller_rabin(5);
    println!("Is {} a prime number? => {}.", num, b_prime);
    println!("-------------------------");
}
