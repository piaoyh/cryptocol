// Copyright 2023, 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

// #![warn(missing_docs)]
// #![warn(missing_doc_code_examples)]
#![allow(missing_docs)]
#![allow(rustdoc::missing_doc_code_examples)]

use std::mem::{ size_of, size_of_val };

use crate::number::{ SmallUInt, ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };

macro_rules! SmallUInt_methods_for_uint_impl {
    ($f:ty) => {
        impl SmallUInt for $f
        {
            /// Calculates `self` + `rhs` + `carry` and returns a tuple
            /// containing the sum and the output carry.
            /// [Read more](trait@SmallUInt#tymethod.carrying_add) in detail.
            fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool)
            {
                let (r1, c1) = self.overflowing_add(rhs);
                let (r2, c2) = r1.overflowing_add(carry as Self);
                (r2, c1 || c2)
            }

            /// Computes `self` + `rhs`, wrapping around at the boundary of
            /// the type.
            /// [Read more](trait@SmallUInt#tymethod.wrapping_add) in detail.
            #[inline] fn wrapping_add(self, rhs: Self) -> Self  { self.wrapping_add(rhs) }

            /// Calculates `self` + `rhs` and returns a tuple of the addition
            /// along with a boolean indicating whether an arithmetic overflow
            /// would occur.
            /// [Read more](trait@SmallUInt#tymethod.overflowing_add) in detail.
            #[inline] fn overflowing_add(self, rhs: Self) -> (Self, bool)   { self.overflowing_add(rhs) }

            /// Computes `self` + `rhs` and returns None if overflow occurred.
            /// [Read more](trait@SmallUInt#tymethod.checked_add) in detail.
            #[inline] fn checked_add(self, rhs: Self) -> Option<Self>   { self.checked_add(rhs) }

            /// Computes `self` + `rhs` and returns None if overflow occurred.
            /// [Read more](trait@SmallUInt#tymethod.checked_add) in detail.
            #[inline] fn unchecked_add(self, rhs: Self) -> Self     { self.checked_add(rhs).unwrap() }

            /// Computes `self` + `rhs`, saturating at the numeric bounds
            /// instead of overflowing.
            /// [Read more](trait@SmallUInt#tymethod.saturating_add) in detail.
            #[inline] fn saturating_add(self, rhs: Self) -> Self    { self.saturating_add(rhs) }

            /// Computes `self` + `rhs`, wrapping at `modulo`
            /// instead of overflowing.
            /// [Read more](trait@SmallUInt#tymethod.modular_add) in detail.
            fn modular_add(self, rhs: Self, modulo: Self) -> Self
            {
                let mlhs = self.wrapping_rem(modulo);
                let mrhs = rhs.wrapping_rem(modulo);
                let diff = modulo.wrapping_sub(mrhs);
                if self >= diff
                    { mlhs.wrapping_sub(diff) }
                else
                    { mlhs.wrapping_add(mrhs) }
            }

            /// Calculates `self` - `rhs` - `borrow`,
            /// wrapping around at the boundary of the type.
            /// [Read more](trait@SmallUInt#tymethod.borrowing_sub) in detail.
            fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool)
            {
                let (r1, b1) = self.overflowing_sub(rhs);
                let (r2, b2) = r1.overflowing_sub(borrow as Self);
                (r2, b1 || b2)
            }

            /// Computes `self` - `rhs`, wrapping around at the boundary of
            /// the type. [Read more](trait@SmallUInt#tymethod.wrapping_sub)
            /// in detail.
            #[inline] fn wrapping_sub(self, rhs: Self) -> Self  { self.wrapping_sub(rhs) }

            /// Calculates `self` - `rhs` and returns a tuple of the subtraction
            /// along with a boolean indicating whether an arithmetic overflow
            /// would occur.
            /// [Read more](trait@SmallUInt#tymethod.overflowing_sub) in detail.
            #[inline] fn overflowing_sub(self, rhs: Self) -> (Self, bool)   { self.overflowing_sub(rhs) }

            /// Computes `self` - `rhs`, returning None if overflow occurred.
            /// [Read more](trait@SmallUInt#tymethod.checked_sub) in detail.
            #[inline] fn checked_sub(self, rhs: Self) -> Option<Self>   { self.checked_sub(rhs) }

            /// Computes `self` - `rhs`, assuming overflow cannot occur.
            /// [Read more](trait@SmallUInt#tymethod.unchecked_sub) in detail.
            #[inline] fn unchecked_sub(self, rhs: Self) -> Self     { self.checked_sub(rhs).unwrap() }

            /// Computes `self` - `rhs`, saturating at the numeric bounds
            /// instead of overflowing.
            /// [Read more](trait@SmallUInt#tymethod.saturating_sub) in detail.
            #[inline] fn saturating_sub(self, rhs: Self) -> Self    { self.saturating_sub(rhs) }

            /// Computes the absolute difference between self and other.
            /// [Read more](trait@SmallUInt#tymethod.abs_diff) in detail.
            #[inline] fn abs_diff(self, other: Self) -> Self    { self.abs_diff(other) }

            /// Computes `self` - `rhs`, wrapping at `modulo`
            /// instead of overflowing.
            /// [Read more](trait@SmallUInt#tymethod.modular_sub) in detail.
            fn modular_sub(self, rhs: Self, modulo: Self) -> Self
            {
                let mlhs = self.wrapping_rem(modulo);
                let mrhs = rhs.wrapping_rem(modulo);
                if mlhs >= mrhs
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
            /// [Read more](trait@SmallUInt#tymethod.carrying_mul) in detail.
            #[inline] fn carrying_mul(self, rhs: Self, carry: Self) -> (Self, Self)
            {
                //  _carrying_mul(self, rhs, carry)
                let overflow;
                let (mut low, mut high) = SmallUInt::widening_mul(self, rhs);
                (low, overflow) = low.overflowing_add(carry);
                if overflow
                    { high = high.wrapping_add(1); }
                (low, high)
            }

            // // fn carrying_mul_for_internal_use(self, rhs: Self, carry: Self) -> (Self, Self);
            // /// It is for internal use. You are recommended to use
            // /// [carrying_mul()](trait@SmallUInt#tymethod.carrying_mul) instead.
            // fn _carrying_mul(self, rhs: Self, carry: Self) -> (Self, Self)
            // {
            //     let overflow;
            //     let (mut low, mut high) = self._widening_mul(rhs);
            //     (low, overflow) = low.overflowing_add(carry);
            //     if overflow
            //         { high = high.wrapping_add(1); }
            //     (low, high)
            // }

            /// Calculates the complete product `self` * `rhs` without the
            /// possibility to overflow.
            /// [Read more](trait@SmallUInt#tymethod.widening_mul) in detail.
            #[inline] fn widening_mul(self, rhs: Self) -> (Self, Self)
            {
                // _widening_mul(self, rhs)
                let zero = SmallUInt::zero();
                if rhs.is_zero() || self.is_zero()
                    { return (zero, zero); }
        
                let mut low = zero;
                let mut high = zero;
                let mut overflow: bool;
                let mut bit_check = Self::one() << (Self::size_in_bits() - 1 - rhs.leading_zeros() as usize);
                let adder = self;
                while bit_check != 0
                {
                    high <<= 1;
                    if low.is_msb_set()
                        { high.set_lsb(); }
                    low <<= 1;
                    if bit_check & rhs != 0
                    {
                        (low, overflow) = low.overflowing_add(adder);
                        if overflow
                            { high = high.wrapping_add(Self::one()); }
                    }
                    bit_check >>= 1;
                }
                (low, high)
            }

            // // fn carrying_mul_for_internal_use(self, rhs: Self, carry: Self) -> (Self, Self);
            // /// It is for internal use. You are recommended to use
            // /// [carrying_mul()](trait@SmallUInt#tymethod.widening_mul) instead.
            // fn _widening_mul(self, rhs: Self) -> (Self, Self)
            // {
            //     if (rhs == 0) || (self == 0)
            //         { return (0, 0); }

            //     let mut low: Self = 0;
            //     let mut high: Self = 0;
            //     let mut overflow: bool;
            //     let mut bit_check: Self = 1 << (Self::size_in_bits() - 1 - rhs.leading_zeros() as usize);
            //     let adder = self;
            //     while bit_check != 0
            //     {
            //         high <<= 1;
            //         if low.is_msb_set()
            //             { high.set_lsb(); }
            //         low <<= 1;
            //         if bit_check & rhs != 0
            //         {
            //             (low, overflow) = low.overflowing_add(adder);
            //             if overflow
            //                 { high = high.wrapping_add(1); }
            //         }
            //         bit_check >>= 1;
            //     }
            //     (low, high)
            // }

            /// Computes `self` * `rhs`, wrapping around at the boundary of
            /// the type. [Read more](trait@SmallUInt#tymethod.wrapping_mul)
            /// in detail.
            #[inline] fn wrapping_mul(self, rhs: Self) -> Self  { self.wrapping_mul(rhs) }

            /// Calculates the multiplication of `self` and `rhs` and returns
            /// a tuple of the multiplication along with a boolean indicating
            /// whether an arithmetic overflow would occur.
            /// [Read more](trait@SmallUInt#tymethod.overflowing_mul) in detail.
            #[inline] fn overflowing_mul(self, rhs: Self) -> (Self, bool)   { self.overflowing_mul(rhs) }

            /// Computes `self` * `rhs`, returning None if overflow occurred.
            /// [Read more](trait@SmallUInt#tymethod.checked_mul) in detail.
            #[inline] fn checked_mul(self, rhs: Self) -> Option<Self>   { self.checked_mul(rhs) }

            /// Computes `self` * `rhs`, assuming overflow cannot occur.
            /// [Read more](trait@SmallUInt#tymethod.unchecked_mul) in detail.
            #[inline] fn unchecked_mul(self, rhs: Self) -> Self     { self.checked_mul(rhs).unwrap() }

            /// Computes `self` * `rhs`, saturating at the numeric bounds
            /// instead of overflowing.
            /// [Read more](trait@SmallUInt#tymethod.saturating_mul) in detail.
            #[inline] fn saturating_mul(self, rhs: Self) -> Self    { self.saturating_mul(rhs) }

            /// Computes `self` * `rhs`, wrapping at `modulo`
            /// instead of overflowing.
            /// [Read more](trait@SmallUInt#tymethod.modular_mul) in detail.
            fn modular_mul(self, rhs: Self, modulo: Self) -> Self
            {
                let mut mlhs = self.wrapping_rem(modulo);
                let mut mrhs = rhs.wrapping_rem(modulo);
                let mut res = Self::zero();
                while mrhs > 0
                {
                    if mrhs.is_odd()
                        { res = res.modular_add(mlhs, modulo); }
                    mlhs = mlhs.modular_add(mlhs, modulo);
                    mrhs >>= 1;
                }
                res
            }

            /// Computes `self` / `rhs`. Wrapped division on unsigned types is
            /// just normal division.
            /// [Read more](trait@SmallUInt#tymethod.wrapping_div) in detail.
            #[inline] fn wrapping_div(self, rhs: Self) -> Self  { self.wrapping_div(rhs) }

            /// Calculates the divisor when `self` is divided by `rhs` and
            /// returns a tuple of the divisor along with a boolean indicating
            /// whether an arithmetic overflow would occur.
            /// [Read more](trait@SmallUInt#tymethod.overflowing_div) in detail.
            #[inline] fn overflowing_div(self, rhs: Self) -> (Self, bool)   { self.overflowing_div(rhs) }

            /// Computes `self` / `rhs`, returning `None` if `rhs` == 0.
            /// [Read more](trait@SmallUInt#tymethod.checked_div) in detail.
            #[inline] fn checked_div(self, rhs: Self) -> Option<Self>   { self.checked_div(rhs) }

            /// Computes `self` / `rhs`, if `rhs` != 0.
            /// [Read more](trait@SmallUInt#tymethod.checked_div) in detail.
            #[inline] fn unchecked_div(self, rhs: Self) -> Self     { self.checked_div(rhs).unwrap() }

            /// Computes `self` / `rhs`, saturating at the numeric bounds
            /// instead of overflowing.
            /// [Read more](trait@SmallUInt#tymethod.saturating_div) in detail.
            #[inline] fn saturating_div(self, rhs: Self) -> Self    { self.saturating_div(rhs) }

            /// Computes `self` % `rhs`. Wrapped remainder calculation on unsigned
            /// types is just the regular remainder calculation.
            /// [Read more](trait@SmallUInt#tymethod.wrapping_rem) in detail.
            #[inline] fn wrapping_rem(self, rhs: Self) -> Self  { self.wrapping_rem(rhs) }

            /// Calculates the remainder when `self` is divided by `rhs`, and returns
            /// a tuple of the remainder after dividing along with a boolean
            /// indicating whether an arithmetic overflow would occur.
            /// [Read more](trait@SmallUInt#tymethod.overflowing_rem) in detail.
            #[inline] fn overflowing_rem(self, rhs: Self) -> (Self, bool)   { self.overflowing_rem(rhs) }

            /// Computes `self` % `rhs`, returning None if `rhs` == 0.
            /// [Read more](trait@SmallUInt#tymethod.checked_rem) in detail.
            #[inline] fn checked_rem(self, rhs: Self) -> Option<Self>   { self.checked_rem(rhs) }

            /// Computes `self` % `rhs`, if `rhs` != 0.
            /// [Read more](trait@SmallUInt#tymethod.checked_rem) in detail.
            #[inline] fn unchecked_rem(self, rhs: Self) -> Self     { self.checked_rem(rhs).unwrap() }

            /// Computes `-self`, wrapping around at the boundary of the type.
            /// [Read more](trait@SmallUInt#tymethod.wrapping_neg) in detail.
            #[inline] fn wrapping_neg(self) -> Self     { self.wrapping_neg() }

            /// Negates `self` in an overflowing fashion.
            /// [Read more](trait@SmallUInt#tymethod.overflowing_neg) in detail.
            #[inline] fn overflowing_neg(self) -> (Self, bool)     { self.overflowing_neg() }

            /// Computes `self.pow(exp)`, wrapping around at the boundary of
            /// the type. [Read more](trait@SmallUInt#tymethod.wrapping_pow)
            /// in detail.
            #[inline] fn wrapping_pow(self, exp: u32) -> Self   { self.wrapping_pow(exp) }

            /// Raises `self` to the power of `exp`, using exponentiation by
            /// squaring. [Read more](trait@SmallUInt#tymethod.overflowing_pow)
            /// in detail.
            #[inline] fn overflowing_pow(self, exp: u32) -> (Self, bool)    { self.overflowing_pow(exp) }

            /// Computes `self.pow(exp)`, returning None if overflow occurred.
            /// [Read more](trait@SmallUInt#tymethod.checked_pow) in detail.
            #[inline] fn checked_pow(self, exp: u32) -> Option<Self>    { self.checked_pow(exp) }

            /// Computes `self.pow(exp)`, unless overflow does not occcurred.
            /// Otherwise, it will panic.
            /// [Read more](trait@SmallUInt#tymethod.unchecked_pow) in detail.
            #[inline] fn unchecked_pow(self, exp: u32) -> Self      { self.checked_pow(exp).unwrap() }

            /// Computes `self`.pow(exp), saturating at the numeric bounds
            /// instead of overflowing.
            /// [Read more](trait@SmallUInt#tymethod.saturating_pow) in detail.
            #[inline] fn saturating_pow(self, exp: u32) -> Self     { self.saturating_pow(exp) }

            /// Raises `self` to the power of `exp`, using exponentiation
            /// by squaring. [Read more](trait@SmallUInt#tymethod.pow)
            /// in detail.
            #[inline] fn pow(self, exp: u32) -> Self    { self.pow(exp) }

            /// Computes `self.pow(exp)`, saturating at `modulo`
            /// instead of overflowing.
            /// [Read more](trait@SmallUInt#tymethod.modular_pow) in detail.
            fn modular_pow(self, exp: Self, modulo: Self) -> Self
            {
                let mut mlhs = self.wrapping_rem(modulo);
                let mut res = Self::one();
                let mut mexp = exp;

                while mexp > 0
                {
                    if mexp.is_odd()
                        { res = res.modular_mul(mlhs, modulo); }
                    mlhs = mlhs.modular_mul(mlhs, modulo);
                    mexp >>= 1;
                }
                res
            }

            /// Returns the logarithm of the number with respect to an
            /// arbitrary base. [Read more](trait@SmallUInt#tymethod.ilog)
            /// in detail.
            #[inline] fn ilog(self, base: Self) -> u32  { self.ilog(base) }

            /// Returns the base 10 logarithm of the number.
            /// [Read more](trait@SmallUInt#tymethod.ilog10) in detail.
            #[inline] fn ilog10(self) -> u32    { self.ilog10() }

            /// Returns the base 2 logarithm of the number, rounded down.
            /// [Read more](trait@SmallUInt#tymethod.ilog2) in detail.
            #[inline] fn ilog2(self) -> u32     { self.ilog2() }

            /// Returns the square root of the number.
            /// [Read more](trait@SmallUInt#tymethod.isqrt) in detail.
            #[inline] fn isqrt(self) -> Self
            {
                // _isqrt(self)
                let mut adder;
                let mut highest = (Self::size_in_bits() - self.leading_zeros() as usize) >> 1;
                let mut high;
                let mut low;
                let mut mid;
                let mut res = Self::zero();
                let mut sum;
                let maximum = highest - 1;
                loop
                {
                    high = highest;
                    low = 0;
                    if high == 0
                    {
                        return res;
                    }
                    else    // if high > 0
                    {
                        loop
                        {
                            mid = (high + low) >> 1;
                            adder = Self::generate_check_bits_(mid);
                            sum = res + adder;
                            let (sq, b_overflow) = sum.overflowing_mul(sum);
                            if !b_overflow && (sq < self)
                            {
                                if mid == maximum
                                {
                                    res = sum;
                                    break;
                                }
                                else if mid == low
                                { 
                                    res = sum;
                                    if mid == 0
                                        { highest = 0; }
                                    break;
                                }
                                low = mid;
                            }
                            else if b_overflow || (sq > self)
                            {
                                if mid == low
                                { 
                                    highest = mid;
                                    break;
                                }
                                high = mid;
                            }
                            else    // if sq == self
                            {
                                return sum;
                            }
                        }
                    }
                }
            }

            // /// Returns the square root of the number.
            // /// [Read more](trait@SmallUInt#tymethod.isqrt) in detail.
            // fn _isqrt(self) -> Self
            // {
            //     let mut adder;
            //     let mut highest = (Self::size_in_bits() - self.leading_zeros() as usize) >> 1;
            //     let mut high;
            //     let mut low;
            //     let mut mid;
            //     let mut res = Self::zero();
            //     let mut sum;
            //     let maximum = highest - 1;
            //     loop
            //     {
            //         high = highest;
            //         low = 0;
            //         if high == 0
            //         {
            //             return res;
            //         }
            //         else    // if high > 0
            //         {
            //             loop
            //             {
            //                 mid = (high + low) >> 1;
            //                 adder = Self::generate_check_bits_(mid);
            //                 sum = res + adder;
            //                 let (sq, b_overflow) = sum.overflowing_mul(sum);
            //                 if !b_overflow && (sq < self)
            //                 {
            //                     if mid == maximum
            //                     {
            //                         res = sum;
            //                         break;
            //                     }
            //                     else if mid == low
            //                     { 
            //                         res = sum;
            //                         if mid == 0
            //                             { highest = 0; }
            //                         break;
            //                     }
            //                     low = mid;
            //                 }
            //                 else if b_overflow || (sq > self)
            //                 {
            //                     if mid == low
            //                     { 
            //                         highest = mid;
            //                         break;
            //                     }
            //                     high = mid;
            //                 }
            //                 else    // if sq == self
            //                 {
            //                     return sum;
            //                 }
            //             }
            //         }
            //     }
            // }

            /// Returns the `exp`-th root of the number.
            /// [Read more](trait@SmallUInt#tymethod.iroot) in detail.
            fn iroot(self, exp: Self) -> Self
            {
                let mut adder;
                let mut highest = (Self::size_in_bits() - self.leading_zeros() as usize) / (exp as usize);
                let mut high;
                let mut low;
                let mut mid;
                let mut res = Self::zero();
                let mut sum;
                let maximum = highest - 1;
                loop
                {
                    high = highest;
                    low = 0;
                    if high == 0
                    {
                        return res;
                    }
                    else    // if high > 0
                    {
                        loop
                        {
                            mid = (high + low) >> 1;
                            adder = Self::generate_check_bits_(mid);
                            sum = res + adder;
                            let (sq, b_overflow) = sum.overflowing_pow(exp as u32);
                            if !b_overflow && (sq < self)
                            {
                                if mid == maximum
                                {
                                    res = sum;
                                    break;
                                }
                                else if mid == low
                                { 
                                    res = sum;
                                    if mid == 0
                                        { highest = 0; }
                                    break;
                                }
                                low = mid;
                            }
                            else if b_overflow || (sq > self)
                            {
                                if mid == low
                                { 
                                    highest = mid;
                                    break;
                                }
                                high = mid;
                            }
                            else    // if sq == self
                            {
                                return sum;
                            }
                        }
                    }
                }
            }

            /// Tests a `SmallUInt`-type object to find whether or not it is a
            /// primne number.
            /// [Read more](trait@SmallUInt#tymethod.is_prime_using_miller_rabin)
            /// in detail.
            fn is_prime_using_miller_rabin(self, repetition: usize) -> bool
            {
                if self == 2 ||  self == 3
                    { return true; }

                if self.is_zero_or_one() || self.is_even()
                    { return false; }
                
                if (self as u128) < 10000_u128
                {
                    let small_self = self.into_u16();
                    let sqrt = SmallUInt::isqrt(small_self);
                    for p in [3_u16, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97]
                    {
                        if (p > sqrt) || (small_self.wrapping_rem(p) == 0)
                            { return false; }              
                    }
                    return true;
                }
                else if self <= (u32::MAX as Self)
                {
                    let a_list = [2_u8, 7, 61];
                    let len = a_list.len();
                    let common = if len < repetition { len } else { repetition };
                    for i in 0..common
                    {
                        if !self.test_miller_rabin(a_list[i] as Self)
                            { return false; }
                    }
                    return true;
                }
                else if self <= u64::MAX as Self
                {
                    let a_list = [2_u64, 325, 9375, 28178, 450775, 9780504, 1795265022];
                    let len = a_list.len();
                    let common = if len < repetition { len } else { repetition };
                    for i in 0..common
                    {
                        if !self.test_miller_rabin(a_list[i] as Self)
                            { return false; }
                    }
                    return true;
                }

                let a_list = [2_u64, 7, 61, 325, 9375, 28178, 450775, 9780504, 1795265022];
                let len = a_list.len();
                let common = if len < repetition { len } else { repetition };
                for i in 0..common
                {
                    if !self.test_miller_rabin(a_list[i] as Self)
                        { return false; }
                }

                let mut a = a_list[len-1] + 1;
                for _ in len..repetition
                {
                    if !self.test_miller_rabin(a as Self)
                        { return false; }
                    a += 1;
                }
                true
            }

            /// Tests a `SmallUInt`-type object to find whether or not `self`
            /// is a prime number.
            /// [Read more](trait@SmallUInt#tymethod.test_miller_rabin)
            /// in detail.
            fn test_miller_rabin(self, a: Self) -> bool
            {
                let self_minus_one = self.wrapping_sub(Self::one());
                let mut d = self_minus_one;
                while d.is_even()
                {
                    if a.modular_pow(d, self) == self_minus_one
                        { return true; }
                    d >>= 1;
                }
                let tmp = a.modular_pow(d, self);
                return tmp == self_minus_one || tmp.is_one();
            }

            /// Reverses the order of bits in the integer.
            /// [Read more](trait@SmallUInt#tymethod.reverse_bits) in detail.
            #[inline] fn reverse_bits(self) -> Self { self.reverse_bits() }
            // #[inline] fn reverse_bits_assign(&mut self) { *self = self.reverse_bits(); }

            /// Shifts the bits to the left by a specified amount, `n`,
            /// wrapping the truncated bits to the end of the resulting integer.
            /// [Read more](trait@SmallUInt#tymethod.rotate_left) in detail.
            #[inline] fn rotate_left(self, n: u32) -> Self  { self.rotate_left(n) }

            /// Shifts the bits to the right by a specified amount, `n`,
            /// wrapping the truncated bits to the end of the resulting integer.
            /// [Read more](trait@SmallUInt#tymethod.rotate_right) in detail.
            #[inline] fn rotate_right(self, n: u32) -> Self { self.rotate_right(n) }

            /// Returns the number of ones in the binary representation of
            /// `self`. [Read more](trait@SmallUInt#tymethod.count_ones)
            /// in detail.
            #[inline] fn count_ones(self) -> u32        { self.count_ones() }

            /// Returns the number of zeros in the binary representation of
            /// `self`. [Read more](trait@SmallUInt#tymethod.count_zeros)
            /// in detail.
            #[inline] fn count_zeros(self) -> u32       { self.count_zeros() }

            /// Returns the number of leading ones
            /// in the binary representation of `self`.
            /// [Read more](trait@SmallUInt#tymethod.leading_ones) in detail.
            #[inline] fn leading_ones(self) -> u32      { self.leading_ones() }

            /// Returns the number of leading zeros
            /// in the binary representation of `self`.
            /// [Read more](trait@SmallUInt#tymethod.leading_zeros) in detail.
            #[inline] fn leading_zeros(self) -> u32     { self.leading_zeros() }

            /// Returns the number of trailing ones
            /// in the binary representation of `self`.
            /// [Read more](trait@SmallUInt#tymethod.trailing_ones) in detail.
            #[inline] fn trailing_ones(self) -> u32     { self.trailing_ones() }

            /// Returns the number of trailing zeros
            /// in the binary representation of `self`.
            /// [Read more](trait@SmallUInt#tymethod.trailing_zeros) in detail.
            #[inline] fn trailing_zeros(self) -> u32    { self.trailing_zeros() }

            /// Converts an integer from big endian to the target’s endianness.
            /// [Read more](trait@SmallUInt#tymethod.from_be) in detail.
            #[inline] fn from_be(x: Self) -> Self   { Self::from_be(x) }

            /// Converts an integer from little endian to the target’s
            /// endianness. [Read more](trait@SmallUInt#tymethod.from_le)
            /// in detail.
            #[inline] fn from_le(x: Self) -> Self   { Self::from_le(x) }

            /// Converts `self` to big endian from the target’s endianness.
            /// [Read more](trait@SmallUInt#tymethod.to_be) in detail.
            #[inline] fn to_be(self) -> Self    { self.to_be() }

            /// Converts self to little endian from the target’s endianness.
            /// [Read more](trait@SmallUInt#tymethod.to_le) in detail.
            #[inline] fn to_le(self) -> Self    { self.to_le() }

            /// Reverses the byte order of the integer.
            /// [Read more](trait@SmallUInt#tymethod.swap_bytes) in detail.
            #[inline] fn swap_bytes(self) -> Self   { self.swap_bytes() }

            /// Returns `true` if and only if `self` == `2^k` for some `k`.
            /// [Read more](trait@SmallUInt#tymethod.is_power_of_two) in detail.
            #[inline] fn is_power_of_two(self) -> bool  { self.is_power_of_two() }

            /// Returns the smallest power of two greater than or equal to
            /// `self`. [Read more](trait@SmallUInt#tymethod.next_power_of_two)
            /// in detail.
            #[inline] fn next_power_of_two(self) -> Self    { self.next_power_of_two() }

            /// Converts `self` into `f64` and return it.
            /// [Read more](trait@SmallUInt#tymethod.into_f64) in detail.
            #[inline] fn into_f64(self) -> f64  { self as f64 }

            /// Converts `self` into `f32` and return it.
            /// [Read more](trait@SmallUInt#tymethod.into_f32) in detail.
            #[inline] fn into_f32(self) -> f32  { self as f32 }

            /// Converts `self` into `u128` and return it.
            /// [Read more](trait@SmallUInt#tymethod.into_u128) in detail.
            #[inline] fn into_u128(self) -> u128    { self as u128 }

            /// Converts `self` into `u64` and return it.
            /// [Read more](trait@SmallUInt#tymethod.into_u64) in detail.
            #[inline] fn into_u64(self) -> u64  { self as u64 }

            /// Converts `self` into `u32` and return it.
            /// [Read more](trait@SmallUInt#tymethod.into_u32) in detail.
            #[inline] fn into_u32(self) -> u32  { self as u32 }

            /// Converts `self` into `u16` and return it.
            /// [Read more](trait@SmallUInt#tymethod.into_u16) in detail.
            #[inline] fn into_u16(self) -> u16  { self as u16 }

            /// Converts `self` into `u8` and return it.
            /// [Read more](trait@SmallUInt#tymethod.into_u8) in detail.
            #[inline] fn into_u8(self) -> u8    { self as u8 }

            /// Converts `self` into `usize` and return it.
            /// [Read more](trait@SmallUInt#tymethod.into_usize) in detail.
            #[inline] fn into_usize(self) -> usize  { self as usize }

            /// Converts `self` into `bool` and return it.
            /// [Read more](trait@SmallUInt#tymethod.into_bool) in detail.
            #[inline] fn into_bool(self) -> bool    { self != 0 }

            /// Converts `self` into `ShortUnion` and return it.
            /// [Read more](trait@SmallUInt#tymethod.into_shortunion)
            /// in detail.
            #[inline] fn into_shortunion(self) -> ShortUnion    { ShortUnion::new_with(self.into_u16() ) }

            /// Converts `self` into `IntUnion` and return it.
            /// [Read more](trait@SmallUInt#tymethod.into_intunion) in detail.
            #[inline] fn into_intunion(self) -> IntUnion    { IntUnion::new_with(self.into_u32() ) }

            /// Converts `self` into `LongUnion` and return it.
            /// [Read more](trait@SmallUInt#tymethod.into_longunion) in detail.
            #[inline] fn into_longunion(self) -> LongUnion  { LongUnion::new_with(self.into_u64() ) }

            /// Converts `self` into `LongerUnion` and return it.
            /// [Read more](trait@SmallUInt#tymethod.into_longerunion)
            /// in detail.
            #[inline] fn into_longerunion(self) -> LongerUnion  { LongerUnion::new_with(self.into_u128() ) }

            /// Converts `self` into `SizeUnion` and return it.
            /// [Read more](trait@SmallUInt#tymethod.into_sizeunion) in detail.
            #[inline] fn into_sizeunion(self) -> SizeUnion  { SizeUnion::new_with(self.into_usize() ) }

            /// Returns `zero` which is of `Self`-type.
            /// [Read more](trait@SmallUInt#tymethod.zero) in detail.
            #[inline] fn zero() -> Self     { 0 }

            /// Returns `one` which is of `Self`-type.
            /// [Read more](trait@SmallUInt#tymethod.one) in detail.
            #[inline] fn one() -> Self      { 1 }

            /// Returns the maximum value of `Self`-type.
            /// [Read more](trait@SmallUInt#tymethod.max) in detail.
            #[inline] fn max() -> Self      { Self::MAX }

            /// Returns the maximum value of `Self`-type.
            /// [Read more](trait@SmallUInt#tymethod.min) in detail.
            #[inline] fn min() -> Self      { Self::MIN }

            /// Converts `u128`-typed number `n` into `Self`-type.
            /// [Read more](trait@SmallUInt#tymethod.u128_as_smalluint)
            /// in detail.
            #[inline] fn u128_as_smalluint(n: u128) -> Self  { n as Self }

            /// Converts `u64`-typed number `n` into `Self`-type.
            /// [Read more](trait@SmallUInt#tymethod.u64_as_smalluint)
            /// in detail.
            #[inline] fn u64_as_smalluint(n: u64) -> Self    { n as Self }

            /// Converts `u32`-typed number `n` into `Self`-type.
            /// [Read more](trait@SmallUInt#tymethod.u32_as_smalluint)
            /// in detail.
            #[inline] fn u32_as_smalluint(n: u32) -> Self    { n as Self }

            /// Converts `u16`-typed number `n` into `Self`-type.
            /// [Read more](trait@SmallUInt#tymethod.u16_as_smalluint)
            /// in detail.
            #[inline] fn u16_as_smalluint(n: u16) -> Self    { n as Self }

            /// Converts `u8`-typed number `n` into `Self`-type.
            /// [Read more](trait@SmallUInt#tymethod.u8_as_smalluint)
            /// in detail.
            #[inline] fn u8_as_smalluint(n: u8) -> Self     { n as Self }

            /// Converts `usize`-typed number `n` into `Self`-type.
            /// [Read more](trait@SmallUInt#tymethod.usize_as_smalluint)
            /// in detail.
            #[inline] fn usize_as_smalluint(n: usize) -> Self   { n as Self }

            /// Converts `bool`-typed number `n` into `Self`-type.
            /// [Read more](trait@SmallUInt#tymethod.bool_as_smalluint)
            /// in detail.
            #[inline] fn bool_as_smalluint(n: bool) -> Self     { n as Self }

            /// Converts `T`-typed number `n` into `Self`-type.
            /// [Read more](trait@SmallUInt#tymethod.num) in detail.
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

            /// Checks whether `SmallUInt` to be zero, and returns true
            /// if it is zero, and returns false if it is not zero.
            /// [Read more](trait@SmallUInt#tymethod.is_zero) in detail.
            #[inline] fn is_zero(self) -> bool     { self == 0 }

            /// Checks whether `SmallUInt` to be one, and returns true
            /// if it is one, and returns false if it is not one.
            /// [Read more](trait@SmallUInt#tymethod.is_one) in detail.
            #[inline] fn is_one(self) -> bool      { self ==  1 }

            /// Checks whether or not `self` is either zero or one,
            /// and returns true if it is either zero or one.
            /// Otherwise, it returns false.
            /// [Read more](trait@SmallUInt#tymethod.is_zero_or_one) in detail.
            #[inline] fn is_zero_or_one(self) -> bool   { self < 2 }

            /// Sets the MSB (Most Significant Bit) of `SmallUInt`-type
            /// number with `1`.
            /// [Read more](trait@SmallUInt#tymethod.set_msb) in detail.
            #[inline] fn set_msb(&mut self)     { *self |= !(Self::MAX >> 1); }

            /// Sets the LSB (Least Significant Bit) of `SmallUInt`-type
            /// number with `1`.
            /// [Read more](trait@SmallUInt#tymethod.set_lsb) in detail.
            #[inline] fn set_lsb(&mut self)     { *self |= 1; }

            /// Constucts a new `SmallUInt` which has the value zero and sets
            /// only the bit specified by the argument bit_pos to be 1.
            /// [Read more](trait@SmallUInt#tymethod.generate_check_bits)
            /// in detail.
            #[inline] fn generate_check_bits(bit_pos: usize) -> Option<Self>    { if bit_pos < Self::size_in_bits() { Some(Self::generate_check_bits_(bit_pos)) } else { None } }

            /// Constucts a new `SmallUInt` which has the value zero and sets
            /// only the bit specified by the argument bit_pos to be 1.
            /// [Read more](trait@SmallUInt#tymethod.generate_check_bits_)
            /// in detail.
            #[inline] fn generate_check_bits_(bit_pos: usize) -> Self    { Self::one() << bit_pos }

            /// Checks whether or not `Self` is an odd number.
            /// [Read more](trait@SmallUInt#tymethod.is_odd) in detail.
            #[inline] fn is_odd(self) -> bool       { (self & 1) != 0 }

            /// Checks whether or not `Self` is an even number.
            /// [Read more](trait@SmallUInt#tymethod.is_even) in detail.
            #[inline] fn is_even(self) -> bool      { !self.is_odd() }

            /// Checks whether or not the MSB (Most Segnificant Bit) of
            /// `self` is set to be one.
            /// [Read more](trait@SmallUInt#tymethod.is_msb_set) in detail.
            #[inline] fn is_msb_set(self) -> bool   { (self & !(Self::MAX >> 1)) != 0 }

            /// Checks whether or not the bit of `self` specified by `bit_pos`
            /// is set one.
            /// [Read more](trait@SmallUInt#tymethod.is_bit_set) in detail.
            #[inline] fn is_bit_set(self, bit_pos: usize) -> Option<bool>  { if bit_pos < Self::size_in_bits() { Some(self & Self::generate_check_bits_(bit_pos) != 0) } else { None } }

            /// Checks whether or not the bit of `self` specified by `bit_pos`
            /// is set one.
            /// [Read more](trait@SmallUInt#tymethod.is_bit_set_) in detail.
            #[inline] fn is_bit_set_(self, bit_pos: usize) -> bool  { self & Self::generate_check_bits_(bit_pos) != 0 }

            /// Sets `Self`-type number to be maximum value in which all bits
            /// are set to be `1`.
            /// [Read more](trait@SmallUInt#tymethod.set_max) in detail.
            #[inline] fn set_max(&mut self)     { *self = Self::MAX }

            /// Checks whether or not `Self`-type number to be maximum value.
            /// [Read more](trait@SmallUInt#tymethod.is_max)
            #[inline] fn is_max(self) -> bool { self == Self::MAX }

            /// Sets `Self`-type number to be `size_in_bits`-bit long maximum
            /// value in which all bits of `size_in_bits`-bit long lower part
            /// are set to be `1`.
            /// [Read more](trait@SmallUInt#tymethod.set_submax) in detail.
            fn set_submax(&mut self, size_in_bits: usize)
            {
                if size_in_bits >= self.length_in_bits()
                    { self.set_max(); }
                else if size_in_bits == 0
                    { self.set_zero(); }
                else
                {
                    self.set_max();
                    *self = *self >> (Self::size_in_bits() - size_in_bits);
                }
            }

            /// Sets `Self`-type number to be half long maximum value
            /// in which all bits of half-lower part are set to be `1`.
            /// [Read more](trait@SmallUInt#tymethod.set_halfmax) in detail.
            #[inline] fn set_halfmax(&mut self) { self.set_submax(self.length_in_bits() >> 1); }

            /// Sets `self` to be one.
            /// [Read more](trait@SmallUInt#tymethod.set_one) in detail.
            #[inline] fn set_one(&mut self)     { *self = Self::one(); }

            /// Sets `self` to be zero.
            /// [Read more](trait@SmallUInt#tymethod.set_zero) in detail.
            #[inline] fn set_zero(&mut self)    { *self = Self::zero(); }

            /// Returns the size of `Self` in bytes
            /// [Read more](trait@SmallUInt#tymethod.size_in_bytes) in detail.
            #[inline] fn size_in_bytes() -> usize   { size_of::<Self>() }

            /// Returns the size of `Self` in bits
            /// [Read more](trait@SmallUInt#tymethod.size_in_bits) in detail.
            #[inline] fn size_in_bits() -> usize    { size_of::<Self>() * 8 }

            /// Returns the size of `self` in bytes
            /// [Read more](trait@SmallUInt#tymethod.length_in_bytes) in detail.
            #[inline] fn length_in_bytes(self) -> usize    { size_of_val(&self) }

            /// Returns the size of `self` in bits
            /// [Read more](trait@SmallUInt#tymethod.length_in_bits) in detail.
            #[inline] fn length_in_bits(self) -> usize     { size_of_val(&self) * 8 }
        }
    }
}


SmallUInt_methods_for_uint_impl! { u8 }
SmallUInt_methods_for_uint_impl! { u16 }
SmallUInt_methods_for_uint_impl! { u32 }
SmallUInt_methods_for_uint_impl! { u64 }
SmallUInt_methods_for_uint_impl! { u128 }
SmallUInt_methods_for_uint_impl! { usize }
