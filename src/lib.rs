// Copyright 2023 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

//! Cryptocol crate provides libraries for cryptography.
//! 
//! In many cases, a lot of parts of the documentations of this crate were made
//! by taking (or copying and pasting) from pre-existing documentation and
//! twicking if the methods that this crate provides are very similar to the
//! pre-existing ones in terms of their interfaces, functionalities and
//! purposes, for example, operators `+`, `<<`, etc., methods `copy_within()`,
//! from_str(), etc. Please don't think they are plagiarism for those cases.
//! 
//! This crate is optimized for Little-endian CPUs because Little-Endian CPUs
//! are far more popular than Big-endian CPUs. For the information about
//! Endianness (including Little-endian and Big-endian) [Read more](https://en.wikipedia.org/wiki/Endianness).
//! 
//! # Big-endian issue
//! This crate is just experimental for Big-endian CPUs. So, you are not
//! encouraged to use this crate for Big-endian CPUs for serious purpose.
//! Only use this crate for Big-endian CPUs with your own full responsibility.
//! 
//! # Road Map
//! This crate Cryptocol is planned to provide the following functionalities.
//! The checked items have already been implemented including documentation
//! at least 80%. The unchecked items have not yet been implemented less than
//! 80% or have not yet even been started to implement.
//! 
//! ## Foundations for Big Numbers
//! - [ ] Unions for primitive data types and their implementation --- 
//! [`ShortUnion`](number/int_unions/union.ShortUnion.html#union.ShortUnion),
//! [`IntUnion`](number/int_unions/union.IntUnion.html#union.IntUnion),
//! [`LongUnion`](number/int_unions/union.LongUnion.html#union.LongUnion),
//! [`LongerUnion`](number/int_unions/union.LongerUnion.html#union.LongerUnion), and
//! [`SizeUnion`](number/int_unions/union.SizeUnion.html#union.SizeUnion)
//! - [ ] Trait UInt and its implementation for primitive data types and Unions ---
//! [Uint](number/uint/trait.Uint.html#trait.Uint)
//! - [ ] Trait SInt and its implementation for primitive data types and Unions
//! 
//! ## Big Numbers
//! - [x] Fixed Sized Big Unsigned Integer Operation --- [`BigUInt`](number/big_uint/struct.BigUInt.html#struct.BigUInt)
//! - [ ] Fixed Sized Big Signed Integer Operation --- BigSInt
//! - [ ] Variable Sized Big Signed Integer Operation --- LargeInt
//! 
//! ## Hash Algorithms
//! - [ ] MD5
//! - [ ] SHA-1
//! - [ ] SHA-2 (SHA-224)
//! - [ ] SHA-2 (SHA-256)
//! - [ ] SHA-2 (SHA-384)
//! - [ ] SHA-2 (SHA-512)
// ! - [ ] SHA-2 (SHA-512/224)
// ! - [ ] SHA-2 (SHA-512/256)
//! - [ ] SHA-3 (SHA3-224)
//! - [ ] SHA-3 (SHA3-256)
//! - [ ] SHA-3 (SHA3-384)
//! - [ ] SHA-3 (SHA3-512)
//! - [ ] SHA-3 (SHAKE 128)
//! - [ ] SHA-3 (SHAKE 256)
//! 
//! ## Symetric-Key Encoding / Decoding Algorithms
//! - [ ] DES
//! - [ ] 3DES
//! - [ ] AES
//! 
//! ## Asymetric-Key Encoding / Decoding Algorithms
//! - [ ] Diffie-Hellman
//! - [ ] RSA (Ron Rivest, Adi Shamir, Leonard Adleman)
//! - [ ] ECC (Elliptic Curve Cryptosystem)
//! 
//! When all the above functionalitis are implemented, the version number
//! 1.0.0.0 will be given. After that whenever another functionality is added to
//! this crate, the version number will get higher beyond 1.0.0.0.


// #![doc(
//     html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png",
//     html_favicon_url = "https://www.rust-lang.org/favicon.ico",
//     html_root_url = "https://rust-random.github.io/rand/"
// )]

pub mod number;
