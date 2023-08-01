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
use std::mem::{ size_of, size_of_val };
use std::ops::*;


/// Trait Uint is for generic type of primitive unsigned integer data types
/// for all modules of the crate Cryptocol.
/// In order to use this trait, you have to import (use) `Cryptocol::number::Uint`.
///  
/// Here, the generic type of primitive unsigned integral data types includes:
/// `u8`, `u16`, `u32`, `u64`, `u128` and `usize`. In order to use this trait,
/// you have to import (use) `Cryptocol::number::Uint`.
/// 
/// You will, however, hardly use the trait Uint unless you improve or modify
/// this crate Cryptocol or create addional libraries that works with the crate
/// Cryptocol. But, if you only use the crate Cryptocol, you can forget about
/// this trait Uint.
pub trait Uint: Copy + Sized //+ Clone + Display + Debug + ToString
{
    /***** ADDITION *****/

    /// Calculates self + rhs + carry,
    /// wrapping around at the boundary of the type.
    /// 
    /// # Functionalities
    /// This allows chaining together multiple additions to create a wider
    /// addition, and can be useful for big integer type addition. This can be
    /// thought of as a 8-bit “full adder”, in the electronics sense.
    /// 
    /// If the input carry is false, this method is equivalent to
    /// overflowing_add, and the output carry is equal to the overflow flag.
    /// 
    /// # Outputs
    /// It returns a tuple containing the sum and the output carry. It performs
    /// “ternary addition” of two integer operands and a carry-in bit, and
    /// returns an output integer and a carry-out bit.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     // a_u16: u16 === (a_high_u8, a_low_u8) == (100_u8, 101_u8) == 25701_u16
    ///     let a_high_u8 = 100_u8;
    ///     let a_low_u8 = 101_u8;
    ///     // b_u16: u16 === (b_high_u8, b_low_u8) == (100_u8, 200_u8) == 25800_u16
    ///     let b_high_u8 = 100_u8;
    ///     let b_low_u8 = 200_u8;
    ///     // c_u16: u16 === (c_high_u8, c_low_u8)
    ///     let c_high_u8: u8;
    ///     let c_low_u8: u8;
    ///     let mut carry: bool;
    ///     // (100_u8, 101_u8) + (100_u8, 200_u8) == 25701_u16 + 25800_u16 == 51501_u16
    ///     //   25701_u16 == (100_u8, 101_u8)
    ///     // + 25800_u16 == (100_u8, 200_u8)
    ///     // -------------------------------
    ///     //   51501_u16 == (201_u8,  45_u8)
    ///     (c_high_u8, c_low_u8, carry) = add_long(a_high_u8, a_low_u8, b_high_u8, b_low_u8);
    ///     println!("{}-{}, {}", c_high_u8, c_low_u8, carry);
    ///     assert_eq!(c_high_u8, 201);
    ///     assert_eq!(c_low_u8, 45);
    ///     assert_eq!(carry, false);
    /// 
    ///     let d_high_u8: u8;
    ///     let d_low_u8: u8;
    ///     //  (201_u8,  45_u8) + (100_u8, 200_u8) == 25701_u16 + 25800_u16 == 51501_u16
    ///     //   25701_u16 == (100_u8, 101_u8)
    ///     // + 25800_u16 == (100_u8, 200_u8)
    ///     // -------------------------------
    ///     //   11765_u16 == ( 45_u8, 245_u8)
    ///     (d_high_u8, d_low_u8, carry) = UInt_carrying_add___func(c_high_u8, c_low_u8, b_high_u8, b_low_u8);
    ///     println!("{}-{}, {}", d_high_u8, d_low_u8, carry);
    ///     assert_eq!(d_high_u8, 45_u8);
    ///     assert_eq!(d_low_u8, 245_u8);
    ///     assert_eq!(carry, true);
    /// 
    ///     let a_high_u128: u128;
    ///     let a_low_u128: u128;
    ///     //   4201016837757989640311993609423984479246482890531986660185 == (12345678901234567890_u128, 6789012345678912345_u128)
    ///     // +                 419908440780438063913804265570801972943493 == (                1234_u128,                6789_u128)
    ///     // ---------------------------------------------------------------------------------------------------------------------
    ///     //   4201016837757990060220434389862048393050748461333959603678 == (12345678901234569124_u128, 6789012345678919134_u128)
    ///     (a_high_u128, a_low_u128, carry) = add_long(12345678901234567890_u128, 6789012345678912345_u128, 1234_u128, 6789_u128);
    ///     println!("{}-{}, {}", a_high_u128, a_low_u128, carry);
    ///     assert_eq!(a_high_u128, 12345678901234569124_u128);
    ///     assert_eq!(a_low_u128, 6789012345678919134_u128);
    ///     assert_eq!(carry, false);
    /// 
    ///     let b_high_u128: u128;
    ///     let b_low_u128: u128;
    ///     //   308778904632843187796189293356501087608549893209439890708590319850715068122315 == (226854911280625642308916404954512140970_u128, 56789012345678912345678901234567890123_u128)
    ///     // +  57896044618658097711785492504343953926307055644800578124155540853313808954190 == (170141183460469231731687303715884105727_u128, 12345678901234567890123456789012345678_u128)
    ///     // -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
    ///     //    19298681539552699237261830834781317975046994857318776714373108680289488156697 == ( 56713727820156410577229101238628035241_u128, 69134691246913480235802358023580235801_u128)
    ///     let (b_high_u128, b_low_u128, carry) = UInt_carrying_add___func(226854911280625642308916404954512140970_u128, 56789012345678912345678901234567890123_u128, 170141183460469231731687303715884105727_u128, 12345678901234567890123456789012345678_u128);
    ///     println!("{}-{}, {}", b_high_u128, b_low_u128, carry);
    ///     assert_eq!(b_high_u128, 56713727820156410577229101238628035241_u128);
    ///     assert_eq!(b_low_u128, 69134691246913480235802358023580235801_u128);
    ///     assert_eq!(carry, true);
    /// }
    /// 
    /// fn add_long<T: Uint>(lhs_high: T, lhs_low: T, rhs_high: T, rhs_low: T) -> (T, T, bool)
    /// {
    ///     let mut carry = false;
    ///     let mut sum_high: T;
    ///     let mut sum_low: T;
    ///     (sum_low, carry) = lhs_low.carrying_add(rhs_low, carry);
    ///     (sum_high, carry) = lhs_high.carrying_add(rhs_high, carry);
    ///     (sum_high, sum_low, carry)
    /// }
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// Even though it does not call the method carrying_add() of implementation
    /// of the primitive unsigned integer types such as `u8`, `u16`, `u32`,
    /// `u64`, `u128` and `usize` directly, all the description of this method
    /// is mainly the same as that of the method carrying_add() of
    /// implementation of the primitive unsigned integer types for nightly
    /// version except example codes. Confer to the descryptions that are linked
    /// to in the section _Reference_. This plagiarism is not made maliciously
    /// but is made for the reason of effectiveness and efficiency so that users
    /// may understand better and easily how to use this method with simiilarity
    /// to the method carrying_add() of implementation of the primitive unsigned
    /// integer types.
    /// 
    /// # Possiible Changes in Future
    /// This method does not call the method carrying_add() of the primitive
    /// unsigned integer types directly. Instead, it is implemented to perform
    /// the same thing as that of carrying_add() of the primitive unsigned
    /// integer types because the methods carrying_add() of the primitive
    /// unsigned integer types are only for nightly version. So, when the method
    /// carrying_add() of the primitive unsigned integer types will become a
    /// part of non-nightly normal version, the implementation of this method
    /// will be changed to call the method carrying_add() of the primitive
    /// unsigned integer types directly.
    /// 
    /// # References
    /// - If you want to know about the definition of the method `carrying_add()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.carrying_add).
    /// - If you want to know about the definition of the method `carrying_add()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.carrying_add).
    /// - If you want to know about the definition of the method `carrying_add()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.carrying_add).
    /// - If you want to know about the definition of the method `carrying_add()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.carrying_add).
    /// - If you want to know about the definition of the method `carrying_add()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.carrying_add).
    /// - If you want to know about the definition of the method `carrying_add()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.carrying_add).
    fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool);

    /// Computes self + rhs, wrapping around at the boundary of the type.
    /// 
    /// # Functionalities
    /// It adds two numbers with wrapping (modular) addition.
    /// 
    /// # Output
    /// It returns the self + rhs in the type of `Self`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a_u8 = func(u8::MAX - 55_u8, 55_u8);
    ///     println!("{} + 55 = {}", u8::MAX - 55_u8, a_u8);
    ///     assert_eq!(a_u8, u8::MAX);
    /// 
    ///     let b_u8 = func(a_u8, 1_u8);
    ///     println!("{} + 1 = {}", a_u8, b_u8);
    ///     assert_eq!(b_u8, 0_u8);
    /// 
    ///     let a_u16 = func(u16::MAX - 55_u16, 55_u16);
    ///     println!("{} + 55 = {}", u16::MAX - 55_u16, a_u16);
    ///     assert_eq!(a_u16, u16::MAX);
    /// 
    ///     let b_u16 = func(a_u16, 1_u16);
    ///     println!("{} + 1 = {}", a_u16, b_u16);
    ///     assert_eq!(b_u16, 0_u16);
    /// 
    ///     let a_u32 = func(u32::MAX - 55_u32, 55_u32);
    ///     println!("{} + 55 = {}", u32::MAX - 55_u32, a_u32);
    ///     assert_eq!(a_u32, u32::MAX);
    /// 
    ///     let b_u32 = func(a_u32, 1_u32);
    ///     println!("{} + 1 = {}", a_u32, b_u32);
    ///     assert_eq!(b_u32, 0_u32);
    /// 
    ///     let a_u64 = func(u64::MAX - 55_u64, 55_u64);
    ///     println!("{} + 55 = {}", u64::MAX - 55_u64, a_u64);
    ///     assert_eq!(a_u64, u64::MAX);
    /// 
    ///     let b_u64 = func(a_u64, 1_u64);
    ///     println!("{} + 1 = {}", a_u64, b_u64);
    ///     assert_eq!(b_u64, 0_u64);
    /// 
    ///     let a_u128 = func(u128::MAX - 55_u128, 55_u128);
    ///     println!("{} + 55 = {}", u128::MAX - 55_u128, a_u128);
    ///     assert_eq!(a_u128, u128::MAX);
    /// 
    ///     let b_u128 = func(a_u128, 1_u128);
    ///     println!("{} + 1 = {}",a_u128, b_u128);
    ///     assert_eq!(b_u128, 0_u128);
    /// 
    ///     let a_usize = func(usize::MAX - 55_usize, 55_usize);
    ///     println!("{} + 55 = {}", usize::MAX - 55_usize, a_usize);
    ///     assert_eq!(a_usize, usize::MAX);
    /// 
    ///     let b_usize = func(a_usize, 1_usize);
    ///     println!("{} + 1 = {}", a_usize, b_usize);
    ///     assert_eq!(b_usize, 0_usize);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_add(rhs)
    /// }
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// It calls the method wrapping_add() of implementation of the primitive
    /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
    /// `usize` directly. So, all the description of this method is mainly the
    /// same as that of the method wrapping_add() of implementation of the
    /// primitive unsigned integer types except example codes. Confer to the
    /// descryptions that are linked to in the section _Reference_. This
    /// plagiarism is not made maliciously but is made for the reason of
    /// effectiveness and efficiency so that users may understand better and
    /// easily how to use this method with simiilarity to the method
    /// wrapping_add() of implementation of the primitive unsigned integer types.
    /// 
    /// # References
    /// - If you want to know about the definition of the method `wrapping_add()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.wrapping_add).
    /// - If you want to know about the definition of the method `wrapping_add()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.wrapping_add).
    /// - If you want to know about the definition of the method `wrapping_add()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.wrapping_add).
    /// - If you want to know about the definition of the method `wrapping_add()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.wrapping_add).
    /// - If you want to know about the definition of the method `wrapping_add()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.wrapping_add).
    /// - If you want to know about the definition of the method `wrapping_add()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.wrapping_add).
    fn wrapping_add(self, rhs: Self) -> Self;

    /// Calculates self + rhs, wrapping around at the boundary of the type.
    /// 
    /// # functionalities 
    /// It adds two numbers with wrapping (modular) addition. It is the same as
    /// the method carrying_add() with the imput carry which is false.
    /// 
    /// # Output
    /// It returns a tuple of the addition along with a boolean indicating
    /// whether an arithmetic overflow would occur. If an overflow would
    /// have occurred then the wrapped value is returned.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a_u8 = func(u8::MAX - 55_u8, 55_u8);
    ///     println!("{} + 55 = {}\nOverflow = {}", u8::MAX - 55_u8, a_u8.0, a_u8.1);
    ///     assert_eq!(a_u8.0, u8::MAX);
    ///     assert_eq!(a_u8.1, false);
    /// 
    ///     let b_u8 = func(a_u8.0, 1_u8);
    ///     println!("{} + 1 = {}\nOverflow = {}", a_u8.0, b_u8.0, b_u8.1);
    ///     assert_eq!(b_u8.0, 0_u8);
    ///     assert_eq!(b_u8.1, true);
    /// 
    ///     let a_u16 = func(u16::MAX - 55_u16, 55_u16);
    ///     println!("{} + 55 = {}\nOverflow = {}", u16::MAX - 55_u16, a_u16.0, a_u16.1);
    ///     assert_eq!(a_u16.0, u16::MAX);
    ///     assert_eq!(a_u16.1, false);
    /// 
    ///     let b_u16 = func(a_u16.0, 1_u16);
    ///     println!("{} + 1 = {}\nOverflow = {}", a_u16.0, b_u16.0, b_u16.1);
    ///     assert_eq!(b_u16.0, 0_u16);
    ///     assert_eq!(b_u16.1, true);
    /// 
    ///     let a_u32 = func(u32::MAX - 55_u32, 55_u32);
    ///     println!("{} + 55 = {}\nOverflow = {}", u32::MAX - 55_u32, a_u32.0, a_u32.1);
    ///     assert_eq!(a_u32.0, u32::MAX);
    ///     assert_eq!(a_u32.1, false);
    /// 
    ///     let b_u32 = func(a_u32.0, 1_u32);
    ///     println!("{} + 1 = {}\nOverflow = {}", a_u32.0, b_u32.0, b_u32.1);
    ///     assert_eq!(b_u32.0, 0_u32);
    ///     assert_eq!(b_u32.1, true);
    /// 
    ///     let a_u64 = func(u64::MAX - 55_u64, 55_u64);
    ///     println!("{} + 55 = {}\nOverflow = {}", u64::MAX - 55_u64, a_u64.0, a_u64.1);
    ///     assert_eq!(a_u64.0, u64::MAX);
    ///     assert_eq!(a_u64.1, false);
    /// 
    ///     let b_u64 = func(a_u64.0, 1_u64);
    ///     println!("{} + 1 = {}\nOverflow = {}", a_u64.0, b_u64.0, b_u64.1);
    ///     assert_eq!(b_u64.0, 0_u64);
    ///     assert_eq!(b_u64.1, true);
    /// 
    ///     let a_u128 = func(u128::MAX - 55_u128, 55_u128);
    ///     println!("{} + 55 = {}\nOverflow = {}", u128::MAX - 55_u128, a_u128.0, a_u128.1);
    ///     assert_eq!(a_u128.0, u128::MAX);
    ///     assert_eq!(a_u128.1, false);
    /// 
    ///     let b_u128 = func(a_u128.0, 1_u128);
    ///     println!("{} + 1 = {}\nOverflow = {}", a_u128.0, b_u128.0, b_u128.1);
    ///     assert_eq!(b_u128.0, 0_u128);
    ///     assert_eq!(b_u128.1, true
    /// 
    ///     let a_usize = func(usize::MAX - 55_usize, 55_usize);
    ///     println!("{} + 55 = {}\nOverflow = {}", usize::MAX - 55_usize, a_usize.0, a_usize.1);
    ///     assert_eq!(a_usize.0, usize::MAX);
    ///     assert_eq!(a_usize.1, false);
    /// 
    ///     let b_usize = func(a_usize.0, 1_usize);
    ///     println!("{} + 1 = {}\nOverflow = {}", a_usize.0, b_usize.0, b_usize.1);
    ///     assert_eq!(b_usize.0, 0_usize);
    ///     assert_eq!(b_usize.1, true);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> (T, bool)
    /// {
    ///     lhs.overflowing_add(rhs)
    /// }
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// It calls the method overflowing_add() of implementation of the primitive
    /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
    /// `usize` directly. So, all the description of this method is mainly the
    /// same as that of the method overflowing_add() of implementation of the
    /// primitive unsigned integer types except example codes. Confer to the
    /// descryptions that are linked to in the section _Reference_. This
    /// plagiarism is not made maliciously but is made for the reason of
    /// effectiveness and efficiency so that users may understand better and
    /// easily how to use this method with simiilarity to the method
    /// overflowing_add() of implementation of the primitive unsigned integer
    /// types.
    /// 
    /// # References
    /// - If you want to know about the definition of the method `overflowing_add()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.overflowing_add).
    /// - If you want to know about the definition of the method `overflowing_add()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.overflowing_add).
    /// - If you want to know about the definition of the method `overflowing_add()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.overflowing_add).
    /// - If you want to know about the definition of the method `overflowing_add()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.overflowing_add).
    /// - If you want to know about the definition of the method `overflowing_add()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.overflowing_add).
    /// - If you want to know about the definition of the method `overflowing_add()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.overflowing_add).
    fn overflowing_add(self, rhs: Self) -> (Self, bool);

    /// Computes self + rhs
    /// 
    /// # Panics
    /// If overflow occurs, this method will panic at this version.
    /// 
    /// # Output
    /// It returns self + rhs in the type Self if overflow did not occur.
    /// Otherwise, its behavior is not defined.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a_u8 = func(u8::MAX - 55_u8, 55_u8);
    ///     match a_u8
    ///     {
    ///         Some(a) => {
    ///                 println!("{} + 55 = {}", u8::MAX - 55_u8, a);
    ///                 assert_eq!(a, u8::MAX);
    ///             },
    ///         None => {
    ///                 println!("Overflow happened.");
    ///                 assert_eq!(a_u8, None);
    ///             },
    ///     }
    /// 
    ///     let b_u8 = func(a_u8.unwrap(), 1_u8);
    ///     match b_u8
    ///     {
    ///         Some(b) => {
    ///                 println!("{} + 1 = {}", a_u8.unwrap(), b);
    ///                 assert_eq!(b, 0_u8);
    ///             },
    ///         None => {
    ///                 println!("Overflow happened.");
    ///                 assert_eq!(b_u8, None);
    ///             },
    ///     }
    /// 
    ///     let a_u16 = func(u16::MAX - 55_u16, 55_u16);
    ///     match a_u16
    ///     {
    ///         Some(a) => {
    ///                 println!("{} + 55 = {}", u16::MAX - 55_u16, a);
    ///                 assert_eq!(a, u16::MAX);
    ///             },
    ///         None => {
    ///                 println!("Overflow happened.");
    ///                 assert_eq!(a_u16, None);
    ///             },
    ///     }
    /// 
    ///     let b_u16 = func(a_u16.unwrap(), 1_u16);
    ///     match b_u16
    ///     {
    ///         Some(b) => {
    ///                 println!("{} + 1 = {}", a_u16.unwrap(), b);
    ///                 assert_eq!(b, 0_u16);
    ///             },
    ///         None => {
    ///                 println!("Overflow happened.");
    ///                 assert_eq!(b_u16, None);
    ///             },
    ///     }
    /// 
    ///     let a_u32 = func(u32::MAX - 55_u32, 55_u32);
    ///     match a_u32
    ///     {
    ///         Some(a) => {
    ///                 println!("{} + 55 = {}", u32::MAX - 55_u32, a);
    ///                 assert_eq!(a, u32::MAX);
    ///             },
    ///         None => {
    ///                 println!("Overflow happened.");
    ///                 assert_eq!(a_u32, None);
    ///             },
    ///     }
    /// 
    ///     let b_u32 = func(a_u32.unwrap(), 1_u32);
    ///     match b_u32
    ///     {
    ///         Some(b) => {
    ///                 println!("{} + 1 = {}", a_u32.unwrap(), b);
    ///                 assert_eq!(b, 0_u32);
    ///             },
    ///         None => {
    ///                 println!("Overflow happened.");
    ///                 assert_eq!(b_u32, None);
    ///             },
    ///     }
    /// 
    ///     let a_u64 = func(u64::MAX - 55_u64, 55_u64);
    ///     match a_u64
    ///     {
    ///         Some(a) => {
    ///                 println!("{} + 55 = {}", u64::MAX - 55_u64, a);
    ///                 assert_eq!(a, u64::MAX);
    ///             },
    ///         None => {
    ///                 println!("Overflow happened.");
    ///                 assert_eq!(a_u64, None);
    ///             },
    ///     }
    /// 
    ///     let b_u64 = func(a_u64.unwrap(), 1_u64);
    ///     match b_u64
    ///     {
    ///         Some(b) => {
    ///                 println!("{} + 1 = {}", a_u64.unwrap(), b);
    ///                 assert_eq!(b, 0_u64);
    ///             },
    ///         None => {
    ///                 println!("Overflow happened.");
    ///                 assert_eq!(b_u64, None);
    ///             },
    ///     }
    /// 
    ///     let a_u128 = func(u128::MAX - 55_u128, 55_u128);
    ///     match a_u128
    ///     {
    ///         Some(a) => {
    ///                 println!("{} + 55 = {}", u128::MAX - 55_u128, a);
    ///                 assert_eq!(a, u128::MAX);
    ///             },
    ///         None => {
    ///                 println!("Overflow happened.");
    ///                 assert_eq!(a_u128, None);
    ///             },
    ///     }
    /// 
    ///     let b_u128 = func(a_u128.unwrap(), 1_u128);
    ///     match b_u128
    ///     {
    ///         Some(b) => {
    ///                 println!("{} + 1 = {}", a_u128.unwrap(), b);
    ///                 assert_eq!(b, 0_u128);
    ///             },
    ///         None => {
    ///                 println!("Overflow happened.");
    ///                 assert_eq!(b_u128, None);
    ///             },
    ///     }
    /// 
    ///     let a_usize = func(usize::MAX - 55_usize, 55_usize);
    ///     match a_usize
    ///     {
    ///         Some(a) => {
    ///                 println!("{} + 55 = {}", usize::MAX - 55_usize, a);
    ///                 assert_eq!(a, usize::MAX);
    ///             },
    ///         None => {
    ///                 println!("Overflow happened.");
    ///                 assert_eq!(a_usize, None);
    ///             },
    ///     }
    /// 
    ///     let b_usize = func(a_usize.unwrap(), 1_usize);
    ///     match b_usize
    ///     {
    ///         Some(b) => {
    ///                 println!("{} + 1 = {}", a_usize.unwrap(), b);
    ///                 assert_eq!(b, 0_usize);
    ///             },
    ///         None => {
    ///                 println!("Overflow happened.");
    ///                 assert_eq!(b_usize, None);
    ///             },
    ///     }
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> Option<T>
    /// {
    ///     lhs.checked_add(rhs)
    /// }
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// It calls the method checked_add() of implementation of the primitive
    /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
    /// `usize` directly. So, all the description of this method is mainly the
    /// same as that of the method checked_add() of implementation of the
    /// primitive unsigned integer types except example codes. Confer to the
    /// descryptions that are linked to in the section _Reference_. This
    /// plagiarism is not made maliciously but is made for the reason of
    /// effectiveness and efficiency so that users may understand better and
    /// easily how to use this method with simiilarity to the method
    /// checked_add() of implementation of the primitive unsigned integer types.
    /// 
    /// # References
    /// - If you want to know about the definition of the method `checked_add()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.checked_add).
    /// - If you want to know about the definition of the method `checked_add()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.checked_add).
    /// - If you want to know about the definition of the method `checked_add()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.checked_add).
    /// - If you want to know about the definition of the method `checked_add()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.checked_add).
    /// - If you want to know about the definition of the method `checked_add()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.checked_add).
    /// - If you want to know about the definition of the method `checked_add()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.checked_add).
    fn checked_add(self, rhs: Self) -> Option<Self>;

    /// Computes self + rhs, assuming overflow cannot occur.
    /// 
    /// # Functionalities
    /// It is virtually same as self.checked_add(rhs).unwrap().
    /// Use only when it is sure that overflow will never happen.
    /// 
    /// # Output
    /// It returns the self + rhs in the type of `Self`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     let a_u8 = func(u8::MAX - 55_u8, 55_u8);
    ///     println!("{} + 55 = {}", u8::MAX - 55_u8, a_u8);
    ///     assert_eq!(a_u8, u8::MAX);
    /// 
    ///     // let b_u8 = func(a_u8, 1_u8);     // It will panic
    /// 
    ///     let a_u16 = func(u16::MAX - 55_u16, 55_u16);
    ///     println!("{} + 55 = {}", u16::MAX - 55_u16, a_u16);
    ///     assert_eq!(a_u16, u16::MAX);
    /// 
    ///     // let b_u16 = func(a_u16, 1_u16);  // It will panic
    /// 
    ///     let a_u32 = func(u32::MAX - 55_u32, 55_u32);
    ///     println!("{} + 55 = {}", u32::MAX - 55_u32, a_u32);
    ///     assert_eq!(a_u32, u32::MAX);
    /// 
    ///     // let b_u32 = func(a_u32, 1_u32);  // It will panic
    /// 
    ///     let a_u64 = func(u64::MAX - 55_u64, 55_u64);
    ///     println!("{} + 55 = {}", u64::MAX - 55_u64, a_u64);
    ///     assert_eq!(a_u64, u64::MAX);
    /// 
    ///     // let b_u64 = func(a_u64, 1_u64);  // It will panic
    /// 
    ///     let a_u128 = func(u128::MAX - 55_u128, 55_u128);
    ///     println!("{} + 55 = {}", u128::MAX - 55_u128, a_u128);
    ///     assert_eq!(a_u128, u128::MAX);
    /// 
    ///     // let b_u128 = func(a_u128, 1_u128);   // It will panic
    /// 
    ///     let a_usize = func(usize::MAX - 55_usize, 55_usize);
    ///     println!("{} + 55 = {}", usize::MAX - 55_usize, a_usize);
    ///     assert_eq!(a_usize, usize::MAX);
    /// 
    ///     // let b_usize = func(a_usize, 1_usize);    // It will panic
    /// }
    ///     
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.unchecked_add(rhs)
    /// }
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// Even though it does not call the method unchecked_add() of implementation
    /// of the primitive unsigned integer types such as `u8`, `u16`, `u32`,
    /// `u64`, `u128` and `usize` directly, all the description of this method
    /// is mainly the same as that of the method unchecked_add() of
    /// implementation of the primitive unsigned integer types for nightly
    /// version except example codes. Confer to the descryptions that are linked
    /// to in the section _Reference_. This plagiarism is not made maliciously
    /// but is made for the reason of effectiveness and efficiency so that users
    /// may understand better and easily how to use this method with simiilarity
    /// to the method unchecked_add() of implementation of the primitive unsigned
    /// integer types.
    /// 
    /// # Possiible Changes in Future
    /// This method does not call the method unchecked_add() of the primitive
    /// unsigned integer types directly. Instead, it is implemented to perform
    /// the same thing as that of unchecked_add() of the primitive unsigned
    /// integer types because the methods unchecked_add() of the primitive
    /// unsigned integer types are only for nightly version. So, when the method
    /// unchecked_add() of the primitive unsigned integer types will become a
    /// part of non-nightly normal version, the implementation of this method
    /// will be changed to call the method unchecked_add() of the primitive
    /// unsigned integer types directly.
    /// 
    /// # References
    /// - If you want to know about the definition of the method `unchecked_add()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.unchecked_add).
    /// - If you want to know about the definition of the method `unchecked_add()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.unchecked_add).
    /// - If you want to know about the definition of the method `unchecked_add()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.unchecked_add).
    /// - If you want to know about the definition of the method `unchecked_add()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.unchecked_add).
    /// - If you want to know about the definition of the method `unchecked_add()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.unchecked_add).
    /// - If you want to know about the definition of the method `unchecked_add()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.unchecked_add).
    fn unchecked_add(self, rhs: Self) -> Self;

    /// Computes self + rhs, saturating at the numeric bounds
    /// instead of overflowing.
    /// 
    /// # Functionalities
    /// It adds two numbers with saturating integer addition
    /// 
    /// # Output
    /// It returns the smaller one of self + rhs and the maxium
    /// of the type of `Self`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn UInt_saturating_add___main()
    /// {
    ///     let a_u8 = func(u8::MAX - 55_u8, 55_u8);
    ///     println!("{} + 55 = {}", u8::MAX - 55_u8, a_u8);
    ///     assert_eq!(a_u8, u8::MAX);
    /// 
    ///     let b_u8 = func(a_u8, 55_u8);
    ///     println!("{} + 55 = {}", a_u8, b_u8);
    ///     assert_eq!(b_u8, u8::MAX);
    /// 
    ///     let a_u16 = func(u16::MAX - 55_u16, 55_u16);
    ///     println!("{} + 55 = {}", u16::MAX - 55_u16, a_u16);
    ///     assert_eq!(a_u16, u16::MAX);
    /// 
    ///     let b_u16 = func(a_u16, 55_u16);
    ///     println!("{} + 55 = {}", a_u16, b_u16);
    ///     assert_eq!(b_u16, u16::MAX);
    /// 
    ///     let a_u32 = func(u32::MAX - 55_u32, 55_u32);
    ///     println!("{} + 55 = {}", u32::MAX - 55_u32, a_u32);
    ///     assert_eq!(a_u32, u32::MAX);
    /// 
    ///     let b_u32 = func(a_u32, 55_u32);
    ///     println!("{} + 55 = {}", a_u32, b_u32);
    ///     assert_eq!(b_u32, u32::MAX);
    /// 
    ///     let a_u64 = func(u64::MAX - 55_u64, 55_u64);
    ///     println!("{} + 55 = {}", u64::MAX - 55_u64, a_u64);
    ///     assert_eq!(a_u64, u64::MAX);
    /// 
    ///     let b_u64 = func(a_u64, 55_u64);
    ///     println!("{} + 55 = {}", a_u64, b_u64);
    ///     assert_eq!(b_u64, u64::MAX);
    /// 
    ///     let a_u128 = func(u128::MAX - 55_u128, 55_u128);
    ///     println!("{} + 55 = {}", u128::MAX - 55_u128, a_u128);
    ///     assert_eq!(a_u128, u128::MAX);
    /// 
    ///     let b_u128 = func(a_u128, 55_u128);
    ///     println!("{} + 55 = {}",a_u128, b_u128);
    ///     assert_eq!(b_u128, u128::MAX);
    /// 
    ///     let a_usize = func(usize::MAX - 55_usize, 55_usize);
    ///     println!("{} + 55 = {}", usize::MAX - 55_usize, a_usize);
    ///     assert_eq!(a_usize, usize::MAX);
    /// 
    ///     let b_usize = func(a_usize, 55_usize);
    ///     println!("{} + 55 = {}", a_usize, b_usize);
    ///     assert_eq!(b_usize, usize::MAX);
    /// }
    /// 
    /// fn func<T: Uint>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.saturating_add(rhs)
    /// }    
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// It calls the method saturating_add() of implementation of the primitive
    /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
    /// `usize` directly. So, all the description of this method is mainly the
    /// same as that of the method saturating_add() of implementation of the
    /// primitive unsigned integer types except example codes. Confer to the
    /// descryptions that are linked to in the section _Reference_. This
    /// plagiarism is not made maliciously but is made for the reason of
    /// effectiveness and efficiency so that users may understand better and
    /// easily how to use this method with simiilarity to the method
    /// saturating_add() of implementation of the primitive unsigned integer types.
    /// 
    /// # References
    /// - If you want to know about the definition of the method `saturating_add()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.saturating_add).
    /// - If you want to know about the definition of the method `saturating_add()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.saturating_add).
    /// - If you want to know about the definition of the method `saturating_add()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.saturating_add).
    /// - If you want to know about the definition of the method `saturating_add()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.saturating_add).
    /// - If you want to know about the definition of the method `saturating_add()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.saturating_add).
    /// - If you want to know about the definition of the method `saturating_add()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.saturating_add).
    fn saturating_add(self, rhs: Self) -> Self;



    /***** SUBTRACTION *****/

    /// Calculates self − rhs − borrow,
    /// wrapping around at the boundary of the type. 
    /// 
    /// # Functionalities
    /// This allows chaining together multiple subtractions to create a wider
    /// subtraction, and can be useful for big integer type subtraction.
    /// This can be thought of as a 8-bit “full subtracter”, in the electronics
    /// sense.
    /// 
    /// If the input borrow is false, this method is equivalent to
    /// overflowing_sub, and the output borrow is equal to the underflow flag.
    /// 
    /// # Outputs
    /// It returns a tuple containing the difference and the output borrow.
    /// It performs “ternary subtraction” by subtracting both an integer operand
    /// and a borrow-in bit from self, and returns an output integer and a
    /// borrow-out bit.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::Uint;
    /// fn main()
    /// {
    ///     // a_u16: u16 === (a_high_u8, a_low_u8) == (100_u8, 200_u8) == 25800_u16
    ///     let a_high_u8 = 100_u8;
    ///     let a_low_u8 = 200_u8;
    ///     // b_u16: u16 === (b_high_u8, b_low_u8) == (100_u8, 101_u8) == 25701_u16
    ///     let b_high_u8 = 100_u8;
    ///     let b_low_u8 = 101_u8;
    ///     // c_u16: u16 === (c_high_u8, c_low_u8)
    ///     let c_high_u8: u8;
    ///     let c_low_u8: u8;
    ///     let mut borrow: bool;
    ///     // (100_u8, 200_u8) - (100_u8, 101_u8) == 25800_u16 - 25701_u16 == 99_u16
    ///     //   25800_u16 == (100_u8, 200_u8)
    ///     // - 25701_u16 == (100_u8, 101_u8)
    ///     // -------------------------------
    ///     //      99_u16 == (  0_u8,  99_u8)
    ///     (c_high_u8, c_low_u8, borrow) = sub_long(a_high_u8, a_low_u8, b_high_u8, b_low_u8);
    ///     println!("{}-{}, {}", c_high_u8, c_low_u8, borrow);
    ///     assert_eq!(c_high_u8, 0_u8);
    ///     assert_eq!(c_low_u8, 99_u8);
    ///     assert_eq!(borrow, false);
    /// 
    ///     let d_high_u8: u8;
    ///     let d_low_u8: u8;
    ///     //  (  0_u8,  99_u8) - (100_u8, 101_u8) == 99_u16 - 25701_u16 == 51501_u16
    ///     //      99_u16 == (  0_u8,  99_u8)
    ///     // - 25701_u16 == (100_u8, 101_u8)
    ///     // -------------------------------
    ///     //   39934_u16 == (155_u8, 254_u8)
    ///     (d_high_u8, d_low_u8, borrow) = sub_long(c_high_u8, c_low_u8, b_high_u8, b_low_u8);
    ///     println!("{}-{}, {}", d_high_u8, d_low_u8, borrow);
    ///     assert_eq!(d_high_u8, 155_u8);
    ///     assert_eq!(d_low_u8, 254_u8);
    ///     assert_eq!(borrow, true);
    /// 
    ///     let a_high_u128: u128;
    ///     let a_low_u128: u128;
    ///     //   4201016837757989640311993609423984479246482890531986660185 == (12345678901234567890_u128, 6789012345678912345_u128)
    ///     // -                 419908440780438063913804265570801972943493 == (                1234_u128,                6789_u128)
    ///     // ---------------------------------------------------------------------------------------------------------------------
    ///     //   4201016837757989220403552828985920565442217319730013716692 == (12345678901234566656_u128, 6789012345678905556_u128)
    ///     (a_high_u128, a_low_u128, borrow) = sub_long(12345678901234567890_u128, 6789012345678912345_u128, 1234_u128, 6789_u128);
    ///     println!("{}-{}, {}", a_high_u128, a_low_u128, borrow);
    ///     assert_eq!(a_high_u128, 12345678901234566656_u128);
    ///     assert_eq!(a_low_u128, 6789012345678905556_u128);
    ///     assert_eq!(borrow, false);
    /// 
    ///     let b_high_u128: u128;
    ///     let b_low_u128: u128;
    ///     //    57896044618658097711785492504343953926307055644800578124155540853313808954190 == (170141183460469231731687303715884105727_u128,  12345678901234567890123456789012345678_u128)
    ///     // - 308778904632843187796189293356501087608549893209439890708590319850715068122315 == (226854911280625642308916404954512140970_u128,  56789012345678912345678901234567890123_u128)
    ///     // --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
    ///     //   328077586172395887033451124191282405584107085763563507612853141042164389031555 == (283568639100782052886145506193140176212_u128, 295839033476494119007819162986212667011_u128)
    ///     (b_high_u128, b_low_u128, borrow) = sub_long(170141183460469231731687303715884105727_u128, 12345678901234567890123456789012345678_u128, 226854911280625642308916404954512140970_u128, 56789012345678912345678901234567890123_u128);
    ///     println!("{}-{}, {}", b_high_u128, b_low_u128, carry);
    ///     assert_eq!(b_high_u128, 283568639100782052886145506193140176212_u128);
    ///     assert_eq!(b_low_u128, 295839033476494119007819162986212667011_u128);
    ///     assert_eq!(borrow, true);
    /// }
    /// 
    /// fn sub_long<T: Uint>(lhs_high: T, lhs_low: T, rhs_high: T, rhs_low: T) -> (T, T, bool)
    /// {
    ///     let mut borrow = false;
    ///     let mut sum_high: T;
    ///     let mut sum_low: T;
    ///     (sum_low, borrow) = lhs_low.borrowing_sub(rhs_low, borrow);
    ///     (sum_high, borrow) = lhs_high.borrowing_sub(rhs_high, borrow);
    ///     (sum_high, sum_low, borrow)
    /// }
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// Even though it does not call the method borrowing_sub() of implementation
    /// of the primitive unsigned integer types such as `u8`, `u16`, `u32`,
    /// `u64`, `u128` and `usize` directly, all the description of this method
    /// is mainly the same as that of the method borrowing_sub() of
    /// implementation of the primitive unsigned integer types for nightly
    /// version except example codes. Confer to the descryptions that are linked
    /// to in the section _Reference_. This plagiarism is not made maliciously
    /// but is made for the reason of effectiveness and efficiency so that users
    /// may understand better and easily how to use this method with simiilarity
    /// to the method borrowing_sub() of implementation of the primitive unsigned
    /// integer types.
    /// 
    /// # Possiible Changes in Future
    /// This method does not call the method borrowing_sub() of the primitive
    /// unsigned integer types directly. Instead, it is implemented to perform
    /// the same thing as that of borrowing_sub() of the primitive unsigned
    /// integer types because the methods borrowing_sub() of the primitive
    /// unsigned integer types are only for nightly version. So, when the method
    /// borrowing_sub() of the primitive unsigned integer types will become a
    /// part of non-nightly normal version, the implementation of this method
    /// will be changed to call the method borrowing_sub() of the primitive
    /// unsigned integer types directly.
    /// 
    /// # References
    /// - If you want to know about the definition of the method `borrowing_sub()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.borrowing_sub).
    /// - If you want to know about the definition of the method `borrowing_sub()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.borrowing_sub).
    /// - If you want to know about the definition of the method `borrowing_sub()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.borrowing_sub).
    /// - If you want to know about the definition of the method `borrowing_sub()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.borrowing_sub).
    /// - If you want to know about the definition of the method `borrowing_sub()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.borrowing_sub).
    /// - If you want to know about the definition of the method `borrowing_sub()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.borrowing_sub).
    fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool);

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
    ///                         /// Wrapping (modular) subtraction. Computes self - rhs, wrapping around
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


    fn wrapping_sub(self, rhs: Self) -> Self;
    fn overflowing_sub(self, rhs: Self) -> (Self, bool);
    fn checked_sub(self, rhs: Self) -> Option<Self>;
    fn unchecked_sub(self, rhs: Self) -> Self;
    fn saturating_sub(self, rhs: Self) -> Self;


    /***** MULTIPLICATION *****/


    /// Computes the absolute difference between self and other.
    /// Here, self can be one of the primitive types `u8`, `u16`,
    /// `u32`, `u64`, `u128`, or `usize`. 
    fn abs_diff(self, other: Self) -> Self;


    fn pow(self, exp: u32) -> Self;
    fn ilog(self, base: Self) -> u32;
    fn ilog10(self) -> u32;
    fn ilog2(self) -> u32;
    fn reverse_bits(self) -> Self;
    fn rotate_left(self, n: u32) -> Self;
    fn rotate_right(self, n: u32) -> Self;





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

    fn checked_mul(self, rhs: Self) -> Option<Self>;
    fn checked_div(self, rhs: Self) -> Option<Self>;
    fn checked_rem(self, rhs: Self) -> Option<Self>;
    fn checked_pow(self, exp: u32) -> Option<Self>;

    fn overflowing_mul(self, rhs: Self) -> (Self, bool);
    fn overflowing_div(self, rhs: Self) -> (Self, bool);
    fn overflowing_rem(self, rhs: Self) -> (Self, bool);
    fn overflowing_pow(self, exp: u32) -> (Self, bool);




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
    fn length_in_bytes(&self) -> usize;
    fn length_in_bits(&self) -> usize;
    fn is_odd(&self) -> bool;
}


