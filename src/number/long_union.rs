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
#![allow(rustdoc::missing_doc_code_examples)]

use std::fmt::{ self, Debug, Display, Formatter };
use std::cmp::{ PartialEq, PartialOrd, Ordering };
use std::ops::*;

use crate::number::SmallUInt;

/// # Introduction
/// This union `LongUnion` is for slicing `u64` into two `u32`s, two `i32`s,
/// four `u16`s, four `i16`s, eight `u8`s and/or eight `i8`s.
/// 
/// Sometimes, for example, we need to slice `u62` data into two `u32` pieces
/// which include a higher four-byte word and a lower four-byte word, and/or
/// into  four `u16` pieces which include a first two-byte word, a second
/// two-byte word, a third two-byte word and a fourth two-byte word.
/// In that case, `LongUnion` will be very helpful.
/// 
/// # Quick Start
/// In order to use this union, you have to import (use)
/// `cryptocol::number::LongUnion` as follows.
/// 
/// ## Example 1
/// ```
/// use cryptocol::number::LongUnion;
/// ```
/// You can use the methods `get()`, `get_signed()`, `get_ulong()`, and
/// `get_slong()` in order to obtain the data of `u64` in various types.
/// And, you can also slice the data of `u64` into two `u32` type data by
/// using the methods `get_uint()`, `get_sint()`, `get_uint_()`, and
/// `get_sint_()`. Or, you can also slice the data of `u64` into four `u16`
/// type data by using the methods `get_ushort()`, `get_sshort()`,
/// `get_ushort_()`, and `get_sshort_()`. Or, you can also slice the data
/// of `u64` into eight `u8` type data by using the methods `get_ubyte()`,
/// `get_sbyte()`, `get_ubyte_()`, and `get_sbyte_()`. If your machine is
/// not 8-bit, 16-bit, or 32-bit machine, `LongUnion` does not have the
/// method `get_usize()` nor `get_ssize()`.
/// 
/// ## Example 2
/// ```
/// use cryptocol::number::LongUnion;
/// 
/// let a = LongUnion::new_with_signed(-1234567890987645_i64);
/// println!("a.get() = {}", a.get());
/// println!("a.get_signed() = {}", a.get_signed());
/// println!("a.get_ulong() = {}", a.get_ulong());
/// println!("a.get_slong() = {}", a.get_slong());
/// assert_eq!(a.get(), 18445509505818563971_u64);
/// assert_eq!(a.get_signed(), -1234567890987645_i64);
/// assert_eq!(a.get_ulong(), 18445509505818563971_u64);
/// assert_eq!(a.get_slong(), -1234567890987645_i64);
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
/// assert_eq!(a.get_uint_(0), 3278378371_u32);
/// assert_eq!(a.get_uint_(1), 4294679850_u32);
/// assert_eq!(a.get_sint_(0), -1016588925_i32);
/// assert_eq!(a.get_sint_(1), -287446_i32);
/// assert_eq!(a.get_ushort_(0), 5507_u16);
/// assert_eq!(a.get_ushort_(1), 50024_u16);
/// assert_eq!(a.get_ushort_(2), 40234_u16);
/// assert_eq!(a.get_ushort_(3), 65531_u16);
/// assert_eq!(a.get_sshort_(0), 5507_i16);
/// assert_eq!(a.get_sshort_(1), -15512_i16);
/// assert_eq!(a.get_sshort_(2), -25302_i16);
/// assert_eq!(a.get_sshort_(3), -5_i16);
/// assert_eq!(a.get_ubyte_(0), 131_u8);
/// assert_eq!(a.get_ubyte_(1), 21_u8);
/// assert_eq!(a.get_ubyte_(2), 104_u8);
/// assert_eq!(a.get_ubyte_(3), 195_u8);
/// assert_eq!(a.get_ubyte_(4), 42_u8);
/// assert_eq!(a.get_ubyte_(5), 157_u8);
/// assert_eq!(a.get_ubyte_(6), 251_u8);
/// assert_eq!(a.get_ubyte_(7), 255_u8);
/// assert_eq!(a.get_sbyte_(0), -125_i8);
/// assert_eq!(a.get_sbyte_(1), 21_i8);
/// assert_eq!(a.get_sbyte_(2), 104_i8);
/// assert_eq!(a.get_sbyte_(3), -61_i8);
/// assert_eq!(a.get_sbyte_(4), 42_i8);
/// assert_eq!(a.get_sbyte_(5), -99_i8);
/// assert_eq!(a.get_sbyte_(6), -5_i8);
/// assert_eq!(a.get_sbyte_(7), -1_i8);
/// 
/// #[cfg(target_pointer_width = "64")]
/// {
///     println!("a.get_usize() = {}", a.get_usize());
///     println!("a.get_ssize() = {}", a.get_ssize());
///     assert_eq!(a.get_usize(), 18445509505818563971_usize);
///     assert_eq!(a.get_ssize(), -1234567890987645_isize);
/// }
/// #[cfg(target_pointer_width = "32")]
/// {
///     const N: usize = 2;
///     for i in 0..N
///         { println!("a.get_usize_({}) = {}", i, a.get_usize_(i)); }
///     for i in 0..N
///         { println!("a.get_ssize_({}) = {}", i, a.get_ssize_(i)); }
///     assert_eq!(a.get_usize_(0), 3278378371_usize);
///     assert_eq!(a.get_usize_(1), 4294679850_usize);
///     assert_eq!(a.get_ssize_(0), -1016588925_isize);
///     assert_eq!(a.get_ssize_(1), -287446_isize);
/// }
/// #[cfg(target_pointer_width = "16")]
/// {
///     const N: usize = 4;
///     for i in 0..N
///         { println!("a.get_usize_({}) = {}", i, a.get_usize_(i)); }
///     for i in 0..N
///         { println!("a.get_ssize_({}) = {}", i, a.get_ssize_(i)); }
///         assert_eq!(a.get_usize_(0), 5507_usize);
///         assert_eq!(a.get_usize_(1), 50024_usize);
///         assert_eq!(a.get_usize_(2), 40234_usize);
///         assert_eq!(a.get_usize_(3), 65531_usize);
///         assert_eq!(a.get_ssize_(0), 5507_isize);
///         assert_eq!(a.get_ssize_(1), -15512_isize);
///         assert_eq!(a.get_ssize_(2), -25302_isize);
///         assert_eq!(a.get_ssize_(3), -5_isize);
/// }
/// #[cfg(target_pointer_width = "8")]
/// {
///     const N: usize = 8;
///     for i in 0..N
///         { println!("a.get_usize_({}) = {}", i, a.get_usize_(i)); }
///     for i in 0..N
///         { println!("a.get_ssize_({}) = {}", i, a.get_ssize_(i)); }
///     assert_eq!(a.get_ubyte_(0), 131_usize);
///     assert_eq!(a.get_ubyte_(1), 21_usize);
///     assert_eq!(a.get_ubyte_(2), 104_usize);
///     assert_eq!(a.get_ubyte_(3), 195_usize);
///     assert_eq!(a.get_ubyte_(4), 42_usize);
///     assert_eq!(a.get_ubyte_(5), 157_usize);
///     assert_eq!(a.get_ubyte_(6), 251_usize);
///     assert_eq!(a.get_ubyte_(7), 255_usize);
///     assert_eq!(a.get_sbyte_(0), -125_isize);
///     assert_eq!(a.get_sbyte_(1), 21_isize);
///     assert_eq!(a.get_sbyte_(2), 104_isize);
///     assert_eq!(a.get_sbyte_(3), -61_isize);
///     assert_eq!(a.get_sbyte_(4), 42_isize);
///     assert_eq!(a.get_sbyte_(5), -99_isize);
///     assert_eq!(a.get_sbyte_(6), -5_isize);
///     assert_eq!(a.get_sbyte_(7), -1_isize);
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
/// use cryptocol::number::SmallUInt;
/// 
/// let a_longunion = 12345678987654321_u64.into_longunion();
/// let b_longunion = 87654321012345678_u64.into_longunion();
/// let c_longunion = a_longunion.wrapping_add(b_longunion);
/// println!("{} + {} = {}", a_longunion, b_longunion, c_longunion);
/// assert_eq!(c_longunion.get(), 99999999999999999_u64);
/// for i in 0..2
///     { println!("c_longunion.get_uint_({}) = {}", i, c_longunion.get_uint_(i)); }
/// assert_eq!(c_longunion.get_uint_(0), 1569325055_u32);
/// assert_eq!(c_longunion.get_uint_(1), 23283064_u32);
/// for i in 0..4
///     { println!("c_longunion.get_ushort_({}) = {}", i, c_longunion.get_ushort_(i)); }
/// assert_eq!(c_longunion.get_ushort_(0), 65535_u16);
/// assert_eq!(c_longunion.get_ushort_(1), 23945_u16);
/// assert_eq!(c_longunion.get_ushort_(2), 17784_u16);
/// assert_eq!(c_longunion.get_ushort_(3), 355_u16);
/// for i in 0..8
///     { println!("c_longunion.get_ubyte_({}) = {}", i, c_longunion.get_ubyte_(i)); }
/// assert_eq!(c_longunion.get_ubyte_(0), 255_u8);
/// assert_eq!(c_longunion.get_ubyte_(1), 255_u8);
/// assert_eq!(c_longunion.get_ubyte_(2), 137_u8);
/// assert_eq!(c_longunion.get_ubyte_(3), 93_u8);
/// assert_eq!(c_longunion.get_ubyte_(4), 120_u8);
/// assert_eq!(c_longunion.get_ubyte_(5), 69_u8);
/// assert_eq!(c_longunion.get_ubyte_(6), 99_u8);
/// assert_eq!(c_longunion.get_ubyte_(7), 1_u8);
/// 
/// let d_longunion = b_longunion - a_longunion;
/// println!("{} - {} = {}", b_longunion, a_longunion, d_longunion);
/// assert_eq!(d_longunion.get(), 75308642024691357_u64);
/// for i in 0..2
///     { println!("d_longunion.get_uint_({}) = {}", i, d_longunion.get_uint_(i)); }
/// assert_eq!(d_longunion.get_uint_(0), 2556827293_u32);
/// assert_eq!(d_longunion.get_uint_(1), 17534159_u32);
/// for i in 0..4
///     { println!("d_longunion.get_ushort_({}) = {}", i, d_longunion.get_ushort_(i)); }
/// assert_eq!(d_longunion.get_ushort_(0), 5789_u16);
/// assert_eq!(d_longunion.get_ushort_(1), 39014_u16);
/// assert_eq!(d_longunion.get_ushort_(2), 36047_u16);
/// assert_eq!(d_longunion.get_ushort_(3), 267_u16);
/// for i in 0..8
///     { println!("d_longunion.get_ubyte_({}) = {}", i, d_longunion.get_ubyte_(i)); }
/// assert_eq!(d_longunion.get_ubyte_(0), 157_u8);
/// assert_eq!(d_longunion.get_ubyte_(1), 22_u8);
/// assert_eq!(d_longunion.get_ubyte_(2), 102_u8);
/// assert_eq!(d_longunion.get_ubyte_(3), 152_u8);
/// assert_eq!(d_longunion.get_ubyte_(4), 207_u8);
/// assert_eq!(d_longunion.get_ubyte_(5), 140_u8);
/// assert_eq!(d_longunion.get_ubyte_(6), 11_u8);
/// assert_eq!(d_longunion.get_ubyte_(7), 1_u8);
/// 
/// let e_longunion = d_longunion * 3_u64.into_longunion();
/// println!("{} * {} = {}", d_longunion, 3_u64.into_longunion(), e_longunion);
/// assert_eq!(e_longunion.get(), 225925926074074071_u64);
/// 
/// let f_longunion = c_longunion / 10_u32.into_longunion();
/// println!("{} / {} = {}", c_longunion, 10_u16.into_longunion(), f_longunion);
/// assert_eq!(f_longunion.get(), 9999999999999999_u64);
/// 
/// let g_longunion = c_longunion % 10_u64.into_longunion();
/// println!("{} % {} = {}", c_longunion, 10_u64.into_longunion(), g_longunion);
/// assert_eq!(g_longunion.get(), 9_u64);
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

    // /// The usize type array whose elements's size is 8-bit size
    // #[cfg(target_pointer_width = "8")] u_size: [usize; 8],

    // /// The isize type array whose elements's size is 8-bit size
    // #[cfg(target_pointer_width = "8")] s_size: [isize; 8],
}

