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
    // BigUInt_submax___main();

    BigUInt_carrying_add___main();
    // BigUInt_wrapping_add___main();
    // BigUInt_wrapping_add_assign___main();

    // BigUInt_abs_diff___main();

    // BigUInt_pow_uint___main();
    // BigUInt_pow___main();
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

fn BigUInt_wrapping_add___main()
{
    println!("BigUInt_wrapping_add___main()");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let a = u512::max() - u512::from(1_u128);

    println!("{} + 1 = {}", a, a.wrapping_add(u512::from(1_u128)));
    assert_eq!(a.wrapping_add(u512::from(1_u128)), u512::max());

    println!("{} + 2 = {}", a, a.wrapping_add(u512::from(2_u128)));
    assert_eq!(a.wrapping_add(u512::from(2_u128)), u512::zero());

    println!("{} + 3 = {}", a, a.wrapping_add(u512::from(3_u128)));
    assert_eq!(a.wrapping_add(u512::from(3_u128)), u512::one());
    println!("---------------------------");
}

fn BigUInt_wrapping_add_assign___main()
{
    println!("BigUInt_wrapping_add_assign___main()");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;

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
    let b = a.pow_uint(37_u8);
    println!("123 ** 37 = {}", b);
    assert_eq!(b.to_string(), "327866917145357122766845521828011845892261593022600989693192403442113095387339");

    // wrapping (modular) exponentiation
    let c = a.pow_uint(38_u8);
    println!("123 ** 38 = {}", c);
    assert_eq!(c.to_string(), "31983754292890092919296401822065111810221278137005446531426388626141617944969");

    // evidence of wrapping (modular) exponentiation
    assert!(b > c);
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
    let b = a.pow(exp);
    println!("234 ** 34 = {}", b);
    assert_eq!(b.to_string(), "333355548155350456483859370069812512225557842608257584768167093353269140914176");

    // wrapping (modular) exponentiation
    exp += 1;
    let c = a.pow(exp);
    println!("234 ** 35 = {}", c);
    assert_eq!(c.to_string(), "308706390112839688006961655506541691236375459687456365275060975355268985520128");

    // evidence of wrapping (modular) exponentiation
    assert!(b > c);
    println!("---------------------------");
}

fn BigUInt_carrying_add___main()
{
    println!("BigUInt_carrying_add___main()");
    use std::str::FromStr;
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);
/*
    let a = u256::from_str("1234567890123456789012345678901234567890123456789012345678901234567890123456").unwrap();
    let b = u256::from_str("9876543210987654321098765432109876543210987654321098765432109876543210987654").unwrap();
    let (c, carry) = a.carrying_add(b, false);
    println!("{} + {} = {}", a, b, c);
    println!("carry = {}", carry);
    assert_eq!(c.to_string(), "11111111101111111110111111111011111111101111111110111111111011111111101111110");
    assert_eq!(carry, false);
*/
    let d = u256::from_str("1234567890123456789012345678901234567890123456789012345678901234567890123456789").unwrap();
    println!("value = {}, overflow = {}", d, d.is_overflow());
}


fn find_maximum()
{
    println!("find_maximum()");
    use Cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let a = u256::from_uint(234_u8);
    let mut exp = u256::from_uint(1_u8);
    loop {
        let bb = a.pow(exp);
        let cc = a.pow(exp+1);
        if bb > cc
        {
            println!("Maximum i is {}", exp);
            break;
        }
        exp += 1;
    }
    println!("---------------------------");
}