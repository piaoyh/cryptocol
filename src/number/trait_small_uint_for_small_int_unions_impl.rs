// Copyright 2023, 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module that contains generic types of primitive unsigned integral
//! data types used in a lot of modules of the crate Cryptocol.
//! __These unions are for segmenting data chunk into smaller pieces.__

#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
// #![allow(missing_docs)]
// #![allow(rustdoc::missing_doc_code_examples)]

use std::fmt::{ self, Debug, Display, Formatter };
use std::mem::{ size_of, size_of_val };
use std::cmp::{ PartialEq, PartialOrd, Ordering };
use std::ops::*;
use super::{ SmallUInt, ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };

macro_rules! SmallUInt_methods_for_integer_unions_impl {
    ($f:ty, $g:ty) => {
        impl SmallUInt for $f
        {
            /// Calculates self + rhs + carry and returns a tuple containing
            /// the sum and the output carry.
            /// [Read more in detail](trait@SmallUInt#tymethod.carrying_add)
            #[inline] fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool)   { self.carrying_add(rhs, carry) }

            /// Computes self + rhs, wrapping around at the boundary of the type.
            /// [Read more in detail](trait@SmallUInt#tymethod.wrapping_add)
            #[inline] fn wrapping_add(self, rhs: Self) -> Self  { self.wrapping_add(rhs) }

            /// Calculates self + rhs and returns a tuple of the addition along
            /// with a boolean indicating whether an arithmetic overflow would
            /// occur. [Read more in detail](trait@SmallUInt#tymethod.overflowing_add)
            #[inline] fn overflowing_add(self, rhs: Self) -> (Self, bool)   { self.overflowing_add(rhs) }
            
            /// Computes self + rhs and returns None if overflow occurred.
            /// [Read more in detail](trait@SmallUInt#tymethod.checked_add)
            #[inline] fn checked_add(self, rhs: Self) -> Option<Self>   { self.checked_add(rhs) }
            
            /// Computes self + rhs, assuming overflow cannot occur.
            /// [Read more in detail](trait@SmallUInt#tymethod.unchecked_add)
            #[inline] fn unchecked_add(self, rhs: Self) -> Self     { self.checked_add(rhs).unwrap() }
            
            /// Computes self + rhs, saturating at the numeric bounds
            /// instead of overflowing.
            /// [Read more in detail](trait@SmallUInt#tymethod.saturating_add)
            #[inline] fn saturating_add(self, rhs: Self) -> Self    { self.saturating_add(rhs) }

            /// Computes `self` + `rhs`, wrapping at `modulo`
            /// instead of overflowing.
            /// [Read more in detail](trait@SmallUInt#tymethod.modular_add)
            fn modular_add(self, rhs: Self, modulo: Self) -> Self
            {
                let mlhs = self.wrapping_rem(modulo);
                let mrhs = rhs.wrapping_rem(modulo);
                let diff = modulo.wrapping_sub(mrhs);
                if self.get() >= diff.get()
                    { mlhs.wrapping_sub(diff) }
                else
                    { mlhs.wrapping_add(mrhs) }
            }

            /// Calculates self - rhs - borrow,
            /// wrapping around at the boundary of the type.
            /// [Read more in detail](trait@SmallUInt#tymethod.borrowing_sub)
            #[inline] fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool)   { self.borrowing_sub(rhs, borrow) }

            /// Computes self - rhs, wrapping around at the boundary of the type.
            /// [Read more in detail](trait@SmallUInt#tymethod.wrapping_sub)
            #[inline] fn wrapping_sub(self, rhs: Self) -> Self  { self.wrapping_sub(rhs) }

            /// Calculates self - rhs and returns a tuple of the subtraction
            /// along with a boolean indicating whether an arithmetic overflow
            /// would occur. [Read more in detail](trait@SmallUInt#tymethod.overflowing_sub)
            #[inline] fn overflowing_sub(self, rhs: Self) -> (Self, bool)   { self.overflowing_sub(rhs) }

            /// Computes self - rhs, returning None if overflow occurred.
            /// [Read more in detail](trait@SmallUInt#tymethod.checked_sub)
            #[inline] fn checked_sub(self, rhs: Self) -> Option<Self>   { self.checked_sub(rhs) }

            /// Computes self - rhs, assuming overflow cannot occur.
            /// [Read more in detail](trait@SmallUInt#tymethod.unchecked_sub)
            #[inline] fn unchecked_sub(self, rhs: Self) -> Self     { self.checked_sub(rhs).unwrap() }

            /// Computes self - rhs, saturating at the numeric bounds
            /// instead of overflowing.
            /// [Read more in detail](trait@SmallUInt#tymethod.saturating_sub)
            #[inline] fn saturating_sub(self, rhs: Self) -> Self    { self.saturating_sub(rhs) }

            /// Computes the absolute difference between `self` and `other`.
            /// [Read more in detail](trait@SmallUInt#tymethod.abs_diff)
            #[inline] fn abs_diff(self, other: Self) -> Self    { self.abs_diff(other) }

            /// Computes `self` - `rhs`, wrapping at `modulo`
            /// instead of overflowing.
            /// [Read more in detail](trait@SmallUInt#tymethod.modular_sub)
            fn modular_sub(self, rhs: Self, modulo: Self) -> Self
            {
                let mlhs = self.wrapping_rem(modulo);
                let mrhs = rhs.wrapping_rem(modulo);
                if mlhs.get() >= mrhs.get()
                {
                    mlhs.wrapping_sub(mrhs)
                }
                else
                {
                    let diff = modulo.wrapping_sub(mrhs);
                    mlhs.wrapping_add(diff)
                }
            }

            /// Calculates the “full multiplication” `self` * `rhs` + `carry` without
            /// the possibility to overflow.
            /// [Read more in detail](trait@SmallUInt#tymethod.carrying_mul)
            #[inline] fn carrying_mul(self, rhs: Self, carry: Self) -> (Self, Self) { self._carrying_mul(rhs, carry) }

            // fn _carrying_mul(self, rhs: Self, carry: Self) -> (Self, Self);
            /// It is for internal use. You are recommended to use [carrying_mul()](trait@SmallUInt#tymethod.carrying_mul) instead.
            fn _carrying_mul(self, rhs: Self, carry: Self) -> (Self, Self)
            {
                let (low, high) = self.get()._carrying_mul(rhs.get(), carry.get());
                (Self::new_with(low), Self::new_with(high))
            }

            /// Calculates the complete product `self` * `rhs` without the possibility
            /// to overflow. [Read more in detail](trait@SmallUInt#tymethod.widening_mul)
            #[inline] fn widening_mul(self, rhs: Self) -> (Self, Self)  { self._widening_mul(rhs) }

            // fn _widening_mul(self, rhs: Self, carry: Self) -> (Self, Self);
            /// It is for internal use. You are recommended to use [widening_mul()](trait@SmallUInt#tymethod.widening_mul) instead.
            fn _widening_mul(self, rhs: Self) -> (Self, Self)
            {
                let (low, high) = self.get()._widening_mul(rhs.get());
                (Self::new_with(low), Self::new_with(high))
            }

            /// Computes self * rhs, wrapping around at the boundary of the type.
            /// [Read more in detail](trait@SmallUInt#tymethod.wrapping_mul)
            #[inline] fn wrapping_mul(self, rhs: Self) -> Self  { self.wrapping_mul(rhs) }

            /// Calculates the multiplication of self and rhs and returns a tuple
            /// of the multiplication along with a boolean indicating whether an
            /// arithmetic overflow would occur.
            /// [Read more in detail](trait@SmallUInt#tymethod.overflowing_mul)
            #[inline] fn overflowing_mul(self, rhs: Self) -> (Self, bool)   { self.overflowing_mul(rhs) }

            /// Computes self * rhs, returning None if overflow occurred.
            /// [Read more in detail](trait@SmallUInt#tymethod.checked_mul)
            #[inline] fn checked_mul(self, rhs: Self) -> Option<Self>   { self.checked_mul(rhs) }

            /// Computes self * rhs, assuming overflow cannot occur.
            /// [Read more in detail](trait@SmallUInt#tymethod.unchecked_mul)
            #[inline] fn unchecked_mul(self, rhs: Self) -> Self     { self.checked_mul(rhs).unwrap() }

            /// Computes self * rhs, saturating at the numeric bounds
            /// instead of overflowing.
            /// [Read more in detail](trait@SmallUInt#tymethod.saturating_mul)
            #[inline] fn saturating_mul(self, rhs: Self) -> Self    { self.saturating_mul(rhs) }

            /// Computes `self` * `rhs`, wrapping at `modulo`
            /// instead of overflowing.
            /// [Read more in detail](trait@SmallUInt#tymethod.modular_mul)
            fn modular_mul(self, rhs: Self, modulo: Self) -> Self
            {
                let mut mrhs = rhs.wrapping_rem(modulo);
                let mut mlhs = self.wrapping_rem(modulo);
                let mut res = Self::zero();
                while mrhs.get() > Self::zero().get()
                {
                    if mrhs.is_odd()
                        { res = res.modular_add(mlhs, modulo); }
                    mlhs = mlhs.modular_add(mlhs, modulo);
                    mrhs.set(mrhs.get() >> 1);
                }
                res
            }

            /// Computes self / rhs. Wrapped division on unsigned types is just
            /// normal division. [Read more in detail](trait@SmallUInt#tymethod.wrapping_div)
            #[inline] fn wrapping_div(self, rhs: Self) -> Self  { self.wrapping_div(rhs) }

            /// Calculates the divisor when self is divided by rhs and returns
            /// a tuple of the divisor along with a boolean indicating whether
            /// an arithmetic overflow would occur.
            /// [Read more in detail](trait@SmallUInt#tymethod.overflowing_div)
            #[inline] fn overflowing_div(self, rhs: Self) -> (Self, bool)   { self.overflowing_div(rhs) }

            /// Computes self / rhs, returning None if rhs == 0.
            /// [Read more in detail](trait@SmallUInt#tymethod.checked_div)
            #[inline] fn checked_div(self, rhs: Self) -> Option<Self>   { self.checked_div(rhs) }

            /// Computes `self` / `rhs`, returning None if `rhs`` == 0.
            /// [Read more in detail](trait@SmallUInt#tymethod.unchecked_div)
            #[inline] fn unchecked_div(self, rhs: Self) -> Self   { self.checked_div(rhs).unwrap() }

            /// Computes self / rhs, saturating at the numeric bounds
            /// instead of overflowing.
            /// [Read more in detail](trait@SmallUInt#tymethod.saturating_div)
            #[inline] fn saturating_div(self, rhs: Self) -> Self    { self.saturating_div(rhs) }


            /// Computes self % rhs. Wrapped remainder calculation on unsigned
            /// types is just the regular remainder calculation.
            /// [Read more in detail](trait@SmallUInt#tymethod.wrapping_rem)
            #[inline] fn wrapping_rem(self, rhs: Self) -> Self  { self.wrapping_rem(rhs) }

            /// Calculates the remainder when self is divided by rhs, and returns
            /// a tuple of the remainder after dividing along with a boolean
            /// indicating whether an arithmetic overflow would occur.
            /// [Read more in detail](trait@SmallUInt#tymethod.overflowing_rem)
            #[inline] fn overflowing_rem(self, rhs: Self) -> (Self, bool)   { self.overflowing_rem(rhs) }

            /// Computes self % rhs, returning None if rhs == 0.
            /// [Read more in detail](trait@SmallUInt#tymethod.checked_rem)
            #[inline] fn checked_rem(self, rhs: Self) -> Option<Self>   { self.checked_rem(rhs) }

            /// Computes `self` % `rhs`, if `rhs` != 0.
            /// [Read more in detail](trait@SmallUInt#tymethod.unchecked_rem)
            #[inline] fn unchecked_rem(self, rhs: Self) -> Self         { self.checked_rem(rhs).unwrap() }

            /// Computes `-self`, wrapping around at the boundary of the type.
            /// [Read more in detail](trait@SmallUInt#tymethod.wrapping_neg)
            #[inline] fn wrapping_neg(self) -> Self                     { self.wrapping_neg() }
            
            /// Raises `self` to the power of `exp`, using exponentiation by squaring.
            /// [Read more in detail](trait@SmallUInt#tymethod.pow)
            #[inline] fn pow(self, exp: u32) -> Self    { self.pow(exp) }

            /// Computes self.pow(exp), wrapping around at the boundary of the type.
            /// [Read more in detail](trait@SmallUInt#tymethod.wrapping_pow)
            #[inline] fn wrapping_pow(self, exp: u32) -> Self   { self.wrapping_pow(exp) }

            /// Raises self to the power of exp, using exponentiation by squaring.
            /// [Read more in detail](trait@SmallUInt#tymethod.overflowing_pow)
            #[inline] fn overflowing_pow(self, exp: u32) -> (Self, bool)    { self.overflowing_pow(exp) }

            /// Computes self.pow(exp), returning None if overflow occurred.
            /// [Read more in detail](trait@SmallUInt#tymethod.checked_pow)
            #[inline] fn checked_pow(self, exp: u32) -> Option<Self>    { self.checked_pow(exp) }

            /// Computes self.pow(exp), unless overflow does not occcurred.
            /// Otherwise, it will panic.
            /// [Read more in detail](trait@SmallUInt#tymethod.unchecked_pow)
            #[inline] fn unchecked_pow(self, exp: u32) -> Self      { self.checked_pow(exp).unwrap() }

            /// Computes self.pow(exp), saturating at the numeric bounds
            /// instead of overflowing.
            /// [Read more in detail](trait@SmallUInt#tymethod.saturating_pow)
            #[inline] fn saturating_pow(self, exp: u32) -> Self     { self.saturating_pow(exp) }

            /// Computes self.pow(exp), saturating at `modulo`
            /// instead of overflowing.
            /// [Read more in detail](trait@SmallUInt#tymethod.modular_pow)
            fn modular_pow(self, exp: Self, modulo: Self) -> Self
            {
                let mut mlhs = self.wrapping_rem(modulo);
                let mut res = Self::one();
                let mut mexp = exp;
                while mexp.get() > 0
                {
                    if mexp.is_odd()
                        { res = res.modular_mul(mlhs, modulo); }
                    mlhs = mlhs.modular_mul(mlhs, modulo);
                    mexp.set(mexp.get() >> 1);
                }
                res
            }

            #[inline] fn ilog(self, base: Self) -> u32  { self.get().ilog(base.get()) }
            #[inline] fn ilog10(self) -> u32            { self.get().ilog10() }
            #[inline] fn ilog2(self) -> u32             { self.get().ilog2() }

            #[inline] fn isqrt(self) -> Self            { Self::new_with(self.get()._isqrt()) }
            #[inline] fn _isqrt(self) -> Self           { Self::new_with(self.get()._isqrt()) }
            #[inline] fn root(self, exp: Self) -> Self  { Self::new_with(self.get().root(exp.get())) }



    /***** METHODS FOR GENERATING RANDOM PRIME NUMBERS *****/

            /// Performs Millar Rabin method with a number less than `self`.
            /// [Read more in detail](trait@SmallUInt#tymethod.is_prime_using_miller_rabin)
            #[inline] fn is_prime_using_miller_rabin(self, repetition: usize) -> bool   { self.get().is_prime_using_miller_rabin(repetition) }

            /// Tests a `SmallUInt`-type object to find whether or not it is a
            /// prime number.
            /// [Read more in detail](trait@SmallUInt#tymethod.test_miller_rabin)
            #[inline] fn test_miller_rabin(self, a: Self) -> bool   { self.get().test_miller_rabin(a.get()) }

            #[inline] fn reverse_bits(self) -> Self     { self.reverse_bits() }
            // #[inline] fn reverse_bits_assign(&mut self) { *self = self.reverse_bits(); }

            #[inline] fn rotate_left(self, n: u32) -> Self  { self.rotate_left(n) }
            #[inline] fn rotate_right(self, n: u32) -> Self { self.rotate_right(n) }

            #[inline] fn count_ones(self) -> u32        { self.count_ones() }
            #[inline] fn count_zeros(self) -> u32       { self.count_zeros() }
            #[inline] fn leading_ones(self) -> u32      { self.leading_ones() }
            #[inline] fn leading_zeros(self) -> u32     { self.leading_zeros() }
            #[inline] fn trailing_ones(self) -> u32     { self.trailing_ones() }
            #[inline] fn trailing_zeros(self) -> u32    { self.trailing_zeros() }

            #[inline] fn from_be(x: Self) -> Self   { Self::from_be(x) }
            #[inline] fn from_le(x: Self) -> Self   { Self::from_le(x) }
            #[inline] fn to_be(self) -> Self        { self.to_be() }
            #[inline] fn to_le(self) -> Self        { self.to_le() }
            #[inline] fn swap_bytes(self) -> Self   { self.swap_bytes() }

            #[inline] fn is_power_of_two(self) -> bool      { self.is_power_of_two() }
            #[inline] fn next_power_of_two(self) -> Self    { self.next_power_of_two() }

            #[inline] fn into_f64(self) -> f64      { self.get() as f64 }
            #[inline] fn into_f32(self) -> f32      { self.get() as f32 }
            #[inline] fn into_u128(self) -> u128    { self.get() as u128 }
            #[inline] fn into_u64(self) -> u64      { self.get() as u64 }
            #[inline] fn into_u32(self) -> u32      { self.get() as u32 }
            #[inline] fn into_u16(self) -> u16      { self.get() as u16 }
            #[inline] fn into_u8(self) -> u8        { self.get() as u8 }
            #[inline] fn into_usize(self) -> usize  { self.get() as usize }
            #[inline] fn into_bool(self) -> bool    { self.get() != 0 }
            #[inline] fn into_shortunion(self) -> ShortUnion    { ShortUnion::new_with(self.into_u16() ) }
            #[inline] fn into_intunion(self) -> IntUnion        { IntUnion::new_with(self.into_u32() ) }
            #[inline] fn into_longunion(self) -> LongUnion      { LongUnion::new_with(self.into_u64() ) }
            #[inline] fn into_longerunion(self) -> LongerUnion  { LongerUnion::new_with(self.into_u128() ) }
            #[inline] fn into_sizeunion(self) -> SizeUnion      { SizeUnion::new_with(self.into_usize() ) }
            #[inline] fn zero() -> Self             { Self::new_with(0) }
            #[inline] fn one() -> Self              { Self::new_with(1) }
            #[inline] fn max() -> Self              { Self::new_with(<$g>::MAX) }
            #[inline] fn min() -> Self              { Self::new_with(<$g>::MIN) }
            #[inline] fn u128_as_smalluint(n: u128) -> Self  { Self::new_with(n as $g) }
            #[inline] fn u64_as_smalluint(n: u64) -> Self    { Self::new_with(n as $g) }
            #[inline] fn u32_as_smalluint(n: u32) -> Self    { Self::new_with(n as $g) }
            #[inline] fn u16_as_smalluint(n: u16) -> Self    { Self::new_with(n as $g) }
            #[inline] fn u8_as_smalluint(n: u8) -> Self      { Self::new_with(n as $g) }
            #[inline] fn usize_as_smalluint(n: usize) -> Self    { Self::new_with(n as $g) }
            #[inline] fn bool_as_smalluint(n: bool) -> Self  { Self::new_with(n as $g) }

            #[inline]
            fn num<T: SmallUInt>(n: T) -> Self
            {
                match size_of::<T>()
                {
                    1 => { return Self::u8_as_smalluint(n.into_u8()); },
                    2 => { return Self::u16_as_smalluint(n.into_u16()); },
                    4 => { return Self::u32_as_smalluint(n.into_u32()); },
                    8 => { return Self::u64_as_smalluint(n.into_u64()); },
                    _ => { return Self::u128_as_smalluint(n.into_u128()); },
                }
            }

            /// Checks whether `SmallUInt` to be zero, and returns true if it is
            /// zero, and returns false if it is not zero.
            /// [Read more in detail](trait@SmallUInt#tymethod.is_zero)
            #[inline] fn is_zero(self) -> bool     { self.get() == 0 }

            /// Checks whether `SmallUInt` to be zero, and returns true if it is
            /// zero, and returns false if it is not zero.
            /// [Read more in detail](trait@SmallUInt#tymethod.is_one)
            #[inline] fn is_one(self) -> bool      { self.get() ==  1 }

            /// Checks whether or not `self` is either zero or one, and returns true
            /// if it is either zero or one. Otherwise, it returns false.
            /// [Read more](trait@SmallUInt#tymethod.is_zero_or_one) in detail.
            #[inline] fn is_zero_or_one(self) -> bool   { self.get() < 2 }

            /// Sets the MSB (Most Significant Bit) of `SmallUInt`-type
            /// number with `1`.
            /// [Read more in detail](trait@SmallUInt#tymethod.set_msb)
            #[inline] fn set_msb(&mut self)     { self.set(self.get() | !(Self::max().get() >> 1)); }

            /// Sets the LSB (Least Significant Bit) of `SmallUInt`-type
            /// number with `1`.
            /// [Read more in detail](trait@SmallUInt#tymethod.set_lsb)
            #[inline] fn set_lsb(&mut self)     { self.set(self.get() | 1); }
    
            /// Constucts a new `SmallUInt` which has the value zero and sets only
            /// the bit specified by the argument bit_pos to be 1.
            /// [Read more](trait@SmallUInt#tymethod.generate_check_bits) in detail.
            #[inline] fn generate_check_bits(bit_pos: usize) -> Option<Self>    { if bit_pos < Self::size_in_bits() { Some(Self::generate_check_bits_(bit_pos)) } else { None } }
    
            /// Constucts a new `SmallUInt` which has the value zero and sets only
            /// the bit specified by the argument bit_pos to be 1.
            /// [Read more](trait@SmallUInt#tymethod.generate_check_bits_) in detail.
            #[inline] fn generate_check_bits_(bit_pos: usize) -> Self    { Self::new_with(Self::one().get() << bit_pos) }


            #[inline] fn is_odd(self) -> bool       { (self.get() & 1) != 0 }
            #[inline] fn is_even(self) -> bool      { !self.is_odd() }
            #[inline] fn is_msb_set(self) -> bool   { (self.get() & !(Self::max().get() >> 1)) != 0 }
            #[inline] fn is_bit_set(self, bit_pos: usize) -> Option<bool>  { if bit_pos < Self::size_in_bits() { Some(self.is_bit_set_(bit_pos)) } else { None } }
            #[inline] fn is_bit_set_(self, bit_pos: usize) -> bool  { self.get() & Self::generate_check_bits_(bit_pos).get() != 0 }

            /// Sets `Self`-type number to be maximum value in which all bits are
            /// set to be `1`.
            /// [Read more](trait@SmallUInt#tymethod.set_max) in detail.
            #[inline] fn set_max(&mut self)     { *self = Self::max() }

            /// Checks whether or not `BigUInt`-type number to be maximum value.
            /// zero, and returns false if it is not zero.
            /// [Read more in detail](trait@SmallUInt#tymethod.set_submax)
            #[inline] fn is_max(self) -> bool { self == Self::max() }

            /// Checks whether `SmallUInt` to be zero, and returns true if it is
            /// zero, and returns false if it is not zero.
            /// [Read more in detail](trait@SmallUInt#tymethod.set_submax)
            fn set_submax(&mut self, size_in_bits: usize)
            {
                if size_in_bits >= self.length_in_bits()
                    { self.set_max(); }
                else if size_in_bits == 0
                    { self.set_zero(); }
                else
                {
                    self.set_max();
                    self.set(self.get() >> (Self::size_in_bits() - size_in_bits));
                }
            }

            /// Sets `Self`-type number to be half long maximum value
            /// in which all bits are set to be `1`.
            /// [Read more](trait@SmallUInt#tymethod.set_max) in detail.
            #[inline] fn set_halfmax(&mut self) { self.set_submax(self.length_in_bits() >> 1); }

            /// Sets `self` to be zero.
            /// [Read more](trait@SmallUInt#tymethod.set_one) in detail.
            #[inline] fn set_one(&mut self)     { self.set(1); }

            /// Sets `self` to be zero.
            /// [Read more](trait@SmallUInt#tymethod.set_one) in detail.
            #[inline] fn set_zero(&mut self)     { self.set(0); }

            #[inline] fn size_in_bytes() -> usize   { size_of::<Self>() }
            #[inline] fn size_in_bits() -> usize    { size_of::<Self>() * 8 }
            #[inline] fn length_in_bytes(self) -> usize    { size_of_val(&self) }
            #[inline] fn length_in_bits(self) -> usize     { size_of_val(&self) * 8 }
        }
    }
}

