// Copyright 2023, 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module that contains SHA1 hash algorithm

#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

use std::ptr::copy_nonoverlapping;
use std::slice::from_raw_parts;
use std::fmt::{ self, Debug, Display, Formatter };

use crate::number::{ SmallUInt, IntUnion };

/// K0 ~ K63 are initialized with array of round constants: the first 32 bits
/// of the fractional parts of the cube roots of the first 64 primes 2..311
#[allow(non_camel_case_types)]
pub type SHA1_Generic_K_fixed<  const H0: u32, const H1: u32, const H2: u32,
                                const H3: u32, const H4: u32, const RL1: u32,
                                const ROUND: usize, const N: usize>
    // Initialize array of round constants: the first 32 bits of
    // the fractional parts of the cube roots of the first 64 primes 2..311
    = SHA1_generic< 0x5a827999, 0x6ed9eba1, 0x8f1bbcdc, 0xca62c1d6,
                    H0, H1, H2, H3, H4, RL1, 5, 30, ROUND, N>;

#[allow(non_camel_case_types)]
pub type SHA1_Expanded<const ROUND: usize, const N: usize>
        = SHA1_Generic_K_fixed< 0x67452301, 0xefcdab89, 0x98badcfe,
                                0x10325476, 0xc3d2e1f0, 1, ROUND, N>;

#[allow(non_camel_case_types)]
pub type SHA0_Expanded<const ROUND: usize, const N: usize>
        = SHA1_Generic_K_fixed< 0x67452301, 0xefcdab89, 0x98badcfe,
                                0x10325476, 0xc3d2e1f0, 0, ROUND, N>;

pub type SHA1 = SHA1_Expanded<80, 5>;

pub type SHA0 = SHA0_Expanded<80, 5>;


