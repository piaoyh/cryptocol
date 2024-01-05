// Copyright 2023, 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module that contains generic types of primitive unsigned integral
//! data types used in a lot of modules of the crate Cryptocol.
//! __These unions are for segmentation.__

#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

use std::fmt::{ self, Debug, Display, Formatter };
use std::mem::{ size_of, size_of_val };
use std::cmp::{ PartialEq, PartialOrd, Ordering };
use std::ops::*;

// use rand::{ RngCore, thread_rng, Rng };
// use rand::rngs::OsRng;

use super::small_uint::*;

/// This union `ShortUnion` is for converting one primitive integral type into
/// another integeral type within 16-bit long type.
/// 
/// # Feature
/// All the fields are pubic but it is highly encouraged to get/set methods
/// instead of accessing to each field directly. The simple get/set methods are
/// all inline methods so that you hardly lose the performance benefit because
/// of using get/set methods.
/// 
/// # Example
/// ```
/// use Cryptocol::number::*;
/// let a = ShortUnion::new_with(55468_u16);
/// println!("a.this = {}, {}", unsafe { a.this }, a.get());
/// println!("a.that = {}, {}", unsafe { a.that }, a.get_signed());
/// println!("a.ushort = {}", unsafe { a.ushort });
/// println!("a.sshort = {}", unsafe { a.sshort });
/// #[cfg(target_endian = "little")]
/// {
///     for i in 0..2
///         { println!("a.ubyte[{}] = {}, {}", i, unsafe { a.ubyte[i] }, a.get_ubyte_(i)); }
///     for i in 0..2
///         { println!("a.sbyte[{}] = {}, {}", i, unsafe { a.sbyte[i] }, a.get_sbyte_(i)); }
///     #[cfg(target_pointer_width = "8")]
///     {
///         const N: usize = 2;
///         for i in 0..N
///             { println!("a.u_size[{}] = {}, {}", i, unsafe { a.u_size[i] }, a.get_usize_(i)); }
///         for i in 0..N
///             { println!("a.s_size[{}] = {}, {}", i, unsafe { a.s_size[i] }, a.get_ssize_(i)); }
///     }
/// }
/// #[cfg(target_pointer_width = "16")]
/// {
///     println!("a.u_size = {}", unsafe { a.u_size });
///     println!("a.s_size = {}", unsafe { a.s_size });
/// }
/// 
/// assert_eq!(unsafe { a.this }, 55468_u16);
/// assert_eq!(unsafe { a.that }, -10068_i16);
/// assert_eq!(unsafe { a.ushort }, 55468_u16);
/// assert_eq!(unsafe { a.sshort }, -10068_i16);
/// #[cfg(target_endian = "little")]
/// {
///     assert_eq!(unsafe { a.ubyte[0] }, 172_u8);
///     assert_eq!(unsafe { a.ubyte[1] }, 216_u8);
///     assert_eq!(unsafe { a.sbyte[0] }, -84_i8);
///     assert_eq!(unsafe { a.sbyte[1] }, -40_i8);
/// }
/// ```
/// 
/// # Big-endian issue
/// It is just experimental for Big Endian CPUs. So, you are not encouraged
/// to use it for serious purpose. Only use this crate for Big-endian CPUs
/// with your own full responsibility.
#[derive(Copy, Clone)]
pub union ShortUnion
{
    /// The biggest unsigned element for compatibility with other unions
    pub this: u16,

    /// The biggest signed element for compatibility with other unions
    pub that: i16,

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
/// use Cryptocol::number::*;
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
pub union IntUnion
{
    /// The biggest unsigned element for compatibility with other unions
    pub this: u32,

    /// The biggest signed element for compatibility with other unions
    pub that: i32,

    /// The biggest unsigned element which is 32-bit unsigned integer
    pub uint: u32,

    /// The biggest signed element which is 32-bit unsigned integer
    pub sint: i32,

    /// The secondly biggest unsigned element array whose elements are
    /// 16-bit unsigned integer
    pub ushort: [u16; 2],

    /// The secondly biggest signed element array whose elements are
    /// 16-bit unsigned integer
    pub sshort: [i16; 2],

    /// The thirdly biggest unsigned element array whose elements are
    /// 8-bit unsigned integer
    pub ubyte: [u8; 4],

    /// The thirdly biggest signed element array whose elements are
    /// 8-bit unsigned integer
    pub sbyte: [i8; 4],

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
/// use Cryptocol::number::*;
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
pub union LongUnion
{
    /// The biggest unsigned element for compatibility with other unions
    pub this: u64,

    /// The biggest signed element for compatibility with other unions
    pub that: i64,

    /// The biggest unsigned element which is 64-bit unsigned integer
    pub ulong: u64,

    /// The biggest signed element which is 64-bit unsigned integer
    pub slong: i64,

    /// The secondly biggest unsigned element array whose elements are
    /// 32-bit unsigned integer
    pub uint: [u32; 2],

    /// The secondly biggest signed element array whose elements are
    /// 32-bit unsigned integer
    pub sint: [i32; 2],

    /// The thirdly biggest unsigned element array whose elements are
    /// 16-bit unsigned integer
    pub ushort: [u16; 4],

    /// The thirdly biggest signed element array whose elements are
    /// 16-bit unsigned integer
    pub sshort: [i16; 4],

    /// The fourthly biggest unsigned element array whose elements are
    /// 8-bit unsigned integer
    pub ubyte: [u8; 8],

    /// The fourthly biggest unsigned element array whose elements are
    /// 8-bit signed integer
    pub sbyte: [i8; 8],

    /// The usize type element whose size is the same as the LongUnion
    #[cfg(target_pointer_width = "64")] pub u_size: usize,

    /// The isize type element whose size is the same as the LongUnion
    #[cfg(target_pointer_width = "64")] pub s_size: isize,

    /// The usize type array whose elements's size is 32-bit size
    #[cfg(target_pointer_width = "32")] pub u_size: [usize; 2],

    /// The isize type array whose elements's size is 32-bit size
    #[cfg(target_pointer_width = "32")] pub s_size: [isize; 2],

    /// The usize type array whose elements's size is 16-bit size
    #[cfg(target_pointer_width = "16")] pub u_size: [usize; 4],

    /// The isize type array whose elements's size is 16-bit size
    #[cfg(target_pointer_width = "16")] pub s_size: [isize; 4],

    /// The usize type array whose elements's size is 8-bit size
    #[cfg(target_pointer_width = "8")] pub u_size: [usize; 8],

    /// The isize type array whose elements's size is 8-bit size
    #[cfg(target_pointer_width = "8")] pub s_size: [isize; 8],
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
/// use Cryptocol::number::*;
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
pub union LongerUnion
{
    /// The biggest unsigned element for compatibility with other unions
    pub this: u128,

    /// The biggest signed element for compatibility with other unions
    pub that: i128,

    /// The biggest unsigned element which is 128-bit unsigned integer
    pub ulonger: u128,

    /// The biggest signed element which is 128-bit unsigned integer
    pub slonger: i128,

    /// The secondly biggest unsigned element array whose elements are
    /// 64-bit unsigned integer
    pub ulong: [u64; 2],

    /// The secondly biggest signed element array whose elements are
    /// 64-bit unsigned integer
    pub slong: [i64; 2],

    /// The thirdly biggest unsigned element array whose elements are
    /// 32-bit unsigned integer
    pub uint: [u32; 4],

