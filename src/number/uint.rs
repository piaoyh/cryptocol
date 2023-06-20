// Copyright 2023 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! A module for generic types of primitive unsigned integral data types
//! used in a lot of modules of the crate Cryptocol.

#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

use std::fmt::{ Debug, Display };
use std::mem::size_of;
use std::ops::*;

/// Trait Uint is for generic type of primitive unsigned integral data types
/// for all modules of the crate Cryptocol.
///  
/// Here, the generic type of primitive unsigned integral data types includes:
/// u8, u16, u32, u64, u128 and usize. You will hardly use the trait Uint unless
/// you improve the crate Cryptocol or create addional libraries that works with
/// the crate Cryptocol. But, if you only use the crate Cryptocol, you can
/// forget about this trait Uint.
///
pub trait Uint: Copy + Clone + Display + Debug + ToString
            + Add + AddAssign + Sub + SubAssign + Mul + MulAssign + Div + DivAssign
            + Shl + ShlAssign + Shr + ShrAssign
            + BitAnd + BitAndAssign + BitOr + BitOrAssign
            + BitXor + BitXorAssign + Not
            + PartialEq + PartialOrd
{
    fn wrapping_add(self, rhs: Self) -> Self;
    fn wrapping_sub(self, rhs: Self) -> Self;
    fn wrapping_mul(self, rhs: Self) -> Self;
    fn wrapping_div(self, rhs: Self) -> Self;
    fn into_f64(self) -> f64;
    fn into_f32(self) -> f32;
    fn into_u128(self) -> u128;
    fn into_u64(self) -> u64;
    fn into_u32(self) -> u32;
    fn into_u16(self) -> u16;
    fn into_u8(self) -> u8;
    fn into_usize(self) -> usize;
    fn into_bool(self) -> bool;
    fn zero() -> Self;
    fn one() -> Self;
    fn Max() -> Self;
    fn Min() -> Self;
    fn num(n: u128) -> Self;
}

