// Copyright 2023, 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module that contains unions of primitive signed/unsigned integral
//! data types used in a lot of modules of the crate Cryptocol.
//! __These unions are for segmentation.__

// #![warn(missing_docs)]
// #![warn(missing_doc_code_examples)]
#![allow(missing_docs)]
#![allow(rustdoc::missing_doc_code_examples)]


#[allow(unused_macros)]
macro_rules! get_set_byte_fit {
    () => {
        // pub fn get_ubyte(self) -> u8
        /// Returns its value as `u8`.
        /// 
        /// # Output
        /// Its value as `u8`
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// let a = SizeUnion::new_with(250_usize);
        /// println!("a = {}", a.get_ubyte());
        /// assert_eq!(a.get_ubyte(), 250_usize);
        /// ```
        #[inline] pub fn get_ubyte(self) -> u8  { unsafe { self.ubyte } }

        // pub fn set_ubyte(&mut self, val: u8)
        /// Sets its value with `val` of type `u8`
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::SizeUnion;    
        /// let mut a = SizeUnion::new();
        /// a.set_ubyte(234_u8);
        /// println!("a = {}", a.get_ubyte());
        /// assert_eq!(a.get_ubyte(), 234_u8);
        /// ```
        #[inline] pub fn set_ubyte(&mut self, val: u8)  { self.ubyte = val; }

        // pub fn get_sbyte(self) -> i8
        /// Returns its value as `i8`.
        /// 
        /// # Output
        /// Its value as `i8`
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::SizeUnion;    
        /// let a = SizeUnion::new_with_signed(-123_i8);
        /// println!("a = {}", a.get_sbyte());
        /// assert_eq!(a.get_sbyte(), -123_i8);
        /// ```
        #[inline] pub fn get_sbyte(self) -> i8      { unsafe { self.sbyte } }

        // pub fn set_sbyte(&mut self, val: i8)
        /// Sets its value with `val` of type `isize`
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::SizeUnion;    
        /// let mut a = SizeUnion::new();
        /// a.set_sbyte(-123_isize);
        /// println!("a = {}", a.get_sbyte());
        /// assert_eq!(a.get_sbyte(), -123_isize);
        /// ```
        #[inline] pub fn set_sbyte(&mut self, val: i8)  { self.sbyte = val; }
    };
}
#[allow(unused_imports)] pub(super) use get_set_byte_fit;

