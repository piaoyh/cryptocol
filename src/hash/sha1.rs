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
use std::ptr::copy_nonoverlapping;
use std::slice::from_raw_parts;

use crate::number::IntUnion;
use crate::number::SmallUInt;


#[derive(Debug, Clone)]
pub struct SHA1
{
    hash_code: [IntUnion; 5],
}

impl SHA1
{
    const K: [u32; 4] = [ 0x5a827999, 0x6ed9eba1, 0x8f1bbcdc, 0xca62c1d6 ];
    const H: [u32; 5] = [ 0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476, 0xc3d2e1f0 ];

    pub fn new() -> Self    { SHA1 { hash_code: [IntUnion::new_with(Self::H[0]),
                                                IntUnion::new_with(Self::H[1]),
                                                IntUnion::new_with(Self::H[2]),
                                                IntUnion::new_with(Self::H[3]),
                                                IntUnion::new_with(Self::H[4])] } }

    /// // pub fn digest(&mut self, message: *const u8, length_in_bytes: u64)
    pub fn digest(&mut self, message: *const u8, length_in_bytes: u64)
    {
        self.initialize();
        let length_done = (length_in_bytes >> 6) as usize;
        for i in 0..length_done
            { self.update(unsafe { from_raw_parts(message.add(i << 6) as *const u32, 16) } ); }
        self.finalize(unsafe { message.add(length_done << 6) }, length_in_bytes);
    }

    // pub fn digest_str(&mut self, message: &str)
    #[inline]
    pub fn digest_str(&mut self, message: &str)
    {
        self.digest(message.as_ptr(), message.len() as u64);
    }

    // pub fn digest_string(&mut self, message: &String)
    #[inline]
    pub fn digest_string(&mut self, message: &String)
    {
        self.digest(message.as_ptr(), message.len() as u64);
    }

    // pub fn digest_array<T, const N: usize>(&mut self, message: &[T; N])
    #[inline]
    pub fn digest_array<T, const N: usize>(&mut self, message: &[T; N])
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
    {
        self.digest(message.as_ptr() as *const u8, (N * T::size_in_bytes()) as u64);
    }

    // pub fn digest_vec<T>(&mut self, message: &Vec<T>)
    #[inline]
    pub fn digest_vec<T>(&mut self, message: &Vec<T>)
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
    {
        self.digest(message.as_ptr() as *const u8, (message.len() * T::size_in_bytes()) as u64);
    }

    // pub fn get_HashValue(&self, hashValue: *mut u8, length: usize)
    pub fn get_HashValue(&self, hashValue: *mut u8, length: usize)
    {
        let n_length = if length < (4 * 5) {length} else {4 * 5};
        unsafe { copy_nonoverlapping(self.hash_code.as_ptr() as *const u8, hashValue, n_length); }
    }

    // pub fn get_HashValue_in_string(&self) -> String
    pub fn get_HashValue_in_string(&self) -> String
    {
        let mut txt = String::new();
        for i in 0..5
        {
            let hs = self.hash_code[i];
            for j in 0..4
            {
                let byte = hs.get_ubyte_(3-j);
                txt.push(Self::to_char(byte >> 4));
                txt.push(Self::to_char(byte & 0b1111));
            }
        }
        txt
    }

    // pub fn get_HashValue_in_array(&self) -> [u32; 5]
    pub fn get_HashValue_in_array(&self) -> [u32; 5]
    {
        let mut res = [0_u32; 5];
        for i in 0..5
            { res[i] = self.hash_code[i].get().to_be(); }
        res
    }

    // pub fn get_HashValue_in_vec(&self) -> Vec<u32>
    #[inline]
    pub fn get_HashValue_in_vec(&self) -> Vec<u32>
    {
        let mut res = Vec::new();
        for i in 0..5
            { res.push(self.hash_code[i].get().to_be()); }
        res
    }
    fn initialize(&mut self)
    {
        for i in 0..5_usize
            { self.hash_code[i] = IntUnion::new_with(Self::getH(i)); }
    }

