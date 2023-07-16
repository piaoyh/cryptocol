// Copyright 2023 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module that contains generic types of primitive unsigned integral
//! data types used in a lot of modules of the crate Cryptocol.

#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

use std::fmt::{ Debug, Display };
use std::mem::size_of;
use std::ops::*;

/// Trait Uint is for generic type of primitive unsigned integer data types
/// for all modules of the crate Cryptocol.
///  
/// Here, the generic type of primitive unsigned integral data types includes:
/// u8, u16, u32, u64, u128 and usize. In order to use this trait, you have to
/// import (use) `Cryptocol::number::Uint`.
/// 
/// You will, however, hardly use the trait Uint unless you improve or modify
/// this crate Cryptocol or create addional libraries that works with the crate
/// Cryptocol. But, if you only use the crate Cryptocol, you can forget about
/// this trait Uint.
pub trait Uint: Copy //+ Clone + Display + Debug + ToString
{
    fn abs_diff(self, other: Self) -> Self;
    fn pow(self, exp: u32) -> Self;
    fn ilog(self, base: Self) -> u32;
    fn ilog10(self) -> u32;
    fn ilog2(self) -> u32;
    fn reverse_bits(self) -> Self;
    fn rotate_left(self, n: u32) -> Self;
    fn rotate_right(self, n: u32) -> Self;

    /// Wrapping (modular) addition. Computes self + rhs, wrapping around
    /// at the boundary of the type. In order to use this trait, you have to
    /// import (use) `Cryptocol::number::Uint`.
    /// [Read more](https://doc.rust-lang.org/core/primitive.u8.html#method.wrapping_add)
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a = func(200_u8, 55_u8);
    ///     println!("200 + 55 = {}", a);
    ///     assert_eq!(a, 255_u8);
    ///     
    ///     let b = func(u128::MAX - 55_u128, 57_u128);
    ///     println!("{} + 57 = {}", u128::MAX - 55_u128, b);
    ///     assert_eq!(b, 1_u128);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_add(rhs)
    /// }
    /// ```
    fn wrapping_add(self, rhs: Self) -> Self;

    /// Wrapping (modular) subtraction. Computes self - rhs, wrapping around
    /// at the boundary of the type. In order to use this trait, you have to
    /// import (use) `Cryptocol::number::Uint`.
    /// [Read more](https://doc.rust-lang.org/core/primitive.u8.html#method.wrapping_sub).
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a = func(200_u8, 55_u8);
    ///     println!("200 - 55 = {}", a);
    ///     assert_eq!(a, 145_u8);
    ///
    ///     let b = func(10_u128, 55_u128);
    ///     println!("10 - 55 = {}", b);
    ///     assert_eq!(b, u128::MAX - 44_u128);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_sub(rhs)
    /// }
    /// ```
    fn wrapping_sub(self, rhs: Self) -> Self;

    /// Wrapping (modular) multiplication. Computes self * rhs, wrapping around
    /// at the boundary of the type. In order to use this trait, you have to
    /// import (use) `Cryptocol::number::Uint`.
    /// [Read more](https://doc.rust-lang.org/core/primitive.u8.html#method.wrapping_mul).
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a = func(50_u8, 4_u8);
    ///     println!("50 + 4 = {}", a);
    ///     assert_eq!(a, 200_u8);
    ///     
    ///     let b = func(u128::MAX, u128::MAX);
    ///     println!("{} * {} = {}", u128::MAX, u128::MAX, b);
    ///     assert_eq!(b, 1_u128);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_mul(rhs)
    /// }
    /// ```
    fn wrapping_mul(self, rhs: Self) -> Self;

    /// Wrapping (modular) division. Computes self / rhs. Wrapped division on
    /// unsigned types is just normal division. There’s no way wrapping could
    /// ever happen. This function exists, so that all operations are accounted
    /// for in the wrapping operations. In order to use this trait, you have to
    /// import (use) `Cryptocol::number::Uint`.
    /// [Read more](https://doc.rust-lang.org/core/primitive.u8.html#method.wrapping_div).
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a = func(50_u8, 4_u8);
    ///     println!("50 / 4 = {}", a);
    ///     assert_eq!(a, 5_u8);
    ///     
    ///     let b = func(u128::MAX, u128::MAX);
    ///     println!("{} / {} = {}", u128::MAX, u128::MAX, b);
    ///     assert_eq!(b, 1_u128);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_div(rhs)
    /// }
    /// ```
    fn wrapping_div(self, rhs: Self) -> Self;

    /// Wrapping (modular) remainder. Computes self % rhs. Wrapped remainder
    /// calculation on unsigned types is just the regular remainder calculation.
    /// There’s no way wrapping could ever happen. This function exists, so that
    /// all operations are accounted for in the wrapping operations. In order to
    /// use this trait, you have to import (use) `Cryptocol::number::Uint`.
    /// [Read more](https://doc.rust-lang.org/core/primitive.u8.html#method.wrapping_rem).
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a = func(50_u8, 4_u8);
    ///     println!("50 % 4 = {}", a);
    ///     assert_eq!(a, 0_u8);
    ///     
    ///     let b = func(u128::MAX, u128::MAX);
    ///     println!("{} % {} = {}", u128::MAX, u128::MAX, b);
    ///     assert_eq!(b, 0_u128);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_rem(rhs)
    /// }
    /// ```
    fn wrapping_rem(self, rhs: Self) -> Self;
    fn wrapping_pow(self, exp: u32) -> Self;

    fn checked_add(self, rhs: Self) -> Option<Self>;
    fn checked_sub(self, rhs: Self) -> Option<Self>;
    fn checked_mul(self, rhs: Self) -> Option<Self>;
    fn checked_div(self, rhs: Self) -> Option<Self>;
    fn checked_rem(self, rhs: Self) -> Option<Self>;
    fn checked_pow(self, exp: u32) -> Option<Self>;

    fn overflowing_add(self, rhs: Self) -> (Self, bool);
    fn overflowing_sub(self, rhs: Self) -> (Self, bool);
    fn overflowing_mul(self, rhs: Self) -> (Self, bool);
    fn overflowing_div(self, rhs: Self) -> (Self, bool);
    fn overflowing_rem(self, rhs: Self) -> (Self, bool);
    fn overflowing_pow(self, exp: u32) -> (Self, bool);

    /// Calculates self + rhs + carry and returns a tuple containing the sum and
    /// the output carry. It performs “ternary addition” of two integer operands
    /// and a carry-in bit, and returns an output integer and a carry-out bit.
    /// This allows chaining together multiple additions to create a wider
    /// addition, and can be useful for big integer type addition. This can be
    /// thought of as a 8-bit “full adder”, in the electronics sense.
    /// 
    /// If the input carry is false, this method is equivalent to
    /// overflowing_add, and the output carry is equal to the overflow flag.
    /// In order to use this trait, you have to import (use) `Cryptocol::number::Uint`.
    /// [Read more](https://doc.rust-lang.org/core/primitive.u8.html#method.carrying_add)
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     a: u16 === (a_high, a_low) == (100_u8, 101u8) == 25701_u16
    ///     let a_high = 100_u8;
    ///     let a_low = 101_u8;
    ///     // b: u16 === (b_high, b_low) == (100_u8, 200u8) == 51300_u16
    ///     let b_high = 100_u8;
    ///     let b_low = 200_u8;
    ///     // c: u16 === (c_high, c_low)
    ///     let c_high: u8;
    ///     let c_low: u8;
    ///     let mut carry: bool;
    ///     // (100_u8, 101_u8) + (100_u8, 200_u8) == 25701_u16 + 25800_u16 == 51501_u16
    ///     //   25701_u16 == (100_u8, 101_u8)
    ///     // + 25800_u16 == (100_u8, 200_u8)
    ///     // -------------------------------
    ///     //   51501_u16 == (201_u8,  45_u8)
    ///     (c_high, c_low, carry) = add_long(a_high, a_low, b_high, b_low);
    ///     println!("{}-{}, {}", c_high, c_low, carry);
    ///     assert_eq!(c_high, 201);
    ///     assert_eq!(c_low, 45);
    ///     assert_eq!(carry, false);
    /// 
    ///     let d_high: u128;
    ///     let d_low: u128;
    ///     //  4201016837757989640311993609423984479246482890531986660185 == (12345678901234567890_u128, 6789012345678912345_u128)
    ///     //+                 419908440780438063913804265570801972943493 == (                1234_u128,                6789_u128)
    ///     //---------------------------------------------------------------------------------------------------------------------
    ///     //  4201016837757990060220434389862048393050748461333959603678 == (12345678901234569124_u128, 6789012345678919134_u128)
    ///     (d_high, d_low, carry) = add_long(12345678901234567890_u128, 6789012345678912345_u128, 1234_u128, 6789_u128);
    ///     println!("{}-{}, {}", d_high, d_low, carry);
    ///     assert_eq!(d_high, 12345678901234569124_u128);
    ///     assert_eq!(d_low, 6789012345678919134_u128);
    ///     assert_eq!(carry, false);
    /// }
    /// 
    /// fn add_long<T: Uint>(lhs_high: T, lhs_low: T, rhs_high: T, rhs_low: T) -> (u8, u8, bool)
    /// {
    ///     let mut carry = false;
    ///     let mut sum_high: T;
    ///     let mut sum_low: T;
    ///     (sum_low, carry) = lhs_low.carrying_add(rhs_low, carry);
    ///     (sum_high, carry) = lhs_high.carrying_add(rhs_high, carry);
    ///     (sum_high, sum_low, carry)
    /// }
    /// ```
    fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool);

    fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool);

    fn saturating_add(self, rhs: Self) -> Self;
    fn saturating_sub(self, rhs: Self) -> Self;
    fn saturating_mul(self, rhs: Self) -> Self;
    fn saturating_div(self, rhs: Self) -> Self;
    fn saturating_pow(self, exp: u32) -> Self;

    fn count_ones(self) -> u32;
    fn count_zeros(self) -> u32;
    fn leading_ones(self) -> u32;
    fn leading_zeros(self) -> u32;
    fn trailing_ones(self) -> u32;
    fn trailing_zeros(self) -> u32;

    fn from_be(x: Self) -> Self;
    fn from_le(x: Self) -> Self;
    fn to_be(self) -> Self;
    fn to_le(self) -> Self;
    fn swap_bytes(self) -> Self;

    fn is_power_of_two(self) -> bool;
    fn next_power_of_two(self) -> Self;

    fn into_f64(self) -> f64;
    fn into_f32(self) -> f32;
    fn into_u128(self) -> u128;
    fn into_u64(self) -> u64;
    fn into_u32(self) -> u32;
    fn into_u16(self) -> u16;
    fn into_u8(self) -> u8;
    fn into_usize(self) -> usize;
    fn into_bool(self) -> bool;

    fn zero() -> Self;
    fn one() -> Self;
    fn max() -> Self;
    fn min() -> Self;
    fn num(n: u128) -> Self;
    fn size_in_bytes() -> usize;
    fn size_in_bits() -> usize;
}

impl Uint for u8
{
    fn abs_diff(self, other: Self) -> Self 
    {
        self.abs_diff(other)
    }

    fn pow(self, exp: u32) -> Self
    {
        self.pow(exp)
    }

    fn ilog(self, base: Self) -> u32
    {
        self.ilog(base)
    }

    fn ilog10(self) -> u32
    {
        self.ilog10()
    }

    fn ilog2(self) -> u32
    {
        self.ilog2()
    }

    fn reverse_bits(self) -> Self
    {
        self.reverse_bits()
    }

    fn rotate_left(self, n: u32) -> Self
    {
        self.rotate_left(n)
    }

    fn rotate_right(self, n: u32) -> Self
    {
        self.rotate_right(n)
    }

    /// Wrapping (modular) addition. Computes self + rhs, wrapping around at the
    /// boundary of the type. In order to use this trait, you have to
    /// import (use) `Cryptocol::number::Uint`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a = func(u8::MAX - 56_u8, 55_u8);
    ///     println!("{} + 55 = {}", u8::MAX - 56_u8, a);
    ///     assert_eq!(a, u8::MAX - 1_u8);
    ///     
    ///     let b = func(u8::MAX - 55_u8, 56_u8);
    ///     println!("{} + 56 = {}", u8::MAX - 55_u8, b);
    ///     assert_eq!(b, 0_u8);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_add(rhs)
    /// }
    /// ```
    /// # References
    /// - If you want to know about the declaration of the method `wrapping_add()`
    /// in trait `Uint`, read [here](trait@Uint#method.wrapping_add).
    /// - If you want to know about the definition of the method `wrapping_add()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.wrapping_add).
    fn wrapping_add(self, rhs: Self) -> Self
    {
        self.wrapping_add(rhs)
    }