macro_rules! Uint_for_uint_impl {
    ($f:ty) => {
        impl Uint for $f
        {
            fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool)
            {
                let (r1, c1) = self.overflowing_add(rhs);
                let (r2, c2) = r1.overflowing_add(carry as Self);
                (r2, c1 || c2)
            }

            #[inline] fn wrapping_add(self, rhs: Self) -> Self              { self.wrapping_add(rhs) }
            #[inline] fn overflowing_add(self, rhs: Self) -> (Self, bool)   { self.overflowing_add(rhs) }
            #[inline] fn checked_add(self, rhs: Self) -> Option<Self>       { self.checked_add(rhs) }
            #[inline] fn unchecked_add(self, rhs: Self) -> Self             { self.checked_add(rhs).unwrap() }
            #[inline] fn saturating_add(self, rhs: Self) -> Self            { self.saturating_add(rhs) }

            fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool)
            {
                let (r1, b1) = self.overflowing_sub(rhs);
                let (r2, b2) = r1.overflowing_sub(borrow as Self);
                (r2, b1 || b2)
            }

            #[inline] fn wrapping_sub(self, rhs: Self) -> Self              { self.wrapping_sub(rhs) }
            #[inline] fn overflowing_sub(self, rhs: Self) -> (Self, bool)   { self.overflowing_sub(rhs) }
            #[inline] fn checked_sub(self, rhs: Self) -> Option<Self>       { self.checked_sub(rhs) }
            #[inline] fn unchecked_sub(self, rhs: Self) -> Self             { self.checked_sub(rhs).unwrap() }
            #[inline] fn saturating_sub(self, rhs: Self) -> Self            { self.saturating_sub(rhs) }






            #[inline]
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
            
            fn checked_mul(self, rhs: Self) -> Option<Self> { self.checked_mul(rhs) }
            fn checked_div(self, rhs: Self) -> Option<Self> { self.checked_div(rhs) }
            fn checked_rem(self, rhs: Self) -> Option<Self> { self.checked_rem(rhs) }
            fn checked_pow(self, exp: u32) -> Option<Self>  { self.checked_pow(exp) }

            fn overflowing_mul(self, rhs: Self) -> (Self, bool) { self.overflowing_mul(rhs) }
            fn overflowing_div(self, rhs: Self) -> (Self, bool) { self.overflowing_div(rhs) }
            fn overflowing_rem(self, rhs: Self) -> (Self, bool) { self.overflowing_rem(rhs) }
            fn overflowing_pow(self, exp: u32) -> (Self, bool)  { self.overflowing_pow(exp) }


            fn saturating_mul(self, rhs: Self) -> Self  { self.saturating_mul(rhs) }
            fn saturating_div(self, rhs: Self) -> Self  { self.saturating_div(rhs) }
            fn saturating_pow(self, exp: u32) -> Self   { self.saturating_pow(exp) }

            fn count_ones(self) -> u32      { self.count_ones() }
            fn count_zeros(self) -> u32     { self.count_zeros() }
            fn leading_ones(self) -> u32    { self.leading_ones() }
            fn leading_zeros(self) -> u32   { self.leading_zeros() }
            fn trailing_ones(self) -> u32   { self.trailing_ones() }
            fn trailing_zeros(self) -> u32  { self.trailing_zeros() }

            fn from_be(x: Self) -> Self { Self::from_be(x) }
            fn from_le(x: Self) -> Self { Self::from_le(x) }
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
            #[inline] fn size_in_bytes() -> usize { size_of::<Self>() }
            #[inline] fn size_in_bits() -> usize  { size_of::<Self>() * 8 }
            #[inline] fn length_in_bytes(&self) -> usize    { size_of_val(self) }
            #[inline] fn length_in_bits(&self) -> usize     { size_of_val(self) * 8 }
            #[inline] fn is_odd(&self) -> bool    { (*self & 1) != 0 }
        }
    }
}



