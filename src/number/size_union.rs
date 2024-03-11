// Copyright 2023, 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module that contains unions of primitive signed/unsigned integral
//! data types used in a lot of modules of the crate Cryptocol.
//! __These unions are for segmentation.__

// #![warn(missing_docs)]
// #![warn(missing_doc_code_examples)]
#![allow(missing_docs)]
#![allow(rustdoc::missing_doc_code_examples)]

use std::fmt::{ self, Debug, Display, Formatter };
use std::cmp::{ PartialEq, PartialOrd, Ordering };
use std::ops::*;

use super::small_uint::SmallUInt;
use super::longer_union::LongerUnion;

/// # Introduction
/// This union `SizeUnion` is for slicing `usize` into two `u64`s, two `i64`s,
/// four `u32`s, four `i32`s, eight `u16`s, eight `i16`s, fourteen `u8`,
/// and/or fourteen `u8`.
/// 
/// Sometimes, for example, we need to slice `usize` data into two `u64` pieces
/// which include a higher eight-byte word and a lower eight-byte word, and/or
/// into  eight `u16` pieces which include a first four-byte word, a second
/// four-byte word, a third four-byte word and a fourth four-byte word.
/// In that case, `SizeUnion` will be very helpful.
/// 
/// # Quick Start
/// In order to use this union, you have to import (use)
/// `cryptocol::number::SizeUnion` as follows.
/// 
/// ## Example 1
/// ```
/// use cryptocol::number::SizeUnion;
/// ```
/// You can use the methods `get()`, `get_signed()`, `get_ulonger()`, and
/// `get_slonger()` in order to obtain the data of `usize` in various types.
/// And, you can also slice the data of `usize` into two `u64` type data or
/// two`i64` by using the methods `get_ulong()`, `get_slong()`, `get_ulong_()`,
/// and `get_slong_()`. Or, you can also slice the data of `usize` into four
/// `u32` type data or four `i32` type databy using the methods `get_uint()`,
/// `get_sint()`, `get_uint_()`, and `get_sint_()`. Or, you can also slice the
/// data of `usize` into eight `u16` type data or eight `i16` type data by using
/// the methods `get_ushort()`, `get_sshort()`, `get_ushort_()`, and
/// `get_sshort_()`. Or, you can also slice the data of `usize` into fourteen
/// `u8` type data or fourteen `i8` type data by using the methods
/// `get_ubyte()`, `get_sbyte()`, `get_ubyte_()`, and `get_sbyte_()`.
/// 
/// ## Example 2
/// ```
/// use cryptocol::number::SizeUnion;
/// 
/// let a = SizeUnion::new_with_signed(-1234567890987654321012345678987654321_isize);
/// println!("a.get() = {}", a.get());
/// println!("a.get_signed() = {}", a.get_signed());
/// println!("a.get_usize() = {}", a.get_usize());
/// println!("a.get_ssize() = {}", a.get_ssize());
/// println!("a.get_ulonger() = {}", a.get_ulonger());
/// println!("a.get_slonger() = {}", a.get_slonger());
/// assert_eq!(a.get(), 339047799029950809142362261752780557135_usize);
/// assert_eq!(a.get_signed(), -1234567890987654321012345678987654321_isize);
/// assert_eq!(a.get_usize(), 339047799029950809142362261752780557135_usize);
/// assert_eq!(a.get_ssize(), -1234567890987654321012345678987654321_isize);
/// assert_eq!(a.get_ulonger(), 339047799029950809142362261752780557135_u128);
/// assert_eq!(a.get_slonger(), -1234567890987654321012345678987654321_i128);
/// 
/// for i in 0..2
///     { println!("a.get_ulong_({}) = {}", i, a.get_ulong_(i)); }
/// for i in 0..2
///     { println!("a.get_slong_({}) = {}", i, a.get_slong_(i)); }
/// for i in 0..4
///     { println!("a.get_uint_({}) = {}", i, a.get_uint_(i)); }
/// for i in 0..4
///     { println!("a.get_sint_({}) = {}", i, a.get_sint_(i)); }
/// for i in 0..8
///     { println!("a.get_ushort_({}) = {}", i, a.get_ushort_(i)); }
/// for i in 0..8
///     { println!("a.get_sshort_({}) = {}", i, a.get_sshort_(i)); }
/// for i in 0..16
///     { println!("a.get_ubyte_({}) = {}", i, a.get_ubyte_(i)); }
/// for i in 0..16
///     { println!("a.get_sbyte_({}) = {}", i, a.get_sbyte_(i)); }
/// assert_eq!(a.get_ulong_(0), 13664881099896654671_u64);
/// assert_eq!(a.get_ulong_(1), 18379818014235068504_u64);
/// assert_eq!(a.get_slong_(0), -4781862973812896945_i64);
/// assert_eq!(a.get_slong_(1), -66926059474483112_i64);
/// assert_eq!(a.get_uint_(0), 4048161615_u32);
/// assert_eq!(a.get_uint_(1), 3181603061_u32);
/// assert_eq!(a.get_uint_(2), 2127464536_u32);
/// assert_eq!(a.get_uint_(3), 4279384858_u32);
/// assert_eq!(a.get_sint_(0), -246805681_i32);
/// assert_eq!(a.get_sint_(1), -1113364235_i32);
/// assert_eq!(a.get_sint_(2), 2127464536_i32);
/// assert_eq!(a.get_sint_(3), -15582438_i32);
/// assert_eq!(a.get_ushort_(0), 2895_u16);
/// assert_eq!(a.get_ushort_(1), 61770_u16);
/// assert_eq!(a.get_ushort_(2), 26869_u16);
/// assert_eq!(a.get_ushort_(3), 48547_u16);
/// assert_eq!(a.get_ushort_(4), 34904_u16);
/// assert_eq!(a.get_ushort_(5), 32462_u16);
/// assert_eq!(a.get_ushort_(6), 15130_u16);
/// assert_eq!(a.get_ushort_(7), 65298_u16);
/// assert_eq!(a.get_sshort_(0), 2895_i16);
/// assert_eq!(a.get_sshort_(1), -3766_i16);
/// assert_eq!(a.get_sshort_(2), 26869_i16);
/// assert_eq!(a.get_sshort_(3), -16989_i16);
/// assert_eq!(a.get_sshort_(4), -30632_i16);
/// assert_eq!(a.get_sshort_(5), 32462_i16);
/// assert_eq!(a.get_sshort_(6), 15130_i16);
/// assert_eq!(a.get_sshort_(7), -238_i16);
/// assert_eq!(a.get_ubyte_(0), 79_u8);
/// assert_eq!(a.get_ubyte_(1), 11_u8);
/// assert_eq!(a.get_ubyte_(2), 74_u8);
/// assert_eq!(a.get_ubyte_(3), 241_u8);
/// assert_eq!(a.get_ubyte_(4), 245_u8);
/// assert_eq!(a.get_ubyte_(5), 104_u8);
/// assert_eq!(a.get_ubyte_(6), 163_u8);
/// assert_eq!(a.get_ubyte_(7), 189_u8);
/// assert_eq!(a.get_ubyte_(8), 88_u8);
/// assert_eq!(a.get_ubyte_(9), 136_u8);
/// assert_eq!(a.get_ubyte_(10), 206_u8);
/// assert_eq!(a.get_ubyte_(11), 126_u8);
/// assert_eq!(a.get_ubyte_(12), 26_u8);
/// assert_eq!(a.get_ubyte_(13), 59_u8);
/// assert_eq!(a.get_ubyte_(14), 18_u8);
/// assert_eq!(a.get_ubyte_(15), 255_u8);
/// assert_eq!(a.get_sbyte_(0), 79_i8);
/// assert_eq!(a.get_sbyte_(1), 11_i8);
/// assert_eq!(a.get_sbyte_(2), 74_i8);
/// assert_eq!(a.get_sbyte_(3), -15_i8);
/// assert_eq!(a.get_sbyte_(4), -11_i8);
/// assert_eq!(a.get_sbyte_(5), 104_i8);
/// assert_eq!(a.get_sbyte_(6), -93_i8);
/// assert_eq!(a.get_sbyte_(7), -67_i8);
/// assert_eq!(a.get_sbyte_(8), 88_i8);
/// assert_eq!(a.get_sbyte_(9), -120_i8);
/// assert_eq!(a.get_sbyte_(10), -50_i8);
/// assert_eq!(a.get_sbyte_(11), 126_i8);
/// assert_eq!(a.get_sbyte_(12), 26_i8);
/// assert_eq!(a.get_sbyte_(13), 59_i8);
/// assert_eq!(a.get_sbyte_(14), 18_i8);
/// assert_eq!(a.get_sbyte_(15), -1_i8);
/// ```
///  
/// # Big-endian issue
/// It is just experimental for Big Endian CPUs. So, you are not encouraged
/// to use it for serious purpose. Only use this crate for Big-endian CPUs
/// with your own full responsibility.
#[cfg(target_pointer_width = "128")]
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub union SizeUnion
{
    /// The biggest unsigned element for compatibility with other unions
    this: usize,

    /// The biggest signed element for compatibility with other unions
    that: isize,

    /// The usize type element whose size is the same as the SizeUnion
    pub u_size: usize,

    /// The isize type element whose size is the same as the SizeUnion
    pub s_size: isize,

    /// The biggest unsigned element which is 128-bit unsigned integer
    pub ulonger: u128,

    /// The biggest signed element which is 128-bit unsigned integer
    pub slonger: i128,

    /// The secondly biggest unsigned element array whose elements are
    /// 64-bit unsigned integer
    pub ulong: [u64; 2],

    /// The secondly biggest unsigned element array whose elements are
    /// 64-bit signed integer
    pub slong: [i64; 2],

    /// The thirdly biggest unsigned element array whose elements are
    /// 32-bit unsigned integer
    pub uint: [u32; 4],

    /// The thirdly biggest unsigned element array whose elements are
    /// 32-bit signed integer
    pub sint: [i32; 4],

    /// The fourthly biggest unsigned element array whose elements are
    /// 16-bit unsigned integer
    pub ushort: [u16; 8],

    /// The fourthly biggest unsigned element array whose elements are
    /// 16-bit signed integer
    pub sshort: [i16; 8],

    /// The fifthly biggest unsigned element array whose elements are
    /// 8-bit unsigned integer
    pub ubyte: [u8; 16],

    /// The fifthly biggest unsigned element array whose elements are
    /// 8-bit signed integer
    pub sbyte: [i8; 16],
}