impl Uint for u8
{
    fn wrapping_add(self, rhs: Self) -> Self    { self.wrapping_add(rhs) }
    fn wrapping_sub(self, rhs: Self) -> Self    { self.wrapping_sub(rhs) }
    fn wrapping_mul(self, rhs: Self) -> Self    { self.wrapping_mul(rhs) }
    fn wrapping_div(self, rhs: Self) -> Self    { self.wrapping_div(rhs) }
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_u64(self) -> u64    { self as u64 }
    fn into_u32(self) -> u32    { self as u32 }
    fn into_u16(self) -> u16    { self as u16 }
    fn into_u8(self) -> u8      { self as u8 }
    fn into_usize(self) -> usize { self as usize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { Self::MAX }
    fn Min() -> Self            { Self::MIN }
    fn num(n: u128) -> Self     { n as Self }
}

impl Uint for u16
{
    fn wrapping_add(self, rhs: Self) -> Self    { self.wrapping_add(rhs) }
    fn wrapping_sub(self, rhs: Self) -> Self    { self.wrapping_sub(rhs) }
    fn wrapping_mul(self, rhs: Self) -> Self    { self.wrapping_mul(rhs) }
    fn wrapping_div(self, rhs: Self) -> Self    { self.wrapping_div(rhs) }
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_u64(self) -> u64    { self as u64 }
    fn into_u32(self) -> u32    { self as u32 }
    fn into_u16(self) -> u16    { self as u16 }
    fn into_u8(self) -> u8      { self as u8 }
    fn into_usize(self) -> usize { self as usize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { Self::MAX }
    fn Min() -> Self            { Self::MIN }
    fn num(n: u128) -> Self     { n as Self }
}

impl Uint for u32
{
    fn wrapping_add(self, rhs: Self) -> Self    { self.wrapping_add(rhs) }
    fn wrapping_sub(self, rhs: Self) -> Self    { self.wrapping_sub(rhs) }
    fn wrapping_mul(self, rhs: Self) -> Self    { self.wrapping_mul(rhs) }
    fn wrapping_div(self, rhs: Self) -> Self    { self.wrapping_div(rhs) }
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_u64(self) -> u64    { self as u64 }
    fn into_u32(self) -> u32    { self as u32 }
    fn into_u16(self) -> u16    { self as u16 }
    fn into_u8(self) -> u8      { self as u8 }
    fn into_usize(self) -> usize { self as usize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { Self::MAX }
    fn Min() -> Self            { Self::MIN }
    fn num(n: u128) -> Self     { n as Self }
}

impl Uint for u64
{
    fn wrapping_add(self, rhs: Self) -> Self    { self.wrapping_add(rhs) }
    fn wrapping_sub(self, rhs: Self) -> Self    { self.wrapping_sub(rhs) }
    fn wrapping_mul(self, rhs: Self) -> Self    { self.wrapping_mul(rhs) }
    fn wrapping_div(self, rhs: Self) -> Self    { self.wrapping_div(rhs) }
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_u64(self) -> u64    { self as u64 }
    fn into_u32(self) -> u32    { self as u32 }
    fn into_u16(self) -> u16    { self as u16 }
    fn into_u8(self) -> u8      { self as u8 }
    fn into_usize(self) -> usize { self as usize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { Self::MAX }
    fn Min() -> Self            { Self::MIN }
    fn num(n: u128) -> Self     { n as Self }
}

impl Uint for u128
{
    fn wrapping_add(self, rhs: Self) -> Self    { self.wrapping_add(rhs) }
    fn wrapping_sub(self, rhs: Self) -> Self    { self.wrapping_sub(rhs) }
    fn wrapping_mul(self, rhs: Self) -> Self    { self.wrapping_mul(rhs) }
    fn wrapping_div(self, rhs: Self) -> Self    { self.wrapping_div(rhs) }
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_u64(self) -> u64    { self as u64 }
    fn into_u32(self) -> u32    { self as u32 }
    fn into_u16(self) -> u16    { self as u16 }
    fn into_u8(self) -> u8      { self as u8 }
    fn into_usize(self) -> usize { self as usize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { Self::MAX }
    fn Min() -> Self            { Self::MIN }
    fn num(n: u128) -> Self     { n as Self }
}

impl Uint for usize
{
    fn wrapping_add(self, rhs: Self) -> Self    { self.wrapping_add(rhs) }
    fn wrapping_sub(self, rhs: Self) -> Self    { self.wrapping_sub(rhs) }
    fn wrapping_mul(self, rhs: Self) -> Self    { self.wrapping_mul(rhs) }
    fn wrapping_div(self, rhs: Self) -> Self    { self.wrapping_div(rhs) }
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_u64(self) -> u64    { self as u64 }
    fn into_u32(self) -> u32    { self as u32 }
    fn into_u16(self) -> u16    { self as u16 }
    fn into_u8(self) -> u8      { self as u8 }
    fn into_usize(self) -> usize { self as usize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { Self::MAX }
    fn Min() -> Self            { Self::MIN }
    fn num(n: u128) -> Self     { n as Self }
}

#[derive(Copy, Clone)]
pub union UShort
{
    pub ushort: u16,
    pub byte: [u8; 2],
}

#[derive(Copy, Clone)]
pub union UInt
{
    pub uint: u32,
    pub ushort: [u16; 2],
    pub byte: [u8; 4],
}

#[derive(Copy, Clone)]
pub union ULong
{
    pub ulong: u64,
    pub uint: [u32; 2],
    pub ushort: [u16; 4],
    pub byte: [u8; 8],
}

#[derive(Copy, Clone)]
pub union ULonger
{
    pub ulonger: u128,
    pub ulong: [u64; 2],
    pub uint: [u32; 4],
    pub ushort: [u16; 8],
    pub byte: [u8; 16],
}

#[cfg(target_pointer_width = "128")]
#[derive(Copy, Clone)]
pub union USize
{
    pub size: usize,
    pub ulonger: u128,
    pub ulong: [u64; 2],
    pub uint: [u32; 4],
    pub ushort: [u16; 8],
    pub byte: [u8; 16],
}

#[cfg(target_pointer_width = "64")]
#[derive(Copy, Clone)]
pub union USize
{
    pub size: usize,
    pub ulong: u64,
    pub uint: [u32; 2],
    pub ushort: [u16; 4],
    pub byte: [u8; 8],
}

#[cfg(target_pointer_width = "32")]
#[derive(Copy, Clone)]
pub union USize
{
    pub size: usize,
    pub uint: u32,
    pub ushort: [u16; 2],
    pub byte: [u8; 4],
}

#[cfg(target_pointer_width = "16")]
#[derive(Copy, Clone)]
pub union USize
{
    pub size: usize,
    pub ushort: u16,
    pub byte: [u8; 2],
}

#[cfg(target_pointer_width = "8")]
#[derive(Copy, Clone)]
pub union USize
{
    pub size: usize,
    pub byte: u8,
}

impl UShort
{
    pub fn new() -> Self    { Self { ushort: 0 } }
    pub fn new_with(ushort: u16) -> Self   { Self { ushort } }
}

impl UInt
{
    pub fn new() -> Self    { Self { uint: 0 } }
    pub fn new_with(uint: u32) -> Self   { Self { uint } }
}

impl ULong
{
    pub fn new() -> Self    { Self { ulong: 0 } }
    pub fn new_with(ulong: u64) -> Self   { Self { ulong } }
}

impl ULonger
{
    pub fn new() -> Self    { Self { ulonger: 0 } }
    pub fn new_with(ulonger: u128) -> Self   { Self { ulonger } }
}

impl USize
{
    pub fn new() -> Self    { Self { size: 0 } }
    pub fn new_with(size: usize) -> Self   { Self { size } }
}

/// union array for transforming from one type into anther type
pub union Share<D, S>
where D: Uint + Add<Output=D> + AddAssign + Sub<Output=D> + SubAssign
        + Mul<Output=D> + MulAssign + Div<Output=D> + DivAssign
        + Shl<Output=D> + ShlAssign + Shr<Output=D> + ShrAssign
        + BitAnd<Output=D> + BitAndAssign + BitOr<Output=D> + BitOrAssign
        + BitXor<Output=D> + BitXorAssign + Not<Output=D>
        + PartialEq + PartialOrd
        + Display + ToString,
      S: Uint + Add<Output=S> + AddAssign + Sub<Output=S> + SubAssign
        + Mul<Output=S> + MulAssign + Div<Output=S> + DivAssign
        + Shl<Output=S> + ShlAssign + Shr<Output=S> + ShrAssign
        + BitAnd<Output=S> + BitAndAssign + BitOr<Output=S> + BitOrAssign
        + BitXor<Output=S> + BitXorAssign + Not<Output=S>
        + PartialEq + PartialOrd
        + Display + ToString
{
    pub des: D,
    pub src: S,
}

impl<D, S> Share<D, S>
where D: Uint + Add<Output=D> + AddAssign + Sub<Output=D> + SubAssign
        + Mul<Output=D> + MulAssign + Div<Output=D> + DivAssign
        + Shl<Output=D> + ShlAssign + Shr<Output=D> + ShrAssign
        + BitAnd<Output=D> + BitAndAssign + BitOr<Output=D> + BitOrAssign
        + BitXor<Output=D> + BitXorAssign + Not<Output=D>
        + PartialEq + PartialOrd
        + Display + ToString,
      S: Uint + Add<Output=S> + AddAssign + Sub<Output=S> + SubAssign
        + Mul<Output=S> + MulAssign + Div<Output=S> + DivAssign
        + Shl<Output=S> + ShlAssign + Shr<Output=S> + ShrAssign
        + BitAnd<Output=S> + BitAndAssign + BitOr<Output=S> + BitOrAssign
        + BitXor<Output=S> + BitXorAssign + Not<Output=S>
        + PartialEq + PartialOrd
        + Display + ToString
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
        let mut me = Share::<D, S>::new();
        unsafe { me.src = src; }
        me
    }

