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
use std::mem::{ size_of, transmute };
use std::cmp::{ PartialEq, PartialOrd, Ordering };
use std::convert::{ From, Into };
use std::str::FromStr;
use std::ops::*;

use super::uint::*;
use super::big_uint::BigUInt;
use super::NumberErr;



impl<T, const N: usize> Add for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self
    {
        let mut s = self.clone();
        s += rhs;
        s
    }
}



impl<T, const N: usize> AddAssign for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /// Adds and assign the result to it.
    /// 
    #[cfg(target_endian = "little")]
    fn add_assign(&mut self, rhs: Self)
    {
        let mut num: T;
        let mut	carry = false;
        for i in 0..N
        {
            (num, carry) = self.get_num_(i).carrying_add(rhs.get_num_(i), carry);
            self.set_num_(i, num);
        }
        if carry
            { self.set_overflow(); }
/*
        let zero = T::zero();
        let mut midres: T;
        let mut cc = zero;
        let mut c: bool;
        for i in 0..N
        {
            midres = self.number[i].wrapping_add(rhs.number[i]);
            c = midres < self.number[i];
            midres = midres.wrapping_add(carry);
            cc = if c || (midres < cc) { T::one() } else { zero };
            self.number[i] = midres;
        }
        if cc != zero
            { self.set_overflow(); }
*/
    }

    #[cfg(target_endian = "big")]
    fn add_assign(&mut self, rhs: Self)
    {
        let mut i = N - 1;
        let mut	carry = false;
        loop
        {
            (self.number[i], carry) = self.number[i].carrying_add(rhs.number[i], carry);
            if i == 0
                { break; }
            i -= 1;
        }
        if carry
            { self.set_overflow(); }
/*
        let zero = T::zero();
        let mut	carry: T = zero;
        let mut midres: T;
        let mut c: bool;

        let mut i = N - 1;
        loop
        {
            midres = self.number[i].wrapping_add(rhs.number[i]);
            c = midres < self.number[i];
            midres = midres.wrapping_add(carry);
            carry = if c || (midres < carry) { T::one() } else { zero };
            self.number[i] = midres;
            if i == 0
                { break; }
            i -= 1;
        }

        if carry != zero
            { self.set_overflow(); }
    */
    }
}



impl<T, const N: usize> Sub for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self
    {
        let mut s = self.clone();
        s -= rhs;
        s
    }
}



impl<T, const N: usize> SubAssign for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /// Subtracts and assign the result to it.
    /// 
    #[cfg(target_endian = "little")]
    fn sub_assign(&mut self, rhs: Self)
    {
        let mut num: T;
        let mut	borrow = false;
        for i in 0..N
        {
            (num, borrow) = self.get_num_(i).borrowing_sub(rhs.get_num_(i), borrow);
            self.set_num_(i, num);
        }
        if borrow
            { self.set_underflow(); }
/*
       let zero = T::zero();
        let mut	carry: T = zero;
        let mut midres: T;
        let mut c: bool;
        let mut cc: T;

        for i in 0..N
        {
            midres = self.number[i].wrapping_sub(rhs.number[i]);
            c = midres > self.number[i];
            cc = midres;
            midres = midres.wrapping_sub(carry);
            carry = if c || (midres > cc) { T::one() } else { zero };
            self.number[i] = midres;
        }
        if carry != zero
            { self.set_underflow(); }
*/
    }

    /// Subtracts and assign the result to it.
    /// 
    #[cfg(target_endian = "big")]
    fn sub_assign(&mut self, rhs: Self)
    {
        let mut num: T;
        let mut i = N - 1;
        let mut	borrow = false;
        loop
        {
            (num, borrow) = self.number[i].borrowing_sub(rhs.number[i], borrow);
            self.set_num_(i, num);
            if i == 0
                { break; }
            i -= 1;
        }
        if carry
            { self.set_underflow(); }
/*
        let zero = T::zero();
        let mut	carry: T = zero;
        let mut midres: T;
        let mut c: bool;
        let mut cc: T;
        let mut i = N;
        loop
        {
            i -= 1;
            midres = self.number[i].wrapping_sub(rhs.number[i]);
            c = midres > self.number[i];
            cc = midres;
            midres = midres.wrapping_sub(carry);
            carry = if c || (midres > cc) { T::one() } else { zero };
            self.number[i] = midres;
            if i == 0
                { break; }
        }
        if carry != zero
            { self.set_underflow(); }
    */
    }
}



