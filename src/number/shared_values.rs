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
use std::cmp::{ PartialEq, PartialOrd };
use std::ops::*;
use std::mem::size_of;
use super::small_uint::SmallUInt;

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
/// ## Example
/// ```
/// use cryptocol::number::{ SmallUInt, SharedValues, IntUnion, LongerUnion };
/// 
/// let a = SharedValues::<u16, u128> { src: 123456789123456789123456789123456789123_u128 };
/// println!("source = {}, Destination = {}", unsafe {a.src}, unsafe {a.des});
/// assert_eq!(unsafe { a.src }, 123456789123456789123456789123456789123_u128);
/// assert_eq!(unsafe { a.des }, 27267_u16);
/// 
/// let b = SharedValues::<IntUnion, u128> { src: 123456789123456789123456789123456789123_u128 };
/// println!("source = {}, Destination = {}", unsafe {b.src}, unsafe {b.des});
/// assert_eq!(unsafe { b.src }, 123456789123456789123456789123456789123_u128);
/// assert_eq!(unsafe { b.des.get() }, 2970839683_u32);
/// 
/// let c = SharedValues::<u16, LongerUnion> { src: 123456789123456789123456789123456789123_u128.into_longerunion() };
/// println!("source = {}, Destination = {}", unsafe {c.src}, unsafe {c.des});
/// assert_eq!(unsafe { c.src.get() }, 123456789123456789123456789123456789123_u128);
/// assert_eq!(unsafe { c.des }, 27267_u16);
/// 
/// let d = SharedValues::<IntUnion, LongerUnion> { src: 123456789123456789123456789123456789123_u128.into_longerunion() };
/// println!("source = {}, Destination = {}", unsafe {d.src}, unsafe {d.des});
/// assert_eq!(unsafe { d.src.get() }, 123456789123456789123456789123456789123_u128);
/// ```
///  
/// # Big-endian issue
/// It is just experimental for Big Endian CPUs. So, you are not encouraged
/// to use it for serious purpose. Only use this crate for Big-endian CPUs
/// with your own full responsibility.
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
    pub fn new() -> Self
    {
        if size_of::<D>() >= size_of::<S>()
            { Self { des: D::zero() } }
        else
            { Self { src: S::zero() } }
    }

    pub fn from_src(src: S) -> Self
    {
        let mut me = SharedValues::<D, S>::new();
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
