// Copyright 2023 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module that contains implementation of external traits.

//#![warn(missing_docs)]
//#![warn(missing_doc_code_examples)]
use std::fmt::{ self, Display, Formatter, Debug };
use std::mem::{ size_of, size_of_val, transmute };
use std::cmp::{ PartialEq, PartialOrd, Ordering };
use std::convert::{ From, Into };
use std::str::FromStr;
use std::ops::*;

use super::uint::*;
use super::sint::*;
use super::big_uint::BigUInt;
use super::NumberErr;



impl<T, const N: usize> Add for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self
    {
        self.wrapping_add(rhs)
    }
}



impl<T, const N: usize> Add<T> for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: T) -> Self
    {
        self.add_uint(rhs)
    }
}



impl<T, const N: usize> AddAssign for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /// Adds and assign the result to it.
    /// 
    #[inline]
    fn add_assign(&mut self, rhs: Self)
    {
        self.wrapping_add_assign(rhs);
    }
}



impl<T, const N: usize> AddAssign<T> for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /// Adds and assign the result to it.
    /// 
    #[inline]
    fn add_assign(&mut self, rhs: T)
    {
        self.accumulate(rhs);
    }
}



impl<T, const N: usize> Sub for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self
    {
        self.wrapping_sub(rhs)
    }
}




impl<T, const N: usize> Sub<T> for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: T) -> Self
    {
        self.sub_uint(rhs)
    }
}



impl<T, const N: usize> SubAssign for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /// Subtracts and assign the result to it.
    #[inline]
    fn sub_assign(&mut self, rhs: Self)
    {
        self.wrapping_sub_assign(rhs);
    }
}



impl<T, const N: usize> SubAssign<T> for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /// Subtracts and assign the result to it.
    #[inline]
    fn sub_assign(&mut self, rhs: T)
    {
        self.dissipate(rhs);
    }
}



impl<T, const N: usize> Mul for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self
    {
        self.wrapping_mul(rhs)
    }
}



impl<T, const N: usize> Mul<T> for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self
    {
        self.mul_uint(rhs)
    }
}



impl<T, const N: usize> MulAssign for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /// Multiplies and assign the result to it.
    #[inline] 
    fn mul_assign(&mut self, rhs: Self)
    {
        self.wrapping_mul_assign(rhs);
    }
}


impl<T, const N: usize> MulAssign<T> for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /// Multiplies and assign the result to it.
    #[inline]
    fn mul_assign(&mut self, rhs: T)
    {
        self.times(rhs);
    }
}



impl<T, const N: usize> Div for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self
    {
        self.wrapping_div(rhs)
    }
}



impl<T, const N: usize> Div<T> for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self
    {
        self.div_uint(rhs)
    }
}



impl<T, const N: usize> DivAssign for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    #[inline]
    fn div_assign(&mut self, rhs: Self)
    {
        self.wrapping_div_assign(rhs);
    }
}



impl<T, const N: usize> DivAssign<T> for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    #[inline]
    fn div_assign(&mut self, rhs: T)
    {
        self.quotient(rhs);
    }
}


impl<T, const N: usize> Rem for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    #[inline]
    fn rem(self, rhs: Self) -> Self
    {
        self.wrapping_rem(rhs)
    }
}



impl<T, const N: usize> Rem<T> for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = T;

    #[inline]
    fn rem(self, rhs: T) -> T
    {
        self.rem_uint(rhs)
    }
}



impl<T, const N: usize> RemAssign for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    #[inline]
    fn rem_assign(&mut self, rhs: Self)
    {
        self.wrapping_rem_assign(rhs);
    }
}



impl<T, const N: usize> RemAssign<T> for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    #[inline]
    fn rem_assign(&mut self, rhs: T)
    {
        self.remainder(rhs);
    }
}