macro_rules! get_set_byte {
    ($f:expr) => {
        const N: usize = $f - 1;

        // pub fn get_ubyte_(&self, i: usize) -> u8
        /// Returns i-th element of array `ubyte` of type `u8`
        /// if `i` is less than the size of this union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Output
        /// i-th element of array `ubyte` of type `u8`
        /// 
        /// # Arguments
        /// `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to the size of this union in
        /// bytes, it will panic.
        /// 
        /// # Caution
        /// Use this method only when you are sure
        /// that the argument `i` is less than the size of this union.
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method `get_ubyte()`.
        /// This method `get_ubyte_()` is considered to be slightly
        /// faster thanthe method `get_ubyte()`.
        /// Use this method only when you are sure that `i` is less than the
        /// size of this union in bytes. Otherwise, use its counterpart method
        /// [get_ubyte()](#method.get_ubyte) for safety.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// let a_shortunion = ShortUnion::new_with(2895_u16);
        /// for i in 0..2
        ///     { println!("a_shortunion.get_ubyte_({}) = {}", i, a_shortunion.get_ubyte_(i)); }
        /// assert_eq!(a_shortunion.get_ubyte_(0), 79_u8);
        /// assert_eq!(a_shortunion.get_ubyte_(1), 11_u8);
        /// 
        /// // It will panic.
        /// // println!("a_shortunion.get_ubyte_(2) = {}", a_shortunion.get_ubyte_(2));
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// let a_intunion = IntUnion::new_with(4048161615_u32);
        /// for i in 0..4
        ///     { println!("a_intunion.get_ubyte_({}) = {}", i, a_intunion.get_ubyte_(i)); }
        /// assert_eq!(a_intunion.get_ubyte_(0), 79_u8);
        /// assert_eq!(a_intunion.get_ubyte_(1), 11_u8);
        /// assert_eq!(a_intunion.get_ubyte_(2), 74_u8);
        /// assert_eq!(a_intunion.get_ubyte_(3), 241_u8);
        /// 
        /// // It will panic.
        /// // println!("a_intunion.get_ubyte_(4) = {}", a_intunion.get_ubyte_(4));
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let a_longunion = LongUnion::new_with(13664881099896654671_u64);
        /// for i in 0..8
        ///     { println!("a_longunion.get_ubyte_({}) = {}", i, a_longunion.get_ubyte_(i)); }
        /// assert_eq!(a_longunion.get_ubyte_(0), 79_u8);
        /// assert_eq!(a_longunion.get_ubyte_(1), 11_u8);
        /// assert_eq!(a_longunion.get_ubyte_(2), 74_u8);
        /// assert_eq!(a_longunion.get_ubyte_(3), 241_u8);
        /// assert_eq!(a_longunion.get_ubyte_(4), 245_u8);
        /// assert_eq!(a_longunion.get_ubyte_(5), 104_u8);
        /// assert_eq!(a_longunion.get_ubyte_(6), 163_u8);
        /// assert_eq!(a_longunion.get_ubyte_(7), 189_u8);
        /// 
        /// // It will panic.
        /// // println!("a_longunion.get_ubyte_(8) = {}", a_longunion.get_ubyte_(8));
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// for i in 0..16
        ///     { println!("a_longerunion.get_ubyte_({}) = {}", i, a_longerunion.get_ubyte_(i)); }
        /// assert_eq!(a_longerunion.get_ubyte_(0), 79_u8);
        /// assert_eq!(a_longerunion.get_ubyte_(1), 11_u8);
        /// assert_eq!(a_longerunion.get_ubyte_(2), 74_u8);
        /// assert_eq!(a_longerunion.get_ubyte_(3), 241_u8);
        /// assert_eq!(a_longerunion.get_ubyte_(4), 245_u8);
        /// assert_eq!(a_longerunion.get_ubyte_(5), 104_u8);
        /// assert_eq!(a_longerunion.get_ubyte_(6), 163_u8);
        /// assert_eq!(a_longerunion.get_ubyte_(7), 189_u8);
        /// assert_eq!(a_longerunion.get_ubyte_(8), 88_u8);
        /// assert_eq!(a_longerunion.get_ubyte_(9), 136_u8);
        /// assert_eq!(a_longerunion.get_ubyte_(10), 206_u8);
        /// assert_eq!(a_longerunion.get_ubyte_(11), 126_u8);
        /// assert_eq!(a_longerunion.get_ubyte_(12), 26_u8);
        /// assert_eq!(a_longerunion.get_ubyte_(13), 59_u8);
        /// assert_eq!(a_longerunion.get_ubyte_(14), 18_u8);
        /// assert_eq!(a_longerunion.get_ubyte_(15), 255_u8);
        /// 
        /// // It will panic.
        /// // println!("a_longerunion.get_ubyte_(16) = {}", a_longerunion.get_ubyte_(16));
        /// ```
        /// 
        /// # Example 5 for SizeUnion for 64-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let a_sizeunion = SizeUnion::new_with(13664881099896654671_usize);
        ///     for i in 0..8
        ///         { println!("a_sizeunion.get_ubyte_({}) = {}", i, a_sizeunion.get_ubyte_(i)); }
        ///     assert_eq!(a_sizeunion.get_ubyte_(0), 79_u8);
        ///     assert_eq!(a_sizeunion.get_ubyte_(1), 11_u8);
        ///     assert_eq!(a_sizeunion.get_ubyte_(2), 74_u8);
        ///     assert_eq!(a_sizeunion.get_ubyte_(3), 241_u8);
        ///     assert_eq!(a_sizeunion.get_ubyte_(4), 245_u8);
        ///     assert_eq!(a_sizeunion.get_ubyte_(5), 104_u8);
        ///     assert_eq!(a_sizeunion.get_ubyte_(6), 163_u8);
        ///     assert_eq!(a_sizeunion.get_ubyte_(7), 189_u8);
        /// 
        ///     // It will panic.
        ///     // println!("a_sizeunion.get_ubyte_(8) = {}", a_sizeunion.get_ubyte_(8));
        /// }
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[inline] pub fn get_ubyte_(&self, i: usize) -> u8
        {
            #[cfg(target_endian = "little")]    unsafe { self.ubyte[i] }
            #[cfg(target_endian = "big")]       unsafe { self.ubyte[Self::N-i] }
        }

        // pub fn set_ubyte_(&mut self, i: usize, val: u8)
        /// Sets i-th element of its array `ubyte` of type `u8`
        /// if `i` is less than the size of this union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Arguments
        /// - `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// - `val` is the value of type `u8` to set the i-th element of its
        ///  array `ubyte` of type `u8`.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to the size of this union in
        /// bytes, it will panic.
        /// 
        /// # Caution
        /// Use this method only when you are sure
        /// that the argument `i` is less than the size of this union.
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_ubyte(). This method
        /// set_ubyte_() is considered to be slightly faster than the method
        /// set_ubyte().
        /// Use this method only when you are sure that `i` is less than the
        /// size of this union in bytes. Otherwise, use its counterpart method
        /// [set_ubyte()](#method.set_ubyte) for safety.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// let mut a_shortunion = ShortUnion::new();
        /// a_shortunion.set_ubyte_(0, 79_u8);
        /// a_shortunion.set_ubyte_(1, 11_u8);
        /// // It will panic.
        /// // a_shortunion.set_ubyte_(2, 100_u8);
        /// println!("a_shortunion.get() = {}", a_shortunion.get());
        /// for i in 0..2
        ///     { println!("a_shortunion.get_ubyte_({}) = {}", i, a_shortunion.get_ubyte_(i)); }
        /// assert_eq!(a_shortunion.get_ubyte_(0), 79_u8);
        /// assert_eq!(a_shortunion.get_ubyte_(1), 11_u8);
        /// assert_eq!(a_shortunion.get(), 2895_u16);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// let mut a_intunion = IntUnion::new();
        /// a_intunion.set_ubyte_(0, 79_u8);
        /// a_intunion.set_ubyte_(1, 11_u8);
        /// a_intunion.set_ubyte_(2, 74_u8);
        /// a_intunion.set_ubyte_(3, 241_u8);
        /// // It will panic.
        /// // a_intunion.set_ubyte_(4, 100_u8);
        /// println!("a_intunion.get() = {}", a_intunion.get());
        /// for i in 0..4
        ///     { println!("a_intunion.get_ubyte_({}) = {}", i, a_intunion.get_ubyte_(i)); }
        /// assert_eq!(a_intunion.get_ubyte_(0), 79_u8);
        /// assert_eq!(a_intunion.get_ubyte_(1), 11_u8);
        /// assert_eq!(a_intunion.get_ubyte_(2), 74_u8);
        /// assert_eq!(a_intunion.get_ubyte_(3), 241_u8);
        /// assert_eq!(a_intunion.get(), 4048161615_u32);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let mut a_longunion = LongUnion::new();
        /// a_longunion.set_ubyte_(0, 79_u8);
        /// a_longunion.set_ubyte_(1, 11_u8);
        /// a_longunion.set_ubyte_(2, 74_u8);
        /// a_longunion.set_ubyte_(3, 241_u8);
        /// a_longunion.set_ubyte_(4, 245_u8);
        /// a_longunion.set_ubyte_(5, 104_u8);
        /// a_longunion.set_ubyte_(6, 163_u8);
        /// a_longunion.set_ubyte_(7, 189_u8);
        /// // It will panic.
        /// // a_longunion.set_ubyte_(8, 100_u8);
        /// for i in 0..8
        ///     { println!("a_longunion.get_ubyte_({}) = {}", i, a_longunion.get_ubyte_(i)); }
        /// assert_eq!(a_longunion.get_ubyte_(0), 79_u8);
        /// assert_eq!(a_longunion.get_ubyte_(1), 11_u8);
        /// assert_eq!(a_longunion.get_ubyte_(2), 74_u8);
        /// assert_eq!(a_longunion.get_ubyte_(3), 241_u8);
        /// assert_eq!(a_longunion.get_ubyte_(4), 245_u8);
        /// assert_eq!(a_longunion.get_ubyte_(5), 104_u8);
        /// assert_eq!(a_longunion.get_ubyte_(6), 163_u8);
        /// assert_eq!(a_longunion.get_ubyte_(7), 189_u8);
        /// assert_eq!(a_longunion.get(), 13664881099896654671_u64);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let mut a_longerunion = LongerUnion::new();
        /// a_longerunion.set_ubyte_(0, 79_u8);
        /// a_longerunion.set_ubyte_(1, 11_u8);
        /// a_longerunion.set_ubyte_(2, 74_u8);
        /// a_longerunion.set_ubyte_(3, 241_u8);
        /// a_longerunion.set_ubyte_(4, 245_u8);
        /// a_longerunion.set_ubyte_(5, 104_u8);
        /// a_longerunion.set_ubyte_(6, 163_u8);
        /// a_longerunion.set_ubyte_(7, 189_u8);
        /// a_longerunion.set_ubyte_(8, 88_u8);
        /// a_longerunion.set_ubyte_(9, 136_u8);
        /// a_longerunion.set_ubyte_(10, 206_u8);
        /// a_longerunion.set_ubyte_(11, 126_u8);
        /// a_longerunion.set_ubyte_(12, 26_u8);
        /// a_longerunion.set_ubyte_(13, 59_u8);
        /// a_longerunion.set_ubyte_(14, 18_u8);
        /// a_longerunion.set_ubyte_(15, 255_u8);
        /// // It will panic.
        /// // a_longerunion.set_ubyte_(16, 100_u8);
        /// for i in 0..16
        ///     { println!("a_longerunion.get_ubyte_({}) = {}", i, a_longerunion.get_ubyte_(i)); }
        /// assert_eq!(a_longerunion.get_ubyte_(0), 79_u8);
        /// assert_eq!(a_longerunion.get_ubyte_(1), 11_u8);
        /// assert_eq!(a_longerunion.get_ubyte_(2), 74_u8);
        /// assert_eq!(a_longerunion.get_ubyte_(3), 241_u8);
        /// assert_eq!(a_longerunion.get_ubyte_(4), 245_u8);
        /// assert_eq!(a_longerunion.get_ubyte_(5), 104_u8);
        /// assert_eq!(a_longerunion.get_ubyte_(6), 163_u8);
        /// assert_eq!(a_longerunion.get_ubyte_(7), 189_u8);
        /// assert_eq!(a_longerunion.get_ubyte_(8), 88_u8);
        /// assert_eq!(a_longerunion.get_ubyte_(9), 136_u8);
        /// assert_eq!(a_longerunion.get_ubyte_(10), 206_u8);
        /// assert_eq!(a_longerunion.get_ubyte_(11), 126_u8);
        /// assert_eq!(a_longerunion.get_ubyte_(12), 26_u8);
        /// assert_eq!(a_longerunion.get_ubyte_(13), 59_u8);
        /// assert_eq!(a_longerunion.get_ubyte_(14), 18_u8);
        /// assert_eq!(a_longerunion.get_ubyte_(15), 255_u8);
        /// assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
        /// ```
        /// 
        /// # Example 5 for SizeUnion for 64-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let mut a_sizeunion = SizeUnion::new();
        ///     a_sizeunion.set_ubyte_(0, 79_u8);
        ///     a_sizeunion.set_ubyte_(1, 11_u8);
        ///     a_sizeunion.set_ubyte_(2, 74_u8);
        ///     a_sizeunion.set_ubyte_(3, 241_u8);
        ///     a_sizeunion.set_ubyte_(4, 245_u8);
        ///     a_sizeunion.set_ubyte_(5, 104_u8);
        ///     a_sizeunion.set_ubyte_(6, 163_u8);
        ///     a_sizeunion.set_ubyte_(7, 189_u8);
        ///     // It will panic.
        ///     // a_sizeunion.set_ubyte_(8, 100_u8);
        ///     for i in 0..8
        ///         { println!("a_sizeunion.get_ubyte_({}) = {}", i, a_sizeunion.get_ubyte_(i)); }
        ///     assert_eq!(a_sizeunion.get_ubyte_(0), 79_u8);
        ///     assert_eq!(a_sizeunion.get_ubyte_(1), 11_u8);
        ///     assert_eq!(a_sizeunion.get_ubyte_(2), 74_u8);
        ///     assert_eq!(a_sizeunion.get_ubyte_(3), 241_u8);
        ///     assert_eq!(a_sizeunion.get_ubyte_(4), 245_u8);
        ///     assert_eq!(a_sizeunion.get_ubyte_(5), 104_u8);
        ///     assert_eq!(a_sizeunion.get_ubyte_(6), 163_u8);
        ///     assert_eq!(a_sizeunion.get_ubyte_(7), 189_u8);
        ///     assert_eq!(a_sizeunion.get(), 13664881099896654671_usize);
        /// }
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[inline] pub fn set_ubyte_(&mut self, i: usize, val: u8)
        {
            #[cfg(target_endian = "little")]    unsafe { self.ubyte[i] = val; }
            #[cfg(target_endian = "big")]       unsafe { self.ubyte[Self::N-i] = val; }
        }

        // pub fn get_ubyte(&self, i: usize) -> Option<u8>
        /// Returns i-th element of array `ubyte` of type `u8` wrapped in `Some`
        /// of enum `Option` if `i` is less than the size of this union in bytes.
        /// Otherwise, it returns None of enum `Option`.
        /// 
        /// # Output
        /// - i-th element of array `ubyte` of type `u8` wrapped in `Some`
        /// of enum `Option` if `i` is less than the size of this union in bytes
        /// - `None` if `i` is greater than or equal to the size of this union
        /// in bytes
        /// 
        /// # Arguments
        /// `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than the
        /// size of this union in bytes. If you are sure that `i` is less than
        /// the size of this union in bytes, you canuse its counterpart method
        /// [get_ubyte_()](#method.get_ubyte_) for performance.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// let a_shortunion = ShortUnion::new_with(2895_u16);
        /// for i in 0..2
        /// {
        ///     match a_shortunion.get_ubyte(i)
        ///     {
        ///         Some(a) => { println!("a_shortunion.get_ubyte({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_shortunion.get_ubyte(0), Some(79_u8));
        /// assert_eq!(a_shortunion.get_ubyte(1), Some(11_u8));
        /// assert_eq!(a_shortunion.get_ubyte(2), None);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// let a_intunion = IntUnion::new_with(4048161615_u32);
        /// for i in 0..4
        /// {
        ///     match a_intunion.get_ubyte(i)
        ///     {
        ///         Some(a) => { println!("a_intunion.get_ubyte({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_intunion.get_ubyte(0), Some(79_u8));
        /// assert_eq!(a_intunion.get_ubyte(1), Some(11_u8));
        /// assert_eq!(a_intunion.get_ubyte(2), Some(74_u8));
        /// assert_eq!(a_intunion.get_ubyte(3), Some(241_u8));
        /// assert_eq!(a_intunion.get_ubyte(4), None);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let a_longunion = LongUnion::new_with(13664881099896654671_u64);
        /// for i in 0..8
        /// {
        ///     match a_longunion.get_ubyte(i)
        ///     {
        ///         Some(a) => { println!("a_longunion.get_ubyte({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longunion.get_ubyte(0), Some(79_u8));
        /// assert_eq!(a_longunion.get_ubyte(1), Some(11_u8));
        /// assert_eq!(a_longunion.get_ubyte(2), Some(74_u8));
        /// assert_eq!(a_longunion.get_ubyte(3), Some(241_u8));
        /// assert_eq!(a_longunion.get_ubyte(4), Some(245_u8));
        /// assert_eq!(a_longunion.get_ubyte(5), Some(104_u8));
        /// assert_eq!(a_longunion.get_ubyte(6), Some(163_u8));
        /// assert_eq!(a_longunion.get_ubyte(7), Some(189_u8));
        /// assert_eq!(a_longunion.get_ubyte(8), None);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// for i in 0..16
        /// {
        ///     match a_longerunion.get_ubyte(i)
        ///     {
        ///         Some(a) => { println!("a_longerunion.get_ubyte({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longerunion.get_ubyte(0), Some(79_u8));
        /// assert_eq!(a_longerunion.get_ubyte(1), Some(11_u8));
        /// assert_eq!(a_longerunion.get_ubyte(2), Some(74_u8));
        /// assert_eq!(a_longerunion.get_ubyte(3), Some(241_u8));
        /// assert_eq!(a_longerunion.get_ubyte(4), Some(245_u8));
        /// assert_eq!(a_longerunion.get_ubyte(5), Some(104_u8));
        /// assert_eq!(a_longerunion.get_ubyte(6), Some(163_u8));
        /// assert_eq!(a_longerunion.get_ubyte(7), Some(189_u8));
        /// assert_eq!(a_longerunion.get_ubyte(8), Some(88_u8));
        /// assert_eq!(a_longerunion.get_ubyte(9), Some(136_u8));
        /// assert_eq!(a_longerunion.get_ubyte(10), Some(206_u8));
        /// assert_eq!(a_longerunion.get_ubyte(11), Some(126_u8));
        /// assert_eq!(a_longerunion.get_ubyte(12), Some(26_u8));
        /// assert_eq!(a_longerunion.get_ubyte(13), Some(59_u8));
        /// assert_eq!(a_longerunion.get_ubyte(14), Some(18_u8));
        /// assert_eq!(a_longerunion.get_ubyte(15), Some(255_u8));
        /// assert_eq!(a_longerunion.get_ubyte(16), None);
        /// ```
        /// 
        /// # Example 5 for SizeUnion for 64-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let a_sizeunion = SizeUnion::new_with(13664881099896654671_usize);
        ///     for i in 0..8
        ///     {
        ///         match a_sizeunion.get_ubyte(i)
        ///         {
        ///             Some(a) => { println!("a_sizeunion.get_ubyte({}) = {}", i, a); },
        ///             _ => {},
        ///         }
        ///     }
        ///     assert_eq!(a_sizeunion.get_ubyte(0), Some(79_u8));
        ///     assert_eq!(a_sizeunion.get_ubyte(1), Some(11_u8));
        ///     assert_eq!(a_sizeunion.get_ubyte(2), Some(74_u8));
        ///     assert_eq!(a_sizeunion.get_ubyte(3), Some(241_u8));
        ///     assert_eq!(a_sizeunion.get_ubyte(4), Some(245_u8));
        ///     assert_eq!(a_sizeunion.get_ubyte(5), Some(104_u8));
        ///     assert_eq!(a_sizeunion.get_ubyte(6), Some(163_u8));
        ///     assert_eq!(a_sizeunion.get_ubyte(7), Some(189_u8));
        ///     assert_eq!(a_sizeunion.get_ubyte(8), None);
        /// }
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[inline] pub fn get_ubyte(&self, i: usize) -> Option<u8>
        {
            if i <= Self::N
            {
                #[cfg(target_endian = "little")]    unsafe { Some(self.ubyte[i]) }
                #[cfg(target_endian = "big")]       unsafe { Some(self.ubyte[Self::N-i]) }
            }
            else
            {
                None
            }
        }

        // pub fn set_ubyte(&mut self, i: usize, val: u8) -> bool
        /// Sets i-th element of its array `ubyte` of type `u8` and returns true
        /// if `i` is less than the size of this union in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Arguments
        /// - `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// - `val` is the value of type `u8` to set the i-th element of its
        ///  array `ubyte` of type `u8`.
        /// 
        /// # Output
        /// - `true` if `i` is less than the size of this union in bytes.
        /// - `false` if `i` is equal to or greater than the size of 
        /// this union in bytes.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than the
        /// size of this union in bytes. Otherwise, you can use its counterpart
        /// method [set_ubyte_()](#method.set_ubyte_) for performance.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// let mut a_shortunion = ShortUnion::new();
        /// let b0 = a_shortunion.set_ubyte(0, 79_u8);
        /// let b1 = a_shortunion.set_ubyte(1, 11_u8);
        /// let b2 = a_shortunion.set_ubyte(2, 100_u8);  // Nothing will be done
        /// assert_eq!(b0, true);
        /// assert_eq!(b1, true);
        /// assert_eq!(b2, false);
        /// println!("a_shortunion.get() = {}", a_shortunion.get());
        /// for i in 0..2
        /// {
        ///     match a_shortunion.get_ubyte(i)
        ///     {
        ///         Some(a) => { println!("a_shortunion.get_ubyte({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_shortunion.get_ubyte(0), Some(79_u8));
        /// assert_eq!(a_shortunion.get_ubyte(1), Some(11_u8));
        /// assert_eq!(a_shortunion.get(), 2895_u16);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// let mut a_intunion = IntUnion::new();
        /// let b0 = a_intunion.set_ubyte(0, 79_u8);
        /// let b1 = a_intunion.set_ubyte(1, 11_u8);
        /// let b2 = a_intunion.set_ubyte(2, 74_u8);
        /// let b3 = a_intunion.set_ubyte(3, 241_u8);
        /// let b4 = a_intunion.set_ubyte(4, 100_u8);
        /// assert_eq!(b0, true);
        /// assert_eq!(b1, true);
        /// assert_eq!(b2, true);
        /// assert_eq!(b3, true);
        /// assert_eq!(b4, false);
        /// println!("a_intunion.get() = {}", a_intunion.get());
        /// for i in 0..4
        /// {
        ///     match a_intunion.get_ubyte(i)
        ///     {
        ///         Some(a) => { println!("a_intunion.get_ubyte({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_intunion.get_ubyte(0), Some(79_u8));
        /// assert_eq!(a_intunion.get_ubyte(1), Some(11_u8));
        /// assert_eq!(a_intunion.get_ubyte(2), Some(74_u8));
        /// assert_eq!(a_intunion.get_ubyte(3), Some(241_u8));
        /// assert_eq!(a_intunion.get(), 4048161615_u32);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let mut a_longunion = LongUnion::new();
        /// let b0 = a_longunion.set_ubyte(0, 79_u8);
        /// let b1 = a_longunion.set_ubyte(1, 11_u8);
        /// let b2 = a_longunion.set_ubyte(2, 74_u8);
        /// let b3 = a_longunion.set_ubyte(3, 241_u8);
        /// let b4 = a_longunion.set_ubyte(4, 245_u8);
        /// let b5 = a_longunion.set_ubyte(5, 104_u8);
        /// let b6 = a_longunion.set_ubyte(6, 163_u8);
        /// let b7 = a_longunion.set_ubyte(7, 189_u8);
        /// let b8 = a_longunion.set_ubyte(8, 100_u8);
        /// assert_eq!(b0, true);
        /// assert_eq!(b1, true);
        /// assert_eq!(b2, true);
        /// assert_eq!(b3, true);
        /// assert_eq!(b4, true);
        /// assert_eq!(b5, true);
        /// assert_eq!(b6, true);
        /// assert_eq!(b7, true);
        /// assert_eq!(b8, false);
        /// println!("a_longunion.get() = {}", a_longunion.get());
        /// for i in 0..8
        /// {
        ///     match a_longunion.get_ubyte(i)
        ///     {
        ///         Some(a) => { println!("a_longunion.get_ubyte({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longunion.get_ubyte(0), Some(79_u8));
        /// assert_eq!(a_longunion.get_ubyte(1), Some(11_u8));
        /// assert_eq!(a_longunion.get_ubyte(2), Some(74_u8));
        /// assert_eq!(a_longunion.get_ubyte(3), Some(241_u8));
        /// assert_eq!(a_longunion.get_ubyte(4), Some(245_u8));
        /// assert_eq!(a_longunion.get_ubyte(5), Some(104_u8));
        /// assert_eq!(a_longunion.get_ubyte(6), Some(163_u8));
        /// assert_eq!(a_longunion.get_ubyte(7), Some(189_u8));
        /// assert_eq!(a_longunion.get(), 13664881099896654671_u64);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let mut a_longerunion = LongerUnion::new();
        /// let b0 = a_longerunion.set_ubyte(0, 79_u8);
        /// let b1 = a_longerunion.set_ubyte(1, 11_u8);
        /// let b2 = a_longerunion.set_ubyte(2, 74_u8);
        /// let b3 = a_longerunion.set_ubyte(3, 241_u8);
        /// let b4 = a_longerunion.set_ubyte(4, 245_u8);
        /// let b5 = a_longerunion.set_ubyte(5, 104_u8);
        /// let b6 = a_longerunion.set_ubyte(6, 163_u8);
        /// let b7 = a_longerunion.set_ubyte(7, 189_u8);
        /// let b8 = a_longerunion.set_ubyte(8, 88_u8);
        /// let b9 = a_longerunion.set_ubyte(9, 136_u8);
        /// let b10 = a_longerunion.set_ubyte(10, 206_u8);
        /// let b11 = a_longerunion.set_ubyte(11, 126_u8);
        /// let b12 = a_longerunion.set_ubyte(12, 26_u8);
        /// let b13 = a_longerunion.set_ubyte(13, 59_u8);
        /// let b14 = a_longerunion.set_ubyte(14, 18_u8);
        /// let b15 = a_longerunion.set_ubyte(15, 255_u8);
        /// let b16 = a_longerunion.set_ubyte(16, 100_u8);
        /// assert_eq!(b0, true);
        /// assert_eq!(b1, true);
        /// assert_eq!(b2, true);
        /// assert_eq!(b3, true);
        /// assert_eq!(b4, true);
        /// assert_eq!(b5, true);
        /// assert_eq!(b6, true);
        /// assert_eq!(b7, true);
        /// assert_eq!(b8, true);
        /// assert_eq!(b9, true);
        /// assert_eq!(b10, true);
        /// assert_eq!(b11, true);
        /// assert_eq!(b12, true);
        /// assert_eq!(b13, true);
        /// assert_eq!(b14, true);
        /// assert_eq!(b15, true);
        /// assert_eq!(b16, false);
        /// println!("a_longerunion.get() = {}", a_longerunion.get());
        /// for i in 0..16
        /// {
        ///     match a_longerunion.get_ubyte(i)
        ///     {
        ///         Some(a) => { println!("a_longerunion.get_ubyte({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longerunion.get_ubyte(0), Some(79_u8));
        /// assert_eq!(a_longerunion.get_ubyte(1), Some(11_u8));
        /// assert_eq!(a_longerunion.get_ubyte(2), Some(74_u8));
        /// assert_eq!(a_longerunion.get_ubyte(3), Some(241_u8));
        /// assert_eq!(a_longerunion.get_ubyte(4), Some(245_u8));
        /// assert_eq!(a_longerunion.get_ubyte(5), Some(104_u8));
        /// assert_eq!(a_longerunion.get_ubyte(6), Some(163_u8));
        /// assert_eq!(a_longerunion.get_ubyte(7), Some(189_u8));
        /// assert_eq!(a_longerunion.get_ubyte(8), Some(88_u8));
        /// assert_eq!(a_longerunion.get_ubyte(9), Some(136_u8));
        /// assert_eq!(a_longerunion.get_ubyte(10), Some(206_u8));
        /// assert_eq!(a_longerunion.get_ubyte(11), Some(126_u8));
        /// assert_eq!(a_longerunion.get_ubyte(12), Some(26_u8));
        /// assert_eq!(a_longerunion.get_ubyte(13), Some(59_u8));
        /// assert_eq!(a_longerunion.get_ubyte(14), Some(18_u8));
        /// assert_eq!(a_longerunion.get_ubyte(15), Some(255_u8));
        /// assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
        /// ```
        /// 
        /// # Example 5 for SizeUnion for 64-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let mut a_sizeunion = SizeUnion::new();
        ///     let b0 = a_sizeunion.set_ubyte(0, 79_u8);
        ///     let b1 = a_sizeunion.set_ubyte(1, 11_u8);
        ///     let b2 = a_sizeunion.set_ubyte(2, 74_u8);
        ///     let b3 = a_sizeunion.set_ubyte(3, 241_u8);
        ///     let b4 = a_sizeunion.set_ubyte(4, 245_u8);
        ///     let b5 = a_sizeunion.set_ubyte(5, 104_u8);
        ///     let b6 = a_sizeunion.set_ubyte(6, 163_u8);
        ///     let b7 = a_sizeunion.set_ubyte(7, 189_u8);
        ///     let b8 = a_sizeunion.set_ubyte(8, 100_u8);
        ///     assert_eq!(b0, true);
        ///     assert_eq!(b1, true);
        ///     assert_eq!(b2, true);
        ///     assert_eq!(b3, true);
        ///     assert_eq!(b4, true);
        ///     assert_eq!(b5, true);
        ///     assert_eq!(b6, true);
        ///     assert_eq!(b7, true);
        ///     assert_eq!(b8, false);
        ///     println!("a_sizeunion.get() = {}", a_sizeunion.get());
        ///     for i in 0..8
        ///     {
        ///         match a_sizeunion.get_ubyte(i)
        ///         {
        ///             Some(a) => { println!("a_sizeunion.get_ubyte({}) = {}", i, a); },
        ///             _ => {},
        ///         }
        ///     }
        ///     assert_eq!(a_sizeunion.get_ubyte(0), Some(79_u8));
        ///     assert_eq!(a_sizeunion.get_ubyte(1), Some(11_u8));
        ///     assert_eq!(a_sizeunion.get_ubyte(2), Some(74_u8));
        ///     assert_eq!(a_sizeunion.get_ubyte(3), Some(241_u8));
        ///     assert_eq!(a_sizeunion.get_ubyte(4), Some(245_u8));
        ///     assert_eq!(a_sizeunion.get_ubyte(5), Some(104_u8));
        ///     assert_eq!(a_sizeunion.get_ubyte(6), Some(163_u8));
        ///     assert_eq!(a_sizeunion.get_ubyte(7), Some(189_u8));
        ///     assert_eq!(a_sizeunion.get(), 13664881099896654671_usize);
        /// }
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        pub fn set_ubyte(&mut self, i: usize, val: u8) -> bool
        {
            if i <= Self::N
            { 
                #[cfg(target_endian = "little")]    unsafe { self.ubyte[i] = val; }
                #[cfg(target_endian = "big")]       unsafe { self.ubyte[Self::N-i] = val; }
                true
            }
            else
            {
                false
            }
        }

        // pub fn get_sbyte_(&self, i: usize) -> i8
        /// Returns i-th element of array `sbyte` of type `i8`
        /// if `i` is less than the size of this union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Output
        /// i-th element of array `sbyte` of type `i8`
        /// 
        /// # Arguments
        /// `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to the size of this union in
        /// bytes, it will panic.
        /// 
        /// # Caution
        /// Use this method only when you are sure
        /// that the argument `i` is less than the size of this union.
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_sbyte(). This method
        /// get_sbyte_() is considered to be slightly faster than the method
        /// get_sbyte().
        /// Use this method only when you are sure that `i` is less than the
        /// size of this union in bytes. Otherwise, use its counterpart method
        /// [get_sbyte()](#method.get_sbyte) for safety.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// let a_shortunion = ShortUnion::new_with(2895_u16);
        /// for i in 0..2
        ///     { println!("a_shortunion.get_sbyte_({}) = {}", i, a_shortunion.get_sbyte_(i)); }
        /// assert_eq!(a_shortunion.get_sbyte_(0), 79_i8);
        /// assert_eq!(a_shortunion.get_sbyte_(1), 11_i8);
        /// 
        /// // It will panic.
        /// // println!("a_shortunion.get_sbyte_(2) = {}", a_shortunion.get_sbyte_(2));
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// let a_intunion = IntUnion::new_with(4048161615_u32);
        /// for i in 0..4
        ///     { println!("a_intunion.get_sbyte_({}) = {}", i, a_intunion.get_sbyte_(i)); }
        /// assert_eq!(a_intunion.get_sbyte_(0), 79_i8);
        /// assert_eq!(a_intunion.get_sbyte_(1), 11_i8);
        /// assert_eq!(a_intunion.get_sbyte_(2), 74_i8);
        /// assert_eq!(a_intunion.get_sbyte_(3), -15_i8);
        /// 
        /// // It will panic.
        /// // println!("a_intunion.get_sbyte_(4) = {}", a_intunion.get_sbyte_(4));
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let a_longunion = LongUnion::new_with(13664881099896654671_u64);
        /// for i in 0..8
        ///     { println!("a_longunion.get_sbyte_({}) = {}", i, a_longunion.get_sbyte_(i)); }
        /// assert_eq!(a_longunion.get_sbyte_(0), 79_i8);
        /// assert_eq!(a_longunion.get_sbyte_(1), 11_i8);
        /// assert_eq!(a_longunion.get_sbyte_(2), 74_i8);
        /// assert_eq!(a_longunion.get_sbyte_(3), -15_i8);
        /// assert_eq!(a_longunion.get_sbyte_(4), -11_i8);
        /// assert_eq!(a_longunion.get_sbyte_(5), 104_i8);
        /// assert_eq!(a_longunion.get_sbyte_(6), -93_i8);
        /// assert_eq!(a_longunion.get_sbyte_(7), -67_i8);
        /// 
        /// // It will panic.
        /// // println!("a_longunion.get_sbyte_(8) = {}", a_longunion.get_sbyte_(8));
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// for i in 0..16
        ///     { println!("a_longerunion.get_sbyte_({}) = {}", i, a_longerunion.get_sbyte_(i)); }
        /// assert_eq!(a_longerunion.get_sbyte_(0), 79_i8);
        /// assert_eq!(a_longerunion.get_sbyte_(1), 11_i8);
        /// assert_eq!(a_longerunion.get_sbyte_(2), 74_i8);
        /// assert_eq!(a_longerunion.get_sbyte_(3), -15_i8);
        /// assert_eq!(a_longerunion.get_sbyte_(4), -11_i8);
        /// assert_eq!(a_longerunion.get_sbyte_(5), 104_i8);
        /// assert_eq!(a_longerunion.get_sbyte_(6), -93_i8);
        /// assert_eq!(a_longerunion.get_sbyte_(7), -67_i8);
        /// assert_eq!(a_longerunion.get_sbyte_(8), 88_i8);
        /// assert_eq!(a_longerunion.get_sbyte_(9), -120_i8);
        /// assert_eq!(a_longerunion.get_sbyte_(10), -50_i8);
        /// assert_eq!(a_longerunion.get_sbyte_(11), 126_i8);
        /// assert_eq!(a_longerunion.get_sbyte_(12), 26_i8);
        /// assert_eq!(a_longerunion.get_sbyte_(13), 59_i8);
        /// assert_eq!(a_longerunion.get_sbyte_(14), 18_i8);
        /// assert_eq!(a_longerunion.get_sbyte_(15), -1_i8);
        /// 
        /// // It will panic.
        /// // println!("a_longerunion.get_sbyte_(16) = {}", a_longerunion.get_sbyte_(16));
        /// ```
        /// 
        /// # Example 5 for SizeUnion for 64-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let a_sizeunion = SizeUnion::new_with(13664881099896654671_usize);
        ///     for i in 0..8
        ///         { println!("a_sizeunion.get_ubyte_({}) = {}", i, a_sizeunion.get_ubyte_(i)); }
        ///     assert_eq!(a_sizeunion.get_sbyte_(0), 79_i8);
        ///     assert_eq!(a_sizeunion.get_sbyte_(1), 11_i8);
        ///     assert_eq!(a_sizeunion.get_sbyte_(2), 74_i8);
        ///     assert_eq!(a_sizeunion.get_sbyte_(3), -15_i8);
        ///     assert_eq!(a_sizeunion.get_sbyte_(4), -11_i8);
        ///     assert_eq!(a_sizeunion.get_sbyte_(5), 104_i8);
        ///     assert_eq!(a_sizeunion.get_sbyte_(6), -93_i8);
        ///     assert_eq!(a_sizeunion.get_sbyte_(7), -67_i8);
        /// 
        ///     // It will panic.
        ///     // println!("a_sizeunion.get_sbyte_(8) = {}", a_sizeunion.get_sbyte_(8));
        /// }
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[inline] pub fn get_sbyte_(&self, i: usize) -> i8
        {
            #[cfg(target_endian = "little")]    unsafe { self.sbyte[i] }
            #[cfg(target_endian = "big")]       unsafe { self.sbyte[Self::N-i] }
        }

        // pub fn set_sbyte_(&mut self, i: usize, val: i8)
        /// Sets i-th element of its array `sbyte` of type `i8`
        /// if `i` is less than the size of this union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Arguments
        /// - `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// - `val` is the value of type `i8` to set the i-th element of its
        ///  array `sbyte` of type `u8`.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to the size of this union in
        /// bytes, it will panic.
        /// 
        /// # Caution
        /// Use this method only when you are sure
        /// that the argument `i` is less than the size of this union.
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_sbyte(). This method
        /// set_sbyte_() is considered to be slightly faster than the method
        /// set_sbyte().
        /// Use this method only when you are sure that `i` is less than the
        /// size of this union in bytes. Otherwise, use its counterpart method
        /// [set_sbyte()](#method.set_sbyte) for safety.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// let mut a_shortunion = ShortUnion::new();
        /// a_shortunion.set_sbyte_(0, 79_i8);
        /// a_shortunion.set_sbyte_(1, 11_i8);
        /// // It will panic.
        /// // a_shortunion.set_sbyte_(2, 100_i8);
        /// println!("a_shortunion.get() = {}", a_shortunion.get());
        /// for i in 0..2
        ///     { println!("a_shortunion.get_sbyte_({}) = {}", i, a_shortunion.get_sbyte_(i)); }
        /// assert_eq!(a_shortunion.get_sbyte_(0), 79_i8);
        /// assert_eq!(a_shortunion.get_sbyte_(1), 11_i8);
        /// assert_eq!(a_shortunion.get(), 2895_u16);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// let mut a_intunion = IntUnion::new();
        /// a_intunion.set_sbyte_(0, 79_i8);
        /// a_intunion.set_sbyte_(1, 11_i8);
        /// a_intunion.set_sbyte_(2, 74_i8);
        /// a_intunion.set_sbyte_(3, -15_i8);
        /// // It will panic.
        /// // a_intunion.set_sbyte_(4, 100_i8);
        /// println!("a_intunion.get() = {}", a_intunion.get());
        /// for i in 0..4
        ///     { println!("a_intunion.get_sbyte_({}) = {}", i, a_intunion.get_sbyte_(i)); }
        /// assert_eq!(a_intunion.get_sbyte_(0), 79_i8);
        /// assert_eq!(a_intunion.get_sbyte_(1), 11_i8);
        /// assert_eq!(a_intunion.get_sbyte_(2), 74_i8);
        /// assert_eq!(a_intunion.get_sbyte_(3), -15_i8);
        /// assert_eq!(a_intunion.get(), 4048161615_u32);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let mut a_longunion = LongUnion::new();
        /// a_longunion.set_sbyte_(0, 79_i8);
        /// a_longunion.set_sbyte_(1, 11_i8);
        /// a_longunion.set_sbyte_(2, 74_i8);
        /// a_longunion.set_sbyte_(3, -15_i8);
        /// a_longunion.set_sbyte_(4, -11_i8);
        /// a_longunion.set_sbyte_(5, 104_i8);
        /// a_longunion.set_sbyte_(6, -93_i8);
        /// a_longunion.set_sbyte_(7, -67_i8);
        /// // It will panic.
        /// // a_intunion.set_sbyte_(8, 100_i8);
        /// println!("a_longunion.get() = {}", a_longunion.get());
        /// for i in 0..8
        ///     { println!("a_longunion.get_sbyte_({}) = {}", i, a_longunion.get_sbyte_(i)); }
        /// assert_eq!(a_longunion.get_sbyte_(0), 79_i8);
        /// assert_eq!(a_longunion.get_sbyte_(1), 11_i8);
        /// assert_eq!(a_longunion.get_sbyte_(2), 74_i8);
        /// assert_eq!(a_longunion.get_sbyte_(3), -15_i8);
        /// assert_eq!(a_longunion.get_sbyte_(4), -11_i8);
        /// assert_eq!(a_longunion.get_sbyte_(5), 104_i8);
        /// assert_eq!(a_longunion.get_sbyte_(6), -93_i8);
        /// assert_eq!(a_longunion.get_sbyte_(7), -67_i8);
        /// assert_eq!(a_longunion.get(), 13664881099896654671_u64);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let mut a_longerunion = LongerUnion::new();
        /// a_longerunion.set_sbyte_(0, 79_i8);
        /// a_longerunion.set_sbyte_(1, 11_i8);
        /// a_longerunion.set_sbyte_(2, 74_i8);
        /// a_longerunion.set_sbyte_(3, -15_i8);
        /// a_longerunion.set_sbyte_(4, -11_i8);
        /// a_longerunion.set_sbyte_(5, 104_i8);
        /// a_longerunion.set_sbyte_(6, -93_i8);
        /// a_longerunion.set_sbyte_(7, -67_i8);
        /// a_longerunion.set_sbyte_(8, 88_i8);
        /// a_longerunion.set_sbyte_(9, -120_i8);
        /// a_longerunion.set_sbyte_(10, -50_i8);
        /// a_longerunion.set_sbyte_(11, 126_i8);
        /// a_longerunion.set_sbyte_(12, 26_i8);
        /// a_longerunion.set_sbyte_(13, 59_i8);
        /// a_longerunion.set_sbyte_(14, 18_i8);
        /// a_longerunion.set_sbyte_(15, -1_i8);
        /// // It will panic.
        /// // a_longerunion.set_sbyte_(16, 100_i8);
        /// println!("a_longerunion.get() = {}", a_longerunion.get());
        /// for i in 0..16
        ///     { println!("a_longerunion.get_sbyte_({}) = {}", i, a_longerunion.get_sbyte_(i)); }
        /// assert_eq!(a_longerunion.get_sbyte_(0), 79_i8);
        /// assert_eq!(a_longerunion.get_sbyte_(1), 11_i8);
        /// assert_eq!(a_longerunion.get_sbyte_(2), 74_i8);
        /// assert_eq!(a_longerunion.get_sbyte_(3), -15_i8);
        /// assert_eq!(a_longerunion.get_sbyte_(4), -11_i8);
        /// assert_eq!(a_longerunion.get_sbyte_(5), 104_i8);
        /// assert_eq!(a_longerunion.get_sbyte_(6), -93_i8);
        /// assert_eq!(a_longerunion.get_sbyte_(7), -67_i8);
        /// assert_eq!(a_longerunion.get_sbyte_(8), 88_i8);
        /// assert_eq!(a_longerunion.get_sbyte_(9), -120_i8);
        /// assert_eq!(a_longerunion.get_sbyte_(10), -50_i8);
        /// assert_eq!(a_longerunion.get_sbyte_(11), 126_i8);
        /// assert_eq!(a_longerunion.get_sbyte_(12), 26_i8);
        /// assert_eq!(a_longerunion.get_sbyte_(13), 59_i8);
        /// assert_eq!(a_longerunion.get_sbyte_(14), 18_i8);
        /// assert_eq!(a_longerunion.get_sbyte_(15), -1_i8);
        /// assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
        /// ```
        /// 
        /// # Example 5 for SizeUnion for 64-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let mut a_sizeunion = SizeUnion::new();
        ///     a_sizeunion.set_sbyte_(0, 79_i8);
        ///     a_sizeunion.set_sbyte_(1, 11_i8);
        ///     a_sizeunion.set_sbyte_(2, 74_i8);
        ///     a_sizeunion.set_sbyte_(3, -15_i8);
        ///     a_sizeunion.set_sbyte_(4, -11_i8);
        ///     a_sizeunion.set_sbyte_(5, 104_i8);
        ///     a_sizeunion.set_sbyte_(6, -93_i8);
        ///     a_sizeunion.set_sbyte_(7, -67_i8);
        ///     // It will panic.
        ///     // a_sizeunion.set_sbyte_(8, 100_i8);
        ///     println!("a_sizeunion.get() = {}", a_sizeunion.get());
        ///     for i in 0..8
        ///         { println!("a_sizeunion.get_ubyte_({}) = {}", i, a_sizeunion.get_ubyte_(i)); }
        ///     assert_eq!(a_sizeunion.get_sbyte_(0), 79_i8);
        ///     assert_eq!(a_sizeunion.get_sbyte_(1), 11_i8);
        ///     assert_eq!(a_sizeunion.get_sbyte_(2), 74_i8);
        ///     assert_eq!(a_sizeunion.get_sbyte_(3), -15_i8);
        ///     assert_eq!(a_sizeunion.get_sbyte_(4), -11_i8);
        ///     assert_eq!(a_sizeunion.get_sbyte_(5), 104_i8);
        ///     assert_eq!(a_sizeunion.get_sbyte_(6), -93_i8);
        ///     assert_eq!(a_sizeunion.get_sbyte_(7), -67_i8);
        ///     assert_eq!(a_sizeunion.get(), 13664881099896654671_usize);
        /// }
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[inline] pub fn set_sbyte_(&mut self, i: usize, val: i8)
        {
            #[cfg(target_endian = "little")]    unsafe { self.sbyte[i] = val; }
            #[cfg(target_endian = "big")]       unsafe { self.sbyte[Self::N-i] = val; }
        }

        // pub fn get_sbyte(&self, i: usize) -> Option<i8>
        /// Returns i-th element of array `sbyte` of type `u8` wrapped in `Some`
        /// of enum `Option` if `i` is less than the size of this union in bytes.
        /// Otherwise, it returns None of enum `Option`.
        /// 
        /// # Output
        /// - i-th element of array `sbyte` of type `i8` wrapped in `Some`
        /// of enum `Option` if `i` is less than the size of this union
        /// in bytes.
        /// - `None` if `i` is greater than or equal to the size of
        /// this union in bytes.
        /// 
        /// # Arguments
        /// `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than the
        /// size of this union in bytes. Otherwise, you can use its counterpart
        /// method [get_sbyte_()](#method.get_sbyte_) for performance.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// let a_shortunion = ShortUnion::new_with(2895_u16);
        /// for i in 0..2
        /// {
        ///     match a_shortunion.get_sbyte(i)
        ///     {
        ///         Some(a) => { println!("a_shortunion.get_sbyte({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_shortunion.get_sbyte(0), Some(79_i8));
        /// assert_eq!(a_shortunion.get_sbyte(1), Some(11_i8));
        /// assert_eq!(a_shortunion.get_sbyte(2), None);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// let a_intunion = IntUnion::new_with(4048161615_u32);
        /// for i in 0..4
        /// {
        ///     match a_intunion.get_sbyte(i)
        ///     {
        ///         Some(a) => { println!("a_intunion.get_sbyte({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_intunion.get_sbyte(0), Some(79_i8));
        /// assert_eq!(a_intunion.get_sbyte(1), Some(11_i8));
        /// assert_eq!(a_intunion.get_sbyte(2), Some(74_i8));
        /// assert_eq!(a_intunion.get_sbyte(3), Some(-15_i8));
        /// assert_eq!(a_intunion.get_sbyte(4), None);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let a_longunion = LongUnion::new_with(13664881099896654671_u64);
        /// for i in 0..8
        /// {
        ///     match a_longunion.get_sbyte(i)
        ///     {
        ///         Some(a) => { println!("a_longunion.get_sbyte({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longunion.get_sbyte(0), Some(79_i8));
        /// assert_eq!(a_longunion.get_sbyte(1), Some(11_i8));
        /// assert_eq!(a_longunion.get_sbyte(2), Some(74_i8));
        /// assert_eq!(a_longunion.get_sbyte(3), Some(-15_i8));
        /// assert_eq!(a_longunion.get_sbyte(4), Some(-11_i8));
        /// assert_eq!(a_longunion.get_sbyte(5), Some(104_i8));
        /// assert_eq!(a_longunion.get_sbyte(6), Some(-93_i8));
        /// assert_eq!(a_longunion.get_sbyte(7), Some(-67_i8));
        /// assert_eq!(a_longunion.get_sbyte(8), None);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// for i in 0..16
        /// {
        ///     match a_longerunion.get_sbyte(i)
        ///     {
        ///         Some(a) => { println!("a_longerunion.get_sbyte({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longerunion.get_sbyte(0), Some(79_i8));
        /// assert_eq!(a_longerunion.get_sbyte(1), Some(11_i8));
        /// assert_eq!(a_longerunion.get_sbyte(2), Some(74_i8));
        /// assert_eq!(a_longerunion.get_sbyte(3), Some(-15_i8));
        /// assert_eq!(a_longerunion.get_sbyte(4), Some(-11_i8));
        /// assert_eq!(a_longerunion.get_sbyte(5), Some(104_i8));
        /// assert_eq!(a_longerunion.get_sbyte(6), Some(-93_i8));
        /// assert_eq!(a_longerunion.get_sbyte(7), Some(-67_i8));
        /// assert_eq!(a_longerunion.get_sbyte(8), Some(88_i8));
        /// assert_eq!(a_longerunion.get_sbyte(9), Some(-120_i8));
        /// assert_eq!(a_longerunion.get_sbyte(10), Some(-50_i8));
        /// assert_eq!(a_longerunion.get_sbyte(11), Some(126_i8));
        /// assert_eq!(a_longerunion.get_sbyte(12), Some(26_i8));
        /// assert_eq!(a_longerunion.get_sbyte(13), Some(59_i8));
        /// assert_eq!(a_longerunion.get_sbyte(14), Some(18_i8));
        /// assert_eq!(a_longerunion.get_sbyte(15), Some(-1_i8));
        /// assert_eq!(a_longerunion.get_sbyte(16), None);
        /// ```
        /// 
        /// # Example 5 for SizeUnion for 64-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let a_sizeunion = SizeUnion::new_with(13664881099896654671_usize);
        ///     for i in 0..8
        ///     {
        ///         match a_sizeunion.get_sbyte(i)
        ///         {
        ///             Some(a) => { println!("a_sizeunion.get_sbyte({}) = {}", i, a); },
        ///             _ => {},
        ///         }
        ///     }
        ///     assert_eq!(a_sizeunion.get_sbyte(0), Some(79_i8));
        ///     assert_eq!(a_sizeunion.get_sbyte(1), Some(11_i8));
        ///     assert_eq!(a_sizeunion.get_sbyte(2), Some(74_i8));
        ///     assert_eq!(a_sizeunion.get_sbyte(3), Some(-15_i8));
        ///     assert_eq!(a_sizeunion.get_sbyte(4), Some(-11_i8));
        ///     assert_eq!(a_sizeunion.get_sbyte(5), Some(104_i8));
        ///     assert_eq!(a_sizeunion.get_sbyte(6), Some(-93_i8));
        ///     assert_eq!(a_sizeunion.get_sbyte(7), Some(-67_i8));
        ///     assert_eq!(a_sizeunion.get_sbyte(8), None);
        /// }
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        pub fn get_sbyte(&self, i: usize) -> Option<i8>
        {
            if i <= Self::N
            {
                #[cfg(target_endian = "little")]    unsafe { Some(self.sbyte[i]) }
                #[cfg(target_endian = "big")]       unsafe { Some(self.sbyte[Self::N-i]) }
            }
            else
            {
                None
            }
        }

        // pub fn set_sbyte(&mut self, i: usize, val: i8) -> bool
        /// Sets i-th element of its array `sbyte` of type `i8` and returns
        /// true if `i` is less than the size of this union in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Output
        /// - `true` if `i` is less than the size of this union in bytes.
        /// - `false` if `i` is equal to or greater than the size of 
        /// this union in bytes.
        /// 
        /// # Arguments
        /// - `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// - `val` is the value of type `u8` to set the i-th element of its
        ///  array `sbyte` of type `i8`.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than the
        /// size of this union in bytes. Otherwise, you can use its counterpart
        /// method [set_sbyte_()](#method.set_sbyte_) for performance.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// let mut a_shortunion = ShortUnion::new();
        /// let b0 = a_shortunion.set_sbyte(0, 79_i8);
        /// let b1 = a_shortunion.set_sbyte(1, 11_i8);
        /// let b2 = a_shortunion.set_sbyte(2, 100_i8);  // Nothing will be done
        /// assert_eq!(b0, true);
        /// assert_eq!(b1, true);
        /// assert_eq!(b2, false);
        /// println!("a_shortunion.get() = {}", a_shortunion.get());
        /// for i in 0..2
        /// {
        ///     match a_shortunion.get_sbyte(i)
        ///     {
        ///         Some(a) => { println!("a_shortunion.get_sbyte({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_shortunion.get_sbyte(0), Some(79_i8));
        /// assert_eq!(a_shortunion.get_sbyte(1), Some(11_i8));
        /// assert_eq!(a_shortunion.get_sbyte(2), None);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// let mut a_intunion = IntUnion::new();
        /// let b0 = a_intunion.set_sbyte(0, 79_i8);
        /// let b1 = a_intunion.set_sbyte(1, 11_i8);
        /// let b2 = a_intunion.set_sbyte(2, 74_i8);
        /// let b3 = a_intunion.set_sbyte(3, -15_i8);
        /// let b4 = a_intunion.set_sbyte(4, 100_i8);
        /// assert_eq!(b0, true);
        /// assert_eq!(b1, true);
        /// assert_eq!(b2, true);
        /// assert_eq!(b3, true);
        /// assert_eq!(b4, false);
        /// println!("a_intunion.get() = {}", a_intunion.get());
        /// for i in 0..4
        /// {
        ///     match a_intunion.get_sbyte(i)
        ///     {
        ///         Some(a) => { println!("a_intunion.get_sbyte({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_intunion.get_sbyte(0), Some(79_i8));
        /// assert_eq!(a_intunion.get_sbyte(1), Some(11_i8));
        /// assert_eq!(a_intunion.get_sbyte(2), Some(74_i8));
        /// assert_eq!(a_intunion.get_sbyte(3), Some(-15_i8));
        /// assert_eq!(a_intunion.get(), 4048161615_u32);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let mut a_longunion = LongUnion::new();
        /// let b0 = a_longunion.set_sbyte(0, 79_i8);
        /// let b1 = a_longunion.set_sbyte(1, 11_i8);
        /// let b2 = a_longunion.set_sbyte(2, 74_i8);
        /// let b3 = a_longunion.set_sbyte(3, -15_i8);
        /// let b4 = a_longunion.set_sbyte(4, -11_i8);
        /// let b5 = a_longunion.set_sbyte(5, 104_i8);
        /// let b6 = a_longunion.set_sbyte(6, -93_i8);
        /// let b7 = a_longunion.set_sbyte(7, -67_i8);
        /// let b8 = a_longunion.set_sbyte(8, 100_i8);
        /// assert_eq!(b0, true);
        /// assert_eq!(b1, true);
        /// assert_eq!(b2, true);
        /// assert_eq!(b3, true);
        /// assert_eq!(b4, true);
        /// assert_eq!(b5, true);
        /// assert_eq!(b6, true);
        /// assert_eq!(b7, true);
        /// assert_eq!(b8, false);
        /// println!("a_longunion.get() = {}", a_longunion.get());
        /// for i in 0..8
        /// {
        ///     match a_longunion.get_ubyte(i)
        ///     {
        ///         Some(a) => { println!("a_longunion.get_ubyte({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longunion.get_sbyte(0), Some(79_i8));
        /// assert_eq!(a_longunion.get_sbyte(1), Some(11_i8));
        /// assert_eq!(a_longunion.get_sbyte(2), Some(74_i8));
        /// assert_eq!(a_longunion.get_sbyte(3), Some(-15_i8));
        /// assert_eq!(a_longunion.get_sbyte(4), Some(-11_i8));
        /// assert_eq!(a_longunion.get_sbyte(5), Some(104_i8));
        /// assert_eq!(a_longunion.get_sbyte(6), Some(-93_i8));
        /// assert_eq!(a_longunion.get_sbyte(7), Some(-67_i8));
        /// assert_eq!(a_longunion.get(), 13664881099896654671_u64);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let mut a_longerunion = LongerUnion::new();
        /// let b0 = a_longerunion.set_sbyte(0, 79_i8);
        /// let b1 = a_longerunion.set_sbyte(1, 11_i8);
        /// let b2 = a_longerunion.set_sbyte(2, 74_i8);
        /// let b3 = a_longerunion.set_sbyte(3, -15_i8);
        /// let b4 = a_longerunion.set_sbyte(4, -11_i8);
        /// let b5 = a_longerunion.set_sbyte(5, 104_i8);
        /// let b6 = a_longerunion.set_sbyte(6, -93_i8);
        /// let b7 = a_longerunion.set_sbyte(7, -67_i8);
        /// let b8 = a_longerunion.set_sbyte(8, 88_i8);
        /// let b9 = a_longerunion.set_sbyte(9, -120_i8);
        /// let b10 = a_longerunion.set_sbyte(10, -50_i8);
        /// let b11 = a_longerunion.set_sbyte(11, 126_i8);
        /// let b12 = a_longerunion.set_sbyte(12, 26_i8);
        /// let b13 = a_longerunion.set_sbyte(13, 59_i8);
        /// let b14 = a_longerunion.set_sbyte(14, 18_i8);
        /// let b15 = a_longerunion.set_sbyte(15, -1_i8);
        /// let b16 = a_longerunion.set_sbyte(16, 100_i8);
        /// assert_eq!(b0, true);
        /// assert_eq!(b1, true);
        /// assert_eq!(b2, true);
        /// assert_eq!(b3, true);
        /// assert_eq!(b4, true);
        /// assert_eq!(b5, true);
        /// assert_eq!(b6, true);
        /// assert_eq!(b7, true);
        /// assert_eq!(b8, true);
        /// assert_eq!(b9, true);
        /// assert_eq!(b10, true);
        /// assert_eq!(b11, true);
        /// assert_eq!(b12, true);
        /// assert_eq!(b13, true);
        /// assert_eq!(b14, true);
        /// assert_eq!(b15, true);
        /// assert_eq!(b16, false);
        /// println!("a_longerunion.get() = {}", a_longerunion.get());
        /// for i in 0..16
        /// {
        ///     match a_longerunion.get_ubyte(i)
        ///     {
        ///         Some(a) => { println!("a_longerunion.get_ubyte({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longerunion.get_sbyte(0), Some(79_i8));
        /// assert_eq!(a_longerunion.get_sbyte(1), Some(11_i8));
        /// assert_eq!(a_longerunion.get_sbyte(2), Some(74_i8));
        /// assert_eq!(a_longerunion.get_sbyte(3), Some(-15_i8));
        /// assert_eq!(a_longerunion.get_sbyte(4), Some(-11_i8));
        /// assert_eq!(a_longerunion.get_sbyte(5), Some(104_i8));
        /// assert_eq!(a_longerunion.get_sbyte(6), Some(-93_i8));
        /// assert_eq!(a_longerunion.get_sbyte(7), Some(-67_i8));
        /// assert_eq!(a_longerunion.get_sbyte(8), Some(88_i8));
        /// assert_eq!(a_longerunion.get_sbyte(9), Some(-120_i8));
        /// assert_eq!(a_longerunion.get_sbyte(10), Some(-50_i8));
        /// assert_eq!(a_longerunion.get_sbyte(11), Some(126_i8));
        /// assert_eq!(a_longerunion.get_sbyte(12), Some(26_i8));
        /// assert_eq!(a_longerunion.get_sbyte(13), Some(59_i8));
        /// assert_eq!(a_longerunion.get_sbyte(14), Some(18_i8));
        /// assert_eq!(a_longerunion.get_sbyte(15), Some(-1_i8));
        /// assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
        /// ```
        /// 
        /// # Example 5 for SizeUnion for 64-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let mut a_sizeunion = SizeUnion::new();
        ///     let b0 = a_sizeunion.set_sbyte(0, 79_i8);
        ///     let b1 = a_sizeunion.set_sbyte(1, 11_i8);
        ///     let b2 = a_sizeunion.set_sbyte(2, 74_i8);
        ///     let b3 = a_sizeunion.set_sbyte(3, -15_i8);
        ///     let b4 = a_sizeunion.set_sbyte(4, -11_i8);
        ///     let b5 = a_sizeunion.set_sbyte(5, 104_i8);
        ///     let b6 = a_sizeunion.set_sbyte(6, -93_i8);
        ///     let b7 = a_sizeunion.set_sbyte(7, -67_i8);
        ///     let b8 = a_sizeunion.set_sbyte(8, 100_i8);
        ///     assert_eq!(b0, true);
        ///     assert_eq!(b1, true);
        ///     assert_eq!(b2, true);
        ///     assert_eq!(b3, true);
        ///     assert_eq!(b4, true);
        ///     assert_eq!(b5, true);
        ///     assert_eq!(b6, true);
        ///     assert_eq!(b7, true);
        ///     assert_eq!(b8, false);
        ///     println!("a_sizeunion.get() = {}", a_sizeunion.get());
        ///     for i in 0..8
        ///     {
        ///         match a_sizeunion.get_sbyte(i)
        ///         {
        ///             Some(a) => { println!("a_sizeunion.get_sbyte({}) = {}", i, a); },
        ///             _ => {},
        ///         }
        ///     }
        ///     assert_eq!(a_sizeunion.get_sbyte(0), Some(79_i8));
        ///     assert_eq!(a_sizeunion.get_sbyte(1), Some(11_i8));
        ///     assert_eq!(a_sizeunion.get_sbyte(2), Some(74_i8));
        ///     assert_eq!(a_sizeunion.get_sbyte(3), Some(-15_i8));
        ///     assert_eq!(a_sizeunion.get_sbyte(4), Some(-11_i8));
        ///     assert_eq!(a_sizeunion.get_sbyte(5), Some(104_i8));
        ///     assert_eq!(a_sizeunion.get_sbyte(6), Some(-93_i8));
        ///     assert_eq!(a_sizeunion.get_sbyte(7), Some(-67_i8));
        ///     assert_eq!(a_sizeunion.get(), 13664881099896654671_usize);
        /// }
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        pub fn set_sbyte(&mut self, i: usize, val: i8) -> bool
        {
            if i <= Self::N
            { 
                #[cfg(target_endian = "little")]    unsafe { self.sbyte[i] = val; }
                #[cfg(target_endian = "big")]       unsafe { self.sbyte[Self::N-i] = val; }
                true
            }
            else
            {
                false
            }
        }
    }
}
pub(super) use get_set_byte;


