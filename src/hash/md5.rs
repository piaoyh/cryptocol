// Copyright 2023 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module that contains a MD5 hash algorithm.

#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

use std::fmt::{ self, Debug, Display, Formatter };
use std::ptr::copy_nonoverlapping;
use std::slice::from_raw_parts;

use crate::number::IntUnion;
use crate::number::SmallUInt;


/// # Introduction
/// A MD5 message-digest algorithm that lossily compresses data of arbitrary
/// length into a 128-bit hash value. MD5 was designed by Ronald Rivest who
/// is one of the inventors of RSA asymmetric cryptographic algorithm. MD5 was
/// invented in 1991 to replace an earlier hash function MD4. It was specified
/// in 1992 as RFC 1321.
/// 
/// # Vulnerability
/// In 2004, it was shown that MD5 is not collision-resistant. Today, MD5 is
/// not recommended for serious cryptographic purposes anymore. So, NIST also
/// does not include MD5 in their list of recommended hashes for password
/// storage. __DO NOT USE MD5 FOR SERIOUS CRYPTOGRAPHIC PURPOSES AT ALL!__
/// If you need to use a hash algorithm for serious cryptographic purposes,
/// you are highly recommended to use SHA-3 hash algorithm instead of MD5,
/// for example.
/// 
/// # Usage of HD5
/// Though MD5 is lack of cryptographic security, MD5 is still widely used
/// for non-cryptograpic purposes such as:
/// - Generating small number of IDs
/// - Integrity test in some collision-free situations
/// - Storing passwords with limited security
/// - Digital Signature
/// 
/// Read [more](https://en.wikipedia.org/wiki/MD5) about MD5 in detail.
/// 
/// # Quick Start
/// In order to use the module md5, the module Cryptocol::hash::md5 is
/// re-exported so that you don't have to import (or use)
/// Cryptocol::hash::md5 directly. You only import MD5 struct in the module
/// Cryptocol::hash. Example 1 shows how to import MD5 struct.
/// 
/// ## Example 1
/// ```
/// use Cryptocol::hash::MD5;
/// ```
/// Then, you create MD5 object by the method MD5::new(). Now, you are ready to
/// use all prepared methods to hash any data. If you want to hash a string,
/// for example, you can use the method digest_str(). Then, the MD5 object that
/// you created will contain its hash value. You can use the macro println!()
/// for instance to print on a commandline screen by `println!("{}", hash)`
/// where hash is the MD5 object. Example 2 shows how to use MD5 struct quickly.
/// 
/// ## Example 2
/// ```
/// use std::string::*;
/// use Cryptocol::hash::MD5;
/// let mut hash = MD5::new();
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
/// txt = "I am testing MD5 for the data whose length is sixty-two bytes.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.to_string(), "6C33614E6317DC4641573E0EBC287F98");
/// 
/// let mut txt = "I am testing MD5 for the data whose length is sixty-four bytes..";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.get_HashValue_in_string(), "200F9A19EA45A830284342114483172B");
/// 
/// txt = "I am testing MD5 for the case data whose length is more than sixty-four bytes is given.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.to_string(), "9831162AB272AE1D85245B75726D215E");
/// ```
/// 
/// # Big-endian issue
/// It is just experimental for Big Endian CPUs. So, you are not encouraged
/// to use it for Big Endian CPUs for serious purpose. Only use this crate
/// for Big-endian CPUs with your own full responsibility.
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
    const R: [[u32; 4]; 4] = [[7, 12, 17, 22], [5,  9, 14, 20], [4, 11, 16, 23], [6, 10, 15, 21]];
    const H: [u32; 4] = [ 0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476 ];


    // pub fn new() -> Self
    /// Constructs a new `MD5`.
    /// 
    /// # Output
    /// A new object of `MD5`.
    /// 
    /// # Initialization
    /// All the attributes of the constructed object, which is initial hash
    /// value, will be initialized with `0123456789ABCDEFFEDCBA9876543210`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::hash::MD5;
    /// let mut hash = MD5::new();
    /// println!("Hash =\t{}", hash);
    /// assert_eq!(hash.to_string(), "0123456789ABCDEFFEDCBA9876543210");
    /// ```
    pub fn new() -> Self    { MD5 { hash_code: [IntUnion::new_with(Self::H[0]),
                                                IntUnion::new_with(Self::H[1]),
                                                IntUnion::new_with(Self::H[2]),
                                                IntUnion::new_with(Self::H[3])] } }

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
    /// [digest_str()](struct@MD5#method.digest_str)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of String
    /// object, you are highly recommended to use the method
    /// [digest_string()](struct@MD5#method.digest_string)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    /// object, you are highly recommended to use the method
    /// [digest_array()](struct@MD5#method.digest_array)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    /// object, you are highly recommended to use the method
    /// [digest_vec()](struct@MD5#method.digest_array)
    /// rather than this method.
    ///
    /// # Example
    /// ```
    /// use Cryptocol::hash::MD5;
    /// let txt = "This is an example of the method digest().";
    /// let mut hash = MD5::new();
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
        self.initialize();
        let length_done = (length_in_bytes >> 6) as usize;
        for i in 0..length_done
            { self.update(unsafe { from_raw_parts(message.add(i << 6) as *const u32, 16) } ); }
        self.finalize(unsafe { message.add(length_done << 6) }, length_in_bytes);
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
    /// [digest_string()](struct@MD5#method.digest_string)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    /// object, you are highly recommended to use the method
    /// [digest_array()](struct@MD5#method.digest_array)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    /// object, you are highly recommended to use the method
    /// [digest_vec()](struct@MD5#method.digest_array)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [digest()](struct@MD5#method.digest) rather than this method.
    ///
    /// # Example
    /// ```
    /// use Cryptocol::hash::MD5;
    /// let txt = "This is an example of the method digest_str().";
    /// let mut hash = MD5::new();
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
    /// [digest_str()](struct@MD5#method.digest_str)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    /// object, you are highly recommended to use the method
    /// [digest_array()](struct@MD5#method.digest_array)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    /// object, you are highly recommended to use the method
    /// [digest_vec()](struct@MD5#method.digest_array)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [digest()](struct@MD5#method.digest) rather than this method.
    ///
    /// # Example
    /// ```
    /// use Cryptocol::hash::MD5;
    /// let txt = "This is an example of the method digest_string().".to_string();
    /// let mut hash = MD5::new();
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
    /// [digest_str()](struct@MD5#method.digest_str)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of String
    /// object, you are highly recommended to use the method
    /// [digest_string()](struct@MD5#method.digest_string)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    /// object, you are highly recommended to use the method
    /// [digest_vec()](struct@MD5#method.digest_array)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [digest()](struct@MD5#method.digest) rather than this method.
    ///
    /// # Example
    /// ```
    /// use Cryptocol::hash::MD5;
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// let mut hash = MD5::new();
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
    pub fn digest_array<T, const N: usize>(&mut self, message: &[T; N])
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
    {
        self.digest(message.as_ptr() as *const u8, (N * T::size_in_bytes()) as u64);
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
    /// [digest_str()](struct@MD5#method.digest_str)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of String
    /// object, you are highly recommended to use the method
    /// [digest_string()](struct@MD5#method.digest_string)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    /// object, you are highly recommended to use the method
    /// [digest_array()](struct@MD5#method.digest_array)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [digest()](struct@MD5#method.digest) rather than this method.
    ///
    /// # Example
    /// ```
    /// use Cryptocol::hash::MD5;
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// let mut hash = MD5::new();
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
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
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
    /// [get_HashValue_string()](struct@MD5#method.get_HashValue_string)
    /// rather than this method.
    /// - If you want to get the hash value in the form of array object,
    /// you are highly recommended to use the method
    /// [get_HashValue_in_array()](struct@MD5#method.get_HashValue_in_array)
    /// rather than this method.
    /// - If you want to get the hash value in the form of Vec object,
    /// you are highly recommended to use the method
    /// [get_HashValue_in_vec()](struct@MD5#method.get_HashValue_in_vec)
    /// rather than this method.
    ///
    /// # Example
    /// ```
    /// use Cryptocol::hash::MD5;
    /// let txt = "This is an example of the method get_HashValue().";
    /// let mut hashValue = [0_u8; 16];
    /// let mut hash = MD5::new();
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
        let n_length = if length < (4 * 4) {length} else {4 * 4};
        #[cfg(target_endian = "little")]
        unsafe { copy_nonoverlapping(self.hash_code.as_ptr() as *const u8, hashValue, n_length); }
        #[cfg(target_endian = "big")]
        {
            let mut hash_code = [IntUnion::new(); 4];
            for i in 0..4
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
    /// [get_HashValue_in_array()](struct@MD5#method.get_HashValue_in_array)
    /// rather than this method.
    /// - If you want to get the hash value in the form of Vec object,
    /// you are highly recommended to use the method
    /// [get_HashValue_in_vec()](struct@MD5#method.get_HashValue_in_vec)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [get_HashValue()](struct@MD5#method.get_HashValue)
    /// rather than this method.
    ///
    /// # Example
    /// ```
    /// use Cryptocol::hash::MD5;
    /// let txt = "This is an example of the method get_HashValue_in_string().";
    /// let mut hash = MD5::new();
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

    // pub fn get_HashValue_in_array(&self) -> [u32; 4]
    /// Returns a hash value in the form of array object.
    /// 
    /// # Counterpart Methods
    /// - If you want to get the hash value in the form of String object,
    /// you are highly recommended to use the method
    /// [get_HashValue_string()](struct@MD5#method.get_HashValue_string)
    /// rather than this method.
    /// - If you want to get the hash value in the form of Vec object,
    /// you are highly recommended to use the method
    /// [get_HashValue_in_vec()](struct@MD5#method.get_HashValue_in_vec)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [get_HashValue()](struct@MD5#method.get_HashValue)
    /// rather than this method.
    ///
    /// # Example
    /// ```
    /// use Cryptocol::hash::MD5;
    /// let txt = "This is an example of the method get_HashValue_in_array().";
    /// let mut hash = MD5::new();
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_HashValue_in_array());
    /// assert_eq!(format!("{:02X?}", hash.get_HashValue_in_array()), "[A4BE6EEF, C9A5DFBA, 558B5ADF, 3B1035F9]");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn get_HashValue_in_array(&self) -> [u32; 4]
    {
        let mut res = [0_u32; 4];
        for i in 0..4
            { res[i] = self.hash_code[i].get().to_le(); }
        res
    }

    // pub fn getHashValue_in_vec(&self) -> Vec
    /// Returns a hash value in the form of Vec object.
    /// 
    /// # Counterpart Methods
    /// - If you want to get the hash value in the form of String object,
    /// you are highly recommended to use the method
    /// [get_HashValue_string()](struct@MD5#method.get_HashValue_string)
    /// rather than this method.
    /// - If you want to get the hash value in the form of array object,
    /// you are highly recommended to use the method
    /// [get_HashValue_in_array()](struct@MD5#method.get_HashValue_in_array)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [get_HashValue()](struct@MD5#method.get_HashValue)
    /// rather than this method.
    ///
    /// # Example
    /// ```
    /// use Cryptocol::hash::MD5;
    /// let txt = "This is an example of the method get_HashValue_in_vec().";
    /// let mut hash = MD5::new();
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
        Vec::<u32>::from(&self.get_HashValue_in_array())
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
                                .rotate_left(Self::R[0][i & 0b11]);
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
                                .rotate_left(Self::R[1][i & 0b11]);
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
                                .rotate_left(Self::R[2][i & 0b11]);
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
                                .rotate_left(Self::R[3][i & 0b11]);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f);
        }

        // Or the above can be shortened as follows but then it will be slower
        // a bit because of some overheads such as comparation, pointer
        // arithmatic operation, etc.
        //
        // for i in 0..64_usize
        // {
        //     let (mut f, g) = Self::func(b, c, d, i);
        //     f = f.wrapping_add(a)
        //             .wrapping_add(Self::getK(i))
        //             .wrapping_add(message[g].to_le())
        //             .rotate_left(Self::getR(i));
        //     a = d;
        //     d = c;
        //     c = b;
        //     b = b.wrapping_add(f);
        // }

        self.hash_code[0].set(self.hash_code[0].get().wrapping_add(a));
        self.hash_code[1].set(self.hash_code[1].get().wrapping_add(b));
        self.hash_code[2].set(self.hash_code[2].get().wrapping_add(c));
        self.hash_code[3].set(self.hash_code[3].get().wrapping_add(d));
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
/*
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
*/
	#[inline] fn getK(idx: usize) -> u32    { Self::K[idx] }
	#[inline] fn getH(idx: usize) -> u32    { Self::H[idx] }
    // #[inline] fn getR(idx: usize) -> u32  { Self::R[idx >> 4][idx & 0b11] }
	#[inline] fn Ff(x: u32, y: u32, z: u32) -> u32  { (x & y) | (!x & z) }
	#[inline] fn Gg(x: u32, y: u32, z: u32) -> u32  { (x & z) | (y & !z) }
	#[inline] fn Hh(x: u32, y: u32, z: u32) -> u32	{ x ^ y ^ z }
    #[inline] fn Ii(x: u32, y: u32, z: u32) -> u32	{ y ^ (x | !z) }
    #[inline] fn to_char(nibble: u8) -> char    { if nibble < 10  { ('0' as u8 + nibble) as u8 as char } else { ('A' as u8 - 10 + nibble) as char } }
}


impl Display for MD5
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