/// A SHA-1 message-digest algorithm that lossily compresses data of arbitrary
/// length into a 160-bit hash value, and its flexible variants that allows
/// you to develop your own `SHA-1`-based hash algorithms
/// 
/// # Introduction
/// SHA-1 was designed by the United States National Security Agency, and is
/// a U.S. Federal Information Processing Standard. SHA-1 produces a message
/// digest based on principles similar to those used by Ronald L. Rivest of
/// MIT in the design of the MD2, MD4 and MD5 message digest algorithms, but
/// generates a larger hash value (160 bits vs. 128 bits).
/// 
/// # Vulnerability
/// In February 2005, Xiaoyun Wang, Yiqun Lisa Yin, and Hongbo Yu announced
/// an attack to find collisions in the full version of SHA-1. Today, SHA-1
/// is not recommended for serious cryptographic purposes anymore.
/// __DO NOT USE SHA1 FOR SERIOUS CRYPTOGRAPHIC PURPOSES AT ALL!__
/// If you need to use a hash algorithm for serious cryptographic purposes,
/// you are highly recommended to use SHA-3 hash algorithm instead of SHA-1,
/// for example.
/// 
/// # About this struct
/// This struct is implemented not only for SHA-1 but also for SHA-0 and for
/// their expanded versions. So, this struct is implemented as a generic
/// version of SHA-1 hash algorithm. Therefore, you can make your own hash
/// algorithm based on SHA-1 algorithm by changing the paramenters such as
/// K0 ~ K3, H0 ~ H4, RL1, RL5, RL30, ROUND, N. Then, the cryptographic security
/// level of your own hash algoritm may be higher or lower than that of the
/// original SHA-1 hash algorithm.
/// 
/// # Use of SHA-1, SHA-0, and other variations
/// Though SHA-1 and SHA-0 are lack of cryptographic security, SHA-1, SHA-0,
/// and other variations can be still used for non-cryptograpic purposes
/// such as:
/// - Generating small number of IDs
/// - Integrity test in some collision-free situations
/// - Storing passwords with limited security
/// - Digital Signature with limited security
/// - Study of hash algorithms
/// 
/// Read [this article](https://en.wikipedia.org/wiki/SHA-1)
/// and/or watch [this video](https://www.youtube.com/watch?v=JIhZWgJA-9o)
/// to learn SHA-1 more in detail.
/// 
/// # Quick Start
/// In order to use the module SHA-1, SHA-0, and other variations, the module
/// Cryptocol::hash::sha1 is re-exported so that you don't have to import
/// (or use) `Cryptocol::hash::sha1::* directly. You only import SHA1, SHA0,
/// and/or SHA1_generic in the module Cryptocol::hash. Example 1 shows how to
/// import SHA1, SHA0, and/or SHA1_generic.
/// 
/// ## Example 1
/// ```
/// use Cryptocol::hash::SHA1;
/// use Cryptocol::hash::SHA0;
/// use Cryptocol::hash::SHA1_generic;
/// ```
/// Then, you can create SHA1 object by the method SHA1::new() for example.
/// Now, you are ready to use all prepared methods to hash any data. If you
/// want to hash a string, for example, you can use the method digest_str().
/// Then, the SHA1 object that you created will contain its hash value. You can
/// use the macro println!() for instance to print on a commandline screen by
/// `println!("{}", hash)` where hash is the SHA1 object.
/// Example 2 shows how to use SHA1 struct quickly.
/// 
/// ## Example 2 for SHA-1
/// ```
/// use std::string::*;
/// use Cryptocol::hash::SHA1;
/// let mut hash = SHA1::new();
/// 
/// let mut txt = "";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.get_HashValue_in_string());
/// assert_eq!(hash.get_HashValue_in_string(), "DA39A3EE5E6B4B0D3255BFEF95601890AFD80709");
/// 
/// let txtStirng = String::from("A");
/// hash.digest_string(&txtStirng);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txtStirng, hash);
/// assert_eq!(hash.to_string(), "6DCD4CE23D88E2EE9568BA546C007C63D9131C1B");
/// 
/// let txtArray = ['W' as u8, 'o' as u8, 'w' as u8];
/// hash.digest_array(&txtArray);
/// println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txtArray, hash);
/// assert_eq!(hash.get_HashValue_in_string(), "0BBCDBD1616A1D2230100F629649DCF5B7A28B7F");
/// 
/// txt = "This data is 26-byte long.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.to_string(), "B82A61505779F6B3ACA4F5E0D54DA44C17375B49");
/// 
/// txt = "The unit of data length is not byte but bit.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.get_HashValue_in_string(), "C6DC54281357FC16D357E1D730BFC313C585DAEC");
/// 
/// txt = "I am testing SHA1 for the data whose length is sixty-two bytes.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.to_string(), "36CD36337097D764797091E5796B6FF45A9FA79F");
/// 
/// let mut txt = "I am testing SHA-1 for the data whose length is sixty-four bytes.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.get_HashValue_in_string(), "E408F6B82DCDDB5EE6613A759AC1B13D0FA1CEF1");
/// 
/// txt = "I am testing SHA1 for the case data whose length is more than sixty-four bytes is given.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
/// assert_eq!(hash.to_string(), "BB2C79F551B95963ECE49D40F8A92349BF66CAE7");
/// ```
/// 
/// # Big-endian issue
/// It is just experimental for Big Endian CPUs. So, you are not encouraged
/// to use it for Big Endian CPUs for serious purpose. Only use this crate
/// for Big-endian CPUs with your own full responsibility.
#[derive(Debug, Clone)]
pub struct SHA1_generic<const K0: u32, const K1: u32, const K2: u32, const K3: u32,
                        const H0: u32, const H1: u32, const H2: u32, const H3: u32,
                        const H4: u32, const RL1: u32, const RL2: u32, const RL3: u32,
                        const ROUND: usize, const N: usize>
                        // N is the number of output bytes of hash value.
                        // RL is 1 for SHA-1 while RL is 0 for SHA-0.
{
    hash_code: [IntUnion; 5],
}

