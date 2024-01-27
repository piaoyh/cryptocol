// Copyright 2023, 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module that contains a MD5 hash algorithm.

#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]


use std::ptr::copy_nonoverlapping;
use std::slice::from_raw_parts;
use std::fmt::{ self, Debug, Display, Formatter };

use crate::number::{ SmallUInt, IntUnion, LongUnion };


/// You have freedom of changing H0 ~ H3, and ROUND.
#[allow(non_camel_case_types)]
pub type MD5_Expanded<const N: usize = 4,
                        const H0: u32 = 0x67452301, const H1: u32 = 0xefcdab89,
                        const H2: u32 = 0x98badcfe, const H3: u32 = 0x10325476,
                        const ROUND: usize = 64>
    = MD5_Generic<N, H0, H1, H2, H3, ROUND>;

/// You have freedom of changing ROUND, and K00 ~ K63.
#[allow(non_camel_case_types)]
pub type MD5_Generic_HR_fixed<const N: usize, const ROUND: usize,
                    const K00: u32, const K01: u32, const K02: u32, const K03: u32,
                    const K04: u32, const K05: u32, const K06: u32, const K07: u32,
                    const K08: u32, const K09: u32, const K10: u32, const K11: u32,
                    const K12: u32, const K13: u32, const K14: u32, const K15: u32,
                    const K16: u32, const K17: u32, const K18: u32, const K19: u32,
                    const K20: u32, const K21: u32, const K22: u32, const K23: u32,
                    const K24: u32, const K25: u32, const K26: u32, const K27: u32,
                    const K28: u32, const K29: u32, const K30: u32, const K31: u32,
                    const K32: u32, const K33: u32, const K34: u32, const K35: u32,
                    const K36: u32, const K37: u32, const K38: u32, const K39: u32,
                    const K40: u32, const K41: u32, const K42: u32, const K43: u32,
                    const K44: u32, const K45: u32, const K46: u32, const K47: u32,
                    const K48: u32, const K49: u32, const K50: u32, const K51: u32,
                    const K52: u32, const K53: u32, const K54: u32, const K55: u32,
                    const K56: u32, const K57: u32, const K58: u32, const K59: u32,
                    const K60: u32, const K61: u32, const K62: u32, const K63: u32>
    = MD5_Generic<N, 0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476, ROUND, 
                    K00, K01, K02, K03, K04, K05, K06, K07,
                    K08, K09, K10, K11, K12, K13, K14, K15,
                    K16, K17, K18, K19, K20, K21, K22, K23,
                    K24, K25, K26, K27, K28, K29, K30, K31,
                    K32, K33, K34, K35, K36, K37, K38, K39,
                    K40, K41, K42, K43, K44, K45, K46, K47,
                    K48, K49, K50, K51, K52, K53, K54, K55,
                    K56, K57, K58, K59,K60, K61, K62, K63>;

/// The official MD5 hash algorithm
#[allow(non_camel_case_types)]
pub type MD5 = MD5_Expanded;


