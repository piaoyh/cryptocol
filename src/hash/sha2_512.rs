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

use crate::number::{ SmallUInt, LongUnion, LongerUnion };


/// You have freedom of changing H0 ~ H7, and ROUND.
#[allow(non_camel_case_types)]
pub type SHA2_512_Expanded<const H0: u64 = 0x6a09e667f3bcc908,
                const H1: u64 = 0xbb67ae8584caa73b, const H2: u64 = 0x3c6ef372fe94f82b,
                const H3: u64 = 0xa54ff53a5f1d36f1, const H4: u64 = 0x510e527fade682d1,
                const H5: u64 = 0x9b05688c2b3e6c1f, const H6: u64 = 0x1f83d9abfb41bd6b,
                const H7: u64 = 0x5be0cd19137e2179, const ROUND: usize = 80>
        = SHA2_512_Generic<8, H0, H1, H2, H3, H4, H5, H6, H7, ROUND>;

/// You have freedom of changing ROUND.
#[allow(non_camel_case_types)]
pub type SHA2_384_Expanded<const ROUND: usize = 80>
        = SHA2_512_Generic<6, 0xcbbb9d5dc1059ed8, 0x629a292a367cd507,
                            0x9159015a3070dd17, 0x152fecd8f70e5939,
                            0x67332667ffc00b31, 0x8eb44a8768581511,
                            0xdb0c2e0d64f98fa7, 0x47b5481dbefa4fa4, ROUND>;

/// You have freedom of changing ROUND.
#[allow(non_camel_case_types)]
pub type SHA2_512_256_Expanded<const ROUND: usize = 80>
        = SHA2_512_Generic<4, 0x22312194FC2BF72C, 0x9F555FA3C84C64C2,
                            0x2393B86B6F53B151, 0x963877195940EABD,
                            0x96283EE2A88EFFE3, 0xBE5E1E2553863992,
                            0x2B0199FC2C85B8AA, 0x0EB72DDC81C52CA2, ROUND>;

// /// You have freedom of changing ROUND.
// #[allow(non_camel_case_types)]
// pub type SHA2_512_224_Expanded<const ROUND: usize = 80>
//         = SHA2_512_Expanded<0x8C3D37C819544DA2, 0x73E1996689DCD4D6,
//                             0x1DFAB7AE32FF9C82, 0x679DD514582F9FCF,
//                             0x0F6D2B697BD44DA8, 0x77E36F7304C48942,
//                             0x3F9D85A86A1D36C8, 0x1112E6AD91D692A1, ROUND>;

/// You have freedom of changing ROUND, and K00 ~ K79.
#[allow(non_camel_case_types)]
pub type SHA2_512_Generic_HRS_fixed<const ROUND: usize,
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
                    const K76: u64, const K77: u64, const K78: u64, const K79: u64>
    = SHA2_512_Generic<8, 0x6a09e667f3bcc908, 0xbb67ae8584caa73b, 0x3c6ef372fe94f82b,
                    0xa54ff53a5f1d36f1, 0x510e527fade682d1, 0x9b05688c2b3e6c1f,
                    0x1f83d9abfb41bd6b, 0x5be0cd19137e2179, ROUND,
                    K00, K01, K02, K03, K04, K05, K06, K07,
                    K08, K09, K10, K11, K12, K13, K14, K15,
                    K16, K17, K18, K19, K20, K21, K22, K23,
                    K24, K25, K26, K27, K28, K29, K30, K31,
                    K32, K33, K34, K35, K36, K37, K38, K39,
                    K40, K41, K42, K43, K44, K45, K46, K47,
                    K48, K49, K50, K51, K52, K53, K54, K55,
                    K56, K57, K58, K59, K60, K61, K62, K63,
                    K64, K65, K66, K67, K68, K69, K70, K71,
                    K72, K73, K74, K75, K76, K77, K78, K79>;

/// You have freedom of changing ROUND, and K00 ~ K79.
#[allow(non_camel_case_types)]
pub type SHA2_384_Generic_HRS_fixed<const ROUND: usize,
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
                    const K76: u64, const K77: u64, const K78: u64, const K79: u64>
    = SHA2_512_Generic<6, 0xcbbb9d5dc1059ed8, 0x629a292a367cd507, 0x9159015a3070dd17,
                    0x152fecd8f70e5939, 0x67332667ffc00b31, 0x8eb44a8768581511,
                    0xdb0c2e0d64f98fa7, 0x47b5481dbefa4fa4, ROUND,
                    K00, K01, K02, K03, K04, K05, K06, K07,
                    K08, K09, K10, K11, K12, K13, K14, K15,
                    K16, K17, K18, K19, K20, K21, K22, K23,
                    K24, K25, K26, K27, K28, K29, K30, K31,
                    K32, K33, K34, K35, K36, K37, K38, K39,
                    K40, K41, K42, K43, K44, K45, K46, K47,
                    K48, K49, K50, K51, K52, K53, K54, K55,
                    K56, K57, K58, K59, K60, K61, K62, K63,
                    K64, K65, K66, K67, K68, K69, K70, K71,
                    K72, K73, K74, K75, K76, K77, K78, K79>;
/// The official SHA-512 hash algorithm
#[allow(non_camel_case_types)]
pub type SHA2_512 = SHA2_512_Expanded;

/// The official SHA-384 hash algorithm
#[allow(non_camel_case_types)]
pub type SHA2_384 = SHA2_384_Expanded;

/// The official SHA-512/256 hash algorithm
#[allow(non_camel_case_types)]
pub type SHA2_512_256 = SHA2_512_256_Expanded;

// /// The official SHA-512/224 hash algorithm
// #[allow(non_camel_case_types)]
// pub type SHA2_512_224 = SHA2_512_224_Expanded;


