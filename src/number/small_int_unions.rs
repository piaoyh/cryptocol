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

use std::fmt::{ Debug, Display };
use std::mem::size_of;
use std::cmp::{ PartialEq, PartialOrd };
use std::ops::*;
use super::small_uint::SmallUInt;

/// # Introduction
/// This union `ShortUnion` is for slicing `u16` into two `u8`s.
/// 
/// Sometimes, we need to slice `u16` data into two `u8` pieces which includes
/// a higher byte and a lower byte. In that purpose, `ShortUnion` will be very
/// helpful.
/// 
/// # Quick Start
/// In order to use this union, you have to import (use)
/// `cryptocol::number::ShortUnion` as follows.
/// 
/// ## Example 1
/// ```
/// use cryptocol::number::ShortUnion;
/// ```
/// You can use the methods `get()`, `get_signed()`, `get_ushort()`, and
/// `get_sshort()` in order to obtain the data of `u16` in various types.
/// And, you can also slice the data of `u16` into two `u8` type data by
/// using the methods `get_ubyte()`, `get_sbyte()`, `get_ubyte_()`, and
/// `get_sbyte_()`. If your machine does not have 8-bit CPU or 16-bit CPU,
/// `ShortUnion` does not have the method `get_usize()` nor `get_ssize()`.
/// 
/// ## Example 2
/// ```
/// use cryptocol::number::ShortUnion;
/// let a = ShortUnion::new_with(55468_u16);
/// println!("a.get() = {}", a.get());
/// println!("a.get_signed() = {}", a.get_signed());
/// println!("a.get_ushort() = {}", a.get_ushort());
/// println!("a.get_sshort() = {}", a.get_sshort());
/// assert_eq!(a.get(), 55468_u16);
/// assert_eq!(a.get_signed(), -10068_i16);
/// assert_eq!(a.get_ushort(), 55468_u16);
/// assert_eq!(a.get_sshort(), -10068_i16);
/// 
/// for i in 0..2
///     { println!("a.get_ubyte_({}) = {}", i, a.get_ubyte_(i)); }
/// for i in 0..2
///     { println!("a.get_sbyte_({}) = {}", i, a.get_sbyte_(i)); }
/// assert_eq!(a.get_ubyte_(0), 172_u8);
/// assert_eq!(a.get_ubyte_(1), 216_u8);
/// assert_eq!(a.get_sbyte_(0), -84_i8);
/// assert_eq!(a.get_sbyte_(1), -40_i8);
/// 
/// #[cfg(target_pointer_width = "8")]
/// {
///     const N: usize = 2;
///     for i in 0..N
///         { println!("a.get_usize_({}) = {}", i, a.get_usize_(i)); }
///     for i in 0..N
///         { println!("a.get_ssize_({}) = {}", i, a.get_ssize_(i)); }
///     assert_eq!(a.get_usize_(0), 172_u8);
///     assert_eq!(a.get_usize_(1), 216_u8);
///     assert_eq!(a.get_usize_(0), -84_i8);
///     assert_eq!(a.get_usize_(1), -40_i8);
/// }
/// #[cfg(target_pointer_width = "16")]
/// {
///     println!("a.get_usize() = {}", a.get_usize());
///     println!("a.get_ssize() = {}", a.get_ssize());
///     assert_eq!(a.get_usize(), 55468_u16);
///     assert_eq!(a.get_ssize(), -10068_i16);
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
/// let a_shortunion = 1234_u16.into_shortunion();
/// let b_shortunion = 4321_u16.into_shortunion();
/// let c_shortunion = a_shortunion.wrapping_add(b_shortunion);
/// println!("{} + {} = {}", a_shortunion, b_shortunion, c_shortunion);
/// assert_eq!(c_shortunion.get(), 5555_u16);
/// for i in 0..2
///     { println!("c_shortunion.get_ubyte_({}) = {}", i, c_shortunion.get_ubyte_(i)); }
/// assert_eq!(c_shortunion.get_ubyte_(0), 179_u8);
/// assert_eq!(c_shortunion.get_ubyte_(1), 21_u8);
/// 
/// let d_shortunion = b_shortunion - a_shortunion;
/// println!("{} - {} = {}", b_shortunion, a_shortunion, d_shortunion);
/// assert_eq!(d_shortunion.get(), 3087_u16);
/// for i in 0..2
///     { println!("d_shortunion.get_ubyte_({}) = {}", i, d_shortunion.get_ubyte_(i)); }
/// assert_eq!(d_shortunion.get_ubyte_(0), 15_u8);
/// assert_eq!(d_shortunion.get_ubyte_(1), 12_u8);
/// 
/// let e_shortunion = d_shortunion * 3_u16.into_shortunion();
/// println!("{} * {} = {}", d_shortunion, 3_u16.into_shortunion(), e_shortunion);
/// assert_eq!(e_shortunion.get(), 9261_u16);
/// 
/// let f_shortunion = c_shortunion / 10_u16.into_shortunion();
/// println!("{} / {} = {}", c_shortunion, 10_u16.into_shortunion(), f_shortunion);
/// assert_eq!(f_shortunion.get(), 555_u16);
/// 
/// let g_shortunion = c_shortunion % 10_u16.into_shortunion();
/// println!("{} % {} = {}", c_shortunion, 10_u16.into_shortunion(), g_shortunion);
/// assert_eq!(g_shortunion.get(), 5_u16);
/// ```
/// 
/// # Big-endian issue
/// It is just experimental for big-endian CPUs. So, you are not encouraged
/// to use it for big-endian CPUs for serious purpose.
/// Only use this crate for big-endian CPUs with your own full responsibility.
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub union ShortUnion
{
    /// The biggest unsigned element for compatibility with other unions
    this: u16,

    /// The biggest signed element for compatibility with other unions
    that: i16,

    /// The biggest unsigned element which is 16-bit unsigned integer
    ushort: u16,

    /// The biggest signed element which is 16-bit unsigned integer
    sshort: i16,

    /// The secondly biggest unsigned element array whose elements are
    /// 8-bit unsigned integer
    ubyte: [u8; 2],

    /// The secondly biggest signed element array whose elements are
    /// 8-bit unsigned integer
    sbyte: [i8; 2],

    /// The usize type element whose size is the same as the ShortUnion
    #[cfg(target_pointer_width = "16")] pub u_size: usize,

    /// The isize type element whose size is the same as the ShortUnion
    #[cfg(target_pointer_width = "16")] pub s_size: isize,

    /// The usize type array whose elements's size is 8-bit size
    #[cfg(target_pointer_width = "8")] pub u_size: [usize; 2],

    /// The isize type array whose elements's size is 8-bit size
    #[cfg(target_pointer_width = "8")] pub s_size: [isize; 2],
}


/// This union is for converting one primitive integral type into another
/// integeral type within 32-bit long type.
/// 
/// # Feature
/// All the fields are pubic but it is highly encouraged to get/set methods
/// instead of accessing to each field directly. The simple get/set methods are
/// all inline methods so that you hardly lose the performance benefit because
/// of using get/set methods.
/// 
/// # Example
/// ```
/// use cryptocol::number::*;
/// let a = IntUnion::new_with_signed(-454688546_i32);
/// println!("a.this = {}, {}", unsafe { a.this }, a.get());
/// println!("a.that = {}, {}", unsafe { a.that }, a.get_signed());
/// println!("a.uint = {}", unsafe { a.uint });
/// println!("a.sint = {}", unsafe { a.uint });
/// #[cfg(target_endian = "little")]
/// {
///     for i in 0..2
///         { println!("a.ushort[{}] = {}, {}", i, unsafe { a.ushort[i] }, a.get_ushort_(i)); }
///     for i in 0..2
///         { println!("a.sshort[{}] = {}, {}", i, unsafe { a.sshort[i] }, a.get_sshort_(i)); }
///     for i in 0..4
///         { println!("a.ubyte[{}] = {}, {}", i, unsafe { a.ubyte[i] }, a.get_ubyte_(i)); }
///     for i in 0..4
///         { println!("a.sbyte[{}] = {}, {}", i, unsafe { a.sbyte[i] }, a.get_sbyte_(i)); }
///     #[cfg(target_pointer_width = "16")]
///     {
///         const N: usize = 2;
///         for i in 0..N
///             { println!("a.u_size[{}] = {}, {}", i, unsafe { a.u_size[i] }, a.get_usize_(i)); }
///         for i in 0..N
///             { println!("a.s_size[{}] = {}, {}", i, unsafe { a.s_size[i] }, a.get_ssize_(i)); }
///     }
///     #[cfg(target_pointer_width = "8")]
///     {
///         const N: usize = 4;
///         for i in 0..N
///             { println!("a.u_size[{}] = {}, {}", i, unsafe { a.u_size[i] }, a.get_usize_(i)); }
///         for i in 0..N
///             { println!("a.s_size[{}] = {}, {}", i, unsafe { a.s_size[i] }, a.get_ssize_(i)); }
///     }
/// }
/// #[cfg(target_pointer_width = "32")]
/// {
///     println!("a.u_size = {}", unsafe { a.u_size });
///     println!("a.s_size = {}", unsafe { a.s_size });
/// }
/// 
/// assert_eq!(unsafe { a.this }, 3840278750_u32);
/// assert_eq!(unsafe { a.that }, -454688546_i32);
/// assert_eq!(unsafe { a.uint }, 3840278750_u32);
/// assert_eq!(unsafe { a.sint }, -454688546_i32);
/// #[cfg(target_endian = "little")]
/// {
///     assert_eq!(unsafe { a.ushort[0] }, 222_u16);
///     assert_eq!(unsafe { a.ushort[1] }, 58598_u16);
///     assert_eq!(unsafe { a.sshort[0] }, 222_i16);
///     assert_eq!(unsafe { a.sshort[1] }, -6938_i16);
///     assert_eq!(unsafe { a.ubyte[0] }, 222_u8);
///     assert_eq!(unsafe { a.ubyte[1] }, 0_u8);
///     assert_eq!(unsafe { a.ubyte[2] }, 230_u8);
///     assert_eq!(unsafe { a.ubyte[3] }, 228_u8);
///     assert_eq!(unsafe { a.sbyte[0] }, -34_i8);
///     assert_eq!(unsafe { a.sbyte[1] }, 0_i8);
///     assert_eq!(unsafe { a.sbyte[2] }, -26_i8);
///     assert_eq!(unsafe { a.sbyte[3] }, -28_i8);
/// }
/// ```
///  
/// # Big-endian issue
/// It is just experimental for Big Endian CPUs. So, you are not encouraged
/// to use it for serious purpose. Only use this crate for Big-endian CPUs
/// with your own full responsibility.
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub union IntUnion
{
    /// The biggest unsigned element for compatibility with other unions
    this: u32,

    /// The biggest signed element for compatibility with other unions
    that: i32,

    /// The biggest unsigned element which is 32-bit unsigned integer
    uint: u32,

    /// The biggest signed element which is 32-bit unsigned integer
    sint: i32,

    /// The secondly biggest unsigned element array whose elements are
    /// 16-bit unsigned integer
    ushort: [u16; 2],

    /// The secondly biggest signed element array whose elements are
    /// 16-bit unsigned integer
    sshort: [i16; 2],

    /// The thirdly biggest unsigned element array whose elements are
    /// 8-bit unsigned integer
    ubyte: [u8; 4],

    /// The thirdly biggest signed element array whose elements are
    /// 8-bit unsigned integer
    sbyte: [i8; 4],

    /// The usize type element whose size is the same as the IntUnion
    #[cfg(target_pointer_width = "32")] pub u_size: usize,

    /// The isize type element whose size is the same as the IntUnion
    #[cfg(target_pointer_width = "32")] pub s_size: isize,

    /// The usize type array whose elements's size is 16-bit size
    #[cfg(target_pointer_width = "16")] pub u_size: [usize; 2],

    /// The isize type array whose elements's size is 16-bit size
    #[cfg(target_pointer_width = "16")] pub s_size: [isize; 2],

    /// The usize type array whose elements's size is 8-bit size
    #[cfg(target_pointer_width = "8")] pub u_size: [usize; 4],

    /// The isize type array whose elements's size is 8-bit size
    #[cfg(target_pointer_width = "8")] pub s_size: [isize; 4],
}


/// This union is for converting one primitive integral type into another
/// integeral type within 64-bit long type.
/// 
/// # Feature
/// All the fields are pubic but it is highly encouraged to get/set methods
/// instead of accessing to each field directly. The simple get/set methods are
/// all inline methods so that you hardly lose the performance benefit because
/// of using get/set methods.
/// 
/// # Example
/// ```
/// use cryptocol::number::*;
/// let a = LongUnion::new_with_signed(-1234567890987645_i64);
/// println!("a.this = {}, {}", unsafe { a.this }, a.get());
/// println!("a.that = {}, {}", unsafe { a.that }, a.get_signed());
/// println!("a.ulong = {}", unsafe { a.ulong });
/// println!("a.slong = {}", unsafe { a.slong });
/// #[cfg(target_endian = "little")]
/// {
///     for i in 0..2
///         { println!("a.uint[{}] = {}, {}", i, unsafe { a.uint[i] }, a.get_uint_(i)); }
///     for i in 0..2
///         { println!("a.sint[{}] = {}, {}", i, unsafe { a.sint[i] }, a.get_sint_(i)); }
///     for i in 0..4
///         { println!("a.ushort[{}] = {}, {}", i, unsafe { a.ushort[i] }, a.get_ushort_(i)); }
///     for i in 0..4
///         { println!("a.sshort[{}] = {}, {}", i, unsafe { a.sshort[i] }, a.get_sshort_(i)); }
///     for i in 0..8
///         { println!("a.ubyte[{}] = {}, {}", i, unsafe { a.ubyte[i] }, a.get_ubyte_(i)); }
///     for i in 0..8
///         { println!("a.sbyte[{}] = {}, {}", i, unsafe { a.sbyte[i] }, a.get_sbyte_(i)); }
///     #[cfg(target_pointer_width = "32")]
///     {
///         const N: usize = 2;
///         for i in 0..N
///             { println!("a.u_size[{}] = {}, {}", i, unsafe { a.u_size[i] }, a.get_usize_(i)); }
///         for i in 0..N
///             { println!("a.s_size[{}] = {}, {}", i, unsafe { a.s_size[i] }, a.get_ssize_(i)); }
///     }
///     #[cfg(target_pointer_width = "16")]
///     {
///         const N: usize = 4;
///         for i in 0..N
///             { println!("a.u_size[{}] = {}, {}", i, unsafe { a.u_size[i] }, a.get_usize_(i)); }
///         for i in 0..N
///             { println!("a.s_size[{}] = {}, {}", i, unsafe { a.s_size[i] }, a.get_ssize_(i)); }
///     }
///     #[cfg(target_pointer_width = "8")]
///     {
///         const N: usize = 8;
///         for i in 0..N
///             { println!("a.u_size[{}] = {}, {}", i, unsafe { a.u_size[i] }, a.get_usize_(i)); }
///         for i in 0..N
///             { println!("a.s_size[{}] = {}, {}", i, unsafe { a.s_size[i] }, a.get_ssize_(i)); }
///     }
/// }
/// #[cfg(target_pointer_width = "64")]
/// {
///     println!("a.u_size = {}", unsafe { a.u_size });
///     println!("a.s_size = {}", unsafe { a.s_size });
/// }
/// 
/// assert_eq!(unsafe { a.this }, 18445509505818563971_u64);
/// assert_eq!(unsafe { a.that }, -1234567890987645_i64);
/// assert_eq!(unsafe { a.ulong }, 18445509505818563971_u64);
/// assert_eq!(unsafe { a.slong }, -1234567890987645_i64);
/// #[cfg(target_endian = "little")]
/// {
///     assert_eq!(unsafe { a.uint[0] }, 3278378371_u32);
///     assert_eq!(unsafe { a.uint[1] }, 4294679850_u32);
///     assert_eq!(unsafe { a.sint[0] }, -1016588925_i32);
///     assert_eq!(unsafe { a.sint[1] }, -287446_i32);
///     assert_eq!(unsafe { a.ushort[0] }, 5507_u16);
///     assert_eq!(unsafe { a.ushort[1] }, 50024_u16);
///     assert_eq!(unsafe { a.ushort[2] }, 40234_u16);
///     assert_eq!(unsafe { a.ushort[3] }, 65531_u16);
///     assert_eq!(unsafe { a.sshort[0] }, 5507_i16);
///     assert_eq!(unsafe { a.sshort[1] }, -15512_i16);
///     assert_eq!(unsafe { a.sshort[2] }, -25302_i16);
///     assert_eq!(unsafe { a.sshort[3] }, -5_i16);
///     assert_eq!(unsafe { a.ubyte[0] }, 131_u8);
///     assert_eq!(unsafe { a.ubyte[1] }, 21_u8);
///     assert_eq!(unsafe { a.ubyte[2] }, 104_u8);
///     assert_eq!(unsafe { a.ubyte[3] }, 195_u8);
///     assert_eq!(unsafe { a.ubyte[4] }, 42_u8);
///     assert_eq!(unsafe { a.ubyte[5] }, 157_u8);
///     assert_eq!(unsafe { a.ubyte[6] }, 251_u8);
///     assert_eq!(unsafe { a.ubyte[7] }, 255_u8);
///     assert_eq!(unsafe { a.sbyte[0] }, -125_i8);
///     assert_eq!(unsafe { a.sbyte[1] }, 21_i8);
///     assert_eq!(unsafe { a.sbyte[2] }, 104_i8);
///     assert_eq!(unsafe { a.sbyte[3] }, -61_i8);
///     assert_eq!(unsafe { a.sbyte[4] }, 42_i8);
///     assert_eq!(unsafe { a.sbyte[5] }, -99_i8);
///     assert_eq!(unsafe { a.sbyte[6] }, -5_i8);
///     assert_eq!(unsafe { a.sbyte[7] }, -1_i8);
/// }
/// ```
///  
/// # Big-endian issue
/// It is just experimental for Big Endian CPUs. So, you are not encouraged
/// to use it for serious purpose. Only use this crate for Big-endian CPUs
/// with your own full responsibility.
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub union LongUnion
{
    /// The biggest unsigned element for compatibility with other unions
    this: u64,

    /// The biggest signed element for compatibility with other unions
    that: i64,

    /// The biggest unsigned element which is 64-bit unsigned integer
    ulong: u64,

    /// The biggest signed element which is 64-bit unsigned integer
    slong: i64,

    /// The secondly biggest unsigned element array whose elements are
    /// 32-bit unsigned integer
    uint: [u32; 2],

    /// The secondly biggest signed element array whose elements are
    /// 32-bit unsigned integer
    sint: [i32; 2],

    /// The thirdly biggest unsigned element array whose elements are
    /// 16-bit unsigned integer
    ushort: [u16; 4],

    /// The thirdly biggest signed element array whose elements are
    /// 16-bit unsigned integer
    sshort: [i16; 4],

    /// The fourthly biggest unsigned element array whose elements are
    /// 8-bit unsigned integer
    ubyte: [u8; 8],

    /// The fourthly biggest unsigned element array whose elements are
    /// 8-bit signed integer
    sbyte: [i8; 8],

    /// The usize type element whose size is the same as the LongUnion
    #[cfg(target_pointer_width = "64")] u_size: usize,

    /// The isize type element whose size is the same as the LongUnion
    #[cfg(target_pointer_width = "64")] s_size: isize,

    /// The usize type array whose elements's size is 32-bit size
    #[cfg(target_pointer_width = "32")] u_size: [usize; 2],

    /// The isize type array whose elements's size is 32-bit size
    #[cfg(target_pointer_width = "32")] s_size: [isize; 2],

    /// The usize type array whose elements's size is 16-bit size
    #[cfg(target_pointer_width = "16")] u_size: [usize; 4],

    /// The isize type array whose elements's size is 16-bit size
    #[cfg(target_pointer_width = "16")] s_size: [isize; 4],

    /// The usize type array whose elements's size is 8-bit size
    #[cfg(target_pointer_width = "8")] u_size: [usize; 8],

    /// The isize type array whose elements's size is 8-bit size
    #[cfg(target_pointer_width = "8")] s_size: [isize; 8],
}


