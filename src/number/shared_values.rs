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

use std::fmt::{ Debug, Display };
use std::cmp::{ PartialEq, PartialOrd };
use std::ops::*;
use std::mem::size_of;

use crate::number::SmallUInt;

/// union for transforming from one type into anther type
/// 
/// The type S is the source type while the type D is the destination type.
/// The type S and D should have `SmallUInt`. In this crate, `u8`, `u16`,
/// `u32`, `u64`, `u128`, `i8`, `i16`, `i32`, `i64`, `i128`, `ShortUnion`,
/// `IntUnion`, `LongUnion`, `LongerUnion`, and `SizeUnion` have the trait
/// `SmallUInt`.
/// 
/// Unlike `ShortUnion`, `IntUnion`, `LongUnion`, `LongerUnion`, and
/// `SizeUnion`, the fields of `SharedValues` are all public. So,
/// `SharedValues` is very flexible and gives you the full control of itself.
/// You can convert data from one type into another type freely by means of
/// `SharedValues` if the data has the trait `SmallUInt`.
/// However, you have to use unsafe {...}.
/// 
/// # Quick Start
/// You can freely convert from a primitive type or a union type into another
/// primitive type or a union type.
/// 
/// ## Example 1 for primitive data types for source and destination
/// ```
/// use cryptocol::number::{ SmallUInt, SharedValues, IntUnion, LongerUnion };
/// let a = SharedValues::<u16, u128> { src: 123456789123456789123456789123456789123_u128 };
/// println!("source = {}, Destination = {}", unsafe {a.src}, unsafe {a.des});
/// assert_eq!(unsafe { a.src }, 123456789123456789123456789123456789123_u128);
/// assert_eq!(unsafe { a.des }, 27267_u16);
/// ```
/// 
/// ## Example 2 for primitive data type for source and union data type for destination
/// ```
/// use cryptocol::number::{ SmallUInt, SharedValues, IntUnion, LongerUnion };
/// let mut b = SharedValues::<IntUnion, u128>::new();
/// b.src = 123456789123456789123456789123456789123_u128;
/// println!("source = {}, Destination = {}", b.get_src(), b.get_des().get());
/// assert_eq!(b.get_src(), 123456789123456789123456789123456789123_u128);
/// assert_eq!(b.get_des().get(), 2970839683_u32);
/// ```
/// 
/// ## Example 3 for primitive data type for destination and union data type for source
/// ```
/// use cryptocol::number::{ SmallUInt, SharedValues, IntUnion, LongerUnion };
/// let c = SharedValues::<u16, LongerUnion>::from_src(123456789123456789123456789123456789123_u128.into_longerunion());
/// println!("source = {}, Destination = {}", unsafe {c.src}, unsafe {c.des});
/// assert_eq!(unsafe { c.src.get() }, 123456789123456789123456789123456789123_u128);
/// assert_eq!(unsafe { c.des }, 27267_u16);
/// ```
/// 
/// ## Example 4 for union data type for source and destination
/// ```
/// use cryptocol::number::{ SmallUInt, SharedValues, IntUnion, LongerUnion };
/// let d = SharedValues::<IntUnion, LongerUnion>::from_src(123456789123456789123456789123456789123_u128.into_longerunion());
/// println!("source = {}, Destination = {}", d.get_src().get(), d.get_des().get());
/// assert_eq!(d.get_src().get(), 123456789123456789123456789123456789123_u128);
/// assert_eq!(d.get_des().get(), 2970839683_u32);
/// ```
/// 
/// # Big-endian issue
/// It is just experimental for Big Endian CPUs. So, you are not encouraged
/// to use it for serious purpose. Only use this crate for Big-endian CPUs
/// with your own full responsibility.
#[derive(Copy, Clone)]
pub union SharedValues<D, S>
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