macro_rules! operators_for_integer_unions_impl {
    ($f:ty) => {
        impl Add for $f
        {
            type Output = Self;

            #[inline]
            fn add(self, rhs: Self) -> Self
            {
                self.wrapping_add(rhs)
            }
        }

        impl AddAssign for $f
        {
            #[inline]
            fn add_assign(&mut self, rhs: Self)
            {
                self.set(self.get().wrapping_add(rhs.get()));
            }
        }

        impl Sub for $f
        {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: Self) -> Self
            {
                self.wrapping_sub(rhs)
            }
        }

        impl SubAssign for $f
        {
            #[inline]
            fn sub_assign(&mut self, rhs: Self)
            {
                self.set(self.get().wrapping_sub(rhs.get()));
            }
        }

        impl Mul for $f
        {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: Self) -> Self
            {
                self.wrapping_mul(rhs)
            }
        }

        impl MulAssign for $f
        {
            #[inline]
            fn mul_assign(&mut self, rhs: Self)
            {
                self.set(self.get().wrapping_mul(rhs.get()));
            }
        }

        impl Div for $f
        {
            type Output = Self;

            #[inline]
            fn div(self, rhs: Self) -> Self
            {
                self.wrapping_div(rhs)
            }
        }

        impl DivAssign for $f
        {
            #[inline]
            fn div_assign(&mut self, rhs: Self)
            {
                self.set(self.get().wrapping_div(rhs.get()));
            }
        }

        impl Rem for $f
        {
            type Output = Self;

            #[inline]
            fn rem(self, rhs: Self) -> Self
            {
                self.wrapping_rem(rhs)
            }
        }

        impl RemAssign for $f
        {
            #[inline]
            fn rem_assign(&mut self, rhs: Self)
            {
                self.set(self.get().wrapping_rem(rhs.get()));
            }
        }

        impl BitAnd for $f
        {
            type Output = Self;

            #[inline]
            fn bitand(self, rhs: Self) -> Self
            {
                let mut s = self.clone();
                s &= rhs;
                s
            }
        }

        impl BitAndAssign for $f
        {
            #[inline]
            fn bitand_assign(&mut self, rhs: Self)
            {
                self.set(self.get() & rhs.get());
            }
        }

        impl BitOr for $f
        {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self
            {
                let mut s = self.clone();
                s |= rhs;
                s
            }
        }

        impl BitOrAssign for $f
        {
            #[inline]
            fn bitor_assign(&mut self, rhs: Self)
            {
                self.set(self.get() | rhs.get());
            }
        }

        impl BitXor for $f
        {
            type Output = Self;

            fn bitxor(self, rhs: Self) -> Self
            {
                let mut s = self.clone();
                s ^= rhs;
                s
            }
        }

        impl BitXorAssign for $f
        {
            #[inline]
            fn bitxor_assign(&mut self, rhs: Self)
            {
                self.set(self.get() ^ rhs.get());
            }
        }

        impl Not for $f
        {
            type Output = Self;

            #[inline]
            fn not(self) -> Self
            {
                Self::new_with(!self.get())
            }
        }

        impl PartialEq for $f
        {
            #[inline]
            fn eq(&self, other: &Self) -> bool
            {
                self.get() == other.get()
            }
        }

        impl PartialOrd for $f
        {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering>
            {
                if self.get() > other.get()
                    { return Some(Ordering::Greater); }
                else if self.get() < other.get()
                    { return Some(Ordering::Less); }
                else
                    { Some(Ordering::Equal) }
            }
        }
    }
}



