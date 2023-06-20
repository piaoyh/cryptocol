// Copyright 2023 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! A module that contains a trait for big unsigned integer and big signed
//! integer with user-defined fixed size.

use std::fmt::{ Display, Debug };
use std::cmp::{ PartialEq, PartialOrd, Ordering };
use std::ops::*;

use super::uint::*;
use super::HugeInteger;

/// A trait for big unsigned integer and big signed integer with user-defined fixed size.
/// 
pub trait BigNumber<T, const N: usize>
where T: Uint + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd,
    Self: Sized + Clone + Copy + Display + Debug + ToString 
        + Add<Output = Self> + AddAssign + Sub<Output = Self> + SubAssign
        + Mul<Output = Self> + MulAssign + Div<Output = Self> + DivAssign
        + Rem<Output = Self> + RemAssign
        + Shl<i32, Output = Self> + ShlAssign<i32>
        + Shr<i32, Output = Self> + ShrAssign<i32>
        + BitAnd<Self, Output = Self> + BitAndAssign + BitOr<Output = Self> + BitOrAssign
        + BitXorAssign + Not<Output = Self>
        + HugeInteger<T>
{
    /// A flag to represent whether or not overflow happened
    /// during previous operations. When divided-by-zero happens,
    /// the flags DIVIDED_BY_ZERO, INFINITY and OVERFLOW will be set.
    const OVERFLOW: u8          = 0b0000_0100;

    /// A flag to represent whether or not underflow happened
    /// during previous operations.
    const UNDERFLOW: u8         = 0b0000_1000;

    fn get_num(&self, i: usize) -> T;
    fn set_num(&mut self, i: usize, val: T);
    fn get_number(&self) -> &[T; N];
    fn set_number(&mut self, val: &[T; N]);
    
    fn set_zero(&mut self)
    {
        for i in 0..N
            { self.set_num(i, T::zero()); }
    }

    fn is_zero(&self) -> bool
    {
        for i in 0..N
        {
            if self.get_num(i) != T::zero()
                { return false; }
        }
        true
    }

    #[cfg(target_endian = "little")]
    fn set_one(&mut self)
    {
        for i in 1..N
            { self.set_num(i, T::zero()); }
        self.set_num(0, T::one());
    }

    #[cfg(target_endian = "big")]
    fn set_one(&mut self)
    {
        for i in 0..N-1
            { self.set_num(i, T::zero()); }
        self.set_num(N-1, T::one());
    }

    fn partial_cmp_uint(&self, other: T) -> Option<Ordering>;
    fn lt_uint(&self, other: T) -> bool  { self.partial_cmp_uint(other).unwrap().is_lt() }
    fn gt_uint(&self, other: T) -> bool  { self.partial_cmp_uint(other).unwrap().is_gt() }
    fn le_uint(&self, other: T) -> bool  { self.partial_cmp_uint(other).unwrap().is_le() }
    fn ge_uint(&self, other: T) -> bool  { self.partial_cmp_uint(other).unwrap().is_ge() }
    fn eq_uint(&self, other: T) -> bool  { self.partial_cmp_uint(other).unwrap().is_eq() }

    fn set_overflow(&mut self)      { self.set_flag_bit(Self::OVERFLOW); }
    fn reset_overflow(&mut self)    { self.reset_flag_bit(Self::OVERFLOW); }
    fn is_overflow(&self) -> bool   { self.is_flag_bit_on(Self::OVERFLOW) }
    fn set_underflow(&mut self)     { self.set_flag_bit(Self::UNDERFLOW); }
    fn reset_underflow(&mut self)   { self.reset_flag_bit(Self::UNDERFLOW); }
    fn is_underflow(&self) -> bool  { self.is_flag_bit_on(Self::UNDERFLOW) }
    fn set_untrustable(&mut self)   { self.set_flag_bit(Self::OVERFLOW | Self::UNDERFLOW); }
    fn reset_untrustable(&mut self) { self.reset_flag_bit(Self::OVERFLOW | Self::UNDERFLOW); }
    fn is_untrustable(&self) -> bool { self.is_flag_bit_on(Self::OVERFLOW | Self::UNDERFLOW) }
}