    /// Wrapping (modular) subtraction. Computes self - rhs, wrapping around
    /// at the boundary of the type. In order to use this trait, you have to
    /// import (use) `Cryptocol::number::Uint`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a = func(200_u8, 55_u8);
    ///     println!("200 - 55 = {}", a);
    ///     assert_eq!(a, 145_u8);
    ///
    ///     let b = func(10_u8, 55_u8);
    ///     println!("10 - 55 = {}", b);
    ///     assert_eq!(b, u8::MAX - 44_u8);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_sub(rhs)
    /// }
    /// ```
    /// # References
    /// - If you want to know about the declaration of the method `wrapping_sub()`
    /// in trait `Uint`, read [here](trait@Uint#method.wrapping_sub).
    /// - If you want to know about the definition of the method `wrapping_sub()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.wrapping_sub).
    fn wrapping_sub(self, rhs: Self) -> Self
    {
        self.wrapping_sub(rhs)
    }

    /// Wrapping (modular) multiplication. Computes self * rhs, wrapping around
    /// at the boundary of the type. In order to use this trait, you have to
    /// import (use) `Cryptocol::number::Uint`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a = func(50_u8, 4_u8);
    ///     println!("50 + 4 = {}", a);
    ///     assert_eq!(a, 200_u8);
    ///     
    ///     let b = func(u8::MAX, u8::MAX);
    ///     println!("{} * {} = {}", u8::MAX, u8::MAX, b);
    ///     assert_eq!(b, 1_u8);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_mul(rhs)
    /// }
    /// ```
    /// # References
    /// - If you want to know about the declaration of the method `wrapping_mul()`
    /// in trait `Uint`, read [here](trait@Uint#method.wrapping_mul).
    /// - If you want to know about the definition of the method `wrapping_mul()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.wrapping_mul).
    fn wrapping_mul(self, rhs: Self) -> Self
    {
        self.wrapping_mul(rhs)
    }

    /// Wrapping (modular) division. Computes self / rhs. Wrapped division on
    /// unsigned types is just normal division. There’s no way wrapping could
    /// ever happen. This function exists, so that all operations are accounted
    /// for in the wrapping operations. In order to use this trait, you have to
    /// import (use) `Cryptocol::number::Uint`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a = func(50_u8, 4_u8);
    ///     println!("50 / 4 = {}", a);
    ///     assert_eq!(a, 5_u8);
    ///     
    ///     let b = func(u8::MAX, u8::MAX);
    ///     println!("{} / {} = {}", u8::MAX, u8::MAX, b);
    ///     assert_eq!(b, 1_u8);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_div(rhs)
    /// }
    /// ```
    /// # References
    /// - If you want to know about the declaration of the method `wrapping_div()`
    /// in trait `Uint`, read [here](trait@Uint#method.wrapping_div).
    /// - If you want to know about the definition of the method `wrapping_div()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.wrapping_div).
    fn wrapping_div(self, rhs: Self) -> Self
    {
        self.wrapping_div(rhs)
    }

    /// Wrapping (modular) remainder. Computes self % rhs. Wrapped remainder
    /// calculation on unsigned types is just the regular remainder calculation.
    /// There’s no way wrapping could ever happen. This function exists, so that
    /// all operations are accounted for in the wrapping operations. In order to
    /// use this trait, you have to import (use) `Cryptocol::number::Uint`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a = func(50_u8, 4_u8);
    ///     println!("50 % 4 = {}", a);
    ///     assert_eq!(a, 0_u8);
    ///     
    ///     let b = func(u8::MAX, u8::MAX);
    ///     println!("{} % {} = {}", u8::MAX, u8::MAX, b);
    ///     assert_eq!(b, 0_u8);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_rem(rhs)
    /// }
    /// ```
    /// # References
    /// - If you want to know about the declaration of the method `wrapping_rem()`
    /// in trait `Uint`, read [here](trait@Uint#method.wrapping_rem).
    /// - If you want to know about the definition of the method `wrapping_rem()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.wrapping_rem).
    fn wrapping_rem(self, rhs: Self) -> Self
    {
        self.wrapping_rem(rhs)
    }

    fn wrapping_pow(self, exp: u32) -> Self { self.wrapping_pow(exp) }
    
    fn checked_add(self, rhs: Self) -> Option<Self> { self.checked_add(rhs) }
    fn checked_sub(self, rhs: Self) -> Option<Self> { self.checked_sub(rhs) }
    fn checked_mul(self, rhs: Self) -> Option<Self> { self.checked_mul(rhs) }
    fn checked_div(self, rhs: Self) -> Option<Self> { self.checked_div(rhs) }
    fn checked_rem(self, rhs: Self) -> Option<Self> { self.checked_rem(rhs) }
    fn checked_pow(self, exp: u32) -> Option<Self>  { self.checked_pow(exp) }

    fn overflowing_add(self, rhs: Self) -> (Self, bool) { self.overflowing_add(rhs) }
    fn overflowing_sub(self, rhs: Self) -> (Self, bool) { self.overflowing_sub(rhs) }
    fn overflowing_mul(self, rhs: Self) -> (Self, bool) { self.overflowing_mul(rhs) }
    fn overflowing_div(self, rhs: Self) -> (Self, bool) { self.overflowing_div(rhs) }
    fn overflowing_rem(self, rhs: Self) -> (Self, bool) { self.overflowing_rem(rhs) }
    fn overflowing_pow(self, exp: u32) -> (Self, bool)  { self.overflowing_pow(exp) }

    /// Calculates self + rhs + carry and returns a tuple containing the sum and
    /// the output carry. It performs “ternary addition” of two integer operands
    /// and a carry-in bit, and returns an output integer and a carry-out bit.
    /// This allows chaining together multiple additions to create a wider
    /// addition, and can be useful for big integer type addition. This can be
    /// thought of as a 8-bit “full adder”, in the electronics sense.
    /// 
    /// If the input carry is false, this method is equivalent to
    /// overflowing_add, and the output carry is equal to the overflow flag.
    /// In order to use this trait, you have to import (use) `Cryptocol::number::Uint`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::{Uint, UShort};
    /// fn main()
    /// {
    ///     a: u16 === (a_high, a_low) == (100_u8, 101u8) == 25701_u16
    ///     let a_high = 100_u8;
    ///     let a_low = 101_u8;
    ///     // b: u16 === (b_high, b_low) == (100_u8, 200u8) == 51300_u16
    ///     let b_high = 100_u8;
    ///     let b_low = 200_u8;
    ///     // c: u16 === (c_high, c_low)
    ///     let c_high: u8;
    ///     let c_low: u8;
    ///     let mut carry: bool;
    ///     // (100_u8, 101_u8) + (100_u8, 200_u8) == 25701_u16 + 25800_u16 == 51501_u16
    ///     //   25701_u16 == (100_u8, 101_u8)
    ///     // + 25800_u16 == (100_u8, 200_u8)
    ///     // -------------------------------
    ///     //   51501_u16 == (201_u8,  45_u8)
    ///     (c_high, c_low, carry) = add_long(a_high, a_low, b_high, b_low);
    ///     println!("{}-{}, {}", c_high, c_low, carry);
    ///     assert_eq!(c_high, 201);
    ///     assert_eq!(c_low, 45);
    ///     assert_eq!(carry, false);
    /// 
    ///     let ss = UShort { byte: [101, 100] };
    ///     unsafe { println!("ss.short = {}", ss.ushort ); }
    ///     println!("{}", (25700_u16 + 25800_u16));
    /// }
    /// 
    /// fn add_long<T: Uint>(lhs_high: T, lhs_low: T, rhs_high: T, rhs_low: T) -> (u8, u8, bool)
    /// {
    ///     let mut carry = false;
    ///     let mut sum_high: T;
    ///     let mut sum_low: T;
    ///     (sum_low, carry) = lhs_low.carrying_add(rhs_low, carry);
    ///     (sum_high, carry) = lhs_high.carrying_add(rhs_high, carry);
    ///     (sum_high, sum_low, carry)
    /// }
    /// ```
    /// # Reference
    /// - If you want to know about the declaration of the method `carrying_add()`
    /// in trait `Uint`, read [here](trait@Uint#method.carrying_add).
    /// - If you want to know about the definition of the method `carrying_add()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.carrying_add)
    fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool)
    {
        let (r1, c1) = self.overflowing_add(rhs);
        let (r2, c2) = r1.overflowing_add(carry as u8);
        (r2, c1 || c2)
    }

    fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool)
    {
        let (r1, b1) = self.overflowing_sub(rhs);
        let (r2, b2) = r1.overflowing_sub(borrow as u8);
        (r2, b1 || b2)
    }

    fn saturating_add(self, rhs: Self) -> Self  { self.saturating_add(rhs) }
    fn saturating_sub(self, rhs: Self) -> Self  { self.saturating_sub(rhs) }
    fn saturating_mul(self, rhs: Self) -> Self  { self.saturating_mul(rhs) }
    fn saturating_div(self, rhs: Self) -> Self  { self.saturating_div(rhs) }
    fn saturating_pow(self, exp: u32) -> Self   { self.saturating_pow(exp) }

    fn count_ones(self) -> u32      { self.count_ones() }
    fn count_zeros(self) -> u32     { self.count_zeros() }
    fn leading_ones(self) -> u32    { self.leading_ones() }
    fn leading_zeros(self) -> u32   { self.leading_zeros() }
    fn trailing_ones(self) -> u32   { self.trailing_ones() }
    fn trailing_zeros(self) -> u32  { self.trailing_zeros() }

    fn from_be(x: Self) -> Self { u8::from_be(x) }
    fn from_le(x: Self) -> Self { u8::from_le(x) }
    fn to_be(self) -> Self      { self.to_be() }
    fn to_le(self) -> Self      { self.to_le() }
    fn swap_bytes(self) -> Self { self.swap_bytes() }

    fn is_power_of_two(self) -> bool    { self.is_power_of_two() }
    fn next_power_of_two(self) -> Self  { self.next_power_of_two() }

    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_u64(self) -> u64    { self as u64 }
    fn into_u32(self) -> u32    { self as u32 }
    fn into_u16(self) -> u16    { self as u16 }
    fn into_u8(self) -> u8      { self as u8 }
    fn into_usize(self) -> usize { self as usize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn max() -> Self            { Self::MAX }
    fn min() -> Self            { Self::MIN }
    fn num(n: u128) -> Self     { n as Self }
    fn size_in_bytes() -> usize { size_of::<Self>() }
    fn size_in_bits() -> usize  { size_of::<Self>() * 8 }
}

impl Uint for u16
{
    fn abs_diff(self, other: Self) -> Self 
    {
        self.abs_diff(other)
    }

    fn pow(self, exp: u32) -> Self
    {
        self.pow(exp)
    }

    fn ilog(self, base: Self) -> u32
    {
        self.ilog(base)
    }

    fn ilog10(self) -> u32
    {
        self.ilog10()
    }

    fn ilog2(self) -> u32
    {
        self.ilog2()
    }

    fn reverse_bits(self) -> Self
    {
        self.reverse_bits()
    }

    fn rotate_left(self, n: u32) -> Self
    {
        self.rotate_left(n)
    }

    fn rotate_right(self, n: u32) -> Self
    {
        self.rotate_right(n)
    }
    
    /// Wrapping (modular) addition. Computes self + rhs, wrapping around
    /// at the boundary of the type. In order to use this trait, you have to
    /// import (use) `Cryptocol::number::Uint`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a = func(u16::MAX - 56_u16, 55_u16);
    ///     println!("{} + 55 = {}", u16::MAX - 56_u16, a);
    ///     assert_eq!(a, u16::MAX - 1_u16);
    ///     
    ///     let b = func(u16::MAX - 55_u16, 56_u16);
    ///     println!("{} + 56 = {}", u16::MAX - 55_u16, b);
    ///     assert_eq!(b, 0_u16);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_add(rhs)
    /// }
    /// ```
    /// # References
    /// - If you want to know about the declaration of the method `wrapping_add()`
    /// in trait `Uint`, read [here](trait@Uint#method.wrapping_add).
    /// - If you want to know about the definition of the method `wrapping_add()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.wrapping_add)
    fn wrapping_add(self, rhs: Self) -> Self
    {
        self.wrapping_add(rhs)
    }

