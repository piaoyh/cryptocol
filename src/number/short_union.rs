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
/// This union `ShortUnion` is for slicing `u16` into two `u8`s,
/// and/or two `i8`.
/// 
/// Sometimes, for example, we need to slice `u16` data into two `u8`
/// pieces which include a higher byte and a lower byte.
/// In such case, `ShortUnion` will be very helpful.
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
/// `get_sbyte_()`. If your machine is not either 8-bit nor 16-bit machine,
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
/// use cryptocol::number::SmallUInt;
/// 
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
    #[cfg(target_pointer_width = "16")] u_size: usize,

    /// The isize type element whose size is the same as the ShortUnion
    #[cfg(target_pointer_width = "16")] s_size: isize,

    /// The usize type array whose elements's size is 8-bit size
    #[cfg(target_pointer_width = "8")] u_size: [usize; 2],

    /// The isize type array whose elements's size is 8-bit size
    #[cfg(target_pointer_width = "8")] s_size: [isize; 2],
}


impl ShortUnion
{
    // pub fn new() -> Self
    /// Constructs a new `ShortUnion`.
    /// 
    /// # Output
    /// A new object of `ShortUnion`.
    /// 
    /// # Initialization
    /// All the fields of the constructed object will be
    /// initialized with `0`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::number::ShortUnion;    
    /// let a = ShortUnion::new();
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 0_u16);
    /// ```
    #[inline] pub fn new() -> Self  { Self { ushort: 0 } }

    // pub fn new_with(ushort: u16) -> Self
    /// Constructs a new `ShortUnion` with initializing it with `ushort`.
    /// 
    /// # Output
    /// A new object of `ShortUnion` initialized with the value `ushort`.
    /// 
    /// # Initialization
    /// The field of the constructed object will be initialized with `ushort`.
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::ShortUnion;    
    /// let a = ShortUnion::new_with(1234_u16);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 1234_u16);
    /// ```
    #[inline] pub fn new_with(ushort: u16) -> Self    { Self { ushort } }

    // pub fn new_with_signed(sshort: i16) -> Self
    /// Constructs a new `ShortUnion` with initializing it with `sshort`.
    /// 
    /// # Output
    /// A new object of `ShortUnion` initialized with the value `sshort`.
    /// 
    /// # Initialization
    /// The field of the constructed object will be initialized with `sshort`.
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::ShortUnion;    
    /// let a = ShortUnion::new_with_signed(-1234_i16);
    /// println!("a = {}", a.get_signed());
    /// assert_eq!(a.get_signed(), -1234_i16);
    /// ```
    #[inline] pub fn new_with_signed(sshort: i16) -> Self   { Self { sshort } }

    // pub fn new_with_ubytes(ubyte: [u8; 2]) -> Self
    /// Constructs a new `ShortUnion` with initializing it with `ubyte`.
    /// 
    /// # Output
    /// A new object of `ShortUnion` initialized with the value `ubyte`.
    /// 
    /// # Initialization
    /// The field of the constructed object will be initialized with `ubyte`.
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::ShortUnion;
    /// let a = ShortUnion::new_with_ubytes([172_u8, 216_u8]);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 55468_u16);
    /// ```
    #[inline] pub fn new_with_ubytes(ubyte: [u8; 2]) -> Self    { Self { ubyte } }

    // pub fn new_with_u128(num: u128) -> Self
    /// Constructs a new `ShortUnion` with initializing it with the lowest
    /// 16-bit part of `num`.
    /// 
    /// # Output
    /// A new object of `ShortUnion` initialized with the value of
    /// the lowest 16-bit part of `num`.
    /// 
    /// # Initialization
    /// The field of the constructed object will be initialized with
    /// the value of the lowest 16-bit part of `num`.
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::ShortUnion;
    /// let a = ShortUnion::new_with_u128(55468_u128);
    /// let b = ShortUnion::new_with_u128(123456789012345678901234567890123456789_u128);
    /// println!("a = {}", a.get());
    /// println!("b = {}", b.get());
    /// assert_eq!(a.get(), 55468_u16);
    /// assert_eq!(b.get(), 33045_u16);
    /// ```
    #[inline] pub fn new_with_u128(num: u128) -> Self   { Self { ushort: num as u16 } }

    // pub fn new_with_bool(b: bool) -> Self
    /// Constructs a new `ShortUnion` with initializing it
    /// with the value of `b`.
    /// 
    /// # Output
    /// A new object of `ShortUnion` initialized with the value of `b`
    /// 
    /// # Initialization
    /// The field of the constructed object will be initialized with
    /// the value of `b`.
    /// If `b` is `true`, `self` will have the value `1`.
    /// If `b` is `false`, `self` will have the value `0`.
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::ShortUnion;
    /// let a = ShortUnion::new_with_bool(true);
    /// let b = ShortUnion::new_with_bool(false);
    /// println!("a = {}", a.get());
    /// println!("b = {}", b.get());
    /// assert_eq!(a.get(), 1_u16);
    /// assert_eq!(b.get(), 0_u16);
    /// ```
    #[inline] pub fn new_with_bool(b: bool) -> Self     { Self { ushort: b as u16 } }

    #[inline] pub fn zero() -> Self     { Self { ushort: 0} }

    #[inline] pub fn one() -> Self      { Self { ushort: 1} }

    // pub fn get(self) -> u16
    /// Returns its value as `u16`.
    /// 
    /// # Output
    /// Its value as `u16`
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::ShortUnion;
    /// let a = ShortUnion::new_with(55468_u16);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 55468_u16);
    /// ```
    #[inline] pub fn get(self) -> u16   { unsafe { self.this } }

    // pub fn set(&mut self, val: u16)
    /// Sets its value with `val` of type `u16`.
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::ShortUnion;    
    /// let mut a = ShortUnion::new();
    /// a.set(54321_u16);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 54321_u16);
    /// ```
    #[inline] pub fn set(&mut self, val: u16)   { self.this = val; }

    // pub fn get_signed(self) -> i16
    /// Returns its value as `i16`.
    /// 
    /// # Output
    /// Its value as `i16`
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::ShortUnion;    
    /// let a = ShortUnion::new_with(54321_u16);
    /// println!("a = {}", a.get_signed());
    /// assert_eq!(a.get_signed(), -11215_i16);
    /// ```
    #[inline] pub fn get_signed(self) -> i16    { unsafe { self.that } }

    // pub fn set_signed(&mut self, val: i16)
    /// Sets its value with `val` of type `i16`.
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::ShortUnion;    
    /// let mut a = ShortUnion::new();
    /// a.set_signed(-11215_i16);
    /// println!("a = {}", a.get_signed());
    /// assert_eq!(a.get_signed(), -11215_i16);
    /// ```
    #[inline] pub fn set_signed(&mut self, val: i16)    { self.that = val; }

    crate::number::get_set_short_fit!();

    crate::number::get_set_byte!(2);

    #[cfg(target_pointer_width = "8")]  crate::number::get_set_size!(2);
    #[cfg(target_pointer_width = "16")] crate::number::get_set_size_fit!();

    crate::number::integer_union_methods!(u16);
}