/// This union is for converting one primitive integral type into another
/// integeral type within 128-bit long type.
/// 
/// # Feature
/// All the fields are pubic but it is highly encouraged to get/set methods
/// instead of accessing to each field directly. The simple get/set methods are
/// all inline methods so that you hardly lose the performance benefit because
/// of using get/set methods.
/// 
/// # Example
/// ```
/// use cryptocol::number::*;
/// let a = LongerUnion::new_with_signed(-1234567890987654321012345678987654321_i128);
/// println!("a.this = {}, {}", unsafe { a.this }, a.get());
/// println!("a.that = {}, {}", unsafe { a.that }, a.get_signed());
/// println!("a.ulonger = {}", unsafe { a.ulonger });
/// println!("a.slonger = {}", unsafe { a.slonger });
/// #[cfg(target_endian = "little")]
/// {
///     for i in 0..2
///         { println!("a.ulong[{}] = {}, {}", i, unsafe { a.ulong[i] }, a.get_ulong_(i)); }
///     for i in 0..2
///         { println!("a.slong[{}] = {}, {}", i, unsafe { a.slong[i] }, a.get_slong_(i)); }
///     for i in 0..4
///         { println!("a.uint[{}] = {}, {}", i, unsafe { a.uint[i] }, a.get_uint_(i)); }
///     for i in 0..4
///         { println!("a.sint[{}] = {}, {}", i, unsafe { a.sint[i] }, a.get_sint_(i)); }
///     for i in 0..8
///         { println!("a.ushort[{}] = {}, {}", i, unsafe { a.ushort[i] }, a.get_ushort_(i)); }
///     for i in 0..8
///         { println!("a.sshort[{}] = {}, {}", i, unsafe { a.sshort[i] }, a.get_sshort_(i)); }
///     for i in 0..16
///         { println!("a.ubyte[{}] = {}, {}", i, unsafe { a.ubyte[i] }, a.get_ubyte_(i)); }
///     for i in 0..16
///         { println!("a.sbyte[{}] = {}, {}", i, unsafe { a.sbyte[i] }, a.get_sbyte_(i)); }
///     #[cfg(target_pointer_width = "64")]
///     {
///         const N: usize = 2;
///         for i in 0..N
///             { println!("a.u_size[{}] = {}, {}", i, unsafe { a.u_size[i] }, a.get_usize_(i)); }
///         for i in 0..N
///             { println!("a.s_size[{}] = {}, {}", i, unsafe { a.s_size[i] }, a.get_ssize_(i)); }
///     }
///     #[cfg(target_pointer_width = "32")]
///     {
///         const N: usize = 4;
///         for i in 0..N
///             { println!("a.u_size[{}] = {}, {}", i, unsafe { a.u_size[i] }, a.get_usize_(i)); }
///         for i in 0..N
///             { println!("a.s_size[{}] = {}, {}", i, unsafe { a.s_size[i] }, a.get_ssize_(i)); }
///     }
///     #[cfg(target_pointer_width = "16")]
///     {
///         const N: usize = 8;
///         for i in 0..N
///             { println!("a.u_size[{}] = {}, {}", i, unsafe { a.u_size[i] }, a.get_usize_(i)); }
///         for i in 0..N
///             { println!("a.s_size[{}] = {}, {}", i, unsafe { a.s_size[i] }, a.get_ssize_(i)); }
///     }
///     #[cfg(target_pointer_width = "8")]
///     {
///         const N: usize = 16;
///         for i in 0..N
///             { println!("a.u_size[{}] = {}, {}", i, unsafe { a.u_size[i] }, a.get_usize_(i)); }
///         for i in 0..N
///             { println!("a.s_size[{}] = {}, {}", i, unsafe { a.s_size[i] }, a.get_ssize_(i)); }
///     }
/// }
/// #[cfg(target_pointer_width = "128")]
/// {
///     println!("a.u_size = {}", unsafe { a.u_size });
///     println!("a.s_size = {}", unsafe { a.s_size });
/// }
/// 
/// assert_eq!(unsafe { a.this }, 339047799029950809142362261752780557135_u128);
/// assert_eq!(unsafe { a.that }, -1234567890987654321012345678987654321_i128);
/// assert_eq!(unsafe { a.ulonger }, 339047799029950809142362261752780557135_u128);
/// assert_eq!(unsafe { a.slonger }, -1234567890987654321012345678987654321_i128);
/// #[cfg(target_endian = "little")]
/// {
///     assert_eq!(unsafe { a.ulong[0] }, 13664881099896654671_u64);
///     assert_eq!(unsafe { a.ulong[1] }, 18379818014235068504_u64);
///     assert_eq!(unsafe { a.slong[0] }, -4781862973812896945_i64);
///     assert_eq!(unsafe { a.slong[1] }, -66926059474483112_i64);
///     assert_eq!(unsafe { a.uint[0] }, 4048161615_u32);
///     assert_eq!(unsafe { a.uint[1] }, 3181603061_u32);
///     assert_eq!(unsafe { a.uint[2] }, 2127464536_u32);
///     assert_eq!(unsafe { a.uint[3] }, 4279384858_u32);
///     assert_eq!(unsafe { a.sint[0] }, -246805681_i32);
///     assert_eq!(unsafe { a.sint[1] }, -1113364235_i32);
///     assert_eq!(unsafe { a.sint[2] }, 2127464536_i32);
///     assert_eq!(unsafe { a.sint[3] }, -15582438_i32);
///     assert_eq!(unsafe { a.ushort[0] }, 2895_u16);
///     assert_eq!(unsafe { a.ushort[1] }, 61770_u16);
///     assert_eq!(unsafe { a.ushort[2] }, 26869_u16);
///     assert_eq!(unsafe { a.ushort[3] }, 48547_u16);
///     assert_eq!(unsafe { a.ushort[4] }, 34904_u16);
///     assert_eq!(unsafe { a.ushort[5] }, 32462_u16);
///     assert_eq!(unsafe { a.ushort[6] }, 15130_u16);
///     assert_eq!(unsafe { a.ushort[7] }, 65298_u16);
///     assert_eq!(unsafe { a.sshort[0] }, 2895_i16);
///     assert_eq!(unsafe { a.sshort[1] }, -3766_i16);
///     assert_eq!(unsafe { a.sshort[2] }, 26869_i16);
///     assert_eq!(unsafe { a.sshort[3] }, -16989_i16);
///     assert_eq!(unsafe { a.sshort[4] }, -30632_i16);
///     assert_eq!(unsafe { a.sshort[5] }, 32462_i16);
///     assert_eq!(unsafe { a.sshort[6] }, 15130_i16);
///     assert_eq!(unsafe { a.sshort[7] }, -238_i16);
///     assert_eq!(unsafe { a.ubyte[0] }, 79_u8);
///     assert_eq!(unsafe { a.ubyte[1] }, 11_u8);
///     assert_eq!(unsafe { a.ubyte[2] }, 74_u8);
///     assert_eq!(unsafe { a.ubyte[3] }, 241_u8);
///     assert_eq!(unsafe { a.ubyte[4] }, 245_u8);
///     assert_eq!(unsafe { a.ubyte[5] }, 104_u8);
///     assert_eq!(unsafe { a.ubyte[6] }, 163_u8);
///     assert_eq!(unsafe { a.ubyte[7] }, 189_u8);
///     assert_eq!(unsafe { a.ubyte[8] }, 88_u8);
///     assert_eq!(unsafe { a.ubyte[9] }, 136_u8);
///     assert_eq!(unsafe { a.ubyte[10] }, 206_u8);
///     assert_eq!(unsafe { a.ubyte[11] }, 126_u8);
///     assert_eq!(unsafe { a.ubyte[12] }, 26_u8);
///     assert_eq!(unsafe { a.ubyte[13] }, 59_u8);
///     assert_eq!(unsafe { a.ubyte[14] }, 18_u8);
///     assert_eq!(unsafe { a.ubyte[15] }, 255_u8);
///     assert_eq!(unsafe { a.sbyte[0] }, 79_i8);
///     assert_eq!(unsafe { a.sbyte[1] }, 11_i8);
///     assert_eq!(unsafe { a.sbyte[2] }, 74_i8);
///     assert_eq!(unsafe { a.sbyte[3] }, -15_i8);
///     assert_eq!(unsafe { a.sbyte[4] }, -11_i8);
///     assert_eq!(unsafe { a.sbyte[5] }, 104_i8);
///     assert_eq!(unsafe { a.sbyte[6] }, -93_i8);
///     assert_eq!(unsafe { a.sbyte[7] }, -67_i8);
///     assert_eq!(unsafe { a.sbyte[8] }, 88_i8);
///     assert_eq!(unsafe { a.sbyte[9] }, -120_i8);
///     assert_eq!(unsafe { a.sbyte[10] }, -50_i8);
///     assert_eq!(unsafe { a.sbyte[11] }, 126_i8);
///     assert_eq!(unsafe { a.sbyte[12] }, 26_i8);
///     assert_eq!(unsafe { a.sbyte[13] }, 59_i8);
///     assert_eq!(unsafe { a.sbyte[14] }, 18_i8);
///     assert_eq!(unsafe { a.sbyte[15] }, -1_i8);
/// }
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


/// This union is for converting one primitive integral type into another
/// integeral type within machine-dependent long type.
/// 
/// # Feature
/// All the fields are pubic but it is highly encouraged to get/set methods
/// instead of accessing to each field directly. The simple get/set methods are
/// all inline methods so that you hardly lose the performance benefit because
/// of using get/set methods.
/// 
/// # Example
/// ```
/// use cryptocol::number::*;
/// let a = SizeUnion::new_with_signed(-1234567890987654321012345678987654321_isize);
/// println!("a.this = {}, {}", unsafe { a.this }, a.get());
/// println!("a.that = {}, {}", unsafe { a.that }, a.get_signed());
/// println!("a.u_size = {}, {}", unsafe { a.u_size }, a.get());
/// println!("a.s_size = {}, {}", unsafe { a.s_size }, a.get_signed()));
/// println!("a.ulonger = {}, {}", unsafe { a.ulonger }, a.get());
/// println!("a.slonger = {}, {}", unsafe { a.slonger }, a.get_signed());
/// #[cfg(target_endian = "little")]
/// {
///     for i in 0..2
///         { println!("a.ulong[{}] = {}, {}", i, unsafe { a.ulong[i] }, a.get_ulong_(i)); }
///     for i in 0..2
///         { println!("a.slong[{}] = {}, {}", i, unsafe { a.slong[i] }, a.get_slong_(i)); }
///     for i in 0..4
///         { println!("a.uint[{}] = {}, {}", i, unsafe { a.uint[i] }, a.get_uint_(i)); }
///     for i in 0..4
///         { println!("a.sint[{}] = {}, {}", i, unsafe { a.sint[i] }, a.get_sint_(i)); }
///     for i in 0..8
///         { println!("a.ushort[{}] = {}, {}", i, unsafe { a.ushort[i] }, a.get_ushort_(i)); }
///     for i in 0..8
///         { println!("a.sshort[{}] = {}, {}", i, unsafe { a.sshort[i] }, a.get_sshort_(i)); }
///     for i in 0..16
///         { println!("a.ubyte[{}] = {}, {}", i, unsafe { a.ubyte[i] }, a.get_ubyte_(i)); }
///     for i in 0..16
///         { println!("a.sbyte[{}] = {}, {}", i, unsafe { a.sbyte[i] }, a.get_sbyte_(i)); }
/// }
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


/// This union is for converting one primitive integral type into another
/// integeral type within machine-dependent long type.
/// 
/// # Feature
/// All the fields are pubic but it is highly encouraged to get/set methods
/// instead of accessing to each field directly. The simple get/set methods are
/// all inline methods so that you hardly lose the performance benefit because
/// of using get/set methods.
/// 
/// # Example
/// ```
/// use cryptocol::number::*;
/// let a = SizeUnion::new_with_signed(-1234567890123456789_isize);
/// println!("a.this = {}, {}", unsafe { a.this }, a.get());
/// println!("a.that = {}, {}", unsafe { a.that }, a.get_signed());
/// println!("a.u_size = {}, {}", unsafe { a.u_size }, a.get());
/// println!("a.s_size = {}, {}", unsafe { a.s_size }, a.get_signed()));
/// println!("a.ulong = {}, {}", unsafe { a.ulong }, a.get());
/// println!("a.slong = {}, {}", unsafe { a.slong }, a.get_signed());
/// #[cfg(target_endian = "little")]
/// {
///     for i in 0..2
///         { println!("a.uint[{}] = {}, {}", i, unsafe { a.uint[i] }, a.get_uint_(i)); }
///     for i in 0..2
///         { println!("a.sint[{}] = {}, {}", i, unsafe { a.sint[i] }, a.get_sint_(i)); }
///     for i in 0..4
///         { println!("a.ushort[{}] = {}, {}", i, unsafe { a.ushort[i] }, a.get_ushort_(i)); }
///     for i in 0..4
///         { println!("a.sshort[{}] = {}, {}", i, unsafe { a.sshort[i] }, a.get_sshort_(i)); }
///     for i in 0..8
///         { println!("a.ubyte[{}] = {}, {}", i, unsafe { a.ubyte[i] }, a.get_ubyte_(i)); }
///     for i in 0..8
///         { println!("a.sbyte[{}] = {}, {}", i, unsafe { a.sbyte[i] }, a.get_sbyte_(i)); }
/// }
/// 
/// assert_eq!(unsafe { a.this }, 17212176183586094827_usize);
/// assert_eq!(unsafe { a.that }, -1234567890123456789_isize);
/// assert_eq!(unsafe { a.u_size }, 17212176183586094827_usize);
/// assert_eq!(unsafe { a.s_size }, -1234567890123456789_isize);
/// assert_eq!(unsafe { a.ulong }, 17212176183586094827_u64);
/// assert_eq!(unsafe { a.slong }, -1234567890123456789_i64);
/// #[cfg(target_endian = "little")]
/// {
///     assert_eq!(unsafe { a.uint[0] }, 2182512363_u32);
///     assert_eq!(unsafe { a.uint[1] }, 4007522059_u32);
///     assert_eq!(unsafe { a.sint[0] }, -2112454933_i32);
///     assert_eq!(unsafe { a.sint[1] }, -287445237_i32);
///     assert_eq!(unsafe { a.ushort[0] }, 32491_u16);
///     assert_eq!(unsafe { a.ushort[1] }, 33302_u16);
///     assert_eq!(unsafe { a.ushort[2] }, 61195_u16);
///     assert_eq!(unsafe { a.ushort[3] }, 61149_u16);
///     assert_eq!(unsafe { a.sshort[0] }, 32491_i16);
///     assert_eq!(unsafe { a.sshort[1] }, -32234_i16);
///     assert_eq!(unsafe { a.sshort[2] }, -4341_i16);
///     assert_eq!(unsafe { a.sshort[3] }, -4387_i16);
///     assert_eq!(unsafe { a.ubyte[0] }, 235_u8);
///     assert_eq!(unsafe { a.ubyte[1] }, 126_u8);
///     assert_eq!(unsafe { a.ubyte[2] }, 22_u8);
///     assert_eq!(unsafe { a.ubyte[3] }, 130_u8);
///     assert_eq!(unsafe { a.ubyte[4] }, 11_u8);
///     assert_eq!(unsafe { a.ubyte[5] }, 239_u8);
///     assert_eq!(unsafe { a.ubyte[6] }, 221_u8);
///     assert_eq!(unsafe { a.ubyte[7] }, 238_u8);
///     assert_eq!(unsafe { a.sbyte[0] }, -21_i8);
///     assert_eq!(unsafe { a.sbyte[1] }, 126_i8);
///     assert_eq!(unsafe { a.sbyte[2] }, 22_i8);
///     assert_eq!(unsafe { a.sbyte[3] }, -126_i8);
///     assert_eq!(unsafe { a.sbyte[4] }, 11_i8);
///     assert_eq!(unsafe { a.sbyte[5] }, -17_i8);
///     assert_eq!(unsafe { a.sbyte[6] }, -35_i8);
///     assert_eq!(unsafe { a.sbyte[7] }, -18_i8);
/// }
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
    pub u_size: usize,

    /// The isize type element whose size is the same as the SizeUnion
    pub s_size: isize,

    /// The biggest unsigned element which is 64-bit unsigned integer
    pub ulong: u64,

    /// The biggest signed element which is 64-bit unsigned integer
    pub slong: i64,

    /// The secondly biggest unsigned element array whose elements are
    /// 32-bit unsigned integer
    pub uint: [u32; 2],

    /// The secondly biggest unsigned element array whose elements are
    /// 32-bit signed integer
    pub sint: [i32; 2],

    /// The thirdly biggest unsigned element array whose elements are
    /// 16-bit unsigned integer
    pub ushort: [u16; 4],

    /// The thirdly biggest unsigned element array whose elements are
    /// 16-bit signed integer
    pub sshort: [i16; 4],

    /// The fourthly biggest unsigned element array whose elements are
    /// 8-bit unsigned integer
    pub ubyte: [u8; 8],

    /// The fourthly biggest unsigned element array whose elements are
    /// 8-bit signed integer
    pub sbyte: [i8; 8],
}