macro_rules! get_set_short_fit {
    () => {
        // pub fn get_ushort(self) -> u16
        /// Returns its value as `u16`.
        /// 
        /// # Output
        /// Its value as `u16`
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// let a = ShortUnion::new_with(55468_u16);
        /// println!("a = {}", a.get_ushort());
        /// assert_eq!(a.get_ushort(), 55468_u16);
        /// ```
        #[inline] pub fn get_ushort(self) -> u16    { unsafe { self.ushort } }

        // pub fn set_ushort(&mut self, val: u16)
        /// Sets its value with `val` of type `u16`.
        /// 
        /// # Arguments
        /// `val` is the value of type `u16`.
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::ShortUnion;    
        /// let mut a = ShortUnion::new();
        /// a.set_ushort(54321_u16);
        /// println!("a = {}", a.get_ushort());
        /// assert_eq!(a.get_ushort(), 54321_u16);
        /// ```
        #[inline] pub fn set_ushort(&mut self, val: u16)    { self.ushort = val; }

        // pub fn get_sshort(self) -> i16
        /// Returns its value as `i16`.
        /// 
        /// # Output
        /// Its value as `i16`
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::ShortUnion;    
        /// let a = ShortUnion::new_with(54321_u16);
        /// println!("a = {}", a.get_sshort());
        /// assert_eq!(a.get_sshort(), -11215_i16);
        /// ```
        #[inline] pub fn get_sshort(self) -> i16    { unsafe { self.sshort } }

        // pub fn set_sshort(&mut self, val: i16)
        /// Sets its value with `val` of type `i16`.
        /// 
        /// # Arguments
        /// `val` is the value of type `i16`.
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::ShortUnion;    
        /// let mut a = ShortUnion::new();
        /// a.set_sshort(-11215_i16);
        /// println!("a = {}", a.get_sshort());
        /// assert_eq!(a.get_sshort(), -11215_i16);
        /// ```
        #[inline] pub fn set_sshort(&mut self, val: i16)    { self.sshort = val; }
    };
}
pub(super) use get_set_short_fit;