Uint_for_uint_impl! { u8 }
Uint_for_uint_impl! { u16 }
Uint_for_uint_impl! { u32 }
Uint_for_uint_impl! { u64 }
Uint_for_uint_impl! { u128 }
Uint_for_uint_impl! { usize }

/*
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
    #[inline] fn size_in_bytes() -> usize { size_of::<Self>() }
    #[inline] fn size_in_bits() -> usize  { size_of::<Self>() * 8 }
    #[inline] fn length_in_bytes(&self) -> usize    { size_of_val(self) }
    #[inline] fn length_in_bits(&self) -> usize     { size_of_val(self) * 8 }
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
    #[inline] fn size_in_bytes() -> usize { size_of::<Self>() }
    #[inline] fn size_in_bits() -> usize  { size_of::<Self>() * 8 }
    #[inline] fn length_in_bytes(&self) -> usize    { size_of_val(self) }
    #[inline] fn length_in_bits(&self) -> usize     { size_of_val(self) * 8 }
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
    #[inline] fn size_in_bytes() -> usize { size_of::<Self>() }
    #[inline] fn size_in_bits() -> usize  { size_of::<Self>() * 8 }
    #[inline] fn length_in_bytes(&self) -> usize    { size_of_val(self) }
    #[inline] fn length_in_bits(&self) -> usize     { size_of_val(self) * 8 }
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
    #[inline] fn size_in_bytes() -> usize { size_of::<Self>() }
    #[inline] fn size_in_bits() -> usize  { size_of::<Self>() * 8 }
    #[inline] fn length_in_bytes(&self) -> usize    { size_of_val(self) }
    #[inline] fn length_in_bits(&self) -> usize     { size_of_val(self) * 8 }
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
    #[inline] fn size_in_bytes() -> usize { size_of::<Self>() }
    #[inline] fn size_in_bits() -> usize  { size_of::<Self>() * 8 }
    #[inline] fn length_in_bytes(&self) -> usize    { size_of_val(self) }
    #[inline] fn length_in_bits(&self) -> usize     { size_of_val(self) * 8 }
}
*/