impl LongUnion
{
    // pub fn new() -> Self
    /// Constructs a new `LongUnion`.
    /// 
    /// # Output
    /// A new object of `LongUnion`.
    /// 
    /// # Initialization
    /// All the fields of the constructed object will be
    /// initialized with `0`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::number::LongUnion;    
    /// let a = LongUnion::new();
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 0_u64);
    /// ```
    #[inline] pub fn new() -> Self  { Self { ulong: 0 } }

    // pub fn new_with(ulong: u64) -> Self
    /// Constructs a new `LongUnion` with initializing it with `ulong`.
    /// 
    /// # Output
    /// A new object of `LongUnion` initialized with the value `ulong`.
    /// 
    /// # Initialization
    /// The field of the constructed object will be initialized with `ulong`.
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::LongUnion;    
    /// let a = LongUnion::new_with(12345678909876456_u64);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 12345678909876456_u64);
    /// ```
    #[inline] pub fn new_with(ulong: u64) -> Self   { Self { ulong } }

    // pub fn new_with_signed(slong: i64) -> Self
    /// Constructs a new `LongUnion` with initializing it with `slong`.
    /// 
    /// # Output
    /// A new object of `LongUnion` initialized with the value `slong`.
    /// 
    /// # Initialization
    /// The field of the constructed object will be initialized with `slong`.
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::LongUnion;    
    /// let a = LongUnion::new_with_signed(-12345678909876456_i64);
    /// println!("a = {}", a.get_signed());
    /// assert_eq!(a.get_signed(), -12345678909876456_i64);
    /// ```
    #[inline] pub fn new_with_signed(slong: i64) -> Self    { Self { slong } }

    // pub fn new_with_ubytes(ubyte: [u8; 8]) -> Self
    /// Constructs a new `LongUnion` with initializing it with `ubyte`.
    /// 
    /// # Output
    /// A new object of `LongUnion` initialized with the value `ubyte`.
    /// 
    /// # Initialization
    /// The field of the constructed object will be initialized with `ubyte`.
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::LongUnion;
    /// let arr = [131_u8, 21_u8, 104_u8, 195_u8, 42_u8, 157_u8, 251_u8, 255_u8];
    /// let a = LongUnion::new_with_ubytes(arr);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 18445509505818563971_u64);
    /// ```
    #[inline] pub fn new_with_ubytes(ubyte: [u8; 8]) -> Self    { Self { ubyte } }

    // pub fn new_with_ushorts(ushort: [u16; 2]) -> Self
    /// Constructs a new `LongUnion` with initializing it with `ushort`.
    /// 
    /// # Output
    /// A new object of `LongUnion` initialized with the value `ushort`.
    /// 
    /// # Initialization
    /// The field of the constructed object will be initialized with `ushort`.
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::LongUnion;
    /// let a = LongUnion::new_with_ushorts([5507_u16, 50024_u16, 40234_u16, 65531_u16]);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 18445509505818563971_u64);
    /// ```
    #[inline] pub fn new_with_ushorts(ushort: [u16; 4])  -> Self   { Self { ushort } }

    // pub fn new_with_uints(uint: [u32; 2]) -> Self
    /// Constructs a new `LongUnion` with initializing it with `uint`.
    /// 
    /// # Output
    /// A new object of `LongUnion` initialized with the value `uint`.
    /// 
    /// # Initialization
    /// The field of the constructed object will be initialized with `uint`.
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::LongUnion;
    /// let a = LongUnion::new_with_uints([3278378371_u32, 4294679850_u32]);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 18445509505818563971_u64);
    /// ```
    #[inline] pub fn new_with_uints(uint: [u32; 2]) -> Self     { Self { uint } }

    // pub fn new_with_u128(num: u128) -> Self
    /// Constructs a new `LongUnion` with initializing it with the lower
    /// 64-bit part of `num`.
    /// 
    /// # Output
    /// A new object of `LongUnion` initialized with the value of
    /// the lower 64-bit part of `num`.
    /// 
    /// # Initialization
    /// The field of the constructed object will be initialized with
    /// the value of the lower 64-bit part of `num`.
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::LongUnion;
    /// let a = LongUnion::new_with_u128(18445509505818563971_u128);
    /// let b = LongUnion::new_with_u128(123456789012345678901234567890123456789_u128);
    /// println!("a = {}", a.get());
    /// println!("b = {}", b.get());
    /// assert_eq!(a.get(), 18445509505818563971_u64);
    /// assert_eq!(b.get(), 12312739301371248917_u64);
    /// ```
    #[inline] pub fn new_with_u128(num: u128) -> Self   { Self { ulong: num as u64 } }

    // pub fn new_with_bool(b: bool) -> Self
    /// Constructs a new `LongUnion` with initializing it
    /// with the value of `b`.
    /// 
    /// # Output
    /// A new object of `LongUnion` initialized with the value of `b`
    /// 
    /// # Initialization
    /// The field of the constructed object will be initialized with
    /// the value of `b`.
    /// If `b` is `true`, `self` will have the value `1`.
    /// If `b` is `false`, `self` will have the value `0`.
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::LongUnion;
    /// let a = LongUnion::new_with_bool(true);
    /// let b = LongUnion::new_with_bool(false);
    /// println!("a = {}", a.get());
    /// println!("b = {}", b.get());
    /// assert_eq!(a.get(), 1_u64);
    /// assert_eq!(b.get(), 0_u64);
    /// ```
    #[inline] pub fn new_with_bool(b: bool) -> Self     { Self { ulong: b as u64 } }

    // pub fn get(self) -> u64
    /// Returns its value as `u64`.
    /// 
    /// # Output
    /// Its value as `u64`
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::LongUnion;    
    /// let a = LongUnion::new_with(654321987654321_u64);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 654321987654321_u64);
    /// ```
    #[inline] pub fn get(self) -> u64           { unsafe { self.this } }

    // pub fn set(&mut self, val: u64)
    /// Sets its value with `val` of type `u64`
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::LongUnion;    
    /// let mut a = LongUnion::new();
    /// a.set(654321987654321_u64);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 654321987654321_u64);
    /// ```
    #[inline] pub fn set(&mut self, val: u64)   { self.this = val; }

    // pub fn get_signed(self) -> i64
    /// Returns its value as `i64`.
    /// 
    /// # Output
    /// Its value as `i64`
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::LongUnion;    
    /// let a = LongUnion::new_with(12345678909876456789_u64);
    /// println!("a = {}", a.get_signed());
    /// assert_eq!(a.get_signed(), -6101065163833094827_i64);
    /// ```
    #[inline] pub fn get_signed(self) -> i64    { unsafe { self.that } }

    // pub fn set_signed(&mut self, val: i64)
    /// Sets its value with `val` of type `i64`
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::LongUnion;    
    /// let mut a = LongUnion::new();
    /// a.set_signed(-6101065163833094827_i64);
    /// println!("a = {}", a.get_signed());
    /// assert_eq!(a.get_signed(), -6101065163833094827_i64);
    /// ```
    #[inline] pub fn set_signed(&mut self, val: i64)    { self.that = val; }

    crate::number::get_set_long_fit!();

    crate::number::get_set_byte!(8);

    crate::number::get_set_short!(4);

    crate::number::get_set_int!(2);

    #[cfg(target_pointer_width = "64")]     crate::number::get_set_size_fit!();
    #[cfg(target_pointer_width = "32")]     crate::number::get_set_usize!(2);
    #[cfg(target_pointer_width = "16")]     crate::number::get_set_usize!(4);
    // #[cfg(target_pointer_width = "8")]      crate::number::get_set_usize!(8);

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

crate::number::shift_ops_for_integer_unions_by_self_impl! { LongUnion }

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
        // #[cfg(target_pointer_width = "8")] ff.field("u_size", &[self.get_usize_(0), self.get_usize_(1), self.get_usize_(2), self.get_usize_(3), self.get_usize_(4), self.get_usize_(5), self.get_usize_(6), self.get_usize_(7)])
        //                                         .field("s_size", &[self.get_ssize_(0), self.get_ssize_(1), self.get_ssize_(2), self.get_ssize_(3), self.get_ssize_(4), self.get_ssize_(5), self.get_ssize_(6), self.get_ssize_(7)]);
        ff.finish()
    }
}
