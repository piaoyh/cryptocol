// Copyright 2023 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! A module that contains a trait for big unsigned integer BigUInt, big signed
//! integer BigInt and large integer LargeInt.

use std::fmt::{ Display, Debug };
use std::cmp::{ PartialEq, PartialOrd, Ordering };
use std::ops::*;

use super::uint::*;

/// A trait for big unsigned integer BigUInt, big signed integer BigInt
/// and large integer LargeInt.
/// 
pub trait HugeInteger<T>
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
{
    /// A flag to represent whether or not overflow happened
    /// during previous operations. When divided-by-zero happens,
    /// the flags `DIVIDED_BY_ZERO`, `INFINITY` and `OVERFLOW` will be set.
    const OVERFLOW: u8          = 0b0000_0001;

    /// A flag to represent whether or not underflow happened
    /// during previous operations.
    const UNDERFLOW: u8         = 0b0000_0010;
    
    /// A flag to represent whether or not the value became extremely big
    /// for some reasons such as "divided by zero" during previous operations.
    /// When divided-by-zero happens,
    /// the flags `DIVIDED_BY_ZERO`, `INFINITY` and `OVERFLOW` will be set.
    const INFINITY: u8          = 0b0000_0100;

    /// A flag to represent whether or not divided-by-zero happened
    /// during previous operations. When divided-by-zero happens,
    /// the flags `DIVIDED_BY_ZERO`, `INFINITY` and `OVERFLOW` will be set.
    const DIVIDED_BY_ZERO: u8   = 0b0000_1000;

    /// Sets i-th element of its array of type `T` and return true if i < `N`.
    /// Otherwise, it sets none of the elements of its array of type `T` and
    /// returns false. `BigUInt` and `BigInt` have an array of `T` in order to
    /// present long-sized unsigned and signed integers, respectively.
    fn set_num(&mut self, i: usize, val: T) -> bool;

    /// Shows `BigInteger` such as `BigUInt`, `BigInt` or `LargeInt` in a string
    /// in order for a human to read. The number will be presented with the
    /// given radix in string. 
    fn to_string_with_radix(&self, radix: usize) -> String;

    /// Divides self which is of `BigUInt`, `BigInt` or `LargeInt` type by rhs
    /// which is of `BigUInt`, `BigInt`, or `LargeInt` type, and returns
    /// quotient and remainder which are `BigUInt`, `BigInt`, or `LargeInt`
    /// type, respectively.
    /// If rhs is zero, the divided_by_zero and overflow flags of quotient will
    /// be set, and the quotient will be set to be max value of `BigUInt`,
    /// `BigInt` or `LargeInt` type, respectively, and the remainder will be set
    /// to be zero of `BigUInt`, `BigInt` or `LargeInt` type, respectively.
    fn divide_fully(&self, rhs: Self) -> (Self, Self);

    /// Accumulates or adds rhs of type `T` to self which is of `BigUInt`,
    /// `BigInt` or `LargeInt` type.
    fn accumulate(&mut self, rhs: T);

    /// Dissipates or subtracts rhs of type `T` from self which is of
    /// `BigUInt`, `BigInt` or `LargeInt` type.
    fn dissipate(&mut self, rhs: T);

    /// Multiplies self which is of `BigUInt`, `BigInt` or `LargeInt` type with
    /// rhs of type `T`.
    fn times(&mut self, rhs: T);

    /// Divides self which is of `BigUInt`, `BigInt` or `LargeInt` type by rhs
    /// which is of type `T`, and returns quotient of `BigUInt`, `BigInt`, or
    /// `LargeInt` type, respectively, and remainder of type `T`.
    /// If rhs is zero, the divided_by_zero and overflow flags of quotient will
    /// be set, and the quotient will be set to be max value of `BigUInt`,
    /// `BigInt` or `LargeInt` type, respectively,
    /// and the remainder will be set and zero.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use Cryptocol::number::BigUInt;
    /// let dividend = u1024::from_string("1234567890157589425462369896");
    /// let divisor = T::num(87_u128);
    /// let (quotient, remainder) = dividend.divide_by_uint_fully(divisor);
    /// ```
    fn divide_by_uint_fully(&self, rhs: T) -> (Self, T);

    /// Divides self which is of `BigUInt`, `BigInt` or `LargeInt` type by rhs
    /// which is of type `T`, and returns quotient of `BigUInt`, `BigInt`, or
    /// `LargeInt` type, respectively. If you get both quotient and remainder,
    /// you'd better use the function divide_by_uint_fully() instead of calling
    /// the functions quotient() and remainder() in series because they call
    /// the function divide_by_uint_fully() internally.
    fn quotient(&mut self, rhs: T) -> Self;

    /// Divides self which is of `BigUInt`, `BigInt` or `LargeInt` type by rhs
    /// which is of type `T`, and returns remainder of type `T`, respectively.
    /// If you get both quotient and remainder, you'd better use the function
    /// divide_by_uint_fully() instead of calling the functions quotient() and
    /// remainder() in series because they call the function
    /// divide_by_uint_fully() internally.
    fn remainder(&mut self, rhs: T) -> T;

    fn set_flag_bit(&mut self, flag: u8);
    fn reset_flag_bit(&mut self, flag: u8);
    fn is_flag_bit_on(&self, flag: u8) -> bool;

    fn set_infinity(&mut self)     { self.set_flag_bit(Self::INFINITY); }
    fn reset_inifinity(&mut self)   { self.reset_flag_bit(Self::INFINITY); }
    fn is_inifinity(&self) -> bool  { self.is_flag_bit_on(Self::INFINITY) }
    fn set_divided_by_zero(&mut self)   { self.set_flag_bit(Self::DIVIDED_BY_ZERO); }
    fn reset_divided_by_zero(&mut self) { self.reset_flag_bit(Self::DIVIDED_BY_ZERO); }
    fn is_divided_by_zero(&self) -> bool { self.is_flag_bit_on(Self::DIVIDED_BY_ZERO) }
}
