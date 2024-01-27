// Copyright 2023, 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

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
//! at least 80%. The unchecked items have not yet been implemented including
//! documentation more than 80% or have not yet even been started to implement.
//! 
//! ## Foundations mainly for Big Numbers and also for other modules
//! - [ ] Unions for primitive data types and their implementation ---
//!     [`ShortUnion`](number/small_int_unions/union.ShortUnion.html#union.ShortUnion),
//!     [`IntUnion`](number/small_int_unions/union.IntUnion.html#union.IntUnion),
//!     [`LongUnion`](number/small_int_unions/union.LongUnion.html#union.LongUnion),
//!     [`LongerUnion`](number/small_int_unions/union.LongerUnion.html#union.LongerUnion),
//!     and
//!     [`SizeUnion`](number/small_int_unions/union.SizeUnion.html#union.SizeUnion)
//! - [ ] Trait SmallUInt and its implementation of primitive data types ---
//!     [SmallUInt](number/small_uint/trait.SmallUInt.html#trait.SmallUInt)
//! - [ ] Trait SmallSInt and its implementation of primitive data types
//! 
//! ## Big Numbers
//! - [ ] Fixed Sized Big Unsigned Integer Operation ---
//!     [`BigUInt`](number/big_uint/struct.BigUInt.html#struct.BigUInt)
//! - [ ] Fixed Sized Big Signed Integer Operation --- BigSInt
//! - [ ] Variable Sized Big Signed Integer Operation --- LargeInt
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
//!     [`SHA1_Generic`](hash/sha1/struct.SHA1_generic.html#struct.SHA1_generic)
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
//! ## Symmetric-Key Cryptographic Algorithms
//! - [ ] DES
//! - [ ] 3DES
//! - [ ] AES
//! 
//! ## Pseudo-Random Number Generator Algorithms
//! - [X] The Pseudo-random number generator wrappers
//!     --- struct [`Random_Generic`](random/random/struct.Random_Generic.html#struct.Random_Generic)
//!     and trait
//!     [Random_Engine](random/trait_random_engine/trait.Random_Engine.html#trait.Random_Engine)
//! - [X] The implementation of `Random_Engine` for hash algorithms such as
//!     [`MD4_Generic`](hash/md4/struct.MD4_Generic.html#struct.MD4_Generic),
//!     [`MD5_Generic`](hash/md5/struct.MD5_Generic.html#struct.MD5_Generic),
//!     [`SHA1_Generic`](hash/sha1/struct.SHA1_generic.html#struct.SHA1_generic),
//!     [`SHA2_256_Generic`](hash/sha2_256/struct.SHA2_256_Generic.html#struct.SHA2_256_Generic), and
//!     [`SHA2_512_Generic`](hash/sha2_512/struct.SHA2_512_Generic.html#struct.SHA2_512_Generic),
//!     and for pseudo-random number generation algorithm such as
//!     [`AnyNumber`](random/any_number/struct.AnyNumber.html#struct.AnyNumber)
//! 
//! ## Asymmetric-Key Cryptographic Algorithms
//! - [ ] Diffie-Hellman
//! - [ ] RSA (Ron Rivest, Adi Shamir, Leonard Adleman)
//! - [ ] ECC (Elliptic Curve Cryptosystem)
//! 
//! When the implementation of all the above functionalitis are completed,
//! the version number 1.0.0.0 will be given. After that whenever another
//! functionality is added to this crate, the version number will get higher
//! beyond 1.0.0.0. Before the version number 1.0.0.0, the maximum version
//! number will be 0.26.x.x since there are all twenty-six functionalities
//! listed above. So, for example, even if the version number is 0.5.0.0,
//! it does not mean that 50% of all functionalities are implemented.


#![doc(
    html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png",
    html_favicon_url = "https://www.rust-lang.org/favicon.ico",
    html_root_url = "https://rust-random.github.io/rand/"
)]

pub mod number;
pub mod hash;
pub mod random;