/// This union is for converting one primitive integral type into another
/// integeral type within machine-dependent long type.
/// 
/// # Feature
/// All the fields are pubic but it is highly encouraged to get/set methods
/// instead of accessing to each field directly. The simple get/set methods are
/// all inline methods so that you hardly lose the performance benefit because
/// of using get/set methods.
/// 
/// # Example
/// ```
/// use cryptocol::number::*;
/// let a = SizeUnion::new_with_signed(2112454933_isize);
/// println!("a.this = {}, {}", unsafe { a.this }, a.get());
/// println!("a.that = {}, {}", unsafe { a.that }, a.get_signed());
/// println!("a.u_size = {}, {}", unsafe { a.u_size }, a.get());
/// println!("a.s_size = {}, {}", unsafe { a.s_size }, a.get_signed()));
/// println!("a.uint = {}, {}", unsafe { a.uint }, a.get());
/// println!("a.sint = {}, {}", unsafe { a.sint }, a.get_signed());
/// #[cfg(target_endian = "little")]
/// {
///     for i in 0..2
///         { println!("a.ushort[{}] = {}, {}", i, unsafe { a.ushort[i] }, a.get_ushort_(i)); }
///     for i in 0..2
///         { println!("a.sshort[{}] = {}, {}", i, unsafe { a.sshort[i] }, a.get_sshort_(i)); }
///     for i in 0..4
///         { println!("a.ubyte[{}] = {}, {}", i, unsafe { a.ubyte[i] }, a.get_ubyte_(i)); }
///     for i in 0..4
///         { println!("a.sbyte[{}] = {}, {}", i, unsafe { a.sbyte[i] }, a.get_sbyte_(i)); }
/// }
/// 
/// assert_eq!(unsafe { a.this }, 2182512363_usize);
/// assert_eq!(unsafe { a.that }, -2112454933_isize);
/// assert_eq!(unsafe { a.u_size }, 2182512363_usize);
/// assert_eq!(unsafe { a.s_size }, -2112454933_isize);
/// assert_eq!(unsafe { a.uint }, 2182512363_u32);
/// assert_eq!(unsafe { a.sint }, -2112454933_i32);
/// #[cfg(target_endian = "little")]
/// {
///     assert_eq!(unsafe { a.ushort[0] }, 32491_u16);
///     assert_eq!(unsafe { a.ushort[1] }, 33302_u16);
///     assert_eq!(unsafe { a.sshort[0] }, 32491_i16);
///     assert_eq!(unsafe { a.sshort[1] }, -32234_i16);
///     assert_eq!(unsafe { a.ubyte[0] }, 235_u8);
///     assert_eq!(unsafe { a.ubyte[1] }, 126_u8);
///     assert_eq!(unsafe { a.ubyte[2] }, 22_u8);
///     assert_eq!(unsafe { a.ubyte[3] }, 130_u8);
///     assert_eq!(unsafe { a.sbyte[0] }, -21_i8);
///     assert_eq!(unsafe { a.sbyte[1] }, 126_i8);
///     assert_eq!(unsafe { a.sbyte[2] }, 22_i8);
///     assert_eq!(unsafe { a.sbyte[3] }, -126_i8);
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


/// This union is for converting one primitive integral type into another
/// integeral type within machine-dependent long type.
/// 
/// # Feature
/// All the fields are pubic but it is highly encouraged to get/set methods
/// instead of accessing to each field directly. The simple get/set methods are
/// all inline methods so that you hardly lose the performance benefit because
/// of using get/set methods.
/// 
/// # Example
/// ```
/// use cryptocol::number::*;
/// let a = SizeUnion::new_with_signed(32491_isize);
/// println!("a.this = {}, {}", unsafe { a.this }, a.get());
/// println!("a.that = {}, {}", unsafe { a.that }, a.get_signed());
/// println!("a.u_size = {}, {}", unsafe { a.u_size }, a.get());
/// println!("a.s_size = {}, {}", unsafe { a.s_size }, a.get_signed());
/// println!("a.ushort = {}, {}", unsafe { a.ushort }, a.get());
/// println!("a.sshort = {}, {}", unsafe { a.sshort }, a.get_signed());
/// #[cfg(target_endian = "little")]
/// {
///     for i in 0..2
///         { println!("a.ubyte[{}] = {}, {}", i, unsafe { a.ubyte[i] }, a.get_ubyte_(i)); }
///     for i in 0..2
///         { println!("a.sbyte[{}] = {}, {}", i, unsafe { a.sbyte[i] }, a.get_sbyte_(i)); }
/// }
/// 
/// assert_eq!(unsafe { a.this }, 32491_usize);
/// assert_eq!(unsafe { a.that }, 32491_isize);
/// assert_eq!(unsafe { a.u_size }, 32491_usize);
/// assert_eq!(unsafe { a.s_size }, 32491_isize);
/// assert_eq!(unsafe { a.ushort }, 32491_u16);
/// assert_eq!(unsafe { a.sshort }, 32491_i16);
/// #[cfg(target_endian = "little")]
/// {
///     assert_eq!(unsafe { a.ubyte[0] }, 235_u8);
///     assert_eq!(unsafe { a.ubyte[1] }, 126_u8);
///     assert_eq!(unsafe { a.sbyte[0] }, -21_i8);
///     assert_eq!(unsafe { a.sbyte[1] }, 126_i8);
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


/// This union is for converting one primitive integral type into another
/// integeral type within machine-dependent long type.
/// 
/// # Feature
/// All the fields are pubic but it is highly encouraged to get/set methods
/// instead of accessing to each field directly. The simple get/set methods are
/// all inline methods so that you hardly lose the performance benefit because
/// of using get/set methods.
/// 
/// # Example
/// ```
/// use cryptocol::number::*;
/// let a = SizeUnion::new_with_signed(-21_isize);
/// println!("a.this = {}, {}", unsafe { a.this }, a.get());
/// println!("a.that = {}, {}", unsafe { a.that }, a.get_signed());
/// println!("a.u_size = {}, {}", unsafe { a.u_size }, a.get());
/// println!("a.s_size = {}, {}", unsafe { a.s_size }, a.get_signed()));
/// println!("a.ubyte = {}, {}", unsafe { a.ubyte }, a.get());
/// println!("a.sbyte = {}, {}", unsafe { a.sbyte }, a.get_signed());
/// 
/// assert_eq!(unsafe { a.this }, 235_usize);
/// assert_eq!(unsafe { a.that }, -21_isize);
/// assert_eq!(unsafe { a.u_size }, 235_usize);
/// assert_eq!(unsafe { a.s_size }, -21_isize);
/// assert_eq!(unsafe { a.ubyte }, 235_u8);
/// assert_eq!(unsafe { a.sbyte }, -21_i8);
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



