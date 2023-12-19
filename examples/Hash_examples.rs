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

use Cryptocol::hash::{MD5, SHA1};
use Cryptocol::number::IntUnion;
use rand_distr::num_traits::PrimInt; //{u256, BigInteger, HugeInteger};


fn main()
{
    Hash_MD5_main();
}

fn Hash_MD5_main()
{
    println!("Hash_MD5_main");
    let mut hash = MD5::new();
    let mut txt = "";
    hash.digest_str(txt);
    println!("Msg =\t{}\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.getHashValue_in_string(), "D41D8CD98F00B204E9800998ECF8427E");

    txt = "A";
    hash.digest_str(txt);
    println!("Msg =\t{}\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.getHashValue_in_string(), "7FC56270E7A70FA81A5935B72EACBE29");

    let mut txt = "Wow";
    hash.digest_str(txt);
    println!("Msg =\t{}\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.getHashValue_in_string(), "49DC5E45FBEC1433E2C612E5AA809C10");

    txt = "This data is 26-byte long.";
    hash.digest_str(txt);
    println!("Msg =\t{}\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.getHashValue_in_string(), "17ED1DB5CD96184041659D84BB36D76B");

    txt = "The unit of data length is not byte but bit.";
    hash.digest_str(txt);
    println!("Msg =\t{}\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.getHashValue_in_string(), "C3EB6D4A1071E1A9C5E08FEF6E8F3FBF");

    txt = "I am testing MD5 for the data whose length is sixty-two bytes.";
    hash.digest_str(txt);
    println!("Msg =\t{}\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.getHashValue_in_string(), "6C33614E6317DC4641573E0EBC287F98");

    let mut txt = "I am testing MD5 for the data whose length is sixty-four bytes..";
    hash.digest_str(txt);
    println!("Msg =\t{}\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.getHashValue_in_string(), "200F9A19EA45A830284342114483172B");

    txt = "I am testing MD5 for the case data whose length is more than sixty-four bytes is given.";
    hash.digest_str(txt);
    println!("Msg =\t{}\nHash =\t{}", txt, hash);
    assert_eq!(hash.getHashValue_in_string(), "9831162AB272AE1D85245B75726D215E");
    println!("-------------------------------");
}