impl<const K0: u32, const K1: u32, const K2: u32, const K3: u32,
    const H0: u32, const H1: u32, const H2: u32, const H3: u32, const H4: u32,
    const RL1: u32, const RL5: u32, const RL30: u32, const ROUND: usize, const N: usize>
SHA1_generic<K0, K1, K2, K3, H0, H1, H2, H3, H4, RL1, RL5, RL30, ROUND, N>
{
    const K: [u32; 4] = [ K0, K1, K2, K3 ];
    const H: [u32; 5] = [ H0, H1, H2, H3, H4 ];

    // pub fn new() -> Self
    /// Constructs a new `SHA1` object.
    /// 
    /// # Output
    /// A new object of `SHA1`.
    /// 
    /// # Initialization
    /// All the attributes of the constructed object, which is initial hash
    /// value, will be initialized with
    /// `67452301EFCDAB8998BADCFE10325476C3D2E1F0`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::hash::SHA1;
    /// let mut hash = SHA1::new();
    /// println!("Hash =\t{}", hash);
    /// assert_eq!(hash.to_string(), "67452301EFCDAB8998BADCFE10325476C3D2E1F0");
    /// ```
    pub fn new() -> Self
    {
        if N > 5 || N == 0
            { panic!("N cannot be equal to 0 or greater than 5."); }
        Self
        {
            hash_code: [IntUnion::new_with(Self::H[0]),
                        IntUnion::new_with(Self::H[1]),
                        IntUnion::new_with(Self::H[2]),
                        IntUnion::new_with(Self::H[3]),
                        IntUnion::new_with(Self::H[4])]
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
    /// [digest_str()](struct@SHA1#method.digest_str)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of String
    /// object, you are highly recommended to use the method
    /// [digest_string()](struct@SHA1#method.digest_string)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    /// object, you are highly recommended to use the method
    /// [digest_array()](struct@SHA1#method.digest_array)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    /// object, you are highly recommended to use the method
    /// [digest_vec()](struct@SHA1#method.digest_array)
    /// rather than this method.
    ///
    /// # Example
    /// ```
    /// use Cryptocol::hash::SHA1;
    /// let txt = "This is an example of the method digest().";
    /// let mut hash = SHA1::new();
    /// hash.digest(txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "9631162DFDAEAB89821256D4585D66D35CD61FD6");
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
    /// [digest_string()](struct@SHA1#method.digest_string)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    /// object, you are highly recommended to use the method
    /// [digest_array()](struct@SHA1#method.digest_array)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    /// object, you are highly recommended to use the method
    /// [digest_vec()](struct@SHA1#method.digest_array)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [digest()](struct@SHA1#method.digest) rather than this method.
    ///
    /// # Example
    /// ```
    /// use Cryptocol::hash::SHA1;
    /// let txt = "This is an example of the method digest_str().";
    /// let mut hash = SHA1::new();
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "9FDE56BBB5028966CC2E7BDCD0758FE3121407E6");
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
    /// [digest_str()](struct@SHA1#method.digest_str)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    /// object, you are highly recommended to use the method
    /// [digest_array()](struct@SHA1#method.digest_array)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    /// object, you are highly recommended to use the method
    /// [digest_vec()](struct@SHA1#method.digest_array)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [digest()](struct@SHA1#method.digest) rather than this method.
    ///
    /// # Example
    /// ```
    /// use Cryptocol::hash::SHA1;
    /// let txt = "This is an example of the method digest_string().".to_string();
    /// let mut hash = SHA1::new();
    /// hash.digest_string(&txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "FDCDC0EBC9181B881BE1F15FECEBB9D70E4DDAAB");
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
    /// [digest_str()](struct@SHA1#method.digest_str)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of String
    /// object, you are highly recommended to use the method
    /// [digest_string()](struct@SHA1#method.digest_string)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    /// object, you are highly recommended to use the method
    /// [digest_vec()](struct@SHA1#method.digest_array)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [digest()](struct@SHA1#method.digest) rather than this method.
    ///
    /// # Example
    /// ```
    /// use Cryptocol::hash::SHA1;
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// let mut hash = SHA1::new();
    /// hash.digest_array(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "76BC87BAECA7725C948FD1C53766454FDA0867AF");
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
    /// [digest_str()](struct@SHA1#method.digest_str)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of String
    /// object, you are highly recommended to use the method
    /// [digest_string()](struct@SHA1#method.digest_string)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    /// object, you are highly recommended to use the method
    /// [digest_array()](struct@SHA1#method.digest_array)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [digest()](struct@SHA1#method.digest) rather than this method.
    ///
    /// # Example
    /// ```
    /// use Cryptocol::hash::SHA1;
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// let mut hash = SHA1::new();
    /// hash.digest_vec(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "76BC87BAECA7725C948FD1C53766454FDA0867AF");
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
    /// [get_HashValue_string()](struct@SHA1#method.get_HashValue_string)
    /// rather than this method.
    /// - If you want to get the hash value in the form of array object,
    /// you are highly recommended to use the method
    /// [get_HashValue_in_array()](struct@SHA1#method.get_HashValue_in_array)
    /// rather than this method.
    /// - If you want to get the hash value in the form of Vec object,
    /// you are highly recommended to use the method
    /// [get_HashValue_in_vec()](struct@SHA1#method.get_HashValue_in_vec)
    /// rather than this method.
    ///
    /// # Example
    /// ```
    /// use Cryptocol::hash::SHA1;
    /// let txt = "This is an example of the method get_HashValue().";
    /// let mut hashValue = [0_u8; 20];
    /// let mut hash = SHA1::new();
    /// hash.digest_str(txt);
    /// hash.get_HashValue(hashValue.as_ptr() as *mut u8, hashValue.len());
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hashValue);
    /// assert_eq!(format!("{:02X?}", hashValue), "[E9, C6, F4, 3B, 77, AA, 27, A1, 6E, B4, F0, F5, 5B, F3, D8, C7, 3A, EB, 7F, 93]");
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
        #[cfg(target_endian = "little")]
        {
            let mut hash_code = [IntUnion::new(); N];
            for i in 0..N
                { hash_code[i].set(self.hash_code[i].get().to_be()); }
            unsafe { copy_nonoverlapping(hash_code.as_ptr() as *const u8, hashValue, n_length); }
        }
        #[cfg(target_endian = "big")]
        unsafe { copy_nonoverlapping(self.hash_code.as_ptr() as *const u8, hashValue, n_length); }
    }

    // pub fn get_HashValue_in_string(&self) -> String
    /// Returns a hash value in the form of String object.
    /// 
    /// # Counterpart Methods
    /// - If you want to get the hash value in the form of array object,
    /// you are highly recommended to use the method
    /// [get_HashValue_in_array()](struct@SHA1#method.get_HashValue_in_array)
    /// rather than this method.
    /// - If you want to get the hash value in the form of Vec object,
    /// you are highly recommended to use the method
    /// [get_HashValue_in_vec()](struct@SHA1#method.get_HashValue_in_vec)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [get_HashValue()](struct@SHA1#method.get_HashValue)
    /// rather than this method.
    ///
    /// # Example
    /// ```
    /// use Cryptocol::hash::SHA1;
    /// let txt = "This is an example of the method get_HashValue_in_string().";
    /// let mut hash = SHA1::new();
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_HashValue_in_string());
    /// assert_eq!(hash.get_HashValue_in_string(), "899B9673103FCB06B237A5A6A7D04D749EA4BD92");
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
                let byte = hs.get_ubyte_(BYTES-1-j);
                txt.push(Self::to_char(byte >> 4));
                txt.push(Self::to_char(byte & 0b1111));
            }
        }
        txt
    }

    // pub fn get_HashValue_in_array(&self) -> [u32; N]
    /// Returns a hash value in the form of array object.
    /// 
    /// # Counterpart Methods
    /// - If you want to get the hash value in the form of String object,
    /// you are highly recommended to use the method
    /// [get_HashValue_string()](struct@SHA1#method.get_HashValue_string)
    /// rather than this method.
    /// - If you want to get the hash value in the form of Vec object,
    /// you are highly recommended to use the method
    /// [get_HashValue_in_vec()](struct@SHA1#method.get_HashValue_in_vec)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [get_HashValue()](struct@SHA1#method.get_HashValue)
    /// rather than this method.
    ///
    /// # Example
    /// ```
    /// use Cryptocol::hash::SHA1;
    /// let txt = "This is an example of the method get_HashValue_in_array().";
    /// let mut hash = SHA1::new();
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_HashValue_in_array());
    /// assert_eq!(format!("{:02X?}", hash.get_HashValue_in_array()), "[E9840962, 837B21A9, D9321727, 74980B51, 364DD5A2]");
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
            { res[i] = self.hash_code[i].get().to_be(); }
        res
    }

    // pub fn getHashValue_in_vec(&self) -> Vec
    /// Returns a hash value in the form of Vec object.
    /// 
    /// # Counterpart Methods
    /// - If you want to get the hash value in the form of String object,
    /// you are highly recommended to use the method
    /// [get_HashValue_string()](struct@SHA1#method.get_HashValue_string)
    /// rather than this method.
    /// - If you want to get the hash value in the form of array object,
    /// you are highly recommended to use the method
    /// [get_HashValue_in_array()](struct@SHA1#method.get_HashValue_in_array)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [get_HashValue()](struct@SHA1#method.get_HashValue)
    /// rather than this method.
    ///
    /// # Example
    /// ```
    /// use Cryptocol::hash::SHA1;
    /// let txt = "This is an example of the method get_HashValue_in_vec().";
    /// let mut hash = SHA1::new();
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_HashValue_in_vec());
    /// assert_eq!(format!("{:02X?}", hash.get_HashValue_in_vec()), "[96E00128, E1E04E29, F65ABA7B, AD10C0A2, 1BC438DA]");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn get_HashValue_in_vec(&self) -> Vec<u32>
    {
        let mut res = Vec::new();
        for i in 0..N
            { res.push(self.hash_code[i].get().to_be()); }
        res
    }

    #[inline]
    pub fn tangle(&mut self)
    {
        self.finalize(self.hash_code.as_ptr() as *const u8, 32);
    }

    // fn initialize(&mut self)
    /// Initializes all five self.hash_code[] with predetermined values H[].
    fn initialize(&mut self)
    {
        for i in 0..5_usize
            { self.hash_code[i] = IntUnion::new_with(Self::getH(i)); }
    }

    // fn update(&mut self, message: &[u32])
    /// This method is the core part of MD5 hash algorithm.
    /// It has eighty rounds. It updates self.hash_code[] for those
    /// eighty rounds.
    fn update(&mut self, message: &[u32])
    {
        let mut W = [0_u32; 16];
        let mut a = self.hash_code[0].get();
        let mut b = self.hash_code[1].get();
        let mut c = self.hash_code[2].get();
        let mut d = self.hash_code[3].get();
        let mut e = self.hash_code[4].get();

        for i in 0..16_usize
        {
            W[i] = message[i].to_be();
            let f = Self::Ff(b, c, d).wrapping_add(e)
                                .wrapping_add(Self::getK(0))
                                .wrapping_add(W[i])
                                .wrapping_add(a.rotate_left(RL5));
            e = d;
            d = c;
            c = b.rotate_left(RL30);
            b = a;
            a = f;
        }
        for i in 16..ROUND
        {
            let j = i & 0b1111;
            W[j] = Self::getW(&W, i);
            let (mut f, z) = Self::func(b, c, d, i);
            f = f.wrapping_add(e)
                .wrapping_add(Self::getK(z))
                .wrapping_add(W[j])
                .wrapping_add(a.rotate_left(RL5));
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

    // fn finalize(&mut self, message: *const u8, length_in_bytes: u64)
    /// finalizes the hash process. In this process, it pads with padding bits,
    /// which are bit one, bits zeros, and eight bytes that show the message
    /// size in the unit of bits with big endianness so as to make the data
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
        unsafe { mu.chunk[7] = (length_in_bytes << 3).to_be(); }    // 데이터 길이의 단위는 바이트가 아니라 비트이다.
        self.update(unsafe {&mu.piece});
    }

    fn func(x: u32, y: u32, z: u32, round: usize) -> (u32, usize)
    {
        let r = (round / 20) & 0b11;
        match r
        {
            0 => { (Self::Ff(x, y, z), r) }
            2 => { (Self::Hh(x, y, z), r) }
            _ => { (Self::Gg(x, y, z), r) }
        }
    }

	#[inline] fn getK(idx: usize) -> u32    { Self::K[idx] }
	#[inline] fn getH(idx: usize) -> u32    { Self::H[idx] }
    #[inline] fn getW(W: &[u32; 16], idx: usize) -> u32   { (W[(idx-3) & 0b1111] ^ W[(idx-8) & 0b1111] ^ W[(idx-14) & 0b1111] ^ W[(idx-16) & 0b1111]).rotate_left(RL1) }
	#[inline] fn Ff(x: u32, y: u32, z: u32) -> u32  { z ^ (x & (y ^ z)) }   // equivalent to { (x & y) | (!x & z) }
	#[inline] fn Gg(x: u32, y: u32, z: u32) -> u32	{ x ^ y ^ z }
	#[inline] fn Hh(x: u32, y: u32, z: u32) -> u32  { (x & y) | (z & (x | y)) } // equivalent to { (x & y) | (y & z) | (z & x) }
    #[inline] fn to_char(nibble: u8) -> char    { if nibble < 10  { ('0' as u8 + nibble) as u8 as char } else { ('A' as u8 - 10 + nibble) as char } }
}


