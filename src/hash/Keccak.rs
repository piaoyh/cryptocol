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
/// cryptocol::hash::keccak::* directly because the module cryptocol::hash::keccak
/// is re-exported. All you have to do is only import Keccak, Keccak_Expanded,
/// Keccak_Generic_HR_fixed and/or Keccak_Generic in the module cryptocol::hash.
/// Example 1 shows how to import structs Keccak, Keccak_Expanded,
/// Keccak_Generic_HR_fixed and/or Keccak_Generic. Plus, what you have to know is
/// these. All the types (or structs) are the specific versions of Keccak_Generic.
/// Actually, Keccak is a specific version of Keccak_Expanded. Keccak_Expanded and
/// Keccak_Generic_HR_fixed are specific versions of Keccak_Generic.
/// 
/// ## Example 1
/// ```
/// use cryptocol::hash::Keccak;
/// use cryptocol::hash::Keccak;
/// use cryptocol::hash::Keccak_Generic_HR_fixed;
/// use cryptocol::hash::Keccak_Generic;
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
pub struct Keccak_Generic<const N: usize = 8, T = u64,
        const THETA_LEFT: usize = 1, const THETA_RIGHT: usize = 1,
        const THETA_RR1: usize = 1,
        const RHO_NEXT1: usize = 1, const RHO_NEXT2: usize = 2, 
        const SIZE: usize = 5, const ROUND: usize = 0>
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
{
    state: [[T; SIZE]; SIZE],
}

impl<const N: usize, T, const THETA_LEFT: usize, const THETA_RIGHT: usize,
    const RR1: usize,
    const SIZE: usize, const ROUND: usize>
Keccak_Generic<N, T, THETA_LEFT, THETA_RIGHT, RR1, X, Y, ROUND>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
{
    const ROUNDS: usize = if ROUND == 0 {12 + T::size_in_bits()} else {ROUND};
    const RC: [u64; 24] = [ 0x0000000000000001, 0x0000000000008082, 0x800000000000808A, 
                            0x8000000080008000, 0x000000000000808B, 0x0000000080000001, 
                            0x8000000080008081, 0x8000000000008009, 0x000000000000008A, 
                            0x0000000000000088, 0x0000000080008009, 0x000000008000000A, 
                            0x000000008000808B, 0x800000000000008B, 0x8000000000008089, 
                            0x8000000000008003, 0x8000000000008002, 0x8000000000000080, 
                            0x000000000000800A, 0x800000008000000A, 0x8000000080008081, 
                            0x8000000000008080, 0x0000000080000001, 0x8000000080008008 ];

    #[inline]
    pub fn new() -> Self
    {
        Self { state: [[T::zero(); SIZE]; SIZE] }
    }

    pub fn initialize()
    {
        for y in 0..SIZE
        {
            for x in 0..SIZE
                { self.state[x][y] = 0; }
        }
    }

    pub fn absorb(message: *const u8, lemgth_in_bute: isize)
    {
        const SHIFT_NUM: usize = T::size_in_bits().ilog(2);
        const CHUNK_NUM: usize = 16;
        self.initialize();
        let length_done = (length_in_bytes >> SHIFT_NUM) as usize;
        for i in 0..length_done
            { self.update(unsafe { from_raw_parts(message.add(i << SHIFT_NUM) as *const T, SIZE * SIZE) } ); }
        self.finalize(unsafe { message.add(length_done << SHIFT_NUM) }, length_in_bytes);

    }

    fn convert_message_to_state(mut message: [T; SIZE * SIZE])
    {
        let mut i = 0_usize;
        for y in 0..SIZE
        {
            for x in 0..SIZE
            {
                self.state[x][y] = message[i];
                i += 1;
            }
        }
    }

    fn convert_state_to_message() -> [T; SIZE * SIZE]
    {
        let mut i = 0_usize;
        let mut message: [T; SIZE * SIZE];
        for y in 0..SIZE
        {
            for x in 0..SIZE
            {
                message[i] = self.state[x][y];
                i += 1;
            }
        }
        message
    }

    fn theta(input: &[[T; SIZE]; SIZE]) -> [[T; SIZE]; SIZE]
    {
        let mut c: [T::zero(); X];
        for x in 0..SIZE
        {
            for y in 0..SIZE
                { c[x] ^= input[x][y]; }
        }

        let mut d: [T::zero(); SIZE];
        for x in 0..SIZE
        {
            for y in 0..SIZE
            {
                let a = c[y.x.modular_add(THETA_RIGHT, SIZE)];
                let b = a.rotate_right(THETA_RR1);
                d[x] = c[x.modular_sub(THETA_LEFT, SIZE)] ^ b;
            }
        }

        let mut output: [[T; SIZE]; SIZE];
        for y in 0..SIZE
        {
            for x in 0..SIZE
                { output[x][y] = input[x][y] ^ d[x]; }
        }
        output
    }

    fn rho(cube: &mut [[T; SIZE]; SIZE])
    {
        const X: usize = SIZE >> 1;
        const Y: usize = X + (SIZE & 1);
        let (mut x, mut y) = (1_usize, 0_usize);
        for t in 0..(SIZE * SIZE)
        {
            let rr = ((t + RHO_NEXT1) * (t + RHO_NEXT2)) >> 1;
            cube[x][y] = cube[x][y].rotate_right(rr);

            let xx = X.modular_mul(x, SIZE);
            let yy = Y.modular_mul(y, SIZE);
            (x, y) = (y, (xx.modular_add(yy, SIZE)));
        }
    }

    fn pi(input: [[T; SIZE]; SIZE]) -> [[T; SIZE]; SIZE]
    {
        const Y: usize = (SIZE >> 1) + (SIZE & 1);
        let mut output: [[T; SIZE]; SIZE];
        for y in 0..SIZE
        {
            for x in 0..SIZE
            {
                let yy = Y.modular_mul(y, SIZE);    // yy = (Y * y) % SIZE;
                let xx = x.modular_add(yy, SIZE);   // XX = (x + yy) % SIZE;
                ouput[x][y] = input[xx][x];
            }
        }
        output
    }

    fn chi(input: [[T; SIZE]; SIZE]) -> [[T; SIZE]; SIZE]
    {
        let mut output: [[T; SIZE]; SIZE];
        for y in 0..SIZE
        {
            for x in 0..SIZE
            {
                let a = !input[x][y.modular_add(1, SIZE)];
                let b = input[x][y.modular_add(2, SIZE)];
                output[x][y] = input[x][y] ^ (a & b);
            }
       }
       output
    }

    #[inline]
    fn iota(cube: &mut [[T; SIZE]; SIZE], round: usize)
    {
        cube[0][0] ^= self.get_RC(round);
    }

    fn rc(t: usize) -> bool
    {
        if t % 255 == 0
            { return true; }

        let R = 10000000;
        for i in 1..(t % 255)
        {
            
        }

    }

    #[inline] fn get_RC(idx: usize) { RC[idx] }
}