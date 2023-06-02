// Copyright 2023 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! Big numbers bigger than 128-bit integer and their arithmatic operation
//! 
//! ## Background: Arithmatic operations of big numbers
//! 
//! Most of the modern programming languages do not support big numbers such as
//! 256-bit, 512-bit, and 1024-bit integers or even bigger integers.
//! Rust support up to u128 and i128 but not bigger numbers than those.
//! In some areas such as cryptography, however, it is required to calculate
//! 1024-bit or even longer bit integers especially for RSA cryptographic
//! alogorithm. Then, we need some libraries to calculate such big numbers.
//! 
//! ## Traits and functionality
//! 
//! 

pub mod huge_number;
pub mod uint;
pub mod big_uint;

pub use huge_number::*;
pub use uint::*;
pub use big_uint::*;

