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

/// You have freedom of changing H0 ~ H3, and ROUND.
#[allow(non_camel_case_types)]
pub type MD4_Expanded<const N: usize = 4,
                        const H0: u32 = 0x67452301, const H1: u32 = 0xefcdab89,
                        const H2: u32 = 0x98badcfe, const H3: u32 = 0x10325476,
                        const ROUND: usize = 48>
                = MD4_Generic<N, H0, H1, H2, H3, ROUND>;

/// You have freedom of changing ROUND, and K0 ~ K2.
#[allow(non_camel_case_types)]
pub type MD4_Generic_HR_fixed<const N: usize, const ROUND: usize,
                                const K0: u32, const K1: u32, const K2: u32>
    = MD4_Generic<N, 0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476, ROUND, K0, K1, K2>;

/// The official MD4 hash algorithm
#[allow(non_camel_case_types)]
pub type MD4 = MD4_Expanded;

/// A MD4 message-digest algorithm that lossily compresses data of arbitrary
/// length into a 128-bit hash value, and its flexible variants that allows
/// you to develop your own `MD4`-based hash algorithms
/// 
/// # Introduction
/// MD4 was designed by Ronald Rivest who is one of the inventors of RSA
/// asymmetric cryptographic algorithm. MD4 was invented in 1990. It was
/// specified in 1992 as RFC 1320.
/// This module provides not only the official MD4 but also its expanded
/// versions which is implemented with the name `MD4_Generic`.
/// MD4 uses the
/// [Merkle–Damgård construction](https://en.wikipedia.org/wiki/Merkle%E2%80%93Damg%C3%A5rd_construction).
/// 
/// # Vulnerability
/// The security of MD4 has been severely compromised. Today, MD4 is
/// not recommended for serious cryptographic purposes anymore. So, NIST also
/// does not include MD4 in their list of recommended hashes for password
/// storage. __DO NOT USE MD4 FOR SERIOUS CRYPTOGRAPHIC PURPOSES AT ALL!__
/// If you need to use a hash algorithm for serious cryptographic purposes,
/// you are highly recommended to use SHA-2 or SHA-3 hash algorithm,
/// for example, instead of MD4.
/// 
/// # Use of MD4 and their variants
/// Though MD4 and its variants are lack of cryptographic security, MD4 and
/// its variants can be still used for non-cryptograpic purposes such as:
/// - Generating small number of IDs
/// - Integrity test in some collision-free situations
/// - Storing passwords with limited security
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
/// 
/// # Security of your own expanded version
/// Your own algrorithm based on MD4 may be stronger or weaker than official
/// MD4. Unless you seriously checked the cryptographic security of your own
/// algorithms, it is hard to assume that your own alogrithms are stronger
/// than the official MD4.
/// 
/// # Reference
/// Read [more](https://en.wikipedia.org/wiki/MD4) about MD4 in detail.
/// 
/// # Quick Start
/// In order to use the module MD4, you don't have to import (or use)
/// Cryptocol::hash::md4::* directly because the module Cryptocol::hash::md4
/// is re-exported. All you have to do is only import MD4, MD4_Expanded,
/// MD4_Generic_HR_fixed and/or MD4_Generic in the module Cryptocol::hash.
/// Example 1 shows how to import structs MD4, MD4_Expanded,
/// MD4_Generic_HR_fixed and/or MD4_Generic. Plus, what you have to know is
/// these. All the types (or structs) are the specific versions of MD4_Generic.
/// Actually, MD4 is a specific version of MD4_Expanded. MD4_Expanded and
/// MD4_Generic_HR_fixed are specific versions of MD4_Generic.
/// 
/// ## Example 1
/// ```
/// use Cryptocol::hash::MD4;
/// use Cryptocol::hash::MD4_Expanded;
/// use Cryptocol::hash::MD4_Generic_HR_fixed;
/// use Cryptocol::hash::MD4_Generic;
/// ```
/// Then, you create MD4 object by the method MD4::new(). Now, you are ready to
/// use all provided methods to hash any data. If you want to hash a string,
/// for example, you can use the method digest_str(). Then, the MD4 object that
/// you created will contain its hash value. You can use the macro println!(),
/// for instance, to print on a commandline screen by `println!("{}", hash)`
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
/// assert_eq!(hash.get_HashValue_in_string(), "31D6CFE0D16AE931B73C59D7E0C089C0");
/// 
/// let txtStirng = String::from("A");
/// hash.digest_string(&txtStirng);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txtStirng, hash);
/// assert_eq!(hash.to_string(), "D5EF20EEB3F75679F86CF57F93ED0FFE");
/// 
/// let txtArray = ['W' as u8, 'o' as u8, 'w' as u8];
/// hash.digest_array(&txtArray);
/// println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txtArray, hash);
/// assert_eq!(hash.get_HashValue_in_string(), "6407C0E728DA762A04924ADFE630974C");
/// 
/// txt = "This data is 26-byte long.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.to_string(), "4F4A24D124B996BEA395344419F9A06B");
/// 
/// txt = "The unit of data length is not byte but bit.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.get_HashValue_in_string(), "9DE35D8FCF68E74867FFB63F28625ABE");
/// 
/// txt = "I am testing MD4 for the data whose length is sixty-two bytes.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.to_string(), "3A9F1487472B3A4315E0C90DC5CB3A2E");
/// 
/// let mut txt = "I am testing MD4 for the message which is sixty-four bytes long.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.get_HashValue_in_string(), "6CDB5B2BFF823A4A7B23675180EB7BEF");
/// 
/// txt = "I am testing MD4 for the case data whose length is more than sixty-four bytes is given.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
/// assert_eq!(hash.to_string(), "56771653687981390B0EB2A7D0A40DBB");
/// ```
/// 
/// # Big-endian issue
/// It is just experimental for Big Endian CPUs. So, you are not encouraged
/// to use it for Big Endian CPUs for serious purpose. Only use this crate
/// for Big-endian CPUs with your own full responsibility.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct MD4_Generic<const N: usize = 4,
        const H0: u32 = 0x67452301, const H1: u32 = 0xefcdab89,
        const H2: u32 = 0x98badcfe, const H3: u32 = 0x10325476, const ROUND: usize = 48,
        const K0: u32 = 0x00000000, const K1: u32 = 0x5A827999, const K2: u32 = 0x6ED9EBA1,
        const R00: u32 = 3, const R01: u32 = 7, const R02: u32 = 11, const R03: u32 = 19,
        const R10: u32 = 3, const R11: u32 = 5, const R12: u32 = 9, const R13: u32 = 13, 
        const R20: u32 = 3, const R21: u32 = 9, const R22: u32 = 11, const R23: u32 = 15>
{
    hash_code: [IntUnion; 4],
}

