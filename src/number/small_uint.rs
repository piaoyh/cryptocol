// Copyright 2023 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module that contains generic types of primitive unsigned integral
//! data types used in a lot of modules of the crate Cryptocol.
//! __The trait SmallUInt is meaningful when you use it in generic context.__

#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

use std::fmt::{ Debug, Display };
use std::mem::{ size_of, size_of_val };
use std::cmp::{ PartialEq, PartialOrd, Ordering };
use std::ops::*;

use rand::{ RngCore, thread_rng, Rng };
use rand::rngs::OsRng;

use super::small_int_unions::*;

/// Trait SmallUInt is for generic type of primitive unsigned integer data types
/// for all modules of the crate Cryptocol.
/// __The trait SmallUInt is meaningful when you use it in generic context.
/// Otherwise, it is pretty hard to imagine its usability.__
/// In order to use this trait, you have to import (use)
/// `Cryptocol::number::SmallUInt`.
///  
/// Here, the generic type of primitive unsigned integral data types includes:
/// `u8`, `u16`, `u32`, `u64`, `u128` and `usize`. In order to use this trait,
/// you have to import (use) `Cryptocol::number::SmallUInt`.
/// 
/// You will, however, hardly use the trait SmallUInt unless you use primitive
/// unsigned integral data types in generic context, or you improve or modify
/// this crate Cryptocol, or you create addional libraries that works with the
/// crate Cryptocol. So, if you only use the crate Cryptocol or you will not
/// use primitive unsigned integral data types in generic context, you can
/// almost forget about this trait SmallUInt.
pub trait SmallUInt: Copy + Clone + Sized //+ Display + Debug + ToString
{
    /***** ADDITION *****/

    // fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool);
    /// Calculates self + rhs + carry,
    /// wrapping around at the boundary of the type.
    /// 
    /// # Features
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
    /// This allows chaining together multiple additions to create a wider
    /// addition, and can be useful for big integer type addition. This can be
    /// thought of as a 8-bit “full adder”, in the electronics sense.
    /// 
    /// If the input carry is false, this method is equivalent to
    /// `overflowing_add()`, and the output carry is equal to the overflow flag.
    /// 
    /// # Outputs
    /// It returns a tuple containing the sum and the output carry. It performs
    /// “ternary addition” of two integer operands and a carry-in bit, and
    /// returns an output integer and a carry-out bit.
    /// 
    /// # Example for u8
    /// ```
    /// use Cryptocol::number::SmallUInt;
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
    /// }
    /// 
    /// fn add_long<T: SmallUInt>(lhs_high: T, lhs_low: T, rhs_high: T, rhs_low: T) -> (T, T, bool)
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
    /// # Example for u128
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
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
    /// fn add_long<T: SmallUInt>(lhs_high: T, lhs_low: T, rhs_high: T, rhs_low: T) -> (T, T, bool)
    /// {
    ///     let mut carry = false;
    ///     let mut sum_high: T;
    ///     let mut sum_low: T;
    ///     (sum_low, carry) = lhs_low.carrying_add(rhs_low, carry);
    ///     (sum_high, carry) = lhs_high.carrying_add(rhs_high, carry);
    ///     (sum_high, sum_low, carry)
    /// }
    /// ```
    /// You can use the above generic function add_long<>() for all primitive
    /// data types in a same scope. Look into the next example.
    /// 
    /// # Collective Example
    /// ```
    /// use Cryptocol::number::SmallUInt;
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
    ///     //  (201_u8,  45_u8) + (100_u8    /// , 200_u8) == 25701_u16 + 25800_u16 == 51501_u16
    ///     //   25701_u16 == (100_u8, 101_u8)
    ///     // + 25800_u16 == (100_u8, 200_u8)
    ///     // -------------------------------
    ///     //   11765_u16 == ( 45_u8, 245_u8)
    ///     (d_high_u8, d_low_u8, carry) = add_long(c_high_u8, c_low_u8, b_high_u8, b_low_u8);
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
    ///     let (b_high_u128, b_low_u128, carry) = add_long(226854911280625642308916404954512140970_u128, 56789012345678912345678901234567890123_u128, 170141183460469231731687303715884105727_u128, 12345678901234567890123456789012345678_u128);
    ///     println!("{}-{}, {}", b_high_u128, b_low_u128, carry);
    ///     assert_eq!(b_high_u128, 56713727820156410577229101238628035241_u128);
    ///     assert_eq!(b_low_u128, 69134691246913480235802358023580235801_u128);
    ///     assert_eq!(carry, true);
    /// }
    /// 
    /// fn add_long<T: SmallUInt>(lhs_high: T, lhs_low: T, rhs_high: T, rhs_low: T) -> (T, T, bool)
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
    /// # Features
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
    /// It adds two numbers with wrapping (modular) addition.
    /// 
    /// # Output
    /// It returns the self + rhs in the type of `Self`.
    /// 
    /// # Example for u8
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u8 = func(u8::MAX - 55_u8, 55_u8);
    ///     println!("{} + 55 = {}", u8::MAX - 55_u8, a_u8);
    ///     assert_eq!(a_u8, u8::MAX);
    /// 
    ///     let b_u8 = func(a_u8, 1_u8);
    ///     println!("{} + 1 = {}", a_u8, b_u8);
    ///     assert_eq!(b_u8, 0_u8);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_add(rhs)
    /// }
    /// ```
    /// 
    /// # Example for u16
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u16 = func(u16::MAX - 55_u16, 55_u16);
    ///     println!("{} + 55 = {}", u16::MAX - 55_u16, a_u16);
    ///     assert_eq!(a_u16, u16::MAX);
    /// 
    ///     let b_u16 = func(a_u16, 1_u16);
    ///     println!("{} + 1 = {}", a_u16, b_u16);
    ///     assert_eq!(b_u16, 0_u16);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_add(rhs)
    /// }
    /// ```
    /// 
    /// # Example for u32
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u32 = func(u32::MAX - 55_u32, 55_u32);
    ///     println!("{} + 55 = {}", u32::MAX - 55_u32, a_u32);
    ///     assert_eq!(a_u32, u32::MAX);
    /// 
    ///     let b_u32 = func(a_u32, 1_u32);
    ///     println!("{} + 1 = {}", a_u32, b_u32);
    ///     assert_eq!(b_u32, 0_u32);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_add(rhs)
    /// }
    /// ```
    /// 
    /// # Example for u64
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u64 = func(u64::MAX - 55_u64, 55_u64);
    ///     println!("{} + 55 = {}", u64::MAX - 55_u64, a_u64);
    ///     assert_eq!(a_u64, u64::MAX);
    /// 
    ///     let b_u64 = func(a_u64, 1_u64);
    ///     println!("{} + 1 = {}", a_u64, b_u64);
    ///     assert_eq!(b_u64, 0_u64);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_add(rhs)
    /// }
    /// ```
    /// 
    /// # Example for u128
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u128 = func(u128::MAX - 55_u128, 55_u128);
    ///     println!("{} + 55 = {}", u128::MAX - 55_u128, a_u128);
    ///     assert_eq!(a_u128, u128::MAX);
    /// 
    ///     let b_u128 = func(a_u128, 1_u128);
    ///     println!("{} + 1 = {}",a_u128, b_u128);
    ///     assert_eq!(b_u128, 0_u128);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_add(rhs)
    /// }
    /// ```
    /// 
    /// # Example for usize
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_usize = func(usize::MAX - 55_usize, 55_usize);
    ///     println!("{} + 55 = {}", usize::MAX - 55_usize, a_usize);
    ///     assert_eq!(a_usize, usize::MAX);
    /// 
    ///     let b_usize = func(a_usize, 1_usize);
    ///     println!("{} + 1 = {}", a_usize, b_usize);
    ///     assert_eq!(b_usize, 0_usize);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_add(rhs)
    /// }
    /// ```
    /// You can use the above generic function func<>() for all SmallUInt-supported
    /// data types in a same scope. Look into the next example.
    /// 
    /// # Collective Example
    /// ```
    /// use Cryptocol::number::SmallUInt;
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
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
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
    /// # Features
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
    /// It adds two numbers with wrapping (modular) addition. It is the same as
    /// the method carrying_add() with the imput carry which is false.
    /// 
    /// # Output
    /// It returns a tuple of the addition along with a boolean indicating
    /// whether an arithmetic overflow would occur. If an overflow would
    /// have occurred then the wrapped value is returned.
    /// 
    /// # Example for u8
    /// ```
    /// use Cryptocol::number::SmallUInt;
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
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> (T, bool)
    /// {
    ///     lhs.overflowing_add(rhs)
    /// }
    /// ```
    /// 
    /// # Example for u16
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u16 = func(u16::MAX - 55_u16, 55_u16);
    ///     println!("{} + 55 = {}\nOverflow = {}", u16::MAX - 55_u16, a_u16.0, a_u16.1);
    ///     assert_eq!(a_u16.0, u16::MAX);
    ///     assert_eq!(a_u16.1, false);
    /// 
    ///     let b_u16 = func(a_u16.0, 1_u16);
    ///     println!("{} + 1 = {}\nOverflow = {}", a_u16.0, b_u16.0, b_u16.1);
    ///     assert_eq!(b_u16.0, 0_u16);
    ///     assert_eq!(b_u16.1, true);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> (T, bool)
    /// {
    ///     lhs.overflowing_add(rhs)
    /// }
    /// ```
    /// 
    /// # Example for u32
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u32 = func(u32::MAX - 55_u32, 55_u32);
    ///     println!("{} + 55 = {}\nOverflow = {}", u32::MAX - 55_u32, a_u32.0, a_u32.1);
    ///     assert_eq!(a_u32.0, u32::MAX);
    ///     assert_eq!(a_u32.1, false);
    /// 
    ///     let b_u32 = func(a_u32.0, 1_u32);
    ///     println!("{} + 1 = {}\nOverflow = {}", a_u32.0, b_u32.0, b_u32.1);
    ///     assert_eq!(b_u32.0, 0_u32);
    ///     assert_eq!(b_u32.1, true);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> (T, bool)
    /// {
    ///     lhs.overflowing_add(rhs)
    /// }
    /// ```
    /// 
    /// # Example for u64
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u64 = func(u64::MAX - 55_u64, 55_u64);
    ///     println!("{} + 55 = {}\nOverflow = {}", u64::MAX - 55_u64, a_u64.0, a_u64.1);
    ///     assert_eq!(a_u64.0, u64::MAX);
    ///     assert_eq!(a_u64.1, false);
    /// 
    ///     let b_u64 = func(a_u64.0, 1_u64);
    ///     println!("{} + 1 = {}\nOverflow = {}", a_u64.0, b_u64.0, b_u64.1);
    ///     assert_eq!(b_u64.0, 0_u64);
    ///     assert_eq!(b_u64.1, true);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> (T, bool)
    /// {
    ///     lhs.overflowing_add(rhs)
    /// }
    /// ```
    /// 
    /// # Example for u128
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u128 = func(u128::MAX - 55_u128, 55_u128);
    ///     println!("{} + 55 = {}\nOverflow = {}", u128::MAX - 55_u128, a_u128.0, a_u128.1);
    ///     assert_eq!(a_u128.0, u128::MAX);
    ///     assert_eq!(a_u128.1, false);
    /// 
    ///     let b_u128 = func(a_u128.0, 1_u128);
    ///     println!("{} + 1 = {}\nOverflow = {}", a_u128.0, b_u128.0, b_u128.1);
    ///     assert_eq!(b_u128.0, 0_u128);
    ///     assert_eq!(b_u128.1, true);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> (T, bool)
    /// {
    ///     lhs.overflowing_add(rhs)
    /// }
    /// ```
    /// 
    /// # Example for usize
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
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
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> (T, bool)
    /// {
    ///     lhs.overflowing_add(rhs)
    /// }
    /// ```
    /// You can use the above generic function func<>() for all SmallUInt-supported
    /// data types in a same scope. Look into the next example.
    /// 
    /// # Collective Example
    /// ```
    /// use Cryptocol::number::SmallUInt;
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
    ///     assert_eq!(b_u128.1, true);
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
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> (T, bool)
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

    /// Computes self + rhs.
    /// 
    /// # Feature
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
    /// 
    /// # Output
    /// It returns self + rhs in the type `Self` wrapped by `Some`
    /// of enum `Option` if overflow did not occur.
    /// And, it returns `None` if overflow occurred.
    /// 
    /// # Example for u8
    /// ```
    /// use Cryptocol::number::SmallUInt;
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
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> Option<T>
    /// {
    ///     lhs.checked_add(rhs)
    /// }
    /// ```
    /// 
    /// # Example for u16
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
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
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> Option<T>
    /// {
    ///     lhs.checked_add(rhs)
    /// }
    /// ```
    /// 
    /// # Example for u32
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
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
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> Option<T>
    /// {
    ///     lhs.checked_add(rhs)
    /// }
    /// ```
    /// 
    /// # Example for u64
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
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
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> Option<T>
    /// {
    ///     lhs.checked_add(rhs)
    /// }
    /// ```
    /// 
    /// # Example for u128
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
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
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> Option<T>
    /// {
    ///     lhs.checked_add(rhs)
    /// }
    /// ```
    /// 
    /// # Example for usize
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
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
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> Option<T>
    /// {
    ///     lhs.checked_add(rhs)
    /// }
    /// ```
    /// You can use the above generic function func<>() for all SmallUInt-supported
    /// data types in a same scope. Look into the next example.
    /// 
    /// # Collective Example
    /// ```
    /// use Cryptocol::number::SmallUInt;
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
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> Option<T>
    /// {
    ///     lhs.checked_add(rhs)
    /// }
    /// ```
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
    /// # Features
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
    /// It is virtually same as self.checked_add(rhs).unwrap().
    /// Use this method only when it is sure that overflow will never happen.
    /// 
    /// # Panics
    /// If overflow occurs, this method will panic at this version.
    /// 
    /// # Output
    /// It returns self + rhs in the type `Self` if overflow did not occur.
    /// Otherwise, its behavior is not defined.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::SmallUInt;
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
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
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
    /// # Features
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
    /// It adds two numbers with saturating integer addition
    /// 
    /// # Output
    /// It returns the smaller one of self + rhs and the maxium
    /// of the type of `Self`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
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
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
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

    // fn modular_add(&self, rhs: Self, modulo: Self) -> Self
    /// Computes (`self` + `rhs`) % `modulo`, wrapping around at `modulo` of the
    /// type `Self` instead of overflowing.
    /// 
    /// # Output
    /// It returns the modulo-sum (`self` + `rhs`) % `modulo` with wrapping
    /// (modular) addition at `modulo`.
    /// 
    /// # Feature
    /// Wrapping (modular) addition at `modulo`. The differences between this
    /// method `modular_add_uint()` and the method `wrapping_add_uint()` are,
    /// first, where wrapping around happens, and, second, whether or not
    /// `OVERFLOW` flag is set. First, this method wraps araound at `modulo`
    /// while the method `wrapping_add_uint()` wraps araound at maximum value.
    /// Second, this method does not set `OVERFLOW` flag even if wrapping
    /// around happens while the method `wrapping_add_uint()` sets `OVERFLOW`
    /// flag when wrapping around happens.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method `modular_add()` is proper
    /// rather than this method `modular_add_uint()`.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    fn modular_add(self, rhs: Self, modulo: Self) -> Self;


    /***** SUBTRACTION *****/

    /// Calculates self − rhs − borrow,
    /// wrapping around at the boundary of the type. 
    /// 
    /// # Features
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
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
    /// use Cryptocol::number::SmallUInt;
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
    /// fn sub_long<T: SmallUInt>(lhs_high: T, lhs_low: T, rhs_high: T, rhs_low: T) -> (T, T, bool)
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

    /// Computes self - rhs, wrapping around at the boundary of the type.
    /// 
    /// # Features
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
    /// It subtracts rhs from self with wrapping (modular) subtraction.
    /// 
    /// # Output
    /// It returns the self - rhs in the type of `Self`.
    /// 
    /// # Example for u8
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u8 = func(55_u8, 55_u8);
    ///     println!("55 - 55 = {}", a_u8);
    ///     assert_eq!(a_u8, 0_u8);
    /// 
    ///     let b_u8 = func(a_u8, 1_u8);
    ///     println!("{} - 1 = {}", a_u8, b_u8);
    ///     assert_eq!(b_u8, u8::MAX);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_sub(rhs)
    /// }
    /// ```
    /// 
    /// # Example for u16
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u16 = func(55_u16, 55_u16);
    ///     println!("55 - 55 = {}", a_u16);
    ///     assert_eq!(a_u16, 0_u16);
    /// 
    ///     let b_u16 = func(a_u16, 1_u16);
    ///     println!("{} - 1 = {}", a_u16, b_u16);
    ///     assert_eq!(b_u16, u16::MAX);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_sub(rhs)
    /// }
    /// ```
    /// 
    /// # Example for u32
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    /// 
    ///     let a_u32 = func(55_u32, 55_u32);
    ///     println!("55 - 55 = {}", a_u32);
    ///     assert_eq!(a_u32, 0_u32);
    /// 
    ///     let b_u32 = func(a_u32, 1_u32);
    ///     println!("{} - 1 = {}", a_u32, b_u32);
    ///     assert_eq!(b_u32, u32::MAX);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_sub(rhs)
    /// }
    /// ```
    /// 
    /// # Example for u64
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u64 = func(55_u64, 55_u64);
    ///     println!("55 - 55 = {}", a_u64);
    ///     assert_eq!(a_u64, 0_u64);
    /// 
    ///     let b_u64 = func(a_u64, 1_u64);
    ///     println!("{} - 1 = {}", a_u64, b_u64);
    ///     assert_eq!(b_u64, u64::MAX);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_sub(rhs)
    /// }
    /// ```
    /// 
    /// # Example for u128
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u128 = func(55_u128, 55_u128);
    ///     println!("55 - 55 = {}", a_u128);
    ///     assert_eq!(a_u128, 0_u128);
    /// 
    ///     let b_u128 = func(a_u128, 1_u128);
    ///     println!("{} - 1 = {}",a_u128, b_u128);
    ///     assert_eq!(b_u128, u128::MAX);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_sub(rhs)
    /// }
    /// ```
    /// 
    /// # Example for usize
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_usize = func(55_usize, 55_usize);
    ///     println!("55 - 55 = {}", a_usize);
    ///     assert_eq!(a_usize, 0_usize);
    /// 
    ///     let b_usize = func(a_usize, 1_usize);
    ///     println!("{} - 1 = {}", a_usize, b_usize);
    ///     assert_eq!(b_usize, usize::MAX);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_sub(rhs)
    /// }
    /// ```
    /// You can use the above generic function func<>() for all SmallUInt-supported
    /// data types in a same scope. Look into the next example.
    /// 
    /// # Collective Example
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u8 = func(55_u8, 55_u8);
    ///     println!("55 - 55 = {}", a_u8);
    ///     assert_eq!(a_u8, 0_u8);
    /// 
    ///     let b_u8 = func(a_u8, 1_u8);
    ///     println!("{} - 1 = {}", a_u8, b_u8);
    ///     assert_eq!(b_u8, u8::MAX);
    /// 
    ///     let a_u16 = func(55_u16, 55_u16);
    ///     println!("55 - 55 = {}", a_u16);
    ///     assert_eq!(a_u16, 0_u16);
    /// 
    ///     let b_u16 = func(a_u16, 1_u16);
    ///     println!("{} - 1 = {}", a_u16, b_u16);
    ///     assert_eq!(b_u16, u16::MAX);
    /// 
    ///     let a_u32 = func(55_u32, 55_u32);
    ///     println!("55 - 55 = {}", a_u32);
    ///     assert_eq!(a_u32, 0_u32);
    /// 
    ///     let b_u32 = func(a_u32, 1_u32);
    ///     println!("{} - 1 = {}", a_u32, b_u32);
    ///     assert_eq!(b_u32, u32::MAX);
    /// 
    ///     let a_u64 = func(55_u64, 55_u64);
    ///     println!("55 - 55 = {}", a_u64);
    ///     assert_eq!(a_u64, 0_u64);
    /// 
    ///     let b_u64 = func(a_u64, 1_u64);
    ///     println!("{} - 1 = {}", a_u64, b_u64);
    ///     assert_eq!(b_u64, u64::MAX);
    /// 
    ///     let a_u128 = func(55_u128, 55_u128);
    ///     println!("55 - 55 = {}", a_u128);
    ///     assert_eq!(a_u128, 0_u128);
    /// 
    ///     let b_u128 = func(a_u128, 1_u128);
    ///     println!("{} - 1 = {}",a_u128, b_u128);
    ///     assert_eq!(b_u128, u128::MAX);
    /// 
    ///     let a_usize = func(55_usize, 55_usize);
    ///     println!("55 - 55 = {}", a_usize);
    ///     assert_eq!(a_usize, 0_usize);
    /// 
    ///     let b_usize = func(a_usize, 1_usize);
    ///     println!("{} - 1 = {}", a_usize, b_usize);
    ///     assert_eq!(b_usize, usize::MAX);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_sub(rhs)
    /// }
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// It calls the method wrapping_sub() of implementation of the primitive
    /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
    /// `usize` directly. So, all the description of this method is mainly the
    /// same as that of the method wrapping_sub() of implementation of the
    /// primitive unsigned integer types except example codes. Confer to the
    /// descryptions that are linked to in the section _Reference_. This
    /// plagiarism is not made maliciously but is made for the reason of
    /// effectiveness and efficiency so that users may understand better and
    /// easily how to use this method with simiilarity to the method
    /// wrapping_sub() of implementation of the primitive unsigned integer types.
    /// 
    /// # References
    /// - If you want to know about the definition of the method `wrapping_sub()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.wrapping_sub).
    /// - If you want to know about the definition of the method `wrapping_sub()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.wrapping_sub).
    /// - If you want to know about the definition of the method `wrapping_sub()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.wrapping_sub).
    /// - If you want to know about the definition of the method `wrapping_sub()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.wrapping_sub).
    /// - If you want to know about the definition of the method `wrapping_sub()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.wrapping_sub).
    /// - If you want to know about the definition of the method `wrapping_sub()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.wrapping_sub).
    fn wrapping_sub(self, rhs: Self) -> Self;
    