impl<T, const N: usize> Mul for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self
    {
        let mut s = self.clone();
        s *= rhs;
        s
    }
}



impl<T, const N: usize> MulAssign for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /// Multiplies and assign the result to it.
    /// 
    #[cfg(target_endian = "little")]
    fn mul_assign(&mut self, rhs: Self)
    {
        if rhs.is_zero()
        {
            self.set_zero();
            return;
        }
        if self.is_zero()
            { return; }

        let zero = T::zero();
        let one = T::one();
        let adder = self.clone();
        let TSIZE_BITS = size_of::<T>() * 8;
        let mut multiply_first = |num: T| {
            let mut bit_check = one;
            bit_check <<= T::num((TSIZE_BITS - 1).into_u128());
            while (bit_check != zero) && (bit_check & num == zero)
                { bit_check >>= one; }

            self.set_zero();
            while bit_check != zero
            {
                *self <<= 1;
                if bit_check & num != zero
                    { *self += adder; }
                bit_check >>= one;
            }
        };

        let mut n = N - 1;
        while rhs.get_num_(n) == zero
            { n -= 1; }
        multiply_first(rhs.get_num_(n));
        if n == 0
            { return; }
        n -= 1;

        let mut multiply = |num: T| {
            if num == T::zero()
            {
                *self <<= TSIZE_BITS as i32;
                return;
            }
            let mut bit_check = one;
            bit_check <<= T::num((TSIZE_BITS - 1).into_u128());
            while bit_check != zero
            {
                *self <<= 1;
                if bit_check & num != zero
                    { *self += adder; }
                bit_check >>= one;
            }
        };

        loop
        {
            multiply(rhs.get_num_(n));
            if n == 0
                { break; }
            n = n.wrapping_sub(1);
        }
    }

    /// Multiplies and assign the result to it.
    /// 
    #[cfg(target_endian = "big")]
    fn mul_assign(&mut self, rhs: Self)
    {
        if rhs.is_zero()
        {
            self.set_zero();
            return;
        }
        if self.is_zero()
            { return; }

        let zero = T::zero();
        let one = T::one();
        let adder = self.clone();
        let TSIZE_BIT = size_of::<T>() * 8;
        let mut multiply_first = |num: T| {
            let mut bit_check = one;
            bit_check <<= T::num((TSIZE_BIT - 1).into_u128());
            while (bit_check != zero) && (bit_check & num == zero)
                { bit_check >>= one; }

            self.set_zero();
            while bit_check != zero
            {
                *self <<= 1;
                if bit_check & num != zero
                    { *self += adder; }
                bit_check >>= one;
            }
        };

        let mut n = 0;
        while rhs.number[n] == zero
            { n += 1; }
        multiply_first(rhs.number[n]);
        n += 1;

        let mut multiply = |num: T| {
            if num == T::zero()
            {
                *self <<= TSIZE_BIT as i32;
                return;
            }
            let mut bit_check = one;
            bit_check <<= T::num((TSIZE_BIT - 1).into_u128());
            while bit_check != zero
            {
                *self <<= 1;
                if bit_check & num != zero
                    { *self += adder; }
                bit_check >>= one;
            }
        };
        while n < N
        {
            multiply(rhs.number[n]);
            n += 1;
        }
    }
}



impl<T, const N: usize> Div for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self
    {
        let (quotient, _) = self.divide_fully(rhs);
        quotient
    }
}



impl<T, const N: usize> DivAssign for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    fn div_assign(&mut self, rhs: Self) { *self = *self / rhs; }
}



impl<T, const N: usize> Rem for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;
    fn rem(self, rhs: Self) -> Self
    {
        let (_, remainder) = self.divide_fully(rhs);
        remainder
    }
}



impl<T, const N: usize> RemAssign for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    fn rem_assign(&mut self, rhs: Self) { *self = *self % rhs; }
}