impl<const N: usize, const H0: u32, const H1: u32, const H2: u32, const H3: u32,
    const ROUND: usize, const K0: u32, const K1: u32, const K2: u32,
    const R00: u32, const R01: u32, const R02: u32, const R03: u32,
    const R10: u32, const R11: u32, const R12: u32, const R13: u32,
    const R20: u32, const R21: u32, const R22: u32, const R23: u32>
MD4_Generic<N, H0, H1, H2, H3, ROUND, K0, K1, K2, 
            R00, R01, R02, R03, R10, R11, R12, R13, R20, R21, R22, R23>
{
    const K: [u32; 3] = [ K0, K1, K2 ];
    const R: [[u32; 4]; 3] = [  [R00, R01, R02, R03],
                                [R10, R11, R12, R13],
                                [R20, R21, R22, R23] ];
    const H: [u32; 4] = [ H0, H1, H2, H3 ];


    // pub fn new() -> Self
    /// Constructs a new `MD4` object or a new MD4-based hash object.
    /// 
    /// # Output
    /// A new object of `MD4` or a new MD4-based hash object.
    /// 
    /// # Initialization
    /// All the attributes of the constructed object, which is initial hash
    /// value, will be initialized with `0123456789ABCDEFFEDCBA9876543210` for
    /// MD4. However, if you use your own MD4-expanded version, it will be
    /// initialized with your special values H0 ~ H3.
    /// 
    /// # Example for MD4
    /// ```
    /// use Cryptocol::hash::MD4;
    /// let mut hash = MD4::new();
    /// println!("Hash =\t{}", hash);
    /// assert_eq!(hash.to_string(), "0123456789ABCDEFFEDCBA9876543210");
    /// ```
    /// 
    /// # Exmaple for MD4_Expanded
    /// ```
    /// use Cryptocol::hash::MD4_Expanded;
    /// type myMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let mut my_hash = myMD4::new();
    /// println!("my_hash =\t{}", my_hash);
    /// assert_eq!(my_hash.to_string(), "111111114444444488888888FFFFFFFF");
    /// ```
    pub fn new() -> Self
    {
        if (N > 4) || (N == 0)
            { panic!("N cannot be either 0 or greater than 4. {}", N); }

        MD4_Generic
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
    /// its data type is `u64`.
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
    /// # Example for MD4
    /// ```
    /// use Cryptocol::hash::MD4;
    /// let txt = "This is an example of the method digest().";
    /// let mut hash = MD4::new();
    /// hash.digest(txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "A18836F660C3C66B8CBEE4BD24FEFFA9");
    /// ```
    /// 
    /// # Example for MD4_Expanded
    /// ```
    /// use Cryptocol::hash::MD4_Expanded;
    /// type myMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let mut my_hash = myMD4::new();
    /// let txt = "This is an example of the method digest().";
    /// let mut my_hash = myMD4::new();
    /// my_hash.digest(txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "B2F465006DCBA147BCE76D7EB8B564E1");
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
    /// # Example for MD4
    /// ```
    /// use Cryptocol::hash::MD4;
    /// let txt = "This is an example of the method digest_str().";
    /// let mut hash = MD4::new();
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "E396CE68E2BE1001BCBFD62B49E226C0");
    /// ```
    /// 
    /// # Example for MD4_Expanded
    /// ```
    /// use Cryptocol::hash::MD4_Expanded;
    /// type myMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let txt = "This is an example of the method digest_str().";
    /// let mut my_hash = myMD4::new();
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "719A1EB0F5077837BB408434B7AAD81E");
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
    /// # Example for MD4
    /// ```
    /// use Cryptocol::hash::MD4;
    /// let txt = "This is an example of the method digest_string().".to_string();
    /// let mut hash = MD4::new();
    /// hash.digest_string(&txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "DF23C7808B2B158C5E2D8C9FE1FF2ECC");
    /// ```
    /// 
    /// # Example for MD4_Expanded
    /// ```
    /// use Cryptocol::hash::MD4_Expanded;
    /// type myMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let txt = "This is an example of the method digest_string().".to_string();
    /// let mut myMD4 = myMD4::new();
    /// myMD4.digest_string(&txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, myMD4);
    /// assert_eq!(myMD4.to_string(), "FD42F7479ED133619D877BB1E6C8A084");
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
    /// # Example for MD4
    /// ```
    /// use Cryptocol::hash::MD4;
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// let mut hash = MD4::new();
    /// hash.digest_array(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "31489CF63B7FC170E9046F0176A60B39");
    /// ```
    /// 
    /// # Example for MD4_Expanded
    /// ```
    /// use Cryptocol::hash::MD4_Expanded;
    /// type myMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// let mut my_hash = myMD4::new();
    /// my_hash.digest_array(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    /// assert_eq!(my_hash.to_string(), "3011AFFDE0C322C2CCEE632FE39AF16D");
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
    /// # Example for MD4
    /// ```
    /// use Cryptocol::hash::MD4;
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// let mut hash = MD4::new();
    /// hash.digest_vec(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "31489CF63B7FC170E9046F0176A60B39");
    /// ```
    /// 
    /// # Example for MD4_Expanded
    /// ```
    /// use Cryptocol::hash::MD4_Expanded;
    /// type myMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// let mut my_hash = myMD4::new();
    /// my_hash.digest_vec(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    /// assert_eq!(my_hash.to_string(), "3011AFFDE0C322C2CCEE632FE39AF16D");
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
    /// # Example for MD4
    /// ```
    /// use Cryptocol::hash::MD4;
    /// let txt = "This is an example of the method get_HashValue().";
    /// let mut hashValue = [0_u8; 16];
    /// let mut hash = MD4::new();
    /// hash.digest_str(txt);
    /// hash.get_HashValue(hashValue.as_ptr() as *mut u8, hashValue.len());
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hashValue);
    /// assert_eq!(format!("{:02X?}", hashValue), "[12, E1, F9, 39, C3, D8, 09, A7, 23, 7D, 94, B4, F1, A0, 1E, FD]");
    /// ```
    /// 
    /// # Example for MD4_Expanded
    /// ```
    /// use Cryptocol::hash::MD4_Expanded;
    /// type myMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let txt = "This is an example of the method get_HashValue().";
    /// let mut hashValue = [0_u8; 16];
    /// let mut my_hash = myMD4::new();
    /// my_hash.digest_str(txt);
    /// my_hash.get_HashValue(hashValue.as_ptr() as *mut u8, hashValue.len());
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hashValue);
    /// assert_eq!(format!("{:02X?}", hashValue), "[62, 62, E3, D7, 06, 11, F5, 55, F9, 1D, 64, 78, C3, 32, C2, B8]");
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
    /// # Output
    /// It returns String object.
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
    /// # Example for MD4
    /// ```
    /// use Cryptocol::hash::MD4;
    /// let txt = "This is an example of the method get_HashValue_in_string().";
    /// let mut hash = MD4::new();
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_HashValue_in_string());
    /// assert_eq!(hash.get_HashValue_in_string(), "B871AC07999486EB0A6450DA5E09E92D");
    /// ```
    /// 
    /// # Example for MD4_Expanded
    /// ```
    /// use Cryptocol::hash::MD4_Expanded;
    /// type myMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let txt = "This is an example of the method get_HashValue_in_string().";
    /// let mut my_hash = myMD4::new();
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.get_HashValue_in_string());
    /// assert_eq!(my_hash.get_HashValue_in_string(), "7FC6C3F917CCA507B21D05B67F3E549B");
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

    // pub fn get_HashValue_in_array(&self) -> [u32; N]
    /// Returns a hash value in the form of array object.
    /// 
    /// # Output
    /// It returns [u32; N].
    /// 
    /// # Counterpart Methods
    /// - If you want to get the hash value in any form of Array object,
    /// you are highly recommended to use the method
    /// [put_HashValue_in_array()](struct@MD4#method.put_HashValue_in_array)
    /// rather than this method.
    /// - If you want to get the hash value in the form of String object,
    /// you are highly recommended to use the method
    /// [get_HashValue_in_string()](struct@MD4#method.get_HashValue_in_string)
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
    /// # Example for MD4
    /// ```
    /// use Cryptocol::hash::MD4;
    /// let txt = "This is an example of the method get_HashValue_in_array().";
    /// let mut hash = MD4::new();
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_HashValue_in_array());
    /// assert_eq!(format!("{:08X?}", hash.get_HashValue_in_array()), "[9F7E4FD8, 906C5422, 9FAAAFBA, 363BE03A]");
    /// ```
    /// 
    /// # Example for MD4_Expanded
    /// ```
    /// use Cryptocol::hash::MD4_Expanded;
    /// type myMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let txt = "This is an example of the method get_HashValue_in_array().";
    /// let mut my_hash = myMD4::new();
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_HashValue_in_array());
    /// assert_eq!(format!("{:08X?}", my_hash.get_HashValue_in_array()), "[E68DA94C, 583C881E, A7D2A6F5, 5BC4347F]");
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
    /// # Output
    /// It returns Vec<u32>.
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
    /// # Example for MD4
    /// ```
    /// use Cryptocol::hash::MD4;
    /// let txt = "This is an example of the method get_HashValue_in_vec().";
    /// let mut hash = MD4::new();
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_HashValue_in_vec());
    /// assert_eq!(format!("{:08X?}", hash.get_HashValue_in_vec()), "[6BD261A9, 47EFE9B7, 04397C1B, 628A61D7]");
    /// ```
    /// 
    /// # Example for MD4_Expanded
    /// ```
    /// use Cryptocol::hash::MD4_Expanded;
    /// type myMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let txt = "This is an example of the method get_HashValue_in_vec().";
    /// let mut my_hash = myMD4::new();
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_HashValue_in_vec());
    /// assert_eq!(format!("{:08X?}", my_hash.get_HashValue_in_vec()), "[093C7233, 97776D39, 5BFDEFCD, 3F679BFF]");
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

    // pub fn put_HashValue_in_array<T, const M: usize>(&self, out: &mut [T; M])
    /// Puts a hash value in the form of array object.
    /// 
    /// # Panics
    /// If M * mem::size_of::<T>() > 16 (= 4 * 4), this method will panic
    /// or its behaviour is undefined even if it won't panic.
    ///
    /// # Example for MD4
    /// ```
    /// use Cryptocol::hash::MD4;
    /// let txt = "This is an example of the method put_HashValue_in_array().";
    /// let mut hash = MD4::new();
    /// let mut hash_code = [0_u32; 4];
    /// hash.digest_str(txt);
    /// hash.put_HashValue_in_array(&mut hash_code);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    /// assert_eq!(format!("{:08X?}", hash_code), "[E4046C5C, 06735474, 4152BA95, 9A4261D8]");
    /// ```
    /// 
    /// # Example for MD4_Expanded
    /// ```
    /// use Cryptocol::hash::MD4_Expanded;
    /// type myMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let txt = "This is an example of the method put_HashValue_in_array().";
    /// let mut my_hash = myMD4::new();
    /// let mut hash_code = [0_u32; 4];
    /// my_hash.digest_str(txt);
    /// my_hash.put_HashValue_in_array(&mut hash_code);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    /// assert_eq!(format!("{:08X?}", hash_code), "[673E3933, 1F45BBC6, F874BCF9, 6C70ED89]");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn put_HashValue_in_array<T, const M: usize>(&self, out: &mut [T; M])
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
    {
        let res = self.get_HashValue_in_array();
        let out_size = T::size_in_bytes() * M;
        let length = if out_size < 16 {out_size} else {16};
        unsafe { copy_nonoverlapping(res.as_ptr() as *const u8, out as *mut T as *mut u8, length); }
    }

    // pub fn tangle(&mut self, tangling: u64)
    /// Tangles the hash value
    /// 
    /// # Argument
    /// u32 constant to tangle the hash value
    /// 
    /// # Features
    /// It is for using this struct as random number generator.
    /// 
    /// Example for MD4
    /// ```
    /// use Cryptocol::hash::MD4;
    /// let txt = "TANGLING";
    /// let mut hash = MD4::new();
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_HashValue_in_array());
    /// assert_eq!(format!("{:08X?}", hash.get_HashValue_in_array()), "[BC65D6E1, F0F37B4E, 2F404331, A8F25E2A]");
    /// hash.tangle(1);
    /// println!("Hash =\t{:08X?}", hash.get_HashValue_in_array());
    /// assert_eq!(format!("{:08X?}", hash.get_HashValue_in_array()), "[CE1E07A3, F3373D70, 95A8F098, 9BC7894E]");
    /// hash.tangle(1);
    /// println!("Hash =\t{:08X?}", hash.get_HashValue_in_array());
    /// assert_eq!(format!("{:08X?}", hash.get_HashValue_in_array()), "[5B9A2D14, 64888002, 15282E23, E5B2F4BD]");
    /// ```
    /// 
    /// Example for MD4_Expanded
    /// ```
    /// use Cryptocol::hash::MD4_Expanded;
    /// type myMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let txt = "TANGLING";
    /// let mut my_hash = myMD4::new();
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_HashValue_in_array());
    /// assert_eq!(format!("{:08X?}", my_hash.get_HashValue_in_array()), "[C4377D49, 05FD9A1F, 3DA4E254, ACF22116]");
    /// my_hash.tangle(1);
    /// println!("Hash =\t{:08X?}", my_hash.get_HashValue_in_array());
    /// assert_eq!(format!("{:08X?}", my_hash.get_HashValue_in_array()), "[8CB0AF83, F75E073C, 77C5BF6C, EDFE1D51]");
    /// my_hash.tangle(1);
    /// println!("Hash =\t{:08X?}", my_hash.get_HashValue_in_array());
    /// assert_eq!(format!("{:08X?}", my_hash.get_HashValue_in_array()), "[A5C900D1, 388193FA, B2C0ED53, 4DE71DDE]");
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

    // fn func(x: u32, y: u32, z: u32, round: usize) -> (u32, usize)
    /// Round function
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


