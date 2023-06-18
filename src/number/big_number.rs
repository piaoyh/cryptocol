// Copyright 2023 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt::{ Display, Debug };
use std::cmp::{ PartialEq, PartialOrd, Ordering };
use std::ops::*;

use super::uint::*;

/// A trait for big unsigned integer and big signed integer with user-defined fixed size.
/// 
pub trait BigNumber<T, const N: usize>
where T: Uint + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd,
    Self: Sized + Clone + Copy
        + Add<Output = Self> + AddAssign + Sub<Output = Self> + SubAssign
        + Mul<Output = Self> + MulAssign + Div<Output = Self> + DivAssign
        + Rem<Output = Self> + RemAssign
        + Shl<i32, Output = Self> + ShlAssign<i32>
        + Shr<i32, Output = Self> + ShrAssign<i32>
        + BitAnd<Self, Output = Self> + BitAndAssign + BitOr<Output = Self> + BitOrAssign
        + BitXorAssign + Not<Output = Self>
        + Display + ToString
{
    const OVERFLOW: u8          = 0b0000_0001;
    const UNDERFLOW: u8         = 0b0000_0010;
    const INFINITY: u8          = 0b0000_0100;
    const DIVIDED_BY_ZERO: u8   = 0b0000_1000;

    fn to_string_with_radix(&self, radix: usize) -> String;
    fn divide_fully(&self, rhs: Self) -> (Self, Self);

    fn accumulate(&mut self, rhs: T);
    fn dissipate(&mut self, rhs: T);
    fn times(&mut self, rhs: T);
    fn divide_by_uint_fully(&self, rhs: T) -> (Self, T);
    fn quotient(&mut self, rhs: T);
    fn remainder(&mut self, rhs: T);

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

    fn set_flag_bit(&mut self, flag: u8);
    fn reset_flag_bit(&mut self, flag: u8);
    fn is_flag_bit_on(&self, flag: u8) -> bool;

    fn set_overflow(&mut self)      { self.set_flag_bit(Self::OVERFLOW); }
    fn reset_overflow(&mut self)    { self.reset_flag_bit(Self::OVERFLOW); }
    fn is_overflow(&self) -> bool   { self.is_flag_bit_on(Self::OVERFLOW) }
    fn set_underflow(&mut self)     { self.set_flag_bit(Self::UNDERFLOW); }
    fn reset_underflow(&mut self)   { self.reset_flag_bit(Self::UNDERFLOW); }
    fn is_underflow(&self) -> bool  { self.is_flag_bit_on(Self::UNDERFLOW) }
    fn set_infinity(&mut self)     { self.set_flag_bit(Self::INFINITY); }
    fn reset_inifinity(&mut self)   { self.reset_flag_bit(Self::INFINITY); }
    fn is_inifinity(&self) -> bool  { self.is_flag_bit_on(Self::INFINITY) }
    fn set_untrustable(&mut self)   { self.set_flag_bit(Self::OVERFLOW | Self::UNDERFLOW); }
    fn reset_untrustable(&mut self) { self.reset_flag_bit(Self::OVERFLOW | Self::UNDERFLOW); }
    fn is_untrustable(&self) -> bool { self.is_flag_bit_on(Self::OVERFLOW | Self::UNDERFLOW) }
    fn set_divided_by_zero(&mut self)   { self.set_flag_bit(Self::DIVIDED_BY_ZERO); }
    fn reset_divided_by_zero(&mut self) { self.reset_flag_bit(Self::DIVIDED_BY_ZERO); }
    fn is_divided_by_zero(&self) -> bool { self.is_flag_bit_on(Self::DIVIDED_BY_ZERO) }
}