/// SHA-2-512, SHA-2-512-384, SHA-2-512-256, and SHA2_512-224 message-digest
/// algorithms that lossily compress data of arbitrary length into a 512-bit
/// hash value, 384-bit hash values, 256-bit hash values, and 224-bit hash
/// values, respectively, and its flexible variants that allow you to develop
/// your own `SHA-2-512`-based hash algorithms
/// 
/// # Introduction
/// SHA-2-512, SHA-2-512-384, SHA-2-512-256, and SHA2-512-224 were designed by
/// the United States National Security Agency, and are a U.S. Federal
/// Information Processing Standard. SHA-2-512, SHA-2-512-384, HA-2-512-256, and
/// SHA2-512-224 produce a message digest based on principles similar to those
/// used by Ronald L. Rivest of MIT in the design of the MD2, MD4, MD5, SHA0,
/// SHA-1, SHA-2-256, and SHA-2-224 message digest algorithms, but generates
/// a larger hash value (512, 384, 256, and 224 bits vs. 256, 224, 160, and
/// 128 bits). SHA-2-512, SHA-2-512-384, SHA-2-512-256, and SHA2-512-224 use the
/// [Merkle–Damgård construction](https://en.wikipedia.org/wiki/Merkle%E2%80%93Damg%C3%A5rd_construction).
/// 
/// # Vulnerability
/// There have been several attacks against SHA-2-512 and SHA-2-512-384
/// but they are all incomplete attacks.
/// Read [more](https://en.wikipedia.org/wiki/SHA-2#Cryptanalysis_and_validation)
/// 
/// # Use of SHA-2-512 and SHA-2-512-384, and their variations
/// You can use SHA-2-512 and SHA-2-512-384 for cryptographic purposes such as:
/// - Generating IDs
/// - Integrity test
/// - Storing passwords
/// - Digital Signature
/// - Implementing proof of work for block chain.
/// - Study of hash algorithms
/// 
/// # Generic Parameters
/// You can create your own expanded version of SHA-2-512 and SHA-2-512-384 by
/// changing the generic parameters N, H0 ~ H7, ROUND, K00 ~ K79, RR1, RR8,
/// RR14, RR18, RR19, RR28, RR34, RR39, RR41, RR61, SR6, and SR7.
/// - N : the size of output. N cannot be 0 or greater than 8.
/// - H0 ~ H7 : the initial hash values, eight u32 values.
/// The default values of H0 ~ H7 for SHA-2-512 are defined to be first 64 bits
/// of the fractional parts of the square roots of the first 8 primes 2..19.
/// So, H0 ~ H7 are 0x6a09e667f3bcc908, 0xbb67ae8584caa73b, 0x3c6ef372fe94f82b,
/// 0xa54ff53a5f1d36f1, 0x510e527fade682d1, 0x9b05688c2b3e6c1f,
/// 0x1f83d9abfb41bd6b, and 0x5be0cd19137e2179, respectively (in big endian
/// representation).
/// The values of H0 ~ H7 for SHA-2-512-384 are defined to be first 64 bits
/// of the fractional parts of the square roots of the the 9th through 16th
/// primes 23..53.
/// So, H0 ~ H7 for SHA-2-512-384 should be changed to be 0xcbbb9d5dc1059ed8,
/// 0x629a292a367cd507, 0x9159015a3070dd17, 0x152fecd8f70e5939,
/// 0x67332667ffc00b31, 0x8eb44a8768581511, 0xdb0c2e0d64f98fa7, and
/// 0x47b5481dbefa4fa4, respectively (in big endian representation).
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
/// Your own algrorithm based on SHA-2-512 or SHA-2-384 may be stronger or
/// weaker than official SHA-2-512 or SHA-2-384. Unless you seriously checked
/// the cryptographic security of your own algorithms, it is hard to assume
/// that your own alogrithms are stronger than the official SHA-2-512 or
/// SHA-2-384.
/// 
/// Read [this](https://doi.org/10.6028/NIST.FIPS.180-4)
/// and [that](https://en.wikipedia.org/wiki/SHA-2)
/// 
/// # Quick Start
/// In order to use the module sha2_512, you don't have to import (or use)
/// Cryptocol::hash::sha2_512::* directly because the module
/// Cryptocol::hash::sha2_512 is re-exported. All you have to do is only
/// import SHA2_512, SHA2_384, SHA2_512_256, SHA2_512_Expanded,
/// SHA2_384_Expanded, SHA2_512_256_Expanded, SHA2_512_Generic_HRS_fixed,
/// SHA2_384_Generic_HRS_fixed, and/or SHA2_512_Generic in the module
/// Cryptocol::hash. Example 1 shows how to import structs SHA2_512, SHA2_384,
/// SHA2_512_256, SHA2_512_Expanded, SHA2_384_Expanded, SHA2_512_256_Expanded,
/// SHA2_512_Generic_HRS_fixed, SHA2_384_Generic_HRS_fixed, and/or
/// SHA2_512_Generic. Plus, what you have to know is these. All the types
/// (or structs) are the specific versions of SHA2_512_Generic. Actually,
/// SHA2_512 is a specific version of SHA2_512_Expanded. SHA2_384 is a specific
/// version of SHA2_384_Expanded. SHA2_512_256 is a specific version of
/// SHA2_512_256_Expanded. SHA2_512_Expanded, SHA2_384_Expanded, and
/// SHA2_512_256_Expanded, SHA2_512_Generic_HRS_fixed, and
/// SHA2_384_Generic_HRS_fixed are specific versions of SHA2_512_Generic.
/// 
/// ## Example 1
/// ```
/// use Cryptocol::hash::SHA2_512;
/// use Cryptocol::hash::SHA2_384;
/// use Cryptocol::hash::SHA2_512_256;
/// use Cryptocol::hash::SHA2_512_Expanded;
/// use Cryptocol::hash::SHA2_384_Expanded;
/// use Cryptocol::hash::SHA2_512_256_Expanded;
/// use Cryptocol::hash::SHA2_512_Generic_HRS_fixed;
/// use Cryptocol::hash::SHA2_384_Generic_HRS_fixed;
/// use Cryptocol::hash::SHA2_512_Generic;
/// ```
/// Then, you can create SHA1 object by the method SHA1::new() for example.
/// Now, you are ready to use all prepared methods to hash any data. If you
/// want to hash a string, for example, you can use the method digest_str().
/// Then, the SHA1 object that you created will contain its hash value. You can
/// use the macro println!() for instance to print on a commandline screen by
/// `println!("{}", hash)` where hash is the SHA1 object.
/// Example 2 shows how to use SHA1 struct quickly.
/// 
/// ## Example 2 for SHA2_512
/// ```
/// use std::string::*;
/// use Cryptocol::hash::SHA2_512;
/// let mut hash = SHA2_512::new();
/// 
/// let mut txt = "";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.get_HashValue_in_string());
/// assert_eq!(hash.get_HashValue_in_string(), "CF83E1357EEFB8BDF1542850D66D8007D620E4050B5715DC83F4A921D36CE9CE47D0D13C5D85F2B0FF8318D2877EEC2F63B931BD47417A81A538327AF927DA3E");
/// 
/// let txtStirng = String::from("A");
/// hash.digest_string(&txtStirng);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txtStirng, hash);
/// assert_eq!(hash.to_string(), "21B4F4BD9E64ED355C3EB676A28EBEDAF6D8F17BDC365995B319097153044080516BD083BFCCE66121A3072646994C8430CC382B8DC543E84880183BF856CFF5");
/// 
/// let txtArray = ['W' as u8, 'o' as u8, 'w' as u8];
/// hash.digest_array(&txtArray);
/// println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txtArray, hash);
/// assert_eq!(hash.get_HashValue_in_string(), "1F7B98E0B65D4CD1DE4C2B9EC77CB5F7FC4C20006BDD1380F7E2A9C95BD5F47162B868E724FEC68ECE02B8C3F79BE0C4AB73EEF8AC84B5537063B1A353074735");
/// 
/// txt = "The length of this message is forty-eight bytes.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.to_string(), "EE74E2DD1FFBFC17A48C80FCBF6A0C55D0A0B4E4F94EDEF7506D28D48EAA5F4DDD7490B3A9CAF212C0CE2101ADABF0C32E4BD91694B8B284C5FAAFE6F54B63D7");
/// 
/// txt = "The unit of the message length is not byte but bit.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.get_HashValue_in_string(), "3893170A81639FB341477704BFB1CDBB8A222F8DAE98B28505EF0552B86DCE1D630ED80DF6CB34D69CD62ADBD88EADD26B550FC9C3CCD7DEFDE4C71594108348");
/// 
/// txt = "This algorithm SHA-2/512 is being tested with this message the length of which is one hundred twelve bytes long.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.to_string(), "134BDEE3708226A589B53F23ED4559203B603D61239B6EBAA4625D4A95ACC5F98182D615194D4035E3CFED16916477C18889E64980A35263B62B8361131B01D4");
/// 
/// txt = "This algorithm SHA-2/512 is being tested for this message the length of which is one hundred sixty-five long so as to check whether or not this algorithm works well.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.get_HashValue_in_string(), "38954039BEA7BFF8DD1DA6E6672A68BD8642F5C4DB7C7CFE11DB2D339DC8FA4EBBC4F1BDC11B4FEC71CB9C84B55FBA85CB11EC4BF72937232044BD004CC90CE7");
/// 
/// txt = "This algorithm SHA-2/512 is being tested with this message the length of which is two hundred ninety-one long so that whether or not this algorithm works well is checked. The message is 'Do you see a man skilled in his work? He will serve before kings; he will not serve before obscure men.'";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
/// assert_eq!(hash.to_string(), "BB87AF8EFB6903DD0FA504AA12E223B00224FF1B6ABF568D7E9C53F65639242E3C0C635A44490D20B4C4B27E748A7675F0C973B6F2784B1105BAFEB91293F0BC");
/// ```
/// 
/// ## Example 3 for SHA2_384
/// ```
/// use std::string::*;
/// use Cryptocol::hash::SHA2_384;
/// let mut hash = SHA2_384::new();
/// let mut txt = "";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.get_HashValue_in_string());
/// assert_eq!(hash.get_HashValue_in_string(), "38B060A751AC96384CD9327EB1B1E36A21FDB71114BE07434C0CC7BF63F6E1DA274EDEBFE76F65FBD51AD2F14898B95B");
/// let txtStirng = String::from("A");
/// hash.digest_string(&txtStirng);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txtStirng, hash);
/// assert_eq!(hash.to_string(), "AD14AAF25020BEF2FD4E3EB5EC0C50272CDFD66074B0ED037C9A11254321AAC0729985374BEEAA5B80A504D048BE1864");
/// let txtArray = ['W' as u8, 'o' as u8, 'w' as u8];
/// hash.digest_array(&txtArray);
/// println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txtArray, hash);
/// assert_eq!(hash.get_HashValue_in_string(), "63DD51EE49AEDD57E85F8BF9A9CF53595FF212BF2E334845AC14CAD17F137C2221D065F8143FB39D3EB2612DD4B429CC");
/// txt = "The length of this message is forty-eight bytes.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.to_string(), "813660BD8EABF78896F5F33727067071635BDACE0E81C158E32E7EB3626820887C42F83E6D9CE973B71B6A0C50C753C0");
/// txt = "The unit of the message length is not byte but bit.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.get_HashValue_in_string(), "435138D7CC9CE82E375B13FE3C75301EB670A8BAFDE4A1952D8D33225E464A62D5747F66F68C8289C5E8BB4C45E16A42");
/// txt = "This algorithm SHA-2/384 is being tested with this message the length of which is one hundred twelve bytes long.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.to_string(), "31D21701C66D9226B1ECEEB713100ECE0C06A1DDCA1EA5B66286316E31B288C7E5147A796195C1A2D93006FFB5372B74");
/// txt = "This algorithm SHA-2/384 is being tested for this message the length of which is one hundred sixty-five long so as to check whether or not this algorithm works well.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.get_HashValue_in_string(), "8B2E5B6105E561A42FC0BE177FEB784321FC67A5024456A48C6A4B481FE483AA3F57A7F57FAE41471362870892465627");
/// txt = "This algorithm SHA-2/384 is being tested with this message the length of which is two hundred ninety-one long so that whether or not this algorithm works well is checked. The message is 'Do you see a man skilled in his work? He will serve before kings; he will not serve before obscure men.'";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
/// assert_eq!(hash.to_string(), "6FE78FC9871EBAF5F19777B7C47B49DB400154DC58684808F06C47CAD1428B46C8AFF2F77C438CCFF287DCA6C60C72FC");
/// ```
/// 
/// ## Example 4 for SHA2_512_256
/// ```
/// use std::string::*;
/// use Cryptocol::hash::SHA2_512_256;
/// let mut hash = SHA2_512_256::new();
/// 
/// let mut txt = "";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.get_HashValue_in_string());
/// assert_eq!(hash.get_HashValue_in_string(), "C672B8D1EF56ED28AB87C3622C5114069BDD3AD7B8F9737498D0C01ECEF0967A");
/// 
/// let txtStirng = String::from("A");
/// hash.digest_string(&txtStirng);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txtStirng, hash);
/// assert_eq!(hash.to_string(), "65A992AD19967492B5780D76A4733AF553F796F688B79102D01EC7FDE5590CAB");
/// 
/// let txtArray = ['W' as u8, 'o' as u8, 'w' as u8];
/// hash.digest_array(&txtArray);
/// println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txtArray, hash);
/// assert_eq!(hash.get_HashValue_in_string(), "E4AF36E824AFDB9E42291983AFA292B894DED2CCAFCCF53346B223FCA846694D");
/// 
/// txt = "The length of this message is forty-eight bytes.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.to_string(), "4E730BDADF49EC9F3E920F72EAD3AC8D09B459900BE4F6E27848652632277205");
/// 
/// txt = "The unit of the message length is not byte but bit.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.get_HashValue_in_string(), "AE0EAB6824897F575FCC051DBC2D1AA7F7BF0DB2C80172F639CE20B3B498C9D5");
/// 
/// txt = "This algorithm SHA-2/512/256 is being tested with this message, the length of which is one hundred twelve bytes.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.to_string(), "7876C6F1285C4B6EC6A2F4A76BBF81815B470536F3A38B7028AA88A3C5C31651");
/// 
/// txt = "This algorithm SHA-2/512/256 is being tested for this message the length of which is one hundred sixty-nine long so as to check whether or not this algorithm works well.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.get_HashValue_in_string(), "6FCE377EA6116BEAC9C11606C59A5D034C8C6EF5A1920B783A9097E07BE36D31");
/// 
/// txt = "This algorithm SHA-2/512/256 is being tested with this message the length of which is two hundred ninety-seven long so that whether or not this algorithm works well is checked. The message is 'Do you see a man skilled in his work? He will serve before kings; he will not serve before obscure men.'";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
/// assert_eq!(hash.to_string(), "63FD06E11EF67F0F5EF598C3B2F2E221D5557AD1EEA46156D1B657F1EDF08D5D");
/// ```
/// 
/// # Big-endian issue
/// It is just experimental for Big Endian CPUs. So, you are not encouraged
/// to use it for Big Endian CPUs for serious purpose. Only use this crate
/// for Big-endian CPUs with your own full responsibility.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct SHA2_512_Generic<const N: usize = 8, const H0: u64 = 0x6a09e667f3bcc908,
                const H1: u64 = 0xbb67ae8584caa73b, const H2: u64 = 0x3c6ef372fe94f82b,
                const H3: u64 = 0xa54ff53a5f1d36f1, const H4: u64 = 0x510e527fade682d1,
                const H5: u64 = 0x9b05688c2b3e6c1f, const H6: u64 = 0x1f83d9abfb41bd6b,
                const H7: u64 = 0x5be0cd19137e2179, const ROUND: usize = 80,
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
}