impl<T, const N: usize> Shl<i32> for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    /// Performs the << operation. If overflow happens during the << operation,
    /// `OVERFLOW` flag is set and the method is_overflow() will return true. 
    /// [Read more](https://doc.rust-lang.org/core/ops/bit/trait.Shl.html#tymethod.shl)
    /// 
    /// # Examples
    /// ```
    /// use std::str::FromStr;
    /// use Cryptocol::number::BigInteger;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let a = u256::from_str("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
    /// let b = a << 4;
    /// println!("b = {}\noverflow: {}", b, b.is_overflow());
    /// assert_eq!(b.is_overflow(), true);
    /// ```
    /// You have to import (use) Cryptocol::number::BigInteger in order to use
    /// its method is_overflow(). If you find headaching to remember what you
    /// should import, you can just import everything (Cryptocol::number::*)
    /// as next example. It is not harmful.
    /// ```
    /// use std::str::FromStr;
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let a = u256::from_str("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
    /// let b = a << 1;
    /// println!("b = {}\noverflow: {}", b, b.is_overflow());
    /// assert_eq!(b.is_overflow(), false);
    /// ```
    fn shl(self, rhs: i32) -> Self
    {
        let mut s = self.clone();
        s <<= rhs;
        s
    }
}



impl<T, const N: usize> ShlAssign<i32> for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /// Performs the <<= operation. If overflow happens during the <<= operation,
    /// `OVERFLOW` flag is set and the method is_overflow() will return true. 
    /// [Read more](https://doc.rust-lang.org/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)
    /// 
    /// # Examples
    /// ```
    /// use std::str::FromStr;
    /// use Cryptocol::number::BigInteger;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let mut a = u256::from_str("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
    /// a <<= 4;
    /// println!("a = {}\noverflow: {}", a, a.is_overflow());
    /// assert_eq!(a.is_overflow(), true);
    /// ```
    /// You have to import (use) Cryptocol::number::BigInteger in order to use
    /// its method is_overflow(). If you find headaching to remember what you
    /// should import, you can just import everything (Cryptocol::number::*)
    /// as next example. It is not harmful.
    /// ```
    /// use std::str::FromStr;
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let mut a = u256::from_str("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
    /// a <<= 1;
    /// println!("a = {}\noverflow: {}", a, a.is_overflow());
    /// assert_eq!(a.is_overflow(), false);
    /// ```
    #[cfg(target_endian = "little")]
    fn shl_assign(&mut self, rhs: i32)
    {
        if rhs < 0
        {
		    *self >>= -rhs;
            return;
        }
        let TSIZE_IN_BITS = T::size_in_bits();
        let chunk_num = rhs as usize / TSIZE_IN_BITS as usize;
        let piece_num = rhs as usize % TSIZE_IN_BITS as usize;
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
        if (self.get_num_(N-1) >> T::num((TSIZE_IN_BITS - piece_num).into_u128())) != zero
            { self.set_overflow(); }

        let mut num: T;
        let mut carry = zero;
        for idx in chunk_num..N
        {
            num = (self.get_num_(idx) << T::num(piece_num.into_u128())) | carry;
            carry = self.get_num_(idx) >> T::num((TSIZE_IN_BITS - piece_num).into_u128());
            self.set_num_(idx, num);
        }
        if carry != zero
            { self.set_overflow(); }
    }
}



impl<T, const N: usize> Shr<i32> for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
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
    /// # Examples
    /// ```
    /// use std::str::FromStr;
    /// use Cryptocol::number::BigInteger;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let a = u256::from_str("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
    /// let b = a >> 2;
    /// println!("b = {}\nunderflow: {}", b, b.is_underflow());
    /// assert_eq!(b.is_underflow(), true);
    /// ```
    /// You have to import (use) Cryptocol::number::BigInteger in order to use
    /// its method is_underflow(). If you find headaching to remember what you
    /// should import, you can just import everything (Cryptocol::number::*)
    /// as next example. It is not harmful.
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
    fn shr(self, rhs: i32) -> Self
    {
        let mut s = self.clone();
        s >>= rhs;
        s
    }
}



