// Copyright 2023 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module that contains SHA1 hash algorithm

#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

use std::fmt::{ self, Debug, Display, Formatter };
use std::vec::*;
use std::mem::{ size_of, size_of_val };
use std::cmp::{ PartialEq, PartialOrd, Ordering };
use std::ops::*;

use crate::number::{IntUnion, LongUnion};

#[derive(Debug, Clone)]
pub struct SHA1
{
    hash_code: [u32; 5],
}

impl SHA1
{
    const K: [u32; 4] = [ 0x5a827999_u32, 0x6ed9eba1_u32, 0x8f1bbcdc_u32, 0xca62c1d6_u32 ];
    const H: [u32; 5] = [ 0x67452301_u32, 0xefcdab89_u32, 0x98badcfe_u32, 0x10325476_u32, 0xc3d2e1f0_u32 ];

    pub fn new() -> Self    { SHA1 { hash_code: Self::H } }

    /// // pub fn digest(&mut self, message: *const u8, length_in_bytes: u64)
    pub fn digest(&mut self, message: *const u8, length_in_bytes: u64)
    {
        self.initialize();
        let length_done = (length_in_bytes / 64) as usize;
        for i in 0..length_done
            { unsafe { self.update(&*(message.add(i * 16 * 4) as *const [u32; 16]) as &[u32; 16]); } }
        self.finalize(unsafe { message.add(length_done * 16 * 4) }, length_in_bytes);
    }

    /// // pub fn digest_str(&mut self, message: &str)
    #[inline]
    pub fn digest_str(&mut self, message: &str)
    {
        self.digest(message.as_ptr(), message.len() as u64);
    }

    /// // pub fn digest_str(&mut self, message: &str)
    #[inline]
    pub fn digest_array<const N: usize>(&mut self, message: [u8; N])
    {
        self.digest(message.as_ptr(), N as u64);
    }

    /// // pub fn getHashValue(&self, hashValue: *mut u8, length: usize)
    /// 
    pub fn getHashValue(&self, hashValue: *mut u8, length: usize)
    {
        let n_length = if length < (5 * 4) {length} else {5 * 4};
        union HU
        {
            hh: [u32; 5],
            hh8: [u8; 5*4],
        }
        let mut hu = HU { hh: [0_u32; 5]};
    
        for i in 0..5_usize
            { unsafe { hu.hh[i] = self.hash_code[i].to_be(); } }
    
        for i in 0..n_length
            { unsafe { *hashValue.add(i) = hu.hh8[i]; } }
    }

    /// // pub fn getHashValue_in_array(&self) -> [u32; 5]
    /// 
    pub fn getHashValue_in_array(&self) -> [u32; 5]
    {
        self.hash_code.clone()
    }

    /// // pub fn getHashValue_in_vec(&self) -> Vec<u32>
    /// 
    pub fn getHashValue_in_vec(&self) -> Vec<u32>
    {
        Vec::from(self.hash_code)
    }

    /// // pub fn getHashValue_in_vec(&self) -> Vec<u32>
    /// 
    pub fn getHashValue_in_string(&self) -> String
    {
        let mut lu = LongUnion::new();
        let mut txt = String::new();
        for i in 0..5
        {
            lu.set_uint_(0, self.hash_code[i]);
            for j in 0..4
            {
                let byte = lu.get_ubyte_(j);
                txt.push(Self::to_char(byte >> 4));
                txt.push(Self::to_char(byte & 0b1111));
            }
        }
        txt
    }

    fn initialize(&mut self)
    {
        for i in 0..5_usize
            { self.hash_code[i] = Self::getH(i); }
    }

    fn update(&mut self, message: &[u32; 16])
    {
        let mut M = [0_u32; 16];
        let mut W = [0_u32; 80];
        let mut T: u32;

        let mut a = self.hash_code[0];
        let mut b = self.hash_code[1];
        let mut c = self.hash_code[2];
        let mut d = self.hash_code[3];
        let mut e = self.hash_code[4];
    
        for i in 0..16
        {
            M[i] = message[i].to_be();
            W[i] = M[i];
        }
        for i in 16..80
            { W[i] = Self::ROTL1(&W, i); }
    
        for i in 0..80
        {
            T = Self::ROTL5(a) + Self::f(b, c, d, i) + e + Self::getK(i) + W[i];
            e = d;
            d = c;
            c = Self::ROTL30(b);
            b = a;
            a = T;
        }
    
        self.hash_code[0] += a;
        self.hash_code[1] += b;
        self.hash_code[2] += c;
        self.hash_code[3] += d;
        self.hash_code[4] += e;
    }

