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


use std::ptr::copy_nonoverlapping;
use std::slice::from_raw_parts;
use std::fmt::{ self, Debug, Display, Formatter };

use crate::number::{ SmallUInt, IntUnion, LongUnion };


/// You have freedom of changing H0 ~ H7, and ROUND for SHA-2-256.
/// 
/// # Generic Parameters
/// You can create your own expanded version of SHA-2-256 by
/// changing the generic parameters H0 ~ H7, and ROUND.
/// - H0 ~ H7 : the initial hash values, eight u32 values.
/// The default values of H0 ~ H7 for SHA-2-256 are defined to be first 32 bits
/// of the fractional parts of the square roots of the first 8 primes 2..19.
/// So, H0 ~ H7 for SHA-2-256 are 0x6a09e667, 0xbb67ae85, 0x3c6ef372,
/// 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, and 0x5be0cd19,
/// respectively (in big endian representation).
/// The values of H0 ~ H7 for SHA-2-224 is defined to be the second 32 bits of
/// the fractional parts of the square roots of the 9th through 16th primes
/// 23..53.  So, H0 ~ H7 for SHA-2-224 should be changed to be 0xc1059ed8,
/// 0x367cd507, 0x3070dd17, 0xf70e5939, 0xffc00b31, 0x68581511, 0x64f98fa7,
/// 0xbefa4fa4, respectively (in big endian representation).
/// - ROUND : the number of rounds. The default value of it is `64` (= 16 * 4).
#[allow(non_camel_case_types)]
pub type SHA2_256_Expanded<const H0: u32 = 0x6a09e667, const H1: u32 = 0xbb67ae85,
                        const H2: u32 = 0x3c6ef372, const H3: u32 = 0xa54ff53a,
                        const H4: u32 = 0x510e527f, const H5: u32 = 0x9b05688c,
                        const H6: u32 = 0x1f83d9ab, const H7: u32 = 0x5be0cd19,
                        const ROUND: usize = 64>
        = SHA2_256_Generic<8, H0, H1, H2, H3, H4, H5, H6, H7, ROUND>;

/// You have freedom of changing ROUND for SHA-2-224.
/// 
/// # Generic Parameters
/// You can create your own expanded version of SHA-2-224 by
/// changing the generic parameters ROUND, and K00 ~ K63.
/// - ROUND : the number of rounds. The default value of it is `64` (= 16 * 4).
#[allow(non_camel_case_types)]
pub type SHA2_224_Expanded<const ROUND: usize = 64>
        = SHA2_256_Generic<7, 0xc1059ed8, 0x367cd507, 0x3070dd17, 0xf70e5939,
                            0xffc00b31, 0x68581511, 0x64f98fa7, 0xbefa4fa4, ROUND>;

/// You have freedom of changing ROUND, and K00 ~ K63 for SHA-2-256.
/// 
/// # Generic Parameters
/// You can create your own expanded version of SHA-2-256 by
/// changing the generic parameters ROUND, and K00 ~ K63.
/// - ROUND : the number of rounds. The default value of it is `64` (= 16 * 4).
/// - K0 ~ K63 : the added values in hashing process, which are sixty-four u32
/// values and called round constants.
/// The default values of K0 ~ K63 are defined to be first 32 bits of the
/// fractional parts of the cube roots of the first 64 primes 2..311,
/// respectivey (in big endian representation). So, K0 ~ K63 are 0x428a2f98,
/// 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
/// 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74,
/// 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6,
/// 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152,
/// 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351,
/// 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354,
/// 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b, 0xc24b8b70,
/// 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
/// 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f,
/// 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa,
/// 0xa4506ceb, 0xbef9a3f7, 0xc67178f2, respectively (in big endian
/// representation).
#[allow(non_camel_case_types)]
pub type SHA2_256_Generic_HRS_fixed<const ROUND: usize = 64,
                        const K00: u32 = 0x428a2f98, const K01: u32 = 0x71374491,
                        const K02: u32 = 0xb5c0fbcf, const K03: u32 = 0xe9b5dba5,
                        const K04: u32 = 0x3956c25b, const K05: u32 = 0x59f111f1,
                        const K06: u32 = 0x923f82a4, const K07: u32 = 0xab1c5ed5,
                        const K08: u32 = 0xd807aa98, const K09: u32 = 0x12835b01,
                        const K10: u32 = 0x243185be, const K11: u32 = 0x550c7dc3,
                        const K12: u32 = 0x72be5d74, const K13: u32 = 0x80deb1fe,
                        const K14: u32 = 0x9bdc06a7, const K15: u32 = 0xc19bf174,
                        const K16: u32 = 0xe49b69c1, const K17: u32 = 0xefbe4786,
                        const K18: u32 = 0x0fc19dc6, const K19: u32 = 0x240ca1cc,
                        const K20: u32 = 0x2de92c6f, const K21: u32 = 0x4a7484aa,
                        const K22: u32 = 0x5cb0a9dc, const K23: u32 = 0x76f988da,
                        const K24: u32 = 0x983e5152, const K25: u32 = 0xa831c66d,
                        const K26: u32 = 0xb00327c8, const K27: u32 = 0xbf597fc7,
                        const K28: u32 = 0xc6e00bf3, const K29: u32 = 0xd5a79147,
                        const K30: u32 = 0x06ca6351, const K31: u32 = 0x14292967,
                        const K32: u32 = 0x27b70a85, const K33: u32 = 0x2e1b2138,
                        const K34: u32 = 0x4d2c6dfc, const K35: u32 = 0x53380d13,
                        const K36: u32 = 0x650a7354, const K37: u32 = 0x766a0abb,
                        const K38: u32 = 0x81c2c92e, const K39: u32 = 0x92722c85,
                        const K40: u32 = 0xa2bfe8a1, const K41: u32 = 0xa81a664b,
                        const K42: u32 = 0xc24b8b70, const K43: u32 = 0xc76c51a3,
                        const K44: u32 = 0xd192e819, const K45: u32 = 0xd6990624,
                        const K46: u32 = 0xf40e3585, const K47: u32 = 0x106aa070,
                        const K48: u32 = 0x19a4c116, const K49: u32 = 0x1e376c08,
                        const K50: u32 = 0x2748774c, const K51: u32 = 0x34b0bcb5,
                        const K52: u32 = 0x391c0cb3, const K53: u32 = 0x4ed8aa4a,
                        const K54: u32 = 0x5b9cca4f, const K55: u32 = 0x682e6ff3,
                        const K56: u32 = 0x748f82ee, const K57: u32 = 0x78a5636f,
                        const K58: u32 = 0x84c87814, const K59: u32 = 0x8cc70208,
                        const K60: u32 = 0x90befffa, const K61: u32 = 0xa4506ceb,
                        const K62: u32 = 0xbef9a3f7, const K63: u32 = 0xc67178f2>
    = SHA2_256_Generic<8, 0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
                        0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19, ROUND,
                        K00, K01, K02, K03, K04, K05, K06, K07,
                        K08, K09, K10, K11, K12, K13, K14, K15,
                        K16, K17, K18, K19, K20, K21, K22, K23,
                        K24, K25, K26, K27, K28, K29, K30, K31,
                        K32, K33, K34, K35, K36, K37, K38, K39,
                        K40, K41, K42, K43, K44, K45, K46, K47,
                        K48, K49, K50, K51, K52, K53, K54, K55,
                        K56, K57, K58, K59,K60, K61, K62, K63,
                        2, 6, 7, 11, 13, 17, 18, 19, 22, 25, 3, 10>;

