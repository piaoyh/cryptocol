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

    crate::number::get_set_byte!(4);

    crate::number::get_set_short!(2);

    #[cfg(target_pointer_width = "32")]     crate::number::get_set_size_fit!();
    #[cfg(target_pointer_width = "16")]     crate::number::get_set_usize!(2);
    #[cfg(target_pointer_width = "8")]      crate::number::get_set_usize!(4);

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

crate::number::display_for_integer_unions_impl! { IntUnion }



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
        #[cfg(target_pointer_width = "8")] ff.field("u_size", &[self.get_usize_(0), self.get_usize_(1), self.get_usize_(2), self.get_usize_(3)])
                                                .field("s_size", &[self.get_ssize_(0), self.get_ssize_(1), self.get_ssize_(2), self.get_ssize_(3)]);
        ff.finish()
    }
}