macro_rules! get_set_byte {
    ($f:expr) => {
        const N: usize = $f;

        // pub fn get_ubyte_(&self, i: usize) -> u8
        /// Returns i-th element of array `ubyte` of type `u8`
        /// if `i` is less than the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument
        /// `i` indicates 0-th element contains LSB (Least Significant Bit),
        /// while (the size of this Union in bytes - 1)-th element contains
        /// MSB (Most Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to the size of this Union in
        /// bytes, it will panic. So, use this method only when you are sure
        /// that the argument i is less than the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_ubyte(). This method
        /// get_ubyte_() is considered to be slightly faster than the method
        /// get_ubyte().
        /// Use this method only when you are sure that `i` is less than the
        /// size of this Union in bytes. Otherwise, use its counterpart method
        /// [get_ubyte()](#method.get_ubyte) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// let a_short = ShortUnion::new_with(2895_u16);
        /// let b_short_u8 = a_short.get_ubyte_(1);
        /// println!("a_short.get_ubyte_(1) = {}", b_short_u8);
        /// assert_eq!(b_short_u8, 11_u8);
        /// 
        /// // It will panic.
        /// // let c_short = a_short.get_ubyte_(2);
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// let a_longer = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// let b_longer_u8 = a_longer.get_ubyte_(3);
        /// println!("a_longer.get_ubyte_(3) = {}", b_longer_u8);
        /// assert_eq!(b_longer_u8, 241_u8);
        /// 
        /// // It will panic.
        /// // let c_longer = a_longer.get_ubyte_(16);
        /// ```
        #[cfg(target_endian = "little")]
        #[inline] pub fn get_ubyte_(&self, i: usize) -> u8 { unsafe { self.ubyte[i] } }

        /// Returns i-th element of array `ubyte` of type `u8`
        /// if `i` is less than the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to the size of this Union in
        /// bytes, it will panic. So, use this method only when you are sure
        /// that the argument i is less than the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_ubyte(). This method
        /// get_ubyte_() is considered to be slightly faster than the method
        /// get_ubyte().
        /// Use this method only when you are sure that `i` is less than the
        /// size of this Union in bytes. Otherwise, use its counterpart method
        /// [get_ubyte()](#method.get_ubyte) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// let a_short = ShortUnion::new_with(2895_u16);
        /// let b_short_u8 = a_short.get_ubyte_(1);
        /// println!("a_short.get_ubyte_(1) = {}", b_short_u8);
        /// assert_eq!(b_short_u8, 11_u8);
        /// 
        /// // It will panic.
        /// // let c_short = a_short.get_ubyte_(2);
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// let a_longer = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// let b_longer_u8 = a_longer.get_ubyte_(3);
        /// println!("a_longer.get_ubyte_(3) = {}", b_longer_u8);
        /// assert_eq!(b_longer_u8, 241_u8);
        /// 
        /// // It will panic.
        /// // let c_longer_u8 = a_longer.get_ubyte_(16);
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not encouraged
        /// to use it for serious purpose. Only use this crate for Big-endian CPUs
        /// with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn get_ubyte_(&self, i: usize) -> u8 { unsafe { self.ubyte[Self::N-i] } }

        /// Returns i-th element of array `sbyte` of type `u8`
        /// if `i` is less than the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to the size of this Union in
        /// bytes, it will panic. So, use this method only when you are sure
        /// that the argument i is less than the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_sbyte(). This method
        /// get_sbyte_() is considered to be slightly faster than the method
        /// get_sbyte().
        /// Use this method only when you are sure that `i` is less than the
        /// size of this Union in bytes. Otherwise, use its counterpart method
        /// [get_sbyte()](#method.get_sbyte) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// let a_short = ShortUnion::new_with(2895_u16);
        /// let b_short_i8 = a_short.get_sbyte_(1);
        /// println!("a_short.get_sbyte_(1) = {}", b_short_i8);
        /// assert_eq!(b_short_i8, 11_i8);
        /// 
        /// // It will panic.
        /// // let c_short_i8 = a_short.get_sbyte_(2);
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// let a_longer = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// let b_longer_u8 = a_longer.get_sbyte_(3);
        /// println!("a_longer.get_sbyte_(3) = {}", b_longer_u8);
        /// assert_eq!(b_longer_u8, -15_i8);
        /// 
        /// // It will panic.
        /// // let c_longer_u8 = a_longer.get_sbyte_(16);
        /// ```
        #[cfg(target_endian = "little")]
        #[inline] pub fn get_sbyte_(&self, i: usize) -> i8 { unsafe { self.sbyte[i] } }

        /// Returns i-th element of array `ubyte` of type `u8`
        /// if `i` is less than the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to the size of this Union in
        /// bytes, it will panic. So, use this method only when you are sure
        /// that the argument i is less than the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_sbyte(). This method
        /// get_sbyte_() is considered to be slightly faster than the method
        /// get_sbyte().
        /// Use this method only when you are sure that `i` is less than the
        /// size of this Union in bytes. Otherwise, use its counterpart method
        /// [get_ubyte()](#method.get_ubyte) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// let a_short = ShortUnion::new_with(2895_u16);
        /// let b_short_i8 = a_short.get_sbyte_(1);
        /// println!("a_short.get_sbyte_(1) = {}", b_short_i8);
        /// assert_eq!(b_short_i8, 11_i8);
        /// 
        /// // It will panic.
        /// // let c_short_i8 = a_short.get_sbyte_(2);
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// let a_longer = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// let b_longer_i8 = a_longer.get_sbyte_(3);
        /// println!("a_longer.get_sbyte_(3) = {}", b_longer_i8);
        /// assert_eq!(b_longer_i8, -15_i8);
        /// 
        /// // It will panic.
        /// // let c_longer_i8 = a_longer.get_sbyte_(16);
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn get_sbyte_(&self, i: usize) -> i8 { unsafe { self.sbyte[Self::N-i] } }

        /// Returns i-th element of array `ubyte` of type `u8` wrapped in Some
        /// of enum Option if `i` is less than the size of this Union in bytes.
        /// Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than the
        /// size of this Union in bytes. Otherwise, use its counterpart method
        /// [get_ubyte_()](#method.get_ubyte_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// let a_short = ShortUnion::new_with(2895_u16);
        /// 
        /// match a_short.get_ubyte(1)
        /// {
        ///     Some(b) =>  {
        ///             println!("a_short.get_ubyte(1) = {}", b);
        ///             assert_eq!(b, 11_u8);
        ///         },
        ///     None =>     { println!("Out of range"); },
        /// }
        /// 
        /// match a_short.get_ubyte(2)
        /// {
        ///     Some(b) =>  { println!("a_short.get_ubyte(2) = {}", b); },
        ///     None =>     {
        ///             println!("Out of range");
        ///             assert_eq!(a_short.get_ubyte(2), None);
        ///         },
        /// }
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// let a_longer = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// match a_longer.get_ubyte(3)
        /// {
        ///     Some(b) =>  {
        ///             println!("a_longer.get_ubyte(3) = {}", b);
        ///             assert_eq!(b, 241_u8);
        ///         },
        ///     None =>     { println!("Out of range"); },
        /// }
        /// 
        /// match a_longer.get_ubyte(16)
        /// {
        ///     Some(b) =>  { println!("a_short.get_ubyte(16) = {}", b); },
        ///     None =>     {
        ///             println!("Out of range");
        ///             assert_eq!(a_longer.get_ubyte(16), None);
        ///         },
        /// }
        /// ```
        #[cfg(target_endian = "little")]
        #[inline] pub fn get_ubyte(&self, i: usize) -> Option<u8>
        {
            if i <= Self::N
                { unsafe { Some(self.ubyte[i]) } }
            else
                { None }
        }

        /// Returns i-th element of array `ubyte` of type `u8` wrapped in Some
        /// of enum Option if `i` is less than the size of this Union in bytes.
        /// Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than the
        /// size of this Union in bytes. Otherwise, use its counterpart method
        /// [get_ubyte_()](#method.get_ubyte_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// let a_short = ShortUnion::new_with(2895_u16);
        /// 
        /// match a_short.get_ubyte(1)
        /// {
        ///     Some(b) =>  {
        ///             println!("a_short.get_ubyte(1) = {}", b);
        ///             assert_eq!(b, 11_u8);
        ///         },
        ///     None =>     { println!("Out of range"); },
        /// }
        /// 
        /// match a_short.get_ubyte(2)
        /// {
        ///     Some(b) =>  { println!("a_short.get_ubyte(2) = {}", b); },
        ///     None =>     {
        ///             println!("Out of range");
        ///             assert_eq!(a_short.get_ubyte(2), None);
        ///         },
        /// }
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// let a_longer = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// match a_longer.get_ubyte(3)
        /// {
        ///     Some(b) =>  {
        ///             println!("a_longer.get_ubyte(3) = {}", b);
        ///             assert_eq!(b, 241_u8);
        ///         },
        ///     None =>     { println!("Out of range"); },
        /// }
        /// 
        /// match a_longer.get_ubyte(16)
        /// {
        ///     Some(b) =>  { println!("a_short.get_ubyte(16) = {}", b); },
        ///     None =>     {
        ///             println!("Out of range");
        ///             assert_eq!(a_longer.get_ubyte(16), None);
        ///         },
        /// }
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        /// with your own full responsibility.
        #[cfg(target_endian = "big")]
        pub fn get_ubyte(&self, i: usize) -> Option<u8>
        {
            if i <= Self::N
                { unsafe { Some(self.ubyte[Self::N-i]) } }
            else
                { None }
        }

        /// Returns i-th element of array `sbyte` of type `u8` wrapped in Some
        /// of enum Option if `i` is less than the size of this Union in bytes.
        /// Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than the
        /// size of this Union in bytes. Otherwise, use its counterpart method
        /// [get_sbyte_()](#method.get_sbyte_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// let a_short = ShortUnion::new_with(2895_u16);
        /// 
        /// match a_short.get_sbyte(1)
        /// {
        ///     Some(b) =>  {
        ///             println!("a_short.get_sbyte(1) = {}", b);
        ///             assert_eq!(b, 11_i8);
        ///         },
        ///     None =>     { println!("Out of range"); },
        /// }
        /// 
        /// match a_short.get_sbyte(2)
        /// {
        ///     Some(b) =>  { println!("a_short.get_sbyte(2) = {}", b); },
        ///     None =>     {
        ///             println!("Out of range");
        ///             assert_eq!(a_short.get_sbyte(2), None);
        ///         },
        /// }
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// let a_longer = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// match a_longer.get_sbyte(3)
        /// {
        ///     Some(b) =>  {
        ///             println!("a_longer.get_sbyte(3) = {}", b);
        ///             assert_eq!(b, -15_i8);
        ///         },
        ///     None =>     { println!("Out of range"); },
        /// }
        /// 
        /// match a_longer.get_sbyte(16)
        /// {
        ///     Some(b) =>  { println!("a_short.get_sbyte(16) = {}", b); },
        ///     None =>     {
        ///             println!("Out of range");
        ///             assert_eq!(a_longer.get_sbyte(16), None);
        ///         },
        /// }
        /// ```
        #[cfg(target_endian = "little")]
        #[inline] pub fn get_sbyte(&self, i: usize) -> Option<i8>
        {
            if i <= Self::N
                { unsafe { Some(self.sbyte[i]) } }
            else
                { None }
        }

        /// Returns i-th element of array `sbyte` of type `u8` wrapped in Some
        /// of enum Option if `i` is less than the size of this Union in bytes.
        /// Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than the
        /// size of this Union in bytes. Otherwise, use its counterpart method
        /// [get_sbyte_()](#method.get_sbyte_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// let a_short = ShortUnion::new_with(2895_u16);
        /// 
        /// match a_short.get_sbyte(1)
        /// {
        ///     Some(b) =>  {
        ///             println!("a_short.get_sbyte(1) = {}", b);
        ///             assert_eq!(b, 11_i8);
        ///         },
        ///     None =>     { println!("Out of range"); },
        /// }
        /// 
        /// match a_short.get_sbyte(2)
        /// {
        ///     Some(b) =>  { println!("a_short.get_sbyte(2) = {}", b); },
        ///     None =>     {
        ///             println!("Out of range");
        ///             assert_eq!(a_short.get_sbyte(2), None);
        ///         },
        /// }
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// let a_longer = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// match a_longer.get_sbyte(3)
        /// {
        ///     Some(b) =>  {
        ///             println!("a_longer.get_sbyte(3) = {}", b);
        ///             assert_eq!(b, -15_i8);
        ///         },
        ///     None =>     { println!("Out of range"); },
        /// }
        /// 
        /// match a_longer.get_sbyte(16)
        /// {
        ///     Some(b) =>  { println!("a_short.get_sbyte(16) = {}", b); },
        ///     None =>     {
        ///             println!("Out of range");
        ///             assert_eq!(a_longer.get_sbyte(16), None);
        ///         },
        /// }
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        pub fn get_sbyte(&self, i: usize) -> Option<i8>
        {
            if i <= Self::N
                { unsafe { Some(self.sbyte[Self::N-i]) } }
            else
                { None }
        }

        /// Sets i-th element of its array `ubyte` of type `u8`
        /// if `i` is less than the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to the size of this Union in
        /// bytes, it will panic. So, use this method only when you are sure
        /// that the argument i is less than the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_ubyte(). This method
        /// set_ubyte_() is considered to be slightly faster than the method
        /// set_ubyte().
        /// Use this method only when you are sure that `i` is less than the
        /// size of this Union in bytes. Otherwise, use its counterpart method
        /// [set_ubyte()](#method.set_ubyte) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// let mut a_short = ShortUnion::new_with(2895_u16);
        /// let mut b_short_u8 = a_short.get_ubyte_(1);
        /// println!("a_short.get_ubyte_(1) = {}", b_short_u8);
        /// a_short.set_ubyte_(1, 0);
        /// b_short_u8 = a_short.get_ubyte_(1);
        /// println!("a_short.get() = {}, a_short.get_ubyte_(1) = {}", a_short, b_short_u8);
        /// assert_eq!(a_short.get(), 79_u16);
        /// assert_eq!(b_short_u8, 0_u8);
        /// 
        /// // It will panic.
        /// // let c_short = a_short.set_ubyte_(2, 0);
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// let mut a_longer = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// let mut b_longer_u8 = a_longer.get_ubyte_(3);
        /// println!("a_longer.get_ubyte_(3) = {}", b_longer_u8);
        /// a_longer.set_ubyte_(3, 0);
        /// println!("a_longer.get() = {}, a_longer.get_ubyte_(3) = {}", a_longer, b_longer_u8);
        /// assert_eq!(a_longer.get(), 339047799029950809142362261748737248079_u128);
        /// assert_eq!(a_longer.get_ubyte_(3), 0_u8);
        /// 
        /// // It will panic.
        /// // let c_longer = a_longer.get_ubyte_(16);
        /// ```
        #[cfg(target_endian = "little")]
        #[inline] pub fn set_ubyte_(&mut self, i: usize, val: u8)  { unsafe { self.ubyte[i] = val; } }

        /// Sets i-th element of its array `ubyte` of type `u8`
        /// if `i` is less than the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to the size of this Union in
        /// bytes, it will panic. So, use this method only when you are sure
        /// that the argument i is less than the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_ubyte(). This method
        /// set_ubyte_() is considered to be slightly faster than the method
        /// set_ubyte().
        /// Use this method only when you are sure that `i` is less than the
        /// size of this Union in bytes. Otherwise, use its counterpart method
        /// [set_ubyte()](#method.set_ubyte) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// let mut a_short = ShortUnion::new_with(2895_u16);
        /// let mut b_short_u8 = a_short.get_ubyte_(1);
        /// println!("a_short.get_ubyte_(1) = {}", b_short_u8);
        /// a_short.set_ubyte_(1, 0);
        /// b_short_u8 = a_short.get_ubyte_(1);
        /// println!("a_short.get() = {}, a_short.get_ubyte_(1) = {}", a_short, b_short_u8);
        /// assert_eq!(a_short.get(), 79_u16);
        /// assert_eq!(b_short_u8, 0_u8);
        /// 
        /// // It will panic.
        /// // let c_short = a_short.set_ubyte_(2, 0);
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// let mut a_longer = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// let mut b_longer_u8 = a_longer.get_ubyte_(3);
        /// println!("a_longer.get_ubyte_(3) = {}", b_longer_u8);
        /// a_longer.set_ubyte_(3, 0);
        /// println!("a_longer.get() = {}, a_longer.get_ubyte_(3) = {}", a_longer, b_longer_u8);
        /// assert_eq!(a_longer.get(), 339047799029950809142362261748737248079_u128);
        /// assert_eq!(a_longer.get_ubyte_(3), 0_u8);
        /// 
        /// // It will panic.
        /// // let c_longer = a_longer.get_ubyte_(16);
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn set_ubyte_(&mut self, i: usize, val: u8)  { unsafe { self.ubyte[Self::N-i] = val; } }

        /// Sets i-th element of its array `sbyte` of type `i8`
        /// if `i` is less than the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to the size of this Union in
        /// bytes, it will panic. So, use this method only when you are sure
        /// that the argument i is less than the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_sbyte(). This method
        /// set_sbyte_() is considered to be slightly faster than the method
        /// set_sbyte().
        /// Use this method only when you are sure that `i` is less than the
        /// size of this Union in bytes. Otherwise, use its counterpart method
        /// [set_sbyte()](#method.set_sbyte) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// let mut a_short = ShortUnion::new_with_signed(79_i16);
        /// let mut b_short_i8 = a_short.get_sbyte_(1);
        /// println!("a_short.get_sbyte_(1) = {}", b_short_i8);
        /// a_short.set_sbyte_(1, 0);
        /// b_short_i8 = a_short.get_sbyte_(1);
        /// println!("a_short.get_signed() = {}, a_short.get_sbyte_(1) = {}", a_short.get_signed(), b_short_i8);
        /// assert_eq!(a_short.get_signed(), 79_i16);
        /// assert_eq!(b_short_i8, 0_i8);
        /// 
        /// // It will panic.
        /// // let c_short = a_short.set_sbyte_(2, 0);
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// let mut a_longer = LongerUnion::new_with_signed(-123456789012345678901234567890123456789_i128);
        /// let mut b_longer_i8 = a_longer.get_sbyte_(3);
        /// println!("a_longer.get_sbyte_(3) = {}", b_longer_i8);
        /// a_longer.set_sbyte_(3, 0);
        /// b_longer_i8 = a_longer.get_sbyte_(3);
        /// println!("a_longer.get_signed() = {}, a_longer.get_sbyte_(3) = {}", a_longer.get_signed(), b_longer_i8);
        /// assert_eq!(a_longer.get_signed(), -123456789012345678901234567891482411285_i128);
        /// assert_eq!(a_longer.get_ubyte_(3), 0_u8);
        /// 
        /// // It will panic.
        /// // let c_longer = a_longer.get_sbyte_(16);
        /// ```
        #[cfg(target_endian = "little")]
        #[inline] pub fn set_sbyte_(&mut self, i: usize, val: i8)  { unsafe { self.sbyte[i] = val; } }

        /// Sets i-th element of its array `sbyte` of type `i8`
        /// if `i` is less than the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to the size of this Union in
        /// bytes, it will panic. So, use this method only when you are sure
        /// that the argument i is less than the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_sbyte(). This method
        /// set_sbyte_() is considered to be slightly faster than the method
        /// set_sbyte().
        /// Use this method only when you are sure that `i` is less than the
        /// size of this Union in bytes. Otherwise, use its counterpart method
        /// [set_sbyte()](#method.set_sbyte) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// let mut a_short = ShortUnion::new_with_signed(79_i16);
        /// let mut b_short_i8 = a_short.get_sbyte_(1);
        /// println!("a_short.get_sbyte_(1) = {}", b_short_i8);
        /// a_short.set_sbyte_(1, 0);
        /// b_short_i8 = a_short.get_sbyte_(1);
        /// println!("a_short.get_signed() = {}, a_short.get_sbyte_(1) = {}", a_short.get_signed(), b_short_i8);
        /// assert_eq!(a_short.get(), 79_i16);
        /// assert_eq!(b_short_i8, 0_i8);
        /// 
        /// // It will panic.
        /// // let c_short = a_short.set_sbyte_(2, 0);
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// let mut a_longer = LongerUnion::new_with_signed(-123456789012345678901234567890123456789_i128);
        /// let mut b_longer_i8 = a_longer.get_sbyte_(3);
        /// println!("a_longer.get_sbyte_(3) = {}", b_longer_i8);
        /// a_longer.set_sbyte_(3, 0);
        /// b_longer_i8 = a_longer.get_sbyte_(3);
        /// println!("a_longer.get_signed() = {}, a_longer.get_sbyte_(3) = {}", a_longer.get_signed(), b_longer_i8);
        /// assert_eq!(a_longer.get_signed(), -123456789012345678901234567891482411285_i128);
        /// assert_eq!(a_longer.get_ubyte_(3), 0_u8);
        /// 
        /// // It will panic.
        /// // let c_longer = a_longer.get_sbyte_(16);
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn set_sbyte_(&mut self, i: usize, val: i8)  { unsafe { self.sbyte[Self::N-i] = val; } }

        /// Sets i-th element of its array `ubyte` of type `u8` and returns true
        /// if `i` is less than the size of this Union in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than the
        /// size of this Union in bytes. Otherwise, use its counterpart method
        /// [set_ubyte_()](#method.set_ubyte_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// let mut a_short = ShortUnion::new();
        /// let mut succ = a_short.set_sbyte(1, 11);
        /// let mut sbyte = a_short.get_sbyte(1);
        /// if succ
        /// {
        ///     println!("a_short.get() = {}, a_short.get_sbyte(1).unwrap() = {}", a_short, sbyte.unwrap());
        ///     assert_eq!(sbyte.unwrap(), 11_i8);
        /// }
        /// else
        /// {
        ///     println!("Out of range");
        ///     assert_eq!(sbyte, None);
        /// }
        /// 
        /// succ = a_short.set_sbyte(2, 11);
        /// sbyte = a_short.get_sbyte(2);
        /// if succ
        /// {
        ///     println!("a_short.get() = {}, a_short.get_sbyte(2).unwrap() = {}", a_short, sbyte.unwrap());
        ///     assert_eq!(sbyte.unwrap(), 11_i8);
        /// }
        /// else
        /// {
        ///     println!("Out of range");
        ///     assert_eq!(sbyte, None);
        /// }
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// let mut a_longer = LongerUnion::new();
        /// let mut succ = a_longer.set_ubyte(3, 241_u8);
        /// let mut ubyte = a_longer.get_ubyte(3);
        /// if succ
        /// {
        ///     println!("a_longer.get() = {}, a_longer.get_ubyte(3).unwrap() = {}", a_longer, ubyte.unwrap());
        ///     assert_eq!(ubyte.unwrap(), 241_u8);
        /// }
        /// else
        /// {
        ///     println!("Out of range");
        ///     assert_eq!(ubyte, None);
        /// }
        /// 
        /// succ = a_longer.set_ubyte(16, 241_u8);
        /// ubyte = a_longer.get_ubyte(16);
        /// if succ
        /// {
        ///     println!("a_longer.get() = {}, a_longer.get_sbyte(16).unwrap() = {}", a_longer, ubyte.unwrap());
        ///     assert_eq!(ubyte.unwrap(), 241_u8);
        /// }
        /// else
        /// {
        ///     println!("Out of range");
        ///     assert_eq!(ubyte, None);
        /// }
        /// ```
        #[cfg(target_endian = "little")]
        pub fn set_ubyte(&mut self, i: usize, val: u8) -> bool
        {
            if i <= Self::N
            { 
                unsafe { self.ubyte[i] = val; }
                true
            }
            else
            {
                false
            }
        }

        /// Sets i-th element of its array `ubyte` of type `u8` and returns true
        /// if `i` is less than the size of this Union in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than the
        /// size of this Union in bytes. Otherwise, use its counterpart method
        /// [set_ubyte_()](#method.set_ubyte_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// let mut a_short = ShortUnion::new();
        /// let mut succ = a_short.set_sbyte(1, 11);
        /// let mut sbyte = a_short.get_sbyte(1);
        /// if succ
        /// {
        ///     println!("a_short.get() = {}, a_short.get_sbyte(1).unwrap() = {}", a_short, sbyte.unwrap());
        ///     assert_eq!(sbyte.unwrap(), 11_i8);
        /// }
        /// else
        /// {
        ///     println!("Out of range");
        ///     assert_eq!(sbyte, None);
        /// }
        /// 
        /// succ = a_short.set_sbyte(2, 11);
        /// sbyte = a_short.get_sbyte(2);
        /// if succ
        /// {
        ///     println!("a_short.get() = {}, a_short.get_sbyte(2).unwrap() = {}", a_short, sbyte.unwrap());
        ///     assert_eq!(sbyte.unwrap(), 11_i8);
        /// }
        /// else
        /// {
        ///     println!("Out of range");
        ///     assert_eq!(sbyte, None);
        /// }
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// let mut a_longer = LongerUnion::new();
        /// let mut succ = a_longer.set_ubyte(3, 241_u8);
        /// let mut ubyte = a_longer.get_ubyte(3);
        /// if succ
        /// {
        ///     println!("a_longer.get() = {}, a_longer.get_ubyte(3).unwrap() = {}", a_longer, ubyte.unwrap());
        ///     assert_eq!(ubyte.unwrap(), 241_u8);
        /// }
        /// else
        /// {
        ///     println!("Out of range");
        ///     assert_eq!(ubyte, None);
        /// }
        /// 
        /// succ = a_longer.set_ubyte(16, 241_u8);
        /// ubyte = a_longer.get_ubyte(16);
        /// if succ
        /// {
        ///     println!("a_longer.get() = {}, a_longer.get_sbyte(16).unwrap() = {}", a_longer, ubyte.unwrap());
        ///     assert_eq!(ubyte.unwrap(), 241_u8);
        /// }
        /// else
        /// {
        ///     println!("Out of range");
        ///     assert_eq!(ubyte, None);
        /// }
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        pub fn set_ubyte(&self, i: usize, val: u8) -> bool
        {
            if i <= Self::N
            { 
                unsafe { self.ubyte[Self::N-i] = val; }
                true
            }
            else
            {
                false
            }
        }

        /// Sets i-th element of its array `sbyte` of type `i8` and returns true
        /// if `i` is less than the size of this Union in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than the
        /// size of this Union in bytes. Otherwise, use its counterpart method
        /// [set_sbyte_()](#method.set_sbyte_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// let mut a_short = ShortUnion::new();
        /// let mut succ = a_short.set_sbyte(1, 11);
        /// let mut sbyte = a_short.get_sbyte(1);
        /// if succ
        /// {
        ///     println!("a_short.get() = {}, a_short.get_sbyte(1).unwrap() = {}", a_short, sbyte.unwrap());
        ///     assert_eq!(sbyte.unwrap(), 11_i8);
        /// }
        /// else
        /// {
        ///     println!("Out of range");
        ///     assert_eq!(sbyte, None);
        /// }
        /// 
        /// succ = a_short.set_sbyte(2, 11);
        /// sbyte = a_short.get_sbyte(2);
        /// if succ
        /// {
        ///     println!("a_short.get() = {}, a_short.get_sbyte(2).unwrap() = {}", a_short, sbyte.unwrap());
        ///     assert_eq!(sbyte.unwrap(), 11_i8);
        /// }
        /// else
        /// {
        ///     println!("Out of range");
        ///     assert_eq!(sbyte, None);
        /// }
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// let mut a_longer = LongerUnion::new();
        /// let mut succ = a_longer.set_sbyte(3, 81_i8);
        /// let mut sbyte = a_longer.get_sbyte(3);
        /// if succ
        /// {
        ///     println!("a_longer.get_signed() = {}, a_longer.get_sbyte(3).unwrap() = {}", a_longer.get_signed(), sbyte.unwrap());
        ///     assert_eq!(sbyte.unwrap(), 81_i8);
        /// }
        /// else
        /// {
        ///     println!("Out of range");
        ///     assert_eq!(sbyte, None);
        /// }
        /// 
        /// succ = a_longer.set_sbyte(16, 81_i8);
        /// sbyte = a_longer.get_sbyte(16);
        /// if succ
        /// {
        ///     println!("a_longer.get_signed() = {}, a_longer.get_sbyte(16).unwrap() = {}", a_longer.get_signed(), sbyte.unwrap());
        ///     assert_eq!(sbyte.unwrap(), 81_i8);
        /// }
        /// else
        /// {
        ///     println!("Out of range");
        ///     assert_eq!(sbyte, None);
        /// }
        /// ```
        #[cfg(target_endian = "little")]
        pub fn set_sbyte(&mut self, i: usize, val: i8) -> bool
        {
            if i <= Self::N
            { 
                unsafe { self.sbyte[i] = val; }
                true
            }
            else
            {
                false
            }
        }

        /// Sets i-th element of its array `sbyte` of type `i8` and returns true
        /// if `i` is less than the size of this Union in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than the
        /// size of this Union in bytes. Otherwise, use its counterpart method
        /// [set_sbyte_()](#method.set_sbyte_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// let mut succ = a_short.set_sbyte(1, 11);
        /// let mut sbyte = a_short.get_sbyte(1);
        /// if succ
        /// {
        ///     println!("a_short.get() = {}, a_short.get_sbyte(1).unwrap() = {}", a_short, sbyte.unwrap());
        ///     assert_eq!(sbyte.unwrap(), 11_i8);
        /// }
        /// else
        /// {
        ///     println!("Out of range");
        ///     assert_eq!(sbyte, None);
        /// }
        /// 
        /// succ = a_short.set_sbyte(2, 11);
        /// sbyte = a_short.get_sbyte(2);
        /// if succ
        /// {
        ///     println!("a_short.get() = {}, a_short.get_sbyte(2).unwrap() = {}", a_short, sbyte.unwrap());
        ///     assert_eq!(sbyte.unwrap(), 11_i8);
        /// }
        /// else
        /// {
        ///     println!("Out of range");
        ///     assert_eq!(sbyte, None);
        /// }
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// let mut a_longer = LongerUnion::new();
        /// let mut succ = a_longer.set_sbyte(3, 81_i8);
        /// let mut sbyte = a_longer.get_sbyte(3);
        /// if succ
        /// {
        ///     println!("a_longer.get_signed() = {}, a_longer.get_sbyte(3).unwrap() = {}", a_longer.get_signed(), sbyte.unwrap());
        ///     assert_eq!(sbyte.unwrap(), 81_i8);
        /// }
        /// else
        /// {
        ///     println!("Out of range");
        ///     assert_eq!(sbyte, None);
        /// }
        /// 
        /// succ = a_longer.set_sbyte(16, 81_i8);
        /// sbyte = a_longer.get_sbyte(16);
        /// if succ
        /// {
        ///     println!("a_longer.get_signed() = {}, a_longer.get_sbyte(16).unwrap() = {}", a_longer.get_signed(), sbyte.unwrap());
        ///     assert_eq!(sbyte.unwrap(), 81_i8);
        /// }
        /// else
        /// {
        ///     println!("Out of range");
        ///     assert_eq!(sbyte, None);
        /// }
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        pub fn set_sbyte(&self, i: usize, val: i8) -> bool
        {
            if i <= Self::N
            { 
                unsafe { self.sbyte[Self::N-i] = val; }
                true
            }
            else
            {
                false
            }
        }
    }
}