/// # Introduction
/// This union `SizeUnion` is for slicing `usize` into two `u32`s, two `i32`s,
/// four `u16`s, four `i16`s, eight `u8`s and/or eight `i8`s.
/// 
/// Sometimes, for example, we need to slice `usize` data into two `u32` pieces
/// which include a higher four-byte word and a lower four-byte word, and/or
/// into  four `u16` pieces which include a first two-byte word, a second
/// two-byte word, a third two-byte word and a fourth two-byte word.
/// In that case, `LongUnion` will be very helpful.
/// 
/// # Quick Start
/// In order to use this union, you have to import (use)
/// `cryptocol::number::SizeUnion` as follows.
/// 
/// ## Example 1
/// ```
/// use cryptocol::number::SizeUnion;
/// ```
/// You can use the methods `get()`, `get_signed()`, `get_ulong()`, and
/// `get_slong()` in order to obtain the data of `u64` in various types.
/// And, you can also slice the data of `usize` into two `u32` type data
/// or two `i32` type data by using the methods `get_uint()`, `get_sint()`,
/// `get_uint_()`, and `get_sint_()`. Or, you can also slice the data of
/// `usize` into four `u16` type data or four `i16` type data by using the
/// methods `get_ushort()`, `get_sshort()`, `get_ushort_()`, and
/// `get_sshort_()`. Or, you can also slice the data of `usize` into eight
/// `u8` type data by using the methods `get_ubyte()`, `get_sbyte()`,
/// `get_ubyte_()`, and `get_sbyte_()`.
/// 
/// ## Example 2
/// ```
/// use cryptocol::number::SizeUnion;
/// 
/// #[cfg(target_pointer_width = "128")]    let a = SizeUnion::new_with_signed(-1234567890987654321012345678987654321_isize);
/// #[cfg(target_pointer_width = "64")]     let a = SizeUnion::new_with_signed(-4781862973812896945_isize);
/// #[cfg(target_pointer_width = "32")]     let a = SizeUnion::new_with_signed(-246805681_isize);
/// #[cfg(target_pointer_width = "16")]     let a = SizeUnion::new_with_signed(2895_isize);
/// #[cfg(target_pointer_width = "8")]      let a = SizeUnion::new_with_signed(79_isize);
/// println!("a.get() = {}", a.get());
/// println!("a.get_signed() = {}", a.get_signed());
/// println!("a.get_usize() = {}", a.get_usize());
/// println!("a.get_ssize() = {}", a.get_ssize());
/// #[cfg(target_pointer_width = "128")]    println!("a.get_ulonger() = {}", a.get_ulonger());
/// #[cfg(target_pointer_width = "128")]    println!("a.get_slonger() = {}", a.get_slonger());
/// #[cfg(target_pointer_width = "64")]     println!("a.get_ulong() = {}", a.get_ulong());
/// #[cfg(target_pointer_width = "64")]     println!("a.get_slong() = {}", a.get_slong());
/// #[cfg(target_pointer_width = "32")]     println!("a.get_uint() = {}", a.get_uint());
/// #[cfg(target_pointer_width = "32")]     println!("a.get_sint() = {}", a.get_sint());
/// #[cfg(target_pointer_width = "16")]     println!("a.get_ushort() = {}", a.get_ushort());
/// #[cfg(target_pointer_width = "16")]     println!("a.get_sshort() = {}", a.get_sshort());
/// #[cfg(target_pointer_width = "128")]
/// {
///     assert_eq!(a.get(), 339047799029950809142362261752780557135_usize);
///     assert_eq!(a.get_signed(), -1234567890987654321012345678987654321_isize);
///     assert_eq!(a.get_usize(), 339047799029950809142362261752780557135_usize);
///     assert_eq!(a.get_ssize(), -1234567890987654321012345678987654321_isize);
/// }
/// #[cfg(target_pointer_width = "64")]
/// {
///     assert_eq!(a.get(), 13664881099896654671_usize);
///     assert_eq!(a.get_signed(), -4781862973812896945_isize);
///     assert_eq!(a.get_usize(), 13664881099896654671_usize);
///     assert_eq!(a.get_ssize(), -4781862973812896945_isize);
/// }
/// #[cfg(target_pointer_width = "32")]
/// {
///     assert_eq!(a.get(), 4048161615_usize);
///     assert_eq!(a.get_signed(), -246805681_isize);
///     assert_eq!(a.get_usize(), 4048161615_usize);
///     assert_eq!(a.get_ssize(), -246805681_isize);
/// }
/// #[cfg(target_pointer_width = "16")]
/// {
///     assert_eq!(a.get(), 2895_usize);
///     assert_eq!(a.get_signed(), 2895_isize);
///     assert_eq!(a.get_usize(), 2895_usize);
///     assert_eq!(a.get_ssize(), 2895_isize);
/// }
/// #[cfg(target_pointer_width = "8")]
/// {
///     assert_eq!(a.get(), 79_usize);
///     assert_eq!(a.get_signed(), 79_isize);
///     assert_eq!(a.get_usize(), 79_usize);
///     assert_eq!(a.get_ssize(), 79_isize);
/// }
/// 
/// #[cfg(target_pointer_width = "128")]    assert_eq!(a.get_ulonger(), 339047799029950809142362261752780557135_u128);
/// #[cfg(target_pointer_width = "128")]    assert_eq!(a.get_slonger(), -1234567890987654321012345678987654321_i128);
/// #[cfg(target_pointer_width = "64")]     assert_eq!(a.get_ulong(), 13664881099896654671_u64);
/// #[cfg(target_pointer_width = "64")]     assert_eq!(a.get_slong(), -4781862973812896945_i64);
/// #[cfg(target_pointer_width = "32")]     assert_eq!(a.get_uint(), 4048161615_u32);
/// #[cfg(target_pointer_width = "32")]     assert_eq!(a.get_sint(), -246805681_i32);
/// #[cfg(target_pointer_width = "16")]     assert_eq!(a.get_ushort(), 2895_u16);
/// #[cfg(target_pointer_width = "16")]     assert_eq!(a.get_sshort(), 2895_i16);
/// #[cfg(target_pointer_width = "8")]      assert_eq!(a.get_ubyte(), 79_u8);
/// #[cfg(target_pointer_width = "8")]      assert_eq!(a.get_sbyte(), 79_u8);
/// 
/// #[cfg(target_pointer_width = "128")]
/// {
///     for i in 0..2
///         { println!("a.get_ulong_({}) = {}", i, a.get_ulong_(i)); }
///     for i in 0..2
///         { println!("a.get_slong_({}) = {}", i, a.get_slong_(i)); }
///     for i in 0..4
///         { println!("a.get_uint_({}) = {}", i, a.get_uint_(i)); }
///     for i in 0..4
///         { println!("a.get_sint_({}) = {}", i, a.get_sint_(i)); }
///     for i in 0..8
///         { println!("a.get_ushort_({}) = {}", i, a.get_ushort_(i)); }
///     for i in 0..8
///         { println!("a.get_sshort_({}) = {}", i, a.get_sshort_(i)); }
///     for i in 0..16
///         { println!("a.get_ubyte_({}) = {}", i, a.get_ubyte_(i)); }
///     for i in 0..16
///         { println!("a.get_sbyte_({}) = {}", i, a.get_sbyte_(i)); }
/// }
/// #[cfg(target_pointer_width = "64")]
/// {
/// for i in 0..2
///     { println!("a.get_uint_({}) = {}", i, a.get_uint_(i)); }
/// for i in 0..2
///     { println!("a.get_sint_({}) = {}", i, a.get_sint_(i)); }
/// for i in 0..4
///     { println!("a.get_ushort_({}) = {}", i, a.get_ushort_(i)); }
/// for i in 0..4
///     { println!("a.get_sshort_({}) = {}", i, a.get_sshort_(i)); }
/// for i in 0..8
///     { println!("a.get_ubyte_({}) = {}", i, a.get_ubyte_(i)); }
/// for i in 0..8
///     { println!("a.get_sbyte_({}) = {}", i, a.get_sbyte_(i)); }
/// }
/// #[cfg(target_pointer_width = "32")]
/// {
///     for i in 0..2
///         { println!("a.get_ushort_({}) = {}", i, a.get_ushort_(i)); }
///     for i in 0..2
///         { println!("a.get_sshort_({}) = {}", i, a.get_sshort_(i)); }
///     for i in 0..4
///         { println!("a.get_ubyte_({}) = {}", i, a.get_ubyte_(i)); }
///     for i in 0..4
///         { println!("a.get_sbyte_({}) = {}", i, a.get_sbyte_(i)); }
/// }
/// #[cfg(target_pointer_width = "16")]
/// {
///     for i in 0..2
///         { println!("a.get_ubyte_({}) = {}", i, a.get_ubyte_(i)); }
///     for i in 0..2
///         { println!("a.get_sbyte_({}) = {}", i, a.get_sbyte_(i)); }
/// }
/// #[cfg(target_pointer_width = "128")]
/// {
///     assert_eq!(a.get_ulong_(0), 13664881099896654671_u64);
///     assert_eq!(a.get_ulong_(1), 18379818014235068504_u64);
///     assert_eq!(a.get_slong_(0), -4781862973812896945_i64);
///     assert_eq!(a.get_slong_(1), -66926059474483112_i64);
///     assert_eq!(a.get_uint_(0), 4048161615_u32);
///     assert_eq!(a.get_uint_(1), 3181603061_u32);
///     assert_eq!(a.get_uint_(2), 2127464536_u32);
///     assert_eq!(a.get_uint_(3), 4279384858_u32);
///     assert_eq!(a.get_sint_(0), -246805681_i32);
///     assert_eq!(a.get_sint_(1), -1113364235_i32);
///     assert_eq!(a.get_sint_(2), 2127464536_i32);
///     assert_eq!(a.get_sint_(3), -15582438_i32);
///     assert_eq!(a.get_ushort_(0), 2895_u16);
///     assert_eq!(a.get_ushort_(1), 61770_u16);
///     assert_eq!(a.get_ushort_(2), 26869_u16);
///     assert_eq!(a.get_ushort_(3), 48547_u16);
///     assert_eq!(a.get_ushort_(4), 34904_u16);
///     assert_eq!(a.get_ushort_(5), 32462_u16);
///     assert_eq!(a.get_ushort_(6), 15130_u16);
///     assert_eq!(a.get_ushort_(7), 65298_u16);
///     assert_eq!(a.get_sshort_(0), 2895_i16);
///     assert_eq!(a.get_sshort_(1), -3766_i16);
///     assert_eq!(a.get_sshort_(2), 26869_i16);
///     assert_eq!(a.get_sshort_(3), -16989_i16);
///     assert_eq!(a.get_sshort_(4), -30632_i16);
///     assert_eq!(a.get_sshort_(5), 32462_i16);
///     assert_eq!(a.get_sshort_(6), 15130_i16);
///     assert_eq!(a.get_sshort_(7), -238_i16);
///     assert_eq!(a.get_ubyte_(0), 79_u8);
///     assert_eq!(a.get_ubyte_(1), 11_u8);
///     assert_eq!(a.get_ubyte_(2), 74_u8);
///     assert_eq!(a.get_ubyte_(3), 241_u8);
///     assert_eq!(a.get_ubyte_(4), 245_u8);
///     assert_eq!(a.get_ubyte_(5), 104_u8);
///     assert_eq!(a.get_ubyte_(6), 163_u8);
///     assert_eq!(a.get_ubyte_(7), 189_u8);
///     assert_eq!(a.get_ubyte_(8), 88_u8);
///     assert_eq!(a.get_ubyte_(9), 136_u8);
///     assert_eq!(a.get_ubyte_(10), 206_u8);
///     assert_eq!(a.get_ubyte_(11), 126_u8);
///     assert_eq!(a.get_ubyte_(12), 26_u8);
///     assert_eq!(a.get_ubyte_(13), 59_u8);
///     assert_eq!(a.get_ubyte_(14), 18_u8);
///     assert_eq!(a.get_ubyte_(15), 255_u8);
///     assert_eq!(a.get_sbyte_(0), 79_i8);
///     assert_eq!(a.get_sbyte_(1), 11_i8);
///     assert_eq!(a.get_sbyte_(2), 74_i8);
///     assert_eq!(a.get_sbyte_(3), -15_i8);
///     assert_eq!(a.get_sbyte_(4), -11_i8);
///     assert_eq!(a.get_sbyte_(5), 104_i8);
///     assert_eq!(a.get_sbyte_(6), -93_i8);
///     assert_eq!(a.get_sbyte_(7), -67_i8);
///     assert_eq!(a.get_sbyte_(8), 88_i8);
///     assert_eq!(a.get_sbyte_(9), -120_i8);
///     assert_eq!(a.get_sbyte_(10), -50_i8);
///     assert_eq!(a.get_sbyte_(11), 126_i8);
///     assert_eq!(a.get_sbyte_(12), 26_i8);
///     assert_eq!(a.get_sbyte_(13), 59_i8);
///     assert_eq!(a.get_sbyte_(14), 18_i8);
///     assert_eq!(a.get_sbyte_(15), -1_i8);
/// }
/// #[cfg(target_pointer_width = "64")]
/// {
///     assert_eq!(a.get_uint_(0), 4048161615_u32);
///     assert_eq!(a.get_uint_(1), 3181603061_u32);
///     assert_eq!(a.get_sint_(0), -246805681_i32);
///     assert_eq!(a.get_sint_(1), -1113364235_i32);
///     assert_eq!(a.get_ushort_(0), 2895_u16);
///     assert_eq!(a.get_ushort_(1), 61770_u16);
///     assert_eq!(a.get_ushort_(2), 26869_u16);
///     assert_eq!(a.get_ushort_(3), 48547_u16);
///     assert_eq!(a.get_sshort_(0), 2895_i16);
///     assert_eq!(a.get_sshort_(1), -3766_i16);
///     assert_eq!(a.get_sshort_(2), 26869_i16);
///     assert_eq!(a.get_sshort_(3), -16989_i16);
///     assert_eq!(a.get_ubyte_(0), 79_u8);
///     assert_eq!(a.get_ubyte_(1), 11_u8);
///     assert_eq!(a.get_ubyte_(2), 74_u8);
///     assert_eq!(a.get_ubyte_(3), 241_u8);
///     assert_eq!(a.get_ubyte_(4), 245_u8);
///     assert_eq!(a.get_ubyte_(5), 104_u8);
///     assert_eq!(a.get_ubyte_(6), 163_u8);
///     assert_eq!(a.get_ubyte_(7), 189_u8);
///     assert_eq!(a.get_sbyte_(0), 79_i8);
///     assert_eq!(a.get_sbyte_(1), 11_i8);
///     assert_eq!(a.get_sbyte_(2), 74_i8);
///     assert_eq!(a.get_sbyte_(3), -15_i8);
///     assert_eq!(a.get_sbyte_(4), -11_i8);
///     assert_eq!(a.get_sbyte_(5), 104_i8);
///     assert_eq!(a.get_sbyte_(6), -93_i8);
///     assert_eq!(a.get_sbyte_(7), -67_i8);
/// }
/// #[cfg(target_pointer_width = "32")]
/// {
///     assert_eq!(a.get_ushort_(0), 2895_u16);
///     assert_eq!(a.get_ushort_(1), 61770_u16);
///     assert_eq!(a.get_sshort_(0), 2895_i16);
///     assert_eq!(a.get_sshort_(1), -3766_i16);
///     assert_eq!(a.get_ubyte_(0), 79_u8);
///     assert_eq!(a.get_ubyte_(1), 11_u8);
///     assert_eq!(a.get_ubyte_(2), 74_u8);
///     assert_eq!(a.get_ubyte_(3), 241_u8);
///     assert_eq!(a.get_sbyte_(0), 79_i8);
///     assert_eq!(a.get_sbyte_(1), 11_i8);
///     assert_eq!(a.get_sbyte_(2), 74_i8);
///     assert_eq!(a.get_sbyte_(3), -15_i8);
/// }
/// #[cfg(target_pointer_width = "16")]
/// {
///     assert_eq!(a.get_ubyte_(0), 79_u8);
///     assert_eq!(a.get_ubyte_(1), 11_u8);
///     assert_eq!(a.get_sbyte_(0), 79_i8);
///     assert_eq!(a.get_sbyte_(1), 11_i8);
/// }
/// ```
/// You can use `ShortUnion` as if you used `u16`. You can perform all kinds of
/// arithmetic operations such as addition, subtraction, multiplication, and
/// division (div and rem), and other operations which are available for
/// `u16`. If you use `ShortUnion` with the help of `SmallUInt`, it will be
/// even more powerful and convenient. In this case, you don't even have to
/// import (use) `cryptocol::number::ShortUnion`.
/// 
/// ## Example 3
/// ```
/// let a_sizeunion = 12345678987654321_u64.into_sizeunion ();
/// let b_sizeunion = 87654321012345678_u64.into_sizeunion ();
/// let c_sizeunion = a_sizeunion.wrapping_add(b_sizeunion );
/// println!("{} + {} = {}", a_sizeunion, b_sizeunion , c_sizeunion );
/// assert_eq!(c_sizeunion .get(), 99999999999999999_u64);
/// for i in 0..2
///     { println!("c_sizeunion .get_uint_({}) = {}", i, c_sizeunion .get_uint_(i)); }
/// assert_eq!(c_sizeunion .get_uint_(0), 1569325055_u32);
/// assert_eq!(c_sizeunion .get_uint_(1), 23283064_u32);
/// for i in 0..4
///     { println!("c_sizeunion .get_ushort_({}) = {}", i, c_sizeunion .get_ushort_(i)); }
/// assert_eq!(c_sizeunion .get_ushort_(0), 65535_u16);
/// assert_eq!(c_sizeunion .get_ushort_(1), 23945_u16);
/// assert_eq!(c_sizeunion .get_ushort_(2), 17784_u16);
/// assert_eq!(c_sizeunion .get_ushort_(3), 355_u16);
/// for i in 0..8
///     { println!("c_sizeunion .get_ubyte_({}) = {}", i, c_sizeunion .get_ubyte_(i)); }
/// assert_eq!(c_sizeunion .get_ubyte_(0), 255_u8);
/// assert_eq!(c_sizeunion .get_ubyte_(1), 255_u8);
/// assert_eq!(c_sizeunion .get_ubyte_(2), 137_u8);
/// assert_eq!(c_sizeunion .get_ubyte_(3), 93_u8);
/// assert_eq!(c_sizeunion .get_ubyte_(4), 120_u8);
/// assert_eq!(c_sizeunion .get_ubyte_(5), 69_u8);
/// assert_eq!(c_sizeunion .get_ubyte_(6), 99_u8);
/// assert_eq!(c_sizeunion .get_ubyte_(7), 1_u8);
/// 
/// let d_sizeunion = b_sizeunion - a_sizeunion ;
/// println!("{} - {} = {}", b_sizeunion , a_sizeunion , d_sizeunion );
/// assert_eq!(d_sizeunion .get(), 75308642024691357_u64);
/// for i in 0..2
///     { println!("d_sizeunion .get_uint_({}) = {}", i, d_sizeunion .get_uint_(i)); }
/// assert_eq!(d_sizeunion .get_uint_(0), 2556827293_u32);
/// assert_eq!(d_sizeunion .get_uint_(1), 17534159_u32);
/// for i in 0..4
///     { println!("d_sizeunion .get_ushort_({}) = {}", i, d_sizeunion .get_ushort_(i)); }
/// assert_eq!(d_sizeunion .get_ushort_(0), 5789_u16);
/// assert_eq!(d_sizeunion .get_ushort_(1), 39014_u16);
/// assert_eq!(d_sizeunion .get_ushort_(2), 36047_u16);
/// assert_eq!(d_sizeunion .get_ushort_(3), 267_u16);
/// for i in 0..8
///     { println!("d_sizeunion .get_ubyte_({}) = {}", i, d_sizeunion .get_ubyte_(i)); }
/// assert_eq!(d_sizeunion .get_ubyte_(0), 157_u8);
/// assert_eq!(d_sizeunion .get_ubyte_(1), 22_u8);
/// assert_eq!(d_sizeunion .get_ubyte_(2), 102_u8);
/// assert_eq!(d_sizeunion .get_ubyte_(3), 152_u8);
/// assert_eq!(d_sizeunion .get_ubyte_(4), 207_u8);
/// assert_eq!(d_sizeunion .get_ubyte_(5), 140_u8);
/// assert_eq!(d_sizeunion .get_ubyte_(6), 11_u8);
/// assert_eq!(d_sizeunion .get_ubyte_(7), 1_u8);
/// 
/// let e_sizeunion = d_sizeunion * 3_usize.into_sizeunion ();
/// println!("{} * {} = {}", d_sizeunion , 3_usize.into_sizeunion (), e_sizeunion );
/// assert_eq!(e_sizeunion .get(), 225925926074074071_usize);
/// 
/// let f_sizeunion = c_sizeunion / 10_usize.into_sizeunion ();
/// println!("{} / {} = {}", c_sizeunion , 10_usize.into_sizeunion (), f_sizeunion );
/// assert_eq!(f_sizeunion .get(), 9999999999999999_usize);
/// 
/// let g_sizeunion = c_sizeunion % 10_usize.into_sizeunion ();
/// println!("{} % {} = {}", c_sizeunion , 10_usize.into_sizeunion(), g_sizeunion );
/// assert_eq!(g_sizeunion .get(), 9_usize);
/// ```
///  
/// # Big-endian issue
/// It is just experimental for Big Endian CPUs. So, you are not encouraged
/// to use it for serious purpose. Only use this crate for Big-endian CPUs
/// with your own full responsibility.
#[cfg(target_pointer_width = "64")]
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub union SizeUnion
{
    /// The biggest unsigned element for compatibility with other unions
    this: usize,

    /// The biggest signed element for compatibility with other unions
    that: isize,

    /// The usize type element whose size is the same as the SizeUnion
    u_size: usize,

    /// The isize type element whose size is the same as the SizeUnion
    s_size: isize,

    /// The biggest unsigned element which is 64-bit unsigned integer
    ulong: u64,

    /// The biggest signed element which is 64-bit unsigned integer
    slong: i64,

    /// The secondly biggest unsigned element array whose elements are
    /// 32-bit unsigned integer
    uint: [u32; 2],

    /// The secondly biggest unsigned element array whose elements are
    /// 32-bit signed integer
    sint: [i32; 2],

    /// The thirdly biggest unsigned element array whose elements are
    /// 16-bit unsigned integer
    ushort: [u16; 4],

    /// The thirdly biggest unsigned element array whose elements are
    /// 16-bit signed integer
    sshort: [i16; 4],

    /// The fourthly biggest unsigned element array whose elements are
    /// 8-bit unsigned integer
    ubyte: [u8; 8],

    /// The fourthly biggest unsigned element array whose elements are
    /// 8-bit signed integer
    sbyte: [i8; 8],
}


