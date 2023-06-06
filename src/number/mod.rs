// Copyright 2023 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! Big numbers bigger than 128-bit integer and their arithmatic operation
//! 
//! # Background: Arithmatic operations of big numbers
//! 
//! Most of the modern programming languages do not support big numbers
//! such as 256-bit, 512-bit, and 1024-bit integers or even bigger integers.
//! Rust supports up to 128-bit integers such as u128 and i128
//! but not bigger numbers than those.
//! 
//! In some areas such as cryptography, however, it is required to calculate
//! 1024-bit or even longer bit integers especially for RSA cryptographic
//! alogorithm. Then, we need special algorithms to calculate such big numbers.
//! This is the library for such special purposes.
//! 
//! # Three kinds of long bit integers
//! This module provides three kinds of long bit integers: BigUInt, BigInt,
//! and LargeInt.
//! - `BigUInt` --- a big _unsigned_ integer with user-defined _fixed_ size.
//! - `BigInt` --- a big _signed_ integer with user-defined _fixed_ size.
//! - `LargeInt` --- a big _signed_ integer with _variable_ size.
//! 

pub mod huge_number;
pub mod uint;
pub mod int;
pub mod numeric;
pub mod float;
pub mod real;
pub mod big_uint;

pub use huge_number::*;
pub use uint::*;
pub use int::*;
pub use numeric::*;
pub use float::*;
pub use real::*;
pub use big_uint::*;

