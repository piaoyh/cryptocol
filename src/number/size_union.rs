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
// #![warn(rustdoc::missing_doc_code_examples)]
#![allow(missing_docs)]
// #![allow(rustdoc::missing_doc_code_examples)]

use std::cmp::{ PartialEq, PartialOrd, Ordering };
use std::ops::{ BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
                Shl, ShlAssign, Shr, ShrAssign, 
                Add, AddAssign, Sub, SubAssign, Mul, MulAssign,
                Div, DivAssign, Rem, RemAssign };
use std::fmt::{ self, Alignment, Error, Formatter, Display, Debug, Pointer,
                Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp };

use crate::number::{ SmallUInt, ShortUnion, IntUnion, LongUnion, LongerUnion };
use crate::number::{ calc_assign_to_calc, fmt_with_radix, fmt_with_exponent };

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
/// You can use `SizeUnion` as if you used `usize`. You can perform all kinds
/// of arithmetic operations such as addition, subtraction, multiplication,
/// and division (div and rem), and other operations which are available for
/// `usize`. If you use `SizeUnion` with the help of `SmallUInt`, it will be
/// even more powerful and convenient. In this case, you don't even have to
/// import (use) `cryptocol::number::SizeUnion`.
/// 
/// ## Example 3
/// ```
/// use cryptocol::number::SmallUInt;
/// 
/// let a_sizeunion = 123456789876543212345678987654321_usize.into_sizeunion();
/// let b_sizeunion = 876543210123456787654321012345678_usize.into_sizeunion();
/// let c_sizeunion = a_sizeunion.wrapping_add(b_sizeunion);
/// 
/// println!("{} + {} = {}", a_sizeunion, b_sizeunion, c_sizeunion);
/// assert_eq!(c_sizeunion.get(), 999999999999999999999999999999999_usize);
/// 
/// for i in 0..2
///     { println!("c_sizeunion.get_ulong_({}) = {}", i, c_sizeunion.get_ulong_(i)); }
/// assert_eq!(c_sizeunion.get_ulong_(0), 4089650035136921599_u64);
/// assert_eq!(c_sizeunion.get_ulong_(1), 54210108624275_u64);
/// 
/// for i in 0..4
///     { println!("c_sizeunion.get_uint_({}) = {}", i, c_sizeunion.get_uint_(i)); }
/// assert_eq!(c_sizeunion.get_uint_(0), 4294967295_u32);
/// assert_eq!(c_sizeunion.get_uint_(1), 952195849_u32);
/// assert_eq!(c_sizeunion.get_uint_(2), 3326381459_u32);
/// assert_eq!(c_sizeunion.get_uint_(3), 12621_u32);
/// 
/// for i in 0..8
///     { println!("c_sizeunion.get_ushort_({}) = {}", i, c_sizeunion.get_ushort_(i)); }
/// assert_eq!(c_sizeunion.get_ushort_(0), 65535_u16);
/// assert_eq!(c_sizeunion.get_ushort_(1), 65535_u16);
/// assert_eq!(c_sizeunion.get_ushort_(2), 23305_u16);
/// assert_eq!(c_sizeunion.get_ushort_(3), 14529_u16);
/// assert_eq!(c_sizeunion.get_ushort_(4), 36243_u16);
/// assert_eq!(c_sizeunion.get_ushort_(5), 50756_u16);
/// assert_eq!(c_sizeunion.get_ushort_(6), 12621_u16);
/// assert_eq!(c_sizeunion.get_ushort_(7), 0_u16);
/// 
/// for i in 0..16
///     { println!("c_sizeunion.get_ubyte_({}) = {}", i, c_sizeunion.get_ubyte_(i)); }
/// assert_eq!(c_sizeunion.get_ubyte_(0), 255_u8);
/// assert_eq!(c_sizeunion.get_ubyte_(1), 255_u8);
/// assert_eq!(c_sizeunion.get_ubyte_(2), 255_u8);
/// assert_eq!(c_sizeunion.get_ubyte_(3), 255_u8);
/// assert_eq!(c_sizeunion.get_ubyte_(4), 9_u8);
/// assert_eq!(c_sizeunion.get_ubyte_(5), 91_u8);
/// assert_eq!(c_sizeunion.get_ubyte_(6), 193_u8);
/// assert_eq!(c_sizeunion.get_ubyte_(7), 56_u8);
/// assert_eq!(c_sizeunion.get_ubyte_(8), 147_u8);
/// assert_eq!(c_sizeunion.get_ubyte_(9), 141_u8);
/// assert_eq!(c_sizeunion.get_ubyte_(10), 68_u8);
/// assert_eq!(c_sizeunion.get_ubyte_(11), 198_u8);
/// assert_eq!(c_sizeunion.get_ubyte_(12), 77_u8);
/// assert_eq!(c_sizeunion.get_ubyte_(13), 49_u8);
/// assert_eq!(c_sizeunion.get_ubyte_(14), 0_u8);
/// assert_eq!(c_sizeunion.get_ubyte_(15), 0_u8);
/// 
/// let d_sizeunion = b_sizeunion - a_sizeunion;
/// println!("{} - {} = {}", b_sizeunion, a_sizeunion, d_sizeunion);
/// assert_eq!(d_sizeunion.get(), 753086420246913575308642024691357_usize);
/// 
/// for i in 0..2
///     { println!("d_sizeunion.get_ulong_({}) = {}", i, d_sizeunion.get_ulong_(i)); }
/// assert_eq!(d_sizeunion.get_ulong_(0), 14084888390109238941_u64);
/// assert_eq!(d_sizeunion.get_ulong_(1), 40824896645051_u64);
/// 
/// for i in 0..4
///     { println!("d_sizeunion.get_uint_({}) = {}", i, d_sizeunion.get_uint_(i)); }
/// assert_eq!(d_sizeunion.get_uint_(0), 2843481757_u32);
/// assert_eq!(d_sizeunion.get_uint_(1), 3279393629_u32);
/// assert_eq!(d_sizeunion.get_uint_(2), 1232496571_u32);
/// assert_eq!(d_sizeunion.get_uint_(3), 9505_u32);
/// 
/// for i in 0..8
///     { println!("d_sizeunion.get_ushort_({}) = {}", i, d_sizeunion.get_ushort_(i)); }
/// assert_eq!(d_sizeunion.get_ushort_(0), 5789_u16);
/// assert_eq!(d_sizeunion.get_ushort_(1), 43388_u16);
/// assert_eq!(d_sizeunion.get_ushort_(2), 37725_u16);
/// assert_eq!(d_sizeunion.get_ushort_(3), 50039_u16);
/// assert_eq!(d_sizeunion.get_ushort_(4), 26555_u16);
/// assert_eq!(d_sizeunion.get_ushort_(5), 18806_u16);
/// assert_eq!(d_sizeunion.get_ushort_(6), 9505_u16);
/// assert_eq!(d_sizeunion.get_ushort_(7), 0_u16);
/// 
/// for i in 0..16
///     { println!("d_sizeunion.get_ubyte_({}) = {}", i, d_sizeunion.get_ubyte_(i)); }
/// assert_eq!(d_sizeunion.get_ubyte_(0), 157_u8);
/// assert_eq!(d_sizeunion.get_ubyte_(1), 22_u8);
/// assert_eq!(d_sizeunion.get_ubyte_(2), 124_u8);
/// assert_eq!(d_sizeunion.get_ubyte_(3), 169_u8);
/// assert_eq!(d_sizeunion.get_ubyte_(4), 93_u8);
/// assert_eq!(d_sizeunion.get_ubyte_(5), 147_u8);
/// assert_eq!(d_sizeunion.get_ubyte_(6), 119_u8);
/// assert_eq!(d_sizeunion.get_ubyte_(7), 195_u8);
/// assert_eq!(d_sizeunion.get_ubyte_(8), 187_u8);
/// assert_eq!(d_sizeunion.get_ubyte_(9), 103_u8);
/// assert_eq!(d_sizeunion.get_ubyte_(10), 118_u8);
/// assert_eq!(d_sizeunion.get_ubyte_(11), 73_u8);
/// assert_eq!(d_sizeunion.get_ubyte_(12), 33_u8);
/// assert_eq!(d_sizeunion.get_ubyte_(13), 37_u8);
/// assert_eq!(d_sizeunion.get_ubyte_(14), 0_u8);
/// assert_eq!(d_sizeunion.get_ubyte_(15), 0_u8);
/// 
/// let e_sizeunion = d_sizeunion * 3_usize.into_sizeunion();
/// println!("{} * {} = {}", d_sizeunion, 3_usize.into_sizeunion(), e_sizeunion);
/// assert_eq!(e_sizeunion.get(), 2259259260740740725925926074074071_usize);
/// 
/// let f_sizeunion = c_sizeunion / 10_usize.into_sizeunion();
/// println!("{} / {} = {}", c_sizeunion, 10_usize.into_sizeunion(), f_sizeunion);
/// assert_eq!(f_sizeunion.get(), 99999999999999999999999999999999_usize);
/// 
/// let g_sizeunion = c_sizeunion % 10_usize.into_sizeunion();
/// println!("{} % {} = {}", c_sizeunion, 10_usize.into_sizeunion(), g_sizeunion);
/// assert_eq!(g_sizeunion.get(), 9_usize);
/// ```
///  
/// # Big-endian issue
/// It is just experimental for Big Endian CPUs. So, you are not encouraged
/// to use it for serious purpose. Only use this crate for Big-endian CPUs
/// with your own full responsibility.
// #[cfg(target_pointer_width = "128")]
// #[derive(Copy, Clone)]
// #[allow(dead_code)]
// pub union SizeUnion
// {
//     /// The biggest unsigned element for compatibility with other unions
//     this: usize,