#[derive(Copy, Clone)]
pub union UShort
{
    pub this: u16,
    pub ushort: u16,
    pub byte: [u8; 2],
    #[cfg(target_pointer_width = "16")] pub size: usize,
}

#[derive(Copy, Clone)]
pub union UInt
{
    pub this: u32,
    pub uint: u32,
    pub ushort: [u16; 2],
    pub byte: [u8; 4],
    #[cfg(target_pointer_width = "32")] pub size: usize,
    #[cfg(target_pointer_width = "16")] pub size: [usize; 2],
}

#[derive(Copy, Clone)]
pub union ULong
{
    pub this: u64,
    pub ulong: u64,
    pub uint: [u32; 2],
    pub ushort: [u16; 4],
    pub byte: [u8; 8],
    #[cfg(target_pointer_width = "64")] pub size: usize,
    #[cfg(target_pointer_width = "32")] pub size: [usize; 2],
    #[cfg(target_pointer_width = "16")] pub size: [usize; 4],
}

#[derive(Copy, Clone)]
pub union ULonger
{
    pub this: u128,
    pub ulonger: u128,
    pub ulong: [u64; 2],
    pub uint: [u32; 4],
    pub ushort: [u16; 8],
    pub byte: [u8; 16],
    #[cfg(target_pointer_width = "128")] pub size: usize,
    #[cfg(target_pointer_width = "64")] pub size: [usize; 2],
    #[cfg(target_pointer_width = "32")] pub size: [usize; 4],
    #[cfg(target_pointer_width = "16")] pub size: [usize; 8],
}