macro_rules! shift_ops_for_integer_unions_impl {
    ($u:ty, $f:ty) => {
        impl Shl<$f> for $u
        {
            type Output = Self;

            fn shl(self, rhs: $f) -> Self
            {
                let mut s = self.clone();
                s <<= rhs;
                s
            }
        }

        impl ShlAssign<$f> for $u
        {
            #[inline]
            fn shl_assign(&mut self, rhs: $f)
            {
                self.set(self.get() << rhs)
            }
        }

        impl Shr<$f> for $u
        {
            type Output = Self;

            fn shr(self, rhs: $f) -> Self
            {
                let mut s = self.clone();
                s >>= rhs;
                s
            }
        }

        impl ShrAssign<$f> for $u
        {
            #[inline]
            fn shr_assign(&mut self, rhs: $f)
            {
                self.set(self.get() >> rhs)
            }
        }
    }
}



macro_rules! display_for_integer_unions_impl {
    ($f:ty) => {
        impl Display for $f
        {
            /// Formats the value using the given formatter.
            /// Automagically the function `to_string()` will be implemented. So, you
            /// can use the function `to_string()` and the macro `println!()`.
            /// `f` is a buffer, this method must write the formatted string into it.
            /// [Read more](https://doc.rust-lang.org/core/fmt/trait.Display.html#tymethod.fmt)
            /// 
            /// # Example
            /// ```
            /// use cryptocol::number::*;
            /// let a = ShortUnion::new_with(60521_u16);
            /// println!("{}", a);
            /// ```
            fn fmt(&self, f: &mut Formatter) -> fmt::Result
            {
                // `write!` is like `format!`, but it will write the formatted string
                // into a buffer (the first argument)
                write!(f, "{}", self.get())
            }
        }
    }
}



