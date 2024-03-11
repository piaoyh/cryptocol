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

/// # Introduction
/// This union `LongerUnion` is for slicing `u128` into two `u64`s, two `i64`s,
/// four `u32`s, four `i32`s, eight `u16`s, eight `i16`s, forteen `u8`,
/// and/or `i8`.
/// 
/// Sometimes, for example, we need to slice `u128` data into two `u64` pieces
/// which include a higher eight-byte word and a lower eight-byte word, and/or
/// into  four `u32` pieces which include a first two-byte word, a second
/// two-byte word, a third two-byte word and a fourth two-byte word.
/// In that case, `LongerUnion` will be very helpful.
/// 
/// # Quick Start
/// In order to use this union, you have to import (use)
/// `cryptocol::number::LongerUnion` as follows.
/// 
/// ## Example 1
/// ```
/// use cryptocol::number::LongerUnion;
/// ```
/// You can use the methods `get()`, `get_signed()`, `get_ulonger()`, and
/// `get_slonger()` in order to obtain the data of `u128` in various types.
/// And, you can also slice the data of `u128` into two `u64` type data by
/// using the methods `get_ulong()`, `get_slong()`, `get_ulong_()`, and
/// `get_slong_()`. Or, you can also slice the data of `u32` into four `u32`
/// type data by using the methods `get_uint()`, `get_sint()`, `get_uint_()`,
/// and `get_sint_()`. Or, you can also slice the data of `u128` into eight
/// `u16` type data by using the methods `get_ushort()`, `get_sshort()`,
/// `get_ushort_()`, and `get_sshort_()`.
/// 
/// ## Example 2
/// ```
/// use cryptocol::number::LongerUnion;
/// 
/// let a = LongerUnion::new_with_signed(-1234567890987654321012345678987654321_i128);
/// 
/// println!("a.get() = {}", a.get());
/// println!("a.get_signed() = {}", a.get_signed());
/// println!("a.get_ulonger() = {}", a.get_ulonger());
/// println!("a.get_slonger() = {}", a.get_slonger());
/// assert_eq!(a.get(), 339047799029950809142362261752780557135_u128);
/// assert_eq!(a.get_signed(), -1234567890987654321012345678987654321_i128);
/// assert_eq!(a.get_ulonger(), 339047799029950809142362261752780557135_u128);
/// assert_eq!(a.get_slonger(), -1234567890987654321012345678987654321_i128);
/// 
/// #[cfg(target_endian = "little")]
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
/// 
///     #[cfg(target_pointer_width = "128")]
///     {
///         println!("a.get_usize() = {}", a.get_usize());
///         println!("a.get_ssize() = {}", a.get_ssize());
///         assert_eq!(a.get_usize(), 339047799029950809142362261752780557135_usize);
///         assert_eq!(a.get_ssize(), 1234567890987654321012345678987654321_isize);
///     }
///     #[cfg(target_pointer_width = "64")]
///     {
///         const N: usize = 2;
///         for i in 0..N
///             { println!("a.get_usize_({}) = {}", i, a.get_usize_(i)); }
///         for i in 0..N
///             { println!("a.get_ssize_({}) = {}", i, a.get_ssize_(i)); }
///         assert_eq!(a.get_usize_(0), 13664881099896654671_usize);
///         assert_eq!(a.get_usize_(1), 18379818014235068504_usize);
///         assert_eq!(a.get_ssize_(0), -4781862973812896945_isize);
///         assert_eq!(a.get_ssize_(1), -66926059474483112_isize);
///     }
///     #[cfg(target_pointer_width = "32")]
///     {
///         const N: usize = 4;
///         for i in 0..N
///             { println!("a.get_usize_({}) = {}", i, a.get_usize_(i)); }
///         for i in 0..N
///             { println!("a.get_ssize_({}) = {}", i, a.get_ssize_(i)); }
///         assert_eq!(a.get_usize_(0), 4048161615_usize);
///         assert_eq!(a.get_usize_(1), 3181603061_usize);
///         assert_eq!(a.get_usize_(2), 2127464536_usize);
///         assert_eq!(a.get_usize_(3), 4279384858_usize);
///         assert_eq!(a.get_ssize_(0), -246805681_isize);
///         assert_eq!(a.get_ssize_(1), -1113364235_isize);
///         assert_eq!(a.get_ssize_(2), 2127464536_isize);
///         assert_eq!(a.get_ssize_(3), -15582438_isize);
///     }
///     #[cfg(target_pointer_width = "16")]
///     {
///         const N: usize = 8;
///         for i in 0..N
///             { println!("a.get_usize_({}) = {}", i, a.get_usize_(i)); }
///         for i in 0..N
///             { println!("a.get_ssize_({}) = {}", i, a.get_ssize_(i)); }
///         assert_eq!(a.get_usize_(0), 2895_usize);
///         assert_eq!(a.get_usize_(1), 61770_usize);
///         assert_eq!(a.get_usize_(2), 26869_usize);
///         assert_eq!(a.get_usize_(3), 48547_usize);
///         assert_eq!(a.get_usize_(4), 34904_usize);
///         assert_eq!(a.get_usize_(5), 32462_usize);
///         assert_eq!(a.get_usize_(6), 15130_usize);
///         assert_eq!(a.get_usize_(7), 65298_usize);
///         assert_eq!(a.get_ssize_(0), 2895_isize);
///         assert_eq!(a.get_ssize_(1), -3766_isize);
///         assert_eq!(a.get_ssize_(2), 26869_isize);
///         assert_eq!(a.get_ssize_(3), -16989_isize);
///         assert_eq!(a.get_ssize_(4), -30632_isize);
///         assert_eq!(a.get_ssize_(5), 32462_isize);
///         assert_eq!(a.get_ssize_(6), 15130_isize);
///         assert_eq!(a.get_ssize_(7), -238_isize);
///     }
///     #[cfg(target_pointer_width = "8")]
///     {
///         const N: usize = 16;
///         for i in 0..N
///             { println!("a.get_usize_({}) = {}", i, a.get_usize_(i)); }
///         for i in 0..N
///             { println!("a.get_ssize_({}) = {}", i, a.get_ssize_(i)); }
///         assert_eq!(a.get_usize_(0), 79_usize);
///         assert_eq!(a.get_usize_(1), 11_usize);
///         assert_eq!(a.get_usize_(2), 74_usize);
///         assert_eq!(a.get_usize_(3), 241_usize);
///         assert_eq!(a.get_usize_(4), 245_usize);
///         assert_eq!(a.get_usize_(5), 104_usize);
///         assert_eq!(a.get_usize_(6), 163_usize);
///         assert_eq!(a.get_usize_(7), 189_usize);
///         assert_eq!(a.get_usize_(8), 88_usize);
///         assert_eq!(a.get_usize_(9), 136_usize);
///         assert_eq!(a.get_usize_(10), 206_usize);
///         assert_eq!(a.get_usize_(11), 126_usize);
///         assert_eq!(a.get_usize_(12), 26_usize);
///         assert_eq!(a.get_usize_(13), 59_usize);
///         assert_eq!(a.get_usize_(14), 18_usize);
///         assert_eq!(a.get_usize_(15), 255_usize);
///         assert_eq!(a.get_ssize_(0), 79_isize);
///         assert_eq!(a.get_ssize_(1), 11_isize);
///         assert_eq!(a.get_ssize_(2), 74_isize);
///         assert_eq!(a.get_ssize_(3), -15_isize);
///         assert_eq!(a.get_ssize_(4), -11_isize);
///         assert_eq!(a.get_ssize_(5), 104_isize);
///         assert_eq!(a.get_ssize_(6), -93_isize);
///         assert_eq!(a.get_ssize_(7), -67_isize);
///         assert_eq!(a.get_ssize_(8), 88_isize);
///         assert_eq!(a.get_ssize_(9), -120_isize);
///         assert_eq!(a.get_ssize_(10), -50_isize);
///         assert_eq!(a.get_ssize_(11), 126_isize);
///         assert_eq!(a.get_ssize_(12), 26_isize);
///         assert_eq!(a.get_ssize_(13), 59_isize);
///         assert_eq!(a.get_ssize_(14), 18_isize);
///         assert_eq!(a.get_ssize_(15), -1_isize);
///     }
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
/// let a_longerunion = 123456789876543212345678987654321_u128.into_longerunion();
/// let b_longerunion = 876543210123456787654321012345678_u128.into_longerunion();
/// let c_longerunion = a_longerunion.wrapping_add(b_longerunion);
/// println!("{} + {} = {}", a_longerunion, b_longerunion, c_longerunion);
/// assert_eq!(c_longerunion.get(), 999999999999999999999999999999999_u128);
/// for i in 0..2
///     { println!("c_longerunion.get_ulong_({}) = {}", i, c_longerunion.get_ulong_(i)); }
/// assert_eq!(c_longerunion.get_ulong_(0), 4089650035136921599_u64);
/// assert_eq!(c_longerunion.get_ulong_(1), 54210108624275_u64);
/// for i in 0..4
///     { println!("c_longerunion.get_uint_({}) = {}", i, c_longerunion.get_uint_(i)); }
/// assert_eq!(c_longerunion.get_uint_(0), 4294967295_u32);
/// assert_eq!(c_longerunion.get_uint_(1), 952195849_u32);
/// assert_eq!(c_longerunion.get_uint_(2), 3326381459_u32);
/// assert_eq!(c_longerunion.get_uint_(3), 12621_u32);
/// for i in 0..8
///     { println!("c_longerunion.get_ushort_({}) = {}", i, c_longerunion.get_ushort_(i)); }
/// assert_eq!(c_longerunion.get_ushort_(0), 65535_u16);
/// assert_eq!(c_longerunion.get_ushort_(1), 65535_u16);
/// assert_eq!(c_longerunion.get_ushort_(2), 23305_u16);
/// assert_eq!(c_longerunion.get_ushort_(3), 14529_u16);
/// assert_eq!(c_longerunion.get_ushort_(4), 36243_u16);
/// assert_eq!(c_longerunion.get_ushort_(5), 50756_u16);
/// assert_eq!(c_longerunion.get_ushort_(6), 12621_u16);
/// assert_eq!(c_longerunion.get_ushort_(7), 0_u16);
/// for i in 0..16
///     { println!("c_longerunion.get_ubyte_({}) = {}", i, c_longerunion.get_ubyte_(i)); }
/// assert_eq!(c_longerunion.get_ubyte_(0), 255_u8);
/// assert_eq!(c_longerunion.get_ubyte_(1), 255_u8);
/// assert_eq!(c_longerunion.get_ubyte_(2), 255_u8);
/// assert_eq!(c_longerunion.get_ubyte_(3), 255_u8);
/// assert_eq!(c_longerunion.get_ubyte_(4), 9_u8);
/// assert_eq!(c_longerunion.get_ubyte_(5), 91_u8);
/// assert_eq!(c_longerunion.get_ubyte_(6), 193_u8);
/// assert_eq!(c_longerunion.get_ubyte_(7), 56_u8);
/// assert_eq!(c_longerunion.get_ubyte_(8), 147_u8);
/// assert_eq!(c_longerunion.get_ubyte_(9), 141_u8);
/// assert_eq!(c_longerunion.get_ubyte_(10), 68_u8);
/// assert_eq!(c_longerunion.get_ubyte_(11), 198_u8);
/// assert_eq!(c_longerunion.get_ubyte_(12), 77_u8);
/// assert_eq!(c_longerunion.get_ubyte_(13), 49_u8);
/// assert_eq!(c_longerunion.get_ubyte_(14), 0_u8);
/// assert_eq!(c_longerunion.get_ubyte_(15), 0_u8);
/// 
/// let d_longerunion = b_longerunion - a_longerunion;
/// println!("{} - {} = {}", b_longerunion, a_longerunion, d_longerunion);
/// assert_eq!(d_longerunion.get(), 753086420246913575308642024691357_u128);
/// for i in 0..2
///     { println!("d_longunion.get_ulong_({}) = {}", i, d_longerunion.get_ulong_(i)); }
/// assert_eq!(d_longerunion.get_ulong_(0), 14084888390109238941_u64);
/// assert_eq!(d_longerunion.get_ulong_(1), 40824896645051_u64);
/// for i in 0..4
///     { println!("d_longunion.get_uint_({}) = {}", i, d_longerunion.get_uint_(i)); }
/// assert_eq!(d_longerunion.get_uint_(0), 2843481757_u32);
/// assert_eq!(d_longerunion.get_uint_(1), 3279393629_u32);
/// assert_eq!(d_longerunion.get_uint_(2), 1232496571_u32);
/// assert_eq!(d_longerunion.get_uint_(3), 9505_u32);
/// for i in 0..8
///     { println!("d_longunion.get_ushort_({}) = {}", i, d_longerunion.get_ushort_(i)); }
/// assert_eq!(d_longerunion.get_ushort_(0), 5789_u16);
/// assert_eq!(d_longerunion.get_ushort_(1), 43388_u16);
/// assert_eq!(d_longerunion.get_ushort_(2), 37725_u16);
/// assert_eq!(d_longerunion.get_ushort_(3), 50039_u16);
/// assert_eq!(d_longerunion.get_ushort_(4), 26555_u16);
/// assert_eq!(d_longerunion.get_ushort_(5), 18806_u16);
/// assert_eq!(d_longerunion.get_ushort_(6), 9505_u16);
/// assert_eq!(d_longerunion.get_ushort_(7), 0_u16);
/// for i in 0..16
///     { println!("d_longunion.get_ubyte_({}) = {}", i, d_longerunion.get_ubyte_(i)); }
/// assert_eq!(d_longerunion.get_ubyte_(0), 157_u8);
/// assert_eq!(d_longerunion.get_ubyte_(1), 22_u8);
/// assert_eq!(d_longerunion.get_ubyte_(2), 124_u8);
/// assert_eq!(d_longerunion.get_ubyte_(3), 169_u8);
/// assert_eq!(d_longerunion.get_ubyte_(4), 93_u8);
/// assert_eq!(d_longerunion.get_ubyte_(5), 147_u8);
/// assert_eq!(d_longerunion.get_ubyte_(6), 119_u8);
/// assert_eq!(d_longerunion.get_ubyte_(7), 195_u8);
/// assert_eq!(d_longerunion.get_ubyte_(8), 187_u8);
/// assert_eq!(d_longerunion.get_ubyte_(9), 103_u8);
/// assert_eq!(d_longerunion.get_ubyte_(10), 118_u8);
/// assert_eq!(d_longerunion.get_ubyte_(11), 73_u8);
/// assert_eq!(d_longerunion.get_ubyte_(12), 33_u8);
/// assert_eq!(d_longerunion.get_ubyte_(13), 37_u8);
/// assert_eq!(d_longerunion.get_ubyte_(14), 0_u8);
/// assert_eq!(d_longerunion.get_ubyte_(15), 0_u8);
/// 
/// let e_longerunion = d_longerunion * 3_u128.into_longerunion();
/// println!("{} * {} = {}", d_longerunion, 3_u128.into_longerunion(), e_longerunion);
/// assert_eq!(e_longerunion.get(), 2259259260740740725925926074074071_u128);
/// 
/// let f_longerunion = c_longerunion / 10_u128.into_longerunion();
/// println!("{} / {} = {}", c_longerunion, 10_u128.into_longerunion(), f_longerunion);
/// assert_eq!(f_longerunion.get(), 99999999999999999999999999999999_u128);
/// 
/// let g_longerunion = c_longerunion % 10_u128.into_longerunion();
/// println!("{} % {} = {}", c_longerunion, 10_u128.into_longerunion(), g_longerunion);
/// assert_eq!(g_longerunion.get(), 9_u128);
/// ```
///  
/// # Big-endian issue
/// It is just experimental for Big Endian CPUs. So, you are not encouraged
/// to use it for serious purpose. Only use this crate for Big-endian CPUs
/// with your own full responsibility.
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub union LongerUnion
{
    /// The biggest unsigned element for compatibility with other unions
    this: u128,

    /// The biggest signed element for compatibility with other unions
    that: i128,

    /// The biggest unsigned element which is 128-bit unsigned integer
    ulonger: u128,

    /// The biggest signed element which is 128-bit unsigned integer
    slonger: i128,

    /// The secondly biggest unsigned element array whose elements are
    /// 64-bit unsigned integer
    ulong: [u64; 2],

    /// The secondly biggest signed element array whose elements are
    /// 64-bit unsigned integer
    slong: [i64; 2],

    /// The thirdly biggest unsigned element array whose elements are
    /// 32-bit unsigned integer
    uint: [u32; 4],

    /// The thirdly biggest signed element array whose elements are
    /// 32-bit unsigned integer
    sint: [i32; 4],

    /// The fourthly biggest unsigned element array whose elements are
    /// 16-bit unsigned integer
    ushort: [u16; 8],

    /// The fourthly biggest unsigned element array whose elements are
    /// 16-bit unsigned integer
    sshort: [i16; 8],

    /// The fifthly biggest unsigned element array whose elements are
    /// 8-bit unsigned integer
    ubyte: [u8; 16],

    /// The fifthly biggest signed element array whose elements are
    /// 8-bit unsigned integer
    sbyte: [i8; 16],

    /// The usize type element whose size is the same as the LongerUnion
    #[cfg(target_pointer_width = "128")] u_size: usize,

    /// The isize type element whose size is the same as the LongerUnion
    #[cfg(target_pointer_width = "128")] s_size: isize,

    /// The isize type array whose elements's size is 64-bit size
    #[cfg(target_pointer_width = "64")] u_size: [usize; 2],

    /// The isize type array whose elements's size is 64-bit size
    #[cfg(target_pointer_width = "64")] s_size: [isize; 2],

    /// The usize type array whose elements's size is 32-bit size
    #[cfg(target_pointer_width = "32")] u_size: [usize; 4],

    /// The isize type array whose elements's size is 32-bit size
    #[cfg(target_pointer_width = "32")] s_size: [isize; 4],

    /// The usize type array whose elements's size is 16-bit size
    #[cfg(target_pointer_width = "16")] u_size: [usize; 8],

    /// The isize type array whose elements's size is 16-bit size
    #[cfg(target_pointer_width = "16")] s_size: [isize; 8],

    /// The usize type array whose elements's size is 8-bit size
    #[cfg(target_pointer_width = "8")] u_size: [usize; 16],

    /// The isize type array whose elements's size is 8-bit size
    #[cfg(target_pointer_width = "8")] s_size: [isize; 16],
}