macro_rules! get_set_short {
    ($f:expr) => {
        const M: usize = $f - 1;

        /// Returns i-th element of array `ushort` of type `u16`
        /// if `i` is less than a half of the size of this union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Output
        /// i-th element of array `ushort` of type `u16`
        /// 
        /// # Arguments
        /// `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a half of the size of this
        /// union in bytes, it will panic.
        /// 
        /// # Caution
        /// So, use this method only when you are sure that the argument `i` is
        /// less than a half of the size of this union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_ushort(). This method
        /// get_ushort_() is considered to be slightly faster than the method
        /// get_ushort().
        /// Use this method only when you are sure that `i` is less than a half
        /// of the size of this union in bytes.
        /// Otherwise, use its counterpart method [get_ushort()](#method.get_ushort)
        /// for safety.
        /// 
        /// # Example 1 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// let a_intunion = IntUnion::new_with(4048161615_u32);
        /// for i in 0..2
        ///     { println!("a_intunion.get_ushort_({}) = {}", i, a_intunion.get_ushort_(i)); }
        /// assert_eq!(a_intunion.get_ushort_(0), 2895_u16);
        /// assert_eq!(a_intunion.get_ushort_(1), 61770_u16);
        /// 
        /// // It will panic.
        /// // println!("a_intunion.get_ushort_(2) = {}", a_intunion.get_ubyte_(2));
        /// ```
        /// 
        /// # Example 2 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let a_longunion = LongUnion::new_with(13664881099896654671_u64);
        /// for i in 0..4
        ///     { println!("a_longunion.get_ushort_({}) = {}", i, a_longunion.get_ushort_(i)); }
        /// assert_eq!(a_longunion.get_ushort_(0), 2895_u16);
        /// assert_eq!(a_longunion.get_ushort_(1), 61770_u16);
        /// assert_eq!(a_longunion.get_ushort_(2), 26869_u16);
        /// assert_eq!(a_longunion.get_ushort_(3), 48547_u16);
        /// 
        /// // It will panic.
        /// // println!("a_longunion.get_ushort_(8) = {}", a_longunion.get_ushort_(8));
        /// ```
        /// 
        /// # Example 3 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// for i in 0..8
        ///     { println!("a_longerunion.get_ushort_({}) = {}", i, a_longerunion.get_ushort_(i)); }
        /// assert_eq!(a_longerunion.get_ushort_(0), 2895_u16);
        /// assert_eq!(a_longerunion.get_ushort_(1), 61770_u16);
        /// assert_eq!(a_longerunion.get_ushort_(2), 26869_u16);
        /// assert_eq!(a_longerunion.get_ushort_(3), 48547_u16);
        /// assert_eq!(a_longerunion.get_ushort_(4), 34904_u16);
        /// assert_eq!(a_longerunion.get_ushort_(5), 32462_u16);
        /// assert_eq!(a_longerunion.get_ushort_(6), 15130_u16);
        /// assert_eq!(a_longerunion.get_ushort_(7), 65298_u16);
        /// 
        /// // It will panic.
        /// // println!("a_longerunion.get_ushort_(8) = {}", a_longerunion.get_ushort_(8));
        /// ```
        /// 
        /// # Example 4 for SizeUnion for 64-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let a_sizeunion = SizeUnion::new_with(13664881099896654671_usize);
        ///     for i in 0..4
        ///         { println!("a_sizeunion.get_ushort_({}) = {}", i, a_sizeunion.get_ushort_(i)); }
        ///     assert_eq!(a_sizeunion.get_ushort_(0), 2895_u16);
        ///     assert_eq!(a_sizeunion.get_ushort_(1), 61770_u16);
        ///     assert_eq!(a_sizeunion.get_ushort_(2), 26869_u16);
        ///     assert_eq!(a_sizeunion.get_ushort_(3), 48547_u16);
        /// 
        ///     // It will panic.
        ///     // println!("a_sizeunion.get_ushort_(4) = {}", a_sizeunion.get_ushort_(4));
        /// }
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[inline] pub fn get_ushort_(&self, i: usize) -> u16
        {
            #[cfg(target_endian = "little")]    unsafe { self.ushort[i] }
            #[cfg(target_endian = "big")]       unsafe { self.ushort[Self::M-i] }
        }

        // pub fn set_ushort_(&mut self, i: usize, val: u16)
        /// Sets i-th element of its array `ushort` of type `u16`
        /// if `i` is less than a half of the size of this union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Arguments
        /// - `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// - `val` is the value of type `u16` to set the i-th element of its
        ///  array `ushort` of type `u16`.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a half of the size of this
        /// union in bytes, it will panic.
        /// 
        /// # Caution
        /// Use this method only when you are sure that the argument i is less
        /// than a half of the size of this union.
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_ushort(). This method
        /// set_ushort_() is considered to be slightly faster than the method
        /// set_ushort().
        /// Use this method only when you are sure that `i` is less than a half
        /// of the size of this union in bytes. Otherwise, use its counterpart
        /// method [set_ushort()](#method.set_ushort) for safety.
        /// 
        /// # Example 1 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// let mut a_intunion = IntUnion::new();
        /// a_intunion.set_ushort_(0, 2895_u16);
        /// a_intunion.set_ushort_(1, 61770_u16);
        /// 
        /// // It will panic.
        /// // a_intunion.set_ushort_(2, 100_u16);
        /// 
        /// println!("a_intunion.get() = {}", a_intunion.get());
        /// for i in 0..2
        ///     { println!("a_intunion.get_ushort_({}) = {}", i, a_intunion.get_ushort_(i)); }
        /// assert_eq!(a_intunion.get_ushort_(0), 2895_u16);
        /// assert_eq!(a_intunion.get_ushort_(1), 61770_u16);
        /// assert_eq!(a_intunion.get(), 4048161615_u32);
        /// ```
        /// 
        /// # Example 2 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let mut a_longunion = LongUnion::new();
        /// a_longunion.set_ushort_(0, 2895_u16);
        /// a_longunion.set_ushort_(1, 61770_u16);
        /// a_longunion.set_ushort_(2, 26869_u16);
        /// a_longunion.set_ushort_(3, 48547_u16);
        /// 
        /// // It will panic.
        /// // a_longunion.set_ushort_(4, 100_u16);
        /// 
        /// for i in 0..4
        ///     { println!("a_longunion.get_ushort_({}) = {}", i, a_longunion.get_ushort_(i)); }
        /// assert_eq!(a_longunion.get_ushort_(0), 2895_u16);
        /// assert_eq!(a_longunion.get_ushort_(1), 61770_u16);
        /// assert_eq!(a_longunion.get_ushort_(2), 26869_u16);
        /// assert_eq!(a_longunion.get_ushort_(3), 48547_u16);
        /// assert_eq!(a_longunion.get(), 13664881099896654671_u64);
        /// ```
        /// 
        /// # Example 3 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let mut a_longerunion = LongerUnion::new();
        /// a_longerunion.set_ushort_(0, 2895_u16);
        /// a_longerunion.set_ushort_(1, 61770_u16);
        /// a_longerunion.set_ushort_(2, 26869_u16);
        /// a_longerunion.set_ushort_(3, 48547_u16);
        /// a_longerunion.set_ushort_(4, 34904_u16);
        /// a_longerunion.set_ushort_(5, 32462_u16);
        /// a_longerunion.set_ushort_(6, 15130_u16);
        /// a_longerunion.set_ushort_(7, 65298_u16);
        /// 
        /// // It will panic.
        /// // a_longerunion.set_ushort_(8, 100_u16);
        /// 
        /// for i in 0..8
        ///     { println!("a_longerunion.get_ushort_({}) = {}", i, a_longerunion.get_ushort_(i)); }
        /// assert_eq!(a_longerunion.get_ushort_(0), 2895_u16);
        /// assert_eq!(a_longerunion.get_ushort_(1), 61770_u16);
        /// assert_eq!(a_longerunion.get_ushort_(2), 26869_u16);
        /// assert_eq!(a_longerunion.get_ushort_(3), 48547_u16);
        /// assert_eq!(a_longerunion.get_ushort_(4), 34904_u16);
        /// assert_eq!(a_longerunion.get_ushort_(5), 32462_u16);
        /// assert_eq!(a_longerunion.get_ushort_(6), 15130_u16);
        /// assert_eq!(a_longerunion.get_ushort_(7), 65298_u16);
        /// assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
        /// ```
        /// 
        /// # Example 4 for SizeUnion for 64-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let mut a_sizeunion = SizeUnion::new();
        ///     a_sizeunion.set_ushort_(0, 2895_u16);
        ///     a_sizeunion.set_ushort_(1, 61770_u16);
        ///     a_sizeunion.set_ushort_(2, 26869_u16);
        ///     a_sizeunion.set_ushort_(3, 48547_u16);
        /// 
        ///     // It will panic.
        ///     // a_sizeunion.set_ushort_(4, 100_u16);
        /// 
        ///     for i in 0..4
        ///         { println!("a_sizeunion.get_ushort_({}) = {}", i, a_sizeunion.get_ushort_(i)); }
        ///     assert_eq!(a_sizeunion.get_ushort_(0), 2895_u16);
        ///     assert_eq!(a_sizeunion.get_ushort_(1), 61770_u16);
        ///     assert_eq!(a_sizeunion.get_ushort_(2), 26869_u16);
        ///     assert_eq!(a_sizeunion.get_ushort_(3), 48547_u16);
        ///     assert_eq!(a_sizeunion.get(), 13664881099896654671_usize);
        /// }
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[inline] pub fn set_ushort_(&mut self, i: usize, val: u16)
        {
            #[cfg(target_endian = "little")]    unsafe { self.ushort[i] = val; }
            #[cfg(target_endian = "big")]       unsafe { self.ushort[Self::M-i] = val; }
        }

        // pub fn get_sshort_(&self, i: usize) -> i16
        /// Returns i-th element of array `sshort` of type `i16`
        /// if `i` is less than a half of the size of this union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Arguments
        /// `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a half of the size of this
        /// union in bytes, it will panic.
        /// 
        /// # Caution
        /// Use this method only when you are sure that the argument `i` is
        /// less than a half of the size of this union.
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_sshort(). This method
        /// get_sshort_() is considered to be slightly faster than the method
        /// get_sshort().
        /// Use this method only when you are sure that `i` is less than a half
        /// of the size of this union in bytes.
        /// Otherwise, use its counterpart method [get_sshort()](#method.get_sshort)
        /// for safety.
        /// 
        /// # Example 1 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// let a_intunion = IntUnion::new_with(4048161615_u32);
        /// for i in 0..2
        ///     { println!("a_intunion.get_sshort_({}) = {}", i, a_intunion.get_sshort_(i)); }
        /// assert_eq!(a_intunion.get_sshort_(0), 2895_i16);
        /// assert_eq!(a_intunion.get_sshort_(1), -3766_i16);
        /// 
        /// // It will panic.
        /// // println!("a_intunion.get_sshort_(2) = {}", a_intunion.get_sshort_(2));
        /// ```
        /// 
        /// # Example 2 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let a_longunion = LongUnion::new_with(13664881099896654671_u64);
        /// for i in 0..4
        ///     { println!("a_longunion.get_sshort_({}) = {}", i, a_longunion.get_sshort_(i)); }
        /// assert_eq!(a_longunion.get_sshort_(0), 2895_i16);
        /// assert_eq!(a_longunion.get_sshort_(1), -3766_i16);
        /// assert_eq!(a_longunion.get_sshort_(2), 26869_i16);
        /// assert_eq!(a_longunion.get_sshort_(3), -16989_i16);
        /// 
        /// // It will panic.
        /// // println!("a_longunion.get_sshort_(4) = {}", a_longunion.get_sshort_(4));
        /// ```
        /// 
        /// # Example 3 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// for i in 0..8
        ///     { println!("a_longerunion.get_sshort_({}) = {}", i, a_longerunion.get_sshort_(i)); }
        /// assert_eq!(a_longerunion.get_sshort_(0), 2895_i16);
        /// assert_eq!(a_longerunion.get_sshort_(1), -3766_i16);
        /// assert_eq!(a_longerunion.get_sshort_(2), 26869_i16);
        /// assert_eq!(a_longerunion.get_sshort_(3), -16989_i16);
        /// assert_eq!(a_longerunion.get_sshort_(4), -30632_i16);
        /// assert_eq!(a_longerunion.get_sshort_(5), 32462_i16);
        /// assert_eq!(a_longerunion.get_sshort_(6), 15130_i16);
        /// assert_eq!(a_longerunion.get_sshort_(7), -238_i16);
        /// 
        /// // It will panic.
        /// // println!("a_longerunion.get_sshort_(8) = {}", a_longerunion.get_sshort_(8));
        /// ```
        /// 
        /// # Example 4 for SizeUnion for 64-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let a_sizeunion = SizeUnion::new_with(13664881099896654671_usize);
        ///     for i in 0..4
        ///         { println!("a_sizeunion.get_sshort_({}) = {}", i, a_sizeunion.get_sshort_(i)); }
        ///     assert_eq!(a_sizeunion.get_sshort_(0), 2895_i16);
        ///     assert_eq!(a_sizeunion.get_sshort_(1), -3766_i16);
        ///     assert_eq!(a_sizeunion.get_sshort_(2), 26869_i16);
        ///     assert_eq!(a_sizeunion.get_sshort_(3), -16989_i16);
        /// 
        ///     // It will panic.
        ///     // println!("a_sizeunion.get_sshort_(4) = {}", a_sizeunion.get_sshort_(4));
        /// }
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[inline] pub fn get_sshort_(&self, i: usize) -> i16
        {
            #[cfg(target_endian = "little")]    unsafe { self.sshort[i] }
            #[cfg(target_endian = "big")]       unsafe { self.sshort[Self::M-i] }
        }

        // pub fn set_sshort_(&mut self, i: usize, val: i16)
        /// Sets i-th element of its array `sshort` of type `i16`
        /// if `i` is less than a half of the size of this union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Arguments
        /// - `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// - `val` is the value of type `i16` to set the i-th element of its
        ///  array `sshort` of type `i16`.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a half of the size of this
        /// union in bytes, it will panic.
        /// 
        /// # Caution
        /// Use this method only when you are sure that the argument i is less
        /// than a half of the size of this union.
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_sshort(). This method
        /// set_sshort_() is considered to be slightly faster than the method
        /// set_sshort().
        /// Use this method only when you are sure that `i` is less than a half
        /// of the size of this union in bytes. Otherwise, use its counterpart
        /// method [set_sshort()](#method.set_sshort) for safety.
        /// 
        /// # Example 1 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// let mut a_intunion = IntUnion::new();
        /// a_intunion.set_sshort_(0, 2895_i16);
        /// a_intunion.set_sshort_(1, -3766_i16);
        /// 
        /// // It will panic.
        /// // a_intunion.set_sshort_(2, 100_i16);
        /// 
        /// println!("a_intunion.get() = {}", a_intunion.get());
        /// for i in 0..2
        ///     { println!("a_intunion.get_sshort_({}) = {}", i, a_intunion.get_sshort_(i)); }
        /// assert_eq!(a_intunion.get_sshort_(0), 2895_i16);
        /// assert_eq!(a_intunion.get_sshort_(1), -3766_i16);
        /// assert_eq!(a_intunion.get(), 4048161615_u32);
        /// ```
        /// 
        /// # Example 2 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let mut a_longunion = LongUnion::new();
        /// a_longunion.set_sshort_(0, 2895_i16);
        /// a_longunion.set_sshort_(1, -3766_i16);
        /// a_longunion.set_sshort_(2, 26869_i16);
        /// a_longunion.set_sshort_(3, -16989_i16);
        /// 
        /// // It will panic.
        /// // a_longunion.set_sshort_(4, 100_i16);
        /// 
        /// for i in 0..4
        ///     { println!("a_longunion.get_sshort_({}) = {}", i, a_longunion.get_sshort_(i)); }
        /// assert_eq!(a_longunion.get_sshort_(0), 2895_i16);
        /// assert_eq!(a_longunion.get_sshort_(1), -3766_i16);
        /// assert_eq!(a_longunion.get_sshort_(2), 26869_i16);
        /// assert_eq!(a_longunion.get_sshort_(3), -16989_i16);
        /// assert_eq!(a_longunion.get(), 13664881099896654671_u64);
        /// ```
        /// 
        /// # Example 3 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let mut a_longerunion = LongerUnion::new();
        /// a_longerunion.set_sshort_(0, 2895_i16);
        /// a_longerunion.set_sshort_(1, -3766_i16);
        /// a_longerunion.set_sshort_(2, 26869_i16);
        /// a_longerunion.set_sshort_(3, -16989_i16);
        /// a_longerunion.set_sshort_(4, -30632_i16);
        /// a_longerunion.set_sshort_(5, 32462_i16);
        /// a_longerunion.set_sshort_(6, 15130_i16);
        /// a_longerunion.set_sshort_(7, -238_i16);
        /// 
        /// // It will panic.
        /// // a_longerunion.set_sshort_(8, 100_i16);
        /// 
        /// for i in 0..8
        ///     { println!("a_longerunion.get_sshort_({}) = {}", i, a_longerunion.get_sshort_(i)); }
        /// assert_eq!(a_longerunion.get_sshort_(0), 2895_i16);
        /// assert_eq!(a_longerunion.get_sshort_(1), -3766_i16);
        /// assert_eq!(a_longerunion.get_sshort_(2), 26869_i16);
        /// assert_eq!(a_longerunion.get_sshort_(3), -16989_i16);
        /// assert_eq!(a_longerunion.get_sshort_(4), -30632_i16);
        /// assert_eq!(a_longerunion.get_sshort_(5), 32462_i16);
        /// assert_eq!(a_longerunion.get_sshort_(6), 15130_i16);
        /// assert_eq!(a_longerunion.get_sshort_(7), -238_i16);
        /// assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
        /// ```
        /// 
        /// # Example 4 for SizeUnion for 64-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let mut a_sizeunion = SizeUnion::new();
        ///     a_sizeunion.set_sshort_(0, 2895_i16);
        ///     a_sizeunion.set_sshort_(1, -3766_i16);
        ///     a_sizeunion.set_sshort_(2, 26869_i16);
        ///     a_sizeunion.set_sshort_(3, -16989_i16);
        /// 
        ///     // It will panic.
        ///     // a_sizeunion.set_sshort_(4, 100_i16);
        /// 
        ///     for i in 0..4
        ///         { println!("a_sizeunion.get_sshort_({}) = {}", i, a_sizeunion.get_sshort_(i)); }
        ///     assert_eq!(a_sizeunion.get_sshort_(0), 2895_i16);
        ///     assert_eq!(a_sizeunion.get_sshort_(1), -3766_i16);
        ///     assert_eq!(a_sizeunion.get_sshort_(2), 26869_i16);
        ///     assert_eq!(a_sizeunion.get_sshort_(3), -16989_i16);
        ///     assert_eq!(a_sizeunion.get(), 13664881099896654671_usize);
        /// }
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[inline] pub fn set_sshort_(&mut self, i: usize, val: i16)
        {
            #[cfg(target_endian = "little")]    unsafe { self.sshort[i] = val; }
            #[cfg(target_endian = "big")]       unsafe { self.sshort[Self::M-i] = val; }
        }

        // pub fn get_ushort(&self, i: usize) -> Option<u16>
        /// Returns i-th element of array `ushort` of type `u16` wrapped in `Some`
        /// of enum `Option` if `i` is less than a half of the size of this union
        /// in bytes. Otherwise, it returns `None` of enum `Option`.
        /// 
        /// # Arguments
        /// `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// 
        /// # Output
        /// - i-th element of array `ushort` of type `u16` wrapped in `Some`
        /// of enum `Option` if `i` is less than a half of the size of this
        /// union in bytes
        /// - `None` if `i` is greater than or equal to a half of the size of
        /// this union in bytes
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than a half
        /// of the size of this union in bytes. Otherwise, you can use its
        /// counterpart method [get_ushort_()](#method.get_ushort_)
        /// for performance.
        /// 
        /// # Example 1 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// let a_intunion = IntUnion::new_with(4048161615_u32);
        /// for i in 0..2
        /// {
        ///     match a_intunion.get_ushort(i)
        ///     {
        ///         Some(a) => { println!("a_intunion.get_ushort({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_intunion.get_ushort(0), Some(2895_u16));
        /// assert_eq!(a_intunion.get_ushort(1), Some(61770_u16));
        /// assert_eq!(a_intunion.get_ushort(2), None);
        /// ```
        /// 
        /// # Example 2 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let a_longunion = LongUnion::new_with(13664881099896654671_u64);
        /// for i in 0..4
        /// {
        ///     match a_longunion.get_ushort(i)
        ///     {
        ///         Some(a) => { println!("a_longunion.get_ushort({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longunion.get_ushort(0), Some(2895_u16));
        /// assert_eq!(a_longunion.get_ushort(1), Some(61770_u16));
        /// assert_eq!(a_longunion.get_ushort(2), Some(26869_u16));
        /// assert_eq!(a_longunion.get_ushort(3), Some(48547_u16));
        /// assert_eq!(a_longunion.get_ushort(4), None);
        /// ```
        /// 
        /// # Example 3 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// for i in 0..8
        /// {
        ///     match a_longerunion.get_ushort(i)
        ///     {
        ///         Some(a) => { println!("a_longerunion.get_ushort({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longerunion.get_ushort(0), Some(2895_u16));
        /// assert_eq!(a_longerunion.get_ushort(1), Some(61770_u16));
        /// assert_eq!(a_longerunion.get_ushort(2), Some(26869_u16));
        /// assert_eq!(a_longerunion.get_ushort(3), Some(48547_u16));
        /// assert_eq!(a_longerunion.get_ushort(4), Some(34904_u16));
        /// assert_eq!(a_longerunion.get_ushort(5), Some(32462_u16));
        /// assert_eq!(a_longerunion.get_ushort(6), Some(15130_u16));
        /// assert_eq!(a_longerunion.get_ushort(7), Some(65298_u16));
        /// assert_eq!(a_longerunion.get_ushort(8), None);
        /// ```
        /// 
        /// # Example 4 for SizeUnion for 64-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let a_sizeunion = SizeUnion::new_with(13664881099896654671_usize);
        ///     for i in 0..8
        ///     {
        ///         match a_sizeunion.get_ushort(i)
        ///         {
        ///             Some(a) => { println!("a_sizeunion.get_ushort({}) = {}", i, a); },
        ///             _ => {},
        ///         }
        ///     }
        ///     assert_eq!(a_sizeunion.get_ushort(0), Some(2895_u16));
        ///     assert_eq!(a_sizeunion.get_ushort(1), Some(61770_u16));
        ///     assert_eq!(a_sizeunion.get_ushort(2), Some(26869_u16));
        ///     assert_eq!(a_sizeunion.get_ushort(3), Some(48547_u16));
        ///     assert_eq!(a_sizeunion.get_ushort(4), None);
        /// }
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        pub fn get_ushort(&self, i: usize) -> Option<u16>
        {
            if i <= Self::M
            {
                #[cfg(target_endian = "little")]    unsafe { Some(self.ushort[i]) }
                #[cfg(target_endian = "big")]       unsafe { Some(self.ushort[Self::M-i]) }
            }
            else
            {
                None
            }
        }

        // pub fn set_ushort(&mut self, i: usize, val: u16) -> bool
        /// Sets i-th element of its array `ushort` of type `u16` and returns
        /// true if `i` is less than a half of the size of this union in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Arguments
        /// - `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// - `val` is the value of type `u16` to set the i-th element of its
        ///  array `ushort` of type `u16`.
        /// 
        /// # Output
        /// - `true` if `i` is less than a half of the size of this union
        /// in bytes
        /// - `false` if `i` is greater than or equal to a half of the size of
        /// this union in bytes
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than a half
        /// of the size of this union in bytes. Otherwise, you can use its
        /// counterpart method [set_ushort_()](#method.set_ushort_)
        /// for performance.
        /// 
        /// # Example 1 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// let mut a_intunion = IntUnion::new();
        /// let b0 = a_intunion.set_ushort(0, 2895_u16);
        /// let b1 = a_intunion.set_ushort(1, 61770_u16);
        /// let b2 = a_intunion.set_ushort(2, 100_u16);
        /// assert_eq!(b0, true);
        /// assert_eq!(b1, true);
        /// assert_eq!(b2, false);
        /// println!("a_intunion.get() = {}", a_intunion.get());
        /// for i in 0..2
        /// {
        ///     match a_intunion.get_ushort(i)
        ///     {
        ///         Some(a) => { println!("a_intunion.get_ushort({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_intunion.get_ushort(0), Some(2895_u16));
        /// assert_eq!(a_intunion.get_ushort(1), Some(61770_u16));
        /// assert_eq!(a_intunion.get(), 4048161615_u32);
        /// ```
        /// 
        /// # Example 2 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let mut a_longunion = LongUnion::new();
        /// let b0 = a_longunion.set_ushort(0, 2895_u16);
        /// let b1 = a_longunion.set_ushort(1, 61770_u16);
        /// let b2 = a_longunion.set_ushort(2, 26869_u16);
        /// let b3 = a_longunion.set_ushort(3, 48547_u16);
        /// let b4 = a_longunion.set_ushort(4, 100_u16);
        /// assert_eq!(b0, true);
        /// assert_eq!(b1, true);
        /// assert_eq!(b2, true);
        /// assert_eq!(b3, true);
        /// assert_eq!(b4, false);
        /// println!("a_longunion.get() = {}", a_longunion.get());
        /// for i in 0..4
        /// {
        ///     match a_longunion.get_ubyte(i)
        ///     {
        ///         Some(a) => { println!("a_longunion.get_ubyte({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longunion.get_ushort(0), Some(2895_u16));
        /// assert_eq!(a_longunion.get_ushort(1), Some(61770_u16));
        /// assert_eq!(a_longunion.get_ushort(2), Some(26869_u16));
        /// assert_eq!(a_longunion.get_ushort(3), Some(48547_u16));
        /// assert_eq!(a_longunion.get(), 13664881099896654671_u64);
        /// ```
        /// 
        /// # Example 3 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let mut a_longerunion = LongerUnion::new();
        /// let b0 = a_longerunion.set_ushort(0, 2895_u16);
        /// let b1 = a_longerunion.set_ushort(1, 61770_u16);
        /// let b2 = a_longerunion.set_ushort(2, 26869_u16);
        /// let b3 = a_longerunion.set_ushort(3, 48547_u16);
        /// let b4 = a_longerunion.set_ushort(4, 34904_u16);
        /// let b5 = a_longerunion.set_ushort(5, 32462_u16);
        /// let b6 = a_longerunion.set_ushort(6, 15130_u16);
        /// let b7 = a_longerunion.set_ushort(7, 65298_u16);
        /// let b8 = a_longerunion.set_ushort(8, 100_u16);
        /// assert_eq!(b0, true);
        /// assert_eq!(b1, true);
        /// assert_eq!(b2, true);
        /// assert_eq!(b3, true);
        /// assert_eq!(b4, true);
        /// assert_eq!(b5, true);
        /// assert_eq!(b6, true);
        /// assert_eq!(b7, true);
        /// assert_eq!(b8, false);
        /// println!("a_longerunion.get() = {}", a_longerunion.get());
        /// for i in 0..8
        /// {
        ///     match a_longerunion.get_ushort(i)
        ///     {
        ///         Some(a) => { println!("a_longerunion.get_ushort({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longerunion.get_ushort(0), Some(2895_u16));
        /// assert_eq!(a_longerunion.get_ushort(1), Some(61770_u16));
        /// assert_eq!(a_longerunion.get_ushort(2), Some(26869_u16));
        /// assert_eq!(a_longerunion.get_ushort(3), Some(48547_u16));
        /// assert_eq!(a_longerunion.get_ushort(4), Some(34904_u16));
        /// assert_eq!(a_longerunion.get_ushort(5), Some(32462_u16));
        /// assert_eq!(a_longerunion.get_ushort(6), Some(15130_u16));
        /// assert_eq!(a_longerunion.get_ushort(7), Some(65298_u16));
        /// assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
        /// ```
        /// 
        /// # Example 4 for SizeUnion for 64-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let mut a_sizeunion = SizeUnion::new();
        ///     let b0 = a_sizeunion.set_ushort(0, 2895_u16);
        ///     let b1 = a_sizeunion.set_ushort(1, 61770_u16);
        ///     let b2 = a_sizeunion.set_ushort(2, 26869_u16);
        ///     let b3 = a_sizeunion.set_ushort(3, 48547_u16);
        ///     let b4 = a_sizeunion.set_ushort(4, 100_u16);
        ///     assert_eq!(b0, true);
        ///     assert_eq!(b1, true);
        ///     assert_eq!(b2, true);
        ///     assert_eq!(b3, true);
        ///     assert_eq!(b4, false);
        ///     println!("a_sizeunion.get() = {}", a_sizeunion.get());
        ///     for i in 0..4
        ///     {
        ///         match a_sizeunion.get_ushort(i)
        ///         {
        ///             Some(a) => { println!("a_sizeunion.get_ushort({}) = {}", i, a); },
        ///             _ => {},
        ///         }
        ///     }
        ///     assert_eq!(a_sizeunion.get_ushort(0), Some(2895_u16));
        ///     assert_eq!(a_sizeunion.get_ushort(1), Some(61770_u16));
        ///     assert_eq!(a_sizeunion.get_ushort(2), Some(26869_u16));
        ///     assert_eq!(a_sizeunion.get_ushort(3), Some(48547_u16));
        ///     assert_eq!(a_sizeunion.get(), 13664881099896654671_usize);
        /// }
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        pub fn set_ushort(&mut self, i: usize, val: u16) -> bool
        {
            if i <= Self::M
            { 
                #[cfg(target_endian = "little")]    unsafe { self.ushort[i] = val; }
                #[cfg(target_endian = "big")]       unsafe { self.ushort[Self::M-i] = val; }
                true
            }
            else
            {
                false
            }
        }

        // pub fn get_sshort(&self, i: usize) -> Option<i16>
        /// Returns i-th element of array `sshort` of type `i16` wrapped in `Some`
        /// of enum `Option` if `i` is less than a half of the size of this union
        /// in bytes. Otherwise, it returns `None` of enum `Option`.
        /// 
        /// # Arguments
        /// `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// 
        /// # Output
        /// - i-th element of array `sshort` of type `i16` wrapped in `Some`
        /// of enum `Option` if `i` is less than a half of the size of this
        /// union in bytes
        /// - `None` if `i` is greater than or equal to a half of the size of
        /// this union in bytes
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than a half
        /// of the size of this union in bytes. Otherwise, use its counterpart
        /// method [get_sshort_()](#method.get_sshort_) for performance.
        /// 
        /// # Example 1 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// let a_intunion = IntUnion::new_with(4048161615_u32);
        /// for i in 0..2
        /// {
        ///     match a_intunion.get_sshort(i)
        ///     {
        ///         Some(a) => { println!("a_intunion.get_sshort({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_intunion.get_sshort(0), Some(2895_i16));
        /// assert_eq!(a_intunion.get_sshort(1), Some(-3766_i16));
        /// assert_eq!(a_intunion.get_sshort(2), None);
        /// ```
        /// 
        /// # Example 2 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let a_longunion = LongUnion::new_with(13664881099896654671_u64);
        /// for i in 0..4
        /// {
        ///     match a_longunion.get_sshort(i)
        ///     {
        ///         Some(a) => { println!("a_longunion.get_sshort({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longunion.get_sshort(0), Some(2895_i16));
        /// assert_eq!(a_longunion.get_sshort(1), Some(-3766_i16));
        /// assert_eq!(a_longunion.get_sshort(2), Some(26869_i16));
        /// assert_eq!(a_longunion.get_sshort(3), Some(-16989_i16));
        /// assert_eq!(a_longunion.get_sshort(4), None);
        /// ```
        /// 
        /// # Example 3 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// for i in 0..8
        /// {
        ///     match a_longerunion.get_sshort(i)
        ///     {
        ///         Some(a) => { println!("a_longerunion.get_sshort({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longerunion.get_sshort(0), Some(2895_i16));
        /// assert_eq!(a_longerunion.get_sshort(1), Some(-3766_i16));
        /// assert_eq!(a_longerunion.get_sshort(2), Some(26869_i16));
        /// assert_eq!(a_longerunion.get_sshort(3), Some(-16989_i16));
        /// assert_eq!(a_longerunion.get_sshort(4), Some(-30632_i16));
        /// assert_eq!(a_longerunion.get_sshort(5), Some(32462_i16));
        /// assert_eq!(a_longerunion.get_sshort(6), Some(15130_i16));
        /// assert_eq!(a_longerunion.get_sshort(7), Some(-238_i16));
        /// assert_eq!(a_longerunion.get_sshort(8), None);
        /// ```
        /// 
        /// # Example 4 for SizeUnion for 64-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let a_sizeunion = SizeUnion::new_with(13664881099896654671_usize);
        ///     for i in 0..8
        ///     {
        ///         match a_sizeunion.get_sshort(i)
        ///         {
        ///             Some(a) => { println!("a_sizeunion.get_sshort({}) = {}", i, a); },
        ///             _ => {},
        ///         }
        ///     }
        ///     assert_eq!(a_sizeunion.get_sshort(0), Some(2895_i16));
        ///     assert_eq!(a_sizeunion.get_sshort(1), Some(-3766_i16));
        ///     assert_eq!(a_sizeunion.get_sshort(2), Some(26869_i16));
        ///     assert_eq!(a_sizeunion.get_sshort(3), Some(-16989_i16));
        ///     assert_eq!(a_sizeunion.get_sshort(4), None);
        /// }
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        pub fn get_sshort(&self, i: usize) -> Option<i16>
        {
            if i <= Self::M
            {
                #[cfg(target_endian = "little")]    unsafe { Some(self.sshort[i]) }
                #[cfg(target_endian = "big")]       unsafe { Some(self.sshort[Self::M-i]) }
            }
            else
            {
                None
            }
        }

        // pub fn set_sshort(&mut self, i: usize, val: i16) -> bool
        /// Sets i-th element of its array `sshort` of type `i16` and returns
        /// true if `i` is less than a half of the size of this union in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Arguments
        /// - `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// - `val` is the value of type `i16` to set the i-th element of its
        ///  array `sshort` of type `i16`.
        /// 
        /// # Output
        /// - `true` if `i` is less than a half of the size of this union
        /// in bytes
        /// - `false` if `i` is greater than or equal to a half of the size
        /// of this union in bytes
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than a half
        /// of the size of this union in bytes. Otherwise, use its counterpart
        /// method [set_sshort_()](#method.set_sshort_) for performance.
        /// 
        /// # Example 1 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// let mut a_intunion = IntUnion::new();
        /// let b0 = a_intunion.set_sshort(0, 2895_i16);
        /// let b1 = a_intunion.set_sshort(1, -3766_i16);
        /// let b2 = a_intunion.set_sshort(2, 100_i16);
        /// assert_eq!(b0, true);
        /// assert_eq!(b1, true);
        /// assert_eq!(b2, false);
        /// println!("a_intunion.get() = {}", a_intunion.get());
        /// for i in 0..2
        /// {
        ///     match a_intunion.get_sshort(i)
        ///     {
        ///         Some(a) => { println!("a_intunion.set_sshort({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_intunion.get_sshort(0), Some(2895_i16));
        /// assert_eq!(a_intunion.get_sshort(1), Some(-3766_i16));
        /// assert_eq!(a_intunion.get(), 4048161615_u32);
        /// ```
        /// # Example 2 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let mut a_longunion = LongUnion::new();
        /// let b0 = a_longunion.set_sshort(0, 2895_i16);
        /// let b1 = a_longunion.set_sshort(1, -3766_i16);
        /// let b2 = a_longunion.set_sshort(2, 26869_i16);
        /// let b3 = a_longunion.set_sshort(3, -16989_i16);
        /// let b4 = a_longunion.set_sshort(4, 100_i16);
        /// assert_eq!(b0, true);
        /// assert_eq!(b1, true);
        /// assert_eq!(b2, true);
        /// assert_eq!(b3, true);
        /// assert_eq!(b4, false);
        /// println!("a_longunion.get() = {}", a_longunion.get());
        /// for i in 0..4
        /// {
        ///     match a_longunion.get_sshort(i)
        ///     {
        ///         Some(a) => { println!("a_longunion.get_sshort({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longunion.get_sshort(0), Some(2895_i16));
        /// assert_eq!(a_longunion.get_sshort(1), Some(-3766_i16));
        /// assert_eq!(a_longunion.get_sshort(2), Some(26869_i16));
        /// assert_eq!(a_longunion.get_sshort(3), Some(-16989_i16));
        /// assert_eq!(a_longunion.get(), 13664881099896654671_u64);
        /// ```
        /// # Example 3 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let mut a_longerunion = LongerUnion::new();
        /// let b0 = a_longerunion.set_sshort(0, 2895_i16);
        /// let b1 = a_longerunion.set_sshort(1, -3766_i16);
        /// let b2 = a_longerunion.set_sshort(2, 26869_i16);
        /// let b3 = a_longerunion.set_sshort(3, -16989_i16);
        /// let b4 = a_longerunion.set_sshort(4, -30632_i16);
        /// let b5 = a_longerunion.set_sshort(5, 32462_i16);
        /// let b6 = a_longerunion.set_sshort(6, 15130_i16);
        /// let b7 = a_longerunion.set_sshort(7, -238_i16);
        /// let b8 = a_longerunion.set_sshort(8, 100_i16);
        /// assert_eq!(b0, true);
        /// assert_eq!(b1, true);
        /// assert_eq!(b2, true);
        /// assert_eq!(b3, true);
        /// assert_eq!(b4, true);
        /// assert_eq!(b5, true);
        /// assert_eq!(b6, true);
        /// assert_eq!(b7, true);
        /// assert_eq!(b8, false);
        /// println!("a_longerunion.get() = {}", a_longerunion.get());
        /// for i in 0..8
        /// {
        ///     match a_longerunion.get_ushort(i)
        ///     {
        ///         Some(a) => { println!("a_longerunion.get_ushort({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longerunion.get_sshort(0), Some(2895_i16));
        /// assert_eq!(a_longerunion.get_sshort(1), Some(-3766_i16));
        /// assert_eq!(a_longerunion.get_sshort(2), Some(26869_i16));
        /// assert_eq!(a_longerunion.get_sshort(3), Some(-16989_i16));
        /// assert_eq!(a_longerunion.get_sshort(4), Some(-30632_i16));
        /// assert_eq!(a_longerunion.get_sshort(5), Some(32462_i16));
        /// assert_eq!(a_longerunion.get_sshort(6), Some(15130_i16));
        /// assert_eq!(a_longerunion.get_sshort(7), Some(-238_i16));
        /// assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
        /// ```
        /// # Example 4 for SizeUnion for 64-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let mut a_sizeunion = SizeUnion::new();
        ///     let b0 = a_sizeunion.set_sshort(0, 2895_i16);
        ///     let b1 = a_sizeunion.set_sshort(1, -3766_i16);
        ///     let b2 = a_sizeunion.set_sshort(2, 26869_i16);
        ///     let b3 = a_sizeunion.set_sshort(3, -16989_i16);
        ///     let b4 = a_sizeunion.set_sshort(4, 100_i16);
        ///     assert_eq!(b0, true);
        ///     assert_eq!(b1, true);
        ///     assert_eq!(b2, true);
        ///     assert_eq!(b3, true);
        ///     assert_eq!(b4, false);
        ///     println!("a_sizeunion.get() = {}", a_sizeunion.get());
        ///     for i in 0..4
        ///     {
        ///         match a_sizeunion.get_sshort(i)
        ///         {
        ///             Some(a) => { println!("a_sizeunion.get_sshort({}) = {}", i, a); },
        ///             _ => {},
        ///         }
        ///     }
        ///     assert_eq!(a_sizeunion.get_sshort(0), Some(2895_i16));
        ///     assert_eq!(a_sizeunion.get_sshort(1), Some(-3766_i16));
        ///     assert_eq!(a_sizeunion.get_sshort(2), Some(26869_i16));
        ///     assert_eq!(a_sizeunion.get_sshort(3), Some(-16989_i16));
        ///     assert_eq!(a_sizeunion.get(), 13664881099896654671_usize);
        /// }
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        pub fn set_sshort(&mut self, i: usize, val: i16) -> bool
        {
            if i <= Self::M
            { 
                #[cfg(target_endian = "little")]    unsafe { self.sshort[i] = val; }
                #[cfg(target_endian = "big")]       unsafe { self.sshort[Self::M-i] = val; }
                true
            }
            else
            {
                false
            }
        }
    }
}
pub(crate) use get_set_short;