    fn finalize(&mut self, message: *const u8, length_in_bytes: u64)
    {
        union MU {
            M64: [u64; 2 * 4],
            M32: [u32; 4 * 4],
            M8: [[u8; 4 * 4]; 4],
            M: [u8; 4 * 4 * 4],
        }
        let mut mu = MU { M: [0; 4 * 4 * 4] };
        let mut msg = message as *const u32;
        let mut msg8 = message as *const [u8; 4];
    //	const uint32_t	lengthDone = nLengthInBytes / (16 * sizeof (baseType));
        let length_padding = (length_in_bytes % (4 * 4 * 4)) as usize;
        let length_in_blocks = length_padding / 4;
        let length_remained = length_padding % 4;

        for i in 0..length_in_blocks
            { unsafe { mu.M32[i] = *msg.add(i); } }

        for i in 0..length_remained
            { unsafe { mu.M8[length_in_blocks][i] = (*msg8.add(length_in_blocks))[i]; } }

        unsafe { mu.M[length_in_blocks * 4 + length_remained] = 0b10000000_u8; }
        if length_in_bytes + 1 >= 64 - 8
        {
            for i in (length_in_blocks * 4 + length_remained + 1)..64
                { unsafe { mu.M[i] = 0; } }
            self.update(unsafe { &mu.M32 } );
            for i in 0..8-1
                { unsafe { mu.M64[i] = 0; } }
        }
        else
        {
            for i in (length_in_blocks * 4 + length_remained + 1)..(64-8)
                { unsafe { mu.M[i] = 0; } }
        }
        unsafe { mu.M64[7] = (length_in_bytes * 8).to_be(); }
        self.update(unsafe { &mu.M32 });
    }

    fn f(x: u32, y: u32, z: u32, round: usize) -> u32
    {
        if round < 20
           { Self::Ch(x, y, z) }
        else if round < 40
            { Self::Parity(x, y, z) }
        else if round < 60
            { Self::Maj(x, y, z) }
        else
            { Self::Parity(x, y, z) }
    }

	#[inline] fn getK(idx: usize) -> u32    { Self::K[idx/20] }
	#[inline] fn getH(idx: usize) -> u32	{ Self::H[idx] }

	#[inline] fn Ch(x: u32, y: u32, z: u32) -> u32      { (x & y) ^ (!x & z) }
	#[inline] fn Maj(x: u32, y: u32, z: u32) -> u32		{ (x & y) ^ (x & z) ^ (y & z) }
	#[inline] fn Parity(x: u32, y: u32, z: u32) -> u32	{ x ^ y ^ z }

	#[inline] fn ROTL1(w: &[u32; 80], round: usize) -> u32	{ (w[round-3] ^ w[round-8] ^ w[round-14] ^ w[round-16]).rotate_left(1) }
	#[inline] fn ROTL5(x: u32) -> u32           { x.rotate_left(5) }
	#[inline] fn ROTL30(x: u32) -> u32          { x.rotate_left(30) }
    #[inline] fn to_char(nibble: u8) -> char    { if nibble < 10  { ('0' as u8 + nibble) as u8 as char } else { ('A' as u8 - 10 + nibble) as char } }
}


impl Display for SHA1
{
    /// Formats the value using the given formatter.
    /// Automagically the function `to_string()` will be implemented. So, you
    /// can use the function `to_string()` and the macro `println!()`.
    /// `f` is a buffer, this method must write the formatted string into it.
    /// [Read more](https://doc.rust-lang.org/core/fmt/trait.Display.html#tymethod.fmt)
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)
        write!(f, "{}", self.getHashValue_in_string())
    }
}
