// Copyright 2023, 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module that contains SHA1 hash algorithm

#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

use std::fmt::{ self, Debug, Display, Formatter };
use std::ptr::copy_nonoverlapping;
use std::slice::from_raw_parts;

use crate::number::{ LongUnion, LongerUnion };
use crate::number::SmallUInt;


/// You have freedom of changing t, A5A5A5A5A5A5A5A5, H0 ~ H7, and ROUND
/// for SHA-2-512/t.
/// 
/// # Generic Parameters
/// You can create your own expanded version of SHA-2-512/t by changing the
/// generic parameters t, A5A5A5A5A5A5A5A5, H0 ~ H7, and ROUND.
/// - t : the truncating bit at which the output is constructed by truncating
/// the concatenation of h0 through h7. t should be multiple of 8 and equal to
/// or less than 512. If t is not a multiple of eight, t will be overestimated
/// to make a multiple of eight. The default value of t is 512.
/// - A5A5A5A5A5A5A5A5 : the hexadecimal constant with which its initial values
/// h0 through h7 have each been XORed.
/// The default value of A5A5A5A5A5A5A5A5 is 0xa5a5a5a5a5a5a5a5. 
/// - H0 ~ H7 : the initial hash values, eight u32 values.
/// The default values of H0 ~ H7 are defined to be first 64 bits of the
/// fractional parts of the square roots of the first 8 primes 2..19. So,
/// H0 ~ H7 are 0x6a09e667f3bcc908, 0xbb67ae8584caa73b, 0x3c6ef372fe94f82b,
/// 0xa54ff53a5f1d36f1, 0x510e527fade682d1, 0x9b05688c2b3e6c1f,
/// 0x1f83d9abfb41bd6b, and 0x5be0cd19137e2179, respectively (in big endian
/// representation).
/// - ROUND : the number of rounds. The default value of it is `80` (= 20 * 4).
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
pub type SHA2_512_t_Expanded<const t: usize = 512,
                const A5A5A5A5A5A5A5A5: u64 = 0xa5a5a5a5a5a5a5a5,
                const H0: u64 = 0x6a09e667f3bcc908, const H1: u64 = 0xbb67ae8584caa73b,
                const H2: u64 = 0x3c6ef372fe94f82b, const H3: u64 = 0xa54ff53a5f1d36f1,
                const H4: u64 = 0x510e527fade682d1, const H5: u64 = 0x9b05688c2b3e6c1f,
                const H6: u64 = 0x1f83d9abfb41bd6b, const H7: u64 = 0x5be0cd19137e2179,
                const ROUND: usize = 80>
    = SHA2_512_t_Generic<t, A5A5A5A5A5A5A5A5, H0, H1, H2, H3, H4, H5, H6, H7, ROUND>;

/// You have freedom of changing A5A5A5A5A5A5A5A5 and ROUND for HA-2-512/256.
/// 
/// # Generic Parameters
/// You can create your own expanded version of SHA-2-512/256 by changing the
/// generic parameters A5A5A5A5A5A5A5A5 and ROUND.
/// - A5A5A5A5A5A5A5A5 : the hexadecimal constant with which its initial values
/// h0 through h7 have each been XORed.
/// The default value of A5A5A5A5A5A5A5A5 is 0xa5a5a5a5a5a5a5a5.
/// - ROUND : the number of rounds. The default value of it is `80` (= 20 * 4).
#[allow(non_camel_case_types)]
pub type SHA2_512_t_256_Expanded<const A5A5A5A5A5A5A5A5: u64 = 0xa5a5a5a5a5a5a5a5,
                                const ROUND: usize = 80>
            = SHA2_512_t_Expanded<256, A5A5A5A5A5A5A5A5,
                                0x6a09e667f3bcc908, 0xbb67ae8584caa73b,
                                0x3c6ef372fe94f82b, 0xa54ff53a5f1d36f1,
                                0x510e527fade682d1, 0x9b05688c2b3e6c1f,
                                0x1f83d9abfb41bd6b, 0x5be0cd19137e2179, ROUND>;

/// You have freedom of changing A5A5A5A5A5A5A5A5 and ROUND for SHA-2-512/224.
/// 
/// # Generic Parameters
/// You can create your own expanded version of SHA-2-512/224 by changing the
/// generic parameters A5A5A5A5A5A5A5A5 and ROUND.
/// - A5A5A5A5A5A5A5A5 : the hexadecimal constant with which its initial values
/// h0 through h7 have each been XORed.
/// The default value of A5A5A5A5A5A5A5A5 is 0xa5a5a5a5a5a5a5a5.
/// - ROUND : the number of rounds. The default value of it is `80` (= 20 * 4).
#[allow(non_camel_case_types)]
pub type SHA2_512_t_224_Expanded<const A5A5A5A5A5A5A5A5: u64 = 0xa5a5a5a5a5a5a5a5,
                                const ROUND: usize = 80>
            = SHA2_512_t_Expanded<224, A5A5A5A5A5A5A5A5,
                                0x6a09e667f3bcc908, 0xbb67ae8584caa73b,
                                0x3c6ef372fe94f82b, 0xa54ff53a5f1d36f1,
                                0x510e527fade682d1, 0x9b05688c2b3e6c1f,
                                0x1f83d9abfb41bd6b, 0x5be0cd19137e2179, ROUND>;

/// You have freedom of changing t, ROUND, and K00 ~ K79 for SHA-2-512/t.
/// 
/// # Generic Parameters
/// You can create your own expanded version of SHA-2-512/t by changing the
/// generic parameters t, ROUND, and K00 ~ K79.
/// - t : the truncating bit at which the output is constructed by truncating
/// the concatenation of h0 through h7. t should be multiple of 8 and equal to
/// or less than 512. If t is not a multiple of eight, t will be overestimated
/// to make a multiple of eight. The default value of t is 512.
/// - ROUND : the number of rounds. The default value of it is `80` (= 20 * 4).
/// - K0 ~ K79 : the added values in hashing process, which are eighty u64
/// values and called round constants.
/// The default values of K0 ~ K79 are defined to be first 64 bits of the
/// fractional parts of the cube roots of the first 80 primes 2..409,
/// respectivey (in big endian representation). So, K0 ~ K79 are 
/// 0x428a2f98d728ae22, 0x7137449123ef65cd, 0xb5c0fbcfec4d3b2f, 0xe9b5dba58189dbbc,
/// 0x3956c25bf348b538, 0x59f111f1b605d019, 0x923f82a4af194f9b, 0xab1c5ed5da6d8118,
/// 0xd807aa98a3030242, 0x12835b0145706fbe, 0x243185be4ee4b28c, 0x550c7dc3d5ffb4e2,
/// 0x72be5d74f27b896f, 0x80deb1fe3b1696b1, 0x9bdc06a725c71235, 0xc19bf174cf692694,
/// 0xe49b69c19ef14ad2, 0xefbe4786384f25e3, 0x0fc19dc68b8cd5b5, 0x240ca1cc77ac9c65,
/// 0x2de92c6f592b0275, 0x4a7484aa6ea6e483, 0x5cb0a9dcbd41fbd4, 0x76f988da831153b5,
/// 0x983e5152ee66dfab, 0xa831c66d2db43210, 0xb00327c898fb213f, 0xbf597fc7beef0ee4,
/// 0xc6e00bf33da88fc2, 0xd5a79147930aa725, 0x06ca6351e003826f, 0x142929670a0e6e70,
/// 0x27b70a8546d22ffc, 0x2e1b21385c26c926, 0x4d2c6dfc5ac42aed, 0x53380d139d95b3df,
/// 0x650a73548baf63de, 0x766a0abb3c77b2a8, 0x81c2c92e47edaee6, 0x92722c851482353b,
/// 0xa2bfe8a14cf10364, 0xa81a664bbc423001, 0xc24b8b70d0f89791, 0xc76c51a30654be30,
/// 0xd192e819d6ef5218, 0xd69906245565a910, 0xf40e35855771202a, 0x106aa07032bbd1b8,
/// 0x19a4c116b8d2d0c8, 0x1e376c085141ab53, 0x2748774cdf8eeb99, 0x34b0bcb5e19b48a8,
/// 0x391c0cb3c5c95a63, 0x4ed8aa4ae3418acb, 0x5b9cca4f7763e373, 0x682e6ff3d6b2b8a3,
/// 0x748f82ee5defb2fc, 0x78a5636f43172f60, 0x84c87814a1f0ab72, 0x8cc702081a6439ec,
/// 0x90befffa23631e28, 0xa4506cebde82bde9, 0xbef9a3f7b2c67915, 0xc67178f2e372532b,
/// 0xca273eceea26619c, 0xd186b8c721c0c207, 0xeada7dd6cde0eb1e, 0xf57d4f7fee6ed178,
/// 0x06f067aa72176fba, 0x0a637dc5a2c898a6, 0x113f9804bef90dae, 0x1b710b35131c471b,
/// 0x28db77f523047d84, 0x32caab7b40c72493, 0x3c9ebe0a15c9bebc, 0x431d67c49c100d4c,
/// 0x4cc5d4becb3e42b6, 0x597f299cfc657e2a, 0x5fcb6fab3ad6faec, 0x6c44198c4a475817,
/// respectively (in big endian representation).
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
pub type SHA2_512_t_Generic_HRS_fixed<const t: usize = 512, const ROUND: usize = 80,
                const K00: u64 = 0x428a2f98d728ae22, const K01: u64 = 0x7137449123ef65cd, 
                const K02: u64 = 0xb5c0fbcfec4d3b2f, const K03: u64 = 0xe9b5dba58189dbbc,
                const K04: u64 = 0x3956c25bf348b538, const K05: u64 = 0x59f111f1b605d019, 
                const K06: u64 = 0x923f82a4af194f9b, const K07: u64 = 0xab1c5ed5da6d8118,
                const K08: u64 = 0xd807aa98a3030242, const K09: u64 = 0x12835b0145706fbe, 
                const K10: u64 = 0x243185be4ee4b28c, const K11: u64 = 0x550c7dc3d5ffb4e2,
                const K12: u64 = 0x72be5d74f27b896f, const K13: u64 = 0x80deb1fe3b1696b1, 
                const K14: u64 = 0x9bdc06a725c71235, const K15: u64 = 0xc19bf174cf692694,
                const K16: u64 = 0xe49b69c19ef14ad2, const K17: u64 = 0xefbe4786384f25e3, 
                const K18: u64 = 0x0fc19dc68b8cd5b5, const K19: u64 = 0x240ca1cc77ac9c65,
                const K20: u64 = 0x2de92c6f592b0275, const K21: u64 = 0x4a7484aa6ea6e483, 
                const K22: u64 = 0x5cb0a9dcbd41fbd4, const K23: u64 = 0x76f988da831153b5,
                const K24: u64 = 0x983e5152ee66dfab, const K25: u64 = 0xa831c66d2db43210, 
                const K26: u64 = 0xb00327c898fb213f, const K27: u64 = 0xbf597fc7beef0ee4,
                const K28: u64 = 0xc6e00bf33da88fc2, const K29: u64 = 0xd5a79147930aa725, 
                const K30: u64 = 0x06ca6351e003826f, const K31: u64 = 0x142929670a0e6e70,
                const K32: u64 = 0x27b70a8546d22ffc, const K33: u64 = 0x2e1b21385c26c926, 
                const K34: u64 = 0x4d2c6dfc5ac42aed, const K35: u64 = 0x53380d139d95b3df,
                const K36: u64 = 0x650a73548baf63de, const K37: u64 = 0x766a0abb3c77b2a8, 
                const K38: u64 = 0x81c2c92e47edaee6, const K39: u64 = 0x92722c851482353b,
                const K40: u64 = 0xa2bfe8a14cf10364, const K41: u64 = 0xa81a664bbc423001, 
                const K42: u64 = 0xc24b8b70d0f89791, const K43: u64 = 0xc76c51a30654be30,
                const K44: u64 = 0xd192e819d6ef5218, const K45: u64 = 0xd69906245565a910, 
                const K46: u64 = 0xf40e35855771202a, const K47: u64 = 0x106aa07032bbd1b8,
                const K48: u64 = 0x19a4c116b8d2d0c8, const K49: u64 = 0x1e376c085141ab53, 
                const K50: u64 = 0x2748774cdf8eeb99, const K51: u64 = 0x34b0bcb5e19b48a8,
                const K52: u64 = 0x391c0cb3c5c95a63, const K53: u64 = 0x4ed8aa4ae3418acb, 
                const K54: u64 = 0x5b9cca4f7763e373, const K55: u64 = 0x682e6ff3d6b2b8a3,
                const K56: u64 = 0x748f82ee5defb2fc, const K57: u64 = 0x78a5636f43172f60, 
                const K58: u64 = 0x84c87814a1f0ab72, const K59: u64 = 0x8cc702081a6439ec,
                const K60: u64 = 0x90befffa23631e28, const K61: u64 = 0xa4506cebde82bde9, 
                const K62: u64 = 0xbef9a3f7b2c67915, const K63: u64 = 0xc67178f2e372532b,
                const K64: u64 = 0xca273eceea26619c, const K65: u64 = 0xd186b8c721c0c207, 
                const K66: u64 = 0xeada7dd6cde0eb1e, const K67: u64 = 0xf57d4f7fee6ed178,
                const K68: u64 = 0x06f067aa72176fba, const K69: u64 = 0x0a637dc5a2c898a6, 
                const K70: u64 = 0x113f9804bef90dae, const K71: u64 = 0x1b710b35131c471b,
                const K72: u64 = 0x28db77f523047d84, const K73: u64 = 0x32caab7b40c72493, 
                const K74: u64 = 0x3c9ebe0a15c9bebc, const K75: u64 = 0x431d67c49c100d4c,
                const K76: u64 = 0x4cc5d4becb3e42b6, const K77: u64 = 0x597f299cfc657e2a, 
                const K78: u64 = 0x5fcb6fab3ad6faec, const K79: u64 = 0x6c44198c4a475817>
    = SHA2_512_t_Generic<t, 0xa5a5a5a5a5a5a5a5,
                            0x6a09e667f3bcc908, 0xbb67ae8584caa73b,
                            0x3c6ef372fe94f82b, 0xa54ff53a5f1d36f1,
                            0x510e527fade682d1, 0x9b05688c2b3e6c1f,
                            0x1f83d9abfb41bd6b, 0x5be0cd19137e2179,
                            ROUND,  
                            K00, K01, K02, K03, K04, K05, K06, K07,
                            K08, K09, K10, K11, K12, K13, K14, K15,
                            K16, K17, K18, K19, K20, K21, K22, K23,
                            K24, K25, K26, K27, K28, K29, K30, K31,
                            K32, K33, K34, K35, K36, K37, K38, K39,
                            K40, K41, K42, K43, K44, K45, K46, K47,
                            K48, K49, K50, K51, K52, K53, K54, K55,
                            K56, K57, K58, K59, K60, K61, K62, K63,
                            K64, K65, K66, K67, K68, K69, K70, K71,
                            K72, K73, K74, K75, K76, K77, K78, K79,
                            1, 8, 14, 18, 19, 28, 34, 39, 41, 61, 6, 7>;

