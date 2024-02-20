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

use crate::number::{ SmallUInt, IntUnion, LongUnion };


/// You have freedom of changing N, H0 ~ H4, and ROUND for SHA-1.
/// 
/// # Generic Parameters
/// You can create your own expanded version of SHA-1 by changing the generic
/// parameters N, H0 ~ H4, and ROUND.
/// - N : the size of output. N cannot be 0 or greater than 5. If N is 5, the
/// output is 160 bits, while if N is 1, the output is 32 bits.
/// The default value of N is `5`.
/// - H0 ~ H4 : the initial hash values, five u32 values.
/// The default values of H0, H1, H2, and H3 are 0x67452301, 0xefcdab89,
/// 0x98badcfe, 0x10325476, and 0xc3d2e1f0, (in big endian representation),
/// respectively.
/// - ROUND : the number of rounds. The default value of it is `80` (= 20 * 4).
#[allow(non_camel_case_types)]
pub type SHA1_Expanded<const N: usize = 5,
                        const H0: u32 = 0x67452301, const H1: u32 = 0xefcdab89,
                        const H2: u32 = 0x98badcfe, const H3: u32 = 0x10325476,
                        const H4: u32 = 0xc3d2e1f0, const ROUND: usize = 80>
    = SHA1_Generic<N, H0, H1, H2, H3, H4, ROUND>;

/// You have freedom of changing N, H0 ~ H4, and ROUND for SHA-0.
/// 
/// # Generic Parameters
/// You can create your own expanded version of SHA-0 by changing the generic
/// parameters N, H0 ~ H4, and ROUND.
/// - N : the size of output. N cannot be 0 or greater than 5. If N is 5, the
/// output is 160 bits, while if N is 1, the output is 32 bits.
/// The default value of N is `5`.
/// - H0 ~ H4 : the initial hash values, five u32 values.
/// The default values of H0, H1, H2, and H3 are 0x67452301, 0xefcdab89,
/// 0x98badcfe, 0x10325476, and 0xc3d2e1f0, (in big endian representation),
/// respectively.
/// - ROUND : the number of rounds. The default value of it is `80` (= 20 * 4).
#[allow(non_camel_case_types)]
pub type SHA0_Expanded<const N: usize = 5,
                        const H0: u32 = 0x67452301, const H1: u32 = 0xefcdab89,
                        const H2: u32 = 0x98badcfe, const H3: u32 = 0x10325476,
                        const H4: u32 = 0xc3d2e1f0, const ROUND: usize = 80>
        = SHA1_Generic<N, H0, H1, H2, H3, H4, ROUND,
                        0x5a827999, 0x6ed9eba1, 0x8f1bbcdc, 0xca62c1d6, 0>;

/// You have freedom of changing N, ROUND, and K0 ~ K3 for SHA-1.
/// 
/// # Generic Parameters
/// You can create your own expanded version of SHA-1 by changing the generic
/// parameters N, ROUND, and K0 ~ K3.
/// - N : the size of output. N cannot be 0 or greater than 5. If N is 5, the
/// output is 160 bits, while if N is 1, the output is 32 bits.
/// The default value of N is `5`.
/// - ROUND : the number of rounds. The default value of it is `80` (= 20 * 4).
/// - K0 ~ K3 : the added values in hashing process, which are four u32 values
/// and called round constants.
/// The default values of K0 ~ K3 are defined to be 2^30 times the square roots
/// of 2, 3, 5 and 10, respectivey.
/// The default values of K0, K1, K2, and K3 are 0x5a827999, 0x6ed9eba1,
/// 0x8f1bbcdc, and 0xca62c1d6 (in big endian representation), respecively.
#[allow(non_camel_case_types)]
pub type SHA1_Generic_HR_fixed<const N: usize = 5, const ROUND: usize = 80,
                    const K0: u32 = 0x5a827999, const K1: u32 = 0x6ed9eba1,
                    const K2: u32 = 0x8f1bbcdc, const K3: u32 = 0xca62c1d6>
        = SHA1_Generic<N, 0x67452301, 0xefcdab89, 0x98badcfe,
                        0x10325476, 0xc3d2e1f0, ROUND, K0, K1, K2, K3>;

/// You have freedom of changing N, ROUND, and K0 ~ K3 for SHA-0.
/// 
/// # Generic Parameters
/// You can create your own expanded version of SHA-0 by changing the generic
/// parameters N, ROUND, and K0 ~ K3.
/// - N : the size of output. N cannot be 0 or greater than 5. If N is 5, the
/// output is 160 bits, while if N is 1, the output is 32 bits.
/// The default value of N is `5`.
/// - ROUND : the number of rounds. The default value of it is `80` (= 20 * 4).
/// - K0 ~ K3 : the added values in hashing process, which are four u32 values
/// and called round constants.
/// The default values of K0 ~ K3 are defined to be 2^30 times the square roots
/// of 2, 3, 5 and 10, respectivey. So, the default values of K0, K1, K2, and
/// K3 are 0x5a827999, 0x6ed9eba1, 0x8f1bbcdc, and 0xca62c1d6
/// (in big endian representation), respecively.
#[allow(non_camel_case_types)]
pub type SHA0_Generic_HR_fixed<const N: usize = 5, const ROUND: usize = 80,
                    const K0: u32 = 0x5a827999, const K1: u32 = 0x6ed9eba1,
                    const K2: u32 = 0x8f1bbcdc, const K3: u32 = 0xca62c1d6>
        = SHA1_Generic<N, 0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476, 0xc3d2e1f0,
                        ROUND, K0, K1, K2, K3, 0>;