    /// Wrapping (modular) subtraction. Computes self - rhs, wrapping around
    /// at the boundary of the type. In order to use this trait, you have to
    /// import (use) `Cryptocol::number::Uint`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a = func(200_u16, 55_u16);
    ///     println!("200 - 55 = {}", a);
    ///     assert_eq!(a, 145_u16);
    ///
    ///     let b = func(10_u16, 55_u16);
    ///     println!("10 - 55 = {}", b);
    ///     assert_eq!(b, u16::MAX - 44_u16);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_sub(rhs)
    /// }
    /// ```
    /// # References
    /// - If you want to know about the declaration of the method `wrapping_sub()`
    /// in trait `Uint`, read [here](trait@Uint#method.wrapping_sub).
    /// - If you want to know about the definition of the method `wrapping_sub()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.wrapping_sub).
    fn wrapping_sub(self, rhs: Self) -> Self
    {
        self.wrapping_sub(rhs)
    }

    /// Wrapping (modular) multiplication. Computes self * rhs, wrapping around
    /// at the boundary of the type. In order to use this trait, you have to
    /// import (use) `Cryptocol::number::Uint`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a = func(50_u16, 4_u16);
    ///     println!("50 + 4 = {}", a);
    ///     assert_eq!(a, 200_u16);
    ///     
    ///     let b = func(u16::MAX, u16::MAX);
    ///     println!("{} * {} = {}", u16::MAX, u16::MAX, b);
    ///     assert_eq!(b, 1_u16);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_mul(rhs)
    /// }
    /// ```
    /// # References
    /// - If you want to know about the declaration of the method `wrapping_mul()`
    /// in trait `Uint`, read [here](trait@Uint#method.wrapping_mul).
    /// - If you want to know about the definition of the method `wrapping_mul()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.wrapping_mul).
    fn wrapping_mul(self, rhs: Self) -> Self
    {
        self.wrapping_mul(rhs)
    }

    /// Wrapping (modular) division. Computes self / rhs. Wrapped division on
    /// unsigned types is just normal division. There’s no way wrapping could
    /// ever happen. This function exists, so that all operations are accounted
    /// for in the wrapping operations. In order to use this trait, you have to
    /// import (use) `Cryptocol::number::Uint`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a = func(50_u16, 4_u16);
    ///     println!("50 / 4 = {}", a);
    ///     assert_eq!(a, 5_u16);
    ///     
    ///     let b = func(u16::MAX, u16::MAX);
    ///     println!("{} / {} = {}", u16::MAX, u16::MAX, b);
    ///     assert_eq!(b, 1_u16);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_div(rhs)
    /// }
    /// ```
    /// # References
    /// - If you want to know about the declaration of the method `wrapping_div()`
    /// in trait `Uint`, read [here](trait@Uint#method.wrapping_div).
    /// - If you want to know about the definition of the method `wrapping_div()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.wrapping_div).
    fn wrapping_div(self, rhs: Self) -> Self
    {
        self.wrapping_div(rhs)
    }

    /// Wrapping (modular) remainder. Computes self % rhs. Wrapped remainder
    /// calculation on unsigned types is just the regular remainder calculation.
    /// There’s no way wrapping could ever happen. This function exists, so that
    /// all operations are accounted for in the wrapping operations. In order to
    /// use this trait, you have to import (use) `Cryptocol::number::Uint`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a = func(50_u16, 4_u16);
    ///     println!("50 % 4 = {}", a);
    ///     assert_eq!(a, 0_u16);
    ///     
    ///     let b = func(u16::MAX, u16::MAX);
    ///     println!("{} % {} = {}", u16::MAX, u16::MAX, b);
    ///     assert_eq!(b, 0_u16);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_rem(rhs)
    /// }
    /// ```
    /// # References
    /// - If you want to know about the declaration of the method `wrapping_rem()`
    /// in trait `Uint`, read [here](trait@Uint#method.wrapping_rem).
    /// - If you want to know about the definition of the method `wrapping_rem()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.wrapping_rem).
    fn wrapping_rem(self, rhs: Self) -> Self
    {
        self.wrapping_rem(rhs)
    }

    fn wrapping_pow(self, exp: u32) -> Self { self.wrapping_pow(exp) }

    fn checked_add(self, rhs: Self) -> Option<Self> { self.checked_add(rhs) }
    fn checked_sub(self, rhs: Self) -> Option<Self> { self.checked_sub(rhs) }
    fn checked_mul(self, rhs: Self) -> Option<Self> { self.checked_mul(rhs) }
    fn checked_div(self, rhs: Self) -> Option<Self> { self.checked_div(rhs) }
    fn checked_rem(self, rhs: Self) -> Option<Self> { self.checked_rem(rhs) }
    fn checked_pow(self, exp: u32) -> Option<Self>  { self.checked_pow(exp) }
    
    fn overflowing_add(self, rhs: Self) -> (Self, bool) { self.overflowing_add(rhs) }
    fn overflowing_sub(self, rhs: Self) -> (Self, bool) { self.overflowing_sub(rhs) }
    fn overflowing_mul(self, rhs: Self) -> (Self, bool) { self.overflowing_mul(rhs) }
    fn overflowing_div(self, rhs: Self) -> (Self, bool) { self.overflowing_div(rhs) }
    fn overflowing_rem(self, rhs: Self) -> (Self, bool) { self.overflowing_rem(rhs) }
    fn overflowing_pow(self, exp: u32) -> (Self, bool)  { self.overflowing_pow(exp) }

    /// Calculates self + rhs + carry and returns a tuple containing the sum and
    /// the output carry. It performs “ternary addition” of two integer operands
    /// and a carry-in bit, and returns an output integer and a carry-out bit.
    /// This allows chaining together multiple additions to create a wider
    /// addition, and can be useful for big integer type addition. This can be
    /// thought of as a 8-bit “full adder”, in the electronics sense.
    /// 
    /// If the input carry is false, this method is equivalent to
    /// overflowing_add, and the output carry is equal to the overflow flag.
    /// In order to use this trait, you have to import (use) `Cryptocol::number::Uint`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     a: u16 === (a_high, a_low) == (100_u8, 101u8) == 25701_u16
    ///     let a_high = 100_u8;
    ///     let a_low = 101_u8;
    ///     // b: u16 === (b_high, b_low) == (100_u8, 200u8) == 51300_u16
    ///     let b_high = 100_u8;
    ///     let b_low = 200_u8;
    ///     // c: u16 === (c_high, c_low)
    ///     let c_high: u8;
    ///     let c_low: u8;
    ///     let mut carry: bool;
    ///     // (100_u8, 101_u8) + (100_u8, 200_u8) == 25701_u16 + 25800_u16 == 51501_u16
    ///     //   25701_u16 == (100_u8, 101_u8)
    ///     // + 25800_u16 == (100_u8, 200_u8)
    ///     // -------------------------------
    ///     //   51501_u16 == (201_u8,  45_u8)
    ///     (c_high, c_low, carry) = add_long(a_high, a_low, b_high, b_low);
    ///     println!("{}-{}, {}", c_high, c_low, carry);
    ///     assert_eq!(c_high, 201);
    ///     assert_eq!(c_low, 45);
    ///     assert_eq!(carry, false);
    /// 
    ///     let d_high: u128;
    ///     let d_low: u128;
    ///     //  4201016837757989640311993609423984479246482890531986660185 == (12345678901234567890_u128, 6789012345678912345_u128)
    ///     //+                 419908440780438063913804265570801972943493 == (                1234_u128,                6789_u128)
    ///     //---------------------------------------------------------------------------------------------------------------------
    ///     //  4201016837757990060220434389862048393050748461333959603678 == (12345678901234569124_u128, 6789012345678919134_u128)
    ///     (d_high, d_low, carry) = add_long(12345678901234567890_u128, 6789012345678912345_u128, 1234_u128, 6789_u128);
    ///     println!("{}-{}, {}", d_high, d_low, carry);
    ///     assert_eq!(d_high, 12345678901234569124_u128);
    ///     assert_eq!(d_low, 6789012345678919134_u128);
    ///     assert_eq!(carry, false);
    /// }
    /// 
    /// fn add_long<T: Uint>(lhs_high: T, lhs_low: T, rhs_high: T, rhs_low: T) -> (u8, u8, bool)
    /// {
    ///     let mut carry = false;
    ///     let mut sum_high: T;
    ///     let mut sum_low: T;
    ///     (sum_low, carry) = lhs_low.carrying_add(rhs_low, carry);
    ///     (sum_high, carry) = lhs_high.carrying_add(rhs_high, carry);
    ///     (sum_high, sum_low, carry)
    /// }
    /// ```
    /// # Reference
    /// - If you want to know about the declaration of the method `carrying_add()`
    /// in trait `Uint`, read [here](trait@Uint#method.carrying_add).
    /// - If you want to know about the definition of the method `carrying_add()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.carrying_add)
    fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool)
    {
        let (r1, c1) = self.overflowing_add(rhs);
        let (r2, c2) = r1.overflowing_add(carry as u16);
        (r2, c1 || c2)
    }

    fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool)
    {
        let (r1, b1) = self.overflowing_sub(rhs);
        let (r2, b2) = r1.overflowing_sub(borrow as u16);
        (r2, b1 || b2)
    }

    fn saturating_add(self, rhs: Self) -> Self  { self.saturating_add(rhs) }
    fn saturating_sub(self, rhs: Self) -> Self  { self.saturating_sub(rhs) }
    fn saturating_mul(self, rhs: Self) -> Self  { self.saturating_mul(rhs) }
    fn saturating_div(self, rhs: Self) -> Self  { self.saturating_div(rhs) }
    fn saturating_pow(self, exp: u32) -> Self   { self.saturating_pow(exp) }

    fn count_ones(self) -> u32      { self.count_ones() }
    fn count_zeros(self) -> u32     { self.count_zeros() }
    fn leading_ones(self) -> u32    { self.leading_ones() }
    fn leading_zeros(self) -> u32   { self.leading_zeros() }
    fn trailing_ones(self) -> u32   { self.trailing_ones() }
    fn trailing_zeros(self) -> u32  { self.trailing_zeros() }
    
    fn from_be(x: Self) -> Self { u16::from_be(x) }
    fn from_le(x: Self) -> Self { u16::from_le(x) }
    fn to_be(self) -> Self      { self.to_be() }
    fn to_le(self) -> Self      { self.to_le() }
    fn swap_bytes(self) -> Self { self.swap_bytes() }

    fn is_power_of_two(self) -> bool    { self.is_power_of_two() }
    fn next_power_of_two(self) -> Self  { self.next_power_of_two() }

    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_u64(self) -> u64    { self as u64 }
    fn into_u32(self) -> u32    { self as u32 }
    fn into_u16(self) -> u16    { self as u16 }
    fn into_u8(self) -> u8      { self as u8 }
    fn into_usize(self) -> usize { self as usize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn max() -> Self            { Self::MAX }
    fn min() -> Self            { Self::MIN }
    fn num(n: u128) -> Self     { n as Self }
    fn size_in_bytes() -> usize { size_of::<Self>() }
    fn size_in_bits() -> usize  { size_of::<Self>() * 8 }
}

impl Uint for u32
{
    fn abs_diff(self, other: Self) -> Self 
    {
        self.abs_diff(other)
    }

    fn pow(self, exp: u32) -> Self
    {
        self.pow(exp)
    }

    fn ilog(self, base: Self) -> u32
    {
        self.ilog(base)
    }

    fn ilog10(self) -> u32
    {
        self.ilog10()
    }

    fn ilog2(self) -> u32
    {
        self.ilog2()
    }

    fn reverse_bits(self) -> Self
    {
        self.reverse_bits()
    }

    fn rotate_left(self, n: u32) -> Self
    {
        self.rotate_left(n)
    }

    fn rotate_right(self, n: u32) -> Self
    {
        self.rotate_right(n)
    }
    
    /// Wrapping (modular) addition. Computes self + rhs, wrapping around
    /// at the boundary of the type. In order to use this trait, you have to
    /// import (use) `Cryptocol::number::Uint`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a = func(u32::MAX - 56_u32, 55_u32);
    ///     println!("{} + 55 = {}", u32::MAX - 56_u32, a);
    ///     assert_eq!(a, u32::MAX - 1_u32);
    ///     
    ///     let b = func(u32::MAX - 55_u32, 56_u32);
    ///     println!("{} + 56 = {}", u32::MAX - 55_u32, b);
    ///     assert_eq!(b, 0_u32);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_add(rhs)
    /// }
    /// ```
    /// # References
    /// - If you want to know about the declaration of the method `wrapping_add()`
    /// in trait `Uint`, read [here](trait@Uint#method.wrapping_add).
    /// - If you want to know about the definition of the method `wrapping_add()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.wrapping_add)
    fn wrapping_add(self, rhs: Self) -> Self
    {
        self.wrapping_add(rhs)
    }