impl<D, S> SharedValues<D, S>
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
    /// Constructs a new `SharedValues<D, S>`.
    /// 
    /// # Output
    /// A new object of `SharedValues<D, S>`.
    /// 
    /// # Initialization
    /// All the fields of the constructed object will be
    /// initialized with `0`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::number::SharedValues;    
    /// let a = SharedValues::<u16, u128>::new();
    /// println!("source = {}, Destination = {}", a.get_src(), a.get_des());
    /// assert_eq!(a.get_src(), 0_u128);
    /// assert_eq!(a.get_des(), 0_u16);
    /// ```
    pub fn new() -> Self
    {
        if size_of::<D>() >= size_of::<S>()
            { Self { des: D::zero() } }
        else
            { Self { src: S::zero() } }
    }

    /// Constructs a new `SharedValues<D, S>` from `S` type value.
    /// 
    /// # Output
    /// A new object of `SharedValues<D, S>`.
    /// 
    /// # Argument
    /// The field `src` of the constructed object will be
    /// initialized with the aregument `src`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::number::SharedValues;    
    /// let a = SharedValues::<u32, u128>::from_src(123456789123456789123456789123456789123_u128);
    /// println!("source = {}, Destination = {}", a.get_src(), a.get_des());
    /// assert_eq!(a.get_src(), 123456789123456789123456789123456789123_u128);
    /// assert_eq!(a.get_des(), 2970839683_u32);
    /// ```
    pub fn from_src(src: S) -> Self
    {
        let mut me = SharedValues::<D, S>::new();
        me.src = src;
        me
    }

    /// Gets a value of source type.
    /// 
    /// # Output
    /// A value of source type.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::number::SharedValues;    
    /// let a = SharedValues::<u32, u128>::from_src(123456789123456789123456789123456789123_u128);
    /// println!("source = {}, Destination = {}", a.get_src(), a.get_des());
    /// assert_eq!(a.get_src(), 123456789123456789123456789123456789123_u128);
    /// assert_eq!(a.get_des(), 2970839683_u32);
    /// ```
    #[inline] pub fn get_src(self) -> S  { unsafe { self.src } }

    /// Gets a value of destination type.
    /// 
    /// # Output
    /// A value of destination type.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::number::SharedValues;
    /// let a = SharedValues::<u16, u128>::from_src(123456789123456789123456789123456789123_u128);
    /// println!("source = {}, Destination = {}", a.get_src(), a.get_des());
    /// assert_eq!(a.get_src(), 123456789123456789123456789123456789123_u128);
    /// assert_eq!(a.get_des(), 27267_u16);
    /// ```
    #[inline] pub fn get_des(self) -> D  { unsafe { self.des } }

    // #[cfg(target_endian = "little")]
    // pub fn into_des(&mut self, pos: usize) -> Option<D>
    // {
    //     let bit_pos = pos * D::size_in_bits();
    //     unsafe { self.src >>= S::usize_as_smalluint(bit_pos); }
    //     if (bit_pos > 0) && self.is_src_zero()
    //         { None }
    //     else
    //         { unsafe { Some(self.des) } }
    // }
    //
    // #[cfg(target_endian = "big")]
    // pub fn into_des(&mut self, pos: usize) -> Option<D>
    // {
    //     let des_size = size_of::<D>();
    //     let src_size = size_of::<S>();
    //     let bit_pos = pos * D::size_in_bits();
    //     unsafe { self.src <<= S::usize_as_smalluint(bit_pos); }
    //     if des_size > src_size
    //         { unsafe { self.des >>= D::num((des_size - src_size).into_u128() * 8); } }
    //     else if src_size > des_size
    //         { unsafe { self.src <<= S::num((src_size - des_size).into_u128() * 8); } }
    //     Some( unsafe { self.des } )
    // }

    /// Checks whether or not `src` is zero.
    /// 
    /// # Output
    /// A boolean value of whether or not `src` is zero.
    /// 
    /// # Example 1 for the case that `src` is zero
    /// ```
    /// use cryptocol::number::SharedValues;
    /// let a = SharedValues::<u32, u64>::new();
    /// println!("Is a.src zero? {}", if a.is_src_zero() {"yes"} else {"no"});
    /// assert_eq!(a.is_src_zero(), true);
    /// ```
    /// 
    /// # Example 2 for the case that `src` is non-zero
    /// ```
    /// use cryptocol::number::SharedValues;
    /// let b = SharedValues::<u16, u128>::from_src(123456789123456789123456789123456789123_u128);
    /// println!("Is b.src zero? {}", if b.is_src_zero() {"yes"} else {"no"});
    /// assert_eq!(b.is_src_zero(), false);
    /// ```
    pub fn is_src_zero(self) -> bool    { unsafe { self.src == S::zero() } }
}