SmallUInt_methods_for_integer_unions_impl! { ShortUnion, u16 }
SmallUInt_methods_for_integer_unions_impl! { IntUnion, u32 }
SmallUInt_methods_for_integer_unions_impl! { LongUnion, u64 }
SmallUInt_methods_for_integer_unions_impl! { LongerUnion, u128 }
SmallUInt_methods_for_integer_unions_impl! { SizeUnion, usize }

operators_for_integer_unions_impl! { ShortUnion }
operators_for_integer_unions_impl! { IntUnion }
operators_for_integer_unions_impl! { LongUnion }
operators_for_integer_unions_impl! { LongerUnion }
operators_for_integer_unions_impl! { SizeUnion }

shift_ops_for_integer_unions_impl! { ShortUnion, i8 }
shift_ops_for_integer_unions_impl! { ShortUnion, i16 }
shift_ops_for_integer_unions_impl! { ShortUnion, i32 }
shift_ops_for_integer_unions_impl! { ShortUnion, i64 }
shift_ops_for_integer_unions_impl! { ShortUnion, i128 }
shift_ops_for_integer_unions_impl! { ShortUnion, isize }

shift_ops_for_integer_unions_impl! { ShortUnion, u8 }
shift_ops_for_integer_unions_impl! { ShortUnion, u16 }
shift_ops_for_integer_unions_impl! { ShortUnion, u32 }
shift_ops_for_integer_unions_impl! { ShortUnion, u64 }
shift_ops_for_integer_unions_impl! { ShortUnion, u128 }
shift_ops_for_integer_unions_impl! { ShortUnion, usize }