#[cfg(target_pointer_width = "128")]
#[derive(Copy, Clone)]
pub union USize
{
    pub this: usize,
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
    pub this: usize,
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
    pub this: usize,
    pub size: usize,
    pub uint: u32,
    pub ushort: [u16; 2],
    pub byte: [u8; 4],
}

#[cfg(target_pointer_width = "16")]
#[derive(Copy, Clone)]
pub union USize
{
    pub this: usize,
    pub size: usize,
    pub ushort: u16,
    pub byte: [u8; 2],
}

#[cfg(target_pointer_width = "8")]
#[derive(Copy, Clone)]
pub union USize
{
    pub this: usize,
    pub size: usize,
    pub byte: u8,
}



macro_rules! get_set_byte {
    ($f:expr) => {
        const N: usize = $f;
    
        #[cfg(target_endian = "little")]
        pub fn get_byte_(&self, i: usize) -> u8 { unsafe { self.byte[i] } }
    
        #[cfg(target_endian = "big")]
        pub fn get_byte_(&self, i: usize) -> u8 { unsafe { self.byte[Self::N-i] } }
    
        #[cfg(target_endian = "little")]
        pub fn get_byte(&self, i: usize) -> Option<u8>
        {
            if i <= Self::N
                { unsafe { Some(self.byte[i]) } }
            else
                { None }
        }
    
        #[cfg(target_endian = "big")]
        pub fn get_byte(&self, i: usize) -> Option<u8>
        {
            if i <= Self::N
                { unsafe { Some(self.byte[Self::N-i]) } }
            else
                { None }
        }
    
        #[cfg(target_endian = "little")]
        pub fn set_byte_(&mut self, i: usize, val: u8)  { unsafe { self.byte[i] = val; } }
    
        #[cfg(target_endian = "big")]
        pub fn set_byte_(&mut self, i: usize, val: u8)  { unsafe { self.byte[Self::N-i] = val; } }
    
        #[cfg(target_endian = "little")]
        pub fn set_byte(&mut self, i: usize, val: u8) -> bool
        {
            if i < Self::N
            { 
                unsafe { self.byte[i] = val; }
                true
            }
            else
            {
                false
            }
        }
    
        #[cfg(target_endian = "big")]
        pub fn set_byte(&self, i: usize, val: u8) -> bool
        {
            if i < Self::N
            { 
                unsafe { self.byte[Self::N-i] = val; }
                true
            }
            else
            {
                false
            }
        }
    }
}