/// You have freedom of changing t for SHA-2-512/t.
/// 
/// # Generic Parameters
/// You can create your own expanded version of SHA-2-512/t by changing the
/// generic parameter t.
/// - t : the truncating bit at which the output is constructed by truncating
/// the concatenation of h0 through h7. t should be multiple of 8 and equal to
/// or less than 512. If t is not a multiple of eight, t will be overestimated
/// to make a multiple of eight. The default value of t is 512.
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
pub type SHA2_512_t<const t: usize = 512> = SHA2_512_t_Generic<t>;
// equivalent to `pub type SHA2_512_t<const t: usize = 512> = SHA2_512_t_Expanded<t>;`

/// The official SHA-512/256 hash algorithm
#[allow(non_camel_case_types)]
pub type SHA2_512_t_256 = SHA2_512_t_Generic<256>;
// equivalent to `pub type SHA2_512_t_256 = SHA2_512_t_256_Expanded;`

/// The official SHA-512/224 hash algorithm
#[allow(non_camel_case_types)]
pub type SHA2_512_t_224 = SHA2_512_t_Generic<224>;
// equivalent to `pub type SHA2_512_t_224 = SHA2_512_t_224_Expanded;`

/// The simulation of the official SHA-512 hash algorithm
#[allow(non_camel_case_types)]
pub type SHA2_512_0 = SHA2_512_t_Generic;
// equivalent to `pub type SHA2_512_0 = SHA2_512_t;`