macro_rules! get_set_int_fit {
    () => {
        // pub fn get_uint(self) -> u32
        /// Returns its value as `u32`.
        /// 
        /// # Output
        /// Its value as `u32`
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::IntUnion;    
        /// let a = IntUnion::new_with(987654321_u32);
        /// println!("a = {}", a.get_uint());
        /// assert_eq!(a.get_uint(), 987654321_u32);
        /// ```
        #[inline] pub fn get_uint(self) -> u32      { unsafe { self.uint } }

        // pub fn set_uint(&mut self, val: u32)
        /// Sets its value with `val` of type `u32`
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::IntUnion;    
        /// let mut a = IntUnion::new();
        /// a.set_uint(987654321_u32);
        /// println!("a = {}", a.get_uint());
        /// assert_eq!(a.get_uint(), 987654321_u32);
        /// ```
        #[inline] pub fn set_uint(&mut self, val: u32)  { self.uint = val; }

        // pub fn get_sint(self) -> i32
        /// Returns its value as `i32`.
        /// 
        /// # Output
        /// Its value as `i32`
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::IntUnion;    
        /// let a = IntUnion::new_with(2345678901_u32);
        /// println!("a = {}", a.get_sint());
        /// assert_eq!(a.get_sint(), -1949288395_i32);
        /// ```
        #[inline] pub fn get_sint(self) -> i32      { unsafe { self.sint } }

        // pub fn set_sint(&mut self, val: i32)
        /// Sets its value with `val` of type `i32`
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::IntUnion;    
        /// let mut a = IntUnion::new();
        /// a.set_sint(-1949288395_i32);
        /// println!("a = {}", a.get_sint());
        /// assert_eq!(a.get_sint(), -1949288395_i32);
        /// ```
        #[inline] pub fn set_sint(&mut self, val: i32)  { self.sint = val; }
    };
}
pub(super) use get_set_int_fit;

