// Copyright 2023, 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! various numbers such as small fixed-sized unsigned integers, small
//! fixed-sized signed integers, small fixed-sized integer unions,
//! big fixed-sized unsigned integers, big fixed-sized signed integers,
//! and large variable-sized signed integers
//!
//! # Introduction
//! This module contains a few sub-modules to define various numbers such as:
//! - small fixed-sized unsigned integers smaller than or same as 128-bit size,
//! - small fixed-sized signed integers smaller than or same as 128-bit size,
//! - small fixed-sized integer unions smaller than or same as 128-bit size,
//! - big fixed-sized unsigned integers bigger than 128-bit size,
//! - big fixed-sized signed integers bigger than 128-bit size, and
//! - large variable-sized signed integers bigger than 128-bit size.
//! 
//! # Background
//! ## Generic Programming of Primitive Data Types
//! When you write any code in Rust, you will find that it is tricky to write
//! generic code for primitive data types such as `u8`, `u16`, `u32`, `u64`,
//! `u128`, `usize`, `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, etc. Writing
//! generic code in Rust is not as straightforward as C++.
//! You have to do so-called 'trait control' in order to write generic code for
//! primitive data types in Rust. The traits `SmallUInt` and `SmallSInt` are
//! written for you who would like to write generic code.
//! ## Additional Useful Methods for Primitive Data Types
//! There are plenty of or even more than enough of methods prepared for
//! primitive data types. However, what if you need some more methods that
//! are not provided? `SmallUInt` and `SmallSInt` provide additional methods
//! for primitive data types. Of course, you can add your own methods that is
//! fit to your purposes if you write your own traits and their implementation.
//! `SmallUInt` and `SmallSInt` will give you hints about how to write your own
//! traits and their implementation to add your own methods to primitive data
//! types.
//! ## Arithmatic Operations of Big Numbers
//! Most of the modern programming languages do not support big numbers
//! such as 256-bit, 512-bit, and 1024-bit integers or even bigger integers.
//! Rust supports up to 128-bit integers such as `u128` and `i128` but not
//! bigger-sized numbers than those. However, in some areas such as
//! cryptography, it is required to calculate 1024-bit or even bigger-sized
//! bit integers especially for such as RSA cryptographic alogorithm.
//! Then, we need special algorithms to calculate such big numbers.
//! This module provides the ability for such special purposes.
//! 
//! # Documentation
//! In many cases, a lot of parts of the documentations of this module were made
//! by taking (or copying and pasting) from pre-existing documentation for
//! standard crates of Rust and tweacking them, espeically when the methods that
//! this crate provides are very similar to the pre-existing ones in terms of
//! their interfaces, functionalities and purposes, for example, operators `+`,
//! `<<`, etc., and methods `from_str()`, `to_be()`, etc.
//! Please don't think they are malicious plagiarism for those cases.
//! 
//! # Two kinds of small-sized bit integers
//! This module provides two kinds of small-sized bit integers:
//! SmallUInt, and SmallSInt.
//! - `SmallUInt` --- a small-sized _unsigned_ integer.
//! [Read more](trait@SmallUInt)
//! - `SmallSInt` --- a small-sized _signed_ integer.
//! 
//! # Five kinds of small-sized bit integer unions
//! This module provides four kinds of small-sized bit integer unions:
//! ShortUnion, IntUnion, LongUnion and LongerUnion.
//! - `ShortUnion` --- a union that `u16`, `i16`, `[u8; 2]`, and `[i8; 2]`
//! share with one another for type conversion and picking specific data
//! [Read more](union@ShortUnion)
//! - `IntUnion` --- a union that `u32`, `i32`, `[u16; 2]`, `[i16; 2]`,
//! `[u8; 4]`, and `[i8; 4]` share with one another for type conversion
//! and picking specific data [Read more](union@IntUnion)
//! - `LongUnion` --- a union that `u64`, `i64`, `[u32; 2]`, `[i32; 2]`,
//! `[u16; 4]`, `[i16; 4]`, `[u8; 8]`, and `[i8; 8]` share with one
//! another for type conversion and picking specific data
//! [Read more](union@LongUnion)
//! - `LongerUnion` --- a union that `u128`, `i128`, `[u64; 2]`, and
//! `[i64; 2]`, `[u32; 4]`, `[i32; 4]`, `[u16; 8]`, `[i16; 8]`, `[u8; 16]`,
//! and `[i8; 16]` share with one another for type conversion and picking
//! specific data [Read more](union@LongerUnion)
//! - `SizeUnion` --- a union that `usize`, `isize`, `[u32; 2]`, and
//! `[i32; 2]`, `[u16; 4]`, `[i16; 4]`, `[u8; 8]`, and `[i8; 8]` share with
//! one another for type conversion and picking specific data in the case of
//! 64-bit machine for example [Read more](union@SizeUnion)
//! - `SharedValues` --- a union that source primitive data type and destination data
//! type share memory with each other. You can use this union to convert data
//! from one data type to another data type by truncating (if destination data
//! type is smaller than source data type) or by filling zeros (if destination
//! data type is bigger than source data type).  [Read more](union@SharedValues)
//! - `SharedArrays` --- a union that the array of source primitive data type and
//! the array of destination data type share memory with each other. You can
//! use this union to convert array data from one array of a certain data type
//! to another array of another data type by truncating (if the total size of
//! array of destination data type is smaller than the total size of array of
//! source data type) or by filling zeros (the total size of array of
//! destination data type is smaller than the total size of array of source
//! data type).  [Read more](union@SharedArrays)
//! 
//! # Three kinds of big-sized bit integers
//! This module provides three kinds of long bit integers: BigUInt, BigInt,
//! and LargeInt.
//! - `BigUInt` --- a big-sized _unsigned_ integer with user-defined _fixed_ size.
//! [Read more](struct@BigUInt) However, docs.rs has been failing in generating `BigUInt` page from ver. 0.8.5 on for some technical reason that has not been solved yet. So, you can download the manual [here](https://drive.google.com/file/d/107hckPdW68sCloCkGS1LaP_7StIJ-quw/view?usp=sharing). I hope that it will be fixed soon.
//! - `BigSInt` --- a big-sized _signed_ integer with user-defined _fixed_ size.
//! - `LargeInt` --- a big-sized _signed_ integer with _variable_ size.
//! 
//! # Predefined big unsigned integer data types
//! There are provided predefined data types: `U256`, `U512`, `U1024`, `U2048`,
//! `U3072`, `U4096`, `U5120`, `U6144`, `U7168`, `U8192`, and `U16384`.
//! And their synonyms are also provided such as `UU32` (= `U256`),
//! `UU64` (= `U512`), `UU128` (= `U1024`), `UU256` (= `U2048`),
//! `UU384` (= `U3072`), `UU512` (= `U4096`), `UU640` (= `U5120`),
//! `UU768` (= `U6144`), `UU896` (= `U7168`), `UU1024` (= `U8192`),
//! and `UU2048` (= `U16384`). You can further define more data types.
//! 
//! # QUICK START
//! - For `SmallUInt`, read [here](trait@SmallUInt#quick-start).
// ! - For `SmallSInt`, read [here](trait@SmallSInt#quick-start).
//! - For `ShortUnion`, read [here](union@ShortUnion#quick-start).
//! - For `IntUnion`, read [here](union@IntUnion#quick-start).
//! - For `LongUnion`, read [here](union@LongUnion#quick-start).
//! - For `LongUnion`, read [here](union@LongerUnion#quick-start).
//! - For `SizeUnion`, read [here](union@SizeUnion#quick-start).
//! - For `SharedValues`, read [here](union@SharedValues#quick-start).
//! - For `SharedArrays`, read [here](union@SharedArrays#quick-start).
//! - For `BigUInt`, read [here](struct@BigUInt#quick-start).
// ! - For `BigSInt`, read [here](struct@BigSInt#quick-start).
// ! - For `LargeInt`, read [here](struct@LargeInt#quick-start).