macro_rules! get_set_ushort {
    ($f:expr) => {
        const M: usize = $f;

        #[cfg(target_endian = "little")]
        pub fn get_ushort_(&self, i: usize) -> u16 { unsafe { self.ushort[i] } }

        #[cfg(target_endian = "big")]
        pub fn get_ushort_(&self, i: usize) -> u16 { unsafe { self.ushort[Self::M-i] } }

        #[cfg(target_endian = "little")]
        pub fn get_ushort(&self, i: usize) -> Option<u16>
        {
            if i <= Self::M
                { unsafe { Some(self.ushort[i]) } }
            else
                { None }
        }

        #[cfg(target_endian = "big")]
        pub fn get_ushort(&self, i: usize) -> Option<u16>
        {
            if i <= Self::M
                { unsafe { Some(self.ushort[Self::M-i]) } }
            else
                { None }
        }

        #[cfg(target_endian = "little")]
        pub fn set_ushort_(&mut self, i: usize, val: u16)  { unsafe { self.ushort[i] = val; } }

        #[cfg(target_endian = "big")]
        pub fn set_ushort_(&mut self, i: usize, val: u16)  { unsafe { self.ushort[Self::M-i] = val; } }

        #[cfg(target_endian = "little")]
        pub fn set_ushort(&mut self, i: usize, val: u16) -> bool
        {
            if i <= Self::M
            { 
                unsafe { self.ushort[i] = val; }
                true
            }
            else
            {
                false
            }
        }

        #[cfg(target_endian = "big")]
        pub fn set_ushort(&self, i: usize, val: u16) -> bool
        {
            if i <= Self::M
            { 
                unsafe { self.ushort[Self::M-i] = val; }
                true
            }
            else
            {
                false
            }
        }
    }
}