    /// The thirdly biggest signed element array whose elements are
    /// 32-bit unsigned integer
    pub sint: [i32; 4],

    /// The fourthly biggest unsigned element array whose elements are
    /// 16-bit unsigned integer
    pub ushort: [u16; 8],

    /// The fourthly biggest unsigned element array whose elements are
    /// 16-bit unsigned integer
    pub sshort: [i16; 8],

    /// The fifthly biggest unsigned element array whose elements are
    /// 8-bit unsigned integer
    pub ubyte: [u8; 16],

    /// The fifthly biggest signed element array whose elements are
    /// 8-bit unsigned integer
    pub sbyte: [i8; 16],

    /// The usize type element whose size is the same as the LongerUnion
    #[cfg(target_pointer_width = "128")] pub u_size: usize,

    /// The isize type element whose size is the same as the LongerUnion
    #[cfg(target_pointer_width = "128")] pub s_size: isize,

    /// The isize type array whose elements's size is 64-bit size
    #[cfg(target_pointer_width = "64")] pub u_size: [usize; 2],

    /// The isize type array whose elements's size is 64-bit size
    #[cfg(target_pointer_width = "64")] pub s_size: [isize; 2],

    /// The usize type array whose elements's size is 32-bit size
    #[cfg(target_pointer_width = "32")] pub u_size: [usize; 4],

    /// The isize type array whose elements's size is 32-bit size
    #[cfg(target_pointer_width = "32")] pub s_size: [isize; 4],

    /// The usize type array whose elements's size is 16-bit size
    #[cfg(target_pointer_width = "16")] pub u_size: [usize; 8],

    /// The isize type array whose elements's size is 16-bit size
    #[cfg(target_pointer_width = "16")] pub s_size: [isize; 8],

    /// The usize type array whose elements's size is 8-bit size
    #[cfg(target_pointer_width = "8")] pub u_size: [usize; 16],

    /// The isize type array whose elements's size is 8-bit size
    #[cfg(target_pointer_width = "8")] pub s_size: [isize; 16],
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
/// use Cryptocol::number::*;
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
/// use Cryptocol::number::*;
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
/// use Cryptocol::number::*;
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
/// use Cryptocol::number::*;
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
/// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        /// use Cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use Cryptocol::number::*;
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
        #[inline] pub fn wrapping_add_assign(&mut self, rhs: Self)  { self.set(self.get().wrapping_add(rhs.get())); }

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
        #[inline] pub fn wrapping_sub_assign(&mut self, rhs: Self)  { self.set(self.get().wrapping_sub(rhs.get())); }
        
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
        #[inline] pub fn wrapping_mul_assign(&mut self, rhs: Self)  { self.set(self.get().wrapping_mul(rhs.get())); }
        
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
        #[inline] pub fn wrapping_div_assign(&mut self, rhs: Self)  { self.this = self.get().wrapping_div(rhs.get()); }
        
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
        #[inline] pub fn wrapping_rem_assign(&mut self, rhs: Self)  { self.set(self.get().wrapping_rem(rhs.get())); }

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

        #[inline] pub fn sqrt(self) -> Self             { Self::new_with( self.get().sqrt() ) }
        #[inline] pub fn root(self, exp: Self) -> Self  { Self::new_with( self.get().root(exp.get()) ) }

        #[inline] pub fn reverse_bits(self) -> Self     { Self::new_with( self.get().reverse_bits() ) }
        #[inline] pub fn reverse_bits_assign(&mut self) { self.get().reverse_bits_assign(); }
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

        #[inline] pub fn into_f64(self) -> f64      { self.get() as f64 }
        #[inline] pub fn into_f32(self) -> f32      { self.get() as f32 }
        #[inline] pub fn into_u128(self) -> u128    { self.get() as u128 }
        #[inline] pub fn into_u64(self) -> u64      { self.get() as u64 }
        #[inline] pub fn into_u32(self) -> u32      { self.get() as u32 }
        #[inline] pub fn into_u16(self) -> u16      { self.get() as u16 }
        #[inline] pub fn into_u8(self) -> u8        { self.get() as u8 }
        #[inline] pub fn into_usize(self) -> usize  { self.get() as usize }
        #[inline] pub fn into_bool(self) -> bool    { self.get() != 0 }
        #[inline] pub fn zero() -> Self             { Self::new_with(0 as $f) }
        #[inline] pub fn one() -> Self              { Self::new_with(1 as $f) }
        #[inline] pub fn max() -> Self              { Self::new_with(<$f>::MAX) }
        #[inline] pub fn min() -> Self              { Self::new_with(<$f>::MIN) }
        #[inline] pub fn num(n: u128) -> Self       { Self::new_with(n as $f) }
        #[inline] pub fn size_in_bytes() -> usize   { size_of::<Self>() }
        #[inline] pub fn size_in_bits() -> usize    { size_of::<Self>() * 8 }
        #[inline] pub fn length_in_bytes(self) -> usize    { size_of_val(&self) }
        #[inline] pub fn length_in_bits(self) -> usize     { size_of_val(&self) * 8 }
        #[inline] pub fn is_odd(self) -> bool       { (self.get() & 1) != 0 }
    }
}



impl ShortUnion
{
    pub fn new() -> Self                    { Self { ushort: 0 } }
    pub fn new_with(ushort: u16) -> Self    { Self { ushort } }
    pub fn new_with_signed(sshort: i16) -> Self { Self { sshort } }
    pub fn onoff(b: bool) -> Self           { Self { ushort: b as u16 } }
    pub fn onoff_signed(b: bool) -> Self    { Self { sshort: b as i16 } }
    pub fn new_with_u128(num: u128) -> Self { Self { ushort: LongerUnion::new_with(num).get_ushort_(0) } }

    #[inline] pub fn get(self) -> u16                 { unsafe { self.ushort } }
    #[inline] pub fn get_signed(self) -> i16          { unsafe { self.sshort } }
    #[inline] pub fn set(&mut self, val: u16)         { self.ushort = val; }
    #[inline] pub fn set_signed(&mut self, val: i16)  { self.sshort = val; }
    get_set_byte!(2-1);

    #[cfg(target_pointer_width = "8")]      get_set_usize!(2-1);

    integer_union_methods!(u16);
}



impl IntUnion
{
    pub fn new() -> Self                { Self { uint: 0 } }
    pub fn new_with(uint: u32) -> Self  { Self { uint } }
    pub fn new_with_signed(sint: i32) -> Self   { Self { sint } }
    pub fn onoff(b: bool) -> Self       { Self { uint: b as u32 } }
    pub fn onoff_signed(b: bool) -> Self    { Self { sint: b as i32 } }
    pub fn new_with_u128(num: u128) -> Self { Self { uint: LongerUnion::new_with(num).get_uint_(0) } }

    #[inline] pub fn get(self) -> u32             { unsafe { self.uint } }
    #[inline] pub fn get_signed(self) -> i32      { unsafe { self.sint } }
    #[inline] pub fn set(&mut self, val: u32)     { self.uint = val; }
    #[inline] pub fn set_signed(&mut self, val: i32)     { self.sint = val; }
    get_set_byte!(4-1);
    get_set_short!(2-1);

    #[cfg(target_pointer_width = "16")]     get_set_usize!(2-1);
    #[cfg(target_pointer_width = "8")]      get_set_usize!(4-1);

