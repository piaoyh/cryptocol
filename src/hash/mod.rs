// Copyright 2023, 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! various cryptographic hash functions
//! 
//! # Introduction
//! The module that contains a few sub-modules
//! to define cryptographic hash functions
//! 
//! # Background: Cryptographic hash functions
//! What if we can make a kind of finger print of data and the finger print is
//! of small size? Then, we can identify data with that small-sized finger
//! print. And, by means of the finger print of the data, we can also determine
//! whether or not certain data has been changed. The finger print of data with
//! small size must be very useful. The output of the cryptographic hash
//! function is called hash value and can virtually play the role of the finger
//! print of data.
//! 
//! Cryptographic hash functions are hash algorithms that have three properties:
//! - Pre-image resistance;
//! - Second pre-image resistance; and
//! - Collision resistance.
//! 
//! Read [this article](https://en.wikipedia.org/wiki/Cryptographic_hash_function)
//! and/or Watch [this lecture](https://www.youtube.com/watch?v=bDIc3jcLlOE)
//! to learn cryptographic hash functions more in detail.
//! 
//! # The algorithms of cryptographic hash functions
//! This module provides several kinds of cryptographic hash algorithms:
//! - MD4 hash algorithms based on 128 bits --- Includes MD4 and its expanded versions. [`MD4_Generic`](struct@MD4_Generic)
//! - MD5 hash algorithms based on 128 bits --- Includes MD5 and its expanded versions. [`MD5_Generic`](struct@MD5_Generic)
//! - SHA-1 hash algorithms based on 160 bits --- Includes SHA-1, SHA-0, and their expanded versions. [`SHA1_Generic`](struct@SHA1_Generic)
//! - SHA-2 hash algorithms based on 256 bits --- Includes SHA-256, SHA-224, and their expanded versions. [`SHA2_Generic_256`](struct@SHA2_256_Generic)
//! - SHA-2 hash algorithms based on 512 bits --- Includes SHA-512, SHA-384, SHA-512/256, and their expanded versions. [`SHA2_512_Generic`](struct@SHA2_512_Generic)
//! - SHA-2 hash algorithms based on 512/t bits --- Includes 512/256, SHA-512/224, and their expanded versions. [`SHA2_512_t_Generic`](struct@SHA2_512_t_Generic)
//! 
//! # QUICK START
//! - For `MD4`, read [here](struct@MD4_Generic#quick-start).
//! - For `MD5`, read [here](struct@MD5_Generic#quick-start).
//! - For `SHA-1`, read [here](struct@SHA1_Generic#quick-start).
//! - For `SHA-256`, read [here](struct@SHA2_256_Generic#quick-start).
//! - For `SHA-512`, read [here](struct@SHA2_512_Generic#quick-start).
//! - For `SHA-512/t`, read [here](struct@SHA2_512_t_Generic#quick-start).
//! 
//! # Application

mod md4;
mod md5;
mod sha1;
mod sha2_256;
mod sha2_512;
mod sha2_512_t;

pub use md4::*;
pub use md5::*;
pub use sha1::*;
pub use sha2_256::*;
pub use sha2_512::*;
pub use sha2_512_t::*;