shift_ops_for_integer_unions_impl! { IntUnion, i8 }
shift_ops_for_integer_unions_impl! { IntUnion, i16 }
shift_ops_for_integer_unions_impl! { IntUnion, i32 }
shift_ops_for_integer_unions_impl! { IntUnion, i64 }
shift_ops_for_integer_unions_impl! { IntUnion, i128 }
shift_ops_for_integer_unions_impl! { IntUnion, isize }

shift_ops_for_integer_unions_impl! { IntUnion, u8 }
shift_ops_for_integer_unions_impl! { IntUnion, u16 }
shift_ops_for_integer_unions_impl! { IntUnion, u32 }
shift_ops_for_integer_unions_impl! { IntUnion, u64 }
shift_ops_for_integer_unions_impl! { IntUnion, u128 }
shift_ops_for_integer_unions_impl! { IntUnion, usize }

shift_ops_for_integer_unions_impl! { LongUnion, i8 }
shift_ops_for_integer_unions_impl! { LongUnion, i16 }
shift_ops_for_integer_unions_impl! { LongUnion, i32 }
shift_ops_for_integer_unions_impl! { LongUnion, i64 }
shift_ops_for_integer_unions_impl! { LongUnion, i128 }
shift_ops_for_integer_unions_impl! { LongUnion, isize }

shift_ops_for_integer_unions_impl! { LongUnion, u8 }
shift_ops_for_integer_unions_impl! { LongUnion, u16 }
shift_ops_for_integer_unions_impl! { LongUnion, u32 }
shift_ops_for_integer_unions_impl! { LongUnion, u64 }
shift_ops_for_integer_unions_impl! { LongUnion, u128 }
shift_ops_for_integer_unions_impl! { LongUnion, usize }

shift_ops_for_integer_unions_impl! { LongerUnion, i8 }
shift_ops_for_integer_unions_impl! { LongerUnion, i16 }
shift_ops_for_integer_unions_impl! { LongerUnion, i32 }
shift_ops_for_integer_unions_impl! { LongerUnion, i64 }
shift_ops_for_integer_unions_impl! { LongerUnion, i128 }
shift_ops_for_integer_unions_impl! { LongerUnion, isize }

shift_ops_for_integer_unions_impl! { LongerUnion, u8 }
shift_ops_for_integer_unions_impl! { LongerUnion, u16 }
shift_ops_for_integer_unions_impl! { LongerUnion, u32 }
shift_ops_for_integer_unions_impl! { LongerUnion, u64 }
shift_ops_for_integer_unions_impl! { LongerUnion, u128 }
shift_ops_for_integer_unions_impl! { LongerUnion, usize }

display_for_integer_unions_impl! { ShortUnion }
display_for_integer_unions_impl! { IntUnion }
display_for_integer_unions_impl! { LongUnion }
display_for_integer_unions_impl! { LongerUnion }
display_for_integer_unions_impl! { SizeUnion }


impl Debug for ShortUnion
{
    /// Formats the value using the given formatter.
    /// 
    /// # Features
    /// When used with format specifier :?, the output is printed for debug.
    /// When used with the alternate format specifier #?, the output is
    /// pretty-printed.
    /// 
    /// # Example for the format specifier :?
    /// ```
    /// use cryptocol::number::*;
    /// let a_short = ShortUnion::new_with_signed(-12345_i16);
    /// println!("a_short = {:?}", a_short);
    /// #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_short:?}"), "ShortUnion { this: 53191, that: -12345, ushort: 53191, sshort: -12345, ubyte: [199, 207], sbyte: [-57, -49] }");
    /// ```
    /// 
    /// # Example for the format specifier :#?
    /// ```
    /// use cryptocol::number::*;
    /// let a_short = ShortUnion::new_with_signed(-12345_i16);
    /// println!("a_short = {:#?}", a_short);
    /// #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_short:#?}"), r#"ShortUnion {
    ///     this: 53191,
    ///     that: -12345,
    ///     ushort: 53191,
    ///     sshort: -12345,
    ///     ubyte: [
    ///         199,
    ///         207,
    ///     ],
    ///     sbyte: [
    ///         -57,
    ///         -49,
    ///     ],
    /// }"#);
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// This method works exactly the same way as the normal method fmt() of
    /// Debug. So, all the description of this method is mainly the
    /// same as that of the implementation of the method fmt() of Debug for the
    /// primitive unsigned integer types except example codes. Confer to the
    /// descryptions that are linked to in the section _Reference_. This
    /// plagiarism is not made maliciously but is made for the reason of
    /// effectiveness and efficiency so that users may understand better and
    /// easily how to use this method with simiilarity to the method
    /// Debug() of implementation for the primitive unsigned integer
    /// types.
    /// 
    /// # References
    /// - If you want to know about the method of Debug for the primitive type,
    /// read [here](https://doc.rust-lang.org/std/fmt/trait.Debug.html).
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let mut ff = f.debug_struct("ShortUnion");
        ff.field("this", &self.get())
            .field("that", &self.get_signed())
            .field("ushort", &self.get_ushort())
            .field("sshort", &self.get_sshort())
            .field("ubyte",  &[self.get_ubyte_(0), self.get_ubyte_(1)])
            .field("sbyte",  &[self.get_sbyte_(0), self.get_sbyte_(1)]);
         #[cfg(target_pointer_width = "16")] ff.field("u_size", unsafe { &self.get_usize() } )
                                                .field("s_size", unsafe { &self.get_ssize() } );
         #[cfg(target_pointer_width = "8")] ff.field("u_size", unsafe { &[self.get_usize(0), self.get_usize(1)] } )
                                                .field("s_size", unsafe { &[self.get_ssize(0), self.get_isize(1)] } );
         ff.finish()
    }
}