/// The official SHA-1 hash algorithm
pub type SHA1 = SHA1_Generic;   // equivalent to `pub type SHA1 = SHA1_Expanded;`

/// The official SHA-0 hash algorithm
pub type SHA0 = SHA0_Expanded;


/// A SHA-1 and SHA-0 message-digest algorithms that lossily compress data
/// of arbitrary length into a 160-bit hash value, and their flexible variants
/// that allow you to develop your own `SHA-1`-based hash algorithms
/// 
/// # Introduction
/// SHA-1 and SHA-0 were designed by the United States National Security Agency,
/// and are a U.S. Federal Information Processing Standard. SHA-1 and SHA-0
/// produces a message digest based on principles similar to those used by
/// Ronald L. Rivest of MIT in the design of the MD2, MD4 and MD5 message digest
/// algorithms, but generates a larger hash value (160 bits vs. 128 bits).
/// SHA-1 and SHA-0 use the
/// [Merkle–Damgård construction](https://en.wikipedia.org/wiki/Merkle%E2%80%93Damg%C3%A5rd_construction).
/// 
/// # Vulnerability
/// In February 2005, Xiaoyun Wang, Yiqun Lisa Yin, and Hongbo Yu announced
/// an attack to find collisions in the full version of SHA-1. Today, SHA-1
/// is not recommended for serious cryptographic purposes anymore.
/// __DO NOT USE SHA1 FOR SERIOUS CRYPTOGRAPHIC PURPOSES AT ALL!__
/// If you need to use a hash algorithm for serious cryptographic purposes,
/// you are highly recommended to use either SHA-2 or SHA-3 hash algorithm,
/// for example, instead of SHA-1 or SHA-0.
///
/// # Use of SHA-1, SHA-0, and their variants
/// Though SHA-1, SHA-0, and their variants are lack of cryptographic security,
/// SHA-1, SHA-0, and their variants can be still used for non-cryptograpic
/// purposes such as:
/// - Generating small number of IDs
/// - Integrity test in some collision-free situations
/// - Storing passwords with limited security
/// - Digital Signature with limited security
/// - Key generation
/// - Study of hash algorithms
/// - Cryptanalysis Research to find the weakness of SHA-1 and Merkle-Damgard
/// construction which MD2, MD4, MD5, SHA0, SHA1, and all SHA2 family use
/// 
/// # Generic Parameters
/// You can create your own expanded version of SHA-1 by changing the generic
/// parameters N, H0 ~ H4, ROUND, K0 ~ K3, RL1, RL5, and R30.
/// - N : the size of output. N cannot be 0 or greater than 5. If N is 5, the
/// output is 160 bits, while if N is 1, the output is 32 bits.
/// The default value of N is `5`.
/// - H0 ~ H4 : the initial hash values, five u32 values.
/// The default values of H0, H1, H2, and H3 are 0x67452301, 0xefcdab89,
/// 0x98badcfe, 0x10325476, and 0xc3d2e1f0 (in big endian representation),
/// respectively.
/// - ROUND : the number of rounds. The default value of it is `80` (= 20 * 4).
/// - K0 ~ K3 : the added values in hashing process, which are four u32 values
/// and called round constants.
/// The default values of K0 ~ K3 are defined to be 2^30 times the square roots
/// of 2, 3, 5 and 10, respectivey. _However, they are incorrectly rounded to
/// the nearest integer instead of being rounded to the nearest odd integer,
/// with equilibrated proportions of zero and one bits. As well, choosing the
/// square root of 10 (which is not a prime) made it a common factor for the
/// two other chosen square roots of primes 2 and 5, with possibly usable
/// arithmetic properties across successive rounds, reducing the strength
/// of the algorithm against finding collisions on some bits._
/// (quoted from [Wikipedia](https://en.wikipedia.org/wiki/SHA-1#SHA-1_pseudocode))
/// So, the default values of K0, K1, K2, and K3 are 0x5a827999, 0x6ed9eba1,
/// 0x8f1bbcdc, and 0xca62c1d6 (in big endian representation), respecively.
/// - RL1, RL5, and RL30 : the amounts of rotate left in the hashing process.
/// The default values of RL1, RL5, and RL30 are 1, 5, and 30, respecively.
/// RL1 is 1 for SHA-1 while RL1 is 0 for SHA-0.
/// 
/// About the parameters and their default values,
/// read [more](https://en.wikipedia.org/wiki/SHA-2#Pseudocode)
/// 
/// # Security of your own expanded version
/// Your own algrorithm based on SHA-0 or SHA-1 may be stronger or weaker than
/// official SHA-0 or SHA-1. Unless you seriously checked the cryptographic
/// security of your own algorithms, it is hard to assume that your own
/// alogrithms are stronger than the official SHA-0 or SHA-1.
/// 
/// Read [more](https://doi.org/10.6028/NIST.FIPS.180-4)
/// and/or watch [this video](https://www.youtube.com/watch?v=JIhZWgJA-9o)
/// to learn SHA-1 more in detail.
/// 
/// # Quick Start
/// In order to use the module sha1, you don't have to import (or use)
/// cryptocol::hash::sha1::* directly because the module cryptocol::hash::sha1
/// is re-exported. All you have to do is only import SHA1, SHA0, SHA1_Expanded,
/// SHA0_Expanded, SHA1_Generic_HR_fixed, SHA0_Generic_HR_fixed and/or
/// SHA1_Generic in the module cryptocol::hash. Example 1 shows how to import
/// structs SHA1, SHA0, SHA1_Expanded, SHA0_Expanded, SHA1_Generic_HR_fixed,
/// SHA0_Generic_HR_fixed and/or SHA1_Generic. Plus, what you have to know is
/// these. All the types (or structs) are the specific versions of SHA1_Generic.
/// Actually, SHA1 is a specific version of SHA1_Expanded. SHA0 is a specific
/// version of SHA0_Expanded. SHA1_Expanded, SHA0_Expanded,
/// SHA1_Generic_HR_fixed and SHA0_Generic_HR_fixed are specific versions
/// of MD5_Generic.
/// 
/// ## Example 1
/// ```
/// use cryptocol::hash::SHA1;
/// use cryptocol::hash::SHA0;
/// use cryptocol::hash::SHA1_Expanded;
/// use cryptocol::hash::SHA0_Expanded;
/// use cryptocol::hash::SHA1_Generic_HR_fixed;
/// use cryptocol::hash::SHA0_Generic_HR_fixed;
/// use cryptocol::hash::SHA1_Generic;
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
/// use cryptocol::hash::SHA1;
/// let mut hash = SHA1::new();
/// 
/// let mut txt = "";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.get_hash_value_in_string());
/// assert_eq!(hash.get_hash_value_in_string(), "DA39A3EE5E6B4B0D3255BFEF95601890AFD80709");
/// 
/// let txt_stirng = String::from("A");
/// hash.digest_string(&txt_stirng);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt_stirng, hash);
/// assert_eq!(hash.to_string(), "6DCD4CE23D88E2EE9568BA546C007C63D9131C1B");
/// 
/// let txt_array = ['W' as u8, 'o' as u8, 'w' as u8];
/// hash.digest_array(&txt_array);
/// println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txt_array, hash);
/// assert_eq!(hash.get_hash_value_in_string(), "0BBCDBD1616A1D2230100F629649DCF5B7A28B7F");
/// 
/// txt = "This data is 26-byte long.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.to_string(), "B82A61505779F6B3ACA4F5E0D54DA44C17375B49");
/// 
/// txt = "The unit of data length is not byte but bit.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.get_hash_value_in_string(), "C6DC54281357FC16D357E1D730BFC313C585DAEC");
/// 
/// txt = "I am testing SHA1 for the data whose length is sixty-two bytes.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.to_string(), "36CD36337097D764797091E5796B6FF45A9FA79F");
/// 
/// txt = "I am testing SHA-1 for the data whose length is sixty-four bytes.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.get_hash_value_in_string(), "E408F6B82DCDDB5EE6613A759AC1B13D0FA1CEF1");
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
/// 
/// # A Simple but Useful Application using cryptocol
/// The following is the source code of the commandline SHA1 hash value 
/// extractor using the struct SHA1 of this module. You can get the hash value
/// from a text or a file. The following source code assumes its executable
/// file name will be "sha1_app". You can find all the examples including the
/// following source code in the folder "examples" of this crate if you
/// download this crate from [github](https://github.com/piaoyh/cryptocol).
/// ```
/// use std::{ env, fs };
/// use cryptocol::hash::SHA1;
/// 
/// fn main()
/// {
///     let args: Vec<String> = env::args().collect();
///     if args.len() < 3
///     {
///         help();
///         return;
///     }
/// 
///     let arg = &args[1][..];
///     match arg
///     {
///         "--text" | "-t" =>  { get_hash_value_from_text(&args[2][..]); },
///         "--file" | "-f" =>  { get_hash_value_from_file(&args[2][..]); },
///         _ =>  { help(); },
///     }
/// }
/// 
/// fn get_hash_value_from_text(txt: &str)
/// {
///     let mut hash = SHA1::new();
///     hash.digest_str(txt);
///     println!("Hash value:\t{}", hash.get_hash_value_in_string());
/// }
/// 
/// fn get_hash_value_from_file(filename: &str)
/// {
///     if let Ok(contents) = fs::read(filename)
///     {
///         let mut hash = SHA1::new();
///         hash.digest_vec(&contents);
///         println!("Hash value:\t{}", hash.get_hash_value_in_string());
///     }
///     else
///     {
///         println!("File Error!");
///     }
/// }
/// 
/// fn help()
/// {
///     println!("This is an SHA1 hash value extractor from a text or a file, using cryptocol.");
///     println!("Usage: sha1_app <option> <source>");
///     println!("options       description");
///     println!("--text, -t    : <source> is given in text just after this option");
///     println!("                In this case, <source> is a text.");
///     println!("                The text should be enclosed by ' or \".");
///     println!("--file, -f    : <source> is given from a file the name of which is");
///     println!("                given just after this option");
///     println!("                In this case, <source> is a file name.");
///     println!("                If <source> is a file name without a path, the file");
///     println!("                will be found in the current directory.");
///     println!("                If <source> is a file name with a path, the file");
///     println!("                will be found in the directory of the path.");
///     println!("                The path can be either full path or relative path.");
///     println!("--help, -h    : print this help message on screen\n");
///     println!("Examples:");
///     println!("\tsha1_app -t 'How are you doing?'");
///     println!("\tsha1_app -f linuxmint-21.3-cinnamon-64bit.iso");
/// }
/// ```
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct SHA1_Generic<const N: usize = 5,
                        const H0: u32 = 0x67452301, const H1: u32 = 0xefcdab89,
                        const H2: u32 = 0x98badcfe, const H3: u32 = 0x10325476,
                        const H4: u32 = 0xc3d2e1f0, const ROUND: usize = 80, 
                        const K0: u32 = 0x5a827999, const K1: u32 = 0x6ed9eba1,
                        const K2: u32 = 0x8f1bbcdc, const K3: u32 = 0xca62c1d6,
                        const RL1: u32 = 1, const RL5: u32 = 5, const RL30: u32 = 30>
{
    hash_code: [IntUnion; 5],
}