macro_rules! get_set_short {
    ($f:expr) => {
        const M: usize = $f;

        /// Returns i-th element of array `ushort` of type `u16`
        /// if `i` is less than a half of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a half of the size of this
        /// Union in bytes, it will panic.
        /// So, use this method only when you are sure that the argument `i` is
        /// less than a half of the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_ushort(). This method
        /// get_ushort_() is considered to be slightly faster than the method
        /// get_ushort().
        /// Use this method only when you are sure that `i` is less than a half
        /// of the size of this Union in bytes.
        /// Otherwise, use its counterpart method [get_ushort()](#method.get_ushort)
        /// for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        #[cfg(target_endian = "little")]
        #[inline] pub fn get_ushort_(&self, i: usize) -> u16 { unsafe { self.ushort[i] } }

        /// Returns i-th element of array `ushort` of type `u16`
        /// if `i` is less than a half of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a half of the size of this
        /// Union in bytes, it will panic.
        /// So, use this method only when you are sure that the argument `i` is
        /// less than a half of the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_ushort(). This method
        /// get_ushort_() is considered to be slightly faster than the method
        /// get_ushort().
        /// Use this method only when you are sure that `i` is less than a half
        /// of the size of this Union in bytes.
        /// Otherwise, use its counterpart method [get_ushort()](#method.get_ushort)
        /// for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn get_ushort_(&self, i: usize) -> u16 { unsafe { self.ushort[Self::M-i] } }

        /// Returns i-th element of array `sshort` of type `i16`
        /// if `i` is less than a half of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a half of the size of this
        /// Union in bytes, it will panic.
        /// So, use this method only when you are sure that the argument `i` is
        /// less than a half of the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_sshort(). This method
        /// get_sshort_() is considered to be slightly faster than the method
        /// get_sshort().
        /// Use this method only when you are sure that `i` is less than a half
        /// of the size of this Union in bytes.
        /// Otherwise, use its counterpart method [get_sshort()](#method.get_sshort)
        /// for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        #[cfg(target_endian = "little")]
        #[inline] pub fn get_sshort_(&self, i: usize) -> i16 { unsafe { self.sshort[i] } }

        /// Returns i-th element of array `sshort` of type `i16`
        /// if `i` is less than a half of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a half of the size of this
        /// Union in bytes, it will panic.
        /// So, use this method only when you are sure that the argument `i` is
        /// less than a half of the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_sshort(). This method
        /// get_sshort_() is considered to be slightly faster than the method
        /// get_sshort().
        /// Use this method only when you are sure that `i` is less than a half
        /// of the size of this Union in bytes.
        /// Otherwise, use its counterpart method [get_sshort()](#method.get_sshort)
        /// for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn get_sshort_(&self, i: usize) -> i16 { unsafe { self.sshort[Self::M-i] } }

        /// Returns i-th element of array `ushort` of type `u16` wrapped in Some
        /// of enum Option if `i` is less than a half of the size of this Union
        /// in bytes. Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than a half
        /// of the size of this Union in bytes. Otherwise, use its counterpart
        /// method [get_ushort_()](#method.get_ushort_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        #[cfg(target_endian = "little")]
        pub fn get_ushort(&self, i: usize) -> Option<u16>
        {
            if i <= Self::M
                { unsafe { Some(self.ushort[i]) } }
            else
                { None }
        }

        /// Returns i-th element of array `ushort` of type `u16` wrapped in Some
        /// of enum Option if `i` is less than a half of the size of this Union
        /// in bytes. Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than a half
        /// of the size of this Union in bytes. Otherwise, use its counterpart
        /// method [get_ushort_()](#method.get_ushort_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        pub fn get_ushort(&self, i: usize) -> Option<u16>
        {
            if i <= Self::M
                { unsafe { Some(self.ushort[Self::M-i]) } }
            else
                { None }
        }

        /// Returns i-th element of array `sshort` of type `i16` wrapped in Some
        /// of enum Option if `i` is less than a half of the size of this Union
        /// in bytes. Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than a half
        /// of the size of this Union in bytes. Otherwise, use its counterpart
        /// method [get_sshort_()](#method.get_sshort_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        #[cfg(target_endian = "little")]
        pub fn get_sshort(&self, i: usize) -> Option<i16>
        {
            if i <= Self::M
                { unsafe { Some(self.sshort[i]) } }
            else
                { None }
        }

        /// Returns i-th element of array `sshort` of type `i16` wrapped in Some
        /// of enum Option if `i` is less than a half of the size of this Union
        /// in bytes. Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than a half
        /// of the size of this Union in bytes. Otherwise, use its counterpart
        /// method [get_sshort_()](#method.get_sshort_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        pub fn get_sshort(&self, i: usize) -> Option<i16>
        {
            if i <= Self::M
                { unsafe { Some(self.sshort[Self::M-i]) } }
            else
                { None }
        }

        /// Sets i-th element of its array `ushort` of type `u16`
        /// if `i` is less than a half of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a half of the size of this
        /// Union in bytes, it will panic. So, use this method only when you
        /// are sure that the argument i is less than a half of the size of
        /// this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_ushort(). This method
        /// set_ushort_() is considered to be slightly faster than the method
        /// set_ushort().
        /// Use this method only when you are sure that `i` is less than a half
        /// of the size of this Union in bytes. Otherwise, use its counterpart
        /// method [set_ushort()](#method.set_ushort) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        #[cfg(target_endian = "little")]
        #[inline] pub fn set_ushort_(&mut self, i: usize, val: u16)  { unsafe { self.ushort[i] = val; } }

        /// Sets i-th element of its array `ushort` of type `u16`
        /// if `i` is less than a half of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a half of the size of this
        /// Union in bytes, it will panic. So, use this method only when you
        /// are sure that the argument i is less than a half of the size of
        /// this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_ushort(). This method
        /// set_ushort_() is considered to be slightly faster than the method
        /// set_ushort().
        /// Use this method only when you are sure that `i` is less than a half
        /// of the size of this Union in bytes. Otherwise, use its counterpart
        /// method [set_ushort()](#method.set_ushort) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn set_ushort_(&mut self, i: usize, val: u16)  { unsafe { self.ushort[Self::M-i] = val; } }

        /// Sets i-th element of its array `sshort` of type `i16`
        /// if `i` is less than a half of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a half of the size of this
        /// Union in bytes, it will panic. So, use this method only when you
        /// are sure that the argument i is less than a half of the size of
        /// this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_sshort(). This method
        /// set_sshort_() is considered to be slightly faster than the method
        /// set_sshort().
        /// Use this method only when you are sure that `i` is less than a half
        /// of the size of this Union in bytes. Otherwise, use its counterpart
        /// method [set_sshort()](#method.set_sshort) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        #[cfg(target_endian = "little")]
        #[inline] pub fn set_sshort_(&mut self, i: usize, val: i16)  { unsafe { self.sshort[i] = val; } }

        /// Sets i-th element of its array `sshort` of type `i16`
        /// if `i` is less than a half of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a half of the size of this
        /// Union in bytes, it will panic. So, use this method only when you
        /// are sure that the argument i is less than a half of the size of
        /// this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_sshort(). This method
        /// set_sshort_() is considered to be slightly faster than the method
        /// set_sshort().
        /// Use this method only when you are sure that `i` is less than a half
        /// of the size of this Union in bytes. Otherwise, use its counterpart
        /// method [set_sshort()](#method.set_sshort) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn set_sshort_(&mut self, i: usize, val: i16)  { unsafe { self.sshort[Self::M-i] = val; } }

        /// Sets i-th element of its array `ushort` of type `u16` and returns
        /// true if `i` is less than a half of the size of this Union in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than a half
        /// of the size of this Union in bytes. Otherwise, use its counterpart
        /// method [set_ushort_()](#method.set_ushort_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        #[cfg(target_endian = "little")]
        pub fn set_ushort(&mut self, i: usize, val: u16) -> bool
        {
            if i <= Self::M
            { 
                unsafe { self.ushort[i] = val; }
                true
            }
            else
            {
                false
            }
        }

        /// Sets i-th element of its array `ushort` of type `u16` and returns
        /// true if `i` is less than a half of the size of this Union in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than a half
        /// of the size of this Union in bytes. Otherwise, use its counterpart
        /// method [set_ushort_()](#method.set_ushort_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        pub fn set_ushort(&self, i: usize, val: u16) -> bool
        {
            if i <= Self::M
            { 
                unsafe { self.ushort[Self::M-i] = val; }
                true
            }
            else
            {
                false
            }
        }

        /// Sets i-th element of its array `sshort` of type `i16` and returns
        /// true if `i` is less than a half of the size of this Union in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than a half
        /// of the size of this Union in bytes. Otherwise, use its counterpart
        /// method [set_sshort_()](#method.set_sshort_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        #[cfg(target_endian = "little")]
        pub fn set_sshort(&mut self, i: usize, val: i16) -> bool
        {
            if i <= Self::M
            { 
                unsafe { self.sshort[i] = val; }
                true
            }
            else
            {
                false
            }
        }

        /// Sets i-th element of its array `sshort` of type `i16` and returns
        /// true if `i` is less than a half of the size of this Union in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than a half
        /// of the size of this Union in bytes. Otherwise, use its counterpart
        /// method [set_sshort_()](#method.set_sshort_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        pub fn set_sshort(&self, i: usize, val: i16) -> bool
        {
            if i <= Self::M
            { 
                unsafe { self.sshort[Self::M-i] = val; }
                true
            }
            else
            {
                false
            }
        }
    }
}


macro_rules! get_set_int {
    ($f:expr) => {
        const L: usize = $f;

        /// Returns i-th element of array `uint` of type `u32`
        /// if `i` is less than a quarter of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a quarter of the size of this
        /// Union in bytes, it will panic.
        /// So, use this method only when you are sure that the argument `i` is
        /// less than a quarter of the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_uint(). This method
        /// get_uint_() is considered to be slightly faster than the method
        /// get_uint().
        /// Use this method only when you are sure that `i` is less than
        /// a quarter of the size of this Union in bytes.
        /// Otherwise, use its counterpart method [get_uint()](#method.get_uint)
        /// for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        #[cfg(target_endian = "little")]
        #[inline] pub fn get_uint_(&self, i: usize) -> u32 { unsafe { self.uint[i] } }

        /// Returns i-th element of array `uint` of type `u32`
        /// if `i` is less than a quarter of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a quarter of the size of this
        /// Union in bytes, it will panic.
        /// So, use this method only when you are sure that the argument `i` is
        /// less than a quarter of the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_uint(). This method
        /// get_uint_() is considered to be slightly faster than the method
        /// get_uint().
        /// Use this method only when you are sure that `i` is less than
        /// a quarter of the size of this Union in bytes.
        /// Otherwise, use its counterpart method [get_uint()](#method.get_uint)
        /// for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn get_uint_(&self, i: usize) -> u32 { unsafe { self.uint[Self::L-i] } }

        /// Returns i-th element of array `sint` of type `i32`
        /// if `i` is less than a quarter of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a quarter of the size of this
        /// Union in bytes, it will panic.
        /// So, use this method only when you are sure that the argument `i` is
        /// less than a quarter of the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_sint(). This method
        /// get_sint_() is considered to be slightly faster than the method
        /// get_sint().
        /// Use this method only when you are sure that `i` is less than
        /// a quarter of the size of this Union in bytes.
        /// Otherwise, use its counterpart method [get_sint()](#method.get_sint)
        /// for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        #[cfg(target_endian = "little")]
        #[inline] pub fn get_sint_(&self, i: usize) -> i32 { unsafe { self.sint[i] } }

        /// Returns i-th element of array `sint` of type `i32`
        /// if `i` is less than a quarter of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a quarter of the size of this
        /// Union in bytes, it will panic.
        /// So, use this method only when you are sure that the argument `i` is
        /// less than a quarter of the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_sint(). This method
        /// get_sint_() is considered to be slightly faster than the method
        /// get_sint().
        /// Use this method only when you are sure that `i` is less than
        /// a quarter of the size of this Union in bytes.
        /// Otherwise, use its counterpart method [get_sint()](#method.get_sint)
        /// for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn get_sint_(&self, i: usize) -> i32 { unsafe { self.sint[Self::L-i] } }

        /// Returns i-th element of array `uint` of type `u32` wrapped in Some
        /// of enum Option if `i` is less than a quarter of the size of this
        /// Union in bytes. Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// a quarter of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [get_uint_()](#method.get_uint_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        #[cfg(target_endian = "little")]
        pub fn get_uint(&self, i: usize) -> Option<u32>
        {
            if i <= Self::L
                { unsafe { Some(self.uint[i]) } }
            else
                { None }
        }

        /// Returns i-th element of array `uint` of type `u32` wrapped in Some
        /// of enum Option if `i` is less than a quarter of the size of this
        /// Union in bytes. Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// a quarter of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [get_uint_()](#method.get_uint_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        pub fn get_uint(&self, i: usize) -> Option<u32>
        {
            if i <= Self::L
                { unsafe { Some(self.uint[Self::L-i]) } }
            else
                { None }
        }

        /// Returns i-th element of array `sint` of type `i32` wrapped in Some
        /// of enum Option if `i` is less than a quarter of the size of this
        /// Union in bytes. Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// a quarter of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [get_sint_()](#method.get_sint_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        #[cfg(target_endian = "little")]
        pub fn get_sint(&self, i: usize) -> Option<i32>
        {
            if i <= Self::L
                { unsafe { Some(self.sint[i]) } }
            else
                { None }
        }

        /// Returns i-th element of array `sint` of type `i32` wrapped in Some
        /// of enum Option if `i` is less than a quarter of the size of this
        /// Union in bytes. Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// a quarter of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [get_sint_()](#method.get_sint_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        pub fn get_sint(&self, i: usize) -> Option<i32>
        {
            if i <= Self::L
                { unsafe { Some(self.sint[Self::L-i]) } }
            else
                { None }
        }

        /// Sets i-th element of its array `uint` of type `u32`
        /// if `i` is less than a quarter of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a quarter of the size of this
        /// Union in bytes, it will panic. So, use this method only when you
        /// are sure that the argument i is less than a quarter of the size of
        /// this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_uint(). This method
        /// set_uint_() is considered to be slightly faster than the method
        /// set_uint().
        /// Use this method only when you are sure that `i` is less than a quarter
        /// of the size of this Union in bytes. Otherwise, use its counterpart
        /// method [set_uint()](#method.set_uint) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        #[cfg(target_endian = "little")]
        #[inline] pub fn set_uint_(&mut self, i: usize, val: u32)  { unsafe { self.uint[i] = val; } }

        /// Sets i-th element of its array `uint` of type `u32`
        /// if `i` is less than a quarter of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a quarter of the size of this
        /// Union in bytes, it will panic. So, use this method only when you
        /// are sure that the argument i is less than a quarter of the size of
        /// this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_uint(). This method
        /// set_uint_() is considered to be slightly faster than the method
        /// set_uint().
        /// Use this method only when you are sure that `i` is less than
        /// a quarter of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [set_uint()](#method.set_uint) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn set_uint_(&mut self, i: usize, val: u32)  { unsafe { self.uint[Self::L-i] = val; } }

        /// Sets i-th element of its array `sint` of type `i32`
        /// if `i` is less than a quarter of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a quarter of the size of this
        /// Union in bytes, it will panic. So, use this method only when you
        /// are sure that the argument i is less than a quarter of the size of
        /// this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_sint(). This method
        /// set_sint_() is considered to be slightly faster than the method
        /// set_sint().
        /// Use this method only when you are sure that `i` is less than
        /// a quarter of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [set_sint()](#method.set_sint) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        #[cfg(target_endian = "little")]
        #[inline] pub fn set_sint_(&mut self, i: usize, val: i32)  { unsafe { self.sint[i] = val; } }

        /// Sets i-th element of its array `sint` of type `i32`
        /// if `i` is less than a quarter of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a quarter of the size of this
        /// Union in bytes, it will panic. So, use this method only when you
        /// are sure that the argument i is less than a quarter of the size of
        /// this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_sint(). This method
        /// set_sint_() is considered to be slightly faster than the method
        /// set_sint().
        /// Use this method only when you are sure that `i` is less than
        /// a quarter of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [set_sint()](#method.set_sint) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn set_sint_(&mut self, i: usize, val: i32)  { unsafe { self.sint[Self::L-i] = val; } }

        /// Sets i-th element of its array `uint` of type `u32` and returns
        /// true if `i` is less than a quarter of the size of this Union in
        /// bytes. Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// a quarter of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [set_uint_()](#method.set_uint_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        #[cfg(target_endian = "little")]
        pub fn set_uint(&mut self, i: usize, val: u32) -> bool
        {
            if i <= Self::L
            { 
                unsafe { self.uint[i] = val; }
                true
            }
            else
            {
                false
            }
        }

        /// Sets i-th element of its array `uint` of type `u32` and returns
        /// true if `i` is less than a quarter of the size of this Union in
        /// bytes. Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// a quarter of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [set_uint_()](#method.set_uint_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        pub fn set_uint(&self, i: usize, val: u32) -> bool
        {
            if i <= Self::L
            { 
                unsafe { self.uint[Self::L-i] = val; }
                true
            }
            else
            {
                false
            }
        }

        /// Sets i-th element of its array `sint` of type `i32` and returns
        /// true if `i` is less than a quarter of the size of this Union in
        /// bytes. Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// a quarter of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [set_sint_()](#method.set_sint_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        #[cfg(target_endian = "little")]
        pub fn set_sint(&mut self, i: usize, val: i32) -> bool
        {
            if i <= Self::L
            { 
                unsafe { self.sint[i] = val; }
                true
            }
            else
            {
                false
            }
        }

        /// Sets i-th element of its array `sint` of type `i32` and returns
        /// true if `i` is less than a quarter of the size of this Union in
        /// bytes. Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// a quarter of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [set_sint_()](#method.set_sint_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        pub fn set_sint(&self, i: usize, val: i32) -> bool
        {
            if i <= Self::L
            { 
                unsafe { self.sint[Self::L-i] = val; }
                true
            }
            else
            {
                false
            }
        }
    }
}