macro_rules! shl_for_BigUInt_impl {
    ($f:ty) => {
        /// Performs the << operation. If overflow happens during the << operation,
        /// `OVERFLOW` flag is set and the method is_overflow() will return true. 
        /// [Read more](https://doc.rust-lang.org/core/ops/bit/trait.Shl.html#tymethod.shl)
        /// 
        /// # Example 1
        /// ```
        /// use std::str::FromStr;
        /// use Cryptocol::number::*;
        /// use Cryptocol::define_utypes_with;
        /// define_utypes_with!(u128);
        /// let a = u256::from_str("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
        /// let b = a << 4_i128;
        /// println!("b = {}\noverflow: {}", b, b.is_overflow());
        /// assert_eq!(b.is_overflow(), true);
        /// ```
        /// 
        /// # Example 2
        /// ```
        /// use std::str::FromStr;
        /// use Cryptocol::number::*;
        /// use Cryptocol::define_utypes_with;
        /// define_utypes_with!(u128);
        /// let a = u256::from_str("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
        /// let b = a << 1_i8;
        /// println!("b = {}\noverflow: {}", b, b.is_overflow());
        /// assert_eq!(b.is_overflow(), false);
        /// ```
        impl<T, const N: usize> Shl<$f> for BigUInt<T, N>
        where T: Uint + Clone + Display + Debug + ToString
                + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
                + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
                + Rem<Output=T> + RemAssign
                + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
                + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
                + BitXor<Output=T> + BitXorAssign + Not<Output=T>
                + PartialEq + PartialOrd
        {
            type Output = Self;

            fn shl(self, rhs: $f) -> Self
            {
                let mut s = self.clone();
                s <<= rhs;
                s
            }
        }
    }
}



macro_rules! shlassign_i_for_BigUInt_impl {
    ($f:ty) => {
        impl<T, const N: usize> ShlAssign<$f> for BigUInt<T, N>
        where T: Uint + Clone + Display + Debug + ToString
                + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
                + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
                + Rem<Output=T> + RemAssign
                + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
                + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
                + BitXor<Output=T> + BitXorAssign + Not<Output=T>
                + PartialEq + PartialOrd
        {
            /// Performs the <<= operation. If overflow happens during the <<= operation,
            /// `OVERFLOW` flag is set and the method is_overflow() will return true. 
            /// [Read more](https://doc.rust-lang.org/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)
            ///
            /// # Example 1
            /// ```
            /// use std::str::FromStr;
            /// use Cryptocol::number::*;
            /// use Cryptocol::define_utypes_with;
            /// define_utypes_with!(u128);
            /// let mut a = u256::from_str("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
            /// a <<= 4_i128;
            /// println!("a = {}\noverflow: {}", a, a.is_overflow());
            /// assert_eq!(a.is_overflow(), true);
            /// ```
            /// 
            /// # Example 2
            /// ```
            /// use std::str::FromStr;
            /// use Cryptocol::number::*;
            /// use Cryptocol::define_utypes_with;
            /// define_utypes_with!(u128);
            /// let mut a = u256::from_str("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
            /// a <<= 1_i8;
            /// println!("a = {}\noverflow: {}", a, a.is_overflow());
            /// assert_eq!(a.is_overflow(), false);
            /// ```
            /// 
            /// # Big-endian issue
            /// It is just experimental for Big Endian CPUs. So, you are not encouraged
            /// to use it for serious purpose. Only use this crate for Big-endian CPUs
            /// with your own full responsibility.
            fn shl_assign(&mut self, rhs: $f)
            {
                if rhs < 0
                {
                    match size_of_val(&rhs)
                    {
                        1 => { *self >>= (-rhs as u8); },
                        2 => { *self >>= (-rhs as u16); },
                        4 => { *self >>= (-rhs as u32); },
                        8 => { *self >>= (-rhs as u64); },
                        16 => { *self >>= (-rhs as u128); },
                        _ => {},
                    }
                }
                else
                {
                    match size_of_val(&rhs)
                    {
                        1 => { *self <<= (rhs as u8); },
                        2 => { *self <<= (rhs as u16); },
                        4 => { *self <<= (rhs as u32); },
                        8 => { *self <<= (rhs as u64); },
                        16 => { *self <<= (rhs as u128); },
                        _ => {},
                    }
                }
            }
        }
    }
}