crate::number::operators_for_integer_unions_impl! { ShortUnion }

crate::number::shift_ops_for_integer_unions_impl! { ShortUnion, i8 }
crate::number::shift_ops_for_integer_unions_impl! { ShortUnion, i16 }
crate::number::shift_ops_for_integer_unions_impl! { ShortUnion, i32 }
crate::number::shift_ops_for_integer_unions_impl! { ShortUnion, i64 }
crate::number::shift_ops_for_integer_unions_impl! { ShortUnion, i128 }
crate::number::shift_ops_for_integer_unions_impl! { ShortUnion, isize }

crate::number::shift_ops_for_integer_unions_impl! { ShortUnion, u8 }
crate::number::shift_ops_for_integer_unions_impl! { ShortUnion, u16 }
crate::number::shift_ops_for_integer_unions_impl! { ShortUnion, u32 }
crate::number::shift_ops_for_integer_unions_impl! { ShortUnion, u64 }
crate::number::shift_ops_for_integer_unions_impl! { ShortUnion, u128 }
crate::number::shift_ops_for_integer_unions_impl! { ShortUnion, usize }

crate::number::shift_ops_for_integer_unions_by_self_impl! { ShortUnion }

crate::number::display_for_integer_unions_impl! { ShortUnion }



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
    /// use cryptocol::number::*;
    /// let a_short = ShortUnion::new_with_signed(-12345_i16);
    /// println!("a_short = {:?}", a_short);
    /// #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_short:?}"), "ShortUnion { this: 53191, that: -12345, ushort: 53191, sshort: -12345, ubyte: [199, 207], sbyte: [-57, -49] }");
    /// ```
    /// 
    /// # Example for the format specifier :#?
    /// ```
    /// use cryptocol::number::*;
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
        ff.field("this", &self.get())
            .field("that", &self.get_signed())
            .field("ushort", &self.get_ushort())
            .field("sshort", &self.get_sshort())
            .field("ubyte",  &[self.get_ubyte_(0), self.get_ubyte_(1)])
            .field("sbyte",  &[self.get_sbyte_(0), self.get_sbyte_(1)]);
         #[cfg(target_pointer_width = "16")] ff.field("u_size", unsafe { &self.get_usize() } )
                                                .field("s_size", unsafe { &self.get_ssize() } );
         #[cfg(target_pointer_width = "8")] ff.field("u_size", unsafe { &[self.get_usize(0), self.get_usize(1)] } )
                                                .field("s_size", unsafe { &[self.get_ssize(0), self.get_isize(1)] } );
         ff.finish()
    }
}