/// # Introduction
/// This union `SizeUnion` is for slicing `usize` into two `u32`s, two `i32`s,
/// four `u16`s, four `i16`s, eight `u8`s and/or eight `i8`s.
/// 
/// Sometimes, for example, we need to slice `usize` data into two `u32` pieces
/// which include a higher four-byte word and a lower four-byte word, and/or
/// into  four `u16` pieces which include a first two-byte word, a second
/// two-byte word, a third two-byte word and a fourth two-byte word.
/// In that case, `LongUnion` will be very helpful.
/// 
/// # Quick Start
/// In order to use this union, you have to import (use)
/// `cryptocol::number::SizeUnion` as follows.
/// 
/// ## Example 1
/// ```
/// use cryptocol::number::SizeUnion;
/// ```
/// You can use the methods `get()`, `get_signed()`, `get_ulong()`, and
/// `get_slong()` in order to obtain the data of `u64` in various types.
/// And, you can also slice the data of `usize` into two `u32` type data
/// or two `i32` type data by using the methods `get_uint()`, `get_sint()`,
/// `get_uint_()`, and `get_sint_()`. Or, you can also slice the data of
/// `usize` into four `u16` type data or four `i16` type data by using the
/// methods `get_ushort()`, `get_sshort()`, `get_ushort_()`, and
/// `get_sshort_()`. Or, you can also slice the data of `usize` into eight
/// `u8` type data by using the methods `get_ubyte()`, `get_sbyte()`,
/// `get_ubyte_()`, and `get_sbyte_()`.
/// 
/// ## Example 2
/// ```
/// use cryptocol::number::SizeUnion;
/// 
/// #[cfg(target_pointer_width = "128")]    let a = SizeUnion::new_with_signed(-1234567890987654321012345678987654321_isize);
/// #[cfg(target_pointer_width = "64")]     let a = SizeUnion::new_with_signed(-4781862973812896945_isize);
/// #[cfg(target_pointer_width = "32")]     let a = SizeUnion::new_with_signed(-246805681_isize);
/// #[cfg(target_pointer_width = "16")]     let a = SizeUnion::new_with_signed(2895_isize);
/// #[cfg(target_pointer_width = "8")]      let a = SizeUnion::new_with_signed(79_isize);
/// println!("a.get() = {}", a.get());
/// println!("a.get_signed() = {}", a.get_signed());
/// println!("a.get_usize() = {}", a.get_usize());
/// println!("a.get_ssize() = {}", a.get_ssize());
/// #[cfg(target_pointer_width = "128")]    println!("a.get_ulonger() = {}", a.get_ulonger());
/// #[cfg(target_pointer_width = "128")]    println!("a.get_slonger() = {}", a.get_slonger());
/// #[cfg(target_pointer_width = "64")]     println!("a.get_ulong() = {}", a.get_ulong());
/// #[cfg(target_pointer_width = "64")]     println!("a.get_slong() = {}", a.get_slong());
/// #[cfg(target_pointer_width = "32")]     println!("a.get_uint() = {}", a.get_uint());
/// #[cfg(target_pointer_width = "32")]     println!("a.get_sint() = {}", a.get_sint());
/// #[cfg(target_pointer_width = "16")]     println!("a.get_ushort() = {}", a.get_ushort());
/// #[cfg(target_pointer_width = "16")]     println!("a.get_sshort() = {}", a.get_sshort());
/// #[cfg(target_pointer_width = "128")]
/// {
///     assert_eq!(a.get(), 339047799029950809142362261752780557135_usize);
///     assert_eq!(a.get_signed(), -1234567890987654321012345678987654321_isize);
///     assert_eq!(a.get_usize(), 339047799029950809142362261752780557135_usize);
///     assert_eq!(a.get_ssize(), -1234567890987654321012345678987654321_isize);
/// }
/// #[cfg(target_pointer_width = "64")]
/// {
///     assert_eq!(a.get(), 13664881099896654671_usize);
///     assert_eq!(a.get_signed(), -4781862973812896945_isize);
///     assert_eq!(a.get_usize(), 13664881099896654671_usize);
///     assert_eq!(a.get_ssize(), -4781862973812896945_isize);
/// }
/// #[cfg(target_pointer_width = "32")]
/// {
///     assert_eq!(a.get(), 4048161615_usize);
///     assert_eq!(a.get_signed(), -246805681_isize);
///     assert_eq!(a.get_usize(), 4048161615_usize);
///     assert_eq!(a.get_ssize(), -246805681_isize);
/// }
/// #[cfg(target_pointer_width = "16")]
/// {
///     assert_eq!(a.get(), 2895_usize);
///     assert_eq!(a.get_signed(), 2895_isize);
///     assert_eq!(a.get_usize(), 2895_usize);
///     assert_eq!(a.get_ssize(), 2895_isize);
/// }
/// #[cfg(target_pointer_width = "8")]
/// {
///     assert_eq!(a.get(), 79_usize);
///     assert_eq!(a.get_signed(), 79_isize);
///     assert_eq!(a.get_usize(), 79_usize);
///     assert_eq!(a.get_ssize(), 79_isize);
/// }
/// 
/// #[cfg(target_pointer_width = "128")]    assert_eq!(a.get_ulonger(), 339047799029950809142362261752780557135_u128);
/// #[cfg(target_pointer_width = "128")]    assert_eq!(a.get_slonger(), -1234567890987654321012345678987654321_i128);
/// #[cfg(target_pointer_width = "64")]     assert_eq!(a.get_ulong(), 13664881099896654671_u64);
/// #[cfg(target_pointer_width = "64")]     assert_eq!(a.get_slong(), -4781862973812896945_i64);
/// #[cfg(target_pointer_width = "32")]     assert_eq!(a.get_uint(), 4048161615_u32);
/// #[cfg(target_pointer_width = "32")]     assert_eq!(a.get_sint(), -246805681_i32);
/// #[cfg(target_pointer_width = "16")]     assert_eq!(a.get_ushort(), 2895_u16);
/// #[cfg(target_pointer_width = "16")]     assert_eq!(a.get_sshort(), 2895_i16);
/// #[cfg(target_pointer_width = "8")]      assert_eq!(a.get_ubyte(), 79_u8);
/// #[cfg(target_pointer_width = "8")]      assert_eq!(a.get_sbyte(), 79_u8);
/// 
/// #[cfg(target_pointer_width = "128")]
/// {
///     for i in 0..2
///         { println!("a.get_ulong_({}) = {}", i, a.get_ulong_(i)); }
///     for i in 0..2
///         { println!("a.get_slong_({}) = {}", i, a.get_slong_(i)); }
///     for i in 0..4
///         { println!("a.get_uint_({}) = {}", i, a.get_uint_(i)); }
///     for i in 0..4
///         { println!("a.get_sint_({}) = {}", i, a.get_sint_(i)); }
///     for i in 0..8
///         { println!("a.get_ushort_({}) = {}", i, a.get_ushort_(i)); }
///     for i in 0..8
///         { println!("a.get_sshort_({}) = {}", i, a.get_sshort_(i)); }
///     for i in 0..16
///         { println!("a.get_ubyte_({}) = {}", i, a.get_ubyte_(i)); }
///     for i in 0..16
///         { println!("a.get_sbyte_({}) = {}", i, a.get_sbyte_(i)); }
/// }
/// #[cfg(target_pointer_width = "64")]
/// {
/// for i in 0..2
///     { println!("a.get_uint_({}) = {}", i, a.get_uint_(i)); }
/// for i in 0..2
///     { println!("a.get_sint_({}) = {}", i, a.get_sint_(i)); }
/// for i in 0..4
///     { println!("a.get_ushort_({}) = {}", i, a.get_ushort_(i)); }
/// for i in 0..4
///     { println!("a.get_sshort_({}) = {}", i, a.get_sshort_(i)); }
/// for i in 0..8
///     { println!("a.get_ubyte_({}) = {}", i, a.get_ubyte_(i)); }
/// for i in 0..8
///     { println!("a.get_sbyte_({}) = {}", i, a.get_sbyte_(i)); }
/// }
/// #[cfg(target_pointer_width = "32")]
/// {
///     for i in 0..2
///         { println!("a.get_ushort_({}) = {}", i, a.get_ushort_(i)); }
///     for i in 0..2
///         { println!("a.get_sshort_({}) = {}", i, a.get_sshort_(i)); }
///     for i in 0..4
///         { println!("a.get_ubyte_({}) = {}", i, a.get_ubyte_(i)); }
///     for i in 0..4
///         { println!("a.get_sbyte_({}) = {}", i, a.get_sbyte_(i)); }
/// }
/// #[cfg(target_pointer_width = "16")]
/// {
///     for i in 0..2
///         { println!("a.get_ubyte_({}) = {}", i, a.get_ubyte_(i)); }
///     for i in 0..2
///         { println!("a.get_sbyte_({}) = {}", i, a.get_sbyte_(i)); }
/// }
/// #[cfg(target_pointer_width = "128")]
/// {
///     assert_eq!(a.get_ulong_(0), 13664881099896654671_u64);
///     assert_eq!(a.get_ulong_(1), 18379818014235068504_u64);
///     assert_eq!(a.get_slong_(0), -4781862973812896945_i64);
///     assert_eq!(a.get_slong_(1), -66926059474483112_i64);
///     assert_eq!(a.get_uint_(0), 4048161615_u32);
///     assert_eq!(a.get_uint_(1), 3181603061_u32);
///     assert_eq!(a.get_uint_(2), 2127464536_u32);
///     assert_eq!(a.get_uint_(3), 4279384858_u32);
///     assert_eq!(a.get_sint_(0), -246805681_i32);
///     assert_eq!(a.get_sint_(1), -1113364235_i32);
///     assert_eq!(a.get_sint_(2), 2127464536_i32);
///     assert_eq!(a.get_sint_(3), -15582438_i32);
///     assert_eq!(a.get_ushort_(0), 2895_u16);
///     assert_eq!(a.get_ushort_(1), 61770_u16);
///     assert_eq!(a.get_ushort_(2), 26869_u16);
///     assert_eq!(a.get_ushort_(3), 48547_u16);
///     assert_eq!(a.get_ushort_(4), 34904_u16);
///     assert_eq!(a.get_ushort_(5), 32462_u16);
///     assert_eq!(a.get_ushort_(6), 15130_u16);
///     assert_eq!(a.get_ushort_(7), 65298_u16);
///     assert_eq!(a.get_sshort_(0), 2895_i16);
///     assert_eq!(a.get_sshort_(1), -3766_i16);
///     assert_eq!(a.get_sshort_(2), 26869_i16);
///     assert_eq!(a.get_sshort_(3), -16989_i16);
///     assert_eq!(a.get_sshort_(4), -30632_i16);
///     assert_eq!(a.get_sshort_(5), 32462_i16);
///     assert_eq!(a.get_sshort_(6), 15130_i16);
///     assert_eq!(a.get_sshort_(7), -238_i16);
///     assert_eq!(a.get_ubyte_(0), 79_u8);
///     assert_eq!(a.get_ubyte_(1), 11_u8);
///     assert_eq!(a.get_ubyte_(2), 74_u8);
///     assert_eq!(a.get_ubyte_(3), 241_u8);
///     assert_eq!(a.get_ubyte_(4), 245_u8);
///     assert_eq!(a.get_ubyte_(5), 104_u8);
///     assert_eq!(a.get_ubyte_(6), 163_u8);
///     assert_eq!(a.get_ubyte_(7), 189_u8);
///     assert_eq!(a.get_ubyte_(8), 88_u8);
///     assert_eq!(a.get_ubyte_(9), 136_u8);
///     assert_eq!(a.get_ubyte_(10), 206_u8);
///     assert_eq!(a.get_ubyte_(11), 126_u8);
///     assert_eq!(a.get_ubyte_(12), 26_u8);
///     assert_eq!(a.get_ubyte_(13), 59_u8);
///     assert_eq!(a.get_ubyte_(14), 18_u8);
///     assert_eq!(a.get_ubyte_(15), 255_u8);
///     assert_eq!(a.get_sbyte_(0), 79_i8);
///     assert_eq!(a.get_sbyte_(1), 11_i8);
///     assert_eq!(a.get_sbyte_(2), 74_i8);
///     assert_eq!(a.get_sbyte_(3), -15_i8);
///     assert_eq!(a.get_sbyte_(4), -11_i8);
///     assert_eq!(a.get_sbyte_(5), 104_i8);
///     assert_eq!(a.get_sbyte_(6), -93_i8);
///     assert_eq!(a.get_sbyte_(7), -67_i8);
///     assert_eq!(a.get_sbyte_(8), 88_i8);
///     assert_eq!(a.get_sbyte_(9), -120_i8);
///     assert_eq!(a.get_sbyte_(10), -50_i8);
///     assert_eq!(a.get_sbyte_(11), 126_i8);
///     assert_eq!(a.get_sbyte_(12), 26_i8);
///     assert_eq!(a.get_sbyte_(13), 59_i8);
///     assert_eq!(a.get_sbyte_(14), 18_i8);
///     assert_eq!(a.get_sbyte_(15), -1_i8);
/// }
/// #[cfg(target_pointer_width = "64")]
/// {
///     assert_eq!(a.get_uint_(0), 4048161615_u32);
///     assert_eq!(a.get_uint_(1), 3181603061_u32);
///     assert_eq!(a.get_sint_(0), -246805681_i32);
///     assert_eq!(a.get_sint_(1), -1113364235_i32);
///     assert_eq!(a.get_ushort_(0), 2895_u16);
///     assert_eq!(a.get_ushort_(1), 61770_u16);
///     assert_eq!(a.get_ushort_(2), 26869_u16);
///     assert_eq!(a.get_ushort_(3), 48547_u16);
///     assert_eq!(a.get_sshort_(0), 2895_i16);
///     assert_eq!(a.get_sshort_(1), -3766_i16);
///     assert_eq!(a.get_sshort_(2), 26869_i16);
///     assert_eq!(a.get_sshort_(3), -16989_i16);
///     assert_eq!(a.get_ubyte_(0), 79_u8);
///     assert_eq!(a.get_ubyte_(1), 11_u8);
///     assert_eq!(a.get_ubyte_(2), 74_u8);
///     assert_eq!(a.get_ubyte_(3), 241_u8);
///     assert_eq!(a.get_ubyte_(4), 245_u8);
///     assert_eq!(a.get_ubyte_(5), 104_u8);
///     assert_eq!(a.get_ubyte_(6), 163_u8);
///     assert_eq!(a.get_ubyte_(7), 189_u8);
///     assert_eq!(a.get_sbyte_(0), 79_i8);
///     assert_eq!(a.get_sbyte_(1), 11_i8);
///     assert_eq!(a.get_sbyte_(2), 74_i8);
///     assert_eq!(a.get_sbyte_(3), -15_i8);
///     assert_eq!(a.get_sbyte_(4), -11_i8);
///     assert_eq!(a.get_sbyte_(5), 104_i8);
///     assert_eq!(a.get_sbyte_(6), -93_i8);
///     assert_eq!(a.get_sbyte_(7), -67_i8);
/// }
/// #[cfg(target_pointer_width = "32")]
/// {
///     assert_eq!(a.get_ushort_(0), 2895_u16);
///     assert_eq!(a.get_ushort_(1), 61770_u16);
///     assert_eq!(a.get_sshort_(0), 2895_i16);
///     assert_eq!(a.get_sshort_(1), -3766_i16);
///     assert_eq!(a.get_ubyte_(0), 79_u8);
///     assert_eq!(a.get_ubyte_(1), 11_u8);
///     assert_eq!(a.get_ubyte_(2), 74_u8);
///     assert_eq!(a.get_ubyte_(3), 241_u8);
///     assert_eq!(a.get_sbyte_(0), 79_i8);
///     assert_eq!(a.get_sbyte_(1), 11_i8);
///     assert_eq!(a.get_sbyte_(2), 74_i8);
///     assert_eq!(a.get_sbyte_(3), -15_i8);
/// }
/// #[cfg(target_pointer_width = "16")]
/// {
///     assert_eq!(a.get_ubyte_(0), 79_u8);
///     assert_eq!(a.get_ubyte_(1), 11_u8);
///     assert_eq!(a.get_sbyte_(0), 79_i8);
///     assert_eq!(a.get_sbyte_(1), 11_i8);
/// }
/// ```
/// 
/// # Big-endian issue
/// It is just experimental for Big Endian CPUs. So, you are not encouraged
/// to use it for serious purpose. Only use this crate for Big-endian CPUs
/// with your own full responsibility.
#[cfg(target_pointer_width = "32")]
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub union SizeUnion
{
    /// The biggest unsigned element for compatibility with other unions
    this: usize,

    /// The biggest signed element for compatibility with other unions
    that: isize,

    /// The usize type element whose size is the same as the SizeUnion
    pub u_size: usize,

    /// The isize type element whose size is the same as the SizeUnion
    pub s_size: isize,

    /// The biggest unsigned element which is 32-bit unsigned integer
    pub uint: u32,

    /// The biggest signed element which is 32-bit unsigned integer
    pub sint: i32,

    /// The secondly biggest unsigned element array whose elements are
    /// 16-bit unsigned integer
    pub ushort: [u16; 2],

    /// The secondly biggest unsigned element array whose elements are
    /// 16-bit signed integer
    pub sshort: [i16; 2],

    /// The thirdly biggest unsigned element array whose elements are
    /// 8-bit unsigned integer
    pub ubyte: [u8; 4],

    /// The thirdly biggest unsigned element array whose elements are
    /// 8-bit signed integer
    pub sbyte: [i8; 4],
}