//     /// The biggest signed element for compatibility with other unions
//     that: isize,

//     /// The usize type element whose size is the same as the SizeUnion
//     pub u_size: usize,

//     /// The isize type element whose size is the same as the SizeUnion
//     pub s_size: isize,

//     /// The biggest unsigned element which is 128-bit unsigned integer
//     pub ulonger: u128,

//     /// The biggest signed element which is 128-bit unsigned integer
//     pub slonger: i128,

//     /// The secondly biggest unsigned element array whose elements are
//     /// 64-bit unsigned integer
//     pub ulong: [u64; 2],

//     /// The secondly biggest unsigned element array whose elements are
//     /// 64-bit signed integer
//     pub slong: [i64; 2],

//     /// The thirdly biggest unsigned element array whose elements are
//     /// 32-bit unsigned integer
//     pub uint: [u32; 4],

//     /// The thirdly biggest unsigned element array whose elements are
//     /// 32-bit signed integer
//     pub sint: [i32; 4],

//     /// The fourthly biggest unsigned element array whose elements are
//     /// 16-bit unsigned integer
//     pub ushort: [u16; 8],

//     /// The fourthly biggest unsigned element array whose elements are
//     /// 16-bit signed integer
//     pub sshort: [i16; 8],

//     /// The fifthly biggest unsigned element array whose elements are
//     /// 8-bit unsigned integer
//     pub ubyte: [u8; 16],