impl LongerUnion
{
    pub fn new() -> Self                    { Self { ulonger: 0 } }
    pub fn new_with(ulonger: u128) -> Self  { Self { ulonger } }
    pub fn new_with_signed(slonger: i128) -> Self   { Self { slonger } }
    pub fn new_with_ubytes(ubyte: [u8; 16]) -> Self { Self { ubyte } }
    pub fn new_with_ushorts(ushort: [u16; 8]) -> Self   { Self { ushort } }
    pub fn new_with_uints(uint: [u32; 4]) -> Self   { Self { uint } }
    pub fn new_with_ulongs(ulong: [u64; 2]) -> Self   { Self { ulong } }
    pub fn onoff(b: bool) -> Self           { Self { ulonger: b as u128 } }
    pub fn onoff_signed(b: bool) -> Self    { Self { slonger: b as i128 } }
    pub fn new_with_u128(num: u128) -> Self { Self { ulonger: num } }

    #[inline] pub fn get(self) -> u128          { unsafe { self.ulonger } }
    #[inline] pub fn get_signed(self) -> i128   { unsafe { self.slonger } }
    #[inline] pub fn set(&mut self, val: u128)  { self.ulonger = val; }
    #[inline] pub fn set_signed(&mut self, val: i128)   { self.slonger = val; }
    crate::number::get_set_longer_fit!();