    /// Wrapping (modular) subtraction. Computes self - rhs, wrapping around
    /// at the boundary of the type. In order to use this trait, you have to
    /// import (use) `Cryptocol::number::Uint`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a = func(200_u32, 55_u32);
    ///     println!("200 - 55 = {}", a);
    ///     assert_eq!(a, 145_u32);
    ///
    ///     let b = func(10_u32, 55_u32);
    ///     println!("10 - 55 = {}", b);
    ///     assert_eq!(b, u32::MAX - 44_u32);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_sub(rhs)
    /// }
    /// ```
    /// # References
    /// - If you want to know about the declaration of the method `wrapping_sub()`
    /// in trait `Uint`, read [here](trait@Uint#method.wrapping_sub).
    /// - If you want to know about the definition of the method `wrapping_sub()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.wrapping_sub).
    fn wrapping_sub(self, rhs: Self) -> Self
    {
        self.wrapping_sub(rhs)
    }

    /// Wrapping (modular) multiplication. Computes self * rhs, wrapping around
    /// at the boundary of the type. In order to use this trait, you have to
    /// import (use) `Cryptocol::number::Uint`.
    ///  
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a = func(50_u32, 4_u32);
    ///     println!("50 + 4 = {}", a);
    ///     assert_eq!(a, 200_u32);
    ///     
    ///     let b = func(u32::MAX, u32::MAX);
    ///     println!("{} * {} = {}", u32::MAX, u32::MAX, b);
    ///     assert_eq!(b, 1_u32);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_mul(rhs)
    /// }
    /// ```
    /// # References
    /// - If you want to know about the declaration of the method `wrapping_mul()`
    /// in trait `Uint`, read [here](trait@Uint#method.wrapping_mul).
    /// - If you want to know about the definition of the method `wrapping_mul()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.wrapping_mul).
    fn wrapping_mul(self, rhs: Self) -> Self
    {
        self.wrapping_mul(rhs)
    }

    /// Wrapping (modular) division. Computes self / rhs. Wrapped division on
    /// unsigned types is just normal division. There’s no way wrapping could
    /// ever happen. This function exists, so that all operations are accounted
    /// for in the wrapping operations. In order to use this trait, you have to
    /// import (use) `Cryptocol::number::Uint`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a = func(50_u32, 4_u32);
    ///     println!("50 / 4 = {}", a);
    ///     assert_eq!(a, 5_u32);
    ///     
    ///     let b = func(u32::MAX, u32::MAX);
    ///     println!("{} / {} = {}", u32::MAX, u32::MAX, b);
    ///     assert_eq!(b, 1_u32);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_div(rhs)
    /// }
    /// ```
    /// # References
    /// - If you want to know about the declaration of the method `wrapping_div()`
    /// in trait `Uint`, read [here](trait@Uint#method.wrapping_div).
    /// - If you want to know about the definition of the method `wrapping_div()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.wrapping_div).
    fn wrapping_div(self, rhs: Self) -> Self
    {
        self.wrapping_div(rhs)
    }

    /// Wrapping (modular) remainder. Computes self % rhs. Wrapped remainder
    /// calculation on unsigned types is just the regular remainder calculation.
    /// There’s no way wrapping could ever happen. This function exists, so that
    /// all operations are accounted for in the wrapping operations. In order to
    /// use this trait, you have to import (use) `Cryptocol::number::Uint`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a = func(50_u32, 4_u32);
    ///     println!("50 % 4 = {}", a);
    ///     assert_eq!(a, 0_u32);
    ///     
    ///     let b = func(u32::MAX, u32::MAX);
    ///     println!("{} % {} = {}", u32::MAX, u32::MAX, b);
    ///     assert_eq!(b, 0_u32);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_rem(rhs)
    /// }
    /// ```
    /// # References
    /// - If you want to know about the declaration of the method `wrapping_rem()`
    /// in trait `Uint`, read [here](trait@Uint#method.wrapping_rem).
    /// - If you want to know about the definition of the method `wrapping_rem()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.wrapping_rem).
    fn wrapping_rem(self, rhs: Self) -> Self
    {
        self.wrapping_rem(rhs)
    }
    
    fn wrapping_pow(self, exp: u32) -> Self { self.wrapping_pow(exp) }

    fn checked_add(self, rhs: Self) -> Option<Self> { self.checked_add(rhs) }
    fn checked_sub(self, rhs: Self) -> Option<Self> { self.checked_sub(rhs) }
    fn checked_mul(self, rhs: Self) -> Option<Self> { self.checked_mul(rhs) }
    fn checked_div(self, rhs: Self) -> Option<Self> { self.checked_div(rhs) }
    fn checked_rem(self, rhs: Self) -> Option<Self> { self.checked_rem(rhs) }
    fn checked_pow(self, exp: u32) -> Option<Self>  { self.checked_pow(exp) }
    
    fn overflowing_add(self, rhs: Self) -> (Self, bool) { self.overflowing_add(rhs) }
    fn overflowing_sub(self, rhs: Self) -> (Self, bool) { self.overflowing_sub(rhs) }
    fn overflowing_mul(self, rhs: Self) -> (Self, bool) { self.overflowing_mul(rhs) }
    fn overflowing_div(self, rhs: Self) -> (Self, bool) { self.overflowing_div(rhs) }
    fn overflowing_rem(self, rhs: Self) -> (Self, bool) { self.overflowing_rem(rhs) }
    fn overflowing_pow(self, exp: u32) -> (Self, bool)  { self.overflowing_pow(exp) }

    /// Calculates self + rhs + carry and returns a tuple containing the sum and
    /// the output carry. It performs “ternary addition” of two integer operands
    /// and a carry-in bit, and returns an output integer and a carry-out bit.
    /// This allows chaining together multiple additions to create a wider
    /// addition, and can be useful for big integer type addition. This can be
    /// thought of as a 8-bit “full adder”, in the electronics sense.
    /// 
    /// If the input carry is false, this method is equivalent to
    /// overflowing_add, and the output carry is equal to the overflow flag.
    /// In order to use this trait, you have to import (use) `Cryptocol::number::Uint`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     a: u16 === (a_high, a_low) == (100_u8, 101u8) == 25701_u16
    ///     let a_high = 100_u8;
    ///     let a_low = 101_u8;
    ///     // b: u16 === (b_high, b_low) == (100_u8, 200u8) == 51300_u16
    ///     let b_high = 100_u8;
    ///     let b_low = 200_u8;
    ///     // c: u16 === (c_high, c_low)
    ///     let c_high: u8;
    ///     let c_low: u8;
    ///     let mut carry: bool;
    ///     // (100_u8, 101_u8) + (100_u8, 200_u8) == 25701_u16 + 25800_u16 == 51501_u16
    ///     //   25701_u16 == (100_u8, 101_u8)
    ///     // + 25800_u16 == (100_u8, 200_u8)
    ///     // -------------------------------
    ///     //   51501_u16 == (201_u8,  45_u8)
    ///     (c_high, c_low, carry) = add_long(a_high, a_low, b_high, b_low);
    ///     println!("{}-{}, {}", c_high, c_low, carry);
    ///     assert_eq!(c_high, 201);
    ///     assert_eq!(c_low, 45);
    ///     assert_eq!(carry, false);
    /// 
    ///     let d_high: u128;
    ///     let d_low: u128;
    ///     //  4201016837757989640311993609423984479246482890531986660185 == (12345678901234567890_u128, 6789012345678912345_u128)
    ///     //+                 419908440780438063913804265570801972943493 == (                1234_u128,                6789_u128)
    ///     //---------------------------------------------------------------------------------------------------------------------
    ///     //  4201016837757990060220434389862048393050748461333959603678 == (12345678901234569124_u128, 6789012345678919134_u128)
    ///     (d_high, d_low, carry) = add_long(12345678901234567890_u128, 6789012345678912345_u128, 1234_u128, 6789_u128);
    ///     println!("{}-{}, {}", d_high, d_low, carry);
    ///     assert_eq!(d_high, 12345678901234569124_u128);
    ///     assert_eq!(d_low, 6789012345678919134_u128);
    ///     assert_eq!(carry, false);
    /// }
    /// 
    /// fn add_long<T: Uint>(lhs_high: T, lhs_low: T, rhs_high: T, rhs_low: T) -> (u8, u8, bool)
    /// {
    ///     let mut carry = false;
    ///     let mut sum_high: T;
    ///     let mut sum_low: T;
    ///     (sum_low, carry) = lhs_low.carrying_add(rhs_low, carry);
    ///     (sum_high, carry) = lhs_high.carrying_add(rhs_high, carry);
    ///     (sum_high, sum_low, carry)
    /// }
    /// ```
    /// # Reference
    /// - If you want to know about the declaration of the method `carrying_add()`
    /// in trait `Uint`, read [here](trait@Uint#method.carrying_add).
    /// - If you want to know about the definition of the method `carrying_add()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.carrying_add)
    fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool)
    {
        let (r1, c1) = self.overflowing_add(rhs);
        let (r2, c2) = r1.overflowing_add(carry as u32);
        (r2, c1 || c2)
    }

    fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool)
    {
        let (r1, b1) = self.overflowing_sub(rhs);
        let (r2, b2) = r1.overflowing_sub(borrow as u32);
        (r2, b1 || b2)
    }

    fn saturating_add(self, rhs: Self) -> Self  { self.saturating_add(rhs) }
    fn saturating_sub(self, rhs: Self) -> Self  { self.saturating_sub(rhs) }
    fn saturating_mul(self, rhs: Self) -> Self  { self.saturating_mul(rhs) }
    fn saturating_div(self, rhs: Self) -> Self  { self.saturating_div(rhs) }
    fn saturating_pow(self, exp: u32) -> Self   { self.saturating_pow(exp) }

    fn count_ones(self) -> u32      { self.count_ones() }
    fn count_zeros(self) -> u32     { self.count_zeros() }
    fn leading_ones(self) -> u32    { self.leading_ones() }
    fn leading_zeros(self) -> u32   { self.leading_zeros() }
    fn trailing_ones(self) -> u32   { self.trailing_ones() }
    fn trailing_zeros(self) -> u32  { self.trailing_zeros() }
    
    fn from_be(x: Self) -> Self { u32::from_be(x) }
    fn from_le(x: Self) -> Self { u32::from_le(x) }
    fn to_be(self) -> Self      { self.to_be() }
    fn to_le(self) -> Self      { self.to_le() }
    fn swap_bytes(self) -> Self { self.swap_bytes() }

    fn is_power_of_two(self) -> bool    { self.is_power_of_two() }
    fn next_power_of_two(self) -> Self  { self.next_power_of_two() }

    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_u64(self) -> u64    { self as u64 }
    fn into_u32(self) -> u32    { self as u32 }
    fn into_u16(self) -> u16    { self as u16 }
    fn into_u8(self) -> u8      { self as u8 }
    fn into_usize(self) -> usize { self as usize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn max() -> Self            { Self::MAX }
    fn min() -> Self            { Self::MIN }
    fn num(n: u128) -> Self     { n as Self }
    fn size_in_bytes() -> usize { size_of::<Self>() }
    fn size_in_bits() -> usize  { size_of::<Self>() * 8 }
}

impl Uint for u64
{
    fn abs_diff(self, other: Self) -> Self 
    {
        self.abs_diff(other)
    }

    fn pow(self, exp: u32) -> Self
    {
        self.pow(exp)
    }

    fn ilog(self, base: Self) -> u32
    {
        self.ilog(base)
    }

    fn ilog10(self) -> u32
    {
        self.ilog10()
    }

    fn ilog2(self) -> u32
    {
        self.ilog2()
    }

    fn reverse_bits(self) -> Self
    {
        self.reverse_bits()
    }

    fn rotate_left(self, n: u32) -> Self
    {
        self.rotate_left(n)
    }

    fn rotate_right(self, n: u32) -> Self
    {
        self.rotate_right(n)
    }
    
    /// Wrapping (modular) addition. Computes self + rhs, wrapping around
    /// at the boundary of the type. In order to use this trait, you have to
    /// import (use) `Cryptocol::number::Uint`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a = func(u64::MAX - 56_u64, 55_u64);
    ///     println!("{} + 55 = {}", u32::MAX - 56_u64, a);
    ///     assert_eq!(a, u64::MAX - 1_u64);
    ///     
    ///     let b = func(u64::MAX - 55_u64, 56_u64);
    ///     println!("{} + 56 = {}", u64::MAX - 55_u64, b);
    ///     assert_eq!(b, 0_u64);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_add(rhs)
    /// }
    /// ```
    /// # References
    /// - If you want to know about the declaration of the method `wrapping_add()`
    /// in trait `Uint`, read [here](trait@Uint#method.wrapping_add).
    /// - If you want to know about the definition of the method `wrapping_add()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.wrapping_add)
    fn wrapping_add(self, rhs: Self) -> Self
    {
        self.wrapping_add(rhs)
    }

