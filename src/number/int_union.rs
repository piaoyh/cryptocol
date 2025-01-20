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
// #![allow(missing_docs)]
// #![allow(rustdoc::missing_doc_code_examples)]

use std::cmp::{ PartialEq, PartialOrd, Ordering };
use std::ops::{ BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
                Shl, ShlAssign, Shr, ShrAssign, 
                Add, AddAssign, Sub, SubAssign, Mul, MulAssign,
                Div, DivAssign, Rem, RemAssign };
use std::fmt::{ self, Alignment, Error, Formatter, Display, Debug, Pointer,
                Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp };

use crate::number::{ SmallUInt, ShortUnion, LongUnion, LongerUnion, SizeUnion };
use crate::number::{ calc_assign_to_calc, fmt_with_radix, fmt_with_exponent };

/// # Introduction
/// This union `IntUnion` is for slicing `u32` into two `u16`s, two `i16`,
/// four `u8`s, and/or four `i8`.
/// 
/// Sometimes, for example, we need to slice `u32` data into two `u16` pieces
/// which include a higher two-byte word and a lower two-byte word, and/or
/// into four `u8` pieces which include a first byte, a second byte, a third
/// byte and a fourth. In that case, `IntUnion` will be very helpful.
/// 
/// # Quick Start
/// In order to use this union, you have to import (use)
/// `cryptocol::number::IntUnion` as follows.
/// 
/// ## Example 1
/// ```
/// use cryptocol::number::IntUnion;
/// ```
/// You can use the methods `get()`, `get_signed()`, `get_uint()`, and
/// `get_sint()` in order to obtain the data of `u32` in various types.
/// And, you can also slice the data of `u32` into two `u16` type data by
/// using the methods `get_ushort()`, `get_sshort()`, `get_ushort_()`, and
/// `get_sshort_()`, Or, you can also slice the data of `u32` into four 
/// `u8` type data by using the methods `get_ubyte()`, `get_sbyte()`,
/// `get_ubyte_()`, and `get_sbyte_()`. If your machine is neither 8-bit,
/// 16-bit, nor 32-bit machine, `IntUnion` does not have the method
/// `get_usize()` nor `get_ssize()`.
/// 
/// ## Example 2
/// ```
/// use cryptocol::number::IntUnion;
/// 
/// let a = IntUnion::new_with_signed(-454688546_i32);
/// println!("a.get() = {}", a.get());
/// println!("a.get_signed() = {}", a.get_signed());
/// println!("a.get_uint() = {}", a.get_uint());
/// println!("a.get_sint() = {}", a.get_sint());
/// assert_eq!(a.get(), 3840278750_u32);
/// assert_eq!(a.get_signed(), -454688546_i32);
/// assert_eq!(a.get_uint(), 3840278750_u32);
/// assert_eq!(a.get_sint(), -454688546_i32);
/// 
/// for i in 0..2
///     { println!("a.get_ushort_({}) = {}", i, a.get_ushort_(i)); }
/// for i in 0..2
///     { println!("a.get_sshort_({}) = {}", i, a.get_sshort_(i)); }
/// for i in 0..4
///     { println!("a.get_ubyte_({}) = {}", i, a.get_ubyte_(i)); }
/// for i in 0..4
///     { println!("a.get_sbyte_({}) = {}", i, a.get_sbyte_(i)); }
/// assert_eq!(a.get_ushort_(0), 222_u16);
/// assert_eq!(a.get_ushort_(1), 58598_u16);
/// assert_eq!(a.get_sshort_(0), 222_i16);
/// assert_eq!(a.get_sshort_(1), -6938_i16);
/// assert_eq!(a.get_ubyte_(0), 222_u8);
/// assert_eq!(a.get_ubyte_(1), 0_u8);
/// assert_eq!(a.get_ubyte_(2), 230_u8);
/// assert_eq!(a.get_ubyte_(3), 228_u8);
/// assert_eq!(a.get_sbyte_(0), -34_i8);
/// assert_eq!(a.get_sbyte_(1), 0_i8);
/// assert_eq!(a.get_sbyte_(2), -26_i8);
/// assert_eq!(a.get_sbyte_(3), -28_i8);
/// #[cfg(target_pointer_width = "16")]
/// {
///     const N: usize = 2;
///     for i in 0..N
///         { println!("a.get_usize_({}) = {}", i, a.get_usize_(i)); }
///     for i in 0..N
///         { println!("a.get_ssize_({}) = {}", i, a.get_ssize_(i)); }
///     assert_eq!(a.get_usize_(0), 222_u16);
///     assert_eq!(a.get_usize_(1), 58598_u16);
///     assert_eq!(a.get_ssize_(0), 222_i16);
///     assert_eq!(a.get_ssize_(1), -6938_i16);
/// }
/// // #[cfg(target_pointer_width = "8")]
/// // {
/// //     const N: usize = 4;
/// //     for i in 0..N
/// //         { println!("a.get_usize_({}) = {}", i, a.get_usize_(i)); }
/// //     for i in 0..N
/// //         { println!("a.get_ssize_({}) = {}", i, a.get_ssize_(i)); }
/// //     assert_eq!(a.get_usize_(0), 222_u8);
/// //     assert_eq!(a.get_usize_(1), 0_u8);
/// //     assert_eq!(a.get_usize_(2), 230_u8);
/// //     assert_eq!(a.get_usize_(3), 228_u8);
/// //     assert_eq!(a.get_ssize_(0), -34_i8);
/// //     assert_eq!(a.get_ssize_(1), 0_i8);
/// //     assert_eq!(a.get_ssize_(2), -26_i8);
/// //     assert_eq!(a.get_ssize_(3), -28_i8);
/// // }
/// #[cfg(target_pointer_width = "32")]
/// {
///     println!("a.get_usize() = {}", a.get_usize());
///     println!("a.get_ssize() = {}", a.get_ssize());
///     assert_eq!(a.get_usize(), 3840278750_u32);
///     assert_eq!(a.get_ssize(), -454688546_i32);
/// }
/// ```
/// You can use `IntUnion` as if you used `u32`. You can perform all kinds of
/// arithmetic operations such as addition, subtraction, multiplication, and
/// division (div and rem), and other operations which are available for
/// `u32`. If you use `IntUnion` with the help of `SmallUInt`, it will be
/// even more powerful and convenient. In this case, you don't even have to
/// import (use) `cryptocol::number::IntUnion`.
/// 
/// ## Example 3
/// ```
/// use cryptocol::number::SmallUInt;
/// 
/// let a_intunion = 12345678_u32.into_intunion();
/// let b_intunion = 87654321_u32.into_intunion();
/// let c_intunion = a_intunion.wrapping_add(b_intunion);
/// println!("{} + {} = {}", a_intunion, b_intunion, c_intunion);
/// assert_eq!(c_intunion.get(), 99999999_u32);
/// for i in 0..2
///     { println!("c_intunion.get_ushort_({}) = {}", i, c_intunion.get_ushort_(i)); }
/// assert_eq!(c_intunion.get_ushort_(0), 57599_u16);
/// assert_eq!(c_intunion.get_ushort_(1), 1525_u16);
/// for i in 0..4
///     { println!("c_intunion.get_ubyte_({}) = {}", i, c_intunion.get_ubyte_(i)); }
/// assert_eq!(c_intunion.get_ubyte_(0), 255_u8);
/// assert_eq!(c_intunion.get_ubyte_(1), 224_u8);
/// assert_eq!(c_intunion.get_ubyte_(2), 245_u8);
/// assert_eq!(c_intunion.get_ubyte_(3), 5_u8);
/// 
/// let d_intunion = b_intunion - a_intunion;
/// println!("{} - {} = {}", b_intunion, a_intunion, d_intunion);
/// assert_eq!(d_intunion.get(), 75308643_u32);
/// for i in 0..2
///     { println!("d_shortunion.get_ushort_({}) = {}", i, d_intunion.get_ushort_(i)); }
/// assert_eq!(d_intunion.get_ushort_(0), 7779_u16);
/// assert_eq!(d_intunion.get_ushort_(1), 1149_u16);
/// for i in 0..4
///     { println!("d_shortunion.get_ubyte_({}) = {}", i, d_intunion.get_ubyte_(i)); }
/// assert_eq!(d_intunion.get_ubyte_(0), 99_u8);
/// assert_eq!(d_intunion.get_ubyte_(1), 30_u8);
/// assert_eq!(d_intunion.get_ubyte_(2), 125_u8);
/// assert_eq!(d_intunion.get_ubyte_(3), 4_u8);
/// 
/// let e_intunion = d_intunion * 3_u32.into_intunion();
/// println!("{} * {} = {}", d_intunion, 3_u32.into_intunion(), e_intunion);
/// assert_eq!(e_intunion.get(), 225925929_u32);
/// 
/// let f_intunion = c_intunion / 10_u32.into_intunion();
/// println!("{} / {} = {}", c_intunion, 10_u16.into_intunion(), f_intunion);
/// assert_eq!(f_intunion.get(), 9999999_u32);
/// 
/// let g_intunion = c_intunion % 10_u32.into_intunion();
/// println!("{} % {} = {}", c_intunion, 10_u16.into_intunion(), g_intunion);
/// assert_eq!(g_intunion.get(), 9_u32);
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

    // /// The usize type array whose elements's size is 8-bit size
    // #[cfg(target_pointer_width = "8")] pub u_size: [usize; 4],

    // /// The isize type array whose elements's size is 8-bit size
    // #[cfg(target_pointer_width = "8")] pub s_size: [isize; 4],
}