//     /// The fifthly biggest unsigned element array whose elements are
//     /// 8-bit signed integer
//     pub sbyte: [i8; 16],
// }


/// # Introduction
/// This union `SizeUnion` is for slicing `usize` into two `u32`s, two `i32`s,
/// four `u16`s, four `i16`s, eight `u8`s and/or eight `i8`s.
/// 
/// Sometimes, for example, we need to slice `usize` data into two `u32` pieces
/// which include a higher four-byte word and a lower four-byte word, and/or
/// into  four `u16` pieces which include a first two-byte word, a second
/// two-byte word, a third two-byte word and a fourth two-byte word.
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
/// let a = SizeUnion::new_with_signed(-4781862973812896945_isize);
/// println!("a.get() = {}", a.get());
/// println!("a.get_signed() = {}", a.get_signed());
/// println!("a.get_usize() = {}", a.get_usize());
/// println!("a.get_ssize() = {}", a.get_ssize());
/// println!("a.get_ulong() = {}", a.get_ulong());
/// println!("a.get_slong() = {}", a.get_slong());
/// assert_eq!(a.get(), 13664881099896654671_usize);
/// assert_eq!(a.get_signed(), -4781862973812896945_isize);
/// assert_eq!(a.get_usize(), 13664881099896654671_usize);
/// assert_eq!(a.get_ssize(), -4781862973812896945_isize);
/// assert_eq!(a.get_ulong(), 13664881099896654671_u64);
/// assert_eq!(a.get_slong(), -4781862973812896945_i64);
/// 
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
/// assert_eq!(a.get_uint_(0), 4048161615_u32);
/// assert_eq!(a.get_uint_(1), 3181603061_u32);
/// assert_eq!(a.get_sint_(0), -246805681_i32);
/// assert_eq!(a.get_sint_(1), -1113364235_i32);
/// assert_eq!(a.get_ushort_(0), 2895_u16);
/// assert_eq!(a.get_ushort_(1), 61770_u16);
/// assert_eq!(a.get_ushort_(2), 26869_u16);
/// assert_eq!(a.get_ushort_(3), 48547_u16);
/// assert_eq!(a.get_sshort_(0), 2895_i16);
/// assert_eq!(a.get_sshort_(1), -3766_i16);
/// assert_eq!(a.get_sshort_(2), 26869_i16);
/// assert_eq!(a.get_sshort_(3), -16989_i16);
/// assert_eq!(a.get_ubyte_(0), 79_u8);
/// assert_eq!(a.get_ubyte_(1), 11_u8);
/// assert_eq!(a.get_ubyte_(2), 74_u8);
/// assert_eq!(a.get_ubyte_(3), 241_u8);
/// assert_eq!(a.get_ubyte_(4), 245_u8);
/// assert_eq!(a.get_ubyte_(5), 104_u8);
/// assert_eq!(a.get_ubyte_(6), 163_u8);
/// assert_eq!(a.get_ubyte_(7), 189_u8);
/// assert_eq!(a.get_sbyte_(0), 79_i8);
/// assert_eq!(a.get_sbyte_(1), 11_i8);
/// assert_eq!(a.get_sbyte_(2), 74_i8);
/// assert_eq!(a.get_sbyte_(3), -15_i8);
/// assert_eq!(a.get_sbyte_(4), -11_i8);
/// assert_eq!(a.get_sbyte_(5), 104_i8);
/// assert_eq!(a.get_sbyte_(6), -93_i8);
/// assert_eq!(a.get_sbyte_(7), -67_i8);
/// ```
/// You can use `SizeUnion` as if you used `usize`. You can perform all kinds
/// of arithmetic operations such as addition, subtraction, multiplication,
/// and division (div and rem), and other operations which are available for
/// `usize`. If you use `SizeUnion` with the help of `SmallUInt`, it will be
/// even more powerful and convenient. In this case, you don't even have to
/// import (use) `cryptocol::number::SizeUnion`.
/// 
/// ## Example 3
/// ```
/// use cryptocol::number::SmallUInt;
/// 
/// let a_sizeunion = 12345678987654321_usize.into_sizeunion();
/// let b_sizeunion = 87654321012345678_usize.into_sizeunion();
/// let c_sizeunion = a_sizeunion.wrapping_add(b_sizeunion);
/// 
/// println!("{} + {} = {}", a_sizeunion, b_sizeunion, c_sizeunion);
/// assert_eq!(c_sizeunion.get(), 99999999999999999_usize);
/// 
/// for i in 0..2
///     { println!("c_sizeunion.get_uint_({}) = {}", i, c_sizeunion.get_uint_(i)); }
/// assert_eq!(c_sizeunion.get_uint_(0), 1569325055_u32);
/// assert_eq!(c_sizeunion.get_uint_(1), 23283064_u32);
/// 
/// for i in 0..4
///     { println!("c_sizeunion.get_ushort_({}) = {}", i, c_sizeunion.get_ushort_(i)); }
/// assert_eq!(c_sizeunion.get_ushort_(0), 65535_u16);
/// assert_eq!(c_sizeunion.get_ushort_(1), 23945_u16);
/// assert_eq!(c_sizeunion.get_ushort_(2), 17784_u16);
/// assert_eq!(c_sizeunion.get_ushort_(3), 355_u16);
///
/// for i in 0..8
///     { println!("c_sizeunion.get_ubyte_({}) = {}", i, c_sizeunion.get_ubyte_(i)); }
/// assert_eq!(c_sizeunion.get_ubyte_(0), 255_u8);
/// assert_eq!(c_sizeunion.get_ubyte_(1), 255_u8);
/// assert_eq!(c_sizeunion.get_ubyte_(2), 137_u8);
/// assert_eq!(c_sizeunion.get_ubyte_(3), 93_u8);
/// assert_eq!(c_sizeunion.get_ubyte_(4), 120_u8);
/// assert_eq!(c_sizeunion.get_ubyte_(5), 69_u8);
/// assert_eq!(c_sizeunion.get_ubyte_(6), 99_u8);
/// assert_eq!(c_sizeunion.get_ubyte_(7), 1_u8);
/// 
/// let d_sizeunion = b_sizeunion - a_sizeunion;
/// println!("{} - {} = {}", b_sizeunion, a_sizeunion, d_sizeunion);
/// assert_eq!(d_sizeunion.get(), 75308642024691357_usize);
///
/// for i in 0..2
///     { println!("d_longunion.get_uint_({}) = {}", i, d_sizeunion.get_uint_(i)); }
/// assert_eq!(d_sizeunion.get_uint_(0), 2556827293_u32);
/// assert_eq!(d_sizeunion.get_uint_(1), 17534159_u32);
///
/// for i in 0..4
///     { println!("d_sizeunion.get_ushort_({}) = {}", i, d_sizeunion.get_ushort_(i)); }
/// assert_eq!(d_sizeunion.get_ushort_(0), 5789_u16);
/// assert_eq!(d_sizeunion.get_ushort_(1), 39014_u16);
/// assert_eq!(d_sizeunion.get_ushort_(2), 36047_u16);
/// assert_eq!(d_sizeunion.get_ushort_(3), 267_u16);
///
/// for i in 0..8
///     { println!("d_sizeunion.get_ubyte_({}) = {}", i, d_sizeunion.get_ubyte_(i)); }
/// assert_eq!(d_sizeunion.get_ubyte_(0), 157_u8);
/// assert_eq!(d_sizeunion.get_ubyte_(1), 22_u8);
/// assert_eq!(d_sizeunion.get_ubyte_(2), 102_u8);
/// assert_eq!(d_sizeunion.get_ubyte_(3), 152_u8);
/// assert_eq!(d_sizeunion.get_ubyte_(4), 207_u8);
/// assert_eq!(d_sizeunion.get_ubyte_(5), 140_u8);
/// assert_eq!(d_sizeunion.get_ubyte_(6), 11_u8);
/// assert_eq!(d_sizeunion.get_ubyte_(7), 1_u8);
/// 
/// let e_sizeunion = d_sizeunion * 3_usize.into_sizeunion();
/// println!("{} * {} = {}", d_sizeunion, 3_usize.into_sizeunion(), e_sizeunion);
/// assert_eq!(e_sizeunion.get(), 225925926074074071_usize);
/// 
/// let f_sizeunion = c_sizeunion / 10_usize.into_sizeunion();
/// println!("{} / {} = {}", c_sizeunion, 10_usize.into_sizeunion(), f_sizeunion);
/// assert_eq!(f_sizeunion.get(), 9999999999999999_usize);
/// 
/// let g_sizeunion = c_sizeunion % 10_usize.into_sizeunion();
/// println!("{} % {} = {}", c_sizeunion, 10_usize.into_sizeunion(), g_sizeunion);
/// assert_eq!(g_sizeunion.get(), 9_usize);
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
/// This union `SizeUnion` is for slicing `usize` into two `u16`s, two `i16`s,
/// four `u8`s, and/or four `i8`s.
/// 
/// Sometimes, for example, we need to slice `usize` data into two `u16` pieces
/// which include a higher two-byte word and a lower two-byte word, and/or
/// into  four `u8` pieces which include a first byte, a second byte, a third
/// byte, and a fourth byte. In that case, `SizeUnion` will be very helpful.
/// 
/// # Quick Start
/// In order to use this union, you have to import (use)
/// `cryptocol::number::SizeUnion` as follows.
/// 
/// ## Example 1
/// ```
/// use cryptocol::number::SizeUnion;
/// ```
/// You can use the methods `get()`, `get_signed()`, `get_uint()`, and
/// `get_sint()` in order to obtain the data of `usize` in various types.
/// And, you can also slice the data of `usize` into two `u16` type data
/// or two `i16` type data by using the methods `get_uint()`, `get_sint()`,
/// `get_uint_()`, and `get_sint_()`. Or, you can also slice the data of
/// `usize` into four `u8` type data or four `i8` type data by using the
/// methods `get_ubyte()`, `get_sbyte()`, `get_ubyte_()`, and `get_sbyte_()`.
/// 
/// ## Example 2
/// ```
/// use cryptocol::number::SizeUnion;
/// 
/// let a = SizeUnion::new_with_signed(-246805681_isize);
/// 
/// println!("a.get() = {}", a.get());
/// println!("a.get_signed() = {}", a.get_signed());
/// println!("a.get_usize() = {}", a.get_usize());
/// println!("a.get_ssize() = {}", a.get_ssize());
/// println!("a.get_uint() = {}", a.get_uint());
/// println!("a.get_sint() = {}", a.get_sint());
/// assert_eq!(a.get(), 4048161615_usize);
/// assert_eq!(a.get_signed(), -246805681_isize);
/// assert_eq!(a.get_usize(), 4048161615_usize);
/// assert_eq!(a.get_ssize(), -246805681_isize);
/// assert_eq!(a.get_uint(), 4048161615_u32);
/// assert_eq!(a.get_sint(), -246805681_i32);
/// 
/// for i in 0..2
///     { println!("a.get_ushort_({}) = {}", i, a.get_ushort_(i)); }
/// for i in 0..2
///     { println!("a.get_sshort_({}) = {}", i, a.get_sshort_(i)); }
/// for i in 0..4
///     { println!("a.get_ubyte_({}) = {}", i, a.get_ubyte_(i)); }
/// for i in 0..4
///     { println!("a.get_sbyte_({}) = {}", i, a.get_sbyte_(i)); }
/// assert_eq!(a.get_ushort_(0), 2895_u16);
/// assert_eq!(a.get_ushort_(1), 61770_u16);
/// assert_eq!(a.get_sshort_(0), 2895_i16);
/// assert_eq!(a.get_sshort_(1), -3766_i16);
/// assert_eq!(a.get_ubyte_(0), 79_u8);
/// assert_eq!(a.get_ubyte_(1), 11_u8);
/// assert_eq!(a.get_ubyte_(2), 74_u8);
/// assert_eq!(a.get_ubyte_(3), 241_u8);
/// assert_eq!(a.get_sbyte_(0), 79_i8);
/// assert_eq!(a.get_sbyte_(1), 11_i8);
/// assert_eq!(a.get_sbyte_(2), 74_i8);
/// assert_eq!(a.get_sbyte_(3), -15_i8);
/// ```
/// You can use `SizeUnion` as if you used `usize`. You can perform all kinds
/// of arithmetic operations such as addition, subtraction, multiplication,
/// and division (div and rem), and other operations which are available for
/// `usize`. If you use `SizeUnion` with the help of `SmallUInt`, it will be
/// even more powerful and convenient. In this case, you don't even have to
/// import (use) `cryptocol::number::SizeUnion`.
/// 
/// ## Example 3
/// ```
/// use cryptocol::number::SmallUInt;
/// 
/// let a_sizeunion = 12345678_usize.into_sizeunion();
/// let b_sizeunion = 87654321_usize.into_sizeunion();
/// let c_sizeunion = a_sizeunion.wrapping_add(b_sizeunion);
/// 
/// println!("{} + {} = {}", a_sizeunion, b_sizeunion, c_sizeunion);
/// assert_eq!(c_sizeunion.get(), 99999999_usize);
/// 
/// for i in 0..2
///     { println!("c_sizeunion.get_ushort_({}) = {}", i, c_sizeunion.get_ushort_(i)); }
/// assert_eq!(c_sizeunion.get_ushort_(0), 57599_u16);
/// assert_eq!(c_sizeunion.get_ushort_(1), 1525_u16);
/// 
/// for i in 0..4
///     { println!("c_sizeunion.get_ubyte_({}) = {}", i, c_sizeunion.get_ubyte_(i)); }
/// assert_eq!(c_sizeunion.get_ubyte_(0), 255_u8);
/// assert_eq!(c_sizeunion.get_ubyte_(1), 224_u8);
/// assert_eq!(c_sizeunion.get_ubyte_(2), 245_u8);
/// assert_eq!(c_sizeunion.get_ubyte_(3), 5_u8);
/// 
/// let d_sizeunion = b_sizeunion - a_sizeunion;
/// println!("{} - {} = {}", b_sizeunion, a_sizeunion, d_sizeunion);
/// assert_eq!(d_sizeunion.get(), 75308643_usize);
/// 
/// for i in 0..2
///     { println!("d_sizeunion.get_ushort_({}) = {}", i, d_sizeunion.get_ushort_(i)); }
/// assert_eq!(d_sizeunion.get_ushort_(0), 7779_u16);
/// assert_eq!(d_sizeunion.get_ushort_(1), 1149_u16);
///
/// for i in 0..4
///     { println!("d_sizeunion.get_ubyte_({}) = {}", i, d_sizeunion.get_ubyte_(i)); }
/// assert_eq!(d_sizeunion.get_ubyte_(0), 99_u8);
/// assert_eq!(d_sizeunion.get_ubyte_(1), 30_u8);
/// assert_eq!(d_sizeunion.get_ubyte_(2), 125_u8);
/// assert_eq!(d_sizeunion.get_ubyte_(3), 4_u8);
/// 
/// let e_sizeunion = d_sizeunion * 3_usize.into_sizeunion();
/// println!("{} * {} = {}", d_sizeunion, 3_usize.into_sizeunion(), e_sizeunion);
/// assert_eq!(e_sizeunion.get(), 225925929_usize);
/// 
/// let f_sizeunion = c_sizeunion / 10_usize.into_sizeunion();
/// println!("{} / {} = {}", c_sizeunion, 10_usize.into_sizeunion(), f_sizeunion);
/// assert_eq!(f_sizeunion.get(), 9999999_usize);
/// 
/// let g_sizeunion = c_sizeunion % 10_usize.into_sizeunion();
/// println!("{} % {} = {}", c_sizeunion, 10_usize.into_sizeunion(), g_sizeunion);
/// assert_eq!(g_sizeunion.get(), 9_usize);
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
/// This union `SizeUnion` is for slicing `usize` into two `u8`s, and/or
/// two `i8`s.
/// 
/// Sometimes, for example, we need to slice `usize` data into two `u8` pieces
/// which include a higher byte and a lower byte.
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
/// You can use the methods `get()`, `get_signed()`, `get_ushort()`, and
/// `get_sshort()` in order to obtain the data of `usize` in various types.
/// And, you can also slice the data of `usize` into two `u8` type data
/// or two `i8` type data by using the methods `get_ubyte()`, `get_sbyte()`,
/// `get_ubyte_()`, and `get_sbyte_()`.
/// 
/// ## Example 2
/// ```
/// use cryptocol::number::SizeUnion;
/// 
/// let a = SizeUnion::new_with_signed(2895_isize);
/// 
/// println!("a.get() = {}", a.get());
/// println!("a.get_signed() = {}", a.get_signed());
/// println!("a.get_usize() = {}", a.get_usize());
/// println!("a.get_ssize() = {}", a.get_ssize());
/// println!("a.get_ushort() = {}", a.get_ushort());
/// println!("a.get_sshort() = {}", a.get_sshort());
/// assert_eq!(a.get(), 2895_usize);
/// assert_eq!(a.get_signed(), 2895_isize);
/// assert_eq!(a.get_usize(), 2895_usize);
/// assert_eq!(a.get_ssize(), 2895_isize);
/// assert_eq!(a.get_ushort(), 2895_u16);
/// assert_eq!(a.get_sshort(), 2895_i16);
/// 
/// for i in 0..2
///     { println!("a.get_ubyte_({}) = {}", i, a.get_ubyte_(i)); }
/// for i in 0..2
///     { println!("a.get_sbyte_({}) = {}", i, a.get_sbyte_(i)); }
/// assert_eq!(a.get_ubyte_(0), 79_u8);
/// assert_eq!(a.get_ubyte_(1), 11_u8);
/// assert_eq!(a.get_sbyte_(0), 79_i8);
/// assert_eq!(a.get_sbyte_(1), 11_i8);
/// ```
/// You can use `SizeUnion` as if you used `usize`. You can perform all kinds
/// of arithmetic operations such as addition, subtraction, multiplication,
/// and division (div and rem), and other operations which are available for
/// `usize`. If you use `SizeUnion` with the help of `SmallUInt`, it will be
/// even more powerful and convenient. In this case, you don't even have to
/// import (use) `cryptocol::number::SizeUnion`.
/// 
/// ## Example 3
/// ```
/// use cryptocol::number::SmallUInt;
/// 
/// let a_sizeunion = 1234_usize.into_sizeunion();
/// let b_sizeunion = 4321_usize.into_sizeunion();
/// let c_sizeunion = a_sizeunion.wrapping_add(b_sizeunion);
/// 
/// println!("{} + {} = {}", a_sizeunion, b_sizeunion, c_sizeunion);
/// assert_eq!(c_sizeunion.get(), 5555_usize);
/// 
/// for i in 0..2
///     { println!("c_sizeunion.get_ubyte_({}) = {}", i, c_sizeunion.get_ubyte_(i)); }
/// assert_eq!(c_sizeunion.get_ubyte_(0), 179_u8);
/// assert_eq!(c_sizeunion.get_ubyte_(1), 21_u8);
/// 
/// let d_sizeunion = b_sizeunion - a_sizeunion;
/// println!("{} - {} = {}", b_sizeunion, a_sizeunion, d_sizeunion);
/// assert_eq!(d_sizeunion.get(), 3087_usize);
///
/// for i in 0..2
///     { println!("d_sizeunion.get_ubyte_({}) = {}", i, d_sizeunion.get_ubyte_(i)); }
/// assert_eq!(d_sizeunion.get_ubyte_(0), 15_u8);
/// assert_eq!(d_sizeunion.get_ubyte_(1), 12_u8);
/// 
/// let e_sizeunion = d_sizeunion * 3_usize.into_sizeunion();
/// println!("{} * {} = {}", d_sizeunion, 3_usize.into_sizeunion(), e_sizeunion);
/// assert_eq!(e_sizeunion.get(), 9261_usize);
/// 
/// let f_sizeunion = c_sizeunion / 10_usize.into_sizeunion();
/// println!("{} / {} = {}", c_sizeunion, 10_usize.into_sizeunion(), f_sizeunion);
/// assert_eq!(f_shortunion.get(), 555_usize);
/// 
/// let g_sizeunion = c_sizeunion % 10_usize.into_sizeunion();
/// println!("{} % {} = {}", c_sizeunion, 10_usize.into_sizeunion(), g_sizeunion);
/// assert_eq!(g_sizeunion.get(), 5_usize);
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