    integer_union_methods!(u32);
}



impl LongUnion
{
    pub fn new() -> Self                    { Self { ulong: 0 } }
    pub fn new_with(ulong: u64) -> Self     { Self { ulong } }
    pub fn new_with_signed(slong: i64) -> Self  { Self { slong } }
    pub fn onoff(b: bool) -> Self           { Self { ulong: b as u64 } }
    pub fn onoff_singed(b: bool) -> Self    { Self { slong: b as i64 } }
    pub fn new_with_u128(num: u128) -> Self { Self { ulong: LongerUnion::new_with(num).get_ulong_(0) } }

    #[inline] pub fn get(self) -> u64           { unsafe { self.ulong } }
    #[inline] pub fn get_signed(self) -> i64    { unsafe { self.slong } }
    #[inline] pub fn set(&mut self, val: u64)   { self.ulong = val; }
    #[inline] pub fn set_signed(&mut self, val: i64)    { self.slong = val; }
    get_set_byte!(8-1);
    get_set_short!(4-1);
    get_set_int!(2-1);

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
    pub fn onoff(b: bool) -> Self           { Self { ulonger: b as u128 } }
    pub fn onoff_signed(b: bool) -> Self    { Self { slonger: b as i128 } }
    pub fn new_with_u128(num: u128) -> Self { Self { ulonger: num } }

