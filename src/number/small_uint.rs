// Copyright 2023, 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module that contains generic types of primitive unsigned integral
//! data types used in a lot of modules of the crate Cryptocol.
//! __The trait SmallUInt is meaningful when you use it in generic context.__

// #![warn(missing_docs)]
// #![warn(missing_doc_code_examples)]
#![allow(missing_docs)]
#![allow(missing_doc_code_examples)]


/// Trait SmallUInt is for generic type of primitive unsigned integer data types
/// for all modules of the crate Cryptocol.
/// __The trait SmallUInt is meaningful when you use it in generic context.
/// Otherwise, it is pretty hard to imagine its usability.__
/// In order to use this trait, you have to import (use)
/// `cryptocol::number::SmallUInt`.
///  
/// Here, the generic type of primitive unsigned integral data types includes:
/// `u8`, `u16`, `u32`, `u64`, `u128` and `usize`. In order to use this trait,
/// you have to import (use) `cryptocol::number::SmallUInt`.
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
    /// Calculates `self` + `rhs` + `carry`,
    /// wrapping around at the boundary of the type.
    /// 
    /// # Features
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
    /// - This allows chaining together multiple additions to create a wider
    /// addition, and can be useful for big integer type addition.
    /// - This can be thought of as a 8-bit “full adder”, in the electronics
    /// sense.
    /// - If the input carry is false, this method is equivalent to
    /// `overflowing_add()`.
    /// 
    /// # Outputs
    /// It returns a tuple containing the sum and the output `carry`.
    /// It performs “ternary addition” of two integer operands and a
    /// carry-in bit, and returns an output integer and a carry-out bit.
    /// 
    /// # Example 1 for u8
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     // a_u16: u16 === (a_high_u8, a_low_u8) == (100_u8, 101_u8) == 25701_u16
    ///     let a_high_u8 = 100_u8;
    ///     let a_low_u8 = 101_u8;
    ///     // b_u16: u16 === (b_high_u8, b_low_u8) == (100_u8, 200_u8) == 25800_u16
    ///     let b_high_u8 = 100_u8;
    ///     let b_low_u8 = 200_u8;
    ///     
    ///     // (100_u8, 101_u8) + (100_u8, 200_u8) == 25701_u16 + 25800_u16 == 51501_u16
    ///     //   25701_u16 == (100_u8, 101_u8)
    ///     // + 25800_u16 == (100_u8, 200_u8)
    ///     // -------------------------------
    ///     //   51501_u16 == (201_u8,  45_u8)
    ///     
    ///     // c: u16 === (c_high, c_low)
    ///     let (c_low_u8, c_high_u8, carry) = small_uint_carrying_add_func(a_low_u8, a_high_u8, b_low_u8, b_high_u8);
    ///     println!("{}-{}, {}", c_high_u8, c_low_u8, carry);
    ///     assert_eq!(c_high_u8, 201);
    ///     assert_eq!(c_low_u8, 45);
    ///     assert_eq!(carry, false);
    ///     
    ///     //  (201_u8,  45_u8) + (100_u8, 200_u8) == 25701_u16 + 25800_u16 == 51501_u16
    ///     //   25701_u16 == (100_u8, 101_u8)
    ///     // + 25800_u16 == (100_u8, 200_u8)
    ///     // -------------------------------
    ///     //   11765_u16 == ( 45_u8, 245_u8)
    ///     let (d_low_u8, d_high_u8, carry) = small_uint_carrying_add_func(c_low_u8, c_high_u8, b_low_u8, b_high_u8);
    ///     println!("{}-{}, {}", d_high_u8, d_low_u8, carry);
    ///     assert_eq!(d_high_u8, 45_u8);
    ///     assert_eq!(d_low_u8, 245_u8);
    ///     assert_eq!(carry, true);
    /// }
    /// 
    /// fn add_long<T: SmallUInt>(lhs_low: T, lhs_high: T, rhs_low: T, rhs_high: T) -> (T, T, bool)
    /// {
    ///     let mut carry = false;
    ///     let mut sum_high: T;
    ///     let mut sum_low: T;
    ///     (sum_low, carry) = lhs_low.carrying_add(rhs_low, carry);
    ///     (sum_high, carry) = lhs_high.carrying_add(rhs_high, carry);
    ///     (sum_low, sum_high, carry)
    /// }
    /// ```
    /// 
    /// # Example 2 for u128
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     //  4201016837757989640311993609423984479246482890531986660185 == (12345678901234567890_u128, 6789012345678912345_u128)
    ///     //+                 419908440780438063913804265570801972943493 == (                1234_u128,                6789_u128)
    ///     //---------------------------------------------------------------------------------------------------------------------
    ///     //  4201016837757990060220434389862048393050748461333959603678 == (12345678901234569124_u128, 6789012345678919134_u128)
    ///     let (a_low_u128, a_high_u128, carry) = small_uint_carrying_add_func(6789012345678912345_u128, 12345678901234567890_u128, 6789_u128, 1234_u128);
    ///     println!("{}-{}, {}", a_high_u128, a_low_u128, carry);
    ///     assert_eq!(a_high_u128, 12345678901234569124_u128);
    ///     assert_eq!(a_low_u128, 6789012345678919134_u128);
    ///     assert_eq!(carry, false);
    ///     
    ///     //  308778904632843187796189293356501087608549893209439890708590319850715068122315 == (226854911280625642308916404954512140970_u128, 56789012345678912345678901234567890123_u128)
    ///     //+  57896044618658097711785492504343953926307055644800578124155540853313808954190 == (170141183460469231731687303715884105727_u128, 12345678901234567890123456789012345678_u128)
    ///     //--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
    ///     //   19298681539552699237261830834781317975046994857318776714373108680289488156697 == ( 56713727820156410577229101238628035241_u128, 69134691246913480235802358023580235801_u128)
    ///     let (b_low_u128, b_high_u128, carry) = small_uint_carrying_add_func(56789012345678912345678901234567890123_u128, 226854911280625642308916404954512140970_u128, 12345678901234567890123456789012345678_u128, 170141183460469231731687303715884105727_u128);
    ///     println!("{}-{}, {}", b_high_u128, b_low_u128, carry);
    ///     assert_eq!(b_high_u128, 56713727820156410577229101238628035241_u128);
    ///     assert_eq!(b_low_u128, 69134691246913480235802358023580235801_u128);
    ///     assert_eq!(carry, true);
    /// }
    /// 
    /// fn add_long<T: SmallUInt>(lhs_low: T, lhs_high: T, rhs_low: T, rhs_high: T) -> (T, T, bool)
    /// {
    ///     let mut carry = false;
    ///     let mut sum_high: T;
    ///     let mut sum_low: T;
    ///     (sum_low, carry) = lhs_low.carrying_add(rhs_low, carry);
    ///     (sum_high, carry) = lhs_high.carrying_add(rhs_high, carry);
    ///     (sum_low, sum_high, carry)
    /// }
    /// ```
    /// You can use the above generic function `func<>()` for all
    /// SmallUInt-supported data types in a same scope.
    /// Look into the following example.
    /// 
    /// # Collective Example
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     // a_u16: u16 === (a_high_u8, a_low_u8) == (100_u8, 101_u8) == 25701_u16
    ///     let a_high_u8 = 100_u8;
    ///     let a_low_u8 = 101_u8;
    ///     // b_u16: u16 === (b_high_u8, b_low_u8) == (100_u8, 200_u8) == 25800_u16
    ///     let b_high_u8 = 100_u8;
    ///     let b_low_u8 = 200_u8;
    ///     
    ///     // (100_u8, 101_u8) + (100_u8, 200_u8) == 25701_u16 + 25800_u16 == 51501_u16
    ///     //   25701_u16 == (100_u8, 101_u8)
    ///     // + 25800_u16 == (100_u8, 200_u8)
    ///     // -------------------------------
    ///     //   51501_u16 == (201_u8,  45_u8)
    ///     
    ///     // c: u16 === (c_high, c_low)
    ///     let (c_low_u8, c_high_u8, carry) = small_uint_carrying_add_func(a_low_u8, a_high_u8, b_low_u8, b_high_u8);
    ///     println!("{}-{}, {}", c_high_u8, c_low_u8, carry);
    ///     assert_eq!(c_high_u8, 201);
    ///     assert_eq!(c_low_u8, 45);
    ///     assert_eq!(carry, false);
    ///     
    ///     //  (201_u8,  45_u8) + (100_u8, 200_u8) == 25701_u16 + 25800_u16 == 51501_u16
    ///     //   25701_u16 == (100_u8, 101_u8)
    ///     // + 25800_u16 == (100_u8, 200_u8)
    ///     // -------------------------------
    ///     //   11765_u16 == ( 45_u8, 245_u8)
    ///     let (d_low_u8, d_high_u8, carry) = small_uint_carrying_add_func(c_low_u8, c_high_u8, b_low_u8, b_high_u8);
    ///     println!("{}-{}, {}", d_high_u8, d_low_u8, carry);
    ///     assert_eq!(d_high_u8, 45_u8);
    ///     assert_eq!(d_low_u8, 245_u8);
    ///     assert_eq!(carry, true);
    ///     
    ///     //  4201016837757989640311993609423984479246482890531986660185 == (12345678901234567890_u128, 6789012345678912345_u128)
    ///     //+                 419908440780438063913804265570801972943493 == (                1234_u128,                6789_u128)
    ///     //---------------------------------------------------------------------------------------------------------------------
    ///     //  4201016837757990060220434389862048393050748461333959603678 == (12345678901234569124_u128, 6789012345678919134_u128)
    ///     let (a_low_u128, a_high_u128, carry) = small_uint_carrying_add_func(6789012345678912345_u128, 12345678901234567890_u128, 6789_u128, 1234_u128);
    ///     println!("{}-{}, {}", a_high_u128, a_low_u128, carry);
    ///     assert_eq!(a_high_u128, 12345678901234569124_u128);
    ///     assert_eq!(a_low_u128, 6789012345678919134_u128);
    ///     assert_eq!(carry, false);
    ///     
    ///     //  308778904632843187796189293356501087608549893209439890708590319850715068122315 == (226854911280625642308916404954512140970_u128, 56789012345678912345678901234567890123_u128)
    ///     //+  57896044618658097711785492504343953926307055644800578124155540853313808954190 == (170141183460469231731687303715884105727_u128, 12345678901234567890123456789012345678_u128)
    ///     //--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
    ///     //   19298681539552699237261830834781317975046994857318776714373108680289488156697 == ( 56713727820156410577229101238628035241_u128, 69134691246913480235802358023580235801_u128)
    ///     let (b_low_u128, b_high_u128, carry) = small_uint_carrying_add_func(56789012345678912345678901234567890123_u128, 226854911280625642308916404954512140970_u128, 12345678901234567890123456789012345678_u128, 170141183460469231731687303715884105727_u128);
    ///     println!("{}-{}, {}", b_high_u128, b_low_u128, carry);
    ///     assert_eq!(b_high_u128, 56713727820156410577229101238628035241_u128);
    ///     assert_eq!(b_low_u128, 69134691246913480235802358023580235801_u128);
    ///     assert_eq!(carry, true);
    /// }
    /// 
    /// fn add_long<T: SmallUInt>(lhs_low: T, lhs_high: T, rhs_low: T, rhs_high: T) -> (T, T, bool)
    /// {
    ///     let mut carry = false;
    ///     let mut sum_high: T;
    ///     let mut sum_low: T;
    ///     (sum_low, carry) = lhs_low.carrying_add(rhs_low, carry);
    ///     (sum_high, carry) = lhs_high.carrying_add(rhs_high, carry);
    ///     (sum_low, sum_high, carry)
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

    // fn wrapping_add(self, rhs: Self) -> Self;
    /// Computes `self` + `rhs`, wrapping around at the boundary of the type.
    /// 
    /// # Features
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
    /// It adds two numbers with wrapping (modular) addition.
    /// 
    /// # Output
    /// It returns the `self` + `rhs` in the type of `Self`.
    /// 
    /// # Example 1 for u8
    /// ```
    /// use cryptocol::number::SmallUInt;
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
    /// # Example 2 for u16
    /// ```
    /// use cryptocol::number::SmallUInt;
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
    /// # Example 3 for u32
    /// ```
    /// use cryptocol::number::SmallUInt;
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
    /// # Example 4 for u64
    /// ```
    /// use cryptocol::number::SmallUInt;
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
    /// # Example 5 for u128
    /// ```
    /// use cryptocol::number::SmallUInt;
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
    /// # Example 6 for usize
    /// ```
    /// use cryptocol::number::SmallUInt;
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
    /// You can use the above generic function `func<>()` for all
    /// SmallUInt-supported data types in a same scope.
    /// Look into the following example.
    /// 
    /// # Collective Example
    /// ```
    /// use cryptocol::number::SmallUInt;
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

    // fn overflowing_add(self, rhs: Self) -> (Self, bool);
    /// Calculates `self` + `rhs`, wrapping around at the boundary of the type.
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
    /// # Example 1 for u8
    /// ```
    /// use cryptocol::number::SmallUInt;
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
    /// # Example 2 for u16
    /// ```
    /// use cryptocol::number::SmallUInt;
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
    /// # Example 3 for u32
    /// ```
    /// use cryptocol::number::SmallUInt;
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
    /// # Example 4 for u64
    /// ```
    /// use cryptocol::number::SmallUInt;
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
    /// # Example 5 for u128
    /// ```
    /// use cryptocol::number::SmallUInt;
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
    /// # Example 6 for usize
    /// ```
    /// use cryptocol::number::SmallUInt;
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
    /// You can use the above generic function `func<>()` for all
    /// SmallUInt-supported data types in a same scope.
    /// Look into the following example.
    /// 
    /// # Collective Example
    /// ```
    /// use cryptocol::number::SmallUInt;
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

    // fn checked_add(self, rhs: Self) -> Option<Self>;
    /// Computes `self` + `rhs`.
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
    /// # Example 1 for u8
    /// ```
    /// use cryptocol::number::SmallUInt;
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
    /// # Example 2 for u16
    /// ```
    /// use cryptocol::number::SmallUInt;
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
    /// # Example 3 for u32
    /// ```
    /// use cryptocol::number::SmallUInt;
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
    /// # Example 4 for u64
    /// ```
    /// use cryptocol::number::SmallUInt;
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
    /// # Example 5 for u128
    /// ```
    /// use cryptocol::number::SmallUInt;
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
    /// # Example 6 for usize
    /// ```
    /// use cryptocol::number::SmallUInt;
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
    /// You can use the above generic function `func<>()` for all
    /// SmallUInt-supported data types in a same scope.
    /// Look into the following example.
    /// 
    /// # Collective Example
    /// ```
    /// use cryptocol::number::SmallUInt;
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

    // fn unchecked_add(self, rhs: Self) -> Self;
    /// Computes `self` + `rhs`, assuming overflow cannot occur.
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
    /// It returns `self` + `rhs` in the type `Self` if overflow did not occur.
    /// Otherwise, its behavior is not defined.
    /// 
    /// # Example 1 for u8
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u8 = func(u8::MAX - 55_u8, 55_u8);
    ///     println!("{} + 55 = {}", u8::MAX - 55_u8, a_u8);
    ///     assert_eq!(a_u8, u8::MAX);
    /// 
    ///     // let b_u8 = func(a_u8, 1_u8);     // It will panic
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.unchecked_add(rhs)
    /// }
    /// ```
    /// 
    /// # Example 2 for u16
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u16 = func(u16::MAX - 55_u16, 55_u16);
    ///     println!("{} + 55 = {}", u16::MAX - 55_u16, a_u16);
    ///     assert_eq!(a_u16, u16::MAX);
    /// 
    ///     // let b_u16 = func(a_u16, 1_u16);  // It will panic
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.unchecked_add(rhs)
    /// }
    /// ```
    /// 
    /// # Example 3 for u32
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u32 = func(u32::MAX - 55_u32, 55_u32);
    ///     println!("{} + 55 = {}", u32::MAX - 55_u32, a_u32);
    ///     assert_eq!(a_u32, u32::MAX);
    /// 
    ///     // let b_u32 = func(a_u32, 1_u32);  // It will panic
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.unchecked_add(rhs)
    /// }
    /// ```
    /// 
    /// # Example 4 for u64
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u64 = func(u64::MAX - 55_u64, 55_u64);
    ///     println!("{} + 55 = {}", u64::MAX - 55_u64, a_u64);
    ///     assert_eq!(a_u64, u64::MAX);
    /// 
    ///     // let b_u64 = func(a_u64, 1_u64);  // It will panic
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.unchecked_add(rhs)
    /// }
    /// ```
    /// 
    /// # Example 5 for u128
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u128 = func(u128::MAX - 55_u128, 55_u128);
    ///     println!("{} + 55 = {}", u128::MAX - 55_u128, a_u128);
    ///     assert_eq!(a_u128, u128::MAX);
    /// 
    ///     // let b_u128 = func(a_u128, 1_u128);   // It will panic
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.unchecked_add(rhs)
    /// }
    /// ```
    /// 
    /// # Example 6 for usize
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
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
    /// You can use the above generic function `func<>()` for all
    /// SmallUInt-supported data types in a same scope.
    /// Look into the following example.
    /// 
    /// # Collective Example
    /// ```
    /// use cryptocol::number::SmallUInt;
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

    // fn saturating_add(self, rhs: Self) -> Self;
    /// Computes `self` + `rhs`, saturating at the numeric bounds
    /// instead of overflowing.
    /// 
    /// # Features
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
    /// It adds two numbers with saturating integer addition
    /// 
    /// # Output
    /// It returns the smaller one of `self` + `rhs` and the maxium
    /// of the type of `Self`.
    /// 
    /// # Example 1 for u8
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u8 = func(u8::MAX - 55_u8, 55_u8);
    ///     println!("{} + 55 = {}", u8::MAX - 55_u8, a_u8);
    ///     assert_eq!(a_u8, u8::MAX);
    /// 
    ///     let b_u8 = func(a_u8, 55_u8);
    ///     println!("{} + 55 = {}", a_u8, b_u8);
    ///     assert_eq!(b_u8, u8::MAX);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.saturating_add(rhs)
    /// }    
    /// ```
    /// 
    /// # Example 2 for u16
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u16 = func(u16::MAX - 55_u16, 55_u16);
    ///     println!("{} + 55 = {}", u16::MAX - 55_u16, a_u16);
    ///     assert_eq!(a_u16, u16::MAX);
    /// 
    ///     let b_u16 = func(a_u16, 55_u16);
    ///     println!("{} + 55 = {}", a_u16, b_u16);
    ///     assert_eq!(b_u16, u16::MAX);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.saturating_add(rhs)
    /// }    
    /// ```
    /// 
    /// # Example 3 for u32
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u32 = func(u32::MAX - 55_u32, 55_u32);
    ///     println!("{} + 55 = {}", u32::MAX - 55_u32, a_u32);
    ///     assert_eq!(a_u32, u32::MAX);
    /// 
    ///     let b_u32 = func(a_u32, 55_u32);
    ///     println!("{} + 55 = {}", a_u32, b_u32);
    ///     assert_eq!(b_u32, u32::MAX);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.saturating_add(rhs)
    /// }    
    /// ```
    /// 
    /// # Example 4 for u64
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u64 = func(u64::MAX - 55_u64, 55_u64);
    ///     println!("{} + 55 = {}", u64::MAX - 55_u64, a_u64);
    ///     assert_eq!(a_u64, u64::MAX);
    /// 
    ///     let b_u64 = func(a_u64, 55_u64);
    ///     println!("{} + 55 = {}", a_u64, b_u64);
    ///     assert_eq!(b_u64, u64::MAX);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.saturating_add(rhs)
    /// }    
    /// ```
    /// 
    /// # Example 5 for u128
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u128 = func(u128::MAX - 55_u128, 55_u128);
    ///     println!("{} + 55 = {}", u128::MAX - 55_u128, a_u128);
    ///     assert_eq!(a_u128, u128::MAX);
    /// 
    ///     let b_u128 = func(a_u128, 55_u128);
    ///     println!("{} + 55 = {}",a_u128, b_u128);
    ///     assert_eq!(b_u128, u128::MAX);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.saturating_add(rhs)
    /// }    
    /// ```
    /// 
    /// # Example 6 for usize
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
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
    /// # Collectie Example
    /// ```
    /// use cryptocol::number::SmallUInt;
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
    /// # Example 1 for u8
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     println!("small_uint_modular_add()");
    ///     let a_u8 = func(60_u8, 15, 100);
    ///     println!("60 + 55 = {} (mod 100)", a_u8);
    ///     assert_eq!(a_u8, 75);
    /// 
    ///     let b_u8 = func(a_u8, 55, 100);
    ///     println!("{} + 55 = {} (mod 100)", a_u8, b_u8);
    ///     assert_eq!(b_u8, 30);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T, modulo: T) -> T
    /// {
    ///     lhs.modular_add(rhs, modulo)
    /// }
    /// ```
    /// 
    /// # Example 2 for u16
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u16 = func(6000_u16, 1500, 1_0000);
    ///     println!("6000 + 1500 = {} (mod 1_0000)", a_u16);
    ///     assert_eq!(a_u16, 7500);
    /// 
    ///     let b_u16 = func(a_u16, 5500, 1_0000);
    ///     println!("{} + 55 = {} (mod 1_0000)", a_u16, b_u16);
    ///     assert_eq!(b_u16, 3000);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T, modulo: T) -> T
    /// {
    ///     lhs.modular_add(rhs, modulo)
    /// }
    /// ```
    /// 
    /// # Example 3 for u32
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u32 = func(6_0000_0000_u32, 1_5000_0000, 10_0000_0000);
    ///     println!("6_0000_0000 + 1_5000_0000 = {} (mod 10_0000_0000)", a_u32);
    ///     assert_eq!(a_u32, 7_5000_0000);
    /// 
    ///     let b_u32 = func(a_u32, 5_5000_0000, 10_0000_0000);
    ///     println!("{} + 5_5000_0000 = {} (mod 10_0000_0000)", a_u32, b_u32);
    ///     assert_eq!(b_u32, 3_0000_0000);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T, modulo: T) -> T
    /// {
    ///     lhs.modular_add(rhs, modulo)
    /// }
    /// ```
    /// 
    /// # Example 4 for u64
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u64 = func(6_0000_0000_0000_0000_u64, 1_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    ///     println!("6_0000_0000_0000_0000 + 1_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_u64);
    ///     assert_eq!(a_u64, 7_5000_0000_0000_0000);
    /// 
    ///     let b_u64 = func(a_u64, 5_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    ///     println!("{} + 5_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_u64, b_u64);
    ///     assert_eq!(b_u64, 3_0000_0000_0000_0000);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T, modulo: T) -> T
    /// {
    ///     lhs.modular_add(rhs, modulo)
    /// }
    /// ```
    /// 
    /// # Example 5 for u128
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u128 = func(6_0000_0000_0000_0000_0000_0000_0000_0000_u128, 1_5000_0000_0000_0000_0000_0000_0000_0000, 10_0000_0000_0000_0000_0000_0000_0000_0000);
    ///     println!("6_0000_0000_0000_0000_0000_0000_0000_0000 + 1_5000_0000_0000_0000_0000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000_0000_0000_0000_0000)", a_u128);
    ///     assert_eq!(a_u128, 7_5000_0000_0000_0000_0000_0000_0000_0000);
    /// 
    ///     let b_u128 = func(a_u128, 5_5000_0000_0000_0000_0000_0000_0000_0000, 10_0000_0000_0000_0000_0000_0000_0000_0000);
    ///     println!("{} + 5_5000_0000_0000_0000_0000_0000_0000_0000 = {}",a_u128, b_u128);
    ///     assert_eq!(b_u128, 3_0000_0000_0000_0000_0000_0000_0000_0000);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T, modulo: T) -> T
    /// {
    ///     lhs.modular_add(rhs, modulo)
    /// }
    /// ```
    /// 
    /// # Example 6 for usize
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_usize = func(6_0000_0000_0000_0000_usize, 1_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    ///     println!("6_0000_0000_0000_0000 + 1_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_usize);
    ///     assert_eq!(a_usize, 7_5000_0000_0000_0000);
    /// 
    ///     let b_usize = func(a_usize, 5_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    ///     println!("{} + 5_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_usize, b_usize);
    ///     assert_eq!(b_usize, 3_0000_0000_0000_0000);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T, modulo: T) -> T
    /// {
    ///     lhs.modular_add(rhs, modulo)
    /// }
    /// ```
    /// You can use the above generic function `func<>()` for all
    /// SmallUInt-supported data types in a same scope.
    /// Look into the following example.
    /// 
    /// # Collective Example
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     println!("small_uint_modular_add()");
    ///     let a_u8 = func(60_u8, 15, 100);
    ///     println!("60 + 55 = {} (mod 100)", a_u8);
    ///     assert_eq!(a_u8, 75);
    /// 
    ///     let b_u8 = func(a_u8, 55, 100);
    ///     println!("{} + 55 = {} (mod 100)", a_u8, b_u8);
    ///     assert_eq!(b_u8, 30);
    /// 
    ///     let a_u16 = func(6000_u16, 1500, 1_0000);
    ///     println!("6000 + 1500 = {} (mod 1_0000)", a_u16);
    ///     assert_eq!(a_u16, 7500);
    /// 
    ///     let b_u16 = func(a_u16, 5500, 1_0000);
    ///     println!("{} + 55 = {} (mod 1_0000)", a_u16, b_u16);
    ///     assert_eq!(b_u16, 3000);
    /// 
    ///     let a_u32 = func(6_0000_0000_u32, 1_5000_0000, 10_0000_0000);
    ///     println!("6_0000_0000 + 1_5000_0000 = {} (mod 10_0000_0000)", a_u32);
    ///     assert_eq!(a_u32, 7_5000_0000);
    /// 
    ///     let b_u32 = func(a_u32, 5_5000_0000, 10_0000_0000);
    ///     println!("{} + 5_5000_0000 = {} (mod 10_0000_0000)", a_u32, b_u32);
    ///     assert_eq!(b_u32, 3_0000_0000);
    /// 
    ///     let a_u64 = func(6_0000_0000_0000_0000_u64, 1_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    ///     println!("6_0000_0000_0000_0000 + 1_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_u64);
    ///     assert_eq!(a_u64, 7_5000_0000_0000_0000);
    /// 
    ///     let b_u64 = func(a_u64, 5_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    ///     println!("{} + 5_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_u64, b_u64);
    ///     assert_eq!(b_u64, 3_0000_0000_0000_0000);
    /// 
    ///     let a_u128 = func(6_0000_0000_0000_0000_0000_0000_0000_0000_u128, 1_5000_0000_0000_0000_0000_0000_0000_0000, 10_0000_0000_0000_0000_0000_0000_0000_0000);
    ///     println!("6_0000_0000_0000_0000_0000_0000_0000_0000 + 1_5000_0000_0000_0000_0000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000_0000_0000_0000_0000)", a_u128);
    ///     assert_eq!(a_u128, 7_5000_0000_0000_0000_0000_0000_0000_0000);
    /// 
    ///     let b_u128 = func(a_u128, 5_5000_0000_0000_0000_0000_0000_0000_0000, 10_0000_0000_0000_0000_0000_0000_0000_0000);
    ///     println!("{} + 5_5000_0000_0000_0000_0000_0000_0000_0000 = {}",a_u128, b_u128);
    ///     assert_eq!(b_u128, 3_0000_0000_0000_0000_0000_0000_0000_0000);
    /// 
    ///     let a_usize = func(6_0000_0000_0000_0000_usize, 1_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    ///     println!("6_0000_0000_0000_0000 + 1_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_usize);
    ///     assert_eq!(a_usize, 7_5000_0000_0000_0000);
    /// 
    ///     let b_usize = func(a_usize, 5_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    ///     println!("{} + 5_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_usize, b_usize);
    ///     assert_eq!(b_usize, 3_0000_0000_0000_0000);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T, modulo: T) -> T
    /// {
    ///     lhs.modular_add(rhs, modulo)
    /// }
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    fn modular_add(self, rhs: Self, modulo: Self) -> Self;


    /***** SUBTRACTION *****/

    // fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool);
    /// Calculates `self` − `rhs` − `borrow`,
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
    /// # Example 1 for u8
    /// ```
    /// use cryptocol::number::SmallUInt;
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
    ///     (c_low_u8, c_high_u8, borrow) = sub_long(a_low_u8, a_high_u8, b_low_u8, b_high_u8);
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
    ///     (d_low_u8, d_high_u8, borrow) = sub_long(c_low_u8, c_high_u8, b_low_u8, b_high_u8);
    ///     println!("{}-{}, {}", d_high_u8, d_low_u8, borrow);
    ///     assert_eq!(d_high_u8, 155_u8);
    ///     assert_eq!(d_low_u8, 254_u8);
    ///     assert_eq!(borrow, true);
    /// }
    /// 
    /// fn sub_long<T: SmallUInt>(lhs_low: T, lhs_high: T, rhs_low: T, rhs_high: T) -> (T, T, bool)
    /// {
    ///     let mut borrow = false;
    ///     let mut dif_high: T;
    ///     let mut dif_low: T;
    ///     (sum_low, borrow) = lhs_low.borrowing_sub(rhs_low, borrow);
    ///     (dif_high, borrow) = lhs_high.borrowing_sub(rhs_high, borrow);
    ///     (dif_low, dif_high, borrow)
    /// }
    /// ```
    /// 
    /// # Example 2 for u128
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_high_u128: u128;
    ///     let a_low_u128: u128;
    ///     //   4201016837757989640311993609423984479246482890531986660185 == (12345678901234567890_u128, 6789012345678912345_u128)
    ///     // -                 419908440780438063913804265570801972943493 == (                1234_u128,                6789_u128)
    ///     // ---------------------------------------------------------------------------------------------------------------------
    ///     //   4201016837757989220403552828985920565442217319730013716692 == (12345678901234566656_u128, 6789012345678905556_u128)
    ///     (a_low_u128, a_high_u128, borrow) = sub_long(6789012345678912345_u128, 12345678901234567890_u128, 6789_u128, 1234_u128);
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
    ///     (b_low_u128, b_high_u128, borrow) = sub_long(12345678901234567890123456789012345678_u128, 170141183460469231731687303715884105727_u128, 56789012345678912345678901234567890123_u128, 226854911280625642308916404954512140970_u128);
    ///     println!("{}-{}, {}", b_high_u128, b_low_u128, carry);
    ///     assert_eq!(b_high_u128, 283568639100782052886145506193140176212_u128);
    ///     assert_eq!(b_low_u128, 295839033476494119007819162986212667011_u128);
    ///     assert_eq!(borrow, true);
    /// }
    /// 
    /// fn sub_long<T: SmallUInt>(lhs_low: T, lhs_high: T, rhs_low: T, rhs_high: T) -> (T, T, bool)
    /// {
    ///     let mut borrow = false;
    ///     let mut dif_high: T;
    ///     let mut dif_low: T;
    ///     (sum_low, borrow) = lhs_low.borrowing_sub(rhs_low, borrow);
    ///     (dif_high, borrow) = lhs_high.borrowing_sub(rhs_high, borrow);
    ///     (dif_low, dif_high, borrow)
    /// }
    /// ```
    /// You can use the above generic function `func<>()` for all
    /// SmallUInt-supported data types in a same scope.
    /// Look into the following example.
    /// 
    /// # Collective Example
    /// ```
    /// use cryptocol::number::SmallUInt;
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
    ///     (c_low_u8, c_high_u8, borrow) = sub_long(a_low_u8, a_high_u8, b_low_u8, b_high_u8);
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
    ///     (d_low_u8, d_high_u8, borrow) = sub_long(c_low_u8, c_high_u8, b_low_u8, b_high_u8);
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
    ///     (a_low_u128, a_high_u128, borrow) = sub_long(6789012345678912345_u128, 12345678901234567890_u128, 6789_u128, 1234_u128);
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
    ///     (b_low_u128, b_high_u128, borrow) = sub_long(12345678901234567890123456789012345678_u128, 170141183460469231731687303715884105727_u128, 56789012345678912345678901234567890123_u128, 226854911280625642308916404954512140970_u128);
    ///     println!("{}-{}, {}", b_high_u128, b_low_u128, carry);
    ///     assert_eq!(b_high_u128, 283568639100782052886145506193140176212_u128);
    ///     assert_eq!(b_low_u128, 295839033476494119007819162986212667011_u128);
    ///     assert_eq!(borrow, true);
    /// }
    /// 
    /// fn sub_long<T: SmallUInt>(lhs_low: T, lhs_high: T, rhs_low: T, rhs_high: T) -> (T, T, bool)
    /// {
    ///     let mut borrow = false;
    ///     let mut dif_high: T;
    ///     let mut dif_low: T;
    ///     (dif_low, borrow) = lhs_low.borrowing_sub(rhs_low, borrow);
    ///     (dif_high, borrow) = lhs_high.borrowing_sub(rhs_high, borrow);
    ///     (dif_low, dif_high, borrow)
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

    // fn wrapping_sub(self, rhs: Self) -> Self;
    /// Computes `self` - `rhs`, wrapping around at the boundary of the type.
    /// 
    /// # Features
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
    /// It subtracts rhs from self with wrapping (modular) subtraction.
    /// 
    /// # Output
    /// It returns the `self` - `rhs` in the type of `Self`.
    /// 
    /// # Example 1 for u8
    /// ```
    /// use cryptocol::number::SmallUInt;
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
    /// # Example 2 for u16
    /// ```
    /// use cryptocol::number::SmallUInt;
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
    /// # Example 3 for u32
    /// ```
    /// use cryptocol::number::SmallUInt;
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
    /// # Example 4 for u64
    /// ```
    /// use cryptocol::number::SmallUInt;
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
    /// # Example 5 for u128
    /// ```
    /// use cryptocol::number::SmallUInt;
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
    /// # Example 6 for usize
    /// ```
    /// use cryptocol::number::SmallUInt;
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
    /// You can use the above generic function `func<>()` for all
    /// SmallUInt-supported data types in a same scope.
    /// Look into the following example.
    /// 
    /// # Collective Example
    /// ```
    /// use cryptocol::number::SmallUInt;
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

    // fn overflowing_sub(self, rhs: Self) -> (Self, bool);
    /// Calculates `self` - `rhs`, wrapping around at the boundary of the type.
    /// 
    /// # Features
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
    /// It subtracts rhs from self with wrapping (modular) subtraction.
    /// It is the same as the method borrowing_sub() with the imput carry which
    /// is false.
    /// 
    /// # Output
    /// It returns a tuple of the subtraction along with a boolean indicating
    /// whether an arithmetic underflow would occur. If an underflow would
    /// have occurred then the wrapped value is returned.
    /// 
    /// # Example 1 for u8
    /// ```
    /// use cryptocol::number::SmallUInt;
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
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> (T, bool)
    /// {
    ///     lhs.overflowing_sub(rhs)
    /// }
    /// ```
    /// 
    /// # Example 2 for u16
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u16 = func(55_u16, 55_u16);
    ///     println!("55 - 55 = {}\nUnderflow = {}", a_u16.0, a_u16.1);
    ///     assert_eq!(a_u16.0, 0_u16);
    ///     assert_eq!(a_u16.1, false);
    ///  
    ///     let b_u16 = func(a_u16.0, 1_u16);
    ///     println!("{} - 1 = {}\nUnderflow = {}", a_u16.0, b_u16.0, b_u16.1);
    ///     assert_eq!(b_u16.0, u16::MAX);
    ///     assert_eq!(b_u16.1, true);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> (T, bool)
    /// {
    ///     lhs.overflowing_sub(rhs)
    /// }
    /// ```
    /// 
    /// # Example 3 for u32
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u32 = func(55_u32, 55_u32);
    ///     println!("55 - 55 = {}\nUnderflow = {}", a_u32.0, a_u32.1);
    ///     assert_eq!(a_u32.0, 0_u32);
    ///     assert_eq!(a_u32.1, false);
    ///  
    ///     let b_u32 = func(a_u32.0, 1_u32);
    ///     println!("{} - 1 = {}\nUnderflow = {}", a_u32.0, b_u32.0, b_u32.1);
    ///     assert_eq!(b_u32.0, u32::MAX);
    ///     assert_eq!(b_u32.1, true);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> (T, bool)
    /// {
    ///     lhs.overflowing_sub(rhs)
    /// }
    /// ```
    /// 
    /// # Example 4 for u64
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u64 = func(55_u64, 55_u64);
    ///     println!("55 - 55 = {}\nUnderflow = {}", a_u64.0, a_u64.1);
    ///     assert_eq!(a_u64.0, 0_u64);
    ///     assert_eq!(a_u64.1, false);
    ///  
    ///     let b_u64 = func(a_u64.0, 1_u64);
    ///     println!("{} - 1 = {}\nUnderflow = {}", a_u64.0, b_u64.0, b_u64.1);
    ///     assert_eq!(b_u64.0, u64::MAX);
    ///     assert_eq!(b_u64.1, true);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> (T, bool)
    /// {
    ///     lhs.overflowing_sub(rhs)
    /// }
    /// ```
    /// 
    /// # Example 5 for u128
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u128 = func(55_u128, 55_u128);
    ///     println!("55 - 55 = {}\nUnderflow = {}", a_u128.0, a_u128.1);
    ///     assert_eq!(a_u128.0, 0_u128);
    ///     assert_eq!(a_u128.1, false);
    ///  
    ///     let b_u128 = func(a_u128.0, 1_u128);
    ///     println!("{} - 1 = {}\nUnderflow = {}", a_u128.0, b_u128.0, b_u128.1);
    ///     assert_eq!(b_u128.0, u128::MAX);
    ///     assert_eq!(b_u128.1, true);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> (T, bool)
    /// {
    ///     lhs.overflowing_sub(rhs)
    /// }
    /// ```
    /// 
    /// # Example 6 for usize
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
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
    /// You can use the above generic function `func<>()` for all
    /// SmallUInt-supported data types in a same scope.
    /// Look into the following example.
    /// 
    /// # Collective Example
    /// ```
    /// use cryptocol::number::SmallUInt;
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

    // fn checked_sub(self, rhs: Self) -> Option<Self>;
    /// Computes `self` - `rhs`.
    /// 
    /// # Feature
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
    /// 
    /// # Output
    /// It returns `self` - `rhs` in the type `Self` wrapped by `Some`
    /// of enum `Option` if overflow did not occur.
    /// And, it returns `None` if overflow occurred.
    /// 
    /// # Example 1 for u8
    /// ```
    /// use cryptocol::number::SmallUInt;
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
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> Option<T>
    /// {
    ///     lhs.checked_sub(rhs)
    /// }
    /// ```
    /// 
    /// # Example 2 for u16
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
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
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> Option<T>
    /// {
    ///     lhs.checked_sub(rhs)
    /// }
    /// ```
    /// 
    /// # Example 3 for u32
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
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
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> Option<T>
    /// {
    ///     lhs.checked_sub(rhs)
    /// }
    /// ```
    /// 
    /// # Example 4 for u64
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
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
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> Option<T>
    /// {
    ///     lhs.checked_sub(rhs)
    /// }
    /// ```
    /// 
    /// # Example 5 for u128
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
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
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> Option<T>
    /// {
    ///     lhs.checked_sub(rhs)
    /// }
    /// ```
    /// 
    /// # Example 6 for usize
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
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
    /// You can use the above generic function `func<>()` for all
    /// SmallUInt-supported data types in a same scope.
    /// Look into the following example.
    /// 
    /// # Collective Example
    /// ```
    /// use cryptocol::number::SmallUInt;
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

    /// fn unchecked_sub(self, rhs: Self) -> Self;
    /// Computes `self` - `rhs`, assuming overflow cannot occur.
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
    /// It returns `self` - `rhs` in the type `Self` if underflow did not occur.
    /// Otherwise, its behavior is not defined.
    /// 
    /// # Example 1 for u8
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u8 = func(55_u8, 55_u8);
    ///     println!("55 - 55 = {}", a_u8);
    ///     assert_eq!(a_u8, 0_u8);
    /// 
    ///     // It will panic
    ///     // let b_u8 = func(a_u8, 1_u8);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.unchecked_sub(rhs)
    /// }
    /// ```
    /// 
    /// # Example 2 for u16
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u16 = func(55_u16, 55_u16);
    ///     println!("55 - 55 = {}", a_u16);
    ///     assert_eq!(a_u16, 0_u16);
    /// 
    ///     // It will panic
    ///     // let b_u16 = func(a_u16, 1_u16);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.unchecked_sub(rhs)
    /// }
    /// ```
    /// 
    /// # Example 3 for u32
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u32 = func(55_u32, 55_u32);
    ///     println!("55 - 55 = {}", a_u32);
    ///     assert_eq!(a_u32, 0_u32);
    /// 
    ///     // It will panic
    ///     // let b_u32 = func(a_u32, 1_u32);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.unchecked_sub(rhs)
    /// }
    /// ```
    /// 
    /// # Example 4 for u64
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u64 = func(55_u64, 55_u64);
    ///     println!("55 - 55 = {}", a_u64);
    ///     assert_eq!(a_u64, 0_u64);
    /// 
    ///     // It will panic
    ///     // let b_u64 = func(a_u64, 1_u64);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.unchecked_sub(rhs)
    /// }
    /// ```
    /// 
    /// # Example 5 for u128
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u128 = func(55_u128, 55_u128);
    ///     println!("55 - 55 = {}", a_u128);
    ///     assert_eq!(a_u128, 0_u128);
    /// 
    ///     // It will panic
    ///     // let b_u128 = func(a_u128, 1_u128);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.unchecked_sub(rhs)
    /// }
    /// ```
    /// 
    /// # Example 6 for usize
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
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
    /// You can use the above generic function `func<>()` for all
    /// SmallUInt-supported data types in a same scope.
    /// Look into the following example.
    /// 
    /// # Collective Example
    /// ```
    /// use cryptocol::number::SmallUInt;
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

    // fn saturating_sub(self, rhs: Self) -> Self;
    /// Computes `self` - `rhs`, saturating at the numeric bounds
    /// instead of underflowing.
    /// 
    /// # Features
    /// __The trait SmallUInt is meaningful when you use it in generic context.
    /// Otherwise, it is pretty hard to imagine its usability.__
    /// It subtracts rhs from self with saturating integer subtraction.
    /// 
    /// # Output
    /// It returns the bigger one of `self` - `rhs` and the zero
    /// of the type of `Self`.
    /// 
    /// # Example 1 for u8
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u8 = func(55_u8, 50_u8);
    ///     println!("55 - 50 = {}", a_u8);
    ///     assert_eq!(a_u8, 5_u8);
    /// 
    ///     let b_u8 = func(a_u8, 55_u8);
    ///     println!("5 - 55 = {}", b_u8);
    ///     assert_eq!(b_u8, 0_u8);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.saturating_sub(rhs)
    /// }
    /// ```
    /// 
    /// # Example 2 for u16
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u16 = func(55_u16, 50_u16);
    ///     println!("55 - 50 = {}", a_u16);
    ///     assert_eq!(a_u16, 5_u16);
    /// 
    ///     let b_u16 = func(a_u16, 55_u16);
    ///     println!("5 - 55 = {}", b_u16);
    ///     assert_eq!(b_u16, 0_u16);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.saturating_sub(rhs)
    /// }
    /// ```
    /// 
    /// # Example 3 for u32
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u32 = func(55_u32, 50_u32);
    ///     println!("55 - 50 = {}", a_u32);
    ///     assert_eq!(a_u32, 5_u32);
    /// 
    ///     let b_u32 = func(a_u32, 55_u32);
    ///     println!("{} - 55 = {}", a_u32, b_u32);
    ///     assert_eq!(b_u32, 0_u32);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.saturating_sub(rhs)
    /// }
    /// ```
    /// 
    /// # Example 4 for u64
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u64 = func(55_u64, 50_u64);
    ///     println!("55 - 50 = {}", a_u64);
    ///     assert_eq!(a_u64, 5_u64);
    /// 
    ///     let b_u64 = func(a_u64, 55_u64);
    ///     println!("{} - 55 = {}", a_u64, b_u64);
    ///     assert_eq!(b_u64, 0_u64);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.saturating_sub(rhs)
    /// }
    /// ```
    /// 
    /// # Example 5 for u128
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u128 = func(55_u128, 50_u128);
    ///     println!("55 - 50 = {}", a_u128);
    ///     assert_eq!(a_u128, 5_u128);
    /// 
    ///     let b_u128 = func(a_u128, 55_u128);
    ///     println!("{} - 55 = {}", a_u128, b_u128);
    ///     assert_eq!(b_u128, 0_u128);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T) -> T
    /// {
    ///     lhs.saturating_sub(rhs)
    /// }
    /// ```
    /// 
    /// # Example 6 for usize
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
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
    /// You can use the above generic function `func<>()` for all
    /// SmallUInt-supported data types in a same scope.
    /// Look into the following example.
    /// 
    /// # Collective Example
    /// ```
    /// use cryptocol::number::SmallUInt;
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

    // fn abs_diff(self, other: Self) -> Self;
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
    /// use cryptocol::number::SmallUInt;
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
    /// use cryptocol::number::SmallUInt;
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
    /// use cryptocol::number::SmallUInt;
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
    /// use cryptocol::number::SmallUInt;
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
    /// use cryptocol::number::SmallUInt;
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
    /// use cryptocol::number::SmallUInt;
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
    /// You can use the above generic function `func<>()` for all
    /// SmallUInt-supported data types in a same scope.
    /// Look into the following example.
    /// 
    /// # Collective Example
    /// ```
    /// use cryptocol::number::SmallUInt;
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
    /// # Example 1 for u8
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u8 = func(60_u8, 55, 100);
    ///     println!("60 - 55 = {} (mod 100)", a_u8);
    ///     assert_eq!(a_u8, 5);
    ///     
    ///     let b_u8 = func(a_u8, 15, 100);
    ///     println!("{} - 15 = {} (mod 100)", a_u8, b_u8);
    ///     assert_eq!(b_u8, 90);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T, modulo: T) -> T
    /// {
    ///     lhs.modular_sub(rhs, modulo)
    /// }
    /// ```
    /// 
    /// # Example 2 for u16
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u16 = func(6000_u16, 5500, 1_0000);
    ///     println!("6000 - 5500 = {} (mod 1_0000)", a_u16);
    ///     assert_eq!(a_u16, 500);
    ///     
    ///     let b_u16 = func(a_u16, 1500, 1_0000);
    ///     println!("{} - 1500 = {} (mod 1_0000)", a_u16, b_u16);
    ///     assert_eq!(b_u16, 9000);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T, modulo: T) -> T
    /// {
    ///     lhs.modular_sub(rhs, modulo)
    /// }
    /// ```
    /// 
    /// # Example 3 for u32
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u32 = func(6_0000_0000_u32, 5_5000_0000, 10_0000_0000);
    ///     println!("6_0000_0000 - 5_5000_0000 = {} (mod 10_0000_0000)", a_u32);
    ///     assert_eq!(a_u32, 5000_0000);
    ///     
    ///     let b_u32 = func(a_u32, 1_5000_0000, 10_0000_0000);
    ///     println!("{} - 1_5000_0000 = {} (mod 10_0000_0000)", a_u32, b_u32);
    ///     assert_eq!(b_u32, 9_0000_0000);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T, modulo: T) -> T
    /// {
    ///     lhs.modular_sub(rhs, modulo)
    /// }
    /// ```
    /// 
    /// # Example 4 for u64
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u64 = func(6_0000_0000_0000_0000_u64, 5_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    ///     println!("6_0000_0000_0000_0000 - 5_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_u64);
    ///     assert_eq!(a_u64, 5000_0000_0000_0000);
    ///     
    ///     let b_u64 = func(a_u64, 1_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    ///     println!("{} - 1_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_u64, b_u64);
    ///     assert_eq!(b_u64, 9_0000_0000_0000_0000);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T, modulo: T) -> T
    /// {
    ///     lhs.modular_sub(rhs, modulo)
    /// }
    /// ```
    /// 
    /// # Example 5 for u128
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u128 = func(6_0000_0000_0000_0000_0000_0000_0000_0000_u128, 5_5000_0000_0000_0000_0000_0000_0000_0000, 10_0000_0000_0000_0000_0000_0000_0000_0000);
    ///     println!("6_0000_0000_0000_0000_0000_0000_0000_0000 - 5_5000_0000_0000_0000_0000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000_0000_0000_0000_0000)", a_u128);
    ///     assert_eq!(a_u128, 5000_0000_0000_0000_0000_0000_0000_0000);
    ///     
    ///     let b_u128 = func(a_u128, 1_5000_0000_0000_0000_0000_0000_0000_0000, 10_0000_0000_0000_0000_0000_0000_0000_0000);
    ///     println!("{} - 1_5000_0000_0000_0000_0000_0000_0000_0000 = {}",a_u128, b_u128);
    ///     assert_eq!(b_u128, 9_0000_0000_0000_0000_0000_0000_0000_0000);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T, modulo: T) -> T
    /// {
    ///     lhs.modular_sub(rhs, modulo)
    /// }
    /// ```
    /// 
    /// # Example 6 for usize
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_usize = func(6_0000_0000_0000_0000_usize, 5_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    ///     println!("6_0000_0000_0000_0000 - 5_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_usize);
    ///     assert_eq!(a_usize, 5000_0000_0000_0000);
    ///     
    ///     let b_usize = func(a_usize, 1_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    ///     println!("{} - 1_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_usize, b_usize);
    ///     assert_eq!(b_usize, 9_0000_0000_0000_0000);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T, modulo: T) -> T
    /// {
    ///     lhs.modular_sub(rhs, modulo)
    /// }
    /// ```
    /// You can use the above generic function `func<>()` for all
    /// SmallUInt-supported data types in a same scope.
    /// Look into the following example.
    /// 
    /// # Collective Example
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     let a_u8 = func(60_u8, 55, 100);
    ///     println!("60 - 55 = {} (mod 100)", a_u8);
    ///     assert_eq!(a_u8, 5);
    ///     
    ///     let b_u8 = func(a_u8, 15, 100);
    ///     println!("{} - 15 = {} (mod 100)", a_u8, b_u8);
    ///     assert_eq!(b_u8, 90);
    ///     
    ///     let a_u16 = func(6000_u16, 5500, 1_0000);
    ///     println!("6000 - 5500 = {} (mod 1_0000)", a_u16);
    ///     assert_eq!(a_u16, 500);
    ///     
    ///     let b_u16 = func(a_u16, 1500, 1_0000);
    ///     println!("{} - 1500 = {} (mod 1_0000)", a_u16, b_u16);
    ///     assert_eq!(b_u16, 9000);
    ///     
    ///     let a_u32 = func(6_0000_0000_u32, 5_5000_0000, 10_0000_0000);
    ///     println!("6_0000_0000 - 5_5000_0000 = {} (mod 10_0000_0000)", a_u32);
    ///     assert_eq!(a_u32, 5000_0000);
    ///     
    ///     let b_u32 = func(a_u32, 1_5000_0000, 10_0000_0000);
    ///     println!("{} - 1_5000_0000 = {} (mod 10_0000_0000)", a_u32, b_u32);
    ///     assert_eq!(b_u32, 9_0000_0000);
    ///     
    ///     let a_u64 = func(6_0000_0000_0000_0000_u64, 5_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    ///     println!("6_0000_0000_0000_0000 - 5_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_u64);
    ///     assert_eq!(a_u64, 5000_0000_0000_0000);
    ///     
    ///     let b_u64 = func(a_u64, 1_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    ///     println!("{} - 1_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_u64, b_u64);
    ///     assert_eq!(b_u64, 9_0000_0000_0000_0000);
    ///     
    ///     let a_u128 = func(6_0000_0000_0000_0000_0000_0000_0000_0000_u128, 5_5000_0000_0000_0000_0000_0000_0000_0000, 10_0000_0000_0000_0000_0000_0000_0000_0000);
    ///     println!("6_0000_0000_0000_0000_0000_0000_0000_0000 - 5_5000_0000_0000_0000_0000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000_0000_0000_0000_0000)", a_u128);
    ///     assert_eq!(a_u128, 5000_0000_0000_0000_0000_0000_0000_0000);
    ///     
    ///     let b_u128 = func(a_u128, 1_5000_0000_0000_0000_0000_0000_0000_0000, 10_0000_0000_0000_0000_0000_0000_0000_0000);
    ///     println!("{} - 1_5000_0000_0000_0000_0000_0000_0000_0000 = {}",a_u128, b_u128);
    ///     assert_eq!(b_u128, 9_0000_0000_0000_0000_0000_0000_0000_0000);
    ///     
    ///     let a_usize = func(6_0000_0000_0000_0000_usize, 5_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    ///     println!("6_0000_0000_0000_0000 - 5_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_usize);
    ///     assert_eq!(a_usize, 5000_0000_0000_0000);
    ///     
    ///     let b_usize = func(a_usize, 1_5000_0000_0000_0000, 10_0000_0000_0000_0000);
    ///     println!("{} - 1_5000_0000_0000_0000 = {} (mod 10_0000_0000_0000_0000)", a_usize, b_usize);
    ///     assert_eq!(b_usize, 9_0000_0000_0000_0000);
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs: T, rhs: T, modulo: T) -> T
    /// {
    ///     lhs.modular_sub(rhs, modulo)
    /// }
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
    /// # Example 1 for u8
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     use cryptocol::number::IntUnion;
    ///     // a_u16: u16 === (a_high_u8, a_low_u8) == (100_u8, 101_u8) == 25701_u16
    ///     let a_high_u8 = 100_u8;
    ///     let a_low_u8 = 101_u8;
    ///     // b_u16: u16 === (b_high_u8, b_low_u8) == (100_u8, 200_u8) == 25800_u16
    ///     let b_high_u8 = 100_u8;
    ///     let b_low_u8 = 200_u8;
    /// 
    ///     // (100_u8, 101_u8) X (100_u8, 200_u8) == 25701_u16 X 25800_u16 == 663085800_u32
    ///     //
    ///     //                  (100_u8, 101_u8) == 25701_u16
    ///     // X                (100_u8, 200_u8) == 25800_u16
    ///     // ---------------------------------
    ///     //                  ( 78_u8, 232_u8)
    ///     //          ( 78_u8,  32_u8)
    ///     //          ( 39_u8, 116_u8)
    ///     // + (39_u8,  16_u8)
    ///     // ---------------------------------
    ///     //   (39_u8, 133_u8, 226_u8, 232_u8) == 663085800_u32
    ///     let (c_lower_u8, c_low_u8, c_high_u8, c_higher_u8 ) = func(a_low_u8, a_high_u8, b_low_u8, b_high_u8);
    ///     println!("{}-{}-{}-{}", c_higher_u8, c_high_u8, c_low_u8, c_lower_u8);
    ///     assert_eq!(c_higher_u8, 39);
    ///     assert_eq!(c_high_u8, 133);
    ///     assert_eq!(c_low_u8, 226);
    ///     assert_eq!(c_lower_u8, 232);
    /// 
    ///     let a = IntUnion::new_with_ubytes([a_low_u8, a_high_u8, 0, 0]);
    ///     let b = IntUnion::new_with_ubytes([b_low_u8, b_high_u8, 0, 0]);
    ///     let c = a * b;
    ///     println!("{} * {} = {}", a.get(), b.get(), c.get());
    ///     assert_eq!(c_higher_u8, c.get_ubyte_(3));
    ///     assert_eq!(c_high_u8, c.get_ubyte_(2));
    ///     assert_eq!(c_low_u8, c.get_ubyte_(1));
    ///     assert_eq!(c_lower_u8, c.get_ubyte_(0));
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs_low: T, lhs_high: T, rhs_low: T, rhs_high: T) -> (T, T, T, T)
    /// {
    ///     let (c_low, c_high ) = rhs_low.carrying_mul(lhs_low, T::zero());
    ///     let (d_low, d_high ) = rhs_low.carrying_mul(lhs_high, c_high);
    ///     let (mut e_low, e_high ) = rhs_high.carrying_mul(lhs_low, T::zero());
    ///     let (mut f_low, mut f_high ) = rhs_high.carrying_mul(lhs_high, e_high);
    /// 
    ///     let mut overflow: bool;
    ///     (e_low, overflow) = e_low.overflowing_add(d_low);
    ///     if overflow
    ///         { (f_low, overflow) = f_low.overflowing_add(T::one()); }
    ///     if overflow
    ///         { f_high = f_high.wrapping_add(T::one()); }
    /// 
    ///     (f_low, overflow) = f_low.overflowing_add(d_high);
    ///     if overflow
    ///         { f_high = f_high.wrapping_add(T::one()); }
    ///     (c_low, e_low, f_low, f_high)
    /// }
    /// ```
    /// 
    /// # Example for u16
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     use cryptocol::number::LongUnion;
    ///     // a_u32: u32 === (a_high_u16, a_low_u16) == (10000_u16, 10100_u16) == 257010000_u32
    ///     let a_high_u16 = 10000_u16;
    ///     let a_low_u16 = 10100_u16;
    ///     // b_u32: u32 === (b_high_u16, b_low_u16) == (10000_u16, 20000_u16) == 258000000_u32
    ///     let b_high_u16 = 10000_u16;
    ///     let b_low_u16 = 20000_u16;
    ///     
    ///     // (10000_u16, 10100_u16) X (10000_u16, 20000_u16) == 257010000_u32 X 258000000_u32 == 66308580000000000_u32
    ///     //
    ///     //                        (10000_u16, 10100_u16) == 655370100_u32
    ///     // X                      (10000_u16, 20000_u16) == 655380000_u32
    ///     // ---------------------------------------------
    ///     //                       (  3082_u16, 18048_u16)
    ///     //            (  3051_u16, 49664_u16)
    ///     //            (  1541_u16,  9024_u16)
    ///     // + (1525_u16, 57600_u16)
    ///     // ---------------------------------
    ///     //   (1525_u16, 62192_u16, 61770_u16, 18048_u16) == 429516456138000000_u64
    ///     let (c_lower_u16, c_low_u16, c_high_u16, c_higher_u16 ) = func(a_low_u16, a_high_u16, b_low_u16, b_high_u16);
    ///     println!("{}-{}-{}-{}", c_higher_u16, c_high_u16, c_low_u16, c_lower_u16);
    ///     assert_eq!(c_higher_u16, 1525);
    ///     assert_eq!(c_high_u16, 62192);
    ///     assert_eq!(c_low_u16, 61770);
    ///     assert_eq!(c_lower_u16, 18048);
    /// 
    ///     let a = LongUnion::new_with_ushorts([a_low_u16, a_high_u16, 0, 0]);
    ///     let b = LongUnion::new_with_ushorts([b_low_u16, b_high_u16, 0, 0]);
    ///     let c = a * b;
    ///     println!("{} * {} = {}", a.get(), b.get(), c.get());
    ///     assert_eq!(c_higher_u16, c.get_ushort_(3));
    ///     assert_eq!(c_high_u16, c.get_ushort_(2));
    ///     assert_eq!(c_low_u16, c.get_ushort_(1));
    ///     assert_eq!(c_lower_u16, c.get_ushort_(0));
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs_low: T, lhs_high: T, rhs_low: T, rhs_high: T) -> (T, T, T, T)
    /// {
    ///     let (c_low, c_high ) = rhs_low.carrying_mul(lhs_low, T::zero());
    ///     let (d_low, d_high ) = rhs_low.carrying_mul(lhs_high, c_high);
    ///     let (mut e_low, e_high ) = rhs_high.carrying_mul(lhs_low, T::zero());
    ///     let (mut f_low, mut f_high ) = rhs_high.carrying_mul(lhs_high, e_high);
    /// 
    ///     let mut overflow: bool;
    ///     (e_low, overflow) = e_low.overflowing_add(d_low);
    ///     if overflow
    ///         { (f_low, overflow) = f_low.overflowing_add(T::one()); }
    ///     if overflow
    ///         { f_high = f_high.wrapping_add(T::one()); }
    /// 
    ///     (f_low, overflow) = f_low.overflowing_add(d_high);
    ///     if overflow
    ///         { f_high = f_high.wrapping_add(T::one()); }
    ///     (c_low, e_low, f_low, f_high)
    /// }
    /// ```
    /// You can use the above generic function `func<>()` for all
    /// SmallUInt-supported data types in a same scope.
    /// Look into the following example.
    /// 
    /// # Collective Example
    /// ```
    /// use cryptocol::number::SmallUInt;
    /// fn main()
    /// {
    ///     use cryptocol::number::IntUnion;
    ///     // a_u16: u16 === (a_high_u8, a_low_u8) == (100_u8, 101_u8) == 25701_u16
    ///     let a_high_u8 = 100_u8;
    ///     let a_low_u8 = 101_u8;
    ///     // b_u16: u16 === (b_high_u8, b_low_u8) == (100_u8, 200_u8) == 25800_u16
    ///     let b_high_u8 = 100_u8;
    ///     let b_low_u8 = 200_u8;
    /// 
    ///     // (100_u8, 101_u8) X (100_u8, 200_u8) == 25701_u16 X 25800_u16 == 663085800_u32
    ///     //
    ///     //                  (100_u8, 101_u8) == 25701_u16
    ///     // X                (100_u8, 200_u8) == 25800_u16
    ///     // ---------------------------------
    ///     //                  ( 78_u8, 232_u8)
    ///     //          ( 78_u8,  32_u8)
    ///     //          ( 39_u8, 116_u8)
    ///     // + (39_u8,  16_u8)
    ///     // ---------------------------------
    ///     //   (39_u8, 133_u8, 226_u8, 232_u8) == 663085800_u32
    ///     let (c_lower_u8, c_low_u8, c_high_u8, c_higher_u8 ) = func(a_low_u8, a_high_u8, b_low_u8, b_high_u8);
    ///     println!("{}-{}-{}-{}", c_higher_u8, c_high_u8, c_low_u8, c_lower_u8);
    ///     assert_eq!(c_higher_u8, 39);
    ///     assert_eq!(c_high_u8, 133);
    ///     assert_eq!(c_low_u8, 226);
    ///     assert_eq!(c_lower_u8, 232);
    /// 
    ///     let a = IntUnion::new_with_ubytes([a_low_u8, a_high_u8, 0, 0]);
    ///     let b = IntUnion::new_with_ubytes([b_low_u8, b_high_u8, 0, 0]);
    ///     let c = a * b;
    ///     println!("{} * {} = {}", a.get(), b.get(), c.get());
    ///     assert_eq!(c_higher_u8, c.get_ubyte_(3));
    ///     assert_eq!(c_high_u8, c.get_ubyte_(2));
    ///     assert_eq!(c_low_u8, c.get_ubyte_(1));
    ///     assert_eq!(c_lower_u8, c.get_ubyte_(0));
    /// 
    ///     use cryptocol::number::LongUnion;
    ///     // a_u32: u32 === (a_high_u16, a_low_u16) == (10000_u16, 10100_u16) == 257010000_u32
    ///     let a_high_u16 = 10000_u16;
    ///     let a_low_u16 = 10100_u16;
    ///     // b_u32: u32 === (b_high_u16, b_low_u16) == (10000_u16, 20000_u16) == 258000000_u32
    ///     let b_high_u16 = 10000_u16;
    ///     let b_low_u16 = 20000_u16;
    ///     
    ///     // (10000_u16, 10100_u16) X (10000_u16, 20000_u16) == 257010000_u32 X 258000000_u32 == 66308580000000000_u32
    ///     //
    ///     //                        (10000_u16, 10100_u16) == 655370100_u32
    ///     // X                      (10000_u16, 20000_u16) == 655380000_u32
    ///     // ---------------------------------------------
    ///     //                       (  3082_u16, 18048_u16)
    ///     //            (  3051_u16, 49664_u16)
    ///     //            (  1541_u16,  9024_u16)
    ///     // + (1525_u16, 57600_u16)
    ///     // ---------------------------------
    ///     //   (1525_u16, 62192_u16, 61770_u16, 18048_u16) == 429516456138000000_u64
    ///     let (c_lower_u16, c_low_u16, c_high_u16, c_higher_u16 ) = func(a_low_u16, a_high_u16, b_low_u16, b_high_u16);
    ///     println!("{}-{}-{}-{}", c_higher_u16, c_high_u16, c_low_u16, c_lower_u16);
    ///     assert_eq!(c_higher_u16, 1525);
    ///     assert_eq!(c_high_u16, 62192);
    ///     assert_eq!(c_low_u16, 61770);
    ///     assert_eq!(c_lower_u16, 18048);
    /// 
    ///     let a = LongUnion::new_with_ushorts([a_low_u16, a_high_u16, 0, 0]);
    ///     let b = LongUnion::new_with_ushorts([b_low_u16, b_high_u16, 0, 0]);
    ///     let c = a * b;
    ///     println!("{} * {} = {}", a.get(), b.get(), c.get());
    ///     assert_eq!(c_higher_u16, c.get_ushort_(3));
    ///     assert_eq!(c_high_u16, c.get_ushort_(2));
    ///     assert_eq!(c_low_u16, c.get_ushort_(1));
    ///     assert_eq!(c_lower_u16, c.get_ushort_(0));
    /// }
    /// 
    /// fn func<T: SmallUInt>(lhs_low: T, lhs_high: T, rhs_low: T, rhs_high: T) -> (T, T, T, T)
    /// {
    ///     let (c_low, c_high ) = rhs_low.carrying_mul(lhs_low, T::zero());
    ///     let (d_low, d_high ) = rhs_low.carrying_mul(lhs_high, c_high);
    ///     let (mut e_low, e_high ) = rhs_high.carrying_mul(lhs_low, T::zero());
    ///     let (mut f_low, mut f_high ) = rhs_high.carrying_mul(lhs_high, e_high);
    /// 
    ///     let mut overflow: bool;
    ///     (e_low, overflow) = e_low.overflowing_add(d_low);
    ///     if overflow
    ///         { (f_low, overflow) = f_low.overflowing_add(T::one()); }
    ///     if overflow
    ///         { f_high = f_high.wrapping_add(T::one()); }
    /// 
    ///     (f_low, overflow) = f_low.overflowing_add(d_high);
    ///     if overflow
    ///         { f_high = f_high.wrapping_add(T::one()); }
    ///     (c_low, e_low, f_low, f_high)
    /// }
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
    /// It is for internal use only. You are supposed to use
    /// [carrying_mul()](trait@SmallUInt#tymethod.carrying_mul) instead.
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
    /// # Feature
    /// It performs “long multiplication” which takes in an extra amount to add,
    /// and may return an additional amount of overflow. This allows for
    /// chaining together multiple multiplications to create “big integers”
    /// which represent larger values.
    /// 
    /// # Counterpart Methods
    /// If you also need to add a carry to the wide result,
    /// then you may want to use
    /// [carrying_mul()](trait@SmallUInt#tymethod.carrying_mul) instead.
    ///     
    /// The value of the first field in the returned tuple matches what you’d
    /// get the `wrapping_mul()` methods.
    /// `self.widening_mul(rhs).0` == `self.wrapping_mul(rhs)`
    /// 
    /// # Example 1 for u8
    /// ```
    /// fn main()
    /// {
    ///     use cryptocol::number::IntUnion;
    ///     // a_u16: u16 === (a_high_u8, a_low_u8) == (100_u8, 101_u8) == 25701_u16
    ///     let a_high_u8 = 100_u8;
    ///     let a_low_u8 = 101_u8;
    ///     // b_u16: u16 === (b_high_u8, b_low_u8) == (100_u8, 200_u8) == 25800_u16
    ///     let b_high_u8 = 100_u8;
    ///     let b_low_u8 = 200_u8;
    /// 
    ///     // (100_u8, 101_u8) X (100_u8, 200_u8) == 25701_u16 X 25800_u16 == 663085800_u32
    ///     //
    ///     //                  (100_u8, 101_u8) == 25701_u16
    ///     // X                (100_u8, 200_u8) == 25800_u16
    ///     // ---------------------------------
    ///     //                  ( 78_u8, 232_u8)
    ///     //          ( 78_u8,  32_u8)
    ///     //          ( 39_u8, 116_u8)
    ///     // + (39_u8,  16_u8)
    ///     // ---------------------------------
    ///     //   (39_u8, 133_u8, 226_u8, 232_u8) == 663085800_u32
    ///     let (c_lower_u8, c_low_u8, c_high_u8, c_higher_u8 ) = func(a_low_u8, a_high_u8, b_low_u8, b_high_u8);
    ///     println!("{}-{}-{}-{}", c_higher_u8, c_high_u8, c_low_u8, c_lower_u8);
    ///     assert_eq!(c_higher_u8, 39);
    ///     assert_eq!(c_high_u8, 133);
    ///     assert_eq!(c_low_u8, 226);
    ///     assert_eq!(c_lower_u8, 232);
    /// 
    ///     let a = IntUnion::new_with_ubytes([a_low_u8, a_high_u8, 0, 0]);
    ///     let b = IntUnion::new_with_ubytes([b_low_u8, b_high_u8, 0, 0]);
    ///     let c = a * b;
    ///     println!("{} * {} = {}", a.get(), b.get(), c.get());
    ///     assert_eq!(c_higher_u8, c.get_ubyte_(3));
    ///     assert_eq!(c_high_u8, c.get_ubyte_(2));
    ///     assert_eq!(c_low_u8, c.get_ubyte_(1));
    ///     assert_eq!(c_lower_u8, c.get_ubyte_(0));
    /// }
    /// 
    /// fn func<T: cryptocol::number::SmallUInt>(lhs_low: T, lhs_high: T, rhs_low: T, rhs_high: T) -> (T, T, T, T)
    /// {
    ///     let (c_low, c_high ) = rhs_low.widening_mul(lhs_low);
    ///     let (d_low, d_high ) = rhs_low.widening_mul(lhs_high);
    ///     let (mut e_low, e_high ) = rhs_high.widening_mul(lhs_low);
    ///     let (mut f_low, mut f_high ) = rhs_high.widening_mul(lhs_high);
    /// 
    ///     let mut overflow: bool;
    ///     (e_low, overflow) = e_low.overflowing_add(d_low);
    ///     if overflow
    ///         { (f_low, overflow) = f_low.overflowing_add(T::one()); }
    ///     if overflow
    ///         { f_high = f_high.wrapping_add(T::one()); }
    ///     (e_low, overflow) = e_low.overflowing_add(c_high);
    ///     if overflow
    ///         { (f_low, overflow) = f_low.overflowing_add(T::one()); }
    ///     if overflow
    ///         { f_high = f_high.wrapping_add(T::one()); }
    /// 
    ///     (f_low, overflow) = f_low.overflowing_add(d_high);
    ///     if overflow
    ///         { f_high = f_high.wrapping_add(T::one()); }
    ///     (f_low, overflow) = f_low.overflowing_add(e_high);
    ///     if overflow
    ///         { f_high = f_high.wrapping_add(T::one()); }
    ///     (c_low, e_low, f_low, f_high)
    /// }
    /// ```
    /// 
    /// # Example 2 for u16
    /// ```
    /// fn main()
    /// {
    ///     use cryptocol::number::LongUnion;
    ///     // a_u32: u32 === (a_high_u16, a_low_u16) == (10000_u16, 10100_u16) == 257010000_u32
    ///     let a_high_u16 = 10000_u16;
    ///     let a_low_u16 = 10100_u16;
    ///     // b_u32: u32 === (b_high_u16, b_low_u16) == (10000_u16, 20000_u16) == 258000000_u32
    ///     let b_high_u16 = 10000_u16;
    ///     let b_low_u16 = 20000_u16;
    ///     
    ///     // (10000_u16, 10100_u16) X (10000_u16, 20000_u16) == 257010000_u32 X 258000000_u32 == 66308580000000000_u32
    ///     //
    ///     //                        (10000_u16, 10100_u16) == 655370100_u32
    ///     // X                      (10000_u16, 20000_u16) == 655380000_u32
    ///     // ---------------------------------------------
    ///     //                       (  3082_u16, 18048_u16)
    ///     //            (  3051_u16, 49664_u16)
    ///     //            (  1541_u16,  9024_u16)
    ///     // + (1525_u16, 57600_u16)
    ///     // ---------------------------------
    ///     //   (1525_u16, 62192_u16, 61770_u16, 18048_u16) == 429516456138000000_u64
    ///     let (c_lower_u16, c_low_u16, c_high_u16, c_higher_u16 ) = func(a_low_u16, a_high_u16, b_low_u16, b_high_u16);
    ///     println!("{}-{}-{}-{}", c_higher_u16, c_high_u16, c_low_u16, c_lower_u16);
    ///     assert_eq!(c_higher_u16, 1525);
    ///     assert_eq!(c_high_u16, 62192);
    ///     assert_eq!(c_low_u16, 61770);
    ///     assert_eq!(c_lower_u16, 18048);
    /// 
    ///     let a = LongUnion::new_with_ushorts([a_low_u16, a_high_u16, 0, 0]);
    ///     let b = LongUnion::new_with_ushorts([b_low_u16, b_high_u16, 0, 0]);
    ///     let c = a * b;
    ///     println!("{} * {} = {}", a.get(), b.get(), c.get());
    ///     assert_eq!(c_higher_u16, c.get_ushort_(3));
    ///     assert_eq!(c_high_u16, c.get_ushort_(2));
    ///     assert_eq!(c_low_u16, c.get_ushort_(1));
    ///     assert_eq!(c_lower_u16, c.get_ushort_(0));
    /// }
    /// 
    /// fn func<T: cryptocol::number::SmallUInt>(lhs_low: T, lhs_high: T, rhs_low: T, rhs_high: T) -> (T, T, T, T)
    /// {
    ///     let (c_low, c_high ) = rhs_low.widening_mul(lhs_low);
    ///     let (d_low, d_high ) = rhs_low.widening_mul(lhs_high);
    ///     let (mut e_low, e_high ) = rhs_high.widening_mul(lhs_low);
    ///     let (mut f_low, mut f_high ) = rhs_high.widening_mul(lhs_high);
    /// 
    ///     let mut overflow: bool;
    ///     (e_low, overflow) = e_low.overflowing_add(d_low);
    ///     if overflow
    ///         { (f_low, overflow) = f_low.overflowing_add(T::one()); }
    ///     if overflow
    ///         { f_high = f_high.wrapping_add(T::one()); }
    ///     (e_low, overflow) = e_low.overflowing_add(c_high);
    ///     if overflow
    ///         { (f_low, overflow) = f_low.overflowing_add(T::one()); }
    ///     if overflow
    ///         { f_high = f_high.wrapping_add(T::one()); }
    /// 
    ///     (f_low, overflow) = f_low.overflowing_add(d_high);
    ///     if overflow
    ///         { f_high = f_high.wrapping_add(T::one()); }
    ///     (f_low, overflow) = f_low.overflowing_add(e_high);
    ///     if overflow
    ///         { f_high = f_high.wrapping_add(T::one()); }
    ///     (c_low, e_low, f_low, f_high)
    /// }
    /// ```
    /// You can use the above generic function `func<>()` for all
    /// SmallUInt-supported data types in a same scope.
    /// Look into the following example.
    /// 
    /// # Collective Example
    /// ```
    /// fn small_uint_widening_mul()
    /// {
    ///     use cryptocol::number::IntUnion;
    ///     // a_u16: u16 === (a_high_u8, a_low_u8) == (100_u8, 101_u8) == 25701_u16
    ///     let a_high_u8 = 100_u8;
    ///     let a_low_u8 = 101_u8;
    ///     // b_u16: u16 === (b_high_u8, b_low_u8) == (100_u8, 200_u8) == 25800_u16
    ///     let b_high_u8 = 100_u8;
    ///     let b_low_u8 = 200_u8;
    /// 
    ///     // (100_u8, 101_u8) X (100_u8, 200_u8) == 25701_u16 X 25800_u16 == 663085800_u32
    ///     //
    ///     //                  (100_u8, 101_u8) == 25701_u16
    ///     // X                (100_u8, 200_u8) == 25800_u16
    ///     // ---------------------------------
    ///     //                  ( 78_u8, 232_u8)
    ///     //          ( 78_u8,  32_u8)
    ///     //          ( 39_u8, 116_u8)
    ///     // + (39_u8,  16_u8)
    ///     // ---------------------------------
    ///     //   (39_u8, 133_u8, 226_u8, 232_u8) == 663085800_u32
    ///     let (c_lower_u8, c_low_u8, c_high_u8, c_higher_u8 ) = func(a_low_u8, a_high_u8, b_low_u8, b_high_u8);
    ///     println!("{}-{}-{}-{}", c_higher_u8, c_high_u8, c_low_u8, c_lower_u8);
    ///     assert_eq!(c_higher_u8, 39);
    ///     assert_eq!(c_high_u8, 133);
    ///     assert_eq!(c_low_u8, 226);
    ///     assert_eq!(c_lower_u8, 232);
    /// 
    ///     let a = IntUnion::new_with_ubytes([a_low_u8, a_high_u8, 0, 0]);
    ///     let b = IntUnion::new_with_ubytes([b_low_u8, b_high_u8, 0, 0]);
    ///     let c = a * b;
    ///     println!("{} * {} = {}", a.get(), b.get(), c.get());
    ///     assert_eq!(c_higher_u8, c.get_ubyte_(3));
    ///     assert_eq!(c_high_u8, c.get_ubyte_(2));
    ///     assert_eq!(c_low_u8, c.get_ubyte_(1));
    ///     assert_eq!(c_lower_u8, c.get_ubyte_(0));
    /// 
    ///     use cryptocol::number::LongUnion;
    ///     // a_u32: u32 === (a_high_u16, a_low_u16) == (10000_u16, 10100_u16) == 257010000_u32
    ///     let a_high_u16 = 10000_u16;
    ///     let a_low_u16 = 10100_u16;
    ///     // b_u32: u32 === (b_high_u16, b_low_u16) == (10000_u16, 20000_u16) == 258000000_u32
    ///     let b_high_u16 = 10000_u16;
    ///     let b_low_u16 = 20000_u16;
    ///     
    ///     // (10000_u16, 10100_u16) X (10000_u16, 20000_u16) == 257010000_u32 X 258000000_u32 == 66308580000000000_u32
    ///     //
    ///     //                        (10000_u16, 10100_u16) == 655370100_u32
    ///     // X                      (10000_u16, 20000_u16) == 655380000_u32
    ///     // ---------------------------------------------
    ///     //                       (  3082_u16, 18048_u16)
    ///     //            (  3051_u16, 49664_u16)
    ///     //            (  1541_u16,  9024_u16)
    ///     // + (1525_u16, 57600_u16)
    ///     // ---------------------------------
    ///     //   (1525_u16, 62192_u16, 61770_u16, 18048_u16) == 429516456138000000_u64
    ///     let (c_lower_u16, c_low_u16, c_high_u16, c_higher_u16 ) = func(a_low_u16, a_high_u16, b_low_u16, b_high_u16);
    ///     println!("{}-{}-{}-{}", c_higher_u16, c_high_u16, c_low_u16, c_lower_u16);
    ///     assert_eq!(c_higher_u16, 1525);
    ///     assert_eq!(c_high_u16, 62192);
    ///     assert_eq!(c_low_u16, 61770);
    ///     assert_eq!(c_lower_u16, 18048);
    /// 
    ///     let a = LongUnion::new_with_ushorts([a_low_u16, a_high_u16, 0, 0]);
    ///     let b = LongUnion::new_with_ushorts([b_low_u16, b_high_u16, 0, 0]);
    ///     let c = a * b;
    ///     println!("{} * {} = {}", a.get(), b.get(), c.get());
    ///     assert_eq!(c_higher_u16, c.get_ushort_(3));
    ///     assert_eq!(c_high_u16, c.get_ushort_(2));
    ///     assert_eq!(c_low_u16, c.get_ushort_(1));
    ///     assert_eq!(c_lower_u16, c.get_ushort_(0));
    /// }
    /// 
    /// fn func<T: cryptocol::number::SmallUInt>(lhs_low: T, lhs_high: T, rhs_low: T, rhs_high: T) -> (T, T, T, T)
    /// {
    ///     let (c_low, c_high ) = rhs_low.widening_mul(lhs_low);
    ///     let (d_low, d_high ) = rhs_low.widening_mul(lhs_high);
    ///     let (mut e_low, e_high ) = rhs_high.widening_mul(lhs_low);
    ///     let (mut f_low, mut f_high ) = rhs_high.widening_mul(lhs_high);
    /// 
    ///     let mut overflow: bool;
    ///     (e_low, overflow) = e_low.overflowing_add(d_low);
    ///     if overflow
    ///         { (f_low, overflow) = f_low.overflowing_add(T::one()); }
    ///     if overflow
    ///         { f_high = f_high.wrapping_add(T::one()); }
    ///     (e_low, overflow) = e_low.overflowing_add(c_high);
    ///     if overflow
    ///         { (f_low, overflow) = f_low.overflowing_add(T::one()); }
    ///     if overflow
    ///         { f_high = f_high.wrapping_add(T::one()); }
    /// 
    ///     (f_low, overflow) = f_low.overflowing_add(d_high);
    ///     if overflow
    ///         { f_high = f_high.wrapping_add(T::one()); }
    ///     (f_low, overflow) = f_low.overflowing_add(e_high);
    ///     if overflow
    ///         { f_high = f_high.wrapping_add(T::one()); }
    ///     (c_low, e_low, f_low, f_high)
    /// }
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
    /// use cryptocol::number::SmallUInt;
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
    /// use cryptocol::number::SmallUInt;
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
    /// use cryptocol::number::SmallUInt;
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
    /// use cryptocol::number::SmallUInt;
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
    /// use cryptocol::number::SmallUInt;
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
    /// use cryptocol::number::SmallUInt;
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
    /// use cryptocol::number::SmallUInt;
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
    /// use cryptocol::number::SmallUInt;
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
    /// use cryptocol::number::SmallUInt;
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
    /// use cryptocol::number::SmallUInt;
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
    /// use cryptocol::number::SmallUInt;
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
    /// use cryptocol::number::SmallUInt;
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
    /// use cryptocol::number::*;
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
    /// use cryptocol::number::*;
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
    /// use cryptocol::number::*;
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
    /// use cryptocol::number::*;
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
    /// use cryptocol::number::*;
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
    /// use cryptocol::number::*;
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
    /// use cryptocol::number::*;
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
    /// use cryptocol::number::*;
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
    /// use cryptocol::number::*;
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
    /// use cryptocol::number::*;
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
    /// use cryptocol::number::*;
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
    /// You can use the above generic function `func<>()` for all
    /// SmallUInt-supported data types in a same scope.
    /// Look into the following example.
    /// 
    /// # Collective Example
    /// ```
    /// use cryptocol::number::*;
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
    /// use cryptocol::number::*;
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
    /// use cryptocol::number::*;
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
    /// use cryptocol::number::*;
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
    /// use cryptocol::number::*;
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
    /// use cryptocol::define_utypes_with;
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

    // fn test_miller_rabin(self, a: Self) -> bool
    /// Tests a `SmallUInt`-type object to find whether or not it is a
    /// prime number.
    /// [Read more in detail](trait@SmallUInt#tymethod.is_prime_using_miller_rabin)
    fn test_miller_rabin(self, a: Self) -> bool;

    // fn is_prime_using_miller_rabin(&self, repetition: usize) -> bool
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
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let num = u1024::random();
    /// let yes = num.is_prime_using_miller_rabin(5);
    /// println!("Is {} a prime number? => {}", num, yes);
    /// if yes  { assert!(yes); }
    /// else    { assert!(!yes); }
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    /// Tests a `SmallUInt`-type object to find whether or not it is a
    /// prime number.
    /// [Read more in detail](trait@SmallUInt#tymethod.is_prime_using_miller_rabin)
    fn is_prime_using_miller_rabin(self, repetition: usize) -> bool;

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
    fn u128_as_smalluint(n: u128) -> Self;
    fn u64_as_smalluint(n: u64) -> Self;
    fn u32_as_smalluint(n: u32) -> Self;
    fn u16_as_smalluint(n: u16) -> Self;
    fn u8_as_smalluint(n: u8) -> Self;
    fn usize_as_smalluint(n: usize) -> Self;
    fn bool_as_smalluint(n: bool) -> Self;
    fn num<T: SmallUInt>(n: T) -> Self;

    // pub fn set_zero(&mut self)
    /// Sets `SmallUInt` to be zero.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::define_utypes_with;
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

    // pub fn set_msb(&mut self)
    /// Sets the MSB (Most Significant Bit) of `BigUInt`-type number with `1`.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a = u256::new();
    /// println!("a = {}", a);
    /// a.set_msb();
    /// println!("a = {}", a);
    /// assert_eq!(a.to_string_with_radix_and_stride(2, 8).unwrap(), "10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    fn set_msb(&mut self);

    // pub fn set_lsb(&mut self)
    /// Sets the LSB (Least Significant Bit) of `BigUInt`-type number with `1`.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a = u256::new();
    /// println!("a = {}", a);
    /// a.set_lsb();
    /// println!("a = {}", a);
    /// assert_eq!(a.to_string_with_radix_and_stride(2, 8).unwrap(), "1");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    fn set_lsb(&mut self);

    // fn generate_check_bits(bit_pos: usize) -> Option<Self>
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
    /// use cryptocol::define_utypes_with_u32;
    /// define_utypes_with_u32!();
    /// 
    /// let a_0 = u256::generate_check_bits(0).unwrap();
    /// println!("a_0 = {}", a_0.to_string_with_radix_and_stride(2, 10).unwrap());
    /// assert_eq!(a_0.to_string_with_radix_and_stride(2, 10).unwrap(), "1");
    /// 
    /// let a_12 = u256::generate_check_bits(12).unwrap();
    /// println!("a_12 = {}", a_12.to_string_with_radix_and_stride(2, 10).unwrap());
    /// assert_eq!(a_12.to_string_with_radix_and_stride(2, 10).unwrap(), "100_0000000000");
    /// 
    /// let a_255 = u256::generate_check_bits(255).unwrap();
    /// println!("a_255 = {}", a_255.to_string_with_radix_and_stride(2, 10).unwrap());
    /// assert_eq!(a_255.to_string_with_radix_and_stride(2, 10).unwrap(), "100000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000");
    /// 
    /// let a_256 = u256::generate_check_bits(256);
    /// println!("a_256 = {}", a_256);
    /// assert_eq!(a_255, None);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline] fn generate_check_bits(bit_pos: usize) -> Option<Self>    { if bit_pos < Self::size_in_bits() { Some(Self::generate_check_bits_(bit_pos)) } else { None } }

    // fn generate_check_bits_(&mut self, bit_pos: usize) -> Self
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
    /// use cryptocol::define_utypes_with_u32;
    /// define_utypes_with_u32!();
    /// 
    /// let a_0 = u256::generate_check_bits_(0);
    /// println!("a_0 = {}", a_0.to_string_with_radix_and_stride(2, 10).unwrap());
    /// assert_eq!(a_0.to_string_with_radix_and_stride(2, 10).unwrap(), "1");
    /// 
    /// let a_12 = u256::generate_check_bits_(12);
    /// println!("a_12 = {}", a_12.to_string_with_radix_and_stride(2, 10).unwrap());
    /// assert_eq!(a_12.to_string_with_radix_and_stride(2, 10).unwrap(), "100_0000000000");
    /// 
    /// let a_255 = u256::generate_check_bits_(255);
    /// println!("a_255 = {}", a_255.to_string_with_radix_and_stride(2, 10).unwrap());
    /// assert_eq!(a_255.to_string_with_radix_and_stride(2, 10).unwrap(), "100000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000");
    /// // It will panic!
    /// // let a_256 = u256::generate_check_bits_(256);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    fn generate_check_bits_(bit_pos: usize) -> Self;

    fn is_odd(self) -> bool;
    fn is_even(self) -> bool;
    fn is_msb_set(self) -> bool;
    fn is_bit_set(self, bit_pos: usize) -> Option<bool>;
    fn is_bit_set_(self, bit_pos: usize) -> bool;

    fn size_in_bytes() -> usize;
    fn size_in_bits() -> usize;
    fn length_in_bytes(self) -> usize;
    fn length_in_bits(self) -> usize;
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
