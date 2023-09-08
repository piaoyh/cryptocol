// Copyright 2023 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(missing_docs)]
#![allow(missing_doc_code_examples)]
/*
use std::time::SystemTime;
use std::fmt::{ Display, Debug };
use std::ops::*;
use std::convert::*;
use std::mem::size_of;
use std::str::FromStr;

use rand_distr::num_traits::PrimInt; //{u256, BigInteger, HugeInteger};

use Cryptocol::number::*;
use Cryptocol::define_utypes_with;
use Cryptocol::define_Utypes_with_utypes;
use Cryptocol::define_utypes_with_u128;
use Cryptocol::define_utypes_with_u64;
use Cryptocol::define_utypes_with_u32;
use Cryptocol::define_utypes_with_u16;
use Cryptocol::define_utypes_with_u8;
*/

pub fn test_main_BigUInt()
{
    // Test();
    // BigUInt_submax___main();

    // BigUInt_carrying_add___main();
    // BigUInt_carrying_add_assign___main();
    //BigUInt_wrapping_add___main();
    //BigUInt_wrapping_add_assign___main();

    // BigUInt_abs_diff___main();

    // BigUInt_pow_uint___main();
    // BigUInt_pow_assign_uint___main();
    // BigUInt_pow___main();
    // BigUInt_pow_assign___main();
}



fn BigUInt_submax___main()
{
    println!("BigUInt_submax___main()");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;

    define_utypes_with!(u64);
    let maximum = u256::max();
    let half = u256::submax(128_usize);
    println!("maximum =\t{}\nhalf maximum = \t{}", maximum, half);
    assert_eq!(maximum, u256::from_str("347376267711948586270712955026063723559809953996921692118372752023739388919807").unwrap());
    assert_eq!(half, u256::from_str("340282366920938463463374607431768211455").unwrap());
    println!("---------------------------");
}

fn BigUInt_carrying_add___main()
{
    println!("BigUInt_carrying_add___main()");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;

    define_utypes_with!(u128);

    let a_hi = u256::from_str("9876543210987654321098765432109876543210987654321098765432109876543210987654").unwrap();
    let a_lo = u256::from_str("91234567890123456789012345678901234567890123456789012345678901234567890123456").unwrap();
    let b_hi = u256::from_str("1111111101111111110111111111011111111101111111110111111111011111111101111110").unwrap();
    let b_lo = u256::from_str("101111111101111111110111111111011111111101111111110111111111011111111101111110").unwrap();

    let (c_lo, carry) = a_lo.carrying_add(&b_lo, false);
    let (c_hi, overflow) = a_hi.carrying_add(&b_hi, carry);

    println!("{}:{} + {}:{} = {}:{}", a_hi, a_lo, b_hi, b_lo, c_hi, c_lo);
    println!("carry = {}, overflow = {}", carry, overflow);

    assert_eq!(c_hi.to_string(), "10987654312098765431209876543120987654312098765431209876543120987654312098765");
    assert_eq!(c_lo.to_string(), "76553589753918372475552471781224437825721249902258559417332328337765861594630");
    assert_eq!(carry, true);
    assert_eq!(overflow, false);
    println!("---------------------------");
}

fn BigUInt_carrying_add_assign___main()
{
    println!("BigUInt_carrying_add_assign___main()");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;

    define_utypes_with!(u128);

    let mut a_hi = u256::from_str("9876543210987654321098765432109876543210987654321098765432109876543210987654").unwrap();
    let mut a_lo = u256::from_str("91234567890123456789012345678901234567890123456789012345678901234567890123456").unwrap();
    let b_hi = u256::from_str("1111111101111111110111111111011111111101111111110111111111011111111101111110").unwrap();
    let b_lo = u256::from_str("101111111101111111110111111111011111111101111111110111111111011111111101111110").unwrap();

    let carry = a_lo.carrying_add_assign(&b_lo, false);
    let overflow = a_hi.carrying_add_assign(&b_hi, carry);

    println!("9876543210987654321098765432109876543210987654321098765432109876543210987654:91234567890123456789012345678901234567890123456789012345678901234567890123456 + {}:{} = {}:{}", b_hi, b_lo, a_hi, a_lo);
    println!("carry = {}, overflow = {}", carry, overflow);

    assert_eq!(a_hi.to_string(), "10987654312098765431209876543120987654312098765431209876543120987654312098765");
    assert_eq!(a_lo.to_string(), "76553589753918372475552471781224437825721249902258559417332328337765861594630");
    assert_eq!(carry, true);
    assert_eq!(overflow, false);
    println!("---------------------------");
}