/// SHA-2-512/t message-digest algorithm that lossily compresses data of
/// arbitrary length into a any-bit hash values less than 512 bits, and
/// its flexible variants that allows you to develop your own
/// `SHA-2-512/t`-based hash algorithms
/// 
/// # Introduction
/// Keccak was designed by the United States National Security Agency,
/// and are a U.S. Federal Information Processing Standard. SHA-2-512/t
/// produces a message digest based on principles similar to those used by
/// Ronald L. Rivest of MIT in the design of the MD2, MD4, MD5, SHA0, SHA-1,
/// SHA-2-256, SHA-2-224, SHA-2-512. and SHA-2-512-384 message digest
/// algorithms, but generates a flexible hash value (t bits vs. 256, 224, 160
/// bits and 128 bits). SHA-2-512/t uses the
/// [Merkle–Damgård construction](https://en.wikipedia.org/wiki/Merkle%E2%80%93Damg%C3%A5rd_construction).
/// 
/// # Vulnerability
/// There have been several attacks against Keccak
/// but they are all incomplete attacks.
/// Read [more](https://en.wikipedia.org/wiki/SHA-2#Cryptanalysis_and_validation)
/// 
/// # Use of SHA-2-512/t and their variations
/// You can use SHA-2-512/t for cryptographic purposes such as:
/// - Generating IDs
/// - Integrity test
/// - Storing passwords
/// - Digital Signature
/// - Key generation
/// - Implementing proof of work for block chain.
/// - Study of hash algorithms
/// - Cryptanalysis Research to find the weakness of SHA-512/t and Merkle-Damgard
/// construction which MD2, MD4, MD5, SHA0, SHA1, and all SHA2 family use
/// 
/// # Generic Parameters
/// You can create your own expanded version of SHA-2-512/t by changing the
/// generic parameters t, A5A5A5A5A5A5A5A5, H0 ~ H7, ROUND, K00 ~ K79, RR1,
/// RR8, RR14, RR18, RR19, RR28, RR34, RR39, RR41, RR61, SR6, and SR7.
/// - t : the truncating bit at which the output is constructed by truncating
/// the concatenation of h0 through h7. t should be multiple of 8 and equal to
/// or less than 512. If t is not a multiple of eight, t will be overestimated
/// to make a multiple of eight. The default value of t is 512.
/// - A5A5A5A5A5A5A5A5 : the hexadecimal constant with which its initial values
/// h0 through h7 have each been XORed.
/// The default value of A5A5A5A5A5A5A5A5 is 0xa5a5a5a5a5a5a5a5. 
/// - H0 ~ H7 : the initial hash values, eight u32 values.
/// The default values of H0 ~ H7 are defined to be first 64 bits of the
/// fractional parts of the square roots of the first 8 primes 2..19. So,
/// H0 ~ H7 are 0x6a09e667f3bcc908, 0xbb67ae8584caa73b, 0x3c6ef372fe94f82b,
/// 0xa54ff53a5f1d36f1, 0x510e527fade682d1, 0x9b05688c2b3e6c1f,
/// 0x1f83d9abfb41bd6b, and 0x5be0cd19137e2179, respectively (in big endian
/// representation).
/// - ROUND : the number of rounds. The default value of it is `80` (= 20 * 4).
/// - K0 ~ K79 : the added values in hashing process, which are eighty u64
/// values and called round constants.
/// The default values of K0 ~ K79 are defined to be first 64 bits of the
/// fractional parts of the cube roots of the first 80 primes 2..409,
/// respectivey (in big endian representation). So, K0 ~ K79 are 
/// 0x428a2f98d728ae22, 0x7137449123ef65cd, 0xb5c0fbcfec4d3b2f, 0xe9b5dba58189dbbc,
/// 0x3956c25bf348b538, 0x59f111f1b605d019, 0x923f82a4af194f9b, 0xab1c5ed5da6d8118,
/// 0xd807aa98a3030242, 0x12835b0145706fbe, 0x243185be4ee4b28c, 0x550c7dc3d5ffb4e2,
/// 0x72be5d74f27b896f, 0x80deb1fe3b1696b1, 0x9bdc06a725c71235, 0xc19bf174cf692694,
/// 0xe49b69c19ef14ad2, 0xefbe4786384f25e3, 0x0fc19dc68b8cd5b5, 0x240ca1cc77ac9c65,
/// 0x2de92c6f592b0275, 0x4a7484aa6ea6e483, 0x5cb0a9dcbd41fbd4, 0x76f988da831153b5,
/// 0x983e5152ee66dfab, 0xa831c66d2db43210, 0xb00327c898fb213f, 0xbf597fc7beef0ee4,
/// 0xc6e00bf33da88fc2, 0xd5a79147930aa725, 0x06ca6351e003826f, 0x142929670a0e6e70,
/// 0x27b70a8546d22ffc, 0x2e1b21385c26c926, 0x4d2c6dfc5ac42aed, 0x53380d139d95b3df,
/// 0x650a73548baf63de, 0x766a0abb3c77b2a8, 0x81c2c92e47edaee6, 0x92722c851482353b,
/// 0xa2bfe8a14cf10364, 0xa81a664bbc423001, 0xc24b8b70d0f89791, 0xc76c51a30654be30,
/// 0xd192e819d6ef5218, 0xd69906245565a910, 0xf40e35855771202a, 0x106aa07032bbd1b8,
/// 0x19a4c116b8d2d0c8, 0x1e376c085141ab53, 0x2748774cdf8eeb99, 0x34b0bcb5e19b48a8,
/// 0x391c0cb3c5c95a63, 0x4ed8aa4ae3418acb, 0x5b9cca4f7763e373, 0x682e6ff3d6b2b8a3,
/// 0x748f82ee5defb2fc, 0x78a5636f43172f60, 0x84c87814a1f0ab72, 0x8cc702081a6439ec,
/// 0x90befffa23631e28, 0xa4506cebde82bde9, 0xbef9a3f7b2c67915, 0xc67178f2e372532b,
/// 0xca273eceea26619c, 0xd186b8c721c0c207, 0xeada7dd6cde0eb1e, 0xf57d4f7fee6ed178,
/// 0x06f067aa72176fba, 0x0a637dc5a2c898a6, 0x113f9804bef90dae, 0x1b710b35131c471b,
/// 0x28db77f523047d84, 0x32caab7b40c72493, 0x3c9ebe0a15c9bebc, 0x431d67c49c100d4c,
/// 0x4cc5d4becb3e42b6, 0x597f299cfc657e2a, 0x5fcb6fab3ad6faec, 0x6c44198c4a475817,
/// respectively (in big endian representation).
/// - RR1, RR8, RR14, RR18, RR19, RR28, RR34, RR39, RR41, and RR61 : the amounts
/// of rotate right in the hashing process.
/// The default values of RR1, RR8, RR14, RR18, RR19, RR28, RR34, RR39, RR41,
/// and RR61 are 1, 8, 14, 18, 19, 28, 34, 39, 41, and 61, respecively.
/// - SR6 and SR7 : the amounts of shift right in the hashing process.
/// The default values of SR6 and SR7 are 6 and 7 respectively.
/// 
/// About the parameters and their default values,
/// read [more](https://en.wikipedia.org/wiki/SHA-2#Pseudocode)
/// 
/// # Security of your own expanded version
/// Your own algrorithm based on SHA-2-512/t may be stronger or weaker than
/// official SHA-2-512/t. Unless you seriously checked the cryptographic
/// security of your own algorithms, it is hard to assume that your own
/// alogrithms are stronger than the official SHA-2-512/t.
/// 
/// Read [this](https://doi.org/10.6028/NIST.FIPS.180-4)
/// and [that](https://en.wikipedia.org/wiki/SHA-2)
/// 
/// # Quick Start
/// In order to use the module sha2_512_t, you don't have to import (or use)
/// cryptocol::hash::sha2_512_t::* directly because the module
/// cryptocol::hash::sha2_512_t is re-exported. All you have to do is only
/// import SHA2_512_t, SHA2_512_t_256, SHA2_512_t_224, SHA2_512_0,
/// SHA2_512_t_Expanded, SHA2_512_t_256_Expanded, SHA2_512_t_224_Expanded,
/// SHA2_512_t_Generic_HRS_fixed, and/or SHA2_512_t_Generic in the module
/// cryptocol::hash. Example 1 shows how to import structs SHA2_512_t,
/// SHA2_512_t_256, SHA2_512_t_224, SHA2_512_0, SHA2_512_t_Expanded,
/// SHA2_512_t_256_Expanded, SHA2_512_t_224_Expanded,
/// SHA2_512_t_Generic_HRS_fixed, and/or SHA2_512_t_Generic. Plus, what you
/// have to know is these. All the types (or structs) are the specific versions
/// of SHA2_512_t_Generic. Actually, SHA2_512_0 is a specific version of
/// SHA2_512_t. SHA2_512_t_256 is a specific version of SHA2_512_t_256_Expanded.
/// SHA2_512_t_224 is a specific version of SHA2_512_t_224_Expanded. 
/// SHA2_512_t, SHA2_512_t_256_Expanded, and SHA2_512_t_224_Expanded
/// are specific versions of SHA2_512_t_Expanded. SHA2_512_t_Expanded and
/// SHA2_512_t_Generic_HRS_fixed are specific versions of SHA2_512_t_Generic.
/// 
/// ## Example 1
/// ```
/// use cryptocol::hash::SHA2_512_0;
/// use cryptocol::hash::SHA2_512_t;
/// use cryptocol::hash::SHA2_512_t_256;
/// use cryptocol::hash::SHA2_512_t_224;
/// use cryptocol::hash::SHA2_512_t_Expanded;
/// use cryptocol::hash::SHA2_512_t_256_Expanded;
/// use cryptocol::hash::SHA2_512_t_224_Expanded;
/// use cryptocol::hash::SHA2_512_t_Generic_HRS_fixed;
/// use cryptocol::hash::SHA2_512_t_Generic;
/// ```
/// Then, you can create SHA1 object by the method SHA1::new() for example.
/// Now, you are ready to use all prepared methods to hash any data. If you
/// want to hash a string, for example, you can use the method digest_str().
/// Then, the SHA1 object that you created will contain its hash value. You can
/// use the macro println!() for instance to print on a commandline screen by
/// `println!("{}", hash)` where hash is the SHA1 object.
/// Example 2 shows how to use SHA1 struct quickly.
/// 
/// ## Example 2 for SHA-512/256
/// ```
/// use std::string::*;
/// use cryptocol::hash::SHA2_512_t_256;
/// let mut hash = SHA2_512_t_256::new();
/// 
/// let mut txt = "";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.get_hash_value_in_string());
/// assert_eq!(hash.get_hash_value_in_string(), "C672B8D1EF56ED28AB87C3622C5114069BDD3AD7B8F9737498D0C01ECEF0967A");
/// 
/// let txt_stirng = String::from("A");
/// hash.digest_string(&txt_stirng);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt_stirng, hash);
/// assert_eq!(hash.to_string(), "65A992AD19967492B5780D76A4733AF553F796F688B79102D01EC7FDE5590CAB");
/// 
/// let txt_array = ['W' as u8, 'o' as u8, 'w' as u8];
/// hash.digest_array(&txt_array);
/// println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txt_array, hash);
/// assert_eq!(hash.get_hash_value_in_string(), "E4AF36E824AFDB9E42291983AFA292B894DED2CCAFCCF53346B223FCA846694D");
/// 
/// txt = "The length of this message is forty-eight bytes.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.to_string(), "4E730BDADF49EC9F3E920F72EAD3AC8D09B459900BE4F6E27848652632277205");
/// 
/// txt = "The unit of the message length is not byte but bit.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.get_hash_value_in_string(), "AE0EAB6824897F575FCC051DBC2D1AA7F7BF0DB2C80172F639CE20B3B498C9D5");
/// 
/// txt = "This algorithm SHA-2/512/256 is being tested with this message, the length of which is one hundred twelve bytes.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.to_string(), "7876C6F1285C4B6EC6A2F4A76BBF81815B470536F3A38B7028AA88A3C5C31651");
/// 
/// txt = "This algorithm SHA-2/512/256 is being tested for this message the length of which is one hundred sixty-nine long so as to check whether or not this algorithm works well.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.get_hash_value_in_string(), "6FCE377EA6116BEAC9C11606C59A5D034C8C6EF5A1920B783A9097E07BE36D31");
/// 
/// txt = "This algorithm SHA-2/512/256 is being tested with this message the length of which is two hundred ninety-seven long so that whether or not this algorithm works well is checked. The message is 'Do you see a man skilled in his work? He will serve before kings; he will not serve before obscure men.'";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
/// assert_eq!(hash.to_string(), "63FD06E11EF67F0F5EF598C3B2F2E221D5557AD1EEA46156D1B657F1EDF08D5D");
/// ```
/// 
/// ## Example 3 for SHA-512/224
/// ```
/// use std::string::*;
/// use cryptocol::hash::SHA2_512_t_224;
/// let mut hash = SHA2_512_t_224::new();
/// 
/// let mut txt = "";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.get_hash_value_in_string());
/// assert_eq!(hash.get_hash_value_in_string(), "6ED0DD02806FA89E25DE060C19D3AC86CABB87D6A0DDD05C333B84F4");
/// 
/// let txt_stirng = String::from("A");
/// hash.digest_string(&txt_stirng);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt_stirng, hash);
/// assert_eq!(hash.to_string(), "1DEF1E6A5344538A07A3C93A3A765FA1D2859A576947791A9047C3E6");
/// 
/// let txt_array = ['W' as u8, 'o' as u8, 'w' as u8];
/// hash.digest_array(&txt_array);
/// println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txt_array, hash);
/// assert_eq!(hash.get_hash_value_in_string(), "021B7E0CFE3FBD598CF0366464AEB4C93A900BBA1DF8CADB5F611345");
/// 
/// txt = "The length of this message is forty-eight bytes.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.to_string(), "1E6EEBF17E8B2B1D2A41B14D9813561E44814E35F01119ED7BA3E19F");
/// 
/// txt = "The unit of the message length is not byte but bit.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.get_hash_value_in_string(), "5251D628FE99DA19238D277DF9AC03382249FF3830AD764EF0A68CDA");
/// 
/// txt = "This algorithm SHA-2/512/224 is being tested with this message, the length of which is one hundred twelve bytes.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.to_string(), "225B3D39D9B91705E7C08DBBF66E5F34E88554685C78AF2535FD3CE2");
/// 
/// txt = "This algorithm SHA-2/512/224 is being tested for this message the length of which is one hundred sixty-nine long so as to check whether or not this algorithm works well.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.get_hash_value_in_string(), "3DD5D6503AFE8247B37AFD72DFD56718E6CA70D0B425739928885D0F");
/// 
/// txt = "This algorithm SHA-2/512/224 is being tested with this message the length of which is two hundred ninety-seven long so that whether or not this algorithm works well is checked. The message is 'Do you see a man skilled in his work? He will serve before kings; he will not serve before obscure men.'";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
/// assert_eq!(hash.to_string(), "D709EC6C2CAA1DAC61B0121675C3B131C23209F9E9ABC60392D99F52");
/// ```
/// # Big-endian issue
/// It is just experimental for Big Endian CPUs. So, you are not encouraged
/// to use it for Big Endian CPUs for serious purpose. Only use this crate
/// for Big-endian CPUs with your own full responsibility.
/// 
/// # A Simple but Useful Application using cryptocol
/// The following is the source code of the commandline SHA2_512_224 hash value
/// extractor using the struct SHA2_512_224 of this module. You can get the
/// hash value from a text or a file. The following source code assumes its
/// executable file name will be "sha2_512_224_app". You can find all the
/// examples including the following source code in the folder "examples"
/// of this crate if you download this crate from
/// [github](https://github.com/piaoyh/cryptocol).
/// ```
/// use std::{ io, env, fs };
/// use std::io::BufRead;
/// use std::convert::From;
/// use cryptocol::hash::SHA2_512_t_224;
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
///         "--check" | "-c" => { check_files(&args[2][..]) },
///         _ =>  { help(); },
///     }
/// }
/// 
/// fn get_hash_value_from_text(txt: &str)
/// {
///     let mut hash = SHA2_512_t_224::new();
///     hash.digest_str(txt);
///     println!("Hash value:\t{}", hash.get_hash_value_in_string());
/// }
/// 
/// fn get_hash_value_from_file(file: &str)
/// {
///     if let Ok(contents) = fs::read(file)
///     {
///         let mut hash = SHA2_512_t_224::new();
///         hash.digest_vec(&contents);
///         println!("Hash value:\t{}", hash.get_hash_value_in_string());
///     }
///     else
///     {
///         println!("File Error!");
///     }
/// }
/// 
/// fn check_files(file_list: &str)
/// {
///     let mut reader;
///     match fs::File::open(file_list)
///     {
///         Ok(file) => {
///                 reader = io::BufReader::new(file);
///                 let mut line = String::new();
///                 while let Ok(n) = reader.read_line(&mut line)
///                 {
///                     if n == 0
///                         { break; }
///                     let txt = line.trim();
///                     if txt.chars().nth(0).unwrap() == '#'
///                     { 
///                         line.clear();
///                         continue;
///                     }
///                     let elem: Vec<&str> = txt.split_whitespace().collect();
///                     let item = elem[0];
///                     let h = String::from(elem[1]).to_uppercase();
///                     if let Ok(contents) = fs::read(item)
///                     {
///                         let mut hash = SHA2_512_t_224::new();
///                         hash.digest_vec(&contents);
///                         if hash.to_string() == h
///                             { println!("{} ---> OK", item); }
///                         else
///                             { println!("{} ---> Corrupted", item); }
///                     }
///                     line.clear();
///                 }
///             },
///         _ => {
///                 println!("File open error");
///                 return;
///             }
///     }
/// }
/// 
/// fn help()
/// {
///     println!("This is an SHA2_512_t_224 hash value extractor from a text or a file, using cryptocol.");
///     println!("Usage: sha2_512_224_app <option> <source>");
///     println!("options       description");
///     println!("--text, -t    : <source> is a text to get a hash code.");
///     println!("                The text should be enclosed by ' or \".");
///     println!("--file, -f    : <source> is the name of the file to get a hash code.");
///     println!("--check, -c   : <source> is the name of the file that contains pairs");
///     println!("                of file and its hash code.");
///     println!("--help, -h    : print this help message on screen\n");
///     println!("Examples:");
///     println!("\tsha2_512_224_app -t 'How are you doing?'");
///     println!("\tsha2_512_224_app -f linuxmint-21.3-cinnamon-64bit.iso");
///     println!("\tsha2_512_224_app -c CHECKSUM");
/// }
/// ```
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
pub struct SHA2_512_t_Generic<const t: usize = 512, 
                const A5A5A5A5A5A5A5A5: u64 = 0xa5a5a5a5a5a5a5a5, 
                const H0: u64 = 0x6a09e667f3bcc908, const H1: u64 = 0xbb67ae8584caa73b, 
                const H2: u64 = 0x3c6ef372fe94f82b, const H3: u64 = 0xa54ff53a5f1d36f1,
                const H4: u64 = 0x510e527fade682d1, const H5: u64 = 0x9b05688c2b3e6c1f, 
                const H6: u64 = 0x1f83d9abfb41bd6b, const H7: u64 = 0x5be0cd19137e2179,
                const ROUND: usize = 80,
                const K00: u64 = 0x428a2f98d728ae22, const K01: u64 = 0x7137449123ef65cd, 
                const K02: u64 = 0xb5c0fbcfec4d3b2f, const K03: u64 = 0xe9b5dba58189dbbc,
                const K04: u64 = 0x3956c25bf348b538, const K05: u64 = 0x59f111f1b605d019, 
                const K06: u64 = 0x923f82a4af194f9b, const K07: u64 = 0xab1c5ed5da6d8118,
                const K08: u64 = 0xd807aa98a3030242, const K09: u64 = 0x12835b0145706fbe, 
                const K10: u64 = 0x243185be4ee4b28c, const K11: u64 = 0x550c7dc3d5ffb4e2,
                const K12: u64 = 0x72be5d74f27b896f, const K13: u64 = 0x80deb1fe3b1696b1, 
                const K14: u64 = 0x9bdc06a725c71235, const K15: u64 = 0xc19bf174cf692694,
                const K16: u64 = 0xe49b69c19ef14ad2, const K17: u64 = 0xefbe4786384f25e3, 
                const K18: u64 = 0x0fc19dc68b8cd5b5, const K19: u64 = 0x240ca1cc77ac9c65,
                const K20: u64 = 0x2de92c6f592b0275, const K21: u64 = 0x4a7484aa6ea6e483, 
                const K22: u64 = 0x5cb0a9dcbd41fbd4, const K23: u64 = 0x76f988da831153b5,
                const K24: u64 = 0x983e5152ee66dfab, const K25: u64 = 0xa831c66d2db43210, 
                const K26: u64 = 0xb00327c898fb213f, const K27: u64 = 0xbf597fc7beef0ee4,
                const K28: u64 = 0xc6e00bf33da88fc2, const K29: u64 = 0xd5a79147930aa725, 
                const K30: u64 = 0x06ca6351e003826f, const K31: u64 = 0x142929670a0e6e70,
                const K32: u64 = 0x27b70a8546d22ffc, const K33: u64 = 0x2e1b21385c26c926, 
                const K34: u64 = 0x4d2c6dfc5ac42aed, const K35: u64 = 0x53380d139d95b3df,
                const K36: u64 = 0x650a73548baf63de, const K37: u64 = 0x766a0abb3c77b2a8, 
                const K38: u64 = 0x81c2c92e47edaee6, const K39: u64 = 0x92722c851482353b,
                const K40: u64 = 0xa2bfe8a14cf10364, const K41: u64 = 0xa81a664bbc423001, 
                const K42: u64 = 0xc24b8b70d0f89791, const K43: u64 = 0xc76c51a30654be30,
                const K44: u64 = 0xd192e819d6ef5218, const K45: u64 = 0xd69906245565a910, 
                const K46: u64 = 0xf40e35855771202a, const K47: u64 = 0x106aa07032bbd1b8,
                const K48: u64 = 0x19a4c116b8d2d0c8, const K49: u64 = 0x1e376c085141ab53, 
                const K50: u64 = 0x2748774cdf8eeb99, const K51: u64 = 0x34b0bcb5e19b48a8,
                const K52: u64 = 0x391c0cb3c5c95a63, const K53: u64 = 0x4ed8aa4ae3418acb, 
                const K54: u64 = 0x5b9cca4f7763e373, const K55: u64 = 0x682e6ff3d6b2b8a3,
                const K56: u64 = 0x748f82ee5defb2fc, const K57: u64 = 0x78a5636f43172f60, 
                const K58: u64 = 0x84c87814a1f0ab72, const K59: u64 = 0x8cc702081a6439ec,
                const K60: u64 = 0x90befffa23631e28, const K61: u64 = 0xa4506cebde82bde9, 
                const K62: u64 = 0xbef9a3f7b2c67915, const K63: u64 = 0xc67178f2e372532b,
                const K64: u64 = 0xca273eceea26619c, const K65: u64 = 0xd186b8c721c0c207, 
                const K66: u64 = 0xeada7dd6cde0eb1e, const K67: u64 = 0xf57d4f7fee6ed178,
                const K68: u64 = 0x06f067aa72176fba, const K69: u64 = 0x0a637dc5a2c898a6, 
                const K70: u64 = 0x113f9804bef90dae, const K71: u64 = 0x1b710b35131c471b,
                const K72: u64 = 0x28db77f523047d84, const K73: u64 = 0x32caab7b40c72493, 
                const K74: u64 = 0x3c9ebe0a15c9bebc, const K75: u64 = 0x431d67c49c100d4c,
                const K76: u64 = 0x4cc5d4becb3e42b6, const K77: u64 = 0x597f299cfc657e2a, 
                const K78: u64 = 0x5fcb6fab3ad6faec, const K79: u64 = 0x6c44198c4a475817,
                const RR1: u32 = 1, const RR8: u32 = 8,  const RR14: u32 = 14,
                const RR18: u32 = 18, const RR19: u32 = 19, const RR28: u32 = 28, 
                const RR34: u32 = 34, const RR39: u32 = 39, const RR41: u32 = 41,
                const RR61: u32 = 61, const SR6: i32 = 6, const SR7: i32 = 7>
{
    hash_code: [LongUnion; 8],
    o: [u64; 8],
}