macro_rules! get_set_int {
    ($f:expr) => {
        const L: usize = $f - 1;

        // pub fn get_uint_(&self, i: usize) -> u32
        /// Returns i-th element of array `uint` of type `u32`
        /// if `i` is less than a quarter of the size of this union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Arguments
        /// `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a quarter of the size of this
        /// union in bytes, it will panic.
        /// 
        /// # Caution
        /// Use this method only when you are sure that the argument `i` is
        /// less than a quarter of the size of this union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_uint(). This method
        /// get_uint_() is considered to be slightly faster than the method
        /// get_uint().
        /// Use this method only when you are sure that `i` is less than a
        /// quarter of the size of this union in bytes.
        /// Otherwise, use its counterpart method [get_uint()](#method.get_uint)
        /// for safety.
        /// 
        /// # Example 1 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let a_longunion = LongUnion::new_with(13664881099896654671_u64);
        /// for i in 0..2
        ///     { println!("a_longunion.get_ushort_({}) = {}", i, a_longunion.get_ushort_(i)); }
        /// assert_eq!(a_longunion.get_uint_(0), 4048161615_u32);
        /// assert_eq!(a_longunion.get_uint_(1), 3181603061_u32);
        /// 
        /// // It will panic.
        /// // println!("a_longunion.get_uint_(2) = {}", a_longunion.get_uint_(2));
        /// ```
        /// 
        /// # Example 2 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// for i in 0..4
        ///     { println!("a_longerunion.get_ushort_({}) = {}", i, a_longerunion.get_ushort_(i)); }
        /// assert_eq!(a_longerunion.get_uint_(0), 4048161615_u32);
        /// assert_eq!(a_longerunion.get_uint_(1), 3181603061_u32);
        /// assert_eq!(a_longerunion.get_uint_(2), 2127464536_u32);
        /// assert_eq!(a_longerunion.get_uint_(3), 4279384858_u32);
        /// 
        /// // It will panic.
        /// // println!("a_longerunion.get_uint_(4) = {}", a_longerunion.get_uint_(4));
        /// ```
        /// 
        /// # Example 3 for SizeUnion for 64-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let a_sizeunion = SizeUnion::new_with(13664881099896654671_usize);
        ///     for i in 0..2
        ///         { println!("a_sizeunion.get_uint_({}) = {}", i, a_sizeunion.get_uint_(i)); }
        ///     assert_eq!(a_sizeunion.get_uint_(0), 4048161615_u32);
        ///     assert_eq!(a_sizeunion.get_uint_(1), 3181603061_u32);
        /// 
        ///     // It will panic.
        ///     // println!("a_sizeunion.get_uint_(2) = {}", a_sizeunion.get_uint_(2));
        /// }
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[inline] pub fn get_uint_(&self, i: usize) -> u32
        {
            #[cfg(target_endian = "little")]    unsafe { self.uint[i] }
            #[cfg(target_endian = "big")]       unsafe { self.uint[Self::L-i] }
        }

        // pub fn set_uint_(&mut self, i: usize, val: u32)
        /// Sets i-th element of its array `uint` of type `u32`
        /// if `i` is less than a quarter of the size of this union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Arguments
        /// - `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// - `val` is the value of type `u32` to set the i-th element of its
        ///  array `uint` of type `u32`.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a quarter of the size of
        /// this union in bytes, it will panic. 
        /// 
        /// # Caution
        /// Use this method only when you are sure that the argument i is less
        /// than a quarter of the size of this union.
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method `set_uint()`. This method
        /// `set_uint_()` is considered to be slightly faster than the method
        /// `set_uint()`.
        /// Use this method only when you are sure that `i` is less than a
        /// quarter of the size of this union in bytes. Otherwise, use its
        /// counterpart method [set_uint()](#method.set_uint) for safety.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let mut a_longunion = LongUnion::new();
        /// a_longunion.set_uint_(0, 4048161615_u32);
        /// a_longunion.set_uint_(1, 3181603061_u32);
        /// 
        /// // It will panic.
        /// // a_longunion.set_ushort_(2, 100_u16);
        /// 
        /// for i in 0..2
        ///     { println!("a_longunion.get_ushort_({}) = {}", i, a_longunion.get_ushort_(i)); }
        /// assert_eq!(a_longunion.get_uint_(0), 4048161615_u32);
        /// assert_eq!(a_longunion.get_uint_(1), 3181603061_u32);
        /// assert_eq!(a_longunion.get(), 13664881099896654671_u64);
        /// ```
        /// 
        /// # Example 2 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let mut a_longerunion = LongerUnion::new();
        /// a_longerunion.set_uint_(0, 4048161615_u32);
        /// a_longerunion.set_uint_(1, 3181603061_u32);
        /// a_longerunion.set_uint_(2, 2127464536_u32);
        /// a_longerunion.set_uint_(3, 4279384858_u32);
        /// 
        /// // It will panic.
        /// // a_longerunion.set_uint_(4, 100_u32);
        /// 
        /// for i in 0..4
        ///     { println!("a_longerunion.get_uint_({}) = {}", i, a_longerunion.get_uint_(i)); }
        /// assert_eq!(a_longerunion.get_uint_(0), 4048161615_u32);
        /// assert_eq!(a_longerunion.get_uint_(1), 3181603061_u32);
        /// assert_eq!(a_longerunion.get_uint_(2), 2127464536_u32);
        /// assert_eq!(a_longerunion.get_uint_(3), 4279384858_u32);
        /// assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
        /// ```
        /// 
        /// # Example 3 for SizeUnion for 64-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let mut a_sizeunion = SizeUnion::new();
        ///     a_sizeunion.set_uint_(0, 4048161615_u32);
        ///     a_sizeunion.set_uint_(1, 3181603061_u32);
        /// 
        ///     // It will panic.
        ///     // a_sizeunion.set_ushort_(2, 100_u16);
        /// 
        ///     for i in 0..2
        ///         { println!("a_sizeunion.get_ushort_({}) = {}", i, a_sizeunion.get_ushort_(i)); }
        ///     assert_eq!(a_sizeunion.get_uint_(0), 4048161615_u32);
        ///     assert_eq!(a_sizeunion.get_uint_(1), 3181603061_u32);
        ///     assert_eq!(a_sizeunion.get(), 13664881099896654671_usize);
        /// }
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[inline] pub fn set_uint_(&mut self, i: usize, val: u32)
        {
            #[cfg(target_endian = "little")]    unsafe { self.uint[i] = val; }  
            #[cfg(target_endian = "big")]       unsafe { self.uint[Self::L-i] = val; } 
        }

        // pub fn get_sint_(&self, i: usize) -> i32
        /// Returns i-th element of array `sint` of type `i32`
        /// if `i` is less than a quarter of the size of this union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Arguments
        /// `i` indicates `i`-th element. 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a quarter of the size of this
        /// union in bytes, it will panic.
        /// 
        /// # Caution
        /// Use this method only when you are sure that the argument `i` is
        /// less than a quarter of the size of this union.
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method `get_sint()`. This method
        /// `get_sint_()` is considered to be slightly faster than the method
        /// `get_sint()`.
        /// Use this method only when you are sure that `i` is less than
        /// a quarter of the size of this union in bytes.
        /// Otherwise, use its counterpart method [get_sint()](#method.get_sint)
        /// for safety.
        /// 
        /// # Example 1 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let a_longunion = LongUnion::new_with(13664881099896654671_u64);
        /// for i in 0..2
        ///     { println!("a_longunion.get_sint_({}) = {}", i, a_longunion.get_sint_(i)); }
        /// assert_eq!(a_longunion.get_sint_(0), -246805681_i32);
        /// assert_eq!(a_longunion.get_sint_(1), -1113364235_i32);
        /// 
        /// // It will panic.
        /// // println!("a_longunion.get_sint_(2) = {}", a_longunion.get_sint_(2));
        /// ```
        /// 
        /// # Example 2 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// for i in 0..4
        ///     { println!("a_longerunion.get_sint_({}) = {}", i, a_longerunion.get_sint_(i)); }
        /// assert_eq!(a_longerunion.get_sint_(0), -246805681_i32);
        /// assert_eq!(a_longerunion.get_sint_(1), -1113364235_i32);
        /// assert_eq!(a_longerunion.get_sint_(2), 2127464536_i32);
        /// assert_eq!(a_longerunion.get_sint_(3), -15582438_i32);
        /// 
        /// // It will panic.
        /// // println!("a_longerunion.get_sint_(4) = {}", a_longerunion.get_sint_(4));
        /// ```
        /// 
        /// # Example 3 for SizeUnion for 64-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let a_sizeunion = SizeUnion::new_with(13664881099896654671_usize);
        ///     for i in 0..2
        ///         { println!("a_sizeunion.get_sint_({}) = {}", i, a_sizeunion.get_sint_(i)); }
        ///     assert_eq!(a_sizeunion.get_sint_(0), -246805681_i32);
        ///     assert_eq!(a_sizeunion.get_sint_(1), -1113364235_i32);
        /// 
        ///     // It will panic.
        ///     // println!("a_sizeunion.get_sint_(2) = {}", a_sizeunion.get_sint_(2));
        /// }
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[inline] pub fn get_sint_(&self, i: usize) -> i32
        {
            #[cfg(target_endian = "little")]    unsafe { self.sint[i] }
            #[cfg(target_endian = "big")]       unsafe { self.sint[Self::L-i] }
        }

        // pub fn set_sint_(&mut self, i: usize, val: i32)
        /// Sets i-th element of its array `sint` of type `i32`
        /// if `i` is less than a quarter of the size of this union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Arguments
        /// - `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// - `val` is the value of type `u32` to set the i-th element of its
        ///  array `sint` of type `u32`.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a quarter of the size of this
        /// union in bytes, it will panic.
        /// 
        /// # Caution
        /// Use this method only when you are sure that the argument i is less
        /// than a quarter of the size of this union.
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_sint(). This method
        /// set_sint_() is considered to be slightly faster than the method
        /// set_sint().
        /// Use this method only when you are sure that `i` is less than
        /// a quarter of the size of this union in bytes. Otherwise, use its
        /// counterpart method [set_sint()](#method.set_sint) for safety.
        /// 
        /// # Example 1 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let mut a_longunion = LongUnion::new();
        /// a_longunion.set_sint_(0, -246805681_i32);
        /// a_longunion.set_sint_(1, -1113364235_i32);
        /// 
        /// // It will panic.
        /// // a_longunion.set_sint_(2, 100_i32);
        /// 
        /// for i in 0..2
        ///     { println!("a_longunion.get_sint_({}) = {}", i, a_longunion.get_sint_(i)); }
        /// assert_eq!(a_longunion.get_sint_(0), -246805681_i32);
        /// assert_eq!(a_longunion.get_sint_(1), -1113364235_i32);
        /// assert_eq!(a_longunion.get(), 13664881099896654671_u64);
        /// ```
        /// 
        /// # Example 2 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let mut a_longerunion = LongerUnion::new();
        /// a_longerunion.set_sint_(0, -246805681_i32);
        /// a_longerunion.set_sint_(1, -1113364235_i32);
        /// a_longerunion.set_sint_(2, 2127464536_i32);
        /// a_longerunion.set_sint_(3, -15582438_i32);
        /// 
        /// // It will panic.
        /// // a_longerunion.set_uint_(4, 100_u32);
        /// 
        /// for i in 0..4
        ///     { println!("a_longerunion.get_uint_({}) = {}", i, a_longerunion.get_uint_(i)); }
        /// assert_eq!(a_longerunion.get_sint_(0), -246805681_i32);
        /// assert_eq!(a_longerunion.get_sint_(1), -1113364235_i32);
        /// assert_eq!(a_longerunion.get_sint_(2), 2127464536_i32);
        /// assert_eq!(a_longerunion.get_sint_(3), -15582438_i32);
        /// assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
        /// ```
        /// 
        /// # Example 3 for SizeUnion for 64-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let mut a_sizeunion = SizeUnion::new();
        ///     a_sizeunion.set_sint_(0, -246805681_i32);
        ///     a_sizeunion.set_sint_(1, -1113364235_i32);
        /// 
        ///     // It will panic.
        ///     // a_sizeunion.set_sint_(2, 100_i32);
        /// 
        ///     for i in 0..2
        ///         { println!("a_sizeunion.get_sint_({}) = {}", i, a_sizeunion.get_sint_(i)); }
        ///     assert_eq!(a_sizeunion.get_sint_(0), -246805681_i32);
        ///     assert_eq!(a_sizeunion.get_sint_(1), -1113364235_i32);
        ///     assert_eq!(a_sizeunion.get(), 13664881099896654671_usize);
        /// }
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[inline] pub fn set_sint_(&mut self, i: usize, val: i32)
        {
            #[cfg(target_endian = "little")]    unsafe { self.sint[i] = val; }
            #[cfg(target_endian = "big")]       unsafe { self.sint[Self::L-i] = val; }
        }

        // pub fn get_uint(&self, i: usize) -> Option<u32>
        /// Returns i-th element of array `uint` of type `u32` wrapped in `Some`
        /// of enum `Option` if `i` is less than a quarter of the size of this
        /// union in bytes. Otherwise, it returns None of enum `Option`.
        /// 
        /// # Arguments
        /// `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// 
        /// # Output
        /// - i-th element of array `uint` of type `u32` wrapped in `Some`
        /// of enum `Option` if `i` is less than a quarter of the size of this
        /// Union in bytes
        /// - `None` if `i` is greater than or equal to a quarter of the size of
        /// this union in bytes
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// a quarter of the size of this union in bytes.
        /// Otherwise, you can use its counterpart method
        /// [get_uint_()](#method.get_uint_) for performance.
        /// 
        /// # Example 1 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let a_longunion = LongUnion::new_with(13664881099896654671_u64);
        /// for i in 0..2
        /// {
        ///     match a_longunion.get_uint(i)
        ///     {
        ///         Some(a) => { println!("a_longunion.get_uint({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longunion.get_uint(0), Some(4048161615_u32));
        /// assert_eq!(a_longunion.get_uint(1), Some(3181603061_u32));
        /// assert_eq!(a_longunion.get_uint(2), None);
        /// ```
        /// 
        /// # Example 2 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// for i in 0..4
        /// {
        ///     match a_longerunion.get_uint(i)
        ///     {
        ///         Some(a) => { println!("a_longerunion.get_uint({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longerunion.get_uint(0), Some(4048161615_u32));
        /// assert_eq!(a_longerunion.get_uint(1), Some(3181603061_u32));
        /// assert_eq!(a_longerunion.get_uint(2), Some(2127464536_u32));
        /// assert_eq!(a_longerunion.get_uint(3), Some(4279384858_u32));
        /// assert_eq!(a_longerunion.get_uint(4), None);
        /// ```
        /// 
        /// # Example 3 for SizeUnion for 64-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let a_sizeunion = SizeUnion::new_with(13664881099896654671_usize);
        ///     for i in 0..2
        ///     {
        ///         match a_sizeunion.get_uint(i)
        ///         {
        ///             Some(a) => { println!("a_sizeunion.get_uint({}) = {}", i, a); },
        ///             _ => {},
        ///         }
        ///     }
        ///     assert_eq!(a_sizeunion.get_uint(0), Some(4048161615_u32));
        ///     assert_eq!(a_sizeunion.get_uint(1), Some(3181603061_u32));
        ///     assert_eq!(a_sizeunion.get_uint(2), None);
        /// }
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        pub fn get_uint(&self, i: usize) -> Option<u32>
        {
            if i <= Self::L
            {
                #[cfg(target_endian = "little")]    unsafe { Some(self.uint[i]) }
                #[cfg(target_endian = "big")]       unsafe { Some(self.uint[Self::L-i]) }
            }
            else
            {
                None
            }
        }

        // pub fn set_uint(&mut self, i: usize, val: u32) -> bool
        /// Sets i-th element of its array `uint` of type `u32` and returns
        /// true if `i` is less than a quarter of the size of this union in
        /// bytes. Otherwise, it will set nothing amd return false.
        /// 
        /// # Arguments
        /// - `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// - `val` is the value of type `u16` to set the i-th element of its
        ///  array `uint` of type `u16`.
        /// 
        /// # Output
        /// - `true` if `i` is less than a quarter of the size of this union
        /// in bytes
        /// - `false` if `i` is greater than or equal to a quarter of the size
        /// of this union in bytes
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// a quarter of the size of this union in bytes. Otherwise, you can
        /// use its counterpart method [set_uint_()](#method.set_uint_)
        /// for performance.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let mut a_longunion = LongUnion::new();
        /// let b0 = a_longunion.set_uint(0, 4048161615_u32);
        /// let b1 = a_longunion.set_uint(1, 3181603061_u32);
        /// let b2 = a_longunion.set_uint(2, 100_u32);
        /// assert_eq!(b0, true);
        /// assert_eq!(b1, true);
        /// assert_eq!(b2, false);
        /// for i in 0..2
        /// {
        ///     match a_longunion.get_uint(i)
        ///     {
        ///         Some(a) => { println!("a_longunion.get_uint({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longunion.get_uint(0), Some(4048161615_u32));
        /// assert_eq!(a_longunion.get_uint(1), Some(3181603061_u32));
        /// assert_eq!(a_longunion.get(), 13664881099896654671_u64);
        /// ```
        /// 
        /// # Example 2 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let mut a_longerunion = LongerUnion::new();
        /// let b0 = a_longerunion.set_uint(0, 4048161615_u32);
        /// let b1 = a_longerunion.set_uint(1, 3181603061_u32);
        /// let b2 = a_longerunion.set_uint(2, 2127464536_u32);
        /// let b3 = a_longerunion.set_uint(3, 4279384858_u32);
        /// let b4 = a_longerunion.set_uint(4, 100_u32);
        /// assert_eq!(b0, true);
        /// assert_eq!(b1, true);
        /// assert_eq!(b2, true);
        /// assert_eq!(b3, true);
        /// assert_eq!(b4, false);
        /// 
        /// for i in 0..4
        /// {
        ///     match a_longerunion.get_uint(i)
        ///     {
        ///         Some(a) => { println!("a_longerunion.get_uint({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longerunion.get_uint(0), Some(4048161615_u32));
        /// assert_eq!(a_longerunion.get_uint(1), Some(3181603061_u32));
        /// assert_eq!(a_longerunion.get_uint(2), Some(2127464536_u32));
        /// assert_eq!(a_longerunion.get_uint(3), Some(4279384858_u32));
        /// assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
        /// ```
        /// 
        /// # Example 3 for SizeUnion for 64-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let mut a_sizeunion = SizeUnion::new();
        ///     let b0 = a_sizeunion.set_uint(0, 4048161615_u32);
        ///     let b1 = a_sizeunion.set_uint(1, 3181603061_u32);
        ///     let b2 = a_sizeunion.set_uint(2, 100_u32);
        ///     assert_eq!(b0, true);
        ///     assert_eq!(b1, true);
        ///     assert_eq!(b2, false);
        ///     for i in 0..2
        ///     {
        ///         match a_sizeunion.get_uint(i)
        ///         {
        ///             Some(a) => { println!("a_sizeunion.get_uint({}) = {}", i, a); },
        ///             _ => {},
        ///         }
        ///     }
        ///     assert_eq!(a_sizeunion.get_uint(0), Some(4048161615_u32));
        ///     assert_eq!(a_sizeunion.get_uint(1), Some(3181603061_u32));
        ///     assert_eq!(a_sizeunion.get(), 13664881099896654671_usize);
        /// }
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        pub fn set_uint(&mut self, i: usize, val: u32) -> bool
        {
            if i <= Self::L
            { 
                #[cfg(target_endian = "little")]    unsafe { self.uint[i] = val; }
                #[cfg(target_endian = "big")]       unsafe { self.uint[Self::L-i] = val; }
                true
            }
            else
            {
                false
            }
        }

        // pub fn get_sint(&self, i: usize) -> Option<i32>
        /// Returns i-th element of array `sint` of type `i32` wrapped in `Some`
        /// of enum `Option` if `i` is less than a quarter of the size of this
        /// union in bytes. Otherwise, it returns None of enum `Option`.
        /// 
        /// # Arguments
        /// `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// 
        /// # Output
        /// - i-th element of array `sint` of type `i32` wrapped in `Some`
        /// of enum `Option` if `i` is less than a quarter of the size of this
        /// union in bytes
        /// - `None` if `i` is greater than or equal to a quarter of the size
        /// of this union in bytes
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// a quarter of the size of this union in bytes.
        /// Otherwise, you can use its counterpart method
        /// [get_sint_()](#method.get_sint_) for performance.
        /// 
        /// # Example 1 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let a_longunion = LongUnion::new_with(13664881099896654671_u64);
        /// for i in 0..2
        /// {
        ///     match a_longunion.get_sint(i)
        ///     {
        ///         Some(a) => { println!("a_longunion.get_sint({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longunion.get_sint(0), Some(-246805681_i32));
        /// assert_eq!(a_longunion.get_sint(1), Some(-1113364235_i32));
        /// assert_eq!(a_longunion.get_sint(2), None);
        /// ```
        /// 
        /// # Example 2 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// for i in 0..4
        /// {
        ///     match a_longerunion.get_sint(i)
        ///     {
        ///         Some(a) => { println!("a_longerunion.get_sint({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longerunion.get_sint(0), Some(-246805681_i32));
        /// assert_eq!(a_longerunion.get_sint(1), Some(-1113364235_i32));
        /// assert_eq!(a_longerunion.get_sint(2), Some(2127464536_i32));
        /// assert_eq!(a_longerunion.get_sint(3), Some(-15582438_i32));
        /// assert_eq!(a_longerunion.get_sint(4), None);
        /// ```
        /// 
        /// # Example 3 for SizeUnion for 64-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let a_sizeunion = SizeUnion::new_with(13664881099896654671_usize);
        ///     for i in 0..2
        ///     {
        ///         match a_sizeunion.get_sint(i)
        ///         {
        ///             Some(a) => { println!("a_sizeunion.get_sint({}) = {}", i, a); },
        ///             _ => {},
        ///         }
        ///     }
        ///     assert_eq!(a_sizeunion.get_sint(0), Some(-246805681_i32));
        ///     assert_eq!(a_sizeunion.get_sint(1), Some(-1113364235_i32));
        ///     assert_eq!(a_sizeunion.get_sint(2), None);
        /// }
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        pub fn get_sint(&self, i: usize) -> Option<i32>
        {
            if i <= Self::L
            {
                #[cfg(target_endian = "little")]    unsafe { Some(self.sint[i]) }
                #[cfg(target_endian = "big")]       unsafe { Some(self.sint[Self::L-i]) }
            }
            else
            {
                None
            }
        }

        // pub fn set_sint(&mut self, i: usize, val: i32) -> bool
        /// Sets i-th element of its array `sint` of type `i32` and returns
        /// true if `i` is less than a quarter of the size of this union in
        /// bytes. Otherwise, it will set nothing amd return false.
        /// 
        /// # Arguments
        /// - `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// - `val` is the value of type `u8` to set the i-th element of its
        ///  array `ubyte` of type `u8`.
        /// 
        /// # Output
        /// - `true` if `i` is less than a quarter of the size of this union
        /// in bytes
        /// - `false` if `i` is greater than or equal to a quarter of the size
        /// of this union in bytes
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// a quarter of the size of this union in bytes. Otherwise, use its
        /// counterpart method [set_sint_()](#method.set_sint_) for performance.
        /// 
        /// # Example 1 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let mut a_longunion = LongUnion::new();
        /// let b0 = a_longunion.set_sint(0, -246805681_i32);
        /// let b1 = a_longunion.set_sint(1, -1113364235_i32);
        /// let b2 = a_longunion.set_sint(2, 100_i32);
        /// assert_eq!(b0, true);
        /// assert_eq!(b1, true);
        /// assert_eq!(b2, false);
        /// for i in 0..2
        /// {
        ///     match a_longunion.get_sint(i)
        ///     {
        ///         Some(a) => { println!("a_longunion.set_sint({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longunion.get_sint(0), Some(-246805681_i32));
        /// assert_eq!(a_longunion.get_sint(1), Some(-1113364235_i32));
        /// assert_eq!(a_longunion.get(), 13664881099896654671_u64);
        /// ```
        /// 
        /// # Example 2 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let mut a_longerunion = LongerUnion::new();
        /// let b0 = a_longerunion.set_sint(0, -246805681_i32);
        /// let b1 = a_longerunion.set_sint(1, -1113364235_i32);
        /// let b2 = a_longerunion.set_sint(2, 2127464536_i32);
        /// let b3 = a_longerunion.set_sint(3, -15582438_i32);
        /// let b4 = a_longerunion.set_sint(4, 100_i32);
        /// assert_eq!(b0, true);
        /// assert_eq!(b1, true);
        /// assert_eq!(b2, true);
        /// assert_eq!(b3, true);
        /// assert_eq!(b4, false);
        /// 
        /// for i in 0..4
        /// {
        ///     match a_longerunion.get_uint(i)
        ///     {
        ///         Some(a) => { println!("a_longerunion.get_uint({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longerunion.get_sint(0), Some(-246805681_i32));
        /// assert_eq!(a_longerunion.get_sint(1), Some(-1113364235_i32));
        /// assert_eq!(a_longerunion.get_sint(2), Some(2127464536_i32));
        /// assert_eq!(a_longerunion.get_sint(3), Some(-15582438_i32));
        /// assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
        /// ```
        /// 
        /// # Example 3 for SizeUnion for 64-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let mut a_sizeunion = SizeUnion::new();
        ///     let b0 = a_sizeunion.set_sint(0, -246805681_i32);
        ///     let b1 = a_sizeunion.set_sint(1, -1113364235_i32);
        ///     let b2 = a_sizeunion.set_sint(2, 100_i32);
        ///     assert_eq!(b0, true);
        ///     assert_eq!(b1, true);
        ///     assert_eq!(b2, false);
        ///     for i in 0..2
        ///     {
        ///         match a_sizeunion.get_sint(i)
        ///         {
        ///             Some(a) => { println!("a_sizeunion.set_sint({}) = {}", i, a); },
        ///             _ => {},
        ///         }
        ///     }
        ///     assert_eq!(a_sizeunion.get_sint(0), Some(-246805681_i32));
        ///     assert_eq!(a_sizeunion.get_sint(1), Some(-1113364235_i32));
        ///     assert_eq!(a_sizeunion.get(), 13664881099896654671_usize);
        /// }
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        pub fn set_sint(&mut self, i: usize, val: i32) -> bool
        {
            if i <= Self::L
            { 
                #[cfg(target_endian = "little")]    unsafe { self.sint[i] = val; }
                #[cfg(target_endian = "big")]       unsafe { self.sint[Self::L-i] = val; }
                true
            }
            else
            {
                false
            }
        }
    }
}
pub(super) use get_set_int;