    fn update(&mut self, message: &[u32])
    {
        let mut a = self.hash_code[0].get();
        let mut b = self.hash_code[1].get();
        let mut c = self.hash_code[2].get();
        let mut d = self.hash_code[3].get();
        let mut e = self.hash_code[4].get();

        for i in 0..16_usize
        {
            let f = Self::Ff(b, c, d).wrapping_add(e)
                                .wrapping_add(Self::getK(i))
                                .wrapping_add(message[i].to_be())
                                .wrapping_add(a.rotate_left(5));
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = f;
        }
        for i in 16..20_usize
        {
            let f = Self::Ff(b, c, d).wrapping_add(e)
                                .wrapping_add(Self::getK(i))
                                .wrapping_add(Self::getW(message, i).to_be())
                                .wrapping_add(a.rotate_left(5));
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = f;
        }
        for i in 20..40_usize
        {
            let f = Self::Gg(b, c, d).wrapping_add(e)
                                .wrapping_add(Self::getK(i))
                                .wrapping_add(Self::getW(message, i).to_be())
                                .wrapping_add(a.rotate_left(5));
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = f;
        }
        for i in 40..60_usize
        {
            let f = Self::Hh(b, c, d).wrapping_add(e)
                                .wrapping_add(Self::getK(i))
                                .wrapping_add(Self::getW(message, i).to_be())
                                .wrapping_add(a.rotate_left(5));
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = f;
        }
        for i in 60..80_usize
        {
            let f = Self::Ff(b, c, d).wrapping_add(e)
                                .wrapping_add(Self::getK(i))
                                .wrapping_add(Self::getW(message, i).to_be())
                                .wrapping_add(a.rotate_left(5));
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = f;
        }

        self.hash_code[0].set(self.hash_code[0].get().wrapping_add(a));
        self.hash_code[1].set(self.hash_code[1].get().wrapping_add(b));
        self.hash_code[2].set(self.hash_code[2].get().wrapping_add(c));
        self.hash_code[3].set(self.hash_code[3].get().wrapping_add(d));
        self.hash_code[4].set(self.hash_code[4].get().wrapping_add(e));
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

	#[inline] fn getK(idx: usize) -> u32    { Self::K[idx] }
	#[inline] fn getH(idx: usize) -> u32    { Self::H[idx] }
    #[inline] fn getW(message: &[u32], idx: usize) -> u32   { (message[idx-3] ^ message[idx-8] ^ message[idx-14] ^ message[idx-16]).rotate_left(1) }
	#[inline] fn Ff(x: u32, y: u32, z: u32) -> u32  { (x & y) | (!x & z) }
	#[inline] fn Gg(x: u32, y: u32, z: u32) -> u32	{ x ^ y ^ z }
	#[inline] fn Hh(x: u32, y: u32, z: u32) -> u32  { (x & z) | (y & z) | (z & x) }
    #[inline] fn to_char(nibble: u8) -> char    { if nibble < 10  { ('0' as u8 + nibble) as u8 as char } else { ('A' as u8 - 10 + nibble) as char } }
}


impl Display for SHA1
{
    /// Formats the value using the given formatter.
    /// You will hardly use this method directly.
    /// Automagically the function `to_string()` will be implemented. So, you
    /// can use the function `to_string()`, and you can also print the MD5
    /// object in the macro `println!()` directly for example.
    /// `f` is a buffer, this method must write the formatted string into it.
    /// [Read more](https://doc.rust-lang.org/core/fmt/trait.Display.html#tymethod.fmt)
    /// 
    /// # Example 1 for the method to_string()
    /// ```
    /// use Cryptocol::hash::MD5;
    /// let mut hash = MD5::new();
    /// let txt = "Display::fmt() automagically implement to_string().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.to_string());
    /// assert_eq!(hash.to_string(), "ED085603C2CDE77DD0C6FED3EC1A8ADB");
    /// ```
    /// 
    /// # Example 2 for the use in the macro println!()
    /// ```
    /// use Cryptocol::hash::MD5;
    /// let mut hash = MD5::new();
    /// let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "6C9494A4A5C313001695262D72571F74");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)
        write!(f, "{}", self.get_HashValue_in_string())
    }
}

/*

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
*/