/// A MD5 message-digest algorithm that lossily compresses data of arbitrary
/// length into a 128-bit hash value, and its flexible variants that allows
/// you to develop your own `MD5`-based hash algorithms
/// 
/// # Introduction
/// MD5 was designed by Ronald Rivest who is one of the inventors of RSA
/// asymmetric cryptographic algorithm. MD5 was invented in 1991 to replace
/// an earlier hash function MD4. It was specified in 1992 as RFC 1321.
/// This module provides not only the official MD5 but also its expanded
/// versions which is implemented with the name `MD5_Generic`.
/// MD5 uses the
/// [Merkle–Damgård construction](https://en.wikipedia.org/wiki/Merkle%E2%80%93Damg%C3%A5rd_construction).
/// 
/// # Vulnerability
/// In 2004, it was shown that MD5 is not collision-resistant. Today, MD5 is
/// not recommended for serious cryptographic purposes anymore. So, NIST also
/// does not include MD5 in their list of recommended hashes for password
/// storage. __DO NOT USE MD5 FOR SERIOUS CRYPTOGRAPHIC PURPOSES AT ALL!__
/// If you need to use a hash algorithm for serious cryptographic purposes,
/// you are highly recommended to use SHA-2 or SHA-3 hash algorithm,
/// for example, instead of MD4.
/// 
/// # Use of MD5 and their variants
/// Though MD5 and its variants are lack of cryptographic security, MD5 and
/// its variants can be still used for non-cryptograpic purposes such as:
/// - Generating small number of IDs
/// - Integrity test in some collision-free situations
/// - Storing passwords with limited security
/// - Study of hash algorithms
/// - Cryptanalysis Research to find the weakness of MD5 and Merkle-Damgard
/// construction which MD2, MD4, MD5, SHA0, SHA1, and all SHA2 family use
/// 
/// # Generic Parameters
/// You can create your own expanded version of MD5 by changing the generic
/// parameters H0 ~ H3, ROUND, K00 ~ K63, R00 ~ R03, R10 ~ R13, and R20 ~ R23,
/// and R30 ~ R33.
/// - N : the size of output. N cannot be 0 or greater than 4. If N is 4, the
/// output is 128 bits, while if N is 1, the output is 32 bits.
/// - H0 ~ H3 : the initial hash values, four u32 values.
/// The default values of H0, H1, H2, and H3 are 0x67452301, 0xefcdab89,
/// 0x98badcfe, and 0x10325476, respectively (in little endian representation).
/// - ROUND : the number of rounds. The default value of it is `64` (= 16 * 4).
/// - K00 ~ K63 : the added values in hashing process, sixty-four u32 values.
/// The default values of K00 ~ K63 are defined to be the integer part of
/// 4294967296 times abs(sin(i)), where i is in radians. So, K00 = 4294967296
/// times abs(sin(1)), K01 = 4294967296 times abs(sin(2)), K02 = 4294967296
/// times abs(sin(3)), and so on (in little endian representation).
/// - R00 ~ R03, R10 ~ R13, R20 ~ R23, and R30 ~ R33 : the amounts of rotate
/// left in the hashing process.
/// The default values of R00, R01, R02, and R03 are 7, 12, 17, and 22, respectively.
/// The default values of R10, R11, R12, and R13 are 5, 9, 14, and 20, respectively.
/// The default values of R20, R11, R12, and R23 are 4, 11, 16, and 23, respecively.
/// The default values of R30, R31, R32, and R33 are 6, 10, 15, and 21, respectively.
/// 
/// About the parameters and their default values,
/// read [more](https://datatracker.ietf.org/doc/html/rfc1321)
/// 
/// # Security of your own expanded version
/// Your own algrorithm based on MD5 may be stronger or weaker than official
/// MD5. Unless you seriously checked the cryptographic security of your own
/// algorithms, it is hard to assume that your own alogrithms are stronger
/// than the official MD5.
/// 
/// # Reference
/// Read [more](https://en.wikipedia.org/wiki/MD5) about MD5 in detail.
/// 
/// # Quick Start
/// In order to use the module md5, you don't have to import (or use)
/// cryptocol::hash::md5::* directly because the module cryptocol::hash::md5
/// is re-exported. All you have to do is only import MD5, MD5_Expanded,
/// MD5_Generic_HR_fixed and/or MD5_Generic in the module cryptocol::hash.
/// Example 1 shows how to import structs MD5, MD5_Expanded,
/// MD5_Generic_HR_fixed and/or MD5_Generic. Plus, what you have to know is
/// these. All the types (or structs) are the specific versions of MD5_Generic.
/// Actually, MD5 is a specific version of MD5_Expanded. MD5_Expanded and
/// MD5_Generic_HR_fixed are specific versions of MD5_Generic.
/// 
/// ## Example 1
/// ```
/// use cryptocol::hash::MD5;
/// use cryptocol::hash::MD5_Expanded;
/// use cryptocol::hash::MD5_Generic_HR_fixed;
/// use cryptocol::hash::MD5_Generic;
/// ```
/// Then, you create MD5 object by the method MD5::new(). Now, you are ready to
/// use all prepared methods to hash any data. If you want to hash a string,
/// for example, you can use the method digest_str(). Then, the MD5 object that
/// you created will contain its hash value. You can use the macro println!(),
/// for instance, to print on a commandline screen by `println!("{}", hash)`
/// where hash is the MD5 object. Example 2 shows how to use MD5 struct quickly.
/// 
/// ## Example 2
/// ```
/// use std::string::*;
/// use cryptocol::hash::MD5;
/// let mut hash = MD5::new();
/// 
/// let mut txt = "";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.get_hash_value_in_string(), "D41D8CD98F00B204E9800998ECF8427E");
/// 
/// let txt_stirng = String::from("A");
/// hash.digest_string(&txt_stirng);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt_stirng, hash);
/// assert_eq!(hash.to_string(), "7FC56270E7A70FA81A5935B72EACBE29");
/// 
/// let txt_array = ['W' as u8, 'o' as u8, 'w' as u8];
/// hash.digest_array(&txt_array);
/// println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txt_array, hash);
/// assert_eq!(hash.get_hash_value_in_string(), "49DC5E45FBEC1433E2C612E5AA809C10");
/// 
/// txt = "This data is 26-byte long.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.to_string(), "17ED1DB5CD96184041659D84BB36D76B");
/// 
/// txt = "The unit of data length is not byte but bit.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.get_hash_value_in_string(), "C3EB6D4A1071E1A9C5E08FEF6E8F3FBF");
/// 
/// txt = "I am testing MD5 for the data whose length is sixty-two bytes.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.to_string(), "6C33614E6317DC4641573E0EBC287F98");
/// 
/// let mut txt = "I am testing MD5 for the message which is sixty-four bytes long.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.get_hash_value_in_string(), "584D41C6837AC714275196E4FF14B2EF");
/// 
/// txt = "I am testing MD5 for the case data whose length is more than sixty-four bytes is given.";
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
#[allow(non_camel_case_types)]
pub struct MD5_Generic<const N: usize = 4,
        const H0: u32 = 0x67452301, const H1: u32 = 0xefcdab89,
        const H2: u32 = 0x98badcfe, const H3: u32 = 0x10325476,
        const ROUND: usize = 64,
        const K00: u32 = 0xd76aa478, const K01: u32 = 0xe8c7b756,
        const K02: u32 = 0x242070db, const K03: u32 = 0xc1bdceee,
        const K04: u32 = 0xf57c0faf, const K05: u32 = 0x4787c62a,
        const K06: u32 = 0xa8304613, const K07: u32 = 0xfd469501,
        const K08: u32 = 0x698098d8, const K09: u32 = 0x8b44f7af,
        const K10: u32 = 0xffff5bb1, const K11: u32 = 0x895cd7be,
        const K12: u32 = 0x6b901122, const K13: u32 = 0xfd987193,
        const K14: u32 = 0xa679438e, const K15: u32 = 0x49b40821,
        const K16: u32 = 0xf61e2562, const K17: u32 = 0xc040b340,
        const K18: u32 = 0x265e5a51, const K19: u32 = 0xe9b6c7aa,
        const K20: u32 = 0xd62f105d, const K21: u32 = 0x02441453,
        const K22: u32 = 0xd8a1e681, const K23: u32 = 0xe7d3fbc8,
        const K24: u32 = 0x21e1cde6, const K25: u32 = 0xc33707d6,
        const K26: u32 = 0xf4d50d87, const K27: u32 = 0x455a14ed,
        const K28: u32 = 0xa9e3e905, const K29: u32 = 0xfcefa3f8,
        const K30: u32 = 0x676f02d9, const K31: u32 = 0x8d2a4c8a,
        const K32: u32 = 0xfffa3942, const K33: u32 = 0x8771f681,
        const K34: u32 = 0x6d9d6122, const K35: u32 = 0xfde5380c,
        const K36: u32 = 0xa4beea44, const K37: u32 = 0x4bdecfa9,
        const K38: u32 = 0xf6bb4b60, const K39: u32 = 0xbebfbc70,
        const K40: u32 = 0x289b7ec6, const K41: u32 = 0xeaa127fa,
        const K42: u32 = 0xd4ef3085, const K43: u32 = 0x04881d05,
        const K44: u32 = 0xd9d4d039, const K45: u32 = 0xe6db99e5,
        const K46: u32 = 0x1fa27cf8, const K47: u32 = 0xc4ac5665,
        const K48: u32 = 0xf4292244, const K49: u32 = 0x432aff97,
        const K50: u32 = 0xab9423a7, const K51: u32 = 0xfc93a039,
        const K52: u32 = 0x655b59c3, const K53: u32 = 0x8f0ccc92,
        const K54: u32 = 0xffeff47d, const K55: u32 = 0x85845dd1,
        const K56: u32 = 0x6fa87e4f, const K57: u32 = 0xfe2ce6e0,
        const K58: u32 = 0xa3014314, const K59: u32 = 0x4e0811a1,
        const K60: u32 = 0xf7537e82, const K61: u32 = 0xbd3af235,
        const K62: u32 = 0x2ad7d2bb, const K63: u32 = 0xeb86d391,
        const R00: u32 = 7, const R01: u32 = 12, const R02: u32 = 17, const R03: u32 = 22,
        const R10: u32 = 5, const R11: u32 = 9,  const R12: u32 = 14, const R13: u32 = 20,
        const R20: u32 = 4, const R21: u32 = 11, const R22: u32 = 16, const R23: u32 = 23,
        const R30: u32 = 6, const R31: u32 = 10, const R32: u32 = 15, const R33: u32 = 21>
{
    hash_code: [IntUnion; 4],
}