/// # Introduction
/// This union `SizeUnion` is for slicing `usize` into two `u32`s, two `i32`s,
/// four `u16`s, four `i16`s, eight `u8`s and/or eight `i8`s.
/// 
/// Sometimes, for example, we need to slice `usize` data into two `u32` pieces
/// which include a higher four-byte word and a lower four-byte word, and/or
/// into  four `u16` pieces which include a first two-byte word, a second
/// two-byte word, a third two-byte word and a fourth two-byte word.
/// In that case, `LongUnion` will be very helpful.
/// 
/// # Quick Start
/// In order to use this union, you have to import (use)
/// `cryptocol::number::SizeUnion` as follows.
/// 
/// ## Example 1
/// ```
/// use cryptocol::number::SizeUnion;
/// ```
/// You can use the methods `get()`, `get_signed()`, `get_ulong()`, and
/// `get_slong()` in order to obtain the data of `u64` in various types.
/// And, you can also slice the data of `usize` into two `u32` type data
/// or two `i32` type data by using the methods `get_uint()`, `get_sint()`,
/// `get_uint_()`, and `get_sint_()`. Or, you can also slice the data of
/// `usize` into four `u16` type data or four `i16` type data by using the
/// methods `get_ushort()`, `get_sshort()`, `get_ushort_()`, and
/// `get_sshort_()`. Or, you can also slice the data of `usize` into eight
/// `u8` type data by using the methods `get_ubyte()`, `get_sbyte()`,
/// `get_ubyte_()`, and `get_sbyte_()`.
/// 
/// ## Example 2
/// ```
/// use cryptocol::number::SizeUnion;
/// 
/// #[cfg(target_pointer_width = "128")]    let a = SizeUnion::new_with_signed(-1234567890987654321012345678987654321_isize);
/// #[cfg(target_pointer_width = "64")]     let a = SizeUnion::new_with_signed(-4781862973812896945_isize);
/// #[cfg(target_pointer_width = "32")]     let a = SizeUnion::new_with_signed(-246805681_isize);
/// #[cfg(target_pointer_width = "16")]     let a = SizeUnion::new_with_signed(2895_isize);
/// #[cfg(target_pointer_width = "8")]      let a = SizeUnion::new_with_signed(79_isize);
/// println!("a.get() = {}", a.get());
/// println!("a.get_signed() = {}", a.get_signed());
/// println!("a.get_usize() = {}", a.get_usize());
/// println!("a.get_ssize() = {}", a.get_ssize());
/// #[cfg(target_pointer_width = "128")]    println!("a.get_ulonger() = {}", a.get_ulonger());
/// #[cfg(target_pointer_width = "128")]    println!("a.get_slonger() = {}", a.get_slonger());
/// #[cfg(target_pointer_width = "64")]     println!("a.get_ulong() = {}", a.get_ulong());
/// #[cfg(target_pointer_width = "64")]     println!("a.get_slong() = {}", a.get_slong());
/// #[cfg(target_pointer_width = "32")]     println!("a.get_uint() = {}", a.get_uint());
/// #[cfg(target_pointer_width = "32")]     println!("a.get_sint() = {}", a.get_sint());
/// #[cfg(target_pointer_width = "16")]     println!("a.get_ushort() = {}", a.get_ushort());
/// #[cfg(target_pointer_width = "16")]     println!("a.get_sshort() = {}", a.get_sshort());
/// #[cfg(target_pointer_width = "128")]
/// {
///     assert_eq!(a.get(), 339047799029950809142362261752780557135_usize);
///     assert_eq!(a.get_signed(), -1234567890987654321012345678987654321_isize);
///     assert_eq!(a.get_usize(), 339047799029950809142362261752780557135_usize);
///     assert_eq!(a.get_ssize(), -1234567890987654321012345678987654321_isize);
/// }
/// #[cfg(target_pointer_width = "64")]
/// {
///     assert_eq!(a.get(), 13664881099896654671_usize);
///     assert_eq!(a.get_signed(), -4781862973812896945_isize);
///     assert_eq!(a.get_usize(), 13664881099896654671_usize);
///     assert_eq!(a.get_ssize(), -4781862973812896945_isize);
/// }
/// #[cfg(target_pointer_width = "32")]
/// {
///     assert_eq!(a.get(), 4048161615_usize);
///     assert_eq!(a.get_signed(), -246805681_isize);
///     assert_eq!(a.get_usize(), 4048161615_usize);
///     assert_eq!(a.get_ssize(), -246805681_isize);
/// }
/// #[cfg(target_pointer_width = "16")]
/// {
///     assert_eq!(a.get(), 2895_usize);
///     assert_eq!(a.get_signed(), 2895_isize);
///     assert_eq!(a.get_usize(), 2895_usize);
///     assert_eq!(a.get_ssize(), 2895_isize);
/// }
/// #[cfg(target_pointer_width = "8")]
/// {
///     assert_eq!(a.get(), 79_usize);
///     assert_eq!(a.get_signed(), 79_isize);
///     assert_eq!(a.get_usize(), 79_usize);
///     assert_eq!(a.get_ssize(), 79_isize);
/// }
/// 
/// #[cfg(target_pointer_width = "128")]    assert_eq!(a.get_ulonger(), 339047799029950809142362261752780557135_u128);
/// #[cfg(target_pointer_width = "128")]    assert_eq!(a.get_slonger(), -1234567890987654321012345678987654321_i128);
/// #[cfg(target_pointer_width = "64")]     assert_eq!(a.get_ulong(), 13664881099896654671_u64);
/// #[cfg(target_pointer_width = "64")]     assert_eq!(a.get_slong(), -4781862973812896945_i64);
/// #[cfg(target_pointer_width = "32")]     assert_eq!(a.get_uint(), 4048161615_u32);
/// #[cfg(target_pointer_width = "32")]     assert_eq!(a.get_sint(), -246805681_i32);
/// #[cfg(target_pointer_width = "16")]     assert_eq!(a.get_ushort(), 2895_u16);
/// #[cfg(target_pointer_width = "16")]     assert_eq!(a.get_sshort(), 2895_i16);
/// #[cfg(target_pointer_width = "8")]      assert_eq!(a.get_ubyte(), 79_u8);
/// #[cfg(target_pointer_width = "8")]      assert_eq!(a.get_sbyte(), 79_u8);
/// 
/// #[cfg(target_pointer_width = "128")]
/// {
///     for i in 0..2
///         { println!("a.get_ulong_({}) = {}", i, a.get_ulong_(i)); }
///     for i in 0..2
///         { println!("a.get_slong_({}) = {}", i, a.get_slong_(i)); }
///     for i in 0..4
///         { println!("a.get_uint_({}) = {}", i, a.get_uint_(i)); }
///     for i in 0..4
///         { println!("a.get_sint_({}) = {}", i, a.get_sint_(i)); }
///     for i in 0..8
///         { println!("a.get_ushort_({}) = {}", i, a.get_ushort_(i)); }
///     for i in 0..8
///         { println!("a.get_sshort_({}) = {}", i, a.get_sshort_(i)); }
///     for i in 0..16
///         { println!("a.get_ubyte_({}) = {}", i, a.get_ubyte_(i)); }
///     for i in 0..16
///         { println!("a.get_sbyte_({}) = {}", i, a.get_sbyte_(i)); }
/// }
/// #[cfg(target_pointer_width = "64")]
/// {
/// for i in 0..2
///     { println!("a.get_uint_({}) = {}", i, a.get_uint_(i)); }
/// for i in 0..2
///     { println!("a.get_sint_({}) = {}", i, a.get_sint_(i)); }
/// for i in 0..4
///     { println!("a.get_ushort_({}) = {}", i, a.get_ushort_(i)); }
/// for i in 0..4
///     { println!("a.get_sshort_({}) = {}", i, a.get_sshort_(i)); }
/// for i in 0..8
///     { println!("a.get_ubyte_({}) = {}", i, a.get_ubyte_(i)); }
/// for i in 0..8
///     { println!("a.get_sbyte_({}) = {}", i, a.get_sbyte_(i)); }
/// }
/// #[cfg(target_pointer_width = "32")]
/// {
///     for i in 0..2
///         { println!("a.get_ushort_({}) = {}", i, a.get_ushort_(i)); }
///     for i in 0..2
///         { println!("a.get_sshort_({}) = {}", i, a.get_sshort_(i)); }
///     for i in 0..4
///         { println!("a.get_ubyte_({}) = {}", i, a.get_ubyte_(i)); }
///     for i in 0..4
///         { println!("a.get_sbyte_({}) = {}", i, a.get_sbyte_(i)); }
/// }
/// #[cfg(target_pointer_width = "16")]
/// {
///     for i in 0..2
///         { println!("a.get_ubyte_({}) = {}", i, a.get_ubyte_(i)); }
///     for i in 0..2
///         { println!("a.get_sbyte_({}) = {}", i, a.get_sbyte_(i)); }
/// }
/// #[cfg(target_pointer_width = "128")]
/// {
///     assert_eq!(a.get_ulong_(0), 13664881099896654671_u64);
///     assert_eq!(a.get_ulong_(1), 18379818014235068504_u64);
///     assert_eq!(a.get_slong_(0), -4781862973812896945_i64);
///     assert_eq!(a.get_slong_(1), -66926059474483112_i64);
///     assert_eq!(a.get_uint_(0), 4048161615_u32);
///     assert_eq!(a.get_uint_(1), 3181603061_u32);
///     assert_eq!(a.get_uint_(2), 2127464536_u32);
///     assert_eq!(a.get_uint_(3), 4279384858_u32);
///     assert_eq!(a.get_sint_(0), -246805681_i32);
///     assert_eq!(a.get_sint_(1), -1113364235_i32);
///     assert_eq!(a.get_sint_(2), 2127464536_i32);
///     assert_eq!(a.get_sint_(3), -15582438_i32);
///     assert_eq!(a.get_ushort_(0), 2895_u16);
///     assert_eq!(a.get_ushort_(1), 61770_u16);
///     assert_eq!(a.get_ushort_(2), 26869_u16);
///     assert_eq!(a.get_ushort_(3), 48547_u16);
///     assert_eq!(a.get_ushort_(4), 34904_u16);
///     assert_eq!(a.get_ushort_(5), 32462_u16);
///     assert_eq!(a.get_ushort_(6), 15130_u16);
///     assert_eq!(a.get_ushort_(7), 65298_u16);
///     assert_eq!(a.get_sshort_(0), 2895_i16);
///     assert_eq!(a.get_sshort_(1), -3766_i16);
///     assert_eq!(a.get_sshort_(2), 26869_i16);
///     assert_eq!(a.get_sshort_(3), -16989_i16);
///     assert_eq!(a.get_sshort_(4), -30632_i16);
///     assert_eq!(a.get_sshort_(5), 32462_i16);
///     assert_eq!(a.get_sshort_(6), 15130_i16);
///     assert_eq!(a.get_sshort_(7), -238_i16);
///     assert_eq!(a.get_ubyte_(0), 79_u8);
///     assert_eq!(a.get_ubyte_(1), 11_u8);
///     assert_eq!(a.get_ubyte_(2), 74_u8);
///     assert_eq!(a.get_ubyte_(3), 241_u8);
///     assert_eq!(a.get_ubyte_(4), 245_u8);
///     assert_eq!(a.get_ubyte_(5), 104_u8);
///     assert_eq!(a.get_ubyte_(6), 163_u8);
///     assert_eq!(a.get_ubyte_(7), 189_u8);
///     assert_eq!(a.get_ubyte_(8), 88_u8);
///     assert_eq!(a.get_ubyte_(9), 136_u8);
///     assert_eq!(a.get_ubyte_(10), 206_u8);
///     assert_eq!(a.get_ubyte_(11), 126_u8);
///     assert_eq!(a.get_ubyte_(12), 26_u8);
///     assert_eq!(a.get_ubyte_(13), 59_u8);
///     assert_eq!(a.get_ubyte_(14), 18_u8);
///     assert_eq!(a.get_ubyte_(15), 255_u8);
///     assert_eq!(a.get_sbyte_(0), 79_i8);
///     assert_eq!(a.get_sbyte_(1), 11_i8);
///     assert_eq!(a.get_sbyte_(2), 74_i8);
///     assert_eq!(a.get_sbyte_(3), -15_i8);
///     assert_eq!(a.get_sbyte_(4), -11_i8);
///     assert_eq!(a.get_sbyte_(5), 104_i8);
///     assert_eq!(a.get_sbyte_(6), -93_i8);
///     assert_eq!(a.get_sbyte_(7), -67_i8);
///     assert_eq!(a.get_sbyte_(8), 88_i8);
///     assert_eq!(a.get_sbyte_(9), -120_i8);
///     assert_eq!(a.get_sbyte_(10), -50_i8);
///     assert_eq!(a.get_sbyte_(11), 126_i8);
///     assert_eq!(a.get_sbyte_(12), 26_i8);
///     assert_eq!(a.get_sbyte_(13), 59_i8);
///     assert_eq!(a.get_sbyte_(14), 18_i8);
///     assert_eq!(a.get_sbyte_(15), -1_i8);
/// }
/// #[cfg(target_pointer_width = "64")]
/// {
///     assert_eq!(a.get_uint_(0), 4048161615_u32);
///     assert_eq!(a.get_uint_(1), 3181603061_u32);
///     assert_eq!(a.get_sint_(0), -246805681_i32);
///     assert_eq!(a.get_sint_(1), -1113364235_i32);
///     assert_eq!(a.get_ushort_(0), 2895_u16);
///     assert_eq!(a.get_ushort_(1), 61770_u16);
///     assert_eq!(a.get_ushort_(2), 26869_u16);
///     assert_eq!(a.get_ushort_(3), 48547_u16);
///     assert_eq!(a.get_sshort_(0), 2895_i16);
///     assert_eq!(a.get_sshort_(1), -3766_i16);
///     assert_eq!(a.get_sshort_(2), 26869_i16);
///     assert_eq!(a.get_sshort_(3), -16989_i16);
///     assert_eq!(a.get_ubyte_(0), 79_u8);
///     assert_eq!(a.get_ubyte_(1), 11_u8);
///     assert_eq!(a.get_ubyte_(2), 74_u8);
///     assert_eq!(a.get_ubyte_(3), 241_u8);
///     assert_eq!(a.get_ubyte_(4), 245_u8);
///     assert_eq!(a.get_ubyte_(5), 104_u8);
///     assert_eq!(a.get_ubyte_(6), 163_u8);
///     assert_eq!(a.get_ubyte_(7), 189_u8);
///     assert_eq!(a.get_sbyte_(0), 79_i8);
///     assert_eq!(a.get_sbyte_(1), 11_i8);
///     assert_eq!(a.get_sbyte_(2), 74_i8);
///     assert_eq!(a.get_sbyte_(3), -15_i8);
///     assert_eq!(a.get_sbyte_(4), -11_i8);
///     assert_eq!(a.get_sbyte_(5), 104_i8);
///     assert_eq!(a.get_sbyte_(6), -93_i8);
///     assert_eq!(a.get_sbyte_(7), -67_i8);
/// }
/// #[cfg(target_pointer_width = "32")]
/// {
///     assert_eq!(a.get_ushort_(0), 2895_u16);
///     assert_eq!(a.get_ushort_(1), 61770_u16);
///     assert_eq!(a.get_sshort_(0), 2895_i16);
///     assert_eq!(a.get_sshort_(1), -3766_i16);
///     assert_eq!(a.get_ubyte_(0), 79_u8);
///     assert_eq!(a.get_ubyte_(1), 11_u8);
///     assert_eq!(a.get_ubyte_(2), 74_u8);
///     assert_eq!(a.get_ubyte_(3), 241_u8);
///     assert_eq!(a.get_sbyte_(0), 79_i8);
///     assert_eq!(a.get_sbyte_(1), 11_i8);
///     assert_eq!(a.get_sbyte_(2), 74_i8);
///     assert_eq!(a.get_sbyte_(3), -15_i8);
/// }
/// #[cfg(target_pointer_width = "16")]
/// {
///     assert_eq!(a.get_ubyte_(0), 79_u8);
///     assert_eq!(a.get_ubyte_(1), 11_u8);
///     assert_eq!(a.get_sbyte_(0), 79_i8);
///     assert_eq!(a.get_sbyte_(1), 11_i8);
/// }
/// ```
/// 
/// # Big-endian issue
/// It is just experimental for Big Endian CPUs. So, you are not encouraged
/// to use it for serious purpose. Only use this crate for Big-endian CPUs
/// with your own full responsibility.
#[cfg(target_pointer_width = "16")]
#[derive(Copy, Clone)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub union SizeUnion
{
    /// The biggest unsigned element for compatibility with other unions
    this: usize,

    /// The biggest signed element for compatibility with other unions
    pub that: isize,

    /// The usize type element whose size is the same as the SizeUnion
    pub u_size: usize,

    /// The isize type element whose size is the same as the SizeUnion
    pub s_size: isize,

    /// The biggest unsigned element which is 16-bit unsigned integer
    pub ushort: u16,

    /// The biggest signed element which is 16-bit unsigned integer
    pub sshort: i16,

    /// The secondly biggest unsigned element array whose elements are
    /// 8-bit unsigned integer
    pub ubyte: [u8; 2],

    /// The secondly biggest signed element array whose elements are
    /// 8-bit unsigned integer
    pub sbyte: [i8; 2],
}