// /// # Introduction
// /// This union `SizeUnion` is for using with `u8` or `i8`.
// /// 
// /// # Quick Start
// /// In order to use this union, you have to import (use)
// /// `cryptocol::number::SizeUnion` as follows.
// /// 
// /// ## Example 1
// /// ```
// /// use cryptocol::number::SizeUnion;
// /// ```
// /// You can use the methods `get()`, `get_signed()`, `get_ubyte()`, and
// /// `get_sbyte()` in order to obtain the data of `usize` in various types.
// /// 
// /// ## Example 2
// /// ```
// /// use cryptocol::number::SizeUnion;
// /// 
// /// let a = SizeUnion::new_with_signed(79_isize);
// /// 
// /// println!("a.get() = {}", a.get());
// /// println!("a.get_signed() = {}", a.get_signed());
// /// println!("a.get_usize() = {}", a.get_usize());
// /// println!("a.get_ssize() = {}", a.get_ssize());
// /// println!("a.get_ubyte() = {}", a.get_ubyte());
// /// println!("a.get_sbyte() = {}", a.get_sbyte());
// /// assert_eq!(a.get(), 79_usize);
// /// assert_eq!(a.get_signed(), 79_isize);
// /// assert_eq!(a.get_usize(), 79_usize);
// /// assert_eq!(a.get_ssize(), 79_isize);
// /// assert_eq!(a.get_ubyte(), 79_u8);
// /// assert_eq!(a.get_sbyte(), 79_u8);
// /// ```
// /// You can use `SizeUnion` as if you used `usize`. You can perform all kinds
// /// of arithmetic operations such as addition, subtraction, multiplication,
// /// and division (div and rem), and other operations which are available for
// /// `usize`. If you use `SizeUnion` with the help of `SmallUInt`, it will be
// /// even more powerful and convenient. In this case, you don't even have to
// /// import (use) `cryptocol::number::SizeUnion`.
// /// 
// /// ## Example 3
// /// ```
// /// use cryptocol::number::SmallUInt;
// /// 
// /// let a_sizeunion = 12_usize.into_sizeunion();
// /// let b_sizeunion = 87_usize.into_sizeunion();
// /// let c_sizeunion = a_sizeunion.wrapping_add(b_sizeunion);
// /// 
// /// println!("{} + {} = {}", a_sizeunion, b_sizeunion, c_sizeunion);
// /// assert_eq!(c_sizeunion.get(), 55_usize);
// /// 
// /// let d_sizeunion = b_sizeunion - a_sizeunion;
// /// println!("{} - {} = {}", b_sizeunion, a_sizeunion, d_sizeunion);
// /// assert_eq!(d_sizeunion.get(), 75_usize);
// /// 
// /// let e_sizeunion = d_sizeunion * 3_usize.into_sizeunion();
// /// println!("{} * {} = {}", d_sizeunion, 3_usize.into_sizeunion(), e_sizeunion);
// /// assert_eq!(e_sizeunion.get(), 225_usize);
// /// 
// /// let f_sizeunion = c_sizeunion / 10_usize.into_sizeunion();
// /// println!("{} / {} = {}", c_sizeunion, 10_usize.into_sizeunion(), f_sizeunion);
// /// assert_eq!(f_shortunion.get(), 9_usize);
// /// 
// /// let g_sizeunion = c_sizeunion % 10_usize.into_sizeunion();
// /// println!("{} % {} = {}", c_sizeunion, 10_usize.into_sizeunion(), g_sizeunion);
// /// assert_eq!(g_sizeunion.get(), 9_usize);
// /// ```
// ///  
// /// # Big-endian issue
// /// It is just experimental for Big Endian CPUs. So, you are not encouraged
// /// to use it for serious purpose. Only use this crate for Big-endian CPUs
// /// with your own full responsibility.
// #[cfg(target_pointer_width = "8")]
// #[derive(Copy, Clone)]
// #[allow(dead_code)]
// #[allow(non_camel_case_types)]
// pub union SizeUnion
// {
//     /// The biggest unsigned element for compatibility with other unions
//     pub this: usize,

