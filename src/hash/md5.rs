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
use std::ptr::copy_nonoverlapping;
use std::mem::{ size_of, size_of_val };
use std::slice::from_raw_parts;
use std::cmp::{ PartialEq, PartialOrd, Ordering };
use std::ops::*;

use crate::number::{IntUnion, LongUnion, LongerUnion, SmallUInt};

#[derive(Debug, Clone)]
pub struct MD5
{
    hash_code: [IntUnion; 4],
}

impl MD5
{
    const K: [u32; 64] = [  0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
                            0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
                            0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
                            0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
                            0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa,
                            0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
                            0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
                            0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
                            0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
                            0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
                            0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05,
                            0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
                            0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
                            0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
                            0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
                            0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391 ];
    const Rot: [[u32; 4]; 4] = [[7, 12, 17, 22], [5,  9, 14, 20], [4, 11, 16, 23], [6, 10, 15, 21]];
    const H: [u32; 4] = [ 0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476 ];

    pub fn new() -> Self    { MD5 { hash_code: [IntUnion::new_with(Self::H[0]),
                                                IntUnion::new_with(Self::H[1]),
                                                IntUnion::new_with(Self::H[2]),
                                                IntUnion::new_with(Self::H[3])] } }

    /// // pub fn digest(&mut self, message: *const u8, length_in_bytes: u64)
    pub fn digest(&mut self, message: *const u8, length_in_bytes: u64)
    {
        self.initialize();
        let length_done = (length_in_bytes >> 6) as usize;
        for i in 0..length_done
            { self.update(unsafe { from_raw_parts(message.add(i << 6) as *const u32, 16) } ); }
        self.finalize(unsafe { message.add(length_done << 6) }, length_in_bytes);
    }

    /// // pub fn digest_str(&mut self, message: &str)
    #[inline]
    pub fn digest_str(&mut self, message: &str)
    {
        self.digest(message.as_ptr(), message.len() as u64);
    }

    /// // pub fn digest_str(&mut self, message: &str)
    #[inline]
    pub fn digest_string(&mut self, message: &String)
    {
        self.digest(message.as_ptr(), message.len() as u64);
    }

    /// // pub fn digest_str(&mut self, message: &str)
    #[inline]
    pub fn digest_array<const N: usize>(&mut self, message: &[u8; N])
    {
        self.digest((message as &[u8]).as_ptr(), N as u64);
    }

    /// // pub fn getHashValue(&self, hashValue: *mut u8, length: usize)
    /// 
    pub fn getHashValue(&self, hashValue: *mut u8, length: usize)
    {
        let n_length = if length < (4 * 4) {length} else {4 * 4};
        for i in 0..n_length
            { unsafe { *hashValue.add(i) = self.hash_code[i/4].get_ubyte_(i%4); } }
    }

    /// // pub fn getHashValue_in_array(&self) -> [u32; 4]
    /// 
    pub fn getHashValue_in_array(&self) -> [u32; 4]
    {
        let mut res = [0_u32; 4];
        for i in 0..4
            { unsafe { res[i] = self.hash_code[i].get().to_le(); } }
        res
    }

    /// // pub fn getHashValue_in_vec(&self) -> Vec<u32>
    /// 
    #[inline]
    pub fn getHashValue_in_vec(&self) -> Vec<u32>
    {
        Vec::<u32>::from(&self.getHashValue_in_array())
    }

    /// // pub fn getHashValue_in_vec(&self) -> Vec<u32>
    /// 
    pub fn getHashValue_in_string(&self) -> String
    {
        let mut txt = String::new();
        for i in 0..4
        {
            let hs = self.hash_code[i];
            for j in 0..4
            {
                let byte = hs.get_ubyte_(j);
                txt.push(Self::to_char(byte >> 4));
                txt.push(Self::to_char(byte & 0b1111));
            }
        }
        txt
    }

    fn initialize(&mut self)
    {
        for i in 0..4_usize
            { self.hash_code[i] = IntUnion::new_with(Self::getH(i)); }
    }