impl<T, const N: usize> ShrAssign<i32> for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
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
    /// # Examples
    /// ```
    /// use std::str::FromStr;
    /// use Cryptocol::number::BigInteger;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let mut a = u256::from_str("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
    /// a >>= 2;
    /// println!("a = {}\nunderflow: {}", a, a.is_underflow());
    /// assert_eq!(a.is_underflow(), true);
    /// ```
    /// You have to import (use) Cryptocol::number::BigInteger in order to use
    /// its method is_underflow(). If you find headaching to remember what you
    /// should import, you can just import everything (Cryptocol::number::*)
    /// as next example. It is not harmful.
    /// ```
    /// use std::str::FromStr;
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let mut a = u256::from_str("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
    /// a >>= 1;
    /// println!("a = {}\nunderflow: {}", a, a.is_underflow());
    /// assert_eq!(a.is_underflow(), false);
    /// ```
    #[cfg(target_endian = "little")]
    fn shr_assign(&mut self, rhs: i32)
    {
        if rhs < 0
        {
		    *self <<= -rhs;
            return;
        }
        let TSIZE_IN_BITS = T::size_in_bits();
        let chunk_num = rhs as usize / TSIZE_IN_BITS as usize;
        let piece_num = rhs as usize % TSIZE_IN_BITS as usize;
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
        if (self.get_num_(0) << T::num((TSIZE_IN_BITS - piece_num).into_u128())) != zero
            { self.set_underflow(); }

        let mut num: T;
        let mut carry = T::zero();
        let mut idx = N - 1 - chunk_num;
        loop
        {
            num = (self.get_num_(idx) >> T::num(piece_num.into_u128())) | carry;
            carry = self.get_num_(idx) << T::num((TSIZE_IN_BITS - piece_num).into_u128());
            self.set_num_(idx, num);
            if idx == 0
                { break; }
            idx -= 1;
        }
        if carry != zero
            { self.set_underflow(); }
    }
}



impl<T, const N: usize> BitAnd for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
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
    /// use Cryptocol::number::HugeInteger;
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
    /// use Cryptocol::number::HugeInteger;
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
    /// use Cryptocol::number::HugeInteger;
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
    /// use Cryptocol::number::HugeInteger;
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
    /// use Cryptocol::number::HugeInteger;
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
    /// use Cryptocol::number::HugeInteger;
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
        write!(f, "{}", self.to_string_with_radix(10))
    }
}



impl<T, const N: usize, S> From<S> for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd,
    S: Uint + Clone + Display + Debug + ToString
        + Add<Output=S> + AddAssign + Sub<Output=S> + SubAssign
        + Mul<Output=S> + MulAssign + Div<Output=S> + DivAssign
        + Shl<Output=S> + ShlAssign + Shr<Output=S> + ShrAssign
        + BitAnd<Output=S> + BitAndAssign + BitOr<Output=S> + BitOrAssign
        + BitXor<Output=S> + BitXorAssign + Not<Output=S>
        + PartialEq + PartialOrd
{
    /// Constructs a new `BigUInt<T, N>` from an unsigned integer
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
    /// 
    #[cfg(target_endian = "little")]
    fn from(val: S) -> Self
    {
        let TSIZE = size_of::<T>();
        let SSIZE = size_of::<S>();
        let mut me = Self::new();
        let mut share = Share::<T, S>::from_src(val);
        
        if TSIZE >= SSIZE
        {
            unsafe { me.set_num(0, share.des); }
        }
        else
        {
            let TSIZE_BITS = TSIZE * 8;
            for i in 0..SSIZE/TSIZE
            {
                unsafe { me.set_num(i, share.des); }
                unsafe { share.src >>= S::num(TSIZE_BITS as u128); }
            }
        }
        return me;
    }
}



impl<T, const N: usize> From<[T; N]> for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /// Constructs a new `BigUInt<T, N>` from an array of type `T`
    /// with `N` elements.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use Cryptocol::number::*;
    /// let big_num = BigUInt::<u8,32>::from(&[1_u8;32]);
    /// println!("big_num = {}", big_num.to_string_with_radix(2));
    /// assert_eq!(big_num, BigUInt::<u8,32>::from_str_radix("00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001", 2).unwrap());
    /// ```
    fn from(val: [T; N]) -> Self
    {
        let mut s = Self::new();
        s.set_number(&val);
        s
    }
}



impl<T, const N: usize> FromStr for BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Err = NumberErr;
    /// Constructs a new `BigUInt<T, N>` from a string with radix 10.
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
