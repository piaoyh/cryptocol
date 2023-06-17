// Copyright 2023 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt::{ self, Display, Formatter };
use std::mem::{ size_of, transmute, transmute_copy, zeroed };
use std::ops::*;
use std::cmp::{ PartialEq, PartialOrd, Ordering };
use super::uint::*;
use super::BigNumber;
use super::BigUInt;

/// A trait for big unsigned integer only with user-defined fixed size.
/// 
pub trait BigUnsignedInt<T, const N: usize>
where T: Uint + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
        + Display + ToString,
    Self: Sized + Clone + Copy + Display + ToString
        + Add<Output = Self> + AddAssign + Sub<Output = Self> + SubAssign
        + Mul<Output = Self> + MulAssign + Div<Output = Self> + DivAssign
        + Rem<Output = Self> + RemAssign
        + Shl<i32, Output = Self> + ShlAssign<i32>
        + Shr<i32, Output = Self> + ShrAssign<i32>
        + BitAnd<Self, Output = Self> + BitAndAssign + BitOr<Output = Self> + BitOrAssign
        + BitXorAssign + Not<Output = Self>
        + BigNumber<T, N>
{
    fn set_max(&mut self);
    fn is_max(&self) -> bool;
    fn set_uint(&mut self, val: T);
    fn is_uint(&self, val: T) -> bool;

    fn add_uint(&self, rhs: T) -> Self;
    fn sub_uint(&self, rhs: T) -> Self;
    fn mul_uint(&self, rhs: T) -> Self;
    fn div_uint(&self, rhs: T) -> Self;
    fn rem_uint(&self, rhs: T) -> Self;

    fn into_biguint<U, const M: usize>(&self) -> BigUInt<U, M>
    where U: Uint + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
            + Display + ToString;
    fn into_u128(&self) -> u128;
    fn into_u64(&self) -> u64;
    fn into_u32(&self) -> u32;
    fn into_u16(&self) -> u16;
    fn into_u8(&self) -> u8;
}