    crate::number::get_set_byte!(16);

    crate::number::get_set_short!(8);

    crate::number::get_set_int!(4);

    crate::number::get_set_long!(2);

    #[cfg(target_pointer_width = "128")]    crate::number::get_set_size_fit!();
    #[cfg(target_pointer_width = "64")]     crate::number::get_set_size!(2);
    #[cfg(target_pointer_width = "32")]     crate::number::get_set_size!(4);
    #[cfg(target_pointer_width = "16")]     crate::number::get_set_size!(8);
    #[cfg(target_pointer_width = "8")]      crate::number::get_set_size!(16);
    
    crate::number::integer_union_methods!(u128);
}



crate::number::operators_for_integer_unions_impl! { LongerUnion }

crate::number::shift_ops_for_integer_unions_impl! { LongerUnion, i8 }
crate::number::shift_ops_for_integer_unions_impl! { LongerUnion, i16 }
crate::number::shift_ops_for_integer_unions_impl! { LongerUnion, i32 }
crate::number::shift_ops_for_integer_unions_impl! { LongerUnion, i64 }
crate::number::shift_ops_for_integer_unions_impl! { LongerUnion, i128 }
crate::number::shift_ops_for_integer_unions_impl! { LongerUnion, isize }

crate::number::shift_ops_for_integer_unions_impl! { LongerUnion, u8 }
crate::number::shift_ops_for_integer_unions_impl! { LongerUnion, u16 }
crate::number::shift_ops_for_integer_unions_impl! { LongerUnion, u32 }
crate::number::shift_ops_for_integer_unions_impl! { LongerUnion, u64 }
crate::number::shift_ops_for_integer_unions_impl! { LongerUnion, u128 }
crate::number::shift_ops_for_integer_unions_impl! { LongerUnion, usize }