    #[inline] pub fn get(self) -> u128          { unsafe { self.ulonger } }
    #[inline] pub fn get_signed(self) -> i128   { unsafe { self.slonger } }
    #[inline] pub fn set(&mut self, val: u128)  { self.ulonger = val; }
    #[inline] pub fn set_signed(&mut self, val: i128)    { self.slonger = val; }
    get_set_byte!(16-1);
    get_set_short!(8-1);
    get_set_int!(4-1);
    get_set_long!(2-1);

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

macro_rules! SmallUInt_methods_for_integer_unions_impl {
    ($f:ty, $g:ty) => {
        /// Calculates self + rhs + carry and returns a tuple containing
        /// the sum and the output carry.
        /// [Read more in detail](trait@SmallUInt#tymethod.carrying_add)
        #[inline] fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool)   { self.carrying_add(rhs, carry) }

        /// Computes self + rhs, wrapping around at the boundary of the type.
        /// [Read more in detail](trait@SmallUInt#tymethod.wrapping_add)
        #[inline] fn wrapping_add(self, rhs: Self) -> Self  { self.wrapping_add(rhs) }

        /// Calculates self + rhs and returns a tuple of the addition along
        /// with a boolean indicating whether an arithmetic overflow would
        /// occur. [Read more in detail](trait@SmallUInt#tymethod.overflowing_add)
        #[inline] fn overflowing_add(self, rhs: Self) -> (Self, bool)   { self.overflowing_add(rhs) }
        
        /// Computes self + rhs and returns None if overflow occurred.
        /// [Read more in detail](trait@SmallUInt#tymethod.checked_add)
        #[inline] fn checked_add(self, rhs: Self) -> Option<Self>   { self.checked_add(rhs) }
        
        /// Computes self + rhs, assuming overflow cannot occur.
        /// [Read more in detail](trait@SmallUInt#tymethod.unchecked_add)
        #[inline] fn unchecked_add(self, rhs: Self) -> Self     { self.checked_add(rhs).unwrap() }
        
        /// Computes self + rhs, saturating at the numeric bounds
        /// instead of overflowing.
        /// [Read more in detail](trait@SmallUInt#tymethod.saturating_add)
        #[inline] fn saturating_add(self, rhs: Self) -> Self    { self.saturating_add(rhs) }

        /// Computes `self` + `rhs`, wrapping at `modulo`
        /// instead of overflowing.
        /// [Read more in detail](trait@SmallUInt#tymethod.modular_add)
        fn modular_add(self, rhs: Self, modulo: Self) -> Self
        {
            let mlhs = self.wrapping_rem(modulo);
            let mrhs = rhs.wrapping_rem(modulo);
            let diff = modulo.wrapping_sub(mrhs);
            if self.get() >= diff.get()
                { mlhs.wrapping_sub(diff) }
            else
                { mlhs.wrapping_add(mrhs) }
        }

        /// Calculates self − rhs − borrow,
        /// wrapping around at the boundary of the type.
        /// [Read more in detail](trait@SmallUInt#tymethod.borrowing_sub)
        #[inline] fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool)   { self.borrowing_sub(rhs, borrow) }

        /// Computes self - rhs, wrapping around at the boundary of the type.
        /// [Read more in detail](trait@SmallUInt#tymethod.wrapping_sub)
        #[inline] fn wrapping_sub(self, rhs: Self) -> Self  { self.wrapping_sub(rhs) }

        /// Calculates self - rhs and returns a tuple of the subtraction
        /// along with a boolean indicating whether an arithmetic overflow
        /// would occur. [Read more in detail](trait@SmallUInt#tymethod.overflowing_sub)
        #[inline] fn overflowing_sub(self, rhs: Self) -> (Self, bool)   { self.overflowing_sub(rhs) }

        /// Computes self - rhs, returning None if overflow occurred.
        /// [Read more in detail](trait@SmallUInt#tymethod.checked_sub)
        #[inline] fn checked_sub(self, rhs: Self) -> Option<Self>   { self.checked_sub(rhs) }

        /// Computes self - rhs, assuming overflow cannot occur.
        /// [Read more in detail](trait@SmallUInt#tymethod.unchecked_sub)
        #[inline] fn unchecked_sub(self, rhs: Self) -> Self     { self.checked_sub(rhs).unwrap() }

        /// Computes self - rhs, saturating at the numeric bounds
        /// instead of overflowing.
        /// [Read more in detail](trait@SmallUInt#tymethod.saturating_sub)
        #[inline] fn saturating_sub(self, rhs: Self) -> Self    { self.saturating_sub(rhs) }

        /// Computes the absolute difference between `self` and `other`.
        /// [Read more in detail](trait@SmallUInt#tymethod.abs_diff)
        #[inline] fn abs_diff(self, other: Self) -> Self    { self.abs_diff(other) }

        /// Computes `self` - `rhs`, wrapping at `modulo`
        /// instead of overflowing.
        /// [Read more in detail](trait@SmallUInt#tymethod.modular_sub)
        fn modular_sub(self, rhs: Self, modulo: Self) -> Self
        {
            let mlhs = self.wrapping_rem(modulo);
            let mrhs = rhs.wrapping_rem(modulo);
            if mlhs.get() >= mrhs.get()
            {
                mlhs.wrapping_sub(mrhs)
            }
            else
            {
                let diff = modulo.wrapping_sub(mrhs);
                mlhs.wrapping_add(diff)
            }
        }

        /// Calculates the “full multiplication” `self` * `rhs` + `carry` without
        /// the possibility to overflow.
        /// [Read more in detail](trait@SmallUInt#tymethod.carrying_mul)
        #[inline] fn carrying_mul(self, rhs: Self, carry: Self) -> (Self, Self) { self.carrying_mul_for_internal_use(rhs, carry) }

        // fn carrying_mul_for_internal_use(self, rhs: Self, carry: Self) -> (Self, Self);
        /// It is for internal use. You are recommended to use [carrying_mul()](trait@SmallUInt#tymethod.carrying_mul) instead.
        fn carrying_mul_for_internal_use(self, rhs: Self, carry: Self) -> (Self, Self)
        {
            let (low, high) = self.get().carrying_mul_for_internal_use(rhs.get(), carry.get());
            (Self::new_with(low), Self::new_with(high))
        }

        /// Calculates the complete product `self` * `rhs` without the possibility
        /// to overflow. [Read more in detail](trait@SmallUInt#tymethod.widening_mul)
        #[inline] fn widening_mul(self, rhs: Self) -> (Self, Self)  { self.widening_mul_for_internal_use(rhs) }

        // fn carrying_mul_for_internal_use(self, rhs: Self, carry: Self) -> (Self, Self);
        /// It is for internal use. You are recommended to use [carrying_mul()](trait@SmallUInt#tymethod.widening_mul) instead.
        fn widening_mul_for_internal_use(self, rhs: Self) -> (Self, Self)
        {
            let (low, high) = self.get().widening_mul_for_internal_use(rhs.get());
            (Self::new_with(low), Self::new_with(high))
        }

        /// Computes self * rhs, wrapping around at the boundary of the type.
        /// [Read more in detail](trait@SmallUInt#tymethod.wrapping_mul)
        #[inline] fn wrapping_mul(self, rhs: Self) -> Self  { self.wrapping_mul(rhs) }

        /// Calculates the multiplication of self and rhs and returns a tuple
        /// of the multiplication along with a boolean indicating whether an
        /// arithmetic overflow would occur.
        /// [Read more in detail](trait@SmallUInt#tymethod.overflowing_mul)
        #[inline] fn overflowing_mul(self, rhs: Self) -> (Self, bool)   { self.overflowing_mul(rhs) }

        /// Computes self * rhs, returning None if overflow occurred.
        /// [Read more in detail](trait@SmallUInt#tymethod.checked_mul)
        #[inline] fn checked_mul(self, rhs: Self) -> Option<Self>   { self.checked_mul(rhs) }

        /// Computes self * rhs, assuming overflow cannot occur.
        /// [Read more in detail](trait@SmallUInt#tymethod.unchecked_mul)
        #[inline] fn unchecked_mul(self, rhs: Self) -> Self     { self.checked_mul(rhs).unwrap() }

        /// Computes self * rhs, saturating at the numeric bounds
        /// instead of overflowing.
        /// [Read more in detail](trait@SmallUInt#tymethod.saturating_mul)
        #[inline] fn saturating_mul(self, rhs: Self) -> Self    { self.saturating_mul(rhs) }

        /// Computes `self` * `rhs`, wrapping at `modulo`
        /// instead of overflowing.
        /// [Read more in detail](trait@SmallUInt#tymethod.modular_mul)
        fn modular_mul(self, rhs: Self, modulo: Self) -> Self
        {
            let mut mrhs = rhs.wrapping_rem(modulo);
            let mut mlhs = self.wrapping_rem(modulo);
            let mut res = Self::zero();
            while mrhs.get() > Self::zero().get()
            {
                if mrhs.is_odd()
                    { res = res.modular_add(mlhs, modulo); }
                mlhs = mlhs.modular_add(mlhs, modulo);
                mrhs.set(mrhs.get() >> 1);
            }
            res
        }

        /// Computes self / rhs. Wrapped division on unsigned types is just
        /// normal division. [Read more in detail](trait@SmallUInt#tymethod.wrapping_div)
        #[inline] fn wrapping_div(self, rhs: Self) -> Self  { self.wrapping_div(rhs) }

        /// Calculates the divisor when self is divided by rhs and returns
        /// a tuple of the divisor along with a boolean indicating whether
        /// an arithmetic overflow would occur.
        /// [Read more in detail](trait@SmallUInt#tymethod.overflowing_div)
        #[inline] fn overflowing_div(self, rhs: Self) -> (Self, bool)   { self.overflowing_div(rhs) }

        /// Computes self / rhs, returning None if rhs == 0.
        /// [Read more in detail](trait@SmallUInt#tymethod.checked_div)
        #[inline] fn checked_div(self, rhs: Self) -> Option<Self>   { self.checked_div(rhs) }

        /// Computes self / rhs, saturating at the numeric bounds
        /// instead of overflowing.
        /// [Read more in detail](trait@SmallUInt#tymethod.saturating_div)
        #[inline] fn saturating_div(self, rhs: Self) -> Self    { self.saturating_div(rhs) }


        /// Computes self % rhs. Wrapped remainder calculation on unsigned
        /// types is just the regular remainder calculation.
        /// [Read more in detail](trait@SmallUInt#tymethod.wrapping_rem)
        #[inline] fn wrapping_rem(self, rhs: Self) -> Self  { self.wrapping_rem(rhs) }

        /// Calculates the remainder when self is divided by rhs, and returns
        /// a tuple of the remainder after dividing along with a boolean
        /// indicating whether an arithmetic overflow would occur.
        /// [Read more in detail](trait@SmallUInt#tymethod.overflowing_rem)
        #[inline] fn overflowing_rem(self, rhs: Self) -> (Self, bool)   { self.overflowing_rem(rhs) }

        /// Computes self % rhs, returning None if rhs == 0.
        /// [Read more in detail](trait@SmallUInt#tymethod.checked_rem)
        #[inline] fn checked_rem(self, rhs: Self) -> Option<Self>   { self.checked_rem(rhs) }


        /// Raises `self` to the power of `exp`, using exponentiation by squaring.
        /// [Read more in detail](trait@SmallUInt#tymethod.pow)
        #[inline] fn pow(self, exp: u32) -> Self    { self.pow(exp) }

        /// Computes self.pow(exp), wrapping around at the boundary of the type.
        /// [Read more in detail](trait@SmallUInt#tymethod.wrapping_pow)
        #[inline] fn wrapping_pow(self, exp: u32) -> Self   { self.wrapping_pow(exp) }

        /// Raises self to the power of exp, using exponentiation by squaring.
        /// [Read more in detail](trait@SmallUInt#tymethod.overflowing_pow)
        #[inline] fn overflowing_pow(self, exp: u32) -> (Self, bool)    { self.overflowing_pow(exp) }

        /// Computes self.pow(exp), returning None if overflow occurred.
        /// [Read more in detail](trait@SmallUInt#tymethod.checked_pow)
        #[inline] fn checked_pow(self, exp: u32) -> Option<Self>    { self.checked_pow(exp) }

        /// Computes self.pow(exp), saturating at the numeric bounds
        /// instead of overflowing.
        /// [Read more in detail](trait@SmallUInt#tymethod.saturating_pow)
        #[inline] fn saturating_pow(self, exp: u32) -> Self     { self.saturating_pow(exp) }

        /// Computes self.pow(exp), saturating at `modulo`
        /// instead of overflowing.
        /// [Read more in detail](trait@SmallUInt#tymethod.modular_pow)
        fn modular_pow(self, exp: Self, modulo: Self) -> Self
        {
            let mut mlhs = self.wrapping_rem(modulo);
            let mut res = Self::one();
            let mut mexp = exp;
            while mexp.get() > 0
            {
                if mexp.is_odd()
                    { res = res.modular_mul(mlhs, modulo); }
                mlhs = mlhs.modular_mul(mlhs, modulo);
                mexp.set(mexp.get() >> 1);
            }
            res
        }

        #[inline] fn ilog(self, base: Self) -> u32  { self.ilog(base) }
        #[inline] fn ilog10(self) -> u32            { self.ilog10() }
        #[inline] fn ilog2(self) -> u32             { self.ilog2() }

        #[inline] fn sqrt(self) -> Self             { self.sqrt() }
        #[inline] fn root(self, exp: Self) -> Self  { self.root(exp) }



/***** METHODS FOR GENERATING RANDOM PRIME NUMBERS *****/

        /// Performs Millar Rabin method with a number less than `self`.
        /// [Read more in detail](trait@SmallUInt#tymethod.is_prime_using_Miller_Rabin)
        #[inline] fn is_prime_using_Miller_Rabin(self, repetition: usize) -> bool   { self.get().is_prime_using_Miller_Rabin(repetition) }

        /// Tests a `SmallUInt`-type object to find whether or not it is a
        /// prime number.
        /// [Read more in detail](trait@SmallUInt#tymethod.test_Miller_Rabin)
        #[inline] fn test_Miller_Rabin(self, a: Self) -> bool   { self.get().test_Miller_Rabin(a.get()) }

        #[inline] fn reverse_bits(self) -> Self     { self.reverse_bits() }
        #[inline] fn reverse_bits_assign(&mut self) { *self = self.reverse_bits(); }

        #[inline] fn rotate_left(self, n: u32) -> Self  { self.rotate_left(n) }
        #[inline] fn rotate_right(self, n: u32) -> Self { self.rotate_right(n) }

        #[inline] fn count_ones(self) -> u32        { self.count_ones() }
        #[inline] fn count_zeros(self) -> u32       { self.count_zeros() }
        #[inline] fn leading_ones(self) -> u32      { self.leading_ones() }
        #[inline] fn leading_zeros(self) -> u32     { self.leading_zeros() }
        #[inline] fn trailing_ones(self) -> u32     { self.trailing_ones() }
        #[inline] fn trailing_zeros(self) -> u32    { self.trailing_zeros() }

        #[inline] fn from_be(x: Self) -> Self   { Self::from_be(x) }
        #[inline] fn from_le(x: Self) -> Self   { Self::from_le(x) }
        #[inline] fn to_be(self) -> Self        { self.to_be() }
        #[inline] fn to_le(self) -> Self        { self.to_le() }
        #[inline] fn swap_bytes(self) -> Self   { self.swap_bytes() }

        #[inline] fn is_power_of_two(self) -> bool      { self.is_power_of_two() }
        #[inline] fn next_power_of_two(self) -> Self    { self.next_power_of_two() }

        #[inline] fn into_f64(self) -> f64      { unsafe { self.this as f64 } }
        #[inline] fn into_f32(self) -> f32      { unsafe { self.this as f32 } }
        #[inline] fn into_u128(self) -> u128    { unsafe { self.this as u128 } }
        #[inline] fn into_u64(self) -> u64      { unsafe { self.this as u64 } }
        #[inline] fn into_u32(self) -> u32      { unsafe { self.this as u32 } }
        #[inline] fn into_u16(self) -> u16      { unsafe { self.this as u16 } }
        #[inline] fn into_u8(self) -> u8        { unsafe { self.this as u8 } }
        #[inline] fn into_usize(self) -> usize  { unsafe { self.this as usize } }
        #[inline] fn into_bool(self) -> bool    { unsafe { self.this != 0 } }
        #[inline] fn zero() -> Self             { Self::new_with(0) }
        #[inline] fn one() -> Self              { Self::new_with(1) }
        #[inline] fn max() -> Self              { Self::new_with(<$g>::MAX) }
        #[inline] fn min() -> Self              { Self::new_with(<$g>::MIN) }
        #[inline] fn u128_as_SmallUInt(n: u128) -> Self  { Self::new_with(n as $g) }
        #[inline] fn u64_as_SmallUInt(n: u64) -> Self    { Self::new_with(n as $g) }
        #[inline] fn u32_as_SmallUInt(n: u32) -> Self    { Self::new_with(n as $g) }
        #[inline] fn u16_as_SmallUInt(n: u16) -> Self    { Self::new_with(n as $g) }
        #[inline] fn u8_as_SmallUInt(n: u8) -> Self      { Self::new_with(n as $g) }
        #[inline] fn usize_as_SmallUInt(n: usize) -> Self    { Self::new_with(n as $g) }
        #[inline] fn bool_as_SmallUInt(n: bool) -> Self  { Self::new_with(n as $g) }

        #[inline]
        fn num<T: SmallUInt>(n: T) -> Self
        {
            match size_of::<T>()
            {
                1 => { return Self::u8_as_SmallUInt(n.into_u8()); },
                2 => { return Self::u16_as_SmallUInt(n.into_u16()); },
                4 => { return Self::u32_as_SmallUInt(n.into_u32()); },
                8 => { return Self::u64_as_SmallUInt(n.into_u64()); },
                _ => { return Self::u128_as_SmallUInt(n.into_u128()); },
            }
        }

        /// Checks whether `SmallUInt` to be zero, and returns true if it is
        /// zero, and returns false if it is not zero.
        /// [Read more in detail](trait@SmallUInt#tymethod.is_zero)
        #[inline] fn is_zero(&self) -> bool     { self.get() == 0 }

        /// Checks whether `SmallUInt` to be zero, and returns true if it is
        /// zero, and returns false if it is not zero.
        /// [Read more in detail](trait@SmallUInt#tymethod.is_one)
        #[inline] fn is_one(&self) -> bool      { self.get() ==  1 }

        /// Checks whether `SmallUInt` to be zero, and returns true if it is
        /// zero, and returns false if it is not zero.
        /// [Read more in detail](trait@SmallUInt#tymethod.set_submax)
        fn set_submax(&mut self, size_in_bits: usize)
        {
            if size_in_bits >= self.length_in_bits()
                { self.set_max(); }
            else if size_in_bits == 0
                { self.set_zero(); }
            else
            {
                self.set_max();
                self.set(self.get() >> (Self::size_in_bits() - size_in_bits));
            }
        }
        /// Checks whether or not `BigUInt`-type number to be maximum value.
        /// zero, and returns false if it is not zero.
        /// [Read more in detail](trait@SmallUInt#tymethod.set_submax)
        #[inline] fn is_max(&self) -> bool { *self == Self::max() }

        /// Sets the MSB (Most Significant Bit) of `SmallUInt`-type
        /// number with `1`.
        /// [Read more in detail](trait@SmallUInt#tymethod.set_MSB)
        #[inline] fn set_MSB(&mut self)     { self.set(self.get() | !(Self::max().get() >> 1)); }

        /// Sets the LSB (Least Significant Bit) of `SmallUInt`-type
        /// number with `1`.
        /// [Read more in detail](trait@SmallUInt#tymethod.set_LSB)
        #[inline] fn set_LSB(&mut self)     { self.set(self.get() | 1); }
/*
        /// Constucts a new `SmallUInt` which has the value zero and sets only
        /// the bit specified by the argument bit_pos to be 1.
        /// [Read more in detail](trait@SmallUInt#tymethod.generate_check_bits)
        #[inline] fn generate_check_bits(bit_pos: usize) -> Option<Self>    { if bit_pos < Self::size_in_bits { Some(generate_check_bits_(bit_pos)) } else { None }
*/
        /// Constucts a new `SmallUInt` which has the value zero and sets only
        /// the bit specified by the argument bit_pos to be 1.
        /// [Read more in detail](trait@SmallUInt#tymethod.generate_check_bits_)
        #[inline] fn generate_check_bits_(bit_pos: usize) -> Self    { Self::new_with(Self::one().get() << bit_pos) }

        #[inline] fn is_odd(self) -> bool       { (self.get() & 1) != 0 }
        #[inline] fn is_even(self) -> bool      { !self.is_odd() }
        #[inline] fn is_MSB_set(self) -> bool   { (self.get() & !(Self::max().get() >> 1)) != 0 }
        #[inline] fn is_bit_set(self, bit_pos: usize) -> Option<bool>  { if bit_pos < Self::size_in_bits() { Some(self.is_bit_set_(bit_pos)) } else { None } }
        #[inline] fn is_bit_set_(self, bit_pos: usize) -> bool  { self.get() & Self::generate_check_bits_(bit_pos).get() != 0 }

        #[inline] fn size_in_bytes() -> usize   { size_of::<Self>() }
        #[inline] fn size_in_bits() -> usize    { size_of::<Self>() * 8 }
        #[inline] fn length_in_bytes(self) -> usize    { size_of_val(&self) }
        #[inline] fn length_in_bits(self) -> usize     { size_of_val(&self) * 8 }
    }
}

macro_rules! operators_for_integer_unions_impl {
    ($f:ty) => {
        impl Add for $f
        {
            type Output = Self;

            #[inline]
            fn add(self, rhs: Self) -> Self
            {
                self.wrapping_add(rhs)
            }
        }

        impl AddAssign for $f
        {
            #[inline]
            fn add_assign(&mut self, rhs: Self)
            {
                self.wrapping_add_assign(rhs);
            }
        }

        impl Sub for $f
        {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: Self) -> Self
            {
                self.wrapping_sub(rhs)
            }
        }

        impl SubAssign for $f
        {
            #[inline]
            fn sub_assign(&mut self, rhs: Self)
            {
                self.wrapping_sub_assign(rhs);
            }
        }

        impl Mul for $f
        {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: Self) -> Self
            {
                self.wrapping_mul(rhs)
            }
        }