    /// Wrapping (modular) subtraction. Computes self - rhs, wrapping around
    /// at the boundary of the type. In order to use this trait, you have to
    /// import (use) `Cryptocol::number::Uint`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a = func(200_u64, 55_u64);
    ///     println!("200 - 55 = {}", a);
    ///     assert_eq!(a, 145_u64);
    ///
    ///     let b = func(10_u64, 55_u64);
    ///     println!("10 - 55 = {}", b);
    ///     assert_eq!(b, u64::MAX - 44_u64);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_sub(rhs)
    /// }
    /// ```
    /// # References
    /// - If you want to know about the declaration of the method `wrapping_sub()`
    /// in trait `Uint`, read [here](trait@Uint#method.wrapping_sub).
    /// - If you want to know about the definition of the method `wrapping_sub()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.wrapping_sub).
    fn wrapping_sub(self, rhs: Self) -> Self
    {
        self.wrapping_sub(rhs)
    }

    /// Wrapping (modular) multiplication. Computes self * rhs, wrapping around
    /// at the boundary of the type. In order to use this trait, you have to
    /// import (use) `Cryptocol::number::Uint`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a = func(50_u64, 4_u64);
    ///     println!("50 + 4 = {}", a);
    ///     assert_eq!(a, 200_u64);
    ///     
    ///     let b = func(u64::MAX, u64::MAX);
    ///     println!("{} * {} = {}", u64::MAX, u64::MAX, b);
    ///     assert_eq!(b, 1_u64);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_mul(rhs)
    /// }
    /// ```
    /// # References
    /// - If you want to know about the declaration of the method `wrapping_mul()`
    /// in trait `Uint`, read [here](trait@Uint#method.wrapping_mul).
    /// - If you want to know about the definition of the method `wrapping_mul()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.wrapping_mul).
    fn wrapping_mul(self, rhs: Self) -> Self
    {
        self.wrapping_mul(rhs)
    }

    /// Wrapping (modular) division. Computes self / rhs. Wrapped division on
    /// unsigned types is just normal division. There’s no way wrapping could
    /// ever happen. This function exists, so that all operations are accounted
    /// for in the wrapping operations. In order to use this trait, you have to
    /// import (use) `Cryptocol::number::Uint`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a = func(50_u64, 4_u64);
    ///     println!("50 / 4 = {}", a);
    ///     assert_eq!(a, 5_u64);
    ///     
    ///     let b = func(u64::MAX, u64::MAX);
    ///     println!("{} / {} = {}", u64::MAX, u64::MAX, b);
    ///     assert_eq!(b, 1_u64);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_div(rhs)
    /// }
    /// ```
    /// # References
    /// - If you want to know about the declaration of the method `wrapping_div()`
    /// in trait `Uint`, read [here](trait@Uint#method.wrapping_div).
    /// - If you want to know about the definition of the method `wrapping_div()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.wrapping_div).
    fn wrapping_div(self, rhs: Self) -> Self
    {
        self.wrapping_div(rhs)
    }

    /// Wrapping (modular) remainder. Computes self % rhs. Wrapped remainder
    /// calculation on unsigned types is just the regular remainder calculation.
    /// There’s no way wrapping could ever happen. This function exists, so that
    /// all operations are accounted for in the wrapping operations. In order to
    /// use this trait, you have to import (use) `Cryptocol::number::Uint`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a = func(50_u64, 4_u64);
    ///     println!("50 % 4 = {}", a);
    ///     assert_eq!(a, 0_u64);
    ///     
    ///     let b = func(u64::MAX, u64::MAX);
    ///     println!("{} % {} = {}", u64::MAX, u64::MAX, b);
    ///     assert_eq!(b, 0_u64);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_rem(rhs)
    /// }
    /// ```
    /// # References
    /// - If you want to know about the declaration of the method `wrapping_rem()`
    /// in trait `Uint`, read [here](trait@Uint#method.wrapping_rem).
    /// - If you want to know about the definition of the method `wrapping_rem()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.wrapping_rem).
    fn wrapping_rem(self, rhs: Self) -> Self
    {
        self.wrapping_rem(rhs)
    }
    
    fn wrapping_pow(self, exp: u32) -> Self { self.wrapping_pow(exp) }

    fn checked_add(self, rhs: Self) -> Option<Self> { self.checked_add(rhs) }
    fn checked_sub(self, rhs: Self) -> Option<Self> { self.checked_sub(rhs) }
    fn checked_mul(self, rhs: Self) -> Option<Self> { self.checked_mul(rhs) }
    fn checked_div(self, rhs: Self) -> Option<Self> { self.checked_div(rhs) }
    fn checked_rem(self, rhs: Self) -> Option<Self> { self.checked_rem(rhs) }
    fn checked_pow(self, exp: u32) -> Option<Self>  { self.checked_pow(exp) }
    
    fn overflowing_add(self, rhs: Self) -> (Self, bool) { self.overflowing_add(rhs) }
    fn overflowing_sub(self, rhs: Self) -> (Self, bool) { self.overflowing_sub(rhs) }
    fn overflowing_mul(self, rhs: Self) -> (Self, bool) { self.overflowing_mul(rhs) }
    fn overflowing_div(self, rhs: Self) -> (Self, bool) { self.overflowing_div(rhs) }
    fn overflowing_rem(self, rhs: Self) -> (Self, bool) { self.overflowing_rem(rhs) }
    fn overflowing_pow(self, exp: u32) -> (Self, bool)  { self.overflowing_pow(exp) }

    /// Calculates self + rhs + carry and returns a tuple containing the sum and
    /// the output carry. It performs “ternary addition” of two integer operands
    /// and a carry-in bit, and returns an output integer and a carry-out bit.
    /// This allows chaining together multiple additions to create a wider
    /// addition, and can be useful for big integer type addition. This can be
    /// thought of as a 8-bit “full adder”, in the electronics sense.
    /// 
    /// If the input carry is false, this method is equivalent to
    /// overflowing_add, and the output carry is equal to the overflow flag.
    /// In order to use this trait, you have to import (use) `Cryptocol::number::Uint`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     a: u16 === (a_high, a_low) == (100_u8, 101u8) == 25701_u16
    ///     let a_high = 100_u8;
    ///     let a_low = 101_u8;
    ///     // b: u16 === (b_high, b_low) == (100_u8, 200u8) == 51300_u16
    ///     let b_high = 100_u8;
    ///     let b_low = 200_u8;
    ///     // c: u16 === (c_high, c_low)
    ///     let c_high: u8;
    ///     let c_low: u8;
    ///     let mut carry: bool;
    ///     // (100_u8, 101_u8) + (100_u8, 200_u8) == 25701_u16 + 25800_u16 == 51501_u16
    ///     //   25701_u16 == (100_u8, 101_u8)
    ///     // + 25800_u16 == (100_u8, 200_u8)
    ///     // -------------------------------
    ///     //   51501_u16 == (201_u8,  45_u8)
    ///     (c_high, c_low, carry) = add_long(a_high, a_low, b_high, b_low);
    ///     println!("{}-{}, {}", c_high, c_low, carry);
    ///     assert_eq!(c_high, 201);
    ///     assert_eq!(c_low, 45);
    ///     assert_eq!(carry, false);
    /// 
    ///     let d_high: u128;
    ///     let d_low: u128;
    ///     //  4201016837757989640311993609423984479246482890531986660185 == (12345678901234567890_u128, 6789012345678912345_u128)
    ///     //+                 419908440780438063913804265570801972943493 == (                1234_u128,                6789_u128)
    ///     //---------------------------------------------------------------------------------------------------------------------
    ///     //  4201016837757990060220434389862048393050748461333959603678 == (12345678901234569124_u128, 6789012345678919134_u128)
    ///     (d_high, d_low, carry) = add_long(12345678901234567890_u128, 6789012345678912345_u128, 1234_u128, 6789_u128);
    ///     println!("{}-{}, {}", d_high, d_low, carry);
    ///     assert_eq!(d_high, 12345678901234569124_u128);
    ///     assert_eq!(d_low, 6789012345678919134_u128);
    ///     assert_eq!(carry, false);
    /// }
    /// 
    /// fn add_long<T: Uint>(lhs_high: T, lhs_low: T, rhs_high: T, rhs_low: T) -> (u8, u8, bool)
    /// {
    ///     let mut carry = false;
    ///     let mut sum_high: T;
    ///     let mut sum_low: T;
    ///     (sum_low, carry) = lhs_low.carrying_add(rhs_low, carry);
    ///     (sum_high, carry) = lhs_high.carrying_add(rhs_high, carry);
    ///     (sum_high, sum_low, carry)
    /// }
    /// ```
    /// # Reference
    /// - If you want to know about the declaration of the method `carrying_add()`
    /// in trait `Uint`, read [here](trait@Uint#method.carrying_add).
    /// - If you want to know about the definition of the method `carrying_add()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.carrying_add)
    fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool)
    {
        let (r1, c1) = self.overflowing_add(rhs);
        let (r2, c2) = r1.overflowing_add(carry as u64);
        (r2, c1 || c2)
    }

    fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool)
    {
        let (r1, b1) = self.overflowing_sub(rhs);
        let (r2, b2) = r1.overflowing_sub(borrow as u64);
        (r2, b1 || b2)
    }

    fn saturating_add(self, rhs: Self) -> Self  { self.saturating_add(rhs) }
    fn saturating_sub(self, rhs: Self) -> Self  { self.saturating_sub(rhs) }
    fn saturating_mul(self, rhs: Self) -> Self  { self.saturating_mul(rhs) }
    fn saturating_div(self, rhs: Self) -> Self  { self.saturating_div(rhs) }
    fn saturating_pow(self, exp: u32) -> Self   { self.saturating_pow(exp) }

    fn count_ones(self) -> u32      { self.count_ones() }
    fn count_zeros(self) -> u32     { self.count_zeros() }
    fn leading_ones(self) -> u32    { self.leading_ones() }
    fn leading_zeros(self) -> u32   { self.leading_zeros() }
    fn trailing_ones(self) -> u32   { self.trailing_ones() }
    fn trailing_zeros(self) -> u32  { self.trailing_zeros() }
    
    fn from_be(x: Self) -> Self { u64::from_be(x) }
    fn from_le(x: Self) -> Self { u64::from_le(x) }
    fn to_be(self) -> Self      { self.to_be() }
    fn to_le(self) -> Self      { self.to_le() }
    fn swap_bytes(self) -> Self { self.swap_bytes() }

    fn is_power_of_two(self) -> bool    { self.is_power_of_two() }
    fn next_power_of_two(self) -> Self  { self.next_power_of_two() }

    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_u64(self) -> u64    { self as u64 }
    fn into_u32(self) -> u32    { self as u32 }
    fn into_u16(self) -> u16    { self as u16 }
    fn into_u8(self) -> u8      { self as u8 }
    fn into_usize(self) -> usize { self as usize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn max() -> Self            { Self::MAX }
    fn min() -> Self            { Self::MIN }
    fn num(n: u128) -> Self     { n as Self }
    fn size_in_bytes() -> usize { size_of::<Self>() }
    fn size_in_bits() -> usize  { size_of::<Self>() * 8 }
}

impl Uint for u128
{
    fn abs_diff(self, other: Self) -> Self 
    {
        self.abs_diff(other)
    }

    fn pow(self, exp: u32) -> Self
    {
        self.pow(exp)
    }

    fn ilog(self, base: Self) -> u32
    {
        self.ilog(base)
    }

    fn ilog10(self) -> u32
    {
        self.ilog10()
    }

    fn ilog2(self) -> u32
    {
        self.ilog2()
    }

    fn reverse_bits(self) -> Self
    {
        self.reverse_bits()
    }

    fn rotate_left(self, n: u32) -> Self
    {
        self.rotate_left(n)
    }

    fn rotate_right(self, n: u32) -> Self
    {
        self.rotate_right(n)
    }
    