impl<const N: usize,
    const H0: u32, const H1: u32, const H2: u32, const H3: u32,
    const ROUND: usize, const K0: u32, const K1: u32, const K2: u32,
    const R00: u32, const R01: u32, const R02: u32, const R03: u32,
    const R10: u32, const R11: u32, const R12: u32, const R13: u32,
    const R20: u32, const R21: u32, const R22: u32, const R23: u32>
Display for MD4_Generic<N, H0, H1, H2, H3, ROUND, K0, K1, K2, 
                        R00, R01, R02, R03, R10, R11, R12, R13, R20, R21, R22, R23>
{
    /// Formats the value using the given formatter.
    /// You will hardly use this method directly.
    /// Automagically the function `to_string()` will be implemented. So, you
    /// can use the function `to_string()`, and you can also print the MD4
    /// object in the macro `println!()` directly for example.
    /// `f` is a buffer, this method must write the formatted string into it.
    /// [Read more](https://doc.rust-lang.org/core/fmt/trait.Display.html#tymethod.fmt)
    /// 
    /// # Example 1 for the method to_string() for MD4
    /// ```
    /// use Cryptocol::hash::MD4;
    /// let mut hash = MD4::new();
    /// let txt = "Display::fmt() automagically implement to_string().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
    /// assert_eq!(hash.to_string(), "E2244B71E17D5BD7E1CCEB58C8F8C82E");
    /// ```
    /// 
    /// # Example 2 for the method to_string() for MD4_Expanded
    /// ```
    /// use Cryptocol::hash::MD4_Expanded;
    /// type myMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let txt = "Display::fmt() automagically implement to_string().";
    /// let mut my_hash = myMD4::new();
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.to_string());
    /// assert_eq!(my_hash.to_string(), "6B0D8F0CE90782E5FF88EE57B5DC5AF1");
    /// ```
    /// 
    /// # Example 3 for the use in the macro println!() for MD4
    /// ```
    /// use Cryptocol::hash::MD4;
    /// let mut hash = MD4::new();
    /// let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "3C607B5548C155DCF4E84C7A6C21D349");
    /// ```
    /// 
    /// # Example 4 for the use in the macro println!() for MD4_Expanded
    /// ```
    /// use Cryptocol::hash::MD4_Expanded;
    /// type myMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let mut my_hash = myMD4::new();
    /// let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "745B42127EC2479032923F2EE368FD92");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)
        write!(f, "{}", self.get_HashValue_in_string())
    }
}