crate::number::display_for_integer_unions_impl! { LongerUnion }



impl Debug for LongerUnion
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
    /// let a_longer = LongerUnion::new_with_signed(-123456789012345678901234567890123456789_i128);
    /// println!("a_longer = {:?}", a_longer);
    /// #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_longer:?}"), "LongerUnion { this: 216825577908592784562140039541644754667, that: -123456789012345678901234567890123456789, ulonger: 216825577908592784562140039541644754667, slonger: -123456789012345678901234567890123456789, ulong: [6134004772338302699, 11754138130946064698], slong: [6134004772338302699, -6692605942763486918], uint: [1371963115, 1428184279, 2682913082, 2736723546], sint: [1371963115, 1428184279, -1612054214, -1558243750], ushort: [32491, 20934, 23767, 21792, 314, 40938, 5722, 41759], sshort: [32491, 20934, 23767, 21792, 314, -24598, 5722, -23777], ubyte: [235, 126, 198, 81, 215, 92, 32, 85, 58, 1, 234, 159, 90, 22, 31, 163], sbyte: [-21, 126, -58, 81, -41, 92, 32, 85, 58, 1, -22, -97, 90, 22, 31, -93], u_size: [6134004772338302699, 11754138130946064698], s_size: [6134004772338302699, -6692605942763486918] }");
    /// ```
    /// 
    /// # Example for the format specifier :#?
    /// ```
    /// use cryptocol::number::*;
    /// let a_longer = LongerUnion::new_with_signed(-123456789012345678901234567890123456789_i128);
    /// println!("a_longer = {:#?}", a_longer);
    /// #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_longer:#?}"), r#"LongerUnion {
    ///     this: 216825577908592784562140039541644754667,
    ///     that: -123456789012345678901234567890123456789,
    ///     ulonger: 216825577908592784562140039541644754667,
    ///     slonger: -123456789012345678901234567890123456789,
    ///     ulong: [
    ///         6134004772338302699,
    ///         11754138130946064698,
    ///     ],
    ///     slong: [
    ///         6134004772338302699,
    ///         -6692605942763486918,
    ///     ],
    ///     uint: [
    ///         1371963115,
    ///         1428184279,
    ///         2682913082,
    ///         2736723546,
    ///     ],
    ///     sint: [
    ///         1371963115,
    ///         1428184279,
    ///         -1612054214,
    ///         -1558243750,
    ///     ],
    ///     ushort: [
    ///         32491,
    ///         20934,
    ///         23767,
    ///         21792,
    ///         314,
    ///         40938,
    ///         5722,
    ///         41759,
    ///     ],
    ///     sshort: [
    ///         32491,
    ///         20934,
    ///         23767,
    ///         21792,
    ///         314,
    ///         -24598,
    ///         5722,
    ///         -23777,
    ///     ],
    ///     ubyte: [
    ///         235,
    ///         126,
    ///         198,
    ///         81,
    ///         215,
    ///         92,
    ///         32,
    ///         85,
    ///         58,
    ///         1,
    ///         234,
    ///         159,
    ///         90,
    ///         22,
    ///         31,
    ///         163,
    ///     ],
    ///     sbyte: [
    ///         -21,
    ///         126,
    ///         -58,
    ///         81,
    ///         -41,
    ///         92,
    ///         32,
    ///         85,
    ///         58,
    ///         1,
    ///         -22,
    ///         -97,
    ///         90,
    ///         22,
    ///         31,
    ///         -93,
    ///     ],
    ///     u_size: [
    ///         6134004772338302699,
    ///         11754138130946064698,
    ///     ],
    ///     s_size: [
    ///         6134004772338302699,
    ///         -6692605942763486918,
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
        let mut ff = f.debug_struct("LongerUnion");
        ff.field("this", &self.get())
            .field("that", &self.get_signed())
            .field("ulonger", &self.get_ulonger())
            .field("slonger", &self.get_slonger())
            .field("ulong", &[self.get_ulong_(0), self.get_ulong_(1)])
            .field("slong", &[self.get_slong_(0), self.get_slong_(1)])
            .field("uint", &[self.get_uint_(0), self.get_uint_(1), self.get_uint_(2), self.get_uint_(3)])
            .field("sint", &[self.get_sint_(0), self.get_sint_(1), self.get_sint_(2), self.get_sint_(3)])
            .field("ushort", &[self.get_ushort_(0), self.get_ushort_(1), self.get_ushort_(2), self.get_ushort_(3), self.get_ushort_(4), self.get_ushort_(5), self.get_ushort_(6), self.get_ushort_(7)])
            .field("sshort", &[self.get_sshort_(0), self.get_sshort_(1), self.get_sshort_(2), self.get_sshort_(3), self.get_sshort_(4), self.get_sshort_(5), self.get_sshort_(6), self.get_sshort_(7)])
            .field("ubyte", &[self.get_ubyte_(0), self.get_ubyte_(1), self.get_ubyte_(2), self.get_ubyte_(3), self.get_ubyte_(4), self.get_ubyte_(5), self.get_ubyte_(6), self.get_ubyte_(7), self.get_ubyte_(8), self.get_ubyte_(9), self.get_ubyte_(10), self.get_ubyte_(11), self.get_ubyte_(12), self.get_ubyte_(13), self.get_ubyte_(14), self.get_ubyte_(15)])
            .field("sbyte", &[self.get_sbyte_(0), self.get_sbyte_(1), self.get_sbyte_(2), self.get_sbyte_(3), self.get_sbyte_(4), self.get_sbyte_(5), self.get_sbyte_(6), self.get_sbyte_(7), self.get_sbyte_(8), self.get_sbyte_(9), self.get_sbyte_(10), self.get_sbyte_(11), self.get_sbyte_(12), self.get_sbyte_(13), self.get_sbyte_(14), self.get_sbyte_(15)]);
        #[cfg(target_pointer_width = "128")] ff.field("u_size",  &self.get_usize())
                                                .field("s_size", &self.get_ssize());
        #[cfg(target_pointer_width = "64")] ff.field("u_size", &[self.get_usize_(0), self.get_usize_(1)])
                                                .field("s_size", &[self.get_ssize_(0), self.get_ssize_(1)]);
        #[cfg(target_pointer_width = "32")] ff.field("u_size", &[self.get_usize_(0), self.get_usize_(1), self.get_usize_(2), self.get_usize_(3)])
                                                .field("s_size", &[self.get_ssize_(0), self.get_ssize_(1), self.get_ssize_(2), self.get_ssize_(3)]);
        #[cfg(target_pointer_width = "16")] ff.field("u_size", &[self.get_usize_(0), self.get_usize_(1), self.get_usize_(2), self.get_usize_(3), self.get_usize_(4), self.get_usize_(5), self.get_usize_(6), self.get_usize_(7)])
                                                .field("s_size", &[self.get_ssize_(0), self.get_ssize_(1), self.get_ssize_(2), self.get_ssize_(3), self.get_ssize_(4), self.get_ssize_(5), self.get_ssize_(6), self.get_ssize_(7)]);
        #[cfg(target_pointer_width = "8")] ff.field("u_size", &[self.get_usize_(0), self.get_usize_(1), self.get_usize_(2), self.get_usize_(3), self.get_usize_(4), self.get_usize_(5), self.get_usize_(6), self.get_usize_(7), self.get_usize_(8), self.get_usize_(9), self.get_usize_(10), self.get_usize_(11), self.get_usize_(12), self.get_usize_(13), self.get_usize_(14), self.get_usize_(15)])
                                                .field("s_size", &[self.get_ssize_(0), self.get_ssize_(1), self.get_ssize_(2), self.get_ssize_(3), self.get_ssize_(4), self.get_ssize_(5), self.get_ssize_(6), self.get_ssize_(7), self.get_ssize_(8), self.get_ssize_(9), self.get_ssize_(10), self.get_ssize_(11), self.get_ssize_(12), self.get_ssize_(13), self.get_ssize_(14), self.get_ssize_(15)]);
        ff.finish()
    }
}