impl<const N: usize, const H0: u32, const H1: u32, const H2: u32,
    const H3: u32, const ROUND: usize, 
    const K00: u32, const K01: u32, const K02: u32, const K03: u32,
    const K04: u32, const K05: u32, const K06: u32, const K07: u32,
    const K08: u32, const K09: u32, const K10: u32, const K11: u32,
    const K12: u32, const K13: u32, const K14: u32, const K15: u32,
    const K16: u32, const K17: u32, const K18: u32, const K19: u32,
    const K20: u32, const K21: u32, const K22: u32, const K23: u32,
    const K24: u32, const K25: u32, const K26: u32, const K27: u32,
    const K28: u32, const K29: u32, const K30: u32, const K31: u32,
    const K32: u32, const K33: u32, const K34: u32, const K35: u32,
    const K36: u32, const K37: u32, const K38: u32, const K39: u32,
    const K40: u32, const K41: u32, const K42: u32, const K43: u32,
    const K44: u32, const K45: u32, const K46: u32, const K47: u32,
    const K48: u32, const K49: u32, const K50: u32, const K51: u32,
    const K52: u32, const K53: u32, const K54: u32, const K55: u32,
    const K56: u32, const K57: u32, const K58: u32, const K59: u32,
    const K60: u32, const K61: u32, const K62: u32, const K63: u32,
    const R00: u32, const R01: u32, const R02: u32, const R03: u32,
    const R10: u32, const R11: u32, const R12: u32, const R13: u32,
    const R20: u32, const R21: u32, const R22: u32, const R23: u32,
    const R30: u32, const R31: u32, const R32: u32, const R33: u32>
