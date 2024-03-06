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