//     /// The biggest signed element for compatibility with other unions
//     pub that: isize,

//     /// The usize type element whose size is the same as the SizeUnion
//     pub u_size: usize,

//     /// The isize type element whose size is the same as the SizeUnion
//     pub s_size: isize,

//     /// The biggest unsigned element which is 8-bit unsigned integer
//     pub ubyte: u8,

//     /// The biggest signed element which is 8-bit unsigned integer
//     pub sbyte: i8,
// }


impl SizeUnion
{
    // pub fn new() -> Self
    /// Constructs a new `SizeUnion`.
    /// 
    /// # Output
    /// A new object of `SizeUnion`.
    /// 
    /// # Initialization
    /// All the fields of the constructed object will be
    /// initialized with `0`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::number::SizeUnion;    
    /// let a = SizeUnion::new();
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 0_usize);
    /// ```
    #[inline] pub fn new() -> Self  { Self { u_size: 0 } }

    // pub fn new_with(u_size: usize) -> Self
    /// Constructs a new `SizeUnion` with initializing it with `u_size`.
    /// 
    /// # Output
    /// A new object of `SizeUnion` initialized with the value `u_size`.
    /// 
    /// # Initialization
    /// The field of the constructed object will be initialized with `u_size`.
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::SizeUnion;    
    /// let a = SizeUnion::new_with(234_usize);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 234_usize);
    /// ```
    #[inline] pub fn new_with(u_size: usize) -> Self  { Self { u_size } }

    // pub fn new_with_signed(s_size: isize) -> Self
    /// Constructs a new `SizeUnion` with initializing it with `isize`.
    /// 
    /// # Output
    /// A new object of `SizeUnion` initialized with the value `isize`.
    /// 
    /// # Initialization
    /// The field of the constructed object will be initialized with `isize`.
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::SizeUnion;    
    /// let a = SizeUnion::new_with_signed(-123_isize);
    /// println!("a = {}", a.get_signed());
    /// assert_eq!(a.get_signed(), -123_isize);
    /// ```
    #[inline] pub fn new_with_signed(s_size: isize) -> Self     { Self { s_size } }

    crate::number::new_with_small_uint!();

    // pub fn new_with_u128(num: u128) -> Self
    /// Constructs a new `SizeUnion` with initializing it with the lowest
    /// `usize`-length part of `num`.
    /// 
    /// # Output
    /// A new object of `SizeUnion` initialized with the value of
    /// the lowest `usize`-length part of `num`.
    /// 
    /// # Initialization
    /// The field of the constructed object will be initialized with
    /// the value of the lowest `usize`-length part of `num`.
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::SizeUnion;
    /// let a = SizeUnion::new_with_u128(123456789012345678901234567890123456789_u128);
    /// println!("a = {}", a.get());
    /// #[cfg(target_pointer_width = "128")]    assert_eq!(a.get(), 123456789012345678901234567890123456789_usize);
    /// #[cfg(target_pointer_width = "64")]     assert_eq!(a.get(), 12312739301371248917_usize);
    /// #[cfg(target_pointer_width = "32")]     assert_eq!(a.get(), 2923004181_usize);
    /// #[cfg(target_pointer_width = "16")]     assert_eq!(a.get(), 33045_usize);
    /// #[cfg(target_pointer_width = "8")]      assert_eq!(a.get(), 21_usize);
    /// ```
    #[inline] pub fn new_with_u128(num: u128) -> Self   { Self { u_size: num as usize } }

    // pub fn new_with_bool(b: bool) -> Self
    /// Constructs a new `SizeUnion` with initializing it with the value of `b`.
    /// 
    /// # Output
    /// A new object of `SizeUnion` initialized with the value of `b`
    /// 
    /// # Initialization
    /// The field of the constructed object will be initialized with
    /// the value of `b`.
    /// If `b` is `true`, `self` will have the value `1`.
    /// If `b` is `false`, `self` will have the value `0`.
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::SizeUnion;
    /// let a = SizeUnion::new_with_bool(true);
    /// let b = SizeUnion::new_with_bool(false);
    /// println!("a = {}", a.get());
    /// println!("b = {}", b.get());
    /// assert_eq!(a.get(), 1_usize);
    /// assert_eq!(b.get(), 0_usize);
    /// ```
    #[inline] pub fn new_with_bool(b: bool) -> Self     { Self { u_size: b as usize } }

    // pub fn get(self) -> usize
    /// Returns its value as `usize`.
    /// 
    /// # Output
    /// Its value as `usize`
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::SizeUnion;
    /// let a = SizeUnion::new_with(250_usize);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 250_usize);
    /// ```
    #[inline] pub fn get(self) -> usize     { unsafe { self.u_size } }

    // pub fn set(&mut self, val: usize)
    /// Sets its value with `val` of type `usize`
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::SizeUnion;    
    /// let mut a = SizeUnion::new();
    /// a.set(234_usize);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 234_usize);
    /// ```
    #[inline] pub fn set(&mut self, val: usize)     { self.u_size = val; }

    // pub fn get_signed(self) -> isize
    /// Returns its value as `isize`.
    /// 
    /// # Output
    /// Its value as `isize`
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::SizeUnion;    
    /// let a = SizeUnion::new_with_signed(-123_isize);
    /// println!("a = {}", a.get_signed());
    /// assert_eq!(a.get_signed(), -123_isize);
    /// ```
    #[inline] pub fn get_signed(self) -> isize      { unsafe { self.s_size } }

    // pub fn set_signed(&mut self, val: isize)
    /// Sets its value with `val` of type `isize`
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::SizeUnion;    
    /// let mut a = SizeUnion::new();
    /// a.set_signed(-123_isize);
    /// println!("a = {}", a.get_signed());
    /// assert_eq!(a.get_signed(), -123_isize);
    /// ```
    #[inline] pub fn set_signed(&mut self, val: isize)  { self.s_size = val; }

    crate::number::get_set_size_fit!();

    // #[cfg(target_pointer_width = "128")]    crate::number::get_set_byte!(16);
    #[cfg(target_pointer_width = "64")]     crate::number::get_set_byte!(8);
    #[cfg(target_pointer_width = "32")]     crate::number::get_set_byte!(4);
    #[cfg(target_pointer_width = "16")]     crate::number::get_set_byte!(2);
    // #[cfg(target_pointer_width = "8")]      crate::number::get_set_byte_fit!();

    // #[cfg(target_pointer_width = "128")]    crate::number::get_set_short!(8);
    #[cfg(target_pointer_width = "64")]     crate::number::get_set_short!(4);
    #[cfg(target_pointer_width = "32")]     crate::number::get_set_short!(2);
    #[cfg(target_pointer_width = "16")]     crate::number::get_set_short_fit!();

    // #[cfg(target_pointer_width = "128")]    crate::number::get_set_int!(4);
    #[cfg(target_pointer_width = "64")]     crate::number::get_set_int!(2);
    #[cfg(target_pointer_width = "32")]     crate::number::get_set_int_fit!();

    // #[cfg(target_pointer_width = "128")]    crate::number::get_set_long!(2);
    #[cfg(target_pointer_width = "64")]     crate::number::get_set_long_fit!();

    // #[cfg(target_pointer_width = "128")]    crate::number::get_set_longer_fit!();

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