MD5_Generic<N, H0, H1, H2, H3, ROUND,
            K00, K01, K02, K03, K04, K05, K06, K07,
            K08, K09, K10, K11, K12, K13, K14, K15,
            K16, K17, K18, K19, K20, K21, K22, K23,
            K24, K25, K26, K27, K28, K29, K30, K31,
            K32, K33, K34, K35, K36, K37, K38, K39,
            K40, K41, K42, K43, K44, K45, K46, K47,
            K48, K49, K50, K51, K52, K53, K54, K55,
            K56, K57, K58, K59, K60, K61, K62, K63,
            R00, R01, R02, R03, R10, R11, R12, R13,
            R20, R21, R22, R23, R30, R31, R32, R33>
{
    const K: [u32; 64] = [  K00, K01, K02, K03, K04, K05, K06, K07,
                            K08, K09, K10, K11, K12, K13, K14, K15,
                            K16, K17, K18, K19, K20, K21, K22, K23,
                            K24, K25, K26, K27, K28, K29, K30, K31,
                            K32, K33, K34, K35, K36, K37, K38, K39,
                            K40, K41, K42, K43, K44, K45, K46, K47,
                            K48, K49, K50, K51, K52, K53, K54, K55,
                            K56, K57, K58, K59, K60, K61, K62, K63 ];
    const R: [[u32; 4]; 4] = [  [R00, R01, R02, R03], [R10, R11, R12, R13],
                                [R20, R21, R22, R23], [R30, R31, R32, R33] ];
    const H: [u32; 4] = [ H0, H1, H2, H3 ];


    // pub fn new() -> Self
    /// Constructs a new `MD5` object or a new MD5-based hash object.
    /// 
    /// # Output
    /// A new object of `MD5` or a new MD5-based hash object.
    /// 
    /// # Initialization
    /// All the attributes of the constructed object, which is initial hash
    /// value, will be initialized with `0123456789ABCDEFFEDCBA9876543210` for
    /// MD5. However, if you use your own MD5-expanded version, it will be
    /// initialized with your special values H0 ~ H3.
    /// 
    /// # Example for MD5
    /// ```
    /// use cryptocol::hash::MD5;
    /// let hash = MD5::new();
    /// println!("Hash =\t{}", hash);
    /// assert_eq!(hash.to_string(), "0123456789ABCDEFFEDCBA9876543210");
    /// ```
    /// 
    /// # Exmaple for MD5_Expanded
    /// ```
    /// use cryptocol::hash::MD5_Expanded;
    /// type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 128>;
    /// let my_hash = MyMD5::new();
    /// println!("Hash =\t{}", my_hash);
    /// assert_eq!(my_hash.to_string(), "111111114444444488888888FFFFFFFF");
    /// ```
    pub fn new() -> Self
    {
        if (N > 4) || (N == 0)
            { panic!("N cannot be either 0 or greater than 4. {}", N); }

        MD5_Generic
        {
            hash_code: [IntUnion::new_with(Self::H[0]),
                        IntUnion::new_with(Self::H[1]),
                        IntUnion::new_with(Self::H[2]),
                        IntUnion::new_with(Self::H[3])]
        }
    }

    // pub fn digest(&mut self, message: *const u8, length_in_bytes: u64)
    /// Computes hash value.
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
    /// # Example for MD5
    /// ```
    /// use cryptocol::hash::MD5;
    /// let mut hash = MD5::new();
    /// let txt = "This is an example of the method digest().";
    /// hash.digest(txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "336EA91DD3216BD0FC841E86F9E722D8");
    /// ```
    /// 
    /// # Example for MD5_Expanded
    /// ```
    /// use cryptocol::hash::MD5_Expanded;
    /// type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 128>;
    /// let mut my_hash = MyMD5::new();
    /// let txt = "This is an example of the method digest().";
    /// my_hash.digest(txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "51F4248FBEFBE0A00196F9F04DD07FF0");
    /// ````
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

    // pub fn digest_str(&mut self, message: &str)
    /// Computes hash value.
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
    /// # Example for MD5
    /// ```
    /// use cryptocol::hash::MD5;
    /// let mut hash = MD5::new();
    /// let txt = "This is an example of the method digest_str().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "F2E455CEB5FB993A980E67D3FA8A3961");
    /// ```
    /// 
    /// # Example for MD5_Expanded
    /// ```
    /// use cryptocol::hash::MD5_Expanded;
    /// type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let mut my_hash = MyMD5::new();
    /// let txt = "This is an example of the method digest_str().";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "21EE03C8185BD65CDB8116D0E2714F09");
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
    /// Computes hash value.
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
    /// # Example for MD5
    /// ```
    /// use cryptocol::hash::MD5;
    /// let mut hash = MD5::new();
    /// let txt = "This is an example of the method digest_string().".to_string();
    /// hash.digest_string(&txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "40929E789D2F5880B85456E289F704C0");
    /// ```
    /// 
    /// # Example for MD5_Expanded
    /// ```
    /// use cryptocol::hash::MD5_Expanded;
    /// type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let mut my_hash = MyMD5::new();
    /// let txt = "This is an example of the method digest_string().".to_string();
    /// my_hash.digest_string(&txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "02BDBC510B949045A131C0C3302027BA");
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

    // pub fn digest_array<T, const M: usize>(&mut self, message: &[T; M])
    /// Computes hash value.
    /// 
    /// # Features
    /// This function is a wrapping function of `digest()`.
    /// This function computes hash value of the content of Array object.
    /// 
    /// # Argument
    /// - message is `&[T; M]`.
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
    /// # Example for MD5
    /// ```
    /// use cryptocol::hash::MD5;
    /// let mut hash = MD5::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// let mut hash = MD5::new();
    /// hash.digest_array(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "054DE9CF5F9EA623BBB8DC4781685A58");
    /// ```
    /// 
    /// # Example for MD5_Expanded
    /// ```
    /// use cryptocol::hash::MD5_Expanded;
    /// type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let mut my_hash = MyMD5::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// my_hash.digest_array(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    /// assert_eq!(my_hash.to_string(), "A5DC1291539528723C6C3E6F7EFDAE94");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn digest_array<T, const M: usize>(&mut self, message: &[T; M])
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
    {
        self.digest(message.as_ptr() as *const u8, (M * T::size_in_bytes()) as u64);
    }

    // pub fn digest_vec<T>(&mut self, message: &Vec<T>)
    /// Computes hash value.
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
    /// # Example for MD5
    /// ```
    /// use cryptocol::hash::MD5;
    /// let mut hash = MD5::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.digest_vec(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "054DE9CF5F9EA623BBB8DC4781685A58");
    /// ```
    /// 
    /// # Example for MD5_Expanded
    /// ```
    /// use cryptocol::hash::MD5_Expanded;
    /// type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let mut my_hash = MyMD5::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// my_hash.digest_vec(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    /// assert_eq!(my_hash.to_string(), "A5DC1291539528723C6C3E6F7EFDAE94");
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

    // pub fn ruminate(&mut self, n: usize, message: *const u8, length_in_bytes: u64)
    /// Computes a hash value of `message`, and then computes a new hash value
    /// of the hash value of the message, and then computes a hash value of the
    /// previous hash value, and then ... `n` times repeatedly.
    /// 
    /// # Arguments
    /// - `n` is the number of repetition of digestion
    /// - `message` is pointer to const u8.
    /// - `length_in_bytes` is the size of message in the unit of bytes, and
    /// data type is `u64`.
    /// 
    /// # Origin
    /// Double hashing is invented by Ferguson and Schneier in their book
    /// Practical Cryptography to countermeasure against length extension
    /// attacks. Plus, Bitcoin uses double hashing.
    /// This is generalized version of it.
    /// 
    /// # Features
    /// This function has the generalized interface (pointer, `*const u8`)
    /// so as to enable other functions to wrap this function with any
    /// convenient interface for uses. So, this function is usually not called
    /// directly in Rust. This function is provided to be called from other
    /// programming languages such as C/C++.
    /// 
    /// # Security Issue
    /// The author doubts that the double hashing is securer than normal
    /// hashing. The double hashing will be as secure as the normal hashing
    /// at most because birthday paradox applies twice for the double hashing
    /// though the size of the domain is the same size of the codomain for
    /// second hashing of the double hashing, while the birthday paradox
    /// applies only once for the normal hashing.
    ///
    /// # Example 1 for MD5
    /// ```
    /// use cryptocol::hash::MD5;
    /// let mut hash = MD5::new();
    /// let txt = "This is an example of the method ruminate().";
    /// hash.ruminate(2, txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "71F09FB7840FA1EB78A88ED071627C0D");
    /// ```
    /// 
    /// # Example 2 for MD5_Expanded
    /// ```
    /// use cryptocol::hash::MD5_Expanded;
    /// type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 128>;
    /// let mut hash = MyMD5::new();
    /// let txt = "This is an example of the method ruminate().";
    /// hash.ruminate(2, txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "CC179809A9DC1475EEF5E4810C272882");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn ruminate(&mut self, n: usize, message: *const u8, length_in_bytes: u64)
    {
        self.digest(message, length_in_bytes);
        for _ in 1..n
            { self.digest_array(&self.get_hash_value_in_array()); }
    }

    // pub fn ruminate_str(&mut self, n: usize, message: &str)
    /// Computes a hash value of `message`, and then computes a new hash value
    /// of the hash value of the message, and then computes a hash value of the
    /// previous hash value, and then ... `n` times repeatedly.
    /// 
    /// # Arguments
    /// - `n` is the number of repetition of digestion
    /// - `message` is `&str`.
    /// 
    /// # Origin
    /// Double hashing is invented by Ferguson and Schneier in their book
    /// Practical Cryptography to countermeasure against length extension
    /// attacks. Plus, Bitcoin uses double hashing.
    /// This is generalized version of it.
    /// 
    /// # Features
    /// This function is a wrapping function of `ruminate()`.
    /// This function computes hash value of the content of string slice.
    /// 
    /// # Security Issue
    /// The author doubts that the double hashing is securer than normal
    /// hashing. The double hashing will be as secure as the normal hashing
    /// at most because birthday paradox applies twice for the double hashing
    /// though the size of the domain is the same size of the codomain for
    /// second hashing of the double hashing, while the birthday paradox
    /// applies only once for the normal hashing.
    ///
    /// # Example 1 for MD5
    /// ```
    /// use cryptocol::hash::MD5;
    /// let mut hash = MD5::new();
    /// let txt = "This is an example of the method ruminate_str().";
    /// hash.ruminate_str(3, txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "68B3B09AE0EED0D15E744671E29824D4");
    /// ```
    /// 
    /// # Example 2 for MD5_Expanded
    /// ```
    /// use cryptocol::hash::MD5_Expanded;
    /// type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 128>;
    /// let mut my_hash = MyMD5::new();
    /// let txt = "This is an example of the method ruminate_str().";
    /// my_hash.ruminate_str(3, txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "7A460BDA766C6A7D4F9A23DCBDB71A4C");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn ruminate_str(&mut self, n: usize, message: &str)
    {
        self.ruminate(n, message.as_ptr(), message.len() as u64);
    }

    // pub fn ruminate_string(&mut self, n: usize, message: &String)
    /// Computes a hash value of `message`, and then computes a new hash value
    /// of the hash value of the message, and then computes a hash value of the
    /// previous hash value, and then ... `n` times repeatedly.
    /// 
    /// # Arguments
    /// - `n` is the number of repetition of digestion
    /// - `message` is `&String`.
    /// 
    /// # Origin
    /// Double hashing is invented by Ferguson and Schneier in their book
    /// Practical Cryptography to countermeasure against length extension
    /// attacks. Plus, Bitcoin uses double hashing.
    /// This is generalized version of it.
    /// 
    /// # Features
    /// This function is a wrapping function of `ruminate()`.
    /// This function computes hash value of the content of String object.
    /// 
    /// # Security Issue
    /// The author doubts that the double hashing is securer than normal
    /// hashing. The double hashing will be as secure as the normal hashing
    /// at most because birthday paradox applies twice for the double hashing
    /// though the size of the domain is the same size of the codomain for
    /// second hashing of the double hashing, while the birthday paradox
    /// applies only once for the normal hashing.
    ///
    /// # Example 1 for MD5
    /// ```
    /// use cryptocol::hash::MD5;
    /// let mut hash = MD5::new();
    /// let txt = "This is an example of the method ruminate_string().".to_string();
    /// hash.ruminate_string(2, &txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "E06B1A664322C1296D1FCD3F28428493");
    /// ```
    /// 
    /// # Example 2 for MD5_Expanded
    /// ```
    /// use cryptocol::hash::MD5_Expanded;
    /// type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 128>;
    /// let mut my_hash = MyMD5::new();
    /// let txt = "This is an example of the method ruminate_string().".to_string();
    /// my_hash.ruminate_string(2, &txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "5018AF48C7606F748073FC5255448BAB");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn ruminate_string(&mut self, n: usize, message: &String)
    {
        self.ruminate(n, message.as_ptr(), message.len() as u64);
    }

    // pub fn ruminate_array<T, const M: usize>(&mut self, n: usize, message: &[T; M])
    /// Computes a hash value of `message`, and then computes a new hash value
    /// of the hash value of the message, and then computes a hash value of the
    /// previous hash value, and then ... `n` times repeatedly.
    /// 
    /// # Arguments
    /// - `n` is the number of repetition of digestion
    /// - `message` is `&[T; M]`.
    /// 
    /// # Origin
    /// Double hashing is invented by Ferguson and Schneier in their book
    /// Practical Cryptography to countermeasure against length extension
    /// attacks. Plus, Bitcoin uses double hashing.
    /// This is generalized version of it.
    /// 
    /// # Features
    /// This function is a wrapping function of `ruminate()`.
    /// This function computes hash value of the content of Array object.
    /// 
    /// # Security Issue
    /// The author doubts that the double hashing is securer than normal
    /// hashing. The double hashing will be as secure as the normal hashing
    /// at most because birthday paradox applies twice for the double hashing
    /// though the size of the domain is the same size of the codomain for
    /// second hashing of the double hashing, while the birthday paradox
    /// applies only once for the normal hashing.
    ///
    /// # Example 1 for MD5
    /// ```
    /// use cryptocol::hash::MD5;
    /// let mut hash = MD5::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.ruminate_array(5,&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "4914D161AE665750248DF91B6E57C7BE");
    /// ```
    /// 
    /// # Example 2 for MD5_Expanded
    /// ```
    /// use cryptocol::hash::MD5_Expanded;
    /// type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 128>;
    /// let mut my_hash = MyMD5::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// my_hash.ruminate_array(5,&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    /// assert_eq!(my_hash.to_string(), "1FBF755293909670FE66B8CA482BCF66");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn ruminate_array<T, const M: usize>(&mut self, n: usize, message: &[T; M])
    where T: SmallUInt + Copy + Clone
    {
        self.ruminate(n, message.as_ptr() as *const u8, (M * T::size_in_bytes()) as u64);
    }

    // pub fn ruminate_vec<T>(&mut self, n: usize, message: &Vec<T>)
    /// Computes a hash value of `message`, and then computes a new hash value
    /// of the hash value of the message, and then computes a hash value of the
    /// previous hash value, and then ... `n` times repeatedly.
    /// 
    /// # Arguments
    /// - `n` is the number of repetition of digestion
    /// - `message` is `&Vec<T>`.
    /// 
    /// # Origin
    /// Double hashing is invented by Ferguson and Schneier in their book
    /// Practical Cryptography to countermeasure against length extension
    /// attacks. Plus, Bitcoin uses double hashing.
    /// This is generalized version of it.
    /// 
    /// # Features
    /// This function is a wrapping function of `ruminate()`.
    /// This function computes hash value of the content of Vec object.
    /// 
    /// # Security Issue
    /// The author doubts that the double hashing is securer than normal
    /// hashing. The double hashing will be as secure as the normal hashing
    /// at most because birthday paradox applies twice for the double hashing
    /// though the size of the domain is the same size of the codomain for
    /// second hashing of the double hashing, while the birthday paradox
    /// applies only once for the normal hashing.
    ///
    /// # Example 1 for MD5
    /// ```
    /// use cryptocol::hash::MD5;
    /// let mut hash = MD5::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.ruminate_vec(2, &data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "BDEE5A3C5B2DB7B6F18B170C2E865FE0");
    /// ```
    /// 
    /// # Example 2 for MD5_Expanded
    /// ```
    /// use cryptocol::hash::MD5_Expanded;
    /// type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 128>;
    /// let mut my_hash = MyMD5::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// my_hash.ruminate_vec(2, &data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    /// assert_eq!(my_hash.to_string(), "DBFD74659889B373D90477B59A193CBD");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn ruminate_vec<T>(&mut self, n: usize, message: &Vec<T>)
    where T: SmallUInt + Copy + Clone
    {
        self.ruminate(n, message.as_ptr() as *const u8, (message.len() * T::size_in_bytes()) as u64);
    }

    // pub fn get_hash_value(&self, hash_value: *mut u8, length: usize)
    /// Gives a hash value to the place where `hash_value` points to.
    /// 
    /// # Features
    /// This function has the generalized interface (pointer, `*mut u8`)
    /// so as to enable other functions to wrap this function with any
    /// convenient interface for uses. So, this function is usually not called
    /// directly in Rust. This function is provided to be called from other
    /// programming languages such as C/C++.
    /// 
    /// # Arguments
    /// - `hash_value` is the pointer to the place to hold the result hash value.
    /// - `length` is the size of the place that `hash_value` points to. 
    /// 
    /// # Counterpart Methods
    /// - If you want to get the hash value in the form of String object,
    /// you are highly recommended to use the method
    /// [get_hash_value_string()](struct@MD5#method.get_hash_value_string)
    /// rather than this method.
    /// - If you want to get the hash value in the form of array object,
    /// you are highly recommended to use the method
    /// [get_hash_value_in_array()](struct@MD5#method.get_hash_value_in_array)
    /// rather than this method.
    /// - If you want to get the hash value in the form of Vec object,
    /// you are highly recommended to use the method
    /// [get_hash_value_in_vec()](struct@MD5#method.get_hash_value_in_vec)
    /// rather than this method.
    ///
    /// # Example for MD5
    /// ```
    /// use cryptocol::hash::MD5;
    /// let mut hash = MD5::new();
    /// let txt = "This is an example of the method get_hash_value().";
    /// let hash_value = [0_u8; 16];
    /// hash.digest_str(txt);
    /// hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    /// assert_eq!(format!("{:02X?}", hash_value), "[91, 57, 43, 58, C7, F9, 04, 83, 60, 63, 15, CD, 1B, 77, 2E, DD]");
    /// ```
    /// 
    /// # Example for MD5_Expanded
    /// ```
    /// use cryptocol::hash::MD5_Expanded;
    /// type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let mut my_hash = MyMD5::new();
    /// let txt = "This is an example of the method get_hash_value().";
    /// let hash_value = [0_u8; 16];
    /// my_hash.digest_str(txt);
    /// my_hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    /// assert_eq!(format!("{:02X?}", hash_value), "[A4, 5C, 46, 58, 29, BB, 83, 06, 32, 4D, 20, 20, 23, 9D, 41, AE]");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn get_hash_value(&self, hash_value: *mut u8, length: usize)
    {
        const BYTES: usize = 4;
        const N: usize = 4;
        let n_length = if length < (BYTES * N) {length} else {BYTES * N};
        #[cfg(target_endian = "little")]   // Because of MD5 is based on Little Endian
        unsafe { copy_nonoverlapping(self.hash_code.as_ptr() as *const u8, hash_value, n_length); }
        #[cfg(target_endian = "big")]   // Because of MD5 is based on Little Endian
        {
            let mut hash_code = [IntUnion::new(); N];
            for i in 0..N
                { hash_code[i].set(self.hash_code[i].get().to_le()); }
            unsafe { copy_nonoverlapping(hash_code.as_ptr() as *const u8, hash_value, n_length); }
        }
    }


    // pub fn get_hash_value_in_string(&self) -> String
    /// Returns a hash value in the form of String object.
    /// 
    /// # Output
    /// It returns String object.
    /// 
    /// # Counterpart Methods
    /// - If you want to get the hash value in the form of array object,
    /// you are highly recommended to use the method
    /// [get_hash_value_in_array()](struct@MD5#method.get_hash_value_in_array)
    /// rather than this method.
    /// - If you want to get the hash value in the form of Vec object,
    /// you are highly recommended to use the method
    /// [get_hash_value_in_vec()](struct@MD5#method.get_hash_value_in_vec)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [get_hash_value()](struct@MD5#method.get_hash_value)
    /// rather than this method.
    ///
    /// # Example for MD5
    /// ```
    /// use cryptocol::hash::MD5;
    /// let mut hash = MD5::new();
    /// let txt = "This is an example of the method get_hash_value_in_string().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_hash_value_in_string());
    /// assert_eq!(hash.get_hash_value_in_string(), "5E9D7F0006214CB49D09FC846FBE2927");
    /// ```
    /// 
    /// # Example for MD5_Expanded
    /// ```
    /// use cryptocol::hash::MD5_Expanded;
    /// type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let mut my_hash = MyMD5::new();
    /// let txt = "This is an example of the method get_hash_value_in_string().";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.get_hash_value_in_string());
    /// assert_eq!(my_hash.get_hash_value_in_string(), "A8BA6619878AE3A8135B7FD2A6ECAE6D");
    /// ```
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn get_hash_value_in_string(&self) -> String
    {
        const BYTES: usize = 4;
        let mut txt = String::new();
        for i in 0..N
        {
            let hs = self.hash_code[i];
            for j in 0..BYTES
            {
                let byte = hs.get_ubyte_(j);    // Because of MD5 is based on Little Endian
                txt.push(Self::to_char(byte >> 4));
                txt.push(Self::to_char(byte & 0b1111));
            }
        }
        txt
    }

    // pub fn get_hash_value_in_array(&self) -> [u32; N]
    /// Returns a hash value in the form of array object.
    /// 
    /// # Output
    /// It returns [u32; N].
    /// 
    /// # Counterpart Methods
    /// - If you want to get the hash value in the form of String object,
    /// you are highly recommended to use the method
    /// [get_hash_value_string()](struct@MD5#method.get_hash_value_string)
    /// rather than this method.
    /// - If you want to get the hash value in the form of Vec object,
    /// you are highly recommended to use the method
    /// [get_hash_value_in_vec()](struct@MD5#method.get_hash_value_in_vec)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [get_hash_value()](struct@MD5#method.get_hash_value)
    /// rather than this method.
    ///
    /// # Example for MD5
    /// ```
    /// use cryptocol::hash::MD5;
    /// let mut hash = MD5::new();
    /// let txt = "This is an example of the method get_hash_value_in_array().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[1FC84032, 1DFA906E, 911B468C, 66EDE0CE]");
    /// ```
    /// 
    /// # Example for MD5_Expanded
    /// ```
    /// use cryptocol::hash::MD5_Expanded;
    /// type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let mut my_hash = MyMD5::new();
    /// let txt = "This is an example of the method get_hash_value_in_array().";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[06813E1E, DA7BA0BF, 4B48D110, 6B111859]");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn get_hash_value_in_array(&self) -> [u32; N]
    {
        let mut res = [0_u32; N];
        for i in 0..N
            { res[i] = self.hash_code[i].get().to_le(); }
        res
    }

    // pub fn get_hash_value_in_vec(&self) -> Vec<u32>
    /// Returns a hash value in the form of Vec object.
    /// 
    /// # Output
    /// It returns Vec<u32>.
    /// 
    /// # Counterpart Methods
    /// - If you want to get the hash value in the form of String object,
    /// you are highly recommended to use the method
    /// [get_hash_value_string()](struct@MD5#method.get_hash_value_string)
    /// rather than this method.
    /// - If you want to get the hash value in the form of array object,
    /// you are highly recommended to use the method
    /// [get_hash_value_in_array()](struct@MD5#method.get_hash_value_in_array)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [get_hash_value()](struct@MD5#method.get_hash_value)
    /// rather than this method.
    ///
    /// # Example for MD5
    /// ```
    /// use cryptocol::hash::MD5;
    /// let mut hash = MD5::new();
    /// let txt = "This is an example of the method get_hash_value_in_vec().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_hash_value_in_vec());
    /// assert_eq!(format!("{:08X?}", hash.get_hash_value_in_vec()), "[D9A44F09, 27F51F07, 4517E390, 4CF17D73]");
    /// ```
    /// # Example for MD5_Expanded
    /// ```
    /// use cryptocol::hash::MD5_Expanded;
    /// type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let mut my_hash = MyMD5::new();
    /// let txt = "This is an example of the method get_hash_value_in_vec().";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_hash_value_in_vec());
    /// assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_vec()), "[5D9AB684, 090F7AEB, 31FD214E, F03D3032]");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn get_hash_value_in_vec(&self) -> Vec<u32>
    {
        let mut res = Vec::new();
        for i in 0..N
            { res.push(self.hash_code[i].get().to_le()); }
        res
    }

    // pub fn put_hash_value_in_array<T, const N: usize>(&self, out: &mut [T; N])
    /// Puts a hash value in the form of array object.
    /// 
    /// # Argument
    /// `out` is the array [T; M] which is the place to put the hash value.
    /// 
    /// # Features
    /// If M * mem::size_of::<T>() > mem::size_of::<u32>() * N,
    /// it pass the output as the amount of mem::size_of::<u32>() * N.
    ///
    /// # Example for MD5
    /// ```
    /// use cryptocol::hash::MD5;
    /// let mut hash = MD5::new();
    /// let txt = "This is an example of the method put_hash_value_in_array().";
    /// let mut hash_code = [0_u32; 4];
    /// hash.digest_str(txt);
    /// hash.put_hash_value_in_array(&mut hash_code);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    /// assert_eq!(format!("{:08X?}", hash_code), "[512E75DE, 4525528D, 41E8D192, 5606EE3B]");
    /// ```
    /// 
    /// # Example for MD5_Expanded
    /// ```
    /// use cryptocol::hash::MD5_Expanded;
    /// type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let mut my_hash = MyMD5::new();
    /// let txt = "This is an example of the method put_hash_value_in_array().";
    /// let mut hash_code = [0_u32; 4];
    /// my_hash.digest_str(txt);
    /// my_hash.put_hash_value_in_array(&mut hash_code);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    /// assert_eq!(format!("{:08X?}", hash_code), "[F8634B80, 96E02659, E26EA89D, EDA8E0C4]");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn put_hash_value_in_array<T, const M: usize>(&self, out: &mut [T; M])
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
    {
        let res = self.get_hash_value_in_array();
        let out_size = T::size_in_bytes() * M;
        let length = if out_size < 16 {out_size} else {16};
        unsafe { copy_nonoverlapping(res.as_ptr() as *const u8, out as *mut T as *mut u8, length); }
    }

    // pub fn tangle(&mut self, tangling: u64)
    /// Tangles the hash value
    /// 
    /// # Argument
    /// u32 constants to tangle the hash value
    /// 
    /// # Features
    /// It is for using this struct as random number generator.
    /// 
    /// Example for MD5
    /// ```
    /// use cryptocol::hash::MD5;
    /// let mut hash = MD5::new();
    /// let txt = "TANGLING";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[E60545F6, 6DCF2B02, 8245048B, AE2A98C6]");
    /// hash.tangle(1);
    /// println!("Hash =\t{:08X?}", hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[E0B5F1C0, 5C62629F, 68D44BC1, D384AB34]");
    /// hash.tangle(1);
    /// println!("Hash =\t{:08X?}", hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[C75EEA9C, 9D5CF62B, 0ABFA634, CD29C2D4]");
    /// ```
    /// 
    /// Example for MD5_Expanded
    /// ```
    /// use cryptocol::hash::MD5_Expanded;
    /// type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let mut my_hash = MyMD5::new();
    /// let txt = "TANGLING";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[9CCE671A, 5366F625, 68056532, D6B0DA5C]");
    /// my_hash.tangle(1);
    /// println!("Hash =\t{:08X?}", my_hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[A12380BC, DE74206D, C145732C, 4CAAD502]");
    /// my_hash.tangle(1);
    /// println!("Hash =\t{:08X?}", my_hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[D9EB87F4, 00C2D299, A492A483, 1C24FCDD]");
    /// ```
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
    /// This method is the core part of MD5 hash algorithm.
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
            let j = i & 0b11_1111;
            let (mut f, g) = Self::func(b, c, d, j);
            f = f.wrapping_add(a)
                    .wrapping_add(Self::get_k(j))
                    .wrapping_add(message[g].to_le())
                    .rotate_left(Self::get_r(j));
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f);
        }

        // Or the above can be divided into four sections as follows but then
        // it will be slower a bit because of some overheads such as
        // comparation, pointer arithmatic operation, etc.
        //
        // for i in 0..16_usize
        // {
        //     let f = Self::ff(b, c, d).wrapping_add(a)
        //                         .wrapping_add(Self::get_k(i))
        //                         .wrapping_add(message[i].to_le())
        //                         .rotate_left(Self::R[0][i & 0b11]);
        //     a = d;
        //     d = c;
        //     c = b;
        //     b = b.wrapping_add(f);
        // }
        // for i in 16..32_usize
        // {
        //     let g = ((i << 2) + i + 1) & 0b1111;
        //     let f = Self::gg(b, c, d).wrapping_add(a)
        //                         .wrapping_add(Self::get_k(i))
        //                         .wrapping_add(message[g].to_le())
        //                         .rotate_left(Self::R[1][i & 0b11]);
        //     a = d;
        //     d = c;
        //     c = b;
        //     b = b.wrapping_add(f);
        // }
        // for i in 32..48_usize
        // {
        //     let g = ((i << 1) + i + 5) & 0b1111;
        //     let f = Self::hh(b, c, d).wrapping_add(a)
        //                         .wrapping_add(Self::get_k(i))
        //                         .wrapping_add(message[g].to_le())
        //                         .rotate_left(Self::R[2][i & 0b11]);
        //     a = d;
        //     d = c;
        //     c = b;
        //     b = b.wrapping_add(f);
        // }
        // for i in 48..64_usize
        // {
        //     let g = ((i << 3) - i) & 0b1111;
        //     let f = Self::ii(b, c, d).wrapping_add(a)
        //                         .wrapping_add(Self::get_k(i))
        //                         .wrapping_add(message[g].to_le())
        //                         .rotate_left(Self::R[3][i & 0b11]);
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
        // ideally round &= 0b11_1111 equivalent to round %= 64;
        if round < 16
            { (Self::ff(x, y, z), round & 0b1111) }
        else if round < 32
            { (Self::gg(x, y, z), ((round << 2) + round + 1) & 0b1111) }    // equivalent to ((5 * round) + 1) % 16
        else if round < 48
            { (Self::hh(x, y, z), ((round << 1) + round + 5) & 0b1111) }    // equivalent to ((3 * round) + 5) % 16
        else
            { (Self::ii(x, y, z), ((round << 3) - round) & 0b1111) }        // equivalent to (7 * round) % 16
    }

	#[inline] fn get_k(idx: usize) -> u32    { Self::K[idx] }    // ideally Self::K[idx & 0b11_1111] equivalent to Self::K[idx % 64]
    #[inline] fn get_r(idx: usize) -> u32    { Self::R[idx >> 4][idx & 0b11] }   // ideally Self::R[(idx & 0b11_1111) >> 4)][idx & 0b11] equivalent to Self::R[(idx % 16) / 4][idx % 4]
    #[inline] fn ff(x: u32, y: u32, z: u32) -> u32  { z ^ (x & (y ^ z)) }   // equivalent to { (x & y) | (!x & z) }
	#[inline] fn gg(x: u32, y: u32, z: u32) -> u32  { (x & z) | (y & !z) }
	#[inline] fn hh(x: u32, y: u32, z: u32) -> u32	{ x ^ y ^ z }
    #[inline] fn ii(x: u32, y: u32, z: u32) -> u32	{ y ^ (x | !z) }
    #[inline] fn to_char(nibble: u8) -> char    { if nibble < 10  { ('0' as u8 + nibble) as u8 as char } else { ('A' as u8 - 10 + nibble) as char } }
}


