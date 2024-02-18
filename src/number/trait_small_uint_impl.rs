// Copyright 2023, 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


use std::mem::{ size_of, size_of_val };

use super::{ SmallUInt, ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };


macro_rules! SmallUInt_methods_for_uint_impl {
    ($f:ty) => {
        impl SmallUInt for $f
        {
            /// Calculates self + rhs + carry and returns a tuple containing
            /// the sum and the output carry.
            /// [Read more in detail](trait@SmallUInt#tymethod.carrying_add)
            fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool)
            {
                let (r1, c1) = self.overflowing_add(rhs);
                let (r2, c2) = r1.overflowing_add(carry as Self);
                (r2, c1 || c2)
            }

            /// Computes self + rhs, wrapping around at the boundary of the type.
            /// [Read more in detail](trait@SmallUInt#tymethod.wrapping_add)
            #[inline] fn wrapping_add(self, rhs: Self) -> Self              { self.wrapping_add(rhs) }

            /// Calculates self + rhs and returns a tuple of the addition along
            /// with a boolean indicating whether an arithmetic overflow would
            /// occur. [Read more in detail](trait@SmallUInt#tymethod.overflowing_add)
            #[inline] fn overflowing_add(self, rhs: Self) -> (Self, bool)   { self.overflowing_add(rhs) }

            /// Computes self + rhs and returns None if overflow occurred.
            /// [Read more in detail](trait@SmallUInt#tymethod.checked_add)
            #[inline] fn checked_add(self, rhs: Self) -> Option<Self>       { self.checked_add(rhs) }

            /// Computes self + rhs and returns None if overflow occurred.
            /// [Read more in detail](trait@SmallUInt#tymethod.checked_add)
            #[inline] fn unchecked_add(self, rhs: Self) -> Self             { self.checked_add(rhs).unwrap() }

            /// Computes self + rhs, saturating at the numeric bounds
            /// instead of overflowing.
            /// [Read more in detail](trait@SmallUInt#tymethod.saturating_add)
            #[inline] fn saturating_add(self, rhs: Self) -> Self            { self.saturating_add(rhs) }

            /// Computes `self` + `rhs`, wrapping at `modulo`
            /// instead of overflowing.
            /// [Read more in detail](trait@SmallUInt#tymethod.modular_add)
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

            /// Calculates self − rhs − borrow,
            /// wrapping around at the boundary of the type.
            /// [Read more in detail](trait@SmallUInt#tymethod.borrowing_sub)
            fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool)
            {
                let (r1, b1) = self.overflowing_sub(rhs);
                let (r2, b2) = r1.overflowing_sub(borrow as Self);
                (r2, b1 || b2)
            }

            /// Computes self - rhs, wrapping around at the boundary of the type.
            /// [Read more in detail](trait@SmallUInt#tymethod.wrapping_sub)
            #[inline] fn wrapping_sub(self, rhs: Self) -> Self              { self.wrapping_sub(rhs) }

            /// Calculates self - rhs and returns a tuple of the subtraction
            /// along with a boolean indicating whether an arithmetic overflow
            /// would occur.
            /// [Read more in detail](trait@SmallUInt#tymethod.overflowing_sub)
            #[inline] fn overflowing_sub(self, rhs: Self) -> (Self, bool)   { self.overflowing_sub(rhs) }

            /// Computes self - rhs, returning None if overflow occurred.
            /// [Read more in detail](trait@SmallUInt#tymethod.checked_sub)
            #[inline] fn checked_sub(self, rhs: Self) -> Option<Self>       { self.checked_sub(rhs) }

            /// Computes self - rhs, assuming overflow cannot occur.
            /// [Read more in detail](trait@SmallUInt#tymethod.unchecked_sub)
            #[inline] fn unchecked_sub(self, rhs: Self) -> Self             { self.checked_sub(rhs).unwrap() }

            /// Computes self - rhs, saturating at the numeric bounds
            /// instead of overflowing.
            /// [Read more in detail](trait@SmallUInt#tymethod.saturating_sub)
            #[inline] fn saturating_sub(self, rhs: Self) -> Self            { self.saturating_sub(rhs) }

            /// Computes the absolute difference between self and other.
            /// [Read more in detail](trait@SmallUInt#tymethod.abs_diff)
            #[inline] fn abs_diff(self, other: Self) -> Self    { self.abs_diff(other) }

            /// Computes `self` - `rhs`, wrapping at `modulo`
            /// instead of overflowing.
            /// [Read more in detail](trait@SmallUInt#tymethod.modular_sub)
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
            /// [Read more in detail](trait@SmallUInt#tymethod.carrying_mul)
            #[inline] fn carrying_mul(self, rhs: Self, carry: Self) -> (Self, Self) { self.carrying_mul_for_internal_use(rhs, carry) }

            // fn carrying_mul_for_internal_use(self, rhs: Self, carry: Self) -> (Self, Self);
            /// It is for internal use. You are recommended to use [carrying_mul()](trait@SmallUInt#tymethod.carrying_mul) instead.
            fn carrying_mul_for_internal_use(self, rhs: Self, carry: Self) -> (Self, Self)
            {
                let overflow;
                let (mut low, mut high) = self.widening_mul_for_internal_use(rhs);
                (low, overflow) = low.overflowing_add(carry);
                if overflow
                    { high = high.wrapping_add(1); }
                (low, high)
            }

            /// Calculates the complete product `self` * `rhs` without the possibility
            /// to overflow. [Read more in detail](trait@SmallUInt#tymethod.widening_mul)
            #[inline] fn widening_mul(self, rhs: Self) -> (Self, Self) { self.widening_mul_for_internal_use(rhs) }

            // fn carrying_mul_for_internal_use(self, rhs: Self, carry: Self) -> (Self, Self);
            /// It is for internal use. You are recommended to use [carrying_mul()](trait@SmallUInt#tymethod.widening_mul) instead.
            fn widening_mul_for_internal_use(self, rhs: Self) -> (Self, Self)
            {
                if (rhs == 0) || (self == 0)
                    { return (0, 0); }

                let mut low: Self = 0;
                let mut high: Self = 0;
                let mut overflow: bool;
                let mut bit_check: Self = 1 << (Self::size_in_bits() - 1 - rhs.leading_zeros() as usize);
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
                            { high = high.wrapping_add(1); }
                    }
                    bit_check >>= 1;
                }
                (low, high)
            }

            /// Computes self * rhs, wrapping around at the boundary of the type.
            /// [Read more in detail](trait@SmallUInt#tymethod.wrapping_mul)
            #[inline] fn wrapping_mul(self, rhs: Self) -> Self              { self.wrapping_mul(rhs) }

            /// Calculates the multiplication of self and rhs and returns a tuple
            /// of the multiplication along with a boolean indicating whether an
            /// arithmetic overflow would occur.
            /// [Read more in detail](trait@SmallUInt#tymethod.overflowing_mul)
            #[inline] fn overflowing_mul(self, rhs: Self) -> (Self, bool)   { self.overflowing_mul(rhs) }

            /// Computes self * rhs, returning None if overflow occurred.
            /// [Read more in detail](trait@SmallUInt#tymethod.checked_mul)
            #[inline] fn checked_mul(self, rhs: Self) -> Option<Self>       { self.checked_mul(rhs) }

            /// Computes self * rhs, assuming overflow cannot occur.
            /// [Read more in detail](trait@SmallUInt#tymethod.unchecked_mul)
            #[inline] fn unchecked_mul(self, rhs: Self) -> Self             { self.checked_mul(rhs).unwrap() }

            /// Computes self * rhs, saturating at the numeric bounds
            /// instead of overflowing.
            /// [Read more in detail](trait@SmallUInt#tymethod.saturating_mul)
            #[inline] fn saturating_mul(self, rhs: Self) -> Self            { self.saturating_mul(rhs) }

            /// Computes `self` * `rhs`, wrapping at `modulo`
            /// instead of overflowing.
            /// [Read more in detail](trait@SmallUInt#tymethod.modular_mul)
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

            /// Computes self / rhs. Wrapped division on unsigned types is just
            /// normal division. [Read more in detail](trait@SmallUInt#tymethod.wrapping_div)
            #[inline] fn wrapping_div(self, rhs: Self) -> Self              { self.wrapping_div(rhs) }

            /// Calculates the divisor when self is divided by rhs and returns
            /// a tuple of the divisor along with a boolean indicating whether
            /// an arithmetic overflow would occur.
            /// [Read more in detail](trait@SmallUInt#tymethod.overflowing_div)
            #[inline] fn overflowing_div(self, rhs: Self) -> (Self, bool)   { self.overflowing_div(rhs) }

            /// Computes self / rhs, returning None if rhs == 0.
            /// [Read more in detail](trait@SmallUInt#tymethod.checked_div)
            #[inline] fn checked_div(self, rhs: Self) -> Option<Self>       { self.checked_div(rhs) }

            /// Computes self / rhs, if `rhs` != 0.
            /// [Read more in detail](trait@SmallUInt#tymethod.checked_div)
            #[inline] fn unchecked_div(self, rhs: Self) -> Self             { self.checked_div(rhs).unwrap() }

            /// Computes self / rhs, saturating at the numeric bounds
            /// instead of overflowing.
            /// [Read more in detail](trait@SmallUInt#tymethod.saturating_div)
            #[inline] fn saturating_div(self, rhs: Self) -> Self            { self.saturating_div(rhs) }


            /// Computes self % rhs. Wrapped remainder calculation on unsigned
            /// types is just the regular remainder calculation.
            /// [Read more in detail](trait@SmallUInt#tymethod.wrapping_rem)
            #[inline] fn wrapping_rem(self, rhs: Self) -> Self              { self.wrapping_rem(rhs) }

            /// Calculates the remainder when self is divided by rhs, and returns
            /// a tuple of the remainder after dividing along with a boolean
            /// indicating whether an arithmetic overflow would occur.
            /// [Read more in detail](trait@SmallUInt#tymethod.overflowing_rem)
            #[inline] fn overflowing_rem(self, rhs: Self) -> (Self, bool)   { self.overflowing_rem(rhs) }

            /// Computes self % rhs, returning None if rhs == 0.
            /// [Read more in detail](trait@SmallUInt#tymethod.checked_rem)
            #[inline] fn checked_rem(self, rhs: Self) -> Option<Self>       { self.checked_rem(rhs) }

            /// Computes `self` % `rhs`, if rhs != 0.
            /// [Read more in detail](trait@SmallUInt#tymethod.checked_rem)
            #[inline] fn unchecked_rem(self, rhs: Self) -> Self             { self.checked_rem(rhs).unwrap() }

            /// Computes `-self`, wrapping around at the boundary of the type.
            /// [Read more in detail](trait@SmallUInt#tymethod.wrapping_neg)
            #[inline] fn wrapping_neg(self) -> Self                         { self.wrapping_neg() }

            /// Computes self.pow(exp), wrapping around at the boundary of the type.
            /// [Read more in detail](trait@SmallUInt#tymethod.wrapping_pow)
            #[inline] fn wrapping_pow(self, exp: u32) -> Self               { self.wrapping_pow(exp) }

            /// Raises self to the power of exp, using exponentiation by squaring.
            /// [Read more in detail](trait@SmallUInt#tymethod.overflowing_pow)
            #[inline] fn overflowing_pow(self, exp: u32) -> (Self, bool)    { self.overflowing_pow(exp) }

            /// Computes self.pow(exp), returning None if overflow occurred.
            /// [Read more in detail](trait@SmallUInt#tymethod.checked_pow)
            #[inline] fn checked_pow(self, exp: u32) -> Option<Self>        { self.checked_pow(exp) }

            /// Computes self.pow(exp), unless overflow does not occcurred.
            /// Otherwise, it will panic.
            /// [Read more in detail](trait@SmallUInt#tymethod.unchecked_pow)
            #[inline] fn unchecked_pow(self, exp: u32) -> Self              { self.checked_pow(exp).unwrap() }

            /// Computes self.pow(exp), saturating at the numeric bounds
            /// instead of overflowing.
            /// [Read more in detail](trait@SmallUInt#tymethod.saturating_pow)
            #[inline] fn saturating_pow(self, exp: u32) -> Self             { self.saturating_pow(exp) }

            #[inline] fn pow(self, exp: u32) -> Self    { self.pow(exp) }

            /// Computes self.pow(exp), saturating at `modulo`
            /// instead of overflowing.
            /// [Read more in detail](trait@SmallUInt#tymethod.modular_pow)
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


            #[inline] fn ilog(self, base: Self) -> u32  { self.ilog(base) }
            #[inline] fn ilog10(self) -> u32            { self.ilog10() }
            #[inline] fn ilog2(self) -> u32             { self.ilog2() }

            fn sqrt(self) -> Self
            {
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

            fn root(self, exp: Self) -> Self
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

            fn is_prime_using_miller_rabin(self, repetition: usize) -> bool
            {
                if self.is_zero_or_one() || self.is_even()
                    { return false; }
                
                if self == 2 ||  self == 3
                    { return true; }

                if (self as u128) < (10000 as u128)
                {
                    let small_self = self.into_u32();
                    let sqrt = small_self.sqrt();
                    let mut i = 3_u32;
                    while i < 100 || i <= sqrt
                    {
                        if small_self.wrapping_rem(i) == 0
                            { return false; }
                        i += 2;
                    }
                    return true;
                }
                else if self <= (u32::MAX as Self)
                {
                    let a_list = [2_u8, 7, 61];
                    for a in a_list
                    {
                        if !self.test_miller_rabin(a as Self)
                            { return false; }
                    }
                }
                else if self <= u64::MAX as Self
                {
                    let a_list = [2_u64, 325, 9375, 28178, 450775, 9780504, 1795265022];
                    for a in a_list
                    {
                        if !self.test_miller_rabin(a as Self)
                            { return false; }
                    }
                }

                let a_list = [2_u64, 7, 61, 325, 9375, 28178, 450775, 9780504, 1795265022];
                let len = a_list.len();
                let common = if len < repetition { len } else { repetition };
                let mut i = 0;
                while i < common
                {
                    if !self.test_miller_rabin(a_list[i] as Self)
                        { return false; }
                    i += 1;
                }

                let mut a = a_list[len-1] + 1;
                for _ in i..repetition
                {
                    if !self.test_miller_rabin(a as Self)
                        { return false; }
                    a += 1;
                }
                true
            }

            /// Performs Millar Rabin method with a number less than `self`.
            /// [Read more in detail](trait@SmallUInt#tymethod.test_miller_rabin)
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
            
            #[inline] fn reverse_bits(self) -> Self     { self.reverse_bits() }
            #[inline] fn reverse_bits_assign(&mut self) { *self = self.reverse_bits(); }

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

            #[inline] fn into_f64(self) -> f64      { self as f64 }
            #[inline] fn into_f32(self) -> f32      { self as f32 }
            #[inline] fn into_u128(self) -> u128    { self as u128 }
            #[inline] fn into_u64(self) -> u64      { self as u64 }
            #[inline] fn into_u32(self) -> u32      { self as u32 }
            #[inline] fn into_u16(self) -> u16      { self as u16 }
            #[inline] fn into_u8(self) -> u8        { self as u8 }
            #[inline] fn into_usize(self) -> usize  { self as usize }
            #[inline] fn into_bool(self) -> bool    { self != 0 }
            #[inline] fn into_shortunion(self) -> ShortUnion    { ShortUnion::new_with(self.into_u16() ) }
            #[inline] fn into_intunion(self) -> IntUnion        { IntUnion::new_with(self.into_u32() ) }
            #[inline] fn into_longunion(self) -> LongUnion      { LongUnion::new_with(self.into_u64() ) }
            #[inline] fn into_longerunion(self) -> LongerUnion  { LongerUnion::new_with(self.into_u128() ) }
            #[inline] fn into_sizeunion(self) -> SizeUnion      { SizeUnion::new_with(self.into_usize() ) }
            #[inline] fn zero() -> Self             { 0 }
            #[inline] fn one() -> Self              { 1 }
            #[inline] fn max() -> Self              { Self::MAX }
            #[inline] fn min() -> Self              { Self::MIN }
            #[inline] fn u128_as_smalluint(n: u128) -> Self  { n as Self }
            #[inline] fn u64_as_smalluint(n: u64) -> Self    { n as Self }
            #[inline] fn u32_as_smalluint(n: u32) -> Self    { n as Self }
            #[inline] fn u16_as_smalluint(n: u16) -> Self    { n as Self }
            #[inline] fn u8_as_smalluint(n: u8) -> Self      { n as Self }
            #[inline] fn usize_as_smalluint(n: usize) -> Self    { n as Self }
            #[inline] fn bool_as_smalluint(n: bool) -> Self    { n as Self }

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
            #[inline] fn is_zero(&self) -> bool     { *self == 0 }

            /// Checks whether `SmallUInt` to be zero, and returns true if it is
            /// zero, and returns false if it is not zero.
            /// [Read more in detail](trait@SmallUInt#tymethod.is_one)
            #[inline] fn is_one(&self) -> bool      { *self ==  1 }

            // pub fn set_submax(&mut self, size_in_bits: usize)
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
                    *self = *self >> (Self::size_in_bits() - size_in_bits);
                }
            }

            /// Checks whether or not `BigUInt`-type number to be maximum value.
            /// zero, and returns false if it is not zero.
            /// [Read more in detail](trait@SmallUInt#tymethod.is_max)
            #[inline] fn is_max(&self) -> bool { *self == Self::MAX }

            /// Sets the MSB (Most Significant Bit) of `SmallUInt`-type
            /// number with `1`.
            /// [Read more in detail](trait@SmallUInt#tymethod.set_msb)
            #[inline] fn set_msb(&mut self)     { *self |= !(Self::MAX >> 1); }

            /// Sets the LSB (Least Significant Bit) of `SmallUInt`-type
            /// number with `1`.
            /// [Read more in detail](trait@SmallUInt#tymethod.set_lsb)
            #[inline] fn set_lsb(&mut self)     { *self |= 1; }
    /*
            /// Constucts a new `SmallUInt` which has the value zero and sets only
            /// the bit specified by the argument bit_pos to be 1.
            /// [Read more in detail](trait@SmallUInt#tymethod.generate_check_bits)
            #[inline] fn generate_check_bits(bit_pos: usize) -> Option<Self>    { if bit_pos < Self::size_in_bits { Some(generate_check_bits_(bit_pos)) } else { None }
    */
            /// Constucts a new `SmallUInt` which has the value zero and sets only
            /// the bit specified by the argument bit_pos to be 1.
            /// [Read more in detail](trait@SmallUInt#tymethod.generate_check_bits_)
            #[inline] fn generate_check_bits_(bit_pos: usize) -> Self    { Self::one() << bit_pos }

            #[inline] fn is_odd(self) -> bool       { (self & 1) != 0 }
            #[inline] fn is_even(self) -> bool      { !self.is_odd() }
            #[inline] fn is_msb_set(self) -> bool   { (self & !(Self::MAX >> 1)) != 0 }
            #[inline] fn is_bit_set(self, bit_pos: usize) -> Option<bool>  { if bit_pos < Self::size_in_bits() { Some(self & Self::generate_check_bits_(bit_pos) != 0) } else { None } }
            #[inline] fn is_bit_set_(self, bit_pos: usize) -> bool  { self & Self::generate_check_bits_(bit_pos) != 0 }

            #[inline] fn size_in_bytes() -> usize   { size_of::<Self>() }
            #[inline] fn size_in_bits() -> usize    { size_of::<Self>() * 8 }
            #[inline] fn length_in_bytes(self) -> usize    { size_of_val(&self) }
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