fn BigUInt_wrapping_add___main()
{
    println!("BigUInt_wrapping_add___main()");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let zero = u512::zero();
    let one = u512::one();
    let two = u512::from(2_u8);
    let three = u512::from(3_u8);
    let a = u512::max().wrapping_sub(&one);
    let b = a.wrapping_add(&one);
    let c = a.wrapping_add(&two);
    let d = a.wrapping_add(&three);

    println!("{} + 1 = {}", a, b);
    assert_eq!(b, u512::max());

    println!("{} + 2 = {}", a, c);
    assert_eq!(c, zero);

    println!("{} + 3 = {}", a, d);
    assert_eq!(d, one);
    println!("---------------------------");
}

fn BigUInt_wrapping_add_assign___main()
{
    println!("BigUInt_wrapping_add_assign___main()");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let zero = u512::zero();
    let one = u512::one();

    let mut a = u512::max().wrapping_sub(&one);
    println!("Originally,\ta = {}", a);
    assert_eq!(a.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");

    a.wrapping_add_assign(&one);
    println!("After a += 1,\ta = {}", a);
    assert_eq!(a, u512::max());

    a.wrapping_add_assign(&one);
    println!("After a += 1,\ta = {}", a);
    assert_eq!(a, zero);

    a.wrapping_add_assign(&one);
    println!("After a += 1,\ta = {}", a);
    assert_eq!(a, one);
    println!("---------------------------");
}

fn BigUInt_abs_diff___main()
{
    println!("BigUInt_abs_diff___main()");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);
    
    let a = u256::from_str("500000000000000000500000000500000000500000000500000000").unwrap();
    let b = u256::from_str("500000000000000000000000000000000000000000000000000000").unwrap();
    let c = a.abs_diff(&b);
    let d = b.abs_diff(&a);
    println!("500000000000000000500000000500000000500000000500000000 <-> 500000000000000000000000000000000000000000000000000000 = {}", c);
    println!("500000000000000000000000000000000000000000000000000000 <-> 500000000000000000500000000500000000500000000500000000 = {}", d);
    assert_eq!(c, u256::from_str("500000000500000000500000000500000000").unwrap());
    assert_eq!(d, u256::from_str("500000000500000000500000000500000000").unwrap());
    println!("---------------------------");
}

fn BigUInt_pow_uint___main()
{
    println!("BigUInt_pow_uint___main()");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let a = u256::from_uint(123_u8);

    // normal exponentiation
    let b = a.pow_uint(36_u8);
    println!("123 ** 36 = {}", b);
    assert_eq!(b.to_string(), "1724185592748300222303045014791251528772289498837076631331177393773983461361");

    // wrapping (modular) exponentiation
    let c = a.pow_uint(37_u8);
    println!("123 ** 37 = {}", c);
    assert_eq!(c.to_string(), "96282738670724731919703551810636030185721623691319861614277235426286836107467");

    // evidence of wrapping (modular) exponentiation
    assert_eq!(b.is_overflow(), false);
    assert_eq!(c.is_overflow(), true);
    println!("---------------------------");
}


