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

use std::fmt::Display;
use std::mem::size_of;
use std::cmp::{ PartialEq, PartialOrd };
use std::ops::*;
use super::small_uint::SmallUInt;


/// union for transforming from an array of one type into another array of
/// anther type
/// 
/// The type S is the source type while the type D is the destination type.
/// The type S and D should have `SmallUInt`. In this crate, `u8`, `u16`,
/// `u32`, `u64`, `u128`, `i8`, `i16`, `i32`, `i64`, `i128`, `ShortUnion`,
/// `IntUnion`, `LongUnion`, `LongerUnion`, and `SizeUnion` have the trait
/// `SmallUInt`.
/// 
/// N is the number of elements of D type while M is the number of elements
/// of S.
/// 
/// Unlike `ShortUnion`, `IntUnion`, `LongUnion`, `LongerUnion`, and
/// `SizeUnion`, the fields of `SharedArrays` are all public. So,
/// `SharedArrays` is very flexible and gives you the full control of itself.
/// You can convert data array from one type into another type freely by
/// means of `SharedArrays` if the element of the data array has the trait
/// `SmallUInt`. However, you have to use unsafe {...}.
/// 
/// # Quick Start
/// You can freely convert from an array of primitive type or another union type
/// into an array of another primitive type or anther union type.
/// 
/// ## Example
/// ```
/// use cryptocol::number::{ SmallUInt, SharedArrays, IntUnion, LongUnion };
/// 
/// let a = SharedArrays::<u16, 8, u64, 2> { src: [123456789123456789_u64, 987654321987654321_u64] };
/// print!("source = [ ");
/// for i in 0..2
///     { print!("{} ", unsafe {a.src[i]}); }
/// println!("]");
/// print!("Destination = [ ");
/// for i in 0..8
///     { print!("{} ", unsafe {a.des[i]}); }
/// println!("]");
/// assert_eq!(unsafe { a.src[0] }, 123456789123456789_u64);
/// assert_eq!(unsafe { a.src[1] }, 987654321987654321_u64);
/// assert_eq!(unsafe { a.des[0] }, 24341_u16);
/// assert_eq!(unsafe { a.des[1] }, 44240_u16);
/// assert_eq!(unsafe { a.des[2] }, 39755_u16);
/// assert_eq!(unsafe { a.des[3] }, 438_u16);
/// assert_eq!(unsafe { a.des[4] }, 4785_u16);
/// assert_eq!(unsafe { a.des[5] }, 32500_u16);
/// assert_eq!(unsafe { a.des[6] }, 55903_u16);
/// assert_eq!(unsafe { a.des[7] }, 3508_u16);
/// 
/// let b = SharedArrays::<IntUnion, 8, u64, 2> { src: [123456789123456789_u64, 987654321987654321_u64] };
/// print!("source = [ ");
/// for i in 0..2
///     { print!("{} ", unsafe {b.src[i]}); }
/// println!("]");
/// print!("Destination = [ ");
/// for i in 0..8
///     { print!("{} ", unsafe {b.des[i]}); }
/// println!("]");
/// assert_eq!(unsafe { b.src[0] }, 123456789123456789_u64);
/// assert_eq!(unsafe { b.src[1] }, 987654321987654321_u64);
/// assert_eq!(unsafe { b.des[0].get() }, 2899336981_u32);
/// assert_eq!(unsafe { b.des[1].get() }, 28744523_u32);
/// assert_eq!(unsafe { b.des[2].get() }, 2129924785_u32);
/// assert_eq!(unsafe { b.des[3].get() }, 229956191_u32);
/// assert_eq!(unsafe { b.des[4].get() }, 229956191_u32);
/// assert_eq!(unsafe { b.des[5].get() }, 229956191_u32);
/// assert_eq!(unsafe { b.des[6].get() }, 229956191_u32);
/// assert_eq!(unsafe { b.des[7].get() }, 229956191_u32);
/// 
/// let c = SharedArrays::<u16, 8, LongUnion, 2> { src: [123456789123456789_u64.into_longunion(), 987654321987654321_u64.into_longunion()] };
/// print!("source = [ ");
/// for i in 0..2
///     { print!("{} ", unsafe {c.src[i]}); }
/// println!("]");
/// print!("Destination = [ ");
/// for i in 0..8
///     { print!("{} ", unsafe {c.des[i]}); }
/// println!("]");
/// assert_eq!(unsafe { c.src[0].get() }, 123456789123456789_u64);
/// assert_eq!(unsafe { c.src[1].get() }, 987654321987654321_u64);
/// assert_eq!(unsafe { c.des[0] }, 24341_u16);
/// assert_eq!(unsafe { c.des[1] }, 44240_u16);
/// assert_eq!(unsafe { c.des[2] }, 39755_u16);
/// assert_eq!(unsafe { c.des[3] }, 438_u16);
/// assert_eq!(unsafe { c.des[4] }, 4785_u16);
/// assert_eq!(unsafe { c.des[5] }, 32500_u16);
/// assert_eq!(unsafe { c.des[6] }, 55903_u16);
/// assert_eq!(unsafe { c.des[7] }, 3508_u16);
///     
/// let d = SharedArrays::<IntUnion, 8, LongUnion, 2> { src: [123456789123456789_u64.into_longunion(), 987654321987654321_u64.into_longunion()] };
/// print!("source = [ ");
/// for i in 0..2
///     { print!("{} ", unsafe {d.src[i]}); }
/// println!("]");
/// print!("Destination = [ ");
/// for i in 0..8
///     { print!("{} ", unsafe {d.des[i]}); }
/// println!("]");
/// assert_eq!(unsafe { d.src[0].get() }, 123456789123456789_u64);
/// assert_eq!(unsafe { d.src[1].get() }, 987654321987654321_u64);
/// assert_eq!(unsafe { d.des[0].get() }, 2899336981_u32);
/// assert_eq!(unsafe { d.des[1].get() }, 28744523_u32);
/// assert_eq!(unsafe { d.des[2].get() }, 2129924785_u32);
/// assert_eq!(unsafe { d.des[3].get() }, 229956191_u32);
/// assert_eq!(unsafe { d.des[4].get() }, 229956191_u32);
/// assert_eq!(unsafe { d.des[5].get() }, 229956191_u32);
/// assert_eq!(unsafe { d.des[6].get() }, 229956191_u32);
/// assert_eq!(unsafe { d.des[7].get() }, 229956191_u32);
/// ```
///  
/// # Big-endian issue
/// It is just experimental for Big Endian CPUs. So, you are not encouraged
/// to use it for serious purpose. Only use this crate for Big-endian CPUs
/// with your own full responsibility.
pub union SharedArrays<D, const N: usize, S, const M: usize>
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

impl<D, const N: usize, S, const M: usize> SharedArrays<D, N, S, M>
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
        let mut me = SharedArrays::<D, N, S, M>::new();
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