    /// Calculates self - rhs, wrapping around at the boundary of the type.
    /// 
    /// # Features
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
    /// It subtracts rhs from self with wrapping (modular) subtraction.
    /// It is the same as the method carrying_sub() with the imput carry which
    /// is false.
    /// 
    /// # Output
    /// It returns a tuple of the subtraction along with a boolean indicating
    /// whether an arithmetic underflow would occur. If an underflow would
    /// have occurred then the wrapped value is returned.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u8 = func(55_u8, 55_u8);
    ///     println!("55 - 55 = {}\nUnderflow = {}", a_u8.0, a_u8.1);
    ///     assert_eq!(a_u8.0, 0_u8);
    ///     assert_eq!(a_u8.1, false);
    ///  
    ///     let b_u8 = func(a_u8.0, 1_u8);
    ///     println!("{} - 1 = {}\nUnderflow = {}", a_u8.0, b_u8.0, b_u8.1);
    ///     assert_eq!(b_u8.0, u8::MAX);
    ///     assert_eq!(b_u8.1, true);
    /// 
    ///     let a_u16 = func(55_u16, 55_u16);
    ///     println!("55 - 55 = {}\nUnderflow = {}", a_u16.0, a_u16.1);
    ///     assert_eq!(a_u16.0, 0_u16);
    ///     assert_eq!(a_u16.1, false);
    ///  
    ///     let b_u16 = func(a_u16.0, 1_u16);
    ///     println!("{} - 1 = {}\nUnderflow = {}", a_u16.0, b_u16.0, b_u16.1);
    ///     assert_eq!(b_u16.0, u16::MAX);
    ///     assert_eq!(b_u16.1, true);
    /// 
    ///     let a_u32 = func(55_u32, 55_u32);
    ///     println!("55 - 55 = {}\nUnderflow = {}", a_u32.0, a_u32.1);
    ///     assert_eq!(a_u32.0, 0_u32);
    ///     assert_eq!(a_u32.1, false);
    ///  
    ///     let b_u32 = func(a_u32.0, 1_u32);
    ///     println!("{} - 1 = {}\nUnderflow = {}", a_u32.0, b_u32.0, b_u32.1);
    ///     assert_eq!(b_u32.0, u32::MAX);
    ///     assert_eq!(b_u32.1, true);
    /// 
    ///     let a_u64 = func(55_u64, 55_u64);
    ///     println!("55 - 55 = {}\nUnderflow = {}", a_u64.0, a_u64.1);
    ///     assert_eq!(a_u64.0, 0_u64);
    ///     assert_eq!(a_u64.1, false);
    ///  
    ///     let b_u64 = func(a_u64.0, 1_u64);
    ///     println!("{} - 1 = {}\nUnderflow = {}", a_u64.0, b_u64.0, b_u64.1);
    ///     assert_eq!(b_u64.0, u64::MAX);
    ///     assert_eq!(b_u64.1, true);
    /// 
    ///     let a_u128 = func(55_u128, 55_u128);
    ///     println!("55 - 55 = {}\nUnderflow = {}", a_u128.0, a_u128.1);
    ///     assert_eq!(a_u128.0, 0_u128);
    ///     assert_eq!(a_u128.1, false);
    ///  
    ///     let b_u128 = func(a_u128.0, 1_u128);
    ///     println!("{} - 1 = {}\nUnderflow = {}", a_u128.0, b_u128.0, b_u128.1);
    ///     assert_eq!(b_u128.0, u128::MAX);
    ///     assert_eq!(b_u128.1, true);
    /// 
    ///     let a_usize = func(55_usize, 55_usize);
    ///     println!("55 - 55 = {}\nUnderflow = {}", a_usize.0, a_usize.1);
    ///     assert_eq!(a_usize.0, 0_usize);
    ///     assert_eq!(a_usize.1, false);
    ///  
    ///     let b_usize = func(a_usize.0, 1_usize);
    ///     println!("{} - 1 = {}\nUnderflow = {}", a_usize.0, b_usize.0, b_usize.1);
    ///     assert_eq!(b_usize.0, usize::MAX);
    ///     assert_eq!(b_usize.1, true);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> (T, bool)
    /// {
    ///     lhs.overflowing_sub(rhs)
    /// }        
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// It calls the method overflowing_sub() of implementation of the primitive
    /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
    /// `usize` directly. So, all the description of this method is mainly the
    /// same as that of the method overflowing_sub() of implementation of the
    /// primitive unsigned integer types except example codes. Confer to the
    /// descryptions that are linked to in the section _Reference_. This
    /// plagiarism is not made maliciously but is made for the reason of
    /// effectiveness and efficiency so that users may understand better and
    /// easily how to use this method with simiilarity to the method
    /// overflowing_sub() of implementation of the primitive unsigned integer
    /// types.
    /// 
    /// # References
    /// - If you want to know about the definition of the method `overflowing_sub()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.overflowing_sub).
    /// - If you want to know about the definition of the method `overflowing_sub()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.overflowing_sub).
    /// - If you want to know about the definition of the method `overflowing_sub()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.overflowing_sub).
    /// - If you want to know about the definition of the method `overflowing_sub()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.overflowing_sub).
    /// - If you want to know about the definition of the method `overflowing_sub()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.overflowing_sub).
    /// - If you want to know about the definition of the method `overflowing_sub()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.overflowing_sub).
    fn overflowing_sub(self, rhs: Self) -> (Self, bool);
    
    /// Computes self - rhs.
    /// 
    /// # Feature
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
    /// 
    /// # Output
    /// It returns self - rhs in the type `Self` wrapped by `Some`
    /// of enum `Option` if overflow did not occur.
    /// And, it returns `None` if overflow occurred.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u8 = func(55_u8, 55_u8);
    ///     match a_u8
    ///     {
    ///         Some(a) => {
    ///                 println!("55 - 55 = {}", a);
    ///                 assert_eq!(a, 0_u8);
    ///             },
    ///         None => {
    ///                 println!("Underflow happened.");
    ///                 assert_eq!(a_u8, None);
    ///             },
    ///     }
    ///  
    ///     let b_u8 = func(a_u8.unwrap(), 1_u8);
    ///     match b_u8
    ///     {
    ///         Some(b) => {
    ///                 println!("{} - 1 = {}", a_u8.unwrap(), b);
    ///                 assert_eq!(b, u8::MAX);
    ///             },
    ///         None => {
    ///                 println!("Underflow happened.");
    ///                 assert_eq!(b_u8, None);
    ///             },
    ///     }
    /// 
    ///     let a_u16 = func(55_u16, 55_u16);
    ///     match a_u16
    ///     {
    ///         Some(a) => {
    ///                 println!("55 - 55 = {}", a);
    ///                 assert_eq!(a, 0_u16);
    ///             },
    ///         None => {
    ///                 println!("Underflow happened.");
    ///                 assert_eq!(a_u16, None);
    ///             },
    ///     }
    /// 
    ///     let b_u16 = func(a_u16.unwrap(), 1_u16);
    ///     match b_u16
    ///     {
    ///         Some(b) => {
    ///                 println!("{} - 1 = {}", a_u16.unwrap(), b);
    ///                 assert_eq!(b, u16::MAX);
    ///             },
    ///         None => {
    ///                 println!("Underflow happened.");
    ///                 assert_eq!(b_u16, None);
    ///             },
    ///     }
    /// 
    ///     let a_u32 = func(55_u32, 55_u32);
    ///     match a_u32
    ///     {
    ///         Some(a) => {
    ///                 println!("55 - 55 = {}", a);
    ///                 assert_eq!(a, 0_u32);
    ///             },
    ///         None => {
    ///                 println!("Underflow happened.");
    ///                 assert_eq!(a_u32, None);
    ///             },
    ///     }
    /// 
    ///     let b_u32 = func(a_u32.unwrap(), 1_u32);
    ///     match b_u32
    ///     {
    ///         Some(b) => {
    ///                 println!("{} - 1 = {}", a_u32.unwrap(), b);
    ///                 assert_eq!(b, u32::MAX);
    ///             },
    ///         None => {
    ///                 println!("Underflow happened.");
    ///                 assert_eq!(b_u32, None);
    ///             },
    ///     }
    /// 
    ///     let a_u64 = func(55_u64, 55_u64);
    ///     match a_u64
    ///     {
    ///         Some(a) => {
    ///                 println!("55 - 55 = {}", a);
    ///                 assert_eq!(a, 0_u64);
    ///             },
    ///         None => {
    ///                 println!("Underflow happened.");
    ///                 assert_eq!(a_u64, None);
    ///             },
    ///     }
    /// 
    ///     let b_u64 = func(a_u64.unwrap(), 1_u64);
    ///     match b_u64
    ///     {
    ///         Some(b) => {
    ///                 println!("{} + 1 = {}", a_u64.unwrap(), b);
    ///                 assert_eq!(b, u64::MAX);
    ///             },
    ///         None => {
    ///                 println!("Underflow happened.");
    ///                 assert_eq!(b_u64, None);
    ///             },
    ///     }
    /// 
    ///     let a_u128 = func(55_u128, 55_u128);
    ///     match a_u128
    ///     {
    ///         Some(a) => {
    ///                 println!("55 - 55 = {}", a);
    ///                 assert_eq!(a, 0_u128);
    ///             },
    ///         None => {
    ///                 println!("Underflow happened.");
    ///                 assert_eq!(a_u128, None);
    ///             },
    ///     }
    /// 
    ///     let b_u128 = func(a_u128.unwrap(), 1_u128);
    ///     match b_u128
    ///     {
    ///         Some(b) => {
    ///                 println!("{} - 1 = {}", a_u128.unwrap(), b);
    ///                 assert_eq!(b, u128::MAX);
    ///             },
    ///         None => {
    ///                 println!("Underflow happened.");
    ///                 assert_eq!(b_u128, None);
    ///             },
    ///     }
    /// 
    ///     let a_usize = func(55_usize, 55_usize);
    ///     match a_usize
    ///     {
    ///         Some(a) => {
    ///                 println!("55 - 55 = {}", a);
    ///                 assert_eq!(a, 0_usize);
    ///             },
    ///         None => {
    ///                 println!("Underflow happened.");
    ///                 assert_eq!(a_usize, None);
    ///             },
    ///     }
    /// 
    ///     let b_usize = func(a_usize.unwrap(), 1_usize);
    ///     match b_usize
    ///     {
    ///         Some(b) => {
    ///                 println!("{} - 1 = {}", a_usize.unwrap(), b);
    ///                 assert_eq!(b, usize::MAX);
    ///             },
    ///         None => {
    ///                 println!("Underflow happened.");
    ///                 assert_eq!(b_usize, None);
    ///             },
    ///     }
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> Option<T>
    /// {
    ///     lhs.checked_sub(rhs)
    /// }    
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// It calls the method checked_sub() of implementation of the primitive
    /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
    /// `usize` directly. So, all the description of this method is mainly the
    /// same as that of the method checked_sub() of implementation of the
    /// primitive unsigned integer types except example codes. Confer to the
    /// descryptions that are linked to in the section _Reference_. This
    /// plagiarism is not made maliciously but is made for the reason of
    /// effectiveness and efficiency so that users may understand better and
    /// easily how to use this method with simiilarity to the method
    /// checked_sub() of implementation of the primitive unsigned integer types.
    /// 
    /// # References
    /// - If you want to know about the definition of the method `checked_sub()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.checked_sub).
    /// - If you want to know about the definition of the method `checked_sub()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.checked_sub).
    /// - If you want to know about the definition of the method `checked_sub()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.checked_sub).
    /// - If you want to know about the definition of the method `checked_sub()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.checked_sub).
    /// - If you want to know about the definition of the method `checked_sub()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.checked_sub).
    /// - If you want to know about the definition of the method `checked_sub()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.checked_sub).
    fn checked_sub(self, rhs: Self) -> Option<Self>;

    /// Computes self - rhs, assuming overflow cannot occur.
    /// 
    /// # Features
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
    /// It is virtually same as self.checked_sub(rhs).unwrap().
    /// Use this method only when it is sure that underflow will never happen.
    /// 
    /// # Panics
    /// If underflow occurs, this method will panic at this version.
    /// 
    /// # Output
    /// It returns self - rhs in the type `Self` if underflow did not occur.
    /// Otherwise, its behavior is not defined.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u8 = func(55_u8, 55_u8);
    ///     println!("55 - 55 = {}", a_u8);
    ///     assert_eq!(a_u8, 0_u8);
    /// 
    ///     // It will panic
    ///     // let b_u8 = func(a_u8, 1_u8);
    /// 
    ///     let a_u16 = func(55_u16, 55_u16);
    ///     println!("55 - 55 = {}", a_u16);
    ///     assert_eq!(a_u16, 0_u16);
    /// 
    ///     // It will panic
    ///     // let b_u16 = func(a_u16, 1_u16);
    /// 
    ///     let a_u32 = func(55_u32, 55_u32);
    ///     println!("55 - 55 = {}", a_u32);
    ///     assert_eq!(a_u32, 0_u32);
    /// 
    ///     // It will panic
    ///     // let b_u32 = func(a_u32, 1_u32);
    /// 
    ///     let a_u64 = func(55_u64, 55_u64);
    ///     println!("55 - 55 = {}", a_u64);
    ///     assert_eq!(a_u64, 0_u64);
    /// 
    ///     // It will panic
    ///     // let b_u64 = func(a_u64, 1_u64);
    /// 
    ///     let a_u128 = func(55_u128, 55_u128);
    ///     println!("55 - 55 = {}", a_u128);
    ///     assert_eq!(a_u128, 0_u128);
    /// 
    ///     // It will panic
    ///     // let b_u128 = func(a_u128, 1_u128);
    /// 
    ///     let a_usize = func(55_usize, 55_usize);
    ///     println!("55 - 55 = {}", a_usize);
    ///     assert_eq!(a_usize, 0_usize);
    /// 
    ///     // It will panic
    ///     // let b_usize = func(a_usize, 1_usize);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.unchecked_sub(rhs)
    /// }
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// Even though it does not call the method unchecked_sub() of implementation
    /// of the primitive unsigned integer types such as `u8`, `u16`, `u32`,
    /// `u64`, `u128` and `usize` directly, all the description of this method
    /// is mainly the same as that of the method unchecked_sub() of
    /// implementation of the primitive unsigned integer types for nightly
    /// version except example codes. Confer to the descryptions that are linked
    /// to in the section _Reference_. This plagiarism is not made maliciously
    /// but is made for the reason of effectiveness and efficiency so that users
    /// may understand better and easily how to use this method with simiilarity
    /// to the method unchecked_sub() of implementation of the primitive unsigned
    /// integer types.
    /// 
    /// # Possiible Changes in Future
    /// This method does not call the method unchecked_sub() of the primitive
    /// unsigned integer types directly. Instead, it is implemented to perform
    /// the same thing as that of unchecked_sub() of the primitive unsigned
    /// integer types because the methods unchecked_sub() of the primitive
    /// unsigned integer types are only for nightly version. So, when the method
    /// unchecked_sub() of the primitive unsigned integer types will become a
    /// part of non-nightly normal version, the implementation of this method
    /// will be changed to call the method unchecked_sub() of the primitive
    /// unsigned integer types directly.
    /// 
    /// # References
    /// - If you want to know about the definition of the method `unchecked_sub()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.unchecked_sub).
    /// - If you want to know about the definition of the method `unchecked_sub()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.unchecked_sub).
    /// - If you want to know about the definition of the method `unchecked_sub()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.unchecked_sub).
    /// - If you want to know about the definition of the method `unchecked_sub()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.unchecked_sub).
    /// - If you want to know about the definition of the method `unchecked_sub()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.unchecked_sub).
    /// - If you want to know about the definition of the method `unchecked_sub()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.unchecked_sub).
    fn unchecked_sub(self, rhs: Self) -> Self;
    
    /// Computes self - rhs, saturating at the numeric bounds
    /// instead of underflowing.
    /// 
    /// # Features
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
    /// It subtracts rhs from self with saturating integer subtraction.
    /// 
    /// # Output
    /// It returns the bigger one of self - rhs and the zero
    /// of the type of `Self`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u8 = func(55_u8, 50_u8);
    ///     println!("55 - 50 = {}", a_u8);
    ///     assert_eq!(a_u8, 5_u8);
    /// 
    ///     let b_u8 = func(a_u8, 55_u8);
    ///     println!("5 - 55 = {}", b_u8);
    ///     assert_eq!(b_u8, 0_u8);
    /// 
    ///     let a_u16 = func(55_u16, 50_u16);
    ///     println!("55 - 50 = {}", a_u16);
    ///     assert_eq!(a_u16, 5_u16);
    /// 
    ///     let b_u16 = func(a_u16, 55_u16);
    ///     println!("5 - 55 = {}", b_u16);
    ///     assert_eq!(b_u16, 0_u16);
    /// 
    ///     let a_u32 = func(55_u32, 50_u32);
    ///     println!("55 - 50 = {}", a_u32);
    ///     assert_eq!(a_u32, 5_u32);
    /// 
    ///     let b_u32 = func(a_u32, 55_u32);
    ///     println!("{} - 55 = {}", a_u32, b_u32);
    ///     assert_eq!(b_u32, 0_u32);
    /// 
    ///     let a_u64 = func(55_u64, 50_u64);
    ///     println!("55 - 50 = {}", a_u64);
    ///     assert_eq!(a_u64, 5_u64);
    /// 
    ///     let b_u64 = func(a_u64, 55_u64);
    ///     println!("{} - 55 = {}", a_u64, b_u64);
    ///     assert_eq!(b_u64, 0_u64);
    /// 
    ///     let a_u128 = func(55_u128, 50_u128);
    ///     println!("55 - 50 = {}", a_u128);
    ///     assert_eq!(a_u128, 5_u128);
    /// 
    ///     let b_u128 = func(a_u128, 55_u128);
    ///     println!("{} - 55 = {}", a_u128, b_u128);
    ///     assert_eq!(b_u128, 0_u128);
    /// 
    ///     let a_usize = func(55_usize, 50_usize);
    ///     println!("55 - 50 = {}", a_usize);
    ///     assert_eq!(a_usize, 5_usize);
    /// 
    ///     let b_usize = func(a_usize, 55_usize);
    ///     println!("{} - 55 = {}", a_usize, b_usize);
    ///     assert_eq!(b_usize, 0_usize);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.saturating_sub(rhs)
    /// }
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// It calls the method saturating_sub() of implementation of the primitive
    /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
    /// `usize` directly. So, all the description of this method is mainly the
    /// same as that of the method saturating_sub() of implementation of the
    /// primitive unsigned integer types except example codes. Confer to the
    /// descryptions that are linked to in the section _Reference_. This
    /// plagiarism is not made maliciously but is made for the reason of
    /// effectiveness and efficiency so that users may understand better and
    /// easily how to use this method with simiilarity to the method
    /// saturating_sub() of implementation of the primitive unsigned integer types.
    /// 
    /// # References
    /// - If you want to know about the definition of the method `saturating_sub()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.saturating_sub).
    /// - If you want to know about the definition of the method `saturating_sub()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.saturating_sub).
    /// - If you want to know about the definition of the method `saturating_sub()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.saturating_sub).
    /// - If you want to know about the definition of the method `saturating_sub()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.saturating_sub).
    /// - If you want to know about the definition of the method `saturating_sub()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.saturating_sub).
    /// - If you want to know about the definition of the method `saturating_sub()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.saturating_sub).
    fn saturating_sub(self, rhs: Self) -> Self;