    /// Wrapping (modular) addition. Computes self + rhs, wrapping around
    /// at the boundary of the type. In order to use this trait, you have to
    /// import (use) `Cryptocol::number::Uint`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a = func(u128::MAX - 56_u128, 55_u128);
    ///     println!("{} + 55 = {}", u128::MAX - 56_u128, a);
    ///     assert_eq!(a, u128::MAX - 1_u128);
    ///     
    ///     let b = func(u128::MAX - 55_u128, 56_u128);
    ///     println!("{} + 56 = {}", u128::MAX - 55_u128, b);
    ///     assert_eq!(b, 0_u128);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_add(rhs)
    /// }
    /// ```
    /// # References
    /// - If you want to know about the declaration of the method `wrapping_add()`
    /// in trait `Uint`, read [here](trait@Uint#method.wrapping_add).
    /// - If you want to know about the definition of the method `wrapping_add()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.wrapping_add)
    fn wrapping_add(self, rhs: Self) -> Self
    {
        self.wrapping_add(rhs)
    }

    /// Wrapping (modular) subtraction. Computes self - rhs, wrapping around
    /// at the boundary of the type. In order to use this trait, you have to
    /// import (use) `Cryptocol::number::Uint`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a = func(200_u128, 55_u128);
    ///     println!("200 - 55 = {}", a);
    ///     assert_eq!(a, 145_u128);
    ///
    ///     let b = func(10_u128, 55_u128);
    ///     println!("10 - 55 = {}", b);
    ///     assert_eq!(b, u128::MAX - 44_u128);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_sub(rhs)
    /// }
    /// ```
    /// # References
    /// - If you want to know about the declaration of the method `wrapping_sub()`
    /// in trait `Uint`, read [here](trait@Uint#method.wrapping_sub).
    /// - If you want to know about the definition of the method `wrapping_sub()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.wrapping_sub).
    fn wrapping_sub(self, rhs: Self) -> Self 
    {
        self.wrapping_sub(rhs)
    }

    /// Wrapping (modular) multiplication. Computes self * rhs, wrapping around
    /// at the boundary of the type. In order to use this trait, you have to
    /// import (use) `Cryptocol::number::Uint`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a = func(50_u128, 4_u128);
    ///     println!("50 + 4 = {}", a);
    ///     assert_eq!(a, 200_u128);
    ///     
    ///     let b = func(u128::MAX, u128::MAX);
    ///     println!("{} * {} = {}", u128::MAX, u128::MAX, b);
    ///     assert_eq!(b, 1_u128);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_mul(rhs)
    /// }
    /// ```
    /// # References
    /// - If you want to know about the declaration of the method `wrapping_mul()`
    /// in trait `Uint`, read [here](trait@Uint#method.wrapping_mul).
    /// - If you want to know about the definition of the method `wrapping_mul()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.wrapping_mul).
    fn wrapping_mul(self, rhs: Self) -> Self
    {
        self.wrapping_mul(rhs)
    }

    /// Wrapping (modular) division. Computes self / rhs. Wrapped division on
    /// unsigned types is just normal division. There’s no way wrapping could
    /// ever happen. This function exists, so that all operations are accounted
    /// for in the wrapping operations. In order to use this trait, you have to
    /// import (use) `Cryptocol::number::Uint`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a = func(50_u128, 4_u128);
    ///     println!("50 / 4 = {}", a);
    ///     assert_eq!(a, 5_u128);
    ///     
    ///     let b = func(u128::MAX, u128::MAX);
    ///     println!("{} / {} = {}", u128::MAX, u128::MAX, b);
    ///     assert_eq!(b, 1_u128);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_div(rhs)
    /// }
    /// ```
    /// # References
    /// - If you want to know about the declaration of the method `wrapping_div()`
    /// in trait `Uint`, read [here](trait@Uint#method.wrapping_div).
    /// - If you want to know about the definition of the method `wrapping_div()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.wrapping_div).
    fn wrapping_div(self, rhs: Self) -> Self
    {
        self.wrapping_div(rhs)
    }
    /// Wrapping (modular) remainder. Computes self % rhs. Wrapped remainder
    /// calculation on unsigned types is just the regular remainder calculation.
    /// There’s no way wrapping could ever happen. This function exists, so that
    /// all operations are accounted for in the wrapping operations. In order to
    /// use this trait, you have to import (use) `Cryptocol::number::Uint`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a = func(50_u128, 4_u128);
    ///     println!("50 % 4 = {}", a);
    ///     assert_eq!(a, 0_u128);
    ///     
    ///     let b = func(u128::MAX, u128::MAX);
    ///     println!("{} % {} = {}", u128::MAX, u128::MAX, b);
    ///     assert_eq!(b, 0_u128);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_rem(rhs)
    /// }
    /// ```
    /// # References
    /// - If you want to know about the declaration of the method `wrapping_rem()`
    /// in trait `Uint`, read [here](trait@Uint#method.wrapping_rem).
    /// - If you want to know about the definition of the method `wrapping_rem()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.wrapping_rem).
    fn wrapping_rem(self, rhs: Self) -> Self
    {
        self.wrapping_rem(rhs)
    }
    
    fn wrapping_pow(self, exp: u32) -> Self { self.wrapping_pow(exp) }

    fn checked_add(self, rhs: Self) -> Option<Self> { self.checked_add(rhs) }
    fn checked_sub(self, rhs: Self) -> Option<Self> { self.checked_sub(rhs) }
    fn checked_mul(self, rhs: Self) -> Option<Self> { self.checked_mul(rhs) }
    fn checked_div(self, rhs: Self) -> Option<Self> { self.checked_div(rhs) }
    fn checked_rem(self, rhs: Self) -> Option<Self> { self.checked_rem(rhs) }
    fn checked_pow(self, exp: u32) -> Option<Self>  { self.checked_pow(exp) }
    
    fn overflowing_add(self, rhs: Self) -> (Self, bool) { self.overflowing_add(rhs) }
    fn overflowing_sub(self, rhs: Self) -> (Self, bool) { self.overflowing_sub(rhs) }
    fn overflowing_mul(self, rhs: Self) -> (Self, bool) { self.overflowing_mul(rhs) }
    fn overflowing_div(self, rhs: Self) -> (Self, bool) { self.overflowing_div(rhs) }
    fn overflowing_rem(self, rhs: Self) -> (Self, bool) { self.overflowing_rem(rhs) }
    fn overflowing_pow(self, exp: u32) -> (Self, bool)  { self.overflowing_pow(exp) }

    /// Calculates self + rhs + carry and returns a tuple containing the sum and
    /// the output carry. It performs “ternary addition” of two integer operands
    /// and a carry-in bit, and returns an output integer and a carry-out bit.
    /// This allows chaining together multiple additions to create a wider
    /// addition, and can be useful for big integer type addition. This can be
    /// thought of as a 8-bit “full adder”, in the electronics sense.
    /// 
    /// If the input carry is false, this method is equivalent to
    /// overflowing_add, and the output carry is equal to the overflow flag.
    /// In order to use this trait, you have to import (use) `Cryptocol::number::Uint`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     a: u16 === (a_high, a_low) == (100_u8, 101u8) == 25701_u16
    ///     let a_high = 100_u8;
    ///     let a_low = 101_u8;
    ///     // b: u16 === (b_high, b_low) == (100_u8, 200u8) == 51300_u16
    ///     let b_high = 100_u8;
    ///     let b_low = 200_u8;
    ///     // c: u16 === (c_high, c_low)
    ///     let c_high: u8;
    ///     let c_low: u8;
    ///     let mut carry: bool;
    ///     // (100_u8, 101_u8) + (100_u8, 200_u8) == 25701_u16 + 25800_u16 == 51501_u16
    ///     //   25701_u16 == (100_u8, 101_u8)
    ///     // + 25800_u16 == (100_u8, 200_u8)
    ///     // -------------------------------
    ///     //   51501_u16 == (201_u8,  45_u8)
    ///     (c_high, c_low, carry) = add_long(a_high, a_low, b_high, b_low);
    ///     println!("{}-{}, {}", c_high, c_low, carry);
    ///     assert_eq!(c_high, 201);
    ///     assert_eq!(c_low, 45);
    ///     assert_eq!(carry, false);
    /// 
    ///     let d_high: u128;
    ///     let d_low: u128;
    ///     //  4201016837757989640311993609423984479246482890531986660185 == (12345678901234567890_u128, 6789012345678912345_u128)
    ///     //+                 419908440780438063913804265570801972943493 == (                1234_u128,                6789_u128)
    ///     //---------------------------------------------------------------------------------------------------------------------
    ///     //  4201016837757990060220434389862048393050748461333959603678 == (12345678901234569124_u128, 6789012345678919134_u128)
    ///     (d_high, d_low, carry) = add_long(12345678901234567890_u128, 6789012345678912345_u128, 1234_u128, 6789_u128);
    ///     println!("{}-{}, {}", d_high, d_low, carry);
    ///     assert_eq!(d_high, 12345678901234569124_u128);
    ///     assert_eq!(d_low, 6789012345678919134_u128);
    ///     assert_eq!(carry, false);
    /// }
    /// 
    /// fn add_long<T: Uint>(lhs_high: T, lhs_low: T, rhs_high: T, rhs_low: T) -> (u8, u8, bool)
    /// {
    ///     let mut carry = false;
    ///     let mut sum_high: T;
    ///     let mut sum_low: T;
    ///     (sum_low, carry) = lhs_low.carrying_add(rhs_low, carry);
    ///     (sum_high, carry) = lhs_high.carrying_add(rhs_high, carry);
    ///     (sum_high, sum_low, carry)
    /// }
    /// ```
    /// # Reference
    /// - If you want to know about the declaration of the method `carrying_add()`
    /// in trait `Uint`, read [here](trait@Uint#method.carrying_add).
    /// - If you want to know about the definition of the method `carrying_add()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.carrying_add)
    fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool)
    {
        let (r1, c1) = self.overflowing_add(rhs);
        let (r2, c2) = r1.overflowing_add(carry as u128);
        (r2, c1 || c2)
    }

    fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool)
    {
        let (r1, b1) = self.overflowing_sub(rhs);
        let (r2, b2) = r1.overflowing_sub(borrow as u128);
        (r2, b1 || b2)
    }

    fn saturating_add(self, rhs: Self) -> Self  { self.saturating_add(rhs) }
    fn saturating_sub(self, rhs: Self) -> Self  { self.saturating_sub(rhs) }
    fn saturating_mul(self, rhs: Self) -> Self  { self.saturating_mul(rhs) }
    fn saturating_div(self, rhs: Self) -> Self  { self.saturating_div(rhs) }
    fn saturating_pow(self, exp: u32) -> Self   { self.saturating_pow(exp) }

    fn count_ones(self) -> u32      { self.count_ones() }
    fn count_zeros(self) -> u32     { self.count_zeros() }
    fn leading_ones(self) -> u32    { self.leading_ones() }
    fn leading_zeros(self) -> u32   { self.leading_zeros() }
    fn trailing_ones(self) -> u32   { self.trailing_ones() }
    fn trailing_zeros(self) -> u32  { self.trailing_zeros() }
    
    fn from_be(x: Self) -> Self { u128::from_be(x) }
    fn from_le(x: Self) -> Self { u128::from_le(x) }
    fn to_be(self) -> Self      { self.to_be() }
    fn to_le(self) -> Self      { self.to_le() }
    fn swap_bytes(self) -> Self { self.swap_bytes() }

    fn is_power_of_two(self) -> bool    { self.is_power_of_two() }
    fn next_power_of_two(self) -> Self  { self.next_power_of_two() }

    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_u64(self) -> u64    { self as u64 }
    fn into_u32(self) -> u32    { self as u32 }
    fn into_u16(self) -> u16    { self as u16 }
    fn into_u8(self) -> u8      { self as u8 }
    fn into_usize(self) -> usize { self as usize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn max() -> Self            { Self::MAX }
    fn min() -> Self            { Self::MIN }
    fn num(n: u128) -> Self     { n as Self }
    fn size_in_bytes() -> usize { size_of::<Self>() }
    fn size_in_bits() -> usize  { size_of::<Self>() * 8 }
}

impl Uint for usize
{
    fn abs_diff(self, other: Self) -> Self 
    {
        self.abs_diff(other)
    }

    fn pow(self, exp: u32) -> Self
    {
        self.pow(exp)
    }

    fn ilog(self, base: Self) -> u32
    {
        self.ilog(base)
    }

    fn ilog10(self) -> u32
    {
        self.ilog10()
    }

    fn ilog2(self) -> u32
    {
        self.ilog2()
    }

    fn reverse_bits(self) -> Self
    {
        self.reverse_bits()
    }

    fn rotate_left(self, n: u32) -> Self
    {
        self.rotate_left(n)
    }

    fn rotate_right(self, n: u32) -> Self
    {
        self.rotate_right(n)
    }
    