impl IntUnion
{
    // pub fn new() -> Self
    /// Constructs a new `IntUnion`.
    /// 
    /// # Output
    /// A new object of `IntUnion`.
    /// 
    /// # Initialization
    /// All the fields of the constructed object will be
    /// initialized with `0`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::number::IntUnion;    
    /// let a = IntUnion::new();
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 0_u32);
    /// ```
    #[inline] pub fn new() -> Self  { Self { uint: 0 } }

    // pub fn new_with(uint: u32) -> Self
    /// Constructs a new `IntUnion` with initializing it with `uint`.
    /// 
    /// # Output
    /// A new object of `IntUnion` initialized with the value `uint`.
    /// 
    /// # Initialization
    /// The field of the constructed object will be initialized with `uint`.
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::IntUnion;    
    /// let a = IntUnion::new_with(1234567890_u32);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 1234567890_u32);
    /// ```
    #[inline] pub fn new_with(uint: u32) -> Self    { Self { uint } }

    // pub fn new_with_signed(sint: i32) -> Self
    /// Constructs a new `IntUnion` with initializing it with `sint`.
    /// 
    /// # Output
    /// A new object of `IntUnion` initialized with the value `sint`.
    /// 
    /// # Initialization
    /// The field of the constructed object will be initialized with `sint`.
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::IntUnion;    
    /// let a = IntUnion::new_with_signed(-1234567890_i32);
    /// println!("a = {}", a.get_signed());
    /// assert_eq!(a.get_signed(), -1234567890_i32);
    /// ```
    #[inline] pub fn new_with_signed(sint: i32) -> Self     { Self { sint } }

    // pub fn new_with_ubytes(ubyte: [u8; 4]) -> Self
    /// Constructs a new `IntUnion` with initializing it with `ubyte`.
    /// 
    /// # Output
    /// A new object of `IntUnion` initialized with the value `ubyte`.
    /// 
    /// # Initialization
    /// The field of the constructed object will be initialized with `ubyte`.
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::IntUnion;
    /// let a = IntUnion::new_with_ubytes([222_u8, 0_u8, 230_u8, 228_u8]);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 3840278750_u32);
    /// ```
    #[inline] pub fn new_with_ubytes(ubyte: [u8; 4]) -> Self   { Self { ubyte } }

    // pub fn new_with_ushorts(ushort: [u16; 2]) -> Self
    /// Constructs a new `IntUnion` with initializing it with `ushort`.
    /// 
    /// # Output
    /// A new object of `IntUnion` initialized with the value `ushort`.
    /// 
    /// # Initialization
    /// The field of the constructed object will be initialized with `ushort`.
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::IntUnion;
    /// let a = IntUnion::new_with_ushorts([222_u16, 58598_u16]);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 3840278750_u32);
    /// ```
    #[inline] pub fn new_with_ushorts(ushort: [u16; 2]) -> Self    { Self { ushort } }

    // pub fn new_with_u128(num: u128) -> Self
    /// Constructs a new `IntUnion` with initializing it with the lowest
    /// 32-bit part of `num`.
    /// 
    /// # Output
    /// A new object of `IntUnion` initialized with the value of
    /// the lowest 32-bit part of `num`.
    /// 
    /// # Initialization
    /// The field of the constructed object will be initialized with
    /// the value of the lowest 32-bit part of `num`.
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::IntUnion;
    /// let a = IntUnion::new_with_u128(3840278750_u128);
    /// let b = IntUnion::new_with_u128(123456789012345678901234567890123456789_u128);
    /// println!("a = {}", a.get());
    /// println!("b = {}", b.get());
    /// assert_eq!(a.get(), 3840278750_u32);
    /// assert_eq!(b.get(), 2923004181_u32);
    /// ```
    #[inline] pub fn new_with_u128(num: u128) -> Self { Self { uint: num as u32 } }

    // pub fn new_with_bool(b: bool) -> Self
    /// Constructs a new `IntUnion` with initializing it
    /// with the value of `b`.
    /// 
    /// # Output
    /// A new object of `IntUnion` initialized with the value of `b`
    /// 
    /// # Initialization
    /// The field of the constructed object will be initialized with
    /// the value of `b`.
    /// If `b` is `true`, `self` will have the value `1`.
    /// If `b` is `false`, `self` will have the value `0`.
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::IntUnion;
    /// let a = IntUnion::new_with_bool(true);
    /// let b = IntUnion::new_with_bool(false);
    /// println!("a = {}", a.get());
    /// println!("b = {}", b.get());
    /// assert_eq!(a.get(), 1_u32);
    /// assert_eq!(b.get(), 0_u32);
    /// ```
    #[inline] pub fn new_with_bool(b: bool) -> Self     { Self { uint: b as u32 } }

    // pub fn get(self) -> u32
    /// Returns its value as `u32`.
    /// 
    /// # Output
    /// Its value as `u32`
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::IntUnion;    
    /// let a = IntUnion::new_with(987654321_u32);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 987654321_u32);
    /// ```
    #[inline] pub fn get(self) -> u32           { unsafe { self.this } }

    // pub fn set(&mut self, val: u32)
    /// Sets its value with `val` of type `u32`
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::IntUnion;    
    /// let mut a = IntUnion::new();
    /// a.set(987654321_u32);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 987654321_u32);
    /// ```
    #[inline] pub fn set(&mut self, val: u32)   { self.this = val; }

    // pub fn get_signed(self) -> i32
    /// Returns its value as `i32`.
    /// 
    /// # Output
    /// Its value as `i32`
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::IntUnion;    
    /// let a = IntUnion::new_with(2345678901_u32);
    /// println!("a = {}", a.get_signed());
    /// assert_eq!(a.get_signed(), -1949288395_i32);
    /// ```
    #[inline] pub fn get_signed(self) -> i32    { unsafe { self.that } }

    // pub fn set_signed(&mut self, val: i32)
    /// Sets its value with `val` of type `i32`
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::IntUnion;    
    /// let mut a = IntUnion::new();
    /// a.set_signed(-1949288395_i32);
    /// println!("a = {}", a.get_signed());
    /// assert_eq!(a.get_signed(), -1949288395_i32);
    /// ```
    #[inline] pub fn set_signed(&mut self, val: i32)    { self.that = val; }

    crate::number::get_set_int_fit!();

    crate::number::get_set_byte!(4);

    crate::number::get_set_short!(2);

    #[cfg(target_pointer_width = "32")]     crate::number::get_set_size_fit!();
    #[cfg(target_pointer_width = "16")]     crate::number::get_set_usize!(2);
    // #[cfg(target_pointer_width = "8")]      crate::number::get_set_usize!(4);

    crate::number::integer_union_methods!(u32);
}



