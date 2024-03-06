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

    #[cfg(target_pointer_width = "128")]    crate::number::get_set_short!(8);
    #[cfg(target_pointer_width = "64")]     crate::number::get_set_short!(4);
    #[cfg(target_pointer_width = "32")]     crate::number::get_set_short!(2);

    #[cfg(target_pointer_width = "128")]    crate::number::get_set_int!(4);
    #[cfg(target_pointer_width = "64")]     crate::number::get_set_int!(2);

    #[cfg(target_pointer_width = "128")]    crate::number::get_set_long!(2);

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
