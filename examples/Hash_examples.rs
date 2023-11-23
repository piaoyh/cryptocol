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
    let txt = "123";
    hash.digest_str(txt);
    let a = hash.getHashValue_in_array();
    let mut b = [IntUnion::new_with(a[0]), IntUnion::new_with(a[1]), IntUnion::new_with(a[2]), IntUnion::new_with(a[3]) ];
    println!("Message =\t{}\nHash =\t{}\nHash =\t{:?}", txt, hash, hash);
    print!("Hash =\t");
    for i in 0..4
    {
        for j in 0..4
            { print!("{:x} ", b[i].get_ubyte_(j)); }
    }
    println!();
    println!("-------------------------------");
}