macro_rules! get_set_long_fit {
    () => {
        // pub fn get_ulong(self) -> u64
        /// Returns its value as `u64`.
        /// 
        /// # Output
        /// Its value as `u64`
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::LongUnion;    
        /// let a = LongUnion::new_with(654321987654321_u64);
        /// println!("a = {}", a.get_ulong());
        /// assert_eq!(a.get_ulong(), 654321987654321_u64);
        /// ```
        #[inline] pub fn get_ulong(self) -> u64         { unsafe { self.ulong } }

        // pub fn set_ulong(&mut self, val: u64)
        /// Sets its value with `val` of type `u64`
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::LongUnion;    
        /// let mut a = LongUnion::new();
        /// a.set_ulong(654321987654321_u64);
        /// println!("a = {}", a.get_ulong());
        /// assert_eq!(a.get_ulong(), 654321987654321_u64);
        /// ```
        #[inline] pub fn set_ulong(&mut self, val: u64)     { self.ulong = val; }

        // pub fn get_slong(self) -> i64
        /// Returns its value as `i64`.
        /// 
        /// # Output
        /// Its value as `i64`
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::LongUnion;    
        /// let a = LongUnion::new_with(12345678909876456789_u64);
        /// println!("a = {}", a.get_slong());
        /// assert_eq!(a.get_slong(), -6101065163833094827_i64);
        /// ```
        #[inline] pub fn get_slong(self) -> i64     { unsafe { self.slong } }

        // pub fn set_slong(&mut self, val: i64)
        /// Sets its value with `val` of type `i64`
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::LongUnion;    
        /// let mut a = LongUnion::new();
        /// a.set_slong(-6101065163833094827_i64);
        /// println!("a = {}", a.get_slong());
        /// assert_eq!(a.get_slong(), -6101065163833094827_i64);
        /// ```
        #[inline] pub fn set_slong(&mut self, val: i64)     { self.slong = val; }
    };
}
pub(super) use get_set_long_fit;

macro_rules! get_set_long {
    ($f:expr) => {
        const K: usize = $f - 1;

        // pub fn get_ulong_(&self, i: usize) -> u64
        /// Returns i-th element of array `ulong` of type `u64`
        /// if `i` is less than an eighth of the size of this union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Arguments
        /// `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// 
        /// # Output
        /// i-th element of array `ulong` of type `u64` if `i` is less than an
        /// eighth of the size of this union in bytes
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to an eighth of the size of this
        /// union in bytes, it will panic.
        /// 
        /// # Caution
        /// Use this method only when you are sure that the argument `i` is
        /// less than an eighth of the size of this union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_ulong(). This method
        /// get_ulong_() is considered to be slightly faster than the method
        /// get_ulong().
        /// Use this method only when you are sure that `i` is less than
        /// an eighth of the size of this union in bytes.
        /// Otherwise, use its counterpart method
        /// [get_ulong()](#method.get_ulong) for safety.
        /// 
        /// # Example
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// for i in 0..2
        ///     { println!("a_longerunion.get_ulong_({}) = {}", i, a_longerunion.get_ulong_(i)); }
        /// assert_eq!(a_longerunion.get_ulong_(0), 13664881099896654671_u64);
        /// assert_eq!(a_longerunion.get_ulong_(1), 18379818014235068504_u64);
        /// 
        /// // It will panic.
        /// // println!("a_longerunion.get_ulong_(2) = {}", a_longerunion.get_ulong_(2));
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[inline] pub fn get_ulong_(&self, i: usize) -> u64
        {
            #[cfg(target_endian = "little")]    unsafe { self.ulong[i] }
            #[cfg(target_endian = "big")]       unsafe { self.ulong[Self::K-i] }
        }

        // pub fn set_ulong_(&mut self, i: usize, val: u64)
        /// Sets i-th element of its array `ulong` of type `u64`
        /// if `i` is less than an eighth of the size of this union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Arguments i
        /// `i` indicates `i`-th element. 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to an eighth of the size of this
        /// union in bytes, it will panic. So, use this method only when you
        /// are sure that the argument i is less than an eighth of the size of
        /// this union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_ulong(). This method
        /// set_ulong_() is considered to be slightly faster than the method
        /// set_ulong().
        /// Use this method only when you are sure that `i` is less than an eighth
        /// of the size of this union in bytes. Otherwise, use its counterpart
        /// method [set_ulong()](#method.set_ulong) for safety.
        /// 
        /// # Example
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let mut a_longerunion = LongerUnion::new();
        /// a_longerunion.set_ulong_(0, 13664881099896654671_u64);
        /// a_longerunion.set_ulong_(1, 18379818014235068504_u64);
        /// 
        /// // It will panic.
        /// // a_longerunion.set_ulong_(2, 100_u64);
        /// 
        /// for i in 0..2
        ///     { println!("a_longerunion.get_ulong_({}) = {}", i, a_longerunion.get_ulong_(i)); }
        /// assert_eq!(a_longerunion.get_ulong_(0), 13664881099896654671_u64);
        /// assert_eq!(a_longerunion.get_ulong_(1), 18379818014235068504_u64);
        /// assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[inline] pub fn set_ulong_(&mut self, i: usize, val: u64) 
        {
            #[cfg(target_endian = "little")]    unsafe { self.ulong[i] = val; }
            #[cfg(target_endian = "big")]       unsafe { self.ulong[Self::K-i] = val; }
        }

        // pub fn get_slong_(&self, i: usize) -> i64
        /// Returns i-th element of array `slong` of type `i64`
        /// if `i` is less than an eighth of the size of this union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Arguments
        /// `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// 
        /// # Output
        /// i-th element of array `slong` of type `i64` if `i` is less than an
        /// eighth of the size of this union in byte
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to an eighth of the size of this
        /// union in bytes, it will panic.
        /// 
        /// # Caution
        /// Use this method only when you are sure that the argument `i` is
        /// less than an eighth of the size of this union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_slong(). This method
        /// get_slong_() is considered to be slightly faster than the method
        /// get_slong().
        /// Use this method only when you are sure that `i` is less than
        /// an eighth of the size of this union in bytes.
        /// Otherwise, use its counterpart method
        /// [get_slong()](#method.get_slong) for safety.
        /// 
        /// # Example
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// for i in 0..2
        ///     { println!("a_longerunion.get_sint_({}) = {}", i, a_longerunion.get_sint_(i)); }
        /// assert_eq!(a_longerunion.get_slong_(0), -4781862973812896945_i64);
        /// assert_eq!(a_longerunion.get_slong_(1), -66926059474483112_i64);
        /// 
        /// // It will panic.
        /// // println!("a_longerunion.get_slong_(2) = {}", a_longerunion.get_slong_(2));
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[inline] pub fn get_slong_(&self, i: usize) -> i64
        {
            #[cfg(target_endian = "little")]    unsafe { self.slong[i] }
            #[cfg(target_endian = "big")]       unsafe { self.slong[Self::K-i] }
        }

        // pub fn set_slong_(&mut self, i: usize, val: i64)
        /// Sets i-th element of its array `slong` of type `i64`
        /// if `i` is less than an eighth of the size of this union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Arguments
        /// - `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness
        /// - `val` is the value of type `i64` to set the i-th element of its
        /// array `slong` of type `i64`.
        /// 
        /// # Output
        /// i-th element of its array `slong` of type `i64`
        /// if `i` is less than an eighth of the size of this union in bytes.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to an eighth of the size of this
        /// union in bytes, it will panic.
        /// 
        /// # Caution
        /// Use this method only when you are sure that the argument i is less
        /// than an eighth of the size of this union.
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_slong(). This method
        /// set_slong_() is considered to be slightly faster than the method
        /// set_slong().
        /// Use this method only only when you are sure that `i` is less than
        /// an eighth of the size of this union in bytes. Otherwise, use its
        /// counterpart method [set_slong()](#method.set_slong) for safety.
        /// 
        /// # Example
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let mut a_longerunion = LongerUnion::new();
        /// a_longerunion.set_slong_(0, -4781862973812896945_i64);
        /// a_longerunion.set_slong_(1, -66926059474483112_i64);
        /// 
        /// // It will panic.
        /// // a_longerunion.set_slong_(2, 100_i64);
        /// 
        /// for i in 0..2
        ///     { println!("a_longerunion.get_slong_({}) = {}", i, a_longerunion.get_slong_(i)); }
        /// assert_eq!(a_longerunion.get_slong_(0), -4781862973812896945_i64);
        /// assert_eq!(a_longerunion.get_slong_(1), -66926059474483112_i64);
        /// assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[inline] pub fn set_slong_(&mut self, i: usize, val: i64) 
        {
            #[cfg(target_endian = "little")]    unsafe { self.slong[i] = val; }
            #[cfg(target_endian = "big")]       unsafe { self.slong[Self::K-i] = val; }
        }

        // pub fn get_ulong(&self, i: usize) -> Option<u64>
        /// Returns i-th element of array `ulong` of type `u64` wrapped in `Some`
        /// of enum `Option` if `i` is less than an eighth of the size of this
        /// union in bytes. Otherwise, it returns None of enum `Option`.
        /// 
        /// # Arguments
        /// `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// 
        /// # Output
        /// - i-th element of array `ulong` of type `u64` wrapped in `Some`
        /// of enum `Option` if `i` is less than an eighth of the size of this
        /// union in bytes
        /// - `None` if `i` is greater than or equal to an eighth of the size
        /// of this union in bytes
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// an eighth of the size of this union in bytes. Otherwise, you can
        /// use its counterpart method [get_ulong_()](#method.get_ulong_)
        /// for performance.
        /// 
        /// # Example
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// for i in 0..2
        /// {
        ///     match a_longerunion.get_ulong(i)
        ///     {
        ///         Some(a) => { println!("a_longerunion.get_ulong({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// 
        /// assert_eq!(a_longerunion.get_ulong(0), Some(13664881099896654671_u64));
        /// assert_eq!(a_longerunion.get_ulong(1), Some(18379818014235068504_u64));
        /// assert_eq!(a_longerunion.get_ulong(4), None);
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        pub fn get_ulong(&self, i: usize) -> Option<u64>
        {
            if i <= Self::K
            {
                #[cfg(target_endian = "little")]    unsafe { Some(self.ulong[i]) }
                #[cfg(target_endian = "big")]       unsafe { Some(self.ulong[Self::K-i]) }
            }
            else
            {
                None
            }
        }

        // pub fn set_ulong(&mut self, i: usize, val: u64) -> bool
        /// Sets i-th element of its array `ulong` of type `u64` and returns
        /// true if `i` is less than an eighth of the size of this union in
        /// bytes. Otherwise, it will set nothing amd return false.
        /// 
        /// # Arguments
        /// - `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// - `val` is the value of type `u64` to set the i-th element of its
        /// array `ulong` of type `u64`.
        /// 
        /// # Output
        /// - `true` if `i` is less than an eighth of the size of this union in
        /// bytes
        /// - `false` if `i` is greater than or equal to an eighth of the size of
        /// this union in bytes
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// an eighth of the size of this union in bytes. Otherwise, you can
        /// use its counterpart method [set_ulong_()](#method.set_ulong_)
        /// for performance.
        /// 
        /// # Example
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let mut a_longerunion = LongerUnion::new();
        /// let b0 = a_longerunion.set_ulong(0, 13664881099896654671_u64);
        /// let b1 = a_longerunion.set_ulong(1, 18379818014235068504_u64);
        /// let b2 = a_longerunion.set_ulong(4, 100_u64);
        /// assert_eq!(b0, true);
        /// assert_eq!(b1, true);
        /// assert_eq!(b2, false);
        /// 
        /// for i in 0..2
        /// {
        ///     match a_longerunion.get_ulong(i)
        ///     {
        ///         Some(a) => { println!("a_longunion.get_ulong({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longerunion.get_ulong(0), Some(13664881099896654671_u64));
        /// assert_eq!(a_longerunion.get_ulong(1), Some(18379818014235068504_u64));
        /// assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        pub fn set_ulong(&mut self, i: usize, val: u64) -> bool
        {
            if i <= Self::L
            { 
                #[cfg(target_endian = "little")]    unsafe { self.ulong[i] = val; }
                #[cfg(target_endian = "big")]       unsafe { self.ulong[Self::K-i] = val; }
                true
            }
            else
            {
                false
            }
        }

        // pub fn get_slong(&self, i: usize) -> Option<i64>
        /// Returns i-th element of array `slong` of type `i64` wrapped in `Some`
        /// of enum `Option` if `i` is less than an eighth of the size of this
        /// union in bytes. Otherwise, it returns None of enum `Option`.
        /// 
        /// # Arguments
        /// `i` indicates `i`-th element. 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Output
        /// - i-th element of array `slong` of type `i64` wrapped in `Some`
        /// of enum `Option` if `i` is less than an eighth of the size of this
        /// Union in bytes
        /// - `None` if `i` is geater than or equal to an eighth of the size of
        /// this union in bytes
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// an eighth of the size of this union in bytes. Otherwise, you can
        /// use its counterpart method [get_slong_()](#method.get_slong_)
        /// for performance.
        /// 
        /// # Example
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// for i in 0..2
        /// {
        ///     match a_longerunion.get_slong(i)
        ///     {
        ///         Some(a) => { println!("a_longerunion.get_slong({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longerunion.get_slong(0), Some(-4781862973812896945_i64));
        /// assert_eq!(a_longerunion.get_slong(1), Some(-66926059474483112_i64));
        /// assert_eq!(a_longerunion.get_sint(4), None);
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        pub fn get_slong(&self, i: usize) -> Option<i64>
        {
            if i <= Self::K
            {
                #[cfg(target_endian = "little")]    unsafe { Some(self.slong[i]) }
                #[cfg(target_endian = "big")]       unsafe { Some(self.slong[Self::K-i]) }
            }
            else
            {
                None
            }
        }

        // pub fn set_slong(&mut self, i: usize, val: i64) -> bool
        /// Sets i-th element of its array `slong` of type `i64` and returns
        /// true if `i` is less than an eighth of the size of this union in
        /// bytes. Otherwise, it will set nothing amd return false.
        /// 
        /// # Arguments
        /// - `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// - `val` is the value of type `i64` to set the i-th element of its
        /// array `slong` of type `i64`.
        /// 
        /// # Output
        /// - `true` if `i` is less than an eighth of the size of this union
        /// in bytes
        /// - `false` if `i` is greater than or equal to an eighth of the size
        /// of this union in bytes
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// an eighth of the size of this union in bytes. Otherwise, you can
        /// use its counterpart method
        /// [set_slong_()](#method.set_slong_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let mut a_longerunion = LongerUnion::new();
        /// let b0 = a_longerunion.set_slong(0, -4781862973812896945_i64);
        /// let b1 = a_longerunion.set_slong(1, -66926059474483112_i64);
        /// let b2 = a_longerunion.set_slong(4, 100_i64);
        /// assert_eq!(b0, true);
        /// assert_eq!(b1, true);
        /// assert_eq!(b2, false);
        /// 
        /// for i in 0..2
        /// {
        ///     match a_longerunion.get_slong(i)
        ///     {
        ///         Some(a) => { println!("a_longerunion.get_slong({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longerunion.get_slong(0), Some(-4781862973812896945_i64));
        /// assert_eq!(a_longerunion.get_slong(1), Some(-66926059474483112_i64));
        /// assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        pub fn set_slong(&mut self, i: usize, val: i64) -> bool
        {
            if i <= Self::L
            { 
                #[cfg(target_endian = "little")]    unsafe { self.slong[i] = val; }
                #[cfg(target_endian = "big")]       unsafe { self.slong[Self::K-i] = val; }
                true
            }
            else
            {
                false
            }
        }
    }
}
pub(super) use get_set_long;


macro_rules! get_set_longer_fit {
    () => {
        // pub fn get_ulonger(self) -> u128
        /// Returns its value as `u128`.
        /// 
        /// # Output
        /// Its value as `u128`
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::LongerUnion;    
        /// let a = LongerUnion::new_with(98765432101234567898765432101234546789_u128);
        /// println!("a = {}", a.get_ulonger());
        /// assert_eq!(a.get_ulonger(), 98765432101234567898765432101234546789_u128);
        /// ```
        #[inline] pub fn get_ulonger(self) -> u128      { unsafe { self.ulonger } }

        // pub fn set_ulonger(&mut self, val: u128)
        /// Sets its value with `val` of type `u128`
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::LongerUnion;    
        /// let mut a = LongerUnion::new();
        /// a.set_ulonger(98765432101234567898765432101234546789_u128);
        /// println!("a = {}", a.get_ulonger());
        /// assert_eq!(a.get_ulonger(), 98765432101234567898765432101234546789_u128);
        /// ```
        #[inline] pub fn set_ulonger(&mut self, val: u128)  { self.ulonger = val; }

        // pub fn get_slonger(self) -> i128
        /// Returns its value as `i128`.
        /// 
        /// # Output
        /// Its value as `i128`
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::LongerUnion;    
        /// let a = LongerUnion::new_with(234567890987654321012345678987654321234_u128);
        /// println!("a = {}", a.get_slonger());
        /// assert_eq!(a.get_slonger(), -105714475933284142451028928444113890222_i128);
        /// ```
        #[inline] pub fn get_slonger(self) -> i128      { unsafe { self.slonger } }

        // pub fn set_slonger(&mut self, val: i128)
        /// Sets its value with `val` of type `i128`
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::LongerUnion;    
        /// let mut a = LongerUnion::new();
        /// a.set_slonger(-105714475933284142451028928444113890222_i128);
        /// println!("a = {}", a.get_slonger());
        /// assert_eq!(a.get_slonger(), -105714475933284142451028928444113890222_i128);
        /// ```
        #[inline] pub fn set_slonger(&mut self, val: i128)  { self.slonger = val; }
    };
}
pub(super) use get_set_longer_fit;


macro_rules! get_set_size_fit {
    () => {
        // pub fn get_usize(self) -> usize
        /// Returns its value as `usize`.
        /// 
        /// # Output
        /// Its value as `usize`
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::SizeUnion;    
        /// let a = SizeUnion::new_with(250_usize);
        /// println!("a = {}", a.get_usize());
        /// assert_eq!(a.get_usize(), 250_usize);
        /// ```
        #[inline] pub fn get_usize(self) -> usize   { unsafe { self.u_size } }

        // pub fn set_usize(&mut self, val: usize)
        /// Sets its value with `val` of type `usize`
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::SizeUnion;    
        /// let mut a = SizeUnion::new();
        /// a.set_usize(234_usize);
        /// println!("a = {}", a.get_usize());
        /// assert_eq!(a.get_usize(), 234_usize);
        /// ```
        #[inline] pub fn set_usize(&mut self, val: usize)   { self.u_size = val; }

        // pub fn get_ssize(self) -> isize
        /// Returns its value as `isize`.
        /// 
        /// # Output
        /// Its value as `isize`
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::SizeUnion;    
        /// let a = SizeUnion::new_with_signed(-123_isize);
        /// println!("a = {}", a.get_ssize());
        /// assert_eq!(a.get_ssize(), -123_isize);
        /// ```
        #[inline] pub fn get_ssize(self) -> isize   { unsafe { self.s_size } }

        // pub fn set_ssize(&mut self, val: isize)
        /// Sets its value with `val` of type `isize`
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::SizeUnion;    
        /// let mut a = SizeUnion::new();
        /// a.set_ssize(-123_isize);
        /// println!("a = {}", a.get_ssize());
        /// assert_eq!(a.get_ssize(), -123_isize);
        /// ```
        #[inline] pub fn set_ssize(&mut self, val: isize)   { self.s_size = val; }
    };
}
pub(super) use get_set_size_fit;