impl Debug for IntUnion
{
    /// Formats the value using the given formatter.
    /// 
    /// # Features
    /// When used with format specifier :?, the output is printed for debug.
    /// When used with the alternate format specifier #?, the output is
    /// pretty-printed.
    /// 
    /// # Example for the format specifier :?
    /// ```
    /// use cryptocol::number::*;
    /// let a_int = IntUnion::new_with_signed(-1234567890_i32);
    /// println!("a_int = {:?}", a_int);
    /// #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_int:?}"), "IntUnion { this: 3060399406, that: -1234567890, uint: 3060399406, sint: -1234567890, ushort: [64814, 46697], sshort: [-722, -18839], ubyte: [46, 253, 105, 182], sbyte: [46, -3, 105, -74] }");
    /// ```
    /// 
    /// # Example for the format specifier :#?
    /// ```
    /// use cryptocol::number::*;
    /// let a_int = IntUnion::new_with_signed(-1234567890_i32);
    /// println!("a_int = {:#?}", a_int);
    /// #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_int:#?}"), r#"IntUnion {
    ///     this: 3060399406,
    ///     that: -1234567890,
    ///     uint: 3060399406,
    ///     sint: -1234567890,
    ///     ushort: [
    ///         64814,
    ///         46697,
    ///     ],
    ///     sshort: [
    ///         -722,
    ///         -18839,
    ///     ],
    ///     ubyte: [
    ///         46,
    ///         253,
    ///         105,
    ///         182,
    ///     ],
    ///     sbyte: [
    ///         46,
    ///         -3,
    ///         105,
    ///         -74,
    ///     ],
    /// }"#);
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// This method works exactly the same way as the normal method fmt() of
    /// Debug. So, all the description of this method is mainly the
    /// same as that of the implementation of the method fmt() of Debug for the
    /// primitive unsigned integer types except example codes. Confer to the
    /// descryptions that are linked to in the section _Reference_. This
    /// plagiarism is not made maliciously but is made for the reason of
    /// effectiveness and efficiency so that users may understand better and
    /// easily how to use this method with simiilarity to the method
    /// Debug() of implementation for the primitive unsigned integer
    /// types.
    /// 
    /// # References
    /// - If you want to know about the method of Debug for the primitive type,
    /// read [here](https://doc.rust-lang.org/std/fmt/trait.Debug.html).
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let mut ff = f.debug_struct("IntUnion");
        ff.field("this", &self.get())
            .field("that", &self.get_signed())
            .field("uint", &self.get_uint())
            .field("sint", &self.get_sint())
            .field("ushort", &[self.get_ushort_(0), self.get_ushort_(1)])
            .field("sshort", &[self.get_sshort_(0),  self.get_sshort_(1)])
            .field("ubyte", &[self.get_ubyte_(0), self.get_ubyte_(1), self.get_ubyte_(2), self.get_ubyte_(3)])
            .field("sbyte", &[self.get_sbyte_(0), self.get_sbyte_(1), self.get_sbyte_(2), self.get_sbyte_(3)]);
        #[cfg(target_pointer_width = "32")] ff.field("u_size", &self.get_usize())
                                                .field("s_size", &self.get_ssize());
        #[cfg(target_pointer_width = "16")] ff.field("u_size", &[self.get_usize_(0), self.get_usize_(1)])
                                                .field("s_size", &[self.get_ssize_(0), self.get_ssize_(1)]);
        #[cfg(target_pointer_width = "8")] ff.field("u_size", &[self.get_usize_(0), self.get_usize_(1), self.get_usize_(2), self.get_usize_(3)])
                                                .field("s_size", &[self.get_ssize_(0), self.get_ssize_(1), self.get_ssize_(2), self.get_ssize_(3)]);
        ff.finish()
    }
}

impl Debug for LongUnion
{
    /// Formats the value using the given formatter.
    /// 
    /// # Features
    /// When used with format specifier :?, the output is printed for debug.
    /// When used with the alternate format specifier #?, the output is
    /// pretty-printed.
    /// 
    /// # Example for the format specifier :?
    /// ```
    /// use cryptocol::number::*;
    /// let a_long = LongUnion::new_with_signed(-1234567890123456789_i64);
    /// println!("a_long = {:?}", a_long);
    /// #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_long:?}"), "LongUnion { this: 17212176183586094827, that: -1234567890123456789, ulong: 17212176183586094827, slong: -1234567890123456789, uint: [2182512363, 4007522059], sint: [-2112454933, -287445237], ushort: [32491, 33302, 61195, 61149], sshort: [32491, -32234, -4341, -4387], ubyte: [235, 126, 22, 130, 11, 239, 221, 238], sbyte: [-21, 126, 22, -126, 11, -17, -35, -18], u_size: 17212176183586094827, s_size: -1234567890123456789 }");
    /// ```
    /// 
    /// # Example for the format specifier :#?
    /// ```
    /// use cryptocol::number::*;
    /// let a_long = LongUnion::new_with_signed(-1234567890123456789_i64);
    /// println!("a_long = {:#?}", a_long);
    /// #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_long:#?}"), r#"LongUnion {
    ///     this: 17212176183586094827,
    ///     that: -1234567890123456789,
    ///     ulong: 17212176183586094827,
    ///     slong: -1234567890123456789,
    ///     uint: [
    ///         2182512363,
    ///         4007522059,
    ///     ],
    ///     sint: [
    ///         -2112454933,
    ///         -287445237,
    ///     ],
    ///     ushort: [
    ///         32491,
    ///         33302,
    ///         61195,
    ///         61149,
    ///     ],
    ///     sshort: [
    ///         32491,
    ///         -32234,
    ///         -4341,
    ///         -4387,
    ///     ],
    ///     ubyte: [
    ///         235,
    ///         126,
    ///         22,
    ///         130,
    ///         11,
    ///         239,
    ///         221,
    ///         238,
    ///     ],
    ///     sbyte: [
    ///         -21,
    ///         126,
    ///         22,
    ///         -126,
    ///         11,
    ///         -17,
    ///         -35,
    ///         -18,
    ///     ],
    ///     u_size: 17212176183586094827,
    ///     s_size: -1234567890123456789,
    /// }"#);
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// This method works exactly the same way as the normal method fmt() of
    /// Debug. So, all the description of this method is mainly the
    /// same as that of the implementation of the method fmt() of Debug for the
    /// primitive unsigned integer types except example codes. Confer to the
    /// descryptions that are linked to in the section _Reference_. This
    /// plagiarism is not made maliciously but is made for the reason of
    /// effectiveness and efficiency so that users may understand better and
    /// easily how to use this method with simiilarity to the method
    /// Debug() of implementation for the primitive unsigned integer
    /// types.
    /// 
    /// # References
    /// - If you want to know about the method of Debug for the primitive type,
    /// read [here](https://doc.rust-lang.org/std/fmt/trait.Debug.html).
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let mut ff = f.debug_struct("LongUnion");
        ff.field("this", &self.get())
            .field("that", &self.get_signed())
            .field("ulong", &self.get_ulong())
            .field("slong", &self.get_slong())
            .field("uint", &[self.get_uint_(0), self.get_uint_(1)])
            .field("sint", &[self.get_sint_(0), self.get_sint_(1)])
            .field("ushort", &[self.get_ushort_(0), self.get_ushort_(1), self.get_ushort_(2), self.get_ushort_(3)])
            .field("sshort", &[self.get_sshort_(0), self.get_sshort_(1), self.get_sshort_(2), self.get_sshort_(3)])
            .field("ubyte", &[self.get_ubyte_(0), self.get_ubyte_(1), self.get_ubyte_(2), self.get_ubyte_(3), self.get_ubyte_(4), self.get_ubyte_(5), self.get_ubyte_(6), self.get_ubyte_(7)])
            .field("sbyte", &[self.get_sbyte_(0), self.get_sbyte_(1), self.get_sbyte_(2), self.get_sbyte_(3), self.get_sbyte_(4), self.get_sbyte_(5), self.get_sbyte_(6), self.get_sbyte_(7)]);
        #[cfg(target_pointer_width = "64")] ff.field("u_size", &self.get_usize())
                                                .field("s_size", &self.get_ssize());
        #[cfg(target_pointer_width = "32")] ff.field("u_size", &[self.get_usize_(0), self.get_usize_(1)])
                                                .field("s_size", &[self.get_ssize_(0), self.get_ssize_(1)]);
        #[cfg(target_pointer_width = "16")] ff.field("u_size", &[self.get_usize_(0), self.get_usize_(1), self.get_usize_(2), self.get_usize_(3)])
                                                .field("s_size", &[self.get_ssize_(0), self.get_ssize_(1), self.get_ssize_(2), self.get_ssize_(3)]);
        #[cfg(target_pointer_width = "8")] ff.field("u_size", &[self.get_usize_(0), self.get_usize_(1), self.get_usize_(2), self.get_usize_(3), self.get_usize_(4), self.get_usize_(5), self.get_usize_(6), self.get_usize_(7)])
                                                .field("s_size", &[self.get_ssize_(0), self.get_ssize_(1), self.get_ssize_(2), self.get_ssize_(3), self.get_ssize_(4), self.get_ssize_(5), self.get_ssize_(6), self.get_ssize_(7)]);
        ff.finish()
    }
}