pub mod small_uint;
pub mod small_sint;
pub mod short_union;
pub mod int_union;
pub mod long_union;
pub mod longer_union;
pub mod size_union;
pub mod shared_values;
pub mod shared_arrays;
pub mod big_uint;
pub mod number_errors;
pub mod macros_for_types;
pub mod macros_for_integer_unions;

/// Implementaion of trait SmallUInt for u8, u16, u32, u64, u128, and usize
pub mod trait_small_uint_for_unsigned_impl;

/// Implementaion of trait SmallUInt for ShortUnion, IntUnion, LongUnion, LongerUnion, and SizeUnion
pub mod trait_small_uint_for_integer_unions_impl;

/// Implementaion of various traits for BigUInt
pub mod traits_for_big_uint_impl;


pub use small_uint::*;
pub use small_sint::*;
pub use short_union::*;
pub use int_union::*;
pub use long_union::*;
pub use longer_union::*;
pub use size_union::*;
pub use shared_values::*;
pub use shared_arrays::*;
pub use big_uint::*;
pub use number_errors::*;
use macros_for_integer_unions::*;



/********** FOR BIG-ENDIANNESS ONLY **********/

#[cfg(target_endian = "big")]
pub mod traits_for_big_uint_for_big_endian_impl;

#[cfg(target_endian = "big")]
pub use traits_for_big_uint_for_big_endian_impl::*;