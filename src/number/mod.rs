// Copyright 2023 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! # Introduction
//! The module that contains a few sub-modules to define Big numbers bigger
//! than 128-bit integer and their arithmatic operations
//! 
//! # Background: Arithmatic operations of big numbers
//! 
//! Most of the modern programming languages do not support big numbers
//! such as 256-bit, 512-bit, and 1024-bit integers or even bigger integers.
//! Rust supports up to 128-bit integers such as u128 and i128
//! but not bigger numbers than those.
//! 
//! In some areas such as cryptography, however, it is required to calculate
//! 1024-bit or even longer bit integers especially for such as RSA
//! cryptographic alogorithm. Then, we need special algorithms to calculate
//! such big numbers. This is the library for such special purposes.
//! 
//! # Three kinds of long bit integers
//! This module provides three kinds of long bit integers: BigUInt, BigInt,
//! and LargeInt.
//! - `BigUInt` --- a big _unsigned_ integer with user-defined _fixed_ size. [Read more](struct@BigUInt)
//! - `BigSInt` --- a big _signed_ integer with user-defined _fixed_ size.
//! - `LargeInt` --- a big _signed_ integer with _variable_ size.
//! 
//! # Predefined big unsigned integer data types
//! There are provided predefined data types: `u256`, `u512`, `u1024`, `u2048`,
//! `u3072`, `u4096`, `u5120`, `u6144`, `u7168`, `u8192`, and `u16384`.
//! And their synonyms are also provided such as `U32` (= `u256`),
//! `U64` (= `u512`), `U128` (= `u1024`), `U256` (= `u2048`),
//! `U384` (= `u3072`), `U512` (= `u4096`), `U640` (= `u5120`),
//! `U768` (= `u6144`), `U896` (= `u7168`), `U1024` (= `u8192`),
//! and `U2048` (= `u16384`). You can further define more data types.
//! 
//! `u256` for 64-bit machines is `BigUInt<u64, 4>` for example
//! while `u256` for 32-bit machines is `BigUInt<u32, 8>` for example.
//! 
//! # QUICK START
//! For `BigUInt`, read [here](struct@BigUInt#quick-start).
//! 

pub mod small_uint;
pub mod small_sint;
pub mod small_int_unions;
pub mod big_uint;
pub mod trait_impl_for_big_uint;
pub mod number_errors;
mod macros_number;

pub use small_uint::*;
pub use small_int_unions::*;
pub use small_sint::*;
pub use big_uint::*;
pub use trait_impl_for_big_uint::*;
pub use number_errors::*;




/********** FOR BIG-ENDIANNESS ONLY **********/

#[cfg(target_endian = "big")]
pub mod big_uint_for_big_endian;

#[cfg(target_endian = "big")]
pub use big_uint_for_big_endian::*;