        impl MulAssign for $f
        {
            #[inline]
            fn mul_assign(&mut self, rhs: Self)
            {
                self.wrapping_mul_assign(rhs);
            }
        }

        impl Div for $f
        {
            type Output = Self;

            #[inline]
            fn div(self, rhs: Self) -> Self
            {
                self.wrapping_div(rhs)
            }
        }

        impl DivAssign for $f
        {
            #[inline]
            fn div_assign(&mut self, rhs: Self)
            {
                self.wrapping_div_assign(rhs);
            }
        }

        impl Rem for $f
        {
            type Output = Self;

            #[inline]
            fn rem(self, rhs: Self) -> Self
            {
                self.wrapping_rem(rhs)
            }
        }

        impl RemAssign for $f
        {
            #[inline]
            fn rem_assign(&mut self, rhs: Self)
            {
                self.wrapping_rem_assign(rhs);
            }
        }

        impl BitAnd for $f
        {
            type Output = Self;

            #[inline]
            fn bitand(self, rhs: Self) -> Self
            {
                let mut s = self.clone();
                s &= rhs;
                s
            }
        }

        impl BitAndAssign for $f
        {
            #[inline]
            fn bitand_assign(&mut self, rhs: Self)
            {
                unsafe { self.this &= rhs.this; }
            }
        }

