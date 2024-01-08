// Copyright 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module that contains a MD4 hash algorithm.

#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]


use std::ptr::copy_nonoverlapping;
use std::slice::from_raw_parts;
use std::fmt::{ self, Debug, Display, Formatter };

use crate::number::{ SmallUInt, IntUnion, LongUnion };


/// K0 ~ K63 are initialized with array of round constants: the first 32 bits
/// of the fractional parts of the cube roots of the first 64 primes 2..311
#[allow(non_camel_case_types)]
pub type MD4_Generic_KR_fixed<const H0: u32, const H1: u32, const H2: u32,
                            const H3: u32, const ROUND: usize, const N: usize>
    // Initialize array of round constants: the first 32 bits of
    // the fractional parts of the cube roots of the first 64 primes 2..311
    = MD4_Generic<0x00000000, 0x5A827999, 0x6ED9EBA1, H0, H1, H2, H3,
                    3, 7, 11, 19, 3, 5, 9, 13, 3, 9, 11, 15, ROUND, N>;

/// K0 ~ K63 are initialized with array of round constants: the first 32 bits
/// of the fractional parts of the cube roots of the first 64 primes 2..311
#[allow(non_camel_case_types)]
pub type MD4_Generic_HR_fixed<const K0: u32, const K1: u32, const K2: u32,
                            const ROUND: usize, const N: usize>
    // Initialize array of round constants: the first 32 bits of
    // the fractional parts of the cube roots of the first 64 primes 2..311
    = MD4_Generic<K0, K1, K2, 0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476,
                    3, 7, 11, 19, 3, 5, 9, 13, 3, 9, 11, 15, ROUND, N>;

/// H0 ~ H7 are the first 32 bits of the fractional parts of the square roots
/// of the first 8 primes 2..19
#[allow(non_camel_case_types)]
pub type MD4_Expended<const ROUND: usize, const N: usize>
                        // H0 ~ H7 are the first 32 bits of the fractional
                        // parts of the square roots of the first 8 primes 2..19
    = MD4_Generic_KR_fixed<0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476, ROUND, N>;

#[allow(non_camel_case_types)]
pub type MD4 = MD4_Expended<48, 4>;