/// # Introduction
/// This union `SizeUnion` is for slicing `usize` into two `u32`s, two `i32`s,
/// four `u16`s, four `i16`s, eight `u8`s and/or eight `i8`s.
/// 
/// Sometimes, for example, we need to slice `usize` data into two `u32` pieces
/// which include a higher four-byte word and a lower four-byte word, and/or
/// into  four `u16` pieces which include a first two-byte word, a second
/// two-byte word, a third two-byte word and a fourth two-byte word.
/// In that case, `LongUnion` will be very helpful.
/// 
/// # Quick Start
/// In order to use this union, you have to import (use)
/// `cryptocol::number::SizeUnion` as follows.
/// 
/// ## Example 1
/// ```
/// use cryptocol::number::SizeUnion;
/// ```
/// You can use the methods `get()`, `get_signed()`, `get_ulong()`, and
/// `get_slong()` in order to obtain the data of `u64` in various types.
/// And, you can also slice the data of `usize` into two `u32` type data
/// or two `i32` type data by using the methods `get_uint()`, `get_sint()`,
/// `get_uint_()`, and `get_sint_()`. Or, you can also slice the data of
/// `usize` into four `u16` type data or four `i16` type data by using the
/// methods `get_ushort()`, `get_sshort()`, `get_ushort_()`, and
/// `get_sshort_()`. Or, you can also slice the data of `usize` into eight
/// `u8` type data by using the methods `get_ubyte()`, `get_sbyte()`,
/// `get_ubyte_()`, and `get_sbyte_()`.
/// 
/// ## Example 2
/// ```
/// use cryptocol::number::SizeUnion;
/// 
/// #[cfg(target_pointer_width = "128")]    let a = SizeUnion::new_with_signed(-1234567890987654321012345678987654321_isize);
/// #[cfg(target_pointer_width = "64")]     let a = SizeUnion::new_with_signed(-4781862973812896945_isize);
/// #[cfg(target_pointer_width = "32")]     let a = SizeUnion::new_with_signed(-246805681_isize);
/// #[cfg(target_pointer_width = "16")]     let a = SizeUnion::new_with_signed(2895_isize);
/// #[cfg(target_pointer_width = "8")]      let a = SizeUnion::new_with_signed(79_isize);
/// #[cfg(target_pointer_width = "8")]
/// {
///     assert_eq!(a.get(), 79_usize);
///     assert_eq!(a.get_signed(), 79_isize);
///     assert_eq!(a.get_usize(), 79_usize);
///     assert_eq!(a.get_ssize(), 79_isize);
/// }
/// 
/// #[cfg(target_pointer_width = "8")]      assert_eq!(a.get_ubyte(), 79_u8);
/// #[cfg(target_pointer_width = "8")]      assert_eq!(a.get_sbyte(), 79_u8);
/// ```
#[cfg(target_pointer_width = "8")]
#[derive(Copy, Clone)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub union SizeUnion
{
    /// The biggest unsigned element for compatibility with other unions
    pub this: usize,

    /// The biggest signed element for compatibility with other unions
    pub that: isize,

    /// The usize type element whose size is the same as the SizeUnion
    pub u_size: usize,

    /// The isize type element whose size is the same as the SizeUnion
    pub s_size: isize,

    /// The biggest unsigned element which is 8-bit unsigned integer
    pub ubyte: u8,

    /// The biggest signed element which is 8-bit unsigned integer
    pub sbyte: i8,
}