macro_rules! shlassign_u_for_BigUInt_impl {
    ($f:ty) => {
        impl<T, const N: usize> ShlAssign<$f> for BigUInt<T, N>
        where T: Uint + Clone + Display + Debug + ToString
                + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
                + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
                + Rem<Output=T> + RemAssign
                + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
                + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
                + BitXor<Output=T> + BitXorAssign + Not<Output=T>
                + PartialEq + PartialOrd
        {
            /// Performs the <<= operation. If overflow happens during the <<= operation,
            /// `OVERFLOW` flag is set and the method is_overflow() will return true. 
            /// [Read more](https://doc.rust-lang.org/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)
            ///
            /// # Example 1
            /// ```
            /// use std::str::FromStr;
            /// use Cryptocol::number::*;
            /// use Cryptocol::define_utypes_with;
            /// define_utypes_with!(u128);
            /// let mut a = u256::from_str("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
            /// a <<= 4_i128;
            /// println!("a = {}\noverflow: {}", a, a.is_overflow());
            /// assert_eq!(a.is_overflow(), true);
            /// ```
            /// 
            /// # Example 2
            /// ```
            /// use std::str::FromStr;
            /// use Cryptocol::number::*;
            /// use Cryptocol::define_utypes_with;
            /// define_utypes_with!(u128);
            /// let mut a = u256::from_str("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
            /// a <<= 1_i8;
            /// println!("a = {}\noverflow: {}", a, a.is_overflow());
            /// assert_eq!(a.is_overflow(), false);
            /// ```
            /// 
            /// # Big-endian issue
            /// It is just experimental for Big Endian CPUs. So, you are not encouraged
            /// to use it for serious purpose. Only use this crate for Big-endian CPUs
            /// with your own full responsibility.
            #[inline]
            fn shl_assign(&mut self, rhs: $f)
            {
                self.shift_left_assign(rhs);
                /*
                let TSIZE_IN_BITS = T::size_in_bits();
                let chunk_num = (rhs / TSIZE_IN_BITS as $f) as usize;
                let piece_num = (rhs % TSIZE_IN_BITS as $f) as usize;
                let zero = T::zero();
                self.reset_all_flags();
                if chunk_num > 0
                {
                    for i in N-chunk_num..N
                    {
                        if self.get_num_(i) > zero
                        {
                            self.set_overflow();
                            break;
                        }
                    }
                    self.copy_within(0..N-chunk_num, chunk_num);
                    for idx in 0..chunk_num
                        { self.set_num_(idx, zero); }
                }
                if piece_num == 0
                    { return; }
                if (self.get_num_(N-1) >> T::usize_as_Uint(TSIZE_IN_BITS - piece_num)) != zero
                    { self.set_overflow(); }

                let mut num: T;
                let mut carry = zero;
                for idx in chunk_num..N
                {
                    num = (self.get_num_(idx) << T::usize_as_Uint(piece_num) | carry;
                    carry = self.get_num_(idx) >> T::usize_as_Uint(TSIZE_IN_BITS - piece_num);
                    self.set_num_(idx, num);
                }
                if carry != zero
                    { self.set_overflow(); }*/
            }
        }
    }
}



shl_for_BigUInt_impl! { i8 }
shl_for_BigUInt_impl! { i16 }
shl_for_BigUInt_impl! { i32 }
shl_for_BigUInt_impl! { i64 }
shl_for_BigUInt_impl! { i128 }
shl_for_BigUInt_impl! { isize }
shl_for_BigUInt_impl! { u8 }
shl_for_BigUInt_impl! { u16 }
shl_for_BigUInt_impl! { u32 }
shl_for_BigUInt_impl! { u64 }
shl_for_BigUInt_impl! { u128 }
shl_for_BigUInt_impl! { usize }
shlassign_i_for_BigUInt_impl! { i8 }
shlassign_i_for_BigUInt_impl! { i16 }
shlassign_i_for_BigUInt_impl! { i32 }
shlassign_i_for_BigUInt_impl! { i64 }
shlassign_i_for_BigUInt_impl! { i128 }
shlassign_i_for_BigUInt_impl! { isize }
shlassign_u_for_BigUInt_impl! { u8 }
shlassign_u_for_BigUInt_impl! { u16 }
shlassign_u_for_BigUInt_impl! { u32 }
shlassign_u_for_BigUInt_impl! { u64 }
shlassign_u_for_BigUInt_impl! { u128 }
shlassign_u_for_BigUInt_impl! { usize }