impl Debug for LongerUnion
{
    /// Formats the value using the given formatter.
    /// 
    /// # Features
    /// When used with format specifier :?, the output is printed for debug.
    /// When used with the alternate format specifier #?, the output is
    /// pretty-printed.
    /// 
    /// # Example for the format specifier :?
    /// ```
    /// use cryptocol::number::*;
    /// let a_longer = LongerUnion::new_with_signed(-123456789012345678901234567890123456789_i128);
    /// println!("a_longer = {:?}", a_longer);
    /// #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_longer:?}"), "LongerUnion { this: 216825577908592784562140039541644754667, that: -123456789012345678901234567890123456789, ulonger: 216825577908592784562140039541644754667, slonger: -123456789012345678901234567890123456789, ulong: [6134004772338302699, 11754138130946064698], slong: [6134004772338302699, -6692605942763486918], uint: [1371963115, 1428184279, 2682913082, 2736723546], sint: [1371963115, 1428184279, -1612054214, -1558243750], ushort: [32491, 20934, 23767, 21792, 314, 40938, 5722, 41759], sshort: [32491, 20934, 23767, 21792, 314, -24598, 5722, -23777], ubyte: [235, 126, 198, 81, 215, 92, 32, 85, 58, 1, 234, 159, 90, 22, 31, 163], sbyte: [-21, 126, -58, 81, -41, 92, 32, 85, 58, 1, -22, -97, 90, 22, 31, -93], u_size: [6134004772338302699, 11754138130946064698], s_size: [6134004772338302699, -6692605942763486918] }");
    /// ```
    /// 
    /// # Example for the format specifier :#?
    /// ```
    /// use cryptocol::number::*;
    /// let a_longer = LongerUnion::new_with_signed(-123456789012345678901234567890123456789_i128);
    /// println!("a_longer = {:#?}", a_longer);
    /// #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_longer:#?}"), r#"LongerUnion {
    ///     this: 216825577908592784562140039541644754667,
    ///     that: -123456789012345678901234567890123456789,
    ///     ulonger: 216825577908592784562140039541644754667,
    ///     slonger: -123456789012345678901234567890123456789,
    ///     ulong: [
    ///         6134004772338302699,
    ///         11754138130946064698,
    ///     ],
    ///     slong: [
    ///         6134004772338302699,
    ///         -6692605942763486918,
    ///     ],
    ///     uint: [
    ///         1371963115,
    ///         1428184279,
    ///         2682913082,
    ///         2736723546,
    ///     ],
    ///     sint: [
    ///         1371963115,
    ///         1428184279,
    ///         -1612054214,
    ///         -1558243750,
    ///     ],
    ///     ushort: [
    ///         32491,
    ///         20934,
    ///         23767,
    ///         21792,
    ///         314,
    ///         40938,
    ///         5722,
    ///         41759,
    ///     ],
    ///     sshort: [
    ///         32491,
    ///         20934,
    ///         23767,
    ///         21792,
    ///         314,
    ///         -24598,
    ///         5722,
    ///         -23777,
    ///     ],
    ///     ubyte: [
    ///         235,
    ///         126,
    ///         198,
    ///         81,
    ///         215,
    ///         92,
    ///         32,
    ///         85,
    ///         58,
    ///         1,
    ///         234,
    ///         159,
    ///         90,
    ///         22,
    ///         31,
    ///         163,
    ///     ],
    ///     sbyte: [
    ///         -21,
    ///         126,
    ///         -58,
    ///         81,
    ///         -41,
    ///         92,
    ///         32,
    ///         85,
    ///         58,
    ///         1,
    ///         -22,
    ///         -97,
    ///         90,
    ///         22,
    ///         31,
    ///         -93,
    ///     ],
    ///     u_size: [
    ///         6134004772338302699,
    ///         11754138130946064698,
    ///     ],
    ///     s_size: [
    ///         6134004772338302699,
    ///         -6692605942763486918,
    ///     ],
    /// }"#);
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// This method works exactly the same way as the normal method fmt() of
    /// Debug. So, all the description of this method is mainly the
    /// same as that of the implementation of the method fmt() of Debug for the
    /// primitive unsigned integer types except example codes. Confer to the
    /// descryptions that are linked to in the section _Reference_. This
    /// plagiarism is not made maliciously but is made for the reason of
    /// effectiveness and efficiency so that users may understand better and
    /// easily how to use this method with simiilarity to the method
    /// Debug() of implementation for the primitive unsigned integer
    /// types.
    /// 
    /// # References
    /// - If you want to know about the method of Debug for the primitive type,
    /// read [here](https://doc.rust-lang.org/std/fmt/trait.Debug.html).
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let mut ff = f.debug_struct("LongerUnion");
        ff.field("this", &self.get())
            .field("that", &self.get_signed())
            .field("ulonger", &self.get_ulonger())
            .field("slonger", &self.get_slonger())
            .field("ulong", &[self.get_ulong_(0), self.get_ulong_(1)])
            .field("slong", &[self.get_slong_(0), self.get_slong_(1)])
            .field("uint", &[self.get_uint_(0), self.get_uint_(1), self.get_uint_(2), self.get_uint_(3)])
            .field("sint", &[self.get_sint_(0), self.get_sint_(1), self.get_sint_(2), self.get_sint_(3)])
            .field("ushort", &[self.get_ushort_(0), self.get_ushort_(1), self.get_ushort_(2), self.get_ushort_(3), self.get_ushort_(4), self.get_ushort_(5), self.get_ushort_(6), self.get_ushort_(7)])
            .field("sshort", &[self.get_sshort_(0), self.get_sshort_(1), self.get_sshort_(2), self.get_sshort_(3), self.get_sshort_(4), self.get_sshort_(5), self.get_sshort_(6), self.get_sshort_(7)])
            .field("ubyte", &[self.get_ubyte_(0), self.get_ubyte_(1), self.get_ubyte_(2), self.get_ubyte_(3), self.get_ubyte_(4), self.get_ubyte_(5), self.get_ubyte_(6), self.get_ubyte_(7), self.get_ubyte_(8), self.get_ubyte_(9), self.get_ubyte_(10), self.get_ubyte_(11), self.get_ubyte_(12), self.get_ubyte_(13), self.get_ubyte_(14), self.get_ubyte_(15)])
            .field("sbyte", &[self.get_sbyte_(0), self.get_sbyte_(1), self.get_sbyte_(2), self.get_sbyte_(3), self.get_sbyte_(4), self.get_sbyte_(5), self.get_sbyte_(6), self.get_sbyte_(7), self.get_sbyte_(8), self.get_sbyte_(9), self.get_sbyte_(10), self.get_sbyte_(11), self.get_sbyte_(12), self.get_sbyte_(13), self.get_sbyte_(14), self.get_sbyte_(15)]);
        #[cfg(target_pointer_width = "128")] ff.field("u_size",  &self.get_usize())
                                                .field("s_size", &self.get_ssize());
        #[cfg(target_pointer_width = "64")] ff.field("u_size", &[self.get_usize_(0), self.get_usize_(1)])
                                                .field("s_size", &[self.get_ssize_(0), self.get_ssize_(1)]);
        #[cfg(target_pointer_width = "32")] ff.field("u_size", &[self.get_usize_(0), self.get_usize_(1), self.get_usize_(2), self.get_usize_(3)])
                                                .field("s_size", &[self.get_ssize_(0), self.get_ssize_(1), self.get_ssize_(2), self.get_ssize_(3)]);
        #[cfg(target_pointer_width = "16")] ff.field("u_size", &[self.get_usize_(0), self.get_usize_(1), self.get_usize_(2), self.get_usize_(3), self.get_usize_(4), self.get_usize_(5), self.get_usize_(6), self.get_usize_(7)])
                                                .field("s_size", &[self.get_ssize_(0), self.get_ssize_(1), self.get_ssize_(2), self.get_ssize_(3), self.get_ssize_(4), self.get_ssize_(5), self.get_ssize_(6), self.get_ssize_(7)]);
        #[cfg(target_pointer_width = "8")] ff.field("u_size", &[self.get_usize_(0), self.get_usize_(1), self.get_usize_(2), self.get_usize_(3), self.get_usize_(4), self.get_usize_(5), self.get_usize_(6), self.get_usize_(7), self.get_usize_(8), self.get_usize_(9), self.get_usize_(10), self.get_usize_(11), self.get_usize_(12), self.get_usize_(13), self.get_usize_(14), self.get_usize_(15)])
                                                .field("s_size", &[self.get_ssize_(0), self.get_ssize_(1), self.get_ssize_(2), self.get_ssize_(3), self.get_ssize_(4), self.get_ssize_(5), self.get_ssize_(6), self.get_ssize_(7), self.get_ssize_(8), self.get_ssize_(9), self.get_ssize_(10), self.get_ssize_(11), self.get_ssize_(12), self.get_ssize_(13), self.get_ssize_(14), self.get_ssize_(15)]);
        ff.finish()
    }
}