/// You have freedom of changing ROUND, and K00 ~ K63 for SHA-2-224.
/// 
/// # Generic Parameters
/// You can create your own expanded version of SHA-2-224 by
/// changing the generic parameters ROUND, and K00 ~ K63.
/// - ROUND : the number of rounds. The default value of it is `64` (= 16 * 4).
/// - K0 ~ K63 : the added values in hashing process, which are sixty-four u32
/// values and called round constants.
/// The default values of K0 ~ K63 are defined to be first 32 bits of the
/// fractional parts of the cube roots of the first 64 primes 2..311,
/// respectivey (in big endian representation). So, K0 ~ K63 are 0x428a2f98,
/// 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
/// 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74,
/// 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6,
/// 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152,
/// 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351,
/// 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354,
/// 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b, 0xc24b8b70,
/// 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
/// 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f,
/// 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa,
/// 0xa4506ceb, 0xbef9a3f7, 0xc67178f2, respectively (in big endian
/// representation).
#[allow(non_camel_case_types)]
pub type SHA2_224_Generic_HRS_fixed<const ROUND: usize = 64,
                        const K00: u32 = 0x428a2f98, const K01: u32 = 0x71374491,
                        const K02: u32 = 0xb5c0fbcf, const K03: u32 = 0xe9b5dba5,
                        const K04: u32 = 0x3956c25b, const K05: u32 = 0x59f111f1,
                        const K06: u32 = 0x923f82a4, const K07: u32 = 0xab1c5ed5,
                        const K08: u32 = 0xd807aa98, const K09: u32 = 0x12835b01,
                        const K10: u32 = 0x243185be, const K11: u32 = 0x550c7dc3,
                        const K12: u32 = 0x72be5d74, const K13: u32 = 0x80deb1fe,
                        const K14: u32 = 0x9bdc06a7, const K15: u32 = 0xc19bf174,
                        const K16: u32 = 0xe49b69c1, const K17: u32 = 0xefbe4786,
                        const K18: u32 = 0x0fc19dc6, const K19: u32 = 0x240ca1cc,
                        const K20: u32 = 0x2de92c6f, const K21: u32 = 0x4a7484aa,
                        const K22: u32 = 0x5cb0a9dc, const K23: u32 = 0x76f988da,
                        const K24: u32 = 0x983e5152, const K25: u32 = 0xa831c66d,
                        const K26: u32 = 0xb00327c8, const K27: u32 = 0xbf597fc7,
                        const K28: u32 = 0xc6e00bf3, const K29: u32 = 0xd5a79147,
                        const K30: u32 = 0x06ca6351, const K31: u32 = 0x14292967,
                        const K32: u32 = 0x27b70a85, const K33: u32 = 0x2e1b2138,
                        const K34: u32 = 0x4d2c6dfc, const K35: u32 = 0x53380d13,
                        const K36: u32 = 0x650a7354, const K37: u32 = 0x766a0abb,
                        const K38: u32 = 0x81c2c92e, const K39: u32 = 0x92722c85,
                        const K40: u32 = 0xa2bfe8a1, const K41: u32 = 0xa81a664b,
                        const K42: u32 = 0xc24b8b70, const K43: u32 = 0xc76c51a3,
                        const K44: u32 = 0xd192e819, const K45: u32 = 0xd6990624,
                        const K46: u32 = 0xf40e3585, const K47: u32 = 0x106aa070,
                        const K48: u32 = 0x19a4c116, const K49: u32 = 0x1e376c08,
                        const K50: u32 = 0x2748774c, const K51: u32 = 0x34b0bcb5,
                        const K52: u32 = 0x391c0cb3, const K53: u32 = 0x4ed8aa4a,
                        const K54: u32 = 0x5b9cca4f, const K55: u32 = 0x682e6ff3,
                        const K56: u32 = 0x748f82ee, const K57: u32 = 0x78a5636f,
                        const K58: u32 = 0x84c87814, const K59: u32 = 0x8cc70208,
                        const K60: u32 = 0x90befffa, const K61: u32 = 0xa4506ceb,
                        const K62: u32 = 0xbef9a3f7, const K63: u32 = 0xc67178f2>
    = SHA2_256_Generic<7, 0xc1059ed8, 0x367cd507, 0x3070dd17, 0xf70e5939,
                        0xffc00b31, 0x68581511, 0x64f98fa7, 0xbefa4fa4, ROUND,
                        K00, K01, K02, K03, K04, K05, K06, K07,
                        K08, K09, K10, K11, K12, K13, K14, K15,
                        K16, K17, K18, K19, K20, K21, K22, K23,
                        K24, K25, K26, K27, K28, K29, K30, K31,
                        K32, K33, K34, K35, K36, K37, K38, K39,
                        K40, K41, K42, K43, K44, K45, K46, K47,
                        K48, K49, K50, K51, K52, K53, K54, K55,
                        K56, K57, K58, K59,K60, K61, K62, K63,
                        2, 6, 7, 11, 13, 17, 18, 19, 22, 25, 3, 10>;

/// The official SHA-256 hash algorithm
#[allow(non_camel_case_types)]
pub type SHA2_256 = SHA2_256_Generic;   // equivalent to `pub type SHA2_256 = SHA2_256_Expanded;`

/// The official SHA-224 hash algorithm
#[allow(non_camel_case_types)]
pub type SHA2_224 = SHA2_256_Generic<7, 0xc1059ed8, 0x367cd507, 0x3070dd17,
                        0xf70e5939, 0xffc00b31, 0x68581511, 0x64f98fa7, 
                        0xbefa4fa4, 64>;    // equivalent to `pub type SHA2_224 = SHA2_224_Expanded;``