    fn update(&mut self, message: &[u32])
    {
        let mut a = self.hash_code[0].get();
        let mut b = self.hash_code[1].get();
        let mut c = self.hash_code[2].get();
        let mut d = self.hash_code[3].get();

        for i in 0..16_usize
        {
            let f = Self::Ff(b, c, d).wrapping_add(a)
                                .wrapping_add(Self::getK(i))
                                .wrapping_add(message[i].to_le())
                                .rotate_left(Self::Rot[0][i & 0b11]);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f);
        }
        for i in 16..32_usize
        {
            let g = ((i << 2) + i + 1) & 0b1111;
            let f = Self::Gg(b, c, d).wrapping_add(a)
                                .wrapping_add(Self::getK(i))
                                .wrapping_add(message[g].to_le())
                                .rotate_left(Self::Rot[1][i & 0b11]);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f);
        }
        for i in 32..48_usize
        {
            let g = ((i << 1) + i + 5) & 0b1111;
            let f = Self::Hh(b, c, d).wrapping_add(a)
                                .wrapping_add(Self::getK(i))
                                .wrapping_add(message[g].to_le())
                                .rotate_left(Self::Rot[2][i & 0b11]);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f);
        }
        for i in 48..64_usize
        {
            let g = ((i << 3) - i) & 0b1111;
            let f = Self::Ii(b, c, d).wrapping_add(a)
                                .wrapping_add(Self::getK(i))
                                .wrapping_add(message[g].to_le())
                                .rotate_left(Self::Rot[3][i & 0b11]);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f);
        }
        // for i in 0..64_usize
        // {
        //     let (mut f, g) = Self::func(b, c, d, i);
        //     f = f.wrapping_add(a)
        //             .wrapping_add(Self::getK(i))
        //             .wrapping_add(message[g].to_le())
        //             .rotate_left(Self::getRot(i));
        //     a = d;
        //     d = c;
        //     c = b;
        //     b = b.wrapping_add(f);
        // }

        self.hash_code[0].set(self.hash_code[0].get() + a);
        self.hash_code[1].set(self.hash_code[1].get() + b);
        self.hash_code[2].set(self.hash_code[2].get() + c);
        self.hash_code[3].set(self.hash_code[3].get() + d);
    }
    
    fn finalize(&mut self, message: *const u8, length_in_bytes: u64)
    {
        union MU
        {
            lu: [u64; 8],
            iu: [u32; 16],
            txt: [u8; 64],
        }

        let mut mu = MU { txt: [0; 64] };
        let last_bytes = (length_in_bytes & 0b11_1111) as usize;    // equivalent to (length_in_bytes % 64) as usize
        unsafe { copy_nonoverlapping(message, mu.txt.as_mut_ptr(), last_bytes); }
        unsafe { mu.txt[last_bytes] = 0b1000_0000; }
        // 데이터 기록후, 데이터의 길이를 비트 단위로 기록하기 위한 64 비트(8 바이트)와
        // 0b1000_0000를 기록하기 위한 한 바이트의 여유공간이 남아있지 않으면,
        if last_bytes > 54  // (64 - 8 - 1)
        {
            self.update(unsafe {&mu.iu});
            for i in 0..7
                { unsafe { mu.lu[i] = 0; } }
        }
        unsafe { mu.lu[7] = (length_in_bytes << 3).to_le(); }    // 데이터 길이의 단위는 바이트가 아니라 비트이다.
        self.update(unsafe {&mu.iu});
    }

    fn func(x: u32, y: u32, z: u32, round: usize) -> (u32, usize)
    {
        if round < 16
            { (Self::Ff(x, y, z), round) }
        else if round < 32
            { (Self::Gg(x, y, z), ((round << 2) + round + 1) & 0b1111) }    // same as ((5 * round) + 1) % 16
        else if round < 48
            { (Self::Hh(x, y, z), ((round << 1) + round + 5) & 0b1111) }    // same as ((3 * round) + 5) % 16
        else
            { (Self::Ii(x, y, z), ((round << 3) - round) & 0b1111) }        // same as (7 * round) % 16
    }

	#[inline] fn getK(idx: usize) -> u32    { Self::K[idx] }
	#[inline] fn getH(idx: usize) -> u32    { Self::H[idx] }
    #[inline] fn getRot(idx: usize) -> u32  { Self::Rot[idx >> 4][idx & 0b11] }
	#[inline] fn Ff(x: u32, y: u32, z: u32) -> u32  { (x & y) | (!x & z) }
	#[inline] fn Gg(x: u32, y: u32, z: u32) -> u32  { (x & z) | (y & !z) }
	#[inline] fn Hh(x: u32, y: u32, z: u32) -> u32	{ x ^ y ^ z }
    #[inline] fn Ii(x: u32, y: u32, z: u32) -> u32	{ y ^ (x | !z) }

    #[inline] fn to_char(nibble: u8) -> char    { if nibble < 10  { ('0' as u8 + nibble) as u8 as char } else { ('A' as u8 - 10 + nibble) as char } }
}


impl Display for MD5
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