        impl BitOr for $f
        {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self
            {
                let mut s = self.clone();
                s |= rhs;
                s
            }
        }

        impl BitOrAssign for $f
        {
            #[inline]
            fn bitor_assign(&mut self, rhs: Self)
            {
                unsafe { self.this |= rhs.this; }
            }
        }

        impl BitXor for $f
        {
            type Output = Self;

            fn bitxor(self, rhs: Self) -> Self
            {
                let mut s = self.clone();
                s ^= rhs;
                s
            }
        }

        impl BitXorAssign for $f
        {
            #[inline]
            fn bitxor_assign(&mut self, rhs: Self)
            {
                unsafe { self.this ^= rhs.this; }
            }
        }

        impl Not for $f
        {
            type Output = Self;

            #[inline]
            fn not(self) -> Self
            {
                Self::new_with(! unsafe { self.this })
            }
        }

        impl PartialEq for $f
        {
            #[inline]
            fn eq(&self, other: &Self) -> bool
            {
                unsafe { self.this == other.this }
            }
        }

        impl PartialOrd for $f
        {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering>
            {
                if unsafe { self.this > other.this }
                    { return Some(Ordering::Greater); }
                else if unsafe { self.this < other.this }
                    { return Some(Ordering::Less); }
                else
                    { Some(Ordering::Equal) }
            }
        }
    }
}



macro_rules! shift_ops_for_integer_unions_impl {
    ($u:ty, $f:ty) => {
        impl Shl<$f> for $u
        {
            type Output = Self;

            fn shl(self, rhs: $f) -> Self
            {
                let mut s = self.clone();
                s <<= rhs;
                s
            }
        }

        impl ShlAssign<$f> for $u
        {
            #[inline]
            fn shl_assign(&mut self, rhs: $f)
            {
                unsafe { self.this <<= rhs }
            }
        }

        impl Shr<$f> for $u
        {
            type Output = Self;

            fn shr(self, rhs: $f) -> Self
            {
                let mut s = self.clone();
                s >>= rhs;
                s
            }
        }

        impl ShrAssign<$f> for $u
        {
            #[inline]
            fn shr_assign(&mut self, rhs: $f)
            {
                unsafe { self.this >>= rhs }
            }
        }
    }
}



macro_rules! display_for_integer_unions_impl {
    ($f:ty) => {
        impl Display for $f
        {
            /// Formats the value using the given formatter.
            /// Automagically the function `to_string()` will be implemented. So, you
            /// can use the function `to_string()` and the macro `println!()`.
            /// `f` is a buffer, this method must write the formatted string into it.
            /// [Read more](https://doc.rust-lang.org/core/fmt/trait.Display.html#tymethod.fmt)
            /// 
            /// # Example
            /// ```
            /// use Cryptocol::number::*;
            /// let a = ShortUnion::new_with(60521_u16);
            /// println!("{}", a);
            /// ```
            fn fmt(&self, f: &mut Formatter) -> fmt::Result
            {
                // `write!` is like `format!`, but it will write the formatted string
                // into a buffer (the first argument)
                write!(f, "{}", self.get())
            }
        }
    }
}



impl SmallUInt for ShortUnion
{
    SmallUInt_methods_for_integer_unions_impl! { ShortUnion, u16 }
    // random_for_unions_impl! { ShortUnion }
}

impl SmallUInt for IntUnion
{
    SmallUInt_methods_for_integer_unions_impl! { IntUnion, u32 }
    // random_for_unions_impl! { IntUnion }
}

impl SmallUInt for LongUnion
{
    SmallUInt_methods_for_integer_unions_impl! { LongUnion, u64 }
    // random_for_unions_impl! { LongUnion }
}

impl SmallUInt for LongerUnion
{
    SmallUInt_methods_for_integer_unions_impl! { LongerUnion, u128 }
    // random_for_unions_impl! { LongerUnion }
}

impl SmallUInt for SizeUnion
{
    SmallUInt_methods_for_integer_unions_impl! { SizeUnion, usize }
    // random_for_unions_impl! { SizeUnion }
}


operators_for_integer_unions_impl! { ShortUnion }
operators_for_integer_unions_impl! { IntUnion }
operators_for_integer_unions_impl! { LongUnion }
operators_for_integer_unions_impl! { LongerUnion }
operators_for_integer_unions_impl! { SizeUnion }

shift_ops_for_integer_unions_impl! { ShortUnion, i8 }
shift_ops_for_integer_unions_impl! { ShortUnion, i16 }
shift_ops_for_integer_unions_impl! { ShortUnion, i32 }
shift_ops_for_integer_unions_impl! { ShortUnion, i64 }
shift_ops_for_integer_unions_impl! { ShortUnion, i128 }
shift_ops_for_integer_unions_impl! { ShortUnion, isize }

shift_ops_for_integer_unions_impl! { ShortUnion, u8 }
shift_ops_for_integer_unions_impl! { ShortUnion, u16 }
shift_ops_for_integer_unions_impl! { ShortUnion, u32 }
shift_ops_for_integer_unions_impl! { ShortUnion, u64 }
shift_ops_for_integer_unions_impl! { ShortUnion, u128 }
shift_ops_for_integer_unions_impl! { ShortUnion, usize }

shift_ops_for_integer_unions_impl! { IntUnion, i8 }
shift_ops_for_integer_unions_impl! { IntUnion, i16 }
shift_ops_for_integer_unions_impl! { IntUnion, i32 }
shift_ops_for_integer_unions_impl! { IntUnion, i64 }
shift_ops_for_integer_unions_impl! { IntUnion, i128 }
shift_ops_for_integer_unions_impl! { IntUnion, isize }

shift_ops_for_integer_unions_impl! { IntUnion, u8 }
shift_ops_for_integer_unions_impl! { IntUnion, u16 }
shift_ops_for_integer_unions_impl! { IntUnion, u32 }
shift_ops_for_integer_unions_impl! { IntUnion, u64 }
shift_ops_for_integer_unions_impl! { IntUnion, u128 }
shift_ops_for_integer_unions_impl! { IntUnion, usize }

shift_ops_for_integer_unions_impl! { LongUnion, i8 }
shift_ops_for_integer_unions_impl! { LongUnion, i16 }
shift_ops_for_integer_unions_impl! { LongUnion, i32 }
shift_ops_for_integer_unions_impl! { LongUnion, i64 }
shift_ops_for_integer_unions_impl! { LongUnion, i128 }
shift_ops_for_integer_unions_impl! { LongUnion, isize }

shift_ops_for_integer_unions_impl! { LongUnion, u8 }
shift_ops_for_integer_unions_impl! { LongUnion, u16 }
shift_ops_for_integer_unions_impl! { LongUnion, u32 }
shift_ops_for_integer_unions_impl! { LongUnion, u64 }
shift_ops_for_integer_unions_impl! { LongUnion, u128 }
shift_ops_for_integer_unions_impl! { LongUnion, usize }

shift_ops_for_integer_unions_impl! { LongerUnion, i8 }
shift_ops_for_integer_unions_impl! { LongerUnion, i16 }
shift_ops_for_integer_unions_impl! { LongerUnion, i32 }
shift_ops_for_integer_unions_impl! { LongerUnion, i64 }
shift_ops_for_integer_unions_impl! { LongerUnion, i128 }
shift_ops_for_integer_unions_impl! { LongerUnion, isize }

shift_ops_for_integer_unions_impl! { LongerUnion, u8 }
shift_ops_for_integer_unions_impl! { LongerUnion, u16 }
shift_ops_for_integer_unions_impl! { LongerUnion, u32 }
shift_ops_for_integer_unions_impl! { LongerUnion, u64 }
shift_ops_for_integer_unions_impl! { LongerUnion, u128 }
shift_ops_for_integer_unions_impl! { LongerUnion, usize }

display_for_integer_unions_impl! { ShortUnion }
display_for_integer_unions_impl! { IntUnion }
display_for_integer_unions_impl! { LongUnion }
display_for_integer_unions_impl! { LongerUnion }
display_for_integer_unions_impl! { SizeUnion }


impl Debug for ShortUnion
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
    /// use Cryptocol::number::*;
    /// let a_short = ShortUnion::new_with_signed(-12345_i16);
    /// println!("a_short = {:?}", a_short);
    /// #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_short:?}"), "ShortUnion { this: 53191, that: -12345, ushort: 53191, sshort: -12345, ubyte: [199, 207], sbyte: [-57, -49] }");
    /// ```
    /// 
    /// # Example for the format specifier :#?
    /// ```
    /// use Cryptocol::number::*;
    /// let a_short = ShortUnion::new_with_signed(-12345_i16);
    /// println!("a_short = {:#?}", a_short);
    /// #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_short:#?}"), r#"ShortUnion {
    ///     this: 53191,
    ///     that: -12345,
    ///     ushort: 53191,
    ///     sshort: -12345,
    ///     ubyte: [
    ///         199,
    ///         207,
    ///     ],
    ///     sbyte: [
    ///         -57,
    ///         -49,
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
        let mut ff = f.debug_struct("ShortUnion");
        ff.field("this", unsafe { &self.this } )
            .field("that", unsafe { &self.that } )
            .field("ushort", unsafe { &self.ushort } )
            .field("sshort", unsafe { &self.sshort } )
            .field("ubyte", unsafe { &self.ubyte } )
            .field("sbyte", unsafe { &self.sbyte } );
         #[cfg(target_pointer_width = "16")] ff.field("u_size", unsafe { &self.u_size } )
                                                .field("s_size", unsafe { &self.s_size } );
         #[cfg(target_pointer_width = "8")] ff.field("u_size", unsafe { &self.u_size } )
                                                .field("s_size", unsafe { &self.s_size } );
         ff.finish()
    }
}