macro_rules! shr_for_BigUInt_impl {
    ($f:ty) => {
        impl<T, const N: usize> Shr<$f> for BigUInt<T, N>
        where T: Uint + Clone + Display + Debug + ToString
                + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
                + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
                + Rem<Output=T> + RemAssign
                + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
                + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
                + BitXor<Output=T> + BitXorAssign + Not<Output=T>
                + PartialEq + PartialOrd
        {
            type Output = Self;

            /// Performs the >> operation. If underflow happens during the >> operation,
            /// `UNDERFLOW` flag is set and the method is_underflow() will return true.
            /// Here, 'underflow' means that none-zero part is shifted out to the right.
            /// [Read more](https://doc.rust-lang.org/core/ops/bit/trait.Shr.html#tymethod.shr)
            /// 
            /// # Example 1
            /// ```
            /// use std::str::FromStr;
            /// use Cryptocol::number::*;
            /// use Cryptocol::define_utypes_with;
            /// define_utypes_with!(u128);
            /// let a = u256::from_str("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
            /// let b = a >> 2;
            /// println!("b = {}\nunderflow: {}", b, b.is_underflow());
            /// assert_eq!(b.is_underflow(), true);
            /// ```
            /// 
            /// # Example 2
            /// ```
            /// use std::str::FromStr;
            /// use Cryptocol::number::*;
            /// use Cryptocol::define_utypes_with;
            /// define_utypes_with!(u128);
            /// let a = u256::from_str("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
            /// let b = a >> 1;
            /// println!("b = {}\nunderflow: {}", b, b.is_underflow());
            /// assert_eq!(b.is_underflow(), false);
            /// ```
            fn shr(self, rhs: $f) -> Self
            {
                let mut s = self.clone();
                s >>= rhs;
                s
            }
        }
    }
}



macro_rules! shrassign_i_for_BigUInt_impl {
    ($f:ty) => {
        impl<T, const N: usize> ShrAssign<$f> for BigUInt<T, N>
        where T: Uint + Clone + Display + Debug + ToString
                + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
                + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
                + Rem<Output=T> + RemAssign
                + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
                + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
                + BitXor<Output=T> + BitXorAssign + Not<Output=T>
                + PartialEq + PartialOrd
        {
            /// Performs the >>= operation. If underflow happens during the >>= operation,
            /// `UNDERFLOW` flag is set and the method is_underflow() will return true.
            /// Here, 'underflow' means that none-zero part is shifted out to the right.
            /// [Read more](https://doc.rust-lang.org/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)
            /// 
            /// # Big-endian issue
            /// It is just experimental for Big Endian CPUs. So, you are not encouraged
            /// to use it for serious purpose. Only use this crate for Big-endian CPUs
            /// with your own full responsibility.
            /// 
            /// # Example 1
            /// ```
            /// use std::str::FromStr;
            /// use Cryptocol::number::*;
            /// use Cryptocol::define_utypes_with;
            /// define_utypes_with!(u128);
            /// let mut a = u256::from_str("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
            /// a >>= 2_i128;
            /// println!("a = {}\nunderflow: {}", a, a.is_underflow());
            /// assert_eq!(a.is_underflow(), true);
            /// ```
            /// 
            /// # Example 2
            /// ```
            /// use std::str::FromStr;
            /// use Cryptocol::number::*;
            /// use Cryptocol::define_utypes_with;
            /// define_utypes_with!(u128);
            /// let mut a = u256::from_str("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
            /// a >>= 1_i8;
            /// println!("a = {}\nunderflow: {}", a, a.is_underflow());
            /// assert_eq!(a.is_underflow(), false);
            /// ```
            fn shr_assign(&mut self, rhs: $f)
            {
                if rhs < 0
                {
                    match size_of_val(&rhs)
                    {
                        1 => { *self <<= (-rhs as u8); },
                        2 => { *self <<= (-rhs as u16); },
                        4 => { *self <<= (-rhs as u32); },
                        8 => { *self <<= (-rhs as u64); },
                        16 => { *self <<= (-rhs as u128); },
                        _ => {},
                    }
                }
                else
                {
                    match size_of_val(&rhs)
                    {
                        1 => { *self >>= (rhs as u8); },
                        2 => { *self >>= (rhs as u16); },
                        4 => { *self >>= (rhs as u32); },
                        8 => { *self >>= (rhs as u64); },
                        16 => { *self >>= (rhs as u128); },
                        _ => {},
                    }
                }
            }
        }
    }
}