impl SizeUnion
{
    pub fn new() -> Self                    { Self { u_size: 0 } }
    pub fn new_with(u_size: usize) -> Self  { Self { u_size } }
    pub fn new_with_signed(s_size: isize) -> Self   { Self { s_size } }
    pub fn onoff(b: bool) -> Self           { Self { u_size: b as usize } }
    pub fn onoff_signed(b: bool) -> Self    { Self { s_size: b as isize } }
    pub fn new_with_u128(num: u128) -> Self { Self { u_size: LongerUnion::new_with(num).get_usize_(0) } }

    #[inline] pub fn get(self) -> usize         { unsafe { self.u_size } }
    #[inline] pub fn get_signed(self) -> isize  { unsafe { self.s_size } }
    #[inline] pub fn set(&mut self, val: usize) { self.u_size = val; }
    #[inline] pub fn set_signed(&mut self, val: isize)   { self.s_size = val; }
    #[cfg(target_pointer_width = "128")]    crate::number::get_set_byte!(16);
    #[cfg(target_pointer_width = "64")]     crate::number::get_set_byte!(8);
    #[cfg(target_pointer_width = "32")]     crate::number::get_set_byte!(4);
    #[cfg(target_pointer_width = "16")]     crate::number::get_set_byte!(2);
    #[cfg(target_pointer_width = "8")]      crate::number::get_set_byte_fit!();

