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

    crate::number::get_set_byte!(8);

    crate::number::get_set_short!(4);

    crate::number::get_set_int!(2);

    #[cfg(target_pointer_width = "64")]     crate::number::get_set_size_fit!();
    #[cfg(target_pointer_width = "32")]     crate::number::get_set_usize!(2);
    #[cfg(target_pointer_width = "16")]     crate::number::get_set_usize!(4);
    #[cfg(target_pointer_width = "8")]      crate::number::get_set_usize!(8);

    crate::number::integer_union_methods!(u64);
}



crate::number::operators_for_integer_unions_impl! { LongUnion }

crate::number::shift_ops_for_integer_unions_impl! { LongUnion, i8 }
crate::number::shift_ops_for_integer_unions_impl! { LongUnion, i16 }
crate::number::shift_ops_for_integer_unions_impl! { LongUnion, i32 }
crate::number::shift_ops_for_integer_unions_impl! { LongUnion, i64 }
crate::number::shift_ops_for_integer_unions_impl! { LongUnion, i128 }
crate::number::shift_ops_for_integer_unions_impl! { LongUnion, isize }

crate::number::shift_ops_for_integer_unions_impl! { LongUnion, u8 }
crate::number::shift_ops_for_integer_unions_impl! { LongUnion, u16 }
crate::number::shift_ops_for_integer_unions_impl! { LongUnion, u32 }
crate::number::shift_ops_for_integer_unions_impl! { LongUnion, u64 }
crate::number::shift_ops_for_integer_unions_impl! { LongUnion, u128 }
crate::number::shift_ops_for_integer_unions_impl! { LongUnion, usize }

crate::number::display_for_integer_unions_impl! { LongUnion }




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
    /// use cryptocol::number::*;
    /// let a_long = LongUnion::new_with_signed(-1234567890123456789_i64);
    /// println!("a_long = {:?}", a_long);
    /// #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_long:?}"), "LongUnion { this: 17212176183586094827, that: -1234567890123456789, ulong: 17212176183586094827, slong: -1234567890123456789, uint: [2182512363, 4007522059], sint: [-2112454933, -287445237], ushort: [32491, 33302, 61195, 61149], sshort: [32491, -32234, -4341, -4387], ubyte: [235, 126, 22, 130, 11, 239, 221, 238], sbyte: [-21, 126, 22, -126, 11, -17, -35, -18], u_size: 17212176183586094827, s_size: -1234567890123456789 }");
    /// ```
    /// 
    /// # Example for the format specifier :#?
    /// ```
    /// use cryptocol::number::*;
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
        ff.field("this", &self.get())
            .field("that", &self.get_signed())
            .field("ulong", &self.get_ulong())
            .field("slong", &self.get_slong())
            .field("uint", &[self.get_uint_(0), self.get_uint_(1)])
            .field("sint", &[self.get_sint_(0), self.get_sint_(1)])
            .field("ushort", &[self.get_ushort_(0), self.get_ushort_(1), self.get_ushort_(2), self.get_ushort_(3)])
            .field("sshort", &[self.get_sshort_(0), self.get_sshort_(1), self.get_sshort_(2), self.get_sshort_(3)])
            .field("ubyte", &[self.get_ubyte_(0), self.get_ubyte_(1), self.get_ubyte_(2), self.get_ubyte_(3), self.get_ubyte_(4), self.get_ubyte_(5), self.get_ubyte_(6), self.get_ubyte_(7)])
            .field("sbyte", &[self.get_sbyte_(0), self.get_sbyte_(1), self.get_sbyte_(2), self.get_sbyte_(3), self.get_sbyte_(4), self.get_sbyte_(5), self.get_sbyte_(6), self.get_sbyte_(7)]);
        #[cfg(target_pointer_width = "64")] ff.field("u_size", &self.get_usize())
                                                .field("s_size", &self.get_ssize());
        #[cfg(target_pointer_width = "32")] ff.field("u_size", &[self.get_usize_(0), self.get_usize_(1)])
                                                .field("s_size", &[self.get_ssize_(0), self.get_ssize_(1)]);
        #[cfg(target_pointer_width = "16")] ff.field("u_size", &[self.get_usize_(0), self.get_usize_(1), self.get_usize_(2), self.get_usize_(3)])
                                                .field("s_size", &[self.get_ssize_(0), self.get_ssize_(1), self.get_ssize_(2), self.get_ssize_(3)]);
        #[cfg(target_pointer_width = "8")] ff.field("u_size", &[self.get_usize_(0), self.get_usize_(1), self.get_usize_(2), self.get_usize_(3), self.get_usize_(4), self.get_usize_(5), self.get_usize_(6), self.get_usize_(7)])
                                                .field("s_size", &[self.get_ssize_(0), self.get_ssize_(1), self.get_ssize_(2), self.get_ssize_(3), self.get_ssize_(4), self.get_ssize_(5), self.get_ssize_(6), self.get_ssize_(7)]);
        ff.finish()
    }
}