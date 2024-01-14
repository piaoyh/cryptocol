// Copyright 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]


use std::ptr::copy_nonoverlapping;
use std::slice::from_raw_parts;
use std::fmt::{ self, Debug, Display, Formatter };

use crate::number::{ SmallUInt, IntUnion, LongUnion };


/// A Keccak message-digest algorithm that lossily compresses data of arbitrary
/// length into a 128-bit hash value, and its flexible variants that allows
/// you to develop your own `Keccak`-based hash algorithms
/// 
/// # Introduction
/// 
/// # Vulnerability
/// There have been several attacks against Keccak
/// but they are all incomplete attacks.
/// Read [more](https://en.wikipedia.org/wiki/SHA-2#Cryptanalysis_and_validation)
/// 
/// # Use of MD4 and their variants
/// Keccak and its variants can be still used for non-cryptograpic purposes such as:
/// - Generating IDs
/// - Integrity test
/// - Storing passwords
/// - Digital Signature
/// - Implementing proof of work for block chain.
/// - Study of hash algorithms
/// 
/// # Generic Parameters
/// You can create your own expanded version of MD4 by changing the generic
/// parameters H0 ~ H3, ROUND, K0 ~ K2, R00 ~ R03, R10 ~ R13, and R20 ~ R23.
/// - N : the size of output. N cannot be 0 or greater than 4. If N is 4, the
/// output is 128 bits, while if N is 1, the output is 32 bits.
/// - H0 ~ H3 : the initial hash values, four u32 values.
/// The default values of H0, H1, H2, and H3 are 0x67452301, 0xefcdab89,
/// 0x98badcfe, and 0x10325476, respectively (in little endian representation).
/// - ROUND : the number of rounds. The default value of it is `48` (= 16 * 3).
/// - K0 ~ K2 : the added values in hashing process, three u32 values.
/// The default values of K0, K1, and K2 are 0x00000000, 0x5A827999, and
/// 0x6ED9EBA1, respectively (in little endian representation).
/// 0x5A827999 is a 32-bit constant of the square root of 2 in little endian
/// prepresentation.
/// 0x6ED9EBA1 is a 32-bit constant of the square root of 3 in little endian
/// prepresentation.
/// - R00 ~ R03, R10 ~ R13, and R20 ~ R23 : the amounts of rotate left in the
/// hashing process.
/// The default values of R00, R01, R02, and R03 are 3, 7, 11, and 19, respectively.
/// The default values of R10, R11, R12, and R13 are 3, 5, 9, and 13, respectively.
/// The default values of R20, R11, R12, and R23 are 3, 9, 11, and 15, respecively.
/// 
/// About the parameters and their default values,
/// read [more](https://datatracker.ietf.org/doc/html/rfc1320)
/// and/or watch [this video](https://www.youtube.com/watch?v=JIhZWgJA-9o)
/// to learn SHA-1 more in detail.
/// 
/// # Security of your own expanded version
/// Your own algrorithm based on MD4 may be stronger or weaker than official
/// MD4. Unless you seriously checked the cryptographic security of your own
/// algorithms, it is hard to assume that your own alogrithms are stronger
/// than the official Keccak.
/// 
/// # Reference
/// Read [more](https://en.wikipedia.org/wiki/MD4) about MD4 in detail.
/// 
/// # Quick Start
/// In order to use the module Keccak, you don't have to import (or use)
/// Cryptocol::hash::keccak::* directly because the module Cryptocol::hash::keccak
/// is re-exported. All you have to do is only import Keccak, Keccak_Expanded,
/// Keccak_Generic_HR_fixed and/or Keccak_Generic in the module Cryptocol::hash.
/// Example 1 shows how to import structs Keccak, Keccak_Expanded,
/// Keccak_Generic_HR_fixed and/or Keccak_Generic. Plus, what you have to know is
/// these. All the types (or structs) are the specific versions of Keccak_Generic.
/// Actually, Keccak is a specific version of Keccak_Expanded. Keccak_Expanded and
/// Keccak_Generic_HR_fixed are specific versions of Keccak_Generic.
/// 
/// ## Example 1
/// ```
/// use Cryptocol::hash::Keccak;
/// use Cryptocol::hash::Keccak;
/// use Cryptocol::hash::Keccak_Generic_HR_fixed;
/// use Cryptocol::hash::Keccak_Generic;
/// ```
/// Then, you create Keccak object by the method Keccak::new(). Now, you are ready to
/// use all provided methods to hash any data. If you want to hash a string,
/// for example, you can use the method absorb_str(). Then, the Keccak object that
/// you created will contain its hash value. You can use the macro println!(),
/// for instance, to print on a commandline screen by `println!("{}", hash)`
/// where hash is the Keccak object. Example 2 shows how to use MD4 struct quickly.
/// 
/// ## Example 2
/// ```
/// ```
/// 
/// # Big-endian issue
/// It is just experimental for Big Endian CPUs. So, you are not encouraged
/// to use it for Big Endian CPUs for serious purpose. Only use this crate
/// for Big-endian CPUs with your own full responsibility.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct Keccak_Generic<const N: usize = 8, const ROUND: usize = 48, T,
        const R00: u32 = 3, const R01: u32 = 7, const R02: u32 = 11, const R03: u32 = 19,
        const R10: u32 = 3, const R11: u32 = 5, const R12: u32 = 9, const R13: u32 = 13, 
        const R20: u32 = 3, const R21: u32 = 9, const R22: u32 = 11, const R23: u32 = 15,
        const X: usize = 5, const Y: usize = 5, const ROUND: usize = 0>
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
{
    hash_code: [LongUnion; 8],
}

impl<const N: usize, T,
    const R00: u32, const R01: u32, const R02: u32, const R03: u32,
    const R10: u32, const R11: u32, const R12: u32, const R13: u32,
    const R20: u32, const R21: u32, const R22: u32, const R23: u32,
    const X: usize, const Y: usize, const ROUND: usize>
Keccak_Generic<N, R00, R01, R02, R03, R10, R11, R12, R13, R20, R21, R22, R23, X, Y, ROUND>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
{
    const ROUNDS: usize = if ROUND == 0 {12 + T::size_in_bits()} else {ROUND};

    fn theta(input: &[[T; X]; Y]) -> [[T; X]; Y]
    {
        let mut c: [T::zero(); X];
        for x in 0..X
        {
            for y in 0..Y
                { c[x] ^= input[x][y]; }
        }

        let mut d: [T::zero(); X];
        for x in 0..X
        {
            for y in 0..Y
                { d[x] = c[x.modular_sub(1, X)] ^ c[y.x.modular_add(1, X)].rotate_right(1); }
        }

        let mut output: [[T; X]; Y];
        for y in 0..Y
        {
            for x in 0..Y
                { output[x][y] = input[x][y] ^ d[x]; }
        }

        output
    }
}