crate::number::format_for_integer_unions_impl! { SizeUnion }


crate::number::shift_ops_for_integer_unions_by_union_impl! { SizeUnion, ShortUnion }
crate::number::shift_ops_for_integer_unions_by_union_impl! { SizeUnion, IntUnion }
crate::number::shift_ops_for_integer_unions_by_union_impl! { SizeUnion, LongUnion }
crate::number::shift_ops_for_integer_unions_by_union_impl! { SizeUnion, LongerUnion }
crate::number::shift_ops_for_integer_unions_by_union_impl! { SizeUnion, SizeUnion }

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

        // #[cfg(target_pointer_width = "128")]
        // ff.field("ulonger", unsafe { &self.ulonger } )
        //     .field("slonger", unsafe { &self.slonger } )
        //     .field("ulong", unsafe { &self.ulong } )
        //     .field("slong", unsafe { &self.slong } )
        //     .field("uint", unsafe { &self.uint } )
        //     .field("sint", unsafe { &self.sint } )
        //     .field("ushort", unsafe { &self.ushort } )
        //     .field("sshort", unsafe { &self.sshort } )
        //     .field("ubyte", unsafe { &self.ubyte } )
        //     .field("sbyte", unsafe { &self.sbyte } );

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

        // #[cfg(target_pointer_width = "8")]
        // ff.field("ubyte", unsafe { &self.ubyte } )
        //     .field("sbyte", unsafe { &self.sbyte } );

        ff.finish()
    }
}