impl<const N: usize, const H0: u32, const H1: u32, const H2: u32, const H3: u32,
        const H4: u32, const ROUND: usize, const K0: u32, const K1: u32,
        const K2: u32, const K3: u32, const RL1: u32, const RL5: u32, const RL30: u32>
SHA1_Generic<N, H0, H1, H2, H3, H4, ROUND, K0, K1, K2, K3, RL1, RL5, RL30>
{
    const K: [u32; 4] = [ K0, K1, K2, K3 ];
    const H: [u32; 5] = [ H0, H1, H2, H3, H4 ];

    // pub fn new() -> Self
    /// Constructs a new object of `SHA1` or `SHA0`, or SHA1-based hash object.
    /// 
    /// # Output
    /// A new object of `SHA1` or `SHA0`, or a new SHA1-based hash object.
    /// 
    /// # Initialization
    /// All the attributes of the constructed object, which is initial hash
    /// value, will be initialized with
    /// `67452301EFCDAB8998BADCFE10325476C3D2E1F0`.
    /// 
    /// # Example 1 for SHA1
    /// ```
    /// use cryptocol::hash::SHA1;
    /// let hash = SHA1::new();
    /// println!("Hash =\t{}", hash);
    /// assert_eq!(hash.to_string(), "67452301EFCDAB8998BADCFE10325476C3D2E1F0");
    /// ```
    /// 
    /// # Exmaple 2 for SHA1_Expanded
    /// ```
    /// use cryptocol::hash::SHA1_Expanded;
    /// type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    /// let my_hash = MySHA1::new();
    /// println!("Hash =\t{}", my_hash);
    /// assert_eq!(my_hash.to_string(), "111111114444444488888888CCCCCCCCFFFFFFFF");
    /// ```
    pub fn new() -> Self
    {
        if (N > 5) || (N == 0)
            { panic!("N cannot be either 0 or greater than 5. {}", N); }

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
    /// [digest_str()](struct@SHA1_Generic#method.digest_str)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of String
    /// object, you are highly recommended to use the method
    /// [digest_string()](struct@SHA1_Generic#method.digest_string)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    /// object, you are highly recommended to use the method
    /// [digest_array()](struct@SHA1_Generic#method.digest_array)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    /// object, you are highly recommended to use the method
    /// [digest_vec()](struct@SHA1_Generic#method.digest_array)
    /// rather than this method.
    ///
    /// # Example 1 for SHA1
    /// ```
    /// use cryptocol::hash::SHA1;
    /// let txt = "This is an example of the method digest().";
    /// let mut hash = SHA1::new();
    /// hash.digest(txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "9631162DFDAEAB89821256D4585D66D35CD61FD6");
    /// ```
    /// 
    /// # Example 2 for SHA1_Expanded
    /// ```
    /// use cryptocol::hash::SHA1_Expanded;
    /// type mySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    /// let txt = "This is an example of the method digest().";
    /// let mut my_hash = mySHA1::new();
    /// my_hash.digest(txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "FAF77A15CDEDFC6A33CE2C4003B119F225CBE414");
    /// ````
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
    /// Computess hash value.
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
    /// [digest_string()](struct@SHA1_Generic#method.digest_string)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    /// object, you are highly recommended to use the method
    /// [digest_array()](struct@SHA1_Generic#method.digest_array)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    /// object, you are highly recommended to use the method
    /// [digest_vec()](struct@SHA1_Generic#method.digest_array)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [digest()](struct@SHA1_Generic#method.digest) rather than this method.
    ///
    /// # Example 1 for SHA1
    /// ```
    /// use cryptocol::hash::SHA1;
    /// let mut hash = SHA1::new();
    /// let txt = "This is an example of the method digest_str().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "9FDE56BBB5028966CC2E7BDCD0758FE3121407E6");
    /// ```
    /// 
    /// # Example 2 for SHA1_Expanded
    /// ```
    /// use cryptocol::hash::SHA1_Expanded;
    /// type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    /// let mut my_hash = MySHA1::new();
    /// let txt = "This is an example of the method digest_str().";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "A6BE8FEA7E3F61508DC0A8BA85A0AEC77D0C0784");
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
    /// Computess hash value.
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
    /// [digest_str()](struct@SHA1_Generic#method.digest_str)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    /// object, you are highly recommended to use the method
    /// [digest_array()](struct@SHA1_Generic#method.digest_array)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    /// object, you are highly recommended to use the method
    /// [digest_vec()](struct@SHA1_Generic#method.digest_array)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [digest()](struct@SHA1_Generic#method.digest) rather than this method.
    ///
    /// # Example 1 for SHA1
    /// ```
    /// use cryptocol::hash::SHA1;
    /// let mut hash = SHA1::new();
    /// let txt = "This is an example of the method digest_string().".to_string();
    /// hash.digest_string(&txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "FDCDC0EBC9181B881BE1F15FECEBB9D70E4DDAAB");
    /// ```
    /// 
    /// # Example 2 for SHA1_Expanded
    /// ```
    /// use cryptocol::hash::SHA1_Expanded;
    /// type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    /// let mut my_hash = MySHA1::new();
    /// let txt = "This is an example of the method digest_string().".to_string();
    /// my_hash.digest_string(&txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "F4FE5C5A4D2A4BD414DDDF1FD32B185F3ED8AA32");
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
    /// [digest_str()](struct@SHA1_Generic#method.digest_str)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of String
    /// object, you are highly recommended to use the method
    /// [digest_string()](struct@SHA1_Generic#method.digest_string)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    /// object, you are highly recommended to use the method
    /// [digest_vec()](struct@SHA1_Generic#method.digest_array)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [digest()](struct@SHA1_Generic#method.digest) rather than this method.
    ///
    /// # Example 1 for SHA1
    /// ```
    /// use cryptocol::hash::SHA1;
    /// let mut hash = SHA1::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.digest_array(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "76BC87BAECA7725C948FD1C53766454FDA0867AF");
    /// ```
    /// 
    /// # Example 2 for SHA1_Expanded
    /// ```
    /// use cryptocol::hash::SHA1_Expanded;
    /// type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    /// let mut my_hash = MySHA1::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// my_hash.digest_array(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    /// assert_eq!(my_hash.to_string(), "A6E00DB72776DEBB7C6DB235024BB3E237E24D18");
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
    /// [digest_str()](struct@SHA1_Generic#method.digest_str)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of String
    /// object, you are highly recommended to use the method
    /// [digest_string()](struct@SHA1_Generic#method.digest_string)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    /// object, you are highly recommended to use the method
    /// [digest_array()](struct@SHA1_Generic#method.digest_array)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [digest()](struct@SHA1_Generic#method.digest) rather than this method.
    ///
    /// # Example 1 for SHA1
    /// ```
    /// use cryptocol::hash::SHA1;
    /// let mut hash = SHA1::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.digest_vec(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "76BC87BAECA7725C948FD1C53766454FDA0867AF");
    /// ```
    /// 
    /// # Example 2 for SHA1_Expanded
    /// ```
    /// use cryptocol::hash::SHA1_Expanded;
    /// type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    /// let mut my_hash = MySHA1::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// my_hash.digest_vec(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    /// assert_eq!(my_hash.to_string(), "A6E00DB72776DEBB7C6DB235024BB3E237E24D18");
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
    /// # Example 1 for SHA1
    /// ```
    /// use cryptocol::hash::SHA1;
    /// let mut hash = SHA1::new();
    /// let txt = "This is an example of the method ruminate().";
    /// hash.ruminate(2, txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "1E91427CF3BBB256A2BD44DA9F89D7406ED5D5FE");
    /// ```
    /// 
    /// # Example 2 for SHA1_Expanded
    /// ```
    /// use cryptocol::hash::SHA1_Expanded;
    /// type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    /// let mut hash = MySHA1::new();
    /// let txt = "This is an example of the method ruminate().";
    /// hash.ruminate(2, txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "509038D0447A5D05F4AD62C25AD6F9E130E694F4");
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
    /// # Example 1 for SHA1
    /// ```
    /// use cryptocol::hash::SHA1;
    /// let mut hash = SHA1::new();
    /// let txt = "This is an example of the method ruminate_str().";
    /// hash.ruminate_str(3, txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "778B3FF529024A46A3CC06F01CBE9078F6447BC0");
    /// ```
    /// 
    /// # Example 2 for SHA1_Expanded
    /// ```
    /// use cryptocol::hash::SHA1_Expanded;
    /// type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    /// let mut my_hash = MySHA1::new();
    /// let txt = "This is an example of the method ruminate_str().";
    /// my_hash.ruminate_str(3, txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "0CFDD49B87B844C4C329C997C1FB650EBEEA4909");
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
    /// # Example 1 for SHA1
    /// ```
    /// use cryptocol::hash::SHA1;
    /// let mut hash = SHA1::new();
    /// let txt = "This is an example of the method ruminate_string().".to_string();
    /// hash.ruminate_string(2, &txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "F4CE0B5A8D93BEB1C0A99F6290B26661C212A8B3");
    /// ```
    /// 
    /// # Example 2 for SHA1_Expanded
    /// ```
    /// use cryptocol::hash::SHA1_Expanded;
    /// type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    /// let mut my_hash = MySHA1::new();
    /// let txt = "This is an example of the method ruminate_string().".to_string();
    /// my_hash.ruminate_string(2, &txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "C4B55C59A15FCDFF6FFD39D3867665F67E89C8FC");
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
    /// # Example 1 for SHA1
    /// ```
    /// use cryptocol::hash::SHA1;
    /// let mut hash = SHA1::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.ruminate_array(5,&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "35BC04C66EBA9751C482FD98BCD1CBDC2C5E56AF");
    /// ```
    /// 
    /// # Example 2 for SHA1_Expanded
    /// ```
    /// use cryptocol::hash::SHA1_Expanded;
    /// type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    /// let mut my_hash = MySHA1::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// my_hash.ruminate_array(5,&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    /// assert_eq!(my_hash.to_string(), "898835EC92B5F7818A25C6645673DED30DA5F78D");
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
    /// # Example 1 for SHA1
    /// ```
    /// use cryptocol::hash::SHA1;
    /// let mut hash = SHA1::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.ruminate_vec(2, &data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "042811212E91F341473A43BF71BD8DA035D23032");
    /// ```
    /// 
    /// # Example 2 for SHA1_Expanded
    /// ```
    /// use cryptocol::hash::SHA1_Expanded;
    /// type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    /// let mut my_hash = MySHA1::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// my_hash.ruminate_vec(2, &data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    /// assert_eq!(my_hash.to_string(), "99B8709ACB93051C4CB238CE9CD9031BD40F2A2B");
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
    /// [get_hash_value_string()](struct@SHA1_Generic#method.get_hash_value_string)
    /// rather than this method.
    /// - If you want to get the hash value in the form of array object,
    /// you are highly recommended to use the method
    /// [get_hash_value_in_array()](struct@SHA1_Generic#method.get_hash_value_in_array)
    /// rather than this method.
    /// - If you want to get the hash value in the form of Vec object,
    /// you are highly recommended to use the method
    /// [get_hash_value_in_vec()](struct@SHA1_Generic#method.get_hash_value_in_vec)
    /// rather than this method.
    ///
    /// # Example 1 for SHA1
    /// ```
    /// use cryptocol::hash::SHA1;
    /// let mut hash = SHA1::new();
    /// let txt = "This is an example of the method get_hash_value().";
    /// let hash_value = [0_u8; 20];
    /// hash.digest_str(txt);
    /// hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    /// assert_eq!(format!("{:02X?}", hash_value), "[82, 62, 1B, E6, A6, 74, 88, 18, 12, 60, 5F, 27, C7, EF, 19, 38, 65, 39, 00, 8A]");
    /// ```
    /// 
    /// # Example 2 for MD5_Expanded
    /// ```
    /// use cryptocol::hash::SHA1_Expanded;
    /// type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    /// let mut my_hash = MySHA1::new();
    /// let txt = "This is an example of the method get_hash_value().";
    /// let hash_value = [0_u8; 20];
    /// my_hash.digest_str(txt);
    /// my_hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    /// assert_eq!(format!("{:02X?}", hash_value), "[F5, DD, 99, 0C, 9B, 5A, 4C, A3, 84, DF, B1, 3D, 73, 5A, CE, CF, 19, BB, 52, B4]");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn get_hash_value(&self, hash_value: *mut u8, length: usize)
    {
        const BYTES: usize = 4;
        let n_length = if length < (BYTES * N) {length} else {BYTES * N};
        #[cfg(target_endian = "little")]
        {
            let mut hash_code = [IntUnion::new(); N];
            for i in 0..N
                { hash_code[i].set(self.hash_code[i].get().to_be()); }
            unsafe { copy_nonoverlapping(hash_code.as_ptr() as *const u8, hash_value, n_length); }
        }
        #[cfg(target_endian = "big")]
        unsafe { copy_nonoverlapping(self.hash_code.as_ptr() as *const u8, hash_value, n_length); }
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
    /// [get_hash_value_in_array()](struct@SHA1_Generic#method.get_hash_value_in_array)
    /// rather than this method.
    /// - If you want to get the hash value in the form of Vec object,
    /// you are highly recommended to use the method
    /// [get_hash_value_in_vec()](struct@SHA1_Generic#method.get_hash_value_in_vec)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [get_hash_value()](struct@SHA1_Generic#method.get_hash_value)
    /// rather than this method.
    ///
    /// # Example 1 for SHA1
    /// ```
    /// use cryptocol::hash::SHA1;
    /// let mut hash = SHA1::new();
    /// let txt = "This is an example of the method get_hash_value_in_string().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_hash_value_in_string());
    /// assert_eq!(hash.get_hash_value_in_string(), "826621B45597FA1B58C855DFCDE111E7500BCC96");
    /// ```
    /// 
    /// # Example 2 for SHA1_Expanded
    /// ```
    /// use cryptocol::hash::SHA1_Expanded;
    /// type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    /// let mut my_hash = MySHA1::new();
    /// let txt = "This is an example of the method get_hash_value_in_string().";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.get_hash_value_in_string());
    /// assert_eq!(my_hash.get_hash_value_in_string(), "72CEC05D49E2FA7206E2BF5A6C9D38F0404E7956");
    /// ```
    /// 
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
                let byte = hs.get_ubyte_(BYTES-1-j);
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
    /// It returns [u32; 5].
    /// 
    /// # Panics
    /// If N > 5, this method will panic
    /// or its behaviour is undefined even if it won't panic.
    /// 
    /// # Counterpart Methods
    /// - If you want to get the hash value in the form of String object,
    /// you are highly recommended to use the method
    /// [get_hash_value_string()](struct@SHA1_Generic#method.get_hash_value_string)
    /// rather than this method.
    /// - If you want to get the hash value in the form of Vec object,
    /// you are highly recommended to use the method
    /// [get_hash_value_in_vec()](struct@SHA1_Generic#method.get_hash_value_in_vec)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [get_hash_value()](struct@SHA1_Generic#method.get_hash_value)
    /// rather than this method.
    ///
    /// # Example 1 for SHA1
    /// ```
    /// use cryptocol::hash::SHA1;
    /// let mut hash = SHA1::new();
    /// let txt = "This is an example of the method get_hash_value_in_array().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:02X?}", hash.get_hash_value_in_array()), "[7647F56F, 1508A320, 2303B1A8, D3BB7325, FC4497F8]");
    /// ```
    /// 
    /// # Example 2 for SHA1_Expanded
    /// ```
    /// use cryptocol::hash::SHA1_Expanded;
    /// type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    /// let mut my_hash = MySHA1::new();
    /// let txt = "This is an example of the method get_hash_value_in_array().";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, my_hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[C7DD61D1, 4E88AC6C, FFFC2A7E, C8E2DA66, 01BD283D]");
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
            { res[i] = self.hash_code[i].get().to_be(); }
        res
    }

    // pub fn get_hash_value_in_vec(&self) -> Vec
    /// Returns a hash value in the form of Vec object.
    /// 
    /// # Output
    /// It returns `Vec<u32>`.
    /// 
    /// # Counterpart Methods
    /// - If you want to get the hash value in the form of String object,
    /// you are highly recommended to use the method
    /// [get_hash_value_string()](struct@SHA1_Generic#method.get_hash_value_string)
    /// rather than this method.
    /// - If you want to get the hash value in the form of array object,
    /// you are highly recommended to use the method
    /// [get_hash_value_in_array()](struct@SHA1_Generic#method.get_hash_value_in_array)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [get_hash_value()](struct@SHA1_Generic#method.get_hash_value)
    /// rather than this method.
    ///
    /// # Example 1 for SHA1
    /// ```
    /// use cryptocol::hash::SHA1;
    /// let mut hash = SHA1::new();
    /// let txt = "This is an example of the method get_hash_value_in_vec().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_hash_value_in_vec());
    /// assert_eq!(format!("{:02X?}", hash.get_hash_value_in_vec()), "[58271E8F, 7E54E508, CF099E8F, 4D3B597B, D3BE3F42]");
    /// ```
    /// # Example 2 for SHA1_Expanded
    /// ```
    /// use cryptocol::hash::SHA1_Expanded;
    /// type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    /// let mut my_hash = MySHA1::new();
    /// let txt = "This is an example of the method get_hash_value_in_vec().";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_hash_value_in_vec());
    /// assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_vec()), "[DA959A5F, A8B581AD, FC006FB0, 9CCB3BCF, 7F4732F3]");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn get_hash_value_in_vec(&self) -> Vec<u32>
    {
        let mut res = Vec::new();
        for i in 0..N
            { res.push(self.hash_code[i].get().to_be()); }
        res
    }

    // pub fn put_hash_value_in_array<T, const M: usize>(&self, out: &mut [T; M])
    /// Puts a hash value in the form of array object.
    /// 
    /// # Argument
    /// `out` is the array [T; M] which is the place to put the hash value.
    /// 
    /// # Features
    /// If `M * mem::size_of::<T>()` > `mem::size_of::<u32>() * N`,
    /// it pass the output as the amount of `mem::size_of::<u32>() * N`.
    ///
    /// # Example 1 for SHA1
    /// ```
    /// use cryptocol::hash::SHA1;
    /// let mut hash = SHA1::new();
    /// let txt = "This is an example of the method put_hash_value_in_array().";
    /// let mut hash_code = [0_u32; 5];
    /// hash.digest_str(txt);
    /// hash.put_hash_value_in_array(&mut hash_code);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    /// assert_eq!(format!("{:08X?}", hash_code), "[BC02B27F, 99A5A1FB, A820CEC4, 19516BC8, E4D2A0D6]");
    /// ```
    /// 
    /// # Example 2 for SHA1_Expanded
    /// ```
    /// use cryptocol::hash::SHA1_Expanded;
    /// type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let mut my_hash = MySHA1::new();
    /// let txt = "This is an example of the method put_hash_value_in_array().";
    /// let mut hash_code = [0_u32; 5];
    /// my_hash.digest_str(txt);
    /// my_hash.put_hash_value_in_array(&mut hash_code);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    /// assert_eq!(format!("{:08X?}", hash_code), "[91EF4936, CFCF8F2D, C581EF30, 450E4E05, 0FBD39A7]");
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
        let length = if out_size < 20 {out_size} else {20};
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
    /// Example 1 for SHA1
    /// ```
    /// use cryptocol::hash::SHA1;
    /// let txt = "TANGLING";
    /// let mut hash = SHA1::new();
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[5B296514, 79D48A17, 1ADABF55, 09CC69B9, 83477776]");
    /// hash.tangle(1);
    /// println!("Hash =\t{:08X?}", hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[6D00CD91, 2A9BAD37, 210A8909, B6A83E2F, 5D986325]");
    /// hash.tangle(1);
    /// println!("Hash =\t{:08X?}", hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[E41C001F, 476FDC14, 1166767C, 3C09AE4D, 447B9B2F]");
    /// ```
    /// 
    /// Example 2 for SHA1_Expanded
    /// ```
    /// use cryptocol::hash::SHA1_Expanded;
    /// type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    /// let txt = "TANGLING";
    /// let mut my_hash = MySHA1::new();
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[570C0960, 44388BBA, 0DD84AC9, 2F78A2F8, E514D1FD]");
    /// my_hash.tangle(1);
    /// println!("Hash =\t{:08X?}", my_hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[AE8C42A9, 4CFC9130, FF606528, E4876633, 27FC359F]");
    /// my_hash.tangle(1);
    /// println!("Hash =\t{:08X?}", my_hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[2E33CBCF, 800599AD, 98827D7A, 41AA8BCB, D2D011FD]");
    /// ```
    #[inline]
    pub fn tangle(&mut self, tangling: u64)
    {
        let common = LongUnion::new_with(tangling);
        let mut m = [0_u32; 7];
        for i in 0..5
            { m[i] = self.hash_code[i].get(); }
        m[5] = common.get_uint_(0);
        m[6] = common.get_uint_(1);
        self.finalize(m.as_ptr() as *const u8, 28);
    }

    // fn initialize(&mut self)
    /// Initializes all five self.hash_code[] with predetermined values H[].
    fn initialize(&mut self)
    {
        for i in 0..5_usize
            { self.hash_code[i] = IntUnion::new_with(Self::get_h(i)); }
    }

    // fn update(&mut self, message: &[u32])
    /// This method is the core part of SHA1 hash algorithm.
    /// It has eighty rounds. It updates self.hash_code[] for those
    /// eighty rounds.
    fn update(&mut self, message: &[u32])
    {
        let mut w = [0_u32; 16];
        let mut a = self.hash_code[0].get();
        let mut b = self.hash_code[1].get();
        let mut c = self.hash_code[2].get();
        let mut d = self.hash_code[3].get();
        let mut e = self.hash_code[4].get();

        for i in 0..16_usize
        {
            w[i] = message[i].to_be();
            let f = Self::ff(b, c, d).wrapping_add(e)
                                .wrapping_add(Self::get_k(0))
                                .wrapping_add(w[i])
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
            w[j] = Self::get_w(&w, i);
            let (mut f, z) = Self::func(b, c, d, i);
            f = f.wrapping_add(e)
                .wrapping_add(Self::get_k(z))
                .wrapping_add(w[j])
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
            0 => { (Self::ff(x, y, z), r) }
            2 => { (Self::hh(x, y, z), r) }
            _ => { (Self::gg(x, y, z), r) }
        }
    }

	#[inline] fn get_k(idx: usize) -> u32    { Self::K[idx] }
	#[inline] fn get_h(idx: usize) -> u32    { Self::H[idx] }
    #[inline] fn get_w(w: &[u32; 16], idx: usize) -> u32   { (w[(idx-3) & 0b1111] ^ w[(idx-8) & 0b1111] ^ w[(idx-14) & 0b1111] ^ w[(idx-16) & 0b1111]).rotate_left(RL1) }
	#[inline] fn ff(x: u32, y: u32, z: u32) -> u32  { z ^ (x & (y ^ z)) }   // equivalent to { (x & y) | (!x & z) }
	#[inline] fn gg(x: u32, y: u32, z: u32) -> u32	{ x ^ y ^ z }
	#[inline] fn hh(x: u32, y: u32, z: u32) -> u32  { (x & y) | (z & (x | y)) } // equivalent to { (x & y) | (y & z) | (z & x) }
    #[inline] fn to_char(nibble: u8) -> char    { if nibble < 10  { ('0' as u8 + nibble) as u8 as char } else { ('A' as u8 - 10 + nibble) as char } }
}


impl<const N: usize, const H0: u32, const H1: u32, const H2: u32, const H3: u32,
        const H4: u32, const ROUND: usize, const K0: u32, const K1: u32, const K2: u32,
        const K3: u32, const RL1: u32, const RL5: u32, const RL30: u32>
Display for SHA1_Generic<N, H0, H1, H2, H3, H4, ROUND, K0, K1, K2, K3, RL1, RL5, RL30>
{
    /// Formats the value using the given formatter.
    /// You will hardly use this method directly.
    /// Automagically the function `to_string()` will be implemented. So, you
    /// can use the function `to_string()`, and you can also print the SHA-1
    /// object in the macro `println!()` directly for example.
    /// `f` is a buffer, this method must write the formatted string into it.
    /// [Read more](https://doc.rust-lang.org/core/fmt/trait.Display.html#tymethod.fmt)
    /// 
    /// # Example 1 for the method to_string() for SHA1
    /// ```
    /// use cryptocol::hash::SHA1;
    /// let mut hash = SHA1::new();
    /// let txt = "Display::fmt() automagically implement to_string().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
    /// assert_eq!(hash.to_string(), "8D0A6284BBFF4DE8D68962A924842C80959B0404");
    /// ```
    /// 
    /// # Example 2 for the method to_string() for SHA1_Expanded
    /// ```
    /// use cryptocol::hash::SHA1_Expanded;
    /// type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    /// let mut my_hash = MySHA1::new();
    /// let txt = "Display::fmt() automagically implement to_string().";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.to_string());
    /// assert_eq!(my_hash.to_string(), "54F0234F7188202D98EDDC643F71D95BEDE77ED7");
    /// ```
    /// 
    /// # Example 3 for the use in the macro println!() for SHA1
    /// ```
    /// use cryptocol::hash::SHA1;
    /// let mut hash = SHA1::new();
    /// let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "835CEFA297628E4DADBDA011C5FDEA68D88A8EE8");
    /// ```
    /// 
    /// # Example 4 for the use in the macro println!() for SHA1_Expanded
    /// ```
    /// use cryptocol::hash::SHA1_Expanded;
    /// type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    /// let mut my_hash = MySHA1::new();
    /// let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "78083F4E573928D6C4E9F869036F8A4D4D549E9F");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)
        write!(f, "{}", self.get_hash_value_in_string())
    }
}