/// A MD4 message-digest algorithm that lossily compresses data of arbitrary
/// length into a 128-bit hash value, and its flexible variants that allows
/// you to develop your own `MD4`-based hash algorithms
/// 
/// # Introduction
/// MD4 was designed by Ronald Rivest who is one of the inventors of RSA
/// asymmetric cryptographic algorithm. MD4 was invented in 1991 to replace
/// an earlier hash function MD4. It was specified in 1992 as RFC 1321.
/// This module provides not only the official MD4 but
/// also its expanded versions which is implemented with the name
/// `MD4_Generic`.
/// 
/// # Vulnerability
/// In 2004, it was shown that MD4 is not collision-resistant. Today, MD4 is
/// not recommended for serious cryptographic purposes anymore. So, NIST also
/// does not include MD4 in their list of recommended hashes for password
/// storage. __DO NOT USE MD4 FOR SERIOUS CRYPTOGRAPHIC PURPOSES AT ALL!__
/// If you need to use a hash algorithm for serious cryptographic purposes,
/// you are highly recommended to use SHA-3 hash algorithm instead of MD4,
/// for example.
/// 
/// # Usage of HD5
/// Though MD4 is lack of cryptographic security, MD4 is still widely used
/// for non-cryptograpic purposes such as:
/// - Generating small number of IDs
/// - Integrity test in some collision-free situations
/// - Storing passwords with limited security
/// - Digital Signature
/// - Study purposes
/// 
/// You can create your own expanded version of MD5 by changing the constants
/// K00 ~ K63, the initial hash values H0 ~ H7, the amount of
/// rotate left R00 ~ R33, the number of round ROUND, and the output amount.
/// Your own algrorithm based on MD5 may be stronger or weaker than official
/// MD5. Unless you seriously checked the cryptographic security of your own
/// algorithms, it is hard to assume that your own alogrithms are stronger
/// than the official MD4.
/// 
/// # Reference
/// Read [more](https://en.wikipedia.org/wiki/MD4) about MD4 in detail.
/// 
/// # Quick Start
/// In order to use the module MD4, the module Cryptocol::hash::MD4 is
/// re-exported so that you don't have to import (or use)
/// Cryptocol::hash::MD4 directly. You only import MD4 struct in the module
/// Cryptocol::hash. Example 1 shows how to import MD4 struct.
/// 
/// ## Example 1
/// ```
/// use Cryptocol::hash::MD4;
/// ```
/// Then, you create MD4 object by the method MD4::new(). Now, you are ready to
/// use all prepared methods to hash any data. If you want to hash a string,
/// for example, you can use the method digest_str(). Then, the MD4 object that
/// you created will contain its hash value. You can use the macro println!()
/// for instance to print on a commandline screen by `println!("{}", hash)`
/// where hash is the MD4 object. Example 2 shows how to use MD4 struct quickly.
/// 
/// ## Example 2
/// ```
/// use std::string::*;
/// use Cryptocol::hash::MD4;
/// let mut hash = MD4::new();
/// 
/// let mut txt = "";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.get_HashValue_in_string(), "D41D8CD98F00B204E9800998ECF8427E");
/// 
/// let txtStirng = String::from("A");
/// hash.digest_string(&txtStirng);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txtStirng, hash);
/// assert_eq!(hash.to_string(), "7FC56270E7A70FA81A5935B72EACBE29");
/// 
/// let txtArray = ['W' as u8, 'o' as u8, 'w' as u8];
/// hash.digest_array(&txtArray);
/// println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txtArray, hash);
/// assert_eq!(hash.get_HashValue_in_string(), "49DC5E45FBEC1433E2C612E5AA809C10");
/// 
/// txt = "This data is 26-byte long.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.to_string(), "17ED1DB5CD96184041659D84BB36D76B");
/// 
/// txt = "The unit of data length is not byte but bit.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.get_HashValue_in_string(), "C3EB6D4A1071E1A9C5E08FEF6E8F3FBF");
/// 
/// txt = "I am testing MD4 for the data whose length is sixty-two bytes.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.to_string(), "6C33614E6317DC4641573E0EBC287F98");
/// 
/// let mut txt = "I am testing MD4 for the data whose length is sixty-four bytes..";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.get_HashValue_in_string(), "200F9A19EA45A830284342114483172B");
/// 
/// txt = "I am testing MD4 for the case data whose length is more than sixty-four bytes is given.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
/// assert_eq!(hash.to_string(), "9831162AB272AE1D85245B75726D215E");
/// ```
/// 
/// # Big-endian issue
/// It is just experimental for Big Endian CPUs. So, you are not encouraged
/// to use it for Big Endian CPUs for serious purpose. Only use this crate
/// for Big-endian CPUs with your own full responsibility.
#[derive(Debug, Clone)]
pub struct MD4_Generic<const K0: u32, const K1: u32, const K2: u32,
                    const H0: u32, const H1: u32, const H2: u32, const H3: u32,
                    const R00: u32, const R01: u32, const R02: u32, const R03: u32,
                    const R10: u32, const R11: u32, const R12: u32, const R13: u32,
                    const R20: u32, const R21: u32, const R22: u32, const R23: u32,
                    const ROUND: usize, const N: usize>
{
    hash_code: [IntUnion; 4],
}

impl<const K0: u32, const K1: u32, const K2: u32,
    const H0: u32, const H1: u32, const H2: u32, const H3: u32,
    const R00: u32, const R01: u32, const R02: u32, const R03: u32,
    const R10: u32, const R11: u32, const R12: u32, const R13: u32,
    const R20: u32, const R21: u32, const R22: u32, const R23: u32,
    const ROUND: usize, const N: usize>