#[allow(non_upper_case_globals)]
impl<const t: usize, const A5A5A5A5A5A5A5A5: u64,
    const H0: u64, const H1: u64, const H2: u64, const H3: u64,
    const H4: u64, const H5: u64, const H6: u64, const H7: u64,
    const ROUND: usize,
    const K00: u64, const K01: u64, const K02: u64, const K03: u64,
    const K04: u64, const K05: u64, const K06: u64, const K07: u64,
    const K08: u64, const K09: u64, const K10: u64, const K11: u64,
    const K12: u64, const K13: u64, const K14: u64, const K15: u64,
    const K16: u64, const K17: u64, const K18: u64, const K19: u64,
    const K20: u64, const K21: u64, const K22: u64, const K23: u64,
    const K24: u64, const K25: u64, const K26: u64, const K27: u64,
    const K28: u64, const K29: u64, const K30: u64, const K31: u64,
    const K32: u64, const K33: u64, const K34: u64, const K35: u64,
    const K36: u64, const K37: u64, const K38: u64, const K39: u64,
    const K40: u64, const K41: u64, const K42: u64, const K43: u64,
    const K44: u64, const K45: u64, const K46: u64, const K47: u64,
    const K48: u64, const K49: u64, const K50: u64, const K51: u64,
    const K52: u64, const K53: u64, const K54: u64, const K55: u64,
    const K56: u64, const K57: u64, const K58: u64, const K59: u64,
    const K60: u64, const K61: u64, const K62: u64, const K63: u64,
    const K64: u64, const K65: u64, const K66: u64, const K67: u64,
    const K68: u64, const K69: u64, const K70: u64, const K71: u64,
    const K72: u64, const K73: u64, const K74: u64, const K75: u64,
    const K76: u64, const K77: u64, const K78: u64, const K79: u64,
    const RR1: u32, const RR8: u32, const RR14: u32, const RR18: u32, 
    const RR19: u32, const RR28: u32, const RR34: u32, const RR39: u32, 
    const RR41: u32, const RR61: u32, const SR6: i32, const SR7: i32>