    /// Wrapping (modular) addition. Computes self + rhs, wrapping around
    /// at the boundary of the type. In order to use this trait, you have to
    /// import (use) `Cryptocol::number::Uint`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a = func(usize::MAX - 56_usize, 55_usize);
    ///     println!("{} + 55 = {}", usize::MAX - 56_usize, a);
    ///     assert_eq!(a, usize::MAX - 1_usize);
    ///     
    ///     let b = func(usize::MAX - 55_usize, 56_usize);
    ///     println!("{} + 56 = {}", usize::MAX - 55_usize, b);
    ///     assert_eq!(b, 0_usize);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_add(rhs)
    /// }
    /// ```
    /// # References
    /// - If you want to know about the declaration of the method `wrapping_add()`
    /// in trait `Uint`, read [here](trait@Uint#method.wrapping_add).
    /// - If you want to know about the definition of the method `wrapping_add()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.wrapping_add)
    fn wrapping_add(self, rhs: Self) -> Self
    {
        self.wrapping_add(rhs)
    }

    /// Wrapping (modular) subtraction. Computes self - rhs, wrapping around
    /// at the boundary of the type. In order to use this trait, you have to
    /// import (use) `Cryptocol::number::Uint`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a = func(200_usize, 55_usize);
    ///     println!("200 - 55 = {}", a);
    ///     assert_eq!(a, 145_usize);
    ///
    ///     let b = func(10_usize, 55_usize);
    ///     println!("10 - 55 = {}", b);
    ///     assert_eq!(b, usize::MAX - 44_usize);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_sub(rhs)
    /// }
    /// ```
    /// # References
    /// - If you want to know about the declaration of the method `wrapping_sub()`
    /// in trait `Uint`, read [here](trait@Uint#method.wrapping_sub).
    /// - If you want to know about the definition of the method `wrapping_sub()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.wrapping_sub).
    fn wrapping_sub(self, rhs: Self) -> Self
    {
        self.wrapping_sub(rhs)
    }

    /// Wrapping (modular) multiplication. Computes self * rhs, wrapping around
    /// at the boundary of the type. In order to use this trait, you have to
    /// import (use) `Cryptocol::number::Uint`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a = func(50_usize, 4_usize);
    ///     println!("50 + 4 = {}", a);
    ///     assert_eq!(a, 200_usize);
    ///     
    ///     let b = func(usize::MAX, usize::MAX);
    ///     println!("{} * {} = {}", usize::MAX, usize::MAX, b);
    ///     assert_eq!(b, 1_usize);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_mul(rhs)
    /// }
    /// ```
    /// # References
    /// - If you want to know about the declaration of the method `wrapping_mul()`
    /// in trait `Uint`, read [here](trait@Uint#method.wrapping_mul).
    /// - If you want to know about the definition of the method `wrapping_mul()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.wrapping_mul).
    fn wrapping_mul(self, rhs: Self) -> Self
    {
        self.wrapping_mul(rhs)
    }

    /// Wrapping (modular) division. Computes self / rhs. Wrapped division on
    /// unsigned types is just normal division. There’s no way wrapping could
    /// ever happen. This function exists, so that all operations are accounted
    /// for in the wrapping operations. In order to use this trait, you have to
    /// import (use) `Cryptocol::number::Uint`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a = func(50_usize, 4_usize);
    ///     println!("50 / 4 = {}", a);
    ///     assert_eq!(a, 5_usize);
    ///     
    ///     let b = func(usize::MAX, usize::MAX);
    ///     println!("{} / {} = {}", usize::MAX, usize::MAX, b);
    ///     assert_eq!(b, 1_usize);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_div(rhs)
    /// }
    /// ```
    /// # References
    /// - If you want to know about the declaration of the method `wrapping_div()`
    /// in trait `Uint`, read [here](trait@Uint#method.wrapping_div).
    /// - If you want to know about the definition of the method `wrapping_div()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.wrapping_div).
    fn wrapping_div(self, rhs: Self) -> Self
    {
        self.wrapping_div(rhs)
    }

    /// Wrapping (modular) remainder. Computes self % rhs. Wrapped remainder
    /// calculation on unsigned types is just the regular remainder calculation.
    /// There’s no way wrapping could ever happen. This function exists, so that
    /// all operations are accounted for in the wrapping operations. In order to
    /// use this trait, you have to import (use) `Cryptocol::number::Uint`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a = func(50_usize, 4_usize);
    ///     println!("50 % 4 = {}", a);
    ///     assert_eq!(a, 0_usize);
    ///     
    ///     let b = func(usize::MAX, usize::MAX);
    ///     println!("{} % {} = {}", usize::MAX, usize::MAX, b);
    ///     assert_eq!(b, 0_usize);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_rem(rhs)
    /// }
    /// ```
    /// # References
    /// - If you want to know about the declaration of the method `wrapping_rem()`
    /// in trait `Uint`, read [here](trait@Uint#method.wrapping_rem).
    /// - If you want to know about the definition of the method `wrapping_rem()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.wrapping_rem).
    fn wrapping_rem(self, rhs: Self) -> Self
    {
        self.wrapping_rem(rhs)
    }
    
    fn wrapping_pow(self, exp: u32) -> Self { self.wrapping_pow(exp) }

    fn checked_add(self, rhs: Self) -> Option<Self> { self.checked_add(rhs) }
    fn checked_sub(self, rhs: Self) -> Option<Self> { self.checked_sub(rhs) }
    fn checked_mul(self, rhs: Self) -> Option<Self> { self.checked_mul(rhs) }
    fn checked_div(self, rhs: Self) -> Option<Self> { self.checked_div(rhs) }
    fn checked_rem(self, rhs: Self) -> Option<Self> { self.checked_rem(rhs) }
    fn checked_pow(self, exp: u32) -> Option<Self>  { self.checked_pow(exp) }
    
    fn overflowing_add(self, rhs: Self) -> (Self, bool) { self.overflowing_add(rhs) }
    fn overflowing_sub(self, rhs: Self) -> (Self, bool) { self.overflowing_sub(rhs) }
    fn overflowing_mul(self, rhs: Self) -> (Self, bool) { self.overflowing_mul(rhs) }
    fn overflowing_div(self, rhs: Self) -> (Self, bool) { self.overflowing_div(rhs) }
    fn overflowing_rem(self, rhs: Self) -> (Self, bool) { self.overflowing_rem(rhs) }
    fn overflowing_pow(self, exp: u32) -> (Self, bool)  { self.overflowing_pow(exp) }

    /// Calculates self + rhs + carry and returns a tuple containing the sum and
    /// the output carry. It performs “ternary addition” of two integer operands
    /// and a carry-in bit, and returns an output integer and a carry-out bit.
    /// This allows chaining together multiple additions to create a wider
    /// addition, and can be useful for big integer type addition. This can be
    /// thought of as a 8-bit “full adder”, in the electronics sense.
    /// 
    /// If the input carry is false, this method is equivalent to
    /// overflowing_add, and the output carry is equal to the overflow flag.
    /// In order to use this trait, you have to import (use) `Cryptocol::number::Uint`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     a: u16 === (a_high, a_low) == (100_u8, 101u8) == 25701_u16
    ///     let a_high = 100_u8;
    ///     let a_low = 101_u8;
    ///     // b: u16 === (b_high, b_low) == (100_u8, 200u8) == 51300_u16
    ///     let b_high = 100_u8;
    ///     let b_low = 200_u8;
    ///     // c: u16 === (c_high, c_low)
    ///     let c_high: u8;
    ///     let c_low: u8;
    ///     let mut carry: bool;
    ///     // (100_u8, 101_u8) + (100_u8, 200_u8) == 25701_u16 + 25800_u16 == 51501_u16
    ///     //   25701_u16 == (100_u8, 101_u8)
    ///     // + 25800_u16 == (100_u8, 200_u8)
    ///     // -------------------------------
    ///     //   51501_u16 == (201_u8,  45_u8)
    ///     (c_high, c_low, carry) = add_long(a_high, a_low, b_high, b_low);
    ///     println!("{}-{}, {}", c_high, c_low, carry);
    ///     assert_eq!(c_high, 201);
    ///     assert_eq!(c_low, 45);
    ///     assert_eq!(carry, false);
    /// 
    ///     let d_high: u128;
    ///     let d_low: u128;
    ///     //  4201016837757989640311993609423984479246482890531986660185 == (12345678901234567890_u128, 6789012345678912345_u128)
    ///     //+                 419908440780438063913804265570801972943493 == (                1234_u128,                6789_u128)
    ///     //---------------------------------------------------------------------------------------------------------------------
    ///     //  4201016837757990060220434389862048393050748461333959603678 == (12345678901234569124_u128, 6789012345678919134_u128)
    ///     (d_high, d_low, carry) = add_long(12345678901234567890_u128, 6789012345678912345_u128, 1234_u128, 6789_u128);
    ///     println!("{}-{}, {}", d_high, d_low, carry);
    ///     assert_eq!(d_high, 12345678901234569124_u128);
    ///     assert_eq!(d_low, 6789012345678919134_u128);
    ///     assert_eq!(carry, false);
    /// }
    /// 
    /// fn add_long<T: Uint>(lhs_high: T, lhs_low: T, rhs_high: T, rhs_low: T) -> (u8, u8, bool)
    /// {
    ///     let mut carry = false;
    ///     let mut sum_high: T;
    ///     let mut sum_low: T;
    ///     (sum_low, carry) = lhs_low.carrying_add(rhs_low, carry);
    ///     (sum_high, carry) = lhs_high.carrying_add(rhs_high, carry);
    ///     (sum_high, sum_low, carry)
    /// }
    /// ```
    /// # Reference
    /// - If you want to know about the declaration of the method `carrying_add()`
    /// in trait `Uint`, read [here](trait@Uint#method.carrying_add).
    /// - If you want to know about the definition of the method `carrying_add()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.carrying_add)
    fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool)
    {
        let (r1, c1) = self.overflowing_add(rhs);
        let (r2, c2) = r1.overflowing_add(carry as usize);
        (r2, c1 || c2)
    }

    fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool)
    {
        let (r1, b1) = self.overflowing_sub(rhs);
        let (r2, b2) = r1.overflowing_sub(borrow as usize);
        (r2, b1 || b2)
    }

    fn saturating_add(self, rhs: Self) -> Self  { self.saturating_add(rhs) }
    fn saturating_sub(self, rhs: Self) -> Self  { self.saturating_sub(rhs) }
    fn saturating_mul(self, rhs: Self) -> Self  { self.saturating_mul(rhs) }
    fn saturating_div(self, rhs: Self) -> Self  { self.saturating_div(rhs) }
    fn saturating_pow(self, exp: u32) -> Self   { self.saturating_pow(exp) }

    fn count_ones(self) -> u32      { self.count_ones() }
    fn count_zeros(self) -> u32     { self.count_zeros() }
    fn leading_ones(self) -> u32    { self.leading_ones() }
    fn leading_zeros(self) -> u32   { self.leading_zeros() }
    fn trailing_ones(self) -> u32   { self.trailing_ones() }
    fn trailing_zeros(self) -> u32  { self.trailing_zeros() }
    
    fn from_be(x: Self) -> Self { usize::from_be(x) }
    fn from_le(x: Self) -> Self { usize::from_le(x) }
    fn to_be(self) -> Self      { self.to_be() }
    fn to_le(self) -> Self      { self.to_le() }
    fn swap_bytes(self) -> Self { self.swap_bytes() }

    fn is_power_of_two(self) -> bool    { self.is_power_of_two() }
    fn next_power_of_two(self) -> Self  { self.next_power_of_two() }

    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_u64(self) -> u64    { self as u64 }
    fn into_u32(self) -> u32    { self as u32 }
    fn into_u16(self) -> u16    { self as u16 }
    fn into_u8(self) -> u8      { self as u8 }
    fn into_usize(self) -> usize { self as usize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn max() -> Self            { Self::MAX }
    fn min() -> Self            { Self::MIN }
    fn num(n: u128) -> Self     { n as Self }
    fn size_in_bytes() -> usize { size_of::<Self>() }
    fn size_in_bits() -> usize  { size_of::<Self>() * 8 }
}


#[derive(Copy, Clone)]
pub union UShort
{
    pub ushort: u16,
    pub byte: [u8; 2],
}

#[derive(Copy, Clone)]
pub union UInt
{
    pub uint: u32,
    pub ushort: [u16; 2],
    pub byte: [u8; 4],
}

#[derive(Copy, Clone)]
pub union ULong
{
    pub ulong: u64,
    pub uint: [u32; 2],
    pub ushort: [u16; 4],
    pub byte: [u8; 8],
}

#[derive(Copy, Clone)]
pub union ULonger
{
    pub ulonger: u128,
    pub ulong: [u64; 2],
    pub uint: [u32; 4],
    pub ushort: [u16; 8],
    pub byte: [u8; 16],
}