crate::number::operators_for_integer_unions_impl! { IntUnion }

crate::number::shift_ops_for_integer_unions_impl! { IntUnion, i8 }
crate::number::shift_ops_for_integer_unions_impl! { IntUnion, i16 }
crate::number::shift_ops_for_integer_unions_impl! { IntUnion, i32 }
crate::number::shift_ops_for_integer_unions_impl! { IntUnion, i64 }
crate::number::shift_ops_for_integer_unions_impl! { IntUnion, i128 }
crate::number::shift_ops_for_integer_unions_impl! { IntUnion, isize }

crate::number::shift_ops_for_integer_unions_impl! { IntUnion, u8 }
crate::number::shift_ops_for_integer_unions_impl! { IntUnion, u16 }
crate::number::shift_ops_for_integer_unions_impl! { IntUnion, u32 }
crate::number::shift_ops_for_integer_unions_impl! { IntUnion, u64 }
crate::number::shift_ops_for_integer_unions_impl! { IntUnion, u128 }
crate::number::shift_ops_for_integer_unions_impl! { IntUnion, usize }

crate::number::shift_ops_for_integer_unions_by_union_impl! { IntUnion, ShortUnion }
crate::number::shift_ops_for_integer_unions_by_union_impl! { IntUnion, IntUnion }
crate::number::shift_ops_for_integer_unions_by_union_impl! { IntUnion, LongUnion }
crate::number::shift_ops_for_integer_unions_by_union_impl! { IntUnion, LongerUnion }
crate::number::shift_ops_for_integer_unions_by_union_impl! { IntUnion, SizeUnion }