MD4_Generic<K0, K1, K2, H0, H1, H2, H3, 
            R00, R01, R02, R03, R10, R11, R12, R13, R20, R21, R22, R23, ROUND, N>
{
    const K: [u32; 3] = [ K0, K1, K2 ];
    const R: [[u32; 4]; 3] = [  [R00, R01, R02, R03],
                                [R10, R11, R12, R13],
                                [R20, R21, R22, R23] ];
    const H: [u32; 4] = [ H0, H1, H2, H3 ];


    // pub fn new() -> Self
    /// Constructs a new `MD4` object.
    /// 
    /// # Output
    /// A new object of `MD4`.
    /// 
    /// # Initialization
    /// All the attributes of the constructed object, which is initial hash
    /// value, will be initialized with `0123456789ABCDEFFEDCBA9876543210`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::hash::MD4;
    /// let mut hash = MD4::new();
    /// println!("Hash =\t{}", hash);
    /// assert_eq!(hash.to_string(), "0123456789ABCDEFFEDCBA9876543210");
    /// ```
    pub fn new() -> Self
    {
        MD4_Generic
        {
            hash_code: [IntUnion::new_with(Self::H[0]),
                        IntUnion::new_with(Self::H[1]),
                        IntUnion::new_with(Self::H[2]),
                        IntUnion::new_with(Self::H[3])]
        }
    }

    // pub fn digest(&mut self, message: *const u8, length_in_bytes: u64)
    /// Compute hash value.
    /// 
    /// # Features
    /// This function has the generalized interface (pointer, `*const u8`)
    /// so as to enable other functions to wrap this function with any
    /// convenient interface for uses. So, this function is usually not called
    /// directly in Rust. This function is provided to be called from other
    /// programming languages such as C/C++.
    /// 
    /// # Arguments
    /// - `message` is pointer to const u8.
    /// - `length_in_bytes` is the size of message in the unit of bytes, and
    /// data type is `u64`.
    /// 
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of a string slice,
    /// you are highly recommended to use the method
    /// [digest_str()](struct@MD4#method.digest_str)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of String
    /// object, you are highly recommended to use the method
    /// [digest_string()](struct@MD4#method.digest_string)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    /// object, you are highly recommended to use the method
    /// [digest_array()](struct@MD4#method.digest_array)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    /// object, you are highly recommended to use the method
    /// [digest_vec()](struct@MD4#method.digest_array)
    /// rather than this method.
    ///
    /// # Example
    /// ```
    /// use Cryptocol::hash::MD4;
    /// let txt = "This is an example of the method digest().";
    /// let mut hash = MD4::new();
    /// hash.digest(txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "336EA91DD3216BD0FC841E86F9E722D8");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn digest(&mut self, message: *const u8, length_in_bytes: u64)
    {
        type MessageType = u32;
        const SHIFT_NUM: usize = 6;
        const CHUNK_NUM: usize = 16;
        self.initialize();
        let length_done = (length_in_bytes >> SHIFT_NUM) as usize;
        for i in 0..length_done
            { self.update(unsafe { from_raw_parts(message.add(i << SHIFT_NUM) as *const MessageType, CHUNK_NUM) } ); }
        self.finalize(unsafe { message.add(length_done << SHIFT_NUM) }, length_in_bytes);
    }

    /// // pub fn digest_str(&mut self, message: &str)
    /// Compute hash value.
    /// 
    /// # Features
    /// This function is a wrapping function of `digest()`.
    /// This function computes hash value of the content of string slice.
    /// 
    /// # Argument
    /// - message is `&str`.
    /// 
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of the content of String
    /// object, you are highly recommended to use the method
    /// [digest_string()](struct@MD4#method.digest_string)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    /// object, you are highly recommended to use the method
    /// [digest_array()](struct@MD4#method.digest_array)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    /// object, you are highly recommended to use the method
    /// [digest_vec()](struct@MD4#method.digest_array)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [digest()](struct@MD4#method.digest) rather than this method.
    ///
    /// # Example
    /// ```
    /// use Cryptocol::hash::MD4;
    /// let txt = "This is an example of the method digest_str().";
    /// let mut hash = MD4::new();
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "F2E455CEB5FB993A980E67D3FA8A3961");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn digest_str(&mut self, message: &str)
    {
        self.digest(message.as_ptr(), message.len() as u64);
    }

    // pub fn digest_string(&mut self, message: &String)
    /// Compute hash value.
    /// 
    /// # Features
    /// This function is a wrapping function of `digest()`.
    /// This function computes hash value of the content of String object.
    /// 
    /// # Argument
    /// - message is `&String`.
    /// 
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of a string slice,
    /// you are highly recommended to use the method
    /// [digest_str()](struct@MD4#method.digest_str)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    /// object, you are highly recommended to use the method
    /// [digest_array()](struct@MD4#method.digest_array)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    /// object, you are highly recommended to use the method
    /// [digest_vec()](struct@MD4#method.digest_array)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [digest()](struct@MD4#method.digest) rather than this method.
    ///
    /// # Example
    /// ```
    /// use Cryptocol::hash::MD4;
    /// let txt = "This is an example of the method digest_string().".to_string();
    /// let mut hash = MD4::new();
    /// hash.digest_string(&txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "40929E789D2F5880B85456E289F704C0");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn digest_string(&mut self, message: &String)
    {
        self.digest(message.as_ptr(), message.len() as u64);
    }

    // pub fn digest_array<const N: usize>(&mut self, message: &[T; N])
    /// Compute hash value.
    /// 
    /// # Features
    /// This function is a wrapping function of `digest()`.
    /// This function computes hash value of the content of Array object.
    /// 
    /// # Argument
    /// - message is `&[T; N]`.
    /// 
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of a string slice,
    /// you are highly recommended to use the method
    /// [digest_str()](struct@MD4#method.digest_str)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of String
    /// object, you are highly recommended to use the method
    /// [digest_string()](struct@MD4#method.digest_string)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    /// object, you are highly recommended to use the method
    /// [digest_vec()](struct@MD4#method.digest_array)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [digest()](struct@MD4#method.digest) rather than this method.
    ///
    /// # Example
    /// ```
    /// use Cryptocol::hash::MD4;
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// let mut hash = MD4::new();
    /// hash.digest_array(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "054DE9CF5F9EA623BBB8DC4781685A58");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn digest_array<T, const M: usize>(&mut self, message: &[T; M])
    where T: SmallUInt + Copy + Clone
    {
        self.digest(message.as_ptr() as *const u8, (M * T::size_in_bytes()) as u64);
    }

    // pub fn digest_vec<T>(&mut self, message: &Vec<T>)
    /// Compute hash value.
    /// 
    /// # Features
    /// This function is a wrapping function of `digest()`.
    /// This function computes hash value of the content of Vec object.
    /// 
    /// # Argument
    /// - message is `&Vec<T>`.
    /// 
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of a string slice,
    /// you are highly recommended to use the method
    /// [digest_str()](struct@MD4#method.digest_str)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of String
    /// object, you are highly recommended to use the method
    /// [digest_string()](struct@MD4#method.digest_string)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    /// object, you are highly recommended to use the method
    /// [digest_array()](struct@MD4#method.digest_array)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [digest()](struct@MD4#method.digest) rather than this method.
    ///
    /// # Example
    /// ```
    /// use Cryptocol::hash::MD4;
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// let mut hash = MD4::new();
    /// hash.digest_vec(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "054DE9CF5F9EA623BBB8DC4781685A58");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn digest_vec<T>(&mut self, message: &Vec<T>)
    where T: SmallUInt + Copy + Clone
    {
        self.digest(message.as_ptr() as *const u8, (message.len() * T::size_in_bytes()) as u64);
    }

    // pub fn get_HashValue(&self, hashValue: *mut u8, length: usize)
    /// Gives a hash value to the place where `hashValue` points to.
    /// 
    /// # Features
    /// This function has the generalized interface (pointer, `*mut u8`)
    /// so as to enable other functions to wrap this function with any
    /// convenient interface for uses. So, this function is usually not called
    /// directly in Rust. This function is provided to be called from other
    /// programming languages such as C/C++.
    /// 
    /// # Arguments
    /// - `hashValue` is the pointer to the place to hold the result hash value.
    /// - `length` is the size of the place that `hashValue` points to. 
    /// 
    /// # Counterpart Methods
    /// - If you want to get the hash value in the form of String object,
    /// you are highly recommended to use the method
    /// [get_HashValue_string()](struct@MD4#method.get_HashValue_string)
    /// rather than this method.
    /// - If you want to get the hash value in the form of array object,
    /// you are highly recommended to use the method
    /// [get_HashValue_in_array()](struct@MD4#method.get_HashValue_in_array)
    /// rather than this method.
    /// - If you want to get the hash value in the form of Vec object,
    /// you are highly recommended to use the method
    /// [get_HashValue_in_vec()](struct@MD4#method.get_HashValue_in_vec)
    /// rather than this method.
    ///
    /// # Example
    /// ```
    /// use Cryptocol::hash::MD4;
    /// let txt = "This is an example of the method get_HashValue().";
    /// let mut hashValue = [0_u8; 16];
    /// let mut hash = MD4::new();
    /// hash.digest_str(txt);
    /// hash.get_HashValue(hashValue.as_ptr() as *mut u8, hashValue.len());
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hashValue);
    /// assert_eq!(format!("{:02X?}", hashValue), "[D9, FB, 90, AB, DD, 2E, 1E, 48, D8, 5E, E5, 08, 4B, AE, 2C, 39]");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn get_HashValue(&self, hashValue: *mut u8, length: usize)
    {
        const BYTES: usize = 4;
        let n_length = if length < (BYTES * N) {length} else {BYTES * N};
        #[cfg(target_endian = "little")]   // Because of MD4 is based on Little Endian
        unsafe { copy_nonoverlapping(self.hash_code.as_ptr() as *const u8, hashValue, n_length); }
        #[cfg(target_endian = "big")]   // Because of MD4 is based on Little Endian
        {
            let mut hash_code = [IntUnion::new(); N];
            for i in 0..N
                { hash_code[i].set(self.hash_code[i].get().to_le()); }
            unsafe { copy_nonoverlapping(hash_code.as_ptr() as *const u8, hashValue, n_length); }
        }
    }


    // pub fn get_HashValue_in_string(&self) -> String
    /// Returns a hash value in the form of String object.
    /// 
    /// # Counterpart Methods
    /// - If you want to get the hash value in the form of array object,
    /// you are highly recommended to use the method
    /// [get_HashValue_in_array()](struct@MD4#method.get_HashValue_in_array)
    /// rather than this method.
    /// - If you want to get the hash value in the form of Vec object,
    /// you are highly recommended to use the method
    /// [get_HashValue_in_vec()](struct@MD4#method.get_HashValue_in_vec)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [get_HashValue()](struct@MD4#method.get_HashValue)
    /// rather than this method.
    ///
    /// # Example
    /// ```
    /// use Cryptocol::hash::MD4;
    /// let txt = "This is an example of the method get_HashValue_in_string().";
    /// let mut hash = MD4::new();
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_HashValue_in_string());
    /// assert_eq!(hash.get_HashValue_in_string(), "7BB1ED16E2E302AA3B16CD24EC3E3093");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn get_HashValue_in_string(&self) -> String
    {
        const BYTES: usize = 4;
        let mut txt = String::new();
        for i in 0..N
        {
            let hs = self.hash_code[i];
            for j in 0..BYTES
            {
                let byte = hs.get_ubyte_(j);    // Because of MD4 is based on Little Endian
                txt.push(Self::to_char(byte >> 4));
                txt.push(Self::to_char(byte & 0b1111));
            }
        }
        txt
    }

    // pub fn get_HashValue_in_array(&self) -> [u32; 4]
    /// Returns a hash value in the form of array object.
    /// 
    /// # Counterpart Methods
    /// - If you want to get the hash value in the form of String object,
    /// you are highly recommended to use the method
    /// [get_HashValue_string()](struct@MD4#method.get_HashValue_string)
    /// rather than this method.
    /// - If you want to get the hash value in the form of Vec object,
    /// you are highly recommended to use the method
    /// [get_HashValue_in_vec()](struct@MD4#method.get_HashValue_in_vec)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [get_HashValue()](struct@MD4#method.get_HashValue)
    /// rather than this method.
    ///
    /// # Example
    /// ```
    /// use Cryptocol::hash::MD4;
    /// let txt = "This is an example of the method get_HashValue_in_array().";
    /// let mut hash = MD4::new();
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_HashValue_in_array());
    /// assert_eq!(format!("{:02X?}", hash.get_HashValue_in_array()), "[A4BE6EEF, C9A5DFBA, 558B5ADF, 3B1035F9]");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn get_HashValue_in_array(&self) -> [u32; N]
    {
        let mut res = [0_u32; N];
        for i in 0..N
            { res[i] = self.hash_code[i].get().to_le(); }
        res
    }

    // pub fn getHashValue_in_vec(&self) -> Vec
    /// Returns a hash value in the form of Vec object.
    /// 
    /// # Counterpart Methods
    /// - If you want to get the hash value in the form of String object,
    /// you are highly recommended to use the method
    /// [get_HashValue_string()](struct@MD4#method.get_HashValue_string)
    /// rather than this method.
    /// - If you want to get the hash value in the form of array object,
    /// you are highly recommended to use the method
    /// [get_HashValue_in_array()](struct@MD4#method.get_HashValue_in_array)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [get_HashValue()](struct@MD4#method.get_HashValue)
    /// rather than this method.
    ///
    /// # Example
    /// ```
    /// use Cryptocol::hash::MD4;
    /// let txt = "This is an example of the method get_HashValue_in_vec().";
    /// let mut hash = MD4::new();
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_HashValue_in_vec());
    /// assert_eq!(format!("{:02X?}", hash.get_HashValue_in_vec()), "[C24C5F26, D87BBAC8, D66148F4, 4D7DE209]");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn get_HashValue_in_vec(&self) -> Vec<u32>
    {
        let mut res = Vec::new();
        for i in 0..N
            { res.push(self.hash_code[i].get().to_le()); }
        res
    }

    #[inline]
    pub fn tangle(&mut self, tangling: u64)
    {
        let common = LongUnion::new_with(tangling);
        let mut m = [0_u32; 6];
        for i in 0..4
            { m[i] = self.hash_code[i].get(); }
        m[4] = common.get_uint_(0);
        m[5] = common.get_uint_(1);
        self.finalize(m.as_ptr() as *const u8, 24);
    }

    // fn initialize(&mut self)
    /// Initializes all four self.hash_code[] with predetermined values H[].
    fn initialize(&mut self)
    {
        for i in 0..4_usize
            { self.hash_code[i] = IntUnion::new_with(Self::H[i]); }
    }

    // fn update(&mut self, message: &[u32])
    /// This method is the core part of MD4 hash algorithm.
    /// It has sixty-four rounds. It updates self.hash_code[] for those
    /// sixty-four rounds.
    fn update(&mut self, message: &[u32])
    {
        let mut a = self.hash_code[0].get();
        let mut b = self.hash_code[1].get();
        let mut c = self.hash_code[2].get();
        let mut d = self.hash_code[3].get();

        for i in 0..ROUND
        {
            let (mut f, g) = Self::func(b, c, d, i);
            f = f.wrapping_add(a)
                    .wrapping_add(Self::getK(i))
                    .wrapping_add(message[g].to_le())
                    .rotate_left(Self::getR(i));
            a = d;
            d = c;
            c = b;
            b = f;
        }

        self.hash_code[0].set(self.hash_code[0].get().wrapping_add(a));
        self.hash_code[1].set(self.hash_code[1].get().wrapping_add(b));
        self.hash_code[2].set(self.hash_code[2].get().wrapping_add(c));
        self.hash_code[3].set(self.hash_code[3].get().wrapping_add(d));
    }

    // fn finalize(&mut self, message: *const u8, length_in_bytes: u64)
    /// finalizes the hash process. In this process, it pads with padding bits,
    /// which are bit one, bits zeros, and eight bytes that show the message
    /// size in the unit of bits with little endianness so as to make the data
    /// (message + padding bits) be multiples of 512 bits (64 bytes).
    fn finalize(&mut self, message: *const u8, length_in_bytes: u64)
    {
        type ChunkType = u64;
        type PieceType = u32;
        const MESSAGE_NUM: usize = 64;
        const LAST_BYTES: ChunkType = 0b11_1111;
        union MU
        {
            chunk: [ChunkType; 8],
            piece: [PieceType; 16],
            txt: [u8; MESSAGE_NUM],
        }

        let mut mu = MU { txt: [0; 64] };
        let last_bytes = (length_in_bytes & LAST_BYTES) as usize;    // equivalent to (length_in_bytes % 64) as usize
        unsafe { copy_nonoverlapping(message, mu.txt.as_mut_ptr(), last_bytes); }
        unsafe { mu.txt[last_bytes] = 0b1000_0000; }
        // 데이터 기록후, 데이터의 길이를 비트 단위로 기록하기 위한 64 비트(8 바이트)와
        // 0b1000_0000를 기록하기 위한 한 바이트의 여유공간이 남아있지 않으면,
        if last_bytes > 54  // (64 - 8 - 1)
        {
            self.update(unsafe {&mu.piece});
            for i in 0..7
                { unsafe { mu.chunk[i] = 0; } }
        }
        unsafe { mu.chunk[7] = (length_in_bytes << 3).to_le(); }    // 데이터 길이의 단위는 바이트가 아니라 비트이다.
        self.update(unsafe {&mu.piece});
    }

    fn func(x: u32, y: u32, z: u32, round: usize) -> (u32, usize)
    {
        let r = round % 48;
        if r < 16
            { (Self::Ff(x, y, z), r & 0b1111) }     // equivalent to r % 16
        else if r < 32
            { (Self::Gg(x, y, z), (((r << 4) + r - 16) >> 2) & 0b1111) }    // equivalent to (17 * r - 16) / 4 % 16
        else
            { (Self::Hh(x, y, z), ((r & 0b1_1111) * 8 + (((r & 0b1_1111) >> 1) << 2) + ((r & 0b1_1111) >> 2) * 10 + ((r & 0b1_1111) >> 3) * 13) & 0b1111) }    // equivalent to ((r % 32) * 8 + ((r % 32) / 2) * 4 + ((r % 32) / 4) * 10 + ((r % 32) / 8) * 13) % 16
    }

	#[inline] fn getK(idx: usize) -> u32    { Self::K[(idx >> 4) % 3] }
    #[inline] fn getR(idx: usize) -> u32    { Self::R[(idx >> 4) % 3][idx & 0b11] }   // equivalent to Self::R[(idx / 16) % 3][idx % 4]
    #[inline] fn Ff(x: u32, y: u32, z: u32) -> u32  { z ^ (x & (y ^ z)) }   // equivalent to { (x & y) | (!x & z) }
	#[inline] fn Gg(x: u32, y: u32, z: u32) -> u32  { (x & y) | (y & z) | (z & x)}
	#[inline] fn Hh(x: u32, y: u32, z: u32) -> u32	{ x ^ y ^ z }
    #[inline] fn to_char(nibble: u8) -> char    { if nibble < 10  { ('0' as u8 + nibble) as u8 as char } else { ('A' as u8 - 10 + nibble) as char } }
}