macro_rules! shrassign_u_for_BigUInt_impl {
    ($f:ty) => {
        impl<T, const N: usize> ShrAssign<$f> for BigUInt<T, N>
        where T: Uint + Clone + Display + Debug + ToString
                + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
                + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
                + Rem<Output=T> + RemAssign
                + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
                + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
                + BitXor<Output=T> + BitXorAssign + Not<Output=T>
                + PartialEq + PartialOrd
        {
            /// Performs the >>= operation. If underflow happens during the >>= operation,
            /// `UNDERFLOW` flag is set and the method is_underflow() will return true.
            /// Here, 'underflow' means that none-zero part is shifted out to the right.
            /// [Read more](https://doc.rust-lang.org/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)
            /// 
            /// # Big-endian issue
            /// It is just experimental for Big Endian CPUs. So, you are not encouraged
            /// to use it for serious purpose. Only use this crate for Big-endian CPUs
            /// with your own full responsibility.
            /// 
            /// # Example 1
            /// ```
            /// use std::str::FromStr;
            /// use Cryptocol::number::*;
            /// use Cryptocol::define_utypes_with;
            /// define_utypes_with!(u128);
            /// let mut a = u256::from_str("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
            /// a >>= 2_i128;
            /// println!("a = {}\nunderflow: {}", a, a.is_underflow());
            /// assert_eq!(a.is_underflow(), true);
            /// ```
            /// 
            /// # Example 2
            /// ```
            /// use std::str::FromStr;
            /// use Cryptocol::number::*;
            /// use Cryptocol::define_utypes_with;
            /// define_utypes_with!(u128);
            /// let mut a = u256::from_str("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
            /// a >>= 1_i8;
            /// println!("a = {}\nunderflow: {}", a, a.is_underflow());
            /// assert_eq!(a.is_underflow(), false);
            /// ```
            #[inline]
            fn shr_assign(&mut self, rhs: $f)
            {
                self.shift_right_assign(rhs);
                /*
                let TSIZE_IN_BITS = T::size_in_bits();
                let chunk_num = (rhs / TSIZE_IN_BITS as $f) as usize;
                let piece_num = (rhs % TSIZE_IN_BITS as $f) as usize;
                let zero = T::zero();
                self.reset_all_flags();
                if chunk_num > 0
                {
                    for i in 0..chunk_num
                    {
                        if self.get_num_(i) > zero
                        {
                            self.set_underflow();
                            break;
                        }
                    }
                    self.copy_within(chunk_num..N, 0);
                    for idx in N-chunk_num..N
                        { self.set_num_(idx, zero); }
                }
                if piece_num == 0
                    { return; }
                if (self.get_num_(0) << T::usize_as_Uint(TSIZE_IN_BITS - piece_num)) != zero
                    { self.set_underflow(); }

                let mut num: T;
                let mut carry = T::zero();
                let mut idx = N - 1 - chunk_num;
                loop
                {
                    num = (self.get_num_(idx) >> T::usize_as_Uint(piece_num)) | carry;
                    carry = self.get_num_(idx) << T::usize_as_Uint(TSIZE_IN_BITS - piece_num);
                    self.set_num_(idx, num);
                    if idx == 0
                        { break; }
                    idx -= 1;
                }
                if carry != zero
                    { self.set_underflow(); }*/
            }
        }
    }
}



shr_for_BigUInt_impl! { i8 }
shr_for_BigUInt_impl! { i16 }
shr_for_BigUInt_impl! { i32 }
shr_for_BigUInt_impl! { i64 }
shr_for_BigUInt_impl! { i128 }
shr_for_BigUInt_impl! { isize }
shr_for_BigUInt_impl! { u8 }
shr_for_BigUInt_impl! { u16 }
shr_for_BigUInt_impl! { u32 }
shr_for_BigUInt_impl! { u64 }
shr_for_BigUInt_impl! { u128 }
shr_for_BigUInt_impl! { usize }
shrassign_i_for_BigUInt_impl! { i8 }
shrassign_i_for_BigUInt_impl! { i16 }
shrassign_i_for_BigUInt_impl! { i32 }
shrassign_i_for_BigUInt_impl! { i64 }
shrassign_i_for_BigUInt_impl! { i128 }
shrassign_i_for_BigUInt_impl! { isize }
shrassign_u_for_BigUInt_impl! { u8 }
shrassign_u_for_BigUInt_impl! { u16 }
shrassign_u_for_BigUInt_impl! { u32 }
shrassign_u_for_BigUInt_impl! { u64 }
shrassign_u_for_BigUInt_impl! { u128 }
shrassign_u_for_BigUInt_impl! { usize }



