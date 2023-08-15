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

use rand_distr::num_traits::PrimInt; //{u256, BigInteger, HugeInteger};

use Cryptocol::number::*;
use Cryptocol::define_utypes_with;
use Cryptocol::define_Utypes_with_utypes;
use Cryptocol::define_utypes_with_u128;
use Cryptocol::define_utypes_with_u64;
use Cryptocol::define_utypes_with_u32;
use Cryptocol::define_utypes_with_u16;
use Cryptocol::define_utypes_with_u8;


pub fn test_main_BigUInt()
{
    BigUInt_submax___main();

    BigUInt_wrapping_add___main();
    BigUInt_wrapping_add_assign___main();
}



fn BigUInt_submax___main()
{
    define_utypes_with!(u64);
    let maximum = u256::max();
    let half = u256::submax(128_usize);
    println!("maximum =\t{}\nhalf maximum = \t{}", maximum, half);
    assert_eq!(maximum, u256::from_str("347376267711948586270712955026063723559809953996921692118372752023739388919807").unwrap());
    assert_eq!(half, u256::from_str("340282366920938463463374607431768211455").unwrap());
}

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