/// SHA-2-256 and SHA-2-224 message-digest algorithms that lossily compress
/// data of arbitrary length into a 256-bit hash value and 224-bit hash value,
/// respectively, and their flexible variants that allow you to develop your
/// own `SHA-2-256`-based hash algorithms.
/// 
/// # Introduction
/// SHA-2-256 and SHA-2-224 were designed by the United States National Security
/// Agency, and are a U.S. Federal Information Processing Standard. SHA-2-256
/// and SHA-2-224 produce a message digest based on principles similar to those
/// used by Ronald L. Rivest of MIT in the design of the MD2, MD4, MD5, SHA0,
/// and SHA-1 message digest algorithms, but generates a larger hash value (256
/// and 224 bits vs. 160 and 128 bits). SHA-2-256 and SHA-2-224 use the
/// [Merkle–Damgård construction](https://en.wikipedia.org/wiki/Merkle%E2%80%93Damg%C3%A5rd_construction).
/// 
/// # Vulnerability
/// There have been several attacks against SHA-2-256 and SHA-2-224
/// but they are all incomplete attacks.
/// Read [more](https://en.wikipedia.org/wiki/SHA-2#Cryptanalysis_and_validation)
/// 
/// # Use of SHA-2-256, SHA-2-224, and their variations
/// You can use SHA-2-256 and SHA-2-224 for cryptographic purposes such as:
/// - Generating IDs
/// - Integrity test
/// - Storing passwords
/// - Digital Signature
/// - Key generation
/// - Implementing proof of work for block chain.
/// - Study of hash algorithms
/// - Cryptanalysis Research to find the weakness of SHA-256 and Merkle-Damgard
/// construction which MD2, MD4, MD5, SHA0, SHA1, and all SHA2 family use
/// 
/// # Generic Parameters
/// You can create your own expanded version of SHA-2-256 or SHA-2-224 by
/// changing the generic parameters N, H0 ~ H7, ROUND, K00 ~ K63, RR2, RR6,
/// RR7, RR11, RR13, RR17, RR18, RR19, RR22, RR25, SR3, and SR10.
/// - N : the size of output. N cannot be 0 or greater than 8. If N is 8, the
/// output is 256 bits, while if N is 1, the output is 32 bits.
/// - H0 ~ H7 : the initial hash values, eight u32 values.
/// The default values of H0 ~ H7 for SHA-2-256 are defined to be first 32 bits
/// of the fractional parts of the square roots of the first 8 primes 2..19.
/// So, H0 ~ H7 for SHA-2-256 are 0x6a09e667, 0xbb67ae85, 0x3c6ef372,
/// 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, and 0x5be0cd19,
/// respectively (in big endian representation).
/// The values of H0 ~ H7 for SHA-2-224 is defined to be the second 32 bits of
/// the fractional parts of the square roots of the 9th through 16th primes
/// 23..53.  So, H0 ~ H7 for SHA-2-224 should be changed to be 0xc1059ed8,
/// 0x367cd507, 0x3070dd17, 0xf70e5939, 0xffc00b31, 0x68581511, 0x64f98fa7,
/// 0xbefa4fa4, respectively (in big endian representation).
/// - ROUND : the number of rounds. The default value of it is `64` (= 16 * 4).
/// - K0 ~ K63 : the added values in hashing process, which are sixty-four u32
/// values and called round constants.
/// The default values of K0 ~ K63 are defined to be first 32 bits of the
/// fractional parts of the cube roots of the first 64 primes 2..311,
/// respectivey (in big endian representation). So, K0 ~ K63 are 0x428a2f98,
/// 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
/// 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74,
/// 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6,
/// 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152,
/// 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351,
/// 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354,
/// 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b, 0xc24b8b70,
/// 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
/// 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f,
/// 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa,
/// 0xa4506ceb, 0xbef9a3f7, 0xc67178f2, respectively (in big endian
/// representation).
/// - RR2, RR6, RR7, RR11, RR13, RR17, RR18, RR19, RR22, and RR25 : the amounts
/// of rotate right in the hashing process.
/// The default values of RR2, RR6, RR7, RR11, RR13, RR17, RR18, RR19, RR22,
/// and RR25 are 2, 6, 7, 11, 13, 17, 18, 19, 22, and 25, respecively.
/// - SR3, and SR10 : the amounts of shift right in the hashing process.
/// The default values of SR3, and SR10 are 3 and 10 respectively.
/// 
/// About the parameters and their default values,
/// read [more](https://en.wikipedia.org/wiki/SHA-2#Pseudocode)
/// 
/// # Security of your own expanded version
/// Your own algrorithm based on SHA-2-256 or SHA-2-224 may be stronger or
/// weaker than official SHA-2-256 or SHA-2-224. Unless you seriously checked
/// the cryptographic security of your own algorithms, it is hard to assume
/// that your own alogrithms are stronger than the official SHA-2-256 or
/// SHA-2-224.
/// 
/// Read [this](https://doi.org/10.6028/NIST.FIPS.180-4)
/// and [that](https://en.wikipedia.org/wiki/SHA-2)
/// 
/// # Quick Start
/// In order to use the module sha2_256, you don't have to import (or use)
/// cryptocol::hash::sha2_256::* directly because the module
/// cryptocol::hash::sha2_256 is re-exported. All you have to do is only
/// import SHA2_256, SHA2_224, SHA2_256_Expanded, SHA2_224_Expanded,
/// SHA2_256_Generic_HRS_fixed, and/or SHA2_256_Generic in the module
/// cryptocol::hash. Example 1 shows how to import structs SHA2_256, SHA2_224,
/// SHA2_256_Expanded, SHA2_224_Expanded, SHA2_256_Generic_HRS_fixed, and/or
/// SHA2_256_Generic. Plus, what you have to know is these. All the types (or
/// structs) are the specific versions of SHA2_256_Generic. Actually, SHA2_256
/// are specific versions of SHA2_256_Expanded. SHA2_224 is a specific version
/// of SHA2_224_Expanded. SHA2_256_Expanded, SHA2_224_Expanded,
/// SHA2_256_Generic_HRS_fixed, and SHA2_224_Generic_HRS_fixed are specific
/// versions of SHA2_256_Generic.
/// 
/// ## Example 1
/// ```
/// use cryptocol::hash::SHA2_256;
/// use cryptocol::hash::SHA2_224;
/// use cryptocol::hash::SHA2_256_Expanded;
/// use cryptocol::hash::SHA2_224_Expanded;
/// use cryptocol::hash::SHA2_256_Generic_HRS_fixed;
/// use cryptocol::hash::SHA2_224_Generic_HRS_fixed;
/// use cryptocol::hash::SHA2_256_Generic;
/// ```
/// Then, you can create SHA2_256 object by the method SHA2_256::new() for
/// example. Now, you are ready to use all prepared methods to hash any data.
/// If you want to hash a string, for example, you can use the method
/// digest_str(). Then, the SHA2_256 object that you created will contain its
/// hash value. You can use the macro println!() for instance to print on a
/// commandline screen by `println!("{}", hash)` where hash is the SHA1 object.
/// Example 2 shows how to use SHA2_256 struct quickly.
/// 
/// ## Example 2 for SHA2_256
/// ```
/// use std::string::*;
/// use cryptocol::hash::SHA2_256;
/// let mut hash = SHA2_256::new();
/// 
/// let mut txt = "";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.get_hash_value_in_string());
/// assert_eq!(hash.get_hash_value_in_string(), "E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855");
/// 
/// let txt_stirng = String::from("A");
/// hash.digest_string(&txt_stirng);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt_stirng, hash);
/// assert_eq!(hash.to_string(), "559AEAD08264D5795D3909718CDD05ABD49572E84FE55590EEF31A88A08FDFFD");
/// 
/// let txt_array = ['W' as u8, 'o' as u8, 'w' as u8];
/// hash.digest_array(&txt_array);
/// println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txt_array, hash);
/// assert_eq!(hash.get_hash_value_in_string(), "3DECCF6826EF78994F099EC321F883527E8218301605E66114268E769D1DF61E");
/// 
/// txt = "This data is 26-byte long.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.to_string(), "9546AE34CBF111CEDC01164DE817512B4DC3B1F9967E208068868BF557E9972A");
/// 
/// txt = "The unit of data length is not byte but bit.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.get_hash_value_in_string(), "5ADF1754644575EB30E7EBCE1F5EA2AD52E99CDD98713B805B2B2F02CACB3E31");
/// 
/// txt = "I am testing SHA-2/256 for the data whose length is 62 bytes..";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.to_string(), "12C0E6762B448D5FBFF941D54F932BBFAE308E3EBDEE1795555681D3D9A2C5CF");
/// 
/// let mut txt = "I am testing SHA-2/256 for the data whose length is sixty-four bytes.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.get_hash_value_in_string(), "222268D061DF342E7142E79B49EAF57D34B74212D2150C5CA93EF05C767EA5F3");
/// 
/// txt = "I am testing SHA-2/256 for the case data whose length is more than sixty-four bytes is given.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.to_string(), "D8B1AFE899AA3E02E44EA1603730790791B92BB5A6D14632EB44131BE21334C7");
/// 
/// txt = "This hash algorithm SHA-2/256 is being tested for the case message whose length is more than one hundred thirty-one bytes is given.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
/// assert_eq!(hash.to_string(), "8604990CF14A490072D6EC3BD5182079C7B46F9F7E18E684C2C9E645CFA8FEF0");
/// ```
/// 
/// ## Example 3 for SHA2_224
/// ```
/// use std::string::*;
/// use cryptocol::hash::SHA2_224;
/// let mut hash = SHA2_224::new();
/// 
/// let mut txt = "";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.get_hash_value_in_string());
/// assert_eq!(hash.get_hash_value_in_string(), "D14A028C2A3A2BC9476102BB288234C415A2B01F828EA62AC5B3E42F");
/// 
/// let txt_stirng = String::from("A");
/// hash.digest_string(&txt_stirng);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt_stirng, hash);
/// assert_eq!(hash.to_string(), "5CFE2CDDBB9940FB4D8505E25EA77E763A0077693DBB01B1A6AA94F2");
/// 
/// let txt_array = ['W' as u8, 'o' as u8, 'w' as u8];
/// hash.digest_array(&txt_array);
/// println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txt_array, hash);
/// assert_eq!(hash.get_hash_value_in_string(), "8B73B7B79FA0E4EC45AF8B4230F88F314554D503FD88F05A48A07DD3");
/// 
/// txt = "This data is 26-byte long.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.to_string(), "0592A67F23DD6B21CA691041B4682831C61D40E0235CEB59AC557358");
/// 
/// txt = "The unit of data length is not byte but bit.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.get_hash_value_in_string(), "8949B6F7EB831F47B81E3361135D835E93576ED5BAAA32209303C37C");
/// 
/// txt = "I am testing SHA-2/224 for the data whose length is 62 bytes..";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.to_string(), "84E6CB12422BB17F614D03B95E0DF142F6FD4EABB69E59A3C7C8A1AA");
/// 
/// let mut txt = "I am testing SHA-2/224 for the data whose length is sixty-four bytes.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.get_hash_value_in_string(), "FB297817767C2236610810DC9BE34EFB2FDCC0E0C7E2D0BA736C59DB");
/// 
/// txt = "I am testing SHA-2/224 for the case data whose length is more than sixty-four bytes is given.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
/// assert_eq!(hash.to_string(), "DAA4205BB0B38C625AD8A53DAF1FC8A61AFA33D7513B3615826750FD");
/// ```
/// 
/// # Big-endian issue
/// It is just experimental for Big Endian CPUs. So, you are not encouraged
/// to use it for Big Endian CPUs for serious purpose. Only use this crate
/// for Big-endian CPUs with your own full responsibility.
/// 
/// # A Simple but Useful Application using cryptocol
/// The following is the source code of the commandline SHA2_256 hash value 
/// extractor using the struct SHA1 of this module. You can get the hash value
/// from a text or a file. The following source code assumes its executable
/// file name will be "sha2_256_app". You can find all the examples including
/// the following source code in the folder "examples" of this crate.
/// ```
/// use std::{ io, env, fs };
/// use std::io::BufRead;
/// use std::convert::From;
/// use cryptocol::hash::SHA2_256;
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
///     let mut hash = SHA2_256::new();
///     hash.digest_str(txt);
///     println!("Hash value:\t{}", hash.get_hash_value_in_string());
/// }
/// 
/// fn get_hash_value_from_file(file: &str)
/// {
///     if let Ok(contents) = fs::read(file)
///     {
///         let mut hash = SHA2_256::new();
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
///                         let mut hash = SHA2_256::new();
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
///     println!("This is an SHA2_256 hash value extractor from a text or a file, using cryptocol.");
///     println!("Usage: sha2_256_app <option> <source>");
///     println!("options       description");
///     println!("--text, -t    : <source> is a text to get a hash code.");
///     println!("                The text should be enclosed by ' or \".");
///     println!("--file, -f    : <source> is the name of the file to get a hash code.");
///     println!("--check, -c   : <source> is the name of the file that contains pairs");
///     println!("                of file and its hash code.");
///     println!("--help, -h    : print this help message on screen\n");
///     println!("Examples:");
///     println!("\tsha2_256_app -t 'How are you doing?'");
///     println!("\tsha2_256_app -f linuxmint-21.3-cinnamon-64bit.iso");
///     println!("\tsha2_256_app -c CHECKSUM");
/// }
/// ```
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct SHA2_256_Generic<const N: usize = 8,
                            const H0: u32 = 0x6a09e667, const H1: u32 = 0xbb67ae85, 
                            const H2: u32 = 0x3c6ef372, const H3: u32 = 0xa54ff53a,
                            const H4: u32 = 0x510e527f, const H5: u32 = 0x9b05688c, 
                            const H6: u32 = 0x1f83d9ab, const H7: u32 = 0x5be0cd19,
                            const ROUND: usize = 64,
                            const K00: u32 = 0x428a2f98, const K01: u32 = 0x71374491,
                            const K02: u32 = 0xb5c0fbcf, const K03: u32 = 0xe9b5dba5,
                            const K04: u32 = 0x3956c25b, const K05: u32 = 0x59f111f1,
                            const K06: u32 = 0x923f82a4, const K07: u32 = 0xab1c5ed5,
                            const K08: u32 = 0xd807aa98, const K09: u32 = 0x12835b01,
                            const K10: u32 = 0x243185be, const K11: u32 = 0x550c7dc3,
                            const K12: u32 = 0x72be5d74, const K13: u32 = 0x80deb1fe,
                            const K14: u32 = 0x9bdc06a7, const K15: u32 = 0xc19bf174,
                            const K16: u32 = 0xe49b69c1, const K17: u32 = 0xefbe4786,
                            const K18: u32 = 0x0fc19dc6, const K19: u32 = 0x240ca1cc,
                            const K20: u32 = 0x2de92c6f, const K21: u32 = 0x4a7484aa,
                            const K22: u32 = 0x5cb0a9dc, const K23: u32 = 0x76f988da,
                            const K24: u32 = 0x983e5152, const K25: u32 = 0xa831c66d,
                            const K26: u32 = 0xb00327c8, const K27: u32 = 0xbf597fc7,
                            const K28: u32 = 0xc6e00bf3, const K29: u32 = 0xd5a79147,
                            const K30: u32 = 0x06ca6351, const K31: u32 = 0x14292967,
                            const K32: u32 = 0x27b70a85, const K33: u32 = 0x2e1b2138,
                            const K34: u32 = 0x4d2c6dfc, const K35: u32 = 0x53380d13,
                            const K36: u32 = 0x650a7354, const K37: u32 = 0x766a0abb,
                            const K38: u32 = 0x81c2c92e, const K39: u32 = 0x92722c85,
                            const K40: u32 = 0xa2bfe8a1, const K41: u32 = 0xa81a664b,
                            const K42: u32 = 0xc24b8b70, const K43: u32 = 0xc76c51a3,
                            const K44: u32 = 0xd192e819, const K45: u32 = 0xd6990624,
                            const K46: u32 = 0xf40e3585, const K47: u32 = 0x106aa070,
                            const K48: u32 = 0x19a4c116, const K49: u32 = 0x1e376c08,
                            const K50: u32 = 0x2748774c, const K51: u32 = 0x34b0bcb5,
                            const K52: u32 = 0x391c0cb3, const K53: u32 = 0x4ed8aa4a,
                            const K54: u32 = 0x5b9cca4f, const K55: u32 = 0x682e6ff3,
                            const K56: u32 = 0x748f82ee, const K57: u32 = 0x78a5636f,
                            const K58: u32 = 0x84c87814, const K59: u32 = 0x8cc70208,
                            const K60: u32 = 0x90befffa, const K61: u32 = 0xa4506ceb,
                            const K62: u32 = 0xbef9a3f7, const K63: u32 = 0xc67178f2,
                            const RR2: u32 = 2, const RR6: u32 = 6, const RR7: u32 = 7, 
                            const RR11: u32 = 11, const RR13: u32 = 13, const RR17: u32 = 17, 
                            const RR18: u32 = 18, const RR19: u32 = 19, const RR22: u32 = 22, 
                            const RR25: u32 = 25, const SR3: i32 = 3, const SR10: i32 = 10>
{
    hash_code: [IntUnion; 8],
}