impl<T, const N: usize> BitAnd for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    /// Performs the & operation.
    /// [Read more](https://doc.rust-lang.org/core/ops/bit/trait.BitAnd.html#tymethod.bitand)
    /// 
    /// # Examples
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::from_str_radix("11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000", 2).unwrap();
    /// let c = a & b;
    /// println!("a = {}", a.to_string_with_radix(2));
    /// println!("b = {}", b.to_string_with_radix(2));
    /// println!("a & b = {}", c.to_string_with_radix(2));
    /// assert_eq!(c, a & b);
    /// ```
    /// You have to import (use) Cryptocol::number::HugeInteger in order to use
    /// its method to_string_with_radix(). If you find headaching to remember
    /// what you should import, you can just import everything
    /// (Cryptocol::number::*) as next example. It is not harmful.
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::zero();
    /// let c = a & b;
    /// println!("a = {}", a.to_string_with_radix(2));
    /// println!("b = {}", b.to_string_with_radix(2));
    /// println!("a & b = {}", c.to_string_with_radix(2));
    /// assert_eq!(c, a & b);
    /// ```
    fn bitand(self, rhs: Self) -> Self
    {
        let mut s = self.clone();
        s &= rhs;
        s
    }
}



impl<T, const N: usize> BitAndAssign for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /// Performs the &= operation.
    /// [Read more](https://doc.rust-lang.org/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)
    /// 
    /// # Examples
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let mut a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::from_str_radix("11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000", 2).unwrap();
    /// a &= b;
    /// println!("a = {}", a.to_string_with_radix(2));
    /// assert_eq!(a, u256::from_str_radix("1111000000000000110000000000001110001000000100011010101000000000111100000000000011000000000000111000100000010001101010100000000011110000000000001100000000000011100010000001000110101010000000001111000000000000110000000000001110001000000100011010101000000000", 2).unwrap());
    /// ```
    /// You have to import (use) Cryptocol::number::* and then import
    /// Cryptocol::define_utypes_with in order to use the macro define_utypes_with
    /// and its method to_string_with_radix(). If you find headaching to remember
    /// what you should import, you can just import everything
    /// (Cryptocol::number::*) as next example. It is not harmful.
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let mut a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::zero();
    /// a &= b;
    /// println!("a = {}", a.to_string_with_radix(2));
    /// assert_eq!(a, u256::zero());
    /// ```
    fn bitand_assign(&mut self, rhs: Self)
    {
        let mut n: T;
        for idx in 0..N
        {
            n = self.get_num_(idx) & rhs.get_num_(idx);
            self.set_num_(idx, n);
        }
    }
}



impl<T, const N: usize> BitOr for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    /// Performs the | operation.
    /// [Read more](https://doc.rust-lang.org/core/ops/bit/trait.BitOr.html#tymethod.bitor)
    /// 
    /// # Examples
    /// ```
    /// use Cryptocol::number::HugeInteger;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::from_str_radix("11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000", 2).unwrap();
    /// let c = a | b;
    /// println!("a = {}", a.to_string_with_radix(2));
    /// println!("b = {}", b.to_string_with_radix(2));
    /// println!("a | b = {}", c.to_string_with_radix(2));
    /// assert_eq!(c, a | b);
    /// ```
    /// You have to import (use) Cryptocol::number::HugeInteger in order to use
    /// its method to_string_with_radix(). If you find headaching to remember
    /// what you should import, you can just import everything
    /// (Cryptocol::number::*) as next example. It is not harmful.
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::max();
    /// let c = a | b;
    /// println!("a = {}", a.to_string_with_radix(2));
    /// println!("b = {}", b.to_string_with_radix(2));
    /// println!("a | b = {}", c.to_string_with_radix(2));
    /// assert_eq!(c, u256::max());
    /// ```
    fn bitor(self, rhs: Self) -> Self
    {
        let mut s = self.clone();
        s |= rhs;
        s
    }
}