impl<const K0: u32, const K1: u32, const K2: u32,
    const H0: u32, const H1: u32, const H2: u32, const H3: u32,
    const R00: u32, const R01: u32, const R02: u32, const R03: u32,
    const R10: u32, const R11: u32, const R12: u32, const R13: u32,
    const R20: u32, const R21: u32, const R22: u32, const R23: u32,
    const ROUND: usize, const N: usize>
Display for MD4_Generic<K0, K1, K2, H0, H1, H2, H3, R00, R01, R02, R03,
                        R10, R11, R12, R13, R20, R21, R22, R23, ROUND, N>
{
    /// Formats the value using the given formatter.
    /// You will hardly use this method directly.
    /// Automagically the function `to_string()` will be implemented. So, you
    /// can use the function `to_string()`, and you can also print the MD4
    /// object in the macro `println!()` directly for example.
    /// `f` is a buffer, this method must write the formatted string into it.
    /// [Read more](https://doc.rust-lang.org/core/fmt/trait.Display.html#tymethod.fmt)
    /// 
    /// # Example 1 for the method to_string()
    /// ```
    /// use Cryptocol::hash::MD4;
    /// let mut hash = MD4::new();
    /// let txt = "Display::fmt() automagically implement to_string().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.to_string());
    /// assert_eq!(hash.to_string(), "ED085603C2CDE77DD0C6FED3EC1A8ADB");
    /// ```
    /// 
    /// # Example 2 for the use in the macro println!()
    /// ```
    /// use Cryptocol::hash::MD4;
    /// let mut hash = MD4::new();
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