crate::number::format_for_integer_unions_impl! { IntUnion }



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
    /// use cryptocol::number::*;
    /// let a_int = IntUnion::new_with_signed(-1234567890_i32);
    /// println!("a_int = {:?}", a_int);
    /// #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_int:?}"), "IntUnion { this: 3060399406, that: -1234567890, uint: 3060399406, sint: -1234567890, ushort: [64814, 46697], sshort: [-722, -18839], ubyte: [46, 253, 105, 182], sbyte: [46, -3, 105, -74] }");
    /// ```
    /// 
    /// # Example for the format specifier :#?
    /// ```
    /// use cryptocol::number::*;
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
        ff.field("this", &self.get())
            .field("that", &self.get_signed())
            .field("uint", &self.get_uint())
            .field("sint", &self.get_sint())
            .field("ushort", &[self.get_ushort_(0), self.get_ushort_(1)])
            .field("sshort", &[self.get_sshort_(0),  self.get_sshort_(1)])
            .field("ubyte", &[self.get_ubyte_(0), self.get_ubyte_(1), self.get_ubyte_(2), self.get_ubyte_(3)])
            .field("sbyte", &[self.get_sbyte_(0), self.get_sbyte_(1), self.get_sbyte_(2), self.get_sbyte_(3)]);
        #[cfg(target_pointer_width = "32")] ff.field("u_size", &self.get_usize())
                                                .field("s_size", &self.get_ssize());
        #[cfg(target_pointer_width = "16")] ff.field("u_size", &[self.get_usize_(0), self.get_usize_(1)])
                                                .field("s_size", &[self.get_ssize_(0), self.get_ssize_(1)]);
        // #[cfg(target_pointer_width = "8")] ff.field("u_size", &[self.get_usize_(0), self.get_usize_(1), self.get_usize_(2), self.get_usize_(3)])
        //                                         .field("s_size", &[self.get_ssize_(0), self.get_ssize_(1), self.get_ssize_(2), self.get_ssize_(3)]);
        ff.finish()
    }
}