impl<const N: usize, const H0: u64, const H1: u64, const H2: u64, const H3: u64,
    const H4: u64, const H5: u64, const H6: u64, const H7: u64, const ROUND: usize,
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
SHA2_512_Generic<N, H0, H1, H2, H3, H4, H5, H6, H7, ROUND,
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
    /// Constructs a new object of `SHA2_512` or `SHA2_384`,
    /// or a new SHA2_512-based object.
    /// 
    /// # Output
    /// A new object of `SHA2_512` or `SHA2_384`,
    /// or a new SHA2_512-based object.
    /// 
    /// # Initialization
    /// All the attributes of the constructed object of SHA2_512, which is
    /// initial hash value, will be initialized with
    /// `6A09E667F3BCC908BB67AE8584CAA73B3C6EF372FE94F82BA54FF53A5F1D36F1510E527FADE682D19B05688C2B3E6C1F1F83D9ABFB41BD6B5BE0CD19137E2179`.
    /// All the attributes of the constructed object of SHA2_384, which is
    /// initial hash value, will be initialized with
    /// `CBBB9D5DC1059ED8629A292A367CD5079159015A3070DD17152FECD8F70E593967332667FFC00B318EB44A8768581511`.
    /// All the attributes of the constructed object of SHA2_256_256, which is
    /// initial hash value, will be initialized with
    /// `22312194FC2BF72C9F555FA3C84C64C22393B86B6F53B151963877195940EABD`.
    /// 
    /// # Example 1 for SHA2_512
    /// ```
    /// use Cryptocol::hash::SHA2_512;
    /// let mut hash = SHA2_512::new();
    /// println!("Hash =\t{}", hash);
    /// assert_eq!(hash.to_string(), "6A09E667F3BCC908BB67AE8584CAA73B3C6EF372FE94F82BA54FF53A5F1D36F1510E527FADE682D19B05688C2B3E6C1F1F83D9ABFB41BD6B5BE0CD19137E2179");
    /// ```
    /// 
    /// # Example 2 for SHA2_512_Expanded
    /// ```
    /// use Cryptocol::hash::SHA2_512_Expanded;
    /// type mySHA2 = SHA2_512_Expanded<0x1111_1111_1111_1111, 0x3333_3333_3333_3333, 0x5555_5555_5555_5555, 0x7777_7777_7777_7777, 0x9999_9999_9999_9999, 0xbbbb_bbbb_bbbb_bbbb, 0xdddd_dddd_dddd_dddd, 0xffff_ffff_ffff_ffff, 160>;
    /// let mut my_hash = mySHA2::new();
    /// println!("Hash =\t{}", my_hash);
    /// assert_eq!(my_hash.to_string(), "11111111111111113333333333333333555555555555555577777777777777779999999999999999BBBBBBBBBBBBBBBBDDDDDDDDDDDDDDDDFFFFFFFFFFFFFFFF");
    /// ```
    /// 
    /// # Example 3 for SHA2_384
    /// ```
    /// use Cryptocol::hash::SHA2_384;
    /// let mut hash = SHA2_384::new();
    /// println!("Hash =\t{}", hash);
    /// assert_eq!(hash.to_string(), "CBBB9D5DC1059ED8629A292A367CD5079159015A3070DD17152FECD8F70E593967332667FFC00B318EB44A8768581511");
    /// ```
    /// 
    /// # Example 4 for SHA2_512_256
    /// ```
    /// use Cryptocol::hash::SHA2_512_256;
    /// let mut hash = SHA2_512_256::new();
    /// println!("Hash =\t{}", hash);
    /// assert_eq!(hash.to_string(), "22312194FC2BF72C9F555FA3C84C64C22393B86B6F53B151963877195940EABD");
    /// ```
    pub fn new() -> Self
    {
        if (N > 8) | (N == 0)
            { panic!("N cannot be 0 or greater than 8."); }
        Self
        {
            hash_code: [LongUnion::new_with(Self::H[0]),
                        LongUnion::new_with(Self::H[1]),
                        LongUnion::new_with(Self::H[2]),
                        LongUnion::new_with(Self::H[3]),
                        LongUnion::new_with(Self::H[4]),
                        LongUnion::new_with(Self::H[5]),
                        LongUnion::new_with(Self::H[6]),
                        LongUnion::new_with(Self::H[7])]
        }
    }


    // pub fn digest_C(&mut self, message: *const u8, length_in_bytes_low: u64, length_in_bytes_high: u64)
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
    /// # Example 1 for SHA2_512
    /// ```
    /// use Cryptocol::hash::SHA2_512;
    /// let txt = "This is an example of the method digest().";
    /// let mut hash = SHA2_512::new();
    /// hash.digest_C(txt.as_ptr(), txt.len() as u64, 0);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "7DE9DD6917A9B3353EA426F9C43894513095E944DBE0678491DDABAC0D87236E007374B7438231AB84DE91271F1BCCD11BA64AEC24E3BDC410DE1115A075E2DC");
    /// ```
    ///
    /// # Example 2 for SHA2_512_Expanded
    /// ```
    /// use Cryptocol::hash::SHA2_512_Expanded;
    /// type mySHA2 = SHA2_512_Expanded<0x1111_1111_1111_1111, 0x3333_3333_3333_3333, 0x5555_5555_5555_5555, 0x7777_7777_7777_7777, 0x9999_9999_9999_9999, 0xbbbb_bbbb_bbbb_bbbb, 0xdddd_dddd_dddd_dddd, 0xffff_ffff_ffff_ffff, 160>;
    /// let mut my_hash = mySHA2::new();
    /// my_hash.digest(txt.as_ptr(), txt.len() as u64, 0);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "54D3E18AD977B18F4E3254FBE759C6D8072EFA95A88C671610C16A19D05253A9B3762FE32D054BADBEB877735287A47C1CD7439CA3AE8BE12489D0E8C7F73945");
    /// ```
    /// 
    /// # Example 3 for SHA2_384
    /// ```
    /// use Cryptocol::hash::SHA2_384;
    /// let txt = "This is an example of the method digest().";
    /// let mut hash = SHA2_384::new();
    /// hash.digest_C(txt.as_ptr(), txt.len() as u64, 0);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "1EFF9CDB108E9FC430650DC0A8FB7195654B225B013ECF90F7949077A299D04A921997536D0E1941734ED63FA68AF5E2");
    /// ```
    /// 
    /// # Example 4 for SHA2_512_256
    /// ```
    /// use Cryptocol::hash::SHA2_512_256;
    /// let txt = "This is an example of the method digest().";
    /// let mut hash = SHA2_512_256::new();
    /// hash.digest_C(txt.as_ptr(), txt.len() as u64, 0);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "BF3A06F51CE91951607AABD2E33AD24D8B75618F2366B90D98991AD28E47FAA5");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn digest_C(&mut self, message: *const u8, length_in_bytes_low: u64, length_in_bytes_high: u64)
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
    /// # Example 1 for SHA2_512
    /// ```
    /// use Cryptocol::hash::SHA2_512;
    /// let txt = "This is an example of the method digest().";
    /// let mut hash = SHA2_512::new();
    /// hash.digest(txt.as_ptr(), txt.len() as u128);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "7DE9DD6917A9B3353EA426F9C43894513095E944DBE0678491DDABAC0D87236E007374B7438231AB84DE91271F1BCCD11BA64AEC24E3BDC410DE1115A075E2DC");
    /// ```
    ///
    /// # Example 2 for SHA2_512_Expanded
    /// ```
    /// use Cryptocol::hash::SHA2_512_Expanded;
    /// type mySHA2 = SHA2_512_Expanded<0x1111_1111_1111_1111, 0x3333_3333_3333_3333, 0x5555_5555_5555_5555, 0x7777_7777_7777_7777, 0x9999_9999_9999_9999, 0xbbbb_bbbb_bbbb_bbbb, 0xdddd_dddd_dddd_dddd, 0xffff_ffff_ffff_ffff, 160>;
    /// let mut my_hash = mySHA2::new();
    /// my_hash.digest(txt.as_ptr(), txt.len() as u128);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "54D3E18AD977B18F4E3254FBE759C6D8072EFA95A88C671610C16A19D05253A9B3762FE32D054BADBEB877735287A47C1CD7439CA3AE8BE12489D0E8C7F73945");
    /// ```
    ///
    /// # Example 2 for SHA2_384
    /// ```
    /// use Cryptocol::hash::SHA2_384;
    /// let txt = "This is an example of the method digest().";
    /// let mut hash = SHA2_384::new();
    /// hash.digest(txt.as_ptr(), txt.len() as u128);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "1EFF9CDB108E9FC430650DC0A8FB7195654B225B013ECF90F7949077A299D04A921997536D0E1941734ED63FA68AF5E2");
    /// ```
    ///
    /// # Example 3 for SHA2_512_256
    /// ```
    /// println!("SHA2_512_256_digest");
    /// use Cryptocol::hash::SHA2_512_256;
    /// let txt = "This is an example of the method digest().";
    /// let mut hash = SHA2_512_256::new();
    /// hash.digest(txt.as_ptr(), txt.len() as u128);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "BF3A06F51CE91951607AABD2E33AD24D8B75618F2366B90D98991AD28E47FAA5");
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
    /// # Example 1 for SHA2_512
    /// ```
    /// use Cryptocol::hash::SHA2_512;
    /// let txt = "This is an example of the method digest_str().";
    /// let mut hash = SHA2_512::new();
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "5FD3D145014F7886E64034CC082ADF48670E797DA1C2DA54DDEAF5513E028EB3712121FE6305DB6524C12CBBBB93DF3C0A4DA54E8D798E2AC2A29BA81FB3BFD9");
    /// ```
    ///
    /// # Example 2 for SHA2_384
    /// ```
    /// use Cryptocol::hash::SHA2_384;
    /// let txt = "This is an example of the method digest_str().";
    /// let mut hash = SHA2_384::new();
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "C1C8355C211B2DF4D562014768ECDF21973D60A25EC0C1038C11510E9996084F4871C15A3578BECDF6EAF2F62A8A56C1");
    /// ```
    ///
    /// # Example 3 for SHA2_512_256
    /// ```
    /// use Cryptocol::hash::SHA2_512_256;
    /// let txt = "This is an example of the method digest_str().";
    /// let mut hash = SHA2_512_256::new();
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "D0ED13389E431C8D74FE6E8DB5B6194682874B52E800524136E35D7E9CFA496B");
    /// ```
    /// 
    ///
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
    /// # Example 1 for SHA2_512
    /// ```
    /// use Cryptocol::hash::SHA2_512;
    /// let txt = "This is an example of the method digest_string().".to_string();
    /// let mut hash = SHA2_512::new();
    /// hash.digest_string(&txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "768AF82F599E230387C1F4A4BA2F97F59C6C96B76735A61CFFF3300E808EE0D9FF497957456BB61AABD0F88C19790F0675DD586DC0F5722C60DCB5BB27D6853B");
    /// ```
    ///
    /// # Example 2 for SHA2_384
    /// ```
    /// use Cryptocol::hash::SHA2_384;
    /// let txt = "This is an example of the method digest_string().".to_string();
    /// let mut hash = SHA2_384::new();
    /// hash.digest_string(&txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "19EA6204374E0C4DB800813E7665350754E7B5E5E3A2FC9B95F3F164D7F1E0493D48F2C4ECC32E2F147EB7789F35B9A4");
    /// ```
    ///
    /// # Example 3 for SHA2_512_256
    /// ```
    /// use Cryptocol::hash::SHA2_512_256;
    /// let txt = "This is an example of the method digest_string().".to_string();
    /// let mut hash = SHA2_512_256::new();
    /// hash.digest_string(&txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "2824B79B5D8A1D02454612B72B9CD9544D0DF8E126E7A01E55AC479B0903297C");
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
    /// # Example 1 for SHA2_512
    /// ```
    /// use Cryptocol::hash::SHA2_512;
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// let mut hash = SHA2_512::new();
    /// hash.digest_array(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "4582725B4E904C9FB7C4072B2E4665FB3E4ADC03CB8061C91C0283D582251EA08226CF5A84D9EE441FB30ADB0AB7E564DD66CE8A2BC2BA4B0E32AD36E3BB1253");
    /// ```
    ///
    /// # Example 2 for SHA2_384
    /// ```
    /// use Cryptocol::hash::SHA2_384;
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// let mut hash = SHA2_384::new();
    /// hash.digest_array(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "9F1D9A9407C018C8D95E4CFBC839121AA45521AC2C6AE0F3140E657A1A55384D7F32ACCBD5FCABC27DD7499DC3DB3F6C");
    /// ```
    ///
    /// # Example 3 for SHA2_512_256
    /// ```
    /// use Cryptocol::hash::SHA2_512_256;
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// let mut hash = SHA2_512_256::new();
    /// hash.digest_array(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "E9A9876BBF1432C27CE58D6B8EA66B5A0B719FA80832D491768033F4DAF65A64");
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
    /// # Example 1 for SHA2_512
    /// ```
    /// use Cryptocol::hash::SHA2_512;
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// let mut hash = SHA2_512::new();
    /// hash.digest_vec(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "4582725B4E904C9FB7C4072B2E4665FB3E4ADC03CB8061C91C0283D582251EA08226CF5A84D9EE441FB30ADB0AB7E564DD66CE8A2BC2BA4B0E32AD36E3BB1253");
    /// ```
    ///
    /// # Example 2 for SHA2_384
    /// ```
    /// use Cryptocol::hash::SHA2_384;
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// let mut hash = SHA2_384::new();
    /// hash.digest_vec(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "9F1D9A9407C018C8D95E4CFBC839121AA45521AC2C6AE0F3140E657A1A55384D7F32ACCBD5FCABC27DD7499DC3DB3F6C");
    /// ```
    ///
    /// # Example 3 for SHA2_512_256
    /// ```
    /// use Cryptocol::hash::SHA2_512_256;
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// let mut hash = SHA2_512_256::new();
    /// hash.digest_vec(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "E9A9876BBF1432C27CE58D6B8EA66B5A0B719FA80832D491768033F4DAF65A64");
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
        self.digest(message.as_ptr() as *const u8, (message.len() * T::size_in_bytes()) as u128);
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
    /// # Example 1 for SHA2_512
    /// ```
    /// use Cryptocol::hash::SHA2_512;
    /// let txt = "This is an example of the method get_HashValue().";
    /// let mut hashValue = [0_u8; 64];
    /// let mut hash = SHA2_512::new();
    /// hash.digest_str(txt);
    /// hash.get_HashValue(hashValue.as_ptr() as *mut u8, hashValue.len());
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hashValue);
    /// assert_eq!(format!("{:02X?}", hashValue), "[C9, 9F, F4, 6D, D1, 68, 11, 1D, 0C, 7A, 3E, 16, 5C, BB, 06, 40, D6, DE, 58, 8F, E8, 7D, DB, 29, A1, 3C, 44, CE, FF, B3, E8, 0E, C7, 7C, F0, 09, 8A, 2E, DF, 2F, 53, 32, 6F, 56, 73, AD, 8E, 3F, 87, 0F, 12, 60, CD, B1, 50, DA, 65, B9, 57, 02, C9, CA, 34, 18]");
    /// ```
    ///
    /// # Example 2 for SHA2_384
    /// ```
    /// use Cryptocol::hash::SHA2_384;
    /// let txt = "This is an example of the method get_HashValue().";
    /// let mut hashValue = [0_u8; 48];
    /// let mut hash = SHA2_384::new();
    /// hash.digest_str(txt);
    /// hash.get_HashValue(hashValue.as_ptr() as *mut u8, hashValue.len());
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hashValue);
    /// assert_eq!(format!("{:02X?}", hashValue), "[BF, C6, 22, 65, 13, A5, 81, E7, 08, 3E, 78, 32, D5, C5, 91, 50, 5B, C8, 4F, 67, 5B, AF, 5D, 39, F1, B9, 06, 0F, 72, 2F, 57, E9, 5C, D1, 18, F3, 5C, 00, CE, BD, 77, 21, 09, 63, 6C, CC, 6C, 7F]");
    /// ```
    ///
    /// # Example 3 for SHA2_512_256
    /// ```
    /// use Cryptocol::hash::SHA2_512_256;
    /// let txt = "This is an example of the method get_HashValue().";
    /// let mut hashValue = [0_u8; 32];
    /// let mut hash = SHA2_512_256::new();
    /// hash.digest_str(txt);
    /// hash.get_HashValue(hashValue.as_ptr() as *mut u8, hashValue.len());
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hashValue);
    /// assert_eq!(format!("{:02X?}", hashValue), "[6B, CD, 05, 58, 76, E2, E2, 10, E1, BA, 59, 18, 0A, A8, 04, 8B, 49, 86, CB, 12, E2, 56, 1F, DB, 26, 0A, 0F, 0C, 25, 8F, 88, DD]");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn get_HashValue(&self, hashValue: *mut u8, length: usize)
    {
        const BYTES: usize = 8;
        let n_length = if length < (BYTES * N) {length} else {BYTES * N};
        #[cfg(target_endian = "little")]
        {
            let mut hash_code = [0_u64; N];
            for i in 0..N
                { hash_code[i] = self.hash_code[i].get().to_be(); }
            unsafe { copy_nonoverlapping(hash_code.as_ptr() as *const u8, hashValue, n_length); }
        }
        #[cfg(target_endian = "big")]
        unsafe { copy_nonoverlapping(self.hash_code.as_ptr() as *const u8, hashValue, n_length); }
    }

    // pub fn get_HashValue_in_string(&self) -> String
    /// Returns a hash value in the form of String object.
    /// 
    /// # Output
    /// A hash value in the form of String object.
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
    /// # Example 1 for SHA2_512
    /// ```
    /// use Cryptocol::hash::SHA2_512;
    /// let txt = "This is an example of the method get_HashValue_in_string().";
    /// let mut hash = SHA2_512::new();
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_HashValue_in_string());
    /// assert_eq!(hash.get_HashValue_in_string(), "C5B844FE2254A12B6C59A0F532DD91FE3B3C00228D21BFBE3C2005FE0F10677BEEAC811DA09AC12CC5E0B8F5C7DB80BE93672891C7D59D19E6BA03BB645E8A6A");
    /// ```
    ///
    /// # Example 2 for SHA2_384
    /// ```
    /// use Cryptocol::hash::SHA2_384;
    /// let txt = "This is an example of the method get_HashValue_in_string().";
    /// let mut hash = SHA2_384::new();
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_HashValue_in_string());
    /// assert_eq!(hash.get_HashValue_in_string(), "01A9DB874F3DFC2C68307813BF93C6DA38E78DB69B9ECDB9D52B4A294B40C5A5FB66887179289A11D282AD203A7A5A3E");
    /// ```
    ///
    /// # Example 3 for SHA2_512_256
    /// ```
    /// use Cryptocol::hash::SHA2_512_256;
    /// let txt = "This is an example of the method get_HashValue_in_string().";
    /// let mut hash = SHA2_512_256::new();
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_HashValue_in_string());
    /// assert_eq!(hash.get_HashValue_in_string(), "CFEEC9D8BAB717BBD52B89202B0D671603C6841A0D81FF08C9E1AC60AAD2D038");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn get_HashValue_in_string(&self) -> String
    {
        const BYTES: usize = 8;
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

    // pub fn get_HashValue_in_array(&self) -> [u64; N]
    /// Returns a hash value in the form of array object.
    /// 
    /// # Output
    /// A hash value in the form of array object [u64; N].
    /// 
    /// # Panics
    /// If N > 8, this method will panic
    /// or its behaviour is undefined even if it won't panic.
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
    /// # Example 1 for SHA2_512
    /// ```
    /// use Cryptocol::hash::SHA2_512;
    /// let txt = "This is an example of the method get_HashValue_in_array().";
    /// let mut hash = SHA2_512::new();
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_HashValue_in_array());
    /// assert_eq!(format!("{:02X?}", hash.get_HashValue_in_array()), "[DF2061589C2CD7CB, BDED67828A40F664, 23ABD9849879BB3C, 45F70A5F7BA2E217, 666F4D1842369657, 99727C580358CA62, 1BDE02FE67C79A69, F86777E8508F7358]");
    /// ```
    ///
    /// # Example 2 for SHA2_384
    /// ```
    /// use Cryptocol::hash::SHA2_384;
    /// let txt = "This is an example of the method get_HashValue_in_array().";
    /// let mut hash = SHA2_384::new();
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_HashValue_in_array());
    /// assert_eq!(format!("{:02X?}", hash.get_HashValue_in_array()), "[18B2F598DD216043, C79769E9313C4A07, 75FF5D2AA370B3AD, 7C08DDEA25D3DC55, 33E87C2233400798, 6EB87DCE52E67FCB]");
    /// ```
    ///
    /// # Example 3 for SHA2_512_256
    /// ```
    /// use Cryptocol::hash::SHA2_512_256;
    /// let txt = "This is an example of the method get_HashValue_in_array().";
    /// let mut hash = SHA2_512_256::new();
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_HashValue_in_array());
    /// assert_eq!(format!("{:02X?}", hash.get_HashValue_in_array()), "[84D92CCECF19A8E1, F10F35786EEE5BC5, B58793B76661CF2, 56EED9A53EDF76F4]");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn get_HashValue_in_array(&self) -> [u64; N]
    {
        let mut res = [0_u64; N];
        for i in 0..N
            { res[i] = self.hash_code[i].get().to_be(); }
        res
    }

    // pub fn getHashValue_in_vec(&self) -> Vec
    /// Returns a hash value in the form of Vec object.
    /// 
    /// # Output
    /// A hash value in the form of Vec object Vec<u64>
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
    /// # Example 1 for SHA2_512
    /// ```
    /// use Cryptocol::hash::SHA2_512;
    /// let txt = "This is an example of the method get_HashValue_in_vec().";
    /// let mut hash = SHA2_512::new();
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_HashValue_in_vec());
    /// assert_eq!(format!("{:02X?}", hash.get_HashValue_in_vec()), "[20F75D6A9595036B, 78467C5E0D27C702, A29A92B4B48FAB0, D94AF909AB89A4FC, 4889F1A8498D68CF, 6478B4272EE058DC, 3FD28DF6187A3DB4, F060D07971BFCFF0]");
    /// ```
    ///
    /// # Example 2 for SHA2_384
    /// ```
    /// use Cryptocol::hash::SHA2_384;
    /// let txt = "This is an example of the method get_HashValue_in_vec().";
    /// let mut hash = SHA2_384::new();
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_HashValue_in_vec());
    /// assert_eq!(format!("{:02X?}", hash.get_HashValue_in_vec()), "[7656E133C42F0A53, 738ED067E15A69D0, E901430889B02383, 909FC0A1EEF00A70, CCB39A7A8482F2B2, 9FF389A078F3A041]");
    /// ```
    ///
    /// # Example 3 for SHA2_512_256
    /// ```
    /// use Cryptocol::hash::SHA2_512_256;
    /// let txt = "This is an example of the method get_HashValue_in_vec().";
    /// let mut hash = SHA2_512_256::new();
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_HashValue_in_vec());
    /// assert_eq!(format!("{:02X?}", hash.get_HashValue_in_vec()), "[3D61494146786869, A70B85EEF50F080B, AC0C9758D1CD8A32, 776CD6411B2BBAAA]");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn get_HashValue_in_vec(&self) -> Vec<u64>
    {
        let mut res = Vec::new();
        for i in 0..N
            { res.push(self.hash_code[i].get().to_be()); }
        res
    }

    // pub fn put_HashValue_in_array<T, const M: usize>(&self, out: &mut [T; M])
    /// Puts a hash value in the form of array object.
    /// 
    /// # Features
    /// If M * mem::size_of::<T>() > mem::size_of::<u32>() * N,
    /// it pass the output as the amount of mem::size_of::<u32>() * N.
    ///
    /// # Example 1 for SHA2_512
    /// ```
    /// use Cryptocol::hash::SHA2_512;
    /// let txt = "This is an example of the method put_HashValue_in_array().";
    /// let mut hash = SHA2_512::new();
    /// let mut hash_code = [0_u64; 8];
    /// hash.digest_str(txt);
    /// hash.put_HashValue_in_array(&mut hash_code);
    /// println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash_code);
    /// assert_eq!(format!("{:016X?}", hash_code), "[A3CC3D33B153F0B9, 34F8FFEBD0AB169C, 44A86EA8AEAD72D8, FA23418B701D96F8, 93061E766F3A07EC, 34F77254BAEB1447, 65E2A891ACA2B972, 15ABD372597E0128]");
    /// ```
    ///
    /// # Example 2 for SHA2_512_Expanded
    /// ```
    /// use Cryptocol::hash::SHA2_512_Expanded;
    /// type mySHA2 = SHA2_512_Expanded<0x1111_1111_1111_1111, 0x3333_3333_3333_3333, 0x5555_5555_5555_5555, 0x7777_7777_7777_7777, 0x9999_9999_9999_9999, 0xbbbb_bbbb_bbbb_bbbb, 0xdddd_dddd_dddd_dddd, 0xffff_ffff_ffff_ffff, 160>;
    /// let txt = "This is an example of the method put_HashValue_in_array().";
    /// let mut my_hash = mySHA2::new();
    /// let mut hash_code = [0_u64; 8];
    /// my_hash.digest_str(txt);
    /// my_hash.put_HashValue_in_array(&mut hash_code);
    /// println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash_code);
    /// assert_eq!(format!("{:016X?}", hash_code), "[842F1672FE5A67DB, 7F1EDB14EFE76CCE, 43235DF2AC2AD679, BBE6DF968F224E0A, D472C47FF927A886, 1BDD155EC5B19274, FDC261D8E469BB4C, 592FD7DEBDE994A1]");
    /// ```
    ///
    /// # Example 3 for SHA2_384
    /// ```
    /// use Cryptocol::hash::SHA2_384;
    /// let txt = "This is an example of the method put_HashValue_in_array().";
    /// let mut hash = SHA2_384::new();
    /// let mut hash_code = [0_u64; 6];
    /// hash.digest_str(txt);
    /// hash.put_HashValue_in_array(&mut hash_code);
    /// println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash_code);
    /// assert_eq!(format!("{:016X?}", hash_code), "[2777A97F91255475, F127D5AC237903EE, 62711EF410BDD62D, 4F880A8577004A7A, 3C982DC71C4E3C21, 55B8486AB8A12ABB]");
    /// ```
    ///
    /// # Example 4 for SHA2_384_Expanded
    /// ```
    /// use Cryptocol::hash::SHA2_384_Expanded;
    /// type mySHA2 = SHA2_384_Expanded<160>;
    /// let txt = "This is an example of the method put_HashValue_in_array().";
    /// let mut my_hash = mySHA2::new();
    /// let mut hash_code = [0_u64; 6];
    /// my_hash.digest_str(txt);
    /// my_hash.put_HashValue_in_array(&mut hash_code);
    /// println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash_code);
    /// assert_eq!(format!("{:016X?}", hash_code), "[40212E268A1D743C, D5FC2CC6747702FD, 571FEBB10FB1B290, 9F7E090DF0195B12, 65DF23A919E58C1B, CE317215CBDF8EC1]");
    /// ```
    ///
    /// # Example 5 for SHA2_512_256
    /// ```
    /// use Cryptocol::hash::SHA2_512_256;
    /// let txt = "This is an example of the method put_HashValue_in_array().";
    /// let mut hash = SHA2_512_256::new();
    /// let mut hash_code = [0_u64; 4];
    /// hash.digest_str(txt);
    /// hash.put_HashValue_in_array(&mut hash_code);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    /// assert_eq!(format!("{:08X?}", hash_code), "[218BF516E454F8E8, 3275DBD07098A67B, B1D289D9DCC2A854, 4D1672BEFB75B043]");
    /// ```
    ///
    /// # Example 6 for SHA2_512_256_Expanded
    /// ```
    /// use Cryptocol::hash::SHA2_512_256_Expanded;
    /// type mySHA2 = SHA2_512_256_Expanded<160>;
    /// let txt = "This is an example of the method put_HashValue_in_array().";
    /// let mut my_hash = mySHA2::new();
    /// let mut hash_code = [0_u64; 4];
    /// my_hash.digest_str(txt);
    /// my_hash.put_HashValue_in_array(&mut hash_code);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    /// assert_eq!(format!("{:08X?}", hash_code), "[DF7A58B7D93D2EA8, 5199F8078F5F6813, D6390AFF49FE37FE, 59E9B161E91C2EB8]");
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
        let length = if out_size < (u64::size_in_bytes() * N) {out_size} else {u64::size_in_bytes() * N};
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
    /// # Example 1 for SHA2_512
    /// ```
    /// use Cryptocol::hash::SHA2_512;
    /// let txt = "TANGLING";
    /// let mut hash = SHA2_512::new();
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash.get_HashValue_in_array());
    /// assert_eq!(format!("{:016X?}", hash.get_HashValue_in_array()), "[070B6A9457F65DD9, A7D2C2326CE14E8A, E870D6939FE02E39, 5CFEEDCA96BF3BA3, 013FFB332B3F51F3, B1D4E16355DBE0A9, E998240787066535, 1D5F597F04F84820]");
    /// hash.tangle(1);
    /// println!("Hash =\t{:016X?}", hash.get_HashValue_in_array());
    /// assert_eq!(format!("{:016X?}", hash.get_HashValue_in_array()), "[4780AEEAD19D5962, C55EAFBA7590FB70, CA6587899B2B276F, 55361EC5C9568667, FFD38C58FF62C288, 5E96A9FFC6B17704, 6D3885C75FE9B667, BFDA80D1514F38E5]");
    /// hash.tangle(1);
    /// println!("Hash =\t{:016X?}", hash.get_HashValue_in_array());
    /// assert_eq!(format!("{:016X?}", hash.get_HashValue_in_array()), "[D7FFE2BEEB81D532, EA420969761C4DAA, 8EE930740ABBBE3E, 0DC90C0705AE5F38, E91531243615F994, 174C4F96168FBFC4, 06373FFDD9C66A16, 910560A5898E3728]");
    /// ```
    ///
    /// # Example 2 for SHA2_512_Expanded
    /// ```
    /// use Cryptocol::hash::SHA2_512_Expanded;
    /// type mySHA2 = SHA2_512_Expanded<0x1111_1111_1111_1111, 0x3333_3333_3333_3333, 0x5555_5555_5555_5555, 0x7777_7777_7777_7777, 0x9999_9999_9999_9999, 0xbbbb_bbbb_bbbb_bbbb, 0xdddd_dddd_dddd_dddd, 0xffff_ffff_ffff_ffff, 160>;
    /// let txt = "TANGLING";
    /// let mut my_hash = mySHA2::new();
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, my_hash.get_HashValue_in_array());
    /// assert_eq!(format!("{:016X?}", my_hash.get_HashValue_in_array()), "[D0EDE5AFDDAB96B5, 78B6CC968AFB83EB, CE2369C35DA4A43F, 4B753CF1D02A1A3F, 29A3861EBD42210C, 952536C0957B0B60, 675FE725336E105E, 6E2ACB9D03A95AD2]");
    /// my_hash.tangle(1);
    /// println!("Hash =\t{:016X?}", my_hash.get_HashValue_in_array());
    /// assert_eq!(format!("{:016X?}", my_hash.get_HashValue_in_array()), "[5C72DB128F57491F, B70402F02D41A779, 1B9B1C9979BD59AF, 90ABF522230D4DB3, 2330B855BB6C253C, 297D4E6FF6B37F70, 929F3A8F3CB9A7FD, 3EDD2459251BB838]");
    /// my_hash.tangle(1);
    /// println!("Hash =\t{:016X?}", my_hash.get_HashValue_in_array());
    /// assert_eq!(format!("{:016X?}", my_hash.get_HashValue_in_array()), "[A2090D429E425CEC, D6FD81EEE61ED3B5, 34D1E87A7B4B06E3, 7415804887A7528D, 89EF9F2F4F6CC538, EED8FE585C02AF99, C20EB506C486C145, 730E9AA7A3B591E6]");
    /// ```
    ///
    /// # Example 3 for SHA2_384
    /// ```
    /// use Cryptocol::hash::SHA2_384;
    /// let txt = "TANGLING";
    /// let mut hash = SHA2_384::new();
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash.get_HashValue_in_array());
    /// assert_eq!(format!("{:016X?}", hash.get_HashValue_in_array()), "[A52945B3E9E6E2E0, 7208374E02CB1DFE, 9481D881D89B7946, C425DF584817FD25, 49001993DD7EB02E, A5BF4D24B77D621E]");
    /// hash.tangle(1);
    /// println!("Hash =\t{:016X?}", hash.get_HashValue_in_array());
    /// assert_eq!(format!("{:016X?}", hash.get_HashValue_in_array()), "[84BE10E10BEB5A66, AF72D1F8D4A763E7, 1B2DFA37B163EDC6, CEABC9EDAC24CB65, 7845447250E564EC, A4FAF9EAEECB878B]");
    /// hash.tangle(1);
    /// println!("Hash =\t{:016X?}", hash.get_HashValue_in_array());
    /// assert_eq!(format!("{:016X?}", hash.get_HashValue_in_array()), "[707481D3670B0FA8, B89726EA56C4170A, DF8C93E221E240BD, AA0DEAEA3D1C891D, 4B8DF37A322EF5FA, E88A2A9E835BAC4D]");
    /// ```
    ///
    /// # Example 4 for SHA2_384_Expanded
    /// ```
    /// use Cryptocol::hash::SHA2_384_Expanded;
    /// type mySHA2 = SHA2_384_Expanded<160>;
    /// let txt = "TANGLING";
    /// let mut my_hash = mySHA2::new();
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, my_hash.get_HashValue_in_array());
    /// assert_eq!(format!("{:016X?}", my_hash.get_HashValue_in_array()), "[8A515773672A0C7A, 8CA30FEB93D3A13D, CB81222CFD104F01, DEAA36FB688514FE, 01377A73FCD823E5, 1E44AB0506043A7F]");
    /// my_hash.tangle(1);
    /// println!("Hash =\t{:016X?}", my_hash.get_HashValue_in_array());
    /// assert_eq!(format!("{:016X?}", my_hash.get_HashValue_in_array()), "[D6DD49E21832A216, 3676FE0F8EEB0A8D, 4029F8BD7C7C64CC, D47CA3DAE698F1CE, 6BA349E4F33F2853, E1A939130FE9CD81]");
    /// my_hash.tangle(1);
    /// println!("Hash =\t{:016X?}", my_hash.get_HashValue_in_array());
    /// assert_eq!(format!("{:016X?}", my_hash.get_HashValue_in_array()), "[CD00FDD3A9E9F113, AF71F8BC3F147BBC, CF679991FC2D4957, 2DA56392E6B94D9F, 749AD435F6772132, 50CD667F09190781]");
    /// ```
    ///
    /// # Example 5 for SHA2_512_256
    /// ```
    /// use Cryptocol::hash::SHA2_512_256;
    /// let txt = "TANGLING";
    /// let mut hash = SHA2_512_256::new();
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash.get_HashValue_in_array());
    /// assert_eq!(format!("{:016X?}", hash.get_HashValue_in_array()), "[FC36648637962C38, BDFBBAE5DEA75E0E, D72827D56EB79EF9, 4969BAA99DB0E42B]");
    /// hash.tangle(1);
    /// println!("Hash =\t{:016X?}", hash.get_HashValue_in_array());
    /// assert_eq!(format!("{:016X?}", hash.get_HashValue_in_array()), "[96CA6859E014C355, 6BBED0E8DA26FFAD, A4F89477C93C9E8C, 806148BDB037AE26]");
    /// hash.tangle(1);
    /// println!("Hash =\t{:016X?}", hash.get_HashValue_in_array());
    /// assert_eq!(format!("{:016X?}", hash.get_HashValue_in_array()), "[11F5369ABC9E3B5D, D3D869131E697AB2, 1899C8D791BB09FC, 0C6CE82AE3B9D583]");
    /// ```
    ///
    /// # Example 6 for SHA2_512_256_Expanded
    /// ```
    /// use Cryptocol::hash::SHA2_512_256_Expanded;
    /// type mySHA2 = SHA2_512_256_Expanded<160>;
    /// let txt = "TANGLING";
    /// let mut my_hash = mySHA2::new();
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, my_hash.get_HashValue_in_array());
    /// assert_eq!(format!("{:016X?}", my_hash.get_HashValue_in_array()), "[9771ACFA1FFE9B55, BF7CF746370F01E7, D68B291C1C3EEB8C, 5E8D5A2DBC792186]");
    /// my_hash.tangle(1);
    /// println!("Hash =\t{:016X?}", my_hash.get_HashValue_in_array());
    /// assert_eq!(format!("{:016X?}", my_hash.get_HashValue_in_array()), "[B4C1735DDC8677A6, 6AF607FE0979BF92, BFD34066C9E1317F, B51988A069D20E75]");
    /// my_hash.tangle(1);
    /// println!("Hash =\t{:016X?}", my_hash.get_HashValue_in_array());
    /// assert_eq!(format!("{:016X?}", my_hash.get_HashValue_in_array()), "[F12B54C8E3F7F9AB, 3EAD06A674A59791, CF3237564DCBF985, EA8A45DFBFD4B2C9]");
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
            { self.hash_code[i].set(Self::getH(i)); }
    }

    // fn update(&mut self, message: &[u32])
    /// This method is the core part of MD5 hash algorithm.
    /// It has sixty-four rounds. It updates self.hash_code[] for those
    /// sixty-four rounds.
    fn update(&mut self, message: &[u64])
    {
        let mut W = [0_u64; 16];
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
            W[i] = message[i].to_be();
            let S1 = e.rotate_right(RR14) ^ e.rotate_right(RR18) ^ e.rotate_right(RR41);
            let t1 = Self::Ch(e, f, g).wrapping_add(h)
                                .wrapping_add(Self::getK(i))
                                .wrapping_add(W[i])
                                .wrapping_add(S1);
            let S0 = a.rotate_right(RR28) ^ a.rotate_right(RR34) ^ a.rotate_right(RR39);
            let t2 = Self::Maj(a, b, c).wrapping_add(S0);
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
            W[j] = Self::getW(&W, i);
            let S1 = e.rotate_right(RR14) ^ e.rotate_right(RR18) ^ e.rotate_right(RR41);
            let t1 = Self::Ch(e, f, g).wrapping_add(h)
                                .wrapping_add(Self::getK(i))
                                .wrapping_add(W[j])
                                .wrapping_add(S1);
            let S0 = a.rotate_right(RR28) ^ a.rotate_right(RR34) ^ a.rotate_right(RR39);
            let t2 = Self::Maj(a, b, c).wrapping_add(S0);
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
        // 데이터 기록후, 데이터의 길이를 비트 단위로 기록하기 위한 128 비트(16 바이트)와
        // 0b1000_0000를 기록하기 위한 한 바이트의 여유공간이 남아있지 않으면,
        if last_bytes > 110  // (>= 128 - 16 - 1)
        {
            self.update(unsafe {&mu.piece});
            for i in 0..7
                { unsafe { mu.chunk[i] = 0; } }
        }
        unsafe { mu.chunk[7] = (length_in_bytes << 3).to_be(); }    // 데이터 길이의 단위는 바이트가 아니라 비트이다.
        self.update(unsafe {&mu.piece});
    }

	#[inline] fn getK(idx: usize) -> u64    { Self::K[idx % 80] }
	#[inline] fn getH(idx: usize) -> u64    { Self::H[idx] }
    #[inline] fn getS0(W: &[u64; 16], idx: usize) -> u64  { let w = W[(idx-15) & 0b1111]; w.rotate_right(RR1) ^ w.rotate_right(RR8) ^ (w >> SR7) }
    #[inline] fn getS1(W: &[u64; 16], idx: usize) -> u64  { let w = W[(idx-2) & 0b1111]; w.rotate_right(RR19) ^ w.rotate_right(RR61) ^ (w >> SR6) }
    #[inline] fn getW(W: &[u64; 16], idx: usize) -> u64   { W[(idx-16) & 0b1111].wrapping_add(Self::getS0(&W, idx)).wrapping_add(W[(idx-7) & 0b1111]).wrapping_add(Self::getS1(&W, idx)) }
	#[inline] fn Ch(x: u64, y: u64, z: u64) -> u64  { z ^ (x & (y ^ z)) }   // equivalent to { (x & y) | (!x & z) }
	#[inline] fn Maj(x: u64, y: u64, z: u64) -> u64 { (x & y) | (z & (x | y)) } // equivalent to { (x & y) | (y & z) | (z & x) }
    #[inline] fn to_char(nibble: u8) -> char    { if nibble < 10  { ('0' as u8 + nibble) as u8 as char } else { ('A' as u8 - 10 + nibble) as char } }
}