impl<const N: usize, const H0: u32, const H1: u32, const H2: u32, const H3: u32,
    const H4: u32, const H5: u32, const H6: u32, const H7: u32, const ROUND: usize,
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
    const RR2: u32, const RR6: u32, const RR7: u32, const RR11: u32, 
    const RR13: u32, const RR17: u32, const RR18: u32, const RR19: u32, 
    const RR22: u32, const RR25: u32, const SR3: i32, const SR10: i32>
SHA2_256_Generic<N, H0, H1, H2, H3, H4, H5, H6, H7, ROUND,
                K00, K01, K02, K03, K04, K05, K06, K07,
                K08, K09, K10, K11, K12, K13, K14, K15,
                K16, K17, K18, K19, K20, K21, K22, K23,
                K24, K25, K26, K27, K28, K29, K30, K31,
                K32, K33, K34, K35, K36, K37, K38, K39,
                K40, K41, K42, K43, K44, K45, K46, K47,
                K48, K49, K50, K51, K52, K53, K54, K55,
                K56, K57, K58, K59, K60, K61, K62, K63,
                RR2, RR6, RR7, RR11, RR13, RR17, RR18, RR19, 
                RR22, RR25, SR3, SR10>
{
    const K: [u32; 64] = [  K00, K01, K02, K03, K04, K05, K06, K07,
                            K08, K09, K10, K11, K12, K13, K14, K15,
                            K16, K17, K18, K19, K20, K21, K22, K23,
                            K24, K25, K26, K27, K28, K29, K30, K31,
                            K32, K33, K34, K35, K36, K37, K38, K39,
                            K40, K41, K42, K43, K44, K45, K46, K47,
                            K48, K49, K50, K51, K52, K53, K54, K55,
                            K56, K57, K58, K59, K60, K61, K62, K63 ];
    const H: [u32; 8] = [ H0, H1, H2, H3, H4, H5, H6, H7 ];

    // pub fn new() -> Self
    /// Constructs a new object of `SHA2_256` or `SHA2_224`,
    /// or a new SHA2_256-based object.
    /// 
    /// # Output
    /// A new object of `SHA2_256` or `SHA2_224`,
    /// or a new SHA2_256-based object.
    /// 
    /// # Initialization
    /// All the attributes of the constructed object of SHA2_256, which is
    /// initial hash value, will be initialized with
    /// `6A09E667BB67AE853C6EF372A54FF53A510E527F9B05688C1F83D9AB5BE0CD19`.
    /// All the attributes of the constructed object of SHA2_224, which is
    /// initial hash value, will be initialized with
    /// `C1059ED8367CD5073070DD17F70E5939FFC00B316858151164F98FA7`.
    /// 
    /// # Example 1 for SHA2_256
    /// ```
    /// use cryptocol::hash::SHA2_256;
    /// let hash = SHA2_256::new();
    /// println!("Hash =\t{}", hash);
    /// assert_eq!(hash.to_string(), "6A09E667BB67AE853C6EF372A54FF53A510E527F9B05688C1F83D9AB5BE0CD19");
    /// ```
    /// 
    /// # Example 2 for SHA2_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_256_Expanded;
    /// type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    /// let my_hash = MySHA2::new();
    /// println!("Hash =\t{}", my_hash);
    /// assert_eq!(my_hash.to_string(), "1111111122222222444444446666666688888888AAAAAAAACCCCCCCCEEEEEEEE");
    /// ```
    /// 
    /// # Example 3 for SHA2_224
    /// ```
    /// use cryptocol::hash::SHA2_224;
    /// let hash = SHA2_224::new();
    /// println!("Hash =\t{}", hash);
    /// assert_eq!(hash.to_string(), "C1059ED8367CD5073070DD17F70E5939FFC00B316858151164F98FA7");
    /// ```
    /// 
    /// # Example 4 for SHA2_224_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_224_Expanded;
    /// type MySHA2 = SHA2_224_Expanded<128>;
    /// let my_hash = MySHA2::new();
    /// println!("Hash =\t{}", my_hash);
    /// assert_eq!(my_hash.to_string(), "C1059ED8367CD5073070DD17F70E5939FFC00B316858151164F98FA7");
    /// ```
    pub fn new() -> Self
    {
        if (N > 8) | (N == 0)
            { panic!("N cannot be 0 or greater than 8."); }
        Self
        {
            hash_code: [ IntUnion::new_with(Self::H[0]),
                        IntUnion::new_with(Self::H[1]),
                        IntUnion::new_with(Self::H[2]),
                        IntUnion::new_with(Self::H[3]),
                        IntUnion::new_with(Self::H[4]),
                        IntUnion::new_with(Self::H[5]),
                        IntUnion::new_with(Self::H[6]),
                        IntUnion::new_with(Self::H[7]) ]
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
    /// [digest_str()](struct@SHA2_256_Generic#method.digest_str)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of String
    /// object, you are highly recommended to use the method
    /// [digest_string()](struct@SHA2_256_Generic#method.digest_string)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    /// object, you are highly recommended to use the method
    /// [digest_array()](struct@SHA2_256_Generic#method.digest_array)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    /// object, you are highly recommended to use the method
    /// [digest_vec()](struct@SHA2_256_Generic#method.digest_array)
    /// rather than this method.
    ///
    /// # Example 1 for SHA2_256
    /// ```
    /// use cryptocol::hash::SHA2_256;
    /// let txt = "This is an example of the method digest().";
    /// let mut hash = SHA2_256::new();
    /// hash.digest(txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "F1ECFB4A9F399E3786FD87ABE5D27DB64ADB61F4754BA68EFADCB3792DD15827");
    /// ```
    ///
    /// # Example 2 for SHA2_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_256_Expanded;
    /// type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    /// let mut hash = MySHA2::new();
    /// let txt = "This is an example of the method digest().";
    /// hash.digest(txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "F7301C3222B8AA48ABDC3917F24B2E6E408601AC123C26B733E3FBDA494ACF7D");
    /// ```
    /// 
    /// # Example 3 for SHA2_224
    /// ```
    /// use cryptocol::hash::SHA2_224;
    /// let mut hash = SHA2_224::new();
    /// let txt = "This is an example of the method digest().";
    /// hash.digest(txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "279C669E4411237490589A794FC2F0F8E256F8FBC58C520601ABF45B");
    /// ```
    ///
    /// # Example 4 for SHA2_224_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_224_Expanded;
    /// type MySHA2 = SHA2_224_Expanded<128>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method digest().";
    /// my_hash.digest(txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "98256F32A77281A8CBCBA9105080A73BB55F0B51CCCBCC4A273D744E");
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
    /// [digest_string()](struct@SHA2_256_Generic#method.digest_string)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    /// object, you are highly recommended to use the method
    /// [digest_array()](struct@SHA2_256_Generic#method.digest_array)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    /// object, you are highly recommended to use the method
    /// [digest_vec()](struct@SHA2_256_Generic#method.digest_array)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [digest()](struct@SHA2_256_Generic#method.digest) rather than this method.
    ///
    /// # Example 1 for SHA2_256
    /// ```
    /// use cryptocol::hash::SHA2_256;
    /// let mut hash = SHA2_256::new();
    /// let txt = "This is an example of the method digest_str().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "B9396CF025B6ECC98178BE081D045DCC2CD3F18FE1450B1B420451A53C571D32");
    /// ```
    ///
    /// # Example 2 for SHA2_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_256_Expanded;
    /// type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method digest_str().";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "853979616624A859070DB313AAE6BFED07A58EFA37571E88276D215AE845645B");
    /// ```
    ///
    /// # Example 3 for SHA2_224
    /// ```
    /// use cryptocol::hash::SHA2_224;
    /// let mut hash = SHA2_224::new();
    /// let txt = "This is an example of the method digest_str().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "5E3731968A757FDFD99F9C9437B1BA2443A66065B362F230AA041C06");
    /// ```
    ///
    /// # Example 4 for SHA2_224_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_224_Expanded;
    /// type MySHA2 = SHA2_224_Expanded<128>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method digest_str().";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "8A6102A3AB8A7154E78D0FEBE130BA04E508AF7933AC88ED75D34BCD");
    /// ```
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
    /// [digest_str()](struct@SHA2_256_Generic#method.digest_str)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    /// object, you are highly recommended to use the method
    /// [digest_array()](struct@SHA2_256_Generic#method.digest_array)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    /// object, you are highly recommended to use the method
    /// [digest_vec()](struct@SHA2_256_Generic#method.digest_array)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [digest()](struct@SHA2_256_Generic#method.digest) rather than this method.
    ///
    /// # Example 1 for SHA2_256
    /// ```
    /// use cryptocol::hash::SHA2_256;
    /// let mut hash = SHA2_256::new();
    /// let txt = "This is an example of the method digest_string().".to_string();
    /// hash.digest_string(&txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "0DA7CA307C40C3661BD59AAF2828CAC1D3E3C82385CC8EC92A2FAFF1C0A5DF43");
    /// ```
    ///
    /// # Example 2 for SHA2_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_256_Expanded;
    /// type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method digest_string().".to_string();
    /// my_hash.digest_string(&txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "662FE0AAEF2070BE79771F3693F0A1CCA8DF6E9E08A5685535D99C77C258F3AC");
    /// ```
    ///
    /// # Example 3 for SHA2_224
    /// ```
    /// use cryptocol::hash::SHA2_224;
    /// let mut hash = SHA2_224::new();
    /// let txt = "This is an example of the method digest_string().".to_string();
    /// hash.digest_string(&txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "C56B613F567923DC558D7BD4D68A88875DD206C53BCC7329448508FA");
    /// ```
    ///
    /// # Example 4 for SHA2_224_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_224_Expanded;
    /// type MySHA2 = SHA2_224_Expanded<128>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method digest_string().".to_string();
    /// my_hash.digest_string(&txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "85229E915A413FA4F90F86A51628834A0D0490B054330E032D93430A");
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
    /// [digest_str()](struct@SHA2_256_Generic#method.digest_str)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of String
    /// object, you are highly recommended to use the method
    /// [digest_string()](struct@SHA2_256_Generic#method.digest_string)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    /// object, you are highly recommended to use the method
    /// [digest_vec()](struct@SHA2_256_Generic#method.digest_array)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [digest()](struct@SHA2_256_Generic#method.digest) rather than this method.
    ///
    /// # Example 1 for SHA2_256
    /// ```
    /// use cryptocol::hash::SHA2_256;
    /// let mut hash = SHA2_256::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.digest_array(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "411D3F1D2390FF3F482AC8DF4E730780BB081A192F283D2F373138FD101DC8FE");
    /// ```
    ///
    /// # Example 2 for SHA2_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_256_Expanded;
    /// type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    /// let mut my_hash = MySHA2::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// my_hash.digest_array(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    /// assert_eq!(my_hash.to_string(), "C5E18AD8B82014203663BD271D12D2BF5F51045D0E9A1BBDE7D7A7B10A125DA0");
    /// ```
    ///
    /// # Example 3 for SHA2_224
    /// ```
    /// use cryptocol::hash::SHA2_224;
    /// let mut hash = SHA2_224::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.digest_array(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "80BB5B27988D4C3E8FFA4429A4D01175498EC57BAE6B9E856A37837B");
    /// ```
    ///
    /// # Example 4 for SHA2_224_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_224_Expanded;
    /// type MySHA2 = SHA2_224_Expanded<128>;
    /// let mut my_hash = MySHA2::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// my_hash.digest_array(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    /// assert_eq!(my_hash.to_string(), "24E317307775C27419D4A65A57A647775216FDEB2416D1B283EB2271");
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
    /// [digest_str()](struct@SHA2_256_Generic#method.digest_str)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of String
    /// object, you are highly recommended to use the method
    /// [digest_string()](struct@SHA2_256_Generic#method.digest_string)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    /// object, you are highly recommended to use the method
    /// [digest_array()](struct@SHA2_256_Generic#method.digest_array)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [digest()](struct@SHA2_256_Generic#method.digest) rather than this method.
    ///
    /// # Example 1 for SHA2_256
    /// ```
    /// use cryptocol::hash::SHA2_256;
    /// let mut hash = SHA2_256::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.digest_vec(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "411D3F1D2390FF3F482AC8DF4E730780BB081A192F283D2F373138FD101DC8FE");
    /// ```
    ///
    /// # Example 2 for SHA2_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_256_Expanded;
    /// type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    /// let mut my_hash = MySHA2::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// my_hash.digest_vec(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    /// assert_eq!(my_hash.to_string(), "C5E18AD8B82014203663BD271D12D2BF5F51045D0E9A1BBDE7D7A7B10A125DA0");
    /// ```
    ///
    /// # Example 3 for SHA2_224
    /// ```
    /// use cryptocol::hash::SHA2_224;
    /// let mut hash = SHA2_224::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.digest_vec(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "80BB5B27988D4C3E8FFA4429A4D01175498EC57BAE6B9E856A37837B");
    /// ```
    ///
    /// # Example 4 for SHA2_224_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_224_Expanded;
    /// type MySHA2 = SHA2_224_Expanded<128>;
    /// let mut my_hash = MySHA2::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// my_hash.digest_vec(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    /// assert_eq!(my_hash.to_string(), "24E317307775C27419D4A65A57A647775216FDEB2416D1B283EB2271");
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
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of a string slice,
    /// you are highly recommended to use the method
    /// [ruminate_str()](struct@SHA2_256_Generic#method.ruminate_str)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of String
    /// object, you are highly recommended to use the method
    /// [ruminate_string()](struct@SHA2_256_Generic#method.ruminate_string)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    /// object, you are highly recommended to use the method
    /// [ruminate_array()](struct@SHA2_256_Generic#method.ruminate_array)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    /// object, you are highly recommended to use the method
    /// [ruminate_vec()](struct@SHA2_256_Generic#method.ruminate_array)
    /// rather than this method.
    ///
    /// # Example 1 for SHA2_256
    /// ```
    /// use cryptocol::hash::SHA2_256;
    /// let mut hash = SHA2_256::new();
    /// let txt = "This is an example of the method ruminate().";
    /// hash.ruminate(2, txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "5EB550DEA1A606FE03338BBEAEB7200003472FDF02556C6E32273C0405EF1443");
    /// ```
    ///
    /// # Example 2 for SHA2_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_256_Expanded;
    /// type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    /// let mut hash = MySHA2::new();
    /// let txt = "This is an example of the method ruminate().";
    /// hash.ruminate(2, txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "78782D14800491F6E66CAA238D955FE11FC9E9750161D51B83429B58AEC3EE0B");
    /// ```
    ///
    /// # Example 3 for SHA2_224
    /// ```
    /// use cryptocol::hash::SHA2_224;
    /// let mut hash = SHA2_224::new();
    /// let txt = "This is an example of the method ruminate().";
    /// hash.ruminate(2, txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "30BC4B7A0C1EE66CBE7F708AB7A8510CA94CE5F76D6442D4A8BD8051");
    /// ```
    /// 
    /// # Example 4 for SHA2_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_224_Expanded;
    /// type MySHA2 = SHA2_224_Expanded<128>;
    /// let mut hash = MySHA2::new();
    /// let txt = "This is an example of the method ruminate().";
    /// hash.ruminate(2, txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "5B377DE6E64005E60ACEB32811BA594006955623A34AF2D71D9A8A84");
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
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of the content of String
    /// object, you are highly recommended to use the method
    /// [digest_string()](struct@SHA2_256_Generic#method.digest_string)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    /// object, you are highly recommended to use the method
    /// [digest_array()](struct@SHA2_256_Generic#method.digest_array)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    /// object, you are highly recommended to use the method
    /// [digest_vec()](struct@SHA2_256_Generic#method.digest_array)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [digest()](struct@SHA2_256_Generic#method.digest) rather than this method.
    ///
    /// # Example 1 for SHA2_256
    /// ```
    /// use cryptocol::hash::SHA2_256;
    /// let mut hash = SHA2_256::new();
    /// let txt = "This is an example of the method ruminate_str().";
    /// hash.ruminate_str(3, txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "CD24A2825FA0CBAB8D70467D82E92D8BF0CBE86C6B0DAFECB87C254376A9323D");
    /// ```
    ///
    /// # Example 2 for SHA2_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_256_Expanded;
    /// type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method ruminate_str().";
    /// my_hash.ruminate_str(3, txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "BF0DAD806A1D09BA917C08C951CB97F8F51C67D0EE8DFDBFCCEE7E5D6DE288C5");
    /// ```
    ///
    /// # Example 3 for SHA2_224
    /// ```
    /// use cryptocol::hash::SHA2_224;
    /// let mut hash = SHA2_224::new();
    /// let txt = "This is an example of the method ruminate_str().";
    /// hash.ruminate_str(3, txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "4410FF448733776380ABF2A899BE858AB8767C3E7DB55F435691F22D");
    /// ```
    /// 
    /// # Example 4 for SHA2_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_224_Expanded;
    /// type MySHA2 = SHA2_224_Expanded<128>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method ruminate_str().";
    /// my_hash.ruminate_str(3, txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "7C4BADDC365A4A0E59B7A1195B4FAEB7222568EF43CA49395F3DFE24");
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
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of a string slice,
    /// you are highly recommended to use the method
    /// [digest_str()](struct@SHA2_256_Generic#method.digest_str)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    /// object, you are highly recommended to use the method
    /// [digest_array()](struct@SHA2_256_Generic#method.digest_array)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    /// object, you are highly recommended to use the method
    /// [digest_vec()](struct@SHA2_256_Generic#method.digest_array)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [digest()](struct@SHA2_256_Generic#method.digest) rather than this method.
    ///
    /// # Example 1 for SHA2_256
    /// ```
    /// use cryptocol::hash::SHA2_256;
    /// let mut hash = SHA2_256::new();
    /// let txt = "This is an example of the method ruminate_string().".to_string();
    /// hash.ruminate_string(2, &txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "B013EDF34D926ABF2BE04489593A214FFECCA1F5334E31D17BFFEA000E4E85FE");
    /// ```
    ///
    /// # Example 2 for SHA2_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_256_Expanded;
    /// type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method ruminate_string().".to_string();
    /// my_hash.ruminate_string(2, &txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "6C72B5A23ED16E4533FA3437AF27E9DE21ECA6D0C947CDCB1CC684FBABA3E720");
    /// ```
    ///
    /// # Example 3 for SHA2_224
    /// ```
    /// use cryptocol::hash::SHA2_224;
    /// let mut hash = SHA2_224::new();
    /// let txt = "This is an example of the method ruminate_string().".to_string();
    /// hash.ruminate_string(2, &txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "267EE9D909B721671130C62C3CF161EE2AEEED638F7FDC85A96EADDA");
    /// ```
    /// 
    /// # Example 4 for SHA2_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_224_Expanded;
    /// type MySHA2 = SHA2_224_Expanded<128>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method ruminate_string().".to_string();
    /// my_hash.ruminate_string(2, &txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "B05B7CB13228323D392B5DE4D54B40AD9E7D68A4683A7DC689B05489");
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
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of a string slice,
    /// you are highly recommended to use the method
    /// [digest_str()](struct@SHA2_256_Generic#method.digest_str)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of String
    /// object, you are highly recommended to use the method
    /// [digest_string()](struct@SHA2_256_Generic#method.digest_string)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    /// object, you are highly recommended to use the method
    /// [digest_vec()](struct@SHA2_256_Generic#method.digest_array)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [digest()](struct@SHA2_256_Generic#method.digest) rather than this method.
    ///
    /// # Example 1 for SHA2_256
    /// ```
    /// use cryptocol::hash::SHA2_256;
    /// let mut hash = SHA2_256::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.ruminate_array(5,&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "CD949334D0E7BBEEC7C0B3FDDED14A2FFFA85EA91DCFB171521C5C74CB989925");
    /// ```
    ///
    /// # Example 2 for SHA2_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_256_Expanded;
    /// type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    /// let mut my_hash = MySHA2::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// my_hash.ruminate_array(5,&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    /// assert_eq!(my_hash.to_string(), "5E2FEBBBED0F9EFC2AFA2C892B9AC09B6A38DA45C0D23633D3C9F58A0C547909");
    /// ```
    ///
    /// # Example 3 for SHA2_224
    /// ```
    /// use cryptocol::hash::SHA2_224;
    /// let mut hash = SHA2_224::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.ruminate_array(5,&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "2ECBE562B252532846838B27F0ABF9EDC82C7112F9705F0FA77E9EF1");
    /// ```
    /// 
    /// # Example 4 for SHA2_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_224_Expanded;
    /// type MySHA2 = SHA2_224_Expanded<128>;
    /// let mut my_hash = MySHA2::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// my_hash.ruminate_array(5,&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    /// assert_eq!(my_hash.to_string(), "0C21E7F3B3E08B5A751E16C701AC2438871084CFF37E5177E15EE168");
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
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of a string slice,
    /// you are highly recommended to use the method
    /// [digest_str()](struct@SHA2_256_Generic#method.digest_str)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of String
    /// object, you are highly recommended to use the method
    /// [digest_string()](struct@SHA2_256_Generic#method.digest_string)
    /// rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    /// object, you are highly recommended to use the method
    /// [digest_array()](struct@SHA2_256_Generic#method.digest_array)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [digest()](struct@SHA2_256_Generic#method.digest) rather than this method.
    ///
    /// # Example 1 for SHA2_256
    /// ```
    /// use cryptocol::hash::SHA2_256;
    /// let mut hash = SHA2_256::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.ruminate_vec(2, &data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "C02DA0025F7228433CB12036E35F60242B2F6C82F55DA1497ACABD491D381EDF");
    /// ```
    ///
    /// # Example 2 for SHA2_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_256_Expanded;
    /// type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    /// let mut my_hash = MySHA2::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// my_hash.ruminate_vec(2, &data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    /// assert_eq!(my_hash.to_string(), "D5E63A6B8D2E4F1912AAD8C6C99C9B063CB001A4FB1AD6B3D08A8F0C1B4CF947");
    /// ```
    ///
    /// # Example 3 for SHA2_224
    /// ```
    /// use cryptocol::hash::SHA2_224;
    /// let mut hash = SHA2_224::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.ruminate_vec(2, &data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "F04D0993A12818003DF69DC5FF2901F766A3CED0D48766E984B4745B");
    /// ```
    /// 
    /// # Example 4 for SHA2_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_224_Expanded;
    /// type MySHA2 = SHA2_224_Expanded<128>;
    /// let mut my_hash = MySHA2::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// my_hash.ruminate_vec(2, &data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    /// assert_eq!(my_hash.to_string(), "D670D49F1DBBE320A11806DAC55BA4E4B0AB2CB3EB4821E711D02E9A");
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
    /// [get_hash_value_string()](struct@SHA2_256_Generic#method.get_hash_value_string)
    /// rather than this method.
    /// - If you want to get the hash value in the form of array object,
    /// you are highly recommended to use the method
    /// [get_hash_value_in_array()](struct@SHA2_256_Generic#method.get_hash_value_in_array)
    /// rather than this method.
    /// - If you want to get the hash value in the form of Vec object,
    /// you are highly recommended to use the method
    /// [get_hash_value_in_vec()](struct@SHA2_256_Generic#method.get_hash_value_in_vec)
    /// rather than this method.
    ///
    /// # Example 1 for SHA2_256
    /// ```
    /// use cryptocol::hash::SHA2_256;
    /// let mut hash = SHA2_256::new();
    /// let txt = "This is an example of the method get_hash_value().";
    /// let hash_value = [0_u8; 32];
    /// hash.digest_str(txt);
    /// hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    /// assert_eq!(format!("{:02X?}", hash_value), "[80, 57, B7, FD, 85, 6A, B5, AD, A5, A8, 25, 83, B2, 29, 91, 7A, 9D, 55, E6, 6C, 06, 4F, 9A, F3, 2A, 0F, BF, BF, D7, E5, CA, 20]");
    /// ```
    ///
    /// # Example 2 for SHA2_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_256_Expanded;
    /// type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method get_hash_value().";
    /// let hash_value = [0_u8; 32];
    /// my_hash.digest_str(txt);
    /// my_hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    /// assert_eq!(format!("{:02X?}", hash_value), "[F7, 71, 58, A3, 6F, 47, 5D, BB, 1C, CB, 40, 5B, C7, DB, 13, 17, 37, 13, CA, 6D, 88, E7, 76, 8D, 71, 4B, CD, 1C, CE, 53, 36, DD]");
    /// ```
    ///
    /// # Example 3 for SHA2_224
    /// ```
    /// use cryptocol::hash::SHA2_224;
    /// let mut hash = SHA2_224::new();
    /// let txt = "This is an example of the method get_hash_value().";
    /// let hash_value = [0_u8; 28];
    /// hash.digest_str(txt);
    /// hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    /// assert_eq!(format!("{:02X?}", hash_value), "[69, DC, CE, 52, 29, 1E, 97, 06, 1A, 6D, 57, 62, 3E, FE, FA, 7C, CF, 74, 35, 30, 40, 04, 5C, 33, 8E, 09, 28, 8C]");
    /// ```
    ///
    /// # Example 4 for SHA2_224_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_224_Expanded;
    /// type MySHA2 = SHA2_224_Expanded<128>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method get_hash_value().";
    /// let hash_value = [0_u8; 28];
    /// my_hash.digest_str(txt);
    /// my_hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    /// assert_eq!(format!("{:02X?}", hash_value), "[09, 89, 84, 69, 9F, C8, DE, 3B, 56, 21, 69, A4, A3, 06, AD, B4, 0F, 7B, A0, 36, 95, 3E, 98, C4, 7A, 5D, 30, 37]");
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
    /// A hash value in the form of String object.
    /// 
    /// # Counterpart Methods
    /// - If you want to get the hash value in the form of array object,
    /// you are highly recommended to use the method
    /// [get_hash_value_in_array()](struct@SHA2_256_Generic#method.get_hash_value_in_array)
    /// rather than this method.
    /// - If you want to get the hash value in the form of Vec object,
    /// you are highly recommended to use the method
    /// [get_hash_value_in_vec()](struct@SHA2_256_Generic#method.get_hash_value_in_vec)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [get_hash_value()](struct@SHA2_256_Generic#method.get_hash_value)
    /// rather than this method.
    ///
    /// # Example 1 for SHA2_256
    /// ```
    /// use cryptocol::hash::SHA2_256;
    /// let mut hash = SHA2_256::new();
    /// let txt = "This is an example of the method get_hash_value_in_string().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_hash_value_in_string());
    /// assert_eq!(hash.get_hash_value_in_string(), "F35FDE269F52ED6D31D38217988B08CB58DC4A5F600ED51AB705DBBD6FBDC4B0");
    /// ```
    ///
    /// # Example 2 for SHA2_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_256_Expanded;
    /// type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method get_hash_value_in_string().";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.get_hash_value_in_string());
    /// assert_eq!(my_hash.get_hash_value_in_string(), "34ECF36DCBB56DC00FBC4E0AFA5A3D6AB3858AEE810017784D61717F6A4E2FC1");
    /// ```
    ///
    /// # Example 3 for SHA2_224
    /// ```
    /// use cryptocol::hash::SHA2_224;
    /// let mut hash = SHA2_224::new();
    /// let txt = "This is an example of the method get_hash_value_in_string().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_hash_value_in_string());
    /// assert_eq!(hash.get_hash_value_in_string(), "A35C7F70866C2D4224B4F66CDF11932955BA312CE04322EF83A680A1");
    /// ```
    ///
    /// # Example 4 for SHA2_224_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_224_Expanded;
    /// type MySHA2 = SHA2_224_Expanded<128>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method get_hash_value_in_string().";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.get_hash_value_in_string());
    /// assert_eq!(my_hash.get_hash_value_in_string(), "7E9B316E6510BA22C8290C8C14C74C90C540097B1DA9A8D840D2EDCC");
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
    /// A hash value in the form of array object [u32; N].
    /// 
    /// # Panics
    /// If N > 8, this method will panic
    /// or its behaviour is undefined even if it won't panic.
    /// 
    /// # Counterpart Methods
    /// - If you want to get the hash value in the form of String object,
    /// you are highly recommended to use the method
    /// [get_hash_value_string()](struct@SHA2_256_Generic#method.get_hash_value_string)
    /// rather than this method.
    /// - If you want to get the hash value in the form of Vec object,
    /// you are highly recommended to use the method
    /// [get_hash_value_in_vec()](struct@SHA2_256_Generic#method.get_hash_value_in_vec)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [get_hash_value()](struct@SHA2_256_Generic#method.get_hash_value)
    /// rather than this method.
    ///
    /// # Example 1 for SHA2_256
    /// ```
    /// use cryptocol::hash::SHA2_256;
    /// let mut hash = SHA2_256::new();
    /// let txt = "This is an example of the method get_hash_value_in_array().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:02X?}", hash.get_hash_value_in_array()), "[3CC694F3, 6ACEFBBA, 6DFC351C, 22F6CCDF, CF8261F8, 52616CBF, B3E7378A, 10A3CFCC]");
    /// ```
    ///
    /// # Example 2 for SHA2_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_256_Expanded;
    /// type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method get_hash_value_in_array().";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, my_hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:02X?}", my_hash.get_hash_value_in_array()), "[5E76B301, 821107BB, 9B4BEEBB, AAE226B7, 7D038946, BE9025FD, FB26F6AF, 6EB1C43E]");
    /// ```
    ///
    /// # Example 3 for SHA2_224
    /// ```
    /// use cryptocol::hash::SHA2_224;
    /// let mut hash = SHA2_224::new();
    /// let txt = "This is an example of the method get_hash_value_in_array().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[95B6C324, ADFEA823, F398E670, 7B44C57B, B7646166, 46A4BD98, 73B076E2]");
    /// ```
    ///
    /// # Example 4 for SHA2_224_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_224_Expanded;
    /// type MySHA2 = SHA2_224_Expanded<128>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method get_hash_value_in_array().";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[FBC6DEA8, 0D40B971, D842AF35, 5CD5ABC9, 309E88E5, 7592A190, 19968AA6]");
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

    // pub fn gethash_value_in_vec(&self) -> Vec
    /// Returns a hash value in the form of Vec object.
    /// 
    /// # Output
    /// A hash value in the form of Vec object.
    /// 
    /// # Counterpart Methods
    /// - If you want to get the hash value in the form of String object,
    /// you are highly recommended to use the method
    /// [get_hash_value_string()](struct@SHA2_256_Generic#method.get_hash_value_string)
    /// rather than this method.
    /// - If you want to get the hash value in the form of array object,
    /// you are highly recommended to use the method
    /// [get_hash_value_in_array()](struct@SHA2_256_Generic#method.get_hash_value_in_array)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [get_hash_value()](struct@SHA2_256_Generic#method.get_hash_value)
    /// rather than this method.
    ///
    /// # Example 1 for SHA2_256
    /// ```
    /// use cryptocol::hash::SHA2_256;
    /// let mut hash = SHA2_256::new();
    /// let txt = "This is an example of the method get_hash_value_in_vec().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_hash_value_in_vec());
    /// assert_eq!(format!("{:02X?}", hash.get_hash_value_in_vec()), "[354B3EE4, 6B5A94AC, 197E0B7B, 38E0489C, 66CC8794, 227B95B, A045CEF0, 8565D27C]");
    /// ```
    ///
    /// # Example 2 for SHA2_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_256_Expanded;
    /// type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x2222_2222, 0x4444_4444, 0x6666_6666, 0x8888_8888, 0xaaaa_aaaa, 0xcccc_cccc, 0xeeee_eeee, 128>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method get_hash_value_in_vec().";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, my_hash.get_hash_value_in_vec());
    /// assert_eq!(format!("{:02X?}", my_hash.get_hash_value_in_vec()), "[992EB41E, 6A60E9AD, B4727373, F724402F, C8C9AC0F, 8C8BF905, 56EAF929, 1817F41C]");
    /// ```
    ///
    /// # Example 3 for SHA2_224
    /// ```
    /// use cryptocol::hash::SHA2_224;
    /// let mut hash = SHA2_224::new();
    /// let txt = "This is an example of the method get_hash_value_in_vec().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_hash_value_in_vec());
    /// assert_eq!(format!("{:08X?}", hash.get_hash_value_in_vec()), "[2D3D3B76, 53CD9A03, 439CFF14, A565148A, 10479BDB, 09CAADF9, 1DD5ABAA]");
    /// ```
    ///
    /// # Example 4 for SHA2_224_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_224_Expanded;
    /// type MySHA2 = SHA2_224_Expanded<128>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method get_hash_value_in_vec().";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_hash_value_in_vec());
    /// assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_vec()), "[C9CEC151, 8A70C4C8, AB43C3BC, 3BF60C45, 00D4D6A0, 2AE25C0D, 4C212514]");
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
    /// # Example 1 for SHA2_256
    /// ```
    /// use cryptocol::hash::SHA2_256;
    /// let mut hash = SHA2_256::new();
    /// let txt = "This is an example of the method put_hash_value_in_array().";
    /// let mut hash_code = [0_u32; 5];
    /// hash.digest_str(txt);
    /// hash.put_hash_value_in_array(&mut hash_code);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    /// assert_eq!(format!("{:08X?}", hash_code), "[1F057A27, D3162A35, 63E11F9D, 7362549D, CB42D322]");
    /// ```
    ///
    /// # Example 2 for SHA2_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_256_Expanded;
    /// type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method put_hash_value_in_array().";
    /// let mut hash_code = [0_u32; 5];
    /// my_hash.digest_str(txt);
    /// my_hash.put_hash_value_in_array(&mut hash_code);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    /// assert_eq!(format!("{:08X?}", hash_code), "[98223576, 8B1EC66B, EF9BCF28, 6A9A89C8, BB42953F]");
    /// ```
    ///
    /// # Example 3 for SHA2_224
    /// ```
    /// use cryptocol::hash::SHA2_224;
    /// let mut hash = SHA2_224::new();
    /// let txt = "This is an example of the method put_hash_value_in_array().";
    /// let mut hash_code = [0_u32; 5];
    /// hash.digest_str(txt);
    /// hash.put_hash_value_in_array(&mut hash_code);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    /// assert_eq!(format!("{:08X?}", hash_code), "[9D43618A, DE2A6358, 53FCAD2D, 6757E0C7, 684E65FB]");
    /// ```
    ///
    /// # Example 4 for SHA2_224_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_224_Expanded;
    /// type MySHA2 = SHA2_224_Expanded<128>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "This is an example of the method put_hash_value_in_array().";
    /// let mut hash_code = [0_u32; 5];
    /// my_hash.digest_str(txt);
    /// my_hash.put_hash_value_in_array(&mut hash_code);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    /// assert_eq!(format!("{:08X?}", hash_code), "[FF72EB17, FBED9CD1, 0BD59F6E, DABDBA49, EA3BDF96]");
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
        let length = if out_size < (u32::size_in_bytes() * N) {out_size} else {u32::size_in_bytes() * N};
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
    /// # Example 1 for SHA2_256
    /// ```
    /// use cryptocol::hash::SHA2_256;
    /// let mut hash = SHA2_256::new();
    /// let txt = "TANGLING";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[D62ABC88, A57B6A04, B82A9922, C0316859, 3D8DDE86, E0D8783C, 07E34E29, 3F65D373]");
    /// hash.tangle(1);
    /// println!("Hash =\t{:08X?}", hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[7FC9659F, 17ACCDB7, 43AA0A92, 160137F1, A2A172A6, 1B42868B, 981CA8B2, 98929E8B]");
    /// hash.tangle(1);
    /// println!("Hash =\t{:08X?}", hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[7D60FABC, 351D79A0, DF04ADC9, A03CE8FB, A7154541, 5DB0A405, CDEE8242, 7D509560]");
    /// ```
    ///
    /// # Example 2 for SHA2_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_256_Expanded;
    /// type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "TANGLING";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[12FFD34D, 9A5B4843, 1D8DBA65, 3C578886, B85EB6B2, 291D1A45, FD72ECFC, AC8D8577]");
    /// my_hash.tangle(1);
    /// println!("Hash =\t{:08X?}", my_hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[FF0C7268, DA3463BD, 6601EC3B, 5D48D7BF, 10C4460B, F11B209E, CBCB2BCE, 08DE13FC]");
    /// my_hash.tangle(1);
    /// println!("Hash =\t{:08X?}", my_hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[FA6EE74C, F322A5D0, E4EFB28A, 6E30F7FB, 5723E91A, F7B0B0CB, 256610EC, 3E6A6A2B]");
    /// ```
    ///
    /// # Example 3 for SHA2_224
    /// ```
    /// use cryptocol::hash::SHA2_224;
    /// let mut hash = SHA2_224::new();
    /// let txt = "TANGLING";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[DFBDC998, 897BDD0A, F99B538F, 178A5EE5, 16C96398, 2D544CAF, DC631DE9]");
    /// hash.tangle(1);
    /// println!("Hash =\t{:08X?}", hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[32CB00E5, 9A09585A, 9051D8FB, F8F6EB0D, FD467652, 46408C7F, F5DD61C8]");
    /// hash.tangle(1);
    /// println!("Hash =\t{:08X?}", hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[7F5D4897, F323EC3E, D47C95D5, 9D77DF01, 9269E780, 3973310E, 142EB013]");
    /// ```
    ///
    /// # Example 4 for SHA2_224_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_224_Expanded;
    /// type MySHA2 = SHA2_224_Expanded<128>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "TANGLING";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[9196ACFD, 94E19450, C9B7D8D3, 220C86A4, 6AC1EE8F, C87D73B4, ECFEE637]");
    /// my_hash.tangle(1);
    /// println!("Hash =\t{:08X?}", my_hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[4D71E0F6, 41D78805, 94358C2C, FAC2356D, AEB666BB, 80880239, B2D1304D]");
    /// my_hash.tangle(1);
    /// println!("Hash =\t{:08X?}", my_hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[851BE861, 5E595131, 072DF35A, 973B5D59, 87DBDC1D, 68BF05A6, 48EAC080]");
    /// ```
    #[inline]
    pub fn tangle(&mut self, tangling: u64)
    {
        let common = LongUnion::new_with(tangling);
        let mut m = [0_u32; 10];
        for i in 0..8
            { m[i] = self.hash_code[i].get(); }
        m[8] = common.get_uint_(0);
        m[9] = common.get_uint_(1);
        self.finalize(m.as_ptr() as *const u8, 40);
    }

    // fn initialize(&mut self)
    /// Initializes all five self.hash_code[] with predetermined values H[].
    fn initialize(&mut self)
    {
        for i in 0..8_usize
            { self.hash_code[i] = IntUnion::new_with(Self::get_h(i)); }
    }

    // fn update(&mut self, message: &[u32])
    /// This method is the core part of MD5 hash algorithm.
    /// It has sixty-four rounds. It updates self.hash_code[] for those
    /// sixty-four rounds.
    fn update(&mut self, message: &[u32])
    {
        let mut w = [0_u32; 16];
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
            let s1 = e.rotate_right(RR6) ^ e.rotate_right(RR11) ^ e.rotate_right(RR25);
            let t1 = Self::ch(e, f, g).wrapping_add(h)
                                .wrapping_add(Self::get_k(i))
                                .wrapping_add(w[i])
                                .wrapping_add(s1);
            let s0 = a.rotate_right(RR2) ^ a.rotate_right(RR13) ^ a.rotate_right(RR22);
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
            let s1 = e.rotate_right(RR6) ^ e.rotate_right(RR11) ^ e.rotate_right(RR25);
            let t1 = Self::ch(e, f, g).wrapping_add(h)
                                .wrapping_add(Self::get_k(i))
                                .wrapping_add(w[j])
                                .wrapping_add(s1);
            let s0 = a.rotate_right(RR2) ^ a.rotate_right(RR13) ^ a.rotate_right(RR22);
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

        let mut mu = MU { txt: [0; MESSAGE_NUM] };
        let last_bytes = (length_in_bytes & LAST_BYTES) as usize;    // equivalent to (length_in_bytes % 64) as usize
        unsafe { copy_nonoverlapping(message, mu.txt.as_mut_ptr(), last_bytes); }
        unsafe { mu.txt[last_bytes] = 0b1000_0000; }
        // 데이터 기록후, 데이터의 길이를 비트 단위로 기록하기 위한 64 비트(8 바이트)와
        // 0b1000_0000를 기록하기 위한 한 바이트의 여유공간이 남아있지 않으면,
        if last_bytes > 54  // (>= 64 - 8 - 1)
        {
            self.update(unsafe {&mu.piece});
            for i in 0..7
                { unsafe { mu.chunk[i] = 0; } }
        }
        unsafe { mu.chunk[7] = (length_in_bytes << 3).to_be(); }    // 데이터 길이의 단위는 바이트가 아니라 비트이다.
        self.update(unsafe {&mu.piece});
    }

	#[inline] fn get_k(idx: usize) -> u32    { Self::K[idx & 0b11_1111] }
	#[inline] fn get_h(idx: usize) -> u32    { Self::H[idx] }
    #[inline] fn get_s0(w: &[u32; 16], idx: usize) -> u32  { let ww = w[(idx-15) & 0b1111]; ww.rotate_right(RR7) ^ ww.rotate_right(RR18) ^ (ww >> SR3) }
    #[inline] fn get_s1(w: &[u32; 16], idx: usize) -> u32  { let ww = w[(idx-2) & 0b1111]; ww.rotate_right(RR17) ^ ww.rotate_right(RR19) ^ (ww >> SR10) }
    #[inline] fn get_w(w: &[u32; 16], idx: usize) -> u32   { w[(idx-16) & 0b1111].wrapping_add(Self::get_s0(w, idx)).wrapping_add(w[(idx-7) & 0b1111]).wrapping_add(Self::get_s1(&w, idx)) }
	#[inline] fn ch(x: u32, y: u32, z: u32) -> u32  { z ^ (x & (y ^ z)) }   // equivalent to { (x & y) | (!x & z) }
	#[inline] fn maj(x: u32, y: u32, z: u32) -> u32  { (x & y) | (z & (x | y)) } // equivalent to { (x & y) | (y & z) | (z & x) }
    #[inline] fn to_char(nibble: u8) -> char    { if nibble < 10  { ('0' as u8 + nibble) as u8 as char } else { ('A' as u8 - 10 + nibble) as char } }
}


impl<const N: usize, const H0: u32, const H1: u32, const H2: u32, const H3: u32,
    const H4: u32, const H5: u32, const H6: u32, const H7: u32, const ROUND: usize,
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
    const RR2: u32, const RR6: u32, const RR7: u32, const RR11: u32, 
    const RR13: u32, const RR17: u32, const RR18: u32, const RR19: u32, 
    const RR22: u32, const RR25: u32, const SR3: i32, const SR10: i32>
Display for SHA2_256_Generic<N, H0, H1, H2, H3, H4, H5, H6, H7, ROUND,
                            K00, K01, K02, K03, K04, K05, K06, K07,
                            K08, K09, K10, K11, K12, K13, K14, K15,
                            K16, K17, K18, K19, K20, K21, K22, K23,
                            K24, K25, K26, K27, K28, K29, K30, K31,
                            K32, K33, K34, K35, K36, K37, K38, K39,
                            K40, K41, K42, K43, K44, K45, K46, K47,
                            K48, K49, K50, K51, K52, K53, K54, K55,
                            K56, K57, K58, K59, K60, K61, K62, K63,
                            RR2, RR6, RR7, RR11, RR13, RR17, RR18, RR19, 
                            RR22, RR25, SR3, SR10>
{
    /// Formats the value using the given formatter.
    /// You will hardly use this method directly.
    /// Automagically the function `to_string()` will be implemented. So, you
    /// can use the function `to_string()`, and you can also print the SHA-1
    /// object in the macro `println!()` directly for example.
    /// `f` is a buffer, this method must write the formatted string into it.
    /// [Read more](https://doc.rust-lang.org/core/fmt/trait.Display.html#tymethod.fmt)
    ///
    /// # Example 1 for SHA2_256 for to_string()
    /// ```
    /// use cryptocol::hash::SHA2_256;
    /// let mut hash = SHA2_256::new();
    /// let txt = "Display::fmt() automagically implement to_string().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
    /// assert_eq!(hash.to_string(), "46577469D8A5CBCA1AC874A5350E4FACD318A468046816B066117D51DB9D1640");
    /// ```
    ///
    /// # Example 2 for SHA2_256_Expanded for to_string()
    /// ```
    /// use cryptocol::hash::SHA2_256_Expanded;
    /// type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "Display::fmt() automagically implement to_string().";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.to_string());
    /// assert_eq!(my_hash.to_string(), "6DED905D80768EE8F19D76233902E6CA1417B23A89845C2DA9127FEDD7CCDB5C");
    /// ```
    ///
    /// # Example 3 for SHA2_224 for to_string()
    /// ```
    /// use cryptocol::hash::SHA2_224;
    /// let mut hash = SHA2_224::new();
    /// let txt = "Display::fmt() automagically implement to_string().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
    /// assert_eq!(hash.to_string(), "979DB3C5F63C2FBB32A72804A991534EB38884EB5B9131AE0EE3A496");
    /// ```
    ///
    /// # Example 4 for SHA2_224_Expanded for to_string()
    /// ```
    /// use cryptocol::hash::SHA2_224_Expanded;
    /// type MySHA2 = SHA2_224_Expanded<128>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "Display::fmt() automagically implement to_string().";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.to_string());
    /// assert_eq!(my_hash.to_string(), "136C899347821BCC7529F3B42C0A9E3E997E156B1E5E081F57BBB15E");
    /// ```
    ///
    /// # Example 5 for SHA2_256 for the macro println!()
    /// ```
    /// use cryptocol::hash::SHA2_256;
    /// let mut hash = SHA2_256::new();
    /// let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "B8338443431AB13309330A8064AF039E39F90CAC334CF8EA1FF0640646AB121C");
    /// ```
    ///
    /// # Example 6 for SHA2_256_Expanded for the macro println!()
    /// ```
    /// use cryptocol::hash::SHA2_256_Expanded;
    /// type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "EF27B2954B124469ACD614F1DE4E99B30C418194B614EE19361674F64F60189C");
    /// ```
    ///
    /// # Example 7 for SHA2_224 for the macro println!()
    /// ```
    /// use cryptocol::hash::SHA2_224;
    /// let mut hash = SHA2_224::new();
    /// let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "E333EE19A56FCDCCB05957F2B6FB0AD1EA11D7B6258DF28DCE3B526B");
    /// ```
    ///
    /// # Example 8 for SHA2_224_Expanded for the macro println!()
    /// ```
    /// use cryptocol::hash::SHA2_224_Expanded;
    /// type MySHA2 = SHA2_224_Expanded<128>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "849F654BAFF41D3025DE982EC410F8EC6991FFD6E5BF4047F45082F6");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)
        write!(f, "{}", self.get_hash_value_in_string())
    }
}