impl<T, const N: usize> BitOrAssign for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /// Performs the |= operation.
    /// [Read more](https://doc.rust-lang.org/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)
    /// 
    /// # Examples
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let mut a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::from_str_radix("11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000", 2).unwrap();
    /// a |= b;
    /// println!("a = {}", a.to_string_with_radix(2));
    /// assert_eq!(a, u256::from_str_radix("1111111100001111111111000011111111101110011101111111111101010101111111110000111111111100001111111110111001110111111111110101010111111111000011111111110000111111111011100111011111111111010101011111111100001111111111000011111111101110011101111111111101010101", 2).unwrap());
    /// ```
    /// You have to import (use) Cryptocol::number::HugeInteger in order to use
    /// its method to_string_with_radix(). If you find headaching to remember
    /// what you should import, you can just import everything
    /// (Cryptocol::number::*) as next example. It is not harmful.
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let mut a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::max();
    /// a |= b;
    /// println!("a = {}", a.to_string_with_radix(2));
    /// assert_eq!(a, u256::max());
    /// ```
    fn bitor_assign(&mut self, rhs: Self)
    {
        let mut n: T;
        for idx in 0..N
        {
            n = self.get_num_(idx) | rhs.get_num_(idx);
            self.set_num_(idx, n);
        }
    }
}



impl<T, const N: usize> BitXor for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    /// Performs the ^ operation.
    /// [Read more](https://doc.rust-lang.org/core/ops/bit/trait.BitXor.html#tymethod.bitxor)
    /// 
    /// # Examples
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::from_str_radix("11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000", 2).unwrap();
    /// let c = a ^ b;
    /// println!("a = {}", a.to_string_with_radix(2));
    /// println!("b = {}", b.to_string_with_radix(2));
    /// println!("a ^ b = {}", c.to_string_with_radix(2));
    /// assert_eq!(c, a ^ b);
    /// ```
    /// You have to import (use) Cryptocol::number::u256 in order to use the
    /// type u256 and import Cryptocol::number::HugeInteger in order to use
    /// its method to_string_with_radix(). If you find headaching to remember
    /// what you should import, you can just import everything
    /// (Cryptocol::number::*) as next example. It is not harmful.
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::max();
    /// let c = a ^ b;
    /// println!("a = {}", a.to_string_with_radix(2));
    /// println!("b = {}", b.to_string_with_radix(2));
    /// println!("a ^ b = {}", c.to_string_with_radix(2));
    /// assert_eq!(c, !a);
    /// ```    
    fn bitxor(self, rhs: Self) -> Self
    {
        let mut s = self.clone();
        s ^= rhs;
        s
    }
}



impl<T, const N: usize> BitXorAssign for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /// Performs the ^= operation.
    /// [Read more](https://doc.rust-lang.org/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)
    /// 
    /// # Examples
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let mut a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::from_str_radix("11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000", 2).unwrap();
    /// a ^= b;
    /// println!("a = {}", a.to_string_with_radix(2));
    /// assert_eq!(a, u256::from_str_radix("111100001111001111000011110001100110011001100101010101010101000011110000111100111100001111000110011001100110010101010101010100001111000011110011110000111100011001100110011001010101010101010000111100001111001111000011110001100110011001100101010101010101", 2).unwrap());
    /// ```
    /// You have to import (use) Cryptocol::number::HugeInteger in order to use
    /// its method to_string_with_radix(). If you find headaching to remember
    /// what you should import, you can just import everything
    /// (Cryptocol::number::*) as next example. It is not harmful.
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let mut a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::max();
    /// a ^= b;
    /// println!("a = {}", a.to_string_with_radix(2));
    /// assert_eq!(a, u256::from_str_radix("11111111000011111111000000110011110011000101010110101010000000001111111100001111111100000011001111001100010101011010101000000000111111110000111111110000001100111100110001010101101010100000000011111111000011111111000000110011110011000101010110101010", 2).unwrap());
    /// ```
    fn bitxor_assign(&mut self, rhs: Self)
    {
        let mut n: T;
        for idx in 0..N
        {
            n = self.get_num_(idx) ^ rhs.get_num_(idx);
            self.set_num_(idx, n);
        }
    }
}



impl<T, const N: usize> Not for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    /// Performs the unary ! operation.
    /// [Read more](https://doc.rust-lang.org/core/ops/bit/trait.Not.html#tymethod.not)
    /// 
    /// # Examples
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let c = !a;
    /// println!("c = {}", c.to_string_with_radix(2));
    /// assert_eq!(!c, a);
    /// ```
    /// You have to import (use) Cryptocol::number::HugeInteger in order to use
    /// its method to_string_with_radix(). If you find headaching to remember
    /// what you should import, you can just import everything
    /// (Cryptocol::number::*) as next example. It is not harmful.
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let c = !a | a;
    /// println!("c = {}", c.to_string_with_radix(2));
    /// assert_eq!(c, u256::max());
    /// ```    
    fn not(self) -> Self
    {
        let mut s = self.clone();
        let mut n: T;
        for idx in 0..N
        {
            n = !s.get_num_(idx);
            s.set_num_(idx, n);
        }
        s
    }
}