impl<const N: usize,
    const H0: u32, const H1: u32, const H2: u32, const H3: u32, const ROUND: usize, 
    const K00: u32, const K01: u32, const K02: u32, const K03: u32,
    const K04: u32, const K05: u32, const K06: u32, const K07: u32,
    const K08: u32, const K09: u32, const K10: u32, const K11: u32,
    const K12: u32, const K13: u32, const K14: u32, const K15: u32,
    const K16: u32, const K17: u32, const K18: u32, const K19: u32,
    const K20: u32, const K21: u32, const K22: u32, const K23: u32,
    const K24: u32, const K25: u32, const K26: u32, const K27: u32,
    const K28: u32, const K29: u32, const K30: u32, const K31: u32,
    const K32: u32, const K33: u32, const K34: u32, const K35: u32,
    const K36: u32, const K37: u32, const K38: u32, const K39: u32,
    const K40: u32, const K41: u32, const K42: u32, const K43: u32,
    const K44: u32, const K45: u32, const K46: u32, const K47: u32,
    const K48: u32, const K49: u32, const K50: u32, const K51: u32,
    const K52: u32, const K53: u32, const K54: u32, const K55: u32,
    const K56: u32, const K57: u32, const K58: u32, const K59: u32,
    const K60: u32, const K61: u32, const K62: u32, const K63: u32,
    const R00: u32, const R01: u32, const R02: u32, const R03: u32,
    const R10: u32, const R11: u32, const R12: u32, const R13: u32,
    const R20: u32, const R21: u32, const R22: u32, const R23: u32,
    const R30: u32, const R31: u32, const R32: u32, const R33: u32>