impl<const N: usize, const H0: u64, const H1: u64, const H2: u64, const H3: u64,
    const H4: u64, const H5: u64, const H6: u64, const H7: u64, const ROUND: usize,
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
Display for SHA2_512_Generic<N, H0, H1, H2, H3, H4, H5, H6, H7, ROUND,
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
    /// # Example 1 for SHA2_512 for the method to_string()
    /// ```
    /// use Cryptocol::hash::SHA2_512;
    /// let mut hash = SHA2_512::new();
    /// let txt = "Display::fmt() automagically implement to_string().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
    /// assert_eq!(hash.to_string(), "4253800692B979FD12F63DD77380BF391AAEC2FB7C599BD447A6E9690F1E7CC06ED615C61CB27514B64F56ACD423A3AC6BE2AEB637885786CE720F1516E38BAD");
    /// ```
    /// 
    /// # Example 2 for SHA2_512 for the macro println!()
    /// ```
    /// use Cryptocol::hash::SHA2_512;
    /// let mut hash = SHA2_512::new();
    /// let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "E960F9A2E39AEBB1C1654B5B7408819AE4507F6983F2D592F232CB64C2CD4DB7265DBF5BCDE9DADA7A1B26618E55B39E54C7A9EC6777B5DA70132160C8E4C837");
    /// ```
    /// 
    /// # Example 3 for SHA2_384 for the method to_string()
    /// ```
    /// use Cryptocol::hash::SHA2_384;
    /// let mut hash = SHA2_384::new();
    /// let txt = "Display::fmt() automagically implement to_string().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
    /// assert_eq!(hash.to_string(), "20109C9D8199547993C91DCC64C07771605EEBC0AADD939E84B98C54C4CCF419B0CD73D5C1D4178902C9CD115077656C");
    /// ```
    /// 
    /// # Example 4 for SHA2_384 for the macro println!()
    /// ```
    /// use Cryptocol::hash::SHA2_384;
    /// let mut hash = SHA2_384::new();
    /// let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "222E7F7B1DB2A566D0665D5790B2A4373F006850F06C1E3E83CE6021AF8761BC738BBF10F75A8E45BA09BDB0814DD8E6");
    /// ```
    /// 
    /// # Example 5 for SHA2_512_256 for the method to_string()
    /// ```
    /// use Cryptocol::hash::SHA2_512_256;
    /// let mut hash = SHA2_512_256::new();
    /// let txt = "Display::fmt() automagically implement to_string().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
    /// assert_eq!(hash.to_string(), "5ED309022841125DE856B25C56A741166872A1D681DF5C69F84AD8B2F30E6DD8");
    /// ```
    /// 
    /// # Example 6 for SHA2_512_256 for the macro println!()
    /// ```
    /// use Cryptocol::hash::SHA2_512_256;
    /// let mut hash = SHA2_512_256::new();
    /// let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "660F8CA5DDC61C43BCEBAB6B8FFD4081F9015CE9A7800BFE29B5100709C3E232");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)
        write!(f, "{}", self.get_HashValue_in_string())
    }
}