impl Debug for IntUnion
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
    /// use Cryptocol::number::*;
    /// let a_int = IntUnion::new_with_signed(-1234567890_i32);
    /// println!("a_int = {:?}", a_int);
    /// #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_int:?}"), "IntUnion { this: 3060399406, that: -1234567890, uint: 3060399406, sint: -1234567890, ushort: [64814, 46697], sshort: [-722, -18839], ubyte: [46, 253, 105, 182], sbyte: [46, -3, 105, -74] }");
    /// ```
    /// 
    /// # Example for the format specifier :#?
    /// ```
    /// use Cryptocol::number::*;
    /// let a_int = IntUnion::new_with_signed(-1234567890_i32);
    /// println!("a_int = {:#?}", a_int);
    /// #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_int:#?}"), r#"IntUnion {
    ///     this: 3060399406,
    ///     that: -1234567890,
    ///     uint: 3060399406,
    ///     sint: -1234567890,
    ///     ushort: [
    ///         64814,
    ///         46697,
    ///     ],
    ///     sshort: [
    ///         -722,
    ///         -18839,
    ///     ],
    ///     ubyte: [
    ///         46,
    ///         253,
    ///         105,
    ///         182,
    ///     ],
    ///     sbyte: [
    ///         46,
    ///         -3,
    ///         105,
    ///         -74,
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
        let mut ff = f.debug_struct("IntUnion");
        ff.field("this", unsafe { &self.this } )
            .field("that", unsafe { &self.that } )
            .field("uint", unsafe { &self.uint } )
            .field("sint", unsafe { &self.sint } )
            .field("ushort", unsafe { &self.ushort } )
            .field("sshort", unsafe { &self.sshort } )
            .field("ubyte", unsafe { &self.ubyte } )
            .field("sbyte", unsafe { &self.sbyte } );
        #[cfg(target_pointer_width = "32")] ff.field("u_size", unsafe { &self.u_size } )
                                                .field("s_size", unsafe { &self.s_size } );
        #[cfg(target_pointer_width = "16")] ff.field("u_size", unsafe { &self.u_size } )
                                                .field("s_size", unsafe { &self.s_size } );
        #[cfg(target_pointer_width = "8")] ff.field("u_size", unsafe { &self.u_size } )
                                                .field("s_size", unsafe { &self.s_size } );
        ff.finish()
    }
}