Display for MD5_Generic<N, H0, H1, H2, H3, ROUND,
                        K00, K01, K02, K03, K04, K05, K06, K07,
                        K08, K09, K10, K11, K12, K13, K14, K15,
                        K16, K17, K18, K19, K20, K21, K22, K23,
                        K24, K25, K26, K27, K28, K29, K30, K31,
                        K32, K33, K34, K35, K36, K37, K38, K39,
                        K40, K41, K42, K43, K44, K45, K46, K47,
                        K48, K49, K50, K51, K52, K53, K54, K55,
                        K56, K57, K58, K59, K60, K61, K62, K63,
                        R00, R01, R02, R03, R10, R11, R12, R13,
                        R20, R21, R22, R23, R30, R31, R32, R33>
{
    /// Formats the value using the given formatter.
    /// You will hardly use this method directly.
    /// Automagically the function `to_string()` will be implemented. So, you
    /// can use the function `to_string()`, and you can also print the MD5
    /// object in the macro `println!()` directly for example.
    /// `f` is a buffer, this method must write the formatted string into it.
    /// [Read more](https://doc.rust-lang.org/core/fmt/trait.Display.html#tymethod.fmt)
    /// 
    /// # Example 1 for the method to_string() for MD5
    /// ```
    /// use cryptocol::hash::MD5;
    /// let mut hash = MD5::new();
    /// let txt = "Display::fmt() automagically implement to_string().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
    /// assert_eq!(hash.to_string(), "ED085603C2CDE77DD0C6FED3EC1A8ADB");
    /// ```
    /// 
    /// # Example 2 for the method to_string() for MD5_Expanded
    /// ```
    /// use cryptocol::hash::MD5_Expanded;
    /// type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let mut my_hash = MyMD5::new();
    /// let txt = "Display::fmt() automagically implement to_string().";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.to_string());
    /// assert_eq!(my_hash.to_string(), "3FDFF3827C89F3C770A0863F069FE766");
    /// ```
    /// 
    /// # Example 3 for the use in the macro println!() for MD5
    /// ```
    /// use cryptocol::hash::MD5;
    /// let mut hash = MD5::new();
    /// let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "6C9494A4A5C313001695262D72571F74");
    /// ```
    /// 
    /// # Example 4 for the use in the macro println!() for MD5_Expanded
    /// ```
    /// use cryptocol::hash::MD5_Expanded;
    /// type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let mut my_hash = MyMD5::new();
    /// let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "45BA0E4FEA1FACF829D19544A77105B8");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)
        write!(f, "{}", self.get_hash_value_in_string())
    }
}