    #[cfg(target_pointer_width = "128")]    crate::number::get_set_short!(8);
    #[cfg(target_pointer_width = "64")]     crate::number::get_set_short!(4);
    #[cfg(target_pointer_width = "32")]     crate::number::get_set_short!(2);
    #[cfg(target_pointer_width = "16")]     crate::number::get_set_short_fit!();

    #[cfg(target_pointer_width = "128")]    crate::number::get_set_int!(4);
    #[cfg(target_pointer_width = "64")]     crate::number::get_set_int!(2);
    #[cfg(target_pointer_width = "32")]     crate::number::get_set_int_fit!();

    #[cfg(target_pointer_width = "128")]    crate::number::get_set_long!(2);
    #[cfg(target_pointer_width = "64")]     crate::number::get_set_long_fit!();

    #[cfg(target_pointer_width = "128")]    crate::number::get_set_longer_fit!();

    crate::number::get_set_size_fit!();

    crate::number::integer_union_methods!(usize);
}



crate::number::operators_for_integer_unions_impl! { SizeUnion }

crate::number::shift_ops_for_integer_unions_impl! { SizeUnion, i8 }
crate::number::shift_ops_for_integer_unions_impl! { SizeUnion, i16 }
crate::number::shift_ops_for_integer_unions_impl! { SizeUnion, i32 }
crate::number::shift_ops_for_integer_unions_impl! { SizeUnion, i64 }
crate::number::shift_ops_for_integer_unions_impl! { SizeUnion, i128 }
crate::number::shift_ops_for_integer_unions_impl! { SizeUnion, isize }