macro_rules! get_set_uint {
    ($f:expr) => {
        const L: usize = $f;

        #[cfg(target_endian = "little")]
        pub fn get_uint_(&self, i: usize) -> u32 { unsafe { self.uint[i] } }

        #[cfg(target_endian = "big")]
        pub fn get_uint_(&self, i: usize) -> u32 { unsafe { self.uint[Self::L-i] } }

        #[cfg(target_endian = "little")]
        pub fn get_uint(&self, i: usize) -> Option<u32>
        {
            if i <= Self::L
                { unsafe { Some(self.uint[i]) } }
            else
                { None }
        }

        #[cfg(target_endian = "big")]
        pub fn get_uint(&self, i: usize) -> Option<u32>
        {
            if i <= Self::L
                { unsafe { Some(self.uint[Self::L-i]) } }
            else
                { None }
        }

        #[cfg(target_endian = "little")]
        pub fn set_uint_(&mut self, i: usize, val: u32)  { unsafe { self.uint[i] = val; } }

        #[cfg(target_endian = "big")]
        pub fn set_uint_(&mut self, i: usize, val: u32)  { unsafe { self.uint[Self::L-i] = val; } }

        #[cfg(target_endian = "little")]
        pub fn set_uint(&mut self, i: usize, val: u32) -> bool
        {
            if i <= Self::L
            { 
                unsafe { self.uint[i] = val; }
                true
            }
            else
            {
                false
            }
        }

        #[cfg(target_endian = "big")]
        pub fn set_uint(&self, i: usize, val: u32) -> bool
        {
            if i <= Self::L
            { 
                unsafe { self.uint[Self::L-i] = val; }
                true
            }
            else
            {
                false
            }
        }
    }
}

