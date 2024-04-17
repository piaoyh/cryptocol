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
    small_uint_quick_start();
    small_uint_add_main();
    small_uint_sub_main();
    small_uint_mul_main();
    small_uint_div_main();
    small_uint_rem_main();
    small_uint_neg_main();
    small_uint_pow_main();
    small_uint_log_main();
    small_uint_root_main();
    small_uint_prime_main();
    small_uint_bits_operation();
    small_uint_bytes_operation();
    small_uint_find_power();
    small_uint_conversion();
    small_uint_constants();
    small_uint_size();
}

fn small_uint_quick_start()
{
    small_uint_quick_start1();
    small_uint_quick_start2();
}

fn small_uint_quick_start1()
{
    println!("small_uint_quick_start1");
    use cryptocol::number::SmallUInt;

    let a_u8 = 60_u8.modular_add(15, 100);
    println!("60 + 55 = {} (mod 100)", a_u8);
    assert_eq!(a_u8, 75);

    let b_u8 = a_u8.modular_add(55, 100);
    println!("{} + 55 = {} (mod 100)", a_u8, b_u8);
    assert_eq!(b_u8, 30);

    let a_u64 = 6_0000_0000_0000_0000_u64.modular_add(1_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    println!("6_0000_0000_0000_0000 + 1_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_u64);
    assert_eq!(a_u64, 7_5000_0000_0000_0000);

    let b_u64 = a_u64.modular_add(5_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    println!("{} + 5_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_u64, b_u64);
    assert_eq!(b_u64, 3_0000_0000_0000_0000);

    let a_u16 = 25469_u16;
    // If the number is less than u32::MAX (= 4294967295_u32),
    // 3 is enough for `repetition` with 2, 7, and 61
    // for 100% certainty for determination of prime number. 
    let prime = a_u16.is_prime_using_miller_rabin(3_usize);
    if prime
        { println!("It is 100% certain that {} is a prime number.", a_u16); }
    else
        { println!("It is 100% certain that {} is a composite number.", a_u16); }

    let a_u128 = 2341058314661067957826634487913509653_u128;
    let prime = a_u128.is_prime_using_miller_rabin(5_usize);
    if prime
        { println!("It is 99.9% certain that {} is a prime number.", a_u128); }
    else
        { println!("It is 100% certain that {} is a composite number.", a_u128); }
    println!("--------------------------------------");
}

fn small_uint_quick_start2()
{
    println!("small_uint_quick_start2");
    #[allow(unused_imports)] use cryptocol::number::SmallUInt;

    let c_u8 = small_uint_quick_start_func1(60_u8, 15, 100);
    println!("60 + 55 = {} (mod 100)", c_u8);
    assert_eq!(c_u8, 75);

    let d_u8 = small_uint_quick_start_func1(c_u8, 55, 100);
    println!("{} + 55 = {} (mod 100)", c_u8, d_u8);
    assert_eq!(d_u8, 30);

    let c_u64 = small_uint_quick_start_func1(6_0000_0000_0000_0000_u64, 1_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    println!("6_0000_0000_0000_0000 + 1_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", c_u64);
    assert_eq!(c_u64, 7_5000_0000_0000_0000);

    let d_u64 = small_uint_quick_start_func1(c_u64, 5_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    println!("{} + 5_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", c_u64, d_u64);
    assert_eq!(d_u64, 3_0000_0000_0000_0000);

    let b_u16 = 25469_u16;
    // If the number is less than u32::MAX (= 4294967295_u32),
    // 3 is enough for `repetition` with 2, 7, and 61
    // for 100% certainty for determination of prime number. 
    let prime = small_uint_quick_start_func2(b_u16, 3_usize);
    if prime
        { println!("It is 100% certain that {} is a prime number.", b_u16); }
    else
        { println!("It is 100% certain that {} is a composite number.", b_u16); }

    let b_u128 = 2341058314661067957826634487913509653_u128;
    let prime = small_uint_quick_start_func2(b_u128, 5_usize);
    if prime
        { println!("It is 75% certain that {} is a prime number.", b_u128); }
    else
        { println!("It is 100% certain that {} is a composite number.", b_u128); }
    
    println!("--------------------------------------");
}

fn small_uint_quick_start_func1<T: cryptocol::number::SmallUInt>(lhs: T, rhs: T, modulo: T) -> T
{
    lhs.modular_add(rhs, modulo)
}

fn small_uint_quick_start_func2<T: cryptocol::number::SmallUInt>(num: T, repetition: usize) -> bool
{
    num.is_prime_using_miller_rabin(repetition)
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
    println!("small_uint_carrying_add");
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

    // Example for u32
    //  1234567890123456789_u64 == ( 287445236_u32, 2112454933_u32)
    //+ 9876543210123456789_u64 == (2299561912_u32, 2956226837_u32)
    //-------------------------------------------------------------
    // 11111111100246913578_u64 == (2587007149_u32,  773714474_u32)

    // a: u256 === (a_high_u32, a_low_u32)
    let (a_low_u32, a_high_u32, carry) = small_uint_carrying_add_func(2112454933_u32, 287445236_u32, 2956226837_u32, 2299561912_u32);
    println!("{}-{}, {}", a_high_u32, a_low_u32, carry);
    assert_eq!(a_high_u32, 2587007149_u32);
    assert_eq!(a_low_u32, 773714474_u32);
    assert_eq!(carry, false);

    //  11111111100246913578_u64 == (2587007149_u32,  773714474_u32)
    //+  9876543210123456789_u64 == (2299561912_u32, 2956226837_u32)
    //--------------------------------------------------------------
    //   2540910236660818751_u64 == ( 591601765_u32, 3729941311_u32)

    // b: u256 === (b_high_u32, b_low_u32)
    let (b_low_u32, b_high_u32, carry) = small_uint_carrying_add_func(773714474_u32, 2587007149_u32, 2956226837_u32, 2299561912_u32);
    println!("{}-{}, {}", b_high_u32, b_low_u32, carry);
    assert_eq!(b_high_u32, 591601765_u32);
    assert_eq!(b_low_u32, 3729941311_u32);
    assert_eq!(carry, true);

    // Example for u64
    // a_u128: u128 === (a_high_u64, a_low_u64) === (6692605942763486917_u64, 12312739301371248917_u64) === 322222221211111111100000000088888888987_u128
    let a_high_u64 = 6692605942763486917_u64;
    let a_low_u64 = 12312739301371248917_u64;
    // b_u128: u128 === (b_high_u64, b_low_u64) === (10775095670246085798_u64, 7681743649119882630_u64) === 198765432198765432198765432198765432198_u128
    let b_high_u64 = 10775095670246085798_u64;
    let b_low_u64 = 7681743649119882630_u64;

    // (6692605942763486917_u64, 12312739301371248917_u64) + (10775095670246085798_u64, 7681743649119882630_u64) == 123456789012345678901234567890123456789_u128 + 198765432198765432198765432198765432198_u128 == 322222221211111111100000000088888888987_u128
    //   123456789012345678901234567890123456789_u128 == ( 6692605942763486917_u64, 12312739301371248917_u64)
    // + 198765432198765432198765432198765432198_u128 == (10775095670246085798_u64,  7681743649119882630_u64)
    // ------------------------------------------------------------------------------------------------------
    //   322222221211111111100000000088888888987_u128 == (17467701613009572716_u64,  1547738876781579931_u64)

    // c_u128: u128 === (c_high_longunion, c_low_longunion)
    let (c_low_u64, c_high_u64, carry) = small_uint_carrying_add_func(a_low_u64, a_high_u64, b_low_u64, b_high_u64);
    println!("{}-{}, {}", c_high_u64, c_low_u64, carry);
    assert_eq!(c_high_u64, 17467701613009572716_u64);
    assert_eq!(c_low_u64, 1547738876781579931_u64);
    assert_eq!(carry, false);

    // (17467701613009572716_u64, 1547738876781579931_u64) + (10775095670246085798_u64, 7681743649119882630_u64) == 322222221211111111100000000088888888987_u128 + 198765432198765432198765432198765432198_u128 == 180705286488938079835390824855886109729_u64
    //   322222221211111111100000000088888888987_u128 == (17467701613009572716_u64, 1547738876781579931_u64)
    // + 198765432198765432198765432198765432198_u128 == (10775095670246085798_u64, 7681743649119882630_u64)
    // -----------------------------------------------------------------------------------------------------
    //   180705286488938079835390824855886109729_u128 == ( 9796053209546106898_u64, 9229482525901462561_u64)

    // d: u128 === (d_high_u64, d_low_u64)
    let (d_low_u64, d_high_u64, carry) = small_uint_carrying_add_func(c_low_u64, c_high_u64, b_low_u64, b_high_u64);
    println!("{}-{}, {}", d_high_u64, d_low_u64, carry);
    assert_eq!(d_high_u64, 9796053209546106898_u64);
    assert_eq!(d_low_u64, 9229482525901462561_u64);
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

    // Example for usize for 64-bit CPU
    // a_u128: u128 === (a_high_usize, a_low_usize) === (6692605942763486917_usize, 12312739301371248917_usize) === 322222221211111111100000000088888888987_u128
    let a_high_usize = 6692605942763486917_usize;
    let a_low_usize = 12312739301371248917_usize;
    // b_u128: u128 === (b_high_usize, b_low_usize) === (10775095670246085798_usize, 7681743649119882630_usize) === 198765432198765432198765432198765432198_u128
    let b_high_usize = 10775095670246085798_usize;
    let b_low_usize = 7681743649119882630_usize;

    // (6692605942763486917_usize, 12312739301371248917_usize) + (10775095670246085798_usize, 7681743649119882630_usize) == 123456789012345678901234567890123456789_u128 + 198765432198765432198765432198765432198_u128 == 322222221211111111100000000088888888987_u128
    //   123456789012345678901234567890123456789_u128 == ( 6692605942763486917_usize, 12312739301371248917_usize)
    // + 198765432198765432198765432198765432198_u128 == (10775095670246085798_usize,  7681743649119882630_usize)
    // ----------------------------------------------------------------------------------------------------------
    //   322222221211111111100000000088888888987_u128 == (17467701613009572716_usize,  1547738876781579931_usize)

    // c_u128: u128 === (c_high_longunion, c_low_longunion)
    let (c_low_usize, c_high_usize, carry) = small_uint_carrying_add_func(a_low_usize, a_high_usize, b_low_usize, b_high_usize);
    println!("{}-{}, {}", c_high_usize, c_low_usize, carry);
    assert_eq!(c_high_usize, 17467701613009572716_usize);
    assert_eq!(c_low_usize, 1547738876781579931_usize);
    assert_eq!(carry, false);

    // (17467701613009572716_usize, 1547738876781579931_usize) + (10775095670246085798_usize, 7681743649119882630_usize) == 322222221211111111100000000088888888987_u128 + 198765432198765432198765432198765432198_u128 == 180705286488938079835390824855886109729_usize
    //   322222221211111111100000000088888888987_u128 == (17467701613009572716_usize, 1547738876781579931_usize)
    // + 198765432198765432198765432198765432198_u128 == (10775095670246085798_usize, 7681743649119882630_usize)
    // -----------------------------------------------------------------------------------------------------
    //   180705286488938079835390824855886109729_u128 == ( 9796053209546106898_usize, 9229482525901462561_usize)

    // d: u128 === (d_high_usize, d_low_usize)
    let (d_low_usize, d_high_usize, carry) = small_uint_carrying_add_func(c_low_usize, c_high_usize, b_low_usize, b_high_usize);
    println!("{}-{}, {}", d_high_usize, d_low_usize, carry);
    assert_eq!(d_high_usize, 9796053209546106898_usize);
    assert_eq!(d_low_usize, 9229482525901462561_usize);
    assert_eq!(carry, true);

    // Example for ShortUnion
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

    // Example for IntUnion
    //  1234567890123456789_u64 == ( 287445236_u32, 2112454933_u32)
    //+ 9876543210123456789_u64 == (2299561912_u32, 2956226837_u32)
    //-------------------------------------------------------------
    // 11111111100246913578_u64 == (2587007149_u32,  773714474_u32)

    // a: u64 === (a_high_intunion, a_low_intunion)
    let (a_low_intunion, a_high_intunion, carry) = small_uint_carrying_add_func(2112454933_u32.into_intunion(), 287445236_u32.into_intunion(), 2956226837_u32.into_intunion(), 2299561912_u32.into_intunion());
    println!("{}-{}, {}", a_high_intunion, a_low_intunion, carry);
    assert_eq!(a_high_intunion.get(), 2587007149_u32);
    assert_eq!(a_low_intunion.get(), 773714474_u32);
    assert_eq!(carry, false);

    //  11111111100246913578_u64 == (2587007149_u32,  773714474_u32)
    //+  9876543210123456789_u64 == (2299561912_u32, 2956226837_u32)
    //--------------------------------------------------------------
    //   2540910236660818751_u64 == ( 591601765_u32, 3729941311_u32)

    // b: u64 === (b_high_intunion, b_low_intunion)
    let (b_low_intunion, b_high_intunion, carry) = small_uint_carrying_add_func(773714474_u32.into_intunion(), 2587007149_u32.into_intunion(), 2956226837_u32.into_intunion(), 2299561912_u32.into_intunion());
    println!("{}-{}, {}", b_high_intunion, b_low_intunion, carry);
    assert_eq!(b_high_intunion.get(), 591601765_u32);
    assert_eq!(b_low_intunion.get(), 3729941311_u32);
    assert_eq!(carry, true);

    // Example for LongUnion
    use cryptocol::number::LongUnion;
    // a_longerunion: LongerUnion === (a_high_longunion, a_low_longunion) === (6692605942763486917_u64, 12312739301371248917_u64) === 322222221211111111100000000088888888987_u128
    let a_high_longunion = LongUnion::new_with(6692605942763486917_u64);
    let a_low_longunion = LongUnion::new_with(12312739301371248917_u64);
    // b_longunion: LongerUnion === (b_high_longunion, b_low_longunion) === (10775095670246085798_u64, 7681743649119882630_u64) === 198765432198765432198765432198765432198_u128
    let b_high_longunion = LongUnion::new_with(10775095670246085798_u64);
    let b_low_longunion = LongUnion::new_with(7681743649119882630_u64);

    // (6692605942763486917_u64, 12312739301371248917_u64) + (10775095670246085798_u64, 7681743649119882630_u64) == 123456789012345678901234567890123456789_u128 + 198765432198765432198765432198765432198_u128 == 322222221211111111100000000088888888987_u128
    //   123456789012345678901234567890123456789_u128 == (6692605942763486917_u64, 12312739301371248917_u64)
    // + 198765432198765432198765432198765432198_u128 == (10775095670246085798_u64, 7681743649119882630_u64)
    // -----------------------------------------------------------------------------------------------------
    //   322222221211111111100000000088888888987_u128 == (17467701613009572716_u64, 1547738876781579931_u64)

    // c_u128: u128 === (c_high_longunion, c_low_longunion)
    let (c_low_longunion, c_high_longunion, carry) = small_uint_carrying_add_func(a_low_longunion, a_high_longunion, b_low_longunion, b_high_longunion);
    println!("{}-{}, {}", c_high_longunion, c_low_longunion, carry);
    assert_eq!(c_high_longunion.get(), 17467701613009572716_u64);
    assert_eq!(c_low_longunion.get(), 1547738876781579931_u64);
    assert_eq!(carry, false);

    // (17467701613009572716_u64, 1547738876781579931_u64) + (10775095670246085798_u64, 7681743649119882630_u64) == 322222221211111111100000000088888888987_u128 + 198765432198765432198765432198765432198_u128 == 180705286488938079835390824855886109729_u64
    //   322222221211111111100000000088888888987_u128 == (17467701613009572716_u64, 1547738876781579931_u64)
    // + 198765432198765432198765432198765432198_u128 == (10775095670246085798_u64, 7681743649119882630_u64)
    // -----------------------------------------------------------------------------------------------------
    //   180705286488938079835390824855886109729_u128 == ( 9796053209546106898_u64, 9229482525901462561_u64)

    // d: u128 === (d_high_longunion, d_low_longunion)
    let (d_low_longunion, d_high_longunion, carry) = small_uint_carrying_add_func(c_low_longunion, c_high_longunion, b_low_longunion, b_high_longunion);
    println!("{}-{}, {}", d_high_longunion, d_low_longunion, carry);
    assert_eq!(d_high_longunion.get(), 9796053209546106898_u64);
    assert_eq!(d_low_longunion.get(), 9229482525901462561_u64);
    assert_eq!(carry, true);

    // Example for LongerUnion
    //  4201016837757989640311993609423984479246482890531986660185 == (12345678901234567890_u128, 6789012345678912345_u128)
    //+                 419908440780438063913804265570801972943493 == (                1234_u128,                6789_u128)
    //---------------------------------------------------------------------------------------------------------------------
    //  4201016837757990060220434389862048393050748461333959603678 == (12345678901234569124_u128, 6789012345678919134_u128)

    // a_u256: u256 === (a_high_longerunion, a_low_longerunion)
    let (a_low_longerunion, a_high_longerunion, carry) = small_uint_carrying_add_func(6789012345678912345_u128.into_longerunion(), 12345678901234567890_u128.into_longerunion(), 6789_u128.into_longerunion(), 1234_u128.into_longerunion());
    println!("{}-{}, {}", a_high_longerunion, a_low_longerunion, carry);
    assert_eq!(a_high_longerunion.get(), 12345678901234569124_u128);
    assert_eq!(a_low_longerunion.get(), 6789012345678919134_u128);
    assert_eq!(carry, false);

    //  308778904632843187796189293356501087608549893209439890708590319850715068122315 == (226854911280625642308916404954512140970_u128, 56789012345678912345678901234567890123_u128)
    //+  57896044618658097711785492504343953926307055644800578124155540853313808954190 == (170141183460469231731687303715884105727_u128, 12345678901234567890123456789012345678_u128)
    //--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
    //   19298681539552699237261830834781317975046994857318776714373108680289488156697 == ( 56713727820156410577229101238628035241_u128, 69134691246913480235802358023580235801_u128)

    // b_u256: u256 === (b_high_longerunion, b_low_longerunion)
    let (b_low_longerunion, b_high_longerunion, carry) = small_uint_carrying_add_func(56789012345678912345678901234567890123_u128.into_longerunion(), 226854911280625642308916404954512140970_u128.into_longerunion(), 12345678901234567890123456789012345678_u128.into_longerunion(), 170141183460469231731687303715884105727_u128.into_longerunion());
    println!("{}-{}, {}", b_high_longerunion, b_low_longerunion, carry);
    assert_eq!(b_high_longerunion.get(), 56713727820156410577229101238628035241_u128);
    assert_eq!(b_low_longerunion.get(), 69134691246913480235802358023580235801_u128);
    assert_eq!(carry, true);

    // Example for SizeUnion for 64-bit CPU
    use cryptocol::number::SizeUnion;
    // a_u128: u128 === (a_high_sizeunion, a_low_sizeunion) === (6692605942763486917_usize, 12312739301371248917_usize) === 322222221211111111100000000088888888987_u128
    let a_high_sizeunion = SizeUnion::new_with(6692605942763486917_usize);
    let a_low_sizeunion = SizeUnion::new_with(12312739301371248917_usize);
    // b_u128: u128 === (b_high_sizeunion, b_low_sizeunion) === (10775095670246085798_usize, 7681743649119882630_usize) === 198765432198765432198765432198765432198_u128
    let b_high_sizeunion = SizeUnion::new_with(10775095670246085798_usize);
    let b_low_sizeunion = SizeUnion::new_with(7681743649119882630_usize);

    // (6692605942763486917_usize, 12312739301371248917_usize) + (10775095670246085798_usize, 7681743649119882630_usize) == 123456789012345678901234567890123456789_u128 + 198765432198765432198765432198765432198_u128 == 322222221211111111100000000088888888987_u128
    //   123456789012345678901234567890123456789_u128 == ( 6692605942763486917_usize, 12312739301371248917_usize)
    // + 198765432198765432198765432198765432198_u128 == (10775095670246085798_usize,  7681743649119882630_usize)
    // ----------------------------------------------------------------------------------------------------------
    //   322222221211111111100000000088888888987_u128 == (17467701613009572716_usize,  1547738876781579931_usize)

    // c_u128: u128 === (c_high_sizeunion, c_low_sizeunion)
    let (c_low_sizeunion, c_high_sizeunion, carry) = small_uint_carrying_add_func(a_low_sizeunion, a_high_sizeunion, b_low_sizeunion, b_high_sizeunion);
    println!("{}-{}, {}", c_high_sizeunion, c_low_sizeunion, carry);
    assert_eq!(c_high_sizeunion.get(), 17467701613009572716_usize);
    assert_eq!(c_low_sizeunion.get(), 1547738876781579931_usize);
    assert_eq!(carry, false);

    // (17467701613009572716_usize, 1547738876781579931_usize) + (10775095670246085798_usize, 7681743649119882630_usize) == 322222221211111111100000000088888888987_u128 + 198765432198765432198765432198765432198_u128 == 180705286488938079835390824855886109729_usize
    //   322222221211111111100000000088888888987_u128 == (17467701613009572716_usize, 1547738876781579931_usize)
    // + 198765432198765432198765432198765432198_u128 == (10775095670246085798_usize, 7681743649119882630_usize)
    // -----------------------------------------------------------------------------------------------------
    //   180705286488938079835390824855886109729_u128 == ( 9796053209546106898_usize, 9229482525901462561_usize)

    // d: u128 === (d_high_sizeunion, d_low_sizeunion)
    let (d_low_sizeunion, d_high_sizeunion, carry) = small_uint_carrying_add_func(c_low_sizeunion, c_high_sizeunion, b_low_sizeunion, b_high_sizeunion);
    println!("{}-{}, {}", d_high_sizeunion, d_low_sizeunion, carry);
    assert_eq!(d_high_sizeunion.get(), 9796053209546106898_usize);
    assert_eq!(d_low_sizeunion.get(), 9229482525901462561_usize);
    assert_eq!(carry, true);
    println!("--------------------------------------");
}

fn small_uint_carrying_add_func<T: cryptocol::number::SmallUInt>(lhs_low: T, lhs_high: T, rhs_low: T, rhs_high: T) -> (T, T, bool)
{
    let (sum_low, carry) = lhs_low.carrying_add(rhs_low, false);
    let (sum_high, carry) = lhs_high.carrying_add(rhs_high, carry);
    (sum_low, sum_high, carry)
}

fn small_uint_wrapping_add()
{
    println!("small_uint_wrapping_add");
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
    println!("{} + 1 = {}", a_longunion, d_longunion);
    assert_eq!(d_longunion.get(), 0_u64);

    // Example for LongerUnion
    let a_longerunion = LongerUnion::new_with(u128::MAX - 55_u128);
    let b_longerunion = LongerUnion::new_with(55);
    let c_longerunion = small_uint_wrapping_add_func(a_longerunion, b_longerunion);
    println!("{} + {} = {}", a_longerunion, b_longerunion, c_longerunion);
    assert_eq!(c_longerunion.get(), u128::MAX);

    let d_longerunion = small_uint_wrapping_add_func(c_longerunion, 1_u128.into_longerunion());
    println!("{} + 1 = {}", a_longerunion, d_longerunion);
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
    println!("small_uint_overflowing_add");
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
    println!("small_uint_checked_add");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = small_uint_checked_add_func(u8::MAX - 55_u8, 55_u8);
    match a_u8
    {
        Some(a) => {
                println!("{} + 55 = {}", u8::MAX - 55_u8, a);
                assert_eq!(a, u8::MAX);
            },
        None => { println!("Overflow happened."); },
    }
 
    let b_u8 = small_uint_checked_add_func(a_u8.unwrap(), 1_u8);
    match b_u8
    {
        Some(b) => { println!("{} + 1 = {}", a_u8.unwrap(), b); },
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
        None => { println!("Overflow happened."); },
    }

    let b_u16 = small_uint_checked_add_func(a_u16.unwrap(), 1_u16);
    match b_u16
    {
        Some(b) => { println!("{} + 1 = {}", a_u16.unwrap(), b); },
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
        None => { println!("Overflow happened."); },
    }

    let b_u32 = small_uint_checked_add_func(a_u32.unwrap(), 1_u32);
    match b_u32
    {
        Some(b) => { println!("{} + 1 = {}", a_u32.unwrap(), b); },
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
        None => { println!("Overflow happened."); },
    }

    let b_u64 = small_uint_checked_add_func(a_u64.unwrap(), 1_u64);
    match b_u64
    {
        Some(b) => { println!("{} + 1 = {}", a_u64.unwrap(), b); },
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
        None => { println!("Overflow happened."); },
    }

    let b_u128 = small_uint_checked_add_func(a_u128.unwrap(), 1_u128);
    match b_u128
    {
        Some(b) => { println!("{} + 1 = {}", a_u128.unwrap(), b); },
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
        None => { println!("Overflow happened."); },
    }

    let b_usize = small_uint_checked_add_func(a_usize.unwrap(), 1_usize);
    match b_usize
    {
        Some(b) => { println!("{} + 1 = {}", a_usize.unwrap(), b); },
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
        None => { println!("Overflow happened."); },
    }

    let d_shortunion = small_uint_checked_add_func(c_shortunion.unwrap(), 1_u16.into_shortunion());
    match d_shortunion
    {
        Some(d) => { println!("{} + 1 = {}", a_shortunion, d); },
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
        None => { println!("Overflow happened."); },
    }

    let d_intunion = small_uint_checked_add_func(c_intunion.unwrap(), 1_u32.into_intunion());
    match d_intunion
    {
        Some(d) => { println!("{} + 1 = {}", a_intunion, d); },
        None => {
                println!("Overflow happened.");
                assert_eq!(d_intunion, None);
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
        None => { println!("Overflow happened."); },
    }

    let d_longunion = small_uint_checked_add_func(c_longunion.unwrap(), 1_u64.into_longunion());
    match d_longunion
    {
        Some(d) => { println!("{} + 1 = {}", a_longunion, d); },
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
        None => { println!("Overflow happened."); },
    }

    let d_longerunion = small_uint_checked_add_func(c_longerunion.unwrap(), 1_u128.into_longerunion());
    match d_longerunion
    {
        Some(d) => { println!("{} + 1 = {}", a_longerunion, d); },
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
        None => { println!("Overflow happened."); },
    }

    let d_sizeunion = small_uint_checked_add_func(c_sizeunion.unwrap(), 1_usize.into_sizeunion());
    match d_sizeunion
    {
        Some(d) => { println!("{} + 1 = {}", a_sizeunion, d); },
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
    println!("small_uint_unchecked_add");
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
    println!("small_uint_saturating_add");
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

    // Example for ShortUnion
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
    println!("small_uint_modular_add");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = 60_u8.modular_add(15, 100);
    println!("60 + 55 = {} (mod 100)", a_u8);
    assert_eq!(a_u8, 75);

    let b_u8 = a_u8.modular_add(55, 100);
    println!("{} + 55 = {} (mod 100)", a_u8, b_u8);
    assert_eq!(b_u8, 30);

    let c_u8 = small_uint_modular_add_func(60_u8, 15, 100);
    println!("60 + 55 = {} (mod 100)", c_u8);
    assert_eq!(c_u8, 75);

    let d_u8 = small_uint_modular_add_func(c_u8, 55, 100);
    println!("{} + 55 = {} (mod 100)", c_u8, d_u8);
    assert_eq!(d_u8, 30);

    // Example for u16
    let a_u16 = 6000_u16.modular_add(1500, 1_0000);
    println!("6000 + 1500 = {} (mod 1_0000)", a_u16);
    assert_eq!(a_u16, 7500);

    let b_u16 = a_u16.modular_add(5500, 1_0000);
    println!("{} + 55 = {} (mod 1_0000)", a_u16, b_u16);
    assert_eq!(b_u16, 3000);

    let c_u16 = small_uint_modular_add_func(6000_u16, 1500, 1_0000);
    println!("6000 + 1500 = {} (mod 1_0000)", c_u16);
    assert_eq!(c_u16, 7500);

    let d_u16 = small_uint_modular_add_func(c_u16, 5500, 1_0000);
    println!("{} + 55 = {} (mod 1_0000)", c_u16, d_u16);
    assert_eq!(d_u16, 3000);

    // Example for u32
    let a_u32 = 6_0000_0000_u32.modular_add(1_5000_0000, 10_0000_0000);
    println!("6_0000_0000 + 1_5000_0000 = {} (mod 10_0000_0000)", a_u32);
    assert_eq!(a_u32, 7_5000_0000);

    let b_u32 = a_u32.modular_add(5_5000_0000, 10_0000_0000);
    println!("{} + 5_5000_0000 = {} (mod 10_0000_0000)", a_u32, b_u32);
    assert_eq!(b_u32, 3_0000_0000);

    let c_u32 = small_uint_modular_add_func(6_0000_0000_u32, 1_5000_0000, 10_0000_0000);
    println!("6_0000_0000 + 1_5000_0000 = {} (mod 10_0000_0000)", c_u32);
    assert_eq!(c_u32, 7_5000_0000);

    let d_u32 = small_uint_modular_add_func(c_u32, 5_5000_0000, 10_0000_0000);
    println!("{} + 5_5000_0000 = {} (mod 10_0000_0000)", c_u32, d_u32);
    assert_eq!(d_u32, 3_0000_0000);

    // Example for u64
    let a_u64 = 6_0000_0000_0000_0000_u64.modular_add(1_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    println!("6_0000_0000_0000_0000 + 1_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_u64);
    assert_eq!(a_u64, 7_5000_0000_0000_0000);

    let b_u64 = a_u64.modular_add(5_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    println!("{} + 5_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_u64, b_u64);
    assert_eq!(b_u64, 3_0000_0000_0000_0000);

    let c_u64 = small_uint_modular_add_func(6_0000_0000_0000_0000_u64, 1_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    println!("6_0000_0000_0000_0000 + 1_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", c_u64);
    assert_eq!(c_u64, 7_5000_0000_0000_0000);

    let d_u64 = small_uint_modular_add_func(c_u64, 5_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    println!("{} + 5_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", c_u64, d_u64);
    assert_eq!(d_u64, 3_0000_0000_0000_0000);

    // Example for u128
    let a_u128 = 6_0000_0000_0000_0000_0000_0000_0000_0000_u128.modular_add(1_5000_0000_0000_0000_0000_0000_0000_0000, 10_0000_0000_0000_0000_0000_0000_0000_0000);
    println!("6_0000_0000_0000_0000_0000_0000_0000_0000 + 1_5000_0000_0000_0000_0000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000_0000_0000_0000_0000)", a_u128);
    assert_eq!(a_u128, 7_5000_0000_0000_0000_0000_0000_0000_0000);

    let b_u128 = a_u128.modular_add(5_5000_0000_0000_0000_0000_0000_0000_0000, 10_0000_0000_0000_0000_0000_0000_0000_0000);
    println!("{} + 5_5000_0000_0000_0000_0000_0000_0000_0000 = {}", a_u128, b_u128);
    assert_eq!(b_u128, 3_0000_0000_0000_0000_0000_0000_0000_0000);

    let c_u128 = small_uint_modular_add_func(6_0000_0000_0000_0000_0000_0000_0000_0000_u128, 1_5000_0000_0000_0000_0000_0000_0000_0000, 10_0000_0000_0000_0000_0000_0000_0000_0000);
    println!("6_0000_0000_0000_0000_0000_0000_0000_0000 + 1_5000_0000_0000_0000_0000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000_0000_0000_0000_0000)", c_u128);
    assert_eq!(c_u128, 7_5000_0000_0000_0000_0000_0000_0000_0000);

    let d_u128 = small_uint_modular_add_func(c_u128, 5_5000_0000_0000_0000_0000_0000_0000_0000, 10_0000_0000_0000_0000_0000_0000_0000_0000);
    println!("{} + 5_5000_0000_0000_0000_0000_0000_0000_0000 = {}", c_u128, d_u128);
    assert_eq!(d_u128, 3_0000_0000_0000_0000_0000_0000_0000_0000);

    // Example for usize
    let a_usize = 6_0000_0000_0000_0000_usize.modular_add(1_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    println!("6_0000_0000_0000_0000 + 1_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_usize);
    assert_eq!(a_usize, 7_5000_0000_0000_0000);

    let b_usize = a_usize.modular_add(5_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    println!("{} + 5_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_usize, b_usize);
    assert_eq!(b_usize, 3_0000_0000_0000_0000);

    let c_usize = small_uint_modular_add_func(6_0000_0000_0000_0000_usize, 1_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    println!("6_0000_0000_0000_0000 + 1_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", c_usize);
    assert_eq!(c_usize, 7_5000_0000_0000_0000);

    let d_usize = small_uint_modular_add_func(c_usize, 5_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    println!("{} + 5_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", c_usize, d_usize);
    assert_eq!(d_usize, 3_0000_0000_0000_0000);

    // Example for ShortUnion
    let a_shortunion = 6000_u16.into_shortunion().modular_add(1500_u16.into_shortunion(), 1_0000_u16.into_shortunion());
    println!("6000 + 1500 = {} (mod 1_0000)", a_shortunion);
    assert_eq!(a_shortunion.get(), 7500_u16);

    let b_shortunion = a_shortunion.into_shortunion().modular_add(5500_u16.into_shortunion(), 1_0000_u16.into_shortunion());
    println!("{} + 55 = {} (mod 1_0000)", a_shortunion, b_shortunion);
    assert_eq!(b_shortunion.get(), 3000_u16);

    let c_shortunion = small_uint_modular_add_func(6000_u16.into_shortunion(), 1500_u16.into_shortunion(), 1_0000_u16.into_shortunion());
    println!("6000 + 1500 = {} (mod 1_0000)", c_shortunion);
    assert_eq!(c_shortunion.get(), 7500_u16);

    let d_shortunion = small_uint_modular_add_func(c_shortunion.into_shortunion(), 5500_u16.into_shortunion(), 1_0000_u16.into_shortunion());
    println!("{} + 55 = {} (mod 1_0000)", c_shortunion, d_shortunion);
    assert_eq!(d_shortunion.get(), 3000_u16);

    // Example for IntUnion
    let a_intunion = 6_0000_0000_u32.into_intunion().modular_add(1_5000_0000_u32.into_intunion(), 10_0000_0000_u32.into_intunion());
    println!("6_0000_0000 + 1_5000_0000 = {} (mod 10_0000_0000)", a_intunion);
    assert_eq!(a_intunion.get(), 7_5000_0000_u32);

    let b_intunion = a_intunion.modular_add(5_5000_0000_u32.into_intunion(), 10_0000_0000_u32.into_intunion());
    println!("{} + 5_5000_0000 = {} (mod 10_0000_0000)", a_intunion, b_intunion);
    assert_eq!(b_intunion.get(), 3_0000_0000_u32);

    let c_intunion = small_uint_modular_add_func(6_0000_0000_u32.into_intunion(), 1_5000_0000_u32.into_intunion(), 10_0000_0000_u32.into_intunion());
    println!("6_0000_0000 + 1_5000_0000 = {} (mod 10_0000_0000)", c_intunion);
    assert_eq!(c_intunion.get(), 7_5000_0000_u32);

    let d_intunion = small_uint_modular_add_func(c_intunion, 5_5000_0000_u32.into_intunion(), 10_0000_0000_u32.into_intunion());
    println!("{} + 5_5000_0000 = {} (mod 10_0000_0000)", c_intunion, d_intunion);
    assert_eq!(d_intunion.get(), 3_0000_0000_u32);

    // Example for LongUnion
    let a_longunion = 6_0000_0000_0000_0000_u64.into_longunion().modular_add(1_5000_0000_0000_0000_u64.into_longunion(), 10_0000_0000_0000_0000_u64.into_longunion());
    println!("6_0000_0000_0000_0000 + 1_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_longunion);
    assert_eq!(a_longunion.get(), 7_5000_0000_0000_0000);

    let b_longunion = a_longunion.modular_add(5_5000_0000_0000_0000_u64.into_longunion(), 10_0000_0000_0000_0000_u64.into_longunion());
    println!("{} + 5_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_longunion, b_longunion);
    assert_eq!(b_longunion, 3_0000_0000_0000_0000_u64.into_longunion());

    let c_longunion = small_uint_modular_add_func(6_0000_0000_0000_0000_u64.into_longunion(), 1_5000_0000_0000_0000_u64.into_longunion(), 10_0000_0000_0000_0000_u64.into_longunion());
    println!("6_0000_0000_0000_0000 + 1_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", c_longunion);
    assert_eq!(c_longunion.get(), 7_5000_0000_0000_0000);

    let d_longunion = small_uint_modular_add_func(c_longunion, 5_5000_0000_0000_0000_u64.into_longunion(), 10_0000_0000_0000_0000_u64.into_longunion());
    println!("{} + 5_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", c_longunion, d_longunion);
    assert_eq!(d_longunion, 3_0000_0000_0000_0000_u64.into_longunion());

    // Example for LongerUnion
    let a_longerunion = 6_0000_0000_0000_0000_0000_0000_0000_0000_u128.into_longerunion().modular_add(1_5000_0000_0000_0000_0000_0000_0000_0000_u128.into_longerunion(), 10_0000_0000_0000_0000_0000_0000_0000_0000_u128.into_longerunion());
    println!("6_0000_0000_0000_0000_0000_0000_0000_0000 + 1_5000_0000_0000_0000_0000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000_0000_0000_0000_0000)", a_longerunion);
    assert_eq!(a_longerunion.get(), 7_5000_0000_0000_0000_0000_0000_0000_0000_u128);

    let b_longerunion = a_longerunion.modular_add(5_5000_0000_0000_0000_0000_0000_0000_0000_u128.into_longerunion(), 10_0000_0000_0000_0000_0000_0000_0000_0000_u128.into_longerunion());
    println!("{} + 5_5000_0000_0000_0000_0000_0000_0000_0000 = {}", a_longerunion, b_longerunion);
    assert_eq!(b_longerunion.get(), 3_0000_0000_0000_0000_0000_0000_0000_0000_u128);

    let c_longerunion = small_uint_modular_add_func(6_0000_0000_0000_0000_0000_0000_0000_0000_u128.into_longerunion(), 1_5000_0000_0000_0000_0000_0000_0000_0000_u128.into_longerunion(), 10_0000_0000_0000_0000_0000_0000_0000_0000_u128.into_longerunion());
    println!("6_0000_0000_0000_0000_0000_0000_0000_0000 + 1_5000_0000_0000_0000_0000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000_0000_0000_0000_0000)", c_longerunion);
    assert_eq!(c_longerunion.get(), 7_5000_0000_0000_0000_0000_0000_0000_0000_u128);

    let d_longerunion = small_uint_modular_add_func(c_longerunion, 5_5000_0000_0000_0000_0000_0000_0000_0000_u128.into_longerunion(), 10_0000_0000_0000_0000_0000_0000_0000_0000_u128.into_longerunion());
    println!("{} + 5_5000_0000_0000_0000_0000_0000_0000_0000 = {}", c_longerunion, d_longerunion);
    assert_eq!(d_longerunion.get(), 3_0000_0000_0000_0000_0000_0000_0000_0000_u128);

    // Example for SizeUnion
    let a_sizeunion = 6_0000_0000_0000_0000_usize.into_sizeunion().modular_add(1_5000_0000_0000_0000_usize.into_sizeunion(), 10_0000_0000_0000_0000_usize.into_sizeunion());
    println!("6_0000_0000_0000_0000 + 1_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_sizeunion);
    assert_eq!(a_sizeunion.get(), 7_5000_0000_0000_0000_usize);

    let b_sizeunion = a_sizeunion.modular_add(5_5000_0000_0000_0000_usize.into_sizeunion(), 10_0000_0000_0000_0000_usize.into_sizeunion());
    println!("{} + 5_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_sizeunion, b_sizeunion);
    assert_eq!(b_sizeunion.get(), 3_0000_0000_0000_0000_usize);

    let c_sizeunion = small_uint_modular_add_func(6_0000_0000_0000_0000_usize.into_sizeunion(), 1_5000_0000_0000_0000_usize.into_sizeunion(), 10_0000_0000_0000_0000_usize.into_sizeunion());
    println!("6_0000_0000_0000_0000 + 1_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", c_sizeunion);
    assert_eq!(c_sizeunion.get(), 7_5000_0000_0000_0000_usize);

    let d_sizeunion = small_uint_modular_add_func(c_sizeunion, 5_5000_0000_0000_0000_usize.into_sizeunion(), 10_0000_0000_0000_0000_usize.into_sizeunion());
    println!("{} + 5_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", c_sizeunion, d_sizeunion);
    assert_eq!(d_sizeunion.get(), 3_0000_0000_0000_0000_usize);
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
    println!("small_uint_borrowing_sub");
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
    let (d_low_u16, d_high_u16, borrow) = small_uint_borrowing_sub_func(b_low_u16, b_high_u16, a_low_u16, a_high_u16);
    println!("{}-{}, {}", d_high_u16, d_low_u16, borrow);
    assert_eq!(d_high_u16, 25535_u16);
    assert_eq!(d_low_u16, 45636_u16);
    assert_eq!(borrow, true);

    // Example for u32
    // a_u64: u64 === (a_high_u32, a_low_u32) == (2299561912_u32, 2956226837_u32) == 9876543210123456789_u64
    let a_high_u32 = 2299561912_u32;
    let a_low_u32 = 2956226837_u32;
    // b_u64: u64 === (b_high_u32, b_low_u32) == (1782160508_u32, 682685733_u32) == 7654321098765432101_u64
    let b_high_u32 = 1782160508_u32;
    let b_low_u32 = 682685733_u32;

    // (2299561912_u32, 2956226837_u32) - (1782160508_u32, 682685733_u32) == 9876543210123456789_u64 - 7654321098765432101_u64 == 2222222111358024688_u64
    //   9876543210123456789_u64 == (2299561912_u32, 2956226837_u32)
    // - 7654321098765432101_u64 == (1782160508_u32,  682685733_u32)
    // -------------------------------------------------------------
    //   2222222111358024688_u64 == ( 517401404_u32, 2273541104_u32)

    // c: u32 === (c_high_u16, c_low_u16)
    let (c_low_u32, c_high_u32, borrow) = small_uint_borrowing_sub_func(a_low_u32, a_high_u32, b_low_u32, b_high_u32);
    println!("{}-{}, {}", c_high_u32, c_low_u32, borrow);
    assert_eq!(c_high_u32, 517401404_u32);
    assert_eq!(c_low_u32, 2273541104_u32);
    assert_eq!(borrow, false);

    // (517401404_u32, 2273541104_u32) - (1782160508_u32,  682685733_u32) == 2222222111358024688_u32 - 7654321098765432101_u32 == 13014645086302144203_u16
    //   2222222111358024688_u64 == ( 517401404_u32, 2273541104_u32)
    // - 7654321098765432101_u64 == (1782160508_u32,  682685733_u32)
    // -------------------------------------------------------------
    //  13014645086302144203_u64 == (3030208192_u32, 1590855371_u32)

    // d: u64 === (d_high_u32, d_low_u32)
    let (d_low_u32, d_high_u32, borrow) = small_uint_borrowing_sub_func(c_low_u32, c_high_u32, b_low_u32, b_high_u32);
    println!("{}-{}, {}", d_high_u32, d_low_u32, borrow);
    assert_eq!(d_high_u32, 3030208192_u32);
    assert_eq!(d_low_u32, 1590855371_u32);
    assert_eq!(borrow, true);

    // Example for u64
    // a_u128: u128 === (a_high_u64, a_low_u64) == (10775095670246085798_u64, 7681743649119882630_u64) == 198765432198765432198765432198765432198_u128
    let a_high_u64 = 10775095670246085798_u64;
    let a_low_u64 = 7681743649119882630_u64;
    // b_u128: u128 === (b_high_u64, b_low_u64) == (6692605942763486917_u64, 12312739301371248917_u64) == 123456789012345678901234567890123456789_u128
    let b_high_u64 = 6692605942763486917_u64;
    let b_low_u64 = 12312739301371248917_u64;

    // (10775095670246085798_u64, 7681743649119882630_u64) - (6692605942763486917_u64, 12312739301371248917_u64) == 198765432198765432198765432198765432198_u128 - 123456789012345678901234567890123456789_u128 == 75308643186419753297530864308641975409_u128
    //   198765432198765432198765432198765432198_u128 == (10775095670246085798_u64,  7681743649119882630_u64)
    // - 123456789012345678901234567890123456789_u128 == ( 6692605942763486917_u64, 12312739301371248917_u64)
    // ------------------------------------------------------------------------------------------------------
    //    75308643186419753297530864308641975409_u128 == ( 4082489727482598880_u64, 13815748421458185329_u64)

    // c: u128 === (c_high_u64, c_low_u64)
    let (c_low_u64, c_high_u64, borrow) = small_uint_borrowing_sub_func(a_low_u64, a_high_u64, b_low_u64, b_high_u64);
    println!("{}-{}, {}", c_high_u64, c_low_u64, borrow);
    assert_eq!(c_high_u64, 4082489727482598880_u64);
    assert_eq!(c_low_u64, 13815748421458185329_u64);
    assert_eq!(borrow, false);

    // (4082489727482598880_u64, 13815748421458185329_u64) - (6692605942763486917_u64, 12312739301371248917_u64) == 75308643186419753297530864308641975409_u128 - 123456789012345678901234567890123456789_u128 == 292134221095012537859670903850286730076_u128
    //    75308643186419753297530864308641975409_u128 == ( 4082489727482598880_u64, 13815748421458185329_u64)
    // - 123456789012345678901234567890123456789_u128 == ( 6692605942763486917_u64, 12312739301371248917_u64)
    // ------------------------------------------------------------------------------------------------------
    //   292134221095012537859670903850286730076_u128 == (14364254346226952735_u64,  4630995652251366287_u64)

    // d: u128 === (d_high_u64, d_low_u64)
    let (d_low_u64, d_high_u64, borrow) = small_uint_borrowing_sub_func(b_low_u64, b_high_u64, a_low_u64, a_high_u64);
    println!("{}-{}, {}", d_high_u64, d_low_u64, borrow);
    assert_eq!(d_high_u64, 14364254346226952735_u64);
    assert_eq!(d_low_u64, 4630995652251366287_u64);
    assert_eq!(borrow, true);

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

    // Example for usize for 64-bit CPU
    // a_u128: u128 === (a_high_usize, a_low_usize) == (10775095670246085798_usize, 7681743649119882630_usize) == 198765432198765432198765432198765432198_u128
    let a_high_usize = 10775095670246085798_usize;
    let a_low_usize = 7681743649119882630_usize;
    // b_u128: u128 === (b_high_usize, b_low_usize) == (6692605942763486917_usize, 12312739301371248917_usize) == 123456789012345678901234567890123456789_u128
    let b_high_usize = 6692605942763486917_usize;
    let b_low_usize = 12312739301371248917_usize;

    // (10775095670246085798_usize, 7681743649119882630_usize) - (6692605942763486917_usize, 12312739301371248917_usize) == 198765432198765432198765432198765432198_u128 - 123456789012345678901234567890123456789_u128 == 75308643186419753297530864308641975409_u128
    //   198765432198765432198765432198765432198_u128 == (10775095670246085798_usize,  7681743649119882630_usize)
    // - 123456789012345678901234567890123456789_u128 == ( 6692605942763486917_usize, 12312739301371248917_usize)
    // ------------------------------------------------------------------------------------------------------
    //    75308643186419753297530864308641975409_u128 == ( 4082489727482598880_usize, 13815748421458185329_usize)

    // c: u128 === (c_high_usize, c_low_usize)
    let (c_low_usize, c_high_usize, borrow) = small_uint_borrowing_sub_func(a_low_usize, a_high_usize, b_low_usize, b_high_usize);
    println!("{}-{}, {}", c_high_usize, c_low_usize, borrow);
    assert_eq!(c_high_usize, 4082489727482598880_usize);
    assert_eq!(c_low_usize, 13815748421458185329_usize);
    assert_eq!(borrow, false);

    // (4082489727482598880_usize, 13815748421458185329_usize) - (6692605942763486917_usize, 12312739301371248917_usize) == 75308643186419753297530864308641975409_u128 - 123456789012345678901234567890123456789_u128 == 292134221095012537859670903850286730076_u128
    //    75308643186419753297530864308641975409_u128 == ( 4082489727482598880_usize, 13815748421458185329_usize)
    // - 123456789012345678901234567890123456789_u128 == ( 6692605942763486917_usize, 12312739301371248917_usize)
    // ------------------------------------------------------------------------------------------------------
    //   292134221095012537859670903850286730076_u128 == (14364254346226952735_usize,  4630995652251366287_usize)

    // d: u128 === (d_high_usize, d_low_usize)
    let (d_low_usize, d_high_usize, borrow) = small_uint_borrowing_sub_func(b_low_usize, b_high_usize, a_low_usize, a_high_usize);
    println!("{}-{}, {}", d_high_usize, d_low_usize, borrow);
    assert_eq!(d_high_usize, 14364254346226952735_usize);
    assert_eq!(d_low_usize, 4630995652251366287_usize);
    assert_eq!(borrow, true);

    // Example for ShortUnion
    // a_u32: u32 === (a_high_shortunion, a_low_shortunion) == (50000_u16, 30000_u16) == 3276830000_u32
    let a_high_shortunion = 50000_u16.into_shortunion();
    let a_low_shortunion = 30000_u16.into_shortunion();
    // b_u32: u32 === (b_high_shortunion, b_low_shortunion) == (10000_u16, 10100_u16) == 655370100_u32
    let b_high_shortunion = 10000_u16.into_shortunion();
    let b_low_shortunion = 10100_u16.into_shortunion();

    // (50000_u16, 30000_u16) - (10000_u16, 10100_u16) == 3276830000_u32 - 655370100_u32 == 99_u16
    //   3276830000_u32 == (50000_u16, 30000_u16)
    // -  655370100_u32 == (10000_u16, 10100_u16)
    // ------------------------------------------
    //   2621459900_u32 == (40000_u16, 19900_u16)

    // c: u32 === (c_high_shortunion, c_low_shortunion)
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

    // d: u32 === (d_high_shortunion, d_low_shortunion)
    let (d_low_shortunion, d_high_shortunion, borrow) = small_uint_borrowing_sub_func(b_low_shortunion, b_high_shortunion, a_low_shortunion, a_high_shortunion);
    println!("{}-{}, {}", d_low_shortunion, d_low_shortunion, borrow);
    assert_eq!(d_high_shortunion.get(), 25535_u16);
    assert_eq!(d_low_shortunion.get(), 45636_u16);
    assert_eq!(borrow, true);

    // Example for IntUnion
    // a_u64: u64 === (a_high_intunion, a_low_intunion) == (2299561912_u32, 2956226837_u32) == 9876543210123456789_u64
    let a_high_intunion = 2299561912_u32.into_intunion();
    let a_low_intunion = 2956226837_u32.into_intunion();
    // b_u64: u64 === (b_high_intunion, b_low_intunion) == (1782160508_u32, 682685733_u32) == 7654321098765432101_u64
    let b_high_intunion = 1782160508_u32.into_intunion();
    let b_low_intunion = 682685733_u32.into_intunion();

    // (2299561912_u32, 2956226837_u32) - (1782160508_u32, 682685733_u32) == 9876543210123456789_u64 - 7654321098765432101_u64 == 2222222111358024688_u64
    //   9876543210123456789_u64 == (2299561912_u32, 2956226837_u32)
    // - 7654321098765432101_u64 == (1782160508_u32,  682685733_u32)
    // -------------------------------------------------------------
    //   2222222111358024688_u64 == ( 517401404_u32, 2273541104_u32)

    // c: u64 === (c_high_intunion, c_low_intunion)
    let (c_low_intunion, c_high_intunion, borrow) = small_uint_borrowing_sub_func(a_low_intunion, a_high_intunion, b_low_intunion, b_high_intunion);
    println!("{}-{}, {}", c_high_intunion, c_low_intunion, borrow);
    assert_eq!(c_high_intunion.get(), 517401404_u32);
    assert_eq!(c_low_intunion.get(), 2273541104_u32);
    assert_eq!(borrow, false);

    // (517401404_u32, 2273541104_u32) - (1782160508_u32,  682685733_u32) == 2222222111358024688_u32 - 7654321098765432101_u32 == 13014645086302144203_u16
    //   2222222111358024688_u64 == ( 517401404_u32, 2273541104_u32)
    // - 7654321098765432101_u64 == (1782160508_u32,  682685733_u32)
    // -------------------------------------------------------------
    //  13014645086302144203_u64 == (3030208192_u32, 1590855371_u32)

    // d: u64 === (d_high_intunion, d_low_intunion)
    let (d_low_intunion, d_high_intunion, borrow) = small_uint_borrowing_sub_func(c_low_intunion, c_high_intunion, b_low_intunion, b_high_intunion);
    println!("{}-{}, {}", d_high_intunion, d_low_intunion, borrow);
    assert_eq!(d_high_intunion.get(), 3030208192_u32);
    assert_eq!(d_low_intunion.get(), 1590855371_u32);
    assert_eq!(borrow, true);

    // Example for LongUnion
    // a_u128: u128 === (a_high_longunion, a_low_longunion) == (10775095670246085798_u64, 7681743649119882630_u64) == 198765432198765432198765432198765432198_u128
    let a_high_longunion = 10775095670246085798_u64.into_longunion();
    let a_low_longunion = 7681743649119882630_u64.into_longunion();
    // b_u128: u128 === (b_high_longunion, b_low_longunion) == (6692605942763486917_u64, 12312739301371248917_u64) == 123456789012345678901234567890123456789_u128
    let b_high_longunion = 6692605942763486917_u64.into_longunion();
    let b_low_longunion = 12312739301371248917_u64.into_longunion();

    // (10775095670246085798_u64, 7681743649119882630_u64) - (6692605942763486917_u64, 12312739301371248917_u64) == 198765432198765432198765432198765432198_u128 - 123456789012345678901234567890123456789_u128 == 75308643186419753297530864308641975409_u128
    //   198765432198765432198765432198765432198_u128 == (10775095670246085798_u64,  7681743649119882630_u64)
    // - 123456789012345678901234567890123456789_u128 == ( 6692605942763486917_u64, 12312739301371248917_u64)
    // ------------------------------------------------------------------------------------------------------
    //    75308643186419753297530864308641975409_u128 == (10775095670246085798_u64,  7681743649119882630_u64)

    // c: u32 === (c_high_u16, c_low_u16)
    let (c_low_longunion, c_high_longunion, borrow) = small_uint_borrowing_sub_func(a_low_longunion, a_high_longunion, b_low_longunion, b_high_longunion);
    println!("{}-{}, {}", c_high_longunion, c_low_longunion, borrow);
    assert_eq!(c_high_longunion.get(), 4082489727482598880_u64);
    assert_eq!(c_low_longunion.get(), 13815748421458185329_u64);
    assert_eq!(borrow, false);

    // (10775095670246085798_u64, 7681743649119882630_u64) - (6692605942763486917_u64, 12312739301371248917_u64) == 75308643186419753297530864308641975409_u128 - 123456789012345678901234567890123456789_u128 == 292134221095012537859670903850286730076_u128
    //    75308643186419753297530864308641975409_u128 == (10775095670246085798_u64,  7681743649119882630_u64)
    // - 123456789012345678901234567890123456789_u128 == ( 6692605942763486917_u64, 12312739301371248917_u64)
    // ------------------------------------------------------------------------------------------------------
    //   292134221095012537859670903850286730076_u128 == (15836627858428663579_u64,  1503009120086936412_u64)

    // d: u128 === (d_high_u64, d_low_u64)
    let (d_low_longunion, d_high_longunion, borrow) = small_uint_borrowing_sub_func(b_low_longunion, b_high_longunion, a_low_longunion, a_high_longunion);
    println!("{}-{}, {}", d_high_longunion, d_low_longunion, borrow);
    assert_eq!(d_high_longunion.get(), 14364254346226952735_u64);
    assert_eq!(d_low_longunion.get(), 4630995652251366287_u64);
    assert_eq!(borrow, true);

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

    // Example for SizeUnion for 64-bit CPU
    // a_u128: u128 === (a_high_usize, a_low_usize) == (10775095670246085798_usize, 7681743649119882630_usize) == 198765432198765432198765432198765432198_u128
    let a_high_sizeunion = 10775095670246085798_usize.into_sizeunion();
    let a_low_sizeunion = 7681743649119882630_usize.into_sizeunion();
    // b_u128: u128 === (b_high_usize, b_low_usize) == (6692605942763486917_usize, 12312739301371248917_usize) == 123456789012345678901234567890123456789_u128
    let b_high_sizeunion = 6692605942763486917_usize.into_sizeunion();
    let b_low_sizeunion = 12312739301371248917_usize.into_sizeunion();

    // (10775095670246085798_usize, 7681743649119882630_usize) - (6692605942763486917_usize, 12312739301371248917_usize) == 198765432198765432198765432198765432198_u128 - 123456789012345678901234567890123456789_u128 == 75308643186419753297530864308641975409_u128
    //   198765432198765432198765432198765432198_u128 == (10775095670246085798_usize,  7681743649119882630_usize)
    // - 123456789012345678901234567890123456789_u128 == ( 6692605942763486917_usize, 12312739301371248917_usize)
    // ------------------------------------------------------------------------------------------------------
    //    75308643186419753297530864308641975409_u128 == ( 4082489727482598880_usize, 13815748421458185329_usize)

    // c: u128 === (c_high_usize, c_low_usize)
    let (c_low_sizeunion, c_high_sizeunion, borrow) = small_uint_borrowing_sub_func(a_low_sizeunion, a_high_sizeunion, b_low_sizeunion, b_high_sizeunion);
    println!("{}-{}, {}", c_high_sizeunion, c_low_sizeunion, borrow);
    assert_eq!(c_high_sizeunion.get(), 4082489727482598880_usize);
    assert_eq!(c_low_sizeunion.get(), 13815748421458185329_usize);
    assert_eq!(borrow, false);

    // (4082489727482598880_usize, 13815748421458185329_usize) - (6692605942763486917_usize, 12312739301371248917_usize) == 75308643186419753297530864308641975409_u128 - 123456789012345678901234567890123456789_u128 == 292134221095012537859670903850286730076_u128
    //    75308643186419753297530864308641975409_u128 == ( 4082489727482598880_usize, 13815748421458185329_usize)
    // - 123456789012345678901234567890123456789_u128 == ( 6692605942763486917_usize, 12312739301371248917_usize)
    // ------------------------------------------------------------------------------------------------------
    //   292134221095012537859670903850286730076_u128 == (14364254346226952735_usize,  4630995652251366287_usize)

    // d: u128 === (d_high_usize, d_low_usize)
    let (d_low_sizeunion, d_high_sizeunion, borrow) = small_uint_borrowing_sub_func(b_low_sizeunion, b_high_sizeunion, a_low_sizeunion, a_high_sizeunion);
    println!("{}-{}, {}", d_high_sizeunion, d_low_sizeunion, borrow);
    assert_eq!(d_high_sizeunion.get(), 14364254346226952735_usize);
    assert_eq!(d_low_sizeunion.get(), 4630995652251366287_usize);
    assert_eq!(borrow, true);
    println!("--------------------------------------");
}

fn small_uint_borrowing_sub_func<T: cryptocol::number::SmallUInt>(lhs_low: T, lhs_high: T, rhs_low: T, rhs_high: T) -> (T, T, bool)
{
    let (dif_low, borrow) = lhs_low.borrowing_sub(rhs_low, false);
    let (dif_high, borrow) = lhs_high.borrowing_sub(rhs_high, borrow);
    (dif_low, dif_high, borrow)
}

fn small_uint_wrapping_sub()
{
    println!("small_uint_wrapping_sub");
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
    println!("small_uint_overflowing_sub");
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
    println!("small_uint_checked_sub");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = small_uint_checked_sub_func(55_u8, 55_u8);
    match a_u8
    {
        Some(a) => {
                println!("55 - 55 = {}", a);
                assert_eq!(a, 0_u8);
            },
        None => { println!("Underflow happened."); },
    }
 
    let b_u8 = small_uint_checked_sub_func(a_u8.unwrap(), 1_u8);
    match b_u8
    {
        Some(b) => { println!("{} - 1 = {}", a_u8.unwrap(), b); },
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
        None => { println!("Underflow happened."); },
    }

    let b_u16 = small_uint_checked_sub_func(a_u16.unwrap(), 1_u16);
    match b_u16
    {
        Some(b) => { println!("{} - 1 = {}", a_u16.unwrap(), b); },
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
        None => { println!("Underflow happened."); },
    }

    let b_u32 = small_uint_checked_sub_func(a_u32.unwrap(), 1_u32);
    match b_u32
    {
        Some(b) => { println!("{} - 1 = {}", a_u32.unwrap(), b); },
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
        None => { println!("Underflow happened."); },
    }

    let b_u64 = small_uint_checked_sub_func(a_u64.unwrap(), 1_u64);
    match b_u64
    {
        Some(b) => { println!("{} - 1 = {}", a_u64.unwrap(), b); },
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
        None => { println!("Underflow happened."); },
    }

    let b_u128 = small_uint_checked_sub_func(a_u128.unwrap(), 1_u128);
    match b_u128
    {
        Some(b) => { println!("{} - 1 = {}", a_u128.unwrap(), b); },
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
        None => { println!("Underflow happened."); },
    }

    let b_usize = small_uint_checked_sub_func(a_usize.unwrap(), 1_usize);
    match b_usize
    {
        Some(b) => { println!("{} - 1 = {}", a_usize.unwrap(), b); },
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
        None => { println!("Underflow happened."); },
    }

    let b_shortunion = small_uint_checked_sub_func(a_shortunion.unwrap(), 1_u16.into_shortunion());
    match b_shortunion
    {
        Some(b) => { println!("{} - 1 = {}", a_shortunion.unwrap(), b); },
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
        None => { println!("Underflow happened."); },
    }

    let b_intunion = small_uint_checked_sub_func(a_intunion.unwrap(), 1_u32.into_intunion());
    match b_intunion
    {
        Some(b) => { println!("{} - 1 = {}", a_intunion.unwrap(), b); },
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
        None => { println!("Underflow happened."); },
    }

    let b_longunion = small_uint_checked_sub_func(a_longunion.unwrap(), 1_u64.into_longunion());
    match b_longunion
    {
        Some(b) => { println!("{} - 1 = {}", a_longunion.unwrap(), b); },
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
        None => { println!("Underflow happened."); },
    }

    let b_longerunion = small_uint_checked_sub_func(a_longerunion.unwrap(), 1_u128.into_longerunion());
    match b_longerunion
    {
        Some(b) => { println!("{} - 1 = {}", a_longerunion.unwrap(), b); },
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
        None => { println!("Underflow happened."); },
    }

    let b_sizeunion = small_uint_checked_sub_func(a_sizeunion.unwrap(), 1_usize.into_sizeunion());
    match b_sizeunion
    {
        Some(b) => { println!("{} - 1 = {}", a_sizeunion.unwrap(), b); },
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
    println!("small_uint_unchecked_sub");
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
    println!("55 - 55 = {}", a_sizeunion);
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
    println!("small_uint_saturating_sub");
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
    println!("{} - 55 = {}", a_intunion, b_intunion);
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
    println!("55 - 50 = {}", a_longerunion);
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
    println!("small_uint_abs_diff");
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
    let a_usize = small_uint_abs_diff_func(55_usize, 50_usize);
    println!("55 <-> 50 = {}", a_usize);
    assert_eq!(a_usize, 5_usize);

    let b_usize = small_uint_abs_diff_func(50_usize, 55_usize);
    println!("50 <-> 55 = {}", b_u8);
    assert_eq!(b_usize, 5_usize);

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
    let a_sizeunion = small_uint_abs_diff_func(55_usize.into_sizeunion(), 50_usize.into_sizeunion());
    println!("55 <-> 50 = {}", a_sizeunion);
    assert_eq!(a_sizeunion.get(), 5_usize);

    let b_sizeunion = small_uint_abs_diff_func(50_usize.into_sizeunion(), 55_usize.into_sizeunion());
    println!("50 <-> 55 = {}", b_sizeunion);
    assert_eq!(b_sizeunion.get(), 5_usize);
    println!("--------------------------------------");
}

fn small_uint_abs_diff_func<T: cryptocol::number::SmallUInt>(lhs: T, rhs: T) -> T
{
    lhs.abs_diff(rhs)
}

fn small_uint_modular_sub()
{
    println!("small_uint_modular_sub");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = 60_u8.modular_sub(55, 100);
    println!("60 - 55 = {} (mod 100)", a_u8);
    assert_eq!(a_u8, 5);

    let b_u8 = a_u8.modular_sub(15, 100);
    println!("{} - 15 = {} (mod 100)", a_u8, b_u8);
    assert_eq!(b_u8, 90);

    let c_u8 = small_uint_modular_sub_func(60_u8, 55, 100);
    println!("60 - 55 = {} (mod 100)", c_u8);
    assert_eq!(c_u8, 5);

    let d_u8 = small_uint_modular_sub_func(c_u8, 15, 100);
    println!("{} - 15 = {} (mod 100)", c_u8, d_u8);
    assert_eq!(d_u8, 90);

    // Example for u16
    let a_u16 = 6000_u16.modular_sub(5500, 1_0000);
    println!("6000 - 5500 = {} (mod 1_0000)", a_u16);
    assert_eq!(a_u16, 500);

    let b_u16 = a_u16.modular_sub(1500, 1_0000);
    println!("{} - 1500 = {} (mod 1_0000)", a_u16, b_u16);
    assert_eq!(b_u16, 9000);

    let c_u16 = small_uint_modular_sub_func(6000_u16, 5500, 1_0000);
    println!("6000 - 5500 = {} (mod 1_0000)", c_u16);
    assert_eq!(c_u16, 500);

    let d_u16 = small_uint_modular_sub_func(c_u16, 1500, 1_0000);
    println!("{} - 1500 = {} (mod 1_0000)", c_u16, d_u16);
    assert_eq!(d_u16, 9000);

    // Example for u32
    let a_u32 = 6_0000_0000_u32.modular_sub(5_5000_0000, 10_0000_0000);
    println!("6_0000_0000 - 5_5000_0000 = {} (mod 10_0000_0000)", a_u32);
    assert_eq!(a_u32, 5000_0000);

    let b_u32 = a_u32.modular_sub(1_5000_0000, 10_0000_0000);
    println!("{} - 1_5000_0000 = {} (mod 10_0000_0000)", a_u32, b_u32);
    assert_eq!(b_u32, 9_0000_0000);

    let c_u32 = small_uint_modular_sub_func(6_0000_0000_u32, 5_5000_0000, 10_0000_0000);
    println!("6_0000_0000 - 5_5000_0000 = {} (mod 10_0000_0000)", c_u32);
    assert_eq!(c_u32, 5000_0000);

    let d_u32 = small_uint_modular_sub_func(c_u32, 1_5000_0000, 10_0000_0000);
    println!("{} - 1_5000_0000 = {} (mod 10_0000_0000)", c_u32, d_u32);
    assert_eq!(d_u32, 9_0000_0000);

    // Example for u64
    let a_u64 = 6_0000_0000_0000_0000_u64.modular_sub(5_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    println!("6_0000_0000_0000_0000 - 5_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_u64);
    assert_eq!(a_u64, 5000_0000_0000_0000);

    let b_u64 = a_u64.modular_sub(1_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    println!("{} - 1_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_u64, b_u64);
    assert_eq!(b_u64, 9_0000_0000_0000_0000);

    let c_u64 = small_uint_modular_sub_func(6_0000_0000_0000_0000_u64, 5_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    println!("6_0000_0000_0000_0000 - 5_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", c_u64);
    assert_eq!(c_u64, 5000_0000_0000_0000);

    let d_u64 = small_uint_modular_sub_func(c_u64, 1_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    println!("{} - 1_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", c_u64, d_u64);
    assert_eq!(d_u64, 9_0000_0000_0000_0000);

    // Example for u128
    let a_u128 = 6_0000_0000_0000_0000_0000_0000_0000_0000_u128.modular_sub(5_5000_0000_0000_0000_0000_0000_0000_0000, 10_0000_0000_0000_0000_0000_0000_0000_0000);
    println!("6_0000_0000_0000_0000_0000_0000_0000_0000 - 5_5000_0000_0000_0000_0000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000_0000_0000_0000_0000)", a_u128);
    assert_eq!(a_u128, 5000_0000_0000_0000_0000_0000_0000_0000);

    let b_u128 = a_u128.modular_sub(1_5000_0000_0000_0000_0000_0000_0000_0000, 10_0000_0000_0000_0000_0000_0000_0000_0000);
    println!("{} - 1_5000_0000_0000_0000_0000_0000_0000_0000 = {}",a_u128, b_u128);
    assert_eq!(b_u128, 9_0000_0000_0000_0000_0000_0000_0000_0000);

    let c_u128 = small_uint_modular_sub_func(6_0000_0000_0000_0000_0000_0000_0000_0000_u128, 5_5000_0000_0000_0000_0000_0000_0000_0000, 10_0000_0000_0000_0000_0000_0000_0000_0000);
    println!("6_0000_0000_0000_0000_0000_0000_0000_0000 - 5_5000_0000_0000_0000_0000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000_0000_0000_0000_0000)", c_u128);
    assert_eq!(c_u128, 5000_0000_0000_0000_0000_0000_0000_0000);

    let d_u128 = small_uint_modular_sub_func(c_u128, 1_5000_0000_0000_0000_0000_0000_0000_0000, 10_0000_0000_0000_0000_0000_0000_0000_0000);
    println!("{} - 1_5000_0000_0000_0000_0000_0000_0000_0000 = {}",c_u128, d_u128);
    assert_eq!(d_u128, 9_0000_0000_0000_0000_0000_0000_0000_0000);

    // Example for usize
    let a_usize = 6_0000_0000_0000_0000_usize.modular_sub(5_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    println!("6_0000_0000_0000_0000 - 5_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_usize);
    assert_eq!(a_usize, 5000_0000_0000_0000);

    let b_usize = a_usize.modular_sub(1_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    println!("{} - 1_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_usize, b_usize);
    assert_eq!(b_usize, 9_0000_0000_0000_0000);

    let c_usize = small_uint_modular_sub_func(6_0000_0000_0000_0000_usize, 5_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    println!("6_0000_0000_0000_0000 - 5_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", c_usize);
    assert_eq!(c_usize, 5000_0000_0000_0000);

    let d_usize = small_uint_modular_sub_func(c_usize, 1_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    println!("{} - 1_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", c_usize, d_usize);
    assert_eq!(d_usize, 9_0000_0000_0000_0000);

    // Example for ShortUnion
    let a_shortunion = 6000_u16.into_shortunion().modular_sub(5500_u16.into_shortunion(), 1_0000_u16.into_shortunion());
    println!("6000 - 5500 = {} (mod 1_0000)", a_shortunion);
    assert_eq!(a_shortunion.get(), 500);

    let b_shortunion = a_shortunion.modular_sub(1500_u16.into_shortunion(), 1_0000_u16.into_shortunion());
    println!("{} - 1500 = {} (mod 1_0000)", a_shortunion, b_shortunion);
    assert_eq!(b_shortunion.get(), 9000);

    let c_shortunion = small_uint_modular_sub_func(6000_u16.into_shortunion(), 5500_u16.into_shortunion(), 1_0000_u16.into_shortunion());
    println!("6000 - 5500 = {} (mod 1_0000)", c_shortunion);
    assert_eq!(c_shortunion.get(), 500);

    let d_shortunion = small_uint_modular_sub_func(c_shortunion, 1500_u16.into_shortunion(), 1_0000_u16.into_shortunion());
    println!("{} - 1500 = {} (mod 1_0000)", c_shortunion, d_shortunion);
    assert_eq!(d_shortunion.get(), 9000);

    // Example for IntUnion
    let a_intunion = 6_0000_0000_u32.into_intunion().modular_sub(5_5000_0000_u32.into_intunion(), 10_0000_0000_u32.into_intunion());
    println!("6_0000_0000 - 5_5000_0000 = {} (mod 10_0000_0000)", a_intunion);
    assert_eq!(a_intunion.get(), 5000_0000);

    let b_intunion = a_intunion.modular_sub(1_5000_0000_u32.into_intunion(), 10_0000_0000_u32.into_intunion());
    println!("{} - 1_5000_0000 = {} (mod 10_0000_0000)", a_intunion, b_intunion);
    assert_eq!(b_intunion.get(), 9_0000_0000);

    let c_intunion = small_uint_modular_sub_func(6_0000_0000_u32.into_intunion(), 5_5000_0000_u32.into_intunion(), 10_0000_0000_u32.into_intunion());
    println!("6_0000_0000 - 5_5000_0000 = {} (mod 10_0000_0000)", c_intunion);
    assert_eq!(c_intunion.get(), 5000_0000);

    let d_intunion = small_uint_modular_sub_func(c_intunion, 1_5000_0000_u32.into_intunion(), 10_0000_0000_u32.into_intunion());
    println!("{} - 1_5000_0000 = {} (mod 10_0000_0000)", c_intunion, d_intunion);
    assert_eq!(d_intunion.get(), 9_0000_0000);

    // Example for LongUnion
    let a_longunion = 6_0000_0000_0000_0000_u64.into_longunion().modular_sub(5_5000_0000_0000_0000_u64.into_longunion(), 10_0000_0000_0000_0000_u64.into_longunion());
    println!("6_0000_0000_0000_0000 - 5_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_longunion);
    assert_eq!(a_longunion.get(), 5000_0000_0000_0000);

    let b_longunion = a_longunion.modular_sub(1_5000_0000_0000_0000_u64.into_longunion(), 10_0000_0000_0000_0000_u64.into_longunion());
    println!("{} - 1_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_longunion, b_longunion);
    assert_eq!(b_longunion.get(), 9_0000_0000_0000_0000);

    let c_longunion = small_uint_modular_sub_func(6_0000_0000_0000_0000_u64.into_longunion(), 5_5000_0000_0000_0000_u64.into_longunion(), 10_0000_0000_0000_0000_u64.into_longunion());
    println!("6_0000_0000_0000_0000 - 5_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", c_longunion);
    assert_eq!(c_longunion.get(), 5000_0000_0000_0000);

    let d_longunion = small_uint_modular_sub_func(c_longunion, 1_5000_0000_0000_0000_u64.into_longunion(), 10_0000_0000_0000_0000_u64.into_longunion());
    println!("{} - 1_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", c_longunion, d_longunion);
    assert_eq!(d_longunion.get(), 9_0000_0000_0000_0000);

    // Example for LongerUnion
    let a_longerunion = 6_0000_0000_0000_0000_0000_0000_0000_0000_u128.into_longerunion().modular_sub(5_5000_0000_0000_0000_0000_0000_0000_0000_u128.into_longerunion(), 10_0000_0000_0000_0000_0000_0000_0000_0000_u128.into_longerunion());
    println!("6_0000_0000_0000_0000_0000_0000_0000_0000 - 5_5000_0000_0000_0000_0000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000_0000_0000_0000_0000)", a_longerunion);
    assert_eq!(a_longerunion.get(), 5000_0000_0000_0000_0000_0000_0000_0000);

    let b_longerunion = a_longerunion.modular_sub(1_5000_0000_0000_0000_0000_0000_0000_0000_u128.into_longerunion(), 10_0000_0000_0000_0000_0000_0000_0000_0000_u128.into_longerunion());
    println!("{} - 1_5000_0000_0000_0000_0000_0000_0000_0000 = {}", a_longerunion, b_longerunion);
    assert_eq!(b_longerunion.get(), 9_0000_0000_0000_0000_0000_0000_0000_0000);

    let c_longerunion = small_uint_modular_sub_func(6_0000_0000_0000_0000_0000_0000_0000_0000_u128.into_longerunion(), 5_5000_0000_0000_0000_0000_0000_0000_0000_u128.into_longerunion(), 10_0000_0000_0000_0000_0000_0000_0000_0000_u128.into_longerunion());
    println!("6_0000_0000_0000_0000_0000_0000_0000_0000 - 5_5000_0000_0000_0000_0000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000_0000_0000_0000_0000)", c_longerunion);
    assert_eq!(c_longerunion.get(), 5000_0000_0000_0000_0000_0000_0000_0000);

    let d_longerunion = small_uint_modular_sub_func(c_longerunion, 1_5000_0000_0000_0000_0000_0000_0000_0000_u128.into_longerunion(), 10_0000_0000_0000_0000_0000_0000_0000_0000_u128.into_longerunion());
    println!("{} - 1_5000_0000_0000_0000_0000_0000_0000_0000 = {}", c_longerunion, d_longerunion);
    assert_eq!(d_longerunion.get(), 9_0000_0000_0000_0000_0000_0000_0000_0000);

    // Example for SizeUnion
    let a_sizeunion = 6_0000_0000_0000_0000_usize.into_sizeunion().modular_sub(5_5000_0000_0000_0000_usize.into_sizeunion(), 10_0000_0000_0000_0000_usize.into_sizeunion());
    println!("6_0000_0000_0000_0000 - 5_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_sizeunion);
    assert_eq!(a_sizeunion.get(), 5000_0000_0000_0000);

    let b_sizeunion = a_sizeunion.modular_sub(1_5000_0000_0000_0000_usize.into_sizeunion(), 10_0000_0000_0000_0000_usize.into_sizeunion());
    println!("{} - 1_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_sizeunion, b_sizeunion);
    assert_eq!(b_sizeunion.get(), 9_0000_0000_0000_0000);

    let c_sizeunion = small_uint_modular_sub_func(6_0000_0000_0000_0000_usize.into_sizeunion(), 5_5000_0000_0000_0000_usize.into_sizeunion(), 10_0000_0000_0000_0000_usize.into_sizeunion());
    println!("6_0000_0000_0000_0000 - 5_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", c_sizeunion);
    assert_eq!(c_sizeunion.get(), 5000_0000_0000_0000);

    let d_sizeunion = small_uint_modular_sub_func(c_sizeunion, 1_5000_0000_0000_0000_usize.into_sizeunion(), 10_0000_0000_0000_0000_usize.into_sizeunion());
    println!("{} - 1_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", c_sizeunion, d_sizeunion);
    assert_eq!(d_sizeunion.get(), 9_0000_0000_0000_0000);
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
    println!("small_uint_carrying_mul");
    use cryptocol::number::{ SmallUInt, IntUnion, LongUnion, LongerUnion };

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

    // Example for u32 for Little Endian
    // a_u64: u64 === (a_high_u32, a_low_u32) == (2299561912_u32, 2956226837_u32) == 9876543210123456789_u64
    let a_high_u32 = 2299561912_u32;
    let a_low_u32 = 2956226837_u32;
    // b_u64: u64 === (b_high_u32, b_low_u32) == (1782160508_u32, 682685733_u32) == 7654321098765432101_u64
    let b_high_u32 = 1782160508_u32;
    let b_low_u32 = 682685733_u32;

    // (2299561912_u32, 2956226837_u32) X (1782160508_u32, 682685733_u32) == 9876543210123456789_u64 X 7654321098765432101_u64 == (4098188426859548455_u64, 17997868695111430409_u64) == 75598233076116445704676116321386983689_u128
    //
    //                                  (2299561912_u32, 2956226837_u32) == 9876543210123456789_u64
    // X                                (1782160508_u32,  682685733_u32) == 7654321098765432101_u64
    // -----------------------------------------------------------------
    //                                  ( 469892724_u32, 2923262217_u32)
    //                  ( 365515730_u32, 2949035416_u32)
    //                  (1226661429_u32,  771527212_u32)
    // + (954183848_u32, 3735936288_u32)
    // -----------------------------------------------------------------
    //   (954183849_u32, 1033146151_u32, 4190455352_u32, 2923262217_u32) == 75598233076116445704676116321386983689_u128
    let (c_lower_u32, c_low_u32, c_high_u32, c_higher_u32 ) = small_uint_carrying_mul_func(a_low_u32, a_high_u32, b_low_u32, b_high_u32);
    println!("{}-{}-{}-{}", c_higher_u32, c_high_u32, c_low_u32, c_lower_u32);
    assert_eq!(c_higher_u32, 954183849_u32);
    assert_eq!(c_high_u32, 1033146151_u32);
    assert_eq!(c_low_u32, 4190455352_u32);
    assert_eq!(c_lower_u32, 2923262217_u32);

    let a = LongerUnion::new_with_uints([a_low_u32, a_high_u32, 0, 0]);
    let b = LongerUnion::new_with_uints([b_low_u32, b_high_u32, 0, 0]);
    let c = a * b;
    println!("{} * {} = {}", a.get(), b.get(), c.get());
    assert_eq!(c_higher_u32, c.get_uint_(3));
    assert_eq!(c_high_u32, c.get_uint_(2));
    assert_eq!(c_low_u32, c.get_uint_(1));
    assert_eq!(c_lower_u32, c.get_uint_(0));

    // Example for u64 for Little Endian
    // a_u128: u128 === (a_high_u64, a_low_u64) == (10775095670246085798_u64, 7681743649119882630_u64) == 198765432198765432198765432198765432198_u128
    let a_high_u64 = 10775095670246085798_u64;
    let a_low_u64 = 7681743649119882630_u64;
    // b_u64: u64 === (b_high_u64, b_low_u64) == (6692605942763486917_u64, 12312739301371248917_u64) == 123456789012345678901234567890123456789_u128
    let b_high_u64 = 6692605942763486917_u64;
    let b_low_u64 = 12312739301371248917_u64;

    // (10775095670246085798_u64, 7681743649119882630_u64) X (6692605942763486917_u64, 12312739301371248917_u64) == 198765432198765432198765432198765432198_u128 X 123456789012345678901234567890123456789_u128 == (72113469316534070997571940237811086202_u128, 284839445932509422190795104397182362110_u128) == 24538942025910684226047858446061575867965995914594253912457079712243362292222_u256
    //
    //                                                      (10775095670246085798_u64,  7681743649119882630_u64) == 198765432198765432198765432198765432198_u128
    // X                                                    ( 6692605942763486917_u64, 12312739301371248917_u64) == 123456789012345678901234567890123456789_u128
    // ---------------------------------------------------------------------------------------------------------
    //                                                      ( 5127371342803972846_u64,  9393535397455192574_u64)
    //                             (7192106282005498115_u64,  3473120370613376926_u64)
    //                             (2786989562573083321_u64,  6840685591062354974_u64)
    // + (3909279004922650219_u64,  1464703988338300862_u64)
    // ---------------------------------------------------------------------------------------------------------
    //   (3909279004922650219_u64, 11443799832916882298_u64, 15441177304479704746_u64,  9393535397455192574_u64) == 24538942025910684226047858446061575867965995914594253912457079712243362292222_u256
    let (c_lower_u64, c_low_u64, c_high_u64, c_higher_u64 ) = small_uint_carrying_mul_func(a_low_u64, a_high_u64, b_low_u64, b_high_u64);
    println!("{}-{}-{}-{}", c_higher_u64, c_high_u64, c_low_u64, c_lower_u64);
    assert_eq!(c_higher_u64, 3909279004922650219_u64);
    assert_eq!(c_high_u64, 11443799832916882298_u64);
    assert_eq!(c_low_u64, 15441177304479704746_u64);
    assert_eq!(c_lower_u64, 9393535397455192574_u64);

    // Example for u128 for Little Endian
    // a_u256: u256 === (a_high_u128, a_low_u128) == (123456789012345678901234567890123456789_u128, 198765432198765432198765432198765432198_u128) == 42010168377579896403540037778015643756626903575004241358522734820017396206982_u256
    let a_high_u128 = 123456789012345678901234567890123456789_u128;
    let a_low_u128 = 198765432198765432198765432198765432198_u128;
    // b_u256: u256 === (b_high_u128, b_low_u128) == (75318642097531864209753186420975318642_u128, 135792468013579246801357924680135792468_u128) == 25629605806219180037134371884461041203042609997744073457419340831856170555220_u256
    let b_high_u128 = 75318642097531864209753186420975318642_u128;
    let b_low_u128 = 135792468013579246801357924680135792468_u128;

    // (123456789012345678901234567890123456789_u128, 198765432198765432198765432198765432198_u128) X (75318642097531864209753186420975318642_u128 - 135792468013579246801357924680135792468_u128) == 42010168377579896403540037778015643756626903575004241358522734820017396206982_u256 X 25629605806219180037134371884461041203042609997744073457419340831856170555220_u256 = 1076704055370267103358067448344494207403929951418850598311166733254725709101675518708273284527051744761749874770306207984521811586513200762632500980546040_u512
    //
    //                                                                                              (123456789012345678901234567890123456789_u128, 198765432198765432198765432198765432198_u128) == 42010168377579896403540037778015643756626903575004241358522734820017396206982_u256
    // X                                                                                            ( 75318642097531864209753186420975318642_u128, 135792468013579246801357924680135792468_u128) == 25629605806219180037134371884461041203042609997744073457419340831856170555220_u256
    // -----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
    //                                                                                              ( 79318975115531594676802389315672824709_u128, 305933135181961371815664194362919418360_u128)
    //                                                ( 49266443702953415606417933871327680361_u128, 301235724958848324675382352967843249636_u128)
    //                                                ( 43995057941448862830514490586650222101_u128,  35386202970580104685103432753963846060_u128)
    // + (27326122685316262062508597076325453266_u128, 184240100967607654057575481238459345242_u128)
    // -----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
    //   (27326122685316262062508597076325453266_u128, 277501602612009932494507905696437247705_u128,  75658536124021560573913567605711708949_u128, 305933135181961371815664194362919418360_u128) == 1076704055370267103358067448344494207403929951418850598311166733254725709101675518708273284527051744761749874770306207984521811586513200762632500980546040_u512
    let (c_lower_u128, c_low_u128, c_high_u128, c_higher_u128 ) = small_uint_carrying_mul_func(a_low_u128, a_high_u128, b_low_u128, b_high_u128);
    println!("{}-{}-{}-{}", c_higher_u128, c_high_u128, c_low_u128, c_lower_u128);
    assert_eq!(c_higher_u128, 27326122685316262062508597076325453266_u128);
    assert_eq!(c_high_u128, 277501602612009932494507905696437247705_u128);
    assert_eq!(c_low_u128, 75658536124021560573913567605711708949_u128);
    assert_eq!(c_lower_u128, 305933135181961371815664194362919418360_u128);

    // Example for usize for 64-bit CPUs for Little Endian
    #[cfg(target_pointer_width = "64")]
    {
        // a_u128: u128 === (a_high_usize, a_low_usize) == (10775095670246085798_usize, 7681743649119882630_usize) == 198765432198765432198765432198765432198_u128
        let a_high_usize = 10775095670246085798_usize;
        let a_low_usize = 7681743649119882630_usize;
        // b_usize: usize === (b_high_usize, b_low_usize) == (6692605942763486917_usize, 12312739301371248917_usize) == 123456789012345678901234567890123456789_u128
        let b_high_usize = 6692605942763486917_usize;
        let b_low_usize = 12312739301371248917_usize;

        // (10775095670246085798_usize, 7681743649119882630_usize) X (6692605942763486917_usize, 12312739301371248917_usize) == 198765432198765432198765432198765432198_u128 X 123456789012345678901234567890123456789_u128 == (72113469316534070997571940237811086202_u128, 284839445932509422190795104397182362110_u128) == 24538942025910684226047858446061575867965995914594253912457079712243362292222_u256
        //
        //                                                          (10775095670246085798_usize,  7681743649119882630_usize) == 198765432198765432198765432198765432198_u128
        // X                                                        ( 6692605942763486917_usize, 12312739301371248917_usize) == 123456789012345678901234567890123456789_u128
        // -----------------------------------------------------------------------------------------------------------------
        //                                                          ( 5127371342803972846_usize,  9393535397455192574_usize)
        //                               (7192106282005498115_usize,  3473120370613376926_usize)
        //                               (2786989562573083321_usize,  6840685591062354974_usize)
        // + (3909279004922650219_usize,  1464703988338300862_usize)
        // -----------------------------------------------------------------------------------------------------------------
        //   (3909279004922650219_usize, 11443799832916882298_usize, 15441177304479704746_usize,  9393535397455192574_usize) == 24538942025910684226047858446061575867965995914594253912457079712243362292222_u256
        let (c_lower_usize, c_low_usize, c_high_usize, c_higher_usize ) = small_uint_carrying_mul_func(a_low_usize, a_high_usize, b_low_usize, b_high_usize);
        println!("{}-{}-{}-{}", c_higher_usize, c_high_usize, c_low_usize, c_lower_usize);
        assert_eq!(c_higher_usize, 3909279004922650219_usize);
        assert_eq!(c_high_usize, 11443799832916882298_usize);
        assert_eq!(c_low_usize, 15441177304479704746_usize);
        assert_eq!(c_lower_usize, 9393535397455192574_usize);
    }

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
    assert_eq!(c_higher_shortunion.get(), 1525_u16);
    assert_eq!(c_high_shortunion.get(), 62192_u16);
    assert_eq!(c_low_shortunion.get(), 61770_u16);
    assert_eq!(c_lower_shortunion.get(), 18048_u16);

    let a = LongUnion::new_with_ushorts([a_low_shortunion.get(), a_high_shortunion.get(), 0, 0]);
    let b = LongUnion::new_with_ushorts([b_low_shortunion.get(), b_high_shortunion.get(), 0, 0]);
    let c = a * b;
    println!("{} * {} = {}", a.get(), b.get(), c.get());
    assert_eq!(c_higher_shortunion.get(), c.get_ushort_(3));
    assert_eq!(c_high_shortunion.get(), c.get_ushort_(2));
    assert_eq!(c_low_shortunion.get(), c.get_ushort_(1));
    assert_eq!(c_lower_shortunion.get(), c.get_ushort_(0));

    // Example for IntUnion for Little Endian
    // a_u64: u64 === (a_high_u32, a_low_u32) == (2299561912_u32, 2956226837_u32) == 9876543210123456789_u64
    let a_high_intunion = 2299561912_u32.into_intunion();
    let a_low_intunion = 2956226837_u32.into_intunion();
    // b_u64: u64 === (b_high_u32, b_low_u32) == (1782160508_u32, 682685733_u32) == 7654321098765432101_u64
    let b_high_intunion = 1782160508_u32.into_intunion();
    let b_low_intunion = 682685733_u32.into_intunion();

    // (2299561912_u32, 2956226837_u32) X (1782160508_u32, 682685733_u32) == 9876543210123456789_u64 X 7654321098765432101_u64 == (4098188426859548455_u64, 17997868695111430409_u64) == 24538942025910684226047858446061575867965995914594253912457079712243362292222_u256
    //
    //                                  (2299561912_u32, 2956226837_u32) == 9876543210123456789_u64
    // X                                (1782160508_u32,  682685733_u32) == 7654321098765432101_u64
    // -----------------------------------------------------------------
    //                                  ( 469892724_u32, 2923262217_u32)
    //                  ( 365515730_u32, 2949035416_u32)
    //                  (1226661429_u32,  771527212_u32)
    // + (954183848_u32, 3735936288_u32)
    // -----------------------------------------------------------------
    //   (954183849_u32, 1033146151_u32, 4190455352_u32, 2923262217_u32) == 429516456138000000_u64
    let (c_lower_intunion, c_low_intunion, c_high_intunion, c_higher_intunion ) = small_uint_carrying_mul_func(a_low_intunion, a_high_intunion, b_low_intunion, b_high_intunion);
    println!("{}-{}-{}-{}", c_higher_intunion, c_high_intunion, c_low_intunion, c_lower_intunion);
    assert_eq!(c_higher_intunion.get(), 954183849_u32);
    assert_eq!(c_high_intunion.get(), 1033146151_u32);
    assert_eq!(c_low_intunion.get(), 4190455352_u32);
    assert_eq!(c_lower_intunion.get(), 2923262217_u32);

    let a = LongerUnion::new_with_uints([a_low_intunion.get(), a_high_intunion.get(), 0, 0]);
    let b = LongerUnion::new_with_uints([b_low_intunion.get(), b_high_intunion.get(), 0, 0]);
    let c = a * b;
    println!("{} * {} = {}", a.get(), b.get(), c.get());
    assert_eq!(c_higher_intunion.get(), c.get_uint_(3));
    assert_eq!(c_high_intunion.get(), c.get_uint_(2));
    assert_eq!(c_low_intunion.get(), c.get_uint_(1));
    assert_eq!(c_lower_intunion.get(), c.get_uint_(0));

    // Example for LongUnion for Little Endian
    // a_u128: u128 === (a_high_u64, a_low_u64) == (10775095670246085798_u64, 7681743649119882630_u64) == 198765432198765432198765432198765432198_u128
    let a_high_longunion = 10775095670246085798_u64.into_longunion();
    let a_low_longunion = 7681743649119882630_u64.into_longunion();
    // b_u64: u64 === (b_high_u64, b_low_u64) == (6692605942763486917_u64, 12312739301371248917_u64) == 123456789012345678901234567890123456789_u128
    let b_high_longunion = 6692605942763486917_u64.into_longunion();
    let b_low_longunion = 12312739301371248917_u64.into_longunion();

    // (10775095670246085798_u64, 7681743649119882630_u64) X (6692605942763486917_u64, 12312739301371248917_u64) == 198765432198765432198765432198765432198_u128 X 123456789012345678901234567890123456789_u128 == (72113469316534070997571940237811086202_u128, 284839445932509422190795104397182362110_u128) == 24538942025910684226047858446061575867965995914594253912457079712243362292222_u256
    //
    //                                                      (10775095670246085798_u64,  7681743649119882630_u64) == 198765432198765432198765432198765432198_u128
    // X                                                    ( 6692605942763486917_u64, 12312739301371248917_u64) == 123456789012345678901234567890123456789_u128
    // ---------------------------------------------------------------------------------------------------------
    //                                                      ( 5127371342803972846_u64,  9393535397455192574_u64)
    //                             (7192106282005498115_u64,  3473120370613376926_u64)
    //                             (2786989562573083321_u64,  6840685591062354974_u64)
    // + (3909279004922650219_u64,  1464703988338300862_u64)
    // ---------------------------------------------------------------------------------------------------------
    //   (3909279004922650219_u64, 11443799832916882298_u64, 15441177304479704746_u64,  9393535397455192574_u64) == 24538942025910684226047858446061575867965995914594253912457079712243362292222_u256
    let (c_lower_longunion, c_low_longunion, c_high_longunion, c_higher_longunion ) = small_uint_carrying_mul_func(a_low_longunion, a_high_longunion, b_low_longunion, b_high_longunion);
    println!("{}-{}-{}-{}", c_higher_longunion, c_high_longunion, c_low_longunion, c_lower_longunion);
    assert_eq!(c_higher_longunion.get(), 3909279004922650219_u64);
    assert_eq!(c_high_longunion.get(), 11443799832916882298_u64);
    assert_eq!(c_low_longunion.get(), 15441177304479704746_u64);
    assert_eq!(c_lower_longunion.get(), 9393535397455192574_u64);

    // Example for LongerUnion for Little Endian
    // a_u256: u256 === (a_high_u128, a_low_u128) == (123456789012345678901234567890123456789_u128, 198765432198765432198765432198765432198_u128) == 42010168377579896403540037778015643756626903575004241358522734820017396206982_u256
    let a_high_longerunion = 123456789012345678901234567890123456789_u128.into_longerunion();
    let a_low_longerunion = 198765432198765432198765432198765432198_u128.into_longerunion();
    // b_u256: u256 === (b_high_u128, b_low_u128) == (75318642097531864209753186420975318642_u128, 135792468013579246801357924680135792468_u128) == 25629605806219180037134371884461041203042609997744073457419340831856170555220_u256
    let b_high_longerunion = 75318642097531864209753186420975318642_u128.into_longerunion();
    let b_low_longerunion = 135792468013579246801357924680135792468_u128.into_longerunion();

    // (123456789012345678901234567890123456789_u128, 198765432198765432198765432198765432198_u128) X (75318642097531864209753186420975318642_u128 - 135792468013579246801357924680135792468_u128) == 42010168377579896403540037778015643756626903575004241358522734820017396206982_u256 X 25629605806219180037134371884461041203042609997744073457419340831856170555220_u256 = 1076704055370267103358067448344494207403929951418850598311166733254725709101675518708273284527051744761749874770306207984521811586513200762632500980546040_u512
    //
    //                                                                                              (123456789012345678901234567890123456789_u128, 198765432198765432198765432198765432198_u128) == 42010168377579896403540037778015643756626903575004241358522734820017396206982_u256
    // X                                                                                            ( 75318642097531864209753186420975318642_u128, 135792468013579246801357924680135792468_u128) == 25629605806219180037134371884461041203042609997744073457419340831856170555220_u256
    // -----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
    //                                                                                              ( 79318975115531594676802389315672824709_u128, 305933135181961371815664194362919418360_u128)
    //                                                ( 49266443702953415606417933871327680361_u128, 301235724958848324675382352967843249636_u128)
    //                                                ( 43995057941448862830514490586650222101_u128,  35386202970580104685103432753963846060_u128)
    // + (27326122685316262062508597076325453266_u128, 184240100967607654057575481238459345242_u128)
    // -----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
    //   (27326122685316262062508597076325453266_u128, 277501602612009932494507905696437247705_u128,  75658536124021560573913567605711708949_u128, 305933135181961371815664194362919418360_u128) == 1076704055370267103358067448344494207403929951418850598311166733254725709101675518708273284527051744761749874770306207984521811586513200762632500980546040_u512
    let (c_lower_longerunion, c_low_longerunion, c_high_longerunion, c_higher_longerunion ) = small_uint_carrying_mul_func(a_low_longerunion, a_high_longerunion, b_low_longerunion, b_high_longerunion);
    println!("{}-{}-{}-{}", c_higher_longerunion, c_high_longerunion, c_low_longerunion, c_lower_longerunion);
    assert_eq!(c_higher_longerunion.get(), 27326122685316262062508597076325453266_u128);
    assert_eq!(c_high_longerunion.get(), 277501602612009932494507905696437247705_u128);
    assert_eq!(c_low_longerunion.get(), 75658536124021560573913567605711708949_u128);
    assert_eq!(c_lower_longerunion.get(), 305933135181961371815664194362919418360_u128);

    // Example for SizeUnion for 64-bit CPUs for Little Endian
    #[cfg(target_pointer_width = "64")]
    {
        // a_u128: u128 === (a_high_usize, a_low_usize) == (10775095670246085798_usize, 7681743649119882630_usize) == 198765432198765432198765432198765432198_u128
        let a_high_sizeunion = 10775095670246085798_usize.into_sizeunion();
        let a_low_sizeunion = 7681743649119882630_usize.into_sizeunion();
        // b_usize: usize === (b_high_usize, b_low_usize) == (6692605942763486917_usize, 12312739301371248917_usize) == 123456789012345678901234567890123456789_u128
        let b_high_sizeunion = 6692605942763486917_usize.into_sizeunion();
        let b_low_sizeunion = 12312739301371248917_usize.into_sizeunion();

        // (10775095670246085798_usize, 7681743649119882630_usize) X (6692605942763486917_usize, 12312739301371248917_usize) == 198765432198765432198765432198765432198_u128 X 123456789012345678901234567890123456789_u128 == (72113469316534070997571940237811086202_u128, 284839445932509422190795104397182362110_u128) == 24538942025910684226047858446061575867965995914594253912457079712243362292222_u256
        //
        //                                                          (10775095670246085798_usize,  7681743649119882630_usize) == 198765432198765432198765432198765432198_u128
        // X                                                        ( 6692605942763486917_usize, 12312739301371248917_usize) == 123456789012345678901234567890123456789_u128
        // -----------------------------------------------------------------------------------------------------------------
        //                                                          ( 5127371342803972846_usize,  9393535397455192574_usize)
        //                               (7192106282005498115_usize,  3473120370613376926_usize)
        //                               (2786989562573083321_usize,  6840685591062354974_usize)
        // + (3909279004922650219_usize,  1464703988338300862_usize)
        // -----------------------------------------------------------------------------------------------------------------
        //   (3909279004922650219_usize, 11443799832916882298_usize, 15441177304479704746_usize,  9393535397455192574_usize) == 24538942025910684226047858446061575867965995914594253912457079712243362292222_u256
        let (c_lower_sizeunion, c_low_sizeunion, c_high_sizeunion, c_higher_sizeunion ) = small_uint_carrying_mul_func(a_low_sizeunion, a_high_sizeunion, b_low_sizeunion, b_high_sizeunion);
        println!("{}-{}-{}-{}", c_higher_sizeunion, c_high_sizeunion, c_low_sizeunion, c_lower_sizeunion);
        assert_eq!(c_higher_sizeunion.get(), 3909279004922650219_usize);
        assert_eq!(c_high_sizeunion.get(), 11443799832916882298_usize);
        assert_eq!(c_low_sizeunion.get(), 15441177304479704746_usize);
        assert_eq!(c_lower_sizeunion.get(), 9393535397455192574_usize);
    }
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
    println!("small_uint_widening_mul");
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
    // ---------------------------------------------
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
    // ---------------------------------------------
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
    println!("small_uint_wrapping_mul");
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
    println!("small_uint_overflowing_mul");
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
    println!("small_uint_checked_mul");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = small_uint_checked_mul_func(u8::MAX / 3, 2_u8);
    match a_u8
    {
        Some(a) => {
                println!("{} * 2 = {}", u8::MAX / 3, a_u8.unwrap());
                assert_eq!(a, 170_u8);
            },
        None => { println!("Overflow happened."); },
    }

    let b_u8 = small_uint_checked_mul_func(a_u8.unwrap(), 2_u8);
    match b_u8
    {
        Some(b) => { println!("{} * 2 = {}", a_u8.unwrap(), b); },
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
        None => { println!("Overflow happened."); },
    }

    let b_u16 = small_uint_checked_mul_func(a_u16.unwrap(), 2_u16);
    match b_u16
    {
        Some(b) => { println!("{} * 2 = {}", a_u16.unwrap(), b); },
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
        None => { println!("Overflow happened."); },
    }

    let b_u32 = small_uint_checked_mul_func(a_u32.unwrap(), 2_u32);
    match b_u32
    {
        Some(b) => { println!("{} * 2 = {}", a_u32.unwrap(), b); },
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
        None => { println!("Overflow happened."); },
    }

    let b_u64 = small_uint_checked_mul_func(a_u64.unwrap(), 2_u64);
    match b_u64
    {
        Some(b) => { println!("{} * 2 = {}", a_u64.unwrap(), b); },
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
        None => { println!("Overflow happened."); },
    }

    let b_u128 = small_uint_checked_mul_func(a_u128.unwrap(), 2_u128);
    match b_u128
    {
        Some(b) => { println!("{} * 2 = {}", a_u128.unwrap(), b); },
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
        None => { println!("Overflow happened."); },
    }

    let b_usize = small_uint_checked_mul_func(a_usize.unwrap(), 2_usize);
    match b_usize
    {
        Some(b) => { println!("{} * 2 = {}", a_usize.unwrap(), b); },
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
        None => { println!("Overflow happened."); },
    }

    let b_shortunion = small_uint_checked_mul_func(a_shortunion.unwrap(), 2_u16.into_shortunion());
    match b_shortunion
    {
        Some(b) => { println!("{} * 2 = {}", a_shortunion.unwrap(), b); },
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
        None => { println!("Overflow happened."); },
    }

    let b_intunion = small_uint_checked_mul_func(a_intunion.unwrap(), 2_u32.into_intunion());
    match b_intunion
    {
        Some(b) => { println!("{} * 2 = {}", a_intunion.unwrap(), b); },
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
        None => { println!("Overflow happened."); },
    }

    let b_longunion = small_uint_checked_mul_func(a_longunion.unwrap(), 2_u64.into_longunion());
    match b_longunion
    {
        Some(b) => { println!("{} * 2 = {}", a_longunion.unwrap(), b); },
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
        None => { println!("Overflow happened."); },
    }

    let b_longerunion = small_uint_checked_mul_func(a_longerunion.unwrap(), 2_u128.into_longerunion());
    match b_longerunion
    {
        Some(b) => { println!("{} * 2 = {}", a_longerunion.unwrap(), b); },
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
        None => { println!("Overflow happened."); },
    }

    let b_sizeunion = small_uint_checked_mul_func(a_sizeunion.unwrap(), 2_usize.into_sizeunion());
    match b_sizeunion
    {
        Some(b) => { println!("{} * 2 = {}", a_sizeunion.unwrap(), b); },
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
    println!("small_uint_unchecked_add");
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
    println!("small_uint_saturating_mul");
    use cryptocol::number::SmallUInt;
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

    // Example for ShortUnion
    let a_shortunion = small_uint_saturating_mul_func((u16::MAX / 3).into_shortunion(), 2_u16.into_shortunion());
    println!("{} * 2 = {}", (u16::MAX / 3).into_shortunion(), a_shortunion);
    assert_eq!(a_shortunion.get(), 43690_u16);

    let b_shortunion = small_uint_saturating_mul_func(a_shortunion, 2_u16.into_shortunion());
    println!("{} * 2 = {}", a_shortunion, b_shortunion);
    assert_eq!(b_shortunion.get(), u16::MAX);

    // Example for IntUnion
    let a_intunion = small_uint_saturating_mul_func((u32::MAX / 3).into_intunion(), 2_u32.into_intunion());
    println!("{} * 2 = {}", (u32::MAX / 3).into_intunion(), a_intunion);
    assert_eq!(a_intunion.get(), 2863311530_u32);

    let b_intunion = small_uint_saturating_mul_func(a_intunion, 2_u32.into_intunion());
    println!("{} * 2 = {}", a_intunion, b_intunion);
    assert_eq!(b_intunion.get(), u32::MAX);

    // Example for LongUnion
    let a_longunion = small_uint_saturating_mul_func((u64::MAX / 3).into_longunion(), 2_u64.into_longunion());
    println!("{} * 2 = {}", (u64::MAX / 3).into_longunion(), a_longunion);
    assert_eq!(a_longunion.get(), 12297829382473034410_u64);

    let b_longunion = small_uint_saturating_mul_func(a_longunion, 2_u64.into_longunion());
    println!("{} * 2 = {}", a_longunion, b_longunion);
    assert_eq!(b_longunion.get(), u64::MAX);

    // Example for LongerUnion
    let a_longerunion = small_uint_saturating_mul_func((u128::MAX / 3).into_longerunion(), 2_u128.into_longerunion());
    println!("{} * 2 = {}", (u128::MAX / 3).into_longerunion(), a_longerunion);
    assert_eq!(a_longerunion.get(), 226854911280625642308916404954512140970_u128);

    let b_longerunion = small_uint_saturating_mul_func(a_longerunion, 2_u128.into_longerunion());
    println!("{} * 2 = {}", a_longerunion, b_longerunion);
    assert_eq!(b_longerunion.get(), u128::MAX);

    // Example for SizeUnion
    let a_sizeunion = small_uint_saturating_mul_func((usize::MAX / 3).into_sizeunion(), 2_usize.into_sizeunion());
    println!("{} * 2 = {}", (usize::MAX / 3).into_sizeunion(), a_sizeunion);
    assert_eq!(a_sizeunion.get(), 12297829382473034410_usize);

    let b_sizeunion = small_uint_saturating_mul_func(a_sizeunion, 2_usize.into_sizeunion());
    println!("{} * 2 = {}", a_sizeunion, b_sizeunion);
    assert_eq!(b_sizeunion.get(), usize::MAX);
    println!("--------------------------------------");
}

fn small_uint_saturating_mul_func<T: cryptocol::number::SmallUInt>(lhs: T, rhs: T) -> T
{
    lhs.saturating_mul(rhs)
}

fn small_uint_modular_mul()
{
    println!("small_uint_modular_mul");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = 90_u8;
    let b_u8 = a_u8.modular_mul(2, 200);
    println!("{} * 2 = {} (mod 200)", a_u8, b_u8);
    assert_eq!(b_u8, 180_u8);

    let c_u8 = b_u8.modular_mul(2, 200);
    println!("{} * 2 = {} (mod 200)", b_u8, c_u8);
    assert_eq!(c_u8, 160_u8);

    let d_u8 = 90_u8;
    let e_u8 = small_uint_modular_mul_func(d_u8, 2, 200);
    println!("{} * 2 = {} (mod 200)", d_u8, e_u8);
    assert_eq!(e_u8, 180_u8);

    let f_u8 = small_uint_modular_mul_func(e_u8, 2, 200);
    println!("{} * 2 = {} (mod 200)", e_u8, f_u8);
    assert_eq!(f_u8, 160_u8);

    // Example for u16
    let a_u16 = 9000_u16;
    let b_u16 = a_u16.modular_mul(2, 20000);
    println!("{} * 2 = {}", a_u16, b_u16);
    assert_eq!(b_u16, 18000_u16);

    let c_u16 = b_u16.modular_mul(2, 20000);
    println!("{} * 2 = {}", b_u16, c_u16);
    assert_eq!(c_u16, 16000_u16);

    let d_u16 = 9000_u16;
    let e_u16 = small_uint_modular_mul_func(d_u16, 2, 20000);
    println!("{} * 2 = {}", d_u16, e_u16);
    assert_eq!(e_u16, 18000_u16);

    let f_u16 = small_uint_modular_mul_func(e_u16, 2, 20000);
    println!("{} * 2 = {}", e_u16, f_u16);
    assert_eq!(f_u16, 16000_u16);

    // Example for u32
    let a_u32 = 9000000_u32;
    let b_u32 = a_u32.modular_mul(2, 20000000);
    println!("{} * 2 = {}", a_u32, b_u32);
    assert_eq!(b_u32, 18000000_u32);

    let c_u32 = b_u32.modular_mul(2, 20000000);
    println!("{} * 2 = {}", b_u32, c_u32);
    assert_eq!(c_u32, 16000000_u32);

    let d_u32 = 9000000_u32;
    let e_u32 = small_uint_modular_mul_func(d_u32, 2, 20000000);
    println!("{} * 2 = {}", d_u32, e_u32);
    assert_eq!(e_u32, 18000000_u32);

    let f_u32 = small_uint_modular_mul_func(e_u32, 2, 20000000);
    println!("{} * 2 = {}", e_u32, f_u32);
    assert_eq!(f_u32, 16000000_u32);

    // Example for u64
    let a_u64 = 900000000000_u64;
    let b_u64 = a_u64.modular_mul(2, 2000000000000);
    println!("{} * 2 = {}", a_u64, b_u64);
    assert_eq!(b_u64, 1800000000000_u64);

    let c_u64 = b_u64.modular_mul(2, 2000000000000);
    println!("{} * 2 = {}", b_u64, c_u64);
    assert_eq!(c_u64, 1600000000000_u64);

    let d_u64 = 900000000000_u64;
    let e_u64 = small_uint_modular_mul_func(d_u64, 2, 2000000000000);
    println!("{} * 2 = {}", d_u64, e_u64);
    assert_eq!(e_u64, 1800000000000_u64);

    let f_u64 = small_uint_modular_mul_func(e_u64, 2, 2000000000000);
    println!("{} * 2 = {}", e_u64, f_u64);
    assert_eq!(f_u64, 1600000000000_u64);

    // Example for u128
    let a_u128 = 90000000000000000000000_u128;
    let b_u128 = a_u128.modular_mul(2, 200000000000000000000000);
    println!("{} * 2 = {}", a_u128, b_u128);
    assert_eq!(b_u128, 180000000000000000000000_u128);

    let c_u128 = b_u128.modular_mul(2, 200000000000000000000000);
    println!("{} * 2 = {}", b_u128, c_u128);
    assert_eq!(c_u128, 160000000000000000000000_u128);

    let d_u128 = 90000000000000000000000_u128;
    let e_u128 = small_uint_modular_mul_func(d_u128, 2, 200000000000000000000000);
    println!("{} * 2 = {}", d_u128, e_u128);
    assert_eq!(e_u128, 180000000000000000000000_u128);

    let f_u128 = small_uint_modular_mul_func(e_u128, 2, 200000000000000000000000);
    println!("{} * 2 = {}", e_u128, f_u128);
    assert_eq!(f_u128, 160000000000000000000000_u128);

    // Example for usize
    let a_usize = 900000000000_usize;
    let b_usize = a_usize.modular_mul(2, 2000000000000);
    println!("{} * 2 = {}", a_usize, b_usize);
    assert_eq!(b_usize, 1800000000000_usize);

    let c_usize = b_usize.modular_mul(2, 2000000000000);
    println!("{} * 2 = {}", b_usize, c_usize);
    assert_eq!(c_usize, 1600000000000_usize);

    let d_usize = 900000000000_usize;
    let e_usize = small_uint_modular_mul_func(d_usize, 2, 2000000000000);
    println!("{} * 2 = {}", d_usize, e_usize);
    assert_eq!(e_usize, 1800000000000_usize);

    let f_usize = small_uint_modular_mul_func(e_usize, 2, 2000000000000);
    println!("{} * 2 = {}", e_usize, f_usize);
    assert_eq!(f_usize, 1600000000000_usize);

    // Example for ShortUnion
    let a_shortunion = 9000_u16.into_shortunion();
    let b_shortunion = a_shortunion.modular_mul(2_u16.into_shortunion(), 20000_u16.into_shortunion());
    println!("{} * 2 = {}", a_shortunion, b_shortunion);
    assert_eq!(b_shortunion.get(), 18000_u16);

    let c_shortunion = b_shortunion.modular_mul(2_u16.into_shortunion(), 20000_u16.into_shortunion());
    println!("{} * 2 = {}", b_shortunion, c_shortunion);
    assert_eq!(c_shortunion.get(), 16000_u16);

    let d_shortunion = 9000_u16.into_shortunion();
    let e_shortunion = small_uint_modular_mul_func(d_shortunion, 2_u16.into_shortunion(), 20000_u16.into_shortunion());
    println!("{} * 2 = {}", d_shortunion, e_shortunion);
    assert_eq!(e_shortunion.get(), 18000_u16);

    let f_shortunion = small_uint_modular_mul_func(e_shortunion, 2_u16.into_shortunion(), 20000_u16.into_shortunion());
    println!("{} * 2 = {}", e_shortunion, f_shortunion);
    assert_eq!(f_shortunion.get(), 16000_u16);

    // Example for IntUnion
    let a_intunion = 9000000_u32.into_intunion();
    let b_intunion = a_intunion.modular_mul(2_u32.into_intunion(), 20000000_u32.into_intunion());
    println!("{} * 2 = {}", a_intunion, b_intunion);
    assert_eq!(b_intunion.get(), 18000000_u32);

    let c_intunion = b_intunion.modular_mul(2_u32.into_intunion(), 20000000_u32.into_intunion());
    println!("{} * 2 = {}", b_intunion, c_intunion);
    assert_eq!(c_intunion.get(), 16000000_u32);

    let d_intunion = 9000000_u32.into_intunion();
    let e_intunion = small_uint_modular_mul_func(d_intunion, 2_u32.into_intunion(), 20000000_u32.into_intunion());
    println!("{} * 2 = {}", d_intunion, e_intunion);
    assert_eq!(e_intunion.get(), 18000000_u32);

    let f_intunion = small_uint_modular_mul_func(e_intunion, 2_u32.into_intunion(), 20000000_u32.into_intunion());
    println!("{} * 2 = {}", e_intunion, f_intunion);
    assert_eq!(f_intunion.get(), 16000000_u32);

    // Example for LongUnion
    let a_longunion = 900000000000_u64.into_longunion();
    let b_longunion = a_longunion.modular_mul(2_u64.into_longunion(), 2000000000000_u64.into_longunion());
    println!("{} * 2 = {}", a_longunion, b_longunion);
    assert_eq!(b_longunion.get(), 1800000000000_u64);

    let c_longunion = b_longunion.modular_mul(2_u64.into_longunion(), 2000000000000_u64.into_longunion());
    println!("{} * 2 = {}", b_longunion, c_longunion);
    assert_eq!(c_longunion.get(), 1600000000000_u64);

    let d_longunion = 900000000000_u64.into_longunion();
    let e_longunion = small_uint_modular_mul_func(d_longunion, 2_u64.into_longunion(), 2000000000000_u64.into_longunion());
    println!("{} * 2 = {}", d_longunion, e_longunion);
    assert_eq!(e_longunion.get(), 1800000000000_u64);

    let f_longunion = small_uint_modular_mul_func(e_longunion, 2_u64.into_longunion(), 2000000000000_u64.into_longunion());
    println!("{} * 2 = {}", e_longunion, f_longunion);
    assert_eq!(f_longunion.get(), 1600000000000_u64);

    // Example for LongerUnion
    let a_longerunion = 90000000000000000000000_u128.into_longerunion();
    let b_longerunion = a_longerunion.modular_mul(2_u128.into_longerunion(), 200000000000000000000000_u128.into_longerunion());
    println!("{} * 2 = {}", a_longerunion, b_longerunion);
    assert_eq!(b_longerunion.get(), 180000000000000000000000_u128);

    let c_longerunion = b_longerunion.modular_mul(2_u128.into_longerunion(), 200000000000000000000000_u128.into_longerunion());
    println!("{} * 2 = {}", b_longerunion, c_longerunion);
    assert_eq!(c_longerunion.get(), 160000000000000000000000_u128);

    let d_longerunion = 90000000000000000000000_u128.into_longerunion();
    let e_longerunion = small_uint_modular_mul_func(d_longerunion, 2_u128.into_longerunion(), 200000000000000000000000_u128.into_longerunion());
    println!("{} * 2 = {}", d_longerunion, e_longerunion);
    assert_eq!(e_longerunion.get(), 180000000000000000000000_u128);

    let f_longerunion = small_uint_modular_mul_func(e_longerunion, 2_u128.into_longerunion(), 200000000000000000000000_u128.into_longerunion());
    println!("{} * 2 = {}", e_longerunion, f_longerunion);
    assert_eq!(f_longerunion.get(), 160000000000000000000000_u128);

    // Example for SizeUnion
    let a_sizeunion = 900000000000_usize.into_sizeunion();
    let b_sizeunion = a_sizeunion.modular_mul(2_usize.into_sizeunion(), 2000000000000_usize.into_sizeunion());
    println!("{} * 2 = {}", a_sizeunion, b_sizeunion);
    assert_eq!(b_sizeunion.get(), 1800000000000_usize);

    let c_sizeunion = b_sizeunion.modular_mul(2_usize.into_sizeunion(), 2000000000000_usize.into_sizeunion());
    println!("{} * 2 = {}", b_sizeunion, c_sizeunion);
    assert_eq!(c_sizeunion.get(), 1600000000000_usize);

    let d_sizeunion = 900000000000_usize.into_sizeunion();
    let e_sizeunion = small_uint_modular_mul_func(d_sizeunion, 2_usize.into_sizeunion(), 2000000000000_usize.into_sizeunion());
    println!("{} * 2 = {}", d_sizeunion, e_sizeunion);
    assert_eq!(e_sizeunion.get(), 1800000000000_usize);

    let f_sizeunion = small_uint_modular_mul_func(e_sizeunion, 2_usize.into_sizeunion(), 2000000000000_usize.into_sizeunion());
    println!("{} * 2 = {}", e_sizeunion, f_sizeunion);
    assert_eq!(f_sizeunion.get(), 1600000000000_usize);
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
    println!("small_uint_wrapping_div");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = small_uint_wrapping_div_func(u8::MAX / 3, 2_u8);
    println!("{} / 2 = {}", u8::MAX / 3, a_u8);
    assert_eq!(a_u8, 42_u8);
    // It will panic.
    // let a_panic = small_uint_wrapping_div_func(u8::MAX / 3, 0_u8);

    // Example for u16
    let a_u16 = small_uint_wrapping_div_func(u16::MAX / 3, 2_u16);
    println!("{} / 2 = {}", u16::MAX / 3, a_u16);
    assert_eq!(a_u16, 10922_u16);
    // It will panic.
    // let a_panic = small_uint_wrapping_div_func(u16::MAX / 3, 0_u16);

    // Example for u32
    let a_u32 = small_uint_wrapping_div_func(u32::MAX / 3, 2_u32);
    println!("{} / 2 = {}", u32::MAX / 3, a_u32);
    assert_eq!(a_u32, 715827882_u32);
    // It will panic.
    // let a_panic = small_uint_wrapping_div_func(u32::MAX / 3, 0_u32);

    // Example for u64
    let a_u64 = small_uint_wrapping_div_func(u64::MAX / 3, 2_u64);
    println!("{} / 2 = {}", u64::MAX / 3, a_u64);
    assert_eq!(a_u64, 3074457345618258602_u64);
    // It will panic.
    // let a_panic = small_uint_wrapping_div_func(u64::MAX / 3, 0_u64);

    // Example for u128
    let a_u128 = small_uint_wrapping_div_func(u128::MAX / 3, 2_u128);
    println!("{} / 2 = {}", u128::MAX / 3, a_u128);
    assert_eq!(a_u128, 56713727820156410577229101238628035242_u128);
    // It will panic.
    // let a_panic = small_uint_wrapping_div_func(u128::MAX / 3, 0_u128);

    // Example for usize
    let a_usize = small_uint_wrapping_div_func(usize::MAX / 3, 2_usize);
    println!("{} / 2 = {}", usize::MAX / 3, a_usize);
    assert_eq!(a_usize, 3074457345618258602_usize);
    // It will panic.
    // let a_panic = small_uint_wrapping_div_func(usize::MAX / 3, 0_usize);

    // Example for ShortUnion
    let a_shortunion = small_uint_wrapping_div_func((u16::MAX / 3).into_shortunion(), 2_u16.into_shortunion());
    println!("{} / 2 = {}", (u16::MAX / 3).into_shortunion(), a_shortunion);
    assert_eq!(a_shortunion.get(), 10922_u16);
    // It will panic.
    // let a_panic = small_uint_wrapping_div_func((u16::MAX / 3).into_shortunion(), 0_u16.into_shortunion());

    // Example for IntUnion
    let a_intunion = small_uint_wrapping_div_func((u32::MAX / 3).into_intunion(), 2_u32.into_intunion());
    println!("{} / 2 = {}", (u32::MAX / 3).into_intunion(), a_intunion.into_intunion());
    assert_eq!(a_intunion.get(), 715827882_u32);
    // It will panic.
    // let a_panic = small_uint_wrapping_div_func((u32::MAX / 3).into_intunion(), 0_u32.into_intunion());

    // Example for LongUnion
    let a_longunion = small_uint_wrapping_div_func((u64::MAX / 3).into_longunion(), 2_u64.into_longunion());
    println!("{} / 2 = {}", (u64::MAX / 3).into_longunion(), a_longunion);
    assert_eq!(a_longunion.get(), 3074457345618258602_u64);
    // It will panic.
    // let a_panic = small_uint_wrapping_div_func(u64::MAX / 3).into_longunion(), 0_u64.into_longunion());

    // Example for LongerUnion
    let a_longerunion = small_uint_wrapping_div_func((u128::MAX / 3).into_longerunion(), 2_u128.into_longerunion());
    println!("{} / 2 = {}", (u128::MAX / 3).into_longerunion(), a_longerunion);
    assert_eq!(a_longerunion.get(), 56713727820156410577229101238628035242_u128);
    // It will panic.
    // let a_panic = small_uint_wrapping_div_func((u128::MAX / 3).into_longerunion(), 0_u128.into_longerunion());

    // Example for SizeUnion
    let a_sizeunion = small_uint_wrapping_div_func((usize::MAX / 3).into_sizeunion(), 2_usize.into_sizeunion());
    println!("{} / 2 = {}", (usize::MAX / 3).into_sizeunion(), a_sizeunion);
    assert_eq!(a_sizeunion.get(), 3074457345618258602_usize);
    // It will panic.
    // let a_panic = small_uint_wrapping_div_func((usize::MAX / 3).into_sizeunion(), 0_usize.into_sizeunion());
    println!("--------------------------------------");
}

fn small_uint_wrapping_div_func<T: cryptocol::number::SmallUInt>(lhs: T, rhs: T) -> T
{
    lhs.wrapping_div(rhs)
}

fn small_uint_overflowing_div()
{
    println!("small_uint_overflowing_div");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = small_uint_overflowing_div_func(u8::MAX / 3, 2_u8);
    println!("{} / 2 = {}\nOverflow = {}", u8::MAX / 3, a_u8.0, a_u8.1);
    assert_eq!(a_u8.0, 42_u8);
    assert_eq!(a_u8.1, false);
    // It will panic.
    // let a_panic = small_uint_overflowing_div_func(a_u8.0, 0_u8);

    // Example for u16
    let a_u16 = small_uint_overflowing_div_func(u16::MAX / 3, 2_u16);
    println!("{} / 2 = {}\nOverflow = {}", u16::MAX / 3, a_u16.0, a_u16.1);
    assert_eq!(a_u16.0, 10922_u16);
    assert_eq!(a_u16.1, false);
    // It will panic.
    // let a_panic = small_uint_overflowing_div_func(a_u16.0, 0_u16);

    // Example for u32
    let a_u32 = small_uint_overflowing_div_func(u32::MAX / 3, 2_u32);
    println!("{} / 2 = {}\nOverflow = {}", u32::MAX / 3, a_u32.0, a_u32.1);
    assert_eq!(a_u32.0, 715827882_u32);
    assert_eq!(a_u32.1, false);
    // It will panic.
    // let a_panic = small_uint_overflowing_div_func(a_u32.0, 0_u32);

    // Example for u64
    let a_u64 = small_uint_overflowing_div_func(u64::MAX / 3, 2_u64);
    println!("{} / 2 = {}\nOverflow = {}", u64::MAX / 3, a_u64.0, a_u64.1);
    assert_eq!(a_u64.0, 3074457345618258602_u64);
    assert_eq!(a_u64.1, false);
    // It will panic.
    // let a_panic = small_uint_overflowing_div_func(a_u64.0, 0_u64);

    // Example for u128
    let a_u128 = small_uint_overflowing_div_func(u128::MAX / 3, 2_u128);
    println!("{} / 2 = {}\nOverflow = {}", u128::MAX / 3, a_u128.0, a_u128.1);
    assert_eq!(a_u128.0, 56713727820156410577229101238628035242_u128);
    assert_eq!(a_u128.1, false);
    // It will panic.
    // let a_panic = small_uint_overflowing_div_func(a_u128.0, 0_u128);

    // Example for usize
    let a_usize = small_uint_overflowing_div_func(usize::MAX / 3, 2_usize);
    println!("{} / 2 = {}\nOverflow = {}", usize::MAX / 3, a_usize.0, a_usize.1);
    assert_eq!(a_usize.0, 3074457345618258602_usize);
    assert_eq!(a_usize.1, false);
    // It will panic.
    // let a_panic = small_uint_overflowing_div_func(a_usize.0, 0_usize);

    // Example for ShortUnion
    let (a_shortunion, overflow) = small_uint_overflowing_div_func((u16::MAX / 3).into_shortunion(), 2_u16.into_shortunion());
    println!("{} / 2 = {}\nOverflow = {}", (u16::MAX / 3).into_shortunion(), a_shortunion, overflow);
    assert_eq!(a_shortunion.get(), 10922_u16);
    assert_eq!(overflow, false);
    // It will panic.
    // let a_panic = small_uint_overflowing_div_func(a_shortunion, 0_u16.into_shortunion());

    // Example for IntUnion
    let (a_intunion, overflow) = small_uint_overflowing_div_func((u32::MAX / 3).into_intunion(), 2_u32.into_intunion());
    println!("{} / 2 = {}\nOverflow = {}", (u32::MAX / 3).into_intunion(), a_intunion, overflow);
    assert_eq!(a_intunion.get(), 715827882_u32);
    assert_eq!(overflow, false);
    // It will panic.
    // let a_panic = small_uint_overflowing_div_func(a_intunion, 0_u32.into_intunion());

    // Example for LongUnion
    let (a_longunion, overflow) = small_uint_overflowing_div_func((u64::MAX / 3).into_longunion(), 2_u64.into_longunion());
    println!("{} / 2 = {}\nOverflow = {}", (u64::MAX / 3).into_longunion(), a_longunion, overflow);
    assert_eq!(a_longunion.get(), 3074457345618258602_u64);
    assert_eq!(overflow, false);
    // It will panic.
    // let a_panic = small_uint_overflowing_div_func(a_longunion, 0_u64.into_longunion());

    // Example for LongerUnion
    let (a_longerunion, overflow) = small_uint_overflowing_div_func((u128::MAX / 3).into_longerunion(), 2_u128.into_longerunion());
    println!("{} / 2 = {}\nOverflow = {}", (u128::MAX / 3).into_longerunion(), a_longerunion, overflow);
    assert_eq!(a_longerunion.get(), 56713727820156410577229101238628035242_u128);
    assert_eq!(overflow, false);
    // It will panic.
    // let a_panic = small_uint_overflowing_div_func(a_longerunion, 0_u128.into_longerunion());

    // Example for SizeUnion
    let (a_sizeunion, overflow) = small_uint_overflowing_div_func((usize::MAX / 3).into_sizeunion(), 2_usize.into_sizeunion());
    println!("{} / 2 = {}\nOverflow = {}", (usize::MAX / 3).into_sizeunion(), a_sizeunion, overflow);
    assert_eq!(a_sizeunion.get(), 3074457345618258602_usize);
    assert_eq!(overflow, false);
    // It will panic.
    // let a_panic = small_uint_overflowing_div_func(a_sizeunion, 0_usize.into_sizeunion());
    println!("--------------------------------------");
}

fn small_uint_overflowing_div_func<T: cryptocol::number::SmallUInt>(lhs: T, rhs: T) -> (T, bool)
{
    lhs.overflowing_div(rhs)
}

fn small_uint_checked_div()
{
    println!("small_uint_checked_div");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = small_uint_checked_div_func(u8::MAX / 3, 2_u8);
    match a_u8
    {
        Some(a) => {
                println!("{} / 2 = {}", u8::MAX / 3, a);
                assert_eq!(a, 42_u8);
            },
        None => { println!("Divided by zero."); },
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
        None => { println!("Divided by zero."); },
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
        None => { println!("Divided by zero."); },
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
        None => { println!("Divided by zero."); },
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
        None => { println!("Divided by zero."); },
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
        None => { println!("Divided by zero."); },
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

    // Example for ShortUnion
    let a_shortunion = small_uint_checked_div_func((u16::MAX / 3).into_shortunion(), 2_u16.into_shortunion());
    match a_shortunion
    {
        Some(a) => {
                println!("{} / 2 = {}", (u16::MAX / 3).into_shortunion(), a);
                assert_eq!(a.get(), 10922_u16);
            },
        None => { println!("Divided by zero."); },
    }

    let b_shortunion = small_uint_checked_div_func((u16::MAX / 3).into_shortunion(), 0_u16.into_shortunion());
    match b_shortunion
    {
        Some(b) => { println!("{} / 2 = {}", (u16::MAX / 3).into_shortunion(), b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_shortunion, None);
            },
    }

    // Example for IntUnion
    let a_intunion = small_uint_checked_div_func((u32::MAX / 3).into_intunion(), 2_u32.into_intunion());
    match a_intunion
    {
        Some(a) => {
                println!("{} / 2 = {}", (u32::MAX / 3).into_intunion(), a);
                assert_eq!(a.get(), 715827882_u32);
            },
        None => { println!("Divided by zero."); },
    }

    let b_intunion = small_uint_checked_div_func((u32::MAX / 3).into_intunion(), 0_u32.into_intunion());
    match b_intunion
    {
        Some(b) => { println!("{} / 2 = {}", (u32::MAX / 3).into_intunion(), b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_intunion, None);
            },
    }

    // Example for LongUnion
    let a_longunion = small_uint_checked_div_func((u64::MAX / 3).into_longunion(), 2_u64.into_longunion());
    match a_longunion
    {
        Some(a) => {
                println!("{} / 2 = {}", (u64::MAX / 3).into_longunion(), a);
                assert_eq!(a.get(), 3074457345618258602_u64);
            },
        None => { println!("Divided by zero."); },
    }

    let b_longunion = small_uint_checked_div_func(u64::MAX / 3, 0_u64);
    match b_longunion
    {
        Some(b) => { println!("{} / 2 = {}", (u64::MAX / 3).into_longunion(), b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_longunion, None);
            },
    }

    // Example for LongerUnion
    let a_longerunion = small_uint_checked_div_func((u128::MAX / 3).into_longerunion(), 2_u128.into_longerunion());
    match a_longerunion
    {
        Some(a) => {
                println!("{} / 2 = {}", (u128::MAX / 3).into_longerunion(), a);
                assert_eq!(a.get(), 56713727820156410577229101238628035242_u128);
            },
        None => { println!("Divided by zero."); },
    }

    let b_longerunion = small_uint_checked_div_func((u128::MAX / 3).into_longerunion(), 0_u128.into_longerunion());
    match b_longerunion
    {
        Some(b) => { println!("{} / 2 = {}", (u128::MAX / 3).into_longerunion(), b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_longerunion, None);
            },
    }

    // Example for SizeUnion
    let a_sizeunion = small_uint_checked_div_func((usize::MAX / 3).into_sizeunion(), 2_usize.into_sizeunion());
    match a_sizeunion
    {
        Some(a) => {
                println!("{} / 2 = {}", (usize::MAX / 3).into_sizeunion(), a);
                assert_eq!(a.get(), 3074457345618258602_usize);
            },
        None => { println!("Divided by zero."); },
    }

    let b_sizeunion = small_uint_checked_div_func((usize::MAX / 3).into_sizeunion(), 0_usize.into_sizeunion());
    match b_sizeunion
    {
        Some(b) => { println!("{} / 2 = {}", (usize::MAX / 3).into_sizeunion(), b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_sizeunion, None);
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
    println!("small_uint_unchecked_div");
    use cryptocol::number::SmallUInt;
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

    // Example for ShortUnion
    let a_shortunion = small_uint_unchecked_div_func((u16::MAX / 3).into_shortunion(), 2_u16.into_shortunion());
    println!("{} / 2 = {}", (u16::MAX / 3).into_shortunion(), a_shortunion);
    assert_eq!(a_shortunion.get(), 10922_u16);
    // It will panic.
    // let b_shortunion = small_uint_unchecked_div_func((u16::MAX / 3).into_shortunion(), 0_u16.into_shortunion());

    // Example for InttUnion
    let a_intunion = small_uint_unchecked_div_func((u32::MAX / 3).into_intunion(), 2_u32.into_intunion());
    println!("{} / 2 = {}", (u32::MAX / 3).into_intunion(), a_intunion);
    assert_eq!(a_intunion.get(), 715827882_u32);
    // It will panic.
    // let b_intunion = small_uint_unchecked_div_func((u32::MAX / 3).into_intunion(), 0_u32.into_intunion());

    // Example for LongUnion
    let a_longunion = small_uint_unchecked_div_func((u64::MAX / 3).into_longunion(), 2_u64.into_longunion());
    println!("{} / 2 = {}", (u64::MAX / 3).into_longunion(), a_longunion);
    assert_eq!(a_longunion.get(), 3074457345618258602_u64);
    // It will panic.
    // let b_longunion = small_uint_unchecked_div_func((u64::MAX / 3).into_longunion(), 0_u64.into_longunion());

    // Example for LongerUnion
    let a_longerunion = small_uint_unchecked_div_func((u128::MAX / 3).into_longerunion(), 2_u128.into_longerunion());
    println!("{} / 2 = {}", (u128::MAX / 3).into_longerunion(), a_longerunion);
    assert_eq!(a_longerunion.get(), 56713727820156410577229101238628035242_u128);
    // It will panic.
    // let b_longerunion = small_uint_unchecked_div_func((u128::MAX / 3).into_longerunion(), 0_u128.into_longerunion());

    // Example for SizeUnion
    let a_sizeunion = small_uint_unchecked_div_func((usize::MAX / 3).into_sizeunion(), 2_usize.into_sizeunion());
    println!("{} / 2 = {}", (usize::MAX / 3).into_sizeunion(), a_sizeunion);
    assert_eq!(a_sizeunion.get(), 3074457345618258602_usize);
    // It will panic.
    // let b_sizeunion = small_uint_unchecked_div_func((usize::MAX / 3).into_sizeunion(), 0_usize.into_sizeunion());
    println!("--------------------------------------");
}

fn small_uint_unchecked_div_func<T: cryptocol::number::SmallUInt>(lhs: T, rhs: T) -> T
{
    lhs.unchecked_div(rhs)
}

fn small_uint_saturating_div()
{
    println!("small_uint_saturating_div");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = small_uint_saturating_div_func(u8::MAX / 3, 2_u8);
    println!("{} / 2 = {}", u8::MAX / 3, a_u8);
    assert_eq!(a_u8, 42_u8);
    // It will panic.
    // let a_panic = small_uint_saturating_div_func(u8::MAX / 3, 0_u8);

    // Example for u16
    let a_u16 = small_uint_saturating_div_func(u16::MAX / 3, 2_u16);
    println!("{} / 2 = {}", u16::MAX / 3, a_u16);
    assert_eq!(a_u16, 10922_u16);
    // It will panic.
    // let a_panic = small_uint_saturating_div_func(u16::MAX / 3, 0_u16);

    // Example for u32
    let a_u32 = small_uint_saturating_div_func(u32::MAX / 3, 2_u32);
    println!("{} / 2 = {}", u32::MAX / 3, a_u32);
    assert_eq!(a_u32, 715827882_u32);
    // It will panic.
    // let a_panic = small_uint_saturating_div_func(u32::MAX / 3, 0_u32);

    // Example for u64
    let a_u64 = small_uint_saturating_div_func(u64::MAX / 3, 2_u64);
    println!("{} / 2 = {}", u64::MAX / 3, a_u64);
    assert_eq!(a_u64, 3074457345618258602_u64);
    // It will panic.
    // let a_panic = small_uint_saturating_div_func(u64::MAX / 3, 0_u64);

    // Example for u128
    let a_u128 = small_uint_saturating_div_func(u128::MAX / 3, 2_u128);
    println!("{} / 2 = {}", u128::MAX / 3, a_u128);
    assert_eq!(a_u128, 56713727820156410577229101238628035242_u128);
    // It will panic.
    // let a_panic = small_uint_saturating_div_func(u128::MAX / 3, 0_u128);

    // Example for usize
    let a_usize = small_uint_saturating_div_func(usize::MAX / 3, 2_usize);
    println!("{} / 2 = {}", usize::MAX / 3, a_usize);
    assert_eq!(a_usize, 3074457345618258602_usize);
    // It will panic.
    // let a_panic = small_uint_saturating_div_func(usize::MAX / 3, 0_usize);

    // Example for ShortUnion
    let a_shortunion = small_uint_saturating_div_func((u16::MAX / 3).into_shortunion(), 2_u16.into_shortunion());
    println!("{} / 2 = {}", (u16::MAX / 3).into_shortunion(), a_shortunion);
    assert_eq!(a_shortunion.get(), 10922_u16);
    // It will panic.
    // let a_panic = small_uint_saturating_div_func((u16::MAX / 3).into_shortunion(), 0_u16.into_shortunion());

    // Example for IntUnion
    let a_intunion = small_uint_saturating_div_func((u32::MAX / 3).into_intunion(), 2_u32.into_intunion());
    println!("{} / 2 = {}", (u32::MAX / 3).into_intunion(), a_intunion);
    assert_eq!(a_intunion.get(), 715827882_u32);
    // It will panic.
    // let a_panic = small_uint_saturating_div_func((u32::MAX / 3).into_intunion(), 0_u32.into_intunion());

    // Example for LongUnion
    let a_longunion = small_uint_saturating_div_func((u64::MAX / 3).into_longunion(), 2_u64.into_longunion());
    println!("{} / 2 = {}", (u64::MAX / 3).into_longunion(), a_longunion);
    assert_eq!(a_longunion.get(), 3074457345618258602_u64);
    // It will panic.
    // let a_panic = small_uint_saturating_div_func((u64::MAX / 3).into_longunion(), 0_u64.into_longunion());

    // Example for LongerUnion
    let a_longerunion = small_uint_saturating_div_func((u128::MAX / 3).into_longerunion(), 2_u128.into_longerunion());
    println!("{} / 2 = {}", (u128::MAX / 3).into_longerunion(), a_longerunion);
    assert_eq!(a_longerunion.get(), 56713727820156410577229101238628035242_u128);
    // It will panic.
    // let a_panic = small_uint_saturating_div_func((u128::MAX / 3).into_longerunion(), 0_u128.into_longerunion());

    // Example for SizeUnion
    let a_sizeunion = small_uint_saturating_div_func((usize::MAX / 3).into_sizeunion(), 2_usize.into_sizeunion());
    println!("{} / 2 = {}", (usize::MAX / 3).into_sizeunion(), a_sizeunion);
    assert_eq!(a_sizeunion.get(), 3074457345618258602_usize);
    // It will panic.
    // let a_panic = small_uint_saturating_div_func((usize::MAX / 3).into_sizeunion(), 0_usize.into_sizeunion());
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
    println!("small_uint_wrapping_rem");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = small_uint_wrapping_rem_func(u8::MAX / 3, 3_u8);
    println!("{} % 3 = {}", u8::MAX / 3, a_u8);
    assert_eq!(a_u8, 1_u8);
    // It will panic.
    // let a_panic = small_uint_wrapping_rem_func(u8::MAX / 3, 0_u8);

    // Example for u16
    let a_u16 = small_uint_wrapping_rem_func(u16::MAX / 3, 3_u16);
    println!("{} % 3 = {}", u16::MAX / 3, a_u16);
    assert_eq!(a_u16, 2_u16);
    // It will panic.
    // let a_panic = small_uint_wrapping_rem_func(u16::MAX / 3, 0_u16);

    // Example for u32
    let a_u32 = small_uint_wrapping_rem_func(u32::MAX / 3, 3_u32);
    println!("{} % 3 = {}", u32::MAX / 3, a_u32);
    assert_eq!(a_u32, 1_u32);
    // It will panic.
    // let a_panic = small_uint_wrapping_rem_func(u32::MAX / 3, 0_u32);

    // Example for u64
    let a_u64 = small_uint_wrapping_rem_func(u64::MAX / 3, 3_u64);
    println!("{} % 3 = {}", u64::MAX / 3, a_u64);
    assert_eq!(a_u64, 2_u64);
    // It will panic.
    // let a_panic = small_uint_wrapping_rem_func(u64::MAX / 3, 0_u64);

    // Example for u128
    let a_u128 = small_uint_wrapping_rem_func(u128::MAX / 3, 3_u128);
    println!("{} % 3 = {}", u128::MAX / 3, a_u128);
    assert_eq!(a_u128, 1_u128);
    // It will panic.
    // let a_panic = small_uint_wrapping_rem_func(u128::MAX / 3, 0_u128);

    // Example for usize
    let a_usize = small_uint_wrapping_rem_func(usize::MAX / 3, 3_usize);
    println!("{} % 3 = {}", usize::MAX / 3, a_usize);
    assert_eq!(a_usize, 2_usize);
    // It will panic.
    // let a_panic = small_uint_wrapping_rem_func(usize::MAX / 3, 0_usize);

    // Example for ShortUnion
    let a_shortunion = small_uint_wrapping_rem_func((u16::MAX / 3).into_shortunion(), 3_u16.into_shortunion());
    println!("{} % 3 = {}", (u16::MAX / 3).into_shortunion(), a_shortunion);
    assert_eq!(a_shortunion.get(), 2_u16);
    // It will panic.
    // let a_panic = small_uint_wrapping_rem_func((u16::MAX / 3).into_shortunion(), 0_u16.into_shortunion());

    // Example for IntUnion
    let a_intunion = small_uint_wrapping_rem_func((u32::MAX / 3).into_intunion(), 3_u32.into_intunion());
    println!("{} % 3 = {}", (u32::MAX / 3).into_intunion(), a_intunion);
    assert_eq!(a_intunion.get(), 1_u32);
    // It will panic.
    // let a_panic = small_uint_wrapping_rem_func((u32::MAX / 3).into_intunion(), 0_u32.into_intunion());

    // Example for LongUnion
    let a_longunion = small_uint_wrapping_rem_func((u64::MAX / 3).into_longunion(), 3_u64.into_longunion());
    println!("{} % 3 = {}", (u64::MAX / 3).into_longunion(), a_longunion);
    assert_eq!(a_longunion.get(), 2_u64);
    // It will panic.
    // let a_panic = small_uint_wrapping_rem_func((u64::MAX / 3).into_longunion(), 0_u64.into_longunion());

    // Example for LongerUnion
    let a_longerunion = small_uint_wrapping_rem_func((u128::MAX / 3).into_longerunion(), 3_u128.into_longerunion());
    println!("{} % 3 = {}", (u128::MAX / 3).into_longerunion(), a_longerunion);
    assert_eq!(a_longerunion, 1_u128.into_longerunion());
    // It will panic.
    // let a_panic = small_uint_wrapping_rem_func((u128::MAX / 3).into_longerunion(), 0_u128.into_longerunion());

    // Example for SizeUnion
    let a_sizeunion = small_uint_wrapping_rem_func((usize::MAX / 3).into_sizeunion(), 3_usize.into_sizeunion());
    println!("{} % 3 = {}", (usize::MAX / 3).into_sizeunion(), a_sizeunion);
    assert_eq!(a_sizeunion, 2_usize.into_sizeunion());
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
    println!("small_uint_overflowing_rem");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = small_uint_overflowing_rem_func(u8::MAX / 3, 3_u8);
    println!("{} % 3 = {}\nOverflow = {}", u8::MAX / 3, a_u8.0, a_u8.1);
    assert_eq!(a_u8.0, 1_u8);
    assert_eq!(a_u8.1, false);
    // It will panic.
    // let a_panic = small_uint_overflowing_rem_func(a_u8.0, 0_u8);

    // Example for u16
    let a_u16 = small_uint_overflowing_rem_func(u16::MAX / 3, 3_u16);
    println!("{} % 3 = {}\nOverflow = {}", u16::MAX / 3, a_u16.0, a_u16.1);
    assert_eq!(a_u16.0, 2_u16);
    assert_eq!(a_u16.1, false);
    // It will panic.
    // let a_panic = small_uint_overflowing_rem_func(a_u16.0, 0_u16);
 
    // Example for u32
    let a_u32 = small_uint_overflowing_rem_func(u32::MAX / 3, 3_u32);
    println!("{} % 3 = {}\nOverflow = {}", u32::MAX / 3, a_u32.0, a_u32.1);
    assert_eq!(a_u32.0, 1_u32);
    assert_eq!(a_u32.1, false);
    // It will panic.
    // let a_panic = small_uint_overflowing_rem_func(a_u32.0, 0_u32);
 
    // Example for u64
    let a_u64 = small_uint_overflowing_rem_func(u64::MAX / 3, 3_u64);
    println!("{} % 3 = {}\nOverflow = {}", u64::MAX / 3, a_u64.0, a_u64.1);
    assert_eq!(a_u64.0, 2_u64);
    assert_eq!(a_u64.1, false);
    // It will panic.
    // let a_panic = small_uint_overflowing_rem_func(a_u62.0, 0_u62);
 
    // Example for u128
    let a_u128 = small_uint_overflowing_rem_func(u128::MAX / 3, 3_u128);
    println!("{} % 3 = {}\nOverflow = {}", u128::MAX / 3, a_u128.0, a_u128.1);
    assert_eq!(a_u128.0, 1_u128);
    assert_eq!(a_u128.1, false);
    // It will panic.
    // let a_panic = small_uint_overflowing_rem_func(a_u128.0, 0_u128);
 
    // Example for usize
    let a_usize = small_uint_overflowing_rem_func(usize::MAX / 3, 3_usize);
    println!("{} % 3 = {}\nOverflow = {}", usize::MAX / 3, a_usize.0, a_usize.1);
    assert_eq!(a_usize.0, 2_usize);
    assert_eq!(a_usize.1, false);
    // It will panic.
    // let a_panic = small_uint_overflowing_rem_func(a_usize.0, 0_usize);

    // Example for ShortUnion
    let (a_shortunion, overflow) = small_uint_overflowing_rem_func((u16::MAX / 3).into_shortunion(), 3_u16.into_shortunion());
    println!("{} % 3 = {}\nOverflow = {}", (u16::MAX / 3).into_shortunion(), a_shortunion.into_shortunion(), overflow);
    assert_eq!(a_shortunion.get(), 2_u16);
    assert_eq!(overflow, false);
    // It will panic.
    // let a_panic = small_uint_overflowing_rem_func(a_shortunion, 0_u16.into_shortunion());
 
    // Example for IntUnion
    let (a_intunion, overflow) = small_uint_overflowing_rem_func((u32::MAX / 3).into_intunion(), 3_u32.into_intunion());
    println!("{} % 3 = {}\nOverflow = {}", (u32::MAX / 3).into_intunion(), a_intunion, overflow);
    assert_eq!(a_intunion.get(), 1_u32);
    assert_eq!(overflow, false);
    // It will panic.
    // let a_panic = small_uint_overflowing_rem_func(a_intunion, 0_u32.into_intunion());
 
    // Example for LongUnion
    let (a_longunion, overflow) = small_uint_overflowing_rem_func((u64::MAX / 3).into_longunion(), 3_u64.into_longunion());
    println!("{} % 3 = {}\nOverflow = {}", (u64::MAX / 3).into_longunion(), a_longunion, overflow);
    assert_eq!(a_longunion.get(), 2_u64);
    assert_eq!(overflow, false);
    // It will panic.
    // let a_panic = small_uint_overflowing_rem_func(a_longunion, 0_u62.into_longunion());
 
    // Example for LongerUnion
    let (a_longerunion, overflow) = small_uint_overflowing_rem_func((u128::MAX / 3).into_longerunion(), 3_u128.into_longerunion());
    println!("{} % 3 = {}\nOverflow = {}", (u128::MAX / 3).into_longerunion(), a_longerunion, overflow);
    assert_eq!(a_longerunion.get(), 1_u128);
    assert_eq!(overflow, false);
    // It will panic.
    // let a_panic = small_uint_overflowing_rem_func(a_longerunion, 0_u128.into_longerunion());
 
    // Example for SizeUnion
    let (a_sizeunion, overflow) = small_uint_overflowing_rem_func((usize::MAX / 3).into_sizeunion(), 3_usize.into_sizeunion());
    println!("{} % 3 = {}\nOverflow = {}", (usize::MAX / 3).into_sizeunion(), a_sizeunion, overflow);
    assert_eq!(a_sizeunion.get(), 2_usize);
    assert_eq!(overflow, false);
    // It will panic.
    // let a_panic = small_uint_overflowing_rem_func(a_sizeunion, 0_usize.into_sizeunion());
    println!("--------------------------------------");
}

fn small_uint_overflowing_rem_func<T: cryptocol::number::SmallUInt>(lhs: T, rhs: T) -> (T, bool)
{
    lhs.overflowing_rem(rhs)
}

fn small_uint_checked_rem()
{
    println!("small_uint_checked_rem");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = small_uint_checked_rem_func(u8::MAX / 3, 3_u8);
    match a_u8
    {
        Some(a) => {
                println!("{} % 3 = {}", u8::MAX / 3, a);
                assert_eq!(a, 1_u8);
            },
        None => { println!("Divided by zero."); },
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
        None => { println!("Divided by zero."); },
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
        None => { println!("Divided by zero."); },
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
        None => { println!("Divided by zero."); },
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
        None => { println!("Divided by zero."); },
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
        None => { println!("Divided by zero."); },
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

    // Example for ShortUnion
    let a_shortunion = small_uint_checked_rem_func((u16::MAX / 3).into_shortunion(), 3_u16.into_shortunion());
    match a_shortunion
    {
        Some(a) => {
                println!("{} % 3 = {}", (u16::MAX / 3).into_shortunion(), a);
                assert_eq!(a.get(), 2_u16);
            },
        None => { println!("Divided by zero."); },
    }

    let b_shortunion = small_uint_checked_rem_func((u16::MAX / 3).into_shortunion(), 0_u16.into_shortunion());
    match b_shortunion
    {
        Some(b) => { println!("{} % 3 = {}", (u16::MAX / 3).into_shortunion(), b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_shortunion, None);
            },
    }

    // Example for IntUnion
    let a_intunion = small_uint_checked_rem_func((u32::MAX / 3).into_intunion(), 3_u32.into_intunion());
    match a_intunion
    {
        Some(a) => {
                println!("{} % 3 = {}", (u32::MAX / 3).into_intunion(), a);
                assert_eq!(a.get(), 1_u32);
            },
        None => { println!("Divided by zero."); },
    }

    let b_intunion = small_uint_checked_rem_func((u32::MAX / 3).into_intunion(), 0_u32.into_intunion());
    match b_intunion
    {
        Some(b) => { println!("{} % 3 = {}", (u32::MAX / 3).into_intunion(), b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_intunion, None);
            },
    }

    // Example for LongUnion
    let a_longunion = small_uint_checked_rem_func((u64::MAX / 3).into_longunion(), 3_u64.into_longunion());
    match a_longunion
    {
        Some(a) => {
                println!("{} % 3 = {}", (u64::MAX / 3).into_longunion(), a);
                assert_eq!(a.get(), 2_u64);
            },
        None => { println!("Divided by zero."); },
    }

    let b_longunion = small_uint_checked_rem_func(u64::MAX / 3, 0_u64);
    match b_longunion
    {
        Some(b) => { println!("{} % 3 = {}", u64::MAX / 3, b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_longunion, None);
            },
    }

    // Example for LongerUnion
    let a_longerunion = small_uint_checked_rem_func((u128::MAX / 3).into_longerunion(), 3_u128.into_longerunion());
    match a_longerunion
    {
        Some(a) => {
                println!("{} % 3 = {}", (u128::MAX / 3).into_longerunion(), a);
                assert_eq!(a.get(), 1_u128);
            },
        None => { println!("Divided by zero."); },
    }

    let b_longerunion = small_uint_checked_rem_func(u128::MAX / 3, 0_u128);
    match b_longerunion
    {
        Some(b) => { println!("{} % 3 = {}", (u128::MAX / 3).into_longerunion(), b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_longerunion, None);
            },
    }

    // Example for SizeUnion
    let a_sizeunion = small_uint_checked_rem_func((usize::MAX / 3).into_sizeunion(), 3_usize.into_sizeunion());
    match a_sizeunion
    {
        Some(a) => {
                println!("{} % 3 = {}", (usize::MAX / 3).into_sizeunion(), a);
                assert_eq!(a.get(), 2_usize);
            },
        None => { println!("Divided by zero."); },
    }

    let b_sizeunion = small_uint_checked_rem_func((usize::MAX / 3).into_sizeunion(), 0_usize.into_sizeunion());
    match b_sizeunion
    {
        Some(b) => { println!("{} % 3 = {}", (usize::MAX / 3).into_sizeunion(), b); },
        None => {
                println!("Divided by zero.");
                assert_eq!(b_sizeunion, None);
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
    println!("small_uint_unchecked_rem");
    use cryptocol::number::SmallUInt;
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

    // Example for usize
    let a_usize = small_uint_unchecked_rem_func(usize::MAX / 3, 3_usize);
    println!("{} % 3 = {}", usize::MAX / 3, a_usize);
    assert_eq!(a_usize, 2_usize);
    // It will panic.
    // let b_usize = small_uint_unchecked_rem_func(usize::MAX / 3, 0_usize);

    // Example for ShortUnion
    let a_shortunion = small_uint_unchecked_rem_func((u16::MAX / 3).into_shortunion(), 3_u16.into_shortunion());
    println!("{} % 3 = {}", (u16::MAX / 3).into_shortunion(), a_shortunion);
    assert_eq!(a_shortunion.get(), 2_u16);
    // It will panic.
    // let b_shortunion = small_uint_unchecked_rem_func((u16::MAX / 3).into_shortunion(), 0_u16.into_shortunion());

    // Example for IntUnion
    let a_inttunion = small_uint_unchecked_rem_func((u32::MAX / 3).into_intunion(), 3_u32.into_intunion());
    println!("{} % 3 = {}", (u32::MAX / 3).into_intunion(), a_inttunion);
    assert_eq!(a_inttunion.get(), 1_u32);
    // It will panic.
    // let b_inttunion = small_uint_unchecked_rem_func((u32::MAX / 3).into_intunion(), 0_u32.into_intunion());

    // Example for LongUnion
    let a_longunion = small_uint_unchecked_rem_func((u64::MAX / 3).into_longunion(), 3_u64.into_longunion());
    println!("{} % 3 = {}", (u64::MAX / 3).into_longunion(), a_longunion);
    assert_eq!(a_longunion.get(), 2_u64);
    // It will panic.
    // let b_longunion = small_uint_unchecked_rem_func((u64::MAX / 3).into_longunion(), 0_u64.into_longunion());

    // Example for LongerUnion
    let a_longerunion = small_uint_unchecked_rem_func((u128::MAX / 3).into_longerunion(), 3_u128.into_longerunion());
    println!("{} % 3 = {}", (u128::MAX / 3).into_longerunion(), a_longerunion);
    assert_eq!(a_longerunion.get(), 1_u128);
    // It will panic.
    // let b_longerunion = small_uint_unchecked_rem_func((u128::MAX / 3).into_longerunion(), 0_u128.into_longerunion());

    // Example for SizeUnion
    let a_sizeunion = small_uint_unchecked_rem_func((usize::MAX / 3).into_sizeunion(), 3_usize.into_sizeunion());
    println!("{} % 3 = {}", (usize::MAX / 3).into_sizeunion(), a_sizeunion);
    assert_eq!(a_sizeunion.get(), 2_usize);
    // It will panic.
    // let b_sizeunion = small_uint_unchecked_rem_func((usize::MAX / 3).into_sizeunion(), 0_usize.into_sizeunion());
    println!("--------------------------------------");
}

fn small_uint_unchecked_rem_func<T: cryptocol::number::SmallUInt>(lhs: T, rhs: T) -> T
{
    lhs.unchecked_rem(rhs)
}

fn small_uint_neg_main()
{
    small_uint_wrapping_neg();
    small_uint_overflowing_neg();
    // small_uint_checked_neg();
    // small_uint_unchecked_neg();
}

fn small_uint_wrapping_neg()
{
    println!("small_uint_wrapping_neg");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = 123_u8;
    let b_u8 = small_uint_wrapping_neg_func(a_u8);
    println!("-{} = {}", a_u8, b_u8);
    assert_eq!(b_u8, 133_u8);
    
    // Example for u16
    let a_u16 = 12345_u16;
    let b_u16 = small_uint_wrapping_neg_func(a_u16);
    println!("-{} = {}", a_u16, b_u16);
    assert_eq!(b_u16, 53191_u16);
    
    // Example for u32
    let a_u32 = 1234567890_u32;
    let b_u32 = small_uint_wrapping_neg_func(a_u32);
    println!("-{} = {}", a_u32, b_u32);
    assert_eq!(b_u32, 3060399406_u32);
    
    // Example for u64
    let a_u64 = 12345678901234567890_u64;
    let b_u64 = small_uint_wrapping_neg_func(a_u64);
    println!("-{} = {}", a_u64, b_u64);
    assert_eq!(b_u64, 6101065172474983726_u64);
    
    // Example for u128
    let a_u128 = 123456789012345678901234567890123456789_u128;
    let b_u128 = small_uint_wrapping_neg_func(a_u128);
    println!("-{} = {}", a_u128, b_u128);
    assert_eq!(b_u128, 216825577908592784562140039541644754667_u128);
    
    // Example for usize
    let a_usize = 1234567890123456789_usize;
    let b_usize = small_uint_wrapping_neg_func(a_usize);
    println!("-{} = {}", a_usize, b_usize);
    assert_eq!(b_usize, 17212176183586094827_usize);
    
    // Example for ShortUnion
    let a_shortunion = 12345_u16.into_shortunion();
    let b_shortunion = small_uint_wrapping_neg_func(a_shortunion);
    println!("-{} = {}", a_shortunion, b_shortunion);
    assert_eq!(b_shortunion.get(), 53191_u16);
    
    // Example for IntUnion
    let a_intunion = 1234567890_u32.into_intunion();
    let b_intunion = small_uint_wrapping_neg_func(a_intunion);
    println!("-{} = {}", a_intunion, b_intunion);
    assert_eq!(b_intunion.get(), 3060399406_u32);
    
    // Example for LongUnion
    let a_longunion = 12345678901234567890_u64.into_longunion();
    let b_longunion = small_uint_wrapping_neg_func(a_longunion);
    println!("-{} = {}", a_longunion, b_longunion);
    assert_eq!(b_longunion.get(), 6101065172474983726_u64);
    
    // Example for LongerUnion
    let a_longerunion = 123456789012345678901234567890123456789_u128.into_longerunion();
    let b_longerunion = small_uint_wrapping_neg_func(a_longerunion);
    println!("-{} = {}", a_longerunion, b_longerunion);
    assert_eq!(b_longerunion.get(), 216825577908592784562140039541644754667_u128);
    
    // Example for SizeUnion
    let a_sizeunion = 1234567890123456789_usize.into_sizeunion();
    let b_sizeunion = small_uint_wrapping_neg_func(a_sizeunion);
    println!("-{} = {}", a_sizeunion, a_sizeunion);
    assert_eq!(b_sizeunion.get(), 17212176183586094827_usize);
    println!("--------------------------------------");

}

fn small_uint_wrapping_neg_func<T: cryptocol::number::SmallUInt>(me: T) -> T
{
    me.wrapping_neg()
}

fn small_uint_overflowing_neg()
{
    println!("small_uint_overflowing_neg");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = 0_u8;
    let (b_u8, overflow) = small_uint_overflowing_neg_func(a_u8);
    println!("-{} = {}, {}", a_u8, b_u8, overflow);
    assert_eq!(b_u8, 0_u8);
    assert_eq!(overflow, false);

    let c_u8 = 123_u8;
    let (d_u8, overflow) = small_uint_overflowing_neg_func(c_u8);
    println!("-{} = {}, {}", c_u8, d_u8, overflow);
    assert_eq!(d_u8, 133_u8);
    assert_eq!(overflow, true);
    
    // Example for u16
    let a_u16 = 0_u16;
    let (b_u16, overflow) = small_uint_overflowing_neg_func(a_u16);
    println!("-{} = {}, {}", a_u16, b_u16, overflow);
    assert_eq!(b_u16, 0_u16);
    assert_eq!(overflow, false);

    let c_u16 = 12345_u16;
    let (d_u16, overflow) = small_uint_overflowing_neg_func(c_u16);
    println!("-{} = {}, {}", c_u16, d_u16, overflow);
    assert_eq!(d_u16, 53191_u16);
    assert_eq!(overflow, true);
    
    // Example for u32
    let a_u32 = 0_u32;
    let (b_u32, overflow) = small_uint_overflowing_neg_func(a_u32);
    println!("-{} = {}, {}", a_u32, b_u32, overflow);
    assert_eq!(b_u32, 0_u32);
    assert_eq!(overflow, false);

    let c_u32 = 1234567890_u32;
    let (d_u32, overflow) = small_uint_overflowing_neg_func(c_u32);
    println!("-{} = {}, {}", c_u32, d_u32, overflow);
    assert_eq!(d_u32, 3060399406_u32);
    assert_eq!(overflow, true);
    
    // Example for u64
    let a_u64 = 0_u64;
    let (b_u64, overflow) = small_uint_overflowing_neg_func(a_u64);
    println!("-{} = {}, {}", a_u64, b_u64, overflow);
    assert_eq!(b_u64, 0_u64);
    assert_eq!(overflow, false);

    let c_u64 = 12345678901234567890_u64;
    let (d_u64, overflow) = small_uint_overflowing_neg_func(c_u64);
    println!("-{} = {}, {}", c_u64, d_u64, overflow);
    assert_eq!(d_u64, 6101065172474983726_u64);
    assert_eq!(overflow, true);
    
    // Example for u128
    let a_u128 = 0_u128;
    let (b_128, overflow) = small_uint_overflowing_neg_func(a_u128);
    println!("-{} = {}, {}", a_u128, b_128, overflow);
    assert_eq!(b_128, 0_u128);
    assert_eq!(overflow, false);

    let c_u128 = 123456789012345678901234567890123456789_u128;
    let (d_u128, overflow) = small_uint_overflowing_neg_func(c_u128);
    println!("-{} = {}, {}", c_u128, d_u128, overflow);
    assert_eq!(d_u128, 216825577908592784562140039541644754667_u128);
    assert_eq!(overflow, true);
    
    // Example for usize for 64-bit CPU
    let a_usize = 0_usize;
    let (b_usize, overflow) = small_uint_overflowing_neg_func(a_usize);
    println!("-{} = {}, {}", a_usize, b_usize, overflow);
    assert_eq!(b_usize, 0_usize);
    assert_eq!(overflow, false);

    let c_usize = 12345678901234567890_usize;
    let (d_usize, overflow) = small_uint_overflowing_neg_func(c_usize);
    println!("-{} = {}, {}", c_usize, d_usize, overflow);
    assert_eq!(d_usize, 6101065172474983726_usize);
    assert_eq!(overflow, true);
    
    // Example for ShortUnion
    let a_shortunion = 0_u16.into_shortunion();
    let (b_shortunion, overflow) = small_uint_overflowing_neg_func(a_shortunion);
    println!("-{} = {}, {}", a_shortunion, b_shortunion, overflow);
    assert_eq!(b_shortunion.get(), 0_u16);
    assert_eq!(overflow, false);

    let c_shortunion = 12345_u16.into_shortunion();
    let (d_shortunion, overflow) = small_uint_overflowing_neg_func(c_shortunion);
    println!("-{} = {}, {}", c_shortunion, d_shortunion, overflow);
    assert_eq!(d_shortunion.get(), 53191_u16);
    assert_eq!(overflow, true);
    
    // Example for IntUnion
    let a_intunion = 0_u32.into_intunion();
    let (b_intunion, overflow) = small_uint_overflowing_neg_func(a_intunion);
    println!("-{} = {}, {}", a_intunion, b_intunion, overflow);
    assert_eq!(b_intunion.get(), 0_u32);
    assert_eq!(overflow, false);

    let c_intunion = 1234567890_u32.into_intunion();
    let (d_intunion, overflow) = small_uint_overflowing_neg_func(c_intunion);
    println!("-{} = {}, {}", c_intunion, d_intunion, overflow);
    assert_eq!(d_intunion.get(), 3060399406_u32);
    assert_eq!(overflow, true);
    
    // Example for LongUnion
    let a_longunion = 0_u64.into_longunion();
    let (b_longunion , overflow) = small_uint_overflowing_neg_func(a_longunion);
    println!("-{} = {}, {}", a_longunion, b_longunion, overflow);
    assert_eq!(b_longunion.get(), 0_u64);
    assert_eq!(overflow, false);

    let c_longunion = 12345678901234567890_u64.into_longunion();
    let (d_longunion, overflow) = small_uint_overflowing_neg_func(c_longunion);
    println!("-{} = {}, {}", c_longunion, d_longunion, overflow);
    assert_eq!(d_longunion.get(), 6101065172474983726_u64);
    assert_eq!(overflow, true);
    
    // Example for LongerUnion
    let a_longerunion = 0_u128.into_longerunion();
    let (b_longerunion, overflow) = small_uint_overflowing_neg_func(a_longerunion);
    println!("-{} = {}, {}", a_longerunion, b_longerunion, overflow);
    assert_eq!(b_longerunion.get(), 0_u128);
    assert_eq!(overflow, false);

    let c_longerunion = 123456789012345678901234567890123456789_u128.into_longerunion();
    let (d_longerunion, overflow) = small_uint_overflowing_neg_func(c_longerunion);
    println!("-{} = {}, {}", c_longerunion, d_longerunion, overflow);
    assert_eq!(d_longerunion.get(), 216825577908592784562140039541644754667_u128);
    assert_eq!(overflow, true);
    
    // Example for SizeUnion
    let a_sizeunion = 0_usize.into_sizeunion();
    let (b_sizeunion, overflow) = small_uint_overflowing_neg_func(a_sizeunion);
    println!("-{} = {}, {}", a_sizeunion, a_sizeunion, overflow);
    assert_eq!(b_sizeunion.get(), 0_usize);
    assert_eq!(overflow, false);

    let c_sizeunion = 1234567890123456789_usize.into_sizeunion();
    let (d_sizeunion, overflow) = small_uint_overflowing_neg_func(c_sizeunion);
    println!("-{} = {}, {}", c_sizeunion, d_sizeunion, overflow);
    assert_eq!(d_sizeunion.get(), 17212176183586094827_usize);
    assert_eq!(overflow, true);
    println!("--------------------------------------");

}

fn small_uint_overflowing_neg_func<T: cryptocol::number::SmallUInt>(me: T) -> (T, bool)
{
    me.overflowing_neg()
}

// fn small_uint_checked_neg()
// {

// }

// fn small_uint_unchecked_neg()
// {

// }

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
    println!("small_uint_pow");
    use cryptocol::number::{ ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };
    // Example for u8
    let a_u8 = small_uint_pow_func(3_u8, 5_u32);
    println!("3 ** 5 = {}, where ** is the power operator", a_u8);
    assert_eq!(a_u8, 243_u8);
    // It will panic.
    // println!("3 ** 6 = {}, where ** is the power operator", small_uint_pow_func(3_u8, 6_u32));

    // Example for u16
    let a_u16 = small_uint_pow_func(9_u16, 5_u32);
    println!("9 ** 5 = {}, where ** is the power operator", a_u16);
    assert_eq!(a_u16, 59049_u16);
    // It will panic.
    // println!("9 ** 6 = {}, where ** is the power operator", small_uint_pow_func(9_u16, 6_u32));

    // Example for u32
    let a_u32 = small_uint_pow_func(81_u32, 5_u32);
    println!("81 ** 5 = {}, where ** is the power operator", a_u32);
    assert_eq!(a_u32, 3486784401_u32);
    // It will panic.
    // println!("81 ** 6 = {}, where ** is the power operator", small_uint_pow_func(81_u32, 6_u32));

    // Example for u64
    let a_u64 = small_uint_pow_func(6561_u64, 5_u32);
    println!("6561 ** 5 = {}, where ** is the power operator", a_u64);
    assert_eq!(a_u64, 12157665459056928801_u64);
    // It will panic.
    // println!("6561 ** 6 = {}, where ** is the power operator", small_uint_pow_func(6561_u64, 6_u32));

    // Example for u128
    let a_u128 = small_uint_pow_func(43046721_u128, 5_u32);
    println!("43046721 ** 5 = {}, where ** is the power operator", a_u128);
    assert_eq!(a_u128, 147808829414345923316083210206383297601_u128);
    // It will panic.
    // println!("43046721 ** 6 = {}, where ** is the power operator", small_uint_pow_func(43046721_u64, 6_u32));

    // Example for usize
    let a_usize = small_uint_pow_func(6561_usize, 5_u32);
    println!("6561 ** 5 = {}, where ** is the power operator", a_usize);
    assert_eq!(a_usize, 12157665459056928801_usize);
    // It will panic.
    // println!("6561 ** 6 = {}, where ** is the power operator", small_uint_pow_func(6561_usize, 6_u32));

    // Example for ShortUnion
    let a_shortunion = ShortUnion::new_with(9);
    let b_shortunion = small_uint_pow_func(a_shortunion, 5_u32);
    println!("9 ** 5 = {}, where ** is the power operator", b_shortunion);
    assert_eq!(b_shortunion.get(), 59049_u16);
    // It will panic.
    // println!("9 ** 5 = {}, where ** is the power operator", small_uint_pow_func(a_shortunion, 6_u32));

    // Example for IntUnion
    let a_intunion = IntUnion::new_with(81);
    let b_intunion = small_uint_pow_func(a_intunion, 5_u32);
    println!("81 ** 5 = {}, where ** is the power operator", b_intunion);
    assert_eq!(b_intunion.get(), 3486784401_u32);
    // It will panic.
    // println!("81 ** 6 = {}, where ** is the power operator", small_uint_pow_func(a_intunion, 6_u32));

    // Example for LongUnion
    let a_longunion = LongUnion::new_with(6561);
    let b_longunion = small_uint_pow_func(a_longunion, 5_u32);
    println!("6561 ** 5 = {}, where ** is the power operator", b_longunion);
    assert_eq!(b_longunion.get(), 12157665459056928801_u64);
    // It will panic.
    // println!("6561 ** 6 = {}, where ** is the power operator", small_uint_pow_func(a_longunion, 6_u32));

    // Example for LongerUnion
    let a_longerunion = LongerUnion::new_with(43046721);
    let b_longerunion = small_uint_pow_func(a_longerunion, 5_u32);
    println!("43046721 ** 5 = {}, where ** is the power operator", b_longerunion);
    assert_eq!(b_longerunion.get(), 147808829414345923316083210206383297601_u128);
    // It will panic.
    // println!("43046721 ** 6 = {}, where ** is the power operator", small_uint_pow_func(a_longerunion, 6_u32));

    // Example for SizeUnion
    let a_sizeunion = SizeUnion::new_with(6561);
    let b_sizeunion = small_uint_pow_func(a_sizeunion, 5_u32);
    println!("6561 ** 5 = {}, where ** is the power operator", b_sizeunion);
    assert_eq!(b_sizeunion.get(), 12157665459056928801_usize);
    // It will panic.
    // println!("6561 ** 6 = {}, where ** is the power operator", small_uint_pow_func(a_sizeunion, 6_u32));
    println!("--------------------------------------");
}

fn small_uint_pow_func<T: cryptocol::number::SmallUInt>(base: T, exp: u32) -> T
{
    base.pow(exp)
}

fn small_uint_wrapping_pow()
{
    println!("small_uint_wrapping_pow");
    use cryptocol::number::{ ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };

    // Example for u8
    let a_u8 = small_uint_wrapping_pow_func(3_u8, 5_u32);
    println!("3 ** 5 = {}, where ** is the power operator", a_u8);
    assert_eq!(a_u8, 243_u8);

    let b_u8 = small_uint_wrapping_pow_func(3_u8, 6_u32);
    println!("3 ** 6 = {}, where ** is the power operator", b_u8);
    assert_eq!(b_u8, 217_u8);

    // Example for u16
    let a_u16 = small_uint_wrapping_pow_func(9_u16, 5_u32);
    println!("9 ** 5 = {}, where ** is the power operator", a_u16);
    assert_eq!(a_u16, 59049_u16);

    let b_u16 = small_uint_wrapping_pow_func(9_u16, 6_u32);
    println!("9 ** 6 = {}, where ** is the power operator", b_u16);
    assert_eq!(b_u16, 7153_u16);

    // Example for u32
    let a_u32 = small_uint_wrapping_pow_func(81_u32, 5_u32);
    println!("81 ** 5 = {}, where ** is the power operator", a_u32);
    assert_eq!(a_u32, 3486784401_u32);

    let b_u32 = small_uint_wrapping_pow_func(81_u32, 6_u32);
    println!("81 ** 6 = {}, where ** is the power operator", b_u32);
    assert_eq!(b_u32, 3256662241_u32);

    // Example for u64
    let a_u64 = small_uint_wrapping_pow_func(6561_u64, 5_u32);
    println!("6561 ** 5 = {}, where ** is the power operator", a_u64);
    assert_eq!(a_u64, 12157665459056928801_u64);

    let b_u64 = small_uint_wrapping_pow_func(6561_u64, 6_u32);
    println!("6561 ** 6 = {}, where ** is the power operator", b_u64);
    assert_eq!(b_u64, 2721702152408675777_u64);

    // Example for u128
    let a_u128 = small_uint_wrapping_pow_func(43046721_u128, 5_u32);
    println!("43046721 ** 5 = {}, where ** is the power operator", a_u128);
    assert_eq!(a_u128, 147808829414345923316083210206383297601_u128);

    let b_u128 = small_uint_wrapping_pow_func(43046721_u128, 6_u32);
    println!("43046721 ** 6 = {}, where ** is the power operator", b_u128);
    assert_eq!(b_u128, 333574137813082321045752866839264852865_u128);

    // Example for usize
    let a_usize = small_uint_wrapping_pow_func(6561_usize, 5_u32);
    println!("6561 ** 5 = {}, where ** is the power operator", a_usize);
    assert_eq!(a_usize, 12157665459056928801_usize);

    let b_usize = small_uint_wrapping_pow_func(6561_usize, 6_u32);
    println!("6561 ** 6 = {}, where ** is the power operator", b_usize);
    assert_eq!(b_usize, 2721702152408675777_usize);

    // Example for ShortUnion
    let a_shortunion = ShortUnion::new_with(9);
    let b_shortunion = small_uint_wrapping_pow_func(a_shortunion, 5_u32);
    println!("9 ** 5 = {}, where ** is the power operator", b_shortunion);
    assert_eq!(b_shortunion.get(), 59049_u16);

    let c_shortunion = small_uint_wrapping_pow_func(a_shortunion, 6_u32);
    println!("9 ** 6 = {}, where ** is the power operator", c_shortunion);
    assert_eq!(c_shortunion.get(), 7153_u16);

    // Example for IntUnion
    let a_intunion = IntUnion::new_with(81);
    let b_intunion = small_uint_wrapping_pow_func(a_intunion, 5_u32);
    println!("81 ** 5 = {}, where ** is the power operator", b_intunion);
    assert_eq!(b_intunion.get(), 3486784401_u32);

    let c_intunion = small_uint_wrapping_pow_func(a_intunion, 6_u32);
    println!("81 ** 6 = {}, where ** is the power operator", c_intunion);
    assert_eq!(c_intunion.get(), 3256662241_u32);

    // Example for LongUnion
    let a_longunion = LongUnion::new_with(6561);
    let b_longunion = small_uint_wrapping_pow_func(a_longunion, 5_u32);
    println!("6561 ** 5 = {}, where ** is the power operator", b_longunion);
    assert_eq!(b_longunion.get(), 12157665459056928801_u64);

    let c_longunion = small_uint_wrapping_pow_func(a_longunion, 6_u32);
    println!("6561 ** 6 = {}, where ** is the power operator", c_longunion);
    assert_eq!(c_longunion.get(), 2721702152408675777_u64);

    // Example for LongerUnion
    let a_longerunion = LongerUnion::new_with(43046721_u128);
    let b_longerunion = small_uint_wrapping_pow_func(a_longerunion, 5_u32);
    println!("43046721 ** 5 = {}, where ** is the power operator", b_longerunion);
    assert_eq!(b_longerunion.get(), 147808829414345923316083210206383297601_u128);

    let c_longerunion = small_uint_wrapping_pow_func(a_longerunion, 6_u32);
    println!("43046721 ** 6 = {}, where ** is the power operator", c_longerunion);
    assert_eq!(c_longerunion.get(), 333574137813082321045752866839264852865_u128);

    // Example for SizeUnion
    let a_sizeunion = SizeUnion::new_with(6561);
    let b_sizeunion = small_uint_wrapping_pow_func(a_sizeunion, 5_u32);
    println!("6561 ** 5 = {}, where ** is the power operator", b_sizeunion);
    assert_eq!(b_sizeunion.get(), 12157665459056928801_usize);

    let c_sizeunion = small_uint_wrapping_pow_func(a_sizeunion, 6_u32);
    println!("6561 ** 6 = {}, where ** is the power operator", c_sizeunion);
    assert_eq!(c_sizeunion.get(), 2721702152408675777_usize);
    println!("--------------------------------------");
}

fn small_uint_wrapping_pow_func<T: cryptocol::number::SmallUInt>(base: T, exp: u32) -> T
{
    base.wrapping_pow(exp)
}

fn small_uint_overflowing_pow()
{
    println!("small_uint_overflowing_pow");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let (a_u8, overflow) = small_uint_overflowing_pow_func(6_u8, 3);
    println!("{} ** 3 = {}, where ** is the power operator\nOverflow = {}", 6, a_u8, overflow);
    assert_eq!(a_u8, 216_u8);
    assert_eq!(overflow, false);

    let (b_u8, overflow) = small_uint_overflowing_pow_func(6_u8, 4);
    println!("{} ** 4 = {}, where ** is the power operator\nOverflow = {}", 6, b_u8, overflow);
    assert_eq!(b_u8, 16_u8);
    assert_eq!(overflow, true);

    // Example for u16
    let (a_u16, overflow) = small_uint_overflowing_pow_func(12_u16, 4);
    println!("{} ** 4 = {}, where ** is the power operator\nOverflow = {}", 12_u16, a_u16, overflow);
    assert_eq!(a_u16, 20736_u16);
    assert_eq!(overflow, false);
 
    let (b_u16, overflow) = small_uint_overflowing_pow_func(12_u16, 5);
    println!("{} ** 5 = {}, where ** is the power operator\nOverflow = {}", 12_u16, b_u16, overflow);
    assert_eq!(b_u16, 52224_u16);
    assert_eq!(overflow, true);

    // Example for u32
    let (a_u32, overflow) = small_uint_overflowing_pow_func(38_u32, 6);
    println!("{} ** 6 = {}, where ** is the power operator\nOverflow = {}", 38_u32, a_u32, overflow);
    assert_eq!(a_u32, 3010936384_u32);
    assert_eq!(overflow, false);
 
    let (b_u32, overflow) = small_uint_overflowing_pow_func(38_u32, 7);
    println!("{} ** 7 = {}, where ** is the power operator\nOverflow = {}", 38_u32, b_u32, overflow);
    assert_eq!(b_u32, 2746432896_u32);
    assert_eq!(overflow, true);

    // Example for u64
    let (a_u64, overflow) = small_uint_overflowing_pow_func(1004_u64, 6);
    println!("{} ** 6 = {}, where ** is the power operator\nOverflow = {}", 1004_u64, a_u64, overflow);
    assert_eq!(a_u64, 1024241283846148096_u64);
    assert_eq!(overflow, false);
 
    let (b_u64, overflow) = small_uint_overflowing_pow_func(1004_u64, 7);
    println!("{} ** 7 = {}, where ** is the power operator\nOverflow = {}", 1004_u64, b_u64, overflow);
    assert_eq!(b_u64, 13767324927507349504_u64);
    assert_eq!(overflow, true);

    // Example for u128
    let (a_u128, overflow) = small_uint_overflowing_pow_func(10003_u128, 9);
    println!("{} ** 9 = {}, where ** is the power operator\nOverflow = {}", 10003_u128, a_u128, overflow);
    assert_eq!(a_u128, 1002703242269020906241243873790509683_u128);
    assert_eq!(overflow, false);
 
    let (b_u128, overflow) = small_uint_overflowing_pow_func(10003_u128, 10);
    println!("{} ** 10 = {}, where ** is the power operator\nOverflow = {}", 10003_u128, b_u128, overflow);
    assert_eq!(b_u128, 161851891709800684693298854005190226825_u128);
    assert_eq!(overflow, true);

    // Example for usize
    let (a_usize, overflow) = small_uint_overflowing_pow_func(1004_usize, 6);
    println!("{} ** 6 = {}, where ** is the power operator\nOverflow = {}", 1004_u64, a_usize, overflow);
    assert_eq!(a_usize, 1024241283846148096_usize);
    assert_eq!(overflow, false);
 
    let (b_usize, overflow) = small_uint_overflowing_pow_func(1004_usize, 7);
    println!("{} ** 7 = {}, where ** is the power operator\nOverflow = {}", 1004_u64, b_usize, overflow);
    assert_eq!(b_usize, 13767324927507349504_usize);
    assert_eq!(overflow, true);

    // Example for ShortUnion
    let (a_shortunion, overflow) = small_uint_overflowing_pow_func(12_u16.into_shortunion(), 4);
    println!("{} ** 4 = {}, where ** is the power operator\nOverflow = {}", 12_u16, a_shortunion, overflow);
    assert_eq!(a_shortunion.get(), 20736_u16);
    assert_eq!(overflow, false);
 
    let (b_shortunion, overflow) = small_uint_overflowing_pow_func(12_u16.into_shortunion(), 5);
    println!("{} ** 5 = {}, where ** is the power operator\nOverflow = {}", 12_u16, b_shortunion, overflow);
    assert_eq!(b_shortunion.get(), 52224_u16);
    assert_eq!(overflow, true);

    // Example for IntUnion
    let (a_intunion, overflow) = small_uint_overflowing_pow_func(38_u32.into_intunion(), 6);
    println!("{} ** 6 = {}, where ** is the power operator\nOverflow = {}", 38_u32, a_intunion, overflow);
    assert_eq!(a_intunion.get(), 3010936384_u32);
    assert_eq!(overflow, false);
 
    let (b_intunion, overflow) = small_uint_overflowing_pow_func(38_u32.into_intunion(), 7);
    println!("{} ** 7 = {}, where ** is the power operator\nOverflow = {}", 38_u32, b_intunion, overflow);
    assert_eq!(b_intunion.get(), 2746432896_u32);
    assert_eq!(overflow, true);

    // Example for LongUnion
    let (a_longunion, overflow) = small_uint_overflowing_pow_func(1004_u64.into_longunion(), 6);
    println!("{} ** 6 = {}, where ** is the power operator\nOverflow = {}", 1004_u64, a_longunion, overflow);
    assert_eq!(a_longunion.get(), 1024241283846148096_u64);
    assert_eq!(overflow, false);
 
    let (b_longunion, overflow) = small_uint_overflowing_pow_func(1004_u64.into_longunion(), 7);
    println!("{} ** 7 = {}, where ** is the power operator\nOverflow = {}", 1004_u64, b_longunion, overflow);
    assert_eq!(b_longunion.get(), 13767324927507349504_u64);
    assert_eq!(overflow, true);

    // Example for LongerUnion
    let (a_longerunion, overflow) = small_uint_overflowing_pow_func(10003_u128.into_longerunion(), 9);
    println!("{} ** 9 = {}, where ** is the power operator\nOverflow = {}", 10003_u128, a_longerunion, overflow);
    assert_eq!(a_longerunion.get(), 1002703242269020906241243873790509683_u128);
    assert_eq!(overflow, false);
 
    let (b_longerunion, overflow) = small_uint_overflowing_pow_func(10003_u128.into_longerunion(), 10);
    println!("{} ** 10 = {}, where ** is the power operator\nOverflow = {}", 10003_u128, b_longerunion, overflow);
    assert_eq!(b_longerunion.get(), 161851891709800684693298854005190226825_u128);
    assert_eq!(overflow, true);

    // Example for SizeUnion
    let (a_sizeunion, overflow) = small_uint_overflowing_pow_func(1004_usize.into_sizeunion(), 6);
    println!("{} ** 6 = {}, where ** is the power operator\nOverflow = {}", 1004_u64, a_sizeunion, overflow);
    assert_eq!(a_sizeunion.get(), 1024241283846148096_usize);
    assert_eq!(overflow, false);
 
    let (b_sizeunion, overflow) = small_uint_overflowing_pow_func(1004_usize.into_sizeunion(), 7);
    println!("{} ** 7 = {}, where ** is the power operator\nOverflow = {}", 1004_u64, b_sizeunion, overflow);
    assert_eq!(b_sizeunion.get(), 13767324927507349504_usize);
    assert_eq!(overflow, true);
    println!("--------------------------------------");
}

fn small_uint_overflowing_pow_func<T: cryptocol::number::SmallUInt>(base: T, exp: u32) -> (T, bool)
{
    base.overflowing_pow(exp)
}

fn small_uint_checked_pow()
{
    println!("small_uint_checked_pow");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = small_uint_checked_pow_func(6_u8, 3);
    match a_u8
    {
        Some(a) => {
                println!("{} ** 3 = {}, where ** is the power operator", 6_u8, a);
                assert_eq!(a, 216_u8);
            },
        None => { println!("Overflow happened."); },
    }

    let b_u8 = small_uint_checked_pow_func(6_u8, 4);
    match b_u8
    {
        Some(b) => { println!("{} ** 4 = {}, where ** is the power operator", 6_u8, b); },
        None => {
                println!("Overflow happened.");
                assert_eq!(b_u8, None);
            },
    }

    // Example for u16
    let a_u16 = small_uint_checked_pow_func(12_u16, 4);
    match a_u16
    {
        Some(a) => {
                println!("{} ** 4 = {}, where ** is the power operator", 12_u16, a);
                assert_eq!(a, 20736_u16);
            },
        None => { println!("Overflow happened."); },
    }

    let b_u16 = small_uint_checked_pow_func(12_u16, 5);
    match b_u16
    {
        Some(b) => { println!("{} ** 5 = {}, where ** is the power operator", 12_u16, b); },
        None => {
                println!("Overflow happened.");
                assert_eq!(b_u16, None);
            },
    }

    // Example for u32
    let a_u32 = small_uint_checked_pow_func(38_u32, 6);
    match a_u32
    {
        Some(a) => {
                println!("{} ** 6 = {}, where ** is the power operator", 38_u32, a);
                assert_eq!(a, 3010936384_u32);
            },
        None => { println!("Overflow happened."); },
    }

    let b_u32 = small_uint_checked_pow_func(38_u32, 7);
    match b_u32
    {
        Some(b) => { println!("{} ** 7 = {}, where ** is the power operator", 38_u32, b); },
        None => {
                println!("Overflow happened.");
                assert_eq!(b_u32, None);
            },
    }

    // Example for u64
    let a_u64 = small_uint_checked_pow_func(1004_u64, 6);
    match a_u64
    {
        Some(a) => {
                println!("{} ** 6 = {}, where ** is the power operator", 1004_u64, a);
                assert_eq!(a, 1024241283846148096_u64);
            },
        None => { println!("Overflow happened."); },
    }

    let b_u64 = small_uint_checked_pow_func(1004_u64, 7);
    match b_u64
    {
        Some(b) => { println!("{} ** 7 = {}, where ** is the power operator", 1004_u64, b); },
        None => {
                println!("Overflow happened.");
                assert_eq!(b_u64, None);
            },
    }

    // Example for u128
    let a_u128 = small_uint_checked_pow_func(10003_u128, 9);
    match a_u128
    {
        Some(a) => {
                println!("{} ** 9 = {}, where ** is the power operator", 10003_u128, a);
                assert_eq!(a, 1002703242269020906241243873790509683_u128);
            },
        None => { println!("Overflow happened."); },
    }

    let b_u128 = small_uint_checked_pow_func(10003_u128, 10);
    match b_u128
    {
        Some(b) => { println!("{} ** 10 = {}, where ** is the power operator", 10003_u128, b); },
        None => {
                println!("Overflow happened.");
                assert_eq!(b_u128, None);
            },
    }

    // Example for usize
    let a_usize = small_uint_checked_pow_func(1004_usize, 6);
    match a_usize
    {
        Some(a) => {
                println!("{} ** 6 = {}, where ** is the power operator", 1004_usize, a);
                assert_eq!(a, 1024241283846148096_usize);
            },
        None => { println!("Overflow happened."); },
    }

    let b_usize = small_uint_checked_pow_func(1004_usize, 7);
    match b_usize
    {
        Some(b) => { println!("{} ** 7 = {}, where ** is the power operator", 1004_usize, b); },
        None => {
                println!("Overflow happened.");
                assert_eq!(b_usize, None);
            },
    }

    // Example for ShortUnion
    let a_shortunion = small_uint_checked_pow_func(12_u16.into_shortunion(), 4);
    match a_shortunion
    {
        Some(a) => {
                println!("{} ** 4 = {}, where ** is the power operator", 12_u16.into_shortunion(), a);
                assert_eq!(a.get(), 20736_u16);
            },
        None => { println!("Overflow happened."); },
    }

    let b_shortunion = small_uint_checked_pow_func(12_u16.into_shortunion(), 5);
    match b_shortunion
    {
        Some(b) => { println!("{} ** 5 = {}, where ** is the power operator", 12_u16.into_shortunion(), b); },
        None => {
                println!("Overflow happened.");
                assert_eq!(b_shortunion, None);
            },
    }

    // Example for IntUnion
    let a_intunion = small_uint_checked_pow_func(38_u32.into_intunion(), 6);
    match a_intunion
    {
        Some(a) => {
                println!("{} ** 6 = {}, where ** is the power operator", 38_u32.into_intunion(), a);
                assert_eq!(a.get(), 3010936384_u32);
            },
        None => { println!("Overflow happened."); },
    }

    let b_intunion = small_uint_checked_pow_func(38_u32.into_intunion(), 7);
    match b_intunion
    {
        Some(b) => { println!("{} ** 7 = {}, where ** is the power operator", 38_u32.into_intunion(), b); },
        None => {
                println!("Overflow happened.");
                assert_eq!(b_intunion, None);
            },
    }

    // Example for LongUnion
    let a_longunion = small_uint_checked_pow_func(1004_u64.into_longunion(), 6);
    match a_longunion
    {
        Some(a) => {
                println!("{} ** 6 = {}, where ** is the power operator", 1004_u64.into_longunion(), a);
                assert_eq!(a.get(), 1024241283846148096_u64);
            },
        None => { println!("Overflow happened."); },
    }

    let b_longunion = small_uint_checked_pow_func(1004_u64.into_longunion(), 7);
    match b_longunion
    {
        Some(b) => { println!("{} ** 7 = {}, where ** is the power operator", 1004_u64.into_longunion(), b); },
        None => {
                println!("Overflow happened.");
                assert_eq!(b_longunion, None);
            },
    }

    // Example for LongerUnion
    let a_longerunion = small_uint_checked_pow_func(10003_u128.into_longerunion(), 9);
    match a_longerunion
    {
        Some(a) => {
                println!("{} ** 9 = {}, where ** is the power operator", 10003_u128.into_longerunion(), a);
                assert_eq!(a.get(), 1002703242269020906241243873790509683_u128);
            },
        None => { println!("Overflow happened."); },
    }

    let b_longerunion = small_uint_checked_pow_func(10003_u128.into_longerunion(), 10);
    match b_longerunion
    {
        Some(b) => { println!("{} ** 10 = {}, where ** is the power operator", 10003_u128.into_longerunion(), b); },
        None => {
                println!("Overflow happened.");
                assert_eq!(b_longerunion, None);
            },
    }

    // Example for SizeUnion
    let a_sizeunion = small_uint_checked_pow_func(1004_usize.into_sizeunion(), 6);
    match a_sizeunion
    {
        Some(a) => {
                println!("{} ** 6 = {}, where ** is the power operator", 1004_usize.into_sizeunion(), a);
                assert_eq!(a.get(), 1024241283846148096_usize);
            },
        None => { println!("Overflow happened."); },
    }

    let b_sizeunion = small_uint_checked_pow_func(1004_usize.into_sizeunion(), 7);
    match b_sizeunion
    {
        Some(b) => { println!("{} ** 7 = {}, where ** is the power operator", 1004_usize.into_sizeunion(), b); },
        None => {
                println!("Overflow happened.");
                assert_eq!(b_sizeunion, None);
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
    println!("small_uint_unchecked_pow");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = small_uint_unchecked_pow_func(6_u8, 3);
    println!("{} ** 3 = {}, where ** is the power operator", 6_u8, a_u8);
    assert_eq!(a_u8, 216_u8);
    // It will panic.
    // let b_u8 = small_uint_unchecked_pow_func(6_u8, 4);

    // Example for u16
    let a_u16 = small_uint_unchecked_pow_func(12_u16, 4);
    println!("{} ** 4 = {}, where ** is the power operator", 12_u16, a_u16);
    assert_eq!(a_u16, 20736_u16);
    // It will panic.
    // let b_u16 = small_uint_unchecked_pow_func(12_u16, 5);

    // Example for u32
    let a_u32 = small_uint_unchecked_pow_func(38_u32, 6);
    println!("{} ** 6 = {}, where ** is the power operator", 38_u32, a_u32);
    assert_eq!(a_u32, 3010936384_u32);
    // It will panic.
    // let b_u32 = small_uint_unchecked_pow_func(38_u32, 7);

    // Example for u64
    let a_u64 = small_uint_unchecked_pow_func(1004_u64, 6);
    println!("{} ** 6 = {}, where ** is the power operator", 1004_u64, a_u64);
    // It will panic.
    // let b_u64 = small_uint_unchecked_pow_func(1004_u64, 7);

    // Example for u128
    let a_u128 = small_uint_unchecked_pow_func(10003_u128, 9);
    println!("{} ** 9 = {}, where ** is the power operator", 10003_u128, a_u128);
    assert_eq!(a_u128, 1002703242269020906241243873790509683_u128);
    // It will panic.
    // let b_u128 = small_uint_unchecked_pow_func(10003_u128, 10);

    // Example for usize
    let a_usize = small_uint_unchecked_pow_func(1004_usize, 6);
    println!("{} ** 6 = {}, where ** is the power operator", 1004_usize, a_usize);
    assert_eq!(a_usize, 1024241283846148096_usize);
    // It will panic.
    // let b_usize = small_uint_unchecked_pow_func(1004_usize, 7);

    // Example for ShortUnion
    let a_shortunion = small_uint_unchecked_pow_func(12_u16.into_shortunion(), 4);
    println!("{} ** 4 = {}, where ** is the power operator", 12_u16.into_shortunion(), a_shortunion);
    assert_eq!(a_shortunion.get(), 20736_u16);
    // It will panic.
    // let b_shortunion = small_uint_unchecked_pow_func(12_u16.into_shortunion(), 5);

    // Example for IntUnion
    let a_intunion = small_uint_unchecked_pow_func(38_u32.into_intunion(), 6);
    println!("{} ** 6 = {}, where ** is the power operator", 38_u32.into_intunion(), a_intunion);
    assert_eq!(a_intunion.get(), 3010936384_u32);
    // It will panic.
    // let b_intunion = small_uint_unchecked_pow_func(38_u32.into_intunion(), 7);

    // Example for LongUnion
    let a_longunion = small_uint_unchecked_pow_func(1004_u64.into_longunion(), 6);
    println!("{} ** 6 = {}, where ** is the power operator", 1004_u64.into_longunion(), a_longunion);
    assert_eq!(a_longunion.get(), 1024241283846148096_u64);
    // It will panic.
    // let b_longunion = small_uint_unchecked_pow_func(1004_u64.into_longunion(), 7);

    // Example for LongerUnion
    let a_longerunion = small_uint_unchecked_pow_func(10003_u128.into_longerunion(), 9);
    println!("{} ** 9 = {}, where ** is the power operator", 10003_u128.into_longerunion(), a_longerunion);
    assert_eq!(a_longerunion.get(), 1002703242269020906241243873790509683_u128);
    // It will panic.
    // let b_longerunion = small_uint_unchecked_pow_func(10003_u128.into_longerunion(), 10);

    // Example for SizeUnion
    let a_sizeunion = small_uint_unchecked_pow_func(1004_usize.into_sizeunion(), 6);
    println!("{} ** 6 = {}, where ** is the power operator", 1004_usize.into_sizeunion(), a_sizeunion);
    assert_eq!(a_sizeunion.get(), 1024241283846148096_usize);
    // It will panic.
    // let b_sizeunion = small_uint_checked_pow_func(1004_usize.into_sizeunion(), 7);
    println!("--------------------------------------");
}

fn small_uint_unchecked_pow_func<T: cryptocol::number::SmallUInt>(base: T, exp: u32) -> T
{
    base.unchecked_pow(exp)
}

fn small_uint_saturating_pow()
{
    println!("small_uint_saturating_pow");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = small_uint_saturating_pow_func(6_u8, 3);
    println!("{} ** 3 = {}, where ** is the power operator", 6_u8, a_u8);
    assert_eq!(a_u8, 216_u8);
    let b_u8 = small_uint_saturating_pow_func(6_u8, 4);
    println!("{} ** 4 = {}, where ** is the power operator", 6_u8, b_u8);
    assert_eq!(b_u8, u8::MAX);

    // Example for u16
    let a_u16 = small_uint_saturating_pow_func(12_u16, 4);
    println!("{} ** 4 = {}, where ** is the power operator", 12_u16, a_u16);
    assert_eq!(a_u16, 20736_u16);
    let b_u16 = small_uint_saturating_pow_func(12_u16, 5);
    println!("{} ** 5 = {}, where ** is the power operator", 12_u16, b_u16);
    assert_eq!(b_u16, u16::MAX);

    // Example for u32
    let a_u32 = small_uint_saturating_pow_func(38_u32, 6);
    println!("{} ** 6 = {}, where ** is the power operator", 38_u32, a_u32);
    assert_eq!(a_u32, 3010936384_u32);
    let b_u32 = small_uint_saturating_pow_func(38_u32, 7);
    println!("{} ** 7 = {}, where ** is the power operator", 38_u32, b_u32);
    assert_eq!(b_u32, u32::MAX);

    // Example for u64
    let a_u64 = small_uint_saturating_pow_func(1004_u64, 6);
    println!("{} ** 6 = {}, where ** is the power operator", 1004_u64, a_u64);
    let b_u64 = small_uint_saturating_pow_func(1004_u64, 7);
    println!("{} ** 7 = {}, where ** is the power operator", 1004_u64, b_u64);
    assert_eq!(b_u64, u64::MAX);

    // Example for u128
    let a_u128 = small_uint_saturating_pow_func(10003_u128, 9);
    println!("{} ** 9 = {}, where ** is the power operator", 10003_u128, a_u128);
    assert_eq!(a_u128, 1002703242269020906241243873790509683_u128);
    let b_u128 = small_uint_saturating_pow_func(10003_u128, 10);
    println!("{} ** 10 = {}, where ** is the power operator", 10003_u128, b_u128);
    assert_eq!(b_u128, u128::MAX);

    // Example for usize
    let a_usize = small_uint_saturating_pow_func(1004_usize, 6);
    println!("{} ** 6 = {}, where ** is the power operator", 1004_usize, a_usize);
    assert_eq!(a_usize, 1024241283846148096_usize);
    let b_u128 = small_uint_saturating_pow_func(1004_usize, 7);
    println!("{} ** 7 = {}, where ** is the power operator", 1004_usize, b_u128);
    assert_eq!(b_u128, usize::MAX);

    // Example for ShortUnion
    let a_shortunion = small_uint_saturating_pow_func(12_u16.into_shortunion(), 4);
    println!("{} ** 4 = {}, where ** is the power operator", 12_u16.into_shortunion(), a_shortunion);
    assert_eq!(a_shortunion.get(), 20736_u16);
    let b_shortunion = small_uint_saturating_pow_func(12_u16.into_shortunion(), 5);
    println!("{} ** 5 = {}, where ** is the power operator", 12_u16.into_shortunion(), b_shortunion);
    assert_eq!(b_shortunion.get(), u16::MAX);

    // Example for IntUnion
    let a_intunion = small_uint_saturating_pow_func(38_u32.into_intunion(), 6);
    println!("{} ** 6 = {}, where ** is the power operator", 38_u32.into_intunion(), a_intunion);
    assert_eq!(a_intunion.get(), 3010936384_u32);
    let b_intunion = small_uint_saturating_pow_func(38_u32.into_intunion(), 7);
    println!("{} ** 7 = {}, where ** is the power operator", 38_u32.into_intunion(), b_intunion);
    assert_eq!(b_intunion.get(), u32::MAX);

    // Example for LongUnion
    let a_longunion = small_uint_saturating_pow_func(1004_u64.into_longunion(), 6);
    println!("{} ** 6 = {}, where ** is the power operator", 1004_u64.into_longunion(), a_longunion);
    assert_eq!(a_longunion.get(), 1024241283846148096_u64);
    let b_longunion = small_uint_saturating_pow_func(1004_u64.into_longunion(), 7);
    println!("{} ** 7 = {}, where ** is the power operator", 1004_u64.into_longunion(), b_longunion);
    assert_eq!(b_longunion.get(), u64::MAX);

    // Example for LongerUnion
    let a_longerunion = small_uint_saturating_pow_func(10003_u128.into_longerunion(), 9);
    println!("{} ** 9 = {}, where ** is the power operator", 10003_u128.into_longerunion(), a_longerunion);
    assert_eq!(a_longerunion.get(), 1002703242269020906241243873790509683_u128);
    let b_longerunion = small_uint_saturating_pow_func(10003_u128.into_longerunion(), 10);
    println!("{} ** 10 = {}, where ** is the power operator", 10003_u128.into_longunion(), b_longerunion);
    assert_eq!(b_longerunion.get(), u128::MAX);

    // Example for SizeUnion
    let a_sizeunion = small_uint_saturating_pow_func(1004_usize.into_sizeunion(), 6);
    println!("{} ** 6 = {}, where ** is the power operator", 1004_usize.into_sizeunion(), a_sizeunion);
    assert_eq!(a_sizeunion.get(), 1024241283846148096_usize);
    let b_sizeunion = small_uint_saturating_pow_func(1004_usize.into_sizeunion(), 7);
    println!("{} ** 7 = {}, where ** is the power operator", 1004_usize.into_longunion(), b_sizeunion);
    assert_eq!(b_sizeunion.get(), usize::MAX);
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

    let a_u8 = 74_u8;
    let b_u8 = 18_u8;
    let modulo_u8 = 100_u8;
    let res_u8 = a_u8.modular_pow(b_u8, modulo_u8);
    println!("{} ** {} = {} (mod {}), where ** is the power operator", a_u8, b_u8, res_u8, modulo_u8);
    assert_eq!(res_u8, 76_u8);

    let c_u8 = 74_u8;
    let d_u8 = 18_u8;
    let modulo_u8 = 100_u8;
    let res_u8 = small_uint_modular_pow_func(c_u8, d_u8, modulo_u8);
    println!("{} ** {} = {} (mod {}), where ** is the power operator", c_u8, d_u8, res_u8, modulo_u8);
    assert_eq!(res_u8, 76_u8);

    let a_u16 = 741_u16;
    let b_u16 = 185_u16;
    let modulo_u16 = 10000_u16;
    let res_u16 = a_u16.modular_pow(b_u16, modulo_u16);
    println!("{} ** {} = {} (mod {}), where ** is the power operator", a_u16, b_u16, res_u16, modulo_u16);
    assert_eq!(res_u16, 8901_u16);

    let c_u16 = 741_u16;
    let d_u16 = 185_u16;
    let modulo_u16 = 10000_u16;
    let res_u16 = small_uint_modular_pow_func(c_u16, d_u16, modulo_u16);
    println!("{} ** {} = {} (mod {}), where ** is the power operator", c_u16, d_u16, res_u16, modulo_u16);
    assert_eq!(res_u16, 8901_u16);

    let a_u32 = 78541_u32;
    let b_u32 = 18575_u32;
    let modulo_u32 = 10000000_u32;
    let res_u32 = a_u32.modular_pow(b_u32, modulo_u32);
    println!("{} ** {} = {} (mod {}), where ** is the power operator", a_u32, b_u32, res_u32, modulo_u32);
    assert_eq!(res_u32, 4370501_u32);

    let c_u32 = 78541_u32;
    let d_u32 = 18575_u32;
    let modulo_u32 = 10000000_u32;
    let res_u32 = small_uint_modular_pow_func(c_u32, d_u32, modulo_u32);
    println!("{} ** {} = {} (mod {}), where ** is the power operator", c_u32, d_u32, res_u32, modulo_u32);
    assert_eq!(res_u32, 4370501_u32);

    let a_u64 = 123456789_u64;
    let b_u64 = 9876543_u64;
    let modulo_u64 = 100000000000_u64;
    let res_u64 = a_u64.modular_pow(b_u64, modulo_u64);
    println!("{} ** {} = {} (mod {}), where ** is the power operator", a_u64, b_u64, res_u64, modulo_u64);
    assert_eq!(res_u64, 75010148669_u64);

    let c_u64 = 123456789_u64;
    let d_u64 = 9876543_u64;
    let modulo_u64 = 100000000000_u64;
    let res_u64 = small_uint_modular_pow_func(c_u64, d_u64, modulo_u64);
    println!("{} ** {} = {} (mod {}), where ** is the power operator", c_u64, d_u64, res_u64, modulo_u64);
    assert_eq!(res_u64, 75010148669_u64);

    let a = 258974_u128;
    let d = 6622882488318_u128;
    let modulo = 4776913109852041418248056622882488319_u128;
    let res = small_uint_modular_pow_func(a, d, modulo);
    println!("{} ** {} = {} (mod {}), where ** is the power operator", a, d, res, modulo);
    assert_eq!(res, 2843356730633772030492705275006525566_u128);

    let a = 258974_u128;
    let d = 6622882488318_u128;
    let modulo = 4776913109852041418248056622882488319_u128;
    let res = small_uint_modular_pow_func(a, d, modulo);
    println!("{} ** {} = {} (mod {}), where ** is the power operator", a, d, res, modulo);
    assert_eq!(res, 2843356730633772030492705275006525566_u128);

    let a_usize = 123456789_usize;
    let b_usize = 9876543_usize;
    let modulo_usize = 100000000000_usize;
    let res_usize = a_usize.modular_pow(b_usize, modulo_usize);
    println!("{} ** {} = {} (mod {}), where ** is the power operator", a_usize, b_usize, res_usize, modulo_usize);
    assert_eq!(res_usize, 75010148669_usize);

    let c_usize = 123456789_usize;
    let d_usize = 9876543_usize;
    let modulo_usize = 100000000000_usize;
    let res_usize = small_uint_modular_pow_func(c_usize, d_usize, modulo_usize);
    println!("{} ** {} = {} (mod {}), where ** is the power operator", c_usize, d_usize, res_usize, modulo_usize);
    assert_eq!(res_usize, 75010148669_usize);

    let a_shortunion = 741_u16.into_shortunion();
    let b_shortunion = 185_u16.into_shortunion();
    let modulo_shortunion = 10000_u16.into_shortunion();
    let res_shortunion = a_shortunion.modular_pow(b_shortunion, modulo_shortunion);
    println!("{} ** {} = {} (mod {}), where ** is the power operator", a_shortunion, b_shortunion, res_shortunion, modulo_shortunion);
    assert_eq!(res_shortunion.get(), 8901_u16);

    let c_shortunion = 741_u16.into_shortunion();
    let d_shortunion = 185_u16.into_shortunion();
    let modulo_shortunion = 10000_u16.into_shortunion();
    let res_shortunion = small_uint_modular_pow_func(c_shortunion, d_shortunion, modulo_shortunion);
    println!("{} ** {} = {} (mod {}), where ** is the power operator", c_shortunion, d_shortunion, res_shortunion, modulo_shortunion);
    assert_eq!(res_shortunion.get(), 8901_u16);

    let a_intunion = 78541_u32.into_intunion();
    let b_intunion = 18575_u32.into_intunion();
    let modulo_intunion = 10000000_u32.into_intunion();
    let res_intunion = a_intunion.modular_pow(b_intunion, modulo_intunion);
    println!("{} ** {} = {} (mod {}), where ** is the power operator", a_intunion, b_intunion, res_intunion, modulo_intunion);
    assert_eq!(res_intunion.get(), 4370501_u32);

    let c_intunion = 78541_u32.into_intunion();
    let d_intunion = 18575_u32.into_intunion();
    let modulo_intunion = 10000000_u32.into_intunion();
    let res_intunion = small_uint_modular_pow_func(c_intunion, d_intunion, modulo_intunion);
    println!("{} ** {} = {} (mod {}), where ** is the power operator", c_intunion, d_intunion, res_intunion, modulo_intunion);
    assert_eq!(res_intunion.get(), 4370501_u32);

    let a_longunion = 123456789_u64.into_longunion();
    let b_longunion = 9876543_u64.into_longunion();
    let modulo_longunion = 100000000000_u64.into_longunion();
    let res_longunion = a_longunion.modular_pow(b_longunion, modulo_longunion);
    println!("{} ** {} = {} (mod {}), where ** is the power operator", a_longunion, b_longunion, res_longunion, modulo_longunion);
    assert_eq!(res_longunion.get(), 75010148669_u64);

    let c_longunion = 123456789_u64.into_longunion();
    let d_longunion = 9876543_u64.into_longunion();
    let modulo_longunion = 100000000000_u64.into_longunion();
    let res_longunion = small_uint_modular_pow_func(c_longunion, d_longunion, modulo_longunion);
    println!("{} ** {} = {} (mod {}), where ** is the power operator", c_longunion, d_longunion, res_longunion, modulo_longunion);
    assert_eq!(res_longunion.get(), 75010148669_u64);

    let a_longerunion = 258974_u128.into_longerunion();
    let b_longerunion = 6622882488318_u128.into_longerunion();
    let modulo_longerunion = 4776913109852041418248056622882488319_u128.into_longerunion();
    let res_longerunion = a_longerunion.modular_pow(b_longerunion, modulo_longerunion);
    println!("{} ** {} = {} (mod {}), where ** is the power operator", a_longerunion, b_longerunion, res_longerunion, modulo_longerunion);
    assert_eq!(res_longerunion.get(), 2843356730633772030492705275006525566_u128);

    let c_longerunion = 258974_u128.into_longerunion();
    let d_longerunion = 6622882488318_u128.into_longerunion();
    let modulo_longerunion = 4776913109852041418248056622882488319_u128.into_longerunion();
    let res_longerunion = small_uint_modular_pow_func(c_longerunion, d_longerunion, modulo_longerunion);
    println!("{} ** {} = {} (mod {}), where ** is the power operator", c_longerunion, d_longerunion, res_longerunion, modulo_longerunion);
    assert_eq!(res_longerunion.get(), 2843356730633772030492705275006525566_u128);

    let a_sizeunion = 123456789_usize.into_sizeunion();
    let b_sizeunion = 9876543_usize.into_sizeunion();
    let modulo_sizeunion = 100000000000_usize.into_sizeunion();
    let res_sizeunion = a_sizeunion.modular_pow(b_sizeunion, modulo_sizeunion);
    println!("{} ** {} = {} (mod {}), where ** is the power operator", a_sizeunion, b_sizeunion, res_sizeunion, modulo_sizeunion);
    assert_eq!(res_sizeunion.get(), 75010148669_usize);

    let c_sizeunion = 123456789_usize.into_sizeunion();
    let d_sizeunion = 9876543_usize.into_sizeunion();
    let modulo_sizeunion = 100000000000_usize.into_sizeunion();
    let res_sizeunion = small_uint_modular_pow_func(c_sizeunion, d_sizeunion, modulo_sizeunion);
    println!("{} ** {} = {} (mod {}), where ** is the power operator", c_sizeunion, d_sizeunion, res_sizeunion, modulo_sizeunion);
    assert_eq!(res_sizeunion.get(), 75010148669_usize);
    println!("--------------------------------------");
}

fn small_uint_modular_pow_func<T: cryptocol::number::SmallUInt>(base: T, exp: T, modulo: T) -> T
{
    base.modular_pow(exp, modulo)
}

fn small_uint_log_main()
{
    small_uint_ilog();
    small_uint_ilog10();
    small_uint_ilog2();
}

fn small_uint_ilog()
{
    println!("small_uint_ilog");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = 100_u8;
    let base_u8 = 3_u8;
    let res = small_uint_ilog_func(a_u8, base_u8);
    println!("log_{}( {} ) = {}", base_u8, a_u8, res);
    assert_eq!(res, 4_u32);

    // It will panic.
    // let res = small_uint_ilog_func(0_u8, base_u8);

    // It will panic.
    // let res = small_uint_ilog_func(a_u8, 0_u8);

    // It will panic.
    // let res = small_uint_ilog_func(0_u8, 0_u8);

    // Example for u16
    let a_u16 = 10000_u16;
    let base_u16 = 5_u16;
    let res = small_uint_ilog_func(a_u16, base_u16);
    println!("log_{}( {} ) = {}", base_u16, a_u16, res);
    assert_eq!(res, 5_u32);

    // It will panic.
    // let res = small_uint_ilog_func(0_u16, base_u16);

    // It will panic.
    // let res = small_uint_ilog_func(a_u16, 0_u16);

    // It will panic.
    // let res = small_uint_ilog_func(0_u16, 0_u16);

    // Example for u32
    let a_u32 = 1000000000_u32;
    let base_u32 = 7_u32;
    let res = small_uint_ilog_func(a_u32, base_u32);
    println!("log_{}( {} ) = {}", base_u32, a_u32, res);
    assert_eq!(res, 10_u32);

    // It will panic.
    // let res = small_uint_ilog_func(0_u32, base_u32);

    // It will panic.
    // let res = small_uint_ilog_func(a_u32, 0_u32);

    // It will panic.
    // let res = small_uint_ilog_func(0_u32, 0_u32);

    // Example for u64
    let a_u64 = 10000000000000000000_u64;
    let base_u64 = 11_u64;
    let res = small_uint_ilog_func(a_u64, base_u64);
    println!("log_{}( {} ) = {}", base_u64, a_u64, res);
    assert_eq!(res, 18_u32);

    // It will panic.
    // let res = small_uint_ilog_func(0_u64, base_u64);

    // It will panic.
    // let res = small_uint_ilog_func(a_u64, 0_u64);

    // It will panic.
    // let res = small_uint_ilog_func(0_u64, 0_u64);


    // Example for u128
    let a_u128 = 100000000000000000000000000000000000000_u128;
    let base_u128 = 13_u128;
    let res = small_uint_ilog_func(a_u128, base_u128);
    println!("log_{}( {} ) = {}", base_u128, a_u128, res);
    assert_eq!(res, 34_u32);

    // It will panic.
    // let res = small_uint_ilog_func(0_u128, base_u128);

    // It will panic.
    // let res = small_uint_ilog_func(a_u128, 0_u128);

    // It will panic.
    // let res = small_uint_ilog_func(0_u128, 0_u128);
    // Example for usize
    let a_usize = 10000000000000000000_usize;
    let base_usize = 17_usize;
    let res = small_uint_ilog_func(a_usize, base_usize);
    println!("log_{}( {} ) = {}", base_usize, a_usize, res);
    assert_eq!(res, 15_u32);

    // It will panic.
    // let res = small_uint_ilog_func(0_usize, base_usize);

    // It will panic.
    // let res = small_uint_ilog_func(a_usize, 0_usize);

    // It will panic.
    // let res = small_uint_ilog_func(0_usize, 0_usize);

    // Example for ShortUnion
    let a_shortunion = 10000_u16.into_shortunion();
    let base_shortunion = 5_u16.into_shortunion();
    let res = small_uint_ilog_func(a_shortunion, base_shortunion);
    println!("log_{}( {} ) = {}", base_shortunion, a_shortunion, res);
    assert_eq!(res, 5_u32);

    // It will panic.
    // let res = small_uint_ilog_func(0_u16.into_shortunion(), base_shortunion);

    // It will panic.
    // let res = small_uint_ilog_func(a_shortunion, 0_u16.into_shortunion());

    // It will panic.
    // let res = small_uint_ilog_func(0_u16.into_shortunion(), 0_u16.into_shortunion());

    // Example for IntUnion
    let a_intunion = 1000000000_u32.into_intunion();
    let base_intunion = 7_u32.into_intunion();
    let res = small_uint_ilog_func(a_intunion, base_intunion);
    println!("log_{}( {} ) = {}", base_intunion, a_intunion, res);
    assert_eq!(res, 10_u32);

    // It will panic.
    // let res = small_uint_ilog_func(0_u32.into_shortunion(), base_intunion);

    // It will panic.
    // let res = small_uint_ilog_func(a_intunion, 0_u32.into_shortunion());

    // It will panic.
    // let res = small_uint_ilog_func(0_u32.into_shortunion(), 0_u32.into_shortunion());

    // Example for LongUnion
    let a_longunion = 10000000000000000000_u64.into_longunion();
    let base_longunion = 11_u64.into_longunion();
    let res = small_uint_ilog_func(a_longunion, base_longunion);
    println!("log_{}( {} ) = {}", base_longunion, a_longunion, res);
    assert_eq!(res, 18_u32);

    // It will panic.
    // let res = small_uint_ilog_func(0_u64.into_longunion(), base_longunion);

    // It will panic.
    // let res = small_uint_ilog_func(a_longunion, 0_u64.into_longunion());

    // It will panic.
    // let res = small_uint_ilog_func(0_u64.into_longunion(), 0_u64.into_longunion());

    // Example for LongerUnion
    let a_longerunion = 100000000000000000000000000000000000000_u128.into_longerunion();
    let base_longerunion = 13_u128.into_longerunion();
    let res = small_uint_ilog_func(a_longerunion, base_longerunion);
    println!("log_{}( {} ) = {}", base_longerunion, a_longerunion, res);
    assert_eq!(res, 34_u32);

    // It will panic.
    // let res = small_uint_ilog_func(0_u128.into_longerunion(), base_longerunion);

    // It will panic.
    // let res = small_uint_ilog_func(a_longerunion, 0_u128.into_longerunion());

    // It will panic.
    // let res = small_uint_ilog_func(0_u128.into_longerunion(), 0_u128.into_longerunion());

    // Example for SizeUnion
    let a_sizeunion = 10000000000000000000_usize.into_sizeunion();
    let base_sizeunion = 17_usize.into_sizeunion();
    let res = small_uint_ilog_func(a_sizeunion, base_sizeunion);
    println!("log_{}( {} ) = {}", base_sizeunion, a_sizeunion, res);
    assert_eq!(res, 15_u32);

    // It will panic.
    // let res = small_uint_ilog_func(0_usize.into_sizeunion(), base_sizeunion);

    // It will panic.
    // let res = small_uint_ilog_func(a_sizeunion, 0_usize.into_sizeunion());

    // It will panic.
    // let res = small_uint_ilog_func(0_usize.into_sizeunion(), 0_usize.into_sizeunion());
    println!("--------------------------------------");
}

fn small_uint_ilog_func<T: cryptocol::number::SmallUInt>(num: T, base: T) -> u32
{
    num.ilog(base)
}

fn small_uint_ilog10()
{
    println!("small_uint_ilog10");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = 100_u8;
    let res = small_uint_ilog10_func(a_u8);
    println!("log_10( {} ) = {}", a_u8, res);
    assert_eq!(res, 2_u32);
    // It will panic.
    // let res = small_uint_ilog10_func(0_u8);

    // Example for u16
    let a_u16 = 10000_u16;
    let res = small_uint_ilog10_func(a_u16);
    println!("log_10( {} ) = {}", a_u16, res);
    assert_eq!(res, 4_u32);
    // It will panic.
    // let res = small_uint_ilog10_func(0_u16);

    // Example for u32
    let a_u32 = 1000000000_u32;
    let res = small_uint_ilog10_func(a_u32);
    println!("log_10( {} ) = {}", a_u32, res);
    assert_eq!(res, 9_u32);
    // It will panic.
    // let res = small_uint_ilog10_func(0_u32);

    // Example for u64
    let a_u64 = 10000000000000000000_u64;
    let res = small_uint_ilog10_func(a_u64);
    println!("log_10( {} ) = {}", a_u64, res);
    assert_eq!(res, 19_u32);
    // It will panic.
    // let res = small_uint_ilog10_func(0_u64);

    // Example for u128
    let a_u128 = 100000000000000000000000000000000000000_u128;
    let res = small_uint_ilog10_func(a_u128);
    println!("log_10( {} ) = {}", a_u128, res);
    assert_eq!(res, 38_u32);
    // It will panic.
    // let res = small_uint_ilog10_func(0_u128);

    // Example for usize
    let a_usize = 10000000000000000000_usize;
    let res = small_uint_ilog10_func(a_usize);
    println!("log_10( {} ) = {}", a_usize, res);
    assert_eq!(res, 19_u32);
    // It will panic.
    // let res = small_uint_ilog10_func(0_usize);

    // Example for ShortUnion
    let a_shortunion = 10000_u16.into_shortunion();
    let res = small_uint_ilog10_func(a_shortunion);
    println!("log_10( {} ) = {}", a_shortunion, res);
    assert_eq!(res, 4_u32);
    // It will panic.
    // let res = small_uint_ilog10_func(0_u16.into_shortunion());

    // Example for IntUnion
    let a_intunion = 1000000000_u32.into_intunion();
    let res = small_uint_ilog10_func(a_intunion);
    println!("log_10( {} ) = {}", a_intunion, res);
    assert_eq!(res, 9_u32);
    // It will panic.
    // let res = small_uint_ilog10_func(0_u32.into_shortunion());

    // Example for LongUnion
    let a_longunion = 10000000000000000000_u64.into_longunion();
    let res = small_uint_ilog10_func(a_longunion);
    println!("log_10( {} ) = {}", a_longunion, res);
    assert_eq!(res, 19_u32);
    // It will panic.
    // let res = small_uint_ilog10_func(0_u64.into_longunion());

    // Example for LongerUnion
    let a_longerunion = 100000000000000000000000000000000000000_u128.into_longerunion();
    let res = small_uint_ilog10_func(a_longerunion);
    println!("log_10( {} ) = {}", a_longerunion, res);
    assert_eq!(res, 38_u32);
    // It will panic.
    // let res = small_uint_ilog10_func(0_u128.into_longerunion());

    // Example for SizeUnion
    let a_sizeunion = 10000000000000000000_usize.into_sizeunion();
    let res = small_uint_ilog10_func(a_sizeunion);
    println!("log_10( {} ) = {}", a_sizeunion, res);
    assert_eq!(res, 19_u32);
    // It will panic.
    // let res = small_uint_ilog10_func(0_usize.into_sizeunion());
    println!("--------------------------------------");
}

fn small_uint_ilog10_func<T: cryptocol::number::SmallUInt>(num: T) -> u32
{
    num.ilog10()
}

fn small_uint_ilog2()
{
    println!("small_uint_ilog2");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = 100_u8;
    let res = small_uint_ilog2_func(a_u8);
    println!("log_10( {} ) = {}", a_u8, res);
    assert_eq!(res, 6_u32);
    // It will panic.
    // let res = small_uint_ilog2_func(0_u8);

    // Example for u16
    let a_u16 = 10000_u16;
    let res = small_uint_ilog2_func(a_u16);
    println!("log_10( {} ) = {}", a_u16, res);
    assert_eq!(res, 13_u32);
    // It will panic.
    // let res = small_uint_ilog2_func(0_u16);

    // Example for u32
    let a_u32 = 1000000000_u32;
    let res = small_uint_ilog2_func(a_u32);
    println!("log_10( {} ) = {}", a_u32, res);
    assert_eq!(res, 29_u32);
    // It will panic.
    // let res = small_uint_ilog2_func(0_u32);

    // Example for u64
    let a_u64 = 10000000000000000000_u64;
    let res = small_uint_ilog2_func(a_u64);
    println!("log_10( {} ) = {}", a_u64, res);
    assert_eq!(res, 63_u32);
    // It will panic.
    // let res = small_uint_ilog2_func(0_u64);

    // Example for u128
    let a_u128 = 100000000000000000000000000000000000000_u128;
    let res = small_uint_ilog2_func(a_u128);
    println!("log_10( {} ) = {}", a_u128, res);
    assert_eq!(res, 126_u32);
    // It will panic.
    // let res = small_uint_ilog2_func(0_u128);

    // Example for usize
    let a_usize = 10000000000000000000_usize;
    let res = small_uint_ilog2_func(a_usize);
    println!("log_10( {} ) = {}", a_usize, res);
    assert_eq!(res, 63_u32);
    // It will panic.
    // let res = small_uint_ilog2_func(0_usize);

    // Example for ShortUnion
    let a_shortunion = 10000_u16.into_shortunion();
    let res = small_uint_ilog2_func(a_shortunion);
    println!("log_10( {} ) = {}", a_shortunion, res);
    assert_eq!(res, 13_u32);
    // It will panic.
    // let res = small_uint_ilog2_func(0_u16.into_shortunion());

    // Example for IntUnion
    let a_intunion = 1000000000_u32.into_intunion();
    let res = small_uint_ilog2_func(a_intunion);
    println!("log_10( {} ) = {}", a_intunion, res);
    assert_eq!(res, 29_u32);
    // It will panic.
    // let res = small_uint_ilog2_func(0_u32.into_shortunion());

    // Example for LongUnion
    let a_longunion = 10000000000000000000_u64.into_longunion();
    let res = small_uint_ilog2_func(a_longunion);
    println!("log_10( {} ) = {}", a_longunion, res);
    assert_eq!(res, 63_u32);
    // It will panic.
    // let res = small_uint_ilog2_func(0_u64.into_longunion());

    // Example for LongerUnion
    let a_longerunion = 100000000000000000000000000000000000000_u128.into_longerunion();
    let res = small_uint_ilog2_func(a_longerunion);
    println!("log_10( {} ) = {}", a_longerunion, res);
    assert_eq!(res, 126_u32);
    // It will panic.
    // let res = small_uint_ilog2_func(0_u128.into_longerunion());

    // Example for SizeUnion
    let a_sizeunion = 10000000000000000000_usize.into_sizeunion();
    let res = small_uint_ilog2_func(a_sizeunion);
    println!("log_10( {} ) = {}", a_sizeunion, res);
    assert_eq!(res, 63_u32);
    // It will panic.
    // let res = small_uint_ilog2_func(0_usize.into_sizeunion());
    println!("--------------------------------------");
}

fn small_uint_ilog2_func<T: cryptocol::number::SmallUInt>(num: T) -> u32
{
    num.ilog2()
}

fn small_uint_root_main()
{
    small_uint_isqrt();
    small_uint_root();
}

fn small_uint_isqrt()
{
    println!("small_uint_isqrt");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = 100_u8;
    let res_u8 = small_uint_isqrt_func(a_u8);
    println!("isqrt( {} ) = {}", a_u8, res_u8);
    assert_eq!(res_u8, 10_u8);

    // Example for u16
    let a_u16 = 10000_u16;
    let res_u16 = small_uint_isqrt_func(a_u16);
    println!("isqrt( {} ) = {}", a_u16, res_u16);
    assert_eq!(res_u16, 100_u16);

    // Example for u32
    let a_u32 = 1000000000_u32;
    let res_u32 = small_uint_isqrt_func(a_u32);
    println!("isqrt( {} ) = {}", a_u32, res_u32);
    assert_eq!(res_u32, 31622_u32);

    // Example for u64
    let a_u64 = 10000000000000000000_u64;
    let res_u64 = small_uint_isqrt_func(a_u64);
    println!("isqrt( {} ) = {}", a_u64, res_u64);
    assert_eq!(res_u64, 3162277660_u64);

    // Example for u128
    let a_u128 = 100000000000000000000000000000000000000_u128;
    let res_u128 = small_uint_isqrt_func(a_u128);
    println!("isqrt( {} ) = {}", a_u128, res_u128);
    assert_eq!(res_u128, 10000000000000000000_u128);

    // Example for usize
    let a_usize = 10000000000000000000_usize;
    let res_usize = small_uint_isqrt_func(a_usize);
    println!("isqrt( {} ) = {}", a_usize, res_usize);
    assert_eq!(res_usize, 3162277660_usize);

    // Example for ShortUnion
    let a_shortunion = 10000_u16.into_shortunion();
    let res_shortunion = small_uint_isqrt_func(a_shortunion);
    println!("isqrt( {} ) = {}", a_shortunion, res_shortunion);
    assert_eq!(res_shortunion.get(), 100_u16);

    // Example for IntUnion
    let a_intunion = 1000000000_u32.into_intunion();
    let res_intunion = small_uint_isqrt_func(a_intunion);
    println!("isqrt( {} ) = {}", a_intunion, res_intunion);
    assert_eq!(res_intunion.get(), 31622_u32);

    // Example for LongUnion
    let a_longunion = 10000000000000000000_u64.into_longunion();
    let res_longunion = small_uint_isqrt_func(a_longunion);
    println!("isqrt( {} ) = {}", a_longunion, res_longunion);
    assert_eq!(res_longunion.get(), 3162277660_u64);

    // Example for LongerUnion
    let a_longerunion = 100000000000000000000000000000000000000_u128.into_longerunion();
    let res_longerunion = small_uint_isqrt_func(a_longerunion);
    println!("isqrt( {} ) = {}", a_longerunion, res_longerunion);
    assert_eq!(res_longerunion.get(), 10000000000000000000_u128);

    // Example for SizeUnion
    let a_sizeunion = 10000000000000000000_usize.into_sizeunion();
    let res_sizeunion = small_uint_isqrt_func(a_sizeunion);
    println!("isqrt( {} ) = {}", a_sizeunion, res_sizeunion);
    assert_eq!(res_sizeunion.get(), 3162277660_usize);
    println!("--------------------------------------");
}

fn small_uint_isqrt_func<T: cryptocol::number::SmallUInt>(num: T) -> T
{
    num.isqrt()
}

fn small_uint_root()
{
    println!("small_uint_root");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = 100_u8;
    let res_u8 = a_u8.root(3_u8);
    println!("root_3( {} ) = {}", a_u8, res_u8);
    assert_eq!(res_u8, 4_u8);

    let b_u8 = 100_u8;
    let res_u8 = small_uint_root_func(b_u8, 3_u8);
    println!("root_3( {} ) = {}", b_u8, res_u8);
    assert_eq!(res_u8, 4_u8);

    // Example for u16
    let a_u16 = 10000_u16;
    let res_u16 = a_u16.root(3_u16);
    println!("root_3( {} ) = {}", a_u16, res_u16);
    assert_eq!(res_u16, 21_u16);

    let b_u16 = 10000_u16;
    let res_u16 = small_uint_root_func(b_u16, 3_u16);
    println!("root_3( {} ) = {}", b_u16, res_u16);
    assert_eq!(res_u16, 21_u16);

    // Example for u32
    let a_u32 = 1000000000_u32;
    let res_u32 = a_u32.root(3_u32);
    println!("root_3( {} ) = {}", a_u32, res_u32);
    assert_eq!(res_u32, 1000_u32);

    let b_u32 = 1000000000_u32;
    let res_u32 = small_uint_root_func(b_u32, 3_u32);
    println!("root_3( {} ) = {}", b_u32, res_u32);
    assert_eq!(res_u32, 1000_u32);

    // Example for u64
    let a_u64 = 10000000000000000000_u64;
    let res_u64 = a_u64.root(3_u64);
    println!("root_3( {} ) = {}", a_u64, res_u64);
    assert_eq!(res_u64, 2154434_u64);

    let b_u64 = 10000000000000000000_u64;
    let res_u64 = small_uint_root_func(b_u64, 3_u64);
    println!("root_3( {} ) = {}", b_u64, res_u64);
    assert_eq!(res_u64, 2154434_u64);

    // Example for u128
    let a_u128 = 100000000000000000000000000000000000000_u128;
    let res_u128 = a_u128.root(3_u128);
    println!("root_3( {} ) = {}", a_u128, res_u128);
    assert_eq!(res_u128, 4641588833612_u128);

    let b_u128 = 100000000000000000000000000000000000000_u128;
    let res_u128 = small_uint_root_func(b_u128, 3_u128);
    println!("root_3( {} ) = {}", b_u128, res_u128);
    assert_eq!(res_u128, 4641588833612_u128);

    // Example for usize
    let a_usize = 10000000000000000000_usize;
    let res_usize = a_usize.root(3_usize);
    println!("root_3( {} ) = {}", a_usize, res_usize);
    assert_eq!(res_usize, 2154434_usize);

    let b_usize = 10000000000000000000_usize;
    let res_usize = small_uint_root_func(b_usize, 3_usize);
    println!("root_3( {} ) = {}", b_usize, res_usize);
    assert_eq!(res_usize, 2154434_usize);

    // Example for ShortUnion
    let a_shortunion = 10000_u16.into_shortunion();
    let res_shortunion = a_shortunion.root(3_u16.into_shortunion());
    println!("root_3( {} ) = {}", a_shortunion, res_shortunion);
    assert_eq!(res_shortunion.get(), 21_u16);

    let b_shortunion = 10000_u16.into_shortunion();
    let res_shortunion = small_uint_root_func(b_shortunion, 3_u16.into_shortunion());
    println!("root_3( {} ) = {}", b_shortunion, res_shortunion);
    assert_eq!(res_shortunion.get(), 21_u16);

    // Example for IntUnion
    let a_intunion = 1000000000_u32.into_intunion();
    let res_intunion = a_intunion.root(3_u32.into_intunion());
    println!("root_3( {} ) = {}", a_intunion, res_intunion);
    assert_eq!(res_intunion.get(), 1000_u32);

    let b_intunion = 1000000000_u32.into_intunion();
    let res_intunion = small_uint_root_func(b_intunion, 3_u32.into_intunion());
    println!("root_3( {} ) = {}", b_intunion, res_intunion);
    assert_eq!(res_intunion.get(), 1000_u32);

    // Example for LongUnion
    let a_longunion = 10000000000000000000_u64.into_longunion();
    let res_longunion = a_longunion.root(3_u64.into_longunion());
    println!("root_3( {} ) = {}", a_longunion, res_longunion);
    assert_eq!(res_longunion.get(), 2154434_u64);

    let b_longunion = 10000000000000000000_u64.into_longunion();
    let res_longunion = small_uint_root_func(b_longunion, 3_u64.into_longunion());
    println!("root_3( {} ) = {}", b_longunion, res_longunion);
    assert_eq!(res_longunion.get(), 2154434_u64);

    // Example for LongerUnion
    let a_longerunion = 100000000000000000000000000000000000000_u128.into_longerunion();
    let res_longerunion = a_longerunion.root(3_u128.into_longerunion());
    println!("root_3( {} ) = {}", a_longerunion, res_longerunion);
    assert_eq!(res_longerunion.get(), 4641588833612_u128);

    let b_longerunion = 100000000000000000000000000000000000000_u128.into_longerunion();
    let res_longerunion = small_uint_root_func(b_longerunion, 3_u128.into_longerunion());
    println!("root_3( {} ) = {}", b_longerunion, res_longerunion);
    assert_eq!(res_longerunion.get(), 4641588833612_u128);

    // Example for SizeUnion
    let a_sizeunion = 10000000000000000000_usize.into_sizeunion();
    let res_sizeunion = a_sizeunion.root(3_usize.into_sizeunion());
    println!("root_3( {} ) = {}", a_sizeunion, res_sizeunion);
    assert_eq!(res_sizeunion.get(), 2154434_usize);

    let b_sizeunion = 10000000000000000000_usize.into_sizeunion();
    let res_sizeunion = small_uint_root_func(b_sizeunion, 3_usize.into_sizeunion());
    println!("root_3( {} ) = {}", b_sizeunion, res_sizeunion);
    assert_eq!(res_sizeunion.get(), 2154434_usize);
    println!("--------------------------------------");
}

fn small_uint_root_func<T: cryptocol::number::SmallUInt>(num: T, exp: T) -> T
{
    num.root(exp)
}

fn small_uint_prime_main()
{
    small_uint_test_miller_rabin();
    small_uint_is_prime_using_miller_rabin();
}

fn small_uint_test_miller_rabin()
{
    println!("small_uint_test_miller_rabin");
    use cryptocol::number::SmallUInt;
    use cryptocol::random::Any;
    // Example for u8
    let mut a_u8 = Any::new().random_u8();
    a_u8.set_lsb();
    let prime = a_u8.test_miller_rabin(2_u8);
    if prime
        { println!("It is 75% certain that {} is a prime number.", a_u8); }
    else
        { println!("It is 100% certain that {} is a composite number.", a_u8); }

    let mut b_u8 = Any::new().random_u8();
    b_u8.set_lsb();
    let prime = small_uint_test_miller_rabin_func(b_u8, 2_u8);
    if prime
        { println!("It is 75% certain that {} is a prime number.", b_u8); }
    else
        { println!("It is 100% certain that {} is a composite number.", b_u8); }

    // Example for u16
    let mut a_u16 = Any::new().random_u16();
    a_u16.set_lsb();
    let prime = a_u16.test_miller_rabin(7_u16);
    if prime
        { println!("It is 75% certain that {} is a prime number.", a_u16); }
    else
        { println!("It is 100% certain that {} is a composite number.", a_u16); }

    let mut b_u16 = Any::new().random_u16();
    b_u16.set_lsb();
    let prime = small_uint_test_miller_rabin_func(b_u16, 7_u16);
    if prime
        { println!("It is 75% certain that {} is a prime number.", b_u16); }
    else
        { println!("It is 100% certain that {} is a composite number.", b_u16); }

    // Example for u32
    let mut a_u32 = Any::new().random_u32();
    a_u32.set_lsb();
    let prime = a_u32.test_miller_rabin(61_u32);
    if prime
        { println!("It is 75% certain that {} is a prime number.", a_u32); }
    else
        { println!("It is 100% certain that {} is a composite number.", a_u32); }

    let mut b_u32 = Any::new().random_u32();
    b_u32.set_lsb();
    let prime = small_uint_test_miller_rabin_func(b_u32, 61_u32);
    if prime
        { println!("It is 75% certain that {} is a prime number.", b_u32); }
    else
        { println!("It is 100% certain that {} is a composite number.", b_u32); }

    // Example for u64
    let mut a_u64 = Any::new().random_u64();
    a_u64.set_lsb();
    let prime = a_u64.test_miller_rabin(325_u64);
    if prime
        { println!("It is 75% certain that {} is a prime number.", a_u64); }
    else
        { println!("It is 100% certain that {} is a composite number.", a_u64); }

    let mut b_u64 = Any::new().random_u64();
    b_u64.set_lsb();
    let prime = small_uint_test_miller_rabin_func(b_u64, 325_u64);
    if prime
        { println!("It is 75% certain that {} is a prime number.", b_u64); }
    else
        { println!("It is 100% certain that {} is a composite number.", b_u64); }

    // Example for u128
    let mut a_u128 = Any::new().random_u128();
    a_u128.set_lsb();
    let prime = a_u128.test_miller_rabin(9375_u128);
    if prime
        { println!("It is 75% certain that {} is a prime number.", a_u128); }
    else
        { println!("It is 100% certain that {} is a composite number.", a_u128); }

    let mut b_u128 = Any::new().random_u128();
    b_u128.set_lsb();
    let prime = small_uint_test_miller_rabin_func(b_u128, 9375_u128);
    if prime
        { println!("It is 75% certain that {} is a prime number.", b_u128); }
    else
        { println!("It is 100% certain that {} is a composite number.", b_u128); }

    // Example for usize
    let mut a_usize = Any::new().random_usize();
    a_usize.set_lsb();
    let prime = a_usize.test_miller_rabin(28178_usize);
    if prime
        { println!("It is 75% certain that {} is a prime number.", a_usize); }
    else
        { println!("It is 100% certain that {} is a composite number.", a_usize); }

    let mut b_usize = Any::new().random_usize();
    b_usize.set_lsb();
    let prime = small_uint_test_miller_rabin_func(b_usize, 28178_usize);
    if prime
        { println!("It is 75% certain that {} is a prime number.", b_usize); }
    else
        { println!("It is 100% certain that {} is a composite number.", b_usize); }

    // Example for ShortUnion
    let mut a_shortunion = Any::new().random_u16().into_shortunion();
    a_shortunion.set_lsb();
    let prime = a_shortunion.test_miller_rabin(7_u16.into_shortunion());
    if prime
        { println!("It is 75% certain that {} is a prime number.", a_shortunion); }
    else
        { println!("It is 100% certain that {} is a composite number.", a_shortunion); }

    let mut b_shortunion = Any::new().random_u16().into_shortunion();
    b_shortunion.set_lsb();
    let prime = small_uint_test_miller_rabin_func(b_shortunion, 7_u16.into_shortunion());
    if prime
        { println!("It is 75% certain that {} is a prime number.", b_shortunion); }
    else
        { println!("It is 100% certain that {} is a composite number.", b_shortunion); }

    // Example for IntUnion
    let mut a_intunion = Any::new().random_u32().into_intunion();
    a_intunion.set_lsb();
    let prime = a_intunion.test_miller_rabin(61_u32.into_intunion());
    if prime
        { println!("It is 75% certain that {} is a prime number.", a_intunion); }
    else
        { println!("It is 100% certain that {} is a composite number.", a_intunion); }

    let mut b_intunion = Any::new().random_u32().into_intunion();
    b_intunion.set_lsb();
    let prime = small_uint_test_miller_rabin_func(b_intunion, 61_u32.into_intunion());
    if prime
        { println!("It is 75% certain that {} is a prime number.", b_intunion); }
    else
        { println!("It is 100% certain that {} is a composite number.", b_intunion); }

    // Example for LongUnion
    let mut a_longunion = Any::new().random_u64().into_longunion();
    a_longunion.set_lsb();
    let prime = a_longunion.test_miller_rabin(325_u64.into_longunion());
    if prime
        { println!("It is 75% certain that {} is a prime number.", a_longunion); }
    else
        { println!("It is 100% certain that {} is a composite number.", a_longunion); }

    let mut b_longunion = Any::new().random_u64().into_longunion();
    b_longunion.set_lsb();
    let prime = small_uint_test_miller_rabin_func(b_longunion, 325_u64.into_longunion());
    if prime
        { println!("It is 75% certain that {} is a prime number.", b_longunion); }
    else
        { println!("It is 100% certain that {} is a composite number.", b_longunion); }

    // Example for LongerUnion
    let mut a_longerunion = Any::new().random_u128().into_longerunion();
    a_longerunion.set_lsb();
    let prime = a_longerunion.test_miller_rabin(9375_u128.into_longerunion());
    if prime
        { println!("It is 75% certain that {} is a prime number.", a_longerunion); }
    else
        { println!("It is 100% certain that {} is a composite number.", a_longerunion); }

    let mut b_longerunion = Any::new().random_u128().into_longerunion();
    b_longerunion.set_lsb();
    let prime = small_uint_test_miller_rabin_func(b_longerunion, 9375_u128.into_longerunion());
    if prime
        { println!("It is 75% certain that {} is a prime number.", b_longerunion); }
    else
        { println!("It is 100% certain that {} is a composite number.", b_longerunion); }

    // Example for SizeUnion
    let mut a_sizeunion = Any::new().random_usize().into_sizeunion();
    a_sizeunion.set_lsb();
    let prime = a_sizeunion.test_miller_rabin(28178_usize.into_sizeunion());
    if prime
        { println!("It is 75% certain that {} is a prime number.", a_sizeunion); }
    else
        { println!("It is 100% certain that {} is a composite number.", a_sizeunion); }

    let mut b_sizeunion = Any::new().random_usize().into_sizeunion();
    b_sizeunion.set_lsb();
    let prime = small_uint_test_miller_rabin_func(b_sizeunion, 28178_usize.into_sizeunion());
    if prime
        { println!("It is 75% certain that {} is a prime number.", b_sizeunion); }
    else
        { println!("It is 100% certain that {} is a composite number.", b_sizeunion); }
    println!("--------------------------------------");
}

fn small_uint_test_miller_rabin_func<T: cryptocol::number::SmallUInt>(num: T, a: T) -> bool
{
    num.test_miller_rabin(a)
}

fn small_uint_is_prime_using_miller_rabin()
{
    println!("small_uint_is_prime_using_miller_rabin");
    use cryptocol::number::SmallUInt;
    use cryptocol::random::Any;

    // Example for u8
    let mut a_u8 = Any::new().random_u8();
    a_u8.set_lsb();

    // For performance, if the number is less than 10000,
    // it does not use Miller-Rabin alogrithm but deterministic algorithm
    // so that the argument `repetition` is meaningless.
    let prime = a_u8.is_prime_using_miller_rabin(0_usize);
    if prime
        { println!("It is 100% certain that {} is a prime number.", a_u8); }
    else
        { println!("It is 100% certain that {} is a composite number.", a_u8); }

    let mut b_u8 = Any::new().random_u8();
    b_u8.set_lsb();

    // For performance, if the number is less than 10000,
    // it does not use Miller-Rabin alogrithm but deterministic algorithm
    // so that the argument `repetition` is meaningless.
    let prime = small_uint_is_prime_using_miller_rabin_func(b_u8, 0_usize);
    if prime
        { println!("It is 100% certain that {} is a prime number.", b_u8); }
    else
        { println!("It is 100% certain that {} is a composite number.", b_u8); }

    // Example for u16
    let mut a_u16 = Any::new().random_u16();
    a_u16.set_lsb();

    // If the number is less than u32::MAX (= 4294967295_u32),
    // 3 is enough for `repetition` with 2, 7, and 61
    // for 100% certainty for determination of prime number. 
    let prime = a_u16.is_prime_using_miller_rabin(3_usize);
    if prime
        { println!("It is 100% certain that {} is a prime number.", a_u16); }
    else
        { println!("It is 100% certain that {} is a composite number.", a_u16); }

    let mut b_u16 = Any::new().random_u16();
    b_u16.set_lsb();

    // If the number is less than u32::MAX (= 4294967295_u32),
    // 3 is enough for `repetition` with 2, 7, and 61
    // for 100% certainty for determination of prime number. 
    let prime = small_uint_is_prime_using_miller_rabin_func(b_u16, 3_usize);
    if prime
        { println!("It is 100% certain that {} is a prime number.", b_u16); }
    else
        { println!("It is 100% certain that {} is a composite number.", b_u16); }

    // Example for u32
    let mut a_u32 = Any::new().random_u32();
    a_u32.set_lsb();

    // If the number is less than u32::MAX (= 4294967295_u32),
    // 3 is enough for `repetition` with 2, 7, and 61
    // for 100% certainty for determination of prime number. 
    let prime = a_u32.is_prime_using_miller_rabin(3_usize);
    if prime
        { println!("It is 100% certain that {} is a prime number.", a_u32); }
    else
        { println!("It is 100% certain that {} is a composite number.", a_u32); }

    let mut b_u32 = Any::new().random_u32();
    b_u32.set_lsb();

    // If the number is less than u32::MAX (= 4294967295_u32),
    // 3 is enough for `repetition` with 2, 7, and 61
    // for 100% certainty for determination of prime number. 
    let prime = small_uint_is_prime_using_miller_rabin_func(b_u32, 3_usize);
    if prime
        { println!("It is 100% certain that {} is a prime number.", b_u32); }
    else
        { println!("It is 100% certain that {} is a composite number.", b_u32); }

    // Example for u64
    let mut a_u64 = Any::new().random_u64();
    a_u64.set_lsb();

    // If the number is less than u64::MAX (= 18446744073709551615_u64),
    // 7 is enough for `repetition` with 2_u64, 325, 9375, 28178, 450775, 9780504, and 1795265022
    // for 100% certainty for determination of prime number. 
    let prime = a_u64.is_prime_using_miller_rabin(7_usize);
    if prime
        { println!("It is 100% certain that {} is a prime number.", a_u64); }
    else
        { println!("It is 100% certain that {} is a composite number.", a_u64); }

    let mut b_u64 = Any::new().random_u64();
    b_u64.set_lsb();

    // If the number is less than u64::MAX (= 18446744073709551615_u64),
    // 7 is enough for `repetition` with 2_u64, 325, 9375, 28178, 450775, 9780504, and 1795265022
    // for 100% certainty for determination of prime number. 
    let prime = small_uint_is_prime_using_miller_rabin_func(b_u64, 7_usize);
    if prime
        { println!("It is 100% certain that {} is a prime number.", b_u64); }
    else
        { println!("It is 100% certain that {} is a composite number.", b_u64); }

    // Example for u128
    let mut a_u128 = Any::new().random_u128();
    a_u128.set_lsb();
    let prime = a_u128.is_prime_using_miller_rabin(5_usize);
    if prime
        { println!("It is 99.9% certain that {} is a prime number.", a_u128); }
    else
        { println!("It is 100% certain that {} is a composite number.", a_u128); }

    let mut b_u128 = Any::new().random_u128();
    b_u128.set_lsb();
    let prime = small_uint_is_prime_using_miller_rabin_func(b_u128, 5_usize);
    if prime
        { println!("It is 99.9% certain that {} is a prime number.", b_u128); }
    else
        { println!("It is 100% certain that {} is a composite number.", b_u128); }

    // Example for usize
    let mut a_usize = Any::new().random_usize();
    a_usize.set_lsb();

    // If the number is less than u64::MAX (= 18446744073709551615_u64),
    // 7 is enough for `repetition` with 2_u64, 325, 9375, 28178, 450775, 9780504, and 1795265022
    // for 100% certainty for determination of prime number. 
    let prime = a_usize.is_prime_using_miller_rabin(7_usize);
    if prime
        { println!("It is 100% certain that {} is a prime number.", a_usize); }
    else
        { println!("It is 100% certain that {} is a composite number.", a_usize); }

    let mut b_usize = Any::new().random_usize();
    b_usize.set_lsb();

    // If the number is less than u64::MAX (= 18446744073709551615_u64),
    // 7 is enough for `repetition` with 2_u64, 325, 9375, 28178, 450775, 9780504, and 1795265022
    // for 100% certainty for determination of prime number. 
    let prime = small_uint_is_prime_using_miller_rabin_func(b_usize, 7_usize);
    if prime
        { println!("It is 100% certain that {} is a prime number.", b_usize); }
    else
        { println!("It is 100% certain that {} is a composite number.", b_usize); }

    // Example for ShortUnion
    let mut a_shortunion = Any::new().random_u16().into_shortunion();
    a_shortunion.set_lsb();

    // If the number is less than u32::MAX (= 4294967295_u32),
    // 3 is enough for `repetition` with 2, 7, and 61
    // for 100% certainty for determination of prime number. 
    let prime = a_shortunion.is_prime_using_miller_rabin(3_usize);
    if prime
        { println!("It is 100% certain that {} is a prime number.", a_shortunion); }
    else
        { println!("It is 100% certain that {} is a composite number.", a_shortunion); }

    let mut b_shortunion = Any::new().random_u16().into_shortunion();
    b_shortunion.set_lsb();

    // If the number is less than u32::MAX (= 4294967295_u32),
    // 3 is enough for `repetition` with 2, 7, and 61
    // for 100% certainty for determination of prime number. 
    let prime = small_uint_is_prime_using_miller_rabin_func(b_shortunion, 3_usize);
    if prime
        { println!("It is 100% certain that {} is a prime number.", b_shortunion); }
    else
        { println!("It is 100% certain that {} is a composite number.", b_shortunion); }

    // Example for IntUnion
    let mut a_intunion = Any::new().random_u32().into_intunion();
    a_intunion.set_lsb();

    // If the number is less than u32::MAX (= 4294967295_u32),
    // 3 is enough for `repetition` with 2, 7, and 61
    // for 100% certainty for determination of prime number. 
    let prime = a_intunion.is_prime_using_miller_rabin(3_usize);
    if prime
        { println!("It is 100% certain that {} is a prime number.", a_intunion); }
    else
        { println!("It is 100% certain that {} is a composite number.", a_intunion); }

    let mut b_intunion = Any::new().random_u32().into_intunion();
    b_intunion.set_lsb();

    // If the number is less than u32::MAX (= 4294967295_u32),
    // 3 is enough for `repetition` with 2, 7, and 61
    // for 100% certainty for determination of prime number. 
    let prime = small_uint_is_prime_using_miller_rabin_func(b_intunion, 3_usize);
    if prime
        { println!("It is 100% certain that {} is a prime number.", b_intunion); }
    else
        { println!("It is 100% certain that {} is a composite number.", b_intunion); }

    // Example for LongUnion
    let mut a_longunion = Any::new().random_u64().into_longunion();
    a_longunion.set_lsb();

    // If the number is less than u64::MAX (= 18446744073709551615_u64),
    // 7 is enough for `repetition` with 2_u64, 325, 9375, 28178, 450775, 9780504, and 1795265022
    // for 100% certainty for determination of prime number. 
    let prime = a_longunion.is_prime_using_miller_rabin(7_usize);
    if prime
        { println!("It is 100% certain that {} is a prime number.", a_longunion); }
    else
        { println!("It is 100% certain that {} is a composite number.", a_longunion); }

    let mut b_longunion = Any::new().random_u64().into_longunion();
    b_longunion.set_lsb();

    // If the number is less than u64::MAX (= 18446744073709551615_u64),
    // 7 is enough for `repetition` with 2_u64, 325, 9375, 28178, 450775, 9780504, and 1795265022
    // for 100% certainty for determination of prime number. 
    let prime = small_uint_is_prime_using_miller_rabin_func(b_longunion, 7_usize);
    if prime
        { println!("It is 100% certain that {} is a prime number.", b_longunion); }
    else
        { println!("It is 100% certain that {} is a composite number.", b_longunion); }

    // Example for LongerUnion
    let mut a_longerunion = Any::new().random_u128().into_longerunion();
    a_longerunion.set_lsb();
    let prime = a_longerunion.is_prime_using_miller_rabin(5_usize);
    if prime
        { println!("It is 99.9% certain that {} is a prime number.", a_longerunion); }
    else
        { println!("It is 100% certain that {} is a composite number.", a_longerunion); }

    let mut b_longerunion = Any::new().random_u128().into_longerunion();
    b_longerunion.set_lsb();
    let prime = small_uint_is_prime_using_miller_rabin_func(b_longerunion, 5_usize);
    if prime
        { println!("It is 99.9% certain that {} is a prime number.", b_longerunion); }
    else
        { println!("It is 100% certain that {} is a composite number.", b_longerunion); }

    // Example for SizeUnion
    let mut a_sizeunion = Any::new().random_usize().into_sizeunion();
    a_sizeunion.set_lsb();

    // If the number is less than u64::MAX (= 18446744073709551615_u64),
    // 7 is enough for `repetition` with 2_u64, 325, 9375, 28178, 450775, 9780504, and 1795265022
    // for 100% certainty for determination of prime number. 
    let prime = a_sizeunion.is_prime_using_miller_rabin(7_usize);
    if prime
        { println!("It is 100% certain that {} is a prime number.", a_sizeunion); }
    else
        { println!("It is 100% certain that {} is a composite number.", a_sizeunion); }

    let mut b_sizeunion = Any::new().random_usize().into_sizeunion();
    b_sizeunion.set_lsb();

    // If the number is less than u64::MAX (= 18446744073709551615_u64),
    // 7 is enough for `repetition` with 2_u64, 325, 9375, 28178, 450775, 9780504, and 1795265022
    // for 100% certainty for determination of prime number. 
    let prime = small_uint_is_prime_using_miller_rabin_func(b_sizeunion, 7_usize);
    if prime
        { println!("It is 100% certain that {} is a prime number.", b_sizeunion); }
    else
        { println!("It is 100% certain that {} is a composite number.", b_sizeunion); }
    println!("--------------------------------------");
}

fn small_uint_is_prime_using_miller_rabin_func<T: cryptocol::number::SmallUInt>(num: T, repetition: usize) -> bool
{
    num.is_prime_using_miller_rabin(repetition)
}

fn small_uint_bits_operation()
{
    small_uint_reverse_bits();
    // small_uint_reverse_bits_assign();
    small_uint_rotate_left();
    small_uint_rotate_right();
    small_uint_count_ones();
    small_uint_count_zeros();
    small_uint_leading_ones();
    small_uint_leading_zeros();
    small_uint_trailing_ones();
    small_uint_trailing_zeros();
    small_uint_set_msb();
    small_uint_set_lsb();
    small_uint_generate_check_bits();
    small_uint_generate_check_bits_();
    small_uint_is_odd();
    small_uint_is_even();
    small_uint_is_msb_set();
    small_uint_is_bit_set();
    small_uint_is_bit_set_();
}

fn small_uint_reverse_bits()
{
    println!("small_uint_reverse_bits");
    use cryptocol::number::SmallUInt;
    // Examples for u8
    let c_u8 = 0b10110011_u8;
    let d_u8 = small_uint_reverse_bits_func(c_u8);
    println!("{:08b} -> {:08b}", c_u8, d_u8);
    assert_eq!(d_u8, 0b11001101_u8);

    // Examples for u16
    let c_u16 = 0b1011001110001111_u16;
    let d_u16 = small_uint_reverse_bits_func(c_u16);
    println!("{:016b} -> {:016b}", c_u16, d_u16);
    assert_eq!(d_u16, 0b1111000111001101_u16);

    // Examples for u32
    let c_u32 = 0b10110011100011110000111110000011_u32;
    let d_u32 = small_uint_reverse_bits_func(c_u32);
    println!("{:032b} -> {:032b}", c_u32, d_u32);
    assert_eq!(d_u32, 0b11000001111100001111000111001101_u32);

    // Examples for u64
    let c_u64 = 0b1011001110001111000011111000001111110000001111111000000011111111_u64;
    let d_u64 = small_uint_reverse_bits_func(c_u64);
    println!("{:064b} -> {:064b}", c_u64, d_u64);
    assert_eq!(d_u64, 0b1111111100000001111111000000111111000001111100001111000111001101_u64);

    // Examples for u128
    let c_u128 = 0b10110011100011110000111110000011111100000011111110000000111111110000000011111111100000000011111111110000000000111111111110000000_u128;
    let d_u128 = small_uint_reverse_bits_func(c_u128);
    println!("{:0128b} -> {:0128b}", c_u128, d_u128);
    assert_eq!(d_u128, 0b00000001111111111100000000001111111111000000000111111111000000001111111100000001111111000000111111000001111100001111000111001101_u128);

    // Examples for usize
    let c_usize = 0b1011001110001111000011111000001111110000001111111000000011111111_usize;
    let d_usize = small_uint_reverse_bits_func(c_usize);
    println!("{:064b} -> {:064b}", c_usize, d_usize);
    assert_eq!(d_usize, 0b1111111100000001111111000000111111000001111100001111000111001101_usize);

    // Examples for ShortUnion
    let c_shortunion = 0b1011001110001111_u16.into_shortunion();
    let d_shortunion = small_uint_reverse_bits_func(c_shortunion);
    println!("{:016b} -> {:016b}", c_shortunion.get(), d_shortunion.get());
    assert_eq!(d_shortunion.get(), 0b1111000111001101_u16);

    // Examples for IntUnion
    let c_intunion = 0b10110011100011110000111110000011_u32.into_intunion();
    let d_intunion = small_uint_reverse_bits_func(c_intunion);
    println!("{:032b} -> {:032b}", c_intunion.get(), d_intunion.get());
    assert_eq!(d_intunion.get(), 0b11000001111100001111000111001101_u32);

    // Examples for LongUnion
    let c_longunion = 0b1011001110001111000011111000001111110000001111111000000011111111_u64.into_longunion();
    let d_longunion = small_uint_reverse_bits_func(c_longunion);
    println!("{:064b} -> {:064b}", c_longunion.get(), d_longunion.get());
    assert_eq!(d_longunion.get(), 0b1111111100000001111111000000111111000001111100001111000111001101_u64);

    // Examples for LongerUnion
    let c_longerunion = 0b10110011100011110000111110000011111100000011111110000000111111110000000011111111100000000011111111110000000000111111111110000000_u128.into_longerunion();
    let d_longerunion = small_uint_reverse_bits_func(c_longerunion);
    println!("{:0128b} -> {:0128b}", c_longerunion.get(), d_longerunion.get());
    assert_eq!(d_longerunion.get(), 0b00000001111111111100000000001111111111000000000111111111000000001111111100000001111111000000111111000001111100001111000111001101_u128);

    // Examples for SizeUnion
    let c_sizeunion = 0b1011001110001111000011111000001111110000001111111000000011111111_usize.into_sizeunion();
    let d_sizeunion = small_uint_reverse_bits_func(c_sizeunion);
    println!("{:064b} -> {:064b}", c_sizeunion.get(), d_sizeunion.get());
    assert_eq!(d_sizeunion.get(), 0b1111111100000001111111000000111111000001111100001111000111001101_usize);
    println!("--------------------------------------");
}

fn small_uint_reverse_bits_func<T: cryptocol::number::SmallUInt>(num: T) -> T
{
    num.reverse_bits()
}

/*
fn small_uint_reverse_bits_assign()
{
    println!("small_uint_reverse_bits_assign");
    use cryptocol::number::SmallUInt;
    // Examples for u8
    let mut a_u8 = 0b10110011_u8;
    println!("origianl a_u8 : {:08b}", a_u8);
    a_u8.reverse_bits_assign();
    println!("after a_u8.reverse_bits_assign : {:08b}", a_u8);
    assert_eq!(a_u8, 0b11001101_u8);

    let mut b_u8 = 0b10110011_u8;
    println!("origianl b_u8 : {:08b}", b_u8);
    small_uint_reverse_bits_assign_func(&mut b_u8);
    println!("after b_u8.reverse_bits_assign : {:08b}", b_u8);
    assert_eq!(b_u8, 0b11001101_u8);

    // Examples for u16
    let mut a_u16 = 0b1011001110001111_u16;
    println!("origianl a_u16 : {:016b}", a_u16);
    a_u16.reverse_bits_assign();
    println!("after a_u16.reverse_bits_assign : {:016b}",  a_u16);
    assert_eq!(a_u16, 0b1111000111001101_u16);

    let mut b_u16 = 0b1011001110001111_u16;
    println!("origianl b_u16 : {:016b}", b_u16);
    small_uint_reverse_bits_assign_func(&mut b_u16);
    println!("after b_u16.reverse_bits_assign : {:016b}",  b_u16);
    assert_eq!(b_u16, 0b1111000111001101_u16);

    // Examples for u32
    let mut a_u32 = 0b10110011100011110000111110000011_u32;
    println!("origianl a_u32 : {:032b}", a_u32);
    a_u32.reverse_bits_assign();
    println!("after a_u32.reverse_bits_assign : {:032b}",  a_u32);
    assert_eq!(a_u32, 0b11000001111100001111000111001101_u32);

    let mut b_u32 = 0b10110011100011110000111110000011_u32;
    println!("origianl b_u32 : {:032b}", b_u32);
    small_uint_reverse_bits_assign_func(&mut b_u32);
    println!("after b_u32.reverse_bits_assign : {:032b}",  b_u32);
    assert_eq!(b_u32, 0b11000001111100001111000111001101_u32);

    // Examples for u64
    let mut a_u64 = 0b1011001110001111000011111000001111110000001111111000000011111111_u64;
    println!("origianl a_u64 : {:064b}", a_u64);
    a_u64.reverse_bits_assign();
    println!("after a_u64.reverse_bits_assign : {:064b}", a_u64);
    assert_eq!(a_u64, 0b1111111100000001111111000000111111000001111100001111000111001101_u64);

    let mut b_u64 = 0b1011001110001111000011111000001111110000001111111000000011111111_u64;
    println!("origianl b_u64 : {:064b}", b_u64);
    small_uint_reverse_bits_assign_func(&mut b_u64);
    println!("after b_u64.reverse_bits_assign : {:064b}", b_u64);
    assert_eq!(b_u64, 0b1111111100000001111111000000111111000001111100001111000111001101_u64);

    // Examples for u128
    let mut a_u128 = 0b10110011100011110000111110000011111100000011111110000000111111110000000011111111100000000011111111110000000000111111111110000000_u128;
    println!("origianl a_u128 : {:0128b}", a_u128);
    a_u128.reverse_bits_assign();
    println!("after a_u128.reverse_bits_assign : {:0128b}", a_u128);
    assert_eq!(a_u128, 0b00000001111111111100000000001111111111000000000111111111000000001111111100000001111111000000111111000001111100001111000111001101_u128);

    let mut b_u128 = 0b10110011100011110000111110000011111100000011111110000000111111110000000011111111100000000011111111110000000000111111111110000000_u128;
    println!("origianl b_u128 : {:0128b}", b_u128);
    small_uint_reverse_bits_assign_func(&mut b_u128);
    println!("after b_u128.reverse_bits_assign : {:0128b}", b_u128);
    assert_eq!(b_u128, 0b00000001111111111100000000001111111111000000000111111111000000001111111100000001111111000000111111000001111100001111000111001101_u128);

    // Examples for usize
    let mut a_usize = 0b1011001110001111000011111000001111110000001111111000000011111111_usize;
    println!("origianl a_usize : {:064b}", a_usize);
    a_usize.reverse_bits_assign();
    println!("after a_usize.reverse_bits_assign : {:064b}", a_usize);
    assert_eq!(a_usize, 0b1111111100000001111111000000111111000001111100001111000111001101_usize);

    let mut b_usize = 0b1011001110001111000011111000001111110000001111111000000011111111_usize;
    println!("origianl b_usize : {:0128b}", b_usize);
    small_uint_reverse_bits_assign_func(&mut b_usize);
    println!("after b_usize.reverse_bits_assign : {:0128b}", b_usize);
    assert_eq!(b_usize, 0b1111111100000001111111000000111111000001111100001111000111001101_usize);

    // Examples for ShortUnion
    let mut a_shortunion = 0b1011001110001111_u16.into_shortunion();
    println!("origianl a_shortunion : {:016b}", a_shortunion.get());
    a_shortunion.reverse_bits_assign();
    println!("after a_shortunion.reverse_bits_assign : {:016b}", a_shortunion.get());
    assert_eq!(a_shortunion.get(), 0b1111000111001101_u16);

    let mut b_shortunion = 0b1011001110001111_u16.into_shortunion();
    println!("origianl b_shortunion : {:016b}", b_shortunion.get());
    small_uint_reverse_bits_assign_func(&mut b_shortunion);
    println!("after b_shortunion.reverse_bits_assign : {:016b}", b_shortunion.get());
    assert_eq!(b_shortunion.get(), 0b1111000111001101_u16);

    // Examples for IntUnion
    let mut a_intunion = 0b10110011100011110000111110000011_u32.into_intunion();
    println!("origianl a_intunion : {:032b}", a_intunion.get());
    a_intunion.reverse_bits_assign();
    println!("after a_intunion.reverse_bits_assign : {:032b}", a_intunion.get());
    assert_eq!(a_intunion.get(), 0b11000001111100001111000111001101_u32);

    let mut b_intunion = 0b10110011100011110000111110000011_u32.into_intunion();
    println!("origianl b_intunion : {:032b}", b_intunion.get());
    small_uint_reverse_bits_assign_func(&mut b_intunion);
    println!("after b_intunion.reverse_bits_assign : {:032b}", b_intunion.get());
    assert_eq!(b_intunion.get(), 0b11000001111100001111000111001101_u32);

    // Examples for LongUnion
    let mut a_longunion = 0b1011001110001111000011111000001111110000001111111000000011111111_u64.into_longunion();
    println!("origianl a_longunion : {:064b}", a_longunion.get());
    a_longunion.reverse_bits_assign();
    println!("after a_longunion.reverse_bits_assign : {:064b}", a_longunion.get());
    assert_eq!(a_longunion.get(), 0b1111111100000001111111000000111111000001111100001111000111001101_u64);

    let mut b_longunion = 0b1011001110001111000011111000001111110000001111111000000011111111_u64.into_longunion();
    println!("origianl b_longunion : {:064b}", b_longunion.get());
    small_uint_reverse_bits_assign_func(&mut b_longunion);
    println!("after b_longunion.reverse_bits_assign : {:064b}", b_longunion.get());
    assert_eq!(b_longunion.get(), 0b1111111100000001111111000000111111000001111100001111000111001101_u64);

    // Examples for LongerUnion
    let mut a_longerunion = 0b10110011100011110000111110000011111100000011111110000000111111110000000011111111100000000011111111110000000000111111111110000000_u128.into_longerunion();
    println!("origianl a_longerunion : {:0128b}", a_longerunion.get());
    a_longerunion.reverse_bits_assign();
    println!("after a_longerunion.reverse_bits_assign : {:0128b}", a_longerunion.get());
    assert_eq!(a_longerunion.get(), 0b00000001111111111100000000001111111111000000000111111111000000001111111100000001111111000000111111000001111100001111000111001101_u128);

    let mut b_longerunion = 0b10110011100011110000111110000011111100000011111110000000111111110000000011111111100000000011111111110000000000111111111110000000_u128.into_longerunion();
    println!("origianl b_longerunion : {:0128b}", b_longerunion.get());
    small_uint_reverse_bits_assign_func(&mut b_longerunion);
    println!("after b_longerunion.reverse_bits_assign : {:0128b}", b_longerunion.get());
    assert_eq!(b_longerunion.get(), 0b00000001111111111100000000001111111111000000000111111111000000001111111100000001111111000000111111000001111100001111000111001101_u128);

    // Examples for SizeUnion
    let mut a_sizeunion = 0b1011001110001111000011111000001111110000001111111000000011111111_usize.into_sizeunion();
    println!("origianl a_sizeunion : {:064b}", a_sizeunion.get());
    a_sizeunion.reverse_bits_assign();
    println!("after a_sizeunion.reverse_bits_assign : {:064b}", a_sizeunion.get());
    assert_eq!(a_sizeunion.get(), 0b1111111100000001111111000000111111000001111100001111000111001101_usize);

    let mut b_sizeunion = 0b1011001110001111000011111000001111110000001111111000000011111111_usize.into_sizeunion();
    println!("origianl b_sizeunion : {:064b}", b_sizeunion.get());
    small_uint_reverse_bits_assign_func(&mut b_sizeunion);
    println!("after b_sizeunion.reverse_bits_assign : {:064b}", b_sizeunion.get());
    assert_eq!(b_sizeunion.get(), 0b1111111100000001111111000000111111000001111100001111000111001101_usize);
    println!("--------------------------------------");
}

fn small_uint_reverse_bits_assign_func<T: cryptocol::number::SmallUInt>(num: &mut T)
{
    num.reverse_bits_assign();
}
*/

fn small_uint_rotate_left()
{
    println!("small_uint_rotate_left");
    use cryptocol::number::SmallUInt;
    // Examples for u8
    let a_u8 = 0b10110011_u8;
    let b_u8 = small_uint_rotate_left_func(a_u8, 2);
    println!("{:08b} -> {:08b}", a_u8, b_u8);
    assert_eq!(b_u8, 0b11001110_u8);

    // Examples for u16
    let a_u16 = 0b1011001110001111_u16;
    let b_u16 = small_uint_rotate_left_func(a_u16, 4);
    println!("{:016b} -> {:016b}", a_u16, b_u16);
    assert_eq!(b_u16, 0b0011100011111011_u16);

    // Examples for u32
    let a_u32 = 0b10110011100011110000111110000011_u32;
    let b_u32 = small_uint_rotate_left_func(a_u32, 8);
    println!("{:032b} -> {:032b}", a_u32, b_u32);
    assert_eq!(b_u32, 0b10001111000011111000001110110011_u32);

    // Examples for u64
    let a_u64 = 0b1011001110001111000011111000001111110000001111111000000011111111_u64;
    let b_u64 = small_uint_rotate_left_func(a_u64, 16);
    println!("{:064b} -> {:064b}", a_u64, b_u64);
    assert_eq!(b_u64, 0b0000111110000011111100000011111110000000111111111011001110001111_u64);

    // Examples for u128
    let a_u128 = 0b10110011100011110000111110000011111100000011111110000000111111110000000011111111100000000011111111110000000000111111111110000000_u128;
    let b_u128 = small_uint_rotate_left_func(a_u128, 32);
    println!("{:0128b} -> {:0128b}", a_u128, b_u128);
    assert_eq!(b_u128, 0b11110000001111111000000011111111000000001111111110000000001111111111000000000011111111111000000010110011100011110000111110000011_u128);

    // Examples for usize
    let a_usize = 0b1011001110001111000011111000001111110000001111111000000011111111_usize;
    let b_usize = small_uint_rotate_left_func(a_usize, 16);
    println!("{:064b} -> {:064b}", a_usize, b_usize);
    assert_eq!(b_usize, 0b0000111110000011111100000011111110000000111111111011001110001111_usize);

    // Examples for ShortUnion
    let a_shortunion = 0b1011001110001111_u16.into_shortunion();
    let b_shortunion = small_uint_rotate_left_func(a_shortunion, 4);
    println!("{:016b} -> {:016b}", a_shortunion.get(), b_shortunion.get());
    assert_eq!(b_shortunion.get(), 0b0011100011111011_u16);

    // Examples for IntUnion
    let a_intunion = 0b10110011100011110000111110000011_u32.into_intunion();
    let b_intunion = small_uint_rotate_left_func(a_intunion, 8);
    println!("{:032b} -> {:032b}", a_intunion.get(), b_intunion.get());
    assert_eq!(b_intunion.get(), 0b10001111000011111000001110110011_u32);

    // Examples for LongUnion
    let a_longunion = 0b1011001110001111000011111000001111110000001111111000000011111111_u64.into_longunion();
    let b_longunion = small_uint_rotate_left_func(a_longunion, 16);
    println!("{:064b} -> {:064b}", a_longunion.get(), b_longunion.get());
    assert_eq!(b_longunion.get(), 0b0000111110000011111100000011111110000000111111111011001110001111_u64);

    // Examples for LongerUnion
    let a_longerunion = 0b10110011100011110000111110000011111100000011111110000000111111110000000011111111100000000011111111110000000000111111111110000000_u128.into_longerunion();
    let b_longerunion = small_uint_rotate_left_func(a_longerunion, 32);
    println!("{:0128b} -> {:0128b}", a_longerunion.get(), b_longerunion.get());
    assert_eq!(b_longerunion.get(), 0b11110000001111111000000011111111000000001111111110000000001111111111000000000011111111111000000010110011100011110000111110000011_u128);

    // Examples for SizeUnion
    let a_sizeunion = 0b1011001110001111000011111000001111110000001111111000000011111111_usize.into_sizeunion();
    let b_sizeunion = small_uint_rotate_left_func(a_sizeunion, 16);
    println!("{:064b} -> {:064b}", a_sizeunion.get(), b_sizeunion.get());
    assert_eq!(b_sizeunion.get(), 0b0000111110000011111100000011111110000000111111111011001110001111_usize);
    println!("--------------------------------------");
}

fn small_uint_rotate_left_func<T: cryptocol::number::SmallUInt>(num: T, rl: u32) -> T
{
    num.rotate_left(rl)
}

fn small_uint_rotate_right()
{
    println!("small_uint_rotate_right");
    use cryptocol::number::SmallUInt;
    // Examples for u8
    let a_u8 = 0b10110011_u8;
    let b_u8 = small_uint_rotate_right_func(a_u8, 2);
    println!("{:08b} -> {:08b}", a_u8, b_u8);
    assert_eq!(b_u8, 0b11101100_u8);

    // Examples for u16
    let a_u16 = 0b1011001110001111_u16;
    let b_u16 = small_uint_rotate_right_func(a_u16, 4);
    println!("{:016b} -> {:016b}", a_u16, b_u16);
    assert_eq!(b_u16, 0b1111101100111000_u16);

    // Examples for u32
    let a_u32 = 0b10110011100011110000111110000011_u32;
    let b_u32 = small_uint_rotate_right_func(a_u32, 8);
    println!("{:032b} -> {:032b}", a_u32, b_u32);
    assert_eq!(b_u32, 0b10000011101100111000111100001111_u32);

    // Examples for u64
    let a_u64 = 0b1011001110001111000011111000001111110000001111111000000011111111_u64;
    let b_u64 = small_uint_rotate_right_func(a_u64, 16);
    println!("{:064b} -> {:064b}", a_u64, b_u64);
    assert_eq!(b_u64, 0b1000000011111111101100111000111100001111100000111111000000111111_u64);

    // Examples for u128
    let a_u128 = 0b10110011100011110000111110000011111100000011111110000000111111110000000011111111100000000011111111110000000000111111111110000000_u128;
    let b_u128 = small_uint_rotate_right_func(a_u128, 32);
    println!("{:0128b} -> {:0128b}", a_u128, b_u128);
    assert_eq!(b_u128, 0b11110000000000111111111110000000101100111000111100001111100000111111000000111111100000001111111100000000111111111000000000111111_u128);

    // Examples for usize
    let a_usize = 0b1011001110001111000011111000001111110000001111111000000011111111_usize;
    let b_usize = small_uint_rotate_right_func(a_usize, 16);
    println!("{:064b} -> {:064b}", a_usize, b_usize);
    assert_eq!(b_usize, 0b1000000011111111101100111000111100001111100000111111000000111111_usize);

    // Examples for ShortUnion
    let a_shortunion = 0b1011001110001111_u16.into_shortunion();
    let b_shortunion = small_uint_rotate_right_func(a_shortunion, 4);
    println!("{:016b} -> {:016b}", a_shortunion.get(), b_shortunion.get());
    assert_eq!(b_shortunion.get(), 0b1111101100111000_u16);

    // Examples for IntUnion
    let a_intunion = 0b10110011100011110000111110000011_u32.into_intunion();
    let b_intunion = small_uint_rotate_right_func(a_intunion, 8);
    println!("{:032b} -> {:032b}", a_intunion.get(), b_intunion.get());
    assert_eq!(b_intunion.get(), 0b10000011101100111000111100001111_u32);

    // Examples for LongUnion
    let a_longunion = 0b1011001110001111000011111000001111110000001111111000000011111111_u64.into_longunion();
    let b_longunion = small_uint_rotate_right_func(a_longunion, 16);
    println!("{:064b} -> {:064b}", a_longunion.get(), b_longunion.get());
    assert_eq!(b_longunion.get(), 0b1000000011111111101100111000111100001111100000111111000000111111_u64);

    // Examples for LongerUnion
    let a_longerunion = 0b10110011100011110000111110000011111100000011111110000000111111110000000011111111100000000011111111110000000000111111111110000000_u128.into_longerunion();
    let b_longerunion = small_uint_rotate_right_func(a_longerunion, 32);
    println!("{:0128b} -> {:0128b}", a_longerunion.get(), b_longerunion.get());
    assert_eq!(b_longerunion.get(), 0b11110000000000111111111110000000101100111000111100001111100000111111000000111111100000001111111100000000111111111000000000111111_u128);

    // Examples for SizeUnion
    let a_sizeunion = 0b1011001110001111000011111000001111110000001111111000000011111111_usize.into_sizeunion();
    let b_sizeunion = small_uint_rotate_right_func(a_sizeunion, 16);
    println!("{:064b} -> {:064b}", a_sizeunion.get(), b_sizeunion.get());
    assert_eq!(b_sizeunion.get(), 0b1000000011111111101100111000111100001111100000111111000000111111_usize);
    println!("--------------------------------------");
}

fn small_uint_rotate_right_func<T: cryptocol::number::SmallUInt>(num: T, rr: u32) -> T
{
    num.rotate_right(rr)
}

fn small_uint_count_ones()
{
    println!("small_uint_count_ones");
    use cryptocol::number::SmallUInt;
    // Examples for u8
    let a_u8 = 0b10110011_u8;
    let ones = small_uint_count_ones_func(a_u8);
    println!("The number of ones of {:08b} is {}.", a_u8, ones);
    assert_eq!(ones, 5_u32);

    // Examples for u16
    let a_u16 = 0b1011001110001111_u16;
    let ones = small_uint_count_ones_func(a_u16);
    println!("The number of ones of {:016b} is {}.", a_u16, ones);
    assert_eq!(ones, 10_u32);

    // Examples for u32
    let a_u32 = 0b10110011100011110000111110000011_u32;
    let ones = small_uint_count_ones_func(a_u32);
    println!("The number of ones of {:032b} is {}.", a_u32, ones);
    assert_eq!(ones, 17_u32);

    // Examples for u64
    let a_u64 = 0b1011001110001111000011111000001111110000001111111000000011111111_u64;
    let ones = small_uint_count_ones_func(a_u64);
    println!("The number of ones of {:064b} is {}.", a_u64, ones);
    assert_eq!(ones, 36_u32);

    // Examples for u128
    let a_u128 = 0b10110011100011110000111110000011111100000011111110000000111111110000000011111111100000000011111111110000000000111111111110000000_u128;
    let ones = small_uint_count_ones_func(a_u128);
    println!("The number of ones of {:0128b} is {}.", a_u128, ones);
    assert_eq!(ones, 66_u32);

    // Examples for usize
    let a_usize = 0b1011001110001111000011111000001111110000001111111000000011111111_usize;
    let ones = small_uint_count_ones_func(a_usize);
    println!("The number of ones of {:064b} is {}.", a_usize, ones);
    assert_eq!(ones, 36_u32);

    // Examples for ShortUnion
    let a_shortunion = 0b1011001110001111_u16.into_shortunion();
    let ones = small_uint_count_ones_func(a_shortunion);
    println!("The number of ones of {:016b} is {}.", a_shortunion.get(), ones);
    assert_eq!(ones, 10_u32);

    // Examples for IntUnion
    let a_intunion = 0b10110011100011110000111110000011_u32.into_intunion();
    let ones = small_uint_count_ones_func(a_intunion);
    println!("The number of ones of {:032b} is {}.", a_intunion.get(), ones);
    assert_eq!(ones, 17_u32);

    // Examples for LongUnion
    let a_longunion = 0b1011001110001111000011111000001111110000001111111000000011111111_u64.into_longunion();
    let ones = small_uint_count_ones_func(a_longunion);
    println!("The number of ones of {:064b} is {}.", a_longunion.get(), ones);
    assert_eq!(ones, 36_u32);

    // Examples for LongerUnion
    let a_longerunion = 0b10110011100011110000111110000011111100000011111110000000111111110000000011111111100000000011111111110000000000111111111110000000_u128.into_longerunion();
    let ones = small_uint_count_ones_func(a_longerunion);
    println!("The number of ones of {:0128b} is {}.", a_longerunion.get(), ones);
    assert_eq!(ones, 66_u32);

    // Examples for SizeUnion
    let a_sizeunion = 0b1011001110001111000011111000001111110000001111111000000011111111_usize.into_sizeunion();
    let ones = small_uint_count_ones_func(a_sizeunion);
    println!("The number of ones of {:064b} is {}.", a_sizeunion.get(), ones);
    assert_eq!(ones, 36_u32);
    println!("--------------------------------------");
}

fn small_uint_count_ones_func<T: cryptocol::number::SmallUInt>(num: T) -> u32
{
    num.count_ones()
}

fn small_uint_count_zeros()
{
    println!("small_uint_count_zeros");
    use cryptocol::number::SmallUInt;
    // Examples for u8
    let a_u8 = 0b10110011_u8;
    let zeros = small_uint_count_zeros_func(a_u8);
    println!("The number of zeros of {:08b} is {}.", a_u8, zeros);
    assert_eq!(zeros, 3_u32);

    // Examples for u16
    let a_u16 = 0b1011001110001111_u16;
    let zeros = small_uint_count_zeros_func(a_u16);
    println!("The number of zeros of {:016b} is {}.", a_u16, zeros);
    assert_eq!(zeros, 6_u32);

    // Examples for u32
    let a_u32 = 0b10110011100011110000111110000011_u32;
    let zeros = small_uint_count_zeros_func(a_u32);
    println!("The number of zeros of {:032b} is {}.", a_u32, zeros);
    assert_eq!(zeros, 15_u32);

    // Examples for u64
    let a_u64 = 0b1011001110001111000011111000001111110000001111111000000011111111_u64;
    let zeros = small_uint_count_zeros_func(a_u64);
    println!("The number of zeros of {:064b} is {}.", a_u64, zeros);
    assert_eq!(zeros, 28_u32);

    // Examples for u128
    let a_u128 = 0b10110011100011110000111110000011111100000011111110000000111111110000000011111111100000000011111111110000000000111111111110000000_u128;
    let zeros = small_uint_count_zeros_func(a_u128);
    println!("The number of zeros of {:0128b} is {}.", a_u128, zeros);
    assert_eq!(zeros, 62_u32);

    // Examples for usize
    let a_usize = 0b1011001110001111000011111000001111110000001111111000000011111111_usize;
    let zeros = small_uint_count_zeros_func(a_usize);
    println!("The number of zeros of {:064b} is {}.", a_usize, zeros);
    assert_eq!(zeros, 28_u32);

    // Examples for ShortUnion
    let a_shortunion = 0b1011001110001111_u16.into_shortunion();
    let zeros = small_uint_count_zeros_func(a_shortunion);
    println!("The number of zeros of {:016b} is {}.", a_shortunion.get(), zeros);
    assert_eq!(zeros, 6_u32);

    // Examples for IntUnion
    let a_intunion = 0b10110011100011110000111110000011_u32.into_intunion();
    let zeros = small_uint_count_zeros_func(a_intunion);
    println!("The number of zeros of {:032b} is {}.", a_intunion.get(), zeros);
    assert_eq!(zeros, 15_u32);

    // Examples for LongUnion
    let a_longunion = 0b1011001110001111000011111000001111110000001111111000000011111111_u64.into_longunion();
    let zeros = small_uint_count_zeros_func(a_longunion);
    println!("The number of zeros of {:064b} is {}.", a_longunion.get(), zeros);
    assert_eq!(zeros, 28_u32);

    // Examples for LongerUnion
    let a_longerunion = 0b10110011100011110000111110000011111100000011111110000000111111110000000011111111100000000011111111110000000000111111111110000000_u128.into_longerunion();
    let zeros = small_uint_count_zeros_func(a_longerunion);
    println!("The number of zeros of {:0128b} is {}.", a_longerunion.get(), zeros);
    assert_eq!(zeros, 62_u32);

    // Examples for SizeUnion
    let a_sizeunion = 0b1011001110001111000011111000001111110000001111111000000011111111_usize.into_sizeunion();
    let zeros = small_uint_count_zeros_func(a_sizeunion);
    println!("The number of zeros of {:064b} is {}.", a_sizeunion.get(), zeros);
    assert_eq!(zeros, 28_u32);
    println!("--------------------------------------");
}

fn small_uint_count_zeros_func<T: cryptocol::number::SmallUInt>(num: T) -> u32
{
    num.count_zeros()
}

fn small_uint_leading_ones()
{
    println!("small_uint_leading_ones");
    use cryptocol::number::SmallUInt;
    // Examples for u8
    let a_u8 = 0b11001110_u8;
    let ones = small_uint_leading_ones_func(a_u8);
    println!("The number of leading ones of {:08b} is {}.", a_u8, ones);
    assert_eq!(ones, 2_u32);

    // Examples for u16
    let a_u16 = 0b1111101100111000_u16;
    let ones = small_uint_leading_ones_func(a_u16);
    println!("The number of leading ones of {:016b} is {}.", a_u16, ones);
    assert_eq!(ones, 5_u32);

    // Examples for u32
    let a_u32 = 0b11100011110000111110000011101100_u32;
    let ones = small_uint_leading_ones_func(a_u32);
    println!("The number of leading ones of {:032b} is {}.", a_u32, ones);
    assert_eq!(ones, 3_u32);

    // Examples for u64
    let a_u64 = 0b1111111110110011100011110000111110000011111100000011111110000000_u64;
    let ones = small_uint_leading_ones_func(a_u64);
    println!("The number of leading ones of {:064b} is {}.", a_u64, ones);
    assert_eq!(ones, 9_u32);

    // Examples for u128
    let a_u128 = 0b10110011100011110000111110000011111100000011111110000000111111110000000011111111100000000011111111110000000000111111111110000000_u128;
    let ones = small_uint_leading_ones_func(a_u128);
    println!("The number of leading ones of {:0128b} is {}.", a_u128, ones);
    assert_eq!(ones, 1_u32);

    // Examples for usize
    let a_usize = 0b0000001111111000000011111111101100111000111100001111100000111111_usize;
    let ones = small_uint_leading_ones_func(a_usize);
    println!("The number of leading ones of {:064b} is {}.", a_usize, ones);
    assert_eq!(ones, 0_u32);

    // Examples for ShortUnion
    let a_shortunion = 0b1100111000111110_u16.into_shortunion();
    let ones = small_uint_leading_ones_func(a_shortunion);
    println!("The number of leading ones of {:016b} is {}.", a_shortunion.get(), ones);
    assert_eq!(ones, 2_u32);

    // Examples for IntUnion
    let a_intunion = 0b11110000111110000011101100111000_u32.into_intunion();
    let ones = small_uint_leading_ones_func(a_intunion);
    println!("The number of leading ones of {:032b} is {}.", a_intunion.get(), ones);
    assert_eq!(ones, 4_u32);

    // Examples for LongUnion
    let a_longunion = 0b1111100000111111000000111111100000001111111110110011100011110000_u64.into_longunion();
    let ones = small_uint_leading_ones_func(a_longunion);
    println!("The number of leading ones of {:064b} is {}.", a_longunion.get(), ones);
    assert_eq!(ones, 5_u32);

    // Examples for LongerUnion
    let a_longerunion = 0b11111111111000000010110011100011110000111110000011111100000011111110000000111111110000000011111111100000000011111111110000000000_u128.into_longerunion();
    let ones = small_uint_leading_ones_func(a_longerunion);
    println!("The number of leading ones of {:0128b} is {}.", a_longerunion.get(), ones);
    assert_eq!(ones, 11_u32);

    // Examples for SizeUnion
    let a_sizeunion = 0b1111111000000011111111101100111000111100001111100000111111000000_usize.into_sizeunion();
    let ones = small_uint_leading_ones_func(a_sizeunion);
    println!("The number of leading ones of {:064b} is {}.", a_sizeunion.get(), ones);
    assert_eq!(ones, 7_u32);
    println!("--------------------------------------");
}

fn small_uint_leading_ones_func<T: cryptocol::number::SmallUInt>(num: T) -> u32
{
    num.leading_ones()
}

fn small_uint_leading_zeros()
{
    println!("small_uint_leading_zeros");
    use cryptocol::number::SmallUInt;
    // Examples for u8
    let a_u8 = 0b10110011_u8;
    let zeros = small_uint_leading_zeros_func(a_u8);
    println!("The number of leading zeros of {:08b} is {}.", a_u8, zeros);
    assert_eq!(zeros, 0_u32);

    // Examples for u16
    let a_u16 = 0b0001111101100111_u16;
    let zeros = small_uint_leading_zeros_func(a_u16);
    println!("The number of leading zeros of {:016b} is {}.", a_u16, zeros);
    assert_eq!(zeros, 3_u32);

    // Examples for u32
    let a_u32 = 0b00111000111100001111100000111011_u32;
    let zeros = small_uint_leading_zeros_func(a_u32);
    println!("The number of leading zeros of {:032b} is {}.", a_u32, zeros);
    assert_eq!(zeros, 2_u32);

    // Examples for u64
    let a_u64 = 0b0000001111111000000011111111101100111000111100001111100000111111_u64;
    let zeros = small_uint_leading_zeros_func(a_u64);
    println!("The number of leading zeros of {:064b} is {}.", a_u64, zeros);
    assert_eq!(zeros, 6_u32);

    // Examples for u128
    let a_u128 = 0b00000000001111111111100000001011001110001111000011111000001111110000001111111000000011111111000000001111111110000000001111111111_u128;
    let zeros = small_uint_leading_zeros_func(a_u128);
    println!("The number of leading zeros of {:0128b} is {}.", a_u128, zeros);
    assert_eq!(zeros, 10_u32);

    // Examples for usize
    let a_usize = 0b0000111110000011111100000011111110000000111111111011001110001111_usize;
    let zeros = small_uint_leading_zeros_func(a_usize);
    println!("The number of leading zeros of {:064b} is {}.", a_usize, zeros);
    assert_eq!(zeros, 4_u32);

    // Examples for ShortUnion
    let a_shortunion = 0b0011100011111011_u16.into_shortunion();
    let zeros = small_uint_leading_zeros_func(a_shortunion);
    println!("The number of leading zeros of {:016b} is {}.", a_shortunion.get(), zeros);
    assert_eq!(zeros, 2_u32);

    // Examples for IntUnion
    let a_intunion = 0b01100111000111100001111100000111_u32.into_intunion();
    let zeros = small_uint_leading_zeros_func(a_intunion);
    println!("The number of leading zeros of {:032b} is {}.", a_intunion.get(), zeros);
    assert_eq!(zeros, 1_u32);

    // Examples for LongUnion
    let a_longunion = 0b0000011111100000011111110000000111111111011001110001111000011111_u64.into_longunion();
    let zeros = small_uint_leading_zeros_func(a_longunion);
    println!("The number of leading zeros of {:064b} is {}.", a_longunion.get(), zeros);
    assert_eq!(zeros, 5_u32);

    // Examples for LongerUnion
    let a_longerunion = 0b00000001111111100000000111111111000000000111111111100000000001111111111100000001011001110001111000011111000001111110000001111111_u128.into_longerunion();
    let zeros = small_uint_leading_zeros_func(a_longerunion);
    println!("The number of leading zeros of {:0128b} is {}.", a_longerunion.get(), zeros);
    assert_eq!(zeros, 7_u32);

    // Examples for SizeUnion
    let a_sizeunion = 0b0000111110000011111100000011111110000000111111111011001110001111_usize.into_sizeunion();
    let zeros = small_uint_leading_zeros_func(a_sizeunion);
    println!("The number of leading zeros of {:064b} is {}.", a_sizeunion.get(), zeros);
    assert_eq!(zeros, 4_u32);
    println!("--------------------------------------");
}

fn small_uint_leading_zeros_func<T: cryptocol::number::SmallUInt>(num: T) -> u32
{
    num.leading_zeros()
}

fn small_uint_trailing_ones()
{
    println!("small_uint_trailing_ones");
    use cryptocol::number::SmallUInt;
    // Examples for u8
    let a_u8 = 0b10110011_u8;
    let ones = small_uint_trailing_ones_func(a_u8);
    println!("The number of trailing ones of {:08b} is {}.", a_u8, ones);
    assert_eq!(ones, 2_u32);

    // Examples for u16
    let a_u16 = 0b1011001110001111_u16;
    let ones = small_uint_trailing_ones_func(a_u16);
    println!("The number of trailing ones of {:016b} is {}.", a_u16, ones);
    assert_eq!(ones, 4_u32);

    // Examples for u32
    let a_u32 = 0b00000111011001110001111000011111_u32;
    let ones = small_uint_trailing_ones_func(a_u32);
    println!("The number of trailing ones of {:032b} is {}.", a_u32, ones);
    assert_eq!(ones, 5_u32);

    // Examples for u64
    let a_u64 = 0b1011001110001111000011111000001111110000001111111000000011111111_u64;
    let ones = small_uint_trailing_ones_func(a_u64);
    println!("The number of trailing ones of {:064b} is {}.", a_u64, ones);
    assert_eq!(ones, 8_u32);

    // Examples for u128
    let a_u128 = 0b10110011100011110000111110000011111100000011111110000000111111110000000011111111100000000011111111110000000000111111111110000000_u128;
    let ones = small_uint_trailing_ones_func(a_u128);
    println!("The number of trailing ones of {:0128b} is {}.", a_u128, ones);
    assert_eq!(ones, 0_u32);

    // Examples for usize
    let a_usize = 0b1111110110011100011110000111110000011111100000011111110000000111_usize;
    let ones = small_uint_trailing_ones_func(a_usize);
    println!("The number of trailing ones of {:064b} is {}.", a_usize, ones);
    assert_eq!(ones, 3_u32);

    // Examples for ShortUnion
    let a_shortunion = 0b1111011001110001_u16.into_shortunion();
    let ones = small_uint_trailing_ones_func(a_shortunion);
    println!("The number of trailing ones of {:016b} is {}.", a_shortunion.get(), ones);
    assert_eq!(ones, 1_u32);

    // Examples for IntUnion
    let a_intunion = 0b00000111011001110001111000011111_u32.into_intunion();
    let ones = small_uint_trailing_ones_func(a_intunion);
    println!("The number of trailing ones of {:032b} is {}.", a_intunion.get(), ones);
    assert_eq!(ones, 5_u32);

    // Examples for LongUnion
    let a_longunion = 0b1011001110001111000011111000001111110000001111111000000011111111_u64.into_longunion();
    let ones = small_uint_trailing_ones_func(a_longunion);
    println!("The number of trailing ones of {:064b} is {}.", a_longunion.get(), ones);
    assert_eq!(ones, 8_u32);

    // Examples for LongerUnion
    let a_longerunion = 0b00000000001111111111100000001011001110001111000011111000001111110000001111111000000011111111000000001111111110000000001111111111_u128.into_longerunion();
    let ones = small_uint_trailing_ones_func(a_longerunion);
    println!("The number of trailing ones of {:0128b} is {}.", a_longerunion.get(), ones);
    assert_eq!(ones, 10_u32);

    // Examples for SizeUnion
    let a_sizeunion = 0b1011001110001111000011111000001111110000001111111000000011111111_usize.into_sizeunion();
    let ones = small_uint_trailing_ones_func(a_sizeunion);
    println!("The number of trailing ones of {:064b} is {}.", a_sizeunion.get(), ones);
    assert_eq!(ones, 8_u32);
    println!("--------------------------------------");
}

fn small_uint_trailing_ones_func<T: cryptocol::number::SmallUInt>(num: T) -> u32
{
    num.trailing_ones()
}

fn small_uint_trailing_zeros()
{
    println!("small_uint_trailing_zeros");
    use cryptocol::number::SmallUInt;
    // Examples for u8
    let a_u8 = 0b10110011_u8;
    let zeros = small_uint_trailing_zeros_func(a_u8);
    println!("The number of trailing zeros of {:08b} is {}.", a_u8, zeros);
    assert_eq!(zeros, 0_u32);

    // Examples for u16
    let a_u16 = 0b1111101100111000_u16;
    let zeros = small_uint_trailing_zeros_func(a_u16);
    println!("The number of trailing zeros of {:016b} is {}.", a_u16, zeros);
    assert_eq!(zeros, 3_u32);

    // Examples for u32
    let a_u32 = 0b11101100111000111100001111100000_u32;
    let zeros = small_uint_trailing_zeros_func(a_u32);
    println!("The number of trailing zeros of {:032b} is {}.", a_u32, zeros);
    assert_eq!(zeros, 5_u32);

    // Examples for u64
    let a_u64 = 0b1111111110110011100011110000111110000011111100000011111110000000_u64;
    let zeros = small_uint_trailing_zeros_func(a_u64);
    println!("The number of trailing zeros of {:064b} is {}.", a_u64, zeros);
    assert_eq!(zeros, 7_u32);

    // Examples for u128
    let a_u128 = 0b11111111111000000010110011100011110000111110000011111100000011111110000000111111110000000011111111100000000011111111110000000000_u128;
    let zeros = small_uint_trailing_zeros_func(a_u128);
    println!("The number of trailing zeros of {:0128b} is {}.", a_u128, zeros);
    assert_eq!(zeros, 10_u32);

    // Examples for usize
    let a_usize = 0b1111100000111111000000111111100000001111111110110011100011110000_usize;
    let zeros = small_uint_trailing_zeros_func(a_usize);
    println!("The number of trailing zeros of {:064b} is {}.", a_usize, zeros);
    assert_eq!(zeros, 4_u32);

    // Examples for ShortUnion
    let a_shortunion = 0b1111101100111000_u16.into_shortunion();
    let zeros = small_uint_trailing_zeros_func(a_shortunion);
    println!("The number of trailing zeros of {:016b} is {}.", a_shortunion.get(), zeros);
    assert_eq!(zeros, 3_u32);

    // Examples for IntUnion
    let a_intunion = 0b11101100111000111100001111100000_u32.into_intunion();
    let zeros = small_uint_trailing_zeros_func(a_intunion);
    println!("The number of trailing zeros of {:032b} is {}.", a_intunion.get(), zeros);
    assert_eq!(zeros, 5_u32);

    // Examples for LongUnion
    let a_longunion = 0b1111111000000011111111101100111000111100001111100000111111000000_u64.into_longunion();
    let zeros = small_uint_trailing_zeros_func(a_longunion);
    println!("The number of trailing zeros of {:064b} is {}.", a_longunion.get(), zeros);
    assert_eq!(zeros, 6_u32);

    // Examples for LongerUnion
    let a_longerunion = 0b10110011100011110000111110000011111100000011111110000000111111110000000011111111100000000011111111110000000000111111111110000000_u128.into_longerunion();
    let zeros = small_uint_trailing_zeros_func(a_longerunion);
    println!("The number of trailing zeros of {:0128b} is {}.", a_longerunion.get(), zeros);
    assert_eq!(zeros, 7_u32);

    // Examples for SizeUnion
    let a_sizeunion = 0b1100111000111100001111100000111111000000111111100000001111111110_usize.into_sizeunion();
    let zeros = small_uint_trailing_zeros_func(a_sizeunion);
    println!("The number of trailing zeros of {:064b} is {}.", a_sizeunion.get(), zeros);
    assert_eq!(zeros, 1_u32);
    println!("--------------------------------------");
}

fn small_uint_trailing_zeros_func<T: cryptocol::number::SmallUInt>(num: T) -> u32
{
    num.trailing_zeros()
}

fn small_uint_set_msb()
{
    println!("small_uint_set_msb");
    use cryptocol::number::SmallUInt;
    // Examples for u8
    let mut a_u8 = 0b00110011_u8;
    println!("Originally, a_u8 = {:08b}", a_u8);
    a_u8.set_msb();
    println!("After a_u8.set_msb(), a_u8 = {:08b}.", a_u8);
    assert_eq!(a_u8, 0b10110011_u8);

    let mut b_u8 = 0b10110011_u8;
    println!("Originally, b_u8 = {:08b}", b_u8);
    b_u8.set_msb();
    println!("After b_u8.set_msb(), b_u8 = {:08b}.", b_u8);
    assert_eq!(b_u8, 0b10110011_u8);

    let mut c_u8 = 0b00110011_u8;
    println!("Originally, c_u8 = {:08b}", c_u8);
    small_uint_set_msb_func(&mut c_u8);
    println!("After c_u8.set_msb(), c_u8 = {:08b}.", c_u8);
    assert_eq!(c_u8, 0b10110011_u8);

    let mut d_u8 = 0b10110011_u8;
    println!("Originally, d_u8 = {:08b}", d_u8);
    small_uint_set_msb_func(&mut d_u8);
    println!("After d_u8.set_msb(), d_u8 = {:08b}.", d_u8);
    assert_eq!(d_u8, 0b10110011_u8);

    // Examples for u16
    let mut a_u16 = 0b0001111101100111_u16;
    println!("Originally, a_u16 = {:016b}", a_u16);
    a_u16.set_msb();
    println!("After a_u16.set_msb(), a_u16 = {:016b}.", a_u16);
    assert_eq!(a_u16, 0b1001111101100111_u16);

    let mut b_u16 = 0b1001111101100111_u16;
    println!("Originally, b_u16 = {:016b}", b_u16);
    b_u16.set_msb();
    println!("After b_u16.set_msb(), b_u16 = {:016b}.", b_u16);
    assert_eq!(b_u16, 0b1001111101100111_u16);

    let mut c_u16 = 0b0001111101100111_u16;
    println!("Originally, c_u16 = {:016b}", c_u16);
    small_uint_set_msb_func(&mut c_u16);
    println!("After c_u16.set_msb(), c_u16 = {:016b}.", c_u16);
    assert_eq!(c_u16, 0b1001111101100111_u16);

    let mut d_u16 = 0b1001111101100111_u16;
    println!("Originally, d_u16 = {:016b}", d_u16);
    small_uint_set_msb_func(&mut d_u16);
    println!("After d_u16.set_msb(), d_u16 = {:016b}.", d_u16);
    assert_eq!(d_u16, 0b1001111101100111_u16);

    // Examples for u32
    let mut a_u32 = 0b00111000111100001111100000111011_u32;
    println!("Originally, a_u32 = {:032b}", a_u32);
    a_u32.set_msb();
    println!("After a_u32.set_msb(), a_u32 = {:032b}.", a_u32);
    assert_eq!(a_u32, 0b10111000111100001111100000111011_u32);

    let mut b_u32 = 0b10111000111100001111100000111011_u32;
    println!("Originally, b_u32 = {:032b}", b_u32);
    b_u32.set_msb();
    println!("After b_u32.set_msb(), b_u32 = {:032b}.", b_u32);
    assert_eq!(b_u32, 0b10111000111100001111100000111011_u32);

    let mut c_u32 = 0b00111000111100001111100000111011_u32;
    println!("Originally, c_u32 = {:032b}", c_u32);
    small_uint_set_msb_func(&mut c_u32);
    println!("After c_u32.set_msb(), c_u32 = {:032b}.", c_u32);
    assert_eq!(c_u32, 0b10111000111100001111100000111011_u32);

    let mut d_u32 = 0b10111000111100001111100000111011_u32;
    println!("Originally, d_u32 = {:032b}", d_u32);
    small_uint_set_msb_func(&mut d_u32);
    println!("After d_u32.set_msb(), d_u32 = {:032b}.", d_u32);
    assert_eq!(d_u32, 0b10111000111100001111100000111011_u32);

    // Examples for u64
    let mut a_u64 = 0b0000001111111000000011111111101100111000111100001111100000111111_u64;
    println!("Originally, a_u64 = {:064b}", a_u64);
    small_uint_set_msb_func(&mut a_u64);
    println!("After a_u64.set_msb(), a_u64 = {:064b}.", a_u64);
    assert_eq!(a_u64, 0b1000001111111000000011111111101100111000111100001111100000111111_u64);

    let mut b_u64 = 0b1000001111111000000011111111101100111000111100001111100000111111_u64;
    println!("Originally, b_u64 = {:064b}", b_u64);
    b_u64.set_msb();
    println!("After b_u64.set_msb(), b_u64 = {:064b}.", b_u64);
    assert_eq!(a_u64, 0b1000001111111000000011111111101100111000111100001111100000111111_u64);

    let mut c_u64 = 0b0000001111111000000011111111101100111000111100001111100000111111_u64;
    println!("Originally, c_u64 = {:064b}", c_u64);
    small_uint_set_msb_func(&mut c_u64);
    println!("After c_u64.set_msb(), c_u64 = {:064b}.", c_u64);
    assert_eq!(c_u64, 0b1000001111111000000011111111101100111000111100001111100000111111_u64);

    let mut d_u64 = 0b1000001111111000000011111111101100111000111100001111100000111111_u64;
    println!("Originally, d_u64 = {:064b}", d_u64);
    small_uint_set_msb_func(&mut d_u64);
    println!("After d_u64.set_msb(), d_u64 = {:064b}.", d_u64);
    assert_eq!(c_u64, 0b1000001111111000000011111111101100111000111100001111100000111111_u64);

    // Examples for u128
    let mut a_u128 = 0b00000000001111111111100000001011001110001111000011111000001111110000001111111000000011111111000000001111111110000000001111111111_u128;
    println!("Originally, a_u128 = {:0128b}", a_u128);
    a_u128.set_msb();
    println!("After a_u128.set_msb(), a_u128 = {:0128b}.", a_u128);
    assert_eq!(a_u128, 0b10000000001111111111100000001011001110001111000011111000001111110000001111111000000011111111000000001111111110000000001111111111_u128);

    let mut b_u128 = 0b10000000001111111111100000001011001110001111000011111000001111110000001111111000000011111111000000001111111110000000001111111111_u128;
    println!("Originally, b_u128 = {:0128b}", b_u128);
    b_u128.set_msb();
    println!("After b_u128.set_msb(), b_u128 = {:0128b}.", b_u128);
    assert_eq!(a_u128, 0b10000000001111111111100000001011001110001111000011111000001111110000001111111000000011111111000000001111111110000000001111111111_u128);

    let mut c_u128 = 0b00000000001111111111100000001011001110001111000011111000001111110000001111111000000011111111000000001111111110000000001111111111_u128;
    println!("Originally, c_u128 = {:0128b}", c_u128);
    small_uint_set_msb_func(&mut c_u128);
    println!("After c_u128.set_msb(), c_u128 = {:0128b}.", c_u128);
    assert_eq!(c_u128, 0b10000000001111111111100000001011001110001111000011111000001111110000001111111000000011111111000000001111111110000000001111111111_u128);

    let mut d_u128 = 0b10000000001111111111100000001011001110001111000011111000001111110000001111111000000011111111000000001111111110000000001111111111_u128;
    println!("Originally, d_u128 = {:0128b}", d_u128);
    small_uint_set_msb_func(&mut d_u128);
    println!("After d_u128.set_msb(), d_u128 = {:0128b}.", d_u128);
    assert_eq!(c_u128, 0b10000000001111111111100000001011001110001111000011111000001111110000001111111000000011111111000000001111111110000000001111111111_u128);

    // Examples for usize
    let mut a_usize = 0b0000111110000011111100000011111110000000111111111011001110001111_usize;
    println!("Originally, a_usize = {:064b}", a_usize);
    a_usize.set_msb();
    println!("After a_usize.set_msb(), a_usize = {:064b}.", a_usize);
    assert_eq!(a_usize, 0b1000111110000011111100000011111110000000111111111011001110001111_usize);

    let mut b_usize = 0b1000111110000011111100000011111110000000111111111011001110001111_usize;
    println!("Originally, b_usize = {:064b}", b_usize);
    b_usize.set_msb();
    println!("After b_usize.set_msb(), b_usize = {:064b}.", b_usize);
    assert_eq!(b_usize, 0b1000111110000011111100000011111110000000111111111011001110001111_usize);

    let mut c_usize = 0b0000111110000011111100000011111110000000111111111011001110001111_usize;
    println!("Originally, c_usize = {:064b}", c_usize);
    small_uint_set_msb_func(&mut c_usize);
    println!("After c_usize.set_msb(), c_usize = {:064b}.", c_usize);
    assert_eq!(c_usize, 0b1000111110000011111100000011111110000000111111111011001110001111_usize);

    let mut d_usize = 0b1000111110000011111100000011111110000000111111111011001110001111_usize;
    println!("Originally, d_usize = {:064b}", d_usize);
    small_uint_set_msb_func(&mut d_usize);
    println!("After d_usize.set_msb(), d_usize = {:064b}.", d_usize);
    assert_eq!(d_usize, 0b1000111110000011111100000011111110000000111111111011001110001111_usize);

    // Examples for ShortUnion
    let mut a_shortunion = 0b0011100011111011_u16.into_shortunion();
    println!("Originally, a_shortunion = {:016b}", a_shortunion.get());
    a_shortunion.set_msb();
    println!("After a_shortunion.set_msb(), a_shortunion = {:016b}.", a_shortunion.get());
    assert_eq!(a_shortunion.get(), 0b1011100011111011_u16);

    let mut b_shortunion = 0b1011100011111011_u16.into_shortunion();
    println!("Originally, b_shortunion = {:016b}", b_shortunion.get());
    b_shortunion.set_msb();
    println!("After b_shortunion.set_msb(), b_shortunion = {:016b}.", b_shortunion.get());
    assert_eq!(a_shortunion.get(), 0b1011100011111011_u16);

    let mut c_shortunion = 0b0011100011111011_u16.into_shortunion();
    println!("Originally, c_shortunion = {:016b}", c_shortunion.get());
    small_uint_set_msb_func(&mut c_shortunion);
    println!("After c_shortunion.set_msb(), c_shortunion = {:016b}.", c_shortunion.get());
    assert_eq!(c_shortunion.get(), 0b1011100011111011_u16);

    let mut d_shortunion = 0b1011100011111011_u16.into_shortunion();
    println!("Originally, d_shortunion = {:016b}", d_shortunion.get());
    small_uint_set_msb_func(&mut d_shortunion);
    println!("After d_shortunion.set_msb(), d_shortunion = {:016b}.", d_shortunion.get());
    assert_eq!(c_shortunion.get(), 0b1011100011111011_u16);

    // Examples for IntUnion
    let mut a_intunion = 0b01100111000111100001111100000111_u32.into_intunion();
    println!("Originally, a_intunion = {:032b}", a_intunion.get());
    a_intunion.set_msb();
    println!("After a_intunion.set_msb(), a_intunion = {:032b}.", a_intunion.get());
    assert_eq!(a_intunion.get(), 0b11100111000111100001111100000111_u32);

    let mut b_intunion = 0b11100111000111100001111100000111_u32.into_intunion();
    println!("Originally, b_intunion = {:032b}", b_intunion.get());
    b_intunion.set_msb();
    println!("After b_intunion.set_msb(), b_intunion = {:032b}.", b_intunion.get());
    assert_eq!(b_intunion.get(), 0b11100111000111100001111100000111_u32);

    let mut c_intunion = 0b01100111000111100001111100000111_u32.into_intunion();
    println!("Originally, c_intunion = {:032b}", c_intunion.get());
    small_uint_set_msb_func(&mut c_intunion);
    println!("After c_intunion.set_msb(), c_intunion = {:032b}.", c_intunion.get());
    assert_eq!(c_intunion.get(), 0b11100111000111100001111100000111_u32);

    let mut d_intunion = 0b11100111000111100001111100000111_u32.into_intunion();
    println!("Originally, d_intunion = {:032b}", d_intunion.get());
    small_uint_set_msb_func(&mut d_intunion);
    println!("After d_intunion.set_msb(), d_intunion = {:032b}.", d_intunion.get());
    assert_eq!(d_intunion.get(), 0b11100111000111100001111100000111_u32);

    // Examples for LongUnion
    let mut a_longunion = 0b0000011111100000011111110000000111111111011001110001111000011111_u64.into_longunion();
    println!("Originally, a_longunion = {:064b}", a_longunion.get());
    a_longunion.set_msb();
    println!("After a_longunion.set_msb(), a_longunion = {:064b}.", a_longunion.get());
    assert_eq!(a_longunion.get(), 0b1000011111100000011111110000000111111111011001110001111000011111_u64);

    let mut b_longunion = 0b1000011111100000011111110000000111111111011001110001111000011111_u64.into_longunion();
    println!("Originally, b_longunion = {:064b}", b_longunion.get());
    b_longunion.set_msb();
    println!("After b_longunion.set_msb(), b_longunion = {:064b}.", b_longunion.get());
    assert_eq!(b_longunion.get(), 0b1000011111100000011111110000000111111111011001110001111000011111_u64);

    let mut c_longunion = 0b0000011111100000011111110000000111111111011001110001111000011111_u64.into_longunion();
    println!("Originally, c_longunion = {:064b}", c_longunion.get());
    small_uint_set_msb_func(&mut c_longunion);
    println!("After c_longunion.set_msb(), c_longunion = {:064b}.", c_longunion.get());
    assert_eq!(c_longunion.get(), 0b1000011111100000011111110000000111111111011001110001111000011111_u64);

    let mut d_longunion = 0b1000011111100000011111110000000111111111011001110001111000011111_u64.into_longunion();
    println!("Originally, d_longunion = {:064b}", d_longunion.get());
    small_uint_set_msb_func(&mut d_longunion);
    println!("After d_longunion.set_msb(), d_longunion = {:064b}.", d_longunion.get());
    assert_eq!(d_longunion.get(), 0b1000011111100000011111110000000111111111011001110001111000011111_u64);

    // Examples for LongerUnion
    let mut a_longerunion = 0b00000001111111100000000111111111000000000111111111100000000001111111111100000001011001110001111000011111000001111110000001111111_u128.into_longerunion();
    println!("Originally, a_longerunion = {:0128b}", a_longerunion.get());
    a_longerunion.set_msb();
    println!("After a_longerunion.set_msb(), a_longerunion = {:0128b}.", a_longerunion.get());
    assert_eq!(a_longerunion.get(), 0b10000001111111100000000111111111000000000111111111100000000001111111111100000001011001110001111000011111000001111110000001111111_u128);

    let mut b_longerunion = 0b10000001111111100000000111111111000000000111111111100000000001111111111100000001011001110001111000011111000001111110000001111111_u128.into_longerunion();
    println!("Originally, b_longerunion = {:0128b}", b_longerunion.get());
    b_longerunion.set_msb();
    println!("After b_longerunion.set_msb(), b_longerunion = {:0128b}.", b_longerunion.get());
    assert_eq!(b_longerunion.get(), 0b10000001111111100000000111111111000000000111111111100000000001111111111100000001011001110001111000011111000001111110000001111111_u128);

    let mut c_longerunion = 0b00000001111111100000000111111111000000000111111111100000000001111111111100000001011001110001111000011111000001111110000001111111_u128.into_longerunion();
    println!("Originally, c_longerunion = {:0128b}", c_longerunion.get());
    small_uint_set_msb_func(&mut c_longerunion);
    println!("After c_longerunion.set_msb(), c_longerunion = {:0128b}.", c_longerunion.get());
    assert_eq!(c_longerunion.get(), 0b10000001111111100000000111111111000000000111111111100000000001111111111100000001011001110001111000011111000001111110000001111111_u128);

    let mut d_longerunion = 0b10000001111111100000000111111111000000000111111111100000000001111111111100000001011001110001111000011111000001111110000001111111_u128.into_longerunion();
    println!("Originally, d_longerunion = {:0128b}", d_longerunion.get());
    small_uint_set_msb_func(&mut d_longerunion);
    println!("After d_longerunion.set_msb(), d_longerunion = {:0128b}.", d_longerunion.get());
    assert_eq!(d_longerunion.get(), 0b10000001111111100000000111111111000000000111111111100000000001111111111100000001011001110001111000011111000001111110000001111111_u128);

    // Examples for SizeUnion
    let mut a_sizeunion = 0b0000111110000011111100000011111110000000111111111011001110001111_usize.into_sizeunion();
    println!("Originally, a_sizeunion = {:064b}", a_sizeunion.get());
    a_sizeunion.set_msb();
    println!("After a_sizeunion.set_msb(), a_sizeunion = {:064b}.", a_sizeunion.get());
    assert_eq!(a_sizeunion.get(), 0b1000111110000011111100000011111110000000111111111011001110001111_usize);

    let mut b_sizeunion = 0b1000111110000011111100000011111110000000111111111011001110001111_usize.into_sizeunion();
    println!("Originally, b_sizeunion = {:064b}", b_sizeunion.get());
    b_sizeunion.set_msb();
    println!("After b_sizeunion.set_msb(), b_sizeunion = {:064b}.", b_sizeunion.get());
    assert_eq!(b_sizeunion.get(), 0b1000111110000011111100000011111110000000111111111011001110001111_usize);

    let mut c_sizeunion = 0b0000111110000011111100000011111110000000111111111011001110001111_usize.into_sizeunion();
    println!("Originally, c_sizeunion = {:064b}", c_sizeunion.get());
    small_uint_set_msb_func(&mut c_sizeunion);
    println!("After c_sizeunion.set_msb(), c_sizeunion = {:064b}.", c_sizeunion.get());
    assert_eq!(c_sizeunion.get(), 0b1000111110000011111100000011111110000000111111111011001110001111_usize);

    let mut d_sizeunion = 0b1000111110000011111100000011111110000000111111111011001110001111_usize.into_sizeunion();
    println!("Originally, d_sizeunion = {:064b}", d_sizeunion.get());
    small_uint_set_msb_func(&mut d_sizeunion);
    println!("After d_sizeunion.set_msb(), d_sizeunion = {:064b}.", d_sizeunion.get());
    assert_eq!(d_sizeunion.get(), 0b1000111110000011111100000011111110000000111111111011001110001111_usize);
    println!("--------------------------------------");
}

fn small_uint_set_msb_func<T: cryptocol::number::SmallUInt>(num: &mut T)
{
    num.set_msb()
}

fn small_uint_set_lsb()
{
    println!("small_uint_set_lsb");
    use cryptocol::number::SmallUInt;
    // Examples for u8
    let mut a_u8 = 0b10110010_u8;
    println!("Originally, a_u8 = {:08b}", a_u8);
    a_u8.set_lsb();
    println!("After a_u8.set_lsb(), a_u8 = {:08b}.", a_u8);
    assert_eq!(a_u8, 0b10110011_u8);

    let mut b_u8 = 0b10110011_u8;
    println!("Originally, b_u8 = {:08b}", b_u8);
    b_u8.set_lsb();
    println!("After b_u8.set_lsb(), b_u8 = {:08b}.", b_u8);
    assert_eq!(b_u8, 0b10110011_u8);

    let mut c_u8 = 0b10110010_u8;
    println!("Originally, c_u8 = {:08b}", c_u8);
    small_uint_set_lsb_func(&mut c_u8);
    println!("After c_u8.set_lsb(), c_u8 = {:08b}.", c_u8);
    assert_eq!(c_u8, 0b10110011_u8);

    let mut d_u8 = 0b10110011_u8;
    println!("Originally, d_u8 = {:08b}", d_u8);
    small_uint_set_lsb_func(&mut d_u8);
    println!("After d_u8.set_lsb(), d_u8 = {:08b}.", d_u8);
    assert_eq!(d_u8, 0b10110011_u8);

    // Examples for u16
    let mut a_u16 = 0b0001111101100110_u16;
    println!("Originally, a_u16 = {:016b}", a_u16);
    a_u16.set_lsb();
    println!("After a_u16.set_lsb(), a_u16 = {:016b}.", a_u16);
    assert_eq!(a_u16, 0b0001111101100111_u16);

    let mut b_u16 = 0b0001111101100111_u16;
    println!("Originally, b_u16 = {:016b}", b_u16);
    b_u16.set_lsb();
    println!("After b_u16.set_lsb(), b_u16 = {:016b}.", b_u16);
    assert_eq!(b_u16, 0b0001111101100111_u16);

    let mut c_u16 = 0b0001111101100110_u16;
    println!("Originally, c_u16 = {:016b}", c_u16);
    small_uint_set_lsb_func(&mut c_u16);
    println!("After c_u16.set_lsb(), c_u16 = {:016b}.", c_u16);
    assert_eq!(c_u16, 0b0001111101100111_u16);

    let mut d_u16 = 0b0001111101100111_u16;
    println!("Originally, d_u16 = {:016b}", d_u16);
    small_uint_set_lsb_func(&mut d_u16);
    println!("After d_u16.set_lsb(), d_u16 = {:016b}.", d_u16);
    assert_eq!(d_u16, 0b0001111101100111_u16);

    // Examples for u32
    let mut a_u32 = 0b00111000111100001111100000111010_u32;
    println!("Originally, a_u32 = {:032b}", a_u32);
    small_uint_set_lsb_func(&mut a_u32);
    println!("After a_u32.set_lsb(), a_u32 = {:032b}.", a_u32);
    assert_eq!(a_u32, 0b00111000111100001111100000111011_u32);

    let mut b_u32 = 0b00111000111100001111100000111011_u32;
    println!("Originally, b_u32 = {:032b}", b_u32);
    small_uint_set_lsb_func(&mut b_u32);
    println!("After b_u32.set_lsb(), b_u32 = {:032b}.", b_u32);
    assert_eq!(b_u32, 0b00111000111100001111100000111011_u32);

    let mut c_u32 = 0b00111000111100001111100000111010_u32;
    println!("Originally, c_u32 = {:032b}", c_u32);
    small_uint_set_lsb_func(&mut c_u32);
    println!("After c_u32.set_lsb(), c_u32 = {:032b}.", c_u32);
    assert_eq!(c_u32, 0b00111000111100001111100000111011_u32);

    let mut d_u32 = 0b00111000111100001111100000111011_u32;
    println!("Originally, d_u32 = {:032b}", d_u32);
    small_uint_set_lsb_func(&mut d_u32);
    println!("After d_u32.set_lsb(), d_u32 = {:032b}.", d_u32);
    assert_eq!(d_u32, 0b00111000111100001111100000111011_u32);

    // Examples for u64
    let mut a_u64 = 0b0000001111111000000011111111101100111000111100001111100000111110_u64;
    println!("Originally, a_u64 = {:064b}", a_u64);
    small_uint_set_lsb_func(&mut a_u64);
    println!("After a_u64.set_lsb(), a_u64 = {:064b}.", a_u64);
    assert_eq!(a_u64, 0b0000001111111000000011111111101100111000111100001111100000111111_u64);

    let mut b_u64 = 0b0000001111111000000011111111101100111000111100001111100000111111_u64;
    println!("Originally, b_u64 = {:064b}", b_u64);
    small_uint_set_lsb_func(&mut b_u64);
    println!("After b_u64.set_lsb(), b_u64 = {:064b}.", b_u64);
    assert_eq!(b_u64, 0b0000001111111000000011111111101100111000111100001111100000111111_u64);

    let mut c_u64 = 0b0000001111111000000011111111101100111000111100001111100000111110_u64;
    println!("Originally, c_u64 = {:064b}", c_u64);
    small_uint_set_lsb_func(&mut c_u64);
    println!("After c_u64.set_lsb(), c_u64 = {:064b}.", c_u64);
    assert_eq!(c_u64, 0b0000001111111000000011111111101100111000111100001111100000111111_u64);

    let mut d_u64 = 0b0000001111111000000011111111101100111000111100001111100000111111_u64;
    println!("Originally, d_u64 = {:064b}", d_u64);
    small_uint_set_lsb_func(&mut d_u64);
    println!("After d_u64.set_lsb(), d_u64 = {:064b}.", d_u64);
    assert_eq!(d_u64, 0b0000001111111000000011111111101100111000111100001111100000111111_u64);

    // Examples for u128
    let mut a_u128 = 0b00000000001111111111100000001011001110001111000011111000001111110000001111111000000011111111000000001111111110000000001111111110_u128;
    println!("Originally, a_u128 = {:0128b}", a_u128);
    a_u128.set_lsb();
    println!("After a_u128.set_lsb(), a_u128 = {:0128b}.", a_u128);
    assert_eq!(a_u128, 0b00000000001111111111100000001011001110001111000011111000001111110000001111111000000011111111000000001111111110000000001111111111_u128);

    let mut b_u128 = 0b00000000001111111111100000001011001110001111000011111000001111110000001111111000000011111111000000001111111110000000001111111111_u128;
    println!("Originally, b_u128 = {:0128b}", b_u128);
    b_u128.set_lsb();
    println!("After b_u128.set_lsb(), b_u128 = {:0128b}.", b_u128);
    assert_eq!(b_u128, 0b00000000001111111111100000001011001110001111000011111000001111110000001111111000000011111111000000001111111110000000001111111111_u128);

    let mut c_u128 = 0b00000000001111111111100000001011001110001111000011111000001111110000001111111000000011111111000000001111111110000000001111111110_u128;
    println!("Originally, c_u128 = {:0128b}", c_u128);
    small_uint_set_lsb_func(&mut c_u128);
    println!("After c_u128.set_lsb(), c_u128 = {:0128b}.", c_u128);
    assert_eq!(c_u128, 0b00000000001111111111100000001011001110001111000011111000001111110000001111111000000011111111000000001111111110000000001111111111_u128);

    let mut d_u128 = 0b00000000001111111111100000001011001110001111000011111000001111110000001111111000000011111111000000001111111110000000001111111111_u128;
    println!("Originally, d_u128 = {:0128b}", d_u128);
    small_uint_set_lsb_func(&mut d_u128);
    println!("After d_u128.set_lsb(), d_u128 = {:0128b}.", d_u128);
    assert_eq!(d_u128, 0b00000000001111111111100000001011001110001111000011111000001111110000001111111000000011111111000000001111111110000000001111111111_u128);

    // Examples for usize
    let mut a_usize = 0b0000111110000011111100000011111110000000111111111011001110001110_usize;
    println!("Originally, a_usize = {:064b}", a_usize);
    small_uint_set_lsb_func(&mut a_usize);
    println!("After a_usize.set_lsb(), a_usize = {:064b}.", a_usize);
    assert_eq!(a_usize, 0b0000111110000011111100000011111110000000111111111011001110001111_usize);

    let mut b_usize = 0b0000111110000011111100000011111110000000111111111011001110001111_usize;
    println!("Originally, b_usize = {:064b}", b_usize);
    small_uint_set_lsb_func(&mut b_usize);
    println!("After b_usize.set_lsb(), b_usize = {:064b}.", b_usize);
    assert_eq!(b_usize, 0b0000111110000011111100000011111110000000111111111011001110001111_usize);

    let mut c_usize = 0b0000111110000011111100000011111110000000111111111011001110001110_usize;
    println!("Originally, c_usize = {:064b}", c_usize);
    small_uint_set_lsb_func(&mut c_usize);
    println!("After c_usize.set_lsb(), c_usize = {:064b}.", c_usize);
    assert_eq!(c_usize, 0b0000111110000011111100000011111110000000111111111011001110001111_usize);

    let mut d_usize = 0b0000111110000011111100000011111110000000111111111011001110001111_usize;
    println!("Originally, d_usize = {:064b}", d_usize);
    small_uint_set_lsb_func(&mut d_usize);
    println!("After d_usize.set_lsb(), d_usize = {:064b}.", d_usize);
    assert_eq!(d_usize, 0b0000111110000011111100000011111110000000111111111011001110001111_usize);

    // Examples for ShortUnion
    let mut a_shortunion = 0b1011100011111010_u16.into_shortunion();
    println!("Originally, a_shortunion = {:016b}", a_shortunion.get());
    small_uint_set_lsb_func(&mut a_shortunion);
    println!("After a_shortunion.set_lsb(), a_shortunion = {:016b}.", a_shortunion.get());
    assert_eq!(a_shortunion.get(), 0b1011100011111011_u16);

    let mut b_shortunion = 0b1011100011111011_u16.into_shortunion();
    println!("Originally, a_shortunion = {:016b}", b_shortunion.get());
    small_uint_set_lsb_func(&mut b_shortunion);
    println!("After b_shortunion.set_lsb(), b_shortunion = {:016b}.", b_shortunion.get());
    assert_eq!(b_shortunion.get(), 0b1011100011111011_u16);

    let mut c_shortunion = 0b1011100011111010_u16.into_shortunion();
    println!("Originally, c_shortunion = {:016b}", c_shortunion.get());
    small_uint_set_lsb_func(&mut c_shortunion);
    println!("After c_shortunion.set_lsb(), c_shortunion = {:016b}.", c_shortunion.get());
    assert_eq!(c_shortunion.get(), 0b1011100011111011_u16);

    let mut d_shortunion = 0b1011100011111011_u16.into_shortunion();
    println!("Originally, c_shortunion = {:016b}", d_shortunion.get());
    small_uint_set_lsb_func(&mut d_shortunion);
    println!("After d_shortunion.set_lsb(), d_shortunion = {:016b}.", d_shortunion.get());
    assert_eq!(d_shortunion.get(), 0b1011100011111011_u16);

    // Examples for IntUnion
    let mut a_intunion = 0b01100111000111100001111100000110_u32.into_intunion();
    println!("Originally, a_intunion = {:032b}", a_intunion.get());
    small_uint_set_lsb_func(&mut a_intunion);
    println!("After a_intunion.set_lsb(), a_intunion = {:032b}.", a_intunion.get());
    assert_eq!(a_intunion.get(), 0b01100111000111100001111100000111_u32);

    let mut b_intunion = 0b01100111000111100001111100000111_u32.into_intunion();
    println!("Originally, b_intunion = {:032b}", b_intunion.get());
    small_uint_set_lsb_func(&mut b_intunion);
    println!("After b_intunion.set_lsb(), b_intunion = {:032b}.", b_intunion.get());
    assert_eq!(b_intunion.get(), 0b01100111000111100001111100000111_u32);

    let mut c_intunion = 0b01100111000111100001111100000110_u32.into_intunion();
    println!("Originally, c_intunion = {:032b}", c_intunion.get());
    small_uint_set_lsb_func(&mut c_intunion);
    println!("After c_intunion.set_lsb(), c_intunion = {:032b}.", c_intunion.get());
    assert_eq!(c_intunion.get(), 0b01100111000111100001111100000111_u32);

    let mut d_intunion = 0b01100111000111100001111100000111_u32.into_intunion();
    println!("Originally, d_intunion = {:032b}", d_intunion.get());
    small_uint_set_lsb_func(&mut d_intunion);
    println!("After d_intunion.set_lsb(), d_intunion = {:032b}.", d_intunion.get());
    assert_eq!(d_intunion.get(), 0b01100111000111100001111100000111_u32);

    // Examples for LongUnion
    let mut a_longunion = 0b0000011111100000011111110000000111111111011001110001111000011110_u64.into_longunion();
    println!("Originally, a_longunion = {:064b}", a_longunion.get());
    small_uint_set_lsb_func(&mut a_longunion);
    println!("After a_longunion.set_lsb(), a_longunion = {:064b}.", a_longunion.get());
    assert_eq!(a_longunion.get(), 0b0000011111100000011111110000000111111111011001110001111000011111_u64);

    let mut b_longunion = 0b0000011111100000011111110000000111111111011001110001111000011111_u64.into_longunion();
    println!("Originally, b_longunion = {:064b}", b_longunion.get());
    small_uint_set_lsb_func(&mut b_longunion);
    println!("After b_longunion.set_lsb(), b_longunion = {:064b}.", b_longunion.get());
    assert_eq!(b_longunion.get(), 0b0000011111100000011111110000000111111111011001110001111000011111_u64);

    let mut c_longunion = 0b0000011111100000011111110000000111111111011001110001111000011110_u64.into_longunion();
    println!("Originally, c_longunion = {:064b}", c_longunion.get());
    small_uint_set_lsb_func(&mut c_longunion);
    println!("After c_longunion.set_lsb(), c_longunion = {:064b}.", c_longunion.get());
    assert_eq!(c_longunion.get(), 0b0000011111100000011111110000000111111111011001110001111000011111_u64);

    let mut d_longunion = 0b0000011111100000011111110000000111111111011001110001111000011111_u64.into_longunion();
    println!("Originally, d_longunion = {:064b}", d_longunion.get());
    small_uint_set_lsb_func(&mut d_longunion);
    println!("After d_longunion.set_lsb(), d_longunion = {:064b}.", d_longunion.get());
    assert_eq!(d_longunion.get(), 0b0000011111100000011111110000000111111111011001110001111000011111_u64);

    // Examples for LongerUnion
    let mut a_longerunion = 0b10000001111111100000000111111111000000000111111111100000000001111111111100000001011001110001111000011111000001111110000001111110_u128.into_longerunion();
    println!("Originally, a_longerunion = {:0128b}", a_longerunion.get());
    small_uint_set_lsb_func(&mut a_longerunion);
    println!("After a_longerunion.set_lsb(), a_longerunion = {:0128b}.", a_longerunion.get());
    assert_eq!(a_longerunion.get(), 0b10000001111111100000000111111111000000000111111111100000000001111111111100000001011001110001111000011111000001111110000001111111_u128);

    let mut b_longerunion = 0b10000001111111100000000111111111000000000111111111100000000001111111111100000001011001110001111000011111000001111110000001111111_u128.into_longerunion();
    println!("Originally, b_longerunion = {:0128b}", b_longerunion.get());
    small_uint_set_lsb_func(&mut b_longerunion);
    println!("After b_longerunion.set_lsb(), b_longerunion = {:0128b}.", b_longerunion.get());
    assert_eq!(b_longerunion.get(), 0b10000001111111100000000111111111000000000111111111100000000001111111111100000001011001110001111000011111000001111110000001111111_u128);

    let mut c_longerunion = 0b10000001111111100000000111111111000000000111111111100000000001111111111100000001011001110001111000011111000001111110000001111110_u128.into_longerunion();
    println!("Originally, c_longerunion = {:0128b}", c_longerunion.get());
    small_uint_set_lsb_func(&mut c_longerunion);
    println!("After c_longerunion.set_lsb(), c_longerunion = {:0128b}.", c_longerunion.get());
    assert_eq!(c_longerunion.get(), 0b10000001111111100000000111111111000000000111111111100000000001111111111100000001011001110001111000011111000001111110000001111111_u128);

    let mut d_longerunion = 0b10000001111111100000000111111111000000000111111111100000000001111111111100000001011001110001111000011111000001111110000001111111_u128.into_longerunion();
    println!("Originally, d_longerunion = {:0128b}", d_longerunion.get());
    small_uint_set_lsb_func(&mut d_longerunion);
    println!("After d_longerunion.set_lsb(), d_longerunion = {:0128b}.", d_longerunion.get());
    assert_eq!(d_longerunion.get(), 0b10000001111111100000000111111111000000000111111111100000000001111111111100000001011001110001111000011111000001111110000001111111_u128);

    // Examples for SizeUnion
    let mut a_sizeunion = 0b0000111110000011111100000011111110000000111111111011001110001110_usize.into_sizeunion();
    println!("Originally, a_sizeunion = {:064b}", a_sizeunion.get());
    small_uint_set_lsb_func(&mut a_sizeunion);
    println!("After a_sizeunion.set_lsb(), a_sizeunion = {:064b}.", a_sizeunion.get());
    assert_eq!(a_sizeunion.get(), 0b0000111110000011111100000011111110000000111111111011001110001111_usize);

    let mut b_sizeunion = 0b0000111110000011111100000011111110000000111111111011001110001111_usize.into_sizeunion();
    println!("Originally, b_sizeunion = {:064b}", b_sizeunion.get());
    small_uint_set_lsb_func(&mut b_sizeunion);
    println!("After b_sizeunion.set_lsb(), b_sizeunion = {:064b}.", b_sizeunion.get());
    assert_eq!(b_sizeunion.get(), 0b0000111110000011111100000011111110000000111111111011001110001111_usize);

    let mut c_sizeunion = 0b0000111110000011111100000011111110000000111111111011001110001110_usize.into_sizeunion();
    println!("Originally, c_sizeunion = {:064b}", c_sizeunion.get());
    small_uint_set_lsb_func(&mut c_sizeunion);
    println!("After c_sizeunion.set_lsb(), c_sizeunion = {:064b}.", c_sizeunion.get());
    assert_eq!(c_sizeunion.get(), 0b0000111110000011111100000011111110000000111111111011001110001111_usize);

    let mut d_sizeunion = 0b0000111110000011111100000011111110000000111111111011001110001111_usize.into_sizeunion();
    println!("Originally, d_sizeunion = {:064b}", d_sizeunion.get());
    small_uint_set_lsb_func(&mut d_sizeunion);
    println!("After d_sizeunion.set_lsb(), d_sizeunion = {:064b}.", d_sizeunion.get());
    assert_eq!(d_sizeunion.get(), 0b0000111110000011111100000011111110000000111111111011001110001111_usize);
    println!("--------------------------------------");
}

fn small_uint_set_lsb_func<T: cryptocol::number::SmallUInt>(num: &mut T)
{
    num.set_lsb()
}

fn small_uint_generate_check_bits()
{
    println!("small_uint_generate_check_bits");
    use cryptocol::number::{ SmallUInt, ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };
    // Example for u8
    let a_u8 = u8::generate_check_bits(3);
    match a_u8
    {
        Some(a) => {
                println!("a_u8 = {:08b}", a);
                assert_eq!(a, 0b00001000_u8);
            },
        None => { println!("bit_pos is out of range."); }
    }

    let b_u8 = u8::generate_check_bits(9);
    match b_u8
    {
        Some(b) => { println!("b_u8 = {:08b}", b); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(b_u8, None);
            }
    }

    let c_u8 = small_uint_generate_check_bits_func::<u8>(3);
    match c_u8
    {
        Some(c) => {
                println!("c_u8 = {:08b}", c);
                assert_eq!(c, 0b00001000_u8);
            },
        None => { println!("bit_pos is out of range."); }
    }

    let d_u8 = small_uint_generate_check_bits_func::<u8>(9);
    match d_u8
    {
        Some(c) => { println!("c_u8 = {:08b}", c); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(d_u8, None);
            }
    }

    // Example for u16
    let a_u16 = u16::generate_check_bits(12);
    match a_u16
    {
        Some(a) => {
                println!("a_u16 = {:016b}", a);
                assert_eq!(a, 0b0001000000000000_u16);
            },
        None => { println!("bit_pos is out of range."); }
    }

    let b_u16 = u16::generate_check_bits(16);
    match b_u16
    {
        Some(b) => { println!("b_u16 = {:016b}", b); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(b_u16, None);
            }
    }

    let c_u16 = small_uint_generate_check_bits_func::<u16>(12);
    match c_u16
    {
        Some(c) => {
                println!("c_u16 = {:016b}", c);
                assert_eq!(c, 0b0001000000000000_u16);
            },
        None => { println!("bit_pos is out of range."); }
    }

    let d_u16 = small_uint_generate_check_bits_func::<u16>(16);
    match d_u16
    {
        Some(d) => { println!("d_u16 = {:016b}", d); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(d_u16, None);
            }
    }

    // Example for u32
    let a_u32 = u32::generate_check_bits(20);
    match a_u32
    {
        Some(a) => {
                println!("a_u32 = {:032b}", a);
                assert_eq!(a, 0b00000000000100000000000000000000_u32);
            },
        None => { println!("bit_pos is out of range."); }
    }

    let b_u32 = u32::generate_check_bits(40);
    match b_u32
    {
        Some(b) => { println!("b_u32 = {:032b}", b); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(b_u32, None);
            }
    }

    let c_u32 = small_uint_generate_check_bits_func::<u32>(20);
    match c_u32
    {
        Some(c) => {
                println!("c_u32 = {:032b}", c);
                assert_eq!(c, 0b00000000000100000000000000000000_u32);
            },
        None => { println!("bit_pos is out of range."); }
    }

    let d_u32 = small_uint_generate_check_bits_func::<u32>(40);
    match d_u32
    {
        Some(d) => { println!("d_u32 = {:032b}", d); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(d_u32, None);
            }
    }

    // Example for u64
    let a_u64 = u64::generate_check_bits(50);
    match a_u64
    {
        Some(a) => {
                println!("a_u64 = {:064b}", a);
                assert_eq!(a, 0b0000000000000100000000000000000000000000000000000000000000000000_u64);
            },
        None => { println!("bit_pos is out of range."); }
    }

    let b_u64 = u64::generate_check_bits(70);
    match b_u64
    {
        Some(b) => { println!("b_u64 = {:064b}", b); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(b_u64, None);
            }
    }

    let c_u64 = small_uint_generate_check_bits_func::<u64>(50);
    match c_u64
    {
        Some(c) => {
                println!("c_u64 = {:064b}", c);
                assert_eq!(c, 0b0000000000000100000000000000000000000000000000000000000000000000_u64);
            },
        None => { println!("bit_pos is out of range."); }
    }

    let d_u64 = small_uint_generate_check_bits_func::<u64>(70);
    match d_u64
    {
        Some(d) => { println!("d_u64 = {:064b}", d); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(d_u64, None);
            }
    }

    // Example for u128
    let a_u128 = u128::generate_check_bits(100);
    match a_u128
    {
        Some(a) => {
                println!("a_u128 = {:0128b}", a);
                assert_eq!(a, 0b00000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000_u128);
            },
        None => { println!("bit_pos is out of range."); }
    }

    let b_u128 = u128::generate_check_bits(200);
    match b_u128
    {
        Some(b) => { println!("b_u128 = {:0128b}", b); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(b_u128, None);
            }
    }

    let c_u128 = small_uint_generate_check_bits_func::<u128>(100);
    match c_u128
    {
        Some(c) => {
                println!("c_u128 = {:0128b}", c);
                assert_eq!(c, 0b00000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000_u128);
            },
        None => { println!("bit_pos is out of range."); }
    }

    let d_u128 = small_uint_generate_check_bits_func::<u128>(200);
    match d_u128
    {
        Some(d) => { println!("d_u128 = {:0128b}", d); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(d_u128, None);
            }
    }

    // Example for usize
    let a_usize = usize::generate_check_bits(30);
    match a_usize
    {
        Some(a) => {
                println!("a_usize = {:064b}", a);
                assert_eq!(a, 0b0000000000000000000000000000000001000000000000000000000000000000_usize);
            },
        None => { println!("bit_pos is out of range."); }
    }

    let b_usize = usize::generate_check_bits(72);
    match b_usize
    {
        Some(b) => { println!("b_usize = {:064b}", b); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(b_usize, None);
            }
    }

    let c_usize = small_uint_generate_check_bits_func::<usize>(30);
    match c_usize
    {
        Some(c) => {
                println!("c_usize = {:064b}", c);
                assert_eq!(c, 0b0000000000000000000000000000000001000000000000000000000000000000_usize);
            },
        None => { println!("bit_pos is out of range."); }
    }

    let d_usize = small_uint_generate_check_bits_func::<usize>(72);
    match d_usize
    {
        Some(d) => { println!("d_usize = {:064b}", d); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(d_usize, None);
            }
    }

    // Example for ShortUnion
    let a_shortunion = ShortUnion::generate_check_bits(12);
    match a_shortunion
    {
        Some(a) => {
                println!("a_shortunion = {:016b}", a.get());
                assert_eq!(a.get(), 0b0001000000000000_u16);
            },
        None => { println!("bit_pos is out of range."); }
    }

    let b_shortunion = ShortUnion::generate_check_bits(16);
    match b_shortunion
    {
        Some(b) => { println!("b_shortunion = {:016b}", b.get()); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(b_shortunion, None);
            }
    }

    let c_shortunion = small_uint_generate_check_bits_func::<ShortUnion>(12);
    match c_shortunion
    {
        Some(c) => {
                println!("c_shortunion = {:016b}", c.get());
                assert_eq!(c.get(), 0b0001000000000000_u16);
            },
        None => { println!("bit_pos is out of range."); }
    }

    let d_shortunion = small_uint_generate_check_bits_func::<ShortUnion>(16);
    match d_shortunion
    {
        Some(d) => { println!("d_shortunion = {:016b}", d.get()); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(d_shortunion, None);
            }
    }

    // Example for IntUnion
    let a_intunion = IntUnion::generate_check_bits(20);
    match a_intunion
    {
        Some(a) => {
                println!("a_intunion = {:032b}", a.get());
                assert_eq!(a.get(), 0b00000000000100000000000000000000_u32);
            },
        None => { println!("bit_pos is out of range."); }
    }

    let b_intunion = IntUnion::generate_check_bits(40);
    match b_intunion
    {
        Some(b) => { println!("b_intunion = {:032b}", b.get()); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(b_intunion, None);
            }
    }

    let c_intunion = small_uint_generate_check_bits_func::<IntUnion>(20);
    match c_intunion
    {
        Some(c) => {
                println!("c_intunion = {:032b}", c.get());
                assert_eq!(c.get(), 0b00000000000100000000000000000000_u32);
            },
        None => { println!("bit_pos is out of range."); }
    }

    let d_intunion = small_uint_generate_check_bits_func::<IntUnion>(40);
    match d_intunion
    {
        Some(d) => { println!("d_intunion = {:032b}", d.get()); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(d_intunion, None);
            }
    }

    // Example for LongUnion
    let a_longunion = LongUnion::generate_check_bits(50);
    match a_longunion
    {
        Some(a) => {
                println!("a_longunion = {:064b}", a.get());
                assert_eq!(a.get(), 0b0000000000000100000000000000000000000000000000000000000000000000_u64);
            },
        None => { println!("bit_pos is out of range."); }
    }

    let b_longunion = LongUnion::generate_check_bits(70);
    match b_longunion
    {
        Some(b) => { println!("b_longunion = {:064b}", b.get()); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(b_longunion, None);
            }
    }

    let c_longunion = small_uint_generate_check_bits_func::<LongUnion>(50);
    match c_longunion
    {
        Some(c) => {
                println!("c_longunion = {:064b}", c.get());
                assert_eq!(c.get(), 0b0000000000000100000000000000000000000000000000000000000000000000_u64);
            },
        None => { println!("bit_pos is out of range."); }
    }

    let d_longunion = small_uint_generate_check_bits_func::<LongUnion>(70);
    match d_longunion
    {
        Some(d) => { println!("d_longunion = {:064b}", d.get()); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(d_longunion, None);
            }
    }

    // Example for LongerUnion
    let a_longerunion = LongerUnion::generate_check_bits(100);
    match a_longerunion
    {
        Some(a) => {
                println!("a_longerunion = {:0128b}", a.get());
                assert_eq!(a.get(), 0b00000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000_u128);
            },
        None => { println!("bit_pos is out of range."); }
    }

    let b_longerunion = LongerUnion::generate_check_bits(200);
    match b_longerunion
    {
        Some(b) => { println!("b_longerunion = {:0128b}", b.get()); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(b_longerunion, None);
            }
    }

    let c_longerunion = small_uint_generate_check_bits_func::<LongerUnion>(100);
    match c_longerunion
    {
        Some(c) => {
                println!("c_longerunion = {:0128b}", c.get());
                assert_eq!(c.get(), 0b00000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000_u128);
            },
        None => { println!("bit_pos is out of range."); }
    }

    let d_longerunion = small_uint_generate_check_bits_func::<LongerUnion>(200);
    match d_longerunion
    {
        Some(d) => { println!("d_longerunion = {:0128b}", d.get()); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(d_longerunion, None);
            }
    }

    // Example for SizeUnion
    let a_sizeunion = SizeUnion::generate_check_bits(30);
    match a_sizeunion
    {
        Some(a) => {
                println!("a_sizeunion = {:064b}", a.get());
                assert_eq!(a.get(), 0b0000000000000000000000000000000001000000000000000000000000000000_usize);
            },
        None => { println!("bit_pos is out of range."); }
    }

    let b_sizeunion = SizeUnion::generate_check_bits(72);
    match b_sizeunion
    {
        Some(b) => { println!("b_sizeunion = {:064b}", b.get()); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(b_sizeunion, None);
            }
    }

    let c_sizeunion = small_uint_generate_check_bits_func::<SizeUnion>(30);
    match c_sizeunion
    {
        Some(c) => {
                println!("c_sizeunion = {:064b}", c.get());
                assert_eq!(c.get(), 0b0000000000000000000000000000000001000000000000000000000000000000_usize);
            },
        None => { println!("bit_pos is out of range."); }
    }

    let d_sizeunion = small_uint_generate_check_bits_func::<SizeUnion>(72);
    match d_sizeunion
    {
        Some(d) => { println!("d_sizeunion = {:064b}", d.get()); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(d_sizeunion, None);
            }
    }
    println!("--------------------------------------");
}

fn small_uint_generate_check_bits_func<T: cryptocol::number::SmallUInt>(bit_pos: usize) -> Option<T>
{
    cryptocol::number::SmallUInt::generate_check_bits(bit_pos)
}

fn small_uint_generate_check_bits_()
{
    println!("small_uint_generate_check_bits_");
    use cryptocol::number::{ SmallUInt, ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };
    // Example for u8
    let a_u8 = u8::generate_check_bits_(3);
    println!("a_u8 = {:08b}", a_u8);
    assert_eq!(a_u8, 0b00001000_u8);
    // It will panic.
    // let b_u8 = u8::generate_check_bits_(9);

    let c_u8 = small_uint_generate_check_bits__func::<u8>(3);
    println!("c_u8 = {:08b}", c_u8);
    assert_eq!(c_u8, 0b00001000_u8);
    // It will panic.
    // let d_u8 = small_uint_generate_check_bits__func::<u8>(9);

    // Example for u16
    let a_u16 = u16::generate_check_bits_(12);
    println!("a_u16 = {:016b}", a_u16);
    assert_eq!(a_u16, 0b0001000000000000_u16);
    // It will panic.
    // let b_u16 = u16::generate_check_bits_(16);

    let c_u16 = small_uint_generate_check_bits__func::<u16>(12);
    println!("c_u16 = {:016b}", c_u16);
    assert_eq!(c_u16, 0b0001000000000000_u16);
    // It will panic.
    // let d_u16 = small_uint_generate_check_bits__func::<u16>(16);

    // Example for u32
    let a_u32 = u32::generate_check_bits_(20);
    println!("a_u32 = {:032b}", a_u32);
    assert_eq!(a_u32, 0b00000000000100000000000000000000_u32);
    // It will panic.
    // let b_u32 = u32::generate_check_bits_(40);

    let c_u32 = small_uint_generate_check_bits__func::<u32>(20);
    println!("c_u32 = {:032b}", c_u32);
    assert_eq!(c_u32, 0b00000000000100000000000000000000_u32);
    // It will panic.
    // let d_u32 = small_uint_generate_check_bits__func::<u32>(40);

    // Example for u64
    let a_u64 = u64::generate_check_bits_(50);
    println!("a_u64 = {:064b}", a_u64);
    assert_eq!(a_u64, 0b0000000000000100000000000000000000000000000000000000000000000000_u64);
    // It will panic.
    // let b_u64 = u64::generate_check_bits_(70);

    let c_u64 = small_uint_generate_check_bits__func::<u64>(50);
    println!("c_u64 = {:064b}", c_u64);
    assert_eq!(c_u64, 0b0000000000000100000000000000000000000000000000000000000000000000_u64);
    // It will panic.
    // let d_u64 = small_uint_generate_check_bits__func::<u64>(70);

    // Example for u128
    let a_u128 = u128::generate_check_bits_(100);
    println!("a_u128 = {:0128b}", a_u128);
    assert_eq!(a_u128, 0b00000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000_u128);
    // It will panic.
    // let b_u128 = u128::generate_check_bits_(200);

    let c_u128 = small_uint_generate_check_bits__func::<u128>(100);
    println!("c_u128 = {:0128b}", c_u128);
    assert_eq!(c_u128, 0b00000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000_u128);
    // It will panic.
    // let d_u128 = small_uint_generate_check_bits__func::<u128>(200);

    // Example for usize
    let a_usize = usize::generate_check_bits_(30);
    println!("a_usize = {:064b}", a_usize);
    assert_eq!(a_usize, 0b0000000000000000000000000000000001000000000000000000000000000000_usize);
    // It will panic.
    // let b_usize = usize::generate_check_bits_(72);

    let c_usize = small_uint_generate_check_bits__func::<usize>(30);
    println!("c_usize = {:064b}", c_usize);
    assert_eq!(c_usize, 0b0000000000000000000000000000000001000000000000000000000000000000_usize);
    // It will panic.
    // let d_usize = small_uint_generate_check_bits__func::<usize>(72);

    // Example for ShortUnion
    let a_shortunion = ShortUnion::generate_check_bits_(12);
    println!("a_shortunion = {:016b}", a_shortunion.get());
    assert_eq!(a_shortunion.get(), 0b0001000000000000_u16);
    // It will panic.
    // let b_shortunion = ShortUnion::generate_check_bits_(16);

    let c_shortunion = small_uint_generate_check_bits__func::<ShortUnion>(12);
    println!("c_shortunion = {:016b}", c_shortunion.get());
    assert_eq!(c_shortunion.get(), 0b0001000000000000_u16);
    // It will panic.
    // let d_shortunion = small_uint_generate_check_bits__func::<ShortUnion>(16);

    // Example for IntUnion
    let a_intunion = IntUnion::generate_check_bits_(20);
    println!("a_intunion = {:032b}", a_intunion.get());
    assert_eq!(a_intunion.get(), 0b00000000000100000000000000000000_u32);
    // It will panic.
    // let b_intunion = IntUnion::generate_check_bits_(40);

    let c_intunion = small_uint_generate_check_bits__func::<IntUnion>(20);
    println!("c_intunion = {:032b}", c_intunion.get());
    assert_eq!(c_intunion.get(), 0b00000000000100000000000000000000_u32);
    // It will panic.
    // let d_intunion = small_uint_generate_check_bits__func::<IntUnion>(40);

    // Example for LongUnion
    let a_longunion = LongUnion::generate_check_bits_(50);
    println!("a_longunion = {:064b}", a_longunion.get());
    assert_eq!(a_longunion.get(), 0b0000000000000100000000000000000000000000000000000000000000000000_u64);
    // It will panic.
    // let b_longunion = LongUnion::generate_check_bits_(70);

    let c_longunion = small_uint_generate_check_bits__func::<LongUnion>(50);
    println!("c_longunion = {:064b}", c_longunion.get());
    assert_eq!(c_longunion.get(), 0b0000000000000100000000000000000000000000000000000000000000000000_u64);
    // It will panic.
    // let d_longunion = small_uint_generate_check_bits__func::<LongUnion>(70);

    // Example for LongerUnion
    let a_longerunion = LongerUnion::generate_check_bits_(100);
    println!("a_longerunion = {:0128b}", a_longerunion.get());
    assert_eq!(a_longerunion.get(), 0b00000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000_u128);
    // It will panic.
    // let b_longerunion = LongerUnion::generate_check_bits_(200);

    let c_longerunion = small_uint_generate_check_bits__func::<LongerUnion>(100);
    println!("c_longerunion = {:0128b}", c_longerunion.get());
    assert_eq!(c_longerunion.get(), 0b00000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000_u128);
    // It will panic.
    // let d_longerunion = small_uint_generate_check_bits__func::<LongerUnion>(200);

    // Example for SizeUnion
    let a_sizeunion = SizeUnion::generate_check_bits_(30);
    println!("a_sizeunion = {:064b}", a_sizeunion.get());
    assert_eq!(a_sizeunion.get(), 0b0000000000000000000000000000000001000000000000000000000000000000_usize);
    // It will panic.
    // let b_sizeunion = SizeUnion::generate_check_bits_(72);

    let c_sizeunion = small_uint_generate_check_bits__func::<SizeUnion>(30);
    println!("c_sizeunion = {:064b}", c_sizeunion.get());
    assert_eq!(c_sizeunion.get(), 0b0000000000000000000000000000000001000000000000000000000000000000_usize);
    // It will panic.
    // let d_sizeunion = small_uint_generate_check_bits__func::<SizeUnion>(72);
    println!("--------------------------------------");
}

#[allow(non_snake_case)]
fn small_uint_generate_check_bits__func<T: cryptocol::number::SmallUInt>(bit_pos: usize) -> T
{
    cryptocol::number::SmallUInt::generate_check_bits_(bit_pos)
}

fn small_uint_is_odd()
{
    println!("small_uint_is_odd");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = 123_u8;
    let a_odd = a_u8.is_odd();
    println!("{} is {}.", a_u8, if a_odd {"odd"} else {"even"});
    assert!(a_odd);

    let b_u8 = 210_u8;
    let b_odd = b_u8.is_odd();
    println!("{} is {}.", b_u8, if b_odd {"odd"} else {"even"});
    assert!(!b_odd);

    let c_u8 = 123_u8;
    let c_odd = small_uint_is_odd_func(c_u8);
    println!("{} is {}.", c_u8, if c_odd {"odd"} else {"even"});
    assert!(c_odd);

    let d_u8 = 210_u8;
    let d_odd = small_uint_is_odd_func(d_u8);
    println!("{} is {}.", d_u8, if d_odd {"odd"} else {"even"});
    assert!(!d_odd);

    // Example for u16
    let a_u16 = 12345_u16;
    let a_odd = a_u16.is_odd();
    println!("{} is {}.", a_u16, if a_odd {"odd"} else {"even"});
    assert!(a_odd);

    let b_u16 = 65432_u16;
    let b_odd = b_u16.is_odd();
    println!("{} is {}.", b_u16, if b_odd {"odd"} else {"even"});
    assert!(!b_odd);

    let c_u16 = 12345_u16;
    let c_odd = small_uint_is_odd_func(c_u16);
    println!("{} is {}.", c_u16, if c_odd {"odd"} else {"even"});
    assert!(c_odd);

    let d_u16 = 65432_u16;
    let d_odd = small_uint_is_odd_func(d_u16);
    println!("{} is {}.", d_u16, if d_odd {"odd"} else {"even"});
    assert!(!d_odd);

    // Example for u32
    let a_u32 = 123456789_u32;
    let a_odd = a_u32.is_odd();
    println!("{} is {}.", a_u32, if a_odd {"odd"} else {"even"});
    assert!(a_odd);

    let b_u32 = 876543210_u32;
    let b_odd = b_u32.is_odd();
    println!("{} is {}.", b_u32, if b_odd {"odd"} else {"even"});
    assert!(!b_odd);

    let c_u32 = 123456789_u32;
    let c_odd = small_uint_is_odd_func(c_u32);
    println!("{} is {}.", c_u32, if c_odd {"odd"} else {"even"});
    assert!(c_odd);

    let d_u32 = 876543210_u32;
    let d_odd = small_uint_is_odd_func(d_u32);
    println!("{} is {}.", d_u32, if d_odd {"odd"} else {"even"});
    assert!(!d_odd);

    // Example for u64
    let a_u64 = 12345678924681357915_u64;
    let a_odd = a_u64.is_odd();
    println!("{} is {}.", a_u64, if a_odd {"odd"} else {"even"});
    assert!(a_odd);

    let b_u64 = 2468135791234567892_u64;
    let b_odd = b_u64.is_odd();
    println!("{} is {}.", b_u64, if b_odd {"odd"} else {"even"});
    assert!(!b_odd);

    let c_u64 = 12345678924681357915_u64;
    let c_odd = small_uint_is_odd_func(c_u64);
    println!("{} is {}.", c_u64, if c_odd {"odd"} else {"even"});
    assert!(c_odd);

    let d_u64 = 2468135791234567892_u64;
    let d_odd = small_uint_is_odd_func(d_u64);
    println!("{} is {}.", d_u64, if d_odd {"odd"} else {"even"});
    assert!(!d_odd);

    // Example for u128
    let a_u128 = 12345678924681357915987654321_u128;
    let a_odd = a_u128.is_odd();
    println!("{} is {}.", a_u128, if a_odd {"odd"} else {"even"});
    assert!(a_odd);

    let b_u128 = 24681357912345678921234567890_u128;
    let b_odd = b_u128.is_odd();
    println!("{} is {}.", b_u128, if b_odd {"odd"} else {"even"});
    assert!(!b_odd);

    let c_u128 = 12345678924681357915987654321_u128;
    let c_odd = small_uint_is_odd_func(c_u128);
    println!("{} is {}.", c_u128, if c_odd {"odd"} else {"even"});
    assert!(c_odd);

    let d_128 = 24681357912345678921234567890_u128;
    let d_odd = small_uint_is_odd_func(d_128);
    println!("{} is {}.", d_128, if d_odd {"odd"} else {"even"});
    assert!(!d_odd);

    // Example for usize
    let a_usize = 12345678924681357915_usize;
    let a_odd = a_usize.is_odd();
    println!("{} is {}.", a_usize, if a_odd {"odd"} else {"even"});
    assert!(a_odd);

    let b_usize = 2468135791234567892_usize;
    let b_odd = b_usize.is_odd();
    println!("{} is {}.", b_usize, if b_odd {"odd"} else {"even"});
    assert!(!b_odd);

    let c_usize = 12345678924681357915_usize;
    let c_odd = small_uint_is_odd_func(c_usize);
    println!("{} is {}.", c_usize, if c_odd {"odd"} else {"even"});
    assert!(c_odd);

    let d_usize = 2468135791234567892_usize;
    let d_odd = small_uint_is_odd_func(d_usize);
    println!("{} is {}.", d_usize, if d_odd {"odd"} else {"even"});
    assert!(!d_odd);

    // Example for ShortUnion
    let a_shortunion = 12345_u16.into_shortunion();
    let a_odd = a_shortunion.is_odd();
    println!("{} is {}.", a_shortunion, if a_odd {"odd"} else {"even"});
    assert!(a_odd);

    let b_shortunion = 65432_u16.into_shortunion();
    let b_odd = b_shortunion.is_odd();
    println!("{} is {}.", b_shortunion, if b_odd {"odd"} else {"even"});
    assert!(!b_odd);

    let c_shortunion = 12345_u16.into_shortunion();
    let c_odd = small_uint_is_odd_func(c_shortunion);
    println!("{} is {}.", c_shortunion, if c_odd {"odd"} else {"even"});
    assert!(c_odd);

    let d_shortunion = 65432_u16.into_shortunion();
    let d_odd = small_uint_is_odd_func(d_shortunion);
    println!("{} is {}.", d_shortunion, if d_odd {"odd"} else {"even"});
    assert!(!d_odd);

    // Example for IntUnion
    let a_intunion = 123456789_u32.into_intunion();
    let a_odd = a_intunion.is_odd();
    println!("{} is {}.", a_intunion, if a_odd {"odd"} else {"even"});
    assert!(a_odd);

    let b_intunion = 876543210_u32.into_intunion();
    let b_odd = b_intunion.is_odd();
    println!("{} is {}.", b_intunion, if b_odd {"odd"} else {"even"});
    assert!(!b_odd);

    let c_intunion = 123456789_u32.into_intunion();
    let c_odd = small_uint_is_odd_func(c_intunion);
    println!("{} is {}.", c_intunion, if c_odd {"odd"} else {"even"});
    assert!(c_odd);

    let d_intunion = 876543210_u32.into_intunion();
    let d_odd = small_uint_is_odd_func(d_intunion);
    println!("{} is {}.", d_intunion, if d_odd {"odd"} else {"even"});
    assert!(!d_odd);

    // Example for LongUnion
    let a_longunion = 12345678924681357915_u64.into_longunion();
    let a_odd = a_longunion.is_odd();
    println!("{} is {}.", a_longunion, if a_odd {"odd"} else {"even"});
    assert!(a_odd);

    let b_longunion = 2468135791234567892_u64.into_longunion();
    let b_odd = b_longunion.is_odd();
    println!("{} is {}.", b_longunion, if b_odd {"odd"} else {"even"});
    assert!(!b_odd);

    let c_longunion = 12345678924681357915_u64.into_longunion();
    let c_odd = small_uint_is_odd_func(c_longunion);
    println!("{} is {}.", c_longunion, if c_odd {"odd"} else {"even"});
    assert!(c_odd);

    let d_longunion = 2468135791234567892_u64.into_longunion();
    let d_odd = small_uint_is_odd_func(d_longunion);
    println!("{} is {}.", d_longunion, if d_odd {"odd"} else {"even"});
    assert!(!d_odd);

    // Example for LongerUnion
    let a_longerunion = 12345678924681357915987654321_u128.into_longerunion();
    let a_odd = a_longerunion.is_odd();
    println!("{} is {}.", a_longerunion, if a_odd {"odd"} else {"even"});
    assert!(a_odd);

    let b_longerunion = 24681357912345678921234567890_u128.into_longerunion();
    let b_odd = b_longerunion.is_odd();
    println!("{} is {}.", b_longerunion, if b_odd {"odd"} else {"even"});
    assert!(!b_odd);

    let c_longerunion = 12345678924681357915987654321_u128.into_longerunion();
    let c_odd = small_uint_is_odd_func(c_longerunion);
    println!("{} is {}.", c_longerunion, if c_odd {"odd"} else {"even"});
    assert!(c_odd);

    let d_longerunion = 24681357912345678921234567890_u128.into_longerunion();
    let d_odd = small_uint_is_odd_func(d_longerunion);
    println!("{} is {}.", d_longerunion, if d_odd {"odd"} else {"even"});
    assert!(!d_odd);

    // Example for SizeUnion
    let a_sizeunion = 12345678924681357915_usize.into_sizeunion();
    let a_odd = a_sizeunion.is_odd();
    println!("{} is {}.", a_sizeunion, if a_odd {"odd"} else {"even"});
    assert!(a_odd);

    let b_sizeunion = 2468135791234567892_usize.into_sizeunion();
    let b_odd = b_sizeunion.is_odd();
    println!("{} is {}.", b_sizeunion, if b_odd {"odd"} else {"even"});
    assert!(!b_odd);

    let c_sizeunion = 12345678924681357915_usize.into_sizeunion();
    let c_odd = small_uint_is_odd_func(c_sizeunion);
    println!("{} is {}.", c_sizeunion, if c_odd {"odd"} else {"even"});
    assert!(c_odd);

    let d_sizeunion = 2468135791234567892_usize.into_sizeunion();
    let d_odd = small_uint_is_odd_func(d_sizeunion);
    println!("{} is {}.", d_sizeunion, if d_odd {"odd"} else {"even"});
    assert!(!d_odd);
    println!("--------------------------------------");
}

fn small_uint_is_odd_func<T: cryptocol::number::SmallUInt>(num: T) -> bool
{
    num.is_odd()
}

fn small_uint_is_even()
{
    println!("small_uint_is_even");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = 210_u8;
    let a_even = a_u8.is_even();
    println!("{} is {}.", a_u8, if a_even {"even"} else {"odd"});
    assert!(a_even);

    let b_u8 = 123_u8;
    let b_even = b_u8.is_even();
    println!("{} is {}.", b_u8, if b_even {"even"} else {"odd"});
    assert!(!b_even);

    let c_u8 = 210_u8;
    let c_even = small_uint_is_even_func(c_u8);
    println!("{} is {}.", c_u8, if c_even {"even"} else {"odd"});
    assert!(c_even);

    let d_u8 = 123_u8;
    let d_even = small_uint_is_even_func(d_u8);
    println!("{} is {}.", d_u8, if d_even {"even"} else {"odd"});
    assert!(!d_even);

    // Example for u16
    let a_u16 = 65432_u16;
    let a_even = a_u16.is_even();
    println!("{} is {}.", a_u16, if a_even {"even"} else {"odd"});
    assert!(a_even);

    let b_u16 = 12345_u16;
    let b_even = b_u16.is_even();
    println!("{} is {}.", b_u16, if b_even {"even"} else {"odd"});
    assert!(!b_even);

    let c_u16 = 65432_u16;
    let c_even = small_uint_is_even_func(c_u16);
    println!("{} is {}.", c_u16, if c_even {"even"} else {"odd"});
    assert!(c_even);

    let d_u16 = 12345_u16;
    let d_even = small_uint_is_even_func(d_u16);
    println!("{} is {}.", d_u16, if d_even {"even"} else {"odd"});
    assert!(!d_even);

    // Example for u32
    let a_u32 = 876543210_u32;
    let a_even = a_u32.is_even();
    println!("{} is {}.", a_u32, if a_even {"even"} else {"odd"});
    assert!(a_even);

    let b_u32 = 123456789_u32;
    let b_even = b_u32.is_even();
    println!("{} is {}.", b_u32, if b_even {"even"} else {"odd"});
    assert!(!b_even);

    let c_u32 = 876543210_u32;
    let c_even = small_uint_is_even_func(c_u32);
    println!("{} is {}.", c_u32, if c_even {"even"} else {"odd"});
    assert!(c_even);

    let d_u32 = 123456789_u32;
    let d_even = small_uint_is_even_func(d_u32);
    println!("{} is {}.", d_u32, if d_even {"even"} else {"odd"});
    assert!(!d_even);

    // Example for u64
    let a_u64 = 2468135791234567892_u64;
    let a_even = a_u64.is_even();
    println!("{} is {}.", a_u64, if a_even {"even"} else {"odd"});
    assert!(a_even);

    let b_u64 = 12345678924681357915_u64;
    let b_even = b_u64.is_even();
    println!("{} is {}.", b_u64, if b_even {"even"} else {"odd"});
    assert!(!b_even);

    let c_u64 = 2468135791234567892_u64;
    let c_even = small_uint_is_even_func(c_u64);
    println!("{} is {}.", c_u64, if c_even {"even"} else {"odd"});
    assert!(c_even);

    let d_u64 = 12345678924681357915_u64;
    let d_even = small_uint_is_even_func(d_u64);
    println!("{} is {}.", d_u64, if d_even {"even"} else {"odd"});
    assert!(!d_even);

    // Example for u128
    let a_u128 = 24681357912345678921234567890_u128;
    let a_even = a_u128.is_even();
    println!("{} is {}.", a_u128, if a_even {"even"} else {"odd"});
    assert!(a_even);

    let b_u128 = 12345678924681357915987654321_u128;
    let b_even = b_u128.is_even();
    println!("{} is {}.", b_u128, if b_even {"even"} else {"odd"});
    assert!(!b_even);

    let c_u128 = 24681357912345678921234567890_u128;
    let c_even = small_uint_is_even_func(c_u128);
    println!("{} is {}.", c_u128, if c_even {"even"} else {"odd"});
    assert!(c_even);

    let d_u128 = 12345678924681357915987654321_u128;
    let d_even = small_uint_is_even_func(d_u128);
    println!("{} is {}.", d_u128, if d_even {"even"} else {"odd"});
    assert!(!d_even);

    // Example for usize
    let a_usize = 2468135791234567892_usize;
    let a_even = a_usize.is_even();
    println!("{} is {}.", a_usize, if a_even {"even"} else {"odd"});
    assert!(a_even);

    let b_usize = 12345678924681357915_usize;
    let b_even = b_usize.is_even();
    println!("{} is {}.", b_usize, if b_even {"even"} else {"odd"});
    assert!(!b_even);

    let c_usize = 2468135791234567892_usize;
    let c_even = small_uint_is_even_func(c_usize);
    println!("{} is {}.", c_usize, if c_even {"even"} else {"odd"});
    assert!(c_even);

    let d_usize = 12345678924681357915_usize;
    let d_even = small_uint_is_even_func(d_usize);
    println!("{} is {}.", d_usize, if d_even {"even"} else {"odd"});
    assert!(!d_even);

    // Example for ShortUnion
    let a_shortunion = 65432_u16.into_shortunion();
    let a_even = a_shortunion.is_even();
    println!("{} is {}.", a_shortunion, if a_even {"even"} else {"odd"});
    assert!(a_even);

    let b_shortunion = 12345_u16.into_shortunion();
    let b_even = b_shortunion.is_even();
    println!("{} is {}.", b_shortunion, if b_even {"even"} else {"odd"});
    assert!(!b_even);

    let c_shortunion = 65432_u16.into_shortunion();
    let c_even = small_uint_is_even_func(c_shortunion);
    println!("{} is {}.", c_shortunion, if c_even {"even"} else {"odd"});
    assert!(c_even);

    let d_shortunion = 12345_u16.into_shortunion();
    let d_even = small_uint_is_even_func(d_shortunion);
    println!("{} is {}.", d_shortunion, if d_even {"even"} else {"odd"});
    assert!(!d_even);

    // Example for IntUnion
    let a_intunion = 876543210_u32.into_intunion();
    let a_even = a_intunion.is_even();
    println!("{} is {}.", a_intunion, if a_even {"even"} else {"odd"});
    assert!(a_even);

    let b_intunion = 123456789_u32.into_intunion();
    let b_even = b_intunion.is_even();
    println!("{} is {}.", b_intunion, if b_even {"even"} else {"odd"});
    assert!(!b_even);

    let c_intunion = 876543210_u32.into_intunion();
    let c_even = small_uint_is_even_func(c_intunion);
    println!("{} is {}.", c_intunion, if c_even {"even"} else {"odd"});
    assert!(c_even);

    let d_intunion = 123456789_u32.into_intunion();
    let d_even = small_uint_is_even_func(d_intunion);
    println!("{} is {}.", d_intunion, if d_even {"even"} else {"odd"});
    assert!(!d_even);

    // Example for LongUnion
    let a_longunion = 2468135791234567892_u64.into_longunion();
    let a_even = a_longunion.is_even();
    println!("{} is {}.", a_longunion, if a_even {"even"} else {"odd"});
    assert!(a_even);

    let b_longunion = 12345678924681357915_u64.into_longunion();
    let b_even = b_longunion.is_even();
    println!("{} is {}.", b_longunion, if b_even {"even"} else {"odd"});
    assert!(!b_even);

    let c_longunion = 2468135791234567892_u64.into_longunion();
    let c_even = small_uint_is_even_func(c_longunion);
    println!("{} is {}.", c_longunion, if c_even {"even"} else {"odd"});
    assert!(c_even);

    let d_longunion = 12345678924681357915_u64.into_longunion();
    let d_even = small_uint_is_even_func(d_longunion);
    println!("{} is {}.", d_longunion, if d_even {"even"} else {"odd"});
    assert!(!d_even);

    // Example for LongerUnion
    let a_longerunion = 24681357912345678921234567890_u128.into_longerunion();
    let a_even = a_longerunion.is_even();
    println!("{} is {}.", a_longerunion, if a_even {"even"} else {"odd"});
    assert!(a_even);

    let b_longerunion = 12345678924681357915987654321_u128.into_longerunion();
    let b_even = b_longerunion.is_even();
    println!("{} is {}.", b_longerunion, if b_even {"even"} else {"odd"});
    assert!(!b_even);

    let c_longerunion = 24681357912345678921234567890_u128.into_longerunion();
    let c_even = small_uint_is_even_func(c_longerunion);
    println!("{} is {}.", c_longerunion, if c_even {"even"} else {"odd"});
    assert!(c_even);

    let d_longerunion = 12345678924681357915987654321_u128.into_longerunion();
    let d_even = small_uint_is_even_func(d_longerunion);
    println!("{} is {}.", d_longerunion, if d_even {"even"} else {"odd"});
    assert!(!d_even);

    // Example for SizeUnion
    let a_sizeunion = 2468135791234567892_usize.into_sizeunion();
    let a_even = a_sizeunion.is_even();
    println!("{} is {}.", a_sizeunion, if a_even {"even"} else {"odd"});
    assert!(a_even);

    let b_sizeunion = 12345678924681357915_usize.into_sizeunion();
    let b_even = b_sizeunion.is_even();
    println!("{} is {}.", b_sizeunion, if b_even {"even"} else {"odd"});
    assert!(!b_even);

    let c_sizeunion = 2468135791234567892_usize.into_sizeunion();
    let c_even = small_uint_is_even_func(c_sizeunion);
    println!("{} is {}.", c_sizeunion, if c_even {"even"} else {"odd"});
    assert!(c_even);

    let d_sizeunion = 12345678924681357915_usize.into_sizeunion();
    let d_even = small_uint_is_even_func(d_sizeunion);
    println!("{} is {}.", d_sizeunion, if d_even {"even"} else {"odd"});
    assert!(!d_even);
    println!("--------------------------------------");
}

fn small_uint_is_even_func<T: cryptocol::number::SmallUInt>(num: T) -> bool
{
    num.is_even()
}

fn small_uint_is_msb_set()
{
    println!("small_uint_is_msb_set");
    use cryptocol::number::SmallUInt;
    let str_g = "greater than";
    let str_l = "less than or equal to";

    // Example for u8
    let half_u8 = u8::MAX / 2;
    let a_u8 = 200_u8;
    let a_set = a_u8.is_msb_set();
    println!("{} is {} {}.", a_u8, if a_set {str_g} else {str_l}, half_u8);
    assert!(a_set);

    let b_u8 = 100_u8;
    let b_set = b_u8.is_msb_set();
    println!("{} is {} {}.", b_u8, if b_set {str_g} else {str_l}, half_u8);
    assert!(!b_set);

    let half_u8 = u8::MAX / 2;
    let c_u8 = 200_u8;
    let c_set = small_uint_is_msb_set_func(c_u8);
    println!("{} is {} {}.", c_u8, if c_set {str_g} else {str_l}, half_u8);
    assert!(c_set);

    let d_u8 = 100_u8;
    let d_set = small_uint_is_msb_set_func(d_u8);
    println!("{} is {} {}.", d_u8, if d_set {str_g} else {str_l}, half_u8);
    assert!(!d_set);

    // Example for u16
    let half_u16 = u16::MAX / 2;
    let a_u16 = 60000_u16;
    let a_set = a_u16.is_msb_set();
    println!("{} is {} {}.", a_u16, if a_set {str_g} else {str_l}, half_u16);
    assert!(a_set);

    let b_u16 = 30000_u16;
    let b_set = b_u16.is_msb_set();
    println!("{} is {} {}.", b_u16, if b_set {str_g} else {str_l}, half_u16);
    assert!(!b_set);

    let half_u16 = u16::MAX / 2;
    let c_u16 = 60000_u16;
    let c_set = small_uint_is_msb_set_func(c_u16);
    println!("{} is {} {}.", c_u16, if c_set {str_g} else {str_l}, half_u16);
    assert!(c_set);

    let d_u16 = 30000_u16;
    let d_set = small_uint_is_msb_set_func(d_u16);
    println!("{} is {} {}.", d_u16, if d_set {str_g} else {str_l}, half_u16);
    assert!(!d_set);

    // Example for u32
    let half_u32 = u32::MAX / 2;
    let a_u32 = 4000000000_u32;
    let a_set = a_u32.is_msb_set();
    println!("{} is {} {}.", a_u32, if a_set {str_g} else {str_l}, half_u32);
    assert!(a_set);

    let b_u32 = 2000000000_u32;
    let b_set = b_u32.is_msb_set();
    println!("{} is {} {}.", b_u32, if b_set {str_g} else {str_l}, half_u32);
    assert!(!b_set);

    let half_u32 = u32::MAX / 2;
    let c_u32 = 4000000000_u32;
    let c_set = small_uint_is_msb_set_func(c_u32);
    println!("{} is {} {}.", c_u32, if c_set {str_g} else {str_l}, half_u32);
    assert!(c_set);

    let d_u32 = 2000000000_u32;
    let d_set = small_uint_is_msb_set_func(d_u32);
    println!("{} is {} {}.", d_u32, if d_set {str_g} else {str_l}, half_u32);
    assert!(!d_set);

    // Example for u64
    let half_u64 = u64::MAX / 2;
    let a_u64 = 10000000000000000000_u64;
    let a_set = a_u64.is_msb_set();
    println!("{} is {} {}.", a_u64, if a_set {str_g} else {str_l}, half_u64);
    assert!(a_set);

    let b_u64 = 5000000000000000000_u64;
    let b_set = b_u64.is_msb_set();
    println!("{} is {} {}.", b_u64, if b_set {str_g} else {str_l}, half_u64);
    assert!(!b_set);

    let half_u64 = u64::MAX / 2;
    let c_u64 = 10000000000000000000_u64;
    let c_set = small_uint_is_msb_set_func(c_u64);
    println!("{} is {} {}.", c_u64, if c_set {str_g} else {str_l}, half_u64);
    assert!(c_set);

    let d_u64 = 5000000000000000000_u64;
    let d_set = small_uint_is_msb_set_func(d_u64);
    println!("{} is {} {}.", d_u64, if d_set {str_g} else {str_l}, half_u64);
    assert!(!d_set);

    // Example for u128
    let half_u128 = u128::MAX / 2;
    let a_u128 = 200000000000000000000000000000000000000_u128;
    let a_set = a_u128.is_msb_set();
    println!("{} is {} {}.", a_u128, if a_set {str_g} else {str_l}, half_u128);
    assert!(a_set);

    let b_u128 = 100000000000000000000000000000000000000_u128;
    let b_set = b_u128.is_msb_set();
    println!("{} is {} {}.", b_u128, if b_set {str_g} else {str_l}, half_u128);
    assert!(!b_set);

    let half_u128 = u128::MAX / 2;
    let c_u128 = 200000000000000000000000000000000000000_u128;
    let c_set = small_uint_is_msb_set_func(c_u128);
    println!("{} is {} {}.", c_u128, if c_set {str_g} else {str_l}, half_u128);
    assert!(c_set);

    let d_u128 = 100000000000000000000000000000000000000_u128;
    let d_set = small_uint_is_msb_set_func(d_u128);
    println!("{} is {} {}.", d_u128, if d_set {str_g} else {str_l}, half_u128);
    assert!(!d_set);

    // Example for usize
    let half_usize = usize::MAX / 2;
    let a_usize = 10000000000000000000_usize;
    let a_set = a_usize.is_msb_set();
    println!("{} is {} {}.", a_usize, if a_set {str_g} else {str_l}, half_usize);
    assert!(a_set);

    let b_usize = 5000000000000000000_usize;
    let b_set = b_usize.is_msb_set();
    println!("{} is {} {}.", b_usize, if b_set {str_g} else {str_l}, half_usize);
    assert!(!b_set);

    let half_usize = usize::MAX / 2;
    let c_usize = 10000000000000000000_usize;
    let c_set = small_uint_is_msb_set_func(c_usize);
    println!("{} is {} {}.", c_usize, if c_set {str_g} else {str_l}, half_usize);
    assert!(c_set);

    let d_usize = 5000000000000000000_usize;
    let d_set = small_uint_is_msb_set_func(d_usize);
    println!("{} is {} {}.", d_usize, if d_set {str_g} else {str_l}, half_usize);
    assert!(!d_set);

    // Example for ShortUnion
    let half_shortunion = (u16::MAX / 2).into_shortunion();
    let a_shortunion = 60000_u16.into_shortunion();
    let a_set = a_shortunion.is_msb_set();
    println!("{} is {} {}.", a_shortunion, if a_set {str_g} else {str_l}, half_shortunion);
    assert!(a_set);

    let b_shortunion = 30000_u16.into_shortunion();
    let b_set = b_shortunion.is_msb_set();
    println!("{} is {} {}.", b_shortunion, if b_set {str_g} else {str_l}, half_shortunion);
    assert!(!b_set);

    let half_shortunion = (u16::MAX / 2).into_shortunion();
    let c_shortunion = 60000_u16.into_shortunion();
    let c_set = small_uint_is_msb_set_func(c_shortunion);
    println!("{} is {} {}.", c_shortunion, if c_set {str_g} else {str_l}, half_shortunion);
    assert!(c_set);

    let d_shortunion = 30000_u16.into_shortunion();
    let d_set = small_uint_is_msb_set_func(d_shortunion);
    println!("{} is {} {}.", d_shortunion, if d_set {str_g} else {str_l}, half_shortunion);
    assert!(!d_set);

    // Example for IntUnion
    let half_intunion = (u32::MAX / 2).into_intunion();
    let a_intunion = 4000000000_u32.into_intunion();
    let a_set = a_intunion.is_msb_set();
    println!("{} is {} {}.", a_intunion, if a_set {str_g} else {str_l}, half_intunion);
    assert!(a_set);

    let b_intunion = 2000000000_u32.into_intunion();
    let b_set = b_intunion.is_msb_set();
    println!("{} is {} {}.", b_intunion, if b_set {str_g} else {str_l}, half_intunion);
    assert!(!b_set);

    let half_intunion = (u32::MAX / 2).into_intunion();
    let c_intunion = 4000000000_u32.into_intunion();
    let c_set = small_uint_is_msb_set_func(c_intunion);
    println!("{} is {} {}.", c_intunion, if c_set {str_g} else {str_l}, half_intunion);
    assert!(c_set);

    let d_intunion = 2000000000_u32.into_intunion();
    let d_set = small_uint_is_msb_set_func(d_intunion);
    println!("{} is {} {}.", d_intunion, if d_set {str_g} else {str_l}, half_intunion);
    assert!(!d_set);

    // Example for LongtUnion
    let half_longunion = (u64::MAX / 2).into_longunion();
    let a_longunion = 10000000000000000000_u64.into_longunion();
    let a_set = a_longunion.is_msb_set();
    println!("{} is {} {}.", a_longunion, if a_set {str_g} else {str_l}, half_longunion);
    assert!(a_set);

    let b_longunion = 5000000000000000000_u64.into_longunion();
    let b_set = small_uint_is_msb_set_func(b_longunion);
    println!("{} is {} {}.", b_longunion, if b_set {str_g} else {str_l}, half_longunion);
    assert!(!b_set);

    let half_longunion = (u64::MAX / 2).into_longunion();
    let c_longunion = 10000000000000000000_u64.into_longunion();
    let c_set = small_uint_is_msb_set_func(c_longunion);
    println!("{} is {} {}.", c_longunion, if c_set {str_g} else {str_l}, half_longunion);
    assert!(c_set);

    let d_longunion = 5000000000000000000_u64.into_longunion();
    let d_set = small_uint_is_msb_set_func(d_longunion);
    println!("{} is {} {}.", d_longunion, if d_set {str_g} else {str_l}, half_longunion);
    assert!(!d_set);

    // Example for LongertUnion
    let half_longerunion = (u128::MAX / 2).into_longerunion();
    let a_longerunion = 200000000000000000000000000000000000000_u128.into_longerunion();
    let a_set = a_longerunion.is_msb_set();
    println!("{} is {} {}.", a_longerunion, if a_set {str_g} else {str_l}, half_longerunion);
    assert!(a_set);

    let b_longerunion = 100000000000000000000000000000000000000_u128.into_longerunion();
    let b_set = b_longerunion.is_msb_set();
    println!("{} is {} {}.", b_longerunion, if b_set {str_g} else {str_l}, half_longerunion);
    assert!(!b_set);

    let half_longerunion = (u128::MAX / 2).into_longerunion();
    let c_longerunion = 200000000000000000000000000000000000000_u128.into_longerunion();
    let c_set = small_uint_is_msb_set_func(c_longerunion);
    println!("{} is {} {}.", c_longerunion, if c_set {str_g} else {str_l}, half_longerunion);
    assert!(c_set);

    let d_longerunion = 100000000000000000000000000000000000000_u128.into_longerunion();
    let d_set = small_uint_is_msb_set_func(d_longerunion);
    println!("{} is {} {}.", d_longerunion, if d_set {str_g} else {str_l}, half_longerunion);
    assert!(!d_set);

    // Example for SizeUnion
    let half_sizeunion = (usize::MAX / 2).into_sizeunion();
    let a_sizeunion = 10000000000000000000_usize.into_sizeunion();
    let a_set = a_sizeunion.is_msb_set();
    println!("{} is {} {}.", a_sizeunion, if a_set {str_g} else {str_l}, half_sizeunion);
    assert!(a_set);

    let b_sizeunione = 000000000000000000_usize.into_sizeunion();
    let b_set = small_uint_is_msb_set_func(b_sizeunione);
    println!("{} is {} {}.", b_sizeunione, if b_set {str_g} else {str_l}, half_sizeunion);
    assert!(!b_set);

    let half_sizeunion = (usize::MAX / 2).into_sizeunion();
    let c_sizeunion = 10000000000000000000_usize.into_sizeunion();
    let c_set = small_uint_is_msb_set_func(c_sizeunion);
    println!("{} is {} {}.", c_sizeunion, if c_set {str_g} else {str_l}, half_sizeunion);
    assert!(c_set);

    let d_sizeunione = 000000000000000000_usize.into_sizeunion();
    let d_set = small_uint_is_msb_set_func(d_sizeunione);
    println!("{} is {} {}.", d_sizeunione, if d_set {str_g} else {str_l}, half_sizeunion);
    assert!(!d_set);
    println!("--------------------------------------");
}

fn small_uint_is_msb_set_func<T: cryptocol::number::SmallUInt>(num: T) -> bool
{
    num.is_msb_set()
}

fn small_uint_is_bit_set()
{
    println!("small_uint_is_bit_set");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = 0b01001100_u8;
    let set = a_u8.is_bit_set(6);
    match set
    {
        Some(a) => {
                println!("The {}-th bit of {:08b} is set to be {}.", 6, a_u8, a as u8);
                assert_eq!(a, true);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = a_u8.is_bit_set(4);
    match set
    {
        Some(a) => {
                println!("The {}-th bit of {:08b} is set to be {}.", 4, a_u8, a as u8);
                assert_eq!(a, false);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = a_u8.is_bit_set(9);
    match set
    {
        Some(a) => { println!("The {}-th bit of {:08b} is set to be {}.", 9, a_u8, a as u8); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(set, None);
            }
    }

    let b_u8 = 0b01001100_u8;
    let set = small_uint_is_bit_set_func(b_u8, 6);
    match set
    {
        Some(b) => {
                println!("The {}-th bit of {:08b} is set to be {}.", 6, b_u8, b as u8);
                assert_eq!(b, true);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = small_uint_is_bit_set_func(b_u8, 4);
    match set
    {
        Some(b) => {
                println!("The {}-th bit of {:08b} is set to be {}.", 4, b_u8, b as u8);
                assert_eq!(b, false);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = small_uint_is_bit_set_func(b_u8, 9);
    match set
    {
        Some(b) => { println!("The {}-th bit of {:08b} is set to be {}.", 9, b_u8, b as u8); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(set, None);
            }
    }

    // Example for u16
    let a_u16 = 0b1011001110001111_u16;
    let set = a_u16.is_bit_set(7);
    match set
    {
        Some(a) => {
                println!("The {}-th bit of {:016b} is set to be {}.", 7, a_u16, a as u8);
                assert_eq!(a, true);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = a_u16.is_bit_set(10);
    match set
    {
        Some(a) => {
                println!("The {}-th bit of {:016b} is set to be {}.", 10, a_u16, a as u8);
                assert_eq!(a, false);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = a_u16.is_bit_set(16);
    match set
    {
        Some(a) => { println!("The {}-th bit of {:016b} is set to be {}.", 16, a_u16, a as u8); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(set, None);
            }
    }

    let b_u16 = 0b1011001110001111_u16;
    let set = small_uint_is_bit_set_func(b_u16, 7);
    match set
    {
        Some(b) => {
                println!("The {}-th bit of {:016b} is set to be {}.", 7, b_u16, b as u8);
                assert_eq!(b, true);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = small_uint_is_bit_set_func(b_u16, 10);
    match set
    {
        Some(b) => {
                println!("The {}-th bit of {:016b} is set to be {}.", 10, b_u16, b as u8);
                assert_eq!(b, false);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = small_uint_is_bit_set_func(b_u16, 16);
    match set
    {
        Some(b) => { println!("The {}-th bit of {:016b} is set to be {}.", 16, b_u16, b as u8); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(set, None);
            }
    }

    // Example for u32
    let a_u32 = 0b10110011100011110000111110000011_u32;
    let set = a_u32.is_bit_set(29);
    match set
    {
        Some(a) => {
                println!("The {}-th bit of {:032b} is set to be {}.", 29, a_u32, a as u8);
                assert_eq!(a, true);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = a_u32.is_bit_set(20);
    match set
    {
        Some(a) => {
                println!("The {}-th bit of {:032b} is set to be {}.", 20, a_u32, a as u8);
                assert_eq!(a, false);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = a_u32.is_bit_set(40);
    match set
    {
        Some(a) => { println!("The {}-th bit of {:032b} is set to be {}.", 40, a_u32, a as u8); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(set, None);
            }
    }

    let b_u32 = 0b10110011100011110000111110000011_u32;
    let set = small_uint_is_bit_set_func(b_u32, 29);
    match set
    {
        Some(b) => {
                println!("The {}-th bit of {:032b} is set to be {}.", 29, b_u32, b as u8);
                assert_eq!(b, true);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = small_uint_is_bit_set_func(b_u32, 20);
    match set
    {
        Some(b) => {
                println!("The {}-th bit of {:032b} is set to be {}.", 20, b_u32, b as u8);
                assert_eq!(b, false);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = small_uint_is_bit_set_func(b_u32, 40);
    match set
    {
        Some(b) => { println!("The {}-th bit of {:032b} is set to be {}.", 40, b_u32, b as u8); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(set, None);
            }
    }

    // Example for u64
    let a_u64 = 0b1011001110001111000011111000001111110000001111111000000011111111_u64;
    let set = a_u64.is_bit_set(18);
    match set
    {
        Some(a) => {
                println!("The {}-th bit of {:064b} is set to be set to be {}.", 18, a_u64, a as u8);
                assert_eq!(a, true);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = a_u64.is_bit_set(34);
    match set
    {
        Some(a) => {
                println!("The {}-th bit of {:064b} is set to be {}.", 34, a_u64, a as u8);
                assert_eq!(a, false);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = a_u64.is_bit_set(70);
    match set
    {
        Some(a) => { println!("The {}-th bit of {:064b} is set to be {}.", 70, a_u64, a as u8); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(set, None);
            }
    }

    let b_u64 = 0b1011001110001111000011111000001111110000001111111000000011111111_u64;
    let set = small_uint_is_bit_set_func(b_u64, 18);
    match set
    {
        Some(b) => {
                println!("The {}-th bit of {:064b} is set to be set to be {}.", 18, b_u64, b as u8);
                assert_eq!(b, true);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = small_uint_is_bit_set_func(b_u64, 34);
    match set
    {
        Some(b) => {
                println!("The {}-th bit of {:064b} is set to be {}.", 34, b_u64, b as u8);
                assert_eq!(b, false);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = small_uint_is_bit_set_func(b_u64, 70);
    match set
    {
        Some(b) => { println!("The {}-th bit of {:064b} is set to be {}.", 70, b_u64, b as u8); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(set, None);
            }
    }

    // Example for u128
    let a_u128 = 0b10110011100011110000111110000011111100000011111110000000111111110000000011111111100000000011111111110000000000111111111110000000_u128;
    let set = a_u128.is_bit_set(103);
    match set
    {
        Some(a) => {
                println!("The {}-th bit of {:0128b} is set to be {}.", 18, a_u128, a as u8);
                assert_eq!(a, true);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = a_u128.is_bit_set(126);
    match set
    {
        Some(a) => {
                println!("The {}-th bit of {:0128b} is set to be {}.", 126, a_u128, a as u8);
                assert_eq!(a, false);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = a_u128.is_bit_set(70);
    match set
    {
        Some(a) => { println!("The {}-th bit of {:0128b} is set to be {}.", 70, a_u128, a as u8); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(set, None);
            }
    }

    let b_u128 = 0b10110011100011110000111110000011111100000011111110000000111111110000000011111111100000000011111111110000000000111111111110000000_u128;
    let set = small_uint_is_bit_set_func(b_u128, 103);
    match set
    {
        Some(b) => {
                println!("The {}-th bit of {:0128b} is set to be {}.", 18, b_u128, b as u8);
                assert_eq!(b, true);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = small_uint_is_bit_set_func(b_u128, 126);
    match set
    {
        Some(b) => {
                println!("The {}-th bit of {:0128b} is set to be {}.", 126, b_u128, b as u8);
                assert_eq!(b, false);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = small_uint_is_bit_set_func(b_u128, 70);
    match set
    {
        Some(b) => { println!("The {}-th bit of {:0128b} is set to be {}.", 70, b_u128, b as u8); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(set, None);
            }
    }

    // Example for usize
    let a_usize = 0b1011001110001111000011111000001111110000001111111000000011111111_usize;
    let set = a_usize.is_bit_set(61);
    match set
    {
        Some(a) => {
                println!("The {}-th bit of {:0128b} is set to be {}.", 61, a_usize, a as u8);
                assert_eq!(a, true);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = a_usize.is_bit_set(8);
    match set
    {
        Some(a) => {
                println!("The {}-th bit of {:0128b} is set to be {}.", 8, a_usize, a as u8);
                assert_eq!(a, false);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = a_usize.is_bit_set(70);
    match set
    {
        Some(a) => { println!("The {}-th bit of {:0128b} is set to be {}.", 70, a_usize, a as u8); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(set, None);
            }
    }

    let b_usize = 0b1011001110001111000011111000001111110000001111111000000011111111_usize;
    let set = small_uint_is_bit_set_func(b_usize, 61);
    match set
    {
        Some(b) => {
                println!("The {}-th bit of {:0128b} is set to be {}.", 61, b_usize, b as u8);
                assert_eq!(b, true);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = small_uint_is_bit_set_func(b_usize, 8);
    match set
    {
        Some(b) => {
                println!("The {}-th bit of {:0128b} is set to be {}.", 8, b_usize, b as u8);
                assert_eq!(b, false);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = small_uint_is_bit_set_func(b_usize, 70);
    match set
    {
        Some(b) => { println!("The {}-th bit of {:0128b} is set to be {}.", 70, b_usize, b as u8); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(set, None);
            }
    }

    // Example for ShortUnion
    let a_shortunion = 0b1011001110001111_u16.into_shortunion();
    let set = a_shortunion.is_bit_set(7);
    match set
    {
        Some(a) => {
                println!("The {}-th bit of {:016b} is set to be {}.", 7, a_shortunion.get(), a as u8);
                assert_eq!(a, true);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = a_shortunion.is_bit_set(10);
    match set
    {
        Some(a) => {
                println!("The {}-th bit of {:016b} is set to be {}.", 10, a_shortunion.get(), a as u8);
                assert_eq!(a, false);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = a_shortunion.is_bit_set(16);
    match set
    {
        Some(a) => { println!("The {}-th bit of {:016b} is set to be {}.", 16, a_shortunion.get(), a as u8); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(set, None);
            }
    }

    let b_shortunion = 0b1011001110001111_u16.into_shortunion();
    let set = small_uint_is_bit_set_func(b_shortunion, 7);
    match set
    {
        Some(b) => {
                println!("The {}-th bit of {:016b} is set to be {}.", 7, b_shortunion.get(), b as u8);
                assert_eq!(b, true);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = small_uint_is_bit_set_func(b_shortunion, 10);
    match set
    {
        Some(b) => {
                println!("The {}-th bit of {:016b} is set to be {}.", 10, b_shortunion.get(), b as u8);
                assert_eq!(b, false);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = small_uint_is_bit_set_func(b_shortunion, 16);
    match set
    {
        Some(b) => { println!("The {}-th bit of {:016b} is set to be {}.", 16, b_shortunion.get(), b as u8); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(set, None);
            }
    }

    // Example for IntUnion
    let a_intunion = 0b10110011100011110000111110000011_u32.into_intunion();
    let set = a_intunion.is_bit_set(29);
    match set
    {
        Some(a) => {
                println!("The {}-th bit of {:032b} is set to be {}.", 29, a_intunion.get(), a as u8);
                assert_eq!(a, true);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = a_intunion.is_bit_set(20);
    match set
    {
        Some(a) => {
                println!("The {}-th bit of {:032b} is set to be {}.", 20, a_intunion.get(), a as u8);
                assert_eq!(a, false);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = a_intunion.is_bit_set(40);
    match set
    {
        Some(a) => { println!("The {}-th bit of {:032b} is set to be {}.", 40, a_intunion.get(), a as u8); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(set, None);
            }
    }

    let b_intunion = 0b10110011100011110000111110000011_u32.into_intunion();
    let set = small_uint_is_bit_set_func(b_intunion, 29);
    match set
    {
        Some(b) => {
                println!("The {}-th bit of {:032b} is set to be {}.", 29, b_intunion.get(), b as u8);
                assert_eq!(b, true);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = small_uint_is_bit_set_func(b_intunion, 20);
    match set
    {
        Some(b) => {
                println!("The {}-th bit of {:032b} is set to be {}.", 20, b_intunion.get(), b as u8);
                assert_eq!(b, false);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = small_uint_is_bit_set_func(b_intunion, 40);
    match set
    {
        Some(b) => { println!("The {}-th bit of {:032b} is set to be {}.", 40, b_intunion.get(), b as u8); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(set, None);
            }
    }

    // Example for LongUnion
    let a_longunion = 0b1011001110001111000011111000001111110000001111111000000011111111_u64.into_longunion();
    let set = a_longunion.is_bit_set(18);
    match set
    {
        Some(a) => {
                println!("The {}-th bit of {:064b} is set to be {}.", 18, a_longunion.get(), a as u8);
                assert_eq!(a, true);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = a_longunion.is_bit_set(34);
    match set
    {
        Some(a) => {
                println!("The {}-th bit of {:064b} is set to be {}.", 34, a_longunion.get(), a as u8);
                assert_eq!(a, false);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = a_longunion.is_bit_set(70);
    match set
    {
        Some(a) => { println!("The {}-th bit of {:064b} is set to be {}.", 70, a_longunion.get(), a as u8); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(set, None);
            }
    }

    let b_longunion = 0b1011001110001111000011111000001111110000001111111000000011111111_u64.into_longunion();
    let set = small_uint_is_bit_set_func(b_longunion, 18);
    match set
    {
        Some(b) => {
                println!("The {}-th bit of {:064b} is set to be {}.", 18, b_longunion.get(), b as u8);
                assert_eq!(b, true);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = small_uint_is_bit_set_func(b_longunion, 34);
    match set
    {
        Some(b) => {
                println!("The {}-th bit of {:064b} is set to be {}.", 34, b_longunion.get(), b as u8);
                assert_eq!(b, false);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = small_uint_is_bit_set_func(b_longunion, 70);
    match set
    {
        Some(b) => { println!("The {}-th bit of {:064b} is set to be {}.", 70, b_longunion.get(), b as u8); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(set, None);
            }
    }

    // Example for LongerUnion
    let a_longerunion = 0b10110011100011110000111110000011111100000011111110000000111111110000000011111111100000000011111111110000000000111111111110000000_u128.into_longerunion();
    let set = a_longerunion.is_bit_set(103);
    match set
    {
        Some(a) => {
                println!("The {}-th bit of {:0128b} is set to be {}.", 18, a_longerunion.get(), a as u8);
                assert_eq!(a, true);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = a_longerunion.is_bit_set(126);
    match set
    {
        Some(a) => {
                println!("The {}-th bit of {:0128b} is set to be {}.", 126, a_longerunion.get(), a as u8);
                assert_eq!(a, false);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = a_longerunion.is_bit_set(70);
    match set
    {
        Some(a) => { println!("The {}-th bit of {:0128b} is set to be {}.", 70, a_longerunion.get(), a as u8); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(set, None);
            }
    }

    let b_longerunion = 0b10110011100011110000111110000011111100000011111110000000111111110000000011111111100000000011111111110000000000111111111110000000_u128.into_longerunion();
    let set = small_uint_is_bit_set_func(b_longerunion, 103);
    match set
    {
        Some(b) => {
                println!("The {}-th bit of {:0128b} is set to be {}.", 18, b_longerunion.get(), b as u8);
                assert_eq!(b, true);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = small_uint_is_bit_set_func(b_longerunion, 126);
    match set
    {
        Some(b) => {
                println!("The {}-th bit of {:0128b} is set to be {}.", 126, b_longerunion.get(), b as u8);
                assert_eq!(b, false);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = small_uint_is_bit_set_func(b_longerunion, 70);
    match set
    {
        Some(b) => { println!("The {}-th bit of {:0128b} is set to be {}.", 70, b_longerunion.get(), b as u8); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(set, None);
            }
    }

    // Example for SizeUnion
    let a_sizeunion = 0b1011001110001111000011111000001111110000001111111000000011111111_usize.into_sizeunion();
    let set = a_sizeunion.is_bit_set(61);
    match set
    {
        Some(a) => {
                println!("The {}-th bit of {:0128b} is set to be {}.", 61, a_sizeunion.get(), a as u8);
                assert_eq!(a, true);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = a_sizeunion.is_bit_set(8);
    match set
    {
        Some(a) => {
                println!("The {}-th bit of {:0128b} is set to be {}.", 8, a_sizeunion.get(), a as u8);
                assert_eq!(a, false);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = a_sizeunion.is_bit_set(70);
    match set
    {
        Some(a) => { println!("The {}-th bit of {:0128b} is set to be {}.", 70, a_sizeunion.get(), a as u8); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(set, None);
            }
    }

    let b_sizeunion = 0b1011001110001111000011111000001111110000001111111000000011111111_usize.into_sizeunion();
    let set = small_uint_is_bit_set_func(b_sizeunion, 61);
    match set
    {
        Some(b) => {
                println!("The {}-th bit of {:0128b} is set to be {}.", 61, b_sizeunion.get(), b as u8);
                assert_eq!(b, true);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = small_uint_is_bit_set_func(b_sizeunion, 8);
    match set
    {
        Some(b) => {
                println!("The {}-th bit of {:0128b} is set to be {}.", 8, b_sizeunion.get(), b as u8);
                assert_eq!(b, false);
            },
        None => { println!("bit_pos is out of range."); }
    }
    let set = small_uint_is_bit_set_func(b_sizeunion, 70);
    match set
    {
        Some(b) => { println!("The {}-th bit of {:0128b} is set to be {}.", 70, b_sizeunion.get(), b as u8); },
        None => {
                println!("bit_pos is out of range.");
                assert_eq!(set, None);
            }
    }
    println!("--------------------------------------");
}

fn small_uint_is_bit_set_func<T: cryptocol::number::SmallUInt>(num: T, bit_pos: usize) -> Option<bool>
{
    num.is_bit_set(bit_pos)
}

fn small_uint_is_bit_set_()
{
    println!("small_uint_is_bit_set_");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = 0b01001100_u8;
    let set = a_u8.is_bit_set_(6);
    println!("The {}-th bit of {:08b} is set to be {}.", 6, a_u8, set as u8);
    assert_eq!(set, true);
    let set = a_u8.is_bit_set_(4);
    println!("The {}-th bit of {:08b} is set to be {}.", 4, a_u8, set as u8);
    assert_eq!(set, false);
    // It will panic.
    // let set = a_u8.is_bit_set_(9);

    let b_u8 = 0b01001100_u8;
    let set = small_uint_is_bit_set__func(b_u8, 6);
    println!("The {}-th bit of {:08b} is set to be {}.", 6, b_u8, set as u8);
    assert_eq!(set, true);
    let set = small_uint_is_bit_set__func(b_u8, 4);
    println!("The {}-th bit of {:08b} is set to be {}.", 4, b_u8, set as u8);
    assert_eq!(set, false);
    // It will panic.
    // let set = small_uint_is_bit_set__func(b_u8, 9);

    // Example for u16
    let a_u16 = 0b1011001110001111_u16;
    let set = a_u16.is_bit_set_(7);
    println!("The {}-th bit of {:016b} is set to be {}.", 7, a_u16, set as u8);
    assert_eq!(set, true);
    let set = a_u16.is_bit_set_(10);
    println!("The {}-th bit of {:016b} is set to be {}.", 10, a_u16, set as u8);
    assert_eq!(set, false);
    // It will panic.
    // let set = a_u16.is_bit_set_(16);

    let b_u16 = 0b1011001110001111_u16;
    let set = small_uint_is_bit_set__func(b_u16, 7);
    println!("The {}-th bit of {:016b} is set to be {}.", 7, b_u16, set as u8);
    assert_eq!(set, true);
    let set = small_uint_is_bit_set__func(b_u16, 10);
    println!("The {}-th bit of {:016b} is set to be {}.", 10, b_u16, set as u8);
    assert_eq!(set, false);
    // It will panic.
    // let set = small_uint_is_bit_set__func(b_u16, 16);

    // Example for u32
    let a_u32 = 0b10110011100011110000111110000011_u32;
    let set = a_u32.is_bit_set_(29);
    println!("The {}-th bit of {:032b} is set to be {}.", 29, a_u32, set as u8);
    assert_eq!(set, true);
    let set = a_u32.is_bit_set_(20);
    println!("The {}-th bit of {:032b} is set to be {}.", 20, a_u32, set as u8);
    assert_eq!(set, false);
    // It will panic.
    // let set = a_u32.is_bit_set_(40);

    let b_u32 = 0b10110011100011110000111110000011_u32;
    let set = small_uint_is_bit_set__func(b_u32, 29);
    println!("The {}-th bit of {:032b} is set to be {}.", 29, b_u32, set as u8);
    assert_eq!(set, true);
    let set = small_uint_is_bit_set__func(b_u32, 20);
    println!("The {}-th bit of {:032b} is set to be {}.", 20, b_u32, set as u8);
    assert_eq!(set, false);
    // It will panic.
    // let set = small_uint_is_bit_set__func(b_u32, 40);

    // Example for u64
    let a_u64 = 0b1011001110001111000011111000001111110000001111111000000011111111_u64;
    let set = a_u64.is_bit_set_(18);
    println!("The {}-th bit of {:064b} is set to be set to be {}.", 18, a_u64, set as u8);
    assert_eq!(set, true);
    let set = a_u64.is_bit_set_(34);
    println!("The {}-th bit of {:064b} is set to be {}.", 34, a_u64, set as u8);
    assert_eq!(set, false);
    // It will panic.
    // let set = a_u64.is_bit_set_(70);

    let b_u64 = 0b1011001110001111000011111000001111110000001111111000000011111111_u64;
    let set = small_uint_is_bit_set__func(b_u64, 18);
    println!("The {}-th bit of {:064b} is set to be set to be {}.", 18, b_u64, set as u8);
    assert_eq!(set, true);
    let set = small_uint_is_bit_set__func(b_u64, 34);
    println!("The {}-th bit of {:064b} is set to be {}.", 34, b_u64, set as u8);
    assert_eq!(set, false);
    // It will panic.
    // let set = small_uint_is_bit_set__func(b_u64, 70);

    // Example for u128
    let a_u128 = 0b10110011100011110000111110000011111100000011111110000000111111110000000011111111100000000011111111110000000000111111111110000000_u128;
    let set = a_u128.is_bit_set_(103);
    println!("The {}-th bit of {:0128b} is set to be {}.", 18, a_u128, set as u8);
    assert_eq!(set, true);
    let set = a_u128.is_bit_set_(126);
    println!("The {}-th bit of {:0128b} is set to be {}.", 126, a_u128, set as u8);
    assert_eq!(set, false);
    // It will panic.
    // let set = a_u128.is_bit_set_(70);

    let b_u128 = 0b10110011100011110000111110000011111100000011111110000000111111110000000011111111100000000011111111110000000000111111111110000000_u128;
    let set = small_uint_is_bit_set__func(b_u128, 103);
    println!("The {}-th bit of {:0128b} is set to be {}.", 18, b_u128, set as u8);
    assert_eq!(set, true);
    let set = small_uint_is_bit_set__func(b_u128, 126);
    println!("The {}-th bit of {:0128b} is set to be {}.", 126, b_u128, set as u8);
    assert_eq!(set, false);
    // It will panic.
    // let set = small_uint_is_bit_set__func(b_u128, 70);

    // Example for usize
    let a_usize = 0b1011001110001111000011111000001111110000001111111000000011111111_usize;
    let set = a_usize.is_bit_set_(61);
    println!("The {}-th bit of {:0128b} is set to be {}.", 61, a_usize, set as u8);
    assert_eq!(set, true);
    let set = a_usize.is_bit_set_(8);
    println!("The {}-th bit of {:0128b} is set to be {}.", 8, a_usize, set as u8);
    assert_eq!(set, false);
    // It will panic.
    // let set = a_usize.is_bit_set_(70);

    let b_usize = 0b1011001110001111000011111000001111110000001111111000000011111111_usize;
    let set = small_uint_is_bit_set__func(b_usize, 61);
    println!("The {}-th bit of {:0128b} is set to be {}.", 61, b_usize, set as u8);
    assert_eq!(set, true);
    let set = small_uint_is_bit_set__func(b_usize, 8);
    println!("The {}-th bit of {:0128b} is set to be {}.", 8, b_usize, set as u8);
    assert_eq!(set, false);
    // It will panic.
    // let set = small_uint_is_bit_set__func(b_usize, 70);

    // Example for ShortUnion
    let a_shortunion = 0b1011001110001111_u16.into_shortunion();
    let set = a_shortunion.is_bit_set_(7);
    println!("The {}-th bit of {:016b} is set to be {}.", 7, a_shortunion.get(), set as u8);
    assert_eq!(set, true);
    let set = a_shortunion.is_bit_set_(10);
    println!("The {}-th bit of {:016b} is set to be {}.", 10, a_shortunion.get(), set as u8);
    assert_eq!(set, false);
    // It will panic.
    // let set = a_shortunion.is_bit_set_(16);

    let b_shortunion = 0b1011001110001111_u16.into_shortunion();
    let set = small_uint_is_bit_set__func(b_shortunion, 7);
    println!("The {}-th bit of {:016b} is set to be {}.", 7, b_shortunion.get(), set as u8);
    assert_eq!(set, true);
    let set = small_uint_is_bit_set__func(b_shortunion, 10);
    println!("The {}-th bit of {:016b} is set to be {}.", 10, b_shortunion.get(), set as u8);
    assert_eq!(set, false);
    // It will panic.
    // let set = small_uint_is_bit_set__func(b_shortunion, 16);

    // Example for IntUnion
    let a_intunion = 0b10110011100011110000111110000011_u32.into_intunion();
    let set = a_intunion.is_bit_set_(29);
    println!("The {}-th bit of {:032b} is set to be {}.", 29, a_intunion.get(), set as u8);
    assert_eq!(set, true);
    let set = a_intunion.is_bit_set_(20);
    println!("The {}-th bit of {:032b} is set to be {}.", 20, a_intunion.get(), set as u8);
    assert_eq!(set, false);
    // It will panic.
    // let set = a_intunion.is_bit_set_(40);

    let b_intunion = 0b10110011100011110000111110000011_u32.into_intunion();
    let set = small_uint_is_bit_set__func(b_intunion, 29);
    println!("The {}-th bit of {:032b} is set to be {}.", 29, b_intunion.get(), set as u8);
    assert_eq!(set, true);
    let set = small_uint_is_bit_set__func(b_intunion, 20);
    println!("The {}-th bit of {:032b} is set to be {}.", 20, b_intunion.get(), set as u8);
    assert_eq!(set, false);
    // It will panic.
    // let set = small_uint_is_bit_set__func(b_intunion, 40);

    // Example for LongUnion
    let a_longunion = 0b1011001110001111000011111000001111110000001111111000000011111111_u64.into_longunion();
    let set = a_longunion.is_bit_set_(18);
    println!("The {}-th bit of {:064b} is set to be {}.", 18, a_longunion.get(), set as u8);
    assert_eq!(set, true);
    let set = a_longunion.is_bit_set_(34);
    println!("The {}-th bit of {:064b} is set to be {}.", 34, a_longunion.get(), set as u8);
    assert_eq!(set, false);
    // It will panic.
    // let set = a_longunion.is_bit_set_(70);

    let b_longunion = 0b1011001110001111000011111000001111110000001111111000000011111111_u64.into_longunion();
    let set = small_uint_is_bit_set__func(b_longunion, 18);
    println!("The {}-th bit of {:064b} is set to be {}.", 18, b_longunion.get(), set as u8);
    assert_eq!(set, true);
    let set = small_uint_is_bit_set__func(b_longunion, 34);
    println!("The {}-th bit of {:064b} is set to be {}.", 34, b_longunion.get(), set as u8);
    assert_eq!(set, false);
    // It will panic.
    // let set = small_uint_is_bit_set__func(b_longunion, 70);

    // Example for LongerUnion
    let a_longerunion = 0b10110011100011110000111110000011111100000011111110000000111111110000000011111111100000000011111111110000000000111111111110000000_u128.into_longerunion();
    let set = a_longerunion.is_bit_set_(103);
    println!("The {}-th bit of {:0128b} is set to be {}.", 18, a_longerunion.get(), set as u8);
    assert_eq!(set, true);
    let set = a_longerunion.is_bit_set_(126);
    println!("The {}-th bit of {:0128b} is set to be {}.", 126, a_longerunion.get(), set as u8);
    assert_eq!(set, false);
    // It will panic.
    // let set = a_longerunion.is_bit_set_(70);

    let b_longerunion = 0b10110011100011110000111110000011111100000011111110000000111111110000000011111111100000000011111111110000000000111111111110000000_u128.into_longerunion();
    let set = small_uint_is_bit_set__func(b_longerunion, 103);
    println!("The {}-th bit of {:0128b} is set to be {}.", 18, b_longerunion.get(), set as u8);
    assert_eq!(set, true);
    let set = small_uint_is_bit_set__func(b_longerunion, 126);
    println!("The {}-th bit of {:0128b} is set to be {}.", 126, b_longerunion.get(), set as u8);
    assert_eq!(set, false);
    // It will panic.
    // let set = small_uint_is_bit_set__func(b_longerunion, 70);

    // Example for SizeUnion
    let a_sizeunion = 0b1011001110001111000011111000001111110000001111111000000011111111_usize.into_sizeunion();
    let set = a_sizeunion.is_bit_set_(61);
    println!("The {}-th bit of {:0128b} is set to be {}.", 61, a_sizeunion.get(), set as u8);
    assert_eq!(set, true);
    let set = a_sizeunion.is_bit_set_(8);
    println!("The {}-th bit of {:0128b} is set to be {}.", 8, a_sizeunion.get(), set as u8);
    assert_eq!(set, false);
    // It will panic.
    // let set = a_sizeunion.is_bit_set_(70);

    let b_sizeunion = 0b1011001110001111000011111000001111110000001111111000000011111111_usize.into_sizeunion();
    let set = small_uint_is_bit_set__func(b_sizeunion, 61);
    println!("The {}-th bit of {:0128b} is set to be {}.", 61, b_sizeunion.get(), set as u8);
    assert_eq!(set, true);
    let set = small_uint_is_bit_set__func(b_sizeunion, 8);
    println!("The {}-th bit of {:0128b} is set to be {}.", 8, b_sizeunion.get(), set as u8);
    assert_eq!(set, false);
    // It will panic.
    // let set = small_uint_is_bit_set__func(b_sizeunion, 70);
    println!("--------------------------------------");
}

#[allow(non_snake_case)]
fn small_uint_is_bit_set__func<T: cryptocol::number::SmallUInt>(num: T, bit_pos: usize) -> bool
{
    num.is_bit_set_(bit_pos)
}

fn small_uint_bytes_operation()
{
    small_uint_from_be();
    small_uint_from_le();
    small_uint_to_be();
    small_uint_to_le();
    small_uint_swap_bytes();
}

fn small_uint_from_be()
{
    println!("small_uint_from_be");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8be = 0x12_u8;
    let b_u8le = small_uint_from_be_func(a_u8be);
    println!("{:02x} -> {:02x}", a_u8be, b_u8le);
    assert_eq!(b_u8le, 0x12_u8);

    // Example for u16 for Little Endianness
    let a_u16be = 0x1234_u16;
    let b_u16le = small_uint_from_be_func(a_u16be);
    println!("{:04x} -> {:04x}", a_u16be, b_u16le);
    assert_eq!(b_u16le, 0x3412_u16);

    // Example for u32 for Little Endianness
    let a_u32be = 0x12345678_u32;
    let b_u32le = small_uint_from_be_func(a_u32be);
    println!("{:08x} -> {:08x}", a_u32be, b_u32le);
    assert_eq!(b_u32le, 0x78563412_u32);

    // Example for u64 for Little Endianness
    let a_u64be = 0x123456789ABCDEF0_u64;
    let b_u64le = small_uint_from_be_func(a_u64be);
    println!("{:016x} -> {:016x}", a_u64be, b_u64le);
    assert_eq!(b_u64le, 0xf0debc9a78563412_u64);

    // Example for u128 for Little Endianness
    let a_u128be = 0x123456789ABCDEF0123456789ABCDEF0_u128;
    let b_u128le = small_uint_from_be_func(a_u128be);
    println!("{:032x} -> {:032x}", a_u128be, b_u128le);
    assert_eq!(b_u128le, 0xf0debc9a78563412f0debc9a78563412_u128);

    // Example for usize for Little Endianness
    let a_usizebe = 0x123456789ABCDEF0_usize;
    let b_usizele = small_uint_from_be_func(a_usizebe);
    println!("{:016x} -> {:016x}", a_usizebe, b_usizele);
    assert_eq!(b_usizele, 0xf0debc9a78563412_usize);

    // Example for ShortUnion for Little Endianness
    let a_shortunionbe = 0x1234_u16.into_shortunion();
    let b_shortunionle = small_uint_from_be_func(a_shortunionbe);
    println!("{:04x} -> {:04x}", a_shortunionbe.get(), b_shortunionle.get());
    assert_eq!(b_shortunionle.get(), 0x3412_u16);

    // Example for IntUnion for Little Endianness
    let a_intunionbe = 0x12345678_u32.into_intunion();
    let b_intunionle = small_uint_from_be_func(a_intunionbe);
    println!("{:08x} -> {:08x}", a_intunionbe.get(), b_intunionle.get());
    assert_eq!(b_intunionle.get(), 0x78563412_u32);

    // Example for LongUnion for Little Endianness
    let a_longunionbe = 0x123456789ABCDEF0_u64.into_longunion();
    let b_longunionle = small_uint_from_be_func(a_longunionbe);
    println!("{:016x} -> {:016x}", a_longunionbe.get(), b_longunionle.get());
    assert_eq!(b_longunionle.get(), 0xf0debc9a78563412_u64);

    // Example for LongerUnion for Little Endianness
    let a_longerunionbe = 0x123456789ABCDEF0123456789ABCDEF0_u128.into_longerunion();
    let b_longerunionle = small_uint_from_be_func(a_longerunionbe);
    println!("{:032x} -> {:032x}", a_longerunionbe.get(), b_longerunionle.get());
    assert_eq!(b_longerunionle.get(), 0xf0debc9a78563412f0debc9a78563412_u128);

    // Example for SizeUnion for Little Endianness
    let a_sizeunionbe = 0x123456789ABCDEF0_usize.into_sizeunion();
    let b_sizeunionle = small_uint_from_be_func(a_sizeunionbe);
    println!("{:016x} -> {:016x}", a_sizeunionbe.get(), b_sizeunionle.get());
    assert_eq!(b_sizeunionle.get(), 0xf0debc9a78563412_usize);
    println!("--------------------------------------");
}

fn small_uint_from_be_func<T: cryptocol::number::SmallUInt>(num: T) -> T
{
    T::from_be(num)
}

fn small_uint_from_le()
{
    println!("small_uint_from_le");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8le = 0x12_u8;
    let b_u8le = small_uint_from_le_func(a_u8le);
    println!("{:02x} -> {:02x}", a_u8le, b_u8le);
    assert_eq!(b_u8le, 0x12_u8);

    // Example for u16 for Little Endianness
    let a_u16le = 0x1234_u16;
    let b_u16le = small_uint_from_le_func(a_u16le);
    println!("{:04x} -> {:04x}", a_u16le, b_u16le);
    assert_eq!(b_u16le, 0x1234_u16);

    // Example for u32 for Little Endianness
    let a_u32le = 0x12345678_u32;
    let b_u32le = small_uint_from_le_func(a_u32le);
    println!("{:08x} -> {:08x}", a_u32le, b_u32le);
    assert_eq!(b_u32le, 0x12345678_u32);

    // Example for u64 for Little Endianness
    let a_u64le = 0x123456789ABCDEF0_u64;
    let b_u64le = small_uint_from_le_func(a_u64le);
    println!("{:016x} -> {:016x}", a_u64le, b_u64le);
    assert_eq!(b_u64le, 0x123456789ABCDEF0_u64);

    // Example for u128 for Little Endianness
    let a_u128le = 0x123456789ABCDEF0123456789ABCDEF0_u128;
    let b_u128le = small_uint_from_le_func(a_u128le);
    println!("{:032x} -> {:032x}", a_u128le, b_u128le);
    assert_eq!(b_u128le, 0x123456789ABCDEF0123456789ABCDEF0_u128);

    // Example for usize for Little Endianness
    let a_usizele = 0x123456789ABCDEF0_usize;
    let b_usizele = small_uint_from_le_func(a_usizele);
    println!("{:016x} -> {:016x}", a_usizele, b_usizele);
    assert_eq!(b_usizele, 0x123456789ABCDEF0_usize);

    // Example for ShortUnion for Little Endianness
    let a_shortunionle = 0x1234_u16.into_shortunion();
    let b_shortunionle = small_uint_from_le_func(a_shortunionle);
    println!("{:04x} -> {:04x}", a_shortunionle.get(), b_shortunionle.get());
    assert_eq!(b_shortunionle.get(), 0x1234_u16);

    // Example for IntUnion for Little Endianness
    let a_intunionle = 0x12345678_u32.into_intunion();
    let b_intunionle = small_uint_from_le_func(a_intunionle);
    println!("{:08x} -> {:08x}", a_intunionle.get(), b_intunionle.get());
    assert_eq!(b_intunionle.get(), 0x12345678_u32);

    // Example for LongUnion for Little Endianness
    let a_longunionle = 0x123456789ABCDEF0_u64.into_longunion();
    let b_longunionle = small_uint_from_le_func(a_longunionle);
    println!("{:016x} -> {:016x}", a_longunionle.get(), b_longunionle.get());
    assert_eq!(b_longunionle.get(), 0x123456789ABCDEF0_u64);

    // Example for LongerUnion for Little Endianness
    let a_longerunionle = 0x123456789ABCDEF0123456789ABCDEF0_u128.into_longerunion();
    let b_longerunionle = small_uint_from_le_func(a_longerunionle);
    println!("{:032x} -> {:032x}", a_longerunionle.get(), b_longerunionle.get());
    assert_eq!(b_longerunionle.get(), 0x123456789ABCDEF0123456789ABCDEF0_u128);

    // Example for SizeUnion for Little Endianness
    let a_sizeunionle = 0x123456789ABCDEF0_usize.into_sizeunion();
    let b_sizeunionle = small_uint_from_le_func(a_sizeunionle);
    println!("{:016x} -> {:016x}", a_sizeunionle.get(), b_sizeunionle.get());
    assert_eq!(b_sizeunionle.get(), 0x123456789ABCDEF0_usize);
    println!("--------------------------------------");
}

fn small_uint_from_le_func<T: cryptocol::number::SmallUInt>(num: T) -> T
{
    T::from_le(num)
}

fn small_uint_to_be()
{
    println!("small_uint_to_be");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8le = 0x12_u8;
    let b_u8be = small_uint_to_be_func(a_u8le);
    println!("{:02x} -> {:02x}", a_u8le, b_u8be);
    assert_eq!(b_u8be, 0x12_u8);

    // Example for u16 for Little Endianness
    let a_u16le = 0x1234_u16;
    let b_u16be = small_uint_to_be_func(a_u16le);
    println!("{:04x} -> {:04x}", a_u16le, b_u16be);
    assert_eq!(b_u16be, 0x3412_u16);

    // Example for u32 for Little Endianness
    let a_u32le = 0x12345678_u32;
    let b_u32be = small_uint_to_be_func(a_u32le);
    println!("{:08x} -> {:08x}", a_u32le, b_u32be);
    assert_eq!(b_u32be, 0x78563412_u32);

    // Example for u64 for Little Endianness
    let a_u64le = 0x123456789ABCDEF0_u64;
    let b_u64be = small_uint_to_be_func(a_u64le);
    println!("{:016x} -> {:016x}", a_u64le, b_u64be);
    assert_eq!(b_u64be, 0xf0debc9a78563412_u64);

    // Example for u128 for Little Endianness
    let a_u128le = 0x123456789ABCDEF0123456789ABCDEF0_u128;
    let b_u128be = small_uint_to_be_func(a_u128le);
    println!("{:032x} -> {:032x}", a_u128le, b_u128be);
    assert_eq!(b_u128be, 0xf0debc9a78563412f0debc9a78563412_u128);

    // Example for usize for Little Endianness
    let a_usizele = 0x123456789ABCDEF0_usize;
    let b_usizebe = small_uint_to_be_func(a_usizele);
    println!("{:016x} -> {:016x}", a_usizele, b_usizebe);
    assert_eq!(b_usizebe, 0xf0debc9a78563412_usize);

    // Example for ShortUnion for Little Endianness
    let a_shortunionle = 0x1234_u16.into_shortunion();
    let b_shortunionbe = small_uint_to_be_func(a_shortunionle);
    println!("{:04x} -> {:04x}", a_shortunionle.get(), b_shortunionbe.get());
    assert_eq!(b_shortunionbe.get(), 0x3412_u16);

    // Example for IntUnion for Little Endianness
    let a_intunionle = 0x12345678_u32.into_intunion();
    let b_intunionbe = small_uint_to_be_func(a_intunionle);
    println!("{:08x} -> {:08x}", a_intunionle.get(), b_intunionbe.get());
    assert_eq!(b_intunionbe.get(), 0x78563412_u32);

    // Example for LongUnion for Little Endianness
    let a_longunionle = 0x123456789ABCDEF0_u64.into_longunion();
    let b_longunionbe = small_uint_to_be_func(a_longunionle);
    println!("{:016x} -> {:016x}", a_longunionle.get(), b_longunionbe.get());
    assert_eq!(b_longunionbe.get(), 0xf0debc9a78563412_u64);

    // Example for LongerUnion for Little Endianness
    let a_longerunionle = 0x123456789ABCDEF0123456789ABCDEF0_u128.into_longerunion();
    let b_longerunionbe = small_uint_to_be_func(a_longerunionle);
    println!("{:032x} -> {:032x}", a_longerunionle.get(), b_longerunionbe.get());
    assert_eq!(b_longerunionbe.get(), 0xf0debc9a78563412f0debc9a78563412_u128);

    // Example for SizeUnion for Little Endianness
    let a_sizeunionle = 0x123456789ABCDEF0_usize.into_sizeunion();
    let b_sizeunionbe = small_uint_to_be_func(a_sizeunionle);
    println!("{:016x} -> {:016x}", a_sizeunionle.get(), b_sizeunionbe.get());
    assert_eq!(b_sizeunionbe.get(), 0xf0debc9a78563412_usize);
    println!("--------------------------------------");
}

fn small_uint_to_be_func<T: cryptocol::number::SmallUInt>(num: T) -> T
{
    num.to_be()
}

fn small_uint_to_le()
{
    println!("small_uint_to_le");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8le = 0x12_u8;
    let b_u8le = small_uint_to_le_func(a_u8le);
    println!("{:02x} -> {:02x}", a_u8le, b_u8le);
    assert_eq!(b_u8le, 0x12_u8);

    // Example for u16 for Little Endianness
    let a_u16le = 0x1234_u16;
    let b_u16le = small_uint_to_le_func(a_u16le);
    println!("{:04x} -> {:04x}", a_u16le, b_u16le);
    assert_eq!(b_u16le, 0x1234_u16);

    // Example for u32 for Little Endianness
    let a_u32le = 0x12345678_u32;
    let b_u32le = small_uint_to_le_func(a_u32le);
    println!("{:08x} -> {:08x}", a_u32le, b_u32le);
    assert_eq!(b_u32le, 0x12345678_u32);

    // Example for u64 for Little Endianness
    let a_u64le = 0x123456789ABCDEF0_u64;
    let b_u64le = small_uint_to_le_func(a_u64le);
    println!("{:016x} -> {:016x}", a_u64le, b_u64le);
    assert_eq!(b_u64le, 0x123456789ABCDEF0_u64);

    // Example for u128 for Little Endianness
    let a_u128le = 0x123456789ABCDEF0123456789ABCDEF0_u128;
    let b_u128le = small_uint_to_le_func(a_u128le);
    println!("{:032x} -> {:032x}", a_u128le, b_u128le);
    assert_eq!(b_u128le, 0x123456789ABCDEF0123456789ABCDEF0_u128);

    // Example for usize for Little Endianness
    let a_usizele = 0x123456789ABCDEF0_usize;
    let b_usizele = small_uint_to_le_func(a_usizele);
    println!("{:016x} -> {:016x}", a_usizele, b_usizele);
    assert_eq!(b_usizele, 0x123456789ABCDEF0_usize);

    // Example for ShortUnion for Little Endianness
    let a_shortunionle = 0x1234_u16.into_shortunion();
    let b_shortunionle = small_uint_to_le_func(a_shortunionle);
    println!("{:04x} -> {:04x}", a_shortunionle.get(), b_shortunionle.get());
    assert_eq!(b_shortunionle.get(), 0x1234_u16);

    // Example for IntUnion for Little Endianness
    let a_intunionle = 0x12345678_u32.into_intunion();
    let b_intunionle = small_uint_to_le_func(a_intunionle);
    println!("{:08x} -> {:08x}", a_intunionle.get(), b_intunionle.get());
    assert_eq!(b_intunionle.get(), 0x12345678_u32);

    // Example for LongUnion for Little Endianness
    let a_longunionle = 0x123456789ABCDEF0_u64.into_longunion();
    let b_longunionle = small_uint_to_le_func(a_longunionle);
    println!("{:016x} -> {:016x}", a_longunionle.get(), b_longunionle.get());
    assert_eq!(b_longunionle.get(), 0x123456789ABCDEF0_u64);

    // Example for LongerUnion for Little Endianness
    let a_longerunionle = 0x123456789ABCDEF0123456789ABCDEF0_u128.into_longerunion();
    let b_longerunionle = small_uint_to_le_func(a_longerunionle);
    println!("{:032x} -> {:032x}", a_longerunionle.get(), b_longerunionle.get());
    assert_eq!(b_longerunionle.get(), 0x123456789ABCDEF0123456789ABCDEF0_u128);

    // Example for SizeUnion for Little Endianness
    let a_sizeunionle = 0x123456789ABCDEF0_usize.into_sizeunion();
    let b_sizeunionle = small_uint_to_le_func(a_sizeunionle);
    println!("{:016x} -> {:016x}", a_sizeunionle.get(), b_sizeunionle.get());
    assert_eq!(b_sizeunionle.get(), 0x123456789ABCDEF0_usize);
    println!("--------------------------------------");
}

fn small_uint_to_le_func<T: cryptocol::number::SmallUInt>(num: T) -> T
{
    num.to_le()
}

fn small_uint_swap_bytes()
{
    println!("small_uint_swap_bytes");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = 0x12_u8;
    let b_u8 = small_uint_swap_bytes_func(a_u8);
    println!("{:02x} -> {:02x}", a_u8, b_u8);
    assert_eq!(b_u8, 0x12_u8);

    // Example for u16
    let a_u16 = 0x1234_u16;
    let b_u16 = small_uint_swap_bytes_func(a_u16);
    println!("{:04x} -> {:04x}", a_u16, b_u16);
    assert_eq!(b_u16, 0x3412_u16);

    // Example for u32
    let a_u32 = 0x12345678_u32;
    let b_u32 = small_uint_swap_bytes_func(a_u32);
    println!("{:08x} -> {:08x}", a_u32, b_u32);
    assert_eq!(b_u32, 0x78563412_u32);

    // Example for u64
    let a_u64 = 0x123456789ABCDEF0_u64;
    let b_u64 = small_uint_swap_bytes_func(a_u64);
    println!("{:016x} -> {:016x}", a_u64, b_u64);
    assert_eq!(b_u64, 0xf0debc9a78563412_u64);

    // Example for u128
    let a_u128 = 0x123456789ABCDEF0123456789ABCDEF0_u128;
    let b_u128 = small_uint_swap_bytes_func(a_u128);
    println!("{:032x} -> {:032x}", a_u128, b_u128);
    assert_eq!(b_u128, 0xf0debc9a78563412f0debc9a78563412_u128);

    // Example for usize
    let a_usize = 0x123456789ABCDEF0_usize;
    let b_usize = small_uint_swap_bytes_func(a_usize);
    println!("{:016x} -> {:016x}", a_usize, b_usize);
    assert_eq!(b_usize, 0xf0debc9a78563412_usize);

    // Example for ShortUnion
    let a_shortunion = 0x1234_u16.into_shortunion();
    let b_shortunion = small_uint_swap_bytes_func(a_shortunion);
    println!("{:04x} -> {:04x}", a_shortunion.get(), b_shortunion.get());
    assert_eq!(b_shortunion.get(), 0x3412_u16);

    // Example for IntUnion
    let a_intunion = 0x12345678_u32.into_intunion();
    let b_intunion = small_uint_swap_bytes_func(a_intunion);
    println!("{:08x} -> {:08x}", a_intunion.get(), b_intunion.get());
    assert_eq!(b_intunion.get(), 0x78563412_u32);

    // Example for LongUnion
    let a_longunion = 0x123456789ABCDEF0_u64.into_longunion();
    let b_longunion = small_uint_swap_bytes_func(a_longunion);
    println!("{:016x} -> {:016x}", a_longunion.get(), b_longunion.get());
    assert_eq!(b_longunion.get(), 0xf0debc9a78563412_u64);

    // Example for LongerUnion
    let a_longerunion = 0x123456789ABCDEF0123456789ABCDEF0_u128.into_longerunion();
    let b_longerunion = small_uint_swap_bytes_func(a_longerunion);
    println!("{:032x} -> {:032x}", a_longerunion.get(), b_longerunion.get());
    assert_eq!(b_longerunion.get(), 0xf0debc9a78563412f0debc9a78563412_u128);

    // Example for SizeUnion
    let a_sizeunion = 0x123456789ABCDEF0_usize.into_sizeunion();
    let b_sizeunion = small_uint_swap_bytes_func(a_sizeunion);
    println!("{:016x} -> {:016x}", a_sizeunion.get(), b_sizeunion.get());
    assert_eq!(b_sizeunion.get(), 0xf0debc9a78563412_usize);
    println!("--------------------------------------");
}

fn small_uint_swap_bytes_func<T: cryptocol::number::SmallUInt>(num: T) -> T
{
    num.swap_bytes()
}

fn small_uint_find_power()
{
    small_uint_is_power_of_two();
    small_uint_next_power_of_two();
}

fn small_uint_is_power_of_two()
{
    println!("small_uint_is_power_of_two");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = 128_u8;
    let a_two = a_u8.is_power_of_two();
    println!("{} is {}power of two.", a_u8, if a_two {""} else {"not "});
    assert!(a_two);

    let b_u8 = 200_u8;
    let b_two = b_u8.is_power_of_two();
    println!("{} is {}power of two.", b_u8, if b_two {""} else {"not "});
    assert!(!b_two);

    let c_u8 = 128_u8;
    let c_two = small_uint_is_power_of_two_func(c_u8);
    println!("{} is {}power of two.", c_u8, if c_two {""} else {"not "});
    assert!(c_two);

    let d_u8 = 200_u8;
    let d_two = small_uint_is_power_of_two_func(d_u8);
    println!("{} is {}power of two.", d_u8, if d_two {""} else {"not "});
    assert!(!d_two);

    // Example for u16
    let a_u16 = 32768_u16;
    let a_two = a_u16.is_power_of_two();
    println!("{} is {}power of two..", a_u16, if a_two {""} else {"not "});
    assert!(a_two);

    let b_u16 = 60000_u16;
    let b_two = b_u16.is_power_of_two();
    println!("{} is {}power of two..", b_u16, if b_two {""} else {"not "});
    assert!(!b_two);

    let c_u16 = 32768_u16;
    let c_two = small_uint_is_power_of_two_func(c_u16);
    println!("{} is {}power of two..", c_u16, if c_two {""} else {"not "});
    assert!(c_two);

    let d_u16 = 60000_u16;
    let d_two = small_uint_is_power_of_two_func(d_u16);
    println!("{} is {}power of two..", d_u16, if d_two {""} else {"not "});
    assert!(!d_two);

    // Example for u32
    let a_u32 = 2147483648_u32;
    let a_two = a_u32.is_power_of_two();
    println!("{} is {}power of two..", a_u32, if a_two {""} else {"not "});
    assert!(a_two);

    let b_u32 = 800000000_u32;
    let b_two = b_u32.is_power_of_two();
    println!("{} is {}power of two..", b_u32, if b_two {""} else {"not "});
    assert!(!b_two);

    let c_u32 = 2147483648_u32;
    let c_two = small_uint_is_power_of_two_func(c_u32);
    println!("{} is {}power of two..", c_u32, if c_two {""} else {"not "});
    assert!(c_two);

    let d_u32 = 800000000_u32;
    let d_two = small_uint_is_power_of_two_func(d_u32);
    println!("{} is {}power of two..", d_u32, if d_two {""} else {"not "});
    assert!(!d_two);

    // Example for u64
    let a_u64 = 9223372036854775808_u64;
    let a_two = a_u64.is_power_of_two();
    println!("{} is {}power of two..", a_u64, if a_two {""} else {"not "});
    assert!(a_two);

    let b_u64 = 2468135791234567892_u64;
    let b_two = b_u64.is_power_of_two();
    println!("{} is {}power of two..", b_u64, if b_two {""} else {"not "});
    assert!(!b_two);

    let c_u64 = 9223372036854775808_u64;
    let c_two = small_uint_is_power_of_two_func(c_u64);
    println!("{} is {}power of two..", c_u64, if c_two {""} else {"not "});
    assert!(c_two);

    let d_u64 = 2468135791234567892_u64;
    let d_two = small_uint_is_power_of_two_func(d_u64);
    println!("{} is {}power of two..", d_u64, if d_two {""} else {"not "});
    assert!(!d_two);

    // Example for u128
    let a_u128 = 170141183460469231731687303715884105728_u128;
    let a_two = a_u128.is_power_of_two();
    println!("{} is {}power of two..", a_u128, if a_two {""} else {"not "});
    assert!(a_two);

    let b_u128 = 200000000000000000000000000000000000000_u128;
    let b_two = b_u128.is_power_of_two();
    println!("{} is {}power of two..", b_u128, if b_two {""} else {"not "});
    assert!(!b_two);

    let c_u128 = 170141183460469231731687303715884105728_u128;
    let c_two = small_uint_is_power_of_two_func(c_u128);
    println!("{} is {}power of two..", c_u128, if c_two {""} else {"not "});
    assert!(c_two);

    let d_u128 = 200000000000000000000000000000000000000_u128;
    let d_two = small_uint_is_power_of_two_func(d_u128);
    println!("{} is {}power of two..", d_u128, if d_two {""} else {"not "});
    assert!(!d_two);

    // Example for usize
    let a_usize = 9223372036854775808_usize;
    let a_two = a_usize.is_power_of_two();
    println!("{} is {}power of two..", a_usize, if a_two {""} else {"not "});
    assert!(a_two);

    let b_usize = 20000000000000000_usize;
    let b_two = b_usize.is_power_of_two();
    println!("{} is {}power of two..", b_usize, if b_two {""} else {"not "});
    assert!(!b_two);

    let c_usize = 9223372036854775808_usize;
    let c_two = small_uint_is_power_of_two_func(c_usize);
    println!("{} is {}power of two..", c_usize, if c_two {""} else {"not "});
    assert!(c_two);

    let d_usize = 20000000000000000_usize;
    let d_two = small_uint_is_power_of_two_func(d_usize);
    println!("{} is {}power of two..", d_usize, if d_two {""} else {"not "});
    assert!(!d_two);

    // Example for ShortUnion
    let a_shortunion = 32768_u16.into_shortunion();
    let a_two = a_shortunion.is_power_of_two();
    println!("{} is {}power of two..", a_shortunion, if a_two {""} else {"not "});
    assert!(a_two);

    let b_shortunion = 65432_u16.into_shortunion();
    let b_two = b_shortunion.is_power_of_two();
    println!("{} is {}power of two..", b_shortunion, if b_two {""} else {"not "});
    assert!(!b_two);

    let c_shortunion = 32768_u16.into_shortunion();
    let c_two = small_uint_is_power_of_two_func(c_shortunion);
    println!("{} is {}power of two..", c_shortunion, if c_two {""} else {"not "});
    assert!(c_two);

    let d_shortunion = 65432_u16.into_shortunion();
    let d_two = small_uint_is_power_of_two_func(d_shortunion);
    println!("{} is {}power of two..", d_shortunion, if d_two {""} else {"not "});
    assert!(!d_two);

    // Example for IntUnion
    let a_intunion = 2147483648_u32.into_intunion();
    let a_two = a_intunion.is_power_of_two();
    println!("{} is {}power of two..", a_intunion, if a_two {""} else {"not "});
    assert!(a_two);

    let b_intunion = 876543210_u32.into_intunion();
    let b_two = b_intunion.is_power_of_two();
    println!("{} is {}power of two..", b_intunion, if b_two {""} else {"not "});
    assert!(!b_two);

    let c_intunion = 2147483648_u32.into_intunion();
    let c_two = small_uint_is_power_of_two_func(c_intunion);
    println!("{} is {}power of two..", c_intunion, if c_two {""} else {"not "});
    assert!(c_two);

    let d_intunion = 876543210_u32.into_intunion();
    let d_two = small_uint_is_power_of_two_func(d_intunion);
    println!("{} is {}power of two..", d_intunion, if d_two {""} else {"not "});
    assert!(!d_two);

    // Example for LongUnion
    let a_longunion = 9223372036854775808_u64.into_longunion();
    let a_two = a_longunion.is_power_of_two();
    println!("{} is {}power of two..", a_longunion, if a_two {""} else {"not "});
    assert!(a_two);

    let b_longunion = 2468135791234567892_u64.into_longunion();
    let b_two = b_longunion.is_power_of_two();
    println!("{} is {}power of two..", b_longunion, if b_two {""} else {"not "});
    assert!(!b_two);

    let c_longunion = 9223372036854775808_u64.into_longunion();
    let c_two = small_uint_is_power_of_two_func(c_longunion);
    println!("{} is {}power of two..", c_longunion, if c_two {""} else {"not "});
    assert!(c_two);

    let d_longunion = 2468135791234567892_u64.into_longunion();
    let d_two = small_uint_is_power_of_two_func(d_longunion);
    println!("{} is {}power of two..", d_longunion, if d_two {""} else {"not "});
    assert!(!d_two);

    // Example for LongerUnion
    let a_longerunion = 170141183460469231731687303715884105728_u128.into_longerunion();
    let a_two = a_longerunion.is_power_of_two();
    println!("{} is {}power of two..", a_longerunion, if a_two {""} else {"not "});
    assert!(a_two);

    let b_longerunion = 200000000000000000000000000000000000000_u128.into_longerunion();
    let b_two = b_longerunion.is_power_of_two();
    println!("{} is {}power of two..", b_longerunion, if b_two {""} else {"not "});
    assert!(!b_two);

    let c_longerunion = 170141183460469231731687303715884105728_u128.into_longerunion();
    let c_two = small_uint_is_power_of_two_func(c_longerunion);
    println!("{} is {}power of two..", c_longerunion, if c_two {""} else {"not "});
    assert!(c_two);

    let d_longerunion = 200000000000000000000000000000000000000_u128.into_longerunion();
    let d_two = small_uint_is_power_of_two_func(d_longerunion);
    println!("{} is {}power of two..", d_longerunion, if d_two {""} else {"not "});
    assert!(!d_two);

    // Example for SizeUnion
    let a_sizeunion = 9223372036854775808_usize.into_sizeunion();
    let a_two = a_sizeunion.is_power_of_two();
    println!("{} is {}power of two..", a_sizeunion, if a_two {""} else {"not "});
    assert!(a_two);

    let b_sizeunion = 2468135791234567882_usize.into_sizeunion();
    let b_two = b_sizeunion.is_power_of_two();
    println!("{} is {}power of two..", b_sizeunion, if b_two {""} else {"not "});
    assert!(!b_two);

    let c_sizeunion = 9223372036854775808_usize.into_sizeunion();
    let c_two = small_uint_is_power_of_two_func(c_sizeunion);
    println!("{} is {}power of two..", c_sizeunion, if c_two {""} else {"not "});
    assert!(c_two);

    let d_sizeunion = 2468135791234567882_usize.into_sizeunion();
    let d_two = small_uint_is_power_of_two_func(d_sizeunion);
    println!("{} is {}power of two..", d_sizeunion, if d_two {""} else {"not "});
    assert!(!d_two);
    println!("--------------------------------------");
}

fn small_uint_is_power_of_two_func<T: cryptocol::number::SmallUInt>(num: T) -> bool
{
    num.is_power_of_two()
}

fn small_uint_next_power_of_two()
{
    println!("small_uint_next_power_of_two");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = 10_u8;
    let b_u8 = a_u8.next_power_of_two();
    println!("{} => {}", a_u8, b_u8);
    assert_eq!(b_u8, 16_u8);

    let c_u8 = 10_u8;
    let d_u8 = small_uint_next_power_of_two_func(c_u8);
    println!("{} => {}", c_u8, d_u8);
    assert_eq!(d_u8, 16_u8);

    // Example for u16
    let a_u16 = 1000_u16;
    let b_u16 = a_u16.next_power_of_two();
    println!("{} => {}", a_u16, b_u16);
    assert_eq!(b_u16, 1024_u16);

    let c_u16 = 1000_u16;
    let d_u16 = small_uint_next_power_of_two_func(c_u16);
    println!("{} => {}", c_u16, d_u16);
    assert_eq!(d_u16, 1024_u16);

    // Example for u32
    let a_u32 = 10000000_u32;
    let b_u32 = a_u32.next_power_of_two();
    println!("{} => {}", a_u32, b_u32);
    assert_eq!(b_u32, 16777216_u32);

    let c_u32 = 10000000_u32;
    let d_u32 = small_uint_next_power_of_two_func(c_u32);
    println!("{} => {}", c_u32, d_u32);
    assert_eq!(d_u32, 16777216_u32);

    // Example for u64
    let a_u64 = 1000000000000000_u64;
    let b_u64 = a_u64.next_power_of_two();
    println!("{} => {}", a_u64, b_u64);
    assert_eq!(b_u64, 1125899906842624_u64);

    let c_u64 = 1000000000000000_u64;
    let d_u64 = small_uint_next_power_of_two_func(c_u64);
    println!("{} => {}", c_u64, d_u64);
    assert_eq!(d_u64, 1125899906842624_u64);

    // Example for u128
    let a_u128 = 100000000000000000000000000000_u128;
    let b_u128 = a_u128.next_power_of_two();
    println!("{} => {}", a_u128, b_u128);
    assert_eq!(b_u128, 158456325028528675187087900672_u128);

    let c_u128 = 100000000000000000000000000000_u128;
    let d_u128 = small_uint_next_power_of_two_func(c_u128);
    println!("{} => {}", c_u128, d_u128);
    assert_eq!(d_u128, 158456325028528675187087900672_u128);

    // Example for usize
    let a_usize = 1000000000000000_usize;
    let b_usize = a_usize.next_power_of_two();
    println!("{} => {}", a_usize, b_usize);
    assert_eq!(b_usize, 1125899906842624_usize);

    let c_usize = 1000000000000000_usize;
    let d_usize = small_uint_next_power_of_two_func(c_usize);
    println!("{} => {}", c_usize, d_usize);
    assert_eq!(d_usize, 1125899906842624_usize);

    // Example for ShortUnion
    let a_shortunion = 400_u16.into_shortunion();
    let b_shortunion = a_shortunion.next_power_of_two();
    println!("{} => {}", a_shortunion.get(), b_shortunion.get());
    assert_eq!(b_shortunion.get(), 512_u16);

    let c_shortunion = 400_u16.into_shortunion();
    let d_shortunion = small_uint_next_power_of_two_func(c_shortunion);
    println!("{} => {}", c_shortunion.get(), d_shortunion.get());
    assert_eq!(d_shortunion.get(), 512_u16);

    // Example for IntUnion
    let a_intunion = 400000_u32.into_intunion();
    let b_intunion = a_intunion.next_power_of_two();
    println!("{} => {}", a_intunion.get(), b_intunion.get());
    assert_eq!(b_intunion.get(), 524288_u32);

    let c_intunion = 400000_u32.into_intunion();
    let d_intunion = small_uint_next_power_of_two_func(c_intunion);
    println!("{} => {}", c_intunion.get(), d_intunion.get());
    assert_eq!(d_intunion.get(), 524288_u32);

    // Example for LongUnion
    let a_longunion = 400000000000_u64.into_longunion();
    let b_longunion = a_longunion.next_power_of_two();
    println!("{} => {}", a_longunion.get(), b_longunion.get());
    assert_eq!(b_longunion.get(), 549755813888_u64);

    let c_longunion = 400000000000_u64.into_longunion();
    let d_longunion = small_uint_next_power_of_two_func(c_longunion);
    println!("{} => {}", c_longunion.get(), d_longunion.get());
    assert_eq!(d_longunion.get(), 549755813888_u64);

    // Example for LongerUnion
    let a_longerunion = 4000000000000000000000000000_u128.into_longerunion();
    let b_longerunion = a_longerunion.next_power_of_two();
    println!("{} => {}", a_longerunion.get(), b_longerunion.get());
    assert_eq!(b_longerunion.get(), 4951760157141521099596496896_u128);

    let c_longerunion = 4000000000000000000000000000_u128.into_longerunion();
    let d_longerunion = small_uint_next_power_of_two_func(c_longerunion);
    println!("{} => {}", c_longerunion.get(), d_longerunion.get());
    assert_eq!(d_longerunion.get(), 4951760157141521099596496896_u128);

    // Example for SizeUnion
    let a_sizeunion = 4000000000000000_usize.into_sizeunion();
    let b_sizeunion = a_sizeunion.next_power_of_two();
    println!("{} => {}", a_sizeunion.get(), b_sizeunion.get());
    assert_eq!(b_sizeunion.get(), 4503599627370496_usize);

    let c_sizeunion = 4000000000000000_usize.into_sizeunion();
    let d_sizeunion = small_uint_next_power_of_two_func(c_sizeunion);
    println!("{} => {}", c_sizeunion.get(), d_sizeunion.get());
    assert_eq!(d_sizeunion.get(), 4503599627370496_usize);
    println!("--------------------------------------");
}

fn small_uint_next_power_of_two_func<T: cryptocol::number::SmallUInt>(num: T) -> T
{
    num.next_power_of_two()
}

fn small_uint_conversion()
{
    small_uint_into_f64();
    small_uint_into_f32();
    small_uint_into_u128();
    small_uint_into_u64();
    small_uint_into_u32();
    small_uint_into_u16();
    small_uint_into_u8();
    small_uint_into_usize();
    small_uint_into_bool();
    small_uint_into_shortunion();
    small_uint_into_intunion();
    small_uint_into_longunion();
    small_uint_into_longerunion();
    small_uint_into_sizeunion();
}

fn small_uint_into_f64()
{
    println!("small_uint_into_f64");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = 123_u8;
    let res = a_u8.into_f64();
    println!("{} -> {:.1}", a_u8, res);
    assert_eq!(res, 123.0_f64);

    let b_u8 = 123_u8;
    let res = small_uint_into_f64_func(b_u8);
    println!("{} -> {:.1}", b_u8, res);
    assert_eq!(res, 123.0_f64);

    // Example for u16
    let a_u16 = 12345_u16;
    let res = a_u16.into_f64();
    println!("{} -> {:.1}", a_u16, res);
    assert_eq!(res, 12345.0_f64);

    let b_u16 = 12345_u16;
    let res = small_uint_into_f64_func(b_u16);
    println!("{} -> {:.1}", b_u16, res);
    assert_eq!(res, 12345.0_f64);

    // Example for u32
    let a_u32 = 1234567890_u32;
    let res = a_u32.into_f64();
    println!("{} -> {:.1}", a_u32, res);
    assert_eq!(res, 1234567890.0_f64);

    let b_u32 = 1234567890_u32;
    let res = small_uint_into_f64_func(b_u32);
    println!("{} -> {:.1}", b_u32, res);
    assert_eq!(res, 1234567890.0_f64);

    // Example for u64
    let a_u64 = 12345678900000000000_u64;
    let res = a_u64.into_f64();
    println!("{} -> {:.1}", a_u64, res);
    assert_eq!(res, 12345678900000000000.0_f64);

    let b_u64 = 12345678900000000000_u64;
    let res = small_uint_into_f64_func(b_u64);
    println!("{} -> {:.1}", b_u64, res);
    assert_eq!(res, 12345678900000000000.0_f64);

    // Example for u128
    let a_u128 = 123456789000000000000000000000000000000_u128;
    let res = a_u128.into_f64();
    println!("{} -> {:.1}", a_u128, res);
    assert_eq!(res, 123456789000000000000000000000000000000.0_f64);

    let b_u128 = 123456789000000000000000000000000000000_u128;
    let res = small_uint_into_f64_func(b_u128);
    println!("{} -> {:.1}", b_u128, res);
    assert_eq!(res, 123456789000000000000000000000000000000.0_f64);

    // Example for usize
    let a_usize = 12345678900000000000_usize;
    let res = a_usize.into_f64();
    println!("{} -> {:.1}", a_usize, res);
    assert_eq!(res, 12345678900000000000.0_f64);

    let b_usize = 12345678900000000000_usize;
    let res = small_uint_into_f64_func(b_usize);
    println!("{} -> {:.1}", b_usize, res);
    assert_eq!(res, 12345678900000000000.0_f64);

    // Example for ShortUnion
    let a_shortunion = 12345_u16.into_shortunion();
    let res = a_shortunion.into_f64();
    println!("{} -> {:.1}", a_shortunion, res);
    assert_eq!(res, 12345.0_f64);

    let b_shortunion = 12345_u16.into_shortunion();
    let res = small_uint_into_f64_func(b_shortunion);
    println!("{} -> {:.1}", b_shortunion, res);
    assert_eq!(res, 12345.0_f64);

    // Example for IntUnion
    let a_intunion = 1234567890_u32.into_intunion();
    let res = a_intunion.into_f64();
    println!("{} -> {:.1}", a_intunion, res);
    assert_eq!(res, 1234567890.0_f64);

    let b_intunion = 1234567890_u32.into_intunion();
    let res = small_uint_into_f64_func(b_intunion);
    println!("{} -> {:.1}", b_intunion, res);
    assert_eq!(res, 1234567890.0_f64);

    // Example for LongUnion
    let a_longunion = 12345678900000000000_u64.into_longunion();
    let res = a_longunion.into_f64();
    println!("{} -> {:.1}", a_longunion, res);
    assert_eq!(res, 12345678900000000000.0_f64);

    let b_longunion = 12345678900000000000_u64.into_longunion();
    let res = small_uint_into_f64_func(b_longunion);
    println!("{} -> {:.1}", b_longunion, res);
    assert_eq!(res, 12345678900000000000.0_f64);

    // Example for LongerUnion
    let a_longerunion = 123456789000000000000000000000000000000_u128.into_longerunion();
    let res = a_longerunion.into_f64();
    println!("{} -> {:.1}", a_longerunion, res);
    assert_eq!(res, 123456789000000000000000000000000000000.0_f64);

    let b_longerunion = 123456789000000000000000000000000000000_u128.into_longerunion();
    let res = small_uint_into_f64_func(b_longerunion);
    println!("{} -> {:.1}", b_longerunion, res);
    assert_eq!(res, 123456789000000000000000000000000000000.0_f64);

    // Example for SizeUnion
    let a_sizeunion = 12345678900000000000_usize.into_sizeunion();
    let res = a_sizeunion.into_f64();
    println!("{} -> {:.1}", a_sizeunion, res);
    assert_eq!(res, 12345678900000000000.0_f64);

    let b_sizeunion = 12345678900000000000_usize.into_sizeunion();
    let res = small_uint_into_f64_func(b_sizeunion);
    println!("{} -> {:.1}", b_sizeunion, res);
    assert_eq!(res, 12345678900000000000.0_f64);
    println!("--------------------------------------");


}

fn small_uint_into_f64_func<T: cryptocol::number::SmallUInt>(num: T) -> f64
{
    num.into_f64()
}

fn small_uint_into_f32()
{
    println!("small_uint_into_f32");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = 123_u8;
    let res = a_u8.into_f32();
    println!("{} -> {:.1}", a_u8, res);
    assert_eq!(res, 123.0_f32);

    let b_u8 = 123_u8;
    let res = small_uint_into_f32_func(b_u8);
    println!("{} -> {:.1}", b_u8, res);
    assert_eq!(res, 123.0_f32);

    // Example for u16
    let a_u16 = 12345_u16;
    let res = a_u16.into_f32();
    println!("{} -> {:.1}", a_u16, res);
    assert_eq!(res, 12345.0_f32);

    let b_u16 = 12345_u16;
    let res = small_uint_into_f32_func(b_u16);
    println!("{} -> {:.1}", b_u16, res);
    assert_eq!(res, 12345.0_f32);

    // Example for u32
    let a_u32 = 1234567890_u32;
    let res = a_u32.into_f32();
    println!("{} -> {:.1}", a_u32, res);
    assert_eq!(res, 1234567890.0_f32);

    let b_u32 = 1234567890_u32;
    let res = small_uint_into_f32_func(b_u32);
    println!("{} -> {:.1}", b_u32, res);
    assert_eq!(res, 1234567890.0_f32);

    // Example for u64
    let a_u64 = 12345678900000000000_u64;
    let res = a_u64.into_f32();
    println!("{} -> {:.1}", a_u64, res);
    assert_eq!(res, 12345678900000000000.0_f32);

    let b_u64 = 12345678900000000000_u64;
    let res = small_uint_into_f32_func(b_u64);
    println!("{} -> {:.1}", b_u64, res);
    assert_eq!(res, 12345678900000000000.0_f32);

    // Example for u128
    let a_u128 = 123456789000000000000000000000000000000_u128;
    let res = a_u128.into_f32();
    println!("{} -> {:.1}", a_u128, res);
    assert_eq!(res, 123456789000000000000000000000000000000.0_f32);

    let b_u128 = 123456789000000000000000000000000000000_u128;
    let res = small_uint_into_f32_func(b_u128);
    println!("{} -> {:.1}", b_u128, res);
    assert_eq!(res, 123456789000000000000000000000000000000.0_f32);

    // Example for usize
    let a_usize = 12345678900000000000_usize;
    let res = a_usize.into_f32();
    println!("{} -> {:.1}", a_usize, res);
    assert_eq!(res, 12345678900000000000.0_f32);

    let b_usize = 12345678900000000000_usize;
    let res = small_uint_into_f32_func(b_usize);
    println!("{} -> {:.1}", b_usize, res);
    assert_eq!(res, 12345678900000000000.0_f32);

    // Example for ShortUnion
    let a_shortunion = 12345_u16.into_shortunion();
    let res = a_shortunion.into_f32();
    println!("{} -> {:.1}", a_shortunion, res);
    assert_eq!(res, 12345.0_f32);

    let b_shortunion = 12345_u16.into_shortunion();
    let res = small_uint_into_f32_func(b_shortunion);
    println!("{} -> {:.1}", b_shortunion, res);
    assert_eq!(res, 12345.0_f32);

    // Example for IntUnion
    let a_intunion = 1234567890_u32.into_intunion();
    let res = a_intunion.into_f32();
    println!("{} -> {:.1}", a_intunion, res);
    assert_eq!(res, 1234567890.0_f32);

    let b_intunion = 1234567890_u32.into_intunion();
    let res = small_uint_into_f32_func(b_intunion);
    println!("{} -> {:.1}", b_intunion, res);
    assert_eq!(res, 1234567890.0_f32);

    // Example for LongUnion
    let a_longunion = 12345678900000000000_u64.into_longunion();
    let res = a_longunion.into_f32();
    println!("{} -> {:.1}", a_longunion, res);
    assert_eq!(res, 12345678900000000000.0_f32);

    let b_longunion = 12345678900000000000_u64.into_longunion();
    let res = small_uint_into_f32_func(b_longunion);
    println!("{} -> {:.1}", b_longunion, res);
    assert_eq!(res, 12345678900000000000.0_f32);

    // Example for LongerUnion
    let a_longerunion = 123456789000000000000000000000000000000_u128.into_longerunion();
    let res = a_longerunion.into_f32();
    println!("{} -> {:.1}", a_longerunion, res);
    assert_eq!(res, 123456789000000000000000000000000000000.0_f32);

    let b_longerunion = 123456789000000000000000000000000000000_u128.into_longerunion();
    let res = small_uint_into_f32_func(b_longerunion);
    println!("{} -> {:.1}", b_longerunion, res);
    assert_eq!(res, 123456789000000000000000000000000000000.0_f32);

    // Example for SizeUnion
    let a_sizeunion = 12345678900000000000_usize.into_sizeunion();
    let res = a_sizeunion.into_f32();
    println!("{} -> {:.1}", a_sizeunion, res);
    assert_eq!(res, 12345678900000000000.0_f32);

    let b_sizeunion = 12345678900000000000_usize.into_sizeunion();
    let res = small_uint_into_f32_func(b_sizeunion);
    println!("{} -> {:.1}", b_sizeunion, res);
    assert_eq!(res, 12345678900000000000.0_f32);
    println!("--------------------------------------");
}

fn small_uint_into_f32_func<T: cryptocol::number::SmallUInt>(num: T) -> f32
{
    num.into_f32()
}

fn small_uint_into_u128()
{
    println!("small_uint_into_u128");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = 0x12_u8;
    let res = a_u8.into_u128();
    println!("{:02x} -> {:032x}", a_u8, res);
    assert_eq!(res, 0x12_u128);

    let b_u8 = 0x12_u8;
    let res = small_uint_into_u128_func(b_u8);
    println!("{:02x} -> {:032x}", b_u8, res);
    assert_eq!(res, 0x12_u128);

    // Example for u16 for Little Endianness
    let a_u16 = 0x1234_u16;
    let res = a_u16.into_u128();
    println!("{:04x} -> {:032x}", a_u16, res);
    assert_eq!(res, 0x1234_u128);

    let b_u16 = 0x1234_u16;
    let res = small_uint_into_u128_func(b_u16);
    println!("{:04x} -> {:032x}", b_u16, res);
    assert_eq!(res, 0x1234_u128);

    // Example for u32 for Little Endianness
    let a_u32 = 0x12345678_u32;
    let res = a_u32.into_u128();
    println!("{:08x} -> {:032x}", a_u32, res);
    assert_eq!(res, 0x12345678_u128);

    let b_u32 = 0x12345678_u32;
    let res = small_uint_into_u128_func(b_u32);
    println!("{:08x} -> {:032x}", b_u32, res);
    assert_eq!(res, 0x12345678_u128);

    // Example for u64 for Little Endianness
    let a_u64 = 0x123456789ABCDEF0_u64;
    let res = a_u64.into_u128();
    println!("{:016x} -> {:032x}", a_u64, res);
    assert_eq!(res, 0x123456789ABCDEF0_u128);

    let b_u64 = 0x123456789ABCDEF0_u64;
    let res = small_uint_into_u128_func(b_u64);
    println!("{:016x} -> {:032x}", b_u64, res);
    assert_eq!(res, 0x123456789ABCDEF0_u128);

    // Example for u128 for Little Endianness
    let a_u128 = 0x123456789ABCDEF0123456789ABCDEF0_u128;
    let res = a_u128.into_u128();
    println!("{:032x} -> {:032x}", a_u128, res);
    assert_eq!(res, 0x123456789ABCDEF0123456789ABCDEF0_u128);

    let b_u128 = 0x123456789ABCDEF0123456789ABCDEF0_u128;
    let res = small_uint_into_u128_func(b_u128);
    println!("{:032x} -> {:032x}", b_u128, res);
    assert_eq!(res, 0x123456789ABCDEF0123456789ABCDEF0_u128);

    // Example for usize for Little Endianness
    let a_usize = 0x123456789ABCDEF0_usize;
    let res = a_usize.into_u128();
    println!("{:016x} -> {:032x}", a_usize, res);
    assert_eq!(res, 0x123456789ABCDEF0_u128);

    let b_usize = 0x123456789ABCDEF0_usize;
    let res = small_uint_into_u128_func(b_usize);
    println!("{:016x} -> {:032x}", b_usize, res);
    assert_eq!(res, 0x123456789ABCDEF0_u128);

    // Example for ShortUnion for Little Endianness
    let a_shortunion = 0x1234_u16.into_shortunion();
    let res = a_shortunion.into_u128();
    println!("{:04x} -> {:032x}", a_shortunion.get(), res);
    assert_eq!(res, 0x1234_u128);

    let b_shortunion = 0x1234_u16.into_shortunion();
    let res = small_uint_into_u128_func(b_shortunion);
    println!("{:04x} -> {:032x}", b_shortunion.get(), res);
    assert_eq!(res, 0x1234_u128);

    // Example for IntUnion for Little Endianness
    let a_intunion = 0x12345678_u32.into_intunion();
    let res = a_intunion.into_u128();
    println!("{:08x} -> {:032x}", a_intunion.get(), res);
    assert_eq!(res, 0x12345678_u128);

    let b_intunion = 0x12345678_u32.into_intunion();
    let res = small_uint_into_u128_func(b_intunion);
    println!("{:08x} -> {:032x}", b_intunion.get(), res);
    assert_eq!(res, 0x12345678_u128);

    // Example for LongUnion for Little Endianness
    let a_longunion = 0x123456789ABCDEF0_u64.into_longunion();
    let res = a_longunion.into_u128();
    println!("{:016x} -> {:032x}", a_longunion.get(), res);
    assert_eq!(res, 0x123456789ABCDEF0_u128);

    let b_longunion = 0x123456789ABCDEF0_u64.into_longunion();
    let res = small_uint_into_u128_func(b_longunion);
    println!("{:016x} -> {:032x}", b_longunion.get(), res);
    assert_eq!(res, 0x123456789ABCDEF0_u128);

    // Example for LongerUnion for Little Endianness
    let a_longerunion = 0x123456789ABCDEF0123456789ABCDEF0_u128.into_longerunion();
    let res = a_longerunion.into_u128();
    println!("{:032x} -> {:032x}", a_longerunion.get(), res);
    assert_eq!(res, 0x123456789ABCDEF0123456789ABCDEF0_u128);

    let b_longerunion = 0x123456789ABCDEF0123456789ABCDEF0_u128.into_longerunion();
    let res = small_uint_into_u128_func(b_longerunion);
    println!("{:032x} -> {:032x}", b_longerunion.get(), res);
    assert_eq!(res, 0x123456789ABCDEF0123456789ABCDEF0_u128);

    // Example for SizeUnion for Little Endianness
    let a_sizeunion = 0x123456789ABCDEF0_usize.into_sizeunion();
    let res = a_sizeunion.into_u128();
    println!("{:016x} -> {:032x}", a_sizeunion.get(), res);
    assert_eq!(res, 0x123456789ABCDEF0_u128);

    let b_sizeunion = 0x123456789ABCDEF0_usize.into_sizeunion();
    let res = small_uint_into_u128_func(b_sizeunion);
    println!("{:016x} -> {:032x}", b_sizeunion.get(), res);
    assert_eq!(res, 0x123456789ABCDEF0_u128);
    println!("--------------------------------------");
}

fn small_uint_into_u128_func<T: cryptocol::number::SmallUInt>(num: T) -> u128
{
    num.into_u128()
}

fn small_uint_into_u64()
{
    println!("small_uint_into_u64");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = 0x12_u8;
    let res = a_u8.into_u64();
    println!("{:02x} -> {:016x}", a_u8, res);
    assert_eq!(res, 0x12_u64);

    let b_u8 = 0x12_u8;
    let res = small_uint_into_u64_func(b_u8);
    println!("{:02x} -> {:016x}", b_u8, res);
    assert_eq!(res, 0x12_u64);

    // Example for u16 for Little Endianness
    let a_u16 = 0x1234_u16;
    let res = a_u16.into_u64();
    println!("{:04x} -> {:016x}", a_u16, res);
    assert_eq!(res, 0x1234_u64);

    let b_u16 = 0x1234_u16;
    let res = small_uint_into_u64_func(b_u16);
    println!("{:04x} -> {:016x}", b_u16, res);
    assert_eq!(res, 0x1234_u64);

    // Example for u32 for Little Endianness
    let a_u32 = 0x12345678_u32;
    let res = a_u32.into_u64();
    println!("{:08x} -> {:016x}", a_u32, res);
    assert_eq!(res, 0x12345678_u64);

    let b_u32 = 0x12345678_u32;
    let res = small_uint_into_u64_func(b_u32);
    println!("{:08x} -> {:016x}", b_u32, res);
    assert_eq!(res, 0x12345678_u64);

    // Example for u64 for Little Endianness
    let a_u64 = 0x123456789ABCDEF0_u64;
    let res = a_u64.into_u64();
    println!("{:016x} -> {:016x}", a_u64, res);
    assert_eq!(res, 0x123456789ABCDEF0_u64);

    let b_u64 = 0x123456789ABCDEF0_u64;
    let res = small_uint_into_u64_func(b_u64);
    println!("{:016x} -> {:016x}", b_u64, res);
    assert_eq!(res, 0x123456789ABCDEF0_u64);

    // Example for u128 for Little Endianness
    let a_u128 = 0x123456789ABCDEF0123456789ABCDEF0_u128;
    let res = a_u128.into_u64();
    println!("{:032x} -> {:016x}", a_u128, res);
    assert_eq!(res, 0x123456789ABCDEF0_u64);

    let b_u128 = 0x123456789ABCDEF0123456789ABCDEF0_u128;
    let res = small_uint_into_u64_func(b_u128);
    println!("{:032x} -> {:016x}", b_u128, res);
    assert_eq!(res, 0x123456789ABCDEF0_u64);

    // Example for usize for Little Endianness
    let a_usize = 0x123456789ABCDEF0_usize;
    let res = a_usize.into_u64();
    println!("{:016x} -> {:016x}", a_usize, res);
    assert_eq!(res, 0x123456789ABCDEF0_u64);

    let b_usize = 0x123456789ABCDEF0_usize;
    let res = small_uint_into_u64_func(b_usize);
    println!("{:016x} -> {:016x}", b_usize, res);
    assert_eq!(res, 0x123456789ABCDEF0_u64);

    // Example for ShortUnion for Little Endianness
    let a_shortunion = 0x1234_u16.into_shortunion();
    let res = a_shortunion.into_u64();
    println!("{:04x} -> {:016x}", a_shortunion.get(), res);
    assert_eq!(res, 0x1234_u64);

    let b_shortunion = 0x1234_u16.into_shortunion();
    let res = small_uint_into_u64_func(b_shortunion);
    println!("{:04x} -> {:016x}", b_shortunion.get(), res);
    assert_eq!(res, 0x1234_u64);

    // Example for IntUnion for Little Endianness
    let a_intunion = 0x12345678_u32.into_intunion();
    let res = a_intunion.into_u64();
    println!("{:08x} -> {:016x}", a_intunion.get(), res);
    assert_eq!(res, 0x12345678_u64);

    let b_intunion = 0x12345678_u32.into_intunion();
    let res = small_uint_into_u64_func(b_intunion);
    println!("{:08x} -> {:016x}", b_intunion.get(), res);
    assert_eq!(res, 0x12345678_u64);

    // Example for LongUnion for Little Endianness
    let a_longunion = 0x123456789ABCDEF0_u64.into_longunion();
    let res = a_longunion.into_u64();
    println!("{:016x} -> {:016x}", a_longunion.get(), res);
    assert_eq!(res, 0x123456789ABCDEF0_u64);

    let b_longunion = 0x123456789ABCDEF0_u64.into_longunion();
    let res = small_uint_into_u64_func(b_longunion);
    println!("{:016x} -> {:016x}", b_longunion.get(), res);
    assert_eq!(res, 0x123456789ABCDEF0_u64);

    // Example for LongerUnion for Little Endianness
    let a_longerunion = 0x123456789ABCDEF0123456789ABCDEF0_u128.into_longerunion();
    let res = a_longerunion.into_u64();
    println!("{:032x} -> {:016x}", a_longerunion.get(), res);
    assert_eq!(res, 0x123456789ABCDEF0_u64);

    let b_longerunion = 0x123456789ABCDEF0123456789ABCDEF0_u128.into_longerunion();
    let res = small_uint_into_u64_func(b_longerunion);
    println!("{:032x} -> {:016x}", b_longerunion.get(), res);
    assert_eq!(res, 0x123456789ABCDEF0_u64);

    // Example for SizeUnion for Little Endianness
    let a_sizeunion = 0x123456789ABCDEF0_usize.into_sizeunion();
    let res = a_sizeunion.into_u64();
    println!("{:016x} -> {:016x}", a_sizeunion.get(), res);
    assert_eq!(res, 0x123456789ABCDEF0_u64);

    let b_sizeunion = 0x123456789ABCDEF0_usize.into_sizeunion();
    let res = small_uint_into_u64_func(b_sizeunion);
    println!("{:016x} -> {:016x}", b_sizeunion.get(), res);
    assert_eq!(res, 0x123456789ABCDEF0_u64);
    println!("--------------------------------------");
}

fn small_uint_into_u64_func<T: cryptocol::number::SmallUInt>(num: T) -> u64
{
    num.into_u64()
}

fn small_uint_into_u32()
{
    println!("small_uint_into_u32");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = 0x12_u8;
    let res = a_u8.into_u32();
    println!("{:02x} -> {:08x}", a_u8, res);
    assert_eq!(res, 0x12_u32);

    let b_u8 = 0x12_u8;
    let res = small_uint_into_u32_func(b_u8);
    println!("{:02x} -> {:08x}", b_u8, res);
    assert_eq!(res, 0x12_u32);

    // Example for u16 for Little Endianness
    let a_u16 = 0x1234_u16;
    let res = a_u16.into_u32();
    println!("{:04x} -> {:08x}", a_u16, res);
    assert_eq!(res, 0x1234_u32);

    let b_u16 = 0x1234_u16;
    let res = small_uint_into_u32_func(b_u16);
    println!("{:04x} -> {:08x}", b_u16, res);
    assert_eq!(res, 0x1234_u32);

    // Example for u32 for Little Endianness
    let a_u32 = 0x12345678_u32;
    let res = a_u32.into_u32();
    println!("{:08x} -> {:08x}", a_u32, res);
    assert_eq!(res, 0x12345678_u32);

    let b_u32 = 0x12345678_u32;
    let res = small_uint_into_u32_func(b_u32);
    println!("{:08x} -> {:08x}", b_u32, res);
    assert_eq!(res, 0x12345678_u32);

    // Example for u64 for Little Endianness
    let a_u64 = 0x123456789ABCDEF0_u64;
    let res = a_u64.into_u32();
    println!("{:016x} -> {:08x}", a_u64, res);
    assert_eq!(res, 0x9ABCDEF0_u32);

    let b_u64 = 0x123456789ABCDEF0_u64;
    let res = small_uint_into_u32_func(b_u64);
    println!("{:016x} -> {:08x}", b_u64, res);
    assert_eq!(res, 0x9ABCDEF0_u32);

    // Example for u128 for Little Endianness
    let a_u128 = 0x123456789ABCDEF0123456789ABCDEF0_u128;
    let res = a_u128.into_u32();
    println!("{:032x} -> {:08x}", a_u128, res);
    assert_eq!(res, 0x9ABCDEF0_u32);

    let b_u128 = 0x123456789ABCDEF0123456789ABCDEF0_u128;
    let res = small_uint_into_u32_func(b_u128);
    println!("{:032x} -> {:08x}", b_u128, res);
    assert_eq!(res, 0x9ABCDEF0_u32);

    // Example for usize for Little Endianness
    let a_usize = 0x123456789ABCDEF0_usize;
    let res = a_usize.into_u32();
    println!("{:016x} -> {:08x}", a_usize, res);
    assert_eq!(res, 0x9ABCDEF0_u32);

    let b_usize = 0x123456789ABCDEF0_usize;
    let res = small_uint_into_u32_func(b_usize);
    println!("{:016x} -> {:08x}", b_usize, res);
    assert_eq!(res, 0x9ABCDEF0_u32);

    // Example for ShortUnion for Little Endianness
    let a_shortunion = 0x1234_u16.into_shortunion();
    let res = a_shortunion.into_u32();
    println!("{:04x} -> {:08x}", a_shortunion.get(), res);
    assert_eq!(res, 0x1234_u32);

    let b_shortunion = 0x1234_u16.into_shortunion();
    let res = small_uint_into_u32_func(b_shortunion);
    println!("{:04x} -> {:08x}", b_shortunion.get(), res);
    assert_eq!(res, 0x1234_u32);

    // Example for IntUnion for Little Endianness
    let a_intunion = 0x12345678_u32.into_intunion();
    let res = a_intunion.into_u32();
    println!("{:08x} -> {:08x}", a_intunion.get(), res);
    assert_eq!(res, 0x12345678_u32);

    let b_intunion = 0x12345678_u32.into_intunion();
    let res = small_uint_into_u32_func(b_intunion);
    println!("{:08x} -> {:08x}", b_intunion.get(), res);
    assert_eq!(res, 0x12345678_u32);

    // Example for LongUnion for Little Endianness
    let a_longunion = 0x123456789ABCDEF0_u64.into_longunion();
    let res = a_longunion.into_u32();
    println!("{:016x} -> {:08x}", a_longunion.get(), res);
    assert_eq!(res, 0x9ABCDEF0_u32);

    let b_longunion = 0x123456789ABCDEF0_u64.into_longunion();
    let res = small_uint_into_u32_func(b_longunion);
    println!("{:016x} -> {:08x}", b_longunion.get(), res);
    assert_eq!(res, 0x9ABCDEF0_u32);

    // Example for LongerUnion for Little Endianness
    let a_longerunion = 0x123456789ABCDEF0123456789ABCDEF0_u128.into_longerunion();
    let res = a_longerunion.into_u32();
    println!("{:032x} -> {:08x}", a_longerunion.get(), res);
    assert_eq!(res, 0x9ABCDEF0_u32);

    let b_longerunion = 0x123456789ABCDEF0123456789ABCDEF0_u128.into_longerunion();
    let res = small_uint_into_u32_func(b_longerunion);
    println!("{:032x} -> {:08x}", b_longerunion.get(), res);
    assert_eq!(res, 0x9ABCDEF0_u32);

    // Example for SizeUnion for Little Endianness
    let a_sizeunion = 0x123456789ABCDEF0_usize.into_sizeunion();
    let res = a_sizeunion.into_u32();
    println!("{:016x} -> {:08x}", a_sizeunion.get(), res);
    assert_eq!(res, 0x9ABCDEF0_u32);

    let b_sizeunion = 0x123456789ABCDEF0_usize.into_sizeunion();
    let res = small_uint_into_u32_func(b_sizeunion);
    println!("{:016x} -> {:08x}", b_sizeunion.get(), res);
    assert_eq!(res, 0x9ABCDEF0_u32);
    println!("--------------------------------------");
}

fn small_uint_into_u32_func<T: cryptocol::number::SmallUInt>(num: T) -> u32
{
    num.into_u32()
}

fn small_uint_into_u16()
{
    println!("small_uint_into_u16");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = 0x12_u8;
    let res = a_u8.into_u16();
    println!("{:02x} -> {:04x}", a_u8, res);
    assert_eq!(res, 0x12_u16);

    let b_u8 = 0x12_u8;
    let res = small_uint_into_u16_func(b_u8);
    println!("{:02x} -> {:04x}", b_u8, res);
    assert_eq!(res, 0x12_u16);

    // Example for u16 for Little Endianness
    let a_u16 = 0x1234_u16;
    let res = a_u16.into_u16();
    println!("{:04x} -> {:04x}", a_u16, res);
    assert_eq!(res, 0x1234_u16);

    let b_u16 = 0x1234_u16;
    let res = small_uint_into_u16_func(b_u16);
    println!("{:04x} -> {:04x}", b_u16, res);
    assert_eq!(res, 0x1234_u16);

    // Example for u32 for Little Endianness
    let a_u32 = 0xDEF0_u16;
    let res = a_u32.into_u16();
    println!("{:08x} -> {:04x}", a_u32, res);
    assert_eq!(res, 0xDEF0_u16);

    let b_u32 = 0xDEF0_u16;
    let res = small_uint_into_u16_func(b_u32);
    println!("{:08x} -> {:04x}", b_u32, res);
    assert_eq!(res, 0xDEF0_u16);

    // Example for u64 for Little Endianness
    let a_u64 = 0x123456789ABCDEF0_u64;
    let res = a_u64.into_u16();
    println!("{:016x} -> {:04x}", a_u64, res);
    assert_eq!(res, 0xDEF0_u16);

    let b_u64 = 0x123456789ABCDEF0_u64;
    let res = small_uint_into_u16_func(b_u64);
    println!("{:016x} -> {:04x}", b_u64, res);
    assert_eq!(res, 0xDEF0_u16);

    // Example for u128 for Little Endianness
    let a_u128 = 0x123456789ABCDEF0123456789ABCDEF0_u128;
    let res = a_u128.into_u16();
    println!("{:032x} -> {:04x}", a_u128, res);
    assert_eq!(res, 0xDEF0_u16);

    let b_u128 = 0x123456789ABCDEF0123456789ABCDEF0_u128;
    let res = small_uint_into_u16_func(b_u128);
    println!("{:032x} -> {:04x}", b_u128, res);
    assert_eq!(res, 0xDEF0_u16);

    // Example for usize for Little Endianness
    let a_usize = 0x123456789ABCDEF0_usize;
    let res = a_usize.into_u16();
    println!("{:016x} -> {:04x}", a_usize, res);
    assert_eq!(res, 0xDEF0_u16);

    let b_usize = 0x123456789ABCDEF0_usize;
    let res = small_uint_into_u16_func(b_usize);
    println!("{:016x} -> {:04x}", b_usize, res);
    assert_eq!(res, 0xDEF0_u16);

    // Example for ShortUnion for Little Endianness
    let a_shortunion = 0x1234_u16.into_shortunion();
    let res = a_shortunion.into_u16();
    println!("{:04x} -> {:04x}", a_shortunion.get(), res);
    assert_eq!(res, 0x1234_u16);

    let b_shortunion = 0x1234_u16.into_shortunion();
    let res = small_uint_into_u16_func(b_shortunion);
    println!("{:04x} -> {:04x}", b_shortunion.get(), res);
    assert_eq!(res, 0x1234_u16);

    // Example for IntUnion for Little Endianness
    let a_intunion = 0x12345678_u32.into_intunion();
    let res = a_intunion.into_u16();
    println!("{:08x} -> {:04x}", a_intunion.get(), res);
    assert_eq!(res, 0x5678_u16);

    let b_intunion = 0x12345678_u32.into_intunion();
    let res = small_uint_into_u16_func(b_intunion);
    println!("{:08x} -> {:04x}", b_intunion.get(), res);
    assert_eq!(res, 0x5678_u16);

    // Example for LongUnion for Little Endianness
    let a_longunion = 0x123456789ABCDEF0_u64.into_longunion();
    let res = a_longunion.into_u16();
    println!("{:016x} -> {:04x}", a_longunion.get(), res);
    assert_eq!(res, 0xDEF0_u16);

    let b_longunion = 0x123456789ABCDEF0_u64.into_longunion();
    let res = small_uint_into_u16_func(b_longunion);
    println!("{:016x} -> {:04x}", b_longunion.get(), res);
    assert_eq!(res, 0xDEF0_u16);

    // Example for LongerUnion for Little Endianness
    let a_longerunion = 0x123456789ABCDEF0123456789ABCDEF0_u128.into_longerunion();
    let res = a_longerunion.into_u16();
    println!("{:032x} -> {:04x}", a_longerunion.get(), res);
    assert_eq!(res, 0xDEF0_u16);

    let b_longerunion = 0x123456789ABCDEF0123456789ABCDEF0_u128.into_longerunion();
    let res = small_uint_into_u16_func(b_longerunion);
    println!("{:032x} -> {:04x}", b_longerunion.get(), res);
    assert_eq!(res, 0xDEF0_u16);

    // Example for SizeUnion for Little Endianness
    let a_sizeunion = 0x123456789ABCDEF0_usize.into_sizeunion();
    let res = a_sizeunion.into_u16();
    println!("{:016x} -> {:04x}", a_sizeunion.get(), res);
    assert_eq!(res, 0xDEF0_u16);

    let b_sizeunion = 0x123456789ABCDEF0_usize.into_sizeunion();
    let res = small_uint_into_u16_func(b_sizeunion);
    println!("{:016x} -> {:04x}", b_sizeunion.get(), res);
    assert_eq!(res, 0xDEF0_u16);
    println!("--------------------------------------");
}

fn small_uint_into_u16_func<T: cryptocol::number::SmallUInt>(num: T) -> u16
{
    num.into_u16()
}

fn small_uint_into_u8()
{
    println!("small_uint_into_u8");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = 0x12_u8;
    let res = a_u8.into_u8();
    println!("{:02x} -> {:02x}", a_u8, res);
    assert_eq!(res, 0x12_u8);

    let b_u8 = 0x12_u8;
    let res = small_uint_into_u8_func(b_u8);
    println!("{:02x} -> {:02x}", b_u8, res);
    assert_eq!(res, 0x12_u8);

    // Example for u16 for Little Endianness
    let a_u16 = 0x1234_u16;
    let res = a_u16.into_u8();
    println!("{:04x} -> {:02x}", a_u16, res);
    assert_eq!(res, 0x34_u8);

    let b_u16 = 0x1234_u16;
    let res = small_uint_into_u8_func(b_u16);
    println!("{:04x} -> {:02x}", b_u16, res);
    assert_eq!(res, 0x34_u8);

    // Example for u32 for Little Endianness
    let a_u32 = 0xDEF0_u16;
    let res = a_u32.into_u8();
    println!("{:08x} -> {:02x}", a_u32, res);
    assert_eq!(res, 0xF0_u8);

    let b_u32 = 0xDEF0_u16;
    let res = small_uint_into_u8_func(b_u32);
    println!("{:08x} -> {:02x}", b_u32, res);
    assert_eq!(res, 0xF0_u8);

    // Example for u64 for Little Endianness
    let a_u64 = 0x123456789ABCDEF0_u64;
    let res = a_u64.into_u8();
    println!("{:016x} -> {:02x}", a_u64, res);
    assert_eq!(res, 0xF0_u8);

    let b_u64 = 0x123456789ABCDEF0_u64;
    let res = small_uint_into_u8_func(b_u64);
    println!("{:016x} -> {:02x}", b_u64, res);
    assert_eq!(res, 0xF0_u8);

    // Example for u128 for Little Endianness
    let a_u128 = 0x123456789ABCDEF0123456789ABCDEF0_u128;
    let res = a_u128.into_u8();
    println!("{:032x} -> {:02x}", a_u128, res);
    assert_eq!(res, 0xF0_u8);

    let b_u128 = 0x123456789ABCDEF0123456789ABCDEF0_u128;
    let res = small_uint_into_u8_func(b_u128);
    println!("{:032x} -> {:02x}", b_u128, res);
    assert_eq!(res, 0xF0_u8);

    // Example for usize for Little Endianness
    let a_usize = 0x123456789ABCDEF0_usize;
    let res = a_usize.into_u8();
    println!("{:016x} -> {:02x}", a_usize, res);
    assert_eq!(res, 0xF0_u8);

    let b_usize = 0x123456789ABCDEF0_usize;
    let res = small_uint_into_u8_func(b_usize);
    println!("{:016x} -> {:02x}", b_usize, res);
    assert_eq!(res, 0xF0_u8);

    // Example for ShortUnion for Little Endianness
    let a_shortunion = 0x1234_u16.into_shortunion();
    let res = a_shortunion.into_u8();
    println!("{:04x} -> {:02x}", a_shortunion.get(), res);
    assert_eq!(res, 0x34_u8);

    let b_shortunion = 0x1234_u16.into_shortunion();
    let res = small_uint_into_u8_func(b_shortunion);
    println!("{:04x} -> {:02x}", b_shortunion.get(), res);
    assert_eq!(res, 0x34_u8);

    // Example for IntUnion for Little Endianness
    let a_intunion = 0x12345678_u32.into_intunion();
    let res = a_intunion.into_u8();
    println!("{:08x} -> {:02x}", a_intunion.get(), res);
    assert_eq!(res, 0x78_u8);

    let b_intunion = 0x12345678_u32.into_intunion();
    let res = small_uint_into_u8_func(b_intunion);
    println!("{:08x} -> {:02x}", b_intunion.get(), res);
    assert_eq!(res, 0x78_u8);

    // Example for LongUnion for Little Endianness
    let a_longunion = 0x123456789ABCDEF0_u64.into_longunion();
    let res = a_longunion.into_u8();
    println!("{:016x} -> {:02x}", a_longunion.get(), res);
    assert_eq!(res, 0xF0_u8);

    let b_longunion = 0x123456789ABCDEF0_u64.into_longunion();
    let res = small_uint_into_u8_func(b_longunion);
    println!("{:016x} -> {:02x}", b_longunion.get(), res);
    assert_eq!(res, 0xF0_u8);

    // Example for LongerUnion for Little Endianness
    let a_longerunion = 0x123456789ABCDEF0123456789ABCDEF0_u128.into_longerunion();
    let res = a_longerunion.into_u8();
    println!("{:032x} -> {:02x}", a_longerunion.get(), res);
    assert_eq!(res, 0xF0_u8);

    let b_longerunion = 0x123456789ABCDEF0123456789ABCDEF0_u128.into_longerunion();
    let res = small_uint_into_u8_func(b_longerunion);
    println!("{:032x} -> {:02x}", b_longerunion.get(), res);
    assert_eq!(res, 0xF0_u8);

    // Example for SizeUnion for Little Endianness
    let a_sizeunion = 0x123456789ABCDEF0_usize.into_sizeunion();
    let res = a_sizeunion.into_u8();
    println!("{:016x} -> {:02x}", a_sizeunion.get(), res);
    assert_eq!(res, 0xF0_u8);

    let b_sizeunion = 0x123456789ABCDEF0_usize.into_sizeunion();
    let res = small_uint_into_u8_func(b_sizeunion);
    println!("{:016x} -> {:02x}", b_sizeunion.get(), res);
    assert_eq!(res, 0xF0_u8);
    println!("--------------------------------------");
}

fn small_uint_into_u8_func<T: cryptocol::number::SmallUInt>(num: T) -> u8
{
    num.into_u8()
}

fn small_uint_into_usize()
{
    println!("small_uint_into_usize");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = 0x12_u8;
    let res = a_u8.into_usize();
    println!("{:02x} -> {:016x}", a_u8, res);
    assert_eq!(res, 0x12_usize);

    let b_u8 = 0x12_u8;
    let res = small_uint_into_usize_func(b_u8);
    println!("{:02x} -> {:016x}", b_u8, res);
    assert_eq!(res, 0x12_usize);

    // Example for u16 for Little Endianness
    let a_u16 = 0x1234_u16;
    let res = a_u16.into_usize();
    println!("{:04x} -> {:016x}", a_u16, res);
    assert_eq!(res, 0x1234_usize);

    let b_u16 = 0x1234_u16;
    let res = small_uint_into_usize_func(b_u16);
    println!("{:04x} -> {:016x}", b_u16, res);
    assert_eq!(res, 0x1234_usize);

    // Example for u32 for Little Endianness
    let a_u32 = 0x12345678_u32;
    let res = a_u32.into_usize();
    println!("{:08x} -> {:016x}", a_u32, res);
    assert_eq!(res, 0x12345678_usize);

    let b_u32 = 0x12345678_u32;
    let res = small_uint_into_usize_func(b_u32);
    println!("{:08x} -> {:016x}", b_u32, res);
    assert_eq!(res, 0x12345678_usize);

    // Example for u64 for Little Endianness
    let a_u64 = 0x123456789ABCDEF0_u64;
    let res = a_u64.into_usize();
    println!("{:016x} -> {:016x}", a_u64, res);
    assert_eq!(res, 0x123456789ABCDEF0_usize);

    let b_u64 = 0x123456789ABCDEF0_u64;
    let res = small_uint_into_usize_func(b_u64);
    println!("{:016x} -> {:016x}", b_u64, res);
    assert_eq!(res, 0x123456789ABCDEF0_usize);

    // Example for u128 for Little Endianness
    let a_u128 = 0x123456789ABCDEF0123456789ABCDEF0_u128;
    let res = a_u128.into_usize();
    println!("{:032x} -> {:016x}", a_u128, res);
    assert_eq!(res, 0x123456789ABCDEF0_usize);

    let b_u128 = 0x123456789ABCDEF0123456789ABCDEF0_u128;
    let res = small_uint_into_usize_func(b_u128);
    println!("{:032x} -> {:016x}", b_u128, res);
    assert_eq!(res, 0x123456789ABCDEF0_usize);

    // Example for usize for Little Endianness
    let a_usize = 0x123456789ABCDEF0_usize;
    let res = a_usize.into_usize();
    println!("{:016x} -> {:016x}", a_usize, res);
    assert_eq!(res, 0x123456789ABCDEF0_usize);

    let b_usize = 0x123456789ABCDEF0_usize;
    let res = small_uint_into_usize_func(b_usize);
    println!("{:016x} -> {:016x}", b_usize, res);
    assert_eq!(res, 0x123456789ABCDEF0_usize);

    // Example for ShortUnion for Little Endianness
    let a_shortunion = 0x1234_u16.into_shortunion();
    let res = a_shortunion.into_usize();
    println!("{:04x} -> {:016x}", a_shortunion.get(), res);
    assert_eq!(res, 0x1234_usize);

    let b_shortunion = 0x1234_u16.into_shortunion();
    let res = small_uint_into_usize_func(b_shortunion);
    println!("{:04x} -> {:016x}", b_shortunion.get(), res);
    assert_eq!(res, 0x1234_usize);

    // Example for IntUnion for Little Endianness
    let a_intunion = 0x12345678_u32.into_intunion();
    let res = a_intunion.into_usize();
    println!("{:08x} -> {:016x}", a_intunion.get(), res);
    assert_eq!(res, 0x12345678_usize);

    let b_intunion = 0x12345678_u32.into_intunion();
    let res = small_uint_into_usize_func(b_intunion);
    println!("{:08x} -> {:016x}", b_intunion.get(), res);
    assert_eq!(res, 0x12345678_usize);

    // Example for LongUnion for Little Endianness
    let a_longunion = 0x123456789ABCDEF0_u64.into_longunion();
    let res = a_longunion.into_usize();
    println!("{:016x} -> {:016x}", a_longunion.get(), res);
    assert_eq!(res, 0x123456789ABCDEF0_usize);

    let b_longunion = 0x123456789ABCDEF0_u64.into_longunion();
    let res = small_uint_into_usize_func(b_longunion);
    println!("{:016x} -> {:016x}", b_longunion.get(), res);
    assert_eq!(res, 0x123456789ABCDEF0_usize);

    // Example for LongerUnion for Little Endianness
    let a_longerunion = 0x123456789ABCDEF0123456789ABCDEF0_u128.into_longerunion();
    let res = a_longerunion.into_usize();
    println!("{:032x} -> {:016x}", a_longerunion.get(), res);
    assert_eq!(res, 0x123456789ABCDEF0_usize);

    let b_longerunion = 0x123456789ABCDEF0123456789ABCDEF0_u128.into_longerunion();
    let res = small_uint_into_usize_func(b_longerunion);
    println!("{:032x} -> {:016x}", b_longerunion.get(), res);
    assert_eq!(res, 0x123456789ABCDEF0_usize);

    // Example for SizeUnion for Little Endianness
    let a_sizeunion = 0x123456789ABCDEF0_usize.into_sizeunion();
    let res = a_sizeunion.into_usize();
    println!("{:016x} -> {:016x}", a_sizeunion.get(), res);
    assert_eq!(res, 0x123456789ABCDEF0_usize);

    let b_sizeunion = 0x123456789ABCDEF0_usize.into_sizeunion();
    let res = small_uint_into_usize_func(b_sizeunion);
    println!("{:016x} -> {:016x}", b_sizeunion.get(), res);
    assert_eq!(res, 0x123456789ABCDEF0_usize);
    println!("--------------------------------------");
}

fn small_uint_into_usize_func<T: cryptocol::number::SmallUInt>(num: T) -> usize
{
    num.into_usize()
}


fn small_uint_into_bool()
{
    println!("small_uint_into_bool");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = 0x12_u8;
    let res = a_u8.into_bool();
    println!("{:02x} -> {}", a_u8, res);
    assert_eq!(res, true);

    let b_u8 = 0_u8;
    let res = b_u8.into_bool();
    println!("{:02x} -> {}", b_u8, res);
    assert_eq!(res, false);

    let c_u8 = 0x12_u8;
    let res = small_uint_into_bool_func(c_u8);
    println!("{:02x} -> {}", c_u8, res);
    assert_eq!(res, true);

    let d_u8 = 0_u8;
    let res = small_uint_into_bool_func(d_u8);
    println!("{:02x} -> {}", d_u8, res);
    assert_eq!(res, false);

    // Example for u16 for Little Endianness
    let a_u16 = 0x1234_u16;
    let res = a_u16.into_bool();
    println!("{:04x} -> {}", a_u16, res);
    assert_eq!(res, true);

    let b_u16 = 0_u16;
    let res = b_u16.into_bool();
    println!("{:04x} -> {}", b_u16, res);
    assert_eq!(res, false);

    let c_u16 = 0x1234_u16;
    let res = small_uint_into_bool_func(c_u16);
    println!("{:04x} -> {}", c_u16, res);
    assert_eq!(res, true);

    let d_u16 = 0_u16;
    let res = small_uint_into_bool_func(d_u16);
    println!("{:04x} -> {}", d_u16, res);
    assert_eq!(res, false);

    // Example for u32 for Little Endianness
    let a_u32 = 0x12345678_u32;
    let res = a_u32.into_bool();
    println!("{:08x} -> {}", a_u32, res);
    assert_eq!(res, true);

    let b_u32 = 0_u32;
    let res = b_u32.into_bool();
    println!("{:08x} -> {}", b_u32, res);
    assert_eq!(res, false);

    let c_u32 = 0x12345678_u32;
    let res = small_uint_into_bool_func(c_u32);
    println!("{:08x} -> {}", c_u32, res);
    assert_eq!(res, true);

    let d_u32 = 0_u32;
    let res = small_uint_into_bool_func(d_u32);
    println!("{:08x} -> {}", d_u32, res);
    assert_eq!(res, false);

    // Example for u64 for Little Endianness
    let a_u64 = 0x123456789ABCDEF0_u64;
    let res = a_u64.into_bool();
    println!("{:016x} -> {}", a_u64, res);
    assert_eq!(res, true);

    let b_u64 = 0_u64;
    let res = b_u64.into_bool();
    println!("{:016x} -> {}", b_u64, res);
    assert_eq!(res, false);

    let c_u64 = 0x123456789ABCDEF0_u64;
    let res = small_uint_into_bool_func(c_u64);
    println!("{:016x} -> {}", c_u64, res);
    assert_eq!(res, true);

    let d_u64 = 0_u64;
    let res = small_uint_into_bool_func(d_u64);
    println!("{:016x} -> {}", d_u64, res);
    assert_eq!(res, false);

    // Example for u128 for Little Endianness
    let a_u128 = 0x123456789ABCDEF0123456789ABCDEF0_u128;
    let res = a_u128.into_bool();
    println!("{:032x} -> {}", a_u128, res);
    assert_eq!(res, true);

    let b_u128 = 0_u128;
    let res = b_u128.into_bool();
    println!("{:032x} -> {}", b_u128, res);
    assert_eq!(res, false);

    let c_u128 = 0x123456789ABCDEF0123456789ABCDEF0_u128;
    let res = small_uint_into_bool_func(c_u128);
    println!("{:032x} -> {}", c_u128, res);
    assert_eq!(res, true);

    let d_u128 = 0_u128;
    let res = small_uint_into_bool_func(d_u128);
    println!("{:032x} -> {}", d_u128, res);
    assert_eq!(res, false);

    // Example for usize for Little Endianness
    let a_usize = 0x123456789ABCDEF0_usize;
    let res = a_usize.into_bool();
    println!("{:016x} -> {}", a_usize, res);
    assert_eq!(res, true);

    let b_usize = 0_usize;
    let res = b_usize.into_bool();
    println!("{:016x} -> {}", b_usize, res);
    assert_eq!(res, false);

    let c_usize = 0x123456789ABCDEF0_usize;
    let res = small_uint_into_bool_func(c_usize);
    println!("{:016x} -> {}", c_usize, res);
    assert_eq!(res, true);

    let d_usize = 0_usize;
    let res = small_uint_into_bool_func(d_usize);
    println!("{:016x} -> {}", d_usize, res);
    assert_eq!(res, false);

    // Example for ShortUnion for Little Endianness
    let a_shortunion = 0x1234_u16.into_shortunion();
    let res = a_shortunion.into_bool();
    println!("{:04x} -> {}", a_shortunion.get(), res);
    assert_eq!(res, true);

    let b_shortunion = 0_u16.into_shortunion();
    let res = b_shortunion.into_bool();
    println!("{:04x} -> {}", b_shortunion.get(), res);
    assert_eq!(res, false);

    let c_shortunion = 0x1234_u16.into_shortunion();
    let res = small_uint_into_bool_func(c_shortunion);
    println!("{:04x} -> {}", c_shortunion.get(), res);
    assert_eq!(res, true);

    let d_shortunion = 0_u16.into_shortunion();
    let res = small_uint_into_bool_func(d_shortunion);
    println!("{:04x} -> {}", d_shortunion.get(), res);
    assert_eq!(res, false);

    // Example for IntUnion for Little Endianness
    let a_intunion = 0x12345678_u32.into_intunion();
    let res = a_intunion.into_bool();
    println!("{:08x} -> {}", a_intunion.get(), res);
    assert_eq!(res, true);

    let b_intunion = 0_u32.into_intunion();
    let res = b_intunion.into_bool();
    println!("{:08x} -> {}", b_intunion.get(), res);
    assert_eq!(res, false);

    let c_intunion = 0x12345678_u32.into_intunion();
    let res = small_uint_into_bool_func(c_intunion);
    println!("{:08x} -> {}", c_intunion.get(), res);
    assert_eq!(res, true);

    let d_intunion = 0_u32.into_intunion();
    let res = small_uint_into_bool_func(d_intunion);
    println!("{:08x} -> {}", d_intunion.get(), res);
    assert_eq!(res, false);

    // Example for LongUnion for Little Endianness
    let a_longunion = 0x123456789ABCDEF0_u64.into_longunion();
    let res = a_longunion.into_bool();
    println!("{:016x} -> {}", a_longunion.get(), res);
    assert_eq!(res, true);

    let b_longunion = 0_u64.into_longunion();
    let res = b_longunion.into_bool();
    println!("{:016x} -> {}", b_longunion.get(), res);
    assert_eq!(res, false);

    let c_longunion = 0x123456789ABCDEF0_u64.into_longunion();
    let res = small_uint_into_bool_func(c_longunion);
    println!("{:016x} -> {}", c_longunion.get(), res);
    assert_eq!(res, true);

    let d_longunion = 0_u64.into_longunion();
    let res = small_uint_into_bool_func(d_longunion);
    println!("{:016x} -> {}", d_longunion.get(), res);
    assert_eq!(res, false);

    // Example for LongerUnion for Little Endianness
    let a_longerunion = 0x123456789ABCDEF0123456789ABCDEF0_u128.into_longerunion();
    let res = a_longerunion.into_bool();
    println!("{:032x} -> {}", a_longerunion.get(), res);
    assert_eq!(res, true);

    let b_longerunion = 0_u128.into_longerunion();
    let res = b_longerunion.into_bool();
    println!("{:032x} -> {}", b_longerunion.get(), res);
    assert_eq!(res, false);

    let c_longerunion = 0x123456789ABCDEF0123456789ABCDEF0_u128.into_longerunion();
    let res = small_uint_into_bool_func(c_longerunion);
    println!("{:032x} -> {}", c_longerunion.get(), res);
    assert_eq!(res, true);

    let d_longerunion = 0_u128.into_longerunion();
    let res = small_uint_into_bool_func(d_longerunion);
    println!("{:032x} -> {}", d_longerunion.get(), res);
    assert_eq!(res, false);

    // Example for SizeUnion for Little Endianness
    let a_sizeunion = 0x123456789ABCDEF0_usize.into_sizeunion();
    let res = a_sizeunion.into_bool();
    println!("{:016x} -> {}", a_sizeunion.get(), res);
    assert_eq!(res, true);

    let b_sizeunion = 0_u128.into_sizeunion();
    let res = b_sizeunion.into_bool();
    println!("{:016x} -> {}", b_sizeunion.get(), res);
    assert_eq!(res, false);

    let c_sizeunion = 0x123456789ABCDEF0_usize.into_sizeunion();
    let res = small_uint_into_bool_func(c_sizeunion);
    println!("{:016x} -> {}", c_sizeunion.get(), res);
    assert_eq!(res, true);

    let d_sizeunion = 0_u128.into_sizeunion();
    let res = small_uint_into_bool_func(d_sizeunion);
    println!("{:016x} -> {}", d_sizeunion.get(), res);
    assert_eq!(res, false);
    println!("--------------------------------------");
}

fn small_uint_into_bool_func<T: cryptocol::number::SmallUInt>(num: T) -> bool
{
    num.into_bool()
}

fn small_uint_into_shortunion()
{
    println!("small_uint_into_shortunion");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = 0x12_u8;
    let res = a_u8.into_shortunion();
    println!("{:02x} -> {:04x}", a_u8, res.get());
    assert_eq!(res.get(), 0x12_u16);

    let b_u8 = 0x12_u8;
    let res = small_uint_into_shortunion_func(b_u8);
    println!("{:02x} -> {:04x}", b_u8, res.get());
    assert_eq!(res.get(), 0x12_u16);

    // Example for u16 for Little Endianness
    let a_u16 = 0x1234_u16;
    let res = a_u16.into_shortunion();
    println!("{:04x} -> {:04x}", a_u16, res.get());
    assert_eq!(res.get(), 0x1234_u16);

    let b_u16 = 0x1234_u16;
    let res = small_uint_into_shortunion_func(b_u16);
    println!("{:04x} -> {:04x}", b_u16, res.get());
    assert_eq!(res.get(), 0x1234_u16);

    // Example for u32 for Little Endianness
    let a_u32 = 0xDEF0_u16;
    let res = a_u32.into_shortunion();
    println!("{:08x} -> {:04x}", a_u32, res.get());
    assert_eq!(res.get(), 0xDEF0_u16);

    let b_u32 = 0xDEF0_u16;
    let res = small_uint_into_shortunion_func(b_u32);
    println!("{:08x} -> {:04x}", b_u32, res.get());
    assert_eq!(res.get(), 0xDEF0_u16);

    // Example for u64 for Little Endianness
    let a_u64 = 0x123456789ABCDEF0_u64;
    let res = a_u64.into_shortunion();
    println!("{:016x} -> {:04x}", a_u64, res.get());
    assert_eq!(res.get(), 0xDEF0_u16);

    let b_u64 = 0x123456789ABCDEF0_u64;
    let res = small_uint_into_shortunion_func(b_u64);
    println!("{:016x} -> {:04x}", b_u64, res.get());
    assert_eq!(res.get(), 0xDEF0_u16);

    // Example for u128 for Little Endianness
    let a_u128 = 0x123456789ABCDEF0123456789ABCDEF0_u128;
    let res = a_u128.into_shortunion();
    println!("{:032x} -> {:04x}", a_u128, res.get());
    assert_eq!(res.get(), 0xDEF0_u16);

    let b_u128 = 0x123456789ABCDEF0123456789ABCDEF0_u128;
    let res = small_uint_into_shortunion_func(b_u128);
    println!("{:032x} -> {:04x}", b_u128, res.get());
    assert_eq!(res.get(), 0xDEF0_u16);

    // Example for usize for Little Endianness
    let a_usize = 0x123456789ABCDEF0_usize;
    let res = a_usize.into_shortunion();
    println!("{:016x} -> {:04x}", a_usize, res.get());
    assert_eq!(res.get(), 0xDEF0_u16);

    let b_usize = 0x123456789ABCDEF0_usize;
    let res = small_uint_into_shortunion_func(b_usize);
    println!("{:016x} -> {:04x}", b_usize, res.get());
    assert_eq!(res.get(), 0xDEF0_u16);

    // Example for ShortUnion for Little Endianness
    let a_shortunion = 0x1234_u16.into_shortunion();
    let res = a_shortunion.into_shortunion();
    println!("{:04x} -> {:04x}", a_shortunion.get(), res.get());
    assert_eq!(res.get(), 0x1234_u16);

    let b_shortunion = 0x1234_u16.into_shortunion();
    let res = small_uint_into_shortunion_func(b_shortunion);
    println!("{:04x} -> {:04x}", b_shortunion.get(), res.get());
    assert_eq!(res.get(), 0x1234_u16);

    // Example for IntUnion for Little Endianness
    let a_intunion = 0x12345678_u32.into_intunion();
    let res = a_intunion.into_shortunion();
    println!("{:08x} -> {:04x}", a_intunion.get(), res.get());
    assert_eq!(res.get(), 0x5678_u16);

    let b_intunion = 0x12345678_u32.into_intunion();
    let res = small_uint_into_shortunion_func(b_intunion);
    println!("{:08x} -> {:04x}", b_intunion.get(), res.get());
    assert_eq!(res.get(), 0x5678_u16);

    // Example for LongUnion for Little Endianness
    let a_longunion = 0x123456789ABCDEF0_u64.into_longunion();
    let res = a_longunion.into_shortunion();
    println!("{:016x} -> {:04x}", a_longunion.get(), res.get());
    assert_eq!(res.get(), 0xDEF0_u16);

    let b_longunion = 0x123456789ABCDEF0_u64.into_longunion();
    let res = small_uint_into_shortunion_func(b_longunion);
    println!("{:016x} -> {:04x}", b_longunion.get(), res.get());
    assert_eq!(res.get(), 0xDEF0_u16);

    // Example for LongerUnion for Little Endianness
    let a_longerunion = 0x123456789ABCDEF0123456789ABCDEF0_u128.into_longerunion();
    let res = a_longerunion.into_shortunion();
    println!("{:032x} -> {:04x}", a_longerunion.get(), res.get());
    assert_eq!(res.get(), 0xDEF0_u16);

    let b_longerunion = 0x123456789ABCDEF0123456789ABCDEF0_u128.into_longerunion();
    let res = small_uint_into_shortunion_func(b_longerunion);
    println!("{:032x} -> {:04x}", b_longerunion.get(), res.get());
    assert_eq!(res.get(), 0xDEF0_u16);

    // Example for SizeUnion for Little Endianness
    let a_sizeunion = 0x123456789ABCDEF0_usize.into_sizeunion();
    let res = a_sizeunion.into_shortunion();
    println!("{:016x} -> {:04x}", a_sizeunion.get(), res.get());
    assert_eq!(res.get(), 0xDEF0_u16);

    let b_sizeunion = 0x123456789ABCDEF0_usize.into_sizeunion();
    let res = small_uint_into_shortunion_func(b_sizeunion);
    println!("{:016x} -> {:04x}", b_sizeunion.get(), res.get());
    assert_eq!(res.get(), 0xDEF0_u16);
    println!("--------------------------------------");
}

fn small_uint_into_shortunion_func<T: cryptocol::number::SmallUInt>(num: T) -> cryptocol::number::ShortUnion
{
    num.into_shortunion()
}

fn small_uint_into_intunion()
{
    println!("small_uint_into_intunion");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = 0x12_u8;
    let res = a_u8.into_intunion();
    println!("{:02x} -> {:08x}", a_u8, res.get());
    assert_eq!(res.get(), 0x12_u32);

    let b_u8 = 0x12_u8;
    let res = small_uint_into_intunion_func(b_u8);
    println!("{:02x} -> {:08x}", b_u8, res.get());
    assert_eq!(res.get(), 0x12_u32);

    // Example for u16 for Little Endianness
    let a_u16 = 0x1234_u16;
    let res = a_u16.into_intunion();
    println!("{:04x} -> {:08x}", a_u16, res.get());
    assert_eq!(res.get(), 0x1234_u32);

    let b_u16 = 0x1234_u16;
    let res = small_uint_into_intunion_func(b_u16);
    println!("{:04x} -> {:08x}", b_u16, res.get());
    assert_eq!(res.get(), 0x1234_u32);

    // Example for u32 for Little Endianness
    let a_u32 = 0x12345678_u32;
    let res = a_u32.into_intunion();
    println!("{:08x} -> {:08x}", a_u32, res.get());
    assert_eq!(res.get(), 0x12345678_u32);

    let b_u32 = 0x12345678_u32;
    let res = small_uint_into_intunion_func(b_u32);
    println!("{:08x} -> {:08x}", b_u32, res.get());
    assert_eq!(res.get(), 0x12345678_u32);

    // Example for u64 for Little Endianness
    let a_u64 = 0x123456789ABCDEF0_u64;
    let res = a_u64.into_intunion();
    println!("{:016x} -> {:08x}", a_u64, res.get());
    assert_eq!(res.get(), 0x9ABCDEF0_u32);

    let b_u64 = 0x123456789ABCDEF0_u64;
    let res = small_uint_into_intunion_func(b_u64);
    println!("{:016x} -> {:08x}", b_u64, res.get());
    assert_eq!(res.get(), 0x9ABCDEF0_u32);

    // Example for u128 for Little Endianness
    let a_u128 = 0x123456789ABCDEF0123456789ABCDEF0_u128;
    let res = a_u128.into_intunion();
    println!("{:032x} -> {:08x}", a_u128, res.get());
    assert_eq!(res.get(), 0x9ABCDEF0_u32);

    let b_u128 = 0x123456789ABCDEF0123456789ABCDEF0_u128;
    let res = small_uint_into_intunion_func(b_u128);
    println!("{:032x} -> {:08x}", b_u128, res.get());
    assert_eq!(res.get(), 0x9ABCDEF0_u32);

    // Example for usize for Little Endianness
    let a_usize = 0x123456789ABCDEF0_usize;
    let res = a_usize.into_intunion();
    println!("{:016x} -> {:08x}", a_usize, res.get());
    assert_eq!(res.get(), 0x9ABCDEF0_u32);

    let b_usize = 0x123456789ABCDEF0_usize;
    let res = small_uint_into_intunion_func(b_usize);
    println!("{:016x} -> {:08x}", b_usize, res.get());
    assert_eq!(res.get(), 0x9ABCDEF0_u32);

    // Example for ShortUnion for Little Endianness
    let a_shortunion = 0x1234_u16.into_shortunion();
    let res = a_shortunion.into_intunion();
    println!("{:04x} -> {:08x}", a_shortunion.get(), res.get());
    assert_eq!(res.get(), 0x1234_u32);

    let b_shortunion = 0x1234_u16.into_shortunion();
    let res = small_uint_into_intunion_func(b_shortunion);
    println!("{:04x} -> {:08x}", b_shortunion.get(), res.get());
    assert_eq!(res.get(), 0x1234_u32);

    // Example for IntUnion for Little Endianness
    let a_intunion = 0x12345678_u32.into_intunion();
    let res = a_intunion.into_intunion();
    println!("{:08x} -> {:08x}", a_intunion.get(), res.get());
    assert_eq!(res.get(), 0x12345678_u32);

    let b_intunion = 0x12345678_u32.into_intunion();
    let res = small_uint_into_intunion_func(b_intunion);
    println!("{:08x} -> {:08x}", b_intunion.get(), res.get());
    assert_eq!(res.get(), 0x12345678_u32);

    // Example for LongUnion for Little Endianness
    let a_longunion = 0x123456789ABCDEF0_u64.into_longunion();
    let res = a_longunion.into_intunion();
    println!("{:016x} -> {:08x}", a_longunion.get(), res.get());
    assert_eq!(res.get(), 0x9ABCDEF0_u32);

    let b_longunion = 0x123456789ABCDEF0_u64.into_longunion();
    let res = small_uint_into_intunion_func(b_longunion);
    println!("{:016x} -> {:08x}", b_longunion.get(), res.get());
    assert_eq!(res.get(), 0x9ABCDEF0_u32);

    // Example for LongerUnion for Little Endianness
    let a_longerunion = 0x123456789ABCDEF0123456789ABCDEF0_u128.into_longerunion();
    let res = a_longerunion.into_intunion();
    println!("{:032x} -> {:08x}", a_longerunion.get(), res.get());
    assert_eq!(res.get(), 0x9ABCDEF0_u32);

    let b_longerunion = 0x123456789ABCDEF0123456789ABCDEF0_u128.into_longerunion();
    let res = small_uint_into_intunion_func(b_longerunion);
    println!("{:032x} -> {:08x}", b_longerunion.get(), res.get());
    assert_eq!(res.get(), 0x9ABCDEF0_u32);

    // Example for SizeUnion for Little Endianness
    let a_sizeunion = 0x123456789ABCDEF0_usize.into_sizeunion();
    let res = a_sizeunion.into_intunion();
    println!("{:016x} -> {:08x}", a_sizeunion.get(), res.get());
    assert_eq!(res.get(), 0x9ABCDEF0_u32);

    let b_sizeunion = 0x123456789ABCDEF0_usize.into_sizeunion();
    let res = small_uint_into_intunion_func(b_sizeunion);
    println!("{:016x} -> {:08x}", b_sizeunion.get(), res.get());
    assert_eq!(res.get(), 0x9ABCDEF0_u32);
    println!("--------------------------------------");
}

fn small_uint_into_intunion_func<T: cryptocol::number::SmallUInt>(num: T) -> cryptocol::number::IntUnion
{
    num.into_intunion()
}

fn small_uint_into_longunion()
{
    println!("small_uint_into_longunion");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = 0x12_u8;
    let res = a_u8.into_longunion();
    println!("{:02x} -> {:016x}", a_u8, res.get());
    assert_eq!(res.get(), 0x12_u64);

    let b_u8 = 0x12_u8;
    let res = small_uint_into_longunion_func(b_u8);
    println!("{:02x} -> {:016x}", b_u8, res.get());
    assert_eq!(res.get(), 0x12_u64);

    // Example for u16 for Little Endianness
    let a_u16 = 0x1234_u16;
    let res = a_u16.into_longunion();
    println!("{:04x} -> {:016x}", a_u16, res.get());
    assert_eq!(res.get(), 0x1234_u64);

    let b_u16 = 0x1234_u16;
    let res = small_uint_into_longunion_func(b_u16);
    println!("{:04x} -> {:016x}", b_u16, res.get());
    assert_eq!(res.get(), 0x1234_u64);

    // Example for u32 for Little Endianness
    let a_u32 = 0x12345678_u32;
    let res = a_u32.into_longunion();
    println!("{:08x} -> {:016x}", a_u32, res.get());
    assert_eq!(res.get(), 0x12345678_u64);

    let b_u32 = 0x12345678_u32;
    let res = small_uint_into_longunion_func(b_u32);
    println!("{:08x} -> {:016x}", b_u32, res.get());
    assert_eq!(res.get(), 0x12345678_u64);

    // Example for u64 for Little Endianness
    let a_u64 = 0x123456789ABCDEF0_u64;
    let res = a_u64.into_longunion();
    println!("{:016x} -> {:016x}", a_u64, res.get());
    assert_eq!(res.get(), 0x123456789ABCDEF0_u64);

    let b_u64 = 0x123456789ABCDEF0_u64;
    let res = small_uint_into_longunion_func(b_u64);
    println!("{:016x} -> {:016x}", b_u64, res.get());
    assert_eq!(res.get(), 0x123456789ABCDEF0_u64);

    // Example for u128 for Little Endianness
    let a_u128 = 0x123456789ABCDEF0123456789ABCDEF0_u128;
    let res = a_u128.into_longunion();
    println!("{:032x} -> {:016x}", a_u128, res.get());
    assert_eq!(res.get(), 0x123456789ABCDEF0_u64);

    let b_u128 = 0x123456789ABCDEF0123456789ABCDEF0_u128;
    let res = small_uint_into_longunion_func(b_u128);
    println!("{:032x} -> {:016x}", b_u128, res.get());
    assert_eq!(res.get(), 0x123456789ABCDEF0_u64);

    // Example for usize for Little Endianness
    let a_usize = 0x123456789ABCDEF0_usize;
    let res = a_usize.into_longunion();
    println!("{:016x} -> {:016x}", a_usize, res.get());
    assert_eq!(res.get(), 0x123456789ABCDEF0_u64);

    let b_usize = 0x123456789ABCDEF0_usize;
    let res = small_uint_into_longunion_func(b_usize);
    println!("{:016x} -> {:016x}", b_usize, res.get());
    assert_eq!(res.get(), 0x123456789ABCDEF0_u64);

    // Example for ShortUnion for Little Endianness
    let a_shortunion = 0x1234_u16.into_shortunion();
    let res = a_shortunion.into_longunion();
    println!("{:04x} -> {:016x}", a_shortunion.get(), res.get());
    assert_eq!(res.get(), 0x1234_u64);

    let b_shortunion = 0x1234_u16.into_shortunion();
    let res = small_uint_into_longunion_func(b_shortunion);
    println!("{:04x} -> {:016x}", b_shortunion.get(), res.get());
    assert_eq!(res.get(), 0x1234_u64);

    // Example for IntUnion for Little Endianness
    let a_intunion = 0x12345678_u32.into_intunion();
    let res = a_intunion.into_longunion();
    println!("{:08x} -> {:016x}", a_intunion.get(), res.get());
    assert_eq!(res.get(), 0x12345678_u64);

    let b_intunion = 0x12345678_u32.into_intunion();
    let res = small_uint_into_longunion_func(b_intunion);
    println!("{:08x} -> {:016x}", b_intunion.get(), res.get());
    assert_eq!(res.get(), 0x12345678_u64);

    // Example for LongUnion for Little Endianness
    let a_longunion = 0x123456789ABCDEF0_u64.into_longunion();
    let res = a_longunion.into_longunion();
    println!("{:016x} -> {:016x}", a_longunion.get(), res.get());
    assert_eq!(res.get(), 0x123456789ABCDEF0_u64);

    let b_longunion = 0x123456789ABCDEF0_u64.into_longunion();
    let res = small_uint_into_longunion_func(b_longunion);
    println!("{:016x} -> {:016x}", b_longunion.get(), res.get());
    assert_eq!(res.get(), 0x123456789ABCDEF0_u64);

    // Example for LongerUnion for Little Endianness
    let a_longerunion = 0x123456789ABCDEF0123456789ABCDEF0_u128.into_longerunion();
    let res = a_longerunion.into_longunion();
    println!("{:032x} -> {:016x}", a_longerunion.get(), res.get());
    assert_eq!(res.get(), 0x123456789ABCDEF0_u64);

    let b_longerunion = 0x123456789ABCDEF0123456789ABCDEF0_u128.into_longerunion();
    let res = small_uint_into_longunion_func(b_longerunion);
    println!("{:032x} -> {:016x}", b_longerunion.get(), res.get());
    assert_eq!(res.get(), 0x123456789ABCDEF0_u64);

    // Example for SizeUnion for Little Endianness
    let a_sizeunion = 0x123456789ABCDEF0_usize.into_sizeunion();
    let res = a_sizeunion.into_longunion();
    println!("{:016x} -> {:016x}", a_sizeunion.get(), res.get());
    assert_eq!(res.get(), 0x123456789ABCDEF0_u64);

    let b_sizeunion = 0x123456789ABCDEF0_usize.into_sizeunion();
    let res = small_uint_into_longunion_func(b_sizeunion);
    println!("{:016x} -> {:016x}", b_sizeunion.get(), res.get());
    assert_eq!(res.get(), 0x123456789ABCDEF0_u64);
    println!("--------------------------------------");
}

fn small_uint_into_longunion_func<T: cryptocol::number::SmallUInt>(num: T) -> cryptocol::number::LongUnion
{
    num.into_longunion()
}

fn small_uint_into_longerunion()
{
    println!("small_uint_into_longerunion");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = 0x12_u8;
    let res = a_u8.into_longerunion();
    println!("{:02x} -> {:032x}", a_u8, res.get());
    assert_eq!(res.get(), 0x12_u128);

    let b_u8 = 0x12_u8;
    let res = small_uint_into_longerunion_func(b_u8);
    println!("{:02x} -> {:032x}", b_u8, res.get());
    assert_eq!(res.get(), 0x12_u128);

    // Example for u16 for Little Endianness
    let a_u16 = 0x1234_u16;
    let res = a_u16.into_longerunion();
    println!("{:04x} -> {:032x}", a_u16, res.get());
    assert_eq!(res.get(), 0x1234_u128);

    let b_u16 = 0x1234_u16;
    let res = small_uint_into_longerunion_func(b_u16);
    println!("{:04x} -> {:032x}", b_u16, res.get());
    assert_eq!(res.get(), 0x1234_u128);

    // Example for u32 for Little Endianness
    let a_u32 = 0x12345678_u32;
    let res = a_u32.into_longerunion();
    println!("{:08x} -> {:032x}", a_u32, res.get());
    assert_eq!(res.get(), 0x12345678_u128);

    let b_u32 = 0x12345678_u32;
    let res = small_uint_into_longerunion_func(b_u32);
    println!("{:08x} -> {:032x}", b_u32, res.get());
    assert_eq!(res.get(), 0x12345678_u128);

    // Example for u64 for Little Endianness
    let a_u64 = 0x123456789ABCDEF0_u64;
    let res = a_u64.into_longerunion();
    println!("{:016x} -> {:032x}", a_u64, res.get());
    assert_eq!(res.get(), 0x123456789ABCDEF0_u128);

    let b_u64 = 0x123456789ABCDEF0_u64;
    let res = small_uint_into_longerunion_func(b_u64);
    println!("{:016x} -> {:032x}", b_u64, res.get());
    assert_eq!(res.get(), 0x123456789ABCDEF0_u128);

    // Example for u128 for Little Endianness
    let a_u128 = 0x123456789ABCDEF0123456789ABCDEF0_u128;
    let res = a_u128.into_longerunion();
    println!("{:032x} -> {:032x}", a_u128, res.get());
    assert_eq!(res.get(), 0x123456789ABCDEF0123456789ABCDEF0_u128);

    let b_u128 = 0x123456789ABCDEF0123456789ABCDEF0_u128;
    let res = small_uint_into_longerunion_func(b_u128);
    println!("{:032x} -> {:032x}", b_u128, res.get());
    assert_eq!(res.get(), 0x123456789ABCDEF0123456789ABCDEF0_u128);

    // Example for usize for Little Endianness
    let a_usize = 0x123456789ABCDEF0_usize;
    let res = a_usize.into_longerunion();
    println!("{:016x} -> {:032x}", a_usize, res.get());
    assert_eq!(res.get(), 0x123456789ABCDEF0_u128);

    let b_usize = 0x123456789ABCDEF0_usize;
    let res = small_uint_into_longerunion_func(b_usize);
    println!("{:016x} -> {:032x}", b_usize, res.get());
    assert_eq!(res.get(), 0x123456789ABCDEF0_u128);

    // Example for ShortUnion for Little Endianness
    let a_shortunion = 0x1234_u16.into_shortunion();
    let res = a_shortunion.into_longerunion();
    println!("{:04x} -> {:032x}", a_shortunion.get(), res.get());
    assert_eq!(res.get(), 0x1234_u128);

    let b_shortunion = 0x1234_u16.into_shortunion();
    let res = small_uint_into_longerunion_func(b_shortunion);
    println!("{:04x} -> {:032x}", b_shortunion.get(), res.get());
    assert_eq!(res.get(), 0x1234_u128);

    // Example for IntUnion for Little Endianness
    let a_intunion = 0x12345678_u32.into_intunion();
    let res = a_intunion.into_longerunion();
    println!("{:08x} -> {:032x}", a_intunion.get(), res.get());
    assert_eq!(res.get(), 0x12345678_u128);

    let b_intunion = 0x12345678_u32.into_intunion();
    let res = small_uint_into_longerunion_func(b_intunion);
    println!("{:08x} -> {:032x}", b_intunion.get(), res.get());
    assert_eq!(res.get(), 0x12345678_u128);

    // Example for LongUnion for Little Endianness
    let a_longunion = 0x123456789ABCDEF0_u64.into_longunion();
    let res = a_longunion.into_longerunion();
    println!("{:016x} -> {:032x}", a_longunion.get(), res.get());
    assert_eq!(res.get(), 0x123456789ABCDEF0_u128);

    let b_longunion = 0x123456789ABCDEF0_u64.into_longunion();
    let res = small_uint_into_longerunion_func(b_longunion);
    println!("{:016x} -> {:032x}", b_longunion.get(), res.get());
    assert_eq!(res.get(), 0x123456789ABCDEF0_u128);

    // Example for LongerUnion for Little Endianness
    let a_longerunion = 0x123456789ABCDEF0123456789ABCDEF0_u128.into_longerunion();
    let res = a_longerunion.into_longerunion();
    println!("{:032x} -> {:032x}", a_longerunion.get(), res.get());
    assert_eq!(res.get(), 0x123456789ABCDEF0123456789ABCDEF0_u128);

    let b_longerunion = 0x123456789ABCDEF0123456789ABCDEF0_u128.into_longerunion();
    let res = small_uint_into_longerunion_func(b_longerunion);
    println!("{:032x} -> {:032x}", b_longerunion.get(), res.get());
    assert_eq!(res.get(), 0x123456789ABCDEF0123456789ABCDEF0_u128);

    // Example for SizeUnion for Little Endianness
    let a_sizeunion = 0x123456789ABCDEF0_usize.into_sizeunion();
    let res = a_sizeunion.into_longerunion();
    println!("{:016x} -> {:032x}", a_sizeunion.get(), res.get());
    assert_eq!(res.get(), 0x123456789ABCDEF0_u128);

    let b_sizeunion = 0x123456789ABCDEF0_usize.into_sizeunion();
    let res = small_uint_into_longerunion_func(b_sizeunion);
    println!("{:016x} -> {:032x}", b_sizeunion.get(), res.get());
    assert_eq!(res.get(), 0x123456789ABCDEF0_u128);
    println!("--------------------------------------");
}

fn small_uint_into_longerunion_func<T: cryptocol::number::SmallUInt>(num: T) -> cryptocol::number::LongerUnion
{
    num.into_longerunion()
}

fn small_uint_into_sizeunion()
{
    println!("small_uint_into_sizeunion");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = 0x12_u8;
    let res = a_u8.into_sizeunion();
    println!("{:02x} -> {:016x}", a_u8, res.get());
    assert_eq!(res.get(), 0x12_usize);

    let b_u8 = 0x12_u8;
    let res = small_uint_into_sizeunion_func(b_u8);
    println!("{:02x} -> {:016x}", b_u8, res.get());
    assert_eq!(res.get(), 0x12_usize);

    // Example for u16 for Little Endianness
    let a_u16 = 0x1234_u16;
    let res = a_u16.into_sizeunion();
    println!("{:04x} -> {:016x}", a_u16, res.get());
    assert_eq!(res.get(), 0x1234_usize);

    let b_u16 = 0x1234_u16;
    let res = small_uint_into_sizeunion_func(b_u16);
    println!("{:04x} -> {:016x}", b_u16, res.get());
    assert_eq!(res.get(), 0x1234_usize);

    // Example for u32 for Little Endianness
    let a_u32 = 0x12345678_u32;
    let res = a_u32.into_sizeunion();
    println!("{:08x} -> {:016x}", a_u32, res.get());
    assert_eq!(res.get(), 0x12345678_usize);

    let b_u32 = 0x12345678_u32;
    let res = small_uint_into_sizeunion_func(b_u32);
    println!("{:08x} -> {:016x}", b_u32, res.get());
    assert_eq!(res.get(), 0x12345678_usize);

    // Example for u64 for Little Endianness
    let a_u64 = 0x123456789ABCDEF0_u64;
    let res = a_u64.into_sizeunion();
    println!("{:016x} -> {:016x}", a_u64, res.get());
    assert_eq!(res.get(), 0x123456789ABCDEF0_usize);

    let b_u64 = 0x123456789ABCDEF0_u64;
    let res = small_uint_into_sizeunion_func(b_u64);
    println!("{:016x} -> {:016x}", b_u64, res.get());
    assert_eq!(res.get(), 0x123456789ABCDEF0_usize);

    // Example for u128 for Little Endianness
    let a_u128 = 0x123456789ABCDEF0123456789ABCDEF0_u128;
    let res = a_u128.into_sizeunion();
    println!("{:032x} -> {:016x}", a_u128, res.get());
    assert_eq!(res.get(), 0x123456789ABCDEF0_usize);

    let b_u128 = 0x123456789ABCDEF0123456789ABCDEF0_u128;
    let res = small_uint_into_sizeunion_func(b_u128);
    println!("{:032x} -> {:016x}", b_u128, res.get());
    assert_eq!(res.get(), 0x123456789ABCDEF0_usize);

    // Example for usize for Little Endianness
    let a_usize = 0x123456789ABCDEF0_usize;
    let res = a_usize.into_sizeunion();
    println!("{:016x} -> {:016x}", a_usize, res.get());
    assert_eq!(res.get(), 0x123456789ABCDEF0_usize);

    let b_usize = 0x123456789ABCDEF0_usize;
    let res = small_uint_into_sizeunion_func(b_usize);
    println!("{:016x} -> {:016x}", b_usize, res.get());
    assert_eq!(res.get(), 0x123456789ABCDEF0_usize);

    // Example for ShortUnion for Little Endianness
    let a_shortunion = 0x1234_u16.into_shortunion();
    let res = a_shortunion.into_sizeunion();
    println!("{:04x} -> {:016x}", a_shortunion.get(), res.get());
    assert_eq!(res.get(), 0x1234_usize);

    let b_shortunion = 0x1234_u16.into_shortunion();
    let res = small_uint_into_sizeunion_func(b_shortunion);
    println!("{:04x} -> {:016x}", b_shortunion.get(), res.get());
    assert_eq!(res.get(), 0x1234_usize);

    // Example for IntUnion for Little Endianness
    let a_intunion = 0x12345678_u32.into_intunion();
    let res = a_intunion.into_sizeunion();
    println!("{:08x} -> {:016x}", a_intunion.get(), res.get());
    assert_eq!(res.get(), 0x12345678_usize);

    let b_intunion = 0x12345678_u32.into_intunion();
    let res = small_uint_into_sizeunion_func(b_intunion);
    println!("{:08x} -> {:016x}", b_intunion.get(), res.get());
    assert_eq!(res.get(), 0x12345678_usize);

    // Example for LongUnion for Little Endianness
    let a_longunion = 0x123456789ABCDEF0_u64.into_longunion();
    let res = a_longunion.into_sizeunion();
    println!("{:016x} -> {:016x}", a_longunion.get(), res.get());
    assert_eq!(res.get(), 0x123456789ABCDEF0_usize);

    let b_longunion = 0x123456789ABCDEF0_u64.into_longunion();
    let res = small_uint_into_sizeunion_func(b_longunion);
    println!("{:016x} -> {:016x}", b_longunion.get(), res.get());
    assert_eq!(res.get(), 0x123456789ABCDEF0_usize);

    // Example for LongerUnion for Little Endianness
    let a_longerunion = 0x123456789ABCDEF0123456789ABCDEF0_u128.into_longerunion();
    let res = a_longerunion.into_sizeunion();
    println!("{:032x} -> {:016x}", a_longerunion.get(), res.get());
    assert_eq!(res.get(), 0x123456789ABCDEF0_usize);

    let b_longerunion = 0x123456789ABCDEF0123456789ABCDEF0_u128.into_longerunion();
    let res = small_uint_into_sizeunion_func(b_longerunion);
    println!("{:032x} -> {:016x}", b_longerunion.get(), res.get());
    assert_eq!(res.get(), 0x123456789ABCDEF0_usize);

    // Example for SizeUnion for Little Endianness
    let a_sizeunion = 0x123456789ABCDEF0_usize.into_sizeunion();
    let res = a_sizeunion.into_sizeunion();
    println!("{:016x} -> {:016x}", a_sizeunion.get(), res.get());
    assert_eq!(res.get(), 0x123456789ABCDEF0_usize);

    let b_sizeunion = 0x123456789ABCDEF0_usize.into_sizeunion();
    let res = small_uint_into_sizeunion_func(b_sizeunion);
    println!("{:016x} -> {:016x}", b_sizeunion.get(), res.get());
    assert_eq!(res.get(), 0x123456789ABCDEF0_usize);
    println!("--------------------------------------");
}

fn small_uint_into_sizeunion_func<T: cryptocol::number::SmallUInt>(num: T) -> cryptocol::number::SizeUnion
{
    num.into_sizeunion()
}

fn small_uint_constants()
{
    small_uint_zero();
    small_uint_one();
    small_uint_max();
    small_uint_min();
    small_uint_u128_as_smalluint();
    small_uint_u64_as_smalluint();
    small_uint_u32_as_smalluint();
    small_uint_u16_as_smalluint();
    small_uint_u8_as_smalluint();
    small_uint_usize_as_smalluint();
    small_uint_bool_as_smalluint();
    small_uint_num();
    small_uint_set_zero();
    small_uint_is_zero();
    small_uint_set_one();
    small_uint_is_one();
    small_uint_is_zero_or_one();
    small_uint_set_max();
    small_uint_is_max();
    small_uint_set_submax();
    small_uint_set_halfmax();
}

fn small_uint_zero()
{
    println!("small_uint_zero");
    use cryptocol::number::{ SmallUInt, ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };
    // Example for u8
    let zero_u8 = u8::zero();
    println!("zero_u8 = {}", zero_u8);
    assert_eq!(zero_u8, 0_u8);

    let zero_u8 = small_uint_zero_func::<u8>();
    println!("zero_u8 = {}", zero_u8);
    assert_eq!(zero_u8, 0_u8);

    // Example for u16
    let zero_u16 = u16::zero();
    println!("zero_u16 = {}", zero_u16);
    assert_eq!(zero_u16, 0_u16);

    let zero_u16 = small_uint_zero_func::<u16>();
    println!("zero_u16 = {}", zero_u16);
    assert_eq!(zero_u16, 0_u16);

    // Example for u32
    let zero_u32 = u32::zero();
    println!("zero_u32 = {}", zero_u32);
    assert_eq!(zero_u32, 0_u32);

    let zero_u32 = small_uint_zero_func::<u32>();
    println!("zero_u32 = {}", zero_u32);
    assert_eq!(zero_u32, 0_u32);

    // Example for u64
    let zero_u64 = u64::zero();
    println!("zero_u64 = {}", zero_u64);
    assert_eq!(zero_u64, 0_u64);

    let zero_u64 = small_uint_zero_func::<u64>();
    println!("zero_u64 = {}", zero_u64);
    assert_eq!(zero_u64, 0_u64);

    // Example for u128
    let zero_u128 = u128::zero();
    println!("zero_u128 = {}", zero_u128);
    assert_eq!(zero_u128, 0_u128);

    let zero_u128 = small_uint_zero_func::<u128>();
    println!("zero_u128 = {}", zero_u128);
    assert_eq!(zero_u128, 0_u128);

    // Example for usize
    let zero_usize = usize::zero();
    println!("zero_usize = {}", zero_usize);
    assert_eq!(zero_usize, 0_usize);

    let zero_usize = small_uint_zero_func::<usize>();
    println!("zero_usize = {}", zero_usize);
    assert_eq!(zero_usize, 0_usize);

    // Example for ShortUnion
    let zero_shortunion = ShortUnion::zero();
    println!("zero_shortunion = {}", zero_shortunion);
    assert_eq!(zero_shortunion.get(), 0_u16);

    let zero_shortunion = small_uint_zero_func::<ShortUnion>();
    println!("zero_shortunion = {}", zero_shortunion);
    assert_eq!(zero_shortunion.get(), 0_u16);

    // Example for IntUnion
    let zero_intunion = IntUnion::zero();
    println!("zero_intunion = {}", zero_intunion);
    assert_eq!(zero_intunion.get(), 0_u32);

    let zero_intunion = small_uint_zero_func::<IntUnion>();
    println!("zero_intunion = {}", zero_intunion);
    assert_eq!(zero_intunion.get(), 0_u32);

    // Example for LongUnion
    let zero_longunion = LongUnion::zero();
    println!("zero_longunion = {}", zero_longunion);
    assert_eq!(zero_longunion.get(), 0_u64);

    let zero_longunion = small_uint_zero_func::<LongUnion>();
    println!("zero_longunion = {}", zero_longunion);
    assert_eq!(zero_longunion.get(), 0_u64);

    // Example for LongerUnion
    let zero_longerunion = LongerUnion::zero();
    println!("zero_longerunion = {}", zero_longerunion);
    assert_eq!(zero_longerunion.get(), 0_u128);

    let zero_longerunion = small_uint_zero_func::<LongerUnion>();
    println!("zero_longerunion = {}", zero_longerunion);
    assert_eq!(zero_longerunion.get(), 0_u128);

    // Example for SizeUnion
    let zero_sizeunion = SizeUnion::zero();
    println!("zero_sizeunion = {}", zero_sizeunion);
    assert_eq!(zero_sizeunion.get(), 0_usize);

    let zero_sizeunion = small_uint_zero_func::<SizeUnion>();
    println!("zero_sizeunion = {}", zero_sizeunion);
    assert_eq!(zero_sizeunion.get(), 0_usize);
    println!("--------------------------------------");
}

fn small_uint_zero_func<T: cryptocol::number::SmallUInt>() -> T
{
    T::zero()
}

fn small_uint_one()
{
    println!("small_uint_one");
    use cryptocol::number::{ SmallUInt, ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };
    // Example for u8
    let a_one_u8 = u8::one();
    println!("a_one_u8 = {}", a_one_u8);
    assert_eq!(a_one_u8, 1_u8);
    let b_one_u8 = small_uint_one_func::<u8>();
    println!("b_one_u8 = {}", b_one_u8);
    assert_eq!(b_one_u8, 1_u8);

    // Example for u16
    let a_one_u16 = u16::one();
    println!("a_one_u16 = {}", a_one_u16);
    assert_eq!(a_one_u16, 1_u16);
    let b_one_u16 = small_uint_one_func::<u16>();
    println!("b_one_u16 = {}", b_one_u16);
    assert_eq!(b_one_u16, 1_u16);

    // Example for u32
    let a_one_u32 = u32::one();
    println!("a_one_u32 = {}", a_one_u32);
    assert_eq!(a_one_u32, 1_u32);
    let b_one_u32 = small_uint_one_func::<u32>();
    println!("b_one_u32 = {}", b_one_u32);
    assert_eq!(b_one_u32, 1_u32);

    // Example for u64
    let a_one_u64 = u64::one();
    println!("a_one_u64 = {}", a_one_u64);
    assert_eq!(a_one_u64, 1_u64);
    let b_one_u64 = small_uint_one_func::<u64>();
    println!("b_one_u64 = {}", b_one_u64);
    assert_eq!(b_one_u64, 1_u64);

    // Example for u128
    let a_one_u128 = u128::one();
    println!("a_one_u128 = {}", a_one_u128);
    assert_eq!(a_one_u128, 1_u128);
    let b_one_u128 = small_uint_one_func::<u128>();
    println!("b_one_u128 = {}", b_one_u128);
    assert_eq!(b_one_u128, 1_u128);

    // Example for usize
    let a_one_usize = usize::one();
    println!("a_one_usize = {}", a_one_usize);
    assert_eq!(a_one_usize, 1_usize);
    let b_one_usize = small_uint_one_func::<usize>();
    println!("b_one_usize = {}", b_one_usize);
    assert_eq!(b_one_usize, 1_usize);

    // Example for ShortUnion
    let a_one_shortunion = ShortUnion::one();
    println!("a_one_shortunion = {}", a_one_shortunion);
    assert_eq!(a_one_shortunion.get(), 1_u16);
    let b_one_shortunion = small_uint_one_func::<ShortUnion>();
    println!("b_one_shortunion = {}", b_one_shortunion);
    assert_eq!(b_one_shortunion.get(), 1_u16);

    // Example for IntUnion
    let a_one_intunion = IntUnion::one();
    println!("a_one_intunion = {}", a_one_intunion);
    assert_eq!(a_one_intunion.get(), 1_u32);
    let b_one_intunion = small_uint_one_func::<IntUnion>();
    println!("b_one_intunion = {}", b_one_intunion);
    assert_eq!(b_one_intunion.get(), 1_u32);

    // Example for LongUnion
    let a_one_longunion = LongUnion::one();
    println!("a_one_longunion = {}", a_one_longunion);
    assert_eq!(a_one_longunion.get(), 1_u64);
    let b_one_longunion = small_uint_one_func::<LongUnion>();
    println!("b_one_longunion = {}", b_one_longunion);
    assert_eq!(b_one_longunion.get(), 1_u64);

    // Example for LongerUnion
    let a_one_longerunion = LongerUnion::one();
    println!("a_one_longerunion = {}", a_one_longerunion);
    assert_eq!(a_one_longerunion.get(), 1_u128);
    let b_one_longerunion = small_uint_one_func::<LongerUnion>();
    println!("b_one_longerunion = {}", b_one_longerunion);
    assert_eq!(b_one_longerunion.get(), 1_u128);

    // Example for SizeUnion
    let a_one_sizeunion = SizeUnion::one();
    println!("a_one_sizeunion = {}", a_one_sizeunion);
    assert_eq!(a_one_sizeunion.get(), 1_usize);
    let b_one_sizeunion = small_uint_one_func::<SizeUnion>();
    println!("b_one_sizeunion = {}", b_one_sizeunion);
    assert_eq!(b_one_sizeunion.get(), 1_usize);
    println!("--------------------------------------");
}

fn small_uint_one_func<T: cryptocol::number::SmallUInt>() -> T
{
    T::one()
}

fn small_uint_max()
{
    println!("small_uint_max");
    use cryptocol::number::{ SmallUInt, ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };
    // Example for u8
    let a_max_u8 = u8::MAX; // You are encouraged to use u8::MAX rather than u8::max().
    println!("a_max_u8 = {}", a_max_u8);
    assert_eq!(a_max_u8, 255_u8);

    let b_max_u8 = small_uint_max_func::<u8>();
    println!("b_max_u8 = {}", b_max_u8);
    assert_eq!(b_max_u8, 255_u8);

    // Example for u16
    let a_max_u16 = u16::MAX; // You are encouraged to use u16::MAX rather than u16::max().
    println!("a_max_u16 = {}", a_max_u16);
    assert_eq!(a_max_u16, 65535_u16);

    let b_max_u16 = small_uint_max_func::<u16>();
    println!("b_max_u16 = {}", b_max_u16);
    assert_eq!(b_max_u16, 65535_u16);

    // Example for u32
    let a_max_u32 = u32::MAX; // You are encouraged to use u32::MAX rather than u32::max().
    println!("a_max_u32 = {}", a_max_u32);
    assert_eq!(a_max_u32, 4294967295_u32);

    let b_max_u32 = small_uint_max_func::<u32>();
    println!("b_max_u32 = {}", b_max_u32);
    assert_eq!(b_max_u32, 4294967295_u32);

    // Example for u64
    let a_max_u64 = u64::MAX; // You are encouraged to use u64::MAX rather than u64::max().
    println!("a_max_u64 = {}", a_max_u64);
    assert_eq!(a_max_u64, 18446744073709551615_u64);

    let b_max_u64 = small_uint_max_func::<u64>();
    println!("b_max_u64 = {}", b_max_u64);
    assert_eq!(b_max_u64, 18446744073709551615_u64);

    // Example for u128
    let a_max_u128 = u128::MAX; // You are encouraged to use u128::MAX rather than u128::max().
    println!("a_max_u128 = {}", a_max_u128);
    assert_eq!(a_max_u128, 340282366920938463463374607431768211455_u128);

    let b_max_u128 = small_uint_max_func::<u128>();
    println!("b_max_u128 = {}", b_max_u128);
    assert_eq!(b_max_u128, 340282366920938463463374607431768211455_u128);

    // Example for usize
    let a_max_usize = usize::MAX; // You are encouraged to use usize::MAX rather than usize::max().
    println!("a_max_usize = {}", a_max_usize);
    assert_eq!(a_max_usize, 18446744073709551615_usize);

    let b_max_usize = small_uint_max_func::<usize>();
    println!("b_max_usize = {}", b_max_usize);
    assert_eq!(b_max_usize, 18446744073709551615_usize);

    // Example for ShortUnion
    let a_max_shortunion = ShortUnion::max();
    println!("a_max_shortunion = {}", a_max_shortunion);
    assert_eq!(a_max_shortunion.get(), 65535_u16);

    let b_max_shortunion = small_uint_max_func::<ShortUnion>();
    println!("b_max_shortunion = {}", b_max_shortunion);
    assert_eq!(b_max_shortunion.get(), 65535_u16);

    // Example for IntUnion
    let a_max_intunion = IntUnion::max();
    println!("a_max_intunion = {}", a_max_intunion);
    assert_eq!(a_max_intunion.get(), 4294967295_u32);

    let b_max_intunion = small_uint_max_func::<IntUnion>();
    println!("b_max_intunion = {}", b_max_intunion);
    assert_eq!(b_max_intunion.get(), 4294967295_u32);

    // Example for LongUnion
    let a_max_longunion = LongUnion::max();
    println!("a_max_longunion = {}", a_max_intunion);
    assert_eq!(a_max_longunion.get(), 18446744073709551615_u64);

    let b_max_longunion = small_uint_max_func::<LongUnion>();
    println!("b_max_longunion = {}", b_max_longunion);
    assert_eq!(b_max_longunion.get(), 18446744073709551615_u64);

    // Example for LongerUnion
    let a_max_longerunion = LongerUnion::max();
    println!("a_max_longerunion = {}", a_max_longerunion);
    assert_eq!(a_max_longerunion.get(), 340282366920938463463374607431768211455_u128);

    let b_max_longerunion = small_uint_max_func::<LongerUnion>();
    println!("b_max_longerunion = {}", b_max_longerunion);
    assert_eq!(b_max_longerunion.get(), 340282366920938463463374607431768211455_u128);

    // Example for SizeUnion
    let a_max_sizeunion = SizeUnion::max();
    println!("a_max_sizeunion = {}", a_max_sizeunion);
    assert_eq!(a_max_sizeunion.get(), 18446744073709551615_usize);

    let b_max_sizeunion = small_uint_max_func::<SizeUnion>();
    println!("b_max_sizeunion = {}", b_max_sizeunion);
    assert_eq!(b_max_sizeunion.get(), 18446744073709551615_usize);
    println!("--------------------------------------");
}

fn small_uint_max_func<T: cryptocol::number::SmallUInt>() -> T
{
    T::max()
}

fn small_uint_min()
{
    println!("small_uint_min");
    use cryptocol::number::{ SmallUInt, ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };
    // Example for u8
    let a_min_u8 = u8::MIN; // You are encouraged to use u8::MIN or 0_u8 rather than u8::min().
    println!("a_min_u8 = {}", a_min_u8);
    assert_eq!(a_min_u8, 0_u8);

    let b_min_u8 = small_uint_min_func::<u8>();
    println!("min_u8 = {}", b_min_u8);
    assert_eq!(b_min_u8, 0_u8);

    // Example for u16
    let a_min_u16 = u16::MIN; // You are encouraged to use u16::MIN or 0_u16 rather than u16::min().
    println!("a_min_u16 = {}", a_min_u16);
    assert_eq!(a_min_u16, 0_u16);

    let b_min_u16 = small_uint_min_func::<u16>();
    println!("b_min_u16 = {}", b_min_u16);
    assert_eq!(b_min_u16, 0_u16);

    // Example for u32
    let a_min_u32 = u32::MIN; // You are encouraged to use u32::MIN or 0_u32 rather than u32::min().
    println!("a_min_u32 = {}", a_min_u32);
    assert_eq!(a_min_u32, 0_u32);

    let b_min_u32 = small_uint_min_func::<u32>();
    println!("b_min_u32 = {}", b_min_u32);
    assert_eq!(b_min_u32, 0_u32);

    // Example for u64
    let a_min_u64 = u64::MIN; // You are encouraged to use u64::MIN or 0_u64 rather than u64::min().
    println!("a_min_u64 = {}", a_min_u64);
    assert_eq!(a_min_u64, 0_u64);

    let b_min_u64 = small_uint_min_func::<u64>();
    println!("b_min_u64 = {}", b_min_u64);
    assert_eq!(b_min_u64, 0_u64);

    // Example for u128
    let a_min_u128 = u128::MIN; // You are encouraged to use u128::MIN or 0_u128 rather than u128::min().
    println!("a_min_u128 = {}", a_min_u128);
    assert_eq!(a_min_u128, 0_u128);

    let b_min_u128 = small_uint_min_func::<u128>();
    println!("b_min_u128 = {}", b_min_u128);
    assert_eq!(b_min_u128, 0_u128);

    // Example for usize
    let a_min_usize = usize::MIN; // You are encouraged to use usize::MIN or 0_usize rather than usize::min().
    println!("a_min_usize = {}", a_min_usize);
    assert_eq!(a_min_usize, 0_usize);

    let b_min_usize = small_uint_min_func::<usize>();
    println!("b_min_usize = {}", b_min_usize);
    assert_eq!(b_min_usize, 0_usize);

    // Example for ShortUnion
    let a_min_shortunion = ShortUnion::min();
    println!("a_min_shortunion = {}", a_min_shortunion);
    assert_eq!(a_min_shortunion.get(), 0_u16);

    let b_min_shortunion = small_uint_min_func::<ShortUnion>();
    println!("b_min_shortunion = {}", b_min_shortunion);
    assert_eq!(b_min_shortunion.get(), 0_u16);

    // Example for IntUnion
    let a_min_intunion = IntUnion::min();
    println!("a_min_intunion = {}", a_min_intunion);
    assert_eq!(a_min_intunion.get(), 0_u32);

    let b_min_intunion = small_uint_min_func::<IntUnion>();
    println!("b_min_intunion = {}", b_min_intunion);
    assert_eq!(b_min_intunion.get(), 0_u32);

    // Example for LongUnion
    let a_min_longunion = LongUnion::min();
    println!("a_min_longunion = {}", a_min_longunion);
    assert_eq!(a_min_longunion.get(), 0_u64);

    let b_min_longunion = small_uint_min_func::<LongUnion>();
    println!("b_min_longunion = {}", b_min_longunion);
    assert_eq!(b_min_longunion.get(), 0_u64);

    // Example for LongerUnion
    let a_min_longerunion = LongerUnion::min();
    println!("a_min_longerunion = {}", a_min_longerunion);
    assert_eq!(a_min_longerunion.get(), 0_u128);

    let b_min_longerunion = small_uint_min_func::<LongerUnion>();
    println!("b_min_longerunion = {}", b_min_longerunion);
    assert_eq!(b_min_longerunion.get(), 0_u128);

    // Example for SizeUnion
    let a_min_sizeunion = SizeUnion::min();
    println!("a_min_sizeunion = {}", a_min_sizeunion);
    assert_eq!(a_min_sizeunion.get(), 0_usize);

    let b_min_sizeunion = small_uint_min_func::<SizeUnion>();
    println!("b_min_sizeunion = {}", b_min_sizeunion);
    assert_eq!(b_min_sizeunion.get(), 0_usize);
    println!("--------------------------------------");
}

fn small_uint_min_func<T: cryptocol::number::SmallUInt>() -> T
{
    T::min()
}

fn small_uint_u128_as_smalluint()
{
    println!("small_uint_u128_as_smalluint");
    use cryptocol::number::{ SmallUInt, ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };
    let val_u128 = 123456789012345678901234567890123456789_u128;
    // Example for u8
    let a_u8 = u8::u128_as_smalluint(val_u128);
    println!("{} -> {}", val_u128, a_u8);
    assert_eq!(a_u8, 21_u8);

    let b_u8 = small_uint_u128_as_smalluint_func::<u8>(val_u128);
    println!("{} -> {}", val_u128, b_u8);
    assert_eq!(b_u8, 21_u8);

    // Example for u16
    let a_u16 = u16::u128_as_smalluint(val_u128);
    println!("{} -> {}", val_u128, a_u16);
    assert_eq!(a_u16, 33045_u16);

    let b_u16 = small_uint_u128_as_smalluint_func::<u16>(val_u128);
    println!("{} -> {}", val_u128, b_u16);
    assert_eq!(b_u16, 33045_u16);

    // Example for u32
    let a_u32 = u32::u128_as_smalluint(val_u128);
    println!("{} -> {}", val_u128, a_u32);
    assert_eq!(a_u32, 2923004181_u32);

    let b_u32 = small_uint_u128_as_smalluint_func::<u32>(val_u128);
    println!("{} -> {}", val_u128, b_u32);
    assert_eq!(b_u32, 2923004181_u32);

    // Example for u64
    let a_u64 = u64::u128_as_smalluint(val_u128);
    println!("{} -> {}", val_u128, a_u64);
    assert_eq!(a_u64, 12312739301371248917_u64);

    let b_u64 = small_uint_u128_as_smalluint_func::<u64>(val_u128);
    println!("{} -> {}", val_u128, b_u64);
    assert_eq!(b_u64, 12312739301371248917_u64);

    // Example for u128
    let a_u128 = u128::u128_as_smalluint(val_u128);
    println!("{} -> {}", val_u128, a_u128);
    assert_eq!(a_u128, 123456789012345678901234567890123456789_u128);

    let b_u128 = small_uint_u128_as_smalluint_func::<u128>(val_u128);
    println!("{} -> {}", val_u128, b_u128);
    assert_eq!(b_u128, 123456789012345678901234567890123456789_u128);

    // Example for usize
    let a_usize = usize::u128_as_smalluint(val_u128);
    println!("{} -> {}", val_u128, a_usize);
    assert_eq!(a_usize, 12312739301371248917_usize);

    let b_usize = small_uint_u128_as_smalluint_func::<usize>(val_u128);
    println!("{} -> {}", val_u128, b_usize);
    assert_eq!(b_usize, 12312739301371248917_usize);

    // Example for ShortUnion
    let a_shortunion = ShortUnion::u128_as_smalluint(val_u128);
    println!("{} -> {}", val_u128, a_shortunion);
    assert_eq!(a_shortunion.get(), 33045_u16);

    let b_shortunion = small_uint_u128_as_smalluint_func::<ShortUnion>(val_u128);
    println!("{} -> {}", val_u128, b_shortunion);
    assert_eq!(b_shortunion.get(), 33045_u16);

    // Example for IntUnion
    let a_intunion = IntUnion::u128_as_smalluint(val_u128);
    println!("{} -> {}", val_u128, a_intunion);
    assert_eq!(a_intunion.get(), 2923004181_u32);

    let b_intunion = small_uint_u128_as_smalluint_func::<IntUnion>(val_u128);
    println!("{} -> {}", val_u128, b_intunion);
    assert_eq!(b_intunion.get(), 2923004181_u32);

    // Example for LongUnion
    let a_longunion = LongUnion::u128_as_smalluint(val_u128);
    println!("{} -> {}", val_u128, a_longunion);
    assert_eq!(a_longunion.get(), 12312739301371248917_u64);

    let b_longunion = small_uint_u128_as_smalluint_func::<LongUnion>(val_u128);
    println!("{} -> {}", val_u128, b_longunion);
    assert_eq!(b_longunion.get(), 12312739301371248917_u64);

    // Example for LongerUnion
    let a_longerunion = LongerUnion::u128_as_smalluint(val_u128);
    println!("{} -> {}", val_u128, a_longerunion);
    assert_eq!(a_longerunion.get(), 123456789012345678901234567890123456789_u128);

    let b_longerunion = small_uint_u128_as_smalluint_func::<LongerUnion>(val_u128);
    println!("{} -> {}", val_u128, b_longerunion);
    assert_eq!(b_longerunion.get(), 123456789012345678901234567890123456789_u128);

    // Example for SizeUnion
    let a_sizeunion = SizeUnion::u128_as_smalluint(val_u128);
    println!("{} -> {}", val_u128, a_sizeunion);
    assert_eq!(a_sizeunion.get(), 12312739301371248917_usize);

    let b_sizeunion = small_uint_u128_as_smalluint_func::<SizeUnion>(val_u128);
    println!("{} -> {}", val_u128, b_sizeunion);
    assert_eq!(b_sizeunion.get(), 12312739301371248917_usize);
    println!("--------------------------------------");
}

fn small_uint_u128_as_smalluint_func<T: cryptocol::number::SmallUInt>(n: u128) -> T
{
    T::u128_as_smalluint(n)
}

fn small_uint_u64_as_smalluint()
{
    println!("small_uint_u64_as_smalluint");
    use cryptocol::number::{ SmallUInt, ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };
    let val_u64 = 12312739301371248917_u64;
    // Example for u8
    let a_u8 = u8::u64_as_smalluint(val_u64);
    println!("{} -> {}", val_u64, a_u8);
    assert_eq!(a_u8, 21_u8);

    let b_u8 = small_uint_u64_as_smalluint_func::<u8>(val_u64);
    println!("{} -> {}", val_u64, b_u8);
    assert_eq!(b_u8, 21_u8);

    // Example for u16
    let a_u16 = u16::u64_as_smalluint(val_u64);
    println!("{} -> {}", val_u64, a_u16);
    assert_eq!(a_u16, 33045_u16);

    let b_u16 = small_uint_u64_as_smalluint_func::<u16>(val_u64);
    println!("{} -> {}", val_u64, b_u16);
    assert_eq!(b_u16, 33045_u16);

    // Example for u32
    let a_u32 = u32::u64_as_smalluint(val_u64);
    println!("{} -> {}", val_u64, a_u32);
    assert_eq!(a_u32, 2923004181_u32);

    let b_u32 = small_uint_u64_as_smalluint_func::<u32>(val_u64);
    println!("{} -> {}", val_u64, b_u32);
    assert_eq!(b_u32, 2923004181_u32);

    // Example for u64
    let a_u64 = u64::u64_as_smalluint(val_u64);
    println!("{} -> {}", val_u64, a_u64);
    assert_eq!(a_u64, 12312739301371248917_u64);

    let b_u64 = small_uint_u64_as_smalluint_func::<u64>(val_u64);
    println!("{} -> {}", val_u64, b_u64);
    assert_eq!(b_u64, 12312739301371248917_u64);

    // Example for u128
    let a_u128 = u128::u64_as_smalluint(val_u64);
    println!("{} -> {}", val_u64, a_u128);
    assert_eq!(a_u128, 12312739301371248917_u128);

    let b_u128 = small_uint_u64_as_smalluint_func::<u128>(val_u64);
    println!("{} -> {}", val_u64, b_u128);
    assert_eq!(b_u128, 12312739301371248917_u128);

    // Example for usize
    let a_usize = usize::u64_as_smalluint(val_u64);
    println!("{} -> {}", val_u64, a_usize);
    assert_eq!(a_usize, 12312739301371248917_usize);

    let b_usize = small_uint_u64_as_smalluint_func::<usize>(val_u64);
    println!("{} -> {}", val_u64, b_usize);
    assert_eq!(b_usize, 12312739301371248917_usize);

    // Example for ShortUnion
    let a_shortunion = ShortUnion::u64_as_smalluint(val_u64);
    println!("{} -> {}", val_u64, a_shortunion);
    assert_eq!(a_shortunion.get(), 33045_u16);

    let b_shortunion = small_uint_u64_as_smalluint_func::<ShortUnion>(val_u64);
    println!("{} -> {}", val_u64, b_shortunion);
    assert_eq!(b_shortunion.get(), 33045_u16);

    // Example for IntUnion
    let a_intunion = IntUnion::u64_as_smalluint(val_u64);
    println!("{} -> {}", val_u64, a_intunion);
    assert_eq!(a_intunion.get(), 2923004181_u32);

    let b_intunion = small_uint_u64_as_smalluint_func::<IntUnion>(val_u64);
    println!("{} -> {}", val_u64, b_intunion);
    assert_eq!(b_intunion.get(), 2923004181_u32);

    // Example for LongUnion
    let a_longunion = LongUnion::u64_as_smalluint(val_u64);
    println!("{} -> {}", val_u64, a_longunion);
    assert_eq!(a_longunion.get(), 12312739301371248917_u64);

    let b_longunion = small_uint_u64_as_smalluint_func::<LongUnion>(val_u64);
    println!("{} -> {}", val_u64, b_longunion);
    assert_eq!(b_longunion.get(), 12312739301371248917_u64);

    // Example for LongerUnion
    let a_longerunion = LongerUnion::u64_as_smalluint(val_u64);
    println!("{} -> {}", val_u64, a_longerunion);
    assert_eq!(a_longerunion.get(), 12312739301371248917_u128);

    let b_longerunion = small_uint_u64_as_smalluint_func::<LongerUnion>(val_u64);
    println!("{} -> {}", val_u64, b_longerunion);
    assert_eq!(b_longerunion.get(), 12312739301371248917_u128);

    // Example for SizeUnion
    let a_sizeunion = SizeUnion::u64_as_smalluint(val_u64);
    println!("{} -> {}", val_u64, a_sizeunion);
    assert_eq!(a_sizeunion.get(), 12312739301371248917_usize);

    let b_sizeunion = small_uint_u64_as_smalluint_func::<SizeUnion>(val_u64);
    println!("{} -> {}", val_u64, b_sizeunion);
    assert_eq!(b_sizeunion.get(), 12312739301371248917_usize);
    println!("--------------------------------------");
}

fn small_uint_u64_as_smalluint_func<T: cryptocol::number::SmallUInt>(n: u64) -> T
{
    T::u64_as_smalluint(n)
}

fn small_uint_u32_as_smalluint()
{
    println!("small_uint_u32_as_smalluint");
    use cryptocol::number::{ SmallUInt, ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };
    let val_u32 = 2923004181_u32;
    // Example for u8
    let a_u8 = u8::u32_as_smalluint(val_u32);
    println!("{} -> {}", val_u32, a_u8);
    assert_eq!(a_u8, 21_u8);

    let b_u8 = small_uint_u32_as_smalluint_func::<u8>(val_u32);
    println!("{} -> {}", val_u32, b_u8);
    assert_eq!(b_u8, 21_u8);

    // Example for u16
    let a_u16 = u16::u32_as_smalluint(val_u32);
    println!("{} -> {}", val_u32, a_u16);
    assert_eq!(a_u16, 33045_u16);

    let b_u16 = small_uint_u32_as_smalluint_func::<u16>(val_u32);
    println!("{} -> {}", val_u32, b_u16);
    assert_eq!(b_u16, 33045_u16);

    // Example for u32
    let a_u32 = u32::u32_as_smalluint(val_u32);
    println!("{} -> {}", val_u32, a_u32);
    assert_eq!(a_u32, 2923004181_u32);

    let b_u32 = small_uint_u32_as_smalluint_func::<u32>(val_u32);
    println!("{} -> {}", val_u32, b_u32);
    assert_eq!(b_u32, 2923004181_u32);

    // Example for u64
    let a_u64 = u64::u32_as_smalluint(val_u32);
    println!("{} -> {}", val_u32, a_u64);
    assert_eq!(a_u64, 2923004181_u64);

    let b_u64 = small_uint_u32_as_smalluint_func::<u64>(val_u32);
    println!("{} -> {}", val_u32, b_u64);
    assert_eq!(b_u64, 2923004181_u64);

    // Example for u128
    let a_u128 = u128::u32_as_smalluint(val_u32);
    println!("{} -> {}", val_u32, a_u128);
    assert_eq!(a_u128, 2923004181_u128);

    let b_u128 = small_uint_u32_as_smalluint_func::<u128>(val_u32);
    println!("{} -> {}", val_u32, b_u128);
    assert_eq!(b_u128, 2923004181_u128);

    // Example for usize
    let a_usize = usize::u32_as_smalluint(val_u32);
    println!("{} -> {}", val_u32, a_usize);
    assert_eq!(a_usize, 2923004181_usize);

    let b_usize = small_uint_u32_as_smalluint_func::<usize>(val_u32);
    println!("{} -> {}", val_u32, b_usize);
    assert_eq!(b_usize, 2923004181_usize);

    // Example for ShortUnion
    let a_shortunion = ShortUnion::u32_as_smalluint(val_u32);
    println!("{} -> {}", val_u32, a_shortunion);
    assert_eq!(a_shortunion.get(), 33045_u16);

    let b_shortunion = small_uint_u32_as_smalluint_func::<ShortUnion>(val_u32);
    println!("{} -> {}", val_u32, b_shortunion);
    assert_eq!(b_shortunion.get(), 33045_u16);

    // Example for IntUnion
    let a_intunion = IntUnion::u32_as_smalluint(val_u32);
    println!("{} -> {}", val_u32, a_intunion);
    assert_eq!(a_intunion.get(), 2923004181_u32);

    let b_intunion = small_uint_u32_as_smalluint_func::<IntUnion>(val_u32);
    println!("{} -> {}", val_u32, b_intunion);
    assert_eq!(b_intunion.get(), 2923004181_u32);

    // Example for LongUnion
    let a_longunion = LongUnion::u32_as_smalluint(val_u32);
    println!("{} -> {}", val_u32, a_longunion);
    assert_eq!(a_longunion.get(), 2923004181_u64);

    let b_longunion = small_uint_u32_as_smalluint_func::<LongUnion>(val_u32);
    println!("{} -> {}", val_u32, b_longunion);
    assert_eq!(b_longunion.get(), 2923004181_u64);

    // Example for LongerUnion
    let a_longerunion = LongerUnion::u32_as_smalluint(val_u32);
    println!("{} -> {}", val_u32, a_longerunion);
    assert_eq!(a_longerunion.get(), 2923004181_u128);

    let b_longerunion = small_uint_u32_as_smalluint_func::<LongerUnion>(val_u32);
    println!("{} -> {}", val_u32, b_longerunion);
    assert_eq!(b_longerunion.get(), 2923004181_u128);

    // Example for SizeUnion
    let a_sizeunion = SizeUnion::u32_as_smalluint(val_u32);
    println!("{} -> {}", val_u32, a_sizeunion);
    assert_eq!(a_sizeunion.get(), 2923004181_usize);

    let b_sizeunion = small_uint_u32_as_smalluint_func::<SizeUnion>(val_u32);
    println!("{} -> {}", val_u32, b_sizeunion);
    assert_eq!(b_sizeunion.get(), 2923004181_usize);
    println!("--------------------------------------");
}

fn small_uint_u32_as_smalluint_func<T: cryptocol::number::SmallUInt>(n: u32) -> T
{
    T::u32_as_smalluint(n)
}

fn small_uint_u16_as_smalluint()
{
    println!("small_uint_u16_as_smalluint");
    use cryptocol::number::{ SmallUInt, ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };
    let val_u16 = 33045_u16;
    // Example for u8
    let a_u8 = u8::u16_as_smalluint(val_u16);
    println!("{} -> {}", val_u16, a_u8);
    assert_eq!(a_u8, 21_u8);

    let b_u8 = small_uint_u16_as_smalluint_func::<u8>(val_u16);
    println!("{} -> {}", val_u16, b_u8);
    assert_eq!(b_u8, 21_u8);

    // Example for u16
    let a_u16 = u16::u16_as_smalluint(val_u16);
    println!("{} -> {}", val_u16, a_u16);
    assert_eq!(a_u16, 33045_u16);

    let b_u16 = small_uint_u16_as_smalluint_func::<u16>(val_u16);
    println!("{} -> {}", val_u16, b_u16);
    assert_eq!(b_u16, 33045_u16);

    // Example for u32
    let a_u32 = u32::u16_as_smalluint(val_u16);
    println!("{} -> {}", val_u16, a_u32);
    assert_eq!(a_u32, 33045_u32);

    let b_u32 = small_uint_u16_as_smalluint_func::<u32>(val_u16);
    println!("{} -> {}", val_u16, b_u32);
    assert_eq!(b_u32, 33045_u32);

    // Example for u64
    let a_u64 = u64::u16_as_smalluint(val_u16);
    println!("{} -> {}", val_u16, a_u64);
    assert_eq!(a_u64, 33045_u64);

    let b_u64 = small_uint_u16_as_smalluint_func::<u64>(val_u16);
    println!("{} -> {}", val_u16, b_u64);
    assert_eq!(b_u64, 33045_u64);

    // Example for u128
    let a_u128 = u128::u16_as_smalluint(val_u16);
    println!("{} -> {}", val_u16, a_u128);
    assert_eq!(a_u128, 33045_u128);

    let b_u128 = small_uint_u16_as_smalluint_func::<u128>(val_u16);
    println!("{} -> {}", val_u16, b_u128);
    assert_eq!(b_u128, 33045_u128);

    // Example for usize
    let a_usize = usize::u16_as_smalluint(val_u16);
    println!("{} -> {}", val_u16, a_usize);
    assert_eq!(a_usize, 33045_usize);

    let b_usize = small_uint_u16_as_smalluint_func::<usize>(val_u16);
    println!("{} -> {}", val_u16, b_usize);
    assert_eq!(b_usize, 33045_usize);

    // Example for ShortUnion
    let a_shortunion = ShortUnion::u16_as_smalluint(val_u16);
    println!("{} -> {}", val_u16, a_shortunion);
    assert_eq!(a_shortunion.get(), 33045_u16);

    let b_shortunion = small_uint_u16_as_smalluint_func::<ShortUnion>(val_u16);
    println!("{} -> {}", val_u16, b_shortunion);
    assert_eq!(b_shortunion.get(), 33045_u16);

    // Example for IntUnion
    let a_intunion = IntUnion::u16_as_smalluint(val_u16);
    println!("{} -> {}", val_u16, a_intunion);
    assert_eq!(a_intunion.get(), 33045_u32);

    let b_intunion = small_uint_u16_as_smalluint_func::<IntUnion>(val_u16);
    println!("{} -> {}", val_u16, b_intunion);
    assert_eq!(b_intunion.get(), 33045_u32);

    // Example for LongUnion
    let a_longunion = LongUnion::u16_as_smalluint(val_u16);
    println!("{} -> {}", val_u16, a_longunion);
    assert_eq!(a_longunion.get(), 33045_u64);

    let b_longunion = small_uint_u16_as_smalluint_func::<LongUnion>(val_u16);
    println!("{} -> {}", val_u16, b_longunion);
    assert_eq!(b_longunion.get(), 33045_u64);

    // Example for LongerUnion
    let a_longerunion = LongerUnion::u16_as_smalluint(val_u16);
    println!("{} -> {}", val_u16, a_longerunion);
    assert_eq!(a_longerunion.get(), 33045_u128);

    let b_longerunion = small_uint_u16_as_smalluint_func::<LongerUnion>(val_u16);
    println!("{} -> {}", val_u16, b_longerunion);
    assert_eq!(b_longerunion.get(), 33045_u128);

    // Example for SizeUnion
    let a_sizeunion = SizeUnion::u16_as_smalluint(val_u16);
    println!("{} -> {}", val_u16, a_sizeunion);
    assert_eq!(a_sizeunion.get(), 33045_usize);

    let b_sizeunion = small_uint_u16_as_smalluint_func::<SizeUnion>(val_u16);
    println!("{} -> {}", val_u16, b_sizeunion);
    assert_eq!(b_sizeunion.get(), 33045_usize);
    println!("--------------------------------------");
}

fn small_uint_u16_as_smalluint_func<T: cryptocol::number::SmallUInt>(n: u16) -> T
{
    T::u16_as_smalluint(n)
}

fn small_uint_u8_as_smalluint()
{
    println!("small_uint_u8_as_smalluint");
    use cryptocol::number::{ SmallUInt, ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };
    let val_u8 = 21_u8;
    // Example for u8
    let a_u8 = u8::u8_as_smalluint(val_u8);
    println!("{} -> {}", val_u8, a_u8);
    assert_eq!(a_u8, 21_u8);

    let b_u8 = small_uint_u8_as_smalluint_func::<u8>(val_u8);
    println!("{} -> {}", val_u8, b_u8);
    assert_eq!(b_u8, 21_u8);

    // Example for u16
    let a_u16 = u16::u8_as_smalluint(val_u8);
    println!("{} -> {}", val_u8, a_u16);
    assert_eq!(a_u16, 21_u16);

    let b_u16 = small_uint_u8_as_smalluint_func::<u16>(val_u8);
    println!("{} -> {}", val_u8, b_u16);
    assert_eq!(b_u16, 21_u16);

    // Example for u32
    let a_u32 = u32::u8_as_smalluint(val_u8);
    println!("{} -> {}", val_u8, a_u32);
    assert_eq!(a_u32, 21_u32);

    let b_u32 = small_uint_u8_as_smalluint_func::<u32>(val_u8);
    println!("{} -> {}", val_u8, b_u32);
    assert_eq!(b_u32, 21_u32);

    // Example for u64
    let a_u64 = u64::u8_as_smalluint(val_u8);
    println!("{} -> {}", val_u8, a_u64);
    assert_eq!(a_u64, 21_u64);

    let b_u64 = small_uint_u8_as_smalluint_func::<u64>(val_u8);
    println!("{} -> {}", val_u8, b_u64);
    assert_eq!(b_u64, 21_u64);

    // Example for u128
    let a_u128 = u128::u8_as_smalluint(val_u8);
    println!("{} -> {}", val_u8, a_u128);
    assert_eq!(a_u128, 21_u128);

    let b_u128 = small_uint_u8_as_smalluint_func::<u128>(val_u8);
    println!("{} -> {}", val_u8, b_u128);
    assert_eq!(b_u128, 21_u128);

    // Example for usize
    let a_usize = usize::u8_as_smalluint(val_u8);
    println!("{} -> {}", val_u8, a_usize);
    assert_eq!(a_usize, 21_usize);

    let b_usize = small_uint_u8_as_smalluint_func::<usize>(val_u8);
    println!("{} -> {}", val_u8, b_usize);
    assert_eq!(b_usize, 21_usize);

    // Example for ShortUnion
    let a_shortunion = ShortUnion::u8_as_smalluint(val_u8);
    println!("{} -> {}", val_u8, a_shortunion);
    assert_eq!(a_shortunion.get(), 21_u16);

    let b_shortunion = small_uint_u8_as_smalluint_func::<ShortUnion>(val_u8);
    println!("{} -> {}", val_u8, b_shortunion);
    assert_eq!(b_shortunion.get(), 21_u16);

    // Example for IntUnion
    let a_intunion = IntUnion::u8_as_smalluint(val_u8);
    println!("{} -> {}", val_u8, a_intunion);
    assert_eq!(a_intunion.get(), 21_u32);

    let b_intunion = small_uint_u8_as_smalluint_func::<IntUnion>(val_u8);
    println!("{} -> {}", val_u8, b_intunion);
    assert_eq!(b_intunion.get(), 21_u32);

    // Example for LongUnion
    let a_longunion = LongUnion::u8_as_smalluint(val_u8);
    println!("{} -> {}", val_u8, a_longunion);
    assert_eq!(a_longunion.get(), 21_u64);

    let b_longunion = small_uint_u8_as_smalluint_func::<LongUnion>(val_u8);
    println!("{} -> {}", val_u8, b_longunion);
    assert_eq!(b_longunion.get(), 21_u64);

    // Example for LongerUnion
    let a_longerunion = LongerUnion::u8_as_smalluint(val_u8);
    println!("{} -> {}", val_u8, a_longerunion);
    assert_eq!(a_longerunion.get(), 21_u128);

    let b_longerunion = small_uint_u8_as_smalluint_func::<LongerUnion>(val_u8);
    println!("{} -> {}", val_u8, b_longerunion);
    assert_eq!(b_longerunion.get(), 21_u128);

    // Example for SizeUnion
    let a_sizeunion = SizeUnion::u8_as_smalluint(val_u8);
    println!("{} -> {}", val_u8, a_sizeunion);
    assert_eq!(a_sizeunion.get(), 21_usize);

    let b_sizeunion = small_uint_u8_as_smalluint_func::<SizeUnion>(val_u8);
    println!("{} -> {}", val_u8, b_sizeunion);
    assert_eq!(b_sizeunion.get(), 21_usize);
    println!("--------------------------------------");
}

fn small_uint_u8_as_smalluint_func<T: cryptocol::number::SmallUInt>(n: u8) -> T
{
    T::u8_as_smalluint(n)
}

fn small_uint_usize_as_smalluint()
{
    println!("small_uint_usize_as_smalluint");
    use cryptocol::number::{ SmallUInt, ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };
    let val_usize = 12312739301371248917_usize;
    // Example for u8
    let a_u8 = u8::usize_as_smalluint(val_usize);
    println!("{} -> {}", val_usize, a_u8);
    assert_eq!(a_u8, 21_u8);

    let b_u8 = small_uint_usize_as_smalluint_func::<u8>(val_usize);
    println!("{} -> {}", val_usize, b_u8);
    assert_eq!(b_u8, 21_u8);

    // Example for u16
    let a_u16 = u16::usize_as_smalluint(val_usize);
    println!("{} -> {}", val_usize, a_u16);
    assert_eq!(a_u16, 33045_u16);

    let b_u16 = small_uint_usize_as_smalluint_func::<u16>(val_usize);
    println!("{} -> {}", val_usize, b_u16);
    assert_eq!(b_u16, 33045_u16);

    // Example for u32
    let a_u32 = u32::usize_as_smalluint(val_usize);
    println!("{} -> {}", val_usize, a_u32);
    assert_eq!(a_u32, 2923004181_u32);

    let b_u32 = small_uint_usize_as_smalluint_func::<u32>(val_usize);
    println!("{} -> {}", val_usize, b_u32);
    assert_eq!(b_u32, 2923004181_u32);

    // Example for u64
    let a_u64 = u64::usize_as_smalluint(val_usize);
    println!("{} -> {}", val_usize, a_u64);
    assert_eq!(a_u64, 12312739301371248917_u64);

    let b_u64 = small_uint_usize_as_smalluint_func::<u64>(val_usize);
    println!("{} -> {}", val_usize, b_u64);
    assert_eq!(b_u64, 12312739301371248917_u64);

    // Example for u128
    let a_u128 = u128::usize_as_smalluint(val_usize);
    println!("{} -> {}", val_usize, a_u128);
    assert_eq!(a_u128, 12312739301371248917_u128);

    let b_u128 = small_uint_usize_as_smalluint_func::<u128>(val_usize);
    println!("{} -> {}", val_usize, b_u128);
    assert_eq!(b_u128, 12312739301371248917_u128);

    // Example for usize
    let a_usize = usize::usize_as_smalluint(val_usize);
    println!("{} -> {}", val_usize, a_usize);
    assert_eq!(a_usize, 12312739301371248917_usize);

    let b_usize = small_uint_usize_as_smalluint_func::<usize>(val_usize);
    println!("{} -> {}", val_usize, b_usize);
    assert_eq!(b_usize, 12312739301371248917_usize);

    // Example for ShortUnion
    let a_shortunion = ShortUnion::usize_as_smalluint(val_usize);
    println!("{} -> {}", val_usize, a_shortunion);
    assert_eq!(a_shortunion.get(), 33045_u16);

    let b_shortunion = small_uint_usize_as_smalluint_func::<ShortUnion>(val_usize);
    println!("{} -> {}", val_usize, b_shortunion);
    assert_eq!(b_shortunion.get(), 33045_u16);

    // Example for IntUnion
    let a_intunion = IntUnion::usize_as_smalluint(val_usize);
    println!("{} -> {}", val_usize, a_intunion);
    assert_eq!(a_intunion.get(), 2923004181_u32);

    let b_intunion = small_uint_usize_as_smalluint_func::<IntUnion>(val_usize);
    println!("{} -> {}", val_usize, b_intunion);
    assert_eq!(b_intunion.get(), 2923004181_u32);

    // Example for LongUnion
    let a_longunion = LongUnion::usize_as_smalluint(val_usize);
    println!("{} -> {}", val_usize, a_longunion);
    assert_eq!(a_longunion.get(), 12312739301371248917_u64);

    let b_longunion = small_uint_usize_as_smalluint_func::<LongUnion>(val_usize);
    println!("{} -> {}", val_usize, b_longunion);
    assert_eq!(b_longunion.get(), 12312739301371248917_u64);

    // Example for LongerUnion
    let a_longerunion = LongerUnion::usize_as_smalluint(val_usize);
    println!("{} -> {}", val_usize, a_longerunion);
    assert_eq!(a_longerunion.get(), 12312739301371248917_u128);

    let b_longerunion = small_uint_usize_as_smalluint_func::<LongerUnion>(val_usize);
    println!("{} -> {}", val_usize, b_longerunion);
    assert_eq!(b_longerunion.get(), 12312739301371248917_u128);

    // Example for SizeUnion
    let a_sizeunion = SizeUnion::usize_as_smalluint(val_usize);
    println!("{} -> {}", val_usize, a_sizeunion);
    assert_eq!(a_sizeunion.get(), 12312739301371248917_usize);

    let b_sizeunion = small_uint_usize_as_smalluint_func::<SizeUnion>(val_usize);
    println!("{} -> {}", val_usize, b_sizeunion);
    assert_eq!(b_sizeunion.get(), 12312739301371248917_usize);
    println!("--------------------------------------");
}

fn small_uint_usize_as_smalluint_func<T: cryptocol::number::SmallUInt>(n: usize) -> T
{
    T::usize_as_smalluint(n)
}

fn small_uint_bool_as_smalluint()
{
    println!("small_uint_bool_as_smalluint");
    use cryptocol::number::{ SmallUInt, ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };
    let val1_bool = true;
    let val2_bool = false;
    // Example for u8
    let a_u8 = u8::bool_as_smalluint(val1_bool);
    println!("{} -> {}", val1_bool, a_u8);
    assert_eq!(a_u8, 1_u8);

    let b_u8 = small_uint_bool_as_smalluint_func::<u8>(val1_bool);
    println!("{} -> {}", val1_bool, b_u8);
    assert_eq!(b_u8, 1_u8);

    let c_u8 = u8::bool_as_smalluint(val2_bool);
    println!("{} -> {}", val2_bool, c_u8);
    assert_eq!(c_u8, 0_u8);

    let d_u8 = small_uint_bool_as_smalluint_func::<u8>(val2_bool);
    println!("{} -> {}", val2_bool, d_u8);
    assert_eq!(d_u8, 0_u8);

    // Example for u16
    let a_u16 = u16::bool_as_smalluint(val1_bool);
    println!("{} -> {}", val1_bool, a_u16);
    assert_eq!(a_u16, 1_u16);

    let b_u16 = small_uint_bool_as_smalluint_func::<u16>(val1_bool);
    println!("{} -> {}", val1_bool, b_u16);
    assert_eq!(b_u16, 1_u16);

    let c_u16 = u16::bool_as_smalluint(val2_bool);
    println!("{} -> {}", val2_bool, c_u16);
    assert_eq!(c_u16, 0_u16);

    let d_u16 = small_uint_bool_as_smalluint_func::<u16>(val2_bool);
    println!("{} -> {}", val2_bool, d_u16);
    assert_eq!(d_u16, 0_u16);

    // Example for u32
    let a_u32 = u32::bool_as_smalluint(val1_bool);
    println!("{} -> {}", val1_bool, a_u32);
    assert_eq!(a_u32, 1_u32);

    let b_u32 = small_uint_bool_as_smalluint_func::<u32>(val1_bool);
    println!("{} -> {}", val1_bool, b_u32);
    assert_eq!(b_u32, 1_u32);

    let c_u32 = u32::bool_as_smalluint(val2_bool);
    println!("{} -> {}", val2_bool, c_u32);
    assert_eq!(c_u32, 0_u32);

    let d_u32 = small_uint_bool_as_smalluint_func::<u32>(val2_bool);
    println!("{} -> {}", val2_bool, d_u32);
    assert_eq!(d_u32, 0_u32);

    // Example for u64
    let a_u64 = u64::bool_as_smalluint(val1_bool);
    println!("{} -> {}", val1_bool, a_u64);
    assert_eq!(a_u64, 1_u64);

    let b_u64 = small_uint_bool_as_smalluint_func::<u64>(val1_bool);
    println!("{} -> {}", val1_bool, b_u64);
    assert_eq!(b_u64, 1_u64);
    
    let c_u64 = u64::bool_as_smalluint(val2_bool);
    println!("{} -> {}", val2_bool, c_u64);
    assert_eq!(c_u64, 0_u64);

    let d_u64 = small_uint_bool_as_smalluint_func::<u64>(val2_bool);
    println!("{} -> {}", val2_bool, d_u64);
    assert_eq!(d_u64, 0_u64);

    // Example for u128
    let a_u128 = u128::bool_as_smalluint(val1_bool);
    println!("{} -> {}", val1_bool, a_u128);
    assert_eq!(a_u128, 1_u128);

    let b_u128 = small_uint_bool_as_smalluint_func::<u128>(val1_bool);
    println!("{} -> {}", val1_bool, b_u128);
    assert_eq!(b_u128, 1_u128);

    let c_u128 = u128::bool_as_smalluint(val2_bool);
    println!("{} -> {}", val2_bool, c_u128);
    assert_eq!(c_u128, 0_u128);

    let d_u128 = small_uint_bool_as_smalluint_func::<u128>(val2_bool);
    println!("{} -> {}", val2_bool, d_u128);
    assert_eq!(d_u128, 0_u128);

    // Example for usize
    let a_usize = usize::bool_as_smalluint(val1_bool);
    println!("{} -> {}", val1_bool, a_usize);
    assert_eq!(a_usize, 1_usize);

    let b_usize = small_uint_bool_as_smalluint_func::<usize>(val1_bool);
    println!("{} -> {}", val1_bool, b_usize);
    assert_eq!(b_usize, 1_usize);

    let c_usize = usize::bool_as_smalluint(val2_bool);
    println!("{} -> {}", val2_bool, c_usize);
    assert_eq!(c_usize, 0_usize);

    let d_usize = small_uint_bool_as_smalluint_func::<usize>(val2_bool);
    println!("{} -> {}", val2_bool, d_usize);
    assert_eq!(d_usize, 0_usize);

    // Example for ShortUnion
    let a_shortunion = ShortUnion::bool_as_smalluint(val1_bool);
    println!("{} -> {}", val1_bool, a_shortunion);
    assert_eq!(a_shortunion.get(), 1_u16);

    let b_shortunion = small_uint_bool_as_smalluint_func::<ShortUnion>(val1_bool);
    println!("{} -> {}", val1_bool, b_shortunion);
    assert_eq!(b_shortunion.get(), 1_u16);

    let c_shortunion = ShortUnion::bool_as_smalluint(val2_bool);
    println!("{} -> {}", val2_bool, c_shortunion);
    assert_eq!(c_shortunion.get(), 0_u16);

    let d_shortunion = small_uint_bool_as_smalluint_func::<ShortUnion>(val2_bool);
    println!("{} -> {}", val2_bool, d_shortunion);
    assert_eq!(d_shortunion.get(), 0_u16);

    // Example for IntUnion
    let a_intunion = IntUnion::bool_as_smalluint(val1_bool);
    println!("{} -> {}", val1_bool, a_intunion);
    assert_eq!(a_intunion.get(), 1_u32);

    let b_intunion = small_uint_bool_as_smalluint_func::<IntUnion>(val1_bool);
    println!("{} -> {}", val1_bool, b_intunion);
    assert_eq!(b_intunion.get(), 1_u32);

    let c_intunion = IntUnion::bool_as_smalluint(val2_bool);
    println!("{} -> {}", val2_bool, c_intunion);
    assert_eq!(c_intunion.get(), 0_u32);

    let d_intunion = small_uint_bool_as_smalluint_func::<IntUnion>(val2_bool);
    println!("{} -> {}", val2_bool, d_intunion);
    assert_eq!(d_intunion.get(), 0_u32);

    // Example for LongUnion
    let a_longunion = LongUnion::bool_as_smalluint(val1_bool);
    println!("{} -> {}", val1_bool, a_longunion);
    assert_eq!(a_longunion.get(), 1_u64);

    let b_longunion = small_uint_bool_as_smalluint_func::<LongUnion>(val1_bool);
    println!("{} -> {}", val1_bool, b_longunion);
    assert_eq!(b_longunion.get(), 1_u64);

    let c_longunion = LongUnion::bool_as_smalluint(val2_bool);
    println!("{} -> {}", val2_bool, c_longunion);
    assert_eq!(c_longunion.get(), 0_u64);

    let d_longunion = small_uint_bool_as_smalluint_func::<LongUnion>(val2_bool);
    println!("{} -> {}", val2_bool, d_longunion);
    assert_eq!(d_longunion.get(), 0_u64);

    // Example for LongerUnion
    let a_longerunion = LongerUnion::bool_as_smalluint(val1_bool);
    println!("{} -> {}", val1_bool, a_longerunion);
    assert_eq!(a_longerunion.get(), 1_u128);

    let b_longerunion = small_uint_bool_as_smalluint_func::<LongerUnion>(val1_bool);
    println!("{} -> {}", val1_bool, b_longerunion);
    assert_eq!(b_longerunion.get(), 1_u128);

    let c_longerunion = LongerUnion::bool_as_smalluint(val2_bool);
    println!("{} -> {}", val2_bool, c_longerunion);
    assert_eq!(c_longerunion.get(), 0_u128);

    let d_longerunion = small_uint_bool_as_smalluint_func::<LongerUnion>(val2_bool);
    println!("{} -> {}", val2_bool, d_longerunion);
    assert_eq!(d_longerunion.get(), 0_u128);

    // Example for SizeUnion
    let a_sizeunion = SizeUnion::bool_as_smalluint(val1_bool);
    println!("{} -> {}", val1_bool, a_sizeunion);
    assert_eq!(a_sizeunion.get(), 1_usize);

    let b_sizeunion = small_uint_bool_as_smalluint_func::<SizeUnion>(val1_bool);
    println!("{} -> {}", val1_bool, b_sizeunion);
    assert_eq!(b_sizeunion.get(), 1_usize);

    let c_sizeunion = SizeUnion::bool_as_smalluint(val2_bool);
    println!("{} -> {}", val2_bool, c_sizeunion);
    assert_eq!(c_sizeunion.get(), 0_usize);

    let d_sizeunion = small_uint_bool_as_smalluint_func::<SizeUnion>(val2_bool);
    println!("{} -> {}", val2_bool, d_sizeunion);
    assert_eq!(d_sizeunion.get(), 0_usize);
    println!("--------------------------------------");
}

fn small_uint_bool_as_smalluint_func<T: cryptocol::number::SmallUInt>(n: bool) -> T
{
    T::bool_as_smalluint(n)
}

fn small_uint_num()
{
    println!("small_uint_num");
    use cryptocol::number::{ SmallUInt, ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };
    let val_u8 = 21_u8;
    let val_u16 = 33045_u16;
    let val_u32 = 2923004181_u32;
    let val_u64 = 12312739301371248917_u64;
    let val_u128 = 123456789012345678901234567890123456789_u128;
    let val_usize = 12312739301371248917_usize;

    // Example for u8
    let a_u8 = u8::num(val_u8);
    println!("{} -> {}", val_u8, a_u8);
    assert_eq!(a_u8, 21_u8);

    let b_u8 = small_uint_num_func::<u8, u8>(val_u8);
    println!("{} -> {}", val_u8, b_u8);
    assert_eq!(b_u8, 21_u8);

    let c_u8 = u8::num(val_u16);
    println!("{} -> {}", val_u16, c_u8);
    assert_eq!(c_u8, 21_u8);

    let d_u8 = small_uint_num_func::<u16, u8>(val_u16);
    println!("{} -> {}", val_u16, d_u8);
    assert_eq!(d_u8, 21_u8);

    let e_u8 = u8::num(val_u32);
    println!("{} -> {}", val_u32, e_u8);
    assert_eq!(e_u8, 21_u8);

    let f_u8 = small_uint_num_func::<u32, u8>(val_u32);
    println!("{} -> {}", val_u32, f_u8);
    assert_eq!(f_u8, 21_u8);

    let g_u8 = u8::num(val_u64);
    println!("{} -> {}", val_u64, g_u8);
    assert_eq!(g_u8, 21_u8);

    let h_u8 = small_uint_num_func::<u64, u8>(val_u64);
    println!("{} -> {}", val_u64, h_u8);
    assert_eq!(h_u8, 21_u8);

    let i_u8 = u8::num(val_u128);
    println!("{} -> {}", val_u128, i_u8);
    assert_eq!(i_u8, 21_u8);

    let j_u8 = small_uint_num_func::<u128, u8>(val_u128);
    println!("{} -> {}", val_u128, j_u8);
    assert_eq!(j_u8, 21_u8);

    let k_u8 = u8::num(val_usize);
    println!("{} -> {}", val_usize, k_u8);
    assert_eq!(k_u8, 21_u8);

    let l_u8 = small_uint_num_func::<usize, u8>(val_usize);
    println!("{} -> {}", val_usize, l_u8);
    assert_eq!(l_u8, 21_u8);

    // Example for u16
    let a_u16 = u16::num(val_u8);
    println!("{} -> {}", val_u8, a_u16);
    assert_eq!(a_u16, 21_u16);

    let b_u16 = small_uint_num_func::<u8, u16>(val_u8);
    println!("{} -> {}", val_u8, b_u16);
    assert_eq!(b_u16, 21_u16);

    let c_u16 = u16::num(val_u16);
    println!("{} -> {}", val_u16, c_u16);
    assert_eq!(c_u16, 33045_u16);

    let d_u16 = small_uint_num_func::<u16, u16>(val_u16);
    println!("{} -> {}", val_u16, d_u16);
    assert_eq!(d_u16, 33045_u16);

    let e_u16 = u16::num(val_u32);
    println!("{} -> {}", val_u32, e_u16);
    assert_eq!(e_u16, 33045_u16);

    let f_u16 = small_uint_num_func::<u32, u16>(val_u32);
    println!("{} -> {}", val_u32, f_u16);
    assert_eq!(f_u16, 33045_u16);

    let g_u16 = u16::num(val_u64);
    println!("{} -> {}", val_u64, g_u16);
    assert_eq!(g_u16, 33045_u16);

    let h_u16 = small_uint_num_func::<u64, u16>(val_u64);
    println!("{} -> {}", val_u64, h_u16);
    assert_eq!(h_u16, 33045_u16);

    let i_u16 = u16::num(val_u128);
    println!("{} -> {}", val_u128, i_u16);
    assert_eq!(i_u16, 33045_u16);

    let j_u16 = small_uint_num_func::<u128, u16>(val_u128);
    println!("{} -> {}", val_u128, j_u16);
    assert_eq!(j_u16, 33045_u16);

    let k_u16 = u16::num(val_usize);
    println!("{} -> {}", val_usize, k_u16);
    assert_eq!(k_u16, 33045_u16);

    let l_u16 = small_uint_num_func::<usize, u16>(val_usize);
    println!("{} -> {}", val_usize, l_u16);
    assert_eq!(l_u16, 33045_u16);

    // Example for u32
    let a_u32 = u32::num(val_u8);
    println!("{} -> {}", val_u8, a_u32);
    assert_eq!(a_u32, 21_u32);

    let b_u32 = small_uint_num_func::<u8, u32>(val_u8);
    println!("{} -> {}", val_u8, b_u32);
    assert_eq!(b_u32, 21_u32);

    let c_u32 = u32::num(val_u16);
    println!("{} -> {}", val_u16, c_u32);
    assert_eq!(c_u32, 33045_u32);

    let d_u32 = small_uint_num_func::<u16, u32>(val_u16);
    println!("{} -> {}", val_u32, d_u32);
    assert_eq!(d_u32, 33045_u32);

    let e_u32 = u32::num(val_u32);
    println!("{} -> {}", val_u32, e_u32);
    assert_eq!(e_u32, 2923004181_u32);

    let f_u32 = small_uint_num_func::<u32, u32>(val_u32);
    println!("{} -> {}", val_u32, f_u32);
    assert_eq!(f_u32, 2923004181_u32);

    let g_u32 = u32::num(val_u64);
    println!("{} -> {}", val_u64, g_u32);
    assert_eq!(g_u32, 2923004181_u32);

    let h_u32 = small_uint_num_func::<u64, u32>(val_u64);
    println!("{} -> {}", val_u64, h_u32);
    assert_eq!(h_u32, 2923004181_u32);

    let i_u32 = u32::num(val_u128);
    println!("{} -> {}", val_u128, i_u32);
    assert_eq!(i_u32, 2923004181_u32);

    let j_u32 = small_uint_num_func::<u128, u32>(val_u128);
    println!("{} -> {}", val_u128, j_u32);
    assert_eq!(j_u32, 2923004181_u32);

    let k_u32 = u32::num(val_usize);
    println!("{} -> {}", val_usize, k_u32);
    assert_eq!(k_u32, 2923004181_u32);

    let l_u32 = small_uint_num_func::<usize, u32>(val_usize);
    println!("{} -> {}", val_usize, l_u32);
    assert_eq!(l_u32, 2923004181_u32);

    // Example for u64
    let a_u64 = u64::num(val_u8);
    println!("{} -> {}", val_u8, a_u64);
    assert_eq!(a_u64, 21_u64);

    let b_u64 = small_uint_num_func::<u8, u64>(val_u8);
    println!("{} -> {}", val_u8, b_u64);
    assert_eq!(b_u64, 21_u64);

    let c_u64 = u64::num(val_u16);
    println!("{} -> {}", val_u16, c_u64);
    assert_eq!(c_u64, 33045_u64);

    let d_u64 = small_uint_num_func::<u16, u64>(val_u16);
    println!("{} -> {}", val_u64, d_u64);
    assert_eq!(d_u64, 33045_u64);

    let e_u64 = u64::num(val_u32);
    println!("{} -> {}", val_u64, e_u64);
    assert_eq!(e_u64, 2923004181_u64);

    let f_u64 = small_uint_num_func::<u32, u64>(val_u32);
    println!("{} -> {}", val_u64, f_u64);
    assert_eq!(f_u64, 2923004181_u64);

    let g_u64 = u64::num(val_u64);
    println!("{} -> {}", val_u64, g_u64);
    assert_eq!(g_u64, 12312739301371248917_u64);

    let h_u64 = small_uint_num_func::<u64, u64>(val_u64);
    println!("{} -> {}", val_u64, h_u64);
    assert_eq!(h_u64, 12312739301371248917_u64);

    let i_u64 = u64::num(val_u128);
    println!("{} -> {}", val_u128, i_u64);
    assert_eq!(i_u64, 12312739301371248917_u64);

    let j_u64 = small_uint_num_func::<u128, u64>(val_u128);
    println!("{} -> {}", val_u128, j_u64);
    assert_eq!(j_u64, 12312739301371248917_u64);

    let k_u64 = u64::num(val_usize);
    println!("{} -> {}", val_usize, k_u64);
    assert_eq!(k_u64, 12312739301371248917_u64);

    let l_u64 = small_uint_num_func::<usize, u64>(val_usize);
    println!("{} -> {}", val_usize, l_u64);
    assert_eq!(l_u64, 12312739301371248917_u64);

    // Example for u128
    let a_u128 = u128::num(val_u8);
    println!("{} -> {}", val_u8, a_u128);
    assert_eq!(a_u128, 21_u128);

    let b_u128 = small_uint_num_func::<u8, u128>(val_u8);
    println!("{} -> {}", val_u8, b_u128);
    assert_eq!(b_u128, 21_u128);

    let c_u128 = u128::num(val_u16);
    println!("{} -> {}", val_u16, c_u128);
    assert_eq!(c_u128, 33045_u128);

    let d_u128 = small_uint_num_func::<u16, u128>(val_u16);
    println!("{} -> {}", val_u128, d_u128);
    assert_eq!(d_u128, 33045_u128);

    let e_u128 = u128::num(val_u32);
    println!("{} -> {}", val_u128, e_u128);
    assert_eq!(e_u128, 2923004181_u128);

    let f_u128 = small_uint_num_func::<u32, u128>(val_u32);
    println!("{} -> {}", val_u128, f_u128);
    assert_eq!(f_u128, 2923004181_u128);

    let g_u128 = u128::num(val_u64);
    println!("{} -> {}", val_u128, g_u128);
    assert_eq!(g_u128, 12312739301371248917_u128);

    let h_u128 = small_uint_num_func::<u64, u128>(val_u64);
    println!("{} -> {}", val_u128, h_u128);
    assert_eq!(h_u128, 12312739301371248917_u128);
    
    let i_u128 = u128::num(val_u128);
    println!("{} -> {}", val_u128, i_u128);
    assert_eq!(i_u128, 123456789012345678901234567890123456789_u128);

    let j_u128 = small_uint_num_func::<u128, u128>(val_u128);
    println!("{} -> {}", val_u128, j_u128);
    assert_eq!(j_u128, 123456789012345678901234567890123456789_u128);

    let k_u128 = u128::num(val_usize);
    println!("{} -> {}", val_usize, k_u128);
    assert_eq!(k_u128, 12312739301371248917_u128);

    let l_u128 = small_uint_num_func::<usize, u128>(val_usize);
    println!("{} -> {}", val_usize, l_u128);
    assert_eq!(l_u128, 12312739301371248917_u128);

    // Example for usize
    let a_usize = usize::num(val_u8);
    println!("{} -> {}", val_u8, a_usize);
    assert_eq!(a_usize, 21_usize);

    let b_usize = small_uint_num_func::<u8, usize>(val_u8);
    println!("{} -> {}", val_u8, b_usize);
    assert_eq!(b_usize, 21_usize);

    let c_usize = usize::num(val_u16);
    println!("{} -> {}", val_u16, c_usize);
    assert_eq!(c_usize, 33045_usize);

    let d_usize = small_uint_num_func::<u16, usize>(val_u16);
    println!("{} -> {}", val_usize, d_usize);
    assert_eq!(d_usize, 33045_usize);

    let e_usize = usize::num(val_u32);
    println!("{} -> {}", val_usize, e_usize);
    assert_eq!(e_usize, 2923004181_usize);

    let f_usize = small_uint_num_func::<u32, usize>(val_u32);
    println!("{} -> {}", val_usize, f_usize);
    assert_eq!(f_usize, 2923004181_usize);

    let g_usize = usize::num(val_u64);
    println!("{} -> {}", val_usize, g_usize);
    assert_eq!(g_usize, 12312739301371248917_usize);

    let h_usize = small_uint_num_func::<u64, usize>(val_u64);
    println!("{} -> {}", val_usize, h_usize);
    assert_eq!(h_usize, 12312739301371248917_usize);

    let i_usize = usize::num(val_u128);
    println!("{} -> {}", val_u128, i_usize);
    assert_eq!(i_usize, 12312739301371248917_usize);

    let j_usize = small_uint_num_func::<u128, usize>(val_u128);
    println!("{} -> {}", val_u128, j_usize);
    assert_eq!(j_usize, 12312739301371248917_usize);

    let k_usize = usize::num(val_usize);
    println!("{} -> {}", val_usize, k_usize);
    assert_eq!(k_usize, 12312739301371248917_usize);

    let l_usize = small_uint_num_func::<usize, usize>(val_usize);
    println!("{} -> {}", val_usize, l_usize);
    assert_eq!(l_usize, 12312739301371248917_usize);

    // Example for ShortUnion
    let a_shortunion = ShortUnion::num(val_u8);
    println!("{} -> {}", val_u8, a_shortunion);
    assert_eq!(a_shortunion.get(), 21_u16);

    let b_shortunion = small_uint_num_func::<u8, ShortUnion>(val_u8);
    println!("{} -> {}", val_u8, b_shortunion);
    assert_eq!(b_shortunion.get(), 21_u16);

    let c_shortunion = ShortUnion::num(val_u16);
    println!("{} -> {}", val_u16, c_shortunion);
    assert_eq!(c_shortunion.get(), 33045_u16);

    let d_shortunion = small_uint_num_func::<u16, ShortUnion>(val_u16);
    println!("{} -> {}", val_u16, d_shortunion);
    assert_eq!(d_shortunion.get(), 33045_u16);

    let e_shortunion = ShortUnion::num(val_u32);
    println!("{} -> {}", val_u32, e_shortunion);
    assert_eq!(e_shortunion.get(), 33045_u16);

    let f_shortunion = small_uint_num_func::<u32, ShortUnion>(val_u32);
    println!("{} -> {}", val_u32, f_shortunion);
    assert_eq!(f_shortunion.get(), 33045_u16);

    let g_shortunion = ShortUnion::num(val_u64);
    println!("{} -> {}", val_u64, g_shortunion);
    assert_eq!(g_shortunion.get(), 33045_u16);

    let h_shortunion = small_uint_num_func::<u64, ShortUnion>(val_u64);
    println!("{} -> {}", val_u64, h_shortunion);
    assert_eq!(h_shortunion.get(), 33045_u16);

    let i_shortunion = ShortUnion::num(val_u128);
    println!("{} -> {}", val_u128, i_shortunion);
    assert_eq!(i_shortunion.get(), 33045_u16);

    let j_shortunion = small_uint_num_func::<u128, ShortUnion>(val_u128);
    println!("{} -> {}", val_u128, j_shortunion);
    assert_eq!(j_shortunion.get(), 33045_u16);

    let k_shortunion = ShortUnion::num(val_usize);
    println!("{} -> {}", val_usize, k_shortunion);
    assert_eq!(k_shortunion.get(), 33045_u16);

    let l_shortunion = small_uint_num_func::<usize, ShortUnion>(val_usize);
    println!("{} -> {}", val_usize, l_shortunion);
    assert_eq!(l_shortunion.get(), 33045_u16);

    // Example for IntUnion
    let a_intunion = IntUnion::num(val_u8);
    println!("{} -> {}", val_u8, a_intunion);
    assert_eq!(a_intunion.get(), 21_u32);

    let b_intunion = small_uint_num_func::<u8, IntUnion>(val_u8);
    println!("{} -> {}", val_u8, b_intunion);
    assert_eq!(b_intunion.get(), 21_u32);

    let c_intunion = IntUnion::num(val_u16);
    println!("{} -> {}", val_u16, c_intunion);
    assert_eq!(c_intunion.get(), 33045_u32);

    let d_intunion = small_uint_num_func::<u16, IntUnion>(val_u16);
    println!("{} -> {}", val_u16, d_intunion);
    assert_eq!(d_intunion.get(), 33045_u32);

    let e_intunion = IntUnion::num(val_u32);
    println!("{} -> {}", val_u32, e_intunion);
    assert_eq!(e_intunion.get(), 2923004181_u32);

    let f_intunion = small_uint_num_func::<u32, IntUnion>(val_u32);
    println!("{} -> {}", val_u32, f_intunion);
    assert_eq!(f_intunion.get(), 2923004181_u32);

    let g_intunion = IntUnion::num(val_u64);
    println!("{} -> {}", val_u64, g_intunion);
    assert_eq!(g_intunion.get(), 2923004181_u32);

    let h_intunion = small_uint_num_func::<u64, IntUnion>(val_u64);
    println!("{} -> {}", val_u64, h_intunion);
    assert_eq!(h_intunion.get(), 2923004181_u32);

    let i_intunion = IntUnion::num(val_u128);
    println!("{} -> {}", val_u128, i_intunion);
    assert_eq!(i_intunion.get(), 2923004181_u32);

    let j_intunion = small_uint_num_func::<u128, IntUnion>(val_u128);
    println!("{} -> {}", val_u128, j_intunion);
    assert_eq!(j_intunion.get(), 2923004181_u32);

    let k_intunion = IntUnion::num(val_usize);
    println!("{} -> {}", val_usize, k_intunion);
    assert_eq!(k_intunion.get(), 2923004181_u32);

    let l_intunion = small_uint_num_func::<usize, IntUnion>(val_usize);
    println!("{} -> {}", val_usize, l_intunion);
    assert_eq!(l_intunion.get(), 2923004181_u32);

    // Example for LongUnion
    let a_longunion = LongUnion::num(val_u8);
    println!("{} -> {}", val_u8, a_longunion);
    assert_eq!(a_longunion.get(), 21_u64);

    let b_longunion = small_uint_num_func::<u8, LongUnion>(val_u8);
    println!("{} -> {}", val_u8, b_longunion);
    assert_eq!(b_longunion.get(), 21_u64);

    let c_longunion = LongUnion::num(val_u16);
    println!("{} -> {}", val_u16, c_longunion);
    assert_eq!(c_longunion.get(), 33045_u64);

    let d_longunion = small_uint_num_func::<u16, LongUnion>(val_u16);
    println!("{} -> {}", val_u16, d_longunion);
    assert_eq!(d_longunion.get(), 33045_u64);

    let e_longunion = LongUnion::num(val_u32);
    println!("{} -> {}", val_u32, e_longunion);
    assert_eq!(e_longunion.get(), 2923004181_u64);

    let f_longunion = small_uint_num_func::<u32, LongUnion>(val_u32);
    println!("{} -> {}", val_u32, f_longunion);
    assert_eq!(f_longunion.get(), 2923004181_u64);

    let g_longunion = LongUnion::num(val_u64);
    println!("{} -> {}", val_u64, g_longunion);
    assert_eq!(g_longunion.get(), 12312739301371248917_u64);

    let h_longunion = small_uint_num_func::<u64, LongUnion>(val_u64);
    println!("{} -> {}", val_u64, h_longunion);
    assert_eq!(h_longunion.get(), 12312739301371248917_u64);

    let i_longunion = LongUnion::num(val_u128);
    println!("{} -> {}", val_u128, i_longunion);
    assert_eq!(i_longunion.get(), 12312739301371248917_u64);

    let j_longunion = small_uint_num_func::<u128, LongUnion>(val_u128);
    println!("{} -> {}", val_u128, j_longunion);
    assert_eq!(j_longunion.get(), 12312739301371248917_u64);

    let k_longunion = LongUnion::num(val_usize);
    println!("{} -> {}", val_usize, k_longunion);
    assert_eq!(k_longunion.get(), 12312739301371248917_u64);

    let l_longunion = small_uint_num_func::<usize, LongUnion>(val_usize);
    println!("{} -> {}", val_usize, l_longunion);
    assert_eq!(l_longunion.get(), 12312739301371248917_u64);

    // Example for LongerUnion
    let a_longerunion = LongerUnion::num(val_u8);
    println!("{} -> {}", val_u8, a_longerunion);
    assert_eq!(a_longerunion.get(), 21_u128);

    let b_longerunion = small_uint_num_func::<u8, LongerUnion>(val_u8);
    println!("{} -> {}", val_u8, b_longerunion);
    assert_eq!(b_longerunion.get(), 21_u128);

    let c_longerunion = LongerUnion::num(val_u16);
    println!("{} -> {}", val_u16, c_longerunion);
    assert_eq!(c_longerunion.get(), 33045_u128);

    let d_longerunion = small_uint_num_func::<u16, LongerUnion>(val_u16);
    println!("{} -> {}", val_u16, d_longerunion);
    assert_eq!(d_longerunion.get(), 33045_u128);

    let e_longerunion = LongerUnion::num(val_u32);
    println!("{} -> {}", val_u32, e_longerunion);
    assert_eq!(e_longerunion.get(), 2923004181_u128);

    let f_longerunion = small_uint_num_func::<u32, LongerUnion>(val_u32);
    println!("{} -> {}", val_u32, f_longerunion);
    assert_eq!(f_longerunion.get(), 2923004181_u128);

    let g_longerunion = LongerUnion::num(val_u64);
    println!("{} -> {}", val_u64, g_longerunion);
    assert_eq!(g_longerunion.get(), 12312739301371248917_u128);

    let h_longerunion = small_uint_num_func::<u64, LongerUnion>(val_u64);
    println!("{} -> {}", val_u64, h_longerunion);
    assert_eq!(h_longerunion.get(), 12312739301371248917_u128);

    let i_longerunion = LongerUnion::num(val_u128);
    println!("{} -> {}", val_u128, i_longerunion);
    assert_eq!(i_longerunion.get(), 123456789012345678901234567890123456789_u128);

    let j_longerunion = small_uint_num_func::<u128, LongerUnion>(val_u128);
    println!("{} -> {}", val_u128, j_longerunion);
    assert_eq!(j_longerunion.get(), 123456789012345678901234567890123456789_u128);

    let k_longerunion = LongerUnion::num(val_usize);
    println!("{} -> {}", val_usize, k_longerunion);
    assert_eq!(k_longerunion.get(), 12312739301371248917_u128);

    let l_longerunion = small_uint_num_func::<usize, LongerUnion>(val_usize);
    println!("{} -> {}", val_usize, l_longerunion);
    assert_eq!(l_longerunion.get(), 12312739301371248917_u128);

    // Example for SizeUnion
    let a_sizeunion = SizeUnion::num(val_u8);
    println!("{} -> {}", val_u8, a_sizeunion);
    assert_eq!(a_sizeunion.get(), 21_usize);

    let b_sizeunion = small_uint_num_func::<u8, SizeUnion>(val_u8);
    println!("{} -> {}", val_u8, b_sizeunion);
    assert_eq!(b_sizeunion.get(), 21_usize);

    let c_sizeunion = SizeUnion::num(val_u16);
    println!("{} -> {}", val_u16, c_sizeunion);
    assert_eq!(c_sizeunion.get(), 33045_usize);

    let d_sizeunion = small_uint_num_func::<u16, SizeUnion>(val_u16);
    println!("{} -> {}", val_u16, d_sizeunion);
    assert_eq!(d_sizeunion.get(), 33045_usize);

    let e_sizeunion = SizeUnion::num(val_u32);
    println!("{} -> {}", val_u32, e_sizeunion);
    assert_eq!(e_sizeunion.get(), 2923004181_usize);

    let f_sizeunion = small_uint_num_func::<u32, SizeUnion>(val_u32);
    println!("{} -> {}", val_u32, f_sizeunion);
    assert_eq!(f_sizeunion.get(), 2923004181_usize);

    let g_sizeunion = SizeUnion::num(val_u64);
    println!("{} -> {}", val_u64, g_sizeunion);
    assert_eq!(g_sizeunion.get(), 12312739301371248917_usize);

    let h_sizeunion = small_uint_num_func::<u64, SizeUnion>(val_u64);
    println!("{} -> {}", val_u64, h_sizeunion);
    assert_eq!(h_sizeunion.get(), 12312739301371248917_usize);

    let i_sizeunion = SizeUnion::num(val_u128);
    println!("{} -> {}", val_u128, i_sizeunion);
    assert_eq!(i_sizeunion.get(), 12312739301371248917_usize);

    let j_sizeunion = small_uint_num_func::<u128, SizeUnion>(val_u128);
    println!("{} -> {}", val_u128, j_sizeunion);
    assert_eq!(j_sizeunion.get(), 12312739301371248917_usize);

    let k_sizeunion = SizeUnion::num(val_usize);
    println!("{} -> {}", val_usize, k_sizeunion);
    assert_eq!(k_sizeunion.get(), 12312739301371248917_usize);
    
    let l_sizeunion = small_uint_num_func::<usize, SizeUnion>(val_usize);
    println!("{} -> {}", val_usize, l_sizeunion);
    assert_eq!(l_sizeunion.get(), 12312739301371248917_usize);
    println!("--------------------------------------");
}

fn small_uint_num_func<T: cryptocol::number::SmallUInt, U: cryptocol::number::SmallUInt>(n: T) -> U
{
    U::num(n)
}

fn small_uint_set_zero()
{
    println!("small_uint_set_zero");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let mut a_u8 = 100_u8;
    println!("Originally, a_u8 = {}", a_u8);
    a_u8.set_zero();
    println!("After a_u8.set_zero(), a_u8 = {}", a_u8);
    assert_eq!(a_u8, 0_u8);

    let mut b_u8 = 200_u8;
    println!("Originally, b_u8 = {}", b_u8);
    small_uint_set_zero_func(&mut b_u8);
    println!("After b_u8.set_zero(), b_u8 = {}", b_u8);
    assert_eq!(b_u8, 0_u8);

    // Example for u16
    let mut a_u16 = 10000_u16;
    println!("Originally, a_u16 = {}", a_u16);
    a_u16.set_zero();
    println!("After a_u16.set_zero(), a_u16 = {}", a_u16);
    assert_eq!(a_u16, 0_u16);

    let mut b_u16 = 20000_u16;
    println!("Originally, b_u16 = {}", b_u16);
    small_uint_set_zero_func(&mut b_u16);
    println!("After b_u16.set_zero(), b_u16 = {}", b_u16);
    assert_eq!(b_u16, 0_u16);

    // Example for u32
    let mut a_u32 = 1000000000_u32;
    println!("Originally, a_u32 = {}", a_u32);
    a_u32.set_zero();
    println!("After a_u32.set_zero(), a_u32 = {}", a_u32);
    assert_eq!(a_u32, 0_u32);

    let mut b_u32 = 2000000000_u32;
    println!("Originally, b_u32 = {}", b_u32);
    small_uint_set_zero_func(&mut b_u32);
    println!("After b_u32.set_zero(), b_u32 = {}", b_u32);
    assert_eq!(b_u32, 0_u32);

    // Example for u64
    let mut a_u64 = 10000000000000000000_u64;
    println!("Originally, a_u64 = {}", a_u64);
    a_u64.set_zero();
    println!("After a_u64.set_zero(), a_u64 = {}", a_u64);
    assert_eq!(a_u64, 0_u64);

    let mut b_u64 = 15000000000000000000_u64;
    println!("Originally, b_u64 = {}", b_u64);
    small_uint_set_zero_func(&mut b_u64);
    println!("After b_u64.set_zero(), b_u64 = {}", b_u64);
    assert_eq!(b_u64, 0_u64);

    // Example for u128
    let mut a_u128 = 100000000000000000000000000000000000000_u128;
    println!("Originally, a_u128 = {}", a_u128);
    a_u128.set_zero();
    println!("After a_u128.set_zero(), a_u128 = {}", a_u128);
    assert_eq!(a_u128, 0_u128);

    let mut b_u128 = 200000000000000000000000000000000000000_u128;
    println!("Originally, b_u128 = {}", b_u128);
    small_uint_set_zero_func(&mut b_u128);
    println!("After b_u128.set_zero(), b_u128 = {}", b_u128);
    assert_eq!(b_u128, 0_u128);

    // Example for usize
    let mut a_usize = 10000000000000000000_usize;
    println!("Originally, a_usize = {}", a_usize);
    a_usize.set_zero();
    println!("After a_usize.set_zero(), a_usize = {}", a_usize);
    assert_eq!(a_usize, 0_usize);

    let mut b_usize = 15000000000000000000_usize;
    println!("Originally, b_usize = {}", b_usize);
    small_uint_set_zero_func(&mut b_usize);
    println!("After b_usize.set_zero(), b_usize = {}", b_usize);
    assert_eq!(b_usize, 0_usize);

    // Example for ShortUnion
    let mut a_shortunion = 10000_u16.into_shortunion();
    println!("Originally, a_shortunion = {}", a_shortunion);
    a_shortunion.set_zero();
    println!("After a_shortunion.set_zero(), a_shortunion = {}", a_shortunion);
    assert_eq!(a_shortunion.get(), 0_u16);

    let mut b_shortunion = 20000_u16.into_shortunion();
    println!("Originally, b_shortunion = {}", b_shortunion);
    small_uint_set_zero_func(&mut b_shortunion);
    println!("After b_u16.set_zero(), b_u16 = {}", b_shortunion);
    assert_eq!(b_shortunion.get(), 0_u16);

    // Example for IntUnion
    let mut a_intunion = 1000000000_u32.into_intunion();
    println!("Originally, a_intunion = {}", a_intunion);
    a_intunion.set_zero();
    println!("After a_intunion.set_zero(), a_intunion = {}", a_intunion);
    assert_eq!(a_intunion.get(), 0_u32);

    let mut b_intunion = 2000000000_u32.into_intunion();
    println!("Originally, b_intunion = {}", b_intunion);
    small_uint_set_zero_func(&mut b_intunion);
    println!("After b_intunion.set_zero(), b_intunion = {}", b_intunion);
    assert_eq!(b_intunion.get(), 0_u32);

    // Example for LongUnion
    let mut a_longunion = 10000000000000000000_u64.into_longunion();
    println!("Originally, a_longunion = {}", a_longunion);
    a_longunion.set_zero();
    println!("After a_longunion.set_zero(), a_longunion = {}", a_longunion);
    assert_eq!(a_longunion.get(), 0_u64);

    let mut b_longunion = 15000000000000000000_u64.into_longunion();
    println!("Originally, b_longunion = {}", b_longunion);
    small_uint_set_zero_func(&mut b_longunion);
    println!("After b_longunion.set_zero(), b_longunion = {}", b_longunion);
    assert_eq!(b_longunion.get(), 0_u64);

    // Example for LongerUnion
    let mut a_longerunion = 100000000000000000000000000000000000000_u128.into_longerunion();
    println!("Originally, a_u128 = {}", a_longerunion);
    a_longerunion.set_zero();
    println!("After a_longerunion.set_zero(), a_longerunion = {}", a_longerunion);
    assert_eq!(a_longerunion.get(), 0_u128);

    let mut b_longerunion = 200000000000000000000000000000000000000_u128.into_longerunion();
    println!("Originally, b_longerunion = {}", b_longerunion);
    small_uint_set_zero_func(&mut b_longerunion);
    println!("After b_longerunion.set_zero(), b_longerunion = {}", b_longerunion);
    assert_eq!(b_longerunion.get(), 0_u128);

    // Example for SizeUnion
    let mut a_sizeunion = 10000000000000000000_usize.into_sizeunion();
    println!("Originally, a_sizeunion = {}", a_sizeunion);
    a_sizeunion.set_zero();
    println!("After a_sizeunion.set_zero(), a_sizeunion = {}", a_sizeunion);
    assert_eq!(a_sizeunion.get(), 0_usize);

    let mut b_sizeunion = 15000000000000000000_usize.into_sizeunion();
    println!("Originally, b_sizeunion = {}", b_sizeunion);
    small_uint_set_zero_func(&mut b_sizeunion);
    println!("After b_sizeunion.set_zero(), b_sizeunion = {}", b_sizeunion);
    assert_eq!(b_sizeunion.get(), 0_usize);
    println!("--------------------------------------");
}

fn small_uint_set_zero_func<T: cryptocol::number::SmallUInt>(num: &mut T)
{
    num.set_zero();
}

fn small_uint_is_zero()
{
    println!("small_uint_is_zero");
    use cryptocol::number::SmallUInt;
    const TXT_ZERO: &str = "zero";
    const TXT_NOT_ZERO: &str = "not zero";

    // Example for u8
    let a_u8 = 0_u8;
    let b_u8 = 200_u8;
    let iszero = a_u8.is_zero();
    println!("a_u8 is {}.", if iszero {TXT_ZERO} else {TXT_NOT_ZERO});
    assert_eq!(iszero, true);
    let iszero = small_uint_is_zero_func(b_u8);
    println!("b_u8 is {}.", if iszero {TXT_ZERO} else {TXT_NOT_ZERO});
    assert_eq!(iszero, false);

    // Example for u16
    let a_u16 = 0_u16;
    let b_u16 = 20000_u16;
    let iszero = a_u16.is_zero();
    println!("a_u16 is {}.", if iszero {TXT_ZERO} else {TXT_NOT_ZERO});
    assert_eq!(iszero, true);
    let iszero = small_uint_is_zero_func(b_u16);
    println!("b_u16 is {}.", if iszero {TXT_ZERO} else {TXT_NOT_ZERO});
    assert_eq!(iszero, false);

    // Example for u32
    let a_u32 = 0_u32;
    let b_u32 = 2000000000_u32;
    let iszero = a_u32.is_zero();
    println!("a_u32 is {}.", if iszero {TXT_ZERO} else {TXT_NOT_ZERO});
    assert_eq!(iszero, true);
    let iszero = small_uint_is_zero_func(b_u32);
    println!("b_u32 is {}.", if iszero {TXT_ZERO} else {TXT_NOT_ZERO});
    assert_eq!(iszero, false);

    // Example for u64
    let a_u64 = 0_u64;
    let b_u64 = 15000000000000000000_u64;
    let iszero = a_u64.is_zero();
    println!("a_u64 is {}.", if iszero {TXT_ZERO} else {TXT_NOT_ZERO});
    assert_eq!(iszero, true);
    let iszero = small_uint_is_zero_func(b_u64);
    println!("b_u64 is {}.", if iszero {TXT_ZERO} else {TXT_NOT_ZERO});
    assert_eq!(iszero, false);

    // Example for u128
    let a_u128 = 0_u128;
    let b_u128 = 200000000000000000000000000000000000000_u128;
    let iszero = a_u128.is_zero();
    println!("a_u128 is {}.", if iszero {TXT_ZERO} else {TXT_NOT_ZERO});
    assert_eq!(iszero, true);
    let iszero = small_uint_is_zero_func(b_u128);
    println!("b_u128 is {}.", if iszero {TXT_ZERO} else {TXT_NOT_ZERO});
    assert_eq!(iszero, false);

    // Example for usize
    let a_usize = 0_usize;
    let b_usize = 15000000000000000000_usize;
    let iszero = a_usize.is_zero();
    println!("a_usize is {}.", if iszero {TXT_ZERO} else {TXT_NOT_ZERO});
    assert_eq!(iszero, true);
    let iszero = small_uint_is_zero_func(b_usize);
    println!("b_usize is {}.", if iszero {TXT_ZERO} else {TXT_NOT_ZERO});
    assert_eq!(iszero, false);

    // Example for ShortUnion
    let a_shortunion = 0_u16.into_shortunion();
    let b_shortunion = 20000_u16.into_shortunion();
    let iszero = a_shortunion.is_zero();
    println!("a_shortunion is {}.", if iszero {TXT_ZERO} else {TXT_NOT_ZERO});
    assert_eq!(iszero, true);
    let iszero = small_uint_is_zero_func(b_shortunion);
    println!("b_shortunion is {}.", if iszero {TXT_ZERO} else {TXT_NOT_ZERO});
    assert_eq!(iszero, false);

    // Example for IntUnionn
    let a_intunionn = 0_u32.into_intunion();
    let b_intunionn = 2000000000_u32.into_intunion();
    let iszero = a_intunionn.is_zero();
    println!("a_intunionn is {}.", if iszero {TXT_ZERO} else {TXT_NOT_ZERO});
    assert_eq!(iszero, true);
    let iszero = small_uint_is_zero_func(b_intunionn);
    println!("b_intunionn is {}.", if iszero {TXT_ZERO} else {TXT_NOT_ZERO});
    assert_eq!(iszero, false);

    // Example for LongUnion
    let a_longunion = 0_u64.into_longunion();
    let b_longunion = 15000000000000000000_u64.into_longunion();
    let iszero = a_longunion.is_zero();
    println!("a_longunion is {}.", if iszero {TXT_ZERO} else {TXT_NOT_ZERO});
    assert_eq!(iszero, true);
    let iszero = small_uint_is_zero_func(b_longunion);
    println!("b_longunion is {}.", if iszero {TXT_ZERO} else {TXT_NOT_ZERO});
    assert_eq!(iszero, false);

    // Example for LongerUnion
    let a_longerunion = 0_u128.into_longerunion();
    let b_longerunion = 200000000000000000000000000000000000000_u128.into_longerunion();
    let iszero = a_longerunion.is_zero();
    println!("a_longerunion is {}.", if iszero {TXT_ZERO} else {TXT_NOT_ZERO});
    assert_eq!(iszero, true);
    let iszero = small_uint_is_zero_func(b_longerunion);
    println!("b_longerunion is {}.", if iszero {TXT_ZERO} else {TXT_NOT_ZERO});
    assert_eq!(iszero, false);

    // Example for SizeUnion
    let a_sizeunionn = 0_usize.into_sizeunion();
    let b_sizeunionn = 15000000000000000000_usize.into_sizeunion();
    let iszero = a_sizeunionn.is_zero();
    println!("a_sizeunionn is {}.", if iszero {TXT_ZERO} else {TXT_NOT_ZERO});
    assert_eq!(iszero, true);
    let iszero = small_uint_is_zero_func(b_sizeunionn);
    println!("b_sizeunionn is {}.", if iszero {TXT_ZERO} else {TXT_NOT_ZERO});
    assert_eq!(iszero, false);
    println!("--------------------------------------");
}

fn small_uint_is_zero_func<T: cryptocol::number::SmallUInt>(num: T) -> bool
{
    num.is_zero()
}

fn small_uint_set_one()
{
    println!("small_uint_set_one");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let mut a_u8 = 100_u8;
    println!("Originally, a_u8 = {}", a_u8);
    a_u8.set_one();
    println!("After a_u8.set_one(), a_u8 = {}", a_u8);
    assert_eq!(a_u8, 1_u8);

    let mut b_u8 = 200_u8;
    println!("Originally, b_u8 = {}", b_u8);
    small_uint_set_one_func(&mut b_u8);
    println!("After b_u8.set_one(), b_u8 = {}", b_u8);
    assert_eq!(b_u8, 1_u8);

    // Example for u16
    let mut a_u16 = 10000_u16;
    println!("Originally, a_u16 = {}", a_u16);
    a_u16.set_one();
    println!("After a_u16.set_one(), a_u16 = {}", a_u16);
    assert_eq!(a_u16, 1_u16);

    let mut b_u16 = 20000_u16;
    println!("Originally, b_u16 = {}", b_u16);
    small_uint_set_one_func(&mut b_u16);
    println!("After b_u16.set_one(), b_u16 = {}", b_u16);
    assert_eq!(b_u16, 1_u16);

    // Example for u32
    let mut a_u32 = 1000000000_u32;
    println!("Originally, a_u32 = {}", a_u32);
    a_u32.set_one();
    println!("After a_u32.set_one(), a_u32 = {}", a_u32);
    assert_eq!(a_u32, 1_u32);

    let mut b_u32 = 2000000000_u32;
    println!("Originally, b_u32 = {}", b_u32);
    small_uint_set_one_func(&mut b_u32);
    println!("After b_u32.set_one(), b_u32 = {}", b_u32);
    assert_eq!(b_u32, 1_u32);

    // Example for u64
    let mut a_u64 = 10000000000000000000_u64;
    println!("Originally, a_u64 = {}", a_u64);
    a_u64.set_one();
    println!("After a_u64.set_one(), a_u64 = {}", a_u64);
    assert_eq!(a_u64, 1_u64);

    let mut b_u64 = 15000000000000000000_u64;
    println!("Originally, b_u64 = {}", b_u64);
    small_uint_set_one_func(&mut b_u64);
    println!("After b_u64.set_one(), b_u64 = {}", b_u64);
    assert_eq!(b_u64, 1_u64);

    // Example for u128
    let mut a_u128 = 100000000000000000000000000000000000000_u128;
    println!("Originally, a_u128 = {}", a_u128);
    a_u128.set_one();
    println!("After a_u128.set_one(), a_u128 = {}", a_u128);
    assert_eq!(a_u128, 1_u128);

    let mut b_u128 = 200000000000000000000000000000000000000_u128;
    println!("Originally, b_u128 = {}", b_u128);
    small_uint_set_one_func(&mut b_u128);
    println!("After b_u128.set_one(), b_u128 = {}", b_u128);
    assert_eq!(b_u128, 1_u128);

    // Example for usize
    let mut a_usize = 10000000000000000000_usize;
    println!("Originally, a_usize = {}", a_usize);
    a_usize.set_one();
    println!("After a_usize.set_one(), a_usize = {}", a_usize);
    assert_eq!(a_usize, 1_usize);

    let mut b_usize = 15000000000000000000_usize;
    println!("Originally, b_usize = {}", b_usize);
    small_uint_set_one_func(&mut b_usize);
    println!("After b_usize.set_one(), b_usize = {}", b_usize);
    assert_eq!(b_usize, 1_usize);

    // Example for ShortUnion
    let mut a_shortunion = 10000_u16.into_shortunion();
    println!("Originally, a_shortunion = {}", a_shortunion);
    a_shortunion.set_one();
    println!("After a_shortunion.set_one(), a_shortunion = {}", a_shortunion);
    assert_eq!(a_shortunion.get(), 1_u16);

    let mut b_shortunion = 20000_u16.into_shortunion();
    println!("Originally, b_shortunion = {}", b_shortunion);
    small_uint_set_one_func(&mut b_shortunion);
    println!("After b_u16.set_one(), b_u16 = {}", b_shortunion);
    assert_eq!(b_shortunion.get(), 1_u16);

    // Example for IntUnion
    let mut a_intunion = 1000000000_u32.into_intunion();
    println!("Originally, a_intunion = {}", a_intunion);
    a_intunion.set_one();
    println!("After a_intunion.set_one(), a_intunion = {}", a_intunion);
    assert_eq!(a_intunion.get(), 1_u32);

    let mut b_intunion = 2000000000_u32.into_intunion();
    println!("Originally, b_intunion = {}", b_intunion);
    small_uint_set_one_func(&mut b_intunion);
    println!("After b_intunion.set_one(), b_intunion = {}", b_intunion);
    assert_eq!(b_intunion.get(), 1_u32);

    // Example for LongUnion
    let mut a_longunion = 10000000000000000000_u64.into_longunion();
    println!("Originally, a_longunion = {}", a_longunion);
    a_longunion.set_one();
    println!("After a_longunion.set_one(), a_longunion = {}", a_longunion);
    assert_eq!(a_longunion.get(), 1_u64);

    let mut b_longunion = 15000000000000000000_u64.into_longunion();
    println!("Originally, b_longunion = {}", b_longunion);
    small_uint_set_one_func(&mut b_longunion);
    println!("After b_longunion.set_one(), b_longunion = {}", b_longunion);
    assert_eq!(b_longunion.get(), 1_u64);

    // Example for LongerUnion
    let mut a_longerunion = 100000000000000000000000000000000000000_u128.into_longerunion();
    println!("Originally, a_u128 = {}", a_longerunion);
    a_longerunion.set_one();
    println!("After a_longerunion.set_one(), a_longerunion = {}", a_longerunion);
    assert_eq!(a_longerunion.get(), 1_u128);

    let mut b_longerunion = 200000000000000000000000000000000000000_u128.into_longerunion();
    println!("Originally, b_longerunion = {}", b_longerunion);
    small_uint_set_one_func(&mut b_longerunion);
    println!("After b_longerunion.set_one(), b_longerunion = {}", b_longerunion);
    assert_eq!(b_longerunion.get(), 1_u128);

    // Example for SizeUnion
    let mut a_sizeunion = 10000000000000000000_usize.into_sizeunion();
    println!("Originally, a_sizeunion = {}", a_sizeunion);
    a_sizeunion.set_one();
    println!("After a_sizeunion.set_one(), a_sizeunion = {}", a_sizeunion);
    assert_eq!(a_sizeunion.get(), 1_usize);

    let mut b_sizeunion = 15000000000000000000_usize.into_sizeunion();
    println!("Originally, b_sizeunion = {}", b_sizeunion);
    small_uint_set_one_func(&mut b_sizeunion);
    println!("After b_sizeunion.set_one(), b_sizeunion = {}", b_sizeunion);
    assert_eq!(b_sizeunion.get(), 1_usize);
    println!("--------------------------------------");
}

fn small_uint_set_one_func<T: cryptocol::number::SmallUInt>(num: &mut T)
{
    num.set_one();
}

fn small_uint_is_one()
{
    println!("small_uint_is_one");
    use cryptocol::number::SmallUInt;
    const TXT_ONE: &str = "one";
    const TXT_NOT_ONE: &str = "not one";

    // Example for u8
    let a_u8 = 1_u8;
    let b_u8 = 200_u8;
    let isone = a_u8.is_one();
    println!("a_u8 is {}.", if isone {TXT_ONE} else {TXT_NOT_ONE});
    assert_eq!(isone, true);
    let isone = small_uint_is_one_func(b_u8);
    println!("b_u8 is {}.", if isone {TXT_ONE} else {TXT_NOT_ONE});
    assert_eq!(isone, false);

    // Example for u16
    let a_u16 = 1_u16;
    let b_u16 = 20000_u16;
    let isone = a_u16.is_one();
    println!("a_u16 is {}.", if isone {TXT_ONE} else {TXT_NOT_ONE});
    assert_eq!(isone, true);
    let isone = small_uint_is_one_func(b_u16);
    println!("b_u16 is {}.", if isone {TXT_ONE} else {TXT_NOT_ONE});
    assert_eq!(isone, false);

    // Example for u32
    let a_u32 = 1_u32;
    let b_u32 = 2000000000_u32;
    let isone = a_u32.is_one();
    println!("a_u32 is {}.", if isone {TXT_ONE} else {TXT_NOT_ONE});
    assert_eq!(isone, true);
    let isone = small_uint_is_one_func(b_u32);
    println!("b_u32 is {}.", if isone {TXT_ONE} else {TXT_NOT_ONE});
    assert_eq!(isone, false);

    // Example for u64
    let a_u64 = 1_u64;
    let b_u64 = 15000000000000000000_u64;
    let isone = a_u64.is_one();
    println!("a_u64 is {}.", if isone {TXT_ONE} else {TXT_NOT_ONE});
    assert_eq!(isone, true);
    let isone = small_uint_is_one_func(b_u64);
    println!("b_u64 is {}.", if isone {TXT_ONE} else {TXT_NOT_ONE});
    assert_eq!(isone, false);

    // Example for u128
    let a_u128 = 1_u128;
    let b_u128 = 200000000000000000000000000000000000000_u128;
    let isone = a_u128.is_one();
    println!("a_u128 is {}.", if isone {TXT_ONE} else {TXT_NOT_ONE});
    assert_eq!(isone, true);
    let isone = small_uint_is_one_func(b_u128);
    println!("b_u128 is {}.", if isone {TXT_ONE} else {TXT_NOT_ONE});
    assert_eq!(isone, false);

    // Example for usize
    let a_usize = 1_usize;
    let b_usize = 15000000000000000000_usize;
    let isone = a_usize.is_one();
    println!("a_usize is {}.", if isone {TXT_ONE} else {TXT_NOT_ONE});
    assert_eq!(isone, true);
    let isone = small_uint_is_one_func(b_usize);
    println!("b_usize is {}.", if isone {TXT_ONE} else {TXT_NOT_ONE});
    assert_eq!(isone, false);

    // Example for ShortUnion
    let a_shortunion = 1_u16.into_shortunion();
    let b_shortunion = 20000_u16.into_shortunion();
    let isone = a_shortunion.is_one();
    println!("a_shortunion is {}.", if isone {TXT_ONE} else {TXT_NOT_ONE});
    assert_eq!(isone, true);
    let isone = small_uint_is_one_func(b_shortunion);
    println!("b_shortunion is {}.", if isone {TXT_ONE} else {TXT_NOT_ONE});
    assert_eq!(isone, false);

    // Example for IntUnionn
    let a_intunionn = 1_u32.into_intunion();
    let b_intunionn = 2000000000_u32.into_intunion();
    let isone = a_intunionn.is_one();
    println!("a_intunionn is {}.", if isone {TXT_ONE} else {TXT_NOT_ONE});
    assert_eq!(isone, true);
    let isone = small_uint_is_one_func(b_intunionn);
    println!("b_intunionn is {}.", if isone {TXT_ONE} else {TXT_NOT_ONE});
    assert_eq!(isone, false);

    // Example for LongUnion
    let a_longunion = 1_u64.into_longunion();
    let b_longunion = 15000000000000000000_u64.into_longunion();
    let isone = a_longunion.is_one();
    println!("a_longunion is {}.", if isone {TXT_ONE} else {TXT_NOT_ONE});
    assert_eq!(isone, true);
    let isone = small_uint_is_one_func(b_longunion);
    println!("b_longunion is {}.", if isone {TXT_ONE} else {TXT_NOT_ONE});
    assert_eq!(isone, false);

    // Example for LongerUnion
    let a_longerunion = 1_u128.into_longerunion();
    let b_longerunion = 200000000000000000000000000000000000000_u128.into_longerunion();
    let isone = a_longerunion.is_one();
    println!("a_longerunion is {}.", if isone {TXT_ONE} else {TXT_NOT_ONE});
    assert_eq!(isone, true);
    let isone = small_uint_is_one_func(b_longerunion);
    println!("b_longerunion is {}.", if isone {TXT_ONE} else {TXT_NOT_ONE});
    assert_eq!(isone, false);

    // Example for SizeUnion
    let a_sizeunionn = 1_usize.into_sizeunion();
    let b_sizeunionn = 15000000000000000000_usize.into_sizeunion();
    let isone = a_sizeunionn.is_one();
    println!("a_sizeunionn is {}.", if isone {TXT_ONE} else {TXT_NOT_ONE});
    assert_eq!(isone, true);
    let isone = small_uint_is_one_func(b_sizeunionn);
    println!("b_sizeunionn is {}.", if isone {TXT_ONE} else {TXT_NOT_ONE});
    assert_eq!(isone, false);
    println!("--------------------------------------");
}

fn small_uint_is_one_func<T: cryptocol::number::SmallUInt>(num: T) -> bool
{
    num.is_one()
}

fn small_uint_is_zero_or_one()
{
    println!("small_uint_is_zero_or_one");
    use cryptocol::number::SmallUInt;
    const TXT_ZERO_OR_ONE: &str = "either zero or one";
    const TXT_NETHER_ZERO_NOR_ONE: &str = "neither zero nor one";

    // Example for u8
    let a_u8 = 0_u8;
    let b_u8 = 1_u8;
    let c_u8 = 200_u8;
    let zero_one = a_u8.is_zero_or_one();
    println!("a_u8 is {}.", if zero_one {TXT_ZERO_OR_ONE} else {TXT_NETHER_ZERO_NOR_ONE});
    assert_eq!(zero_one, true);
    let zero_one = small_uint_is_zero_or_one_func(b_u8);
    println!("b_u8 is {}.", if zero_one {TXT_ZERO_OR_ONE} else {TXT_NETHER_ZERO_NOR_ONE});
    assert_eq!(zero_one, true);
    let zero_one = c_u8.is_zero_or_one();
    println!("c_u8 is {}.", if zero_one {TXT_ZERO_OR_ONE} else {TXT_NETHER_ZERO_NOR_ONE});
    assert_eq!(zero_one, false);

    // Example for u16
    let a_u16 = 0_u16;
    let b_u16 = 1_u16;
    let c_u16 = 20000_u16;
    let zero_one = a_u16.is_zero_or_one();
    println!("a_u16 is {}.", if zero_one {TXT_ZERO_OR_ONE} else {TXT_NETHER_ZERO_NOR_ONE});
    assert_eq!(zero_one, true);
    let zero_one = small_uint_is_zero_or_one_func(b_u16);
    println!("b_u16 is {}.", if zero_one {TXT_ZERO_OR_ONE} else {TXT_NETHER_ZERO_NOR_ONE});
    assert_eq!(zero_one, true);
    let zero_one = c_u16.is_zero_or_one();
    println!("c_u16 is {}.", if zero_one {TXT_ZERO_OR_ONE} else {TXT_NETHER_ZERO_NOR_ONE});
    assert_eq!(zero_one, false);

    // Example for u32
    let a_u32 = 0_u32;
    let b_u32 = 1_u32;
    let c_u32 = 2000000000_u32;
    let zero_one = a_u32.is_zero_or_one();
    println!("a_u32 is {}.", if zero_one {TXT_ZERO_OR_ONE} else {TXT_NETHER_ZERO_NOR_ONE});
    assert_eq!(zero_one, true);
    let zero_one = small_uint_is_zero_or_one_func(b_u32);
    println!("b_u32 is {}.", if zero_one {TXT_ZERO_OR_ONE} else {TXT_NETHER_ZERO_NOR_ONE});
    assert_eq!(zero_one, true);
    let zero_one = c_u32.is_zero_or_one();
    println!("c_u32 is {}.", if zero_one {TXT_ZERO_OR_ONE} else {TXT_NETHER_ZERO_NOR_ONE});
    assert_eq!(zero_one, false);

    // Example for u64
    let a_u64 = 0_u64;
    let b_u64 = 1_u64;
    let c_u64 = 15000000000000000000_u64;
    let zero_one = a_u64.is_zero_or_one();
    println!("a_u64 is {}.", if zero_one {TXT_ZERO_OR_ONE} else {TXT_NETHER_ZERO_NOR_ONE});
    assert_eq!(zero_one, true);
    let zero_one = small_uint_is_zero_or_one_func(b_u64);
    println!("b_u64 is {}.", if zero_one {TXT_ZERO_OR_ONE} else {TXT_NETHER_ZERO_NOR_ONE});
    assert_eq!(zero_one, true);
    let zero_one = c_u64.is_zero_or_one();
    println!("c_u64 is {}.", if zero_one {TXT_ZERO_OR_ONE} else {TXT_NETHER_ZERO_NOR_ONE});
    assert_eq!(zero_one, false);

    // Example for u128
    let a_u128 = 0_u128;
    let b_u128 = 1_u128;
    let c_u128 = 200000000000000000000000000000000000000_u128;
    let zero_one = a_u128.is_zero_or_one();
    println!("a_u128 is {}.", if zero_one {TXT_ZERO_OR_ONE} else {TXT_NETHER_ZERO_NOR_ONE});
    assert_eq!(zero_one, true);
    let zero_one = small_uint_is_zero_or_one_func(b_u128);
    println!("b_u128 is {}.", if zero_one {TXT_ZERO_OR_ONE} else {TXT_NETHER_ZERO_NOR_ONE});
    assert_eq!(zero_one, true);
    let zero_one = c_u128.is_zero_or_one();
    println!("c_u128 is {}.", if zero_one {TXT_ZERO_OR_ONE} else {TXT_NETHER_ZERO_NOR_ONE});
    assert_eq!(zero_one, false);

    // Example for usize
    let a_usize = 0_usize;
    let b_usize = 1_usize;
    let c_usize = 15000000000000000000_usize;
    let zero_one = a_usize.is_zero_or_one();
    println!("a_usize is {}.", if zero_one {TXT_ZERO_OR_ONE} else {TXT_NETHER_ZERO_NOR_ONE});
    assert_eq!(zero_one, true);
    let zero_one = small_uint_is_zero_or_one_func(b_usize);
    println!("b_usize is {}.", if zero_one {TXT_ZERO_OR_ONE} else {TXT_NETHER_ZERO_NOR_ONE});
    assert_eq!(zero_one, true);
    let zero_one = c_usize.is_zero_or_one();
    println!("c_usize is {}.", if zero_one {TXT_ZERO_OR_ONE} else {TXT_NETHER_ZERO_NOR_ONE});
    assert_eq!(zero_one, false);

    // Example for ShortUnion
    let a_shortunion = 0_u16.into_shortunion();
    let b_shortunion = 1_u16.into_shortunion();
    let c_shortunion = 20000_u16.into_shortunion();
    let zero_one = a_shortunion.is_zero_or_one();
    println!("a_shortunion is {}.", if zero_one {TXT_ZERO_OR_ONE} else {TXT_NETHER_ZERO_NOR_ONE});
    assert_eq!(zero_one, true);
    let zero_one = small_uint_is_zero_or_one_func(b_shortunion);
    println!("b_shortunion is {}.", if zero_one {TXT_ZERO_OR_ONE} else {TXT_NETHER_ZERO_NOR_ONE});
    assert_eq!(zero_one, true);
    let zero_one = c_shortunion.is_zero_or_one();
    println!("b_shortunion is {}.", if zero_one {TXT_ZERO_OR_ONE} else {TXT_NETHER_ZERO_NOR_ONE});
    assert_eq!(zero_one, false);

    // Example for IntUnionn
    let a_intunionn = 0_u32.into_intunion();
    let b_intunionn = 1_u32.into_intunion();
    let c_intunionn = 2000000000_u32.into_intunion();
    let zero_one = a_intunionn.is_zero_or_one();
    println!("a_intunionn is {}.", if zero_one {TXT_ZERO_OR_ONE} else {TXT_NETHER_ZERO_NOR_ONE});
    assert_eq!(zero_one, true);
    let zero_one = small_uint_is_zero_or_one_func(b_intunionn);
    println!("b_intunionn is {}.", if zero_one {TXT_ZERO_OR_ONE} else {TXT_NETHER_ZERO_NOR_ONE});
    assert_eq!(zero_one, true);
    let zero_one = c_intunionn.is_zero_or_one();
    println!("c_intunionn is {}.", if zero_one {TXT_ZERO_OR_ONE} else {TXT_NETHER_ZERO_NOR_ONE});
    assert_eq!(zero_one, false);

    // Example for LongUnion
    let a_longunion = 0_u64.into_longunion();
    let b_longunion = 1_u64.into_longunion();
    let c_longunion = 15000000000000000000_u64.into_longunion();
    let zero_one = a_longunion.is_zero_or_one();
    println!("a_longunion is {}.", if zero_one {TXT_ZERO_OR_ONE} else {TXT_NETHER_ZERO_NOR_ONE});
    assert_eq!(zero_one, true);
    let zero_one = small_uint_is_zero_or_one_func(b_longunion);
    println!("b_longunion is {}.", if zero_one {TXT_ZERO_OR_ONE} else {TXT_NETHER_ZERO_NOR_ONE});
    assert_eq!(zero_one, true);
    let zero_one = c_longunion.is_zero_or_one();
    println!("c_longunion is {}.", if zero_one {TXT_ZERO_OR_ONE} else {TXT_NETHER_ZERO_NOR_ONE});
    assert_eq!(zero_one, false);

    // Example for LongerUnion
    let a_longerunion = 0_u128.into_longerunion();
    let b_longerunion = 1_u128.into_longerunion();
    let c_longerunion = 200000000000000000000000000000000000000_u128.into_longerunion();
    let zero_one = a_longerunion.is_zero_or_one();
    println!("a_longerunion is {}.", if zero_one {TXT_ZERO_OR_ONE} else {TXT_NETHER_ZERO_NOR_ONE});
    assert_eq!(zero_one, true);
    let zero_one = small_uint_is_zero_or_one_func(b_longerunion);
    println!("b_longerunion is {}.", if zero_one {TXT_ZERO_OR_ONE} else {TXT_NETHER_ZERO_NOR_ONE});
    assert_eq!(zero_one, true);
    let zero_one = c_longerunion.is_zero_or_one();
    println!("c_longerunion is {}.", if zero_one {TXT_ZERO_OR_ONE} else {TXT_NETHER_ZERO_NOR_ONE});
    assert_eq!(zero_one, false);

    // Example for SizeUnion
    let a_sizeunionn = 0_usize.into_sizeunion();
    let b_sizeunionn = 1_usize.into_sizeunion();
    let c_sizeunionn = 15000000000000000000_usize.into_sizeunion();
    let zero_one = a_sizeunionn.is_zero_or_one();
    println!("a_sizeunionn is {}.", if zero_one {TXT_ZERO_OR_ONE} else {TXT_NETHER_ZERO_NOR_ONE});
    assert_eq!(zero_one, true);
    let zero_one = small_uint_is_zero_or_one_func(b_sizeunionn);
    println!("b_sizeunionn is {}.", if zero_one {TXT_ZERO_OR_ONE} else {TXT_NETHER_ZERO_NOR_ONE});
    assert_eq!(zero_one, true);
    let zero_one = c_sizeunionn.is_zero_or_one();
    println!("c_sizeunionn is {}.", if zero_one {TXT_ZERO_OR_ONE} else {TXT_NETHER_ZERO_NOR_ONE});
    assert_eq!(zero_one, false);
    println!("--------------------------------------");
}

fn small_uint_is_zero_or_one_func<T: cryptocol::number::SmallUInt>(num: T) -> bool
{
    num.is_zero_or_one()
}

fn small_uint_set_max()
{
    println!("small_uint_set_max");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let mut a_u8 = 100_u8;
    println!("Originally, a_u8 = {}", a_u8);
    a_u8.set_max();
    println!("After a_u8.set_max(), a_u8 = {}", a_u8);
    assert_eq!(a_u8, u8::MAX);

    let mut b_u8 = 200_u8;
    println!("Originally, b_u8 = {}", b_u8);
    small_uint_set_max_func(&mut b_u8);
    println!("After b_u8.set_max(), b_u8 = {}", b_u8);
    assert_eq!(b_u8, u8::MAX);

    // Example for u16
    let mut a_u16 = 10000_u16;
    println!("Originally, a_u16 = {}", a_u16);
    a_u16.set_max();
    println!("After a_u16.set_max(), a_u16 = {}", a_u16);
    assert_eq!(a_u16, u16::MAX);

    let mut b_u16 = 20000_u16;
    println!("Originally, b_u16 = {}", b_u16);
    small_uint_set_max_func(&mut b_u16);
    println!("After b_u16.set_max(), b_u16 = {}", b_u16);
    assert_eq!(b_u16, u16::MAX);

    // Example for u32
    let mut a_u32 = 1000000000_u32;
    println!("Originally, a_u32 = {}", a_u32);
    a_u32.set_max();
    println!("After a_u32.set_max(), a_u32 = {}", a_u32);
    assert_eq!(a_u32, u32::MAX);

    let mut b_u32 = 2000000000_u32;
    println!("Originally, b_u32 = {}", b_u32);
    small_uint_set_max_func(&mut b_u32);
    println!("After b_u32.set_max(), b_u32 = {}", b_u32);
    assert_eq!(b_u32, u32::MAX);

    // Example for u64
    let mut a_u64 = 10000000000000000000_u64;
    println!("Originally, a_u64 = {}", a_u64);
    a_u64.set_max();
    println!("After a_u64.set_max(), a_u64 = {}", a_u64);
    assert_eq!(a_u64, u64::MAX);

    let mut b_u64 = 15000000000000000000_u64;
    println!("Originally, b_u64 = {}", b_u64);
    small_uint_set_max_func(&mut b_u64);
    println!("After b_u64.set_max(), b_u64 = {}", b_u64);
    assert_eq!(b_u64, u64::MAX);

    // Example for u128
    let mut a_u128 = 100000000000000000000000000000000000000_u128;
    println!("Originally, a_u128 = {}", a_u128);
    a_u128.set_max();
    println!("After a_u128.set_max(), a_u128 = {}", a_u128);
    assert_eq!(a_u128, u128::MAX);

    let mut b_u128 = 200000000000000000000000000000000000000_u128;
    println!("Originally, b_u128 = {}", b_u128);
    small_uint_set_max_func(&mut b_u128);
    println!("After b_u128.set_max(), b_u128 = {}", b_u128);
    assert_eq!(b_u128, u128::MAX);

    // Example for usize
    let mut a_usize = 10000000000000000000_usize;
    println!("Originally, a_usize = {}", a_usize);
    a_usize.set_max();
    println!("After a_usize.set_max(), a_usize = {}", a_usize);
    assert_eq!(a_usize, usize::MAX);

    let mut b_usize = 15000000000000000000_usize;
    println!("Originally, b_usize = {}", b_usize);
    small_uint_set_max_func(&mut b_usize);
    println!("After b_usize.set_max(), b_usize = {}", b_usize);
    assert_eq!(b_usize, usize::MAX);

    // Example for ShortUnion
    let mut a_shortunion = 10000_u16.into_shortunion();
    println!("Originally, a_shortunion = {}", a_shortunion);
    a_shortunion.set_max();
    println!("After a_shortunion.set_max(), a_shortunion = {}", a_shortunion);
    assert_eq!(a_shortunion.get(), u16::MAX);

    let mut b_shortunion = 20000_u16.into_shortunion();
    println!("Originally, b_shortunion = {}", b_shortunion);
    small_uint_set_max_func(&mut b_shortunion);
    println!("After b_u16.set_max(), b_u16 = {}", b_shortunion);
    assert_eq!(b_shortunion.get(), u16::MAX);

    // Example for IntUnion
    let mut a_intunion = 1000000000_u32.into_intunion();
    println!("Originally, a_intunion = {}", a_intunion);
    a_intunion.set_max();
    println!("After a_intunion.set_max(), a_intunion = {}", a_intunion);
    assert_eq!(a_intunion.get(), u32::MAX);

    let mut b_intunion = 2000000000_u32.into_intunion();
    println!("Originally, b_intunion = {}", b_intunion);
    small_uint_set_max_func(&mut b_intunion);
    println!("After b_intunion.set_max(), b_intunion = {}", b_intunion);
    assert_eq!(b_intunion.get(), u32::MAX);

    // Example for LongUnion
    let mut a_longunion = 10000000000000000000_u64.into_longunion();
    println!("Originally, a_longunion = {}", a_longunion);
    a_longunion.set_max();
    println!("After a_longunion.set_max(), a_longunion = {}", a_longunion);
    assert_eq!(a_longunion.get(), u64::MAX);

    let mut b_longunion = 15000000000000000000_u64.into_longunion();
    println!("Originally, b_longunion = {}", b_longunion);
    small_uint_set_max_func(&mut b_longunion);
    println!("After b_longunion.set_max(), b_longunion = {}", b_longunion);
    assert_eq!(b_longunion.get(), u64::MAX);

    // Example for LongerUnion
    let mut a_longerunion = 100000000000000000000000000000000000000_u128.into_longerunion();
    println!("Originally, a_u128 = {}", a_longerunion);
    a_longerunion.set_max();
    println!("After a_longerunion.set_max(), a_longerunion = {}", a_longerunion);
    assert_eq!(a_longerunion.get(), u128::MAX);

    let mut b_longerunion = 200000000000000000000000000000000000000_u128.into_longerunion();
    println!("Originally, b_longerunion = {}", b_longerunion);
    small_uint_set_max_func(&mut b_longerunion);
    println!("After b_longerunion.set_max(), b_longerunion = {}", b_longerunion);
    assert_eq!(b_longerunion.get(), u128::MAX);

    // Example for SizeUnion
    let mut a_sizeunion = 10000000000000000000_usize.into_sizeunion();
    println!("Originally, a_sizeunion = {}", a_sizeunion);
    a_sizeunion.set_max();
    println!("After a_sizeunion.set_max(), a_sizeunion = {}", a_sizeunion);
    assert_eq!(a_sizeunion.get(), usize::MAX);

    let mut b_sizeunion = 15000000000000000000_usize.into_sizeunion();
    println!("Originally, b_sizeunion = {}", b_sizeunion);
    small_uint_set_max_func(&mut b_sizeunion);
    println!("After b_sizeunion.set_max(), b_sizeunion = {}", b_sizeunion);
    assert_eq!(b_sizeunion.get(), usize::MAX);
    println!("--------------------------------------");
}

fn small_uint_set_max_func<T: cryptocol::number::SmallUInt>(num: &mut T)
{
    num.set_max();
}

fn small_uint_is_max()
{
    println!("small_uint_is_max");
    use cryptocol::number::SmallUInt;
    const TXT_MAX: &str = "maximum";
    const TXT_NOT_MAX: &str = "not maximum";

    // Example for u8
    let a_u8 = u8::MAX;
    let b_u8 = 200_u8;
    let ismax = a_u8.is_max();
    println!("a_u8 is {}.", if ismax {TXT_MAX} else {TXT_NOT_MAX});
    assert_eq!(ismax, true);
    let ismax = small_uint_is_max_func(b_u8);
    println!("b_u8 is {}.", if ismax {TXT_MAX} else {TXT_NOT_MAX});
    assert_eq!(ismax, false);

    // Example for u16
    let a_u16 = u16::MAX;
    let b_u16 = 20000_u16;
    let ismax = a_u16.is_max();
    println!("a_u16 is {}.", if ismax {TXT_MAX} else {TXT_NOT_MAX});
    assert_eq!(ismax, true);
    let ismax = small_uint_is_max_func(b_u16);
    println!("b_u16 is {}.", if ismax {TXT_MAX} else {TXT_NOT_MAX});
    assert_eq!(ismax, false);

    // Example for u32
    let a_u32 = u32::MAX;
    let b_u32 = 2000000000_u32;
    let ismax = a_u32.is_max();
    println!("a_u32 is {}.", if ismax {TXT_MAX} else {TXT_NOT_MAX});
    assert_eq!(ismax, true);
    let ismax = small_uint_is_max_func(b_u32);
    println!("b_u32 is {}.", if ismax {TXT_MAX} else {TXT_NOT_MAX});
    assert_eq!(ismax, false);

    // Example for u64
    let a_u64 = u64::MAX;
    let b_u64 = 15000000000000000000_u64;
    let ismax = a_u64.is_max();
    println!("a_u64 is {}.", if ismax {TXT_MAX} else {TXT_NOT_MAX});
    assert_eq!(ismax, true);
    let ismax = small_uint_is_max_func(b_u64);
    println!("b_u64 is {}.", if ismax {TXT_MAX} else {TXT_NOT_MAX});
    assert_eq!(ismax, false);

    // Example for u128
    let a_u128 = u128::MAX;
    let b_u128 = 200000000000000000000000000000000000000_u128;
    let ismax = a_u128.is_max();
    println!("a_u128 is {}.", if ismax {TXT_MAX} else {TXT_NOT_MAX});
    assert_eq!(ismax, true);
    let ismax = small_uint_is_max_func(b_u128);
    println!("b_u128 is {}.", if ismax {TXT_MAX} else {TXT_NOT_MAX});
    assert_eq!(ismax, false);

    // Example for usize
    let a_usize = usize::MAX;
    let b_usize = 15000000000000000000_usize;
    let ismax = a_usize.is_max();
    println!("a_usize is {}.", if ismax {TXT_MAX} else {TXT_NOT_MAX});
    assert_eq!(ismax, true);
    let ismax = small_uint_is_max_func(b_usize);
    println!("b_usize is {}.", if ismax {TXT_MAX} else {TXT_NOT_MAX});
    assert_eq!(ismax, false);

    // Example for ShortUnion
    let a_shortunion = u16::MAX.into_shortunion();
    let b_shortunion = 20000_u16.into_shortunion();
    let ismax = a_shortunion.is_max();
    println!("a_shortunion is {}.", if ismax {TXT_MAX} else {TXT_NOT_MAX});
    assert_eq!(ismax, true);
    let ismax = small_uint_is_max_func(b_shortunion);
    println!("b_shortunion is {}.", if ismax {TXT_MAX} else {TXT_NOT_MAX});
    assert_eq!(ismax, false);

    // Example for IntUnionn
    let a_intunionn = u32::MAX.into_intunion();
    let b_intunionn = 2000000000_u32.into_intunion();
    let ismax = a_intunionn.is_max();
    println!("a_intunionn is {}.", if ismax {TXT_MAX} else {TXT_NOT_MAX});
    assert_eq!(ismax, true);
    let ismax = small_uint_is_max_func(b_intunionn);
    println!("b_intunionn is {}.", if ismax {TXT_MAX} else {TXT_NOT_MAX});
    assert_eq!(ismax, false);

    // Example for LongUnion
    let a_longunion = u64::MAX.into_longunion();
    let b_longunion = 15000000000000000000_u64.into_longunion();
    let ismax = a_longunion.is_max();
    println!("a_longunion is {}.", if ismax {TXT_MAX} else {TXT_NOT_MAX});
    assert_eq!(ismax, true);
    let ismax = small_uint_is_max_func(b_longunion);
    println!("b_longunion is {}.", if ismax {TXT_MAX} else {TXT_NOT_MAX});
    assert_eq!(ismax, false);

    // Example for LongerUnion
    let a_longerunion = u128::MAX.into_longerunion();
    let b_longerunion = 200000000000000000000000000000000000000_u128.into_longerunion();
    let ismax = a_longerunion.is_max();
    println!("a_longerunion is {}.", if ismax {TXT_MAX} else {TXT_NOT_MAX});
    assert_eq!(ismax, true);
    let ismax = small_uint_is_max_func(b_longerunion);
    println!("b_longerunion is {}.", if ismax {TXT_MAX} else {TXT_NOT_MAX});
    assert_eq!(ismax, false);

    // Example for SizeUnion
    let a_sizeunionn = usize::MAX.into_sizeunion();
    let b_sizeunionn = 15000000000000000000_usize.into_sizeunion();
    let ismax = a_sizeunionn.is_max();
    println!("a_sizeunionn is {}.", if ismax {TXT_MAX} else {TXT_NOT_MAX});
    assert_eq!(ismax, true);
    let ismax = small_uint_is_max_func(b_sizeunionn);
    println!("b_sizeunionn is {}.", if ismax {TXT_MAX} else {TXT_NOT_MAX});
    assert_eq!(ismax, false);
    println!("--------------------------------------");
}

fn small_uint_is_max_func<T: cryptocol::number::SmallUInt>(num: T) -> bool
{
    num.is_max()
}

fn small_uint_set_submax()
{
    println!("small_uint_set_submax");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let mut a_u8 = 100_u8;
    println!("Originally, a_u8 = {}", a_u8);
    a_u8.set_submax(5);
    println!("After a_u8.set_submax(5), a_u8 = {}", a_u8);
    assert_eq!(a_u8, 31_u8);

    let mut b_u8 = 200_u8;
    println!("Originally, b_u8 = {}", b_u8);
    small_uint_set_submax_func(&mut b_u8, 5);
    println!("After small_uint_set_submax_func(&mut b_u8, 5), b_u8 = {}", b_u8);
    assert_eq!(b_u8, 31_u8);

    // Example for u16
    let mut a_u16 = 10000_u16;
    println!("Originally, a_u16 = {}", a_u16);
    a_u16.set_submax(10);
    println!("After a_u16.set_submax(10), a_u16 = {}", a_u16);
    assert_eq!(a_u16, 1023_u16);

    let mut b_u16 = 20000_u16;
    println!("Originally, b_u16 = {}", b_u16);
    small_uint_set_submax_func(&mut b_u16, 10);
    println!("After small_uint_set_submax_func(&mut b_u16, 10), b_u16 = {}", b_u16);
    assert_eq!(b_u16, 1023_u16);

    // Example for u32
    let mut a_u32 = 1000000000_u32;
    println!("Originally, a_u32 = {}", a_u32);
    a_u32.set_submax(20);
    println!("After a_u32.set_submax(20), a_u32 = {}", a_u32);
    assert_eq!(a_u32, 1048575_u32);

    let mut b_u32 = 2000000000_u32;
    println!("Originally, b_u32 = {}", b_u32);
    small_uint_set_submax_func(&mut b_u32, 20);
    println!("After small_uint_set_submax_func(&mut b_u32, 20), b_u32 = {}", b_u32);
    assert_eq!(b_u32, 1048575_u32);

    // Example for u64
    let mut a_u64 = 10000000000000000000_u64;
    println!("Originally, a_u64 = {}", a_u64);
    a_u64.set_submax(50);
    println!("After a_u64.set_submax(50), a_u64 = {}", a_u64);
    assert_eq!(a_u64, 1125899906842623_u64);

    let mut b_u64 = 15000000000000000000_u64;
    println!("Originally, b_u64 = {}", b_u64);
    small_uint_set_submax_func(&mut b_u64, 50);
    println!("After small_uint_set_submax_func(&mut b_u64, 50), b_u64 = {}", b_u64);
    assert_eq!(b_u64, 1125899906842623_u64);

    // Example for u128
    let mut a_u128 = 100000000000000000000000000000000000000_u128;
    println!("Originally, a_u128 = {}", a_u128);
    a_u128.set_submax(100);
    println!("After a_u128.set_submax(100), a_u128 = {}", a_u128);
    assert_eq!(a_u128, 1267650600228229401496703205375_u128);

    let mut b_u128 = 200000000000000000000000000000000000000_u128;
    println!("Originally, b_u128 = {}", b_u128);
    small_uint_set_submax_func(&mut b_u128, 100);
    println!("After small_uint_set_submax_func(&mut b_u128, 100), b_u128 = {}", b_u128);
    assert_eq!(b_u128, 1267650600228229401496703205375_u128);

    // Example for usize
    let mut a_usize = 10000000000000000000_usize;
    println!("Originally, a_usize = {}", a_usize);
    a_usize.set_submax(50);
    println!("After a_usize.set_submax(50), a_usize = {}", a_usize);
    assert_eq!(a_usize, 1125899906842623_usize);

    let mut b_usize = 15000000000000000000_usize;
    println!("Originally, b_usize = {}", b_usize);
    small_uint_set_submax_func(&mut b_usize, 50);
    println!("After small_uint_set_submax_func(&mut b_usize, 50), b_usize = {}", b_usize);
    assert_eq!(b_usize, 1125899906842623_usize);

    // Example for ShortUnion
    let mut a_shortunion = 10000_u16.into_shortunion();
    println!("Originally, a_shortunion = {}", a_shortunion);
    a_shortunion.set_submax(10);
    println!("After a_shortunion.set_submax(10), a_shortunion = {}", a_shortunion);
    assert_eq!(a_shortunion.get(), 1023_u16);

    let mut b_shortunion = 20000_u16.into_shortunion();
    println!("Originally, b_shortunion = {}", b_shortunion);
    small_uint_set_submax_func(&mut b_shortunion, 10);
    println!("After small_uint_set_submax_func(&mut b_shortunion, 10), b_u16 = {}", b_shortunion);
    assert_eq!(b_shortunion.get(), 1023_u16);

    // Example for IntUnion
    let mut a_intunion = 1000000000_u32.into_intunion();
    println!("Originally, a_intunion = {}", a_intunion);
    a_intunion.set_submax(20);
    println!("After a_intunion.set_submax(20), a_intunion = {}", a_intunion);
    assert_eq!(a_intunion.get(), 1048575_u32);

    let mut b_intunion = 2000000000_u32.into_intunion();
    println!("Originally, b_intunion = {}", b_intunion);
    small_uint_set_submax_func(&mut b_intunion, 20);
    println!("After small_uint_set_submax_func(&mut b_intunion, 20), b_intunion = {}", b_intunion);
    assert_eq!(b_intunion.get(), 1048575_u32);

    // Example for LongUnion
    let mut a_longunion = 10000000000000000000_u64.into_longunion();
    println!("Originally, a_longunion = {}", a_longunion);
    a_longunion.set_submax(50);
    println!("After a_longunion.set_submax(50), a_longunion = {}", a_longunion);
    assert_eq!(a_longunion.get(), 1125899906842623_u64);

    let mut b_longunion = 15000000000000000000_u64.into_longunion();
    println!("Originally, b_longunion = {}", b_longunion);
    small_uint_set_submax_func(&mut b_longunion, 50);
    println!("After small_uint_set_submax_func(&mut b_longunion, 50), b_longunion = {}", b_longunion);
    assert_eq!(b_longunion.get(), 1125899906842623_u64);

    // Example for LongerUnion
    let mut a_longerunion = 100000000000000000000000000000000000000_u128.into_longerunion();
    println!("Originally, a_u128 = {}", a_longerunion);
    a_longerunion.set_submax(100);
    println!("After a_longerunion.set_submax(100), a_longerunion = {}", a_longerunion);
    assert_eq!(a_longerunion.get(), 1267650600228229401496703205375_u128);

    let mut b_longerunion = 200000000000000000000000000000000000000_u128.into_longerunion();
    println!("Originally, b_longerunion = {}", b_longerunion);
    small_uint_set_submax_func(&mut b_longerunion, 100);
    println!("After small_uint_set_submax_func(&mut b_longerunion, 100), b_longerunion = {}", b_longerunion);
    assert_eq!(b_longerunion.get(), 1267650600228229401496703205375_u128);

    // Example for SizeUnion
    let mut a_sizeunion = 10000000000000000000_usize.into_sizeunion();
    println!("Originally, a_sizeunion = {}", a_sizeunion);
    a_sizeunion.set_submax(50);
    println!("After a_sizeunion.set_submax(50), a_sizeunion = {}", a_sizeunion);
    assert_eq!(a_sizeunion.get(), 1125899906842623_usize);

    let mut b_sizeunion = 15000000000000000000_usize.into_sizeunion();
    println!("Originally, b_sizeunion = {}", b_sizeunion);
    small_uint_set_submax_func(&mut b_sizeunion, 50);
    println!("After small_uint_set_submax_func(&mut b_sizeunion, 50), b_sizeunion = {}", b_sizeunion);
    assert_eq!(b_sizeunion.get(), 1125899906842623_usize);
    println!("--------------------------------------");
}

fn small_uint_set_submax_func<T: cryptocol::number::SmallUInt>(num: &mut T, size_in_bits: usize)
{
    num.set_submax(size_in_bits);
}

fn small_uint_set_halfmax()
{
    println!("small_uint_set_halfmax");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let mut a_u8 = 100_u8;
    println!("Originally, a_u8 = {}", a_u8);
    a_u8.set_halfmax();
    println!("After a_u8.set_halfmax(), a_u8 = {}", a_u8);
    assert_eq!(a_u8, u8::MAX >> 4);

    let mut b_u8 = 200_u8;
    println!("Originally, b_u8 = {}", b_u8);
    small_uint_set_halfmax_func(&mut b_u8);
    println!("After b_u8.set_halfmax(), b_u8 = {}", b_u8);
    assert_eq!(b_u8, u8::MAX >> 4);

    // Example for u16
    let mut a_u16 = 10000_u16;
    println!("Originally, a_u16 = {}", a_u16);
    a_u16.set_halfmax();
    println!("After a_u16.set_halfmax(), a_u16 = {}", a_u16);
    assert_eq!(a_u16, u16::MAX >> 8);

    let mut b_u16 = 20000_u16;
    println!("Originally, b_u16 = {}", b_u16);
    small_uint_set_halfmax_func(&mut b_u16);
    println!("After b_u16.set_halfmax(), b_u16 = {}", b_u16);
    assert_eq!(b_u16, u16::MAX >> 8);

    // Example for u32
    let mut a_u32 = 1000000000_u32;
    println!("Originally, a_u32 = {}", a_u32);
    a_u32.set_halfmax();
    println!("After a_u32.set_halfmax(), a_u32 = {}", a_u32);
    assert_eq!(a_u32, u32::MAX >> 16);

    let mut b_u32 = 2000000000_u32;
    println!("Originally, b_u32 = {}", b_u32);
    small_uint_set_halfmax_func(&mut b_u32);
    println!("After b_u32.set_halfmax(), b_u32 = {}", b_u32);
    assert_eq!(b_u32, u32::MAX >> 16);

    // Example for u64
    let mut a_u64 = 10000000000000000000_u64;
    println!("Originally, a_u64 = {}", a_u64);
    a_u64.set_halfmax();
    println!("After a_u64.set_halfmax(), a_u64 = {}", a_u64);
    assert_eq!(a_u64, u64::MAX >> 32);

    let mut b_u64 = 15000000000000000000_u64;
    println!("Originally, b_u64 = {}", b_u64);
    small_uint_set_halfmax_func(&mut b_u64);
    println!("After b_u64.set_halfmax(), b_u64 = {}", b_u64);
    assert_eq!(b_u64, u64::MAX >> 32);

    // Example for u128
    let mut a_u128 = 100000000000000000000000000000000000000_u128;
    println!("Originally, a_u128 = {}", a_u128);
    a_u128.set_halfmax();
    println!("After a_u128.set_halfmax(), a_u128 = {}", a_u128);
    assert_eq!(a_u128, u128::MAX >> 64);

    let mut b_u128 = 200000000000000000000000000000000000000_u128;
    println!("Originally, b_u128 = {}", b_u128);
    small_uint_set_halfmax_func(&mut b_u128);
    println!("After b_u128.set_halfmax(), b_u128 = {}", b_u128);
    assert_eq!(b_u128, u128::MAX >> 64);

    // Example for usize
    let mut a_usize = 10000000000000000000_usize;
    println!("Originally, a_usize = {}", a_usize);
    a_usize.set_halfmax();
    println!("After a_usize.set_halfmax(), a_usize = {}", a_usize);
    assert_eq!(a_usize, usize::MAX >> 32);

    let mut b_usize = 15000000000000000000_usize;
    println!("Originally, b_usize = {}", b_usize);
    small_uint_set_halfmax_func(&mut b_usize);
    println!("After b_usize.set_halfmax(), b_usize = {}", b_usize);
    assert_eq!(b_usize, usize::MAX >> 32);

    // Example for ShortUnion
    let mut a_shortunion = 10000_u16.into_shortunion();
    println!("Originally, a_shortunion = {}", a_shortunion);
    a_shortunion.set_halfmax();
    println!("After a_shortunion.set_halfmax(), a_shortunion = {}", a_shortunion);
    assert_eq!(a_shortunion.get(), u16::MAX >> 8);

    let mut b_shortunion = 20000_u16.into_shortunion();
    println!("Originally, b_shortunion = {}", b_shortunion);
    small_uint_set_halfmax_func(&mut b_shortunion);
    println!("After b_u16.set_halfmax(), b_u16 = {}", b_shortunion);
    assert_eq!(b_shortunion.get(), u16::MAX >> 8);

    // Example for IntUnion
    let mut a_intunion = 1000000000_u32.into_intunion();
    println!("Originally, a_intunion = {}", a_intunion);
    a_intunion.set_halfmax();
    println!("After a_intunion.set_halfmax(), a_intunion = {}", a_intunion);
    assert_eq!(a_intunion.get(), u32::MAX >> 16);

    let mut b_intunion = 2000000000_u32.into_intunion();
    println!("Originally, b_intunion = {}", b_intunion);
    small_uint_set_halfmax_func(&mut b_intunion);
    println!("After b_intunion.set_halfmax(), b_intunion = {}", b_intunion);
    assert_eq!(b_intunion.get(), u32::MAX >> 16);

    // Example for LongUnion
    let mut a_longunion = 10000000000000000000_u64.into_longunion();
    println!("Originally, a_longunion = {}", a_longunion);
    a_longunion.set_halfmax();
    println!("After a_longunion.set_halfmax(), a_longunion = {}", a_longunion);
    assert_eq!(a_longunion.get(), u64::MAX >> 32);

    let mut b_longunion = 15000000000000000000_u64.into_longunion();
    println!("Originally, b_longunion = {}", b_longunion);
    small_uint_set_halfmax_func(&mut b_longunion);
    println!("After b_longunion.set_halfmax(), b_longunion = {}", b_longunion);
    assert_eq!(b_longunion.get(), u64::MAX >> 32);

    // Example for LongerUnion
    let mut a_longerunion = 100000000000000000000000000000000000000_u128.into_longerunion();
    println!("Originally, a_u128 = {}", a_longerunion);
    a_longerunion.set_halfmax();
    println!("After a_longerunion.set_halfmax(), a_longerunion = {}", a_longerunion);
    assert_eq!(a_longerunion.get(), u128::MAX >> 64);

    let mut b_longerunion = 200000000000000000000000000000000000000_u128.into_longerunion();
    println!("Originally, b_longerunion = {}", b_longerunion);
    small_uint_set_halfmax_func(&mut b_longerunion);
    println!("After b_longerunion.set_halfmax(), b_longerunion = {}", b_longerunion);
    assert_eq!(b_longerunion.get(), u128::MAX >> 64);

    // Example for SizeUnion
    let mut a_sizeunion = 10000000000000000000_usize.into_sizeunion();
    println!("Originally, a_sizeunion = {}", a_sizeunion);
    a_sizeunion.set_halfmax();
    println!("After a_sizeunion.set_halfmax(), a_sizeunion = {}", a_sizeunion);
    assert_eq!(a_sizeunion.get(), usize::MAX >> 32);

    let mut b_sizeunion = 15000000000000000000_usize.into_sizeunion();
    println!("Originally, b_sizeunion = {}", b_sizeunion);
    small_uint_set_halfmax_func(&mut b_sizeunion);
    println!("After b_sizeunion.set_halfmax(), b_sizeunion = {}", b_sizeunion);
    assert_eq!(b_sizeunion.get(), usize::MAX >> 32);
    println!("--------------------------------------");
}

fn small_uint_set_halfmax_func<T: cryptocol::number::SmallUInt>(num: &mut T)
{
    num.set_halfmax();
}

fn small_uint_size()
{
    small_uint_size_in_bytes();
    small_uint_size_in_bits();
    small_uint_length_in_bytes();
    small_uint_length_in_bits();
}

fn small_uint_size_in_bytes()
{
    println!("small_uint_size_in_bytes");
    use cryptocol::number::{ SmallUInt, ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };
    // Example for u8
    let size_usize = u8::size_in_bytes();
    println!("The size of u8 is {} bytes.", size_usize);
    assert_eq!(size_usize, 1_usize);
    let size_usize = small_uint_size_in_bytes_func::<u8>();
    println!("The size of u8 is {} bytes.", size_usize);
    assert_eq!(size_usize, 1_usize);

    // Example for u16
    let size_usize = u16::size_in_bytes();
    println!("The size of u16 is {} bytes.", size_usize);
    assert_eq!(size_usize, 2_usize);
    let size_usize = small_uint_size_in_bytes_func::<u16>();
    println!("The size of u16 is {} bytes.", size_usize);
    assert_eq!(size_usize, 2_usize);

    // Example for u32
    let size_usize = u32::size_in_bytes();
    println!("The size of u32 is {} bytes.", size_usize);
    assert_eq!(size_usize, 4_usize);
    let size_usize = small_uint_size_in_bytes_func::<u32>();
    println!("The size of u32 is {} bytes.", size_usize);
    assert_eq!(size_usize, 4_usize);

    // Example for u64
    let size_usize = u64::size_in_bytes();
    println!("The size of u64 is {} bytes.", size_usize);
    assert_eq!(size_usize, 8_usize);
    let size_usize = small_uint_size_in_bytes_func::<u64>();
    println!("The size of u64 is {} bytes.", size_usize);
    assert_eq!(size_usize, 8_usize);

    // Example for u128
    let size_usize = u128::size_in_bytes();
    println!("The size of u128 is {} bytes.", size_usize);
    assert_eq!(size_usize, 16_usize);
    let size_usize = small_uint_size_in_bytes_func::<u128>();
    println!("The size of u128 is {} bytes.", size_usize);
    assert_eq!(size_usize, 16_usize);

    // Example for usize
    let size_usize = usize::size_in_bytes();
    println!("The size of u64 is {} bytes.", size_usize);
    assert_eq!(size_usize, 8_usize);
    let size_usize = small_uint_size_in_bytes_func::<usize>();
    println!("The size of u64 is {} bytes.", size_usize);
    assert_eq!(size_usize, 8_usize);

    // Example for ShortUnion
    let size_usize = ShortUnion::size_in_bytes();
    println!("The size of u16 is {} bytes.", size_usize);
    assert_eq!(size_usize, 2_usize);
    let size_usize = small_uint_size_in_bytes_func::<ShortUnion>();
    println!("The size of u16 is {} bytes.", size_usize);
    assert_eq!(size_usize, 2_usize);

    // Example for IntUnion
    let size_usize = IntUnion::size_in_bytes();
    println!("The size of u32 is {} bytes.", size_usize);
    assert_eq!(size_usize, 4_usize);
    let size_usize = small_uint_size_in_bytes_func::<IntUnion>();
    println!("The size of u32 is {} bytes.", size_usize);
    assert_eq!(size_usize, 4_usize);

    // Example for LongUnion
    let size_usize = LongUnion::size_in_bytes();
    println!("The size of u64 is {} bytes.", size_usize);
    assert_eq!(size_usize, 8_usize);
    let size_usize = small_uint_size_in_bytes_func::<LongUnion>();
    println!("The size of u64 is {} bytes.", size_usize);
    assert_eq!(size_usize, 8_usize);

    // Example for LongerUnion
    let size_usize = LongerUnion::size_in_bytes();
    println!("The size of u128 is {} bytes.", size_usize);
    assert_eq!(size_usize, 16_usize);
    let size_usize = small_uint_size_in_bytes_func::<LongerUnion>();
    println!("The size of u128 is {} bytes.", size_usize);
    assert_eq!(size_usize, 16_usize);

    // Example for SizeUnion
    let size_usize = SizeUnion::size_in_bytes();
    println!("The size of u64 is {} bytes.", size_usize);
    assert_eq!(size_usize, 8_usize);
    let size_usize = small_uint_size_in_bytes_func::<SizeUnion>();
    println!("The size of u64 is {} bytes.", size_usize);
    assert_eq!(size_usize, 8_usize);
    println!("--------------------------------------");
}

fn small_uint_size_in_bytes_func<T: cryptocol::number::SmallUInt>() -> usize
{
    T::size_in_bytes()
}

fn small_uint_size_in_bits()
{
    println!("small_uint_size_in_bits");
    use cryptocol::number::{ SmallUInt, ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };
    // Example for u8
    let size_usize = u8::size_in_bits();
    println!("The size of u8 is {} bits.", size_usize);
    assert_eq!(size_usize, 8_usize);
    let size_usize = small_uint_size_in_bits_func::<u8>();
    println!("The size of u8 is {} bits.", size_usize);
    assert_eq!(size_usize, 8_usize);

    // Example for u16
    let size_usize = u16::size_in_bits();
    println!("The size of u16 is {} bits.", size_usize);
    assert_eq!(size_usize, 16_usize);
    let size_usize = small_uint_size_in_bits_func::<u16>();
    println!("The size of u16 is {} bits.", size_usize);
    assert_eq!(size_usize, 16_usize);

    // Example for u32
    let size_usize = u32::size_in_bits();
    println!("The size of u32 is {} bits.", size_usize);
    assert_eq!(size_usize, 32_usize);
    let size_usize = small_uint_size_in_bits_func::<u32>();
    println!("The size of u32 is {} bits.", size_usize);
    assert_eq!(size_usize, 32_usize);

    // Example for u64
    let size_usize = u64::size_in_bits();
    println!("The size of u64 is {} bits.", size_usize);
    assert_eq!(size_usize, 64_usize);
    let size_usize = small_uint_size_in_bits_func::<u64>();
    println!("The size of u64 is {} bits.", size_usize);
    assert_eq!(size_usize, 64_usize);

    // Example for u128
    let size_usize = u128::size_in_bits();
    println!("The size of u128 is {} bits.", size_usize);
    assert_eq!(size_usize, 128_usize);
    let size_usize = small_uint_size_in_bits_func::<u128>();
    println!("The size of u128 is {} bits.", size_usize);
    assert_eq!(size_usize, 128_usize);

    // Example for usize
    let size_usize = usize::size_in_bits();
    println!("The size of u64 is {} bits.", size_usize);
    assert_eq!(size_usize, 64_usize);
    let size_usize = small_uint_size_in_bits_func::<usize>();
    println!("The size of u64 is {} bits.", size_usize);
    assert_eq!(size_usize, 64_usize);

    // Example for ShortUnion
    let size_usize = ShortUnion::size_in_bits();
    println!("The size of ShortUnion is {} bits.", size_usize);
    assert_eq!(size_usize, 16_usize);
    let size_usize = small_uint_size_in_bits_func::<ShortUnion>();
    println!("The size of ShortUnion is {} bits.", size_usize);
    assert_eq!(size_usize, 16_usize);

    // Example for IntUnion
    let size_usize = IntUnion::size_in_bits();
    println!("The size of IntUnion is {} bits.", size_usize);
    assert_eq!(size_usize, 32_usize);
    let size_usize = small_uint_size_in_bits_func::<IntUnion>();
    println!("The size of IntUnion is {} bits.", size_usize);
    assert_eq!(size_usize, 32_usize);

    // Example for LongUnion
    let size_usize = LongUnion::size_in_bits();
    println!("The size of LongUnion is {} bits.", size_usize);
    assert_eq!(size_usize, 64_usize);
    let size_usize = small_uint_size_in_bits_func::<LongUnion>();
    println!("The size of LongUnion is {} bits.", size_usize);
    assert_eq!(size_usize, 64_usize);

    // Example for LongerUnion
    let size_usize = LongerUnion::size_in_bits();
    println!("The size of LongerUnion is {} bits.", size_usize);
    assert_eq!(size_usize, 128_usize);
    let size_usize = small_uint_size_in_bits_func::<LongerUnion>();
    println!("The size of LongerUnion is {} bits.", size_usize);
    assert_eq!(size_usize, 128_usize);

    // Example for SizeUnion
    let size_usize = SizeUnion::size_in_bits();
    println!("The size of SizeUnion is {} bits.", size_usize);
    assert_eq!(size_usize, 64_usize);
    let size_usize = small_uint_size_in_bits_func::<SizeUnion>();
    println!("The size of SizeUnion is {} bits.", size_usize);
    assert_eq!(size_usize, 64_usize);
    println!("--------------------------------------");
}

fn small_uint_size_in_bits_func<T: cryptocol::number::SmallUInt>() -> usize
{
    T::size_in_bits()
}

fn small_uint_length_in_bytes()
{
    println!("small_uint_length_in_bytes");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = 100_u8;
    let size_usize = a_u8.length_in_bytes();
    println!("The size of u8 is {} bytes.", size_usize);
    assert_eq!(size_usize, 1_usize);
    let size_usize = small_uint_length_in_bytes_func(a_u8);
    println!("The size of u8 is {} bytes.", size_usize);
    assert_eq!(size_usize, 1_usize);

    // Example for u16
    let a_u16 = 10000_u16;
    let size_usize = a_u16.length_in_bytes();
    println!("The size of u16 is {} bytes.", size_usize);
    assert_eq!(size_usize, 2_usize);
    let size_usize = small_uint_length_in_bytes_func(a_u16);
    println!("The size of u16 is {} bytes.", size_usize);
    assert_eq!(size_usize, 2_usize);

    // Example for u32
    let a_u32 = 1000000000_u32;
    let size_usize = a_u32.length_in_bytes();
    println!("The size of u32 is {} bytes.", size_usize);
    assert_eq!(size_usize, 4_usize);
    let size_usize = small_uint_length_in_bytes_func(a_u32);
    println!("The size of u32 is {} bytes.", size_usize);
    assert_eq!(size_usize, 4_usize);

    // Example for u64
    let a_u64 = 10000000000000000_u64;
    let size_usize = a_u64.length_in_bytes();
    println!("The size of u64 is {} bytes.", size_usize);
    assert_eq!(size_usize, 8_usize);
    let size_usize = small_uint_length_in_bytes_func(a_u64);
    println!("The size of u64 is {} bytes.", size_usize);
    assert_eq!(size_usize, 8_usize);

    // Example for u128
    let a_u128 = 100000000000000000000000000000000000_u128;
    let size_usize = a_u128.length_in_bytes();
    println!("The size of u128 is {} bytes.", size_usize);
    assert_eq!(size_usize, 16_usize);
    let size_usize = small_uint_length_in_bytes_func(a_u128);
    println!("The size of u128 is {} bytes.", size_usize);
    assert_eq!(size_usize, 16_usize);

    // Example for usize
    let a_usize = 10000000000000000_usize;
    let size_usize = a_usize.length_in_bytes();
    println!("The size of u64 is {} bytes.", size_usize);
    assert_eq!(size_usize, 8_usize);
    let size_usize = small_uint_length_in_bytes_func(a_usize);
    println!("The size of u64 is {} bytes.", size_usize);
    assert_eq!(size_usize, 8_usize);

    // Example for ShortUnion
    let a_shortunion = 10000_u16.into_shortunion();
    let size_usize = a_shortunion.length_in_bytes();
    println!("The size of u16 is {} bytes.", size_usize);
    assert_eq!(size_usize, 2_usize);
    let size_usize = small_uint_length_in_bytes_func(a_shortunion);
    println!("The size of u16 is {} bytes.", size_usize);
    assert_eq!(size_usize, 2_usize);

    // Example for IntUnion
    let a_intunion = 1000000000_u32.into_intunion();
    let size_usize = a_intunion.length_in_bytes();
    println!("The size of u32 is {} bytes.", size_usize);
    assert_eq!(size_usize, 4_usize);
    let size_usize = small_uint_length_in_bytes_func(a_intunion);
    println!("The size of u32 is {} bytes.", size_usize);
    assert_eq!(size_usize, 4_usize);

    // Example for LongUnion
    let a_longunion = 10000000000000000_u64.into_longunion();
    let size_usize = a_longunion.length_in_bytes();
    println!("The size of u64 is {} bytes.", size_usize);
    assert_eq!(size_usize, 8_usize);
    let size_usize = small_uint_length_in_bytes_func(a_longunion);
    println!("The size of u64 is {} bytes.", size_usize);
    assert_eq!(size_usize, 8_usize);

    // Example for LongerUnion
    let a_longerunion = 100000000000000000000000000000000000_u128.into_longerunion();
    let size_usize = a_longerunion.length_in_bytes();
    println!("The size of u128 is {} bytes.", size_usize);
    assert_eq!(size_usize, 16_usize);
    let size_usize = small_uint_length_in_bytes_func(a_longerunion);
    println!("The size of u128 is {} bytes.", size_usize);
    assert_eq!(size_usize, 16_usize);

    // Example for SizeUnion
    let a_sizeunion = 10000000000000000_usize.into_sizeunion();
    let size_usize = a_sizeunion.length_in_bytes();
    println!("The size of u64 is {} bytes.", size_usize);
    assert_eq!(size_usize, 8_usize);
    let size_usize = small_uint_length_in_bytes_func(a_sizeunion);
    println!("The size of u64 is {} bytes.", size_usize);
    assert_eq!(size_usize, 8_usize);
    println!("--------------------------------------");
}

fn small_uint_length_in_bytes_func<T: cryptocol::number::SmallUInt>(num: T) -> usize
{
    num.length_in_bytes()
}

fn small_uint_length_in_bits()
{
    println!("small_uint_length_in_bits");
    use cryptocol::number::SmallUInt;
    // Example for u8
    let a_u8 = 100_u8;
    let size_usize = a_u8.length_in_bits();
    println!("The size of u8 is {} bits.", size_usize);
    assert_eq!(size_usize, 8_usize);
    let size_usize = small_uint_length_in_bits_func(a_u8);
    println!("The size of u8 is {} bits.", size_usize);
    assert_eq!(size_usize, 8_usize);

    // Example for u16
    let a_u16 = 10000_u16;
    let size_usize = a_u16.length_in_bits();
    println!("The size of u16 is {} bits.", size_usize);
    assert_eq!(size_usize, 16_usize);
    let size_usize = small_uint_length_in_bits_func(a_u16);
    println!("The size of u16 is {} bits.", size_usize);
    assert_eq!(size_usize, 16_usize);

    // Example for u32
    let a_u32 = 1000000000_u32;
    let size_usize = a_u32.length_in_bits();
    println!("The size of u32 is {} bits.", size_usize);
    assert_eq!(size_usize, 32_usize);
    let size_usize = small_uint_length_in_bits_func(a_u32);
    println!("The size of u32 is {} bits.", size_usize);
    assert_eq!(size_usize, 32_usize);

    // Example for u64
    let a_u64 = 10000000000000000_u64;
    let size_usize = a_u64.length_in_bits();
    println!("The size of u64 is {} bits.", size_usize);
    assert_eq!(size_usize, 64_usize);
    let size_usize = small_uint_length_in_bits_func(a_u64);
    println!("The size of u64 is {} bits.", size_usize);
    assert_eq!(size_usize, 64_usize);

    // Example for u128
    let a_u128 = 100000000000000000000000000000000000_u128;
    let size_usize = a_u128.length_in_bits();
    println!("The size of u128 is {} bits.", size_usize);
    assert_eq!(size_usize, 128_usize);
    let size_usize = small_uint_length_in_bits_func(a_u128);
    println!("The size of u128 is {} bits.", size_usize);
    assert_eq!(size_usize, 128_usize);

    // Example for usize
    let a_usize = 10000000000000000_usize;
    let size_usize = a_usize.length_in_bits();
    println!("The size of u64 is {} bits.", size_usize);
    assert_eq!(size_usize, 64_usize);
    let size_usize = small_uint_length_in_bits_func(a_usize);
    println!("The size of u64 is {} bits.", size_usize);
    assert_eq!(size_usize, 64_usize);

    // Example for ShortUnion
    let a_shortunion = 10000_u16.into_shortunion();
    let size_usize = a_shortunion.length_in_bits();
    println!("The size of u16 is {} bits.", size_usize);
    assert_eq!(size_usize, 16_usize);
    let size_usize = small_uint_length_in_bits_func(a_shortunion);
    println!("The size of u16 is {} bits.", size_usize);
    assert_eq!(size_usize, 16_usize);

    // Example for IntUnion
    let a_intunion = 1000000000_u32.into_intunion();
    let size_usize = a_intunion.length_in_bits();
    println!("The size of u32 is {} bits.", size_usize);
    assert_eq!(size_usize, 32_usize);
    let size_usize = small_uint_length_in_bits_func(a_intunion);
    println!("The size of u32 is {} bits.", size_usize);
    assert_eq!(size_usize, 32_usize);

    // Example for LongUnion
    let a_longunion = 10000000000000000_u64.into_longunion();
    let size_usize = a_longunion.length_in_bits();
    println!("The size of u64 is {} bits.", size_usize);
    assert_eq!(size_usize, 64_usize);
    let size_usize = small_uint_length_in_bits_func(a_longunion);
    println!("The size of u64 is {} bits.", size_usize);
    assert_eq!(size_usize, 64_usize);

    // Example for LongerUnion
    let a_longerunion = 100000000000000000000000000000000000_u128.into_longerunion();
    let size_usize = a_longerunion.length_in_bits();
    println!("The size of u128 is {} bits.", size_usize);
    assert_eq!(size_usize, 128_usize);
    let size_usize = small_uint_length_in_bits_func(a_longerunion);
    println!("The size of u128 is {} bits.", size_usize);
    assert_eq!(size_usize, 128_usize);

    // Example for SizeUnion
    let a_sizeunion = 10000000000000000_usize.into_sizeunion();
    let size_usize = a_sizeunion.length_in_bits();
    println!("The size of u64 is {} bits.", size_usize);
    assert_eq!(size_usize, 64_usize);
    let size_usize = small_uint_length_in_bits_func(a_sizeunion);
    println!("The size of u64 is {} bits.", size_usize);
    assert_eq!(size_usize, 64_usize);
    println!("--------------------------------------");
}

fn small_uint_length_in_bits_func<T: cryptocol::number::SmallUInt>(num: T) -> usize
{
    num.length_in_bits()
}