crate::number::shift_ops_for_integer_unions_impl! { SizeUnion, u8 }
crate::number::shift_ops_for_integer_unions_impl! { SizeUnion, u16 }
crate::number::shift_ops_for_integer_unions_impl! { SizeUnion, u32 }
crate::number::shift_ops_for_integer_unions_impl! { SizeUnion, u64 }
crate::number::shift_ops_for_integer_unions_impl! { SizeUnion, u128 }
crate::number::shift_ops_for_integer_unions_impl! { SizeUnion, usize }

crate::number::display_for_integer_unions_impl! { SizeUnion }




impl Debug for SizeUnion
{
    /// Formats the value using the given formatter.
    /// 
    /// # Features
    /// When used with format specifier :?, the output is printed for debug.
    /// When used with the alternate format specifier #?, the output is
    /// pretty-printed.
    /// 
    /// # Example for the format specifier :?
    /// ```
    /// use cryptocol::number::*;
    /// let a_size = SizeUnion::new_with_signed(-1234567890123456789_isize);
    /// println!("a_size = {:?}", a_size);
    /// #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_size:?}"), "SizeUnion { this: 17212176183586094827, that: -1234567890123456789, u_size: 17212176183586094827, s_size: -1234567890123456789, ulong: 17212176183586094827, slong: -1234567890123456789, uint: [2182512363, 4007522059], sint: [-2112454933, -287445237], ushort: [32491, 33302, 61195, 61149], sshort: [32491, -32234, -4341, -4387], ubyte: [235, 126, 22, 130, 11, 239, 221, 238], sbyte: [-21, 126, 22, -126, 11, -17, -35, -18] }");
    /// ```
    /// 
    /// # Example for the format specifier :#?
    /// ```
    /// use cryptocol::number::*;
    /// let a_size = SizeUnion::new_with_signed(-1234567890123456789_isize);
    /// println!("a_size = {:#?}", a_size);
    /// #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_size:#?}"), r#"SizeUnion {
    ///     this: 17212176183586094827,
    ///     that: -1234567890123456789,
    ///     u_size: 17212176183586094827,
    ///     s_size: -1234567890123456789,
    ///     ulong: 17212176183586094827,
    ///     slong: -1234567890123456789,
    ///     uint: [
    ///         2182512363,
    ///         4007522059,
    ///     ],
    ///     sint: [
    ///         -2112454933,
    ///         -287445237,
    ///     ],
    ///     ushort: [
    ///         32491,
    ///         33302,
    ///         61195,
    ///         61149,
    ///     ],
    ///     sshort: [
    ///         32491,
    ///         -32234,
    ///         -4341,
    ///         -4387,
    ///     ],
    ///     ubyte: [
    ///         235,
    ///         126,
    ///         22,
    ///         130,
    ///         11,
    ///         239,
    ///         221,
    ///         238,
    ///     ],
    ///     sbyte: [
    ///         -21,
    ///         126,
    ///         22,
    ///         -126,
    ///         11,
    ///         -17,
    ///         -35,
    ///         -18,
    ///     ],
    /// }"#);
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// This method works exactly the same way as the normal method fmt() of
    /// Debug. So, all the description of this method is mainly the
    /// same as that of the implementation of the method fmt() of Debug for the
    /// primitive unsigned integer types except example codes. Confer to the
    /// descryptions that are linked to in the section _Reference_. This
    /// plagiarism is not made maliciously but is made for the reason of
    /// effectiveness and efficiency so that users may understand better and
    /// easily how to use this method with simiilarity to the method
    /// Debug() of implementation for the primitive unsigned integer
    /// types.
    /// 
    /// # References
    /// - If you want to know about the method of Debug for the primitive type,
    /// read [here](https://doc.rust-lang.org/std/fmt/trait.Debug.html).
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let mut ff = f.debug_struct("SizeUnion");
        ff.field("this", &self.get())
            .field("that", &self.get_signed())
            .field("u_size", &self.get_usize())
            .field("s_size", &self.get_ssize());

        #[cfg(target_pointer_width = "128")]
        ff.field("ulonger", unsafe { &self.ulonger } )
            .field("slonger", unsafe { &self.slonger } )
            .field("ulong", unsafe { &self.ulong } )
            .field("slong", unsafe { &self.slong } )
            .field("uint", unsafe { &self.uint } )
            .field("sint", unsafe { &self.sint } )
            .field("ushort", unsafe { &self.ushort } )
            .field("sshort", unsafe { &self.sshort } )
            .field("ubyte", unsafe { &self.ubyte } )
            .field("sbyte", unsafe { &self.sbyte } );

        #[cfg(target_pointer_width = "64")] 
        ff.field("ulong", unsafe { &self.ulong } )
            .field("slong", unsafe { &self.slong } )
            .field("uint", unsafe { &self.uint } )
            .field("sint", unsafe { &self.sint } )
            .field("ushort", unsafe { &self.ushort } )
            .field("sshort", unsafe { &self.sshort } )
            .field("ubyte", unsafe { &self.ubyte } )
            .field("sbyte", unsafe { &self.sbyte } );

        #[cfg(target_pointer_width = "32")]
        ff.field("uint", unsafe { &self.uint } )
            .field("sint", unsafe { &self.sint } )
            .field("ushort", unsafe { &self.ushort } )
            .field("sshort", unsafe { &self.sshort } )
            .field("ubyte", unsafe { &self.ubyte } )
            .field("sbyte", unsafe { &self.sbyte } );

        #[cfg(target_pointer_width = "16")]
        ff.field("ushort", unsafe { &self.ushort } )
            .field("sshort", unsafe { &self.sshort } )
            .field("ubyte", unsafe { &self.ubyte } )
            .field("sbyte", unsafe { &self.sbyte } );

        #[cfg(target_pointer_width = "8")]
        ff.field("ubyte", unsafe { &self.ubyte } )
            .field("sbyte", unsafe { &self.sbyte } );

        ff.finish()
    }
}