    /// Computes the absolute difference between `self` and `other`.
    /// 
    /// # Feature
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
    /// 
    /// # Output
    /// It returns the absolute difference between `self` and `other`.
    /// 
    /// # Example for u8
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u8 = func(55_u8, 50_u8);
    ///     println!("55 <-> 50 = {}", a_u8);
    ///     assert_eq!(a_u8, 5_u8);
    ///     
    ///     let b_u8 = func(50_u8, 55_u8);
    ///     println!("50 <-> 55 = {}", b_u8);
    ///     assert_eq!(b_u8, 5_u8);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.abs_diff(rhs)
    /// }
    /// ```
    /// 
    /// # Example for u16
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {   
    ///     let a_u16 = func(5050_u16, 5000_u16);
    ///     println!("5050 <-> 5000 = {}", a_u16);
    ///     assert_eq!(a_u16, 50_u16);
    ///     
    ///     let b_u16 = func(5000_u16, 5050_u16);
    ///     println!("5000 <-> 5050 = {}", b_u16);
    ///     assert_eq!(b_u16, 50_u16);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.abs_diff(rhs)
    /// }
    /// ```
    /// 
    /// # Example for u32
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u32 = func(500500_u32, 500000_u32);
    ///     println!("500500 <-> 500000 = {}", a_u32);
    ///     assert_eq!(a_u32, 500_u32);
    ///     
    ///     let b_u32 = func(500000_u32, 500500_u32);
    ///     println!("500000 <-> 500500 = {}", b_u32);
    ///     assert_eq!(b_u32, 500_u32);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.abs_diff(rhs)
    /// }
    /// ```
    /// 
    /// # Example for u64
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u64 = func(5000050000_u64, 5000000000_u64);
    ///     println!("5000050000 <-> 5000000000 = {}", a_u64);
    ///     assert_eq!(a_u64, 50000_u64);
    ///     
    ///     let b_u64 = func(5000000000_u64, 5000050000_u64);
    ///     println!("5000000000 <-> 5000050000 = {}", b_u64);
    ///     assert_eq!(b_u64, 50000_u64);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.abs_diff(rhs)
    /// }
    /// ```
    /// 
    /// # Example for u128
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u128 = func(500000000500000000_u128, 500000000000000000_u128);
    ///     println!("500000000500000000 <-> 500000000000000000 = {}", a_u128);
    ///     assert_eq!(a_u128, 500000000_u128);
    ///     
    ///     let b_u128 = func(500000000000000000_u128, 500000000500000000_u128);
    ///     println!("500000000000000000 <-> 500000000500000000 = {}", b_u128);
    ///     assert_eq!(b_u128, 500000000_u128);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.abs_diff(rhs)
    /// }
    /// ```
    /// 
    /// # Example for usize
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_usize = func(5000050000_usize, 5000000000_usize);
    ///     println!("5000050000 <-> 5000000000 = {}", a_usize);
    ///     assert_eq!(a_usize, 50000_usize);
    ///     
    ///     let b_usize = func(5000000000_usize, 5000050000_usize);
    ///     println!("5000000000 <-> 5000050000 = {}", b_usize);
    ///     assert_eq!(b_usize, 50000_usize);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.abs_diff(rhs)
    /// }
    /// ```
    /// You can use the above generic function func<>() for all SmallUInt-supported
    /// data types in a same scope. Look into the next example.
    /// 
    /// # Collective Example
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u8 = func(55_u8, 50_u8);
    ///     println!("55 <-> 50 = {}", a_u8);
    ///     assert_eq!(a_u8, 5_u8);
    ///     
    ///     let b_u8 = func(50_u8, 55_u8);
    ///     println!("50 <-> 55 = {}", b_u8);
    ///     assert_eq!(b_u8, 5_u8);
    ///     
    ///     let a_u16 = func(5050_u16, 5000_u16);
    ///     println!("5050 <-> 5000 = {}", a_u16);
    ///     assert_eq!(a_u16, 50_u16);
    ///     
    ///     let b_u16 = func(5000_u16, 5050_u16);
    ///     println!("5000 <-> 5050 = {}", b_u16);
    ///     assert_eq!(b_u16, 50_u16);
    ///     
    ///     let a_u32 = func(500500_u32, 500000_u32);
    ///     println!("500500 <-> 500000 = {}", a_u32);
    ///     assert_eq!(a_u32, 500_u32);
    ///     
    ///     let b_u32 = func(500000_u32, 500500_u32);
    ///     println!("500000 <-> 500500 = {}", b_u32);
    ///     assert_eq!(b_u32, 500_u32);
    ///     
    ///     let a_u64 = func(5000050000_u64, 5000000000_u64);
    ///     println!("5000050000 <-> 5000000000 = {}", a_u64);
    ///     assert_eq!(a_u64, 50000_u64);
    ///     
    ///     let b_u64 = func(5000000000_u64, 5000050000_u64);
    ///     println!("5000000000 <-> 5000050000 = {}", b_u64);
    ///     assert_eq!(b_u64, 50000_u64);
    ///     
    ///     let a_u128 = func(500000000500000000_u128, 500000000000000000_u128);
    ///     println!("500000000500000000 <-> 500000000000000000 = {}", a_u128);
    ///     assert_eq!(a_u128, 500000000_u128);
    ///     
    ///     let b_u128 = func(500000000000000000_u128, 500000000500000000_u128);
    ///     println!("500000000000000000 <-> 500000000500000000 = {}", b_u128);
    ///     assert_eq!(b_u128, 500000000_u128);
    ///     
    ///     let a_usize = func(5000050000_usize, 5000000000_usize);
    ///     println!("5000050000 <-> 5000000000 = {}", a_usize);
    ///     assert_eq!(a_usize, 50000_usize);
    ///     
    ///     let b_usize = func(5000000000_usize, 5000050000_usize);
    ///     println!("5000000000 <-> 5000050000 = {}", b_usize);
    ///     assert_eq!(b_usize, 50000_usize);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.abs_diff(rhs)
    /// }
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// It calls the method abs_diff() of implementation of the primitive
    /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
    /// `usize` directly. So, all the description of this method is mainly the
    /// same as that of the method abs_diff() of implementation of the
    /// primitive unsigned integer types except example codes. Confer to the
    /// descryptions that are linked to in the section _Reference_. This
    /// plagiarism is not made maliciously but is made for the reason of
    /// effectiveness and efficiency so that users may understand better and
    /// easily how to use this method with simiilarity to the method
    /// abs_diff() of implementation of the primitive unsigned integer types.
    /// 
    /// # References
    /// - If you want to know about the definition of the method `abs_diff()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.abs_diff).
    /// - If you want to know about the definition of the method `abs_diff()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.abs_diff).
    /// - If you want to know about the definition of the method `abs_diff()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.abs_diff).
    /// - If you want to know about the definition of the method `abs_diff()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.abs_diff).
    /// - If you want to know about the definition of the method `abs_diff()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.abs_diff).
    /// - If you want to know about the definition of the method `abs_diff()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.abs_diff).
    fn abs_diff(self, other: Self) -> Self;

    // pub fn modular_sub(self, rhs: Self, modulo: Self) -> Self
    /// Computes (`self` - `rhs`) % `modulo`, wrapping around at `modulo` of the
    /// type `Self` instead of underflowing.
    /// 
    /// # Output
    /// It returns the modulo-difference (`self` - `rhs`) % `modulo` with
    /// wrapping (modular) subtraction at `modulo`.
    /// 
    /// # Feature
    /// Wrapping (modular) subtraction at `modulo`. The differences between
    /// this method `modular_sub_uint()` and the method `wrapping_sub_uint()`
    /// are, first, where wrapping around happens, and, second, whether or not
    /// `UNDERFLOW` flag is set. First, this method wraps araound at `modulo`
    /// while the method `wrapping_sub_uint()` wraps araound at maximum value.
    /// Second, this method does not set `UNDERFLOW` flag even if wrapping
    /// around happens while the method `wrapping_sub_uint()` sets `UNDERFLOW`
    /// flag when wrapping around happens.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method `modular_sub_uint()` is
    /// proper rather than this method.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    fn modular_sub(self, rhs: Self, modulo: Self) -> Self;


    /***** MULTIPLICATION *****/

    // pub fn carrying_mul(self, rhs: Self, carry: Self) -> (Self, Self)
    /// Calculates the “full multiplication” `self` * `rhs` + `carry` without
    /// the possibility to overflow.
    /// 
    /// # Output
    /// It returns `self` * `rhs` + `carry` in the form of a tuple of the
    /// low-order (wrapping) bits and the high-order (overflow) bits of the
    /// result as two separate values, in that order.
    /// 
    /// # Feature
    /// It performs “long multiplication” which takes in an extra amount to add,
    /// and may return an additional amount of overflow. This allows for
    /// chaining together multiple multiplications to create “big integers”
    /// which represent larger values.
    /// 
    /// # Counterpart Methods
    /// If you don’t need the carry, then you can use `widening_mul()` instead.
    /// 
    /// The value of the first field in the returned tuple matches what you’d
    /// get by combining the `wrapping_mul()` and `wrapping_add()` methods:
    /// `self.wrapping_mul(rhs).wrapping_add(carry)`. So,
    /// `self.carrying_mul(rhs, carry).0` == `self.wrapping_mul(rhs).wrapping_add(carry)`
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// Even though it does not call the method `carrying_mul()` of
    /// implementation of the primitive unsigned integer types such as `u8`,
    /// `u16`, `u32`, `u64`, `u128` and `usize` directly, all the description
    /// of this method is mainly the same as that of the method `carrying_mul()`
    /// of implementation of the primitive unsigned integer types for nightly
    /// version except example codes. Confer to the descryptions that are linked
    /// to in the section _Reference_. This plagiarism is not made maliciously
    /// but is made for the reason of effectiveness and efficiency so that users
    /// may understand better and easily how to use this method with simiilarity
    /// to the method `carrying_mul()` of implementation of the primitive
    /// unsigned integer types.
    /// 
    /// # Possiible Changes in Future
    /// This method does not call the method `carrying_mul()` of the primitive
    /// unsigned integer types directly. Instead, it is implemented to perform
    /// the same thing as that of `carrying_mul()` of the primitive unsigned
    /// integer types because the methods `carrying_mul()` of the primitive
    /// unsigned integer types are only for nightly version. So, when the method
    /// `carrying_mul()` of the primitive unsigned integer types will become a
    /// part of non-nightly normal version, the implementation of this method
    /// will be changed to call the method `carrying_mul()` of the primitive
    /// unsigned integer types directly.
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    /// 
    /// # References
    /// - If you want to know about the definition of the method `carrying_mul()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.carrying_mul).
    /// - If you want to know about the definition of the method `carrying_mul()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.carrying_mul).
    /// - If you want to know about the definition of the method `carrying_mul()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.carrying_mul).
    /// - If you want to know about the definition of the method `carrying_mul()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.carrying_mul).
    /// - If you want to know about the definition of the method `carrying_mul()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.carrying_mul).
    /// - If you want to know about the definition of the method `carrying_mul()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.carrying_mul).
    fn carrying_mul(self, rhs: Self, carry: Self) -> (Self, Self);

    // fn carrying_mul_for_internal_use(self, rhs: Self, carry: Self) -> (Self, Self);
    /// It is for internal use. You are recommended to use [carrying_mul()](trait@SmallUInt#tymethod.carrying_mul) instead.
    fn carrying_mul_for_internal_use(self, rhs: Self, carry: Self) -> (Self, Self);
    
    // pub fn widening_mul(self, rhs: Self) -> (Self, Self)
    /// Calculates the complete product `self` * `rhs` without the possibility
    /// to overflow.
    /// 
    /// # Output
    /// It returns `self` * `rhs` in the form of a tuple of the low-order
    /// (wrapping) bits and the high-order (overflow) bits of the result as
    /// two separate values, in that order.
    /// 
    /// /// # Feature
    /// It performs “long multiplication” which takes in an extra amount to add,
    /// and may return an additional amount of overflow. This allows for
    /// chaining together multiple multiplications to create “big integers”
    /// which represent larger values.
    /// 
    /// # Counterpart Methods
    /// If you also need to add a carry to the wide result, then you want to use
    /// `carrying_mul()` instead.
    ///     
    /// The value of the first field in the returned tuple matches what you’d
    /// get the `wrapping_mul()` methods.
    /// `self.widening_mul(rhs).0` == `self.wrapping_mul(rhs)`
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// Even though it does not call the method `widening_mul()` of
    /// implementation of the primitive unsigned integer types such as `u8`,
    /// `u16`, `u32`, `u64`, `u128` and `usize` directly, all the description
    /// of this method is mainly the same as that of the method `widening_mul()`
    /// of implementation of the primitive unsigned integer types for nightly
    /// version except example codes. Confer to the descryptions that are linked
    /// to in the section _Reference_. This plagiarism is not made maliciously
    /// but is made for the reason of effectiveness and efficiency so that users
    /// may understand better and easily how to use this method with simiilarity
    /// to the method `widening_mul()` of implementation of the primitive
    /// unsigned integer types.
    /// 
    /// # Possiible Changes in Future
    /// This method does not call the method widening_mul() of the primitive
    /// unsigned integer types directly. Instead, it is implemented to perform
    /// the same thing as that of widening_mul() of the primitive unsigned
    /// integer types because the methods widening_mul() of the primitive
    /// unsigned integer types are only for nightly version. So, when the method
    /// widening_mul() of the primitive unsigned integer types will become a
    /// part of non-nightly normal version, the implementation of this method
    /// will be changed to call the method widening_mul() of the primitive
    /// unsigned integer types directly.
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    /// 
    /// # References
    /// - If you want to know about the definition of the method `widening_mul()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.widening_mul).
    /// - If you want to know about the definition of the method `widening_mul()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.widening_mul).
    /// - If you want to know about the definition of the method `widening_mul()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.widening_mul).
    /// - If you want to know about the definition of the method `widening_mul()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.widening_mul).
    /// - If you want to know about the definition of the method `widening_mul()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.widening_mul).
    /// - If you want to know about the definition of the method `widening_mul()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.widening_mul).
    fn widening_mul(self, rhs: Self) -> (Self, Self);

    // fn carrying_mul_for_internal_use(self, rhs: Self, carry: Self) -> (Self, Self);
    /// It is for internal use. You are recommended to use [carrying_mul()](trait@SmallUInt#tymethod.widening_mul) instead.
    fn widening_mul_for_internal_use(self, rhs: Self) -> (Self, Self);

    // fn wrapping_mul(self, rhs: Self) -> Self
    /// Computes self * rhs, wrapping around at the boundary of the type.
    /// 
    /// # Features
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
    /// It multiplies two numbers with wrapping (modular) multiplication.
    /// 
    /// # Output
    /// It returns the self * rhs in the type of `Self`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u8 = func(u8::MAX / 3, 2_u8);
    ///     println!("{} * 2 = {}", u8::MAX / 3, a_u8);
    ///     assert_eq!(a_u8, 170_u8);
    /// 
    ///     let b_u8 = func(a_u8, 2_u8);
    ///     println!("{} * 2 = {}", a_u8, b_u8);
    ///     assert_eq!(b_u8, 84_u8);
    /// 
    ///     let a_u16 = func(u16::MAX / 3, 2_u16);
    ///     println!("{} * 2 = {}", u16::MAX / 3, a_u16);
    ///     assert_eq!(a_u16, 43690_u16);
    /// 
    ///     let b_u16 = func(a_u16, 2_u16);
    ///     println!("{} * 2 = {}", a_u16, b_u16);
    ///     assert_eq!(b_u16, 21844_u16);
    /// 
    ///     let a_u32 = func(u32::MAX / 3, 2_u32);
    ///     println!("{} * 2 = {}", u32::MAX / 3, a_u32);
    ///     assert_eq!(a_u32, 2863311530_u32);
    /// 
    ///     let b_u32 = func(a_u32, 2_u32);
    ///     println!("{} * 2 = {}", a_u32, b_u32);
    ///     assert_eq!(b_u32, 1431655764_u32);
    /// 
    ///     let a_u64 = func(u64::MAX / 3, 2_u64);
    ///     println!("{} * 2 = {}", u64::MAX / 3, a_u64);
    ///     assert_eq!(a_u64, 12297829382473034410_u64);
    /// 
    ///     let b_u64 = func(a_u64, 2_u64);
    ///     println!("{} * 2 = {}", a_u64, b_u64);
    ///     assert_eq!(b_u64, 6148914691236517204_u64);
    /// 
    ///     let a_u128 = func(u128::MAX / 3, 2_u128);
    ///     println!("{} * 2 = {}", u128::MAX / 3, a_u128);
    ///     assert_eq!(a_u128,226854911280625642308916404954512140970_u128);
    /// 
    ///     let b_u128 = func(a_u128, 2_u128);
    ///     println!("{} * 2 = {}", a_u128, b_u128);
    ///     assert_eq!(b_u128, 113427455640312821154458202477256070484_u128);
    /// 
    ///     let a_usize = func(usize::MAX / 3, 2_usize);
    ///     println!("{} * 2 = {}", usize::MAX / 3, a_usize);
    ///     assert_eq!(a_usize, 12297829382473034410_usize);
    /// 
    ///     let b_usize = func(a_usize, 2_usize);
    ///     println!("{} * 2 = {}", a_usize, b_usize);
    ///     assert_eq!(b_usize, 6148914691236517204_usize);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_mul(rhs)
    /// }
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// It calls the method wrapping_mul() of implementation of the primitive
    /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
    /// `usize` directly. So, all the description of this method is mainly the
    /// same as that of the method wrapping_mul() of implementation of the
    /// primitive unsigned integer types except example codes. Confer to the
    /// descryptions that are linked to in the section _Reference_. This
    /// plagiarism is not made maliciously but is made for the reason of
    /// effectiveness and efficiency so that users may understand better and
    /// easily how to use this method with simiilarity to the method
    /// wrapping_mul() of implementation of the primitive unsigned integer types.
    /// 
    /// # References
    /// - If you want to know about the definition of the method `wrapping_mul()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.wrapping_mul).
    /// - If you want to know about the definition of the method `wrapping_mul()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.wrapping_mul).
    /// - If you want to know about the definition of the method `wrapping_mul()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.wrapping_mul).
    /// - If you want to know about the definition of the method `wrapping_mul()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.wrapping_mul).
    /// - If you want to know about the definition of the method `wrapping_mul()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.wrapping_mul).
    /// - If you want to know about the definition of the method `wrapping_mul()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.wrapping_mul).
    fn wrapping_mul(self, rhs: Self) -> Self;

    /// Calculates self * rhs, wrapping around at the boundary of the type.
    /// 
    /// # Features
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
    /// It multiplies two numbers with wrapping (modular) multiplication.
    /// 
    /// # Output
    /// It returns a tuple of the multiplication along with a boolean indicating
    /// whether an arithmetic overflow would occur. If an overflow would
    /// have occurred then the wrapped value is returned.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u8 = func(u8::MAX / 3, 2_u8);
    ///     println!("{} * 2 = {}\nOverflow = {}", u8::MAX / 3, a_u8.0, a_u8.1);
    ///     assert_eq!(a_u8.0, 170_u8);
    ///     assert_eq!(a_u8.1, false);
    /// 
    ///     let b_u8 = func(a_u8.0, 2_u8);
    ///     println!("{} * 2 = {}\nOverflow = {}", a_u8.0, b_u8.0, b_u8.1);
    ///     assert_eq!(b_u8.0, 84_u8);
    ///     assert_eq!(b_u8.1, true);
    /// 
    ///     let a_u16 = func(u16::MAX / 3, 2_u16);
    ///     println!("{} * 2 = {}\nOverflow = {}", u16::MAX / 3, a_u16.0, a_u16.1);
    ///     assert_eq!(a_u16.0, 43690_u16);
    ///     assert_eq!(a_u16.1, false);
    /// 
    ///     let b_u16 = func(a_u16.0, 2_u16);
    ///     println!("{} * 2 = {}\nOverflow = {}", a_u16.0, b_u16.0, b_u16.1);
    ///     assert_eq!(b_u16.0, 21844_u16);
    ///     assert_eq!(b_u16.1, true);
    /// 
    ///     let a_u32 = func(u32::MAX / 3, 2_u32);
    ///     println!("{} * 2 = {}\nOverflow = {}", u32::MAX / 3, a_u32.0, a_u32.1);
    ///     assert_eq!(a_u32.0, 2863311530_u32);
    ///     assert_eq!(a_u32.1, false);
    /// 
    ///     let b_u32 = func(a_u32.0, 2_u32);
    ///     println!("{} * 2 = {}\nOverflow = {}", a_u32.0, b_u32.0, b_u32.1);
    ///     assert_eq!(b_u32.0, 1431655764_u32);
    ///     assert_eq!(b_u32.1, true);
    /// 
    ///     let a_u64 = func(u64::MAX / 3, 2_u64);
    ///     println!("{} * 2 = {}\nOverflow = {}", u64::MAX / 3, a_u64.0, a_u64.1);
    ///     assert_eq!(a_u64.0, 12297829382473034410_u64);
    ///     assert_eq!(a_u64.1, false);
    /// 
    ///     let b_u64 = func(a_u64.0, 2_u64);
    ///     println!("{} * 2 = {}\nOverflow = {}", a_u64.0, b_u64.0, b_u64.1);
    ///     assert_eq!(b_u64.0, 6148914691236517204_u64);
    ///     assert_eq!(b_u64.1, true);
    /// 
    ///     let a_u128 = func(u128::MAX / 3, 2_u128);
    ///     println!("{} * 2 = {}\nOverflow = {}", u128::MAX / 3, a_u128.0, a_u128.1);
    ///     assert_eq!(a_u128.0, 226854911280625642308916404954512140970_u128);
    ///     assert_eq!(a_u128.1, false);
    /// 
    ///     let b_u128 = func(a_u128.0, 2_u128);
    ///     println!("{} * 2 = {}\nOverflow = {}", a_u128.0, b_u128.0, b_u128.1);
    ///     assert_eq!(b_u128.0, 113427455640312821154458202477256070484_u128);
    ///     assert_eq!(b_u128.1, true);
    /// 
    ///     let a_usize = func(usize::MAX / 3, 2_usize);
    ///     println!("{} * 2 = {}\nOverflow = {}", usize::MAX / 3, a_usize.0, a_usize.1);
    ///     assert_eq!(a_usize.0, 12297829382473034410_usize);
    ///     assert_eq!(a_usize.1, false);
    /// 
    ///     let b_usize = func(a_usize.0, 2_usize);
    ///     println!("{} * 2 = {}\nOverflow = {}", a_usize.0, b_usize.0, b_usize.1);
    ///     assert_eq!(b_usize.0, 6148914691236517204_usize);
    ///     assert_eq!(b_usize.1, true);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> (T, bool)
    /// {
    ///     lhs.overflowing_mul(rhs)
    /// }
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// It calls the method overflowing_mul() of implementation of the primitive
    /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
    /// `usize` directly. So, all the description of this method is mainly the
    /// same as that of the method overflowing_mul() of implementation of the
    /// primitive unsigned integer types except example codes. Confer to the
    /// descryptions that are linked to in the section _Reference_. This
    /// plagiarism is not made maliciously but is made for the reason of
    /// effectiveness and efficiency so that users may understand better and
    /// easily how to use this method with simiilarity to the method
    /// overflowing_mul() of implementation of the primitive unsigned integer
    /// types.
    /// 
    /// # References
    /// - If you want to know about the definition of the method `overflowing_mul()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.overflowing_mul).
    /// - If you want to know about the definition of the method `overflowing_mul()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.overflowing_mul).
    /// - If you want to know about the definition of the method `overflowing_mul()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.overflowing_mul).
    /// - If you want to know about the definition of the method `overflowing_mul()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.overflowing_mul).
    /// - If you want to know about the definition of the method `overflowing_mul()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.overflowing_mul).
    /// - If you want to know about the definition of the method `overflowing_mul()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.overflowing_mul).
    fn overflowing_mul(self, rhs: Self) -> (Self, bool);

    /// Computes self * rhs.
    /// 
    /// # Feature
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
    /// 
    /// # Output
    /// It returns self * rhs in the type `Self` wrapped by `Some`
    /// of enum `Option` if overflow did not occur.
    /// And, it returns `None` if overflow occurred.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u8 = func(u8::MAX / 3, 2_u8);
    ///     match a_u8
    ///     {
    ///         Some(a) => {
    ///                 println!("{} * 2 = {}", u8::MAX / 3, a_u8.unwrap());
    ///                 assert_eq!(a, 170_u8);
    ///             },
    ///         None => {
    ///                 println!("Overflow happened.");
    ///                 assert_eq!(a_u8, None);
    ///             },
    ///     }
    ///     
    ///     let b_u8 = func(a_u8.unwrap(), 2_u8);
    ///     match b_u8
    ///     {
    ///         Some(b) => {
    ///                 println!("{} * 2 = {}", a_u8.unwrap(), b_u8.unwrap());
    ///                 assert_eq!(b, 84_u8);
    ///             },
    ///         None => {
    ///                 println!("Overflow happened.");
    ///                 assert_eq!(b_u8, None);
    ///             },
    ///     }
    ///     
    ///     let a_u16 = func(u16::MAX / 3, 2_u16);
    ///     match a_u16
    ///     {
    ///         Some(a) => {
    ///                 println!("{} * 2 = {}", u16::MAX / 3, a_u16.unwrap());
    ///                 assert_eq!(a, 43690_u16);
    ///             },
    ///         None => {
    ///                 println!("Overflow happened.");
    ///                 assert_eq!(a_u16, None);
    ///             },
    ///     }
    ///     
    ///     let b_u16 = func(a_u16.unwrap(), 2_u16);
    ///     match b_u16
    ///     {
    ///         Some(b) => {
    ///                 println!("{} * 2 = {}", a_u16.unwrap(), b);
    ///                 assert_eq!(b, 21844_u16);
    ///             },
    ///         None => {
    ///                 println!("Overflow happened.");
    ///                 assert_eq!(b_u16, None);
    ///             },
    ///     }
    ///     
    ///     let a_u32 = func(u32::MAX / 3, 2_u32);
    ///     match a_u32
    ///     {
    ///         Some(a) => {
    ///                 println!("{} * 2 = {}", u32::MAX / 3, a_u32.unwrap());
    ///                 assert_eq!(a, 2863311530_u32);
    ///             },
    ///         None => {
    ///                 println!("Overflow happened.");
    ///                 assert_eq!(a_u32, None);
    ///             },
    ///     }
    ///     
    ///     let b_u32 = func(a_u32.unwrap(), 2_u32);
    ///     match b_u32
    ///     {
    ///         Some(b) => {
    ///                 println!("{} * 2 = {}", a_u32.unwrap(), b);
    ///                 assert_eq!(b, 1431655764_u32);
    ///             },
    ///         None => {
    ///                 println!("Overflow happened.");
    ///                 assert_eq!(b_u32, None);
    ///             },
    ///     }
    ///     
    ///     let a_u64 = func(u64::MAX / 3, 2_u64);
    ///     match a_u64
    ///     {
    ///         Some(a) => {
    ///                 println!("{} * 2 = {}", u64::MAX / 3, a_u64.unwrap());
    ///                 assert_eq!(a, 12297829382473034410_u64);
    ///             },
    ///         None => {
    ///                 println!("Overflow happened.");
    ///                 assert_eq!(a_u64, None);
    ///             },
    ///     }
    ///     
    ///     let b_u64 = func(a_u64.unwrap(), 2_u64);
    ///     match b_u64
    ///     {
    ///         Some(b) => {
    ///                 println!("{} * 2 = {}", a_u64.unwrap(), b);
    ///                 assert_eq!(b, 6148914691236517204_u64);
    ///             },
    ///         None => {
    ///                 println!("Overflow happened.");
    ///                 assert_eq!(b_u64, None);
    ///             },
    ///     }
    ///     
    ///     let a_u128 = func(u128::MAX / 3, 2_u128);
    ///     match a_u128
    ///     {
    ///         Some(a) => {
    ///                 println!("{} * 2 = {}", u128::MAX / 3, a_u128.unwrap());
    ///                 assert_eq!(a, 226854911280625642308916404954512140970_u128);
    ///             },
    ///         None => {
    ///                 println!("Overflow happened.");
    ///                 assert_eq!(a_u128, None);
    ///             },
    ///     }
    ///     
    ///     let b_u128 = func(a_u128.unwrap(), 2_u128);
    ///     match b_u128
    ///     {
    ///         Some(b) => {
    ///                 println!("{} * 2 = {}", a_u128.unwrap(), b);
    ///                 assert_eq!(b, 113427455640312821154458202477256070484_u128);
    ///             },
    ///         None => {
    ///                 println!("Overflow happened.");
    ///                 assert_eq!(b_u128, None);
    ///             },
    ///     }
    ///     
    ///     let a_usize = func(usize::MAX / 3, 2_usize);
    ///     match a_usize
    ///     {
    ///         Some(a) => {
    ///                 println!("{} * 2 = {}", usize::MAX / 3, a_usize.unwrap());
    ///                 assert_eq!(a, 12297829382473034410_usize);
    ///             },
    ///         None => {
    ///                 println!("Overflow happened.");
    ///                 assert_eq!(a_usize, None);
    ///             },
    ///     }
    ///     
    ///     let b_usize = func(a_usize.unwrap(), 2_usize);
    ///     match b_usize
    ///     {
    ///         Some(b) => {
    ///                 println!("{} * 2 = {}", a_usize.unwrap(), b);
    ///                 assert_eq!(b, 6148914691236517204_usize);
    ///             },
    ///         None => {
    ///                 println!("Overflow happened.");
    ///                 assert_eq!(b_usize, None);
    ///             },
    ///     }
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> Option<T>
    /// {
    ///     lhs.checked_mul(rhs)
    /// }
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// It calls the method checked_mul() of implementation of the primitive
    /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
    /// `usize` directly. So, all the description of this method is mainly the
    /// same as that of the method checked_mul() of implementation of the
    /// primitive unsigned integer types except example codes. Confer to the
    /// descryptions that are linked to in the section _Reference_. This
    /// plagiarism is not made maliciously but is made for the reason of
    /// effectiveness and efficiency so that users may understand better and
    /// easily how to use this method with simiilarity to the method
    /// checked_mul() of implementation of the primitive unsigned integer types.
    /// 
    /// # References
    /// - If you want to know about the definition of the method `checked_mul()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.checked_mul).
    /// - If you want to know about the definition of the method `checked_mul()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.checked_mul).
    /// - If you want to know about the definition of the method `checked_mul()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.checked_mul).
    /// - If you want to know about the definition of the method `checked_mul()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.checked_mul).
    /// - If you want to know about the definition of the method `checked_mul()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.checked_mul).
    /// - If you want to know about the definition of the method `checked_mul()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.checked_mul).
    fn checked_mul(self, rhs: Self) -> Option<Self>;

    /// Computes self + rhs, assuming overflow cannot occur.
    /// 
    /// # Features
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
    /// It is virtually same as self.checked_add(rhs).unwrap().
    /// Use this method only when it is sure that overflow will never happen.
    /// 
    /// # Panics
    /// If overflow occurs, this method will panic at this version.
    /// 
    /// # Output
    /// It returns self + rhs in the type `Self` if overflow did not occur.
    /// Otherwise, its behavior is not defined.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u8 = func(u8::MAX / 3, 2_u8);
    ///     println!("{} * 2 = {}", u8::MAX / 3, a_u8);
    ///     assert_eq!(a_u8, 170_u8);
    /// 
    ///     // It will panic
    ///     // let b_u8 = func(a_u8, 2_u8);
    /// 
    ///     let a_u16 = func(u16::MAX / 3, 2_u16);
    ///     println!("{} * 2 = {}", u16::MAX / 3, a_u16);
    ///     assert_eq!(a_u16, 43690_u16);
    /// 
    ///     // It will panic
    ///     // let b_u16 = func(a_u16, 2_u16);
    /// 
    ///     let a_u32 = func(u32::MAX / 3, 2_u32);
    ///     println!("{} * 2 = {}", u32::MAX / 3, a_u32);
    ///     assert_eq!(a_u32, 2863311530_u32);
    /// 
    ///     // It will panic
    ///     // let b_u32 = func(a_u32, 2_u32);
    /// 
    ///     let a_u64 = func(u64::MAX / 3, 2_u64);
    ///     println!("{} * 2 = {}", u64::MAX / 3, a_u64);
    ///     assert_eq!(a_u64, 12297829382473034410_u64);
    /// 
    ///     // It will panic
    ///     // let b_u64 = func(a_u64, 2_u64);
    /// 
    ///     let a_u128 = func(u128::MAX / 3, 2_u128);
    ///     println!("{} * 2 = {}", u128::MAX / 3, a_u128);
    ///     assert_eq!(a_u128, 226854911280625642308916404954512140970_u128);
    /// 
    ///     // It will panic
    ///     // let b_u128 = func(a_u128, 2_u128);
    /// 
    ///     let a_usize = func(usize::MAX / 3, 2_usize);
    ///     println!("{} * 2 = {}", usize::MAX / 3, a_usize);
    ///     assert_eq!(a_usize, 12297829382473034410_usize);
    /// 
    ///     // It will panic
    ///     // let b_usize = func(a_usize, 2_usize);
    /// }
    ///     
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.unchecked_mul(rhs)
    /// }
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// Even though it does not call the method unchecked_mul() of implementation
    /// of the primitive unsigned integer types such as `u8`, `u16`, `u32`,
    /// `u64`, `u128` and `usize` directly, all the description of this method
    /// is mainly the same as that of the method unchecked_mul() of
    /// implementation of the primitive unsigned integer types for nightly
    /// version except example codes. Confer to the descryptions that are linked
    /// to in the section _Reference_. This plagiarism is not made maliciously
    /// but is made for the reason of effectiveness and efficiency so that users
    /// may understand better and easily how to use this method with simiilarity
    /// to the method unchecked_mul() of implementation of the primitive unsigned
    /// integer types.
    /// 
    /// # Possiible Changes in Future
    /// This method does not call the method unchecked_mul() of the primitive
    /// unsigned integer types directly. Instead, it is implemented to perform
    /// the same thing as that of unchecked_mul() of the primitive unsigned
    /// integer types because the methods unchecked_mul() of the primitive
    /// unsigned integer types are only for nightly version. So, when the method
    /// unchecked_mul() of the primitive unsigned integer types will become a
    /// part of non-nightly normal version, the implementation of this method
    /// will be changed to call the method unchecked_mul() of the primitive
    /// unsigned integer types directly.
    /// 
    /// # References
    /// - If you want to know about the definition of the method `unchecked_mul()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.unchecked_mul).
    /// - If you want to know about the definition of the method `unchecked_mul()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.unchecked_mul).
    /// - If you want to know about the definition of the method `unchecked_mul()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.unchecked_mul).
    /// - If you want to know about the definition of the method `unchecked_mul()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.unchecked_mul).
    /// - If you want to know about the definition of the method `unchecked_mul()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.unchecked_mul).
    /// - If you want to know about the definition of the method `unchecked_mul()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.unchecked_mul).
    fn unchecked_mul(self, rhs: Self) -> Self;

    /// Computes self * rhs, saturating at the numeric bounds
    /// instead of overflowing.
    /// 
    /// # Features
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
    /// It multiplies two numbers with saturating integer multiplication
    /// 
    /// # Output
    /// It returns the smaller one of self * rhs and the maxium
    /// of the type of `Self`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u8 = func(u8::MAX / 3, 2_u8);
    ///     println!("{} * 2 = {}", u8::MAX / 3, a_u8);
    ///     assert_eq!(a_u8, 170_u8);
    /// 
    ///     let b_u8 = func(a_u8, 2_u8);
    ///     println!("{} * 2 = {}", a_u8, b_u8);
    ///     assert_eq!(b_u8, u8::MAX);
    /// 
    ///     let a_u16 = func(u16::MAX / 3, 2_u16);
    ///     println!("{} * 2 = {}", u16::MAX / 3, a_u16);
    ///     assert_eq!(a_u16, 43690_u16);
    /// 
    ///     let b_u16 = func(a_u16, 2_u16);
    ///     println!("{} * 2 = {}", a_u16, b_u16);
    ///     assert_eq!(b_u16, u16::MAX);
    /// 
    ///     let a_u32 = func(u32::MAX / 3, 2_u32);
    ///     println!("{} * 2 = {}", u32::MAX / 3, a_u32);
    ///     assert_eq!(a_u32, 2863311530_u32);
    /// 
    ///     let b_u32 = func(a_u32, 2_u32);
    ///     println!("{} * 2 = {}", a_u32, b_u32);
    ///     assert_eq!(b_u32, u32::MAX);
    /// 
    ///     let a_u64 = func(u64::MAX / 3, 2_u64);
    ///     println!("{} * 2 = {}", u64::MAX / 3, a_u64);
    ///     assert_eq!(a_u64, 12297829382473034410_u64);
    /// 
    ///     let b_u64 = func(a_u64, 2_u64);
    ///     println!("{} * 2 = {}", a_u64, b_u64);
    ///     assert_eq!(b_u64, u64::MAX);
    /// 
    ///     let a_u128 = func(u128::MAX / 3, 2_u128);
    ///     println!("{} * 2 = {}", u128::MAX / 3, a_u128);
    ///     assert_eq!(a_u128, 226854911280625642308916404954512140970_u128);
    /// 
    ///     let b_u128 = func(a_u128, 2_u128);
    ///     println!("{} * 2 = {}",a_u128, b_u128);
    ///     assert_eq!(b_u128, u128::MAX);
    /// 
    ///     let a_usize = func(usize::MAX / 3, 2_usize);
    ///     println!("{} * 2 = {}", usize::MAX / 3, a_usize);
    ///     assert_eq!(a_usize, 12297829382473034410_usize);
    /// 
    ///     let b_usize = func(a_usize, 2_usize);
    ///     println!("{} * 2 = {}", a_usize, b_usize);
    ///     assert_eq!(b_usize, usize::MAX);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.saturating_mul(rhs)
    /// }    
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// It calls the method saturating_mul() of implementation of the primitive
    /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
    /// `usize` directly. So, all the description of this method is mainly the
    /// same as that of the method saturating_mul() of implementation of the
    /// primitive unsigned integer types except example codes. Confer to the
    /// descryptions that are linked to in the section _Reference_. This
    /// plagiarism is not made maliciously but is made for the reason of
    /// effectiveness and efficiency so that users may understand better and
    /// easily how to use this method with simiilarity to the method
    /// saturating_mul() of implementation of the primitive unsigned integer types.
    /// 
    /// # References
    /// - If you want to know about the definition of the method `saturating_mul()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.saturating_mul).
    /// - If you want to know about the definition of the method `saturating_mul()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.saturating_mul).
    /// - If you want to know about the definition of the method `saturating_mul()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.saturating_mul).
    /// - If you want to know about the definition of the method `saturating_mul()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.saturating_mul).
    /// - If you want to know about the definition of the method `saturating_mul()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.saturating_mul).
    /// - If you want to know about the definition of the method `saturating_mul()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.saturating_mul).
    fn saturating_mul(self, rhs: Self) -> Self;


    // fn modular_mul(self, rhs: Self, modulo: Self) -> Self
    /// Computes (`self` * `rhs`) % `modulo`, wrapping around at `modulo`
    /// of the type `Self`.
    /// 
    /// # Feature
    /// Wrapping (modular) multiplication at `modulo`. The differences between
    /// this method `modular_mul_uint()` and the method `wrapping_mul_uint()`
    /// are, first, where wrapping around happens, and, second, whether or not
    /// `OVERFLOW` flag is set. First, this method wraps araound at `modulo`
    /// while the method `wrapping_mul_uint()` wraps araound at maximum value.
    /// Second, this method does not set `OVERFLOW` flag even if wrapping around
    /// happens, while the method `wrapping_mul_uint()` sets `OVERFLOW` flag
    /// when wrapping around happens.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method `modular_mul()` is proper
    /// rather than this method.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    fn modular_mul(self, rhs: Self, modulo: Self) -> Self;


    /***** DIVISION *****/

    /// Computes self / rhs.
    /// 
    /// # Features
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
    /// Wrapped division on unsigned types is just normal division. There’s no
    /// way wrapping could ever happen. This function exists, so that all
    /// operations are accounted for in the wrapping operations.
    /// 
    /// # Output
    /// It returns the self / rhs in the type of `Self`.
    /// 
    /// # Panics
    /// It will panic if rhs is zero.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u8 = func(u8::MAX / 3, 2_u8);
    ///     println!("{} / 2 = {}", u8::MAX / 3, a_u8);
    ///     assert_eq!(a_u8, 42_u8);
    /// 
    ///     let a_u16 = func(u16::MAX / 3, 2_u16);
    ///     println!("{} / 2 = {}", u16::MAX / 3, a_u16);
    ///     assert_eq!(a_u16, 10922_u16);
    /// 
    ///     let a_u32 = func(u32::MAX / 3, 2_u32);
    ///     println!("{} / 2 = {}", u32::MAX / 3, a_u32);
    ///     assert_eq!(a_u32, 715827882_u32);
    /// 
    ///     let a_u64 = func(u64::MAX / 3, 2_u64);
    ///     println!("{} / 2 = {}", u64::MAX / 3, a_u64);
    ///     assert_eq!(a_u64, 3074457345618258602_u64);
    /// 
    ///     let a_u128 = func(u128::MAX / 3, 2_u128);
    ///     println!("{} / 2 = {}", u128::MAX / 3, a_u128);
    ///     assert_eq!(a_u128, 56713727820156410577229101238628035242_u128);
    /// 
    ///     let a_usize = func(usize::MAX / 3, 2_usize);
    ///     println!("{} / 2 = {}", usize::MAX / 3, a_usize);
    ///     assert_eq!(a_usize, 3074457345618258602_usize);
    /// 
    ///     // It will panic.
    ///     // let a_panic = func(usize::MAX / 3, 0_usize);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_div(rhs)
    /// }
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// It calls the method wrapping_div() of implementation of the primitive
    /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
    /// `usize` directly. So, all the description of this method is mainly the
    /// same as that of the method wrapping_div() of implementation of the
    /// primitive unsigned integer types except example codes. Confer to the
    /// descryptions that are linked to in the section _Reference_. This
    /// plagiarism is not made maliciously but is made for the reason of
    /// effectiveness and efficiency so that users may understand better and
    /// easily how to use this method with simiilarity to the method
    /// wrapping_div() of implementation of the primitive unsigned integer types.
    /// 
    /// # References
    /// - If you want to know about the definition of the method `wrapping_div()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.wrapping_div).
    /// - If you want to know about the definition of the method `wrapping_div()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.wrapping_div).
    /// - If you want to know about the definition of the method `wrapping_div()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.wrapping_div).
    /// - If you want to know about the definition of the method `wrapping_div()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.wrapping_div).
    /// - If you want to know about the definition of the method `wrapping_div()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.wrapping_div).
    /// - If you want to know about the definition of the method `wrapping_div()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.wrapping_div).
    fn wrapping_div(self, rhs: Self) -> Self;

    /// Calculates self / rhs.
    /// 
    /// # Features
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
    /// It divides self by rhs.
    /// 
    /// # Output
    /// It returns a tuple of the quotient along with a boolean indicating
    /// whether an arithmetic overflow would occur. Note that for unsigned
    /// integers overflow never occurs, so the second value is always false.
    /// 
    /// # Panics
    /// It will panic if rhs is zero.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u8 = func(u8::MAX / 3, 2_u8);
    ///     println!("{} * 2 = {}\nOverflow = {}", u8::MAX / 3, a_u8.0, a_u8.1);
    ///     assert_eq!(a_u8.0, 170_u8);
    ///     assert_eq!(a_u8.1, false);
    /// 
    ///     let b_u8 = func(a_u8.0, 2_u8);
    ///     println!("{} * 2 = {}\nOverflow = {}", a_u8.0, b_u8.0, b_u8.1);
    ///     assert_eq!(b_u8.0, 84_u8);
    ///     assert_eq!(b_u8.1, true);
    /// 
    ///     let a_u16 = func(u16::MAX / 3, 2_u16);
    ///     println!("{} * 2 = {}\nOverflow = {}", u16::MAX / 3, a_u16.0, a_u16.1);
    ///     assert_eq!(a_u16.0, 43690_u16);
    ///     assert_eq!(a_u16.1, false);
    /// 
    ///     let b_u16 = func(a_u16.0, 2_u16);
    ///     println!("{} * 2 = {}\nOverflow = {}", a_u16.0, b_u16.0, b_u16.1);
    ///     assert_eq!(b_u16.0, 21844_u16);
    ///     assert_eq!(b_u16.1, true);
    /// 
    ///     let a_u32 = func(u32::MAX / 3, 2_u32);
    ///     println!("{} * 2 = {}\nOverflow = {}", u32::MAX / 3, a_u32.0, a_u32.1);
    ///     assert_eq!(a_u32.0, 2863311530_u32);
    ///     assert_eq!(a_u32.1, false);
    /// 
    ///     let b_u32 = func(a_u32.0, 2_u32);
    ///     println!("{} * 2 = {}\nOverflow = {}", a_u32.0, b_u32.0, b_u32.1);
    ///     assert_eq!(b_u32.0, 1431655764_u32);
    ///     assert_eq!(b_u32.1, true);
    /// 
    ///     let a_u64 = func(u64::MAX / 3, 2_u64);
    ///     println!("{} * 2 = {}\nOverflow = {}", u64::MAX / 3, a_u64.0, a_u64.1);
    ///     assert_eq!(a_u64.0, 12297829382473034410_u64);
    ///     assert_eq!(a_u64.1, false);
    /// 
    ///     let b_u64 = func(a_u64.0, 2_u64);
    ///     println!("{} * 2 = {}\nOverflow = {}", a_u64.0, b_u64.0, b_u64.1);
    ///     assert_eq!(b_u64.0, 6148914691236517204_u64);
    ///     assert_eq!(b_u64.1, true);
    /// 
    ///     let a_u128 = func(u128::MAX / 3, 2_u128);
    ///     println!("{} * 2 = {}\nOverflow = {}", u128::MAX / 3, a_u128.0, a_u128.1);
    ///     assert_eq!(a_u128.0, 226854911280625642308916404954512140970_u128);
    ///     assert_eq!(a_u128.1, false);
    /// 
    ///     let b_u128 = func(a_u128.0, 2_u128);
    ///     println!("{} * 2 = {}\nOverflow = {}", a_u128.0, b_u128.0, b_u128.1);
    ///     assert_eq!(b_u128.0, 113427455640312821154458202477256070484_u128);
    ///     assert_eq!(b_u128.1, true);
    /// 
    ///     let a_usize = func(usize::MAX / 3, 2_usize);
    ///     println!("{} * 2 = {}\nOverflow = {}", usize::MAX / 3, a_usize.0, a_usize.1);
    ///     assert_eq!(a_usize.0, 12297829382473034410_usize);
    ///     assert_eq!(a_usize.1, false);
    /// 
    ///     let b_usize = func(a_usize.0, 2_usize);
    ///     println!("{} * 2 = {}\nOverflow = {}", a_usize.0, b_usize.0, b_usize.1);
    ///     assert_eq!(b_usize.0, 6148914691236517204_usize);
    ///     assert_eq!(b_usize.1, true);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> (T, bool)
    /// {
    ///     lhs.overflowing_div(rhs)
    /// }
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// It calls the method overflowing_div() of implementation of the primitive
    /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
    /// `usize` directly. So, all the description of this method is mainly the
    /// same as that of the method overflowing_div() of implementation of the
    /// primitive unsigned integer types except example codes. Confer to the
    /// descryptions that are linked to in the section _Reference_. This
    /// plagiarism is not made maliciously but is made for the reason of
    /// effectiveness and efficiency so that users may understand better and
    /// easily how to use this method with simiilarity to the method
    /// overflowing_div() of implementation of the primitive unsigned integer
    /// types.
    /// 
    /// # References
    /// - If you want to know about the definition of the method `overflowing_div()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.overflowing_div).
    /// - If you want to know about the definition of the method `overflowing_div()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.overflowing_div).
    /// - If you want to know about the definition of the method `overflowing_div()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.overflowing_div).
    /// - If you want to know about the definition of the method `overflowing_div()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.overflowing_div).
    /// - If you want to know about the definition of the method `overflowing_div()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.overflowing_div).
    /// - If you want to know about the definition of the method `overflowing_div()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.overflowing_div).
    fn overflowing_div(self, rhs: Self) -> (Self, bool);

    /// Computes self / rhs.
    /// 
    /// # Feature
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
    /// 
    /// # Output
    /// It returns self / rhs in the type `Self` wrapped by `Some`
    /// of enum `Option`. And, it returns `None` if rhs is zero.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u8 = UInt_checked_mul___func(u8::MAX / 3, 2_u8);
    ///     match a_u8
    ///     {
    ///         Some(a) => {
    ///                 println!("{} / 2 = {}", u8::MAX / 3, a);
    ///                 assert_eq!(a, 42_u8);
    ///             },
    ///         None => {
    ///                 println!("Divided by zero.");
    ///                 assert_eq!(a_u8, None);
    ///             },
    ///     }
    /// 
    ///     let b_u8 = UInt_checked_mul___func(u8::MAX / 3, 0_u8);
    ///     match b_u8
    ///     {
    ///         Some(b) => { println!("{} / 2 = {}", u8::MAX / 3, b); },
    ///         None => {
    ///                 println!("Divided by zero.");
    ///                 assert_eq!(b_u8, None);
    ///             },
    ///     }
    /// 
    ///     let a_u16 = UInt_checked_mul___func(u16::MAX / 3, 2_u16);
    ///     match a_u16
    ///     {
    ///         Some(a) => {
    ///                 println!("{} / 2 = {}", u16::MAX / 3, a);
    ///                 assert_eq!(a, 10922_u16);
    ///             },
    ///         None => {
    ///                 println!("Divided by zero.");
    ///                 assert_eq!(a_u16, None);
    ///             },
    ///     }
    /// 
    ///     let b_u16 = UInt_checked_mul___func(u16::MAX / 3, 0_u16);
    ///     match b_u16
    ///     {
    ///         Some(b) => { println!("{} / 2 = {}", u16::MAX / 3, b); },
    ///         None => {
    ///                 println!("Divided by zero.");
    ///                 assert_eq!(b_u16, None);
    ///             },
    ///     }
    /// 
    ///     let a_u32 = UInt_checked_mul___func(u32::MAX / 3, 2_u32);
    ///     match a_u32
    ///     {
    ///         Some(a) => {
    ///                 println!("{} / 2 = {}", u32::MAX / 3, a);
    ///                 assert_eq!(a, 715827882_u32);
    ///             },
    ///         None => {
    ///                 println!("Divided by zero.");
    ///                 assert_eq!(a_u32, None);
    ///             },
    ///     }
    /// 
    ///     let b_u32 = UInt_checked_mul___func(u32::MAX / 3, 0_u32);
    ///     match b_u32
    ///     {
    ///         Some(b) => { println!("{} / 2 = {}", u32::MAX / 3, b); },
    ///         None => {
    ///                 println!("Divided by zero.");
    ///                 assert_eq!(b_u32, None);
    ///             },
    ///     }
    /// 
    ///     let a_u64 = UInt_checked_mul___func(u64::MAX / 3, 2_u64);
    ///     match a_u64
    ///     {
    ///         Some(a) => {
    ///                 println!("{} / 2 = {}", u64::MAX / 3, a);
    ///                 assert_eq!(a, 3074457345618258602_u64);
    ///             },
    ///         None => {
    ///                 println!("Divided by zero.");
    ///                 assert_eq!(a_u64, None);
    ///             },
    ///     }
    /// 
    ///     let b_u64 = UInt_checked_mul___func(u64::MAX / 3, 0_u64);
    ///     match b_u64
    ///     {
    ///         Some(b) => { println!("{} / 2 = {}", u64::MAX / 3, b); },
    ///         None => {
    ///                 println!("Divided by zero.");
    ///                 assert_eq!(b_u64, None);
    ///             },
    ///     }
    /// 
    ///     let a_u128 = UInt_checked_mul___func(u128::MAX / 3, 2_u128);
    ///     match a_u128
    ///     {
    ///         Some(a) => {
    ///                 println!("{} / 2 = {}", u128::MAX / 3, a);
    ///                 assert_eq!(a, 56713727820156410577229101238628035242_u128);
    ///             },
    ///         None => {
    ///                 println!("Divided by zero.");
    ///                 assert_eq!(a_u128, None);
    ///             },
    ///     }
    /// 
    ///     let b_u128 = UInt_checked_mul___func(u128::MAX / 3, 0_u128);
    ///     match b_u128
    ///     {
    ///         Some(b) => { println!("{} / 2 = {}", u128::MAX / 3, b); },
    ///         None => {
    ///                 println!("Divided by zero.");
    ///                 assert_eq!(b_u128, None);
    ///             },
    ///     }
    /// 
    ///     let a_usize = UInt_checked_mul___func(usize::MAX / 3, 2_usize);
    ///     match a_usize
    ///     {
    ///         Some(a) => {
    ///                 println!("{} / 2 = {}", usize::MAX / 3, a);
    ///                 assert_eq!(a, 3074457345618258602_usize);
    ///             },
    ///         None => {
    ///                 println!("Divided by zero.");
    ///                 assert_eq!(a_usize, None);
    ///             },
    ///     }
    /// 
    ///     let b_usize = UInt_checked_mul___func(usize::MAX / 3, 0_usize);
    ///     match b_usize
    ///     {
    ///         Some(b) => { println!("{} / 2 = {}", usize::MAX / 3, b); },
    ///         None => {
    ///                 println!("Divided by zero.");
    ///                 assert_eq!(b_usize, None);
    ///             },
    ///     }
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> Option<T>
    /// {
    ///     lhs.checked_div(rhs)
    /// }
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// It calls the method checked_div() of implementation of the primitive
    /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
    /// `usize` directly. So, all the description of this method is mainly the
    /// same as that of the method checked_div() of implementation of the
    /// primitive unsigned integer types except example codes. Confer to the
    /// descryptions that are linked to in the section _Reference_. This
    /// plagiarism is not made maliciously but is made for the reason of
    /// effectiveness and efficiency so that users may understand better and
    /// easily how to use this method with simiilarity to the method
    /// checked_div() of implementation of the primitive unsigned integer types.
    /// 
    /// # References
    /// - If you want to know about the definition of the method `checked_div()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.checked_div).
    /// - If you want to know about the definition of the method `checked_div()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.checked_div).
    /// - If you want to know about the definition of the method `checked_div()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.checked_div).
    /// - If you want to know about the definition of the method `checked_div()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.checked_div).
    /// - If you want to know about the definition of the method `checked_div()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.checked_div).
    /// - If you want to know about the definition of the method `checked_div()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.checked_div).
    fn checked_div(self, rhs: Self) -> Option<Self>;

    /// Computes self / rhs.
    /// 
    /// # Features
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
    /// 
    /// # Output
    /// It returns self / rhs in the type `Self`.
    /// 
    /// # Panics
    /// It will panic if rhs is zero.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u8 = func(u8::MAX / 3, 2_u8);
    ///     println!("{} / 2 = {}", u8::MAX / 3, a_u8);
    ///     assert_eq!(a_u8, 42_u8);
    ///     
    ///     let a_u16 = func(u16::MAX / 3, 2_u16);
    ///     println!("{} / 2 = {}", u16::MAX / 3, a_u16);
    ///     assert_eq!(a_u16, 10922_u16);
    ///     
    ///     let a_u32 = func(u32::MAX / 3, 2_u32);
    ///     println!("{} / 2 = {}", u32::MAX / 3, a_u32);
    ///     assert_eq!(a_u32, 715827882_u32);
    ///     
    ///     let a_u64 = func(u64::MAX / 3, 2_u64);
    ///     println!("{} / 2 = {}", u64::MAX / 3, a_u64);
    ///     assert_eq!(a_u64, 3074457345618258602_u64);
    ///     
    ///     let a_u128 = func(u128::MAX / 3, 2_u128);
    ///     println!("{} / 2 = {}", u128::MAX / 3, a_u128);
    ///     assert_eq!(a_u128, 56713727820156410577229101238628035242_u128);
    ///     
    ///     let a_usize = func(usize::MAX / 3, 2_usize);
    ///     println!("{} / 2 = {}", usize::MAX / 3, a_usize);
    ///     assert_eq!(a_usize, 3074457345618258602_usize);
    ///     
    ///     // It will panic.
    ///     // let a_panic = UInt_saturating_div___func(usize::MAX / 3, 0_usize);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.saturating_div(rhs)
    /// }    
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// It calls the method saturating_div() of implementation of the primitive
    /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
    /// `usize` directly. So, all the description of this method is mainly the
    /// same as that of the method saturating_div() of implementation of the
    /// primitive unsigned integer types except example codes. Confer to the
    /// descryptions that are linked to in the section _Reference_. This
    /// plagiarism is not made maliciously but is made for the reason of
    /// effectiveness and efficiency so that users may understand better and
    /// easily how to use this method with simiilarity to the method
    /// saturating_div() of implementation of the primitive unsigned integer types.
    /// 
    /// # References
    /// - If you want to know about the definition of the method `saturating_div()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.saturating_div).
    /// - If you want to know about the definition of the method `saturating_div()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.saturating_div).
    /// - If you want to know about the definition of the method `saturating_div()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.saturating_div).
    /// - If you want to know about the definition of the method `saturating_div()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.saturating_div).
    /// - If you want to know about the definition of the method `saturating_div()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.saturating_div).
    /// - If you want to know about the definition of the method `saturating_div()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.saturating_div).
    fn saturating_div(self, rhs: Self) -> Self;


    /***** MODULO *****/

    /// Computes self % rhs.
    /// 
    /// # Features
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
    /// Wrapped remainder calculation on unsigned types is just the regular
    /// remainder calculation. There’s no way wrapping could ever happen.
    /// This function exists, so that all operations are accounted for in the
    /// wrapping operations.
    /// 
    /// # Output
    /// It returns the self % rhs in the type of `Self`.
    /// 
    /// # Panics
    /// It will panic if rhs is zero.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u8 = func(u8::MAX / 3, 3_u8);
    ///     println!("{} % 3 = {}", u8::MAX / 3, a_u8);
    ///     assert_eq!(a_u8, 1_u8);
    /// 
    ///     let a_u16 = func(u16::MAX / 3, 3_u16);
    ///     println!("{} % 3 = {}", u16::MAX / 3, a_u16);
    ///     assert_eq!(a_u16, 2_u16);
    /// 
    ///     let a_u32 = func(u32::MAX / 3, 3_u32);
    ///     println!("{} % 3 = {}", u32::MAX / 3, a_u32);
    ///     assert_eq!(a_u32, 1_u32);
    /// 
    ///     let a_u64 = func(u64::MAX / 3, 3_u64);
    ///     println!("{} % 3 = {}", u64::MAX / 3, a_u64);
    ///     assert_eq!(a_u64, 2_u64);
    /// 
    ///     let a_u128 = func(u128::MAX / 3, 3_u128);
    ///     println!("{} % 3 = {}", u128::MAX / 3, a_u128);
    ///     assert_eq!(a_u128, 1_u128);
    ///
    ///     let a_usize = func(usize::MAX / 3, 3_usize);
    ///     println!("{} % 3 = {}", usize::MAX / 3, a_usize);
    ///     assert_eq!(a_usize, 2_usize);
    /// 
    ///     // It will panic.
    ///     // let a_panic = func(usize::MAX / 3, 0_usize);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.wrapping_rem(rhs)
    /// }
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// It calls the method wrapping_rem() of implementation of the primitive
    /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
    /// `usize` directly. So, all the description of this method is mainly the
    /// same as that of the method wrapping_rem() of implementation of the
    /// primitive unsigned integer types except example codes. Confer to the
    /// descryptions that are linked to in the section _Reference_. This
    /// plagiarism is not made maliciously but is made for the reason of
    /// effectiveness and efficiency so that users may understand better and
    /// easily how to use this method with simiilarity to the method
    /// wrapping_rem() of implementation of the primitive unsigned integer types.
    /// 
    /// # References
    /// - If you want to know about the definition of the method `wrapping_rem()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.wrapping_rem).
    /// - If you want to know about the definition of the method `wrapping_rem()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.wrapping_rem).
    /// - If you want to know about the definition of the method `wrapping_rem()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.wrapping_rem).
    /// - If you want to know about the definition of the method `wrapping_rem()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.wrapping_rem).
    /// - If you want to know about the definition of the method `wrapping_rem()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.wrapping_rem).
    /// - If you want to know about the definition of the method `wrapping_rem()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.wrapping_rem).
    fn wrapping_rem(self, rhs: Self) -> Self;

    /// Calculates self % rhs.
    /// 
    /// # Features
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
    /// It calculates the remainder when self is divided by rhs.
    /// 
    /// # Output
    /// It returns a tuple of the remainder along with a boolean indicating
    /// whether an arithmetic overflow would occur. Note that for unsigned
    /// integers overflow never occurs, so the second value is always false.
    /// 
    /// # Panics
    /// It will panic if rhs is zero.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u8 = func(u8::MAX / 3, 3_u8);
    ///     println!("{} % 3 = {}\nOverflow = {}", u8::MAX / 3, a_u8.0, a_u8.1);
    ///     assert_eq!(a_u8.0, 1_u8);
    ///     assert_eq!(a_u8.1, false);
    ///     
    ///     let a_u16 = func(u16::MAX / 3, 3_u16);
    ///     println!("{} % 3 = {}\nOverflow = {}", u16::MAX / 3, a_u16.0, a_u16.1);
    ///     assert_eq!(a_u16.0, 2_u16);
    ///     assert_eq!(a_u16.1, false);
    ///     
    ///     let a_u32 = func(u32::MAX / 3, 3_u32);
    ///     println!("{} % 3 = {}\nOverflow = {}", u32::MAX / 3, a_u32.0, a_u32.1);
    ///     assert_eq!(a_u32.0, 1_u32);
    ///     assert_eq!(a_u32.1, false);
    ///     
    ///     let a_u64 = func(u64::MAX / 3, 3_u64);
    ///     println!("{} % 3 = {}\nOverflow = {}", u64::MAX / 3, a_u64.0, a_u64.1);
    ///     assert_eq!(a_u64.0, 2_u64);
    ///     assert_eq!(a_u64.1, false);
    ///     
    ///     let a_u128 = func(u128::MAX / 3, 3_u128);
    ///     println!("{} % 3 = {}\nOverflow = {}", u128::MAX / 3, a_u128.0, a_u128.1);
    ///     assert_eq!(a_u128.0, 1_u128);
    ///     assert_eq!(a_u128.1, false);
    ///     
    ///     let a_usize = func(usize::MAX / 3, 3_usize);
    ///     println!("{} % 3 = {}\nOverflow = {}", usize::MAX / 3, a_usize.0, a_usize.1);
    ///     assert_eq!(a_usize.0, 2_usize);
    ///     assert_eq!(a_usize.1, false);
    ///     
    ///     // It will panic.
    ///     // let a_panic = func(a_usize.0, 0_usize);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> (T, bool)
    /// {
    ///     lhs.overflowing_rem(rhs)
    /// }
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// It calls the method overflowing_rem() of implementation of the primitive
    /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
    /// `usize` directly. So, all the description of this method is mainly the
    /// same as that of the method overflowing_rem() of implementation of the
    /// primitive unsigned integer types except example codes. Confer to the
    /// descryptions that are linked to in the section _Reference_. This
    /// plagiarism is not made maliciously but is made for the reason of
    /// effectiveness and efficiency so that users may understand better and
    /// easily how to use this method with simiilarity to the method
    /// overflowing_rem() of implementation of the primitive unsigned integer
    /// types.
    /// 
    /// # References
    /// - If you want to know about the definition of the method `overflowing_rem()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.overflowing_rem).
    /// - If you want to know about the definition of the method `overflowing_rem()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.overflowing_rem).
    /// - If you want to know about the definition of the method `overflowing_rem()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.overflowing_rem).
    /// - If you want to know about the definition of the method `overflowing_rem()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.overflowing_rem).
    /// - If you want to know about the definition of the method `overflowing_rem()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.overflowing_rem).
    /// - If you want to know about the definition of the method `overflowing_rem()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.overflowing_rem).
    fn overflowing_rem(self, rhs: Self) -> (Self, bool);

    /// Computes self % rhs.
    /// 
    /// # Feature
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
    /// 
    /// # Output
    /// It returns self % rhs in the type `Self` wrapped by `Some`
    /// of enum `Option`. And, it returns `None` if rhs is zero.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u8 = func(u8::MAX / 3, 3_u8);
    ///     match a_u8
    ///     {
    ///         Some(a) => {
    ///                 println!("{} % 3 = {}", u8::MAX / 3, a);
    ///                 assert_eq!(a, 1_u8);
    ///             },
    ///         None => {
    ///                 println!("Divided by zero.");
    ///                 assert_eq!(a_u8, None);
    ///             },
    ///     }
    /// 
    ///     let b_u8 = func(u8::MAX / 3, 0_u8);
    ///     match b_u8
    ///     {
    ///         Some(b) => { println!("{} % 3 = {}", u8::MAX / 3, b); },
    ///         None => {
    ///                 println!("Divided by zero.");
    ///                 assert_eq!(b_u8, None);
    ///             },
    ///     }
    /// 
    ///     let a_u16 = func(u16::MAX / 3, 3_u16);
    ///     match a_u16
    ///     {
    ///         Some(a) => {
    ///                 println!("{} % 3 = {}", u16::MAX / 3, a);
    ///                 assert_eq!(a, 2_u16);
    ///             },
    ///         None => {
    ///                 println!("Divided by zero.");
    ///                 assert_eq!(a_u16, None);
    ///             },
    ///     }
    /// 
    ///     let b_u16 = func(u16::MAX / 3, 0_u16);
    ///     match b_u16
    ///     {
    ///         Some(b) => { println!("{} % 3 = {}", u16::MAX / 3, b); },
    ///         None => {
    ///                 println!("Divided by zero.");
    ///                 assert_eq!(b_u16, None);
    ///             },
    ///     }
    /// 
    ///     let a_u32 = func(u32::MAX / 3, 3_u32);
    ///     match a_u32
    ///     {
    ///         Some(a) => {
    ///                 println!("{} % 3 = {}", u32::MAX / 3, a);
    ///                 assert_eq!(a, 1_u32);
    ///             },
    ///         None => {
    ///                 println!("Divided by zero.");
    ///                 assert_eq!(a_u32, None);
    ///             },
    ///     }
    /// 
    ///     let b_u32 = func(u32::MAX / 3, 0_u32);
    ///     match b_u32
    ///     {
    ///         Some(b) => { println!("{} % 3 = {}", u32::MAX / 3, b); },
    ///         None => {
    ///                 println!("Divided by zero.");
    ///                 assert_eq!(b_u32, None);
    ///             },
    ///     }
    /// 
    ///     let a_u64 = func(u64::MAX / 3, 3_u64);
    ///     match a_u64
    ///     {
    ///         Some(a) => {
    ///                 println!("{} % 3 = {}", u64::MAX / 3, a);
    ///                 assert_eq!(a, 2_u64);
    ///             },
    ///         None => {
    ///                 println!("Divided by zero.");
    ///                 assert_eq!(a_u64, None);
    ///             },
    ///     }
    /// 
    ///     let b_u64 = func(u64::MAX / 3, 0_u64);
    ///     match b_u64
    ///     {
    ///         Some(b) => { println!("{} % 3 = {}", u64::MAX / 3, b); },
    ///         None => {
    ///                 println!("Divided by zero.");
    ///                 assert_eq!(b_u64, None);
    ///             },
    ///     }
    /// 
    ///     let a_u128 = func(u128::MAX / 3, 3_u128);
    ///     match a_u128
    ///     {
    ///         Some(a) => {
    ///                 println!("{} % 3 = {}", u128::MAX / 3, a);
    ///                 assert_eq!(a, 1_u128);
    ///             },
    ///         None => {
    ///                 println!("Divided by zero.");
    ///                 assert_eq!(a_u128, None);
    ///             },
    ///     }
    /// 
    ///     let b_u128 = func(u128::MAX / 3, 0_u128);
    ///     match b_u128
    ///     {
    ///         Some(b) => { println!("{} % 3 = {}", u128::MAX / 3, b); },
    ///         None => {
    ///                 println!("Divided by zero.");
    ///                 assert_eq!(b_u128, None);
    ///             },
    ///     }
    /// 
    ///     let a_usize = func(usize::MAX / 3, 3_usize);
    ///     match a_usize
    ///     {
    ///         Some(a) => {
    ///                 println!("{} % 3 = {}", usize::MAX / 3, a);
    ///                 assert_eq!(a, 2_usize);
    ///             },
    ///         None => {
    ///                 println!("Divided by zero.");
    ///                 assert_eq!(a_usize, None);
    ///             },
    ///     }
    /// 
    ///     let b_usize = func(usize::MAX / 3, 0_usize);
    ///     match b_usize
    ///     {
    ///         Some(b) => { println!("{} % 3 = {}", usize::MAX / 3, b); },
    ///         None => {
    ///                 println!("Divided by zero.");
    ///                 assert_eq!(b_usize, None);
    ///             },
    ///     }
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> Option<T>
    /// {
    ///     lhs.checked_rem(rhs)
    /// }
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// It calls the method checked_rem() of implementation of the primitive
    /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
    /// `usize` directly. So, all the description of this method is mainly the
    /// same as that of the method checked_rem() of implementation of the
    /// primitive unsigned integer types except example codes. Confer to the
    /// descryptions that are linked to in the section _Reference_. This
    /// plagiarism is not made maliciously but is made for the reason of
    /// effectiveness and efficiency so that users may understand better and
    /// easily how to use this method with simiilarity to the method
    /// checked_rem() of implementation of the primitive unsigned integer types.
    /// 
    /// # References
    /// - If you want to know about the definition of the method `checked_rem()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.checked_rem).
    /// - If you want to know about the definition of the method `checked_rem()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.checked_rem).
    /// - If you want to know about the definition of the method `checked_rem()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.checked_rem).
    /// - If you want to know about the definition of the method `checked_rem()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.checked_rem).
    /// - If you want to know about the definition of the method `checked_rem()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.checked_rem).
    /// - If you want to know about the definition of the method `checked_rem()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.checked_rem).
    fn checked_rem(self, rhs: Self) -> Option<Self>;


    /*** Power ***/

    // fn pow(self, exp: u32) -> Self;
    /// Raises `self` to the power of `exp`, using exponentiation by squaring.
    /// 
    /// # Features
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
    /// 
    /// # Panics
    /// It will panic if the result of this method is more than
    /// the possible maximum value.
    /// 
    /// # Output
    /// It returns the self raised to the power of exp, in the type of `Self`.
    /// 
    /// # Example for u8
    /// ```
    /// use Cryptocol::number::*;
    /// fn main()
    /// {
    ///     let a_u8 = func(3_u8, 5_u32);
    ///     println!("3 ** 5 = {}", a_u8);
    ///     assert_eq!(a_u8, 243_u8);
    ///     // It will panic.
    ///     // println!("3 ** 5 = {}", func(3_u8, 6_u32));
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: u32) -> T
    /// {
    ///     lhs.pow(rhs)
    /// }
    /// ```
    /// 
    /// # Example for u16
    /// ```
    /// use Cryptocol::number::*;
    /// fn main()
    /// {
    ///     let a_u16 = func(9_u16, 5_u32);
    ///     println!("9 ** 5 = {}", a_u16);
    ///     assert_eq!(a_u16, 59049_u16);
    ///     // It will panic.
    ///     // println!("9 ** 5 = {}", func(9_u16, 6_u32));
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: u32) -> T
    /// {
    ///     lhs.pow(rhs)
    /// }
    /// ```
    /// 
    /// # Example for u32
    /// ```
    /// use Cryptocol::number::*;
    /// fn main()
    /// {
    ///     let a_u32 = func(81_u32, 5_u32);
    ///     println!("81 ** 5 = {}", a_u32);
    ///     assert_eq!(a_u32, 3486784401_u32);
    ///     // It will panic.
    ///     // println!("81 ** 6 = {}", func(81_u32, 6_u32));
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: u32) -> T
    /// {
    ///     lhs.pow(rhs)
    /// }
    /// ```
    /// 
    /// # Example for u64
    /// ```
    /// use Cryptocol::number::*;
    /// fn main()
    /// {
    ///     let a_u64 = func(6561_u64, 5_u32);
    ///     println!("6561 ** 5 = {}", a_u64);
    ///     assert_eq!(a_u64, 12157665459056928801_u64);
    ///     // It will panic.
    ///     // println!("6561 ** 6 = {}", func(6561_u64, 6_u32));
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: u32) -> T
    /// {
    ///     lhs.pow(rhs)
    /// }
    /// ```
    /// 
    /// # Example for u128
    /// ```
    /// use Cryptocol::number::*;
    /// fn main()
    /// {
    ///     let a_u128 = func(43046721_u128, 5_u32);
    ///     println!("43046721 ** 5 = {}", a_u128);
    ///     assert_eq!(a_u128, 147808829414345923316083210206383297601_u128);
    ///     // It will panic.
    ///     // println!("43046721 ** 6 = {}", func(43046721_u64, 6_u32));
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: u32) -> T
    /// {
    ///     lhs.pow(rhs)
    /// }
    /// ```
    /// 
    /// # Example for usize
    /// ```
    /// use Cryptocol::number::*;
    /// fn main()
    /// {
    ///     let a_usize = func(6561_usize, 5_u32);
    ///     println!("6561 ** 5 = {}", a_usize);
    ///     assert_eq!(a_usize, 12157665459056928801_usize);
    ///     // It will panic.
    ///     // println!("6561 ** 6 = {}", func(6561_usize, 6_u32));
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: u32) -> T
    /// {
    ///     lhs.pow(rhs)
    /// }
    /// ```
    /// 
    /// # Example for ShortUnion
    /// ```
    /// use Cryptocol::number::*;
    /// fn main()
    /// {
    ///     let a_Short = ShortUnion::new_with(9);
    ///     let b_Short = func(a_Short, 5_u32);
    ///     println!("9 ** 5 = {}", unsafe { b_Short.Short } );
    ///     assert_eq!(unsafe { b_Short.Short }, 59049_u16);
    ///     // It will panic.
    ///     // println!("9 ** 5 = {}", func(a_Short, 6_u32));
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: u32) -> T
    /// {
    ///     lhs.pow(rhs)
    /// }
    /// ```
    /// 
    /// # Example for IntUnion
    /// ```
    /// use Cryptocol::number::*;
    /// fn main()
    /// {
    ///     let a_uint = IntUnion::new_with(81);
    ///     let b_uint = func(a_uint, 5_u32);
    ///     println!("81 ** 5 = {}", unsafe { b_uint.uint } );
    ///     assert_eq!(unsafe { b_uint.uint }, 3486784401_u32);
    ///     // It will panic.
    ///     // println!("81 ** 6 = {}", func(a_uint, 6_u32));
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: u32) -> T
    /// {
    ///     lhs.pow(rhs)
    /// }
    /// ```
    /// 
    /// # Example for LongUnion
    /// ```
    /// use Cryptocol::number::*;
    /// fn main()
    /// {
    ///     let a_ulong = LongUnion::new_with(6561);
    ///     let b_ulong = func(a_ulong, 5_u32);
    ///     println!("6561 ** 5 = {}", unsafe { b_ulong.ulong } );
    ///     assert_eq!(unsafe { b_ulong.ulong }, 12157665459056928801_u64);
    ///     // It will panic.
    ///     // println!("6561 ** 6 = {}", func(a_ulong, 6_u32));
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: u32) -> T
    /// {
    ///     lhs.pow(rhs)
    /// }
    /// ```
    /// 
    /// # Example for LongerUnion
    /// ```
    /// use Cryptocol::number::*;
    /// fn main()
    /// {
    ///     let a_ulonger = LongerUnion::new_with(43046721);
    ///     let b_ulonger = func(a_ulonger, 5_u32);
    ///     println!("43046721 ** 5 = {}", unsafe { b_ulonger.ulonger } );
    ///     assert_eq!(unsafe { b_ulonger.ulonger }, 147808829414345923316083210206383297601_u128);
    ///     // It will panic.
    ///     // println!("43046721 ** 6 = {}", func(a_ulonger, 6_u32));
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: u32) -> T
    /// {
    ///     lhs.pow(rhs)
    /// }
    /// ```
    /// 
    /// # Example for SizeUnion
    /// ```
    /// use Cryptocol::number::*;
    /// fn main()
    /// {
    ///     let a_size = SizeUnion::new_with(6561);
    ///     let b_size = func(a_size, 5_u32);
    ///     println!("6561 ** 5 = {}", unsafe { b_size.size } );
    ///     assert_eq!(unsafe { b_size.size }, 12157665459056928801_usize);
    ///     // It will panic.
    ///     // println!("6561 ** 6 = {}", func(a_size, 6_u32));
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: u32) -> T
    /// {
    ///     lhs.pow(rhs)
    /// }
    /// ```
    /// 
    /// # Collective Example
    /// ```
    /// use Cryptocol::number::*;
    /// fn main()
    /// {
    ///     let a_u8 = func(3_u8, 5_u32);
    ///     println!("3 ** 5 = {}", a_u8);
    ///     assert_eq!(a_u8, 243_u8);
    ///     // It will panic.
    ///     // println!("3 ** 5 = {}", func(3_u8, 6_u32));
    ///     
    ///     let a_u16 = func(9_u16, 5_u32);
    ///     println!("9 ** 5 = {}", a_u16);
    ///     assert_eq!(a_u16, 59049_u16);
    ///     // It will panic.
    ///     // println!("9 ** 5 = {}", func(9_u16, 6_u32));
    /// 
    ///     let a_u32 = func(81_u32, 5_u32);
    ///     println!("81 ** 5 = {}", a_u32);
    ///     assert_eq!(a_u32, 3486784401_u32);
    ///     // It will panic.
    ///     // println!("81 ** 6 = {}", func(81_u32, 6_u32));
    /// 
    ///     let a_u64 = func(6561_u64, 5_u32);
    ///     println!("6561 ** 5 = {}", a_u64);
    ///     assert_eq!(a_u64, 12157665459056928801_u64);
    ///     // It will panic.
    ///     // println!("6561 ** 6 = {}", func(6561_u64, 6_u32));
    /// 
    ///     let a_u128 = func(43046721_u128, 5_u32);
    ///     println!("43046721 ** 5 = {}", a_u128);
    ///     assert_eq!(a_u128, 147808829414345923316083210206383297601_u128);
    ///     // It will panic.
    ///     // println!("43046721 ** 6 = {}", func(43046721_u64, 6_u32));
    /// 
    ///     let a_usize = func(6561_usize, 5_u32);
    ///     println!("6561 ** 5 = {}", a_usize);
    ///     assert_eq!(a_usize, 12157665459056928801_usize);
    ///     // It will panic.
    ///     // println!("6561 ** 6 = {}", func(6561_usize, 6_u32));
    /// 
    ///     let a_Short = Short::new_with(9);
    ///     let b_Short = func(a_Short, 5_u32);
    ///     println!("9 ** 5 = {}", unsafe { b_Short.Short } );
    ///     assert_eq!(unsafe { b_Short.Short }, 59049_u16);
    ///     // It will panic.
    ///     // println!("9 ** 5 = {}", func(a_Short, 6_u32));
    ///     
    ///     let a_uint = UInt::new_with(81);
    ///     let b_uint = func(a_uint, 5_u32);
    ///     println!("81 ** 5 = {}", unsafe { b_uint.uint } );
    ///     assert_eq!(unsafe { b_uint.uint }, 3486784401_u32);
    ///     // It will panic.
    ///     // println!("81 ** 6 = {}", func(a_uint, 6_u32));
    ///     
    ///     let a_ulong = ULong::new_with(6561);
    ///     let b_ulong = func(a_ulong, 5_u32);
    ///     println!("6561 ** 5 = {}", unsafe { b_ulong.ulong } );
    ///     assert_eq!(unsafe { b_ulong.ulong }, 12157665459056928801_u64);
    ///     // It will panic.
    ///     // println!("6561 ** 6 = {}", func(a_ulong, 6_u32));
    ///     
    ///     let a_ulonger = ULonger::new_with(43046721);
    ///     let b_ulonger = func(a_ulonger, 5_u32);
    ///     println!("43046721 ** 5 = {}", unsafe { b_ulonger.ulonger } );
    ///     assert_eq!(unsafe { b_ulonger.ulonger }, 147808829414345923316083210206383297601_u128);
    ///     // It will panic.
    ///     // println!("43046721 ** 6 = {}", func(a_ulonger, 6_u32));
    ///     
    ///     let a_size = USize::new_with(6561);
    ///     let b_size = func(a_size, 5_u32);
    ///     println!("6561 ** 5 = {}", unsafe { b_size.size } );
    ///     assert_eq!(unsafe { b_size.size }, 12157665459056928801_usize);
    ///     // It will panic.
    ///     // println!("6561 ** 6 = {}", func(a_size, 6_u32));
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: u32) -> T
    /// {
    ///     lhs.pow(rhs)
    /// }
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// It calls the method pow() of implementation of the primitive
    /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
    /// `usize` directly. So, all the description of this method is mainly the
    /// same as that of the method pow() of implementation of the
    /// primitive unsigned integer types except example codes. Confer to the
    /// descryptions that are linked to in the section _Reference_. This
    /// plagiarism is not made maliciously but is made for the reason of
    /// effectiveness and efficiency so that users may understand better and
    /// easily how to use this method with simiilarity to the method
    /// pow() of implementation of the primitive unsigned integer types.
    /// 
    /// # References
    /// - If you want to know about the definition of the method `pow()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.pow).
    /// - If you want to know about the definition of the method `pow()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.pow).
    /// - If you want to know about the definition of the method `pow()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.pow).
    /// - If you want to know about the definition of the method `pow()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.pow).
    /// - If you want to know about the definition of the method `pow()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.pow).
    /// - If you want to know about the definition of the method `pow()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.pow).
    fn pow(self, exp: u32) -> Self;

    /// Computes self.pow(exp) with wrapping around at the boundary of the type.
    /// 
    /// # Features
    /// Wrapping (modular) exponentiation.
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
    /// 
    /// # Output
    /// It returns the self raised to the power of exp, in the type of `Self`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::*;
    /// fn main()
    /// {
    ///     let a_u8 = func(3_u8, 5_u32);
    ///     println!("3 ** 5 = {}", a_u8);
    ///     assert_eq!(a_u8, 243_u8);
    ///     
    ///     let b_u8 = func(3_u8, 6_u32);
    ///     println!("3 ** 6 = {}", b_u8);
    ///     assert_eq!(b_u8, 217_u8);
    ///     
    ///     let a_u16 = func(9_u16, 5_u32);
    ///     println!("9 ** 5 = {}", a_u16);
    ///     assert_eq!(a_u16, 59049_u16);
    ///     
    ///     let b_u16 = func(9_u16, 6_u32);
    ///     println!("9 ** 6 = {}", b_u16);
    ///     assert_eq!(b_u16, 7153_u16);
    ///     
    ///     let a_u32 = func(81_u32, 5_u32);
    ///     println!("81 ** 5 = {}", a_u32);
    ///     assert_eq!(a_u32, 3486784401_u32);
    ///     
    ///     let b_u32 = func(81_u32, 6_u32);
    ///     println!("81 ** 6 = {}", b_u32);
    ///     assert_eq!(b_u32, 3256662241_u32);
    ///     
    ///     let a_u64 = func(6561_u64, 5_u32);
    ///     println!("6561 ** 5 = {}", a_u64);
    ///     assert_eq!(a_u64, 12157665459056928801_u64);
    ///     
    ///     let b_u64 = func(6561_u64, 6_u32);
    ///     println!("6561 ** 6 = {}", b_u64);
    ///     assert_eq!(b_u64, 2721702152408675777_u64);
    ///     
    ///     let a_u128 = func(43046721_u128, 5_u32);
    ///     println!("43046721 ** 5 = {}", a_u128);
    ///     assert_eq!(a_u128, 147808829414345923316083210206383297601_u128);
    ///     
    ///     let b_u128 = func(43046721_u128, 6_u32);
    ///     println!("43046721 ** 6 = {}", b_u128);
    ///     assert_eq!(b_u128, 333574137813082321045752866839264852865_u128);
    ///     
    ///     let a_usize = func(6561_usize, 5_u32);
    ///     println!("6561 ** 5 = {}", a_usize);
    ///     assert_eq!(a_usize, 12157665459056928801_usize);
    ///     
    ///     let b_usize = func(6561_usize, 6_u32);
    ///     println!("6561 ** 6 = {}", b_usize);
    ///     assert_eq!(b_usize, 2721702152408675777_usize);
    ///     
    ///     let a_ushort = ShortUnion::new_with(9);
    ///     let b_ushort = func(a_ushort, 5_u32);
    ///     println!("9 ** 5 = {}", b_ushort);
    ///     assert_eq!(b_ushort.get(), 59049_u16);
    ///     
    ///     let c_ushort = func(a_ushort, 6_u32);
    ///     println!("9 ** 6 = {}", c_ushort);
    ///     assert_eq!(c_ushort.get(), 7153_u16);
    ///     
    ///     let a_uint = IntUnion::new_with(81);
    ///     let b_uint = func(a_uint, 5_u32);
    ///     println!("81 ** 5 = {}", b_uint);
    ///     assert_eq!(b_uint.get(), 3486784401_u32);
    ///     
    ///     let c_uint = func(a_uint, 6_u32);
    ///     println!("81 ** 6 = {}", c_uint);
    ///     assert_eq!(c_uint.get(), 3256662241_u32);
    ///     
    ///     let a_ulong = LongUnion::new_with(6561);
    ///     let b_ulong = func(a_ulong, 5_u32);
    ///     println!("6561 ** 5 = {}", b_ulong);
    ///     assert_eq!(b_ulong.get(), 12157665459056928801_u64);
    ///     
    ///     let c_ulong = func(a_ulong, 6_u32);
    ///     println!("6561 ** 6 = {}", c_ulong);
    ///     assert_eq!(c_ulong.get(), 2721702152408675777_u64);
    ///     
    ///     let a_ulonger = LongerUnion::new_with(43046721);
    ///     let b_ulonger = func(a_ulonger, 5_u32);
    ///     println!("43046721 ** 5 = {}", b_ulonger);
    ///     assert_eq!(b_ulonger.get(), 147808829414345923316083210206383297601_u128);
    ///     
    ///     let c_ulonger = func(a_ulonger, 6_u32);
    ///     println!("43046721 ** 6 = {}", c_ulonger);
    ///     assert_eq!(c_ulonger.get(), 333574137813082321045752866839264852865_u128);
    ///     
    ///     let a_size = SizeUnion::new_with(6561);
    ///     let b_size = func(a_size, 5_u32);
    ///     println!("6561 ** 5 = {}", b_size);
    ///     assert_eq!(b_size.get(), 12157665459056928801_usize);
    ///     
    ///     let c_size = func(a_size, 6_u32);
    ///     println!("6561 ** 6 = {}", c_size);
    ///     assert_eq!(c_size.get(), 2721702152408675777_usize);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: u32) -> T
    /// {
    ///     lhs.wrapping_pow(rhs)
    /// }
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// It calls the method wrapping_pow() of implementation of the primitive
    /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
    /// `usize` directly. So, all the description of this method is mainly the
    /// same as that of the method wrapping_pow() of implementation of the
    /// primitive unsigned integer types except example codes. Confer to the
    /// descryptions that are linked to in the section _Reference_. This
    /// plagiarism is not made maliciously but is made for the reason of
    /// effectiveness and efficiency so that users may understand better and
    /// easily how to use this method with simiilarity to the method
    /// wrapping_pow() of implementation of the primitive unsigned integer types.
    /// 
    /// # References
    /// - If you want to know about the definition of the method `wrapping_pow()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.wrapping_pow).
    /// - If you want to know about the definition of the method `wrapping_pow()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.wrapping_pow).
    /// - If you want to know about the definition of the method `wrapping_pow()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.wrapping_pow).
    /// - If you want to know about the definition of the method `wrapping_pow()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.wrapping_pow).
    /// - If you want to know about the definition of the method `wrapping_pow()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.wrapping_pow).
    /// - If you want to know about the definition of the method `wrapping_pow()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.wrapping_pow).
    fn wrapping_pow(self, exp: u32) -> Self;

    /// Raises self to the power of exp, using exponentiation by squaring.
    /// 
    /// # Features
    /// Exponentiation by squaring.
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
    /// 
    /// # Output
    /// It returns a tuple of the exponentiation along with a bool indicating
    /// whether an overflow happened.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::*;
    /// 
    /// fn main()
    /// {
    ///     // Todo
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: u32) -> (T, bool)
    /// {
    ///     lhs.overflowing_pow(rhs)
    /// }
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// It calls the method overflowing_pow() of implementation of the primitive
    /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
    /// `usize` directly. So, all the description of this method is mainly the
    /// same as that of the method overflowing_pow() of implementation of the
    /// primitive unsigned integer types except example codes. Confer to the
    /// descryptions that are linked to in the section _Reference_. This
    /// plagiarism is not made maliciously but is made for the reason of
    /// effectiveness and efficiency so that users may understand better and
    /// easily how to use this method with simiilarity to the method
    /// overflowing_pow() of implementation of the primitive unsigned integer types.
    /// 
    /// # References
    /// - If you want to know about the definition of the method `overflowing_pow()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.overflowing_pow).
    /// - If you want to know about the definition of the method `overflowing_pow()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.overflowing_pow).
    /// - If you want to know about the definition of the method `overflowing_pow()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.overflowing_pow).
    /// - If you want to know about the definition of the method `overflowing_pow()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.overflowing_pow).
    /// - If you want to know about the definition of the method `overflowing_pow()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.overflowing_pow).
    /// - If you want to know about the definition of the method `overflowing_pow()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.overflowing_pow).
    fn overflowing_pow(self, exp: u32) -> (Self, bool);

    /// Computes self.pow(exp), returning None if overflow occurred.
    /// 
    /// # Features
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
    /// 
    /// # Output
    /// It returns None if overflow occurred. Otherwise, it returns 'self
    /// raised to the power of exp' wrapped by `Some` of enum `Option`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::*;
    /// fn main()
    /// {
    ///     // Todo
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: u32) -> Option<T>
    /// {
    ///     lhs.checked_pow(rhs)
    /// }
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// It calls the method checked_pow() of implementation of the primitive
    /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
    /// `usize` directly. So, all the description of this method is mainly the
    /// same as that of the method checked_pow() of implementation of the
    /// primitive unsigned integer types except example codes. Confer to the
    /// descryptions that are linked to in the section _Reference_. This
    /// plagiarism is not made maliciously but is made for the reason of
    /// effectiveness and efficiency so that users may understand better and
    /// easily how to use this method with simiilarity to the method
    /// checked_pow() of implementation of the primitive unsigned integer types.
    /// 
    /// # References
    /// - If you want to know about the definition of the method `checked_pow()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.checked_pow).
    /// - If you want to know about the definition of the method `checked_pow()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.checked_pow).
    /// - If you want to know about the definition of the method `checked_pow()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.checked_pow).
    /// - If you want to know about the definition of the method `checked_pow()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.checked_pow).
    /// - If you want to know about the definition of the method `checked_pow()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.checked_pow).
    /// - If you want to know about the definition of the method `checked_pow()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.checked_pow).
    fn checked_pow(self, exp: u32) -> Option<Self>;

    /// Computes self.pow(exp), saturating at the numeric bounds instead of overflowing.
    /// 
    /// # Features
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
    /// 
    /// # Output
    /// It returns 'self raised to the power of exp' in the type of `Self`
    /// if overflow does not happen. Otherwise, it returns the maximum value
    /// of the type `Self`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::*;
    /// fn main()
    /// {
    ///     // Todo
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: u32) -> T
    /// {
    ///     lhs.saturating_pow(rhs)
    /// }
    /// ```
    /// 
    /// # Plagiarism in descryption
    /// It calls the method saturating_pow() of implementation of the primitive
    /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
    /// `usize` directly. So, all the description of this method is mainly the
    /// same as that of the method saturating_pow() of implementation of the
    /// primitive unsigned integer types except example codes. Confer to the
    /// descryptions that are linked to in the section _Reference_. This
    /// plagiarism is not made maliciously but is made for the reason of
    /// effectiveness and efficiency so that users may understand better and
    /// easily how to use this method with simiilarity to the method
    /// saturating_pow() of implementation of the primitive unsigned integer types.
    /// 
    /// # References
    /// - If you want to know about the definition of the method `saturating_pow()`
    /// for the primitive type `u8`, read [here](https://doc.rust-lang.org/core/primitive.u8.html#method.saturating_pow).
    /// - If you want to know about the definition of the method `saturating_pow()`
    /// for the primitive type `u16`, read [here](https://doc.rust-lang.org/core/primitive.u16.html#method.saturating_pow).
    /// - If you want to know about the definition of the method `saturating_pow()`
    /// for the primitive type `u32`, read [here](https://doc.rust-lang.org/core/primitive.u32.html#method.saturating_pow).
    /// - If you want to know about the definition of the method `saturating_pow()`
    /// for the primitive type `u64`, read [here](https://doc.rust-lang.org/core/primitive.u64.html#method.saturating_pow).
    /// - If you want to know about the definition of the method `saturating_pow()`
    /// for the primitive type `u128`, read [here](https://doc.rust-lang.org/core/primitive.u128.html#method.saturating_pow).
    /// - If you want to know about the definition of the method `saturating_pow()`
    /// for the primitive type `usize`, read [here](https://doc.rust-lang.org/core/primitive.usize.html#method.saturating_pow).
    fn saturating_pow(self, exp: u32) -> Self;


    // fn modular_pow(self, exp: Self, modulo: Self) -> Self
    /// Raises `BigUInt` type number to the power of exp, using exponentiation
    /// of type `U` by squaring, wrapping around at `modulo` of the
    /// type `U`. The type `U` has the trait `SmallUInt`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` < `size_of::<U>()`, This method may panic
    /// or its behavior may undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the result of `self` raised to the power of `exp`, wrapping
    /// around at `modulo`.
    /// 
    /// # Argument
    /// The argument `exp` is the primitive unsigned integer type.
    /// 
    /// # Feature
    /// Wrapping (modular) exponentiation, wrapping around at `modulo`.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, use the method `modular_pow()` instead.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a = u256::from_uint(123_u8);
    /// 
    /// // normal exponentiation
    /// let b = a.wrapping_pow_uint(37_u8);
    /// println!("123 ** 37 = {}", b);
    /// assert_eq!(b.to_string(), "96282738670724731919703551810636030185721623691319861614277235426286836107467");
    /// 
    /// // wrapping (modular) exponentiation
    /// let c = a.wrapping_pow_uint(38_u8);
    /// println!("123 ** 38 = {}", c);
    /// assert_eq!(c.to_string(), "31983754292890092919296401822065111810221278137005446531426388626141617944969");
    /// 
    /// // evidence of wrapping (modular) exponentiation
    /// assert!(b > c);
    /// ```
    fn modular_pow(self, exp: Self, modulo: Self) -> Self;

    fn ilog(self, base: Self) -> u32;
    fn ilog10(self) -> u32;
    fn ilog2(self) -> u32;

    fn sqrt(self) -> Self;
    fn root(self, exp: Self) -> Self;


    /***** METHODS FOR GENERATING RANDOM NUMBERS AND PRIME NUMBERS *****/

    // fn any() -> Self
    /// Constucts a new `SmallUInt`-type object which has the random value.
    /// 
    /// # Output
    /// The random number that this method any() returns is a pure random
    /// number whose range is from 0 up to `SmallUInt::max()` inclusively.
    /// 
    /// # Features
    /// This method basically uses the method randomize() that fills all the
    /// elements of the array number[T; N] in struct BigUInt<T, N> with the
    /// cryptographically secure random numbers by means of
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html))
    /// which is considered to be cryptographically secure.
    /// 
    /// # Cryptographical Security
    /// This method generates cryptographically insecure random number.
    /// 
    /// # Example
    /// ```
    /// ```
    fn any() -> Self;

    // fn any_odd() -> Self
    /// Constucts a new `BigUInt<T, N>`-type object which has the random odd
    /// value.
    /// 
    /// # Output
    /// The random number that this method random_odd() returns is a pure
    /// random odd number whose range is from 1 up to BigUInt::max() inclusively.
    /// 
    /// # Features
    /// This method random_odd() generates a random number and then simply sets
    /// its LSB (Least Significant Bit) to be one.
    /// This method basically uses the method randomize() that fills all the
    /// elements of the array number[T; N] in struct BigUInt<T, N> with the
    /// cryptographically secure random numbers by means of
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html))
    /// which is considered to be cryptographically secure.
    /// 
    /// # Cryptographical Security
    /// It is not sure that the random number generated by this method
    /// random_odd() can be considered cryptographically secure though this
    /// method random_odd() is based on the crate [rand](https://docs.rs/rand/latest/rand/index.html)
    /// (especially, [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// The author is not sure that the _extended_ random number generated
    /// in the way explained in the section 'Features' is also
    /// cryptographically secure recursively.
    /// 
    /// # Counterpart Methods
    /// - If you want to be sure to use cryptographically secure pure random
    /// number whose range is from 0 up to BigUInt::max() inclusively, you are
    /// highly encouraged to use the method [random()](struct@BigUInt#method.random)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than)
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use (N * 8)-bit long random number for cryptographic
    /// purpose, you are highly recommended to use the method [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use random odd number for cryptographic purpose, you
    /// are highly recommended to use this method [random_odd()](struct@BigUInt#method.random_odd)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than),
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use random number less than a certain value for
    /// cryptographic purpose, you are highly recommended to use this method
    /// [random_less_than()](struct@BigUInt#method.random_less_than)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than),
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use random odd number less than a certain value for
    /// cryptographic purpose, you are highly recommended to use this method
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use (N * 8)-bit long random odd number for
    /// cryptographic purpose, you are highly recommended to use this method
    /// [random_odd_with_MSB_set()](struct@BigUInt#method.random_odd_with_MSB_set)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd_less_than](struct@BigUInt#method.random_odd_less_than), and
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set).
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let r = u1024::random_odd();
    /// println!("Random Odd Number: {}", r);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    fn any_odd() -> Self
    {
        let mut res = Self::any();
        res.set_LSB();
        res
    }

    // fn any_less_than(ceiling: Self) -> Self
    /// Constucts a new `BigUInt<T, N>`-type object which has the random
    /// value less than a certain value.
    /// 
    /// # Output
    /// The random number that this method random_less_than() returns is
    /// a pure random number whose range is between 0 inclusively
    /// and the certain value exclusively.
    /// 
    /// # Features
    /// This method random_less_than() generates a random number,
    /// and then simply divides it by the certain value to get its remainder.
    /// This method basically uses the method randomize() that fills all the
    /// elements of the array number[T; N] in struct BigUInt<T, N> with the
    /// cryptographically secure random numbers by means of
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html))
    /// which is considered to be cryptographically secure.
    /// 
    /// # Cryptographical Security
    /// It is not sure that the random number generated by this method
    /// random_less_than() can be considered cryptographically secure though
    /// this method random_less_than() is based on the crate [rand](https://docs.rs/rand/latest/rand/index.html)
    /// (especially, [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// The author is not sure that the _extended_ random number generated
    /// in the way explained in the section 'Features' is also
    /// cryptographically secure recursively.
    /// 
    /// # Counterpart Methods
    /// - If you want to be sure to use cryptographically secure pure random
    /// number whose range is from 0 up to BigUInt::max() inclusively, you are
    /// highly encouraged to use the method [random()](struct@BigUInt#method.random)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than)
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use (N * 8)-bit long random number for cryptographic
    /// purpose, you are highly recommended to use the method [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use random odd number for cryptographic purpose, you
    /// are highly recommended to use this method [random_odd()](struct@BigUInt#method.random_odd)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than),
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use random number less than a certain value for
    /// cryptographic purpose, you are highly recommended to use this method
    /// [random_less_than()](struct@BigUInt#method.random_less_than)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than),
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use random odd number less than a certain value for
    /// cryptographic purpose, you are highly recommended to use this method
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use (N * 8)-bit long random odd number for
    /// cryptographic purpose, you are highly recommended to use this method
    /// [random_odd_with_MSB_set()](struct@BigUInt#method.random_odd_with_MSB_set)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd_less_than](struct@BigUInt#method.random_odd_less_than), and
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set).
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let ceiling = u1024::max() / u1024::from_uint::<u32>(3);
    /// let r = u1024::random_less_than(&ceiling);
    /// println!("Random Number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// ```
    #[inline] fn any_less_than(ceiling: Self) -> Self   { Self::any().wrapping_rem(ceiling) }

    // fn any_odd_less_than(ceiling: Self) -> Self
    /// Constucts a new `BigUInt<T, N>`-type object which has the random odd
    /// value less than a certain value.
    /// 
    /// # Output
    /// The random number that this method random_odd_less_than() returns is
    /// a pure random odd number whose range is between 0 inclusively and
    /// the certain value exclusively.
    /// 
    /// # Features
    /// This method random_odd_less_than() generates a random number
    /// and then simply divides it by the certain value to get its remainder.
    /// This method basically uses the method randomize() that fills all the
    /// elements of the array number[T; N] in struct BigUInt<T, N> with the
    /// cryptographically secure random numbers by means of
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html))
    /// which is considered to be cryptographically secure.
    /// 
    /// # Cryptographical Security
    /// It is not sure that the random number generated by this method
    /// random_odd_less_than() can be considered cryptographically secure
    /// though this method random_odd_less_than() is based on the crate
    /// [rand](https://docs.rs/rand/latest/rand/index.html) (especially,
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// The author is not sure that the _extended_ random number generated
    /// in the way explained in the section 'Features' is also
    /// cryptographically secure recursively.
    /// 
    /// # Counterpart Methods
    /// - If you want to be sure to use cryptographically secure pure random
    /// number whose range is from 0 up to BigUInt::max() inclusively, you are
    /// highly encouraged to use the method [random()](struct@BigUInt#method.random)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than)
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use (N * 8)-bit long random number for cryptographic
    /// purpose, you are highly recommended to use the method [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use random odd number for cryptographic purpose, you
    /// are highly recommended to use this method [random_odd()](struct@BigUInt#method.random_odd)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than),
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use random number less than a certain value for
    /// cryptographic purpose, you are highly recommended to use this method
    /// [random_less_than()](struct@BigUInt#method.random_less_than)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than),
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use random odd number less than a certain value for
    /// cryptographic purpose, you are highly recommended to use this method
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use (N * 8)-bit long random odd number for
    /// cryptographic purpose, you are highly recommended to use this method
    /// [random_odd_with_MSB_set()](struct@BigUInt#method.random_odd_with_MSB_set)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd_less_than](struct@BigUInt#method.random_odd_less_than), and
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set).
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let ceiling = u1024::max() / u1024::from_uint::<u32>(3);
    /// let r = u1024::random_odd_less_than(&ceiling);
    /// println!("Random Odd Number less than {} is\n{}", ceiling, u1024::random_odd_less_than(&ceiling));
    /// assert!(r < ceiling);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    fn any_odd_less_than(ceiling: Self) -> Self
    {
        let mut res = Self::any_less_than(ceiling);
        res.set_LSB();
        res
    }

    // fn any_with_MSB_set() -> Self
    /// Constucts a new `BigUInt<T, N>`-type object which has the random value
    /// with MSB (Most Significant Bit) is set.
    /// 
    /// # Output
    /// The random number that this
    /// method random_with_MSB_set() returns is a random number whose range
    /// is from !(BigUInt::max() >> 1) up to BigUInt::max() inclusively.
    /// 
    /// # Features
    /// This method random_with_MSB_set() generates a random number and then
    /// simply sets its MSB (Most Significant Bit) to be one.
    /// This method basically uses the method randomize() that fills all the
    /// elements of the array number[T; N] in struct BigUInt<T, N> with the
    /// cryptographically secure random numbers by means of
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html))
    /// which is considered to be cryptographically secure.
    /// 
    /// # Cryptographical Security
    /// It is not sure that the random number generated by this method
    /// random_with_MSB_set() can be considered to be cryptographically secure
    /// though it uses the crate [rand](https://docs.rs/rand/latest/rand/index.html)
    /// ([rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// The author is not sure that the _extended_ random number generated
    /// in the way explained in the section 'Features' is also
    /// cryptographically secure recursively.
    /// 
    /// # Counterpart Methods
    /// - If you want to be sure to use cryptographically secure pure random
    /// number whose range is from 0 up to BigUInt::max() inclusively, you are
    /// highly encouraged to use the method [random()](struct@BigUInt#method.random)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than)
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use (N * 8)-bit long random number for cryptographic
    /// purpose, you are highly recommended to use the method [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use random odd number for cryptographic purpose, you
    /// are highly recommended to use this method [random_odd()](struct@BigUInt#method.random_odd)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than),
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use random number less than a certain value for
    /// cryptographic purpose, you are highly recommended to use this method
    /// [random_less_than()](struct@BigUInt#method.random_less_than)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than),
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use random odd number less than a certain value for
    /// cryptographic purpose, you are highly recommended to use this method
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use (N * 8)-bit long random odd number for
    /// cryptographic purpose, you are highly recommended to use this method
    /// [random_odd_with_MSB_set()](struct@BigUInt#method.random_odd_with_MSB_set)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd_less_than](struct@BigUInt#method.random_odd_less_than), and
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set).
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let num = u1024::random_with_MSB_set();
    /// println!("Random Number = {}", u1024::random());
    /// println!("1024-bit Random Number = {}", num);
    /// assert!(num > u1024::submax(1023));
    /// ```
    ///
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    fn any_with_MSB_set() -> Self
    {
        let mut r = Self::any();
        r.set_MSB();
        r
    }

    // fn any_odd_with_MSB_set() -> Self
    /// Constucts a new `BigUInt<T, N>`-type object which has the random odd
    /// value with MSB (Most Significant Bit) is set.
    /// 
    /// # Output
    /// The random number that this method random_odd_with_MSB_set() returns is
    /// a random odd number whose range is from !(BigUInt::max() >> 1) + 1 up to
    /// BigUInt::max() inclusively.
    /// 
    /// # Features
    /// This method random_odd_with_MSB_set() generates a random number and then
    /// simply sets its MSB (Most Significant Bit) and LSB (Least Significant
    /// Bit) to be one.
    /// This method basically uses the method randomize() that fills all the
    /// elements of the array number[T; N] in struct BigUInt<T, N> with the
    /// cryptographically secure random numbers by means of
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html))
    /// which is considered to be cryptographically secure.
    /// 
    /// # Cryptographical Security
    /// It is not sure that the random number generated by this method
    /// random_odd_with_MSB_set() can be considered to be cryptographically
    /// secure though it uses the crate [rand](https://docs.rs/rand/latest/rand/index.html)
    /// ([rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)),
    /// The author is not sure that the _extended_ random number generated
    /// in the way explained in the section 'Features' is also
    /// cryptographically secure recursively.
    /// 
    /// # Counterpart Methods
    /// - If you want to be sure to use cryptographically secure pure random
    /// number whose range is from 0 up to BigUInt::max() inclusively, you are
    /// highly encouraged to use the method [random()](struct@BigUInt#method.random)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than)
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use (N * 8)-bit long random number for cryptographic
    /// purpose, you are highly recommended to use the method [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use random odd number for cryptographic purpose, you
    /// are highly recommended to use this method [random_odd()](struct@BigUInt#method.random_odd)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than),
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use random number less than a certain value for
    /// cryptographic purpose, you are highly recommended to use this method
    /// [random_less_than()](struct@BigUInt#method.random_less_than)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than),
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use random odd number less than a certain value for
    /// cryptographic purpose, you are highly recommended to use this method
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use (N * 8)-bit long random odd number for
    /// cryptographic purpose, you are highly recommended to use this method
    /// [random_odd_with_MSB_set()](struct@BigUInt#method.random_odd_with_MSB_set)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd_less_than](struct@BigUInt#method.random_odd_less_than), and
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set).
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let num = u1024::random_odd_with_MSB_set();
    /// println!("Random Number = {}", u1024::random());
    /// println!("1024-bit Random Odd Number = {}", num);
    /// assert!(num > u1024::submax(1023));
    /// assert!(num.is_odd());
    /// ```
    fn any_odd_with_MSB_set() -> Self
    {
        let mut r = Self::any_with_MSB_set();
        r.set_LSB();
        r
    }

    // fn any_prime_using_Miller_Rabin(repetition: usize) -> Self
    /// Constucts a new `BigUInt<T, N>`-type object which represents a random
    /// prime number with MSB (Most Significant Bit) is set.
    /// 
    /// # Output
    /// The random prime number that this method random_prime_Miller_Rabin()
    /// returns is a random prime number whose range is from
    /// !(BigUInt::max() >> 1) + 1 up to BigUInt::max() inclusively.
    /// 
    /// # Features
    /// It uses [Miller Rabin algorithm](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test).
    /// If this test results in composite number, the tested number is surely a
    /// composite number. If this test results in prime number, the probability
    /// that the tested number is not a prime number is 1/4. So, if the test
    /// results in prime number twice, the probability that the tested number
    /// is not a prime number is 1/16 (= 1/4 * 1/4). Therefore, if you test any
    /// number 5 times and they all result in a prime number, it is 99.9% that
    /// the number is a prime number.
    /// This method basically uses the method randomize() that fills all the
    /// elements of the array number[T; N] in struct BigUInt<T, N> with the
    /// cryptographically secure random numbers by means of
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html))
    /// which is considered to be cryptographically secure.
    /// 
    /// # Argument
    /// The argument `repetition` defines how many times it tests whether the
    /// generated random number is prime. Usually, `repetition` is given to be
    /// 5 to have 99.9% accuracy. 
    /// 
    /// # Cryptographical Security
    /// It is not sure that the random number generated by this method
    /// random_prime_Miller_Rabin() can be considered to be cryptographically
    /// secure though it uses the crate [rand](https://docs.rs/rand/latest/rand/index.html)
    /// ([rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// This method random_prime_Miller_Rabin() generates a random number, and
    /// then simply sets its MSB (Most Significant Bit) and LSB (Least
    /// Significant Bit) to be one, and then checks whether the generated random
    /// number is prime number, and then it repeats until it will generate a
    /// prime number.
    /// The author is not sure that the _extended_ random number generated
    /// in the way explained in the section 'Features' is also
    /// cryptographically secure recursively.
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    ///
    /// let num = u256::random_prime_using_Miller_Rabin(5);
    /// println!("Random Prime Number = {}", num);
    /// assert!(num.is_prime_using_Miller_Rabin(5));
    /// ```
    fn any_prime_using_Miller_Rabin(repetition: usize) -> Self
    {
        let mut complete = false;
        let mut res = Self::zero();
        while !complete
        {
            res.turn_any();
            res.set_MSB();
            res.set_LSB();
            complete = res.is_prime_using_Miller_Rabin(repetition);
        }
        res
    }

    // fn turn_any(&mut self)
    /// Make a `SmallUInt<T, N>`-type object to have a random value.
    /// The random number that this method randomize() makes is a pure random
    /// number whose range is from 0 up to SmallUInt::max() inclusively.
    /// 
    /// # Features
    /// This method randomize() fills all the elements of the array number[T; N]
    /// in struct BigUInt<T, N> with the cryptographically secure random numbers
    /// by means of [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html))
    /// which is considered to be cryptographically secure.
    /// 
    /// # Cryptographical Security
    /// It is not sure that the random number generated by this method random()
    /// can be considered cryptographically secure though this method random()
    /// is based on the crate [rand](https://docs.rs/rand/latest/rand/index.html) (especially,
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// The author is not sure that the _extended_ random number generated
    /// in the way explained in the section 'Features' is also
    /// cryptographically secure recursively.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut r = u256::new();
    /// println!("original number = {}", r);
    /// assert_eq!(r, u256::zero());
    /// r.randomize();
    /// println!("random number = {}", r);
    /// assert_ne!(r, u256::zero());
    /// ```
    #[inline] fn turn_any(&mut self)   { *self = Self::any(); }

    // fn random() -> Self
    /// Constucts a new `SmallUInt`-type object which has the random value.
    /// 
    /// # Output
    /// The random number that this method random() returns is a pure random
    /// number whose range is from 0 up to `SmallUInt::max()` inclusively.
    /// 
    /// # Features
    /// This method basically uses the method randomize() that fills all the
    /// elements of the array number[T; N] in struct BigUInt<T, N> with the
    /// cryptographically secure random numbers by means of
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html))
    /// which is considered to be cryptographically secure.
    /// 
    /// # Cryptographical Security
    /// It is not sure that the random number generated by this method random()
    /// can be considered cryptographically secure though this method random()
    /// is based on the crate [rand](https://docs.rs/rand/latest/rand/index.html)
    /// (especially, [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// The author is not sure that the _extended_ random number generated
    /// in the way explained in the section 'Features' is also
    /// cryptographically secure recursively.
    /// 
    /// # Counterpart Methods
    /// - If you want to be sure to use cryptographically secure pure random
    /// number whose range is from 0 up to BigUInt::max() inclusively, you are
    /// highly encouraged to use the method [random()](struct@BigUInt#method.random)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than)
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use (N * 8)-bit long random number for cryptographic
    /// purpose, you are highly recommended to use the method [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to uKerckhoffs principleBigUInt#method.random),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than),
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use random number less than a certain value for
    /// cryptographic purpose, you are highly recommended to use this method
    /// [random_less_than()](struct@BigUInt#method.random_less_than)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than),
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use random odd number less than a certain value for
    /// cryptographic purpose, you are highly recommended to use this method
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use (N * 8)-bit long random odd number for
    /// cryptographic purpose, you are highly recommended to use this method
    /// [random_odd_with_MSB_set()](struct@BigUInt#method.random_odd_with_MSB_set)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd_less_than](struct@BigUInt#method.random_odd_less_than), and
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set).
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// println!("Random Number: {}", u1024::random());
    /// ```
    fn random() -> Self;

    // fn random_odd() -> Self
    /// Constucts a new `BigUInt<T, N>`-type object which has the random odd
    /// value.
    /// 
    /// # Output
    /// The random number that this method random_odd() returns is a pure
    /// random odd number whose range is from 1 up to BigUInt::max() inclusively.
    /// 
    /// # Features
    /// This method random_odd() generates a random number and then simply sets
    /// its LSB (Least Significant Bit) to be one.
    /// This method basically uses the method randomize() that fills all the
    /// elements of the array number[T; N] in struct BigUInt<T, N> with the
    /// cryptographically secure random numbers by means of
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html))
    /// which is considered to be cryptographically secure.
    /// 
    /// # Cryptographical Security
    /// It is not sure that the random number generated by this method
    /// random_odd() can be considered cryptographically secure though this
    /// method random_odd() is based on the crate [rand](https://docs.rs/rand/latest/rand/index.html)
    /// (especially, [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// The author is not sure that the _extended_ random number generated
    /// in the way explained in the section 'Features' is also
    /// cryptographically secure recursively.
    /// 
    /// # Counterpart Methods
    /// - If you want to be sure to use cryptographically secure pure random
    /// number whose range is from 0 up to BigUInt::max() inclusively, you are
    /// highly encouraged to use the method [random()](struct@BigUInt#method.random)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than)
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use (N * 8)-bit long random number for cryptographic
    /// purpose, you are highly recommended to use the method [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use random odd number for cryptographic purpose, you
    /// are highly recommended to use this method [random_odd()](struct@BigUInt#method.random_odd)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than),
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use random number less than a certain value for
    /// cryptographic purpose, you are highly recommended to use this method
    /// [random_less_than()](struct@BigUInt#method.random_less_than)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than),
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use random odd number less than a certain value for
    /// cryptographic purpose, you are highly recommended to use this method
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use (N * 8)-bit long random odd number for
    /// cryptographic purpose, you are highly recommended to use this method
    /// [random_odd_with_MSB_set()](struct@BigUInt#method.random_odd_with_MSB_set)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd_less_than](struct@BigUInt#method.random_odd_less_than), and
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set).
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let r = u1024::random_odd();
    /// println!("Random Odd Number: {}", r);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    fn random_odd() -> Self
    {
        let mut res = Self::random();
        res.set_LSB();
        res
    }

    // fn random_less_than(ceiling: Self) -> Self
    /// Constucts a new `BigUInt<T, N>`-type object which has the random
    /// value less than a certain value.
    /// 
    /// # Output
    /// The random number that this method random_less_than() returns is
    /// a pure random number whose range is between 0 inclusively
    /// and the certain value exclusively.
    /// 
    /// # Features
    /// This method random_less_than() generates a random number,
    /// and then simply divides it by the certain value to get its remainder.
    /// This method basically uses the method randomize() that fills all the
    /// elements of the array number[T; N] in struct BigUInt<T, N> with the
    /// cryptographically secure random numbers by means of
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html))
    /// which is considered to be cryptographically secure.
    /// 
    /// # Cryptographical Security
    /// It is not sure that the random number generated by this method
    /// random_less_than() can be considered cryptographically secure though
    /// this method random_less_than() is based on the crate [rand](https://docs.rs/rand/latest/rand/index.html)
    /// (especially, [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// The author is not sure that the _extended_ random number generated
    /// in the way explained in the section 'Features' is also
    /// cryptographically secure recursively.
    /// 
    /// # Counterpart Methods
    /// - If you want to be sure to use cryptographically secure pure random
    /// number whose range is from 0 up to BigUInt::max() inclusively, you are
    /// highly encouraged to use the method [random()](struct@BigUInt#method.random)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than)
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use (N * 8)-bit long random number for cryptographic
    /// purpose, you are highly recommended to use the method [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use random odd number for cryptographic purpose, you
    /// are highly recommended to use this method [random_odd()](struct@BigUInt#method.random_odd)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than),
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use random number less than a certain value for
    /// cryptographic purpose, you are highly recommended to use this method
    /// [random_less_than()](struct@BigUInt#method.random_less_than)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than),
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use random odd number less than a certain value for
    /// cryptographic purpose, you are highly recommended to use this method
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use (N * 8)-bit long random odd number for
    /// cryptographic purpose, you are highly recommended to use this method
    /// [random_odd_with_MSB_set()](struct@BigUInt#method.random_odd_with_MSB_set)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd_less_than](struct@BigUInt#method.random_odd_less_than), and
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set).
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let ceiling = u1024::max() / u1024::from_uint::<u32>(3);
    /// let r = u1024::random_less_than(&ceiling);
    /// println!("Random Number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// ```
    #[inline] fn random_less_than(ceiling: Self) -> Self   { Self::random().wrapping_rem(ceiling) }

    // fn random_odd_less_than(ceiling: Self) -> Self
    /// Constucts a new `BigUInt<T, N>`-type object which has the random odd
    /// value less than a certain value.
    /// 
    /// # Output
    /// The random number that this method random_odd_less_than() returns is
    /// a pure random odd number whose range is between 0 inclusively and
    /// the certain value exclusively.
    /// 
    /// # Features
    /// This method random_odd_less_than() generates a random number
    /// and then simply divides it by the certain value to get its remainder.
    /// This method basically uses the method randomize() that fills all the
    /// elements of the array number[T; N] in struct BigUInt<T, N> with the
    /// cryptographically secure random numbers by means of
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html))
    /// which is considered to be cryptographically secure.
    /// 
    /// # Cryptographical Security
    /// It is not sure that the random number generated by this method
    /// random_odd_less_than() can be considered cryptographically secure
    /// though this method random_odd_less_than() is based on the crate
    /// [rand](https://docs.rs/rand/latest/rand/index.html) (especially,
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// The author is not sure that the _extended_ random number generated
    /// in the way explained in the section 'Features' is also
    /// cryptographically secure recursively.
    /// 
    /// # Counterpart Methods
    /// - If you want to be sure to use cryptographically secure pure random
    /// number whose range is from 0 up to BigUInt::max() inclusively, you are
    /// highly encouraged to use the method [random()](struct@BigUInt#method.random)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than)
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use (N * 8)-bit long random number for cryptographic
    /// purpose, you are highly recommended to use the method [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use random odd number for cryptographic purpose, you
    /// are highly recommended to use this method [random_odd()](struct@BigUInt#method.random_odd)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than),
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use random number less than a certain value for
    /// cryptographic purpose, you are highly recommended to use this method
    /// [random_less_than()](struct@BigUInt#method.random_less_than)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than),
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use random odd number less than a certain value for
    /// cryptographic purpose, you are highly recommended to use this method
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use (N * 8)-bit long random odd number for
    /// cryptographic purpose, you are highly recommended to use this method
    /// [random_odd_with_MSB_set()](struct@BigUInt#method.random_odd_with_MSB_set)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd_less_than](struct@BigUInt#method.random_odd_less_than), and
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set).
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let ceiling = u1024::max() / u1024::from_uint::<u32>(3);
    /// let r = u1024::random_odd_less_than(&ceiling);
    /// println!("Random Odd Number less than {} is\n{}", ceiling, u1024::random_odd_less_than(&ceiling));
    /// assert!(r < ceiling);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    fn random_odd_less_than(ceiling: Self) -> Self
    {
        let mut res = Self::random_less_than(ceiling);
        res.set_LSB();
        res
    }

    // fn random_with_MSB_set() -> Self
    /// Constucts a new `BigUInt<T, N>`-type object which has the random value
    /// with MSB (Most Significant Bit) is set.
    /// 
    /// # Output
    /// The random number that this
    /// method random_with_MSB_set() returns is a random number whose range
    /// is from !(BigUInt::max() >> 1) up to BigUInt::max() inclusively.
    /// 
    /// # Features
    /// This method random_with_MSB_set() generates a random number and then
    /// simply sets its MSB (Most Significant Bit) to be one.
    /// This method basically uses the method randomize() that fills all the
    /// elements of the array number[T; N] in struct BigUInt<T, N> with the
    /// cryptographically secure random numbers by means of
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html))
    /// which is considered to be cryptographically secure.
    /// 
    /// # Cryptographical Security
    /// It is not sure that the random number generated by this method
    /// random_with_MSB_set() can be considered to be cryptographically secure
    /// though it uses the crate [rand](https://docs.rs/rand/latest/rand/index.html)
    /// ([rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// The author is not sure that the _extended_ random number generated
    /// in the way explained in the section 'Features' is also
    /// cryptographically secure recursively.
    /// 
    /// # Counterpart Methods
    /// - If you want to be sure to use cryptographically secure pure random
    /// number whose range is from 0 up to BigUInt::max() inclusively, you are
    /// highly encouraged to use the method [random()](struct@BigUInt#method.random)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than)
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use (N * 8)-bit long random number for cryptographic
    /// purpose, you are highly recommended to use the method [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use random odd number for cryptographic purpose, you
    /// are highly recommended to use this method [random_odd()](struct@BigUInt#method.random_odd)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than),
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use random number less than a certain value for
    /// cryptographic purpose, you are highly recommended to use this method
    /// [random_less_than()](struct@BigUInt#method.random_less_than)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than),
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use random odd number less than a certain value for
    /// cryptographic purpose, you are highly recommended to use this method
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use (N * 8)-bit long random odd number for
    /// cryptographic purpose, you are highly recommended to use this method
    /// [random_odd_with_MSB_set()](struct@BigUInt#method.random_odd_with_MSB_set)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd_less_than](struct@BigUInt#method.random_odd_less_than), and
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set).
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let num = u1024::random_with_MSB_set();
    /// println!("Random Number = {}", u1024::random());
    /// println!("1024-bit Random Number = {}", num);
    /// assert!(num > u1024::submax(1023));
    /// ```
    ///
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    fn random_with_MSB_set() -> Self
    {
        let mut r = Self::random();
        r.set_MSB();
        r
    }

    // fn random_odd_with_MSB_set() -> Self
    /// Constucts a new `BigUInt<T, N>`-type object which has the random odd
    /// value with MSB (Most Significant Bit) is set.
    /// 
    /// # Output
    /// The random number that this method random_odd_with_MSB_set() returns is
    /// a random odd number whose range is from !(BigUInt::max() >> 1) + 1 up to
    /// BigUInt::max() inclusively.
    /// 
    /// # Features
    /// This method random_odd_with_MSB_set() generates a random number and then
    /// simply sets its MSB (Most Significant Bit) and LSB (Least Significant
    /// Bit) to be one.
    /// This method basically uses the method randomize() that fills all the
    /// elements of the array number[T; N] in struct BigUInt<T, N> with the
    /// cryptographically secure random numbers by means of
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html))
    /// which is considered to be cryptographically secure.
    /// 
    /// # Cryptographical Security
    /// It is not sure that the random number generated by this method
    /// random_odd_with_MSB_set() can be considered to be cryptographically
    /// secure though it uses the crate [rand](https://docs.rs/rand/latest/rand/index.html)
    /// ([rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)),
    /// The author is not sure that the _extended_ random number generated
    /// in the way explained in the section 'Features' is also
    /// cryptographically secure recursively.
    /// 
    /// # Counterpart Methods
    /// - If you want to be sure to use cryptographically secure pure random
    /// number whose range is from 0 up to BigUInt::max() inclusively, you are
    /// highly encouraged to use the method [random()](struct@BigUInt#method.random)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than)
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use (N * 8)-bit long random number for cryptographic
    /// purpose, you are highly recommended to use the method [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use random odd number for cryptographic purpose, you
    /// are highly recommended to use this method [random_odd()](struct@BigUInt#method.random_odd)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than),
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use random number less than a certain value for
    /// cryptographic purpose, you are highly recommended to use this method
    /// [random_less_than()](struct@BigUInt#method.random_less_than)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than),
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use random odd number less than a certain value for
    /// cryptographic purpose, you are highly recommended to use this method
    /// [random_odd_less_than()](struct@BigUInt#method.random_odd_less_than)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set), and
    /// [random_odd_with_MSB_set](struct@BigUInt#method.random_odd_with_MSB_set).
    /// - If you want to use (N * 8)-bit long random odd number for
    /// cryptographic purpose, you are highly recommended to use this method
    /// [random_odd_with_MSB_set()](struct@BigUInt#method.random_odd_with_MSB_set)
    /// rather than other methods that generate different ramdom numbers such as
    /// [random()](struct@BigUInt#method.random),
    /// [random_odd()](struct@BigUInt#method.random_odd),
    /// [random_less_than()](struct@BigUInt#method.random_less_than),
    /// [random_odd_less_than](struct@BigUInt#method.random_odd_less_than), and
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set).
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let num = u1024::random_odd_with_MSB_set();
    /// println!("Random Number = {}", u1024::random());
    /// println!("1024-bit Random Odd Number = {}", num);
    /// assert!(num > u1024::submax(1023));
    /// assert!(num.is_odd());
    /// ```
    fn random_odd_with_MSB_set() -> Self
    {
        let mut r = Self::random_with_MSB_set();
        r.set_LSB();
        r
    }

    // fn random_prime_using_Miller_Rabin(repetition: usize) -> Self
    /// Constucts a new `BigUInt<T, N>`-type object which represents a random
    /// prime number with MSB (Most Significant Bit) is set.
    /// 
    /// # Output
    /// The random prime number that this method random_prime_Miller_Rabin()
    /// returns is a random prime number whose range is from
    /// !(BigUInt::max() >> 1) + 1 up to BigUInt::max() inclusively.
    /// 
    /// # Features
    /// It uses [Miller Rabin algorithm](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test).
    /// If this test results in composite number, the tested number is surely a
    /// composite number. If this test results in prime number, the probability
    /// that the tested number is not a prime number is 1/4. So, if the test
    /// results in prime number twice, the probability that the tested number
    /// is not a prime number is 1/16 (= 1/4 * 1/4). Therefore, if you test any
    /// number 5 times and they all result in a prime number, it is 99.9% that
    /// the number is a prime number.
    /// This method basically uses the method randomize() that fills all the
    /// elements of the array number[T; N] in struct BigUInt<T, N> with the
    /// cryptographically secure random numbers by means of
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html))
    /// which is considered to be cryptographically secure.
    /// 
    /// # Argument
    /// The argument `repetition` defines how many times it tests whether the
    /// generated random number is prime. Usually, `repetition` is given to be
    /// 5 to have 99.9% accuracy. 
    /// 
    /// # Cryptographical Security
    /// It is not sure that the random number generated by this method
    /// random_prime_Miller_Rabin() can be considered to be cryptographically
    /// secure though it uses the crate [rand](https://docs.rs/rand/latest/rand/index.html)
    /// ([rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// This method random_prime_Miller_Rabin() generates a random number, and
    /// then simply sets its MSB (Most Significant Bit) and LSB (Least
    /// Significant Bit) to be one, and then checks whether the generated random
    /// number is prime number, and then it repeats until it will generate a
    /// prime number.
    /// The author is not sure that the _extended_ random number generated
    /// in the way explained in the section 'Features' is also
    /// cryptographically secure recursively.
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    ///
    /// let num = u256::random_prime_using_Miller_Rabin(5);
    /// println!("Random Prime Number = {}", num);
    /// assert!(num.is_prime_using_Miller_Rabin(5));
    /// ```
    fn random_prime_using_Miller_Rabin(repetition: usize) -> Self
    {
        let mut complete = false;
        let mut res = Self::u8_as_SmallUInt(0);
        while !complete
        {
            res.randomize();
            res.set_MSB();
            res.set_LSB();
            complete = res.is_prime_using_Miller_Rabin(repetition);
        }
        res
    }

    // fn randomize(&mut self)
    /// Make a `SmallUInt<T, N>`-type object to have a random value.
    /// The random number that this method randomize() makes is a pure random
    /// number whose range is from 0 up to SmallUInt::max() inclusively.
    /// 
    /// # Features
    /// This method randomize() fills all the elements of the array number[T; N]
    /// in struct BigUInt<T, N> with the cryptographically secure random numbers
    /// by means of [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html))
    /// which is considered to be cryptographically secure.
    /// 
    /// # Cryptographical Security
    /// It is not sure that the random number generated by this method random()
    /// can be considered cryptographically secure though this method random()
    /// is based on the crate [rand](https://docs.rs/rand/latest/rand/index.html) (especially,
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// The author is not sure that the _extended_ random number generated
    /// in the way explained in the section 'Features' is also
    /// cryptographically secure recursively.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut r = u256::new();
    /// println!("original number = {}", r);
    /// assert_eq!(r, u256::zero());
    /// r.randomize();
    /// println!("random number = {}", r);
    /// assert_ne!(r, u256::zero());
    /// ```
    #[inline] fn randomize(&mut self)   { *self = Self::random(); }

    // fn is_prime_using_Miller_Rabin(&self, repetition: usize) -> bool
    /// Tests a `BigUInt<T, N>`-type object to find whether or not it is a
    /// primne number.
    /// 
    /// # Output
    /// It returns `true` if it is a primne number.
    /// Otherwise, it returns `false`.
    /// 
    /// # Features
    /// It uses [Miller Rabin algorithm](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test).
    /// If this test results in composite number, the tested number is surely a
    /// composite number. If this test results in prime number, the probability
    /// that the tested number is not a prime number is 1/4. So, if the test
    /// results in prime number twice, the probability that the tested number
    /// is not a prime number is 1/16 (= 1/4 * 1/4). Therefore, if you test any
    /// number 5 times and they all result in a prime number, it is 99.9% that
    /// the number is a prime number.
    /// 
    /// # Argument
    /// The argument `repetition` defines how many times it tests whether the
    /// generated random number is prime. Usually, `repetition` is given to be
    /// 5 to have 99.9% accuracy.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, This method may panic
    /// or its behavior may undefined though it may not panic.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let num = u1024::random();
    /// let yes = num.is_prime_using_Miller_Rabin(5);
    /// println!("Is {} a prime number? => {}", num, yes);
    /// if yes  { assert!(yes); }
    /// else    { assert!(!yes); }
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    fn is_prime_using_Miller_Rabin(self, repetition: usize) -> bool;
    fn test_Miller_Rabin(self, a: Self) -> bool;

    fn reverse_bits(self) -> Self;
    fn reverse_bits_assign(&mut self);
    fn rotate_left(self, n: u32) -> Self;
    fn rotate_right(self, n: u32) -> Self;

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
    fn u128_as_SmallUInt(n: u128) -> Self;
    fn u64_as_SmallUInt(n: u64) -> Self;
    fn u32_as_SmallUInt(n: u32) -> Self;
    fn u16_as_SmallUInt(n: u16) -> Self;
    fn u8_as_SmallUInt(n: u8) -> Self;
    fn usize_as_SmallUInt(n: usize) -> Self;
    fn bool_as_SmallUInt(n: bool) -> Self;
    fn num<T: SmallUInt>(n: T) -> Self;

    // pub fn set_zero(&mut self)
    /// Sets `SmallUInt` to be zero.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a = u256::new();
    /// a.set_number(&[1_u16, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    /// println!("a = {}", a);
    /// a.set_zero();
    /// println!("a = {}", a);
    /// assert_eq!(a, u256::zero());
    /// ```
    #[inline] fn set_zero(&mut self)    { *self = Self::zero(); }

    // pub fn is_zero(&self) -> bool
    /// Checks whether `BigUInt` to be zero and returns true if it is
    /// zero and returns false if it is not zero.
    /// 
    /// # Output
    /// It returns true if it is zero. Otherwise, it returns false.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let mut a = u1024::zero();
    /// if a.is_zero()
    ///     { println!("a is Zero"); }
    /// else
    ///     { println!("a is Not Zero"); }
    /// assert!(a.is_zero());
    /// 
    /// a.set_one();
    /// if a.is_zero()
    ///     { println!("a is Zero"); }
    /// else
    ///     { println!("a is Not Zero"); }
    /// assert!(!a.is_zero());
    /// ```
    fn is_zero(&self) -> bool;

    // pub fn set_one(&mut self)
    /// Sets `BigUInt` to be one.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a = u256::new();
    /// a.set_number(&[1_u16, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    /// println!("a = {}", a);
    /// a.set_one();
    /// println!("a = {}", a);
    /// assert_eq!(a, u256::one());
    /// ```
    #[inline] fn set_one(&mut self)     { *self = Self::one(); }

    // pub fn is_one(self) -> bool
    /// Checks whether `BigUInt` to be one and returns true if it is
    /// one, and returns false if it is not one.
    /// 
    /// # Output
    /// It returns `true` if it is one. Otherwise, it returns `false`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = u256::one();
    /// if a.is_one()
    ///     { println!("a is One"); }
    /// else
    ///     { println!("a is Not One"); }
    /// assert!(a.is_one());
    /// 
    /// a.set_max();
    /// if a.is_one()
    ///     { println!("a is One"); }
    /// else
    ///     { println!("a is Not One"); }
    /// assert!(!a.is_one());
    /// ```
    fn is_one(&self) -> bool;

    // pub fn is_zero_or_one(&self) -> bool
    /// Checks whether `BigUInt` to be either zero or one and returns true if it
    /// is either zero or one. Otherwise, it returns false.
    /// 
    /// # Output
    /// It returns true if it is either zero or one. Otherwise, it returns false.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a = u256::zero();
    /// println!("a = {}", a);
    /// if a.is_zero_or_one()
    ///     { println!("a is One or Zero."); }
    /// else
    ///     { println!("a is Neither One nor Zero."); }
    /// assert!(a.is_zero_or_one());
    /// 
    /// a.wrapping_add_assign_uint(1_u8);
    /// println!("a = {}", a);
    /// if a.is_zero_or_one()
    ///     { println!("a is One or Zero."); }
    /// else
    ///     { println!("a is Neither One nor Zero."); }
    /// assert!(a.is_zero_or_one());
    /// 
    /// a.wrapping_add_assign_uint(1_u8);
    /// println!("a = {}", a);
    /// if a.is_zero_or_one()
    ///     { println!("a is One or Zero."); }
    /// else
    ///     { println!("a is Neither One nor Zero."); }
    /// assert!(!a.is_zero_or_one());
    /// ```
    #[inline] fn is_zero_or_one(self) -> bool   { self.is_zero() || self.is_one() }

    // pub fn set_max(&mut self)
    /// Sets `SmallUInt`-type number to be maximum value in which all bits are
    /// set to be `1`.
    /// 
    /// # Examples
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a = u256::new();
    /// println!("a = {}", a);
    /// a.set_max();
    /// println!("a = {}", a);
    /// assert_eq!(a.to_string(), "115792089237316195423570985008687907853269984665640564039457584007913129639935");
    /// ```
    #[inline] fn set_max(&mut self)     { *self = Self::max() }

    // pub fn set_submax(&mut self, size_in_bits: usize)
    /// Sets `BigUInt`-type number to be `size_in_bits`-bit long maximum value
    /// in which all bits are set to be `1`.
    /// 
    /// # Features
    /// This method will make all the `size_in_bits` bits of `number[T;N]` of
    /// `self` from LSB (Least Significant Bit) to be `1` and the rest of the
    /// bits up to MSB (Most Significant Bit) to be `0`.
    /// 
    /// # Examples
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a = u256::new();
    /// a.set_max();
    /// println!("a = {}", a);
    /// assert_eq!(a, u256::max());
    /// 
    /// a.set_submax(200_usize);
    /// println!("a = {}", a);
    /// assert_eq!(a.to_string_with_radix_and_stride(16, 8).unwrap(), "FF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF");
    /// ```
    fn set_submax(&mut self, size_in_bits: usize);

    // pub fn set_halfmax(&mut self)
    /// Sets `BigUInt`-type number to be half long maximum value
    /// in which all bits are set to be `1`.
    /// 
    /// # Features
    /// This method will make all the half lower bits of `number[T;N]` of
    /// `self` from LSB (Least Significant Bit) to be `1` and the rest of the
    /// bits up to MSB (Most Significant Bit) to be `0`.
    /// 
    /// # Examples
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a = u256::new();
    /// println!("a = {}", a);
    /// a.set_halfmax();
    /// println!("a = {}", a);
    /// assert_eq!(a.to_string_with_radix_and_stride(16, 8).unwrap(), "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF");
    /// ```
    #[inline] fn set_halfmax(&mut self) { self.set_submax(self.length_in_bits() >> 1); }

    // pub fn is_max(&self) -> bool
    /// Checks whether or not `BigUInt`-type number to be maximum value.
    /// 
    /// # Output
    /// It returns `true` if it has maxmum number.
    /// Otherwise, it returns `false`.
    /// 
    /// # Examples
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a = u256::max();
    /// println!("Is {} a 256-bit maximun? - {}", a, a.is_max());
    /// assert_eq!(a.is_max(), true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    fn is_max(&self) -> bool;

    // pub fn set_MSB(&mut self)
    /// Sets the MSB (Most Significant Bit) of `BigUInt`-type number with `1`.
    /// 
    /// # Examples
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a = u256::new();
    /// println!("a = {}", a);
    /// a.set_MSB();
    /// println!("a = {}", a);
    /// assert_eq!(a.to_string_with_radix_and_stride(2, 8).unwrap(), "10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    fn set_MSB(&mut self);

    // pub fn set_LSB(&mut self)
    /// Sets the LSB (Least Significant Bit) of `BigUInt`-type number with `1`.
    /// 
    /// # Examples
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a = u256::new();
    /// println!("a = {}", a);
    /// a.set_LSB();
    /// println!("a = {}", a);
    /// assert_eq!(a.to_string_with_radix_and_stride(2, 8).unwrap(), "1");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    fn set_LSB(&mut self);

    // fn generate_check_bits(&mut self, bit_pos: usize) -> Self
    /// Constucts a new `BigUInt<T, N>` which has the value zero and sets only
    /// the bit specified by the argument bit_pos to be 1.
    /// 
    /// # Output
    /// It returns a big unsigned integer `BigUInt<T, N>` whose bit specified
    /// by the argument bit_posvalue is set to be 1.
    /// 
    /// # Bit Position
    /// The bit positon bit_pos is zero-based and should be counted from LSB
    /// (Least Significant Bit) reguardless endian. So, if the bit_pos is `0`,
    /// only LSB is set to be `1` and all the other bits will be set to `0`.
    /// 
    /// # Panics
    /// If the bit positon `bit_pos` is greater than or equal to
    /// `size_of::<T>() * N * 8`, this method will panic.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with_u32;
    /// define_utypes_with_u32!();
    /// 
    /// let a_0 = u256::generate_check_bits(0);
    /// println!("a_0 = {}", a_0.to_string_with_radix_and_stride(2, 10).unwrap());
    /// assert_eq!(a_0.to_string_with_radix_and_stride(2, 10).unwrap(), "1");
    /// 
    /// let a_12 = u256::generate_check_bits(12);
    /// println!("a_12 = {}", a_12.to_string_with_radix_and_stride(2, 10).unwrap());
    /// assert_eq!(a_12.to_string_with_radix_and_stride(2, 10).unwrap(), "100_0000000000");
    /// 
    /// let a_255 = u256::generate_check_bits(255);
    /// println!("a_255 = {}", a_255.to_string_with_radix_and_stride(2, 10).unwrap());
    /// assert_eq!(a_255.to_string_with_radix_and_stride(2, 10).unwrap(), "100000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000");
    /// // It will panic!
    /// // let a_256 = u256::generate_check_bits(256);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    fn generate_check_bits(bit_pos: usize) -> Self;

    fn is_odd(self) -> bool;
    fn is_even(self) -> bool;

    fn size_in_bytes() -> usize;
    fn size_in_bits() -> usize;
    fn length_in_bytes(self) -> usize;
    fn length_in_bits(self) -> usize;
}



