// Copyright 2023 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! # Introduction
//! The module that contains a few sub-modules
//! to define Cryptographic hash functions
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
//! - `MD5` --- [Read more](struct@MD5)
//! - `SHA1` --- [Read more](struct@SHA1)
//! - `SHA-2 (SHA-224)` --- [Read more](struct@SHA2_224)
//! - `SHA-2 (SHA-256)` --- [Read more](struct@SHA2_256)
//! - `SHA-2 (SHA-384)` --- [Read more](struct@SHA2_384)
//! - `SHA-2 (SHA-512)` --- [Read more](struct@SHA2_512)
// ! - `SHA-2 (SHA-512/224)` --- [Read more](struct@SHA2_512_224)
// ! - `SHA-2 (SHA-512/256)` --- [Read more](struct@SHA2_512_256)
//! - `SHA-3 (SHA3-224)` --- [Read more](struct@SHA3_224)
//! - `SHA-3 (SHA3-256)` --- [Read more](struct@SHA3_256)
//! - `SHA-3 (SHA3-384)` --- [Read more](struct@SHA3_348)
//! - `SHA-3 (SHA3-512)` --- [Read more](struct@SHA3_512)
//! - `SHA-3 (SHAKE 128)` --- [Read more](struct@SHA3_SHAKE_128)
//! - `SHA-3 (SHAKE 256)` --- [Read more](struct@SHA3_SHAKE_256)
//! 
//! # QUICK START
//! - For `MD5`, read [here](struct@MD5#quick-start).
//! - For `SHA1`, read [here](struct@SHA1#quick-start).

mod sha1;
mod md5;

pub use sha1::*;
pub use md5::*;
