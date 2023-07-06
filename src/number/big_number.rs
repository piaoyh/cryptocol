// Copyright 2023 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module that contains a trait for big unsigned integer
//! and for big signed integer with user-defined fixed size.

use std::fmt::{ Display, Debug };
use std::cmp::{ PartialEq, PartialOrd, Ordering };
use std::ops::*;

use super::uint::*;
use super::HugeInteger;

/// A trait for big unsigned integer and big signed integer with user-defined fixed size.
/// 
pub trait BigInteger<T, const N: usize>
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
    /// Returns i-th element of its array of type T wrapped in Some if i < N.
    /// Otherwise, it returns None. BigUInt and BigInt have an array of type T
    /// in order to present long-sized unsigned and signed integers, respectively.
    fn get_num(&self, i: usize) -> Option<T>
    {
        if i < N
            { Some(self.get_number()[i]) }
        else
            { None }
    }

    /// Returns the reference of its array of T-type for borrowing instead
    /// of giving its ownership. BigUInt and BigInt have an array of T in order
    /// to present long-sized unsigned and signed integers, respectively.
    fn get_number(&self) -> &[T; N];

    /// Sets the contents of its array of T-type. The argument val is the
    /// reference of array of type T with the length N. BigUInt and BigInt
    /// have an array of T in order to present long-sized unsigned and signed
    /// integers, respectively.
    fn set_number(&mut self, val: &[T; N]);

    /// Sets BigUInt or BigInt to be zero.
    fn set_zero(&mut self)
    {
        for i in 0..N
            { self.set_num(i, T::zero()); }
    }

    /// Checks whether BigUInt or BigInt to be zero and returns true if it is
    /// zero and returns false if it is not zero. 
    fn is_zero(&self) -> bool
    {
        for i in 0..N
        {
            if self.get_num(i).unwrap() != T::zero()
                { return false; }
        }
        true
    }

    /// Sets BigUInt or BigInt to be one.
    #[cfg(target_endian = "little")]
    fn set_one(&mut self)
    {
        for i in 1..N
            { self.set_num(i, T::zero()); }
        self.set_num(0, T::one());
    }

    /// Sets BigUInt or BigInt to be one.
    #[cfg(target_endian = "big")]
    fn set_one(&mut self)
    {
        for i in 0..N-1
            { self.set_num(i, T::zero()); }
        self.set_num(N-1, T::one());
    }

    /// Compares BigUInt or BigInt with a value of type T and returns the
    /// result of the comparison in the type `Option<Ordering>`. However, you'd
    /// better use the functions lt_uint(), gt_uint(), le_uint(), ge_uint(),
    /// and eq_uint(). Then, you don't have to use partial_cmp_uint() directly.
    fn partial_cmp_uint(&self, other: T) -> Option<Ordering>;

    /// Returns true if self is less than other. Otherwise, it returns false.
    fn lt_uint(&self, other: T) -> bool  { self.partial_cmp_uint(other).unwrap().is_lt() }

    /// Returns true if self is greater than other. Otherwise, it returns false.
    fn gt_uint(&self, other: T) -> bool  { self.partial_cmp_uint(other).unwrap().is_gt() }

    /// Returns true if self is less than or equal to other.
    /// Otherwise, it returns false.
    fn le_uint(&self, other: T) -> bool  { self.partial_cmp_uint(other).unwrap().is_le() }

    /// Returns true if self is greater than or equal to other.
    /// Otherwise, it returns false.
    fn ge_uint(&self, other: T) -> bool  { self.partial_cmp_uint(other).unwrap().is_ge() }

    /// Returns true if self is equal to other. Otherwise, it returns false.
    fn eq_uint(&self, other: T) -> bool  { self.partial_cmp_uint(other).unwrap().is_eq() }

    /// Sets overflow flag.
    fn set_overflow(&mut self)      { self.set_flag_bit(Self::OVERFLOW); }

    /// Resets overflow flag.
    fn reset_overflow(&mut self)    { self.reset_flag_bit(Self::OVERFLOW); }

    /// Checks whether or not overflow flag is set, and returns true
    /// if the overflow flag is set. Otherwise, it returns false.
    fn is_overflow(&self) -> bool   { self.is_flag_bit_on(Self::OVERFLOW) }

    /// Sets underflow flag.
    fn set_underflow(&mut self)     { self.set_flag_bit(Self::UNDERFLOW); }

    /// Reets underflow flag.
    fn reset_underflow(&mut self)   { self.reset_flag_bit(Self::UNDERFLOW); }

    /// Checks whether or not underflow flag is set, and returns true
    /// if the underflow flag is set. Otherwise, it returns false.
    fn is_underflow(&self) -> bool  { self.is_flag_bit_on(Self::UNDERFLOW) }

    /// Sets both overflow flag and underflow flag.
    fn set_untrustable(&mut self)   { self.set_flag_bit(Self::OVERFLOW | Self::UNDERFLOW); }

    /// Resets both overflow flag and underflow flag.
    fn reset_untrustable(&mut self) { self.reset_flag_bit(Self::OVERFLOW | Self::UNDERFLOW); }

    /// Checks whether or not both overflow flag and underflow flag are all set,
    /// and returns true if both of the overflow flag and the underflow flag
    /// are all set. Otherwise, it returns false.
    fn is_untrustable(&self) -> bool { self.is_flag_bit_on(Self::OVERFLOW | Self::UNDERFLOW) }
}