impl<const K0: u32, const K1: u32, const K2: u32, const K3: u32,
    const H0: u32, const H1: u32, const H2: u32, const H3: u32, const H4: u32,
    const RL1: u32, const RL5: u32, const RL30: u32, const ROUND: usize, const N: usize>
Display for SHA1_generic<K0, K1, K2, K3, H0, H1, H2, H3, H4, RL1, RL5, RL30, ROUND, N>
{
    /// Formats the value using the given formatter.
    /// You will hardly use this method directly.
    /// Automagically the function `to_string()` will be implemented. So, you
    /// can use the function `to_string()`, and you can also print the SHA-1
    /// object in the macro `println!()` directly for example.
    /// `f` is a buffer, this method must write the formatted string into it.
    /// [Read more](https://doc.rust-lang.org/core/fmt/trait.Display.html#tymethod.fmt)
    /// 
    /// # Example 1 for the method to_string()
    /// ```
    /// use Cryptocol::hash::SHA1;
    /// let mut hash = SHA1::new();
    /// let txt = "Display::fmt() automagically implement to_string().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.to_string());
    /// assert_eq!(hash.to_string(), "8D0A6284BBFF4DE8D68962A924842C80959B0404");
    /// ```
    /// 
    /// # Example 2 for the use in the macro println!()
    /// ```
    /// use Cryptocol::hash::SHA1;
    /// let mut hash = SHA1::new();
    /// let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "835CEFA297628E4DADBDA011C5FDEA68D88A8EE8");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)
        write!(f, "{}", self.get_HashValue_in_string())
    }
}