macro_rules! get_set_size {
    ($f:expr) => {
        const J: usize = $f - 1;

        // pub fn get_usize_(&self, i: usize) -> usize
        /// Returns i-th element of array `u_size` of type `usize`
        /// if `i` is less than the size of this union in bytes divided by
        /// the size of the type `usize` in bytes. Otherwise, it will panic.
        /// 
        /// # Arguments
        /// `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// 
        /// # Output
        /// i-th element of array `u_size` of type `usize`
        /// if `i` is less than the size of this union in bytes divided by
        /// the size of the type `usize` in bytes
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to the size of this union
        /// in bytes divided by the size of the type `usize` in bytes,
        /// it will panic.
        /// 
        /// # Caution
        /// Use this method only when you are sure that the argument `i` is
        /// less than the size of this union in bytes divided by the size of
        /// the type `usize` in bytes
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_usize(). This method
        /// get_usize_() is considered to be slightly faster than the method
        /// get_usize().
        /// Use this method only when you are sure that `i` is less than
        /// the size of this union in bytes divided by the size of the type
        /// `usize` in bytes. Otherwise, use its counterpart method
        /// [get_usize()](#method.get_usize) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```

        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[inline] pub fn get_usize_(&self, i: usize) -> usize
        {
            #[cfg(target_endian = "little")]    unsafe { self.u_size[i] }
            #[cfg(target_endian = "big")]       unsafe { self.u_size[Self::J-i] }
        }

        // pub fn set_usize_(&mut self, i: usize, val: usize) 
        /// Sets i-th element of its array `u_size` of type `usize`
        /// if `i` is less than the size of this union in bytes divided
        /// by the size of the type `usize` in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Arguments
        /// - `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// - `val` is the value of type `usize` to set the i-th element of its
        ///  array `u_size` of type `usize`.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to the size of this union
        /// in bytes divided by the size of the type `usize` in bytes,
        /// it will panic.
        /// 
        /// # Caution
        /// Use this method only when you are sure that the argument `i` is
        /// less than the size of this union in bytes divided by the size of
        /// the type `usize` in bytes
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_usize(). This method
        /// set_usize_() is considered to be slightly faster than the method
        /// set_usize().
        /// Use this method only when you are sure that `i` is less than
        /// the size of this union in bytes divided by the size of the type
        /// `usize` in bytes. Otherwise, use its counterpart
        /// method [set_usize()](#method.set_usize) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[inline] pub fn set_usize_(&mut self, i: usize, val: usize) 
        {
            #[cfg(target_endian = "little")]    unsafe { self.u_size[i] = val; }
            #[cfg(target_endian = "big")]       unsafe { self.u_size[Self::J-i] = val; }
        }

        // pub fn get_ssize_(&self, i: usize) -> isize
        /// Returns i-th element of array `s_size` of type `isize`
        /// if `i` is less than the size of this union in bytes divided by
        /// the size of the type `isize` in bytes. Otherwise, it will panic.
        /// 
        /// # Arguments
        /// - `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// 
        /// # Output
        /// i-th element of array `s_size` of type `isize`
        /// if `i` is less than the size of this union in bytes divided by
        /// the size of the type `isize` in bytes
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to the size of this union
        /// in bytes divided by the size of the type `isize` in bytes,
        /// it will panic.
        /// 
        /// # Caution
        /// Use this method only when you are sure that the argument `i` is
        /// less than the size of this union in bytes divided by the size of
        /// the type `usize` in bytes
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_ssize(). This method
        /// get_ssize_() is considered to be slightly faster than the method
        /// get_ssize().
        /// Use this method only when you are sure that `i` is less than
        /// the size of this union in bytes divided by the size of the type
        /// `isize` in bytes. Otherwise, use its counterpart method
        /// [get_ssize()](#method.get_ssize) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[inline] pub fn get_ssize_(&self, i: usize) -> isize
        {
            #[cfg(target_endian = "little")]    unsafe { self.s_size[i] }
            #[cfg(target_endian = "big")]       unsafe { self.s_size[Self::J-i] }
        }

        // pub fn set_ssize_(&mut self, i: usize, val: isize) 
        /// Sets i-th element of its array `s_size` of type `isize`
        /// if `i` is less than the size of this union in bytes divided
        /// by the size of the type `isize` in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Arguments
        /// - `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// - `val` is the value of type `isize` to set the i-th element of its
        ///  array `s_size` of type `isize`.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to the size of this union
        /// in bytes divided by the size of the type `isize` in bytes,
        /// it will panic.
        /// 
        /// # Caution
        /// Use this method only when you are sure that the argument `i` is
        /// less than the size of this union in bytes divided by the size of
        /// the type `isize` in bytes
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_ssize(). This method
        /// set_ssize_() is considered to be slightly faster than the method
        /// set_ssize().
        /// Use this method only when you are sure that `i` is less than
        /// the size of this union in bytes divided by the size of the type
        /// `isize` in bytes. Otherwise, use its counterpart method
        /// [set_ssize()](#method.set_ssize) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[inline] pub fn set_ssize_(&mut self, i: usize, val: isize) 
        {
            #[cfg(target_endian = "little")]    unsafe { self.s_size[i] = val; }
            #[cfg(target_endian = "big")]       unsafe { self.s_size[Self::J-i] = val; }
        }

        // pub fn get_usize(&self, i: usize) -> Option<usize>
        /// Returns i-th element of array `u_size` of type `usize` wrapped
        /// in Some of enum `Option` if `i` is less than the size of this union
        /// in bytes divided by the size of the type `usize` in bytes.
        /// Otherwise, it returns `None` of enum `Option`.
        /// 
        /// # Arguments
        /// `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// 
        /// # Output
        /// - i-th element of array `u_size` of type `usize` wrapped
        /// in Some of enum `Option` if `i` is less than the size of this
        /// union in bytes divided by the size of the type `usize` in bytes
        /// - `None` if `i` is greater than or equal to the size of this
        /// union in bytes divided by the size of the type `usize` in bytes
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// the size of this union in bytes divided by the size of the type
        /// `usize` in bytes. Otherwise, you can use its counterpart method
        /// [get_usize_()](#method.get_usize_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        pub fn get_usize(&self, i: usize) -> Option<usize>
        {
            if i <= Self::J
            {
                #[cfg(target_endian = "little")]    unsafe { Some(self.u_size[i]) }
                #[cfg(target_endian = "big")]       unsafe { Some(self.u_size[Self::J-i]) }
            }
            else
            {
                None
            }
        }

        // pub fn get_ssize(&self, i: usize) -> Option<isize>
        /// Returns i-th element of array `s_size` of type `isize` wrapped
        /// in Some of enum `Option` if `i` is less than the size of this union
        /// in bytes divided by the size of the type `isize` in bytes.
        /// Otherwise, it returns None of enum `Option`.
        /// 
        /// # Arguments
        /// `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// 
        /// # Output
        /// i-th element of array `s_size` of type `isize` wrapped
        /// in Some of enum `Option` if `i` is less than the size of this union
        /// in bytes divided by the size of the type `isize` in bytes
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// the size of this union in bytes divided by the size of the type
        /// `isize` in bytes. Otherwise, you can use its counterpart method
        /// [get_ssize_()](#method.get_ssize_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // to do
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        pub fn get_ssize(&self, i: usize) -> Option<isize>
        {
            if i <= Self::J
            {
                #[cfg(target_endian = "little")]    unsafe { Some(self.s_size[i]) }
                #[cfg(target_endian = "big")]       unsafe { Some(self.s_size[Self::J-i]) }
            }
            else
            {
                None
            }
        }

        // pub fn set_usize(&mut self, i: usize, val:usize) -> bool
        /// Sets i-th element of its array `u_size` of type `usize` and returns
        /// true if `i` is less than the size of this union in bytes divided
        /// by the size of the type `usize` in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Arguments
        /// `i` indicates `i`-th element. 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// the size of this union in bytes divided by the size of the type
        /// `usize` in bytes. Otherwise, use its counterpart
        /// method [set_usize_()](#method.set_usize_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        #[cfg(target_endian = "little")]
        pub fn set_usize(&mut self, i: usize, val:usize) -> bool
        {
            if i <= Self::J
            { 
                unsafe { self.u_size[i] = val; }
                true
            }
            else
            {
                false
            }
        }

        /// Sets i-th element of its array `u_size` of type `usize` and returns
        /// true if `i` is less than the size of this union in bytes divided
        /// by the size of the type `usize` in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Arguments i
        /// `i` indicates `i`-th element. 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// the size of this union in bytes divided by the size of the type
        /// `usize` in bytes. Otherwise, use its counterpart
        /// method [set_usize_()](#method.set_usize_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        pub fn set_usize(&self, i: usize, val: usize) -> bool
        {
            if i <= Self::J
            { 
                unsafe { self.u_size[Self::J-i] = val; }
                true
            }
            else
            {
                false
            }
        }

        /// Sets i-th element of its array `ssize` of type `isize` and returns
        /// true if `i` is less than the size of this union in bytes divided
        /// by the size of the type `isize` in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Arguments i
        /// `i` indicates `i`-th element. 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// the size of this union in bytes divided by the size of the type
        /// `isize` in bytes. Otherwise, use its counterpart
        /// method [set_ssize_()](#method.set_ssize_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        #[cfg(target_endian = "little")]
        pub fn set_ssize(&mut self, i: usize, val: isize) -> bool
        {
            if i <= Self::J
            { 
                unsafe { self.s_size[i] = val; }
                true
            }
            else
            {
                false
            }
        }

        /// Sets i-th element of its array `ssize` of type `isize` and returns
        /// true if `i` is less than the size of this union in bytes divided
        /// by the size of the type `isize` in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Arguments i
        /// `i` indicates `i`-th element. 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// the size of this union in bytes divided by the size of the type
        /// `isize` in bytes. Otherwise, use its counterpart
        /// method [set_ssize_()](#method.set_ssize_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// // todo
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        pub fn set_ssize(&self, i: usize, val: isize) -> bool
        {
            if i <= Self::J
            { 
                unsafe { self.s_size[Self::J-i] = val; }
                true
            }
            else
            {
                false
            }
        }
    }
}
pub(super) use get_set_size;

macro_rules! integer_union_methods {
    ($f:ty) => {
        pub fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool)
        {
            let (r_this, c1) = self.get().overflowing_add(rhs.get());
            let cc = if carry { 1 as $f } else { 0 as $f };
            let (res_this, c2) = r_this.overflowing_add(cc);
            let res = Self::new_with(res_this);
            (res, c1 || c2)
        }

        #[inline] pub fn wrapping_add(self, rhs: Self) -> Self      { Self::new_with( self.get().wrapping_add(rhs.get()) ) }
        // #[inline] pub fn wrapping_add_assign(&mut self, rhs: Self)  { self.set(self.get().wrapping_add(rhs.get())); }

        pub fn overflowing_add(self, rhs: Self) -> (Self, bool)
        {
            let (res_this, carry) = self.get().overflowing_add(rhs.get());
            (Self::new_with(res_this), carry)
        }

        pub fn checked_add(self, rhs: Self) -> Option<Self>
        {
            match self.get().checked_add(rhs.get())
            {
                Some(res_this) =>   { Some(Self::new_with(res_this)) },
                None =>             { None },
            }
        }

        #[inline] pub fn unchecked_add(self, rhs: Self) -> Self     { Self::new_with( self.get().checked_add(rhs.get()).unwrap() ) }
        #[inline] pub fn saturating_add(self, rhs: Self) -> Self    { Self::new_with( self.get().saturating_add(rhs.get()) ) }

        pub fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool)
        {
            let (r_this, b1) = self.get().overflowing_sub(rhs.get());
            let (res_this, b2) = r_this.overflowing_sub(borrow as $f);
            (Self::new_with(res_this), b1 || b2)
        }

        #[inline] pub fn wrapping_sub(self, rhs: Self) -> Self      { Self::new_with( self.get().wrapping_sub(rhs.get()) ) }
        // #[inline] pub fn wrapping_sub_assign(&mut self, rhs: Self)  { self.set(self.get().wrapping_sub(rhs.get())); }
        
        pub fn overflowing_sub(self, rhs: Self) -> (Self, bool)
        {
            let (res_this, borrow) = self.get().overflowing_sub(rhs.get());
            (Self::new_with(res_this), borrow)
        }

        pub fn checked_sub(self, rhs: Self) -> Option<Self>
        {
            match self.get().checked_sub(rhs.get())
            {
                Some(res_this) =>   { Some(Self::new_with(res_this)) },
                None =>             { None },
            }
        }
        
        #[inline] pub fn unchecked_sub(self, rhs: Self) -> Self     { Self::new_with( self.get().checked_sub(rhs.get()).unwrap() ) }
        #[inline] pub fn saturating_sub(self, rhs: Self) -> Self    { Self::new_with( self.get().saturating_sub(rhs.get()) ) }

        #[inline] pub fn abs_diff(self, other: Self) -> Self    { Self::new_with( self.get().abs_diff(other.get()) ) }

        #[inline] pub fn wrapping_mul(self, rhs: Self) -> Self      { Self::new_with( self.get().wrapping_mul(rhs.get()) ) }
        // #[inline] pub fn wrapping_mul_assign(&mut self, rhs: Self)  { self.set(self.get().wrapping_mul(rhs.get())); }
        
        pub fn overflowing_mul(self, rhs: Self) -> (Self, bool)
        {
            let (res_this, carry) = self.get().overflowing_mul(rhs.get());
            (Self::new_with(res_this), carry)
        }

        pub fn checked_mul(self, rhs: Self) -> Option<Self>
        {
            match self.get().checked_mul(rhs.get())
            {
                Some(res_this) =>   { Some(Self::new_with(res_this)) },
                None =>             { None },
            }
        }

        #[inline] pub fn unchecked_mul(self, rhs: Self) -> Self     { Self::new_with( self.get().checked_mul(rhs.get()).unwrap() ) }
        #[inline] pub fn saturating_mul(self, rhs: Self) -> Self    { Self::new_with( self.get().saturating_mul(rhs.get()) ) }

        #[inline] pub fn wrapping_div(self, rhs: Self) -> Self      { Self::new_with( self.get().wrapping_div(rhs.get()) ) }
        // #[inline] pub fn wrapping_div_assign(&mut self, rhs: Self)  { self.this = self.get().wrapping_div(rhs.get()); }
        
        pub fn overflowing_div(self, rhs: Self) -> (Self, bool)
        {
            let (res_this, carry) = self.get().overflowing_div(rhs.get());
            (Self::new_with(res_this), carry)
        }

        pub fn checked_div(self, rhs: Self) -> Option<Self>
        {
            match self.get().checked_div(rhs.get())
            {
                Some(res_this) =>   { Some(Self::new_with(res_this)) },
                None =>             { None },
            }
        }

        #[inline] pub fn saturating_div(self, rhs: Self) -> Self    { Self::new_with( self.get().saturating_div(rhs.get()) ) }

        #[inline] pub fn wrapping_rem(self, rhs: Self) -> Self      { Self::new_with( self.get().wrapping_rem(rhs.get()) ) }
        // #[inline] pub fn wrapping_rem_assign(&mut self, rhs: Self)  { self.set(self.get().wrapping_rem(rhs.get())); }

        pub fn overflowing_rem(self, rhs: Self) -> (Self, bool)
        {
            let (res_this, carry) = self.get().overflowing_rem(rhs.get());
            (Self::new_with(res_this), carry)
        }

        pub fn checked_rem(self, rhs: Self) -> Option<Self>
        {
            match self.get().checked_rem(rhs.get())
            {
                Some(res_this) =>   { Some(Self::new_with(res_this)) },
                None =>             { None },
            }
        }

        #[inline] pub fn wrapping_neg(self) -> Self             { Self::new_with( self.get().wrapping_neg() ) }

        #[inline] pub fn wrapping_pow(self, exp: u32) -> Self   { Self::new_with( self.get().wrapping_pow(exp) ) }
        
        pub fn checked_pow(self, exp: u32) -> Option<Self>
        {
            match self.get().checked_pow(exp)
            {
                Some(res_this) =>   { Some(Self::new_with(res_this)) },
                None =>             { None },
            }
        }

        pub fn overflowing_pow(self, exp: u32) -> (Self, bool)
        {
            let (res_this, carry) = self.get().overflowing_pow(exp);
            (Self::new_with(res_this), carry)
        }

        #[inline] pub fn saturating_pow(self, exp: u32) -> Self { Self::new_with( self.get().saturating_pow(exp) ) }

        #[inline] pub fn pow(self, exp: u32) -> Self    { Self::new_with( self.get().pow(exp) ) }

        #[inline] pub fn ilog(self, base: Self) -> u32  { self.get().ilog(base.get()) }
        #[inline] pub fn ilog10(self) -> u32            { self.get().ilog10() }
        #[inline] pub fn ilog2(self) -> u32             { self.get().ilog2() }

        #[inline] pub fn isqrt(self) -> Self             { Self::new_with( self.get()._isqrt() ) }
        #[inline] pub fn root(self, exp: Self) -> Self  { Self::new_with( self.get().root(exp.get()) ) }

        #[inline] pub fn reverse_bits(self) -> Self     { Self::new_with( self.get().reverse_bits() ) }
        // #[inline] pub fn reverse_bits_assign(&mut self) { self.get().reverse_bits_assign(); }
        #[inline] pub fn rotate_left(self, n: u32) -> Self  { Self::new_with( self.get().rotate_left(n) ) }
        #[inline] pub fn rotate_right(self, n: u32) -> Self { Self::new_with( self.get().rotate_right(n) ) }

        #[inline] pub fn count_ones(self) -> u32        { self.get().count_ones() }
        #[inline] pub fn count_zeros(self) -> u32       { self.get().count_zeros() }
        #[inline] pub fn leading_ones(self) -> u32      { self.get().leading_ones() }
        #[inline] pub fn leading_zeros(self) -> u32     { self.get().leading_zeros() }
        #[inline] pub fn trailing_ones(self) -> u32     { self.get().trailing_ones() }
        #[inline] pub fn trailing_zeros(self) -> u32    { self.get().trailing_zeros() }

        #[inline] pub fn from_be(x: Self) -> Self   { Self::new_with( <$f>::from_be(x.get()) ) }
        #[inline] pub fn from_le(x: Self) -> Self   { Self::new_with( <$f>::from_le(x.get()) ) }
        #[inline] pub fn to_be(self) -> Self        { Self::new_with( self.get().to_be() ) }
        #[inline] pub fn to_le(self) -> Self        { Self::new_with( self.get().to_le() ) }
        #[inline] pub fn swap_bytes(self) -> Self   { Self::new_with( self.get().swap_bytes() ) }

        #[inline] pub fn is_power_of_two(self) -> bool    { self.get().is_power_of_two() }
        #[inline] pub fn next_power_of_two(self) -> Self  { Self::new_with( self.get().next_power_of_two() ) }

        // #[inline] pub fn into_f64(self) -> f64      { self.get() as f64 }
        // #[inline] pub fn into_f32(self) -> f32      { self.get() as f32 }
        // #[inline] pub fn into_u128(self) -> u128    { self.get() as u128 }
        // #[inline] pub fn into_u64(self) -> u64      { self.get() as u64 }
        // #[inline] pub fn into_u32(self) -> u32      { self.get() as u32 }
        // #[inline] pub fn into_u16(self) -> u16      { self.get() as u16 }
        // #[inline] pub fn into_u8(self) -> u8        { self.get() as u8 }
        // #[inline] pub fn into_usize(self) -> usize  { self.get() as usize }
        // #[inline] pub fn into_bool(self) -> bool    { self.get() != 0 }
        // #[inline] pub fn zero() -> Self             { Self::new_with(0 as $f) }
        // #[inline] pub fn one() -> Self              { Self::new_with(1 as $f) }
        // #[inline] pub fn max() -> Self              { Self::new_with(<$f>::MAX) }
        // #[inline] pub fn min() -> Self              { Self::new_with(<$f>::MIN) }
        // #[inline] pub fn num(n: u128) -> Self       { Self::new_with(n as $f) }
        // #[inline] pub fn size_in_bytes() -> usize   { size_of::<Self>() }
        // #[inline] pub fn size_in_bits() -> usize    { size_of::<Self>() * 8 }
        // #[inline] pub fn length_in_bytes(self) -> usize    { size_of_val(&self) }
        // #[inline] pub fn length_in_bits(self) -> usize     { size_of_val(&self) * 8 }
        // #[inline] pub fn is_odd(self) -> bool       { (self.get() & 1) != 0 }
    }
}
pub(super) use integer_union_methods;


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
pub(super) use operators_for_integer_unions_impl;


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
pub(super) use shift_ops_for_integer_unions_impl;

macro_rules! shift_ops_for_integer_unions_by_self_impl {
    ($f:ty) => {
        impl Shl<$f> for $f
        {
            type Output = Self;

            fn shl(self, rhs: $f) -> Self
            {
                let mut s = self.clone();
                s <<= rhs.get();
                s
            }
        }

        impl ShlAssign<$f> for $f
        {
            #[inline]
            fn shl_assign(&mut self, rhs: $f)
            {
                self.set(self.get() << rhs.get())
            }
        }

        impl Shr<$f> for $f
        {
            type Output = Self;

            fn shr(self, rhs: $f) -> Self
            {
                let mut s = self.clone();
                s >>= rhs.get();
                s
            }
        }

        impl ShrAssign<$f> for $f
        {
            #[inline]
            fn shr_assign(&mut self, rhs: $f)
            {
                self.set(self.get() >> rhs.get())
            }
        }
    }
}
pub(super) use shift_ops_for_integer_unions_by_self_impl;

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
pub(super) use display_for_integer_unions_impl;







/*
macro_rules! random_for_unions_impl {
    (ShortUnion) => {
        /// Constucts a new `SmallUInt`-type object which has the random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.any)
        #[inline] fn any() -> Self      { Self::new_with(thread_rng().gen()) }

        /// Make a `SmallUInt`-type object to have a random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.random)
        #[inline] fn random() -> Self   { Self::new_with(OsRng.next_u32() as u16) }
    };
    (IntUnion) => {
        /// Constucts a new `SmallUInt`-type object which has the random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.any)
        #[inline] fn any() -> Self      { Self::new_with(thread_rng().gen()) }

        /// Make a `SmallUInt`-type object to have a random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.random)
        #[inline] fn random() -> Self   { Self::new_with(OsRng.next_u32() as u32) }
    };
    (LongUnion) => {
        /// Constucts a new `SmallUInt`-type object which has the random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.any)
        #[inline] fn any() -> Self      { Self::new_with(thread_rng().gen()) }
        
        /// Make a `SmallUInt`-type object to have a random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.random)
        #[inline] fn random() -> Self   { Self::new_with(OsRng.next_u64()) }
    };
    (LongerUnion) => {
        /// Make a `SmallUInt`-type object to have a random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.random)
        #[inline] fn any() -> Self      { Self::new_with(thread_rng().gen()) }

        /// Make a `SmallUInt`-type object to have a random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.random)
        fn random() -> Self
        {
            let mut common = LongerUnion::new();
            common.set_ulong_(0, OsRng.next_u64());
            common.set_ulong_(1, OsRng.next_u64());
            common
        }
    };
    (SizeUnion) => {
        /// Constucts a new `SmallUInt`-type object which has the random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.any)
        #[inline] fn any() -> Self      { Self::new_with(thread_rng().gen()) }

        /// Make a `SmallUInt`-type object to have a random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.random)
        #[cfg(target_pointer_width = "8")]
        #[inline] fn random() -> Self   { Self::new_with(OsRng.next_u32() as usize) }

        /// Make a `SmallUInt`-type object to have a random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.random)
        #[cfg(target_pointer_width = "16")]
        #[inline] fn random() -> Self   { Self::new_with(OsRng.next_u32() as usize) }

        /// Make a `SmallUInt`-type object to have a random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.random)
        #[cfg(target_pointer_width = "32")]
        #[inline] fn random() -> Self   { Self::new_with(OsRng.next_u32() as usize) }
        
        /// Make a `SmallUInt`-type object to have a random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.random)
        #[cfg(target_pointer_width = "64")]
        #[inline] fn random() -> Self   { Self::new_with(OsRng.next_u64() as usize) }
        
        /// Make a `SmallUInt`-type object to have a random value.
        /// [Read more in detail](trait@SmallUInt#tymethod.random)
        #[cfg(target_pointer_width = "128")]
        fn random() -> Self
        {
            let mut common = SizeUnion::new();
            common.set_ulong_(0, OsRng.next_u64());
            common.set_ulong_(1, OsRng.next_u64());
            common
        }
    };
}
*/