macro_rules! get_set_long {
    ($f:expr) => {
        const K: usize = $f;

        /// Returns i-th element of array `ulong` of type `u64`
        /// if `i` is less than an eighth of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to an eighth of the size of this
        /// Union in bytes, it will panic.
        /// So, use this method only when you are sure that the argument `i` is
        /// less than an eighth of the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_ulong(). This method
        /// get_ulong_() is considered to be slightly faster than the method
        /// get_ulong().
        /// Use this method only when you are sure that `i` is less than
        /// an eighth of the size of this Union in bytes.
        /// Otherwise, use its counterpart method [get_ulong()](#method.get_ulong)
        /// for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        #[cfg(target_endian = "little")]
        #[inline] pub fn get_ulong_(&self, i: usize) -> u64 { unsafe { self.ulong[i] } }

        /// Returns i-th element of array `ulong` of type `u64`
        /// if `i` is less than an eighth of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to an eighth of the size of this
        /// Union in bytes, it will panic.
        /// So, use this method only when you are sure that the argument `i` is
        /// less than an eighth of the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_ulong(). This method
        /// get_ulong_() is considered to be slightly faster than the method
        /// get_ulong().
        /// Use this method only when you are sure that `i` is less than
        /// an eighth of the size of this Union in bytes.
        /// Otherwise, use its counterpart method [get_ulong()](#method.get_ulong)
        /// for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn get_ulong_(&self, i: usize) -> u64 { unsafe { self.ulong[Self::K-i] } }

        /// Returns i-th element of array `slong` of type `i64`
        /// if `i` is less than an eighth of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to an eighth of the size of this
        /// Union in bytes, it will panic.
        /// So, use this method only when you are sure that the argument `i` is
        /// less than an eighth of the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_slong(). This method
        /// get_slong_() is considered to be slightly faster than the method
        /// get_slong().
        /// Use this method only when you are sure that `i` is less than
        /// an eighth of the size of this Union in bytes.
        /// Otherwise, use its counterpart method [get_slong()](#method.get_slong)
        /// for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        #[cfg(target_endian = "little")]
        #[inline] pub fn get_slong_(&self, i: usize) -> i64 { unsafe { self.slong[i] } }

        /// Returns i-th element of array `slong` of type `i64`
        /// if `i` is less than an eighth of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to an eighth of the size of this
        /// Union in bytes, it will panic.
        /// So, use this method only when you are sure that the argument `i` is
        /// less than an eighth of the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_slong(). This method
        /// get_slong_() is considered to be slightly faster than the method
        /// get_slong().
        /// Use this method only when you are sure that `i` is less than
        /// an eighth of the size of this Union in bytes.
        /// Otherwise, use its counterpart method [get_slong()](#method.get_slong)
        /// for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn get_slong_(&self, i: usize) -> i64 { unsafe { self.slong[Self::K-i] } }

        /// Returns i-th element of array `ulong` of type `u64` wrapped in Some
        /// of enum Option if `i` is less than an eighth of the size of this
        /// Union in bytes. Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// an eighth of the size of this Union in bytes. Otherwise, l;you can
        /// use its counterpart method [get_ulong_()](#method.get_ulong_)
        /// for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        #[cfg(target_endian = "little")]
        pub fn get_ulong(&self, i: usize) -> Option<u64>
        {
            if i <= Self::K
                { unsafe { Some(self.ulong[i]) } }
            else
                { None }
        }

        /// Returns i-th element of array `ulong` of type `u64` wrapped in Some
        /// of enum Option if `i` is less than an eighth of the size of this
        /// Union in bytes. Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// an eighth of the size of this Union in bytes. Otherwise, l;you can
        /// use its counterpart method [get_ulong_()](#method.get_ulong_)
        /// for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn get_ulong(&self, i: usize) -> Option<u64>
        {
            if i <= Self::K
                { unsafe { Some(self.ulong[Self::K-i]) } }
            else
                { None }
        }

        /// Returns i-th element of array `slong` of type `i64` wrapped in Some
        /// of enum Option if `i` is less than an eighth of the size of this
        /// Union in bytes. Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// an eighth of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [get_slong_()](#method.get_slong_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        #[cfg(target_endian = "little")]
        pub fn get_slong(&self, i: usize) -> Option<i64>
        {
            if i <= Self::K
                { unsafe { Some(self.slong[i]) } }
            else
                { None }
        }

        /// Returns i-th element of array `slong` of type `i64` wrapped in Some
        /// of enum Option if `i` is less than an eighth of the size of this
        /// Union in bytes. Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// an eighth of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [get_slong_()](#method.get_slong_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn get_slong(&self, i: usize) -> Option<i64>
        {
            if i <= Self::K
                { unsafe { Some(self.slong[Self::K-i]) } }
            else
                { None }
        }

        /// Sets i-th element of its array `ulong` of type `u64`
        /// if `i` is less than an eighth of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to an eighth of the size of this
        /// Union in bytes, it will panic. So, use this method only when you
        /// are sure that the argument i is less than an eighth of the size of
        /// this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_ulong(). This method
        /// set_ulong_() is considered to be slightly faster than the method
        /// set_ulong().
        /// Use this method only when you are sure that `i` is less than an eighth
        /// of the size of this Union in bytes. Otherwise, use its counterpart
        /// method [set_ulong()](#method.set_ulong) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        #[cfg(target_endian = "little")]
        #[inline] pub fn set_ulong_(&mut self, i: usize, val: u64)  { unsafe { self.ulong[i] = val; } }

        /// Sets i-th element of its array `ulong` of type `u64`
        /// if `i` is less than an eighth of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to an eighth of the size of this
        /// Union in bytes, it will panic. So, use this method only when you
        /// are sure that the argument i is less than an eighth of the size of
        /// this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_ulong(). This method
        /// set_ulong_() is considered to be slightly faster than the method
        /// set_ulong().
        /// Use this method only when you are sure that `i` is less than an eighth
        /// of the size of this Union in bytes. Otherwise, use its counterpart
        /// method [set_ulong()](#method.set_ulong) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn set_ulong_(&mut self, i: usize, val: u64)  { unsafe { self.ulong[Self::K-i] = val; } }

        /// Sets i-th element of its array `slong` of type `i64`
        /// if `i` is less than an eighth of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to an eighth of the size of this
        /// Union in bytes, it will panic. So, use this method only when you
        /// are sure that the argument i is less than an eighth of the size of
        /// this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_slong(). This method
        /// set_slong_() is considered to be slightly faster than the method
        /// set_slong().
        /// Use this method only only when you are sure that `i` is less than
        /// an eighth of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [set_slong()](#method.set_slong) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        #[cfg(target_endian = "little")]
        #[inline] pub fn set_slong_(&mut self, i: usize, val: i64)  { unsafe { self.slong[i] = val; } }

        /// Sets i-th element of its array `slong` of type `i64`
        /// if `i` is less than an eighth of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to an eighth of the size of this
        /// Union in bytes, it will panic. So, use this method only when you
        /// are sure that the argument i is less than an eighth of the size of
        /// this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_slong(). This method
        /// set_slong_() is considered to be slightly faster than the method
        /// set_slong().
        /// Use this method only only when you are sure that `i` is less than
        /// an eighth of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [set_slong()](#method.set_slong) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn set_slong_(&mut self, i: usize, val: i64)  { unsafe { self.slong[Self::K-i] = val; } }

        /// Sets i-th element of its array `ulong` of type `u64` and returns
        /// true if `i` is less than an eighth of the size of this Union in
        /// bytes. Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// an eighth of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [set_ulong_()](#method.set_ulong_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        #[cfg(target_endian = "little")]
        pub fn set_ulong(&mut self, i: usize, val: u64) -> bool
        {
            if i <= Self::L
            { 
                unsafe { self.ulong[i] = val; }
                true
            }
            else
            {
                false
            }
        }

        /// Sets i-th element of its array `ulong` of type `u64` and returns
        /// true if `i` is less than an eighth of the size of this Union in
        /// bytes. Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// an eighth of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [set_ulong_()](#method.set_ulong_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        pub fn set_ulong(&self, i: usize, val: u64) -> bool
        {
            if i <= Self::K
            { 
                unsafe { self.ulong[Self::K-i] = val; }
                true
            }
            else
            {
                false
            }
        }

        /// Sets i-th element of its array `slong` of type `i64` and returns
        /// true if `i` is less than an eighth of the size of this Union in
        /// bytes. Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// an eighth of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [set_slong_()](#method.set_slong_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        #[cfg(target_endian = "little")]
        pub fn set_slong(&mut self, i: usize, val: i64) -> bool
        {
            if i <= Self::L
            { 
                unsafe { self.slong[i] = val; }
                true
            }
            else
            {
                false
            }
        }

        /// Sets i-th element of its array `slong` of type `i64` and returns
        /// true if `i` is less than an eighth of the size of this Union in
        /// bytes. Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// an eighth of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [set_slong_()](#method.set_slong_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        pub fn set_ulong(&self, i: usize, val: i64) -> bool
        {
            if i <= Self::K
            { 
                unsafe { self.slong[Self::K-i] = val; }
                true
            }
            else
            {
                false
            }
        }
    }
}


macro_rules! get_set_size {
    ($f:expr) => {
        const J: usize = $f;

        /// Returns i-th element of array `u_size` of type `usize`
        /// if `i` is less than the size of this Union in bytes divided by
        /// the size of the type `usize` in bytes. Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to the size of this Union
        /// in bytes divided by the size of the type `usize` in bytes,
        /// it will panic.
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_usize(). This method
        /// get_usize_() is considered to be slightly faster than the method
        /// get_usize().
        /// Use this method only when you are sure that `i` is less than
        /// the size of this Union in bytes divided by the size of the type
        /// `usize` in bytes. Otherwise, use its counterpart method
        /// [get_usize()](#method.get_usize) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        #[cfg(target_endian = "little")]
        #[inline] pub fn get_usize_(&self, i: usize) -> usize { unsafe { self.u_size[i] } }

        /// Returns i-th element of array `u_size` of type `usize`
        /// if `i` is less than the size of this Union in bytes divided by
        /// the size of the type `usize` in bytes. Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to the size of this Union
        /// in bytes divided by the size of the type `usize` in bytes,
        /// it will panic.
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_usize(). This method
        /// get_usize_() is considered to be slightly faster than the method
        /// get_usize().
        /// Use this method only when you are sure that `i` is less than
        /// the size of this Union in bytes divided by the size of the type
        /// `usize` in bytes. Otherwise, use its counterpart method
        /// [get_usize()](#method.get_usize) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn get_usize_(&self, i: usize) -> usize { unsafe { self.u_size[Self::J-i] } }

        /// Returns i-th element of array `s_size` of type `isize`
        /// if `i` is less than the size of this Union in bytes divided by
        /// the size of the type `isize` in bytes. Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to the size of this Union
        /// in bytes divided by the size of the type `isize` in bytes,
        /// it will panic.
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_ssize(). This method
        /// get_ssize_() is considered to be slightly faster than the method
        /// get_ssize().
        /// Use this method only when you are sure that `i` is less than
        /// the size of this Union in bytes divided by the size of the type
        /// `isize` in bytes. Otherwise, use its counterpart method
        /// [get_ssize()](#method.get_ssize) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        #[cfg(target_endian = "little")]
        #[inline] pub fn get_ssize_(&self, i: usize) -> isize { unsafe { self.s_size[i] } }

        /// Returns i-th element of array `s_size` of type `isize`
        /// if `i` is less than the size of this Union in bytes divided by
        /// the size of the type `isize` in bytes. Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to the size of this Union
        /// in bytes divided by the size of the type `isize` in bytes,
        /// it will panic.
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_ssize(). This method
        /// get_ssize_() is considered to be slightly faster than the method
        /// get_ssize().
        /// Use this method only when you are sure that `i` is less than
        /// the size of this Union in bytes divided by the size of the type
        /// `isize` in bytes. Otherwise, use its counterpart method
        /// [get_ssize()](#method.get_ssize) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn get_ssize_(&self, i: usize) -> isize { unsafe { self.s_size[Self::J-i] } }

        /// Returns i-th element of array `u_size` of type `usize` wrapped
        /// in Some of enum Option if `i` is less than the size of this Union
        /// in bytes divided by the size of the type `usize` in bytes.
        /// Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// the size of this Union in bytes divided by the size of the type
        /// `usize` in bytes. Otherwise, use its counterpart method
        /// [get_usize_()](#method.get_usize_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        #[cfg(target_endian = "little")]
        pub fn get_usize(&self, i: usize) -> Option<usize>
        {
            if i <= Self::J
                { unsafe { Some(self.u_size[i]) } }
            else
                { None }
        }

        /// Returns i-th element of array `u_size` of type `usize` wrapped
        /// in Some of enum Option if `i` is less than the size of this Union
        /// in bytes divided by the size of the type `usize` in bytes.
        /// Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// the size of this Union in bytes divided by the size of the type
        /// `usize` in bytes. Otherwise, use its counterpart method
        /// [get_usize_()](#method.get_usize_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        pub fn get_usize(&self, i: usize) -> Option<usize>
        {
            if i <= Self::J
                { unsafe { Some(self.u_size[Self::J-i]) } }
            else
                { None }
        }

        /// Returns i-th element of array `s_size` of type `isize` wrapped
        /// in Some of enum Option if `i` is less than the size of this Union
        /// in bytes divided by the size of the type `isize` in bytes.
        /// Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// the size of this Union in bytes divided by the size of the type
        /// `isize` in bytes. Otherwise, use its counterpart method
        /// [get_ssize_()](#method.get_ssize_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        #[cfg(target_endian = "little")]
        pub fn get_ssize(&self, i: usize) -> Option<isize>
        {
            if i <= Self::J
                { unsafe { Some(self.s_size[i]) } }
            else
                { None }
        }

        /// Returns i-th element of array `s_size` of type `isize` wrapped
        /// in Some of enum Option if `i` is less than the size of this Union
        /// in bytes divided by the size of the type `isize` in bytes.
        /// Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// the size of this Union in bytes divided by the size of the type
        /// `isize` in bytes. Otherwise, use its counterpart method
        /// [get_ssize_()](#method.get_ssize_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        pub fn get_ssize(&self, i: usize) -> Option<isize>
        {
            if i <= Self::J
                { unsafe { Some(self.s_size[Self::J-i]) } }
            else
                { None }
        }

        /// Sets i-th element of its array `u_size` of type `usize`
        /// if `i` is less than the size of this Union in bytes divided
        /// by the size of the type `usize` in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_usize(). This method
        /// set_usize_() is considered to be slightly faster than the method
        /// set_usize().
        /// Use this method only when you are sure that `i` is less than
        /// the size of this Union in bytes divided by the size of the type
        /// `usize` in bytes. Otherwise, use its counterpart
        /// method [set_ssize()](#method.set_ssize) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        #[cfg(target_endian = "little")]
        #[inline] pub fn set_usize_(&mut self, i: usize, val: usize)  { unsafe { self.u_size[i] = val; } }

        /// Sets i-th element of its array `u_size` of type `usize`
        /// if `i` is less than the size of this Union in bytes divided
        /// by the size of the type `usize` in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_usize(). This method
        /// set_usize_() is considered to be slightly faster than the method
        /// set_usize().
        /// Use this method only when you are sure that `i` is less than
        /// the size of this Union in bytes divided by the size of the type
        /// `usize` in bytes. Otherwise, use its counterpart
        /// method [set_ssize()](#method.set_ssize) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn set_usize_(&mut self, i: usize, val: usize)  { unsafe { self.u_size[Self::J-i] = val; } }

        /// Sets i-th element of its array `s_size` of type `isize`
        /// if `i` is less than the size of this Union in bytes divided
        /// by the size of the type `isize` in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_ssize(). This method
        /// set_ssize_() is considered to be slightly faster than the method
        /// set_ssize().
        /// Use this method only when you are sure that `i` is less than
        /// the size of this Union in bytes divided by the size of the type
        /// `isize` in bytes. Otherwise, use its counterpart method
        /// [set_ssize()](#method.set_ssize) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        #[cfg(target_endian = "little")]
        #[inline] pub fn set_ssize_(&mut self, i: usize, val: isize)  { unsafe { self.s_size[i] = val; } }

        /// Sets i-th element of its array `s_size` of type `isize`
        /// if `i` is less than the size of this Union in bytes divided
        /// by the size of the type `isize` in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_ssize(). This method
        /// set_ssize_() is considered to be slightly faster than the method
        /// set_ssize().
        /// Use this method only when you are sure that `i` is less than
        /// the size of this Union in bytes divided by the size of the type
        /// `isize` in bytes. Otherwise, use its counterpart method
        /// [set_ssize()](#method.set_ssize) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn set_ssize_(&mut self, i: usize, val: isize)  { unsafe { self.s_size[Self::J-i] = val; } }

        /// Sets i-th element of its array `u_size` of type `usize` and returns
        /// true if `i` is less than the size of this Union in bytes divided
        /// by the size of the type `usize` in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// the size of this Union in bytes divided by the size of the type
        /// `usize` in bytes. Otherwise, use its counterpart
        /// method [set_usize_()](#method.set_usize_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        #[cfg(target_endian = "little")]
        pub fn set_usize(&mut self, i: usize, val:usize) -> bool
        {
            if i <= Self::J
            { 
                unsafe { self.u_size[i] = val; }
                true
            }
            else
            {
                false
            }
        }

        /// Sets i-th element of its array `u_size` of type `usize` and returns
        /// true if `i` is less than the size of this Union in bytes divided
        /// by the size of the type `usize` in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// the size of this Union in bytes divided by the size of the type
        /// `usize` in bytes. Otherwise, use its counterpart
        /// method [set_usize_()](#method.set_usize_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        pub fn set_usize(&self, i: usize, val: usize) -> bool
        {
            if i <= Self::J
            { 
                unsafe { self.u_size[Self::J-i] = val; }
                true
            }
            else
            {
                false
            }
        }

        /// Sets i-th element of its array `ssize` of type `isize` and returns
        /// true if `i` is less than the size of this Union in bytes divided
        /// by the size of the type `isize` in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// the size of this Union in bytes divided by the size of the type
        /// `isize` in bytes. Otherwise, use its counterpart
        /// method [set_ssize_()](#method.set_ssize_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        #[cfg(target_endian = "little")]
        pub fn set_ssize(&mut self, i: usize, val: isize) -> bool
        {
            if i <= Self::J
            { 
                unsafe { self.s_size[i] = val; }
                true
            }
            else
            {
                false
            }
        }

        /// Sets i-th element of its array `ssize` of type `isize` and returns
        /// true if `i` is less than the size of this Union in bytes divided
        /// by the size of the type `isize` in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// the size of this Union in bytes divided by the size of the type
        /// `isize` in bytes. Otherwise, use its counterpart
        /// method [set_ssize_()](#method.set_ssize_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        pub fn set_ssize(&self, i: usize, val: isize) -> bool
        {
            if i <= Self::J
            { 
                unsafe { self.s_size[Self::J-i] = val; }
                true
            }
            else
            {
                false
            }
        }
    }
}