impl<T, const N: usize> PartialEq for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    fn eq(&self, other: &Self) -> bool
    {
        for idx in 0..N
        {
            if self.get_num_(idx) != other.get_num_(idx)
                { return false; }
        }
        true
    }
}



impl<T, const N: usize> PartialOrd for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    #[cfg(target_endian = "little")]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        let mut idx = N - 1;
        loop
        {
            if self.get_num_(idx) > other.get_num_(idx)
                { return Some(Ordering::Greater); }
            else if self.get_num_(idx) < other.get_num_(idx)
                { return Some(Ordering::Less); }
            if idx == 0
                { break; }
            idx -= 1;
        }
        Some(Ordering::Equal)
    }

    #[cfg(target_endian = "big")]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        for idx in 0..N
        {
            if self.get_num_(idx) > other.get_num_(idx)
                { return Some(Ordering::Greater); }
            else if self.get_num_(idx) < other.get_num_(idx)
                { return Some(Ordering::Less); }
        }
        Some(Ordering::Equal)
    }
}



impl<T, const N: usize> Display for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /// Formats the value using the given formatter.
    /// Automagically the function `to_string()` will be implemented. So, you
    /// can use the function `to_string()` and the macro `println!()`.
    /// `f` is a buffer, this method must write the formatted string into it.
    /// [Read more](https://doc.rust-lang.org/core/fmt/trait.Display.html#tymethod.fmt)
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let a = u256::from_str("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
    /// println!("{}", a);
    /// ```
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)
        write!(f, "{}", self.to_string_with_radix(10).unwrap())
    }
}



impl<T, const N: usize, S> From<S> for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd,
    S: Uint + Clone + Display + Debug + ToString
        + Add<Output=S> + AddAssign + Sub<Output=S> + SubAssign
        + Mul<Output=S> + MulAssign + Div<Output=S> + DivAssign
        + Rem<Output=S> + RemAssign
        + Shl<Output=S> + ShlAssign + Shr<Output=S> + ShrAssign
        + BitAnd<Output=S> + BitAndAssign + BitOr<Output=S> + BitOrAssign
        + BitXor<Output=S> + BitXorAssign + Not<Output=S>
        + PartialEq + PartialOrd
{
    /// Constructs a new `BigUInt<T, N>`-type object from an unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, `u128` and `usize`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use Cryptocol::number::BigUInt;
    /// let cc = BigUInt::<u16,32>::from(1004_u32);
    /// println!("cc = {}", cc);
    /// assert_eq!(cc.into_u32(), 1004);
    /// ```
    #[inline]
    fn from(val: S) -> Self
    {
        Self::from_uint(val)
    }
}



impl<T, const N: usize> From<[T; N]> for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /// Constructs a new `BigUInt<T, N>`-type object from an array of type `T`
    /// with `N` elements.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use Cryptocol::number::*;
    /// let big_num = BigUInt::<u8,32>::from([1_u8;32]);
    /// println!("big_num = {}", big_num.to_string_with_radix(2));
    /// assert_eq!(big_num, BigUInt::<u8,32>::from_str_radix("00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001", 2).unwrap());
    /// ```
    fn from(val: [T; N]) -> Self
    {
        Self::from_array(&val)
    }
}



impl<T, const N: usize> FromStr for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Err = NumberErr;
    /// Constructs a new `BigUInt<T, N>`-type object from a string with radix 10.
    /// The constructed object will be wrapped in `Ok(BigUInt<T, N>)` if it is
    /// successfully created. Otherwise, this method returns
    /// `Err(NumberErr::ParsingError)`. And, if you import (use)
    /// std::str::FromStr, you can automagically use str::parse::<BigUInt>() too.
    /// 
    /// # Examples
    /// ```
    /// use std::str::FromStr;
    /// use Cryptocol::number::BigUInt;
    /// use Cryptocol::define_utypes_with_u8;
    /// define_utypes_with_u8!();
    /// let a = u256::from_str("1234").unwrap();
    /// let b = "123_4566".parse::<u256>().unwrap();
    /// println!("a = {}, b = {}", a, b);
    /// ```
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        Self::from_str_radix(s, 10)
    }
}