impl Debug for LongUnion
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
    /// use Cryptocol::number::*;
    /// let a_long = LongUnion::new_with_signed(-1234567890123456789_i64);
    /// println!("a_long = {:?}", a_long);
    /// #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_long:?}"), "LongUnion { this: 17212176183586094827, that: -1234567890123456789, ulong: 17212176183586094827, slong: -1234567890123456789, uint: [2182512363, 4007522059], sint: [-2112454933, -287445237], ushort: [32491, 33302, 61195, 61149], sshort: [32491, -32234, -4341, -4387], ubyte: [235, 126, 22, 130, 11, 239, 221, 238], sbyte: [-21, 126, 22, -126, 11, -17, -35, -18], u_size: 17212176183586094827, s_size: -1234567890123456789 }");
    /// ```
    /// 
    /// # Example for the format specifier :#?
    /// ```
    /// use Cryptocol::number::*;
    /// let a_long = LongUnion::new_with_signed(-1234567890123456789_i64);
    /// println!("a_long = {:#?}", a_long);
    /// #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_long:#?}"), r#"LongUnion {
    ///     this: 17212176183586094827,
    ///     that: -1234567890123456789,
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
    ///     u_size: 17212176183586094827,
    ///     s_size: -1234567890123456789,
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
        let mut ff = f.debug_struct("LongUnion");
        ff.field("this", unsafe { &self.this } )
            .field("that", unsafe { &self.that } )
            .field("ulong", unsafe { &self.ulong } )
            .field("slong", unsafe { &self.slong } )
            .field("uint", unsafe { &self.uint } )
            .field("sint", unsafe { &self.sint } )
            .field("ushort", unsafe { &self.ushort } )
            .field("sshort", unsafe { &self.sshort } )
            .field("ubyte", unsafe { &self.ubyte } )
            .field("sbyte", unsafe { &self.sbyte } );
        #[cfg(target_pointer_width = "64")] ff.field("u_size", unsafe { &self.u_size } )
                                                .field("s_size", unsafe { &self.s_size } );
        #[cfg(target_pointer_width = "32")] ff.field("u_size", unsafe { &self.u_size } )
                                                .field("s_size", unsafe { &self.s_size } );
        #[cfg(target_pointer_width = "16")] ff.field("u_size", unsafe { &self.u_size } )
                                                .field("s_size", unsafe { &self.s_size } );
        #[cfg(target_pointer_width = "8")] ff.field("u_size", unsafe { &self.u_size } )
                                                .field("s_size", unsafe { &self.s_size } );
        ff.finish()
    }
}

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
    /// use Cryptocol::number::*;
    /// let a_longer = LongerUnion::new_with_signed(-123456789012345678901234567890123456789_i128);
    /// println!("a_longer = {:?}", a_longer);
    /// #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_longer:?}"), "LongerUnion { this: 216825577908592784562140039541644754667, that: -123456789012345678901234567890123456789, ulonger: 216825577908592784562140039541644754667, slonger: -123456789012345678901234567890123456789, ulong: [6134004772338302699, 11754138130946064698], slong: [6134004772338302699, -6692605942763486918], uint: [1371963115, 1428184279, 2682913082, 2736723546], sint: [1371963115, 1428184279, -1612054214, -1558243750], ushort: [32491, 20934, 23767, 21792, 314, 40938, 5722, 41759], sshort: [32491, 20934, 23767, 21792, 314, -24598, 5722, -23777], ubyte: [235, 126, 198, 81, 215, 92, 32, 85, 58, 1, 234, 159, 90, 22, 31, 163], sbyte: [-21, 126, -58, 81, -41, 92, 32, 85, 58, 1, -22, -97, 90, 22, 31, -93], u_size: [6134004772338302699, 11754138130946064698], s_size: [6134004772338302699, -6692605942763486918] }");
    /// ```
    /// 
    /// # Example for the format specifier :#?
    /// ```
    /// use Cryptocol::number::*;
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
        ff.field("this", unsafe { &self.this } )
            .field("that", unsafe { &self.that } )
            .field("ulonger", unsafe { &self.ulonger } )
            .field("slonger", unsafe { &self.slonger } )
            .field("ulong", unsafe { &self.ulong } )
            .field("slong", unsafe { &self.slong } )
            .field("uint", unsafe { &self.uint } )
            .field("sint", unsafe { &self.sint } )
            .field("ushort", unsafe { &self.ushort } )
            .field("sshort", unsafe { &self.sshort } )
            .field("ubyte", unsafe { &self.ubyte } )
            .field("sbyte", unsafe { &self.sbyte } );
        #[cfg(target_pointer_width = "128")] ff.field("u_size", unsafe { &self.u_size } )
                                                .field("s_size", unsafe { &self.s_size } );
        #[cfg(target_pointer_width = "64")] ff.field("u_size", unsafe { &self.u_size } )
                                                .field("s_size", unsafe { &self.s_size } );
        #[cfg(target_pointer_width = "32")] ff.field("u_size", unsafe { &self.u_size } )
                                                .field("s_size", unsafe { &self.s_size } );
        #[cfg(target_pointer_width = "16")] ff.field("u_size", unsafe { &self.u_size } )
                                                .field("s_size", unsafe { &self.s_size } );
        #[cfg(target_pointer_width = "8")] ff.field("u_size", unsafe { &self.u_size } )
                                                .field("s_size", unsafe { &self.s_size } );
        ff.finish()
    }
}


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
    /// use Cryptocol::number::*;
    /// let a_size = SizeUnion::new_with_signed(-1234567890123456789_isize);
    /// println!("a_size = {:?}", a_size);
    /// #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_size:?}"), "SizeUnion { this: 17212176183586094827, that: -1234567890123456789, u_size: 17212176183586094827, s_size: -1234567890123456789, ulong: 17212176183586094827, slong: -1234567890123456789, uint: [2182512363, 4007522059], sint: [-2112454933, -287445237], ushort: [32491, 33302, 61195, 61149], sshort: [32491, -32234, -4341, -4387], ubyte: [235, 126, 22, 130, 11, 239, 221, 238], sbyte: [-21, 126, 22, -126, 11, -17, -35, -18] }");
    /// ```
    /// 
    /// # Example for the format specifier :#?
    /// ```
    /// use Cryptocol::number::*;
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
        ff.field("this", unsafe { &self.this } )
            .field("that", unsafe { &self.that } )
            .field("u_size", unsafe { &self.u_size } )
            .field("s_size", unsafe { &self.s_size } );

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
        unsafe { self.src >>= S::usize_as_SmallUInt(bit_pos); }
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
        unsafe { self.src <<= S::usize_as_SmallUInt(bit_pos); }
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