fn BigUInt_pow_assign_uint___main()
{
    println!("BigUInt_pow_assign_uint___main()");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let mut a = u256::from_uint(123_u8);
    let mut b = a.clone();

    // normal exponentiation
    a.pow_assign_uint(36_u8);
    println!("123 ** 36 = {}", a);
    assert_eq!(a.to_string(), "1724185592748300222303045014791251528772289498837076631331177393773983461361");

    // wrapping (modular) exponentiation
    b.pow_assign_uint(37_u8);
    println!("123 ** 37 = {}", b);
    assert_eq!(b.to_string(), "96282738670724731919703551810636030185721623691319861614277235426286836107467");

    // evidence of wrapping (modular) exponentiation
    assert_eq!(a.is_overflow(), false);
    assert_eq!(b.is_overflow(), true);
    println!("---------------------------");
}

fn BigUInt_pow___main()
{
    println!("BigUInt_pow___main()");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let a = u256::from_uint(234_u8);
    let mut exp = u256::from_uint(34_u8);

    // normal exponentiation
    let b = a.pow(&exp);
    println!("234 ** 34 = {}", b);
    assert_eq!(b.to_string(), "101771369680718065636717400052436696519017873276976456689251925337442881634304");
    println!("{}", b.is_overflow());
    // wrapping (modular) exponentiation
    exp += 1;
    let c = a.pow(&exp);
    println!("234 ** 35 = {}", c);
    assert_eq!(c.to_string(), "77122211638207297159819685489165875529835490356175237196145807339442726240256");

    // evidence of wrapping (modular) exponentiation
    assert!(b > c);
    println!("---------------------------");
}

fn BigUInt_pow_assign___main()
{
    println!("BigUInt_pow_assign___main()");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let mut a = u256::from_uint(234_u8);
    let mut exp = u256::from_uint(34_u8);

    // normal exponentiation
    a.pow_assign(&exp);
    println!("234 ** 34 = {}", a);
    assert_eq!(a.to_string(), "101771369680718065636717400052436696519017873276976456689251925337442881634304");
    println!("{}", a.is_overflow());
    // wrapping (modular) exponentiation
    let old = a.clone();
    a = u256::from_uint(234_u8);
    exp += 1;
    a.pow_assign(&exp);
    println!("234 ** 35 = {}", a);
    assert_eq!(a.to_string(), "77122211638207297159819685489165875529835490356175237196145807339442726240256");

    // evidence of wrapping (modular) exponentiation
    assert!(old > a);
    println!("---------------------------");
}

pub fn find_maximum()
{
    println!("find_maximum()");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let a = u256::from_uint(123_u8);
    let mut exp = u256::from_uint(1_u8);
    loop {
        let b = a.pow(&exp);
        if b.is_overflow()
        {
            println!("Maximum i is {}", exp);
            break;
        }
        exp.wrapping_add_assign_uint(1_u8);
    }
    println!("---------------------------");
}

pub fn Test()
{
    println!("Test()");
    use Cryptocol::number::*;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let p = u256::from_uint(12345678901234567890123456789_u128);
    let q = u256::from_uint(12345678901234567890_u128);
    let r = p.gcd(&q);

    println!("{} , {} => {}", p, q, r);

    let a = u256::from_uint(254_u8);
    let b = u256::from_uint(123_u8);
    let c = a.divide_fully(&b);
    let d = a.divide_fully_uint(123_u8);
    let aa = LongerUnion::new_with(254_u128);
    let bb = LongerUnion::new_with(123_u128);

    let cc = aa % bb;

    println!("c: {}  {}", c.0, c.1);
    println!("d: {}  {}", d.0, d.1);
    println!("{}", cc);

    let e = a.divide_fully_uint(4_u8);
    println!("{:?} {:?}", e.0, e.1);

    println!("a == b {}", a == b);
    println!("a != b {}", a != b);
    println!("a > b {}", a > b);
    println!("a >= b {}", a >= b);
    println!("a < b {}", a < b);
    println!("a <= b {}", a <= b);
}