macro_rules! integer_union_methods {
    ($f:ty) => {
        pub fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool)
        {
            let (r_this, c1) = self.get().overflowing_add(rhs.get());
            let cc = if carry { 1 as $f } else { 0 as $f };
            let (res_this, c2) = r_this.overflowing_add(cc);
            let res = Self::new_with(res_this);
            (res, c1 || c2)
        }

        #[inline] pub fn wrapping_add(self, rhs: Self) -> Self      { Self::new_with( self.get().wrapping_add(rhs.get()) ) }
        // #[inline] pub fn wrapping_add_assign(&mut self, rhs: Self)  { self.set(self.get().wrapping_add(rhs.get())); }

        pub fn overflowing_add(self, rhs: Self) -> (Self, bool)
        {
            let (res_this, carry) = self.get().overflowing_add(rhs.get());
            (Self::new_with(res_this), carry)
        }

        pub fn checked_add(self, rhs: Self) -> Option<Self>
        {
            match self.get().checked_add(rhs.get())
            {
                Some(res_this) =>   { Some(Self::new_with(res_this)) },
                None =>             { None },
            }
        }

        #[inline] pub fn unchecked_add(self, rhs: Self) -> Self     { Self::new_with( self.get().checked_add(rhs.get()).unwrap() ) }
        #[inline] pub fn saturating_add(self, rhs: Self) -> Self    { Self::new_with( self.get().saturating_add(rhs.get()) ) }

        pub fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool)
        {
            let (r_this, b1) = self.get().overflowing_sub(rhs.get());
            let (res_this, b2) = r_this.overflowing_sub(borrow as $f);
            (Self::new_with(res_this), b1 || b2)
        }

        #[inline] pub fn wrapping_sub(self, rhs: Self) -> Self      { Self::new_with( self.get().wrapping_sub(rhs.get()) ) }
        // #[inline] pub fn wrapping_sub_assign(&mut self, rhs: Self)  { self.set(self.get().wrapping_sub(rhs.get())); }
        
        pub fn overflowing_sub(self, rhs: Self) -> (Self, bool)
        {
            let (res_this, borrow) = self.get().overflowing_sub(rhs.get());
            (Self::new_with(res_this), borrow)
        }

        pub fn checked_sub(self, rhs: Self) -> Option<Self>
        {
            match self.get().checked_sub(rhs.get())
            {
                Some(res_this) =>   { Some(Self::new_with(res_this)) },
                None =>             { None },
            }
        }
        
        #[inline] pub fn unchecked_sub(self, rhs: Self) -> Self     { Self::new_with( self.get().checked_sub(rhs.get()).unwrap() ) }
        #[inline] pub fn saturating_sub(self, rhs: Self) -> Self    { Self::new_with( self.get().saturating_sub(rhs.get()) ) }

        #[inline] pub fn abs_diff(self, other: Self) -> Self    { Self::new_with( self.get().abs_diff(other.get()) ) }

        #[inline] pub fn wrapping_mul(self, rhs: Self) -> Self      { Self::new_with( self.get().wrapping_mul(rhs.get()) ) }
        // #[inline] pub fn wrapping_mul_assign(&mut self, rhs: Self)  { self.set(self.get().wrapping_mul(rhs.get())); }
        
        pub fn overflowing_mul(self, rhs: Self) -> (Self, bool)
        {
            let (res_this, carry) = self.get().overflowing_mul(rhs.get());
            (Self::new_with(res_this), carry)
        }

        pub fn checked_mul(self, rhs: Self) -> Option<Self>
        {
            match self.get().checked_mul(rhs.get())
            {
                Some(res_this) =>   { Some(Self::new_with(res_this)) },
                None =>             { None },
            }
        }

        #[inline] pub fn unchecked_mul(self, rhs: Self) -> Self     { Self::new_with( self.get().checked_mul(rhs.get()).unwrap() ) }
        #[inline] pub fn saturating_mul(self, rhs: Self) -> Self    { Self::new_with( self.get().saturating_mul(rhs.get()) ) }

        #[inline] pub fn wrapping_div(self, rhs: Self) -> Self      { Self::new_with( self.get().wrapping_div(rhs.get()) ) }
        // #[inline] pub fn wrapping_div_assign(&mut self, rhs: Self)  { self.this = self.get().wrapping_div(rhs.get()); }
        
        pub fn overflowing_div(self, rhs: Self) -> (Self, bool)
        {
            let (res_this, carry) = self.get().overflowing_div(rhs.get());
            (Self::new_with(res_this), carry)
        }

        pub fn checked_div(self, rhs: Self) -> Option<Self>
        {
            match self.get().checked_div(rhs.get())
            {
                Some(res_this) =>   { Some(Self::new_with(res_this)) },
                None =>             { None },
            }
        }

        #[inline] pub fn saturating_div(self, rhs: Self) -> Self    { Self::new_with( self.get().saturating_div(rhs.get()) ) }

        #[inline] pub fn wrapping_rem(self, rhs: Self) -> Self      { Self::new_with( self.get().wrapping_rem(rhs.get()) ) }
        // #[inline] pub fn wrapping_rem_assign(&mut self, rhs: Self)  { self.set(self.get().wrapping_rem(rhs.get())); }

        pub fn overflowing_rem(self, rhs: Self) -> (Self, bool)
        {
            let (res_this, carry) = self.get().overflowing_rem(rhs.get());
            (Self::new_with(res_this), carry)
        }

        pub fn checked_rem(self, rhs: Self) -> Option<Self>
        {
            match self.get().checked_rem(rhs.get())
            {
                Some(res_this) =>   { Some(Self::new_with(res_this)) },
                None =>             { None },
            }
        }

        #[inline] pub fn wrapping_neg(self) -> Self             { Self::new_with( self.get().wrapping_neg() ) }

        #[inline] pub fn wrapping_pow(self, exp: u32) -> Self   { Self::new_with( self.get().wrapping_pow(exp) ) }
        
        pub fn checked_pow(self, exp: u32) -> Option<Self>
        {
            match self.get().checked_pow(exp)
            {
                Some(res_this) =>   { Some(Self::new_with(res_this)) },
                None =>             { None },
            }
        }

        pub fn overflowing_pow(self, exp: u32) -> (Self, bool)
        {
            let (res_this, carry) = self.get().overflowing_pow(exp);
            (Self::new_with(res_this), carry)
        }

        #[inline] pub fn saturating_pow(self, exp: u32) -> Self { Self::new_with( self.get().saturating_pow(exp) ) }

        #[inline] pub fn pow(self, exp: u32) -> Self    { Self::new_with( self.get().pow(exp) ) }

        #[inline] pub fn ilog(self, base: Self) -> u32  { self.get().ilog(base.get()) }
        #[inline] pub fn ilog10(self) -> u32            { self.get().ilog10() }
        #[inline] pub fn ilog2(self) -> u32             { self.get().ilog2() }

        #[inline] pub fn isqrt(self) -> Self             { Self::new_with( self.get()._isqrt() ) }
        #[inline] pub fn root(self, exp: Self) -> Self  { Self::new_with( self.get().root(exp.get()) ) }

        #[inline] pub fn reverse_bits(self) -> Self     { Self::new_with( self.get().reverse_bits() ) }
        // #[inline] pub fn reverse_bits_assign(&mut self) { self.get().reverse_bits_assign(); }
        #[inline] pub fn rotate_left(self, n: u32) -> Self  { Self::new_with( self.get().rotate_left(n) ) }
        #[inline] pub fn rotate_right(self, n: u32) -> Self { Self::new_with( self.get().rotate_right(n) ) }

        #[inline] pub fn count_ones(self) -> u32        { self.get().count_ones() }
        #[inline] pub fn count_zeros(self) -> u32       { self.get().count_zeros() }
        #[inline] pub fn leading_ones(self) -> u32      { self.get().leading_ones() }
        #[inline] pub fn leading_zeros(self) -> u32     { self.get().leading_zeros() }
        #[inline] pub fn trailing_ones(self) -> u32     { self.get().trailing_ones() }
        #[inline] pub fn trailing_zeros(self) -> u32    { self.get().trailing_zeros() }

        #[inline] pub fn from_be(x: Self) -> Self   { Self::new_with( <$f>::from_be(x.get()) ) }
        #[inline] pub fn from_le(x: Self) -> Self   { Self::new_with( <$f>::from_le(x.get()) ) }
        #[inline] pub fn to_be(self) -> Self        { Self::new_with( self.get().to_be() ) }
        #[inline] pub fn to_le(self) -> Self        { Self::new_with( self.get().to_le() ) }
        #[inline] pub fn swap_bytes(self) -> Self   { Self::new_with( self.get().swap_bytes() ) }

        #[inline] pub fn is_power_of_two(self) -> bool    { self.get().is_power_of_two() }
        #[inline] pub fn next_power_of_two(self) -> Self  { Self::new_with( self.get().next_power_of_two() ) }

        // #[inline] pub fn into_f64(self) -> f64      { self.get() as f64 }
        // #[inline] pub fn into_f32(self) -> f32      { self.get() as f32 }
        // #[inline] pub fn into_u128(self) -> u128    { self.get() as u128 }
        // #[inline] pub fn into_u64(self) -> u64      { self.get() as u64 }
        // #[inline] pub fn into_u32(self) -> u32      { self.get() as u32 }
        // #[inline] pub fn into_u16(self) -> u16      { self.get() as u16 }
        // #[inline] pub fn into_u8(self) -> u8        { self.get() as u8 }
        // #[inline] pub fn into_usize(self) -> usize  { self.get() as usize }
        // #[inline] pub fn into_bool(self) -> bool    { self.get() != 0 }
        // #[inline] pub fn zero() -> Self             { Self::new_with(0 as $f) }
        // #[inline] pub fn one() -> Self              { Self::new_with(1 as $f) }
        // #[inline] pub fn max() -> Self              { Self::new_with(<$f>::MAX) }
        // #[inline] pub fn min() -> Self              { Self::new_with(<$f>::MIN) }
        // #[inline] pub fn num(n: u128) -> Self       { Self::new_with(n as $f) }
        // #[inline] pub fn size_in_bytes() -> usize   { size_of::<Self>() }
        // #[inline] pub fn size_in_bits() -> usize    { size_of::<Self>() * 8 }
        // #[inline] pub fn length_in_bytes(self) -> usize    { size_of_val(&self) }
        // #[inline] pub fn length_in_bits(self) -> usize     { size_of_val(&self) * 8 }
        // #[inline] pub fn is_odd(self) -> bool       { (self.get() & 1) != 0 }
    }
}



impl ShortUnion
{
    pub fn new() -> Self                    { Self { ushort: 0 } }
    pub fn new_with(ushort: u16) -> Self    { Self { ushort } }
    pub fn new_with_signed(sshort: i16) -> Self { Self { sshort } }
    pub fn new_with_ubytes(ubyte: [u8; 2]) -> Self  { Self { ubyte } }
    pub fn onoff(b: bool) -> Self           { Self { ushort: b as u16 } }
    pub fn onoff_signed(b: bool) -> Self    { Self { sshort: b as i16 } }
    pub fn new_with_u128(num: u128) -> Self { Self { ushort: LongerUnion::new_with(num).get_ushort_(0) } }

    #[inline] pub fn get(self) -> u16                   { unsafe { self.this } }
    #[inline] pub fn set(&mut self, val: u16)           { self.this = val; }
    #[inline] pub fn get_signed(self) -> i16            { unsafe { self.that } }
    #[inline] pub fn set_signed(&mut self, val: i16)    { self.that = val; }
    #[inline] pub fn get_ushort(self) -> u16            { unsafe { self.ushort } }
    #[inline] pub fn set_ushort(&mut self, val: u16)    { self.ushort = val; }
    #[inline] pub fn get_sshort(self) -> i16            { unsafe { self.sshort } }
    #[inline] pub fn set_sshort(&mut self, val: i16)    { self.sshort = val; }
    get_set_byte!(2-1);

    #[cfg(target_pointer_width = "8")]
    get_set_size!(2-1);

    #[cfg(target_pointer_width = "16")]
    #[inline] pub fn get_usize(self) -> usize       { unsafe { self.u_size } }

    #[cfg(target_pointer_width = "16")]
    #[inline] pub fn set_usize(self, val: usize)    { self.u_size = val; }

    #[cfg(target_pointer_width = "16")]
    #[inline] pub fn get_ssize(self) -> isize       { unsafe { self.s_size } }

    #[cfg(target_pointer_width = "16")]
    #[inline] pub fn set_ssize(self, val: isize)    { self.s_size = val; }

    integer_union_methods!(u16);
}



impl IntUnion
{
    pub fn new() -> Self                { Self { uint: 0 } }
    pub fn new_with(uint: u32) -> Self  { Self { uint } }
    pub fn new_with_signed(sint: i32) -> Self   { Self { sint } }
    pub fn new_with_ubytes(ubyte: [u8; 4]) -> Self  { Self { ubyte } }
    pub fn new_with_ushorts(ushort: [u16; 2]) -> Self   { Self { ushort } }
    pub fn onoff(b: bool) -> Self       { Self { uint: b as u32 } }
    pub fn onoff_signed(b: bool) -> Self    { Self { sint: b as i32 } }
    pub fn new_with_u128(num: u128) -> Self { Self { uint: LongerUnion::new_with(num).get_uint_(0) } }

    #[inline] pub fn get(self) -> u32               { unsafe { self.this } }
    #[inline] pub fn set(&mut self, val: u32)       { self.this = val; }
    #[inline] pub fn get_signed(self) -> i32        { unsafe { self.that } }
    #[inline] pub fn set_signed(&mut self, val: i32)    { self.that = val; }
    #[inline] pub fn get_uint(self) -> u32          { unsafe { self.uint } }
    #[inline] pub fn set_uint(&mut self, val: u32)  { self.uint = val; }
    #[inline] pub fn get_sint(self) -> i32          { unsafe { self.sint } }
    #[inline] pub fn set_sint(&mut self, val: i32)  { self.sint = val; }