#[cfg(target_pointer_width = "128")]
#[derive(Copy, Clone)]
pub union USize
{
    pub size: usize,
    pub ulonger: u128,
    pub ulong: [u64; 2],
    pub uint: [u32; 4],
    pub ushort: [u16; 8],
    pub byte: [u8; 16],
}

#[cfg(target_pointer_width = "64")]
#[derive(Copy, Clone)]
pub union USize
{
    pub size: usize,
    pub ulong: u64,
    pub uint: [u32; 2],
    pub ushort: [u16; 4],
    pub byte: [u8; 8],
}

#[cfg(target_pointer_width = "32")]
#[derive(Copy, Clone)]
pub union USize
{
    pub size: usize,
    pub uint: u32,
    pub ushort: [u16; 2],
    pub byte: [u8; 4],
}

#[cfg(target_pointer_width = "16")]
#[derive(Copy, Clone)]
pub union USize
{
    pub size: usize,
    pub ushort: u16,
    pub byte: [u8; 2],
}

#[cfg(target_pointer_width = "8")]
#[derive(Copy, Clone)]
pub union USize
{
    pub size: usize,
    pub byte: u8,
}

impl UShort
{
    pub fn new() -> Self    { Self { ushort: 0 } }
    pub fn new_with(ushort: u16) -> Self   { Self { ushort } }
}

impl UInt
{
    pub fn new() -> Self    { Self { uint: 0 } }
    pub fn new_with(uint: u32) -> Self   { Self { uint } }
}

impl ULong
{
    pub fn new() -> Self    { Self { ulong: 0 } }
    pub fn new_with(ulong: u64) -> Self   { Self { ulong } }
}

impl ULonger
{
    pub fn new() -> Self    { Self { ulonger: 0 } }
    pub fn new_with(ulonger: u128) -> Self   { Self { ulonger } }
}

impl USize
{
    pub fn new() -> Self    { Self { size: 0 } }
    pub fn new_with(size: usize) -> Self   { Self { size } }
}

/// union array for transforming from one type into anther type
pub union Share<D, S>
where D: Uint + Copy + Clone + Display + Debug + ToString
        + Add<Output=D> + AddAssign + Sub<Output=D> + SubAssign
        + Mul<Output=D> + MulAssign + Div<Output=D> + DivAssign
        + Shl<Output=D> + ShlAssign + Shr<Output=D> + ShrAssign
        + BitAnd<Output=D> + BitAndAssign + BitOr<Output=D> + BitOrAssign
        + BitXor<Output=D> + BitXorAssign + Not<Output=D>
        + PartialEq + PartialOrd,
      S: Uint + Copy + Clone + Display + Debug + ToString
        + Add<Output=S> + AddAssign + Sub<Output=S> + SubAssign
        + Mul<Output=S> + MulAssign + Div<Output=S> + DivAssign
        + Shl<Output=S> + ShlAssign + Shr<Output=S> + ShrAssign
        + BitAnd<Output=S> + BitAndAssign + BitOr<Output=S> + BitOrAssign
        + BitXor<Output=S> + BitXorAssign + Not<Output=S>
        + PartialEq + PartialOrd
{
    pub des: D,
    pub src: S,
}

impl<D, S> Share<D, S>
where D: Uint + Copy + Clone + Display + Debug + ToString
        + Add<Output=D> + AddAssign + Sub<Output=D> + SubAssign
        + Mul<Output=D> + MulAssign + Div<Output=D> + DivAssign
        + Shl<Output=D> + ShlAssign + Shr<Output=D> + ShrAssign
        + BitAnd<Output=D> + BitAndAssign + BitOr<Output=D> + BitOrAssign
        + BitXor<Output=D> + BitXorAssign + Not<Output=D>
        + PartialEq + PartialOrd,
      S: Uint + Copy + Clone + Display + Debug + ToString
        + Add<Output=S> + AddAssign + Sub<Output=S> + SubAssign
        + Mul<Output=S> + MulAssign + Div<Output=S> + DivAssign
        + Shl<Output=S> + ShlAssign + Shr<Output=S> + ShrAssign
        + BitAnd<Output=S> + BitAndAssign + BitOr<Output=S> + BitOrAssign
        + BitXor<Output=S> + BitXorAssign + Not<Output=S>
        + PartialEq + PartialOrd
{
    pub fn new() -> Self
    {
        if size_of::<D>() >= size_of::<S>()
            { Self { des: D::zero() } }
        else
            { Self { src: S::zero() } }
    }

    pub fn from_src(src: S) -> Self
    {
        let mut me = Share::<D, S>::new();
        unsafe { me.src = src; }
        me
    }

    #[cfg(target_endian = "little")]
    pub fn into_des(&mut self, pos: usize) -> Option<D>
    {
        let bit_pos = pos * size_of::<D>() * 8;
        unsafe { self.src >>= S::num(bit_pos as u128); }
        if (bit_pos > 0) && self.is_src_zero()
            { None }
        else
            { unsafe { Some(self.des) } }
    }

    #[cfg(target_endian = "big")]
    pub fn into_des1(&mut self, pos: usize) -> Option<D>
    {
        let des_size = size_of::<D>();
        let src_size = size_of::<S>();
        let bit_pos = pos * size_of::<D>() * 8;
        unsafe { self.src <<= S::num(bit_pos as u128); }
        if des_size > src_size
            { unsafe { self.des >>= D::num((des_size - src_size).into_u128() * 8); } }
        else if src_size > des_size
            { unsafe { self.src <<= S::num((src_size - des_size).into_u128() * 8); } }
        Some( unsafe { self.des } )
    }

    pub fn is_src_zero(&self) -> bool    { unsafe { self.src == S::zero() } }
}

/// union array for transforming from one type into anther type
pub union Common<D, const N: usize, S, const M: usize>
where D: Uint + Add<Output=D> + AddAssign + Sub<Output=D> + SubAssign
        + Mul<Output=D> + MulAssign + Div<Output=D> + DivAssign
        + Shl<Output=D> + ShlAssign + Shr<Output=D> + ShrAssign
        + BitAnd<Output=D> + BitAndAssign + BitOr<Output=D> + BitOrAssign
        + BitXor<Output=D> + BitXorAssign + Not<Output=D>
        + PartialEq + PartialOrd
        + Display + ToString,
      S: Uint + Add<Output=S> + AddAssign + Sub<Output=S> + SubAssign
        + Mul<Output=S> + MulAssign + Div<Output=S> + DivAssign
        + Shl<Output=S> + ShlAssign + Shr<Output=S> + ShrAssign
        + BitAnd<Output=S> + BitAndAssign + BitOr<Output=S> + BitOrAssign
        + BitXor<Output=S> + BitXorAssign + Not<Output=S>
        + PartialEq + PartialOrd
        + Display + ToString
{
    pub des: [D; N],
    pub src: [S; M],
}

impl<D, const N: usize, S, const M: usize> Common<D, N, S, M>
where D: Uint + Add<Output=D> + AddAssign + Sub<Output=D> + SubAssign
        + Mul<Output=D> + MulAssign + Div<Output=D> + DivAssign
        + Shl<Output=D> + ShlAssign + Shr<Output=D> + ShrAssign
        + BitAnd<Output=D> + BitAndAssign + BitOr<Output=D> + BitOrAssign
        + BitXor<Output=D> + BitXorAssign + Not<Output=D>
        + PartialEq + PartialOrd
        + Display + ToString,
      S: Uint + Add<Output=S> + AddAssign + Sub<Output=S> + SubAssign
        + Mul<Output=S> + MulAssign + Div<Output=S> + DivAssign
        + Shl<Output=S> + ShlAssign + Shr<Output=S> + ShrAssign
        + BitAnd<Output=S> + BitAndAssign + BitOr<Output=S> + BitOrAssign
        + BitXor<Output=S> + BitXorAssign + Not<Output=S>
        + PartialEq + PartialOrd
        + Display + ToString
{
    pub fn new() -> Self
    {
        if Self::size_of_des() >= Self::size_of_src()
            { Self { des: [D::zero(); N] } }
        else
            { Self { src: [S::zero(); M] } }
    }

    pub fn from_src(src: &[S; M]) -> Self
    {
        let mut me = Common::<D, N, S, M>::new();
        unsafe { me.src.copy_from_slice(src); }
        me
    }

    #[cfg(target_endian = "little")]
    #[inline]
    pub fn into_des(&mut self, des: &mut [D; N])
    {
        unsafe { des.copy_from_slice(&self.des); }
    }

    #[cfg(target_endian = "big")]
    pub fn into_des(&mut self, des: &mut [D; N])
    {
        let des_size = Self::size_of_des();
        let src_size = Self::size_of_src();
        if src_size > des_size
            { self.shl_assign_src((src_size - des_size) * 8); }
        else
            { self.shr_assign_des((des_size - src_size) * 8); }
        des.copy_from_slice(&self.des);
    }

    #[cfg(target_endian = "big")]
    fn shl_assign_src(&mut self, rhs: usize)
    {
        let TSIZE_BIT = size_of::<D>() * 8;
        let chunk_num = rhs as usize / TSIZE_BIT as usize;
        let piece_num = rhs as usize % TSIZE_BIT as usize;
        let zero = S::zero();
        if chunk_num > 0
        {
            self.src.copy_within(chunk_num..N, 0);
            for idx in N-chunk_num..N
                { self.src[idx] = zero; }
        }
        if piece_num == 0
            { return; }

        let mut num: S;
        let mut carry = zero;
        for idx in 0..N-chunk_num
        {
            num = (self.src[idx] << S::num(piece_num.into_u128())) | carry;
            carry = self.src[idx] >> S::num((TSIZE_BIT - piece_num).into_u128());
            self.src[idx] = num;
        }
    }

    #[cfg(target_endian = "big")]
    fn shr_assign_des(&mut self, rhs: usize)
    {
        let TSIZE_BIT = size_of::<T>() * 8;
        let chunk_num = rhs as usize / TSIZE_BIT as usize;
        let piece_num = rhs as usize % TSIZE_BIT as usize;
        let zero = D::zero();
        if chunk_num > 0
        {
            self.des.copy_within(0..N-chunk_num, chunk_num);
            for idx in 0..chunk_num
                { self.des[idx] = zero; }
        }
        if piece_num == 0
            { return; }

        let mut num: D;
        let mut carry = D::zero();
        let mut idx = 0;
        loop
        {
            num = (self.des[idx] >> D::num(piece_num.into_u128())) | carry;
            carry = self.des[idx] << D::num((TSIZE_BIT - piece_num).into_u128());
            self.des[idx] = num;
            if idx == N - 1 - chunk_num
                { break; }
            idx += 1;
        }
    }

    pub fn size_of_des() -> usize   { size_of::<D>() * N }
    pub fn size_of_src() -> usize   { size_of::<S>() * M }
}
/*
impl Shr<usize> for USize
{
    type Output = Self;
    fn shr(self, rhs: usize) -> Self   { let mut s = self; s.size >>= rhs; s}
}

impl ShrAssign<usize> for USize
{
    fn shr_assign(&mut self, rhs: usize)    { self.size >>= rhs; }
}
*/

/*
pub trait Bool: Copy + Eq
{
    fn into_f64(self) -> f64;
    fn into_f32(self) -> f32;
    fn into_u128(self) -> u128;
    fn into_i128(self) -> i128;
    fn into_usize(self) -> usize;
    fn zero() -> Self;
    fn one() -> Self;
    fn max() -> Self;
    fn min() -> Self;
}

impl Bool for u8
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn max() -> Self            { 1 }
    fn min() -> Self            { 0 }
}

impl Bool for u16
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn max() -> Self            { 1 }
    fn min() -> Self            { 0 }
}

impl Bool for u32
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn max() -> Self            { 1 }
    fn min() -> Self            { 0 }
}

impl Bool for u64
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn max() -> Self            { 1 }
    fn min() -> Self            { 0 }
}

impl Bool for u128
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn max() -> Self            { 1 }
    fn min() -> Self            { 0 }
}

impl Bool for i8
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn max() -> Self            { 1 }
    fn min() -> Self            { 0 }
}

impl Bool for i16
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn max() -> Self            { 1 }
    fn min() -> Self            { 0 }
}

impl Bool for i32
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn max() -> Self            { 1 }
    fn min() -> Self            { 0 }
}

impl Bool for i64
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn max() -> Self            { 1 }
    fn min() -> Self            { 0 }
}

impl Bool for i128
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn max() -> Self            { 1 }
    fn min() -> Self            { 0 }
}
*/
