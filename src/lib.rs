// Copyright 2023, 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

//! cryptocol crate provides libraries for cryptography.
//! 
//! This crate is optimized for Little-endian CPUs because Little-Endian CPUs
//! are far more popular than Big-endian CPUs. For the information about
//! Endianness (including Little-endian and Big-endian)
//! [Read more](https://en.wikipedia.org/wiki/Endianness).
//! 
//! # Big-endian issue
//! This crate is just experimental for Big-endian CPUs. So, you are not
//! encouraged to use this crate for Big-endian CPUs for serious purpose.
//! Only use this crate for Big-endian CPUs with your own full responsibility.
//! 
//! # Road Map for Version 1.0
//! This crate Cryptocol is planned to provide the following functionalities.
//! The checked items have already been implemented including documentation
//! at least 95%. The unchecked items have been implemented including
//! documentation less than 95% or have not yet even been started to implement.
//! 
//! ## Small Numbers: meaningful as itself, and also foundations mainly for Big Numbers as well as for other modules
//! - [X] Unions for primitive data types and their implementation, and the
//!         implementation of trait SmallUInt for the Unions ---
//!     [`ShortUnion`](number/short_union/union.ShortUnion.html#union.ShortUnion),
//!     [`IntUnion`](number/int_union/union.IntUnion.html#union.IntUnion),
//!     [`LongUnion`](number/long_union/union.LongUnion.html#union.LongUnion),
//!     [`LongerUnion`](number/longer_union/union.LongerUnion.html#union.LongerUnion),
//!     [`SizeUnion`](number/size_union/union.SizeUnion.html#union.SizeUnion),
//!     [`SharedValues`](number/shared_values/union.SharedValues.html#union.SharedValues), and
//!     [`SharedArrays`](number/shared_arrays/union.SharedArrays.html#union.SharedArrays)
//! - [X] Trait SmallUInt, its implementation for primitive data types, and the
//!         implementation of it for the Unions ---
//!     [`SmallUInt`](number/small_uint/trait.SmallUInt.html#trait.SmallUInt)
// ! - [ ] Trait SmallSInt, its implementation for primitive data types, and the
// !        implementation of it for the Unions ---
// !    [`SmallSInt`](number/small_sint/trait.SmallSInt.html#trait.SmallSInt)
// !     _--> Thinking about postponing to Roadmap for ver. 2.0_
//! 
//! ## Big Numbers: meaningful as itself and also the foundation for Asymmetric-Key Algorithms
//! - [ ] Fixed Sized Big Unsigned Integer Operation ---
//!     [`BigUInt`](number/big_uint/struct.BigUInt.html#struct.BigUInt)
// ! - [ ] Fixed Sized Big Signed Integer Operation --- BigSInt
// !    _--> Thinking about postponing to Roadmap for ver. 2.0_
// ! - [ ] Variable Sized Big Signed Integer Operation --- LargeInt
// !    _--> Thinking about postponing to Roadmap for ver. 2.0 or higher_
//! 
//! ## Hash Algorithms
//! - [X] MD4 hash algorithms based on 128 bits
//!     --- Includes MD4 and its expanded versions.
//!     [`MD4_Generic`](hash/md4/struct.MD4_Generic.html#struct.MD4_Generic)
//! - [X] MD5 hash algorithms based on 128 bits
//!     --- Includes MD5 and its expanded versions.
//!     [`MD5_Generic`](hash/md5/struct.MD5_Generic.html#struct.MD5_Generic)
//! - [X] SHA-1 hash algorithms based on 160 bits
//!     --- Includes SHA-1, SHA-0, and their expanded versions.
//!     [`SHA1_Generic`](hash/sha1/struct.SHA1_Generic.html#struct.SHA1_Generic)
//! - [X] SHA-2 hash algorithms based on 256 bits
//!     --- Includes SHA-256, SHA-224, and their expanded versions.
//!     [`SHA2_256_Generic`](hash/sha2_256/struct.SHA2_256_Generic.html#struct.SHA2_256_Generic)
//! - [X] SHA-2 hash algorithms based on 512 bits
//!     --- Includes SHA-512, SHA-384, SHA-512/256, and their expanded versions.
//!     [`SHA2_512_Generic`](hash/sha2_512/struct.SHA2_512_Generic.html#struct.SHA2_512_Generic)
//! - [X] SHA-2 hash algorithms based on 512/t bits
//!     --- Includes 512/256, SHA-512/224, and their expanded versions.
//!     [`SHA2_512_t_Generic`](hash/sha2_512_t/struct.SHA2_512_t_Generic.html#struct.SHA2_512_t_Generic)
//! - [ ] SHA-3 (SHA3-224)
//! - [ ] SHA-3 (SHA3-256)
//! - [ ] SHA-3 (SHA3-384)
//! - [ ] SHA-3 (SHA3-512)
//! - [ ] SHA-3 (SHAKE 128)
//! - [ ] SHA-3 (SHAKE 256)
//! 
//! ## Symmetric-key Algorithms for the Encryption/Decryption of digital data
//! - [ ] DES symmetric-key encryption/decryption algorithm
//!     --- Includes DES and its expanded versions.
//!     `DES_Generic`
// ! (symmetric/des/struct.DES_Generic.html#struct.DES_Generic)
//! - [ ] NDES symmetric-key encryption/decryption algorithm
//!     --- Includes 2DES, 3DES, 4DES, etc., and their expanded versions.
//!     `NDES_Generic`
// ! (symmetric/ndes/struct.NDES_Generic.html#struct.NDES_Generic)
//! - [ ] AES symmetric-key encryption/decryption algorithm
//!     --- Includes AES and its expanded versions.
//!     `AES_Generic`
// ! (symmetric/des/struct.AES_Generic.html#struct.AES_Generic)
//! - [ ] NAES symmetric-key encryption/decryption algorithm
//!     --- Includes 2AES, 3AES, 4AES, etc., and their expanded versions.
//!     `NAES_Generic`
// ! (symmetric/naes/struct.NAES_Generic.html#struct.NAES_Generic)
//! 
//! ## Pseudo-Random Number Generator Algorithms
//! - [ ] Pseudo-random number generator --- struct
//!     [`Random_Generic`](random/random/struct.Random_Generic.html#struct.Random_Generic)
//!     and trait
//!     [Random_Engine](random/trait_random_engine/trait.Random_Engine.html#trait.Random_Engine)
//! - [ ] Pseudo-random number generator engines using hash algorithms ---
//!     [`Any_MD4`](random/random/type.Any_MD4.html#type.Any_MD4),
//!     [`Any_MD5`](random/random/type.Any_MD5.html#type.Any_MD5),
//!     [`Any_SHA0`](random/random/type.Any_SHA0.html#type.Any_SHA0),
//!     [`Any_SHA1`](random/random/type.Any_SHA1.html#type.Any_SHA1),
//!     [`Any_SHA2_256`](random/random/type.Any_SHA2_256.html#type.Any_SHA2_256)
//!     [`Any_SHA2_512`](random/random/type.Any_SHA2_512.html#type.Any_SHA2_512), and
//!     [`Random_SHA2_512`](random/random/type.Random_SHA2_512.html#type.Random_SHA2_512).
//! - [ ] Pseudo-random number generator engines using symmetric-key encryption algorithms ---
//!     `Any_DES`, `Any_NDES`, `Any_AES`, `Any_NAES`, `Random_AES`, and `Random_NAES`.
//! - [ ] Pseudo-random number generator engines using simple randomization algorithm
//!     --- [`Any_Num_C`](random/random/type.Any_Num.html#type.Any_Num_C)
//! 
//! ## Asymmetric-Key Algorithms for the Encryption/Decryption of digital data
// ! - [ ] Diffie-Hellman _--> Thinking about postponing to Roadmap for ver. 2.0_
//! - [ ] RSA (Ron Rivest, Adi Shamir, Leonard Adleman)
//! - [ ] ECC (Elliptic Curve Cryptosystem)
//! 
//! When the implementation of all the above functionalitis are completed,
//! the version number 1.0.0.0 will be given. After that whenever another
//! functionality is added to this crate, the version number will get higher
//! beyond 1.0.0.0. Before the version number 1.0.0.0, the maximum version
//! number will be 0.25.x.x since there are all twenty-five functionalities
//! listed above. So, for example, even if the version number is 0.5.0.0,
//! it does not mean that 50% of all functionalities are implemented.


#![doc(
    html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png",
    html_favicon_url = "https://www.rust-lang.org/favicon.ico",
    html_root_url = "https://rust-random.github.io/rand/"
)]

pub mod number;
pub mod hash;
pub mod symmetric;
pub mod random;