    get_set_byte!(4-1);

    get_set_short!(2-1);

    #[cfg(target_pointer_width = "32")]
    #[inline] pub fn get_usize(self) -> usize   { unsafe { self.u_size } }

    #[cfg(target_pointer_width = "32")]
    #[inline] pub fn set_usize(&mut self, val: usize)   { self.u_size = val; }

    #[cfg(target_pointer_width = "32")]
    #[inline] pub fn get_ssize(self) -> isize   { unsafe { self.s_size } }

    #[cfg(target_pointer_width = "32")]
    #[inline] pub fn set_ssize(&mut self, val: isize)   { self.s_size = val; }

    #[cfg(target_pointer_width = "16")]     get_set_usize!(2-1);
    #[cfg(target_pointer_width = "8")]      get_set_usize!(4-1);

    integer_union_methods!(u32);
}



impl LongUnion
{
    pub fn new() -> Self                    { Self { ulong: 0 } }
    pub fn new_with(ulong: u64) -> Self     { Self { ulong } }
    pub fn new_with_signed(slong: i64) -> Self  { Self { slong } }
    pub fn new_with_ubytes(ubyte: [u8; 8])  -> Self { Self { ubyte } }
    pub fn new_with_ushorts(ushort: [u16; 4])  -> Self  { Self { ushort } }
    pub fn new_with_uints(uint: [u32; 2])  -> Self  { Self { uint } }
    pub fn onoff(b: bool) -> Self           { Self { ulong: b as u64 } }
    pub fn onoff_singed(b: bool) -> Self    { Self { slong: b as i64 } }
    pub fn new_with_u128(num: u128) -> Self { Self { ulong: LongerUnion::new_with(num).get_ulong_(0) } }

    #[inline] pub fn get(self) -> u64           { unsafe { self.this } }
    #[inline] pub fn get_signed(self) -> i64    { unsafe { self.that } }
    #[inline] pub fn set(&mut self, val: u64)   { self.this = val; }
    #[inline] pub fn set_signed(&mut self, val: i64)    { self.that = val; }
    #[inline] pub fn get_ulong(self) -> u64             { unsafe { self.ulong } }
    #[inline] pub fn set_ulong(&mut self, val: u64)     { self.ulong = val; }
    #[inline] pub fn get_slong(self) -> i64             { unsafe { self.slong } }
    #[inline] pub fn set_slong(&mut self, val: i64)     { self.slong = val; }
    get_set_byte!(8-1);
    get_set_short!(4-1);
    get_set_int!(2-1);

    #[cfg(target_pointer_width = "64")]
    #[inline] pub fn get_usize(self) -> usize           { unsafe { self.u_size } }

    #[cfg(target_pointer_width = "64")]
    #[inline] pub fn set_usize(&mut self, val: usize)   { self.u_size = val; }

    #[cfg(target_pointer_width = "64")]
    #[inline] pub fn get_ssize(self) -> isize           { unsafe { self.s_size } }

    #[cfg(target_pointer_width = "64")]
    #[inline] pub fn set_ssize(&mut self, val: isize)   { self.s_size = val; }

    #[cfg(target_pointer_width = "32")]     get_set_usize!(2-1);
    #[cfg(target_pointer_width = "16")]     get_set_usize!(4-1);
    #[cfg(target_pointer_width = "8")]      get_set_usize!(8-1);

    integer_union_methods!(u64);
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
    #[inline] pub fn get_ulonger(self) -> u128          { unsafe { self.ulonger } }
    #[inline] pub fn set_ulonger(&mut self, val: u128)  { self.ulonger = val; }
    #[inline] pub fn get_slonger(self) -> i128          { unsafe { self.slonger } }
    #[inline] pub fn set_slonger(&mut self, val: i128)  { self.slonger = val; }
    get_set_byte!(16-1);
    get_set_short!(8-1);
    get_set_int!(4-1);
    get_set_long!(2-1);

    #[cfg(target_pointer_width = "128")]
    #[inline] pub fn get_usize(self) -> usize           { unsafe { self.u_size } }

    #[cfg(target_pointer_width = "128")]
    #[inline] pub fn set_usize(&mut self, val: usize)   { self.u_size = val; }

    #[cfg(target_pointer_width = "128")]
    #[inline] pub fn get_ssize(self) -> isize           { unsafe { self.s_size } }

    #[cfg(target_pointer_width = "128")]
    #[inline] pub fn set_ssize(&mut self, val: isize)   { self.s_size = val; }

    #[cfg(target_pointer_width = "64")]     get_set_size!(2-1);
    #[cfg(target_pointer_width = "32")]     get_set_size!(4-1);
    #[cfg(target_pointer_width = "16")]     get_set_size!(8-1);
    #[cfg(target_pointer_width = "8")]      get_set_size!(16-1);

    integer_union_methods!(u128);
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
    #[cfg(target_pointer_width = "128")]    get_set_byte!(16-1);
    #[cfg(target_pointer_width = "64")]     get_set_byte!(8-1);
    #[cfg(target_pointer_width = "32")]     get_set_byte!(4-1);
    #[cfg(target_pointer_width = "16")]     get_set_byte!(2-1);

    #[cfg(target_pointer_width = "128")]    get_set_short!(8-1);
    #[cfg(target_pointer_width = "64")]     get_set_short!(4-1);
    #[cfg(target_pointer_width = "32")]     get_set_short!(2-1);

    #[cfg(target_pointer_width = "128")]    get_set_int!(4-1);
    #[cfg(target_pointer_width = "64")]     get_set_int!(2-1);

    #[cfg(target_pointer_width = "128")]    get_set_long!(2-1);

    pub fn set_usize(&mut self, val: usize)     { self.u_size = val; }
    pub fn get_usize(self) -> usize             { unsafe { self.u_size } }
    pub fn set_ssize(&mut self, val: isize)     { self.s_size = val; }
    pub fn get_ssize(&self) -> isize            { unsafe { self.s_size } }

    integer_union_methods!(usize);
}


/*
macro_rules! random_for_unions_impl {
    (ShortUnion) => {
        /// Constucts a new `SmallUInt`-type object which has the random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.any)
        #[inline] fn any() -> Self      { Self::new_with(thread_rng().gen()) }

        /// Make a `SmallUInt`-type object to have a random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.random)
        #[inline] fn random() -> Self   { Self::new_with(OsRng.next_u32() as u16) }
    };
    (IntUnion) => {
        /// Constucts a new `SmallUInt`-type object which has the random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.any)
        #[inline] fn any() -> Self      { Self::new_with(thread_rng().gen()) }

        /// Make a `SmallUInt`-type object to have a random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.random)
        #[inline] fn random() -> Self   { Self::new_with(OsRng.next_u32() as u32) }
    };
    (LongUnion) => {
        /// Constucts a new `SmallUInt`-type object which has the random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.any)
        #[inline] fn any() -> Self      { Self::new_with(thread_rng().gen()) }
        
        /// Make a `SmallUInt`-type object to have a random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.random)
        #[inline] fn random() -> Self   { Self::new_with(OsRng.next_u64()) }
    };
    (LongerUnion) => {
        /// Make a `SmallUInt`-type object to have a random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.random)
        #[inline] fn any() -> Self      { Self::new_with(thread_rng().gen()) }

        /// Make a `SmallUInt`-type object to have a random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.random)
        fn random() -> Self
        {
            let mut common = LongerUnion::new();
            common.set_ulong_(0, OsRng.next_u64());
            common.set_ulong_(1, OsRng.next_u64());
            common
        }
    };
    (SizeUnion) => {
        /// Constucts a new `SmallUInt`-type object which has the random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.any)
        #[inline] fn any() -> Self      { Self::new_with(thread_rng().gen()) }

        /// Make a `SmallUInt`-type object to have a random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.random)
        #[cfg(target_pointer_width = "8")]
        #[inline] fn random() -> Self   { Self::new_with(OsRng.next_u32() as usize) }

        /// Make a `SmallUInt`-type object to have a random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.random)
        #[cfg(target_pointer_width = "16")]
        #[inline] fn random() -> Self   { Self::new_with(OsRng.next_u32() as usize) }

        /// Make a `SmallUInt`-type object to have a random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.random)
        #[cfg(target_pointer_width = "32")]
        #[inline] fn random() -> Self   { Self::new_with(OsRng.next_u32() as usize) }
        
        /// Make a `SmallUInt`-type object to have a random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.random)
        #[cfg(target_pointer_width = "64")]
        #[inline] fn random() -> Self   { Self::new_with(OsRng.next_u64() as usize) }
        
        /// Make a `SmallUInt`-type object to have a random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.random)
        #[cfg(target_pointer_width = "128")]
        fn random() -> Self
        {
            let mut common = SizeUnion::new();
            common.set_ulong_(0, OsRng.next_u64());
            common.set_ulong_(1, OsRng.next_u64());
            common
        }
    };
}
*/


/// union array for transforming from one type into anther type
pub union Share<D, S>
where D: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=D> + AddAssign + Sub<Output=D> + SubAssign
        + Mul<Output=D> + MulAssign + Div<Output=D> + DivAssign
        + Rem<Output=D> + RemAssign
        + Shl<Output=D> + ShlAssign + Shr<Output=D> + ShrAssign
        + BitAnd<Output=D> + BitAndAssign + BitOr<Output=D> + BitOrAssign
        + BitXor<Output=D> + BitXorAssign + Not<Output=D>
        + PartialEq + PartialOrd,
      S: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=S> + AddAssign + Sub<Output=S> + SubAssign
        + Mul<Output=S> + MulAssign + Div<Output=S> + DivAssign
        + Rem<Output=S> + RemAssign
        + Shl<Output=S> + ShlAssign + Shr<Output=S> + ShrAssign
        + BitAnd<Output=S> + BitAndAssign + BitOr<Output=S> + BitOrAssign
        + BitXor<Output=S> + BitXorAssign + Not<Output=S>
        + PartialEq + PartialOrd
{
    pub des: D,
    pub src: S,
}

impl<D, S> Share<D, S>
where D: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=D> + AddAssign + Sub<Output=D> + SubAssign
        + Mul<Output=D> + MulAssign + Div<Output=D> + DivAssign
        + Rem<Output=D> + RemAssign
        + Shl<Output=D> + ShlAssign + Shr<Output=D> + ShrAssign
        + BitAnd<Output=D> + BitAndAssign + BitOr<Output=D> + BitOrAssign
        + BitXor<Output=D> + BitXorAssign + Not<Output=D>
        + PartialEq + PartialOrd,
      S: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=S> + AddAssign + Sub<Output=S> + SubAssign
        + Mul<Output=S> + MulAssign + Div<Output=S> + DivAssign
        + Rem<Output=S> + RemAssign
        + Shl<Output=S> + ShlAssign + Shr<Output=S> + ShrAssign
        + BitAnd<Output=S> + BitAndAssign + BitOr<Output=S> + BitOrAssign
        + BitXor<Output=S> + BitXorAssign + Not<Output=S>
        + PartialEq + PartialOrd
{
    pub fn new() -> Self
    {
        if size_of::<D>() >= size_of::<S>()
            { Self { des: D::zero() } }
        else
            { Self { src: S::zero() } }
    }

    pub fn from_src(src: S) -> Self
    {
        let mut me = Share::<D, S>::new();
        me.src = src;
        me
    }

    #[inline] pub fn get_des(&self) -> D  { unsafe { self.des } }
    #[inline] pub fn get_src(&self) -> S  { unsafe { self.src } }

    #[cfg(target_endian = "little")]
    pub fn into_des(&mut self, pos: usize) -> Option<D>
    {
        let bit_pos = pos * D::size_in_bits();
        unsafe { self.src >>= S::usize_as_smalluint(bit_pos); }
        if (bit_pos > 0) && self.is_src_zero()
            { None }
        else
            { unsafe { Some(self.des) } }
    }

    #[cfg(target_endian = "big")]
    pub fn into_des(&mut self, pos: usize) -> Option<D>
    {
        let des_size = size_of::<D>();
        let src_size = size_of::<S>();
        let bit_pos = pos * D::size_in_bits();
        unsafe { self.src <<= S::usize_as_smalluint(bit_pos); }
        if des_size > src_size
            { unsafe { self.des >>= D::num((des_size - src_size).into_u128() * 8); } }
        else if src_size > des_size
            { unsafe { self.src <<= S::num((src_size - des_size).into_u128() * 8); } }
        Some( unsafe { self.des } )
    }

    pub fn is_src_zero(&self) -> bool    { unsafe { self.src == S::zero() } }
}

/// union array for transforming from one type into anther type
pub union Common<D, const N: usize, S, const M: usize>
where D: SmallUInt + Add<Output=D> + AddAssign + Sub<Output=D> + SubAssign
        + Mul<Output=D> + MulAssign + Div<Output=D> + DivAssign
        + Rem<Output=D> + RemAssign
        + Shl<Output=D> + ShlAssign + Shr<Output=D> + ShrAssign
        + BitAnd<Output=D> + BitAndAssign + BitOr<Output=D> + BitOrAssign
        + BitXor<Output=D> + BitXorAssign + Not<Output=D>
        + PartialEq + PartialOrd
        + Display + ToString,
      S: SmallUInt + Add<Output=S> + AddAssign + Sub<Output=S> + SubAssign
        + Mul<Output=S> + MulAssign + Div<Output=S> + DivAssign
        + Shl<Output=S> + ShlAssign + Shr<Output=S> + ShrAssign
        + Rem<Output=S> + RemAssign
        + BitAnd<Output=S> + BitAndAssign + BitOr<Output=S> + BitOrAssign
        + BitXor<Output=S> + BitXorAssign + Not<Output=S>
        + PartialEq + PartialOrd
        + Display + ToString
{
    pub des: [D; N],
    pub src: [S; M],
}

impl<D, const N: usize, S, const M: usize> Common<D, N, S, M>
where D: SmallUInt + Add<Output=D> + AddAssign + Sub<Output=D> + SubAssign
        + Mul<Output=D> + MulAssign + Div<Output=D> + DivAssign
        + Rem<Output=D> + RemAssign
        + Shl<Output=D> + ShlAssign + Shr<Output=D> + ShrAssign
        + BitAnd<Output=D> + BitAndAssign + BitOr<Output=D> + BitOrAssign
        + BitXor<Output=D> + BitXorAssign + Not<Output=D>
        + PartialEq + PartialOrd
        + Display + ToString,
      S: SmallUInt + Add<Output=S> + AddAssign + Sub<Output=S> + SubAssign
        + Mul<Output=S> + MulAssign + Div<Output=S> + DivAssign
        + Rem<Output=S> + RemAssign
        + Shl<Output=S> + ShlAssign + Shr<Output=S> + ShrAssign
        + BitAnd<Output=S> + BitAndAssign + BitOr<Output=S> + BitOrAssign
        + BitXor<Output=S> + BitXorAssign + Not<Output=S>
        + PartialEq + PartialOrd
        + Display + ToString
{
    pub fn new() -> Self
    {
        if Self::size_of_des() >= Self::size_of_src()
            { Self { des: [D::zero(); N] } }
        else
            { Self { src: [S::zero(); M] } }
    }

    pub fn from_src(src: &[S; M]) -> Self
    {
        let mut me = Common::<D, N, S, M>::new();
        unsafe { me.src.copy_from_slice(src); }
        me
    }

    #[cfg(target_endian = "little")]
    #[inline]
    pub fn into_des(&mut self, des: &mut [D; N])
    {
        unsafe { des.copy_from_slice(&self.des); }
    }

    #[cfg(target_endian = "big")]
    pub fn into_des(&mut self, des: &mut [D; N])
    {
        let des_size = Self::size_of_des();
        let src_size = Self::size_of_src();
        if src_size > des_size
            { self.shl_assign_src((src_size - des_size) * 8); }
        else
            { self.shr_assign_des((des_size - src_size) * 8); }
        des.copy_from_slice(&self.des);
    }

    #[cfg(target_endian = "big")]
    fn shl_assign_src(&mut self, rhs: usize)
    {
        let TSIZE_BIT = size_of::<D>() * 8;
        let chunk_num = rhs as usize / TSIZE_BIT as usize;
        let piece_num = rhs as usize % TSIZE_BIT as usize;
        let zero = S::zero();
        if chunk_num > 0
        {
            self.src.copy_within(chunk_num..N, 0);
            for idx in N-chunk_num..N
                { self.src[idx] = zero; }
        }
        if piece_num == 0
            { return; }

        let mut num: S;
        let mut carry = zero;
        for idx in 0..N-chunk_num
        {
            num = (self.src[idx] << S::num(piece_num.into_u128())) | carry;
            carry = self.src[idx] >> S::num((TSIZE_BIT - piece_num).into_u128());
            self.src[idx] = num;
        }
    }

    #[cfg(target_endian = "big")]
    fn shr_assign_des(&mut self, rhs: usize)
    {
        let TSIZE_BIT = size_of::<T>() * 8;
        let chunk_num = rhs as usize / TSIZE_BIT as usize;
        let piece_num = rhs as usize % TSIZE_BIT as usize;
        let zero = D::zero();
        if chunk_num > 0
        {
            self.des.copy_within(0..N-chunk_num, chunk_num);
            for idx in 0..chunk_num
                { self.des[idx] = zero; }
        }
        if piece_num == 0
            { return; }

        let mut num: D;
        let mut carry = D::zero();
        let mut idx = 0;
        loop
        {
            num = (self.des[idx] >> D::num(piece_num.into_u128())) | carry;
            carry = self.des[idx] << D::num((TSIZE_BIT - piece_num).into_u128());
            self.des[idx] = num;
            if idx == N - 1 - chunk_num
                { break; }
            idx += 1;
        }
    }

    pub fn size_of_des() -> usize   { size_of::<D>() * N }
    pub fn size_of_src() -> usize   { size_of::<S>() * M }
}