macro_rules! get_set_ulong {
    ($f:expr) => {
        const K: usize = $f;

        #[cfg(target_endian = "little")]
        pub fn get_ulong_(&self, i: usize) -> u64 { unsafe { self.ulong[i] } }

        #[cfg(target_endian = "big")]
        pub fn get_ulong_(&self, i: usize) -> u64 { unsafe { self.ulong[Self::K-i] } }

        #[cfg(target_endian = "little")]
        pub fn get_ulong(&self, i: usize) -> Option<u64>
        {
            if i <= Self::K
                { unsafe { Some(self.ulong[i]) } }
            else
                { None }
        }

        #[cfg(target_endian = "big")]
        pub fn get_ulong(&self, i: usize) -> Option<u64>
        {
            if i <= Self::K
                { unsafe { Some(self.ulong[Self::K-i]) } }
            else
                { None }
        }

        #[cfg(target_endian = "little")]
        pub fn set_ulong_(&mut self, i: usize, val: u64)  { unsafe { self.ulong[i] = val; } }

        #[cfg(target_endian = "big")]
        pub fn set_ulong_(&mut self, i: usize, val: u64)  { unsafe { self.ulong[Self::K-i] = val; } }

        #[cfg(target_endian = "little")]
        pub fn set_ulong(&mut self, i: usize, val: u64) -> bool
        {
            if i <= Self::L
            { 
                unsafe { self.ulong[i] = val; }
                true
            }
            else
            {
                false
            }
        }

        #[cfg(target_endian = "big")]
        pub fn set_ulong(&self, i: usize, val: u64) -> bool
        {
            if i <= Self::K
            { 
                unsafe { self.ulong[Self::K-i] = val; }
                true
            }
            else
            {
                false
            }
        }
    }
}

macro_rules! get_set_usize {
    ($f:expr) => {
        const J: usize = $f;

        #[cfg(target_endian = "little")]
        pub fn get_usize_(&self, i: usize) -> usize { unsafe { self.size[i] } }

        #[cfg(target_endian = "big")]
        pub fn get_usize_(&self, i: usize) -> usize { unsafe { self.size[Self::J-i] } }

        #[cfg(target_endian = "little")]
        pub fn get_usize(&self, i: usize) -> Option<usize>
        {
            if i <= Self::J
                { unsafe { Some(self.size[i]) } }
            else
                { None }
        }

        #[cfg(target_endian = "big")]
        pub fn get_usize(&self, i: usize) -> Option<usize>
        {
            if i <= Self::J
                { unsafe { Some(self.size[Self::J-i]) } }
            else
                { None }
        }

        #[cfg(target_endian = "little")]
        pub fn set_usize_(&mut self, i: usize, val: usize)  { unsafe { self.size[i] = val; } }

        #[cfg(target_endian = "big")]
        pub fn set_usize_(&mut self, i: usize, val: usize)  { unsafe { self.size[Self::J-i] = val; } }

        #[cfg(target_endian = "little")]
        pub fn set_usize(&mut self, i: usize, val: usize) -> bool
        {
            if i <= Self::J
            { 
                unsafe { self.size[i] = val; }
                true
            }
            else
            {
                false
            }
        }

        #[cfg(target_endian = "big")]
        pub fn set_ulong(&self, i: usize, val: usize) -> bool
        {
            if i <= Self::J
            { 
                unsafe { self.size[Self::J-i] = val; }
                true
            }
            else
            {
                false
            }
        }
    }
}



impl UShort
{
    pub fn new() -> Self                    { Self { ushort: 0 } }
    pub fn new_with(ushort: u16) -> Self    { Self { ushort } }

    get_set_byte!(2-1);
    #[cfg(target_pointer_width = "8")]      get_set_usize!(2-1);
}

impl UInt
{
    pub fn new() -> Self                { Self { uint: 0 } }
    pub fn new_with(uint: u32) -> Self  { Self { uint } }

    get_set_byte!(4-1);
    get_set_ushort!(2-1);
    #[cfg(target_pointer_width = "16")]     get_set_usize!(2-1);
    #[cfg(target_pointer_width = "8")]      get_set_usize!(4-1);
}

impl ULong
{
    pub fn new() -> Self                    { Self { ulong: 0 } }
    pub fn new_with(ulong: u64) -> Self     { Self { ulong } }

    get_set_byte!(8-1);
    get_set_ushort!(4-1);
    get_set_uint!(2-1);
    #[cfg(target_pointer_width = "32")]     get_set_usize!(2-1);
    #[cfg(target_pointer_width = "16")]     get_set_usize!(4-1);
    #[cfg(target_pointer_width = "8")]      get_set_usize!(8-1);
}

impl ULonger
{
    pub fn new() -> Self                    { Self { ulonger: 0 } }
    pub fn new_with(ulonger: u128) -> Self  { Self { ulonger } }

    get_set_byte!(16-1);
    get_set_ushort!(8-1);
    get_set_uint!(4-1);
    get_set_ulong!(2-1);
    #[cfg(target_pointer_width = "64")]     get_set_usize!(2-1);
    #[cfg(target_pointer_width = "32")]     get_set_usize!(4-1);
    #[cfg(target_pointer_width = "16")]     get_set_usize!(8-1);
    #[cfg(target_pointer_width = "8")]      get_set_usize!(16-1);
}

impl USize
{
    pub fn new() -> Self                    { Self { size: 0 } }
    pub fn new_with(size: usize) -> Self    { Self { size } }

    #[cfg(target_pointer_width = "128")]    get_set_byte!(16-1);
    #[cfg(target_pointer_width = "64")]     get_set_byte!(8-1);
    #[cfg(target_pointer_width = "32")]     get_set_byte!(4-1);
    #[cfg(target_pointer_width = "16")]     get_set_byte!(2-1);

    #[cfg(target_pointer_width = "128")]    get_set_ushort!(8-1);
    #[cfg(target_pointer_width = "64")]     get_set_ushort!(4-1);
    #[cfg(target_pointer_width = "32")]     get_set_ushort!(2-1);

    #[cfg(target_pointer_width = "128")]    get_set_uint!(4-1);
    #[cfg(target_pointer_width = "64")]     get_set_uint!(2-1);

    #[cfg(target_pointer_width = "128")]    get_set_ulong!(2-1);
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
    pub this: usize,

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