macro_rules! random_for_uint_impl {
    (u8) => {
        /// Constucts a new `SmallUInt`-type object which has the random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.any)
        #[inline] fn any() -> Self      { thread_rng().gen::<u8>() }

        /// Make a `SmallUInt`-type object to have a random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.random)
        #[inline] fn random() -> Self   { OsRng.next_u32() as u8 }
    };
    (u16) => {
        /// Constucts a new `SmallUInt`-type object which has the random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.any)
        #[inline] fn any() -> Self      { thread_rng().gen::<u16>() }

        /// Make a `SmallUInt`-type object to have a random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.random)
        #[inline] fn random() -> Self   { OsRng.next_u32() as u16 }
    };
    (u32) => {
        /// Constucts a new `SmallUInt`-type object which has the random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.any)
        #[inline] fn any() -> Self      { thread_rng().gen::<u32>() }

        /// Make a `SmallUInt`-type object to have a random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.random)
        #[inline] fn random() -> Self   { OsRng.next_u32() }
    };
    (u64) => {
        /// Constucts a new `SmallUInt`-type object which has the random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.any)
        #[inline] fn any() -> Self      { thread_rng().gen::<u64>() }

        /// Make a `SmallUInt`-type object to have a random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.random)
        #[inline] fn random() -> Self   { OsRng.next_u64() }
    };
    (u128) => {
        /// Constucts a new `SmallUInt`-type object which has the random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.any)
        #[inline] fn any() -> Self      { thread_rng().gen::<u128>() }

        /// Make a `SmallUInt`-type object to have a random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.random)
        fn random() -> Self
        {
            let mut common = LongerUnion::new();
            common.set_ulong_(0, OsRng.next_u64());
            common.set_ulong_(1, OsRng.next_u64());
            common.get()
        }
    };
    (usize) => {
        /// Constucts a new `SmallUInt`-type object which has the random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.any)
        #[inline] fn any() -> Self      { thread_rng().gen::<usize>() }

        /// Make a `SmallUInt`-type object to have a random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.random)
        #[cfg(target_pointer_width = "8")]
        #[inline] fn random() -> Self   { OsRng.next_u32() as usize }

        /// Make a `SmallUInt`-type object to have a random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.random)
        #[cfg(target_pointer_width = "16")]
        #[inline] fn random() -> Self   { OsRng.next_u32() as usize }

        /// Make a `SmallUInt`-type object to have a random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.random)
        #[cfg(target_pointer_width = "32")]
        #[inline] fn random() -> Self   { OsRng.next_u32() as usize }
        
        /// Make a `SmallUInt`-type object to have a random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.random)
        #[cfg(target_pointer_width = "64")]
        #[inline] fn random() -> Self   { OsRng.next_u64() as usize }
        
        /// Make a `SmallUInt`-type object to have a random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.random)
        #[cfg(target_pointer_width = "128")]
        #[inline] fn random() -> Self
        {
            let mut common = LongerUnion::new();
            common.set_ulong_(0, OsRng.next_u64());
            common.set_ulong_(1, OsRng.next_u64());
            common.get()
        }
    };
}