impl Debug for SizeUnion
{
    /// Formats the value using the given formatter.
    /// 
    /// # Features
    /// When used with format specifier :?, the output is printed for debug.
    /// When used with the alternate format specifier #?, the output is
    /// pretty-printed.
    /// 
    /// # Example for the format specifier :?
    /// ```
    /// use cryptocol::number::*;
    /// let a_size = SizeUnion::new_with_signed(-1234567890123456789_isize);
    /// println!("a_size = {:?}", a_size);
    /// #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_size:?}"), "SizeUnion { this: 17212176183586094827, that: -1234567890123456789, u_size: 17212176183586094827, s_size: -1234567890123456789, ulong: 17212176183586094827, slong: -1234567890123456789, uint: [2182512363, 4007522059], sint: [-2112454933, -287445237], ushort: [32491, 33302, 61195, 61149], sshort: [32491, -32234, -4341, -4387], ubyte: [235, 126, 22, 130, 11, 239, 221, 238], sbyte: [-21, 126, 22, -126, 11, -17, -35, -18] }");
    /// ```
    /// 
    /// # Example for the format specifier :#?
    /// ```
    /// use cryptocol::number::*;
    /// let a_size = SizeUnion::new_with_signed(-1234567890123456789_isize);
    /// println!("a_size = {:#?}", a_size);
    /// #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_size:#?}"), r#"SizeUnion {
    ///     this: 17212176183586094827,
    ///     that: -1234567890123456789,
    ///     u_size: 17212176183586094827,
    ///     s_size: -1234567890123456789,
    ///     ulong: 17212176183586094827,
    ///     slong: -1234567890123456789,
    ///     uint: [
    ///         2182512363,
    ///         4007522059,
    ///     ],
    ///     sint: [
    ///         -2112454933,
    ///         -287445237,
    ///     ],
    ///     ushort: [
    ///         32491,
    ///         33302,
    ///         61195,
    ///         61149,
    ///     ],
    ///     sshort: [
    ///         32491,
    ///         -32234,
    ///         -4341,
    ///         -4387,
    ///     ],
    ///     ubyte: [
    ///         235,
    ///         126,
    ///         22,
    ///         130,
    ///         11,
    ///         239,
    ///         221,
    ///         238,
    ///     ],
    ///     sbyte: [
    ///         -21,
    ///         126,
    ///         22,
    ///         -126,
    ///         11,
    ///         -17,
    ///         -35,
    ///         -18,
    ///     ],
    /// }"#);
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// This method works exactly the same way as the normal method fmt() of
    /// Debug. So, all the description of this method is mainly the
    /// same as that of the implementation of the method fmt() of Debug for the
    /// primitive unsigned integer types except example codes. Confer to the
    /// descryptions that are linked to in the section _Reference_. This
    /// plagiarism is not made maliciously but is made for the reason of
    /// effectiveness and efficiency so that users may understand better and
    /// easily how to use this method with simiilarity to the method
    /// Debug() of implementation for the primitive unsigned integer
    /// types.
    /// 
    /// # References
    /// - If you want to know about the method of Debug for the primitive type,
    /// read [here](https://doc.rust-lang.org/std/fmt/trait.Debug.html).
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let mut ff = f.debug_struct("SizeUnion");
        ff.field("this", &self.get())
            .field("that", &self.get_signed())
            .field("u_size", &self.get_usize())
            .field("s_size", &self.get_ssize());

        #[cfg(target_pointer_width = "128")]
        ff.field("ulonger", unsafe { &self.ulonger } )
            .field("slonger", unsafe { &self.slonger } )
            .field("ulong", unsafe { &self.ulong } )
            .field("slong", unsafe { &self.slong } )
            .field("uint", unsafe { &self.uint } )
            .field("sint", unsafe { &self.sint } )
            .field("ushort", unsafe { &self.ushort } )
            .field("sshort", unsafe { &self.sshort } )
            .field("ubyte", unsafe { &self.ubyte } )
            .field("sbyte", unsafe { &self.sbyte } );

        #[cfg(target_pointer_width = "64")] 
        ff.field("ulong", unsafe { &self.ulong } )
            .field("slong", unsafe { &self.slong } )
            .field("uint", unsafe { &self.uint } )
            .field("sint", unsafe { &self.sint } )
            .field("ushort", unsafe { &self.ushort } )
            .field("sshort", unsafe { &self.sshort } )
            .field("ubyte", unsafe { &self.ubyte } )
            .field("sbyte", unsafe { &self.sbyte } );

        #[cfg(target_pointer_width = "32")]
        ff.field("uint", unsafe { &self.uint } )
            .field("sint", unsafe { &self.sint } )
            .field("ushort", unsafe { &self.ushort } )
            .field("sshort", unsafe { &self.sshort } )
            .field("ubyte", unsafe { &self.ubyte } )
            .field("sbyte", unsafe { &self.sbyte } );

        #[cfg(target_pointer_width = "16")]
        ff.field("ushort", unsafe { &self.ushort } )
            .field("sshort", unsafe { &self.sshort } )
            .field("ubyte", unsafe { &self.ubyte } )
            .field("sbyte", unsafe { &self.sbyte } );

        #[cfg(target_pointer_width = "8")]
        ff.field("ubyte", unsafe { &self.ubyte } )
            .field("sbyte", unsafe { &self.sbyte } );

        ff.finish()
    }
}