    #[cfg(target_endian = "little")]
    pub fn into_des(&mut self, pos: usize) -> Option<D>
    {
        let bit_pos = pos * size_of::<D>() * 8;
        unsafe { self.src >>= S::num(bit_pos as u128); }
        if (bit_pos > 0) && self.is_src_zero()
            { None }
        else
            { unsafe { Some(self.des) } }
    }

    #[cfg(target_endian = "big")]
    pub fn into_des1(&mut self, pos: usize) -> Option<D>
    {
        let des_size = size_of::<D>();
        let src_size = size_of::<S>();
        let bit_pos = pos * size_of::<D>() * 8;
        unsafe { self.src <<= S::num(bit_pos as u128); }
        if des_size > src_size
            { unsafe { self.des >>= D::num((des_size - src_size).into_u128() * 8); } }
        else if src_size > des_size
            { unsafe { self.src <<= S::num((src_size - des_size).into_u128() * 8); } }
        Some( unsafe { self.des } )
    }

    pub fn is_src_zero(&self) -> bool    { unsafe { self.src == S::zero() } }
}

/// union array for transforming from one type into anther type
pub union Common<D, const N: usize, S, const M: usize>
where D: Uint + Add<Output=D> + AddAssign + Sub<Output=D> + SubAssign
        + Mul<Output=D> + MulAssign + Div<Output=D> + DivAssign
        + Shl<Output=D> + ShlAssign + Shr<Output=D> + ShrAssign
        + BitAnd<Output=D> + BitAndAssign + BitOr<Output=D> + BitOrAssign
        + BitXor<Output=D> + BitXorAssign + Not<Output=D>
        + PartialEq + PartialOrd
        + Display + ToString,
      S: Uint + Add<Output=S> + AddAssign + Sub<Output=S> + SubAssign
        + Mul<Output=S> + MulAssign + Div<Output=S> + DivAssign
        + Shl<Output=S> + ShlAssign + Shr<Output=S> + ShrAssign
        + BitAnd<Output=S> + BitAndAssign + BitOr<Output=S> + BitOrAssign
        + BitXor<Output=S> + BitXorAssign + Not<Output=S>
        + PartialEq + PartialOrd
        + Display + ToString
{
    pub des: [D; N],
    pub src: [S; M],
}

impl<D, const N: usize, S, const M: usize> Common<D, N, S, M>
where D: Uint + Add<Output=D> + AddAssign + Sub<Output=D> + SubAssign
        + Mul<Output=D> + MulAssign + Div<Output=D> + DivAssign
        + Shl<Output=D> + ShlAssign + Shr<Output=D> + ShrAssign
        + BitAnd<Output=D> + BitAndAssign + BitOr<Output=D> + BitOrAssign
        + BitXor<Output=D> + BitXorAssign + Not<Output=D>
        + PartialEq + PartialOrd
        + Display + ToString,
      S: Uint + Add<Output=S> + AddAssign + Sub<Output=S> + SubAssign
        + Mul<Output=S> + MulAssign + Div<Output=S> + DivAssign
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
        let mut me = Common::<D, N, S, M>::new();
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
/*
impl Shr<usize> for USize
{
    type Output = Self;
    fn shr(self, rhs: usize) -> Self   { let mut s = self; s.size >>= rhs; s}
}

impl ShrAssign<usize> for USize
{
    fn shr_assign(&mut self, rhs: usize)    { self.size >>= rhs; }
}
*/

/*
pub trait Bool: Copy + Eq
{
    fn into_f64(self) -> f64;
    fn into_f32(self) -> f32;
    fn into_u128(self) -> u128;
    fn into_i128(self) -> i128;
    fn into_usize(self) -> usize;
    fn zero() -> Self;
    fn one() -> Self;
    fn Max() -> Self;
    fn Min() -> Self;
}

impl Bool for u8
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { 1 }
    fn Min() -> Self            { 0 }
}

impl Bool for u16
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { 1 }
    fn Min() -> Self            { 0 }
}

impl Bool for u32
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { 1 }
    fn Min() -> Self            { 0 }
}

impl Bool for u64
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { 1 }
    fn Min() -> Self            { 0 }
}

impl Bool for u128
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { 1 }
    fn Min() -> Self            { 0 }
}

impl Bool for i8
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { 1 }
    fn Min() -> Self            { 0 }
}

impl Bool for i16
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { 1 }
    fn Min() -> Self            { 0 }
}

impl Bool for i32
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { 1 }
    fn Min() -> Self            { 0 }
}

impl Bool for i64
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { 1 }
    fn Min() -> Self            { 0 }
}

impl Bool for i128
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { 1 }
    fn Min() -> Self            { 0 }
}
*/