macro_rules! SmallUInt_methods_for_uint_impl {
    ($f:ty) => {
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
            if (rhs == 0) || (self == 0)
                { return (0, 0); }

            let mut low: Self = 0;
            let mut high: Self = 0;
            let adder = self;
            let mut bit_check: Self = 1 << (Self::size_in_bits() - 1 - rhs.leading_zeros() as usize);

            while bit_check != 0
            {
                low <<= 1;
                high <<= 1;
                if bit_check & rhs != 0
                {
                    let old = low;
                    low += adder;
                    if low < old
                        { high += 1; }
                }
                bit_check >>= 1;
            }
            let mut c = false;
            (low, c) = low.overflowing_add(carry);
            if c
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
            let adder = self;
            let mut bit_check: Self = 1 << (Self::size_in_bits() - 1 - rhs.leading_zeros() as usize);

            while bit_check != 0
            {
                low <<= 1;
                high <<= 1;
                if bit_check & rhs != 0
                {
                    let old = low;
                    low += adder;
                    if low < old
                        { high += 1; }
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
            let mut mrhs = rhs.wrapping_rem(modulo);
            let mut mlhs = self.wrapping_rem(modulo);
            let mut res = Self::zero();
            while mrhs > Self::zero()
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


        /// Computes self.pow(exp), wrapping around at the boundary of the type.
        /// [Read more in detail](trait@SmallUInt#tymethod.wrapping_pow)
        #[inline] fn wrapping_pow(self, exp: u32) -> Self               { self.wrapping_pow(exp) }

        /// Raises self to the power of exp, using exponentiation by squaring.
        /// [Read more in detail](trait@SmallUInt#tymethod.overflowing_pow)
        #[inline] fn overflowing_pow(self, exp: u32) -> (Self, bool)    { self.overflowing_pow(exp) }

        /// Computes self.pow(exp), returning None if overflow occurred.
        /// [Read more in detail](trait@SmallUInt#tymethod.checked_pow)
        #[inline] fn checked_pow(self, exp: u32) -> Option<Self>        { self.checked_pow(exp) }

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
            let mut adder = Self::zero();
            let mut highest = (Self::size_in_bits() - self.leading_zeros() as usize) >> 1;
            let mut high = highest;
            let mut low = 0;
            let mut mid = (high + low) >> 1;
            let mut res = Self::zero();
            let mut sum = Self::zero();
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
                        adder = Self::generate_check_bits(mid);
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
            let mut adder = Self::zero();
            let mut highest = (Self::size_in_bits() - self.leading_zeros() as usize) / (exp as usize);
            let mut high = highest;
            let mut low = 0;
            let mut mid = (high + low) >> 1;
            let mut res = Self::zero();
            let mut sum = Self::zero();
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
                        adder = Self::generate_check_bits(mid);
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

/***** METHODS FOR GENERATING RANDOM PRIME NUMBERS *****/

        /// Tests a `SmallUInt`-type object to find whether or not it is a
        /// prime number.
        /// [Read more in detail](trait@SmallUInt#tymethod.is_prime_using_Miller_Rabin)
        fn is_prime_using_Miller_Rabin(self, repetition: usize) -> bool
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
                    if !self.test_Miller_Rabin(a as Self)
                        { return false; }
                }
            }
            else if self <= u64::MAX as Self
            {
                let a_list = [2_u64, 325, 9375, 28178, 450775, 9780504, 1795265022];
                for a in a_list
                {
                    if !self.test_Miller_Rabin(a as Self)
                        { return false; }
                }
            }

            let a_list = [2_u64, 7, 61, 325, 9375, 28178, 450775, 9780504, 1795265022];
            let len = a_list.len();
            let common = if len < repetition { len } else { repetition };
            let mut i = 0;
            while i < common
            {
                if !self.test_Miller_Rabin(a_list[i] as Self)
                    { return false; }
                i += 1;
            }

            let mut a = a_list[len-1] + 1;
            for _ in i..repetition
            {
                if !self.test_Miller_Rabin(a as Self)
                    { return false; }
                a += 1;
            }
            true
        }
        /// Performs Millar Rabin method with a number less than `self`.
        /// [Read more in detail](trait@SmallUInt#tymethod.test_Miller_Rabin)
        fn test_Miller_Rabin(self, a: Self) -> bool
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
        #[inline] fn zero() -> Self             { 0 }
        #[inline] fn one() -> Self              { 1 }
        #[inline] fn max() -> Self              { Self::MAX }
        #[inline] fn min() -> Self              { Self::MIN }
        #[inline] fn u128_as_SmallUInt(n: u128) -> Self  { n as Self }
        #[inline] fn u64_as_SmallUInt(n: u64) -> Self    { n as Self }
        #[inline] fn u32_as_SmallUInt(n: u32) -> Self    { n as Self }
        #[inline] fn u16_as_SmallUInt(n: u16) -> Self    { n as Self }
        #[inline] fn u8_as_SmallUInt(n: u8) -> Self      { n as Self }
        #[inline] fn usize_as_SmallUInt(n: usize) -> Self    { n as Self }
        #[inline] fn bool_as_SmallUInt(n: bool) -> Self    { n as Self }

        #[inline]
        fn num<T: SmallUInt>(n: T) -> Self
        {
            match size_of::<T>()
            {
                1 => { return Self::u8_as_SmallUInt(n.into_u8()); },
                2 => { return Self::u16_as_SmallUInt(n.into_u16()); },
                4 => { return Self::u32_as_SmallUInt(n.into_u32()); },
                8 => { return Self::u64_as_SmallUInt(n.into_u64()); },
                _ => { return Self::u128_as_SmallUInt(n.into_u128()); },
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
        /// [Read more in detail](trait@SmallUInt#tymethod.set_MSB)
        #[inline] fn set_MSB(&mut self)     { *self |= !(Self::MAX >> 1); }

        /// Sets the LSB (Least Significant Bit) of `SmallUInt`-type
        /// number with `1`.
        /// [Read more in detail](trait@SmallUInt#tymethod.set_LSB)
        #[inline] fn set_LSB(&mut self)     { *self |= 1; }

        /// Constucts a new `SmallUInt` which has the value zero and sets only
        /// the bit specified by the argument bit_pos to be 1.
        /// [Read more in detail](trait@SmallUInt#tymethod.generate_check_bits)
        #[inline] fn generate_check_bits(bit_pos: usize) -> Self    { Self::one() << bit_pos }

        #[inline] fn is_odd(self) -> bool       { (self & 1) != 0 }
        #[inline] fn is_even(self) -> bool      { !self.is_odd() }

        #[inline] fn size_in_bytes() -> usize   { size_of::<Self>() }
        #[inline] fn size_in_bits() -> usize    { size_of::<Self>() * 8 }
        #[inline] fn length_in_bytes(self) -> usize    { size_of_val(&self) }
        #[inline] fn length_in_bits(self) -> usize     { size_of_val(&self) * 8 }
    }
}



impl SmallUInt for u8
{
    SmallUInt_methods_for_uint_impl! { u8 }
    random_for_uint_impl! { u8 }
}

impl SmallUInt for u16
{
    SmallUInt_methods_for_uint_impl! { u16 }
    random_for_uint_impl! { u16 }
}

impl SmallUInt for u32
{
    SmallUInt_methods_for_uint_impl! { u32 }
    random_for_uint_impl! { u32 }
}

impl SmallUInt for u64
{
    SmallUInt_methods_for_uint_impl! { u64 }
    random_for_uint_impl! { u64 }
}

impl SmallUInt for u128
{
    SmallUInt_methods_for_uint_impl! { u128 }
    random_for_uint_impl! { u128 }
}

impl SmallUInt for usize
{
    SmallUInt_methods_for_uint_impl! { usize }
    random_for_uint_impl! { usize }
}



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