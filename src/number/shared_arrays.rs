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
/// `SmallUInt`.
/// 
/// # Quick Start
/// ```
/// // Todo
/// ```
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