SHA2_512_t_Generic<t, A5A5A5A5A5A5A5A5,
                    H0, H1, H2, H3, H4, H5, H6, H7, ROUND,
                    K00, K01, K02, K03, K04, K05, K06, K07,
                    K08, K09, K10, K11, K12, K13, K14, K15,
                    K16, K17, K18, K19, K20, K21, K22, K23,
                    K24, K25, K26, K27, K28, K29, K30, K31,
                    K32, K33, K34, K35, K36, K37, K38, K39,
                    K40, K41, K42, K43, K44, K45, K46, K47,
                    K48, K49, K50, K51, K52, K53, K54, K55,
                    K56, K57, K58, K59, K60, K61, K62, K63,
                    K64, K65, K66, K67, K68, K69, K70, K71,
                    K72, K73, K74, K75, K76, K77, K78, K79,
                    RR1, RR8, RR14, RR18, RR19, RR28, RR34,
                    RR39, RR41, RR61, SR6, SR7>
{
    const K: [u64; 80] = [  K00, K01, K02, K03, K04, K05, K06, K07,
                            K08, K09, K10, K11, K12, K13, K14, K15,
                            K16, K17, K18, K19, K20, K21, K22, K23,
                            K24, K25, K26, K27, K28, K29, K30, K31,
                            K32, K33, K34, K35, K36, K37, K38, K39,
                            K40, K41, K42, K43, K44, K45, K46, K47,
                            K48, K49, K50, K51, K52, K53, K54, K55,
                            K56, K57, K58, K59, K60, K61, K62, K63,
                            K64, K65, K66, K67, K68, K69, K70, K71,
                            K72, K73, K74, K75, K76, K77, K78, K79 ];
    const H: [u64; 8] = [ H0, H1, H2, H3, H4, H5, H6, H7 ];

    // pub fn new() -> Self
    /// Constructs a new object of `SHA2_256` or `SHA2_224`,
    /// or a new SHA2_256-based object.
    /// 
    /// # Output
    /// A new object of `SHA2_512_t_256` or `SHA2_512_t_224`,
    /// or a new SHA2_512_t-based object.
    /// 
    /// # Example 1 for SHA2_512_t_256
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256;
    /// let hash = SHA2_512_t_256::new();
    /// println!("Hash =\t{}", hash);
    /// assert_eq!(hash.to_string(), "22312194FC2BF72C9F555FA3C84C64C22393B86B6F53B151963877195940EABD");
    /// ```
    /// 
    /// # Example 2 for SHA2_512_t_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256_Expanded;
    /// type MySHA2 = SHA2_512_t_256_Expanded<160>;
    /// let my_hash = MySHA2::new();
    /// println!("Hash =\t{}", my_hash);
    /// assert_eq!(my_hash.to_string(), "B80E7C569545AF48629EF11E2E14B8204F74747C4F949C6D60FEB4CC233775A7");
    /// ```
    /// 
    /// # Example 3 for SHA2_512_t_224
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224;
    /// let hash = SHA2_512_t_224::new();
    /// println!("Hash =\t{}", hash);
    /// assert_eq!(hash.to_string(), "8C3D37C819544DA273E1996689DCD4D61DFAB7AE32FF9C82679DD514");
    /// ```
    /// 
    /// # Example 4 for SHA2_512_t_224_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224_Expanded;
    /// type MySHA2 = SHA2_512_t_224_Expanded<160>;
    /// let my_hash = MySHA2::new();
    /// println!("Hash =\t{}", my_hash);
    /// assert_eq!(my_hash.to_string(), "6053A0C18224AF65E6633DEA9B125B74309B64519F70586FF009DFF5");
    /// ```
    #[inline]
    pub fn new() -> Self
    {
        Self::new_with_seed_text(format!("SHA-512/{}", t).as_str())
    }

    // pub fn new_with_seed_text(seed_text: &str) -> Self
    /// Constructs a new object of `SHA2_256` or `SHA2_224`,
    /// or a new SHA2_256-based object with seed text
    /// 
    /// # Output
    /// A new object of `SHA2_512_t_256` or `SHA2_512_t_224`,
    /// or a new SHA2_512_t-based object.
    /// 
    /// # Argument
    /// The `seed_text` to make initial hash values.
    /// 
    /// # Example 1 for SHA2_512_t_256
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256;
    /// let hash = SHA2_512_t_256::new_with_seed_text("샤-");
    /// // '샤' is from Hangeul which is Korean letter, sounds like 'sha'
    /// println!("Hash =\t{}", hash);
    /// assert_eq!(hash.to_string(), "6E231779CE7B233F74077E896D4ABCCA8B31054CB94168164E08BD8F31764DCB");
    /// ```
    /// 
    /// # Example 2 for SHA2_512_t_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256_Expanded;
    /// type MySHA2 = SHA2_512_t_256_Expanded<160>;
    /// let my_hash = MySHA2::new_with_seed_text("샤-");
    /// // '샤' is from Hangeul which is Korean letter, sounds like 'sha'
    /// println!("Hash =\t{}", my_hash);
    /// assert_eq!(my_hash.to_string(), "A15939C18C313184EA37451948F708F5C7B1FBE11E40F8795EF6BF52DB4EC9E9");
    /// ```
    /// 
    /// # Example 3 for SHA2_512_t_224
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224;
    /// let hash = SHA2_512_t_224::new_with_seed_text("샤-");
    /// // '샤' is from Hangeul which is Korean letter, sounds like 'sha'
    /// println!("Hash =\t{}", hash);
    /// assert_eq!(hash.to_string(), "6E231779CE7B233F74077E896D4ABCCA8B31054CB94168164E08BD8F");
    /// ```
    /// 
    /// # Example 4 for SHA2_512_t_224_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224_Expanded;
    /// type MySHA2 = SHA2_512_t_224_Expanded<160>;
    /// let my_hash = MySHA2::new_with_seed_text("샤-");
    /// // '샤' is from Hangeul which is Korean letter, sounds like 'sha'
    /// println!("Hash =\t{}", my_hash);
    /// assert_eq!(my_hash.to_string(), "A15939C18C313184EA37451948F708F5C7B1FBE11E40F8795EF6BF52");
    /// ```
    pub fn new_with_seed_text(seed_text: &str) -> Self
    {
        if t > 512
            { panic!("t cannot be greater than 512."); }
        if (t & 0b111) != 0
            { panic!("t should be multiple of 8."); }

        let seed_text = format!("{}", seed_text);
        let o = [ Self::H[0] ^ A5A5A5A5A5A5A5A5,
                            Self::H[1] ^ A5A5A5A5A5A5A5A5, 
                            Self::H[2] ^ A5A5A5A5A5A5A5A5,
                            Self::H[3] ^ A5A5A5A5A5A5A5A5,
                            Self::H[4] ^ A5A5A5A5A5A5A5A5,
                            Self::H[5] ^ A5A5A5A5A5A5A5A5, 
                            Self::H[6] ^ A5A5A5A5A5A5A5A5,
                            Self::H[7] ^ A5A5A5A5A5A5A5A5 ];

        let mut h = SHA2_512_0::new_with_h(&o);
        h.digest(seed_text.as_ptr(), seed_text.len() as u128);
        let mut o = [0_u64; 8];
        h.put_hash_value_in_array(&mut o);
        for i in 0..8
            { o[i] = o[i].to_be(); }
        Self::new_with_h(&o)
    }

    // pub fn new_with_h(H: &[u64; 8]) -> Self
    /// Constructs a new object of `SHA2_256` or `SHA2_224`,
    /// or a new SHA2_256-based object with initial hash value
    /// 
    /// # Output
    /// A new object of `SHA2_512_t_256` or `SHA2_512_t_224`,
    /// or a new SHA2_512_t-based object.
    /// 
    /// # Argument
    /// The initial hash value `H` to set initial value.
    fn new_with_h(h: &[u64; 8]) -> Self
    {
        Self
        {
            hash_code: [LongUnion::new_with(h[0]),
                        LongUnion::new_with(h[1]),
                        LongUnion::new_with(h[2]),
                        LongUnion::new_with(h[3]),
                        LongUnion::new_with(h[4]),
                        LongUnion::new_with(h[5]),
                        LongUnion::new_with(h[6]),
                        LongUnion::new_with(h[7])],
            o: [ h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7] ],
        }
    }

    // pub fn digest_c(&mut self, message: *const u8, length_in_bytes_low: u64, length_in_bytes_high: u64)
    /// Computes hash value.
    /// 
    /// # Features
    /// This function has the generalized interface (pointer, `*const u8`).
    /// So, this function is usually not called directly in Rust. This function
    /// is provided to be called from other programming languages such as C/C++.
    /// 
    /// # Arguments
    /// - `message` is pointer to const u8.
    /// - `length_in_bytes_low` is the lower 64 bits of the size of message
    /// in the unit of bytes.
    /// - `length_in_bytes_high` is the higher 64 bits of the size of message
    /// in the unit of bytes.
    /// 
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of a string slice,
    /// you are highly recommended to use the method
    /// [digest_str()](struct@SHA2_512_t_Generic#method.digest_str)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of String
    /// object, you are highly recommended to use the method
    /// [digest_string()](struct@SHA2_512_t_Generic#method.digest_string)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    /// object, you are highly recommended to use the method
    /// [digest_array()](struct@SHA2_512_t_Generic#method.digest_array)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    /// object, you are highly recommended to use the method
    /// [digest_vec()](struct@SHA2_512_t_Generic#method.digest_array)
    /// rather than this method.
    /// 
    /// # Example 1 for SHA2_512_t_256
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256;
    /// let mut hash = SHA2_512_t_256::new();
    /// let txt = "This is an example of the method digest_c().";
    /// hash.digest_c(txt.as_ptr(), txt.len() as u64, 0);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "AE67F5B190BB09DC615859EC2D11736DA6CBE00340EE39396FE76257238E3AF1");
    /// ```
    /// 
    /// # Example 2 for SHA2_512_t_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256_Expanded;
    /// type MySHA2 = SHA2_512_t_256_Expanded<0x123456789abcdef0, 160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method digest_c().";
    /// my_hash.digest_c(txt.as_ptr(), txt.len() as u64, 0);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "D9BE41EF1B7AFDCF7E3E8256661ACD436E3D0811FD433D5A6BF48823F2A004B4");
    /// ```
    /// 
    /// # Example 3 for SHA2_512_t_224
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224;
    /// let mut hash = SHA2_512_t_224::new();
    /// let txt = "This is an example of the method digest_c().";
    /// hash.digest_c(txt.as_ptr(), txt.len() as u64, 0);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "E7B8A450F1F3E90B361BED00083D6E14A90C2A074C71038D0743E384");
    /// ```
    /// 
    /// # Example 4 for SHA2_512_t_224_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224_Expanded;
    /// type MySHA2 = SHA2_512_t_224_Expanded<0x123456789abcdef0, 160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method digest_c().";
    /// my_hash.digest_c(txt.as_ptr(), txt.len() as u64, 0);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "9B38B1C0434F66DB99A76273D167237ABC3BF8BC96F91DF051A3E31B");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn digest_c(&mut self, message: *const u8, length_in_bytes_low: u64, length_in_bytes_high: u64)
    {
        let mut vu = LongerUnion::new();
        vu.set_ulong_(0, length_in_bytes_low);
        vu.set_ulong_(1, length_in_bytes_high);
        self.digest(message, vu.get());
    }

    // pub fn digest(&mut self, message: *const u8, length_in_bytes: u128)
    /// Computes hash value.
    /// 
    /// # Features
    /// This function has the generalized interface (pointer, `*const u8`)
    /// so as to enable other functions to wrap this function with any
    /// convenient interface for uses. So, this function can be called in Rust.
    /// 
    /// # Arguments
    /// - `message` is pointer to const u8.
    /// - `length_in_bytes` is the size of message in the unit of bytes, and
    /// data type is `u128`.
    /// 
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of a string slice,
    /// you are highly recommended to use the method
    /// [digest_str()](struct@SHA2_512_t_Generic#method.digest_str)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of String
    /// object, you are highly recommended to use the method
    /// [digest_string()](struct@SHA2_512_t_Generic#method.digest_string)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    /// object, you are highly recommended to use the method
    /// [digest_array()](struct@SHA2_512_t_Generic#method.digest_array)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    /// object, you are highly recommended to use the method
    /// [digest_vec()](struct@SHA2_512_t_Generic#method.digest_array)
    /// rather than this method.
    /// 
    /// # Example 1 for SHA2_512_t_256
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256;
    /// let mut hash = SHA2_512_t_256::new();
    /// let txt = "This is an example of the method digest().";
    /// hash.digest(txt.as_ptr(), txt.len() as u128);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "BF3A06F51CE91951607AABD2E33AD24D8B75618F2366B90D98991AD28E47FAA5");
    /// ```
    /// 
    /// # Example 2 for SHA2_512_t_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256_Expanded;
    /// type MySHA2 = SHA2_512_t_256_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// my_hash.digest(txt.as_ptr(), txt.len() as u128);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "645C53583A01ABF44F279BEC2CC07AB072B57AA319962B524C73435DBE564CEF");
    /// ```
    /// 
    /// # Example 3 for SHA2_512_t_224
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224;
    /// let mut hash = SHA2_512_t_224::new();
    /// let txt = "This is an example of the method digest().";
    /// hash.digest(txt.as_ptr(), txt.len() as u128);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "2269C5A3791E72D00337D9EDDE9BA9568539F4E131B7DB7555545633");
    /// ```
    /// 
    /// # Example 4 for SHA2_512_t_224_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224_Expanded;
    /// type MySHA2 = SHA2_512_t_224_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method digest().";
    /// my_hash.digest(txt.as_ptr(), txt.len() as u128);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "1DCBF56DC6F3387734139CC5CA14FAC05DF67CD4B14AE86E474F421C");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn digest(&mut self, message: *const u8, length_in_bytes: u128)
    {
        type MessageType = u64;
        const SHIFT_NUM: usize = 7;
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
    /// [digest_string()](struct@SHA2_512_t_Generic#method.digest_string)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    /// object, you are highly recommended to use the method
    /// [digest_array()](struct@SHA2_512_t_Generic#method.digest_array)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    /// object, you are highly recommended to use the method
    /// [digest_vec()](struct@SHA2_512_t_Generic#method.digest_array)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [digest()](struct@SHA2_512_t_Generic#method.digest)
    /// rather than this method.
    /// 
    /// # Example 1 for SHA2_512_t_256
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256;
    /// let mut hash = SHA2_512_t_256::new();
    /// let txt = "This is an example of the method digest_str().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "D0ED13389E431C8D74FE6E8DB5B6194682874B52E800524136E35D7E9CFA496B");
    /// ```
    /// 
    /// # Example 2 for SHA2_512_t_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256_Expanded;
    /// type MySHA2 = SHA2_512_t_256_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method digest_str().";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "2ABEF10487ECC51EA8953654E972C7C57817D674B12B89E175E569169F43ED9B");
    /// ```
    /// 
    /// # Example 3 for SHA2_512_t_224
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224;
    /// let mut hash = SHA2_512_t_224::new();
    /// let txt = "This is an example of the method digest_str().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "17E80E466E706474DB2C9E39691150805AC536319125AFB1E436BE8F");
    /// ```
    /// 
    /// # Example 4 for SHA2_512_t_224_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224_Expanded;
    /// type MySHA2 = SHA2_512_t_224_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method digest_str().";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "44C91FB5B6352E89DF5B5230A004B8594FC7B7AF6F61D3E332C4AC01");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn digest_str(&mut self, message: &str)
    {
        self.digest(message.as_ptr(), message.len() as u128);
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
    /// [digest_str()](struct@SHA2_512_t_Generic#method.digest_str)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    /// object, you are highly recommended to use the method
    /// [digest_array()](struct@SHA2_512_t_Generic#method.digest_array)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    /// object, you are highly recommended to use the method
    /// [digest_vec()](struct@SHA2_512_t_Generic#method.digest_array)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [digest()](struct@SHA2_512_t_Generic#method.digest)
    /// rather than this method.
    /// 
    /// # Example 1 for SHA2_512_t_256
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256;
    /// let mut hash = SHA2_512_t_256::new();
    /// let txt = "This is an example of the method digest_string().".to_string();
    /// hash.digest_string(&txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "2824B79B5D8A1D02454612B72B9CD9544D0DF8E126E7A01E55AC479B0903297C");
    /// ```
    /// 
    /// # Example 2 for SHA2_512_t_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256_Expanded;
    /// type MySHA2 = SHA2_512_t_256_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method digest_string().".to_string();
    /// my_hash.digest_string(&txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "B9D855F972D884C200B5EFECB105B115065AC58540099777A84766623BF87C15");
    /// ```
    /// 
    /// # Example 3 for SHA2_512_t_224
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224;
    /// let mut hash = SHA2_512_t_224::new();
    /// let txt = "This is an example of the method digest_string().".to_string();
    /// hash.digest_string(&txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "E1423096CED4DC8D9522C75C8BBB12B59A4510093CFA4FD480D270FD");
    /// ```
    /// 
    /// # Example 4 for SHA2_512_t_224_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224_Expanded;
    /// type MySHA2 = SHA2_512_t_224_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method digest_string().".to_string();
    /// my_hash.digest_string(&txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "092A7203B3BD5C72852B0507989257577808C453C2C7F915BAD1CF5C");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn digest_string(&mut self, message: &String)
    {
        self.digest(message.as_ptr(), message.len() as u128);
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
    /// [digest_str()](struct@SHA2_512_t_Generic#method.digest_str)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of String
    /// object, you are highly recommended to use the method
    /// [digest_string()](struct@SHA2_512_t_Generic#method.digest_string)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    /// object, you are highly recommended to use the method
    /// [digest_vec()](struct@SHA2_512_t_Generic#method.digest_array)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [digest()](struct@SHA2_512_t_Generic#method.digest)
    /// rather than this method.
    /// 
    /// # Example 1 for SHA2_512_t_256
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256;
    /// let mut hash = SHA2_512_t_256::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.digest_array(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "E9A9876BBF1432C27CE58D6B8EA66B5A0B719FA80832D491768033F4DAF65A64");
    /// ```
    /// 
    /// # Example 2 for SHA2_512_t_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256_Expanded;
    /// type MySHA2 = SHA2_512_t_256_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// my_hash.digest_array(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    /// assert_eq!(my_hash.to_string(), "4AA24C35A21F9D0552E0C3A69A5A59EFE1936FD361ABA1C6E8F6DA22FC39D236");
    /// ```
    /// 
    /// # Example 3 for SHA2_512_t_224
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224;
    /// let mut hash = SHA2_512_t_224::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.digest_array(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "3F600A922240910231ACA350DEDD49BD875936BE5AAB8A034D09334B");
    /// ```
    /// 
    /// # Example 4 for SHA2_512_t_224_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224_Expanded;
    /// type MySHA2 = SHA2_512_t_224_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// my_hash.digest_array(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    /// assert_eq!(my_hash.to_string(), "504B348485EC8CF96E630DFC90D75DC1543A3A2B3B895A0261CAF0CE");
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
        self.digest(message.as_ptr() as *const u8, (M * T::size_in_bytes()) as u128);
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
    /// [digest_str()](struct@SHA2_512_t_Generic#method.digest_str)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of String
    /// object, you are highly recommended to use the method
    /// [digest_string()](struct@SHA2_512_t_Generic#method.digest_string)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    /// object, you are highly recommended to use the method
    /// [digest_array()](struct@SHA2_512_t_Generic#method.digest_array)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [digest()](struct@SHA2_512_t_Generic#method.digest)
    /// rather than this method.
    /// 
    /// # Example 1 for SHA2_512_t_256
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256;
    /// let mut hash = SHA2_512_t_256::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.digest_vec(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "E9A9876BBF1432C27CE58D6B8EA66B5A0B719FA80832D491768033F4DAF65A64");
    /// ```
    /// 
    /// # Example 2 for SHA2_512_t_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256_Expanded;
    /// type MySHA2 = SHA2_512_t_256_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// my_hash.digest_vec(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    /// assert_eq!(my_hash.to_string(), "4AA24C35A21F9D0552E0C3A69A5A59EFE1936FD361ABA1C6E8F6DA22FC39D236");
    /// ```
    /// 
    /// # Example 3 for SHA2_512_t_224
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224;
    /// let mut hash = SHA2_512_t_224::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.digest_vec(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "3F600A922240910231ACA350DEDD49BD875936BE5AAB8A034D09334B");
    /// ```
    /// 
    /// # Example 4 for SHA2_512_t_224_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224_Expanded;
    /// type MySHA2 = SHA2_512_t_224_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// my_hash.digest_vec(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    /// assert_eq!(my_hash.to_string(), "504B348485EC8CF96E630DFC90D75DC1543A3A2B3B895A0261CAF0CE");
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
        self.digest(message.as_ptr() as *const u8, (message.len() * T::size_in_bytes()) as u128);
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
    /// # Example 1 for SHA2_512_t_256
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256;
    /// let mut hash = SHA2_512_t_256::new();
    /// let txt = "This is an example of the method ruminate().";
    /// hash.ruminate(2, txt.as_ptr(), txt.len() as u128);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "EBA9C4DE950CE07EDB662147C3246779660F03607D27493A0D62ECC6282C4501");
    /// ```
    /// 
    /// # Example 2 for SHA2_512_t_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256_Expanded;
    /// type MySHA2 = SHA2_512_t_256_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method ruminate().";
    /// my_hash.ruminate(2, txt.as_ptr(), txt.len() as u128);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "B60D75418A979C6B4E444B755D535257969C5FFC465FA84988026219FC7BD8B7");
    /// ```
    /// 
    /// # Example 3 for SHA2_512_t_224
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224;
    /// let mut hash = SHA2_512_t_224::new();
    /// let txt = "This is an example of the method ruminate().";
    /// hash.ruminate(2, txt.as_ptr(), txt.len() as u128);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "A3280359EA2135FE3E2667724FCA6996A47B362544FA60FD59D95DBF");
    /// ```
    /// 
    /// # Example 4 for SHA2_512_t_224_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224_Expanded;
    /// type MySHA2 = SHA2_512_t_224_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method ruminate().";
    /// my_hash.ruminate(2, txt.as_ptr(), txt.len() as u128);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "E587737E4BC3E1D859AA7FDDD90D3E769158173B7A22FA4BC76E47BA");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn ruminate(&mut self, n: usize, message: *const u8, length_in_bytes: u128)
    {
        self.digest(message, length_in_bytes);
        for _ in 1..n
            { self.digest_array(&self.get_hash_value_in_array()); }
    }

    // pub fn ruminate_c(&mut self, n: usize, message: *const u8, length_in_bytes_low: u64, length_in_bytes_high: u64)
    /// Computes a hash value of `message`, and then computes a new hash value
    /// of the hash value of the message, and then computes a hash value of the
    /// previous hash value, and then ... `n` times repeatedly.
    /// 
    /// # Features
    /// This function has the generalized interface (pointer, `*const u8`).
    /// So, this function is usually not called directly in Rust. This function
    /// is provided to be called from other programming languages such as C/C++.
    /// 
    /// # Arguments
    /// - `n` is the number of repetition of digestion
    /// - `message` is pointer to const u8.
    /// - `length_in_bytes_low` is the lower 64 bits of the size of message
    /// in the unit of bytes.
    /// - `length_in_bytes_high` is the higher 64 bits of the size of message
    /// in the unit of bytes.
    /// 
    /// # Origin
    /// Double hashing is invented by Ferguson and Schneier in their book
    /// Practical Cryptography to countermeasure against length extension
    /// attacks. Plus, Bitcoin uses double hashing.
    /// This is generalized version of it.
    /// 
    /// # Security Issue
    /// The author doubts that the double hashing is securer than normal
    /// hashing. The double hashing will be as secure as the normal hashing
    /// at most because birthday paradox applies twice for the double hashing
    /// though the size of the domain is the same size of the codomain for
    /// second hashing of the double hashing, while the birthday paradox
    /// applies only once for the normal hashing.
    /// 
    /// # Example 1 for SHA2_512_t_256
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256;
    /// let mut hash = SHA2_512_t_256::new();
    /// let txt = "This is an example of the method ruminate_c().";
    /// hash.ruminate_c(2, txt.as_ptr(), txt.len() as u64, 0);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "71D8FB0BC160A3EAA18ED54D48EC54A2FBA4364D4592917CEB8846CAB1492DB6");
    /// ```
    /// 
    /// # Example 2 for SHA2_512_t_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256_Expanded;
    /// type MySHA2 = SHA2_512_t_256_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method ruminate_c().";
    /// my_hash.ruminate_c(2, txt.as_ptr(), txt.len() as u64, 0);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "A5098CDF4CAFEC47765E2D87557587D50CFA802385C39B3596A816B863C45F82");
    /// ```
    /// # Example 3 for SHA2_512_t_224
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224;
    /// let mut hash = SHA2_512_t_224::new();
    /// let txt = "This is an example of the method ruminate_c().";
    /// hash.ruminate_c(2, txt.as_ptr(), txt.len() as u64, 0);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "4DA0CB3085D73CA7459E326D51349B5A7C065A270347558DA7FB3784");
    /// ```
    /// 
    /// # Example 4 for SHA2_512_t_224_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224_Expanded;
    /// type MySHA2 = SHA2_512_t_224_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method ruminate_c().";
    /// my_hash.ruminate_c(2, txt.as_ptr(), txt.len() as u64, 0);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "4AAE39BF545F153044E1A9D10CDAA98F56D048619C406770709FB015");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn ruminate_c(&mut self, n: usize, message: *const u8, length_in_bytes_low: u64, length_in_bytes_high: u64)
    {
        self.digest_c(message, length_in_bytes_low, length_in_bytes_high);
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
    /// # Example 1 for SHA2_512_t_256
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256;
    /// let mut hash = SHA2_512_t_256::new();
    /// let txt = "This is an example of the method ruminate_str().";
    /// hash.ruminate_str(3, txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "D2ACE4176F3EBD2F5786EFD459D72AD44D24425D05494A8FFEFCA75BAB007FA6");
    /// ```
    /// 
    /// # Example 2 for SHA2_512_t_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256_Expanded;
    /// type MySHA2 = SHA2_512_t_256_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method ruminate_str().";
    /// my_hash.ruminate_str(3, txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "C5FE6148F6177C5208C8992D2ED20C3016681289ACF5B161D0AD95FB5C4CE5EA");
    /// ```
    /// 
    /// # Example 3 for SHA2_512_t_224
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224;
    /// let mut hash = SHA2_512_t_224::new();
    /// let txt = "This is an example of the method ruminate_str().";
    /// hash.ruminate_str(3, txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "7FB2230906052932F044352E65F590C416C09C3A7290EF3BC39635EF");
    /// ```
    /// 
    /// # Example 4 for SHA2_512_t_224_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224_Expanded;
    /// type MySHA2 = SHA2_512_t_224_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method ruminate_str().";
    /// my_hash.ruminate_str(3, txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "9FD72DC6516F3E01FAE29244B70D501F6AE73B25CB462F816F01C6F0");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn ruminate_str(&mut self, n: usize, message: &str)
    {
        self.ruminate(n, message.as_ptr(), message.len() as u128);
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
    /// # Example 1 for SHA2_512_t_256
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256;
    /// let mut hash = SHA2_512_t_256::new();
    /// let txt = "This is an example of the method ruminate_string().".to_string();
    /// hash.ruminate_string(2, &txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "7E93544875E40C8F25DDD93AEC9A447C124B22C3DDCDB7479FAD6C144FFC74B2");
    /// ```
    /// 
    /// # Example 2 for SHA2_512_t_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256_Expanded;
    /// type MySHA2 = SHA2_512_t_256_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method ruminate_string().".to_string();
    /// my_hash.ruminate_string(2, &txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "8391EA69564967DF827872115E35A98DFEFF72894F6497C369D83C25C8C50E2E");
    /// ```
    /// 
    /// # Example 3 for SHA2_512_t_224
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224;
    /// let mut hash = SHA2_512_t_224::new();
    /// let txt = "This is an example of the method ruminate_string().".to_string();
    /// hash.ruminate_string(2, &txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "C6887398FA4DA83CD5039DC2764BB363B65F1C557006D627F95B5392");
    /// ```
    /// 
    /// # Example 4 for SHA2_512_t_224_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224_Expanded;
    /// type MySHA2 = SHA2_512_t_224_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method ruminate_string().".to_string();
    /// my_hash.ruminate_string(2, &txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "9B1BFD42888B1655735F6E0F1122E25D33F8DBF6E65D54D4EA0884A3");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn ruminate_string(&mut self, n: usize, message: &String)
    {
        self.ruminate(n, message.as_ptr(), message.len() as u128);
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
    /// # Example 1 for SHA2_512_t_256
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256;
    /// let mut hash = SHA2_512_t_256::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.ruminate_array(5,&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "5C8D4F9C47C99BD322E44AA2B6F265D7A788B8898F072E9E998122EB3DE256F9");
    /// ```
    /// 
    /// # Example 2 for SHA2_512_t_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256_Expanded;
    /// type MySHA2 = SHA2_512_t_256_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// my_hash.ruminate_array(5,&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    /// assert_eq!(my_hash.to_string(), "1A86BD0B53A9CD64FB43CF6BD82107782210A3F5FEC34CAF23B33D51A3B66011");
    /// ```
    /// 
    /// # Example 3 for SHA2_512_t_224
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224;
    /// let mut hash = SHA2_512_t_224::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.ruminate_array(5,&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "987227F7EC37FCF30A83BE661BF7018616CE5B9C9553AA7892C738D3");
    /// ```
    /// 
    /// # Example 4 for SHA2_512_t_224_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224_Expanded;
    /// type MySHA2 = SHA2_512_t_224_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// my_hash.ruminate_array(5,&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    /// assert_eq!(my_hash.to_string(), "BDF3FB6FCEAEB9A75FAA42CE759019A60FEB23E40DBC676F9BE36DE4");
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
        self.ruminate(n, message.as_ptr() as *const u8, (M * T::size_in_bytes()) as u128);
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
    /// # Example 1 for SHA2_512_t_256
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256;
    /// let mut hash = SHA2_512_t_256::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.ruminate_vec(2, &data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "6A999B22F62122B781705BBEB635E0DFD6F922FB2B0921F912ACA585B618D7F0");
    /// ```
    /// 
    /// # Example 2 for SHA2_512_t_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256_Expanded;
    /// type MySHA2 = SHA2_512_t_256_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// my_hash.ruminate_vec(2, &data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    /// assert_eq!(my_hash.to_string(), "2396C1C598B43E445261971C74170745DDB2FD0527684545FFB9818D1D0057AD");
    /// ```
    /// 
    /// # Example 3 for SHA2_512_t_224
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224;
    /// let mut hash = SHA2_512_t_224::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.ruminate_vec(2, &data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "ACA9B9D5B327FFE4140F131642F92DCBDFD678FA5F7A42536D27BAF8");
    /// ```
    /// 
    /// # Example 4 for SHA2_512_t_224_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224_Expanded;
    /// type MySHA2 = SHA2_512_t_224_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// my_hash.ruminate_vec(2, &data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    /// assert_eq!(my_hash.to_string(), "1AAC9EE0F6B45991CC58691DF19079E99422925DC600789343BEAA24");
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
        self.ruminate(n, message.as_ptr() as *const u8, (message.len() * T::size_in_bytes()) as u128);
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
    /// [get_hash_value_string()](struct@SHA2_512_t_Generic#method.get_hash_value_string)
    /// rather than this method.
    /// - If you want to get the hash value in the form of array object,
    /// you are highly recommended to use the method
    /// [get_hash_value_in_array()](struct@SHA2_512_t_Generic#method.get_hash_value_in_array)
    /// rather than this method.
    /// - If you want to get the hash value in the form of Vec object,
    /// you are highly recommended to use the method
    /// [get_hash_value_in_vec()](struct@SHA2_512_t_Generic#method.get_hash_value_in_vec)
    /// rather than this method.
    /// 
    /// # Example 1 for SHA2_512_t_256
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256;
    /// let mut hash = SHA2_512_t_256::new();
    /// let txt = "This is an example of the method get_hash_value().";
    /// let hash_value = [0_u8; 32];
    /// hash.digest_str(txt);
    /// hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    /// assert_eq!(format!("{:02X?}", hash_value), "[50, EA, 83, BF, 41, 5D, 1C, C0, 15, 6C, BF, 90, 5B, AC, BD, 72, A3, BD, 62, 1B, 94, 3A, 64, 64, 13, 05, CF, 17, 43, 52, CF, AD]");
    /// ```
    /// 
    /// # Example 2 for SHA2_512_t_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256_Expanded;
    /// type MySHA2 = SHA2_512_t_256_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method get_hash_value().";
    /// let hash_value = [0_u8; 32];
    /// my_hash.digest_str(txt);
    /// my_hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    /// assert_eq!(format!("{:02X?}", hash_value), "[D1, 15, 58, 28, FA, A6, 27, F9, 7E, DE, D0, 98, 74, C0, A1, DB, FA, 5E, C0, E9, A9, 98, 35, DD, B8, 00, DC, B4, 28, 79, A9, D3]");
    /// ```
    /// 
    /// # Example 3 for SHA2_512_t_224
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224;
    /// let mut hash = SHA2_512_t_224::new();
    /// let txt = "This is an example of the method get_hash_value().";
    /// let hash_value = [0_u8; 28];
    /// hash.digest_str(txt);
    /// hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    /// assert_eq!(format!("{:02X?}", hash_value), "[8B, 40, A3, E7, 02, A8, 18, 25, 12, 2C, C8, 55, 07, 4F, 5B, 0F, 73, BD, 30, 42, 5F, 3A, A9, 55, 92, 28, 27, 9E]");
    /// ```
    /// 
    /// # Example 4 for SHA2_512_t_224_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224_Expanded;
    /// type MySHA2 = SHA2_512_t_224_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method get_hash_value().";
    /// let hash_value = [0_u8; 28];
    /// my_hash.digest_str(txt);
    /// my_hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    /// assert_eq!(format!("{:02X?}", hash_value), "[A7, A9, A3, 52, EB, E6, 06, 3E, 80, F1, 7E, 62, 27, 6B, AB, F6, 5C, 21, 8E, 56, B7, 2A, 04, 4C, 7D, 11, C5, 40]");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn get_hash_value(&self, hash_value: *mut u8, length: usize)
    {
        let n = t & 0b11_1111;
        let nn = (t >> 6) + if n == 0 {0} else {1};
        let n_length = if length < (t >> 3) {length} else {t >> 3};
        let mut hash_code = [0_u64; 8];
        for i in 0..nn
            { hash_code[i] = self.hash_code[i].get(); }
        if n != 0
        {
            let mask = (!0_u64) << (64-n);
            hash_code[nn-1] &= mask;
        }
        for i in 0..nn
            { hash_code[i] = hash_code[i].to_be(); }
        unsafe { copy_nonoverlapping(hash_code.as_ptr() as *const u8, hash_value, n_length); }
    }

    // pub fn get_hash_value_in_string(&self) -> String
    /// Returns a hash value in the form of String object.
    /// 
    /// # Counterpart Methods
    /// - If you want to get the hash value in the form of array object,
    /// you are highly recommended to use the method
    /// [get_hash_value_in_array()](struct@SHA2_512_t_Generic#method.get_hash_value_in_array)
    /// rather than this method.
    /// - If you want to get the hash value in the form of Vec object,
    /// you are highly recommended to use the method
    /// [get_hash_value_in_vec()](struct@SHA2_512_t_Generic#method.get_hash_value_in_vec)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [get_hash_value()](struct@SHA2_512_t_Generic#method.get_hash_value)
    /// rather than this method.
    /// 
    /// # Example 1 for SHA2_512_t_256
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256;
    /// let mut hash = SHA2_512_t_256::new();
    /// let txt = "This is an example of the method get_hash_value_in_string().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_hash_value_in_string());
    /// assert_eq!(hash.get_hash_value_in_string(), "F3E8E24304CD04DBE509FE47FFA84DA4CF15E70EEFD447F34A069047735014DC");
    /// ```
    /// 
    /// # Example 2 for SHA2_512_t_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256_Expanded;
    /// type MySHA2 = SHA2_512_t_256_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method get_hash_value_in_string().";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.get_hash_value_in_string());
    /// assert_eq!(my_hash.get_hash_value_in_string(), "9FF616CE58C7CD8988DD1AF54F9AB5E8674ADF5037A7B059AF7B608023D44FBC");
    /// ```
    /// 
    /// # Example 3 for SHA2_512_t_224
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224;
    /// let mut hash = SHA2_512_t_224::new();
    /// let txt = "This is an example of the method get_hash_value_in_string().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_hash_value_in_string());
    /// assert_eq!(hash.get_hash_value_in_string(), "412823A4ED7BBA8C052F3C9B218A9847CDD341818E773F5593011135");
    /// ```
    /// 
    /// # Example 4 for SHA2_512_t_224_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224_Expanded;
    /// type MySHA2 = SHA2_512_t_224_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method get_hash_value_in_string().";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.get_hash_value_in_string());
    /// assert_eq!(my_hash.get_hash_value_in_string(), "1E5E301B37EC1336D5EB0D9A4AE18833F418C93F8ADBD87BE6817922");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn get_hash_value_in_string(&self) -> String
    {
        let n = t & 0b11_1111;  // equivalent to = t % 64
        let nn = (t >> 6) + if n != 0 {1} else {0};
        let mut txt = String::new();
        for i in 0..if n == 0 {nn} else {nn-1}
        {
            let hs = self.hash_code[i];
            for j in 0..8
            {
                let byte = hs.get_ubyte_(7-j);
                txt.push(Self::to_char(byte >> 4));
                txt.push(Self::to_char(byte & 0b1111));
            }
        }
        if n != 0
        {
            let hs = self.hash_code[nn-1];
            let m = n >> 3; // equivalent to = n / 8
            for j in 0..m
            {
                let byte = hs.get_ubyte_(7-j);
                txt.push(Self::to_char(byte >> 4));
                txt.push(Self::to_char(byte & 0b1111));
            }
        }
        txt
    }

    // pub fn get_hash_value_in_array(&self) -> [u64; 8]
    /// Returns a hash value in the form of array object.
    /// 
    /// # Counterpart Methods
    /// - If you want to get the hash value in the form of String object,
    /// you are highly recommended to use the method
    /// [get_hash_value_string()](struct@SHA2_512_t_Generic#method.get_hash_value_string)
    /// rather than this method.
    /// - If you want to get the hash value in the form of Vec object,
    /// you are highly recommended to use the method
    /// [get_hash_value_in_vec()](struct@SHA2_512_t_Generic#method.get_hash_value_in_vec)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [get_hash_value()](struct@SHA2_512_t_Generic#method.get_hash_value)
    /// rather than this method.
    /// 
    /// # Example 1 for SHA2_512_t_256
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256;
    /// let mut hash = SHA2_512_t_256::new();
    /// let txt = "This is an example of the method get_hash_value_in_array().";
    /// hash.digest_str(txt);
    /// let h = hash.get_hash_value_in_array();
    /// println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, h);
    /// assert_eq!(format!("{:016X?}", h), "[7770814665222F53, FAF871C4D20657F0, 4E3F488853C5C485, CDCFE5F1EB447C2F, 14F3BFD7D115FE93, 308218F3657D3CE6, 5D68300E49B0BE02, 9F8286AC65BAC220]");
    /// ```
    /// 
    /// # Example 2 for SHA2_512_t_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256_Expanded;
    /// type MySHA2 = SHA2_512_t_256_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method get_hash_value_in_array().";
    /// my_hash.digest_str(txt);
    /// let h = my_hash.get_hash_value_in_array();
    /// println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, h);
    /// assert_eq!(format!("{:016X?}", h), "[74688674B8E8B4BA, 439E1BEC604C9A30, C3E0398F8D52D970, 44B615CD387A9826, AE64BC4B33F5B6B3, 8D7A9CE85CB18255, E30515AA6BC2C25F, 2BA7DE436CE08812]");
    /// ```
    /// 
    /// # Example 3 for SHA2_512_t_224
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224;
    /// let mut hash = SHA2_512_t_224::new();
    /// let txt = "This is an example of the method get_hash_value_in_array().";
    /// hash.digest_str(txt);
    /// let h = hash.get_hash_value_in_array();
    /// println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, h);
    /// assert_eq!(format!("{:016X?}", h), "[5D4F9158C3082FB5, DA01BFDDDDC44C7B, 8E72845FCC6EF467, EBCF28927DFDDD35, E9ACDB58F2E01FAE, E62CF6411757DAD2, 9CC9EF5BD989F543, 055C3169B2B23276]");
    /// ```
    /// 
    /// # Example 4 for SHA2_512_t_224_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224_Expanded;
    /// type MySHA2 = SHA2_512_t_224_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method get_hash_value_in_array().";
    /// my_hash.digest_str(txt);
    /// let h = my_hash.get_hash_value_in_array();
    /// println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, h);
    /// assert_eq!(format!("{:016X?}", h), "[66E3FC505AE3D81E, 09B33F26C109A6D8, 0965E49C9CEC98D8, BEDFF6D3ACA89A9C, 95C0831C1F650A37, BEAAC6B754481D81, D4EEECD4ADE7BF93, FFA8A6B1808E60DD]");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn get_hash_value_in_array(&self) -> [u64; 8]
    {
        let mut res = [0_u64; 8];
        for i in 0..8
            { res[i] = self.hash_code[i].get().to_be(); }
        res
    }

    // pub fn get_hash_value_in_array_tm<T, const M: usize>(&self) -> [T; M]
    /// Returns a hash value in the form of array object.
    /// 
    /// # Generic Parameters
    /// - `T`: primitive datatype of each element.
    /// - `M`: the number of elements
    /// 
    /// # Counterpart Methods
    /// - If you want to get the hash value in the form of String object,
    /// you are highly recommended to use the method
    /// [get_hash_value_string()](struct@SHA2_512_t_Generic#method.get_hash_value_string)
    /// rather than this method.
    /// - If you want to get the hash value in the form of Vec object,
    /// you are highly recommended to use the method
    /// [get_hash_value_in_vec()](struct@SHA2_512_t_Generic#method.get_hash_value_in_vec)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [get_hash_value()](struct@SHA2_512_t_Generic#method.get_hash_value)
    /// rather than this method.
    /// 
    /// # Example 1 for SHA2_512_t_256
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256;
    /// let mut hash = SHA2_512_t_256::new();
    /// let txt = "This is an example of the method get_hash_value_in_array_tm().";
    /// hash.digest_str(txt);
    /// let h: [u64; 4] = hash.get_hash_value_in_array_tm();
    /// println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, h);
    /// assert_eq!(format!("{:016X?}", h), "[B2CB9E8954581373, CB03D5E9B4A232D3, B6A92CB91A33C2B6, A78A5A9914FFAAFD]");
    /// ```
    /// 
    /// # Example 2 for SHA2_512_t_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256_Expanded;
    /// type MySHA2 = SHA2_512_t_256_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method get_hash_value_in_array_tm().";
    /// my_hash.digest_str(txt);
    /// let h: [u64; 4] = hash.get_hash_value_in_array_tm();
    /// println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, h);
    /// assert_eq!(format!("{:016X?}", h), "[B2CB9E8954581373, CB03D5E9B4A232D3, B6A92CB91A33C2B6, A78A5A9914FFAAFD]");
    /// ```
    /// 
    /// # Example 3 for SHA2_512_t_224
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256;
    /// let mut hash = SHA2_512_t_256::new();
    /// let txt = "This is an example of the method get_hash_value_in_array_tm().";
    /// hash.digest_str(txt);
    /// let h: [u32; 7] = hash.get_hash_value_in_array_tm();
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, h);
    /// assert_eq!(format!("{:08X?}", h), "[54581373, B2CB9E89, B4A232D3, CB03D5E9, 1A33C2B6, B6A92CB9, 14FFAAFD]");
    /// ```
    /// 
    /// # Example 4 for SHA2_512_t_224_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256_Expanded;
    /// type MySHA2 = SHA2_512_t_256_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method get_hash_value_in_array_tm().";
    /// my_hash.digest_str(txt);
    /// let h: [u32; 7] = my_hash.get_hash_value_in_array_tm();
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, h);
    /// assert_eq!(format!("{:08X?}", h), "[63D2D0E1, A3378487, 116930CC, 1DD5D525, 47DAE024, 0B502841, AC13B293]");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn get_hash_value_in_array_tm<T, const M: usize>(&self) -> [T; M]
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
    {
        let mut o = [T::zero(); M];
        self.put_hash_value_in_array(&mut o);
        o
    }

    // pub fn get_hash_value_in_vec(&self) -> Vec
    /// Returns a hash value in the form of Vec object.
    /// 
    /// # Counterpart Methods
    /// - If you want to get the hash value in the form of String object,
    /// you are highly recommended to use the method
    /// [get_hash_value_string()](struct@SHA2_512_t_Generic#method.get_hash_value_string)
    /// rather than this method.
    /// - If you want to get the hash value in the form of array object,
    /// you are highly recommended to use the method
    /// [get_hash_value_in_array()](struct@SHA2_512_t_Generic#method.get_hash_value_in_array)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [get_hash_value()](struct@SHA2_512_t_Generic#method.get_hash_value)
    /// rather than this method.
    /// 
    /// # Example 1 for SHA2_512_t_256
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256;
    /// let mut hash = SHA2_512_t_256::new();
    /// let txt = "This is an example of the method get_hash_value_in_vec().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash.get_hash_value_in_vec());
    /// assert_eq!(format!("{:016X?}", hash.get_hash_value_in_vec()), "[80A8B6995518FCAE, 88552E1A484EDBE2, 0D97F5D05378D628, 5B7CE15DDBCA6AFA]");
    /// ```
    /// 
    /// # Example 2 for SHA2_512_t_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256_Expanded;
    /// type MySHA2 = SHA2_512_t_256_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method get_hash_value_in_vec().";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, my_hash.get_hash_value_in_vec());
    /// assert_eq!(format!("{:016X?}", my_hash.get_hash_value_in_vec()), "[CAE7273F660C1371, F0A69EBDB143A63D, 37701C05D8CAA659, 76C307D312210B47]");
    /// ```
    /// 
    /// # Example 3 for SHA2_512_t_224
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224;
    /// let mut hash = SHA2_512_t_224::new();
    /// let txt = "This is an example of the method get_hash_value_in_vec().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash.get_hash_value_in_vec());
    /// assert_eq!(format!("{:016X?}", hash.get_hash_value_in_vec()), "[CDDD34F4216F38F0, CF7779A43F982E1A, 964CE2DBE181F3D3, 95AFDE2500000000]");
    /// ```
    /// 
    /// # Example 4 for SHA2_512_t_224_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224_Expanded;
    /// type MySHA2 = SHA2_512_t_224_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method get_hash_value_in_vec().";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, my_hash.get_hash_value_in_vec());
    /// assert_eq!(format!("{:016X?}", my_hash.get_hash_value_in_vec()), "[BEBD35E3ECE3EE83, 1A9F9889D1926D37, 08CF548C8A943F0A, 1BFECCAF00000000]");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn get_hash_value_in_vec(&self) -> Vec<u64>
    {
        let n = t & 0b11_1111;  // equivalent to = t % 64
        let nn = (t >> 6) + if n == 0 {0} else {1};
        let mut res = Vec::new();
        for i in 0..nn
            { res.push(self.hash_code[i].get().to_be()); }
        if n != 0
            { res[nn-1] &= (!0_u64) << (64-n); }
        res
    }

    // pub fn put_hash_value_in_array<T, const M: usize>(&self, out: &mut [T; M])
    /// Puts a hash value in the form of array object.
    /// 
    /// # Argument
    /// `out` is the array [T; M] which is the place to put the hash value.
    /// 
    /// # Features
    /// If `M * mem::size_of::<T>()` > `64`,
    /// it pass the output as the amount of `64`.
    /// 
    /// # Example 1 for SHA2_512_t_256
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256;
    /// let mut hash = SHA2_512_t_256::new();
    /// let txt = "This is an example of the method put_hash_value_in_array().";
    /// let mut hash_code = [0_u64; 4];
    /// hash.digest_str(txt);
    /// hash.put_hash_value_in_array(&mut hash_code);
    /// println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash_code);
    /// assert_eq!(format!("{:016X?}", hash_code), "[9BEF237372571C24, 77A1E2AFFDC98530, A0B9D10323B70681, 436DAE1631785347]");
    /// ```
    /// 
    /// # Example 2 for SHA2_512_t_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256_Expanded;
    /// type MySHA2 = SHA2_512_t_256_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method put_hash_value_in_array().";
    /// let mut hash_code = [0_u64; 4];
    /// my_hash.digest_str(txt);
    /// my_hash.put_hash_value_in_array(&mut hash_code);
    /// println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash_code);
    /// assert_eq!(format!("{:016X?}", hash_code), "[DFE2E754EF11C0D9, 01EAC6F6FF8C0BDB, FF5A2F28DA6C75FA, E0A9B70B2498F0AC]");
    /// ```
    /// 
    /// # Example 3 for SHA2_512_t_224
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224;
    /// let mut hash = SHA2_512_t_224::new();
    /// let txt = "This is an example of the method put_hash_value_in_array().";
    /// let mut hash_code = [0_u32; 7];
    /// hash.digest_str(txt);
    /// hash.put_hash_value_in_array(&mut hash_code);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    /// assert_eq!(format!("{:08X?}", hash_code), "[4475114A, BD7D631C, 0A487709, 8B533EBA, 29C83AAF, BD7EB4EF, 77256C3D]");
    /// ```
    /// 
    /// # Example 4 for SHA2_512_t_224_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224_Expanded;
    /// type MySHA2 = SHA2_512_t_224_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method put_hash_value_in_array().";
    /// let mut hash_code = [0_u32; 7];
    /// my_hash.digest_str(txt);
    /// my_hash.put_hash_value_in_array(&mut hash_code);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    /// assert_eq!(format!("{:08X?}", hash_code), "[3AAAFB02, A794F5B0, 917078C8, 20548087, 3A7C909F, 4F998CBE, 3A926E6B]");
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
        let length = if out_size < 64 {out_size} else {64};
        unsafe { copy_nonoverlapping(res.as_ptr() as *const u8, out as *mut T as *mut u8, length); }
    }

    // pub fn tangle(&mut self, tangling: u64)
    /// Tangles the hash value
    /// 
    /// # Argument
    /// u64 constants to tangle the hash value
    /// 
    /// # Features
    /// It is for using this struct as random number generator.
    /// 
    /// # Example 1 for SHA2_512_t_256
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256;
    /// let mut hash = SHA2_512_t_256::new();
    /// let txt = "TANGLING";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash.get_hash_value_in_array_tm::<u64, 4>());
    /// assert_eq!(format!("{:016X?}", hash.get_hash_value_in_array_tm::<u64, 4>()), "[FC36648637962C38, BDFBBAE5DEA75E0E, D72827D56EB79EF9, 4969BAA99DB0E42B]");
    /// hash.tangle(1);
    /// println!("Hash =\t{:016X?}", hash.get_hash_value_in_array_tm::<u64, 4>());
    /// assert_eq!(format!("{:016X?}", hash.get_hash_value_in_array_tm::<u64, 4>()), "[96CA6859E014C355, 6BBED0E8DA26FFAD, A4F89477C93C9E8C, 806148BDB037AE26]");
    /// hash.tangle(1);
    /// println!("Hash =\t{:016X?}", hash.get_hash_value_in_array_tm::<u64, 4>());
    /// assert_eq!(format!("{:016X?}", hash.get_hash_value_in_array_tm::<u64, 4>()), "[11F5369ABC9E3B5D, D3D869131E697AB2, 1899C8D791BB09FC, 0C6CE82AE3B9D583]");
    /// ```
    /// 
    /// # Example 2 for SHA2_512_t_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256_Expanded;
    /// type MySHA2 = SHA2_512_t_256_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "TANGLING";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, my_hash.get_hash_value_in_array_tm::<u64, 4>());
    /// assert_eq!(format!("{:016X?}", my_hash.get_hash_value_in_array_tm::<u64, 4>()), "[C60A42A16859F8B8, 7EAB94538B024642, 654DD7795DDDD39B, 12E1A03748AEFFF3]");
    /// my_hash.tangle(1);
    /// println!("Hash =\t{:016X?}", my_hash.get_hash_value_in_array_tm::<u64, 4>());
    /// assert_eq!(format!("{:016X?}", my_hash.get_hash_value_in_array_tm::<u64, 4>()), "[05A82162DE47FEE5, 4B7C2320AF525665, 0D9A9FC79B16B8E6, B51D2D5242BADECD]");
    /// my_hash.tangle(1);
    /// println!("Hash =\t{:016X?}", my_hash.get_hash_value_in_array_tm::<u64, 4>());
    /// assert_eq!(format!("{:016X?}", my_hash.get_hash_value_in_array_tm::<u64, 4>()), "[BC74B5902DD2AB00, 680C9FE85FED5E60, 4FAAF51214292837, B9292AFDBF94B64E]");
    /// ```
    /// 
    /// # Example 3 for SHA2_512_t_224
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224;
    /// let mut hash = SHA2_512_t_224::new();
    /// let txt = "TANGLING";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_hash_value_in_array_tm::<u32, 7>());
    /// assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array_tm::<u32, 7>()), "[72E2E82F, C78389DA, 112F494F, B415B8C4, EF993BFA, EDB5091B, 8C03F067]");
    /// hash.tangle(1);
    /// println!("Hash =\t{:08X?}", hash.get_hash_value_in_array_tm::<u32, 7>());
    /// assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array_tm::<u32, 7>()), "[A7CED549, 2C050740, 9BC2F6E5, EAC6D908, 26148AE9, 966D5E72, ED5DF840]");
    /// hash.tangle(1);
    /// println!("Hash =\t{:08X?}", hash.get_hash_value_in_array_tm::<u32, 7>());
    /// assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array_tm::<u32, 7>()), "[14C24EAE, B39CD243, 8C484722, CB1A03AA, F1F9F55E, 955A27D8, 70A3ED4F]");
    /// ```
    /// 
    /// # Example 4 for SHA2_512_t_224_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224_Expanded;
    /// type MySHA2 = SHA2_512_t_224_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "TANGLING";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_hash_value_in_array_tm::<u32, 7>());
    /// assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array_tm::<u32, 7>()), "[6EF90662, CD08A7EA, 93D0EDFC, 390175A6, 53368038, ADC8BCC8, 11351AB8]");
    /// my_hash.tangle(1);
    /// println!("Hash =\t{:08X?}", my_hash.get_hash_value_in_array_tm::<u32, 7>());
    /// assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array_tm::<u32, 7>()), "[F7566CAF, B1039FF1, 722C9B99, 5AA84D67, E6C1182A, 3B4D2DBF, 7F1FA1C8]");
    /// my_hash.tangle(1);
    /// println!("Hash =\t{:08X?}", my_hash.get_hash_value_in_array_tm::<u32, 7>());
    /// assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array_tm::<u32, 7>()), "[5B74C46E, F433ACC6, 6A402398, 39126678, 581E67AD, 14A4C823, 4B387049]");
    /// ```
    #[inline]
    pub fn tangle(&mut self, tangling: u64)
    {
        let mut m = [0_u64; 9];
        for i in 0..8
            { m[i] = self.hash_code[i].get(); }
        m[8] = tangling;
        self.finalize(m.as_ptr() as *const u8, 72);
    }

    // fn initialize(&mut self)
    /// Initializes all five self.hash_code[] with predetermined values H[].
    fn initialize(&mut self)
    {
        for i in 0..8_usize
            { self.hash_code[i] = LongUnion::new_with(self.o[i]); }
    }

    // fn update(&mut self, message: &[u32])
    /// This method is the core part of MD5 hash algorithm.
    /// It has sixty-four rounds. It updates self.hash_code[] for those
    /// sixty-four rounds.
    fn update(&mut self, message: &[u64])
    {
        let mut w = [0_u64; 16];
        let mut a = self.hash_code[0].get();
        let mut b = self.hash_code[1].get();
        let mut c = self.hash_code[2].get();
        let mut d = self.hash_code[3].get();
        let mut e = self.hash_code[4].get();
        let mut f = self.hash_code[5].get();
        let mut g = self.hash_code[6].get();
        let mut h = self.hash_code[7].get();

        for i in 0..16_usize
        {
            w[i] = message[i].to_be();
            let s1 = e.rotate_right(RR14) ^ e.rotate_right(RR18) ^ e.rotate_right(RR41);
            let t1 = Self::ch(e, f, g).wrapping_add(h)
                                .wrapping_add(Self::get_k(i))
                                .wrapping_add(w[i])
                                .wrapping_add(s1);
            let s0 = a.rotate_right(RR28) ^ a.rotate_right(RR34) ^ a.rotate_right(RR39);
            let t2 = Self::maj(a, b, c).wrapping_add(s0);
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        for i in 16..ROUND // ROUND = 64_usize for officiial SHA-2/256
        {
            let j = i & 0b1111;
            w[j] = Self::get_w(&w, i);
            let s1 = e.rotate_right(RR14) ^ e.rotate_right(RR18) ^ e.rotate_right(RR41);
            let t1 = Self::ch(e, f, g).wrapping_add(h)
                                .wrapping_add(Self::get_k(i))
                                .wrapping_add(w[j])
                                .wrapping_add(s1);
            let s0 = a.rotate_right(RR28) ^ a.rotate_right(RR34) ^ a.rotate_right(RR39);
            let t2 = Self::maj(a, b, c).wrapping_add(s0);
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }

        self.hash_code[0].set(self.hash_code[0].get().wrapping_add(a));
        self.hash_code[1].set(self.hash_code[1].get().wrapping_add(b));
        self.hash_code[2].set(self.hash_code[2].get().wrapping_add(c));
        self.hash_code[3].set(self.hash_code[3].get().wrapping_add(d));
        self.hash_code[4].set(self.hash_code[4].get().wrapping_add(e));
        self.hash_code[5].set(self.hash_code[5].get().wrapping_add(f));
        self.hash_code[6].set(self.hash_code[6].get().wrapping_add(g));
        self.hash_code[7].set(self.hash_code[7].get().wrapping_add(h));
    }

    // fn finalize(&mut self, message: *const u8, length_in_bytes: u64)
    /// finalizes the hash process. In this process, it pads with padding bits,
    /// which are bit one, bits zeros, and eight bytes that show the message
    /// size in the unit of bits with big endianness so as to make the data
    /// (message + padding bits) be multiples of 512 bits (64 bytes).
    fn finalize(&mut self, message: *const u8, length_in_bytes: u128)
    {
        type ChunkType = u128;
        type PieceType = u64;
        const MESSAGE_NUM: usize = 128;
        const LAST_BYTES: ChunkType = 0b111_1111;
        union MU
        {
            chunk: [ChunkType; 8],
            piece: [PieceType; 16],
            txt: [u8; MESSAGE_NUM],
        }

        let mut mu = MU { txt: [0; MESSAGE_NUM] };
        let last_bytes = (length_in_bytes & LAST_BYTES) as usize;    // equivalent to (length_in_bytes % 128) as usize
        unsafe { copy_nonoverlapping(message, mu.txt.as_mut_ptr(), last_bytes); }
        unsafe { mu.txt[last_bytes] = 0b1000_0000; }
        // 데이터 기록후, 데이터의 길이를 비트 단위로 기록하기 위한 64 비트(8 바이트)와
        // 0b1000_0000를 기록하기 위한 한 바이트의 여유공간이 남아있지 않으면,
        if last_bytes > 110  // (128 - 16 - 1)
        {
            self.update(unsafe {&mu.piece});
            for i in 0..7
                { unsafe { mu.chunk[i] = 0; } }
        }
        unsafe { mu.chunk[7] = (length_in_bytes << 3).to_be(); }    // 데이터 길이의 단위는 바이트가 아니라 비트이다.
        self.update(unsafe {&mu.piece});
    }

	#[inline] fn get_k(idx: usize) -> u64    { Self::K[idx % 80] }
    #[inline] fn get_s0(w: &[u64; 16], idx: usize) -> u64  { let ww = w[(idx-15) & 0b1111]; ww.rotate_right(RR1) ^ ww.rotate_right(RR8) ^ (ww >> SR7) }
    #[inline] fn get_s1(w: &[u64; 16], idx: usize) -> u64  { let ww = w[(idx-2) & 0b1111]; ww.rotate_right(RR19) ^ ww.rotate_right(RR61) ^ (ww >> SR6) }
    #[inline] fn get_w(w: &[u64; 16], idx: usize) -> u64   { w[(idx-16) & 0b1111].wrapping_add(Self::get_s0(&w, idx)).wrapping_add(w[(idx-7) & 0b1111]).wrapping_add(Self::get_s1(&w, idx)) }
	#[inline] fn ch(x: u64, y: u64, z: u64) -> u64  { z ^ (x & (y ^ z)) }   // equivalent to { (x & y) | (!x & z) }
	#[inline] fn maj(x: u64, y: u64, z: u64) -> u64  { (x & y) | (z & (x | y)) } // equivalent to { (x & y) | (y & z) | (z & x) }
    #[inline] fn to_char(nibble: u8) -> char    { if nibble < 10  { ('0' as u8 + nibble) as u8 as char } else { ('A' as u8 - 10 + nibble) as char } }
}


#[allow(non_upper_case_globals)]
impl<const t: usize, const A5A5A5A5A5A5A5A5: u64,
    const H0: u64, const H1: u64, const H2: u64, const H3: u64,
    const H4: u64, const H5: u64, const H6: u64, const H7: u64,
    const ROUND: usize,
    const K00: u64, const K01: u64, const K02: u64, const K03: u64,
    const K04: u64, const K05: u64, const K06: u64, const K07: u64,
    const K08: u64, const K09: u64, const K10: u64, const K11: u64,
    const K12: u64, const K13: u64, const K14: u64, const K15: u64,
    const K16: u64, const K17: u64, const K18: u64, const K19: u64,
    const K20: u64, const K21: u64, const K22: u64, const K23: u64,
    const K24: u64, const K25: u64, const K26: u64, const K27: u64,
    const K28: u64, const K29: u64, const K30: u64, const K31: u64,
    const K32: u64, const K33: u64, const K34: u64, const K35: u64,
    const K36: u64, const K37: u64, const K38: u64, const K39: u64,
    const K40: u64, const K41: u64, const K42: u64, const K43: u64,
    const K44: u64, const K45: u64, const K46: u64, const K47: u64,
    const K48: u64, const K49: u64, const K50: u64, const K51: u64,
    const K52: u64, const K53: u64, const K54: u64, const K55: u64,
    const K56: u64, const K57: u64, const K58: u64, const K59: u64,
    const K60: u64, const K61: u64, const K62: u64, const K63: u64,
    const K64: u64, const K65: u64, const K66: u64, const K67: u64,
    const K68: u64, const K69: u64, const K70: u64, const K71: u64,
    const K72: u64, const K73: u64, const K74: u64, const K75: u64,
    const K76: u64, const K77: u64, const K78: u64, const K79: u64,
    const RR1: u32, const RR8: u32, const RR14: u32, const RR18: u32, 
    const RR19: u32, const RR28: u32, const RR34: u32, const RR39: u32, 
    const RR41: u32, const RR61: u32, const SR6: i32, const SR7: i32>
Display for SHA2_512_t_Generic<t, A5A5A5A5A5A5A5A5,
                                H0, H1, H2, H3, H4, H5, H6, H7, ROUND,
                                K00, K01, K02, K03, K04, K05, K06, K07,
                                K08, K09, K10, K11, K12, K13, K14, K15,
                                K16, K17, K18, K19, K20, K21, K22, K23,
                                K24, K25, K26, K27, K28, K29, K30, K31,
                                K32, K33, K34, K35, K36, K37, K38, K39,
                                K40, K41, K42, K43, K44, K45, K46, K47,
                                K48, K49, K50, K51, K52, K53, K54, K55,
                                K56, K57, K58, K59, K60, K61, K62, K63,
                                K64, K65, K66, K67, K68, K69, K70, K71,
                                K72, K73, K74, K75, K76, K77, K78, K79,
                                RR1, RR8, RR14, RR18, RR19, RR28, RR34,
                                RR39, RR41, RR61, SR6, SR7>
{
    /// Formats the value using the given formatter.
    /// You will hardly use this method directly.
    /// Automagically the function `to_string()` will be implemented. So, you
    /// can use the function `to_string()`, and you can also print the SHA-1
    /// object in the macro `println!()` directly for example.
    /// `f` is a buffer, this method must write the formatted string into it.
    /// [Read more](https://doc.rust-lang.org/core/fmt/trait.Display.html#tymethod.fmt)
    /// 
    /// # Example 1 for SHA2_512_t_256 for the method to_string()
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256;
    /// let mut hash = SHA2_512_t_256::new();
    /// let txt = "Display::fmt() automagically implement to_string().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
    /// assert_eq!(hash.to_string(), "5ED309022841125DE856B25C56A741166872A1D681DF5C69F84AD8B2F30E6DD8");
    /// ```
    /// 
    /// # Example 2 for SHA2_512_t_256_Expanded for the method to_string()
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256_Expanded;
    /// type MySHA2 = SHA2_512_t_256_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "Display::fmt() automagically implement to_string().";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.to_string());
    /// assert_eq!(my_hash.to_string(), "377687DECF57B2340B282130B55C74C349376F8727BECA86C904673CD8CD50A7");
    /// ```
    /// 
    /// # Example 3 for SHA2_512_t_224 for the method to_string()
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224;
    /// let mut hash = SHA2_512_t_224::new();
    /// let txt = "Display::fmt() automagically implement to_string().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
    /// assert_eq!(hash.to_string(), "0FFD651E288004466FF247808E1FF5B482AFF547E94C66FF507BF021");
    /// ```
    /// 
    /// # Example 4 for SHA2_512_t_224_Expanded for the method to_string()
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224_Expanded;
    /// type MySHA2 = SHA2_512_t_224_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "Display::fmt() automagically implement to_string().";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.to_string());
    /// assert_eq!(my_hash.to_string(), "B130035D26D3BED1F991CB78DFC93F39F8CEF176BC4D7CF8B266027B");
    /// ```
    /// 
    /// # Example 5 for SHA2_512_t_256 for the use in the macro println!()
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256;
    /// let mut hash = SHA2_512_t_256::new();
    /// let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "660F8CA5DDC61C43BCEBAB6B8FFD4081F9015CE9A7800BFE29B5100709C3E232");
    /// ```
    /// 
    /// # Example 6 for SHA2_512_t_256_Expanded for the use in the macro println!()
    /// ```
    /// use cryptocol::hash::SHA2_512_t_256_Expanded;
    /// type MySHA2 = SHA2_512_t_256_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "C44370416B0925577339DECCC8529A68D29D4E79083658F260D4219DA6C2B0D1");
    /// ```
    /// 
    /// # Example 7 for SHA2_512_t_224 for the use in the macro println!()
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224;
    /// let mut hash = SHA2_512_t_224::new();
    /// let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "7988DFC3FB4BB8DB449B189C5D906901921C1AC0D60D94376B498795");
    /// ```
    /// 
    /// # Example 8 for SHA2_512_t_224_Expanded for the use in the macro println!()
    /// ```
    /// use cryptocol::hash::SHA2_512_t_224_Expanded;
    /// type MySHA2 = SHA2_512_t_224_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "831E095076EAE29CBC5BD2960D074BAC9E07C9189B9A5FCAE29FD5DB");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)
        write!(f, "{}", self.get_hash_value_in_string())
    }
}
