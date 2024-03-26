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
        /// if `i` is less than the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument
        /// `i` indicates 0-th element contains LSB (Least Significant Bit),
        /// while (the size of this Union in bytes - 1)-th element contains
        /// MSB (Most Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to the size of this Union in
        /// bytes, it will panic. So, use this method only when you are sure
        /// that the argument i is less than the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_ubyte(). This method
        /// get_ubyte_() is considered to be slightly faster than the method
        /// get_ubyte().
        /// Use this method only when you are sure that `i` is less than the
        /// size of this Union in bytes. Otherwise, use its counterpart method
        /// [get_ubyte()](#method.get_ubyte) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// let a_short = ShortUnion::new_with(2895_u16);
        /// let b_short_u8 = a_short.get_ubyte_(1);
        /// println!("a_short.get_ubyte_(1) = {}", b_short_u8);
        /// assert_eq!(b_short_u8, 11_u8);
        /// 
        /// // It will panic.
        /// // let c_short = a_short.get_ubyte_(2);
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// let a_longer = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// let b_longer_u8 = a_longer.get_ubyte_(3);
        /// println!("a_longer.get_ubyte_(3) = {}", b_longer_u8);
        /// assert_eq!(b_longer_u8, 241_u8);
        /// 
        /// // It will panic.
        /// // let c_longer = a_longer.get_ubyte_(16);
        /// ```
        #[cfg(target_endian = "little")]
        #[inline] pub fn get_ubyte_(&self, i: usize) -> u8 { unsafe { self.ubyte[i] } }

        /// Returns i-th element of array `ubyte` of type `u8`
        /// if `i` is less than the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to the size of this Union in
        /// bytes, it will panic. So, use this method only when you are sure
        /// that the argument i is less than the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_ubyte(). This method
        /// get_ubyte_() is considered to be slightly faster than the method
        /// get_ubyte().
        /// Use this method only when you are sure that `i` is less than the
        /// size of this Union in bytes. Otherwise, use its counterpart method
        /// [get_ubyte()](#method.get_ubyte) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// let a_short = ShortUnion::new_with(2895_u16);
        /// let b_short_u8 = a_short.get_ubyte_(1);
        /// println!("a_short.get_ubyte_(1) = {}", b_short_u8);
        /// assert_eq!(b_short_u8, 11_u8);
        /// 
        /// // It will panic.
        /// // let c_short = a_short.get_ubyte_(2);
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// let a_longer = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// let b_longer_u8 = a_longer.get_ubyte_(3);
        /// println!("a_longer.get_ubyte_(3) = {}", b_longer_u8);
        /// assert_eq!(b_longer_u8, 241_u8);
        /// 
        /// // It will panic.
        /// // let c_longer_u8 = a_longer.get_ubyte_(16);
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not encouraged
        /// to use it for serious purpose. Only use this crate for Big-endian CPUs
        /// with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn get_ubyte_(&self, i: usize) -> u8 { unsafe { self.ubyte[Self::N-i] } }

        /// Returns i-th element of array `sbyte` of type `u8`
        /// if `i` is less than the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to the size of this Union in
        /// bytes, it will panic. So, use this method only when you are sure
        /// that the argument i is less than the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_sbyte(). This method
        /// get_sbyte_() is considered to be slightly faster than the method
        /// get_sbyte().
        /// Use this method only when you are sure that `i` is less than the
        /// size of this Union in bytes. Otherwise, use its counterpart method
        /// [get_sbyte()](#method.get_sbyte) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// let a_short = ShortUnion::new_with(2895_u16);
        /// let b_short_i8 = a_short.get_sbyte_(1);
        /// println!("a_short.get_sbyte_(1) = {}", b_short_i8);
        /// assert_eq!(b_short_i8, 11_i8);
        /// 
        /// // It will panic.
        /// // let c_short_i8 = a_short.get_sbyte_(2);
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// let a_longer = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// let b_longer_u8 = a_longer.get_sbyte_(3);
        /// println!("a_longer.get_sbyte_(3) = {}", b_longer_u8);
        /// assert_eq!(b_longer_u8, -15_i8);
        /// 
        /// // It will panic.
        /// // let c_longer_u8 = a_longer.get_sbyte_(16);
        /// ```
        #[cfg(target_endian = "little")]
        #[inline] pub fn get_sbyte_(&self, i: usize) -> i8 { unsafe { self.sbyte[i] } }

        /// Returns i-th element of array `ubyte` of type `u8`
        /// if `i` is less than the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to the size of this Union in
        /// bytes, it will panic. So, use this method only when you are sure
        /// that the argument i is less than the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_sbyte(). This method
        /// get_sbyte_() is considered to be slightly faster than the method
        /// get_sbyte().
        /// Use this method only when you are sure that `i` is less than the
        /// size of this Union in bytes. Otherwise, use its counterpart method
        /// [get_ubyte()](#method.get_ubyte) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// let a_short = ShortUnion::new_with(2895_u16);
        /// let b_short_i8 = a_short.get_sbyte_(1);
        /// println!("a_short.get_sbyte_(1) = {}", b_short_i8);
        /// assert_eq!(b_short_i8, 11_i8);
        /// 
        /// // It will panic.
        /// // let c_short_i8 = a_short.get_sbyte_(2);
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// let a_longer = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// let b_longer_i8 = a_longer.get_sbyte_(3);
        /// println!("a_longer.get_sbyte_(3) = {}", b_longer_i8);
        /// assert_eq!(b_longer_i8, -15_i8);
        /// 
        /// // It will panic.
        /// // let c_longer_i8 = a_longer.get_sbyte_(16);
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn get_sbyte_(&self, i: usize) -> i8 { unsafe { self.sbyte[Self::N-i] } }

        /// Returns i-th element of array `ubyte` of type `u8` wrapped in Some
        /// of enum Option if `i` is less than the size of this Union in bytes.
        /// Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than the
        /// size of this Union in bytes. Otherwise, use its counterpart method
        /// [get_ubyte_()](#method.get_ubyte_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// let a_short = ShortUnion::new_with(2895_u16);
        /// 
        /// match a_short.get_ubyte(1)
        /// {
        ///     Some(b) =>  {
        ///             println!("a_short.get_ubyte(1) = {}", b);
        ///             assert_eq!(b, 11_u8);
        ///         },
        ///     None =>     { println!("Out of range"); },
        /// }
        /// 
        /// match a_short.get_ubyte(2)
        /// {
        ///     Some(b) =>  { println!("a_short.get_ubyte(2) = {}", b); },
        ///     None =>     {
        ///             println!("Out of range");
        ///             assert_eq!(a_short.get_ubyte(2), None);
        ///         },
        /// }
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// let a_longer = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// match a_longer.get_ubyte(3)
        /// {
        ///     Some(b) =>  {
        ///             println!("a_longer.get_ubyte(3) = {}", b);
        ///             assert_eq!(b, 241_u8);
        ///         },
        ///     None =>     { println!("Out of range"); },
        /// }
        /// 
        /// match a_longer.get_ubyte(16)
        /// {
        ///     Some(b) =>  { println!("a_short.get_ubyte(16) = {}", b); },
        ///     None =>     {
        ///             println!("Out of range");
        ///             assert_eq!(a_longer.get_ubyte(16), None);
        ///         },
        /// }
        /// ```
        #[cfg(target_endian = "little")]
        #[inline] pub fn get_ubyte(&self, i: usize) -> Option<u8>
        {
            if i <= Self::N
                { unsafe { Some(self.ubyte[i]) } }
            else
                { None }
        }

        /// Returns i-th element of array `ubyte` of type `u8` wrapped in Some
        /// of enum Option if `i` is less than the size of this Union in bytes.
        /// Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than the
        /// size of this Union in bytes. Otherwise, use its counterpart method
        /// [get_ubyte_()](#method.get_ubyte_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// let a_short = ShortUnion::new_with(2895_u16);
        /// 
        /// match a_short.get_ubyte(1)
        /// {
        ///     Some(b) =>  {
        ///             println!("a_short.get_ubyte(1) = {}", b);
        ///             assert_eq!(b, 11_u8);
        ///         },
        ///     None =>     { println!("Out of range"); },
        /// }
        /// 
        /// match a_short.get_ubyte(2)
        /// {
        ///     Some(b) =>  { println!("a_short.get_ubyte(2) = {}", b); },
        ///     None =>     {
        ///             println!("Out of range");
        ///             assert_eq!(a_short.get_ubyte(2), None);
        ///         },
        /// }
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// let a_longer = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// match a_longer.get_ubyte(3)
        /// {
        ///     Some(b) =>  {
        ///             println!("a_longer.get_ubyte(3) = {}", b);
        ///             assert_eq!(b, 241_u8);
        ///         },
        ///     None =>     { println!("Out of range"); },
        /// }
        /// 
        /// match a_longer.get_ubyte(16)
        /// {
        ///     Some(b) =>  { println!("a_short.get_ubyte(16) = {}", b); },
        ///     None =>     {
        ///             println!("Out of range");
        ///             assert_eq!(a_longer.get_ubyte(16), None);
        ///         },
        /// }
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        /// with your own full responsibility.
        #[cfg(target_endian = "big")]
        pub fn get_ubyte(&self, i: usize) -> Option<u8>
        {
            if i <= Self::N
                { unsafe { Some(self.ubyte[Self::N-i]) } }
            else
                { None }
        }

        /// Returns i-th element of array `sbyte` of type `u8` wrapped in Some
        /// of enum Option if `i` is less than the size of this Union in bytes.
        /// Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than the
        /// size of this Union in bytes. Otherwise, use its counterpart method
        /// [get_sbyte_()](#method.get_sbyte_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// let a_short = ShortUnion::new_with(2895_u16);
        /// 
        /// match a_short.get_sbyte(1)
        /// {
        ///     Some(b) =>  {
        ///             println!("a_short.get_sbyte(1) = {}", b);
        ///             assert_eq!(b, 11_i8);
        ///         },
        ///     None =>     { println!("Out of range"); },
        /// }
        /// 
        /// match a_short.get_sbyte(2)
        /// {
        ///     Some(b) =>  { println!("a_short.get_sbyte(2) = {}", b); },
        ///     None =>     {
        ///             println!("Out of range");
        ///             assert_eq!(a_short.get_sbyte(2), None);
        ///         },
        /// }
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// let a_longer = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// match a_longer.get_sbyte(3)
        /// {
        ///     Some(b) =>  {
        ///             println!("a_longer.get_sbyte(3) = {}", b);
        ///             assert_eq!(b, -15_i8);
        ///         },
        ///     None =>     { println!("Out of range"); },
        /// }
        /// 
        /// match a_longer.get_sbyte(16)
        /// {
        ///     Some(b) =>  { println!("a_short.get_sbyte(16) = {}", b); },
        ///     None =>     {
        ///             println!("Out of range");
        ///             assert_eq!(a_longer.get_sbyte(16), None);
        ///         },
        /// }
        /// ```
        #[cfg(target_endian = "little")]
        #[inline] pub fn get_sbyte(&self, i: usize) -> Option<i8>
        {
            if i <= Self::N
                { unsafe { Some(self.sbyte[i]) } }
            else
                { None }
        }

        /// Returns i-th element of array `sbyte` of type `u8` wrapped in Some
        /// of enum Option if `i` is less than the size of this Union in bytes.
        /// Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than the
        /// size of this Union in bytes. Otherwise, use its counterpart method
        /// [get_sbyte_()](#method.get_sbyte_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// let a_short = ShortUnion::new_with(2895_u16);
        /// 
        /// match a_short.get_sbyte(1)
        /// {
        ///     Some(b) =>  {
        ///             println!("a_short.get_sbyte(1) = {}", b);
        ///             assert_eq!(b, 11_i8);
        ///         },
        ///     None =>     { println!("Out of range"); },
        /// }
        /// 
        /// match a_short.get_sbyte(2)
        /// {
        ///     Some(b) =>  { println!("a_short.get_sbyte(2) = {}", b); },
        ///     None =>     {
        ///             println!("Out of range");
        ///             assert_eq!(a_short.get_sbyte(2), None);
        ///         },
        /// }
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// let a_longer = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// match a_longer.get_sbyte(3)
        /// {
        ///     Some(b) =>  {
        ///             println!("a_longer.get_sbyte(3) = {}", b);
        ///             assert_eq!(b, -15_i8);
        ///         },
        ///     None =>     { println!("Out of range"); },
        /// }
        /// 
        /// match a_longer.get_sbyte(16)
        /// {
        ///     Some(b) =>  { println!("a_short.get_sbyte(16) = {}", b); },
        ///     None =>     {
        ///             println!("Out of range");
        ///             assert_eq!(a_longer.get_sbyte(16), None);
        ///         },
        /// }
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        pub fn get_sbyte(&self, i: usize) -> Option<i8>
        {
            if i <= Self::N
                { unsafe { Some(self.sbyte[Self::N-i]) } }
            else
                { None }
        }

        /// Sets i-th element of its array `ubyte` of type `u8`
        /// if `i` is less than the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to the size of this Union in
        /// bytes, it will panic. So, use this method only when you are sure
        /// that the argument i is less than the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_ubyte(). This method
        /// set_ubyte_() is considered to be slightly faster than the method
        /// set_ubyte().
        /// Use this method only when you are sure that `i` is less than the
        /// size of this Union in bytes. Otherwise, use its counterpart method
        /// [set_ubyte()](#method.set_ubyte) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// let mut a_short = ShortUnion::new_with(2895_u16);
        /// let mut b_short_u8 = a_short.get_ubyte_(1);
        /// println!("a_short.get_ubyte_(1) = {}", b_short_u8);
        /// a_short.set_ubyte_(1, 0);
        /// b_short_u8 = a_short.get_ubyte_(1);
        /// println!("a_short.get() = {}, a_short.get_ubyte_(1) = {}", a_short, b_short_u8);
        /// assert_eq!(a_short.get(), 79_u16);
        /// assert_eq!(b_short_u8, 0_u8);
        /// 
        /// // It will panic.
        /// // let c_short = a_short.set_ubyte_(2, 0);
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// let mut a_longer = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// let mut b_longer_u8 = a_longer.get_ubyte_(3);
        /// println!("a_longer.get_ubyte_(3) = {}", b_longer_u8);
        /// a_longer.set_ubyte_(3, 0);
        /// println!("a_longer.get() = {}, a_longer.get_ubyte_(3) = {}", a_longer, b_longer_u8);
        /// assert_eq!(a_longer.get(), 339047799029950809142362261748737248079_u128);
        /// assert_eq!(a_longer.get_ubyte_(3), 0_u8);
        /// 
        /// // It will panic.
        /// // let c_longer = a_longer.get_ubyte_(16);
        /// ```
        #[cfg(target_endian = "little")]
        #[inline] pub fn set_ubyte_(&mut self, i: usize, val: u8)  { unsafe { self.ubyte[i] = val; } }

        /// Sets i-th element of its array `ubyte` of type `u8`
        /// if `i` is less than the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to the size of this Union in
        /// bytes, it will panic. So, use this method only when you are sure
        /// that the argument i is less than the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_ubyte(). This method
        /// set_ubyte_() is considered to be slightly faster than the method
        /// set_ubyte().
        /// Use this method only when you are sure that `i` is less than the
        /// size of this Union in bytes. Otherwise, use its counterpart method
        /// [set_ubyte()](#method.set_ubyte) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// let mut a_short = ShortUnion::new_with(2895_u16);
        /// let mut b_short_u8 = a_short.get_ubyte_(1);
        /// println!("a_short.get_ubyte_(1) = {}", b_short_u8);
        /// a_short.set_ubyte_(1, 0);
        /// b_short_u8 = a_short.get_ubyte_(1);
        /// println!("a_short.get() = {}, a_short.get_ubyte_(1) = {}", a_short, b_short_u8);
        /// assert_eq!(a_short.get(), 79_u16);
        /// assert_eq!(b_short_u8, 0_u8);
        /// 
        /// // It will panic.
        /// // let c_short = a_short.set_ubyte_(2, 0);
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// let mut a_longer = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// let mut b_longer_u8 = a_longer.get_ubyte_(3);
        /// println!("a_longer.get_ubyte_(3) = {}", b_longer_u8);
        /// a_longer.set_ubyte_(3, 0);
        /// println!("a_longer.get() = {}, a_longer.get_ubyte_(3) = {}", a_longer, b_longer_u8);
        /// assert_eq!(a_longer.get(), 339047799029950809142362261748737248079_u128);
        /// assert_eq!(a_longer.get_ubyte_(3), 0_u8);
        /// 
        /// // It will panic.
        /// // let c_longer = a_longer.get_ubyte_(16);
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn set_ubyte_(&mut self, i: usize, val: u8)  { unsafe { self.ubyte[Self::N-i] = val; } }

        /// Sets i-th element of its array `sbyte` of type `i8`
        /// if `i` is less than the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to the size of this Union in
        /// bytes, it will panic. So, use this method only when you are sure
        /// that the argument i is less than the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_sbyte(). This method
        /// set_sbyte_() is considered to be slightly faster than the method
        /// set_sbyte().
        /// Use this method only when you are sure that `i` is less than the
        /// size of this Union in bytes. Otherwise, use its counterpart method
        /// [set_sbyte()](#method.set_sbyte) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// let mut a_short = ShortUnion::new_with_signed(79_i16);
        /// let mut b_short_i8 = a_short.get_sbyte_(1);
        /// println!("a_short.get_sbyte_(1) = {}", b_short_i8);
        /// a_short.set_sbyte_(1, 0);
        /// b_short_i8 = a_short.get_sbyte_(1);
        /// println!("a_short.get_signed() = {}, a_short.get_sbyte_(1) = {}", a_short.get_signed(), b_short_i8);
        /// assert_eq!(a_short.get_signed(), 79_i16);
        /// assert_eq!(b_short_i8, 0_i8);
        /// 
        /// // It will panic.
        /// // let c_short = a_short.set_sbyte_(2, 0);
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// let mut a_longer = LongerUnion::new_with_signed(-123456789012345678901234567890123456789_i128);
        /// let mut b_longer_i8 = a_longer.get_sbyte_(3);
        /// println!("a_longer.get_sbyte_(3) = {}", b_longer_i8);
        /// a_longer.set_sbyte_(3, 0);
        /// b_longer_i8 = a_longer.get_sbyte_(3);
        /// println!("a_longer.get_signed() = {}, a_longer.get_sbyte_(3) = {}", a_longer.get_signed(), b_longer_i8);
        /// assert_eq!(a_longer.get_signed(), -123456789012345678901234567891482411285_i128);
        /// assert_eq!(a_longer.get_ubyte_(3), 0_u8);
        /// 
        /// // It will panic.
        /// // let c_longer = a_longer.get_sbyte_(16);
        /// ```
        #[cfg(target_endian = "little")]
        #[inline] pub fn set_sbyte_(&mut self, i: usize, val: i8)  { unsafe { self.sbyte[i] = val; } }

        /// Sets i-th element of its array `sbyte` of type `i8`
        /// if `i` is less than the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to the size of this Union in
        /// bytes, it will panic. So, use this method only when you are sure
        /// that the argument i is less than the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_sbyte(). This method
        /// set_sbyte_() is considered to be slightly faster than the method
        /// set_sbyte().
        /// Use this method only when you are sure that `i` is less than the
        /// size of this Union in bytes. Otherwise, use its counterpart method
        /// [set_sbyte()](#method.set_sbyte) for safety.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// let mut a_short = ShortUnion::new_with_signed(79_i16);
        /// let mut b_short_i8 = a_short.get_sbyte_(1);
        /// println!("a_short.get_sbyte_(1) = {}", b_short_i8);
        /// a_short.set_sbyte_(1, 0);
        /// b_short_i8 = a_short.get_sbyte_(1);
        /// println!("a_short.get_signed() = {}, a_short.get_sbyte_(1) = {}", a_short.get_signed(), b_short_i8);
        /// assert_eq!(a_short.get(), 79_i16);
        /// assert_eq!(b_short_i8, 0_i8);
        /// 
        /// // It will panic.
        /// // let c_short = a_short.set_sbyte_(2, 0);
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// let mut a_longer = LongerUnion::new_with_signed(-123456789012345678901234567890123456789_i128);
        /// let mut b_longer_i8 = a_longer.get_sbyte_(3);
        /// println!("a_longer.get_sbyte_(3) = {}", b_longer_i8);
        /// a_longer.set_sbyte_(3, 0);
        /// b_longer_i8 = a_longer.get_sbyte_(3);
        /// println!("a_longer.get_signed() = {}, a_longer.get_sbyte_(3) = {}", a_longer.get_signed(), b_longer_i8);
        /// assert_eq!(a_longer.get_signed(), -123456789012345678901234567891482411285_i128);
        /// assert_eq!(a_longer.get_ubyte_(3), 0_u8);
        /// 
        /// // It will panic.
        /// // let c_longer = a_longer.get_sbyte_(16);
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn set_sbyte_(&mut self, i: usize, val: i8)  { unsafe { self.sbyte[Self::N-i] = val; } }

        /// Sets i-th element of its array `ubyte` of type `u8` and returns true
        /// if `i` is less than the size of this Union in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than the
        /// size of this Union in bytes. Otherwise, use its counterpart method
        /// [set_ubyte_()](#method.set_ubyte_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// let mut a_short = ShortUnion::new();
        /// let mut succ = a_short.set_sbyte(1, 11);
        /// let mut sbyte = a_short.get_sbyte(1);
        /// if succ
        /// {
        ///     println!("a_short.get() = {}, a_short.get_sbyte(1).unwrap() = {}", a_short, sbyte.unwrap());
        ///     assert_eq!(sbyte.unwrap(), 11_i8);
        /// }
        /// else
        /// {
        ///     println!("Out of range");
        ///     assert_eq!(sbyte, None);
        /// }
        /// 
        /// succ = a_short.set_sbyte(2, 11);
        /// sbyte = a_short.get_sbyte(2);
        /// if succ
        /// {
        ///     println!("a_short.get() = {}, a_short.get_sbyte(2).unwrap() = {}", a_short, sbyte.unwrap());
        ///     assert_eq!(sbyte.unwrap(), 11_i8);
        /// }
        /// else
        /// {
        ///     println!("Out of range");
        ///     assert_eq!(sbyte, None);
        /// }
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// let mut a_longer = LongerUnion::new();
        /// let mut succ = a_longer.set_ubyte(3, 241_u8);
        /// let mut ubyte = a_longer.get_ubyte(3);
        /// if succ
        /// {
        ///     println!("a_longer.get() = {}, a_longer.get_ubyte(3).unwrap() = {}", a_longer, ubyte.unwrap());
        ///     assert_eq!(ubyte.unwrap(), 241_u8);
        /// }
        /// else
        /// {
        ///     println!("Out of range");
        ///     assert_eq!(ubyte, None);
        /// }
        /// 
        /// succ = a_longer.set_ubyte(16, 241_u8);
        /// ubyte = a_longer.get_ubyte(16);
        /// if succ
        /// {
        ///     println!("a_longer.get() = {}, a_longer.get_sbyte(16).unwrap() = {}", a_longer, ubyte.unwrap());
        ///     assert_eq!(ubyte.unwrap(), 241_u8);
        /// }
        /// else
        /// {
        ///     println!("Out of range");
        ///     assert_eq!(ubyte, None);
        /// }
        /// ```
        #[cfg(target_endian = "little")]
        pub fn set_ubyte(&mut self, i: usize, val: u8) -> bool
        {
            if i <= Self::N
            { 
                unsafe { self.ubyte[i] = val; }
                true
            }
            else
            {
                false
            }
        }

        /// Sets i-th element of its array `ubyte` of type `u8` and returns true
        /// if `i` is less than the size of this Union in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than the
        /// size of this Union in bytes. Otherwise, use its counterpart method
        /// [set_ubyte_()](#method.set_ubyte_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// let mut a_short = ShortUnion::new();
        /// let mut succ = a_short.set_sbyte(1, 11);
        /// let mut sbyte = a_short.get_sbyte(1);
        /// if succ
        /// {
        ///     println!("a_short.get() = {}, a_short.get_sbyte(1).unwrap() = {}", a_short, sbyte.unwrap());
        ///     assert_eq!(sbyte.unwrap(), 11_i8);
        /// }
        /// else
        /// {
        ///     println!("Out of range");
        ///     assert_eq!(sbyte, None);
        /// }
        /// 
        /// succ = a_short.set_sbyte(2, 11);
        /// sbyte = a_short.get_sbyte(2);
        /// if succ
        /// {
        ///     println!("a_short.get() = {}, a_short.get_sbyte(2).unwrap() = {}", a_short, sbyte.unwrap());
        ///     assert_eq!(sbyte.unwrap(), 11_i8);
        /// }
        /// else
        /// {
        ///     println!("Out of range");
        ///     assert_eq!(sbyte, None);
        /// }
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// let mut a_longer = LongerUnion::new();
        /// let mut succ = a_longer.set_ubyte(3, 241_u8);
        /// let mut ubyte = a_longer.get_ubyte(3);
        /// if succ
        /// {
        ///     println!("a_longer.get() = {}, a_longer.get_ubyte(3).unwrap() = {}", a_longer, ubyte.unwrap());
        ///     assert_eq!(ubyte.unwrap(), 241_u8);
        /// }
        /// else
        /// {
        ///     println!("Out of range");
        ///     assert_eq!(ubyte, None);
        /// }
        /// 
        /// succ = a_longer.set_ubyte(16, 241_u8);
        /// ubyte = a_longer.get_ubyte(16);
        /// if succ
        /// {
        ///     println!("a_longer.get() = {}, a_longer.get_sbyte(16).unwrap() = {}", a_longer, ubyte.unwrap());
        ///     assert_eq!(ubyte.unwrap(), 241_u8);
        /// }
        /// else
        /// {
        ///     println!("Out of range");
        ///     assert_eq!(ubyte, None);
        /// }
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        pub fn set_ubyte(&self, i: usize, val: u8) -> bool
        {
            if i <= Self::N
            { 
                unsafe { self.ubyte[Self::N-i] = val; }
                true
            }
            else
            {
                false
            }
        }

        /// Sets i-th element of its array `sbyte` of type `i8` and returns true
        /// if `i` is less than the size of this Union in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than the
        /// size of this Union in bytes. Otherwise, use its counterpart method
        /// [set_sbyte_()](#method.set_sbyte_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// let mut a_short = ShortUnion::new();
        /// let mut succ = a_short.set_sbyte(1, 11);
        /// let mut sbyte = a_short.get_sbyte(1);
        /// if succ
        /// {
        ///     println!("a_short.get() = {}, a_short.get_sbyte(1).unwrap() = {}", a_short, sbyte.unwrap());
        ///     assert_eq!(sbyte.unwrap(), 11_i8);
        /// }
        /// else
        /// {
        ///     println!("Out of range");
        ///     assert_eq!(sbyte, None);
        /// }
        /// 
        /// succ = a_short.set_sbyte(2, 11);
        /// sbyte = a_short.get_sbyte(2);
        /// if succ
        /// {
        ///     println!("a_short.get() = {}, a_short.get_sbyte(2).unwrap() = {}", a_short, sbyte.unwrap());
        ///     assert_eq!(sbyte.unwrap(), 11_i8);
        /// }
        /// else
        /// {
        ///     println!("Out of range");
        ///     assert_eq!(sbyte, None);
        /// }
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// let mut a_longer = LongerUnion::new();
        /// let mut succ = a_longer.set_sbyte(3, 81_i8);
        /// let mut sbyte = a_longer.get_sbyte(3);
        /// if succ
        /// {
        ///     println!("a_longer.get_signed() = {}, a_longer.get_sbyte(3).unwrap() = {}", a_longer.get_signed(), sbyte.unwrap());
        ///     assert_eq!(sbyte.unwrap(), 81_i8);
        /// }
        /// else
        /// {
        ///     println!("Out of range");
        ///     assert_eq!(sbyte, None);
        /// }
        /// 
        /// succ = a_longer.set_sbyte(16, 81_i8);
        /// sbyte = a_longer.get_sbyte(16);
        /// if succ
        /// {
        ///     println!("a_longer.get_signed() = {}, a_longer.get_sbyte(16).unwrap() = {}", a_longer.get_signed(), sbyte.unwrap());
        ///     assert_eq!(sbyte.unwrap(), 81_i8);
        /// }
        /// else
        /// {
        ///     println!("Out of range");
        ///     assert_eq!(sbyte, None);
        /// }
        /// ```
        #[cfg(target_endian = "little")]
        pub fn set_sbyte(&mut self, i: usize, val: i8) -> bool
        {
            if i <= Self::N
            { 
                unsafe { self.sbyte[i] = val; }
                true
            }
            else
            {
                false
            }
        }

        /// Sets i-th element of its array `sbyte` of type `i8` and returns true
        /// if `i` is less than the size of this Union in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than the
        /// size of this Union in bytes. Otherwise, use its counterpart method
        /// [set_sbyte_()](#method.set_sbyte_) for performance.
        /// 
        /// # Example for ShortUnion
        /// ```
        /// use cryptocol::number::*;
        /// let mut succ = a_short.set_sbyte(1, 11);
        /// let mut sbyte = a_short.get_sbyte(1);
        /// if succ
        /// {
        ///     println!("a_short.get() = {}, a_short.get_sbyte(1).unwrap() = {}", a_short, sbyte.unwrap());
        ///     assert_eq!(sbyte.unwrap(), 11_i8);
        /// }
        /// else
        /// {
        ///     println!("Out of range");
        ///     assert_eq!(sbyte, None);
        /// }
        /// 
        /// succ = a_short.set_sbyte(2, 11);
        /// sbyte = a_short.get_sbyte(2);
        /// if succ
        /// {
        ///     println!("a_short.get() = {}, a_short.get_sbyte(2).unwrap() = {}", a_short, sbyte.unwrap());
        ///     assert_eq!(sbyte.unwrap(), 11_i8);
        /// }
        /// else
        /// {
        ///     println!("Out of range");
        ///     assert_eq!(sbyte, None);
        /// }
        /// ```
        /// 
        /// # Example of LongerUnion
        /// ```
        /// use cryptocol::number::*;
        /// let mut a_longer = LongerUnion::new();
        /// let mut succ = a_longer.set_sbyte(3, 81_i8);
        /// let mut sbyte = a_longer.get_sbyte(3);
        /// if succ
        /// {
        ///     println!("a_longer.get_signed() = {}, a_longer.get_sbyte(3).unwrap() = {}", a_longer.get_signed(), sbyte.unwrap());
        ///     assert_eq!(sbyte.unwrap(), 81_i8);
        /// }
        /// else
        /// {
        ///     println!("Out of range");
        ///     assert_eq!(sbyte, None);
        /// }
        /// 
        /// succ = a_longer.set_sbyte(16, 81_i8);
        /// sbyte = a_longer.get_sbyte(16);
        /// if succ
        /// {
        ///     println!("a_longer.get_signed() = {}, a_longer.get_sbyte(16).unwrap() = {}", a_longer.get_signed(), sbyte.unwrap());
        ///     assert_eq!(sbyte.unwrap(), 81_i8);
        /// }
        /// else
        /// {
        ///     println!("Out of range");
        ///     assert_eq!(sbyte, None);
        /// }
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        pub fn set_sbyte(&self, i: usize, val: i8) -> bool
        {
            if i <= Self::N
            { 
                unsafe { self.sbyte[Self::N-i] = val; }
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
        /// if `i` is less than a half of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a half of the size of this
        /// Union in bytes, it will panic.
        /// So, use this method only when you are sure that the argument `i` is
        /// less than a half of the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_ushort(). This method
        /// get_ushort_() is considered to be slightly faster than the method
        /// get_ushort().
        /// Use this method only when you are sure that `i` is less than a half
        /// of the size of this Union in bytes.
        /// Otherwise, use its counterpart method [get_ushort()](#method.get_ushort)
        /// for safety.
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
        #[cfg(target_endian = "little")]
        #[inline] pub fn get_ushort_(&self, i: usize) -> u16 { unsafe { self.ushort[i] } }

        /// Returns i-th element of array `ushort` of type `u16`
        /// if `i` is less than a half of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a half of the size of this
        /// Union in bytes, it will panic.
        /// So, use this method only when you are sure that the argument `i` is
        /// less than a half of the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_ushort(). This method
        /// get_ushort_() is considered to be slightly faster than the method
        /// get_ushort().
        /// Use this method only when you are sure that `i` is less than a half
        /// of the size of this Union in bytes.
        /// Otherwise, use its counterpart method [get_ushort()](#method.get_ushort)
        /// for safety.
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
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn get_ushort_(&self, i: usize) -> u16 { unsafe { self.ushort[Self::M-i] } }

        /// Returns i-th element of array `sshort` of type `i16`
        /// if `i` is less than a half of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a half of the size of this
        /// Union in bytes, it will panic.
        /// So, use this method only when you are sure that the argument `i` is
        /// less than a half of the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_sshort(). This method
        /// get_sshort_() is considered to be slightly faster than the method
        /// get_sshort().
        /// Use this method only when you are sure that `i` is less than a half
        /// of the size of this Union in bytes.
        /// Otherwise, use its counterpart method [get_sshort()](#method.get_sshort)
        /// for safety.
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
        #[cfg(target_endian = "little")]
        #[inline] pub fn get_sshort_(&self, i: usize) -> i16 { unsafe { self.sshort[i] } }

        /// Returns i-th element of array `sshort` of type `i16`
        /// if `i` is less than a half of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a half of the size of this
        /// Union in bytes, it will panic.
        /// So, use this method only when you are sure that the argument `i` is
        /// less than a half of the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_sshort(). This method
        /// get_sshort_() is considered to be slightly faster than the method
        /// get_sshort().
        /// Use this method only when you are sure that `i` is less than a half
        /// of the size of this Union in bytes.
        /// Otherwise, use its counterpart method [get_sshort()](#method.get_sshort)
        /// for safety.
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
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn get_sshort_(&self, i: usize) -> i16 { unsafe { self.sshort[Self::M-i] } }

        /// Returns i-th element of array `ushort` of type `u16` wrapped in Some
        /// of enum Option if `i` is less than a half of the size of this Union
        /// in bytes. Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than a half
        /// of the size of this Union in bytes. Otherwise, use its counterpart
        /// method [get_ushort_()](#method.get_ushort_) for performance.
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
        #[cfg(target_endian = "little")]
        pub fn get_ushort(&self, i: usize) -> Option<u16>
        {
            if i <= Self::M
                { unsafe { Some(self.ushort[i]) } }
            else
                { None }
        }

        /// Returns i-th element of array `ushort` of type `u16` wrapped in Some
        /// of enum Option if `i` is less than a half of the size of this Union
        /// in bytes. Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than a half
        /// of the size of this Union in bytes. Otherwise, use its counterpart
        /// method [get_ushort_()](#method.get_ushort_) for performance.
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
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        pub fn get_ushort(&self, i: usize) -> Option<u16>
        {
            if i <= Self::M
                { unsafe { Some(self.ushort[Self::M-i]) } }
            else
                { None }
        }

        /// Returns i-th element of array `sshort` of type `i16` wrapped in Some
        /// of enum Option if `i` is less than a half of the size of this Union
        /// in bytes. Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than a half
        /// of the size of this Union in bytes. Otherwise, use its counterpart
        /// method [get_sshort_()](#method.get_sshort_) for performance.
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
        #[cfg(target_endian = "little")]
        pub fn get_sshort(&self, i: usize) -> Option<i16>
        {
            if i <= Self::M
                { unsafe { Some(self.sshort[i]) } }
            else
                { None }
        }

        /// Returns i-th element of array `sshort` of type `i16` wrapped in Some
        /// of enum Option if `i` is less than a half of the size of this Union
        /// in bytes. Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than a half
        /// of the size of this Union in bytes. Otherwise, use its counterpart
        /// method [get_sshort_()](#method.get_sshort_) for performance.
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
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        pub fn get_sshort(&self, i: usize) -> Option<i16>
        {
            if i <= Self::M
                { unsafe { Some(self.sshort[Self::M-i]) } }
            else
                { None }
        }

        /// Sets i-th element of its array `ushort` of type `u16`
        /// if `i` is less than a half of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a half of the size of this
        /// Union in bytes, it will panic. So, use this method only when you
        /// are sure that the argument i is less than a half of the size of
        /// this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_ushort(). This method
        /// set_ushort_() is considered to be slightly faster than the method
        /// set_ushort().
        /// Use this method only when you are sure that `i` is less than a half
        /// of the size of this Union in bytes. Otherwise, use its counterpart
        /// method [set_ushort()](#method.set_ushort) for safety.
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
        #[cfg(target_endian = "little")]
        #[inline] pub fn set_ushort_(&mut self, i: usize, val: u16)  { unsafe { self.ushort[i] = val; } }

        /// Sets i-th element of its array `ushort` of type `u16`
        /// if `i` is less than a half of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a half of the size of this
        /// Union in bytes, it will panic. So, use this method only when you
        /// are sure that the argument i is less than a half of the size of
        /// this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_ushort(). This method
        /// set_ushort_() is considered to be slightly faster than the method
        /// set_ushort().
        /// Use this method only when you are sure that `i` is less than a half
        /// of the size of this Union in bytes. Otherwise, use its counterpart
        /// method [set_ushort()](#method.set_ushort) for safety.
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
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn set_ushort_(&mut self, i: usize, val: u16)  { unsafe { self.ushort[Self::M-i] = val; } }

        /// Sets i-th element of its array `sshort` of type `i16`
        /// if `i` is less than a half of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a half of the size of this
        /// Union in bytes, it will panic. So, use this method only when you
        /// are sure that the argument i is less than a half of the size of
        /// this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_sshort(). This method
        /// set_sshort_() is considered to be slightly faster than the method
        /// set_sshort().
        /// Use this method only when you are sure that `i` is less than a half
        /// of the size of this Union in bytes. Otherwise, use its counterpart
        /// method [set_sshort()](#method.set_sshort) for safety.
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
        #[cfg(target_endian = "little")]
        #[inline] pub fn set_sshort_(&mut self, i: usize, val: i16)  { unsafe { self.sshort[i] = val; } }

        /// Sets i-th element of its array `sshort` of type `i16`
        /// if `i` is less than a half of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a half of the size of this
        /// Union in bytes, it will panic. So, use this method only when you
        /// are sure that the argument i is less than a half of the size of
        /// this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_sshort(). This method
        /// set_sshort_() is considered to be slightly faster than the method
        /// set_sshort().
        /// Use this method only when you are sure that `i` is less than a half
        /// of the size of this Union in bytes. Otherwise, use its counterpart
        /// method [set_sshort()](#method.set_sshort) for safety.
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
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn set_sshort_(&mut self, i: usize, val: i16)  { unsafe { self.sshort[Self::M-i] = val; } }

        /// Sets i-th element of its array `ushort` of type `u16` and returns
        /// true if `i` is less than a half of the size of this Union in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than a half
        /// of the size of this Union in bytes. Otherwise, use its counterpart
        /// method [set_ushort_()](#method.set_ushort_) for performance.
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

        /// Sets i-th element of its array `ushort` of type `u16` and returns
        /// true if `i` is less than a half of the size of this Union in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than a half
        /// of the size of this Union in bytes. Otherwise, use its counterpart
        /// method [set_ushort_()](#method.set_ushort_) for performance.
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

        /// Sets i-th element of its array `sshort` of type `i16` and returns
        /// true if `i` is less than a half of the size of this Union in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than a half
        /// of the size of this Union in bytes. Otherwise, use its counterpart
        /// method [set_sshort_()](#method.set_sshort_) for performance.
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
        pub fn set_sshort(&mut self, i: usize, val: i16) -> bool
        {
            if i <= Self::M
            { 
                unsafe { self.sshort[i] = val; }
                true
            }
            else
            {
                false
            }
        }

        /// Sets i-th element of its array `sshort` of type `i16` and returns
        /// true if `i` is less than a half of the size of this Union in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than a half
        /// of the size of this Union in bytes. Otherwise, use its counterpart
        /// method [set_sshort_()](#method.set_sshort_) for performance.
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
        pub fn set_sshort(&self, i: usize, val: i16) -> bool
        {
            if i <= Self::M
            { 
                unsafe { self.sshort[Self::M-i] = val; }
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

        /// Returns i-th element of array `uint` of type `u32`
        /// if `i` is less than a quarter of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a quarter of the size of this
        /// Union in bytes, it will panic.
        /// So, use this method only when you are sure that the argument `i` is
        /// less than a quarter of the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_uint(). This method
        /// get_uint_() is considered to be slightly faster than the method
        /// get_uint().
        /// Use this method only when you are sure that `i` is less than
        /// a quarter of the size of this Union in bytes.
        /// Otherwise, use its counterpart method [get_uint()](#method.get_uint)
        /// for safety.
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
        #[cfg(target_endian = "little")]
        #[inline] pub fn get_uint_(&self, i: usize) -> u32 { unsafe { self.uint[i] } }

        /// Returns i-th element of array `uint` of type `u32`
        /// if `i` is less than a quarter of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a quarter of the size of this
        /// Union in bytes, it will panic.
        /// So, use this method only when you are sure that the argument `i` is
        /// less than a quarter of the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_uint(). This method
        /// get_uint_() is considered to be slightly faster than the method
        /// get_uint().
        /// Use this method only when you are sure that `i` is less than
        /// a quarter of the size of this Union in bytes.
        /// Otherwise, use its counterpart method [get_uint()](#method.get_uint)
        /// for safety.
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
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn get_uint_(&self, i: usize) -> u32 { unsafe { self.uint[Self::L-i] } }

        /// Returns i-th element of array `sint` of type `i32`
        /// if `i` is less than a quarter of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a quarter of the size of this
        /// Union in bytes, it will panic.
        /// So, use this method only when you are sure that the argument `i` is
        /// less than a quarter of the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_sint(). This method
        /// get_sint_() is considered to be slightly faster than the method
        /// get_sint().
        /// Use this method only when you are sure that `i` is less than
        /// a quarter of the size of this Union in bytes.
        /// Otherwise, use its counterpart method [get_sint()](#method.get_sint)
        /// for safety.
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
        #[cfg(target_endian = "little")]
        #[inline] pub fn get_sint_(&self, i: usize) -> i32 { unsafe { self.sint[i] } }

        /// Returns i-th element of array `sint` of type `i32`
        /// if `i` is less than a quarter of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a quarter of the size of this
        /// Union in bytes, it will panic.
        /// So, use this method only when you are sure that the argument `i` is
        /// less than a quarter of the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_sint(). This method
        /// get_sint_() is considered to be slightly faster than the method
        /// get_sint().
        /// Use this method only when you are sure that `i` is less than
        /// a quarter of the size of this Union in bytes.
        /// Otherwise, use its counterpart method [get_sint()](#method.get_sint)
        /// for safety.
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
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn get_sint_(&self, i: usize) -> i32 { unsafe { self.sint[Self::L-i] } }

        /// Returns i-th element of array `uint` of type `u32` wrapped in Some
        /// of enum Option if `i` is less than a quarter of the size of this
        /// Union in bytes. Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// a quarter of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [get_uint_()](#method.get_uint_) for performance.
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
        #[cfg(target_endian = "little")]
        pub fn get_uint(&self, i: usize) -> Option<u32>
        {
            if i <= Self::L
                { unsafe { Some(self.uint[i]) } }
            else
                { None }
        }

        /// Returns i-th element of array `uint` of type `u32` wrapped in Some
        /// of enum Option if `i` is less than a quarter of the size of this
        /// Union in bytes. Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// a quarter of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [get_uint_()](#method.get_uint_) for performance.
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
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        pub fn get_uint(&self, i: usize) -> Option<u32>
        {
            if i <= Self::L
                { unsafe { Some(self.uint[Self::L-i]) } }
            else
                { None }
        }

        /// Returns i-th element of array `sint` of type `i32` wrapped in Some
        /// of enum Option if `i` is less than a quarter of the size of this
        /// Union in bytes. Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// a quarter of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [get_sint_()](#method.get_sint_) for performance.
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
        #[cfg(target_endian = "little")]
        pub fn get_sint(&self, i: usize) -> Option<i32>
        {
            if i <= Self::L
                { unsafe { Some(self.sint[i]) } }
            else
                { None }
        }

        /// Returns i-th element of array `sint` of type `i32` wrapped in Some
        /// of enum Option if `i` is less than a quarter of the size of this
        /// Union in bytes. Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// a quarter of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [get_sint_()](#method.get_sint_) for performance.
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
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        pub fn get_sint(&self, i: usize) -> Option<i32>
        {
            if i <= Self::L
                { unsafe { Some(self.sint[Self::L-i]) } }
            else
                { None }
        }

        /// Sets i-th element of its array `uint` of type `u32`
        /// if `i` is less than a quarter of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a quarter of the size of this
        /// Union in bytes, it will panic. So, use this method only when you
        /// are sure that the argument i is less than a quarter of the size of
        /// this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_uint(). This method
        /// set_uint_() is considered to be slightly faster than the method
        /// set_uint().
        /// Use this method only when you are sure that `i` is less than a quarter
        /// of the size of this Union in bytes. Otherwise, use its counterpart
        /// method [set_uint()](#method.set_uint) for safety.
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
        #[cfg(target_endian = "little")]
        #[inline] pub fn set_uint_(&mut self, i: usize, val: u32)  { unsafe { self.uint[i] = val; } }

        /// Sets i-th element of its array `uint` of type `u32`
        /// if `i` is less than a quarter of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a quarter of the size of this
        /// Union in bytes, it will panic. So, use this method only when you
        /// are sure that the argument i is less than a quarter of the size of
        /// this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_uint(). This method
        /// set_uint_() is considered to be slightly faster than the method
        /// set_uint().
        /// Use this method only when you are sure that `i` is less than
        /// a quarter of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [set_uint()](#method.set_uint) for safety.
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
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn set_uint_(&mut self, i: usize, val: u32)  { unsafe { self.uint[Self::L-i] = val; } }

        /// Sets i-th element of its array `sint` of type `i32`
        /// if `i` is less than a quarter of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a quarter of the size of this
        /// Union in bytes, it will panic. So, use this method only when you
        /// are sure that the argument i is less than a quarter of the size of
        /// this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_sint(). This method
        /// set_sint_() is considered to be slightly faster than the method
        /// set_sint().
        /// Use this method only when you are sure that `i` is less than
        /// a quarter of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [set_sint()](#method.set_sint) for safety.
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
        #[cfg(target_endian = "little")]
        #[inline] pub fn set_sint_(&mut self, i: usize, val: i32)  { unsafe { self.sint[i] = val; } }

        /// Sets i-th element of its array `sint` of type `i32`
        /// if `i` is less than a quarter of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to a quarter of the size of this
        /// Union in bytes, it will panic. So, use this method only when you
        /// are sure that the argument i is less than a quarter of the size of
        /// this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_sint(). This method
        /// set_sint_() is considered to be slightly faster than the method
        /// set_sint().
        /// Use this method only when you are sure that `i` is less than
        /// a quarter of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [set_sint()](#method.set_sint) for safety.
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
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn set_sint_(&mut self, i: usize, val: i32)  { unsafe { self.sint[Self::L-i] = val; } }

        /// Sets i-th element of its array `uint` of type `u32` and returns
        /// true if `i` is less than a quarter of the size of this Union in
        /// bytes. Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// a quarter of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [set_uint_()](#method.set_uint_) for performance.
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

        /// Sets i-th element of its array `uint` of type `u32` and returns
        /// true if `i` is less than a quarter of the size of this Union in
        /// bytes. Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// a quarter of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [set_uint_()](#method.set_uint_) for performance.
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

        /// Sets i-th element of its array `sint` of type `i32` and returns
        /// true if `i` is less than a quarter of the size of this Union in
        /// bytes. Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// a quarter of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [set_sint_()](#method.set_sint_) for performance.
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
        pub fn set_sint(&mut self, i: usize, val: i32) -> bool
        {
            if i <= Self::L
            { 
                unsafe { self.sint[i] = val; }
                true
            }
            else
            {
                false
            }
        }

        /// Sets i-th element of its array `sint` of type `i32` and returns
        /// true if `i` is less than a quarter of the size of this Union in
        /// bytes. Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// a quarter of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [set_sint_()](#method.set_sint_) for performance.
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
        pub fn set_sint(&self, i: usize, val: i32) -> bool
        {
            if i <= Self::L
            { 
                unsafe { self.sint[Self::L-i] = val; }
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

        /// Returns i-th element of array `ulong` of type `u64`
        /// if `i` is less than an eighth of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to an eighth of the size of this
        /// Union in bytes, it will panic.
        /// So, use this method only when you are sure that the argument `i` is
        /// less than an eighth of the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_ulong(). This method
        /// get_ulong_() is considered to be slightly faster than the method
        /// get_ulong().
        /// Use this method only when you are sure that `i` is less than
        /// an eighth of the size of this Union in bytes.
        /// Otherwise, use its counterpart method [get_ulong()](#method.get_ulong)
        /// for safety.
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
        #[cfg(target_endian = "little")]
        #[inline] pub fn get_ulong_(&self, i: usize) -> u64 { unsafe { self.ulong[i] } }

        /// Returns i-th element of array `ulong` of type `u64`
        /// if `i` is less than an eighth of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to an eighth of the size of this
        /// Union in bytes, it will panic.
        /// So, use this method only when you are sure that the argument `i` is
        /// less than an eighth of the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_ulong(). This method
        /// get_ulong_() is considered to be slightly faster than the method
        /// get_ulong().
        /// Use this method only when you are sure that `i` is less than
        /// an eighth of the size of this Union in bytes.
        /// Otherwise, use its counterpart method [get_ulong()](#method.get_ulong)
        /// for safety.
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
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn get_ulong_(&self, i: usize) -> u64 { unsafe { self.ulong[Self::K-i] } }

        /// Returns i-th element of array `slong` of type `i64`
        /// if `i` is less than an eighth of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to an eighth of the size of this
        /// Union in bytes, it will panic.
        /// So, use this method only when you are sure that the argument `i` is
        /// less than an eighth of the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_slong(). This method
        /// get_slong_() is considered to be slightly faster than the method
        /// get_slong().
        /// Use this method only when you are sure that `i` is less than
        /// an eighth of the size of this Union in bytes.
        /// Otherwise, use its counterpart method [get_slong()](#method.get_slong)
        /// for safety.
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
        #[cfg(target_endian = "little")]
        #[inline] pub fn get_slong_(&self, i: usize) -> i64 { unsafe { self.slong[i] } }

        /// Returns i-th element of array `slong` of type `i64`
        /// if `i` is less than an eighth of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to an eighth of the size of this
        /// Union in bytes, it will panic.
        /// So, use this method only when you are sure that the argument `i` is
        /// less than an eighth of the size of this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_slong(). This method
        /// get_slong_() is considered to be slightly faster than the method
        /// get_slong().
        /// Use this method only when you are sure that `i` is less than
        /// an eighth of the size of this Union in bytes.
        /// Otherwise, use its counterpart method [get_slong()](#method.get_slong)
        /// for safety.
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
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn get_slong_(&self, i: usize) -> i64 { unsafe { self.slong[Self::K-i] } }

        /// Returns i-th element of array `ulong` of type `u64` wrapped in Some
        /// of enum Option if `i` is less than an eighth of the size of this
        /// Union in bytes. Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// an eighth of the size of this Union in bytes. Otherwise, l;you can
        /// use its counterpart method [get_ulong_()](#method.get_ulong_)
        /// for performance.
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
        #[cfg(target_endian = "little")]
        pub fn get_ulong(&self, i: usize) -> Option<u64>
        {
            if i <= Self::K
                { unsafe { Some(self.ulong[i]) } }
            else
                { None }
        }

        /// Returns i-th element of array `ulong` of type `u64` wrapped in Some
        /// of enum Option if `i` is less than an eighth of the size of this
        /// Union in bytes. Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// an eighth of the size of this Union in bytes. Otherwise, l;you can
        /// use its counterpart method [get_ulong_()](#method.get_ulong_)
        /// for performance.
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
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn get_ulong(&self, i: usize) -> Option<u64>
        {
            if i <= Self::K
                { unsafe { Some(self.ulong[Self::K-i]) } }
            else
                { None }
        }

        /// Returns i-th element of array `slong` of type `i64` wrapped in Some
        /// of enum Option if `i` is less than an eighth of the size of this
        /// Union in bytes. Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// an eighth of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [get_slong_()](#method.get_slong_) for performance.
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
        #[cfg(target_endian = "little")]
        pub fn get_slong(&self, i: usize) -> Option<i64>
        {
            if i <= Self::K
                { unsafe { Some(self.slong[i]) } }
            else
                { None }
        }

        /// Returns i-th element of array `slong` of type `i64` wrapped in Some
        /// of enum Option if `i` is less than an eighth of the size of this
        /// Union in bytes. Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// an eighth of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [get_slong_()](#method.get_slong_) for performance.
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
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn get_slong(&self, i: usize) -> Option<i64>
        {
            if i <= Self::K
                { unsafe { Some(self.slong[Self::K-i]) } }
            else
                { None }
        }

        /// Sets i-th element of its array `ulong` of type `u64`
        /// if `i` is less than an eighth of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to an eighth of the size of this
        /// Union in bytes, it will panic. So, use this method only when you
        /// are sure that the argument i is less than an eighth of the size of
        /// this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_ulong(). This method
        /// set_ulong_() is considered to be slightly faster than the method
        /// set_ulong().
        /// Use this method only when you are sure that `i` is less than an eighth
        /// of the size of this Union in bytes. Otherwise, use its counterpart
        /// method [set_ulong()](#method.set_ulong) for safety.
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
        #[cfg(target_endian = "little")]
        #[inline] pub fn set_ulong_(&mut self, i: usize, val: u64)  { unsafe { self.ulong[i] = val; } }

        /// Sets i-th element of its array `ulong` of type `u64`
        /// if `i` is less than an eighth of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to an eighth of the size of this
        /// Union in bytes, it will panic. So, use this method only when you
        /// are sure that the argument i is less than an eighth of the size of
        /// this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_ulong(). This method
        /// set_ulong_() is considered to be slightly faster than the method
        /// set_ulong().
        /// Use this method only when you are sure that `i` is less than an eighth
        /// of the size of this Union in bytes. Otherwise, use its counterpart
        /// method [set_ulong()](#method.set_ulong) for safety.
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
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn set_ulong_(&mut self, i: usize, val: u64)  { unsafe { self.ulong[Self::K-i] = val; } }

        /// Sets i-th element of its array `slong` of type `i64`
        /// if `i` is less than an eighth of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to an eighth of the size of this
        /// Union in bytes, it will panic. So, use this method only when you
        /// are sure that the argument i is less than an eighth of the size of
        /// this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_slong(). This method
        /// set_slong_() is considered to be slightly faster than the method
        /// set_slong().
        /// Use this method only only when you are sure that `i` is less than
        /// an eighth of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [set_slong()](#method.set_slong) for safety.
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
        #[cfg(target_endian = "little")]
        #[inline] pub fn set_slong_(&mut self, i: usize, val: i64)  { unsafe { self.slong[i] = val; } }

        /// Sets i-th element of its array `slong` of type `i64`
        /// if `i` is less than an eighth of the size of this Union in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to an eighth of the size of this
        /// Union in bytes, it will panic. So, use this method only when you
        /// are sure that the argument i is less than an eighth of the size of
        /// this Union
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_slong(). This method
        /// set_slong_() is considered to be slightly faster than the method
        /// set_slong().
        /// Use this method only only when you are sure that `i` is less than
        /// an eighth of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [set_slong()](#method.set_slong) for safety.
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
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn set_slong_(&mut self, i: usize, val: i64)  { unsafe { self.slong[Self::K-i] = val; } }

        /// Sets i-th element of its array `ulong` of type `u64` and returns
        /// true if `i` is less than an eighth of the size of this Union in
        /// bytes. Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// an eighth of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [set_ulong_()](#method.set_ulong_) for performance.
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

        /// Sets i-th element of its array `ulong` of type `u64` and returns
        /// true if `i` is less than an eighth of the size of this Union in
        /// bytes. Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// an eighth of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [set_ulong_()](#method.set_ulong_) for performance.
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

        /// Sets i-th element of its array `slong` of type `i64` and returns
        /// true if `i` is less than an eighth of the size of this Union in
        /// bytes. Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// an eighth of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [set_slong_()](#method.set_slong_) for performance.
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
        pub fn set_slong(&mut self, i: usize, val: i64) -> bool
        {
            if i <= Self::L
            { 
                unsafe { self.slong[i] = val; }
                true
            }
            else
            {
                false
            }
        }

        /// Sets i-th element of its array `slong` of type `i64` and returns
        /// true if `i` is less than an eighth of the size of this Union in
        /// bytes. Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// an eighth of the size of this Union in bytes. Otherwise, use its
        /// counterpart method [set_slong_()](#method.set_slong_) for performance.
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
        pub fn set_ulong(&self, i: usize, val: i64) -> bool
        {
            if i <= Self::K
            { 
                unsafe { self.slong[Self::K-i] = val; }
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

        /// Returns i-th element of array `u_size` of type `usize`
        /// if `i` is less than the size of this Union in bytes divided by
        /// the size of the type `usize` in bytes. Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to the size of this Union
        /// in bytes divided by the size of the type `usize` in bytes,
        /// it will panic.
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_usize(). This method
        /// get_usize_() is considered to be slightly faster than the method
        /// get_usize().
        /// Use this method only when you are sure that `i` is less than
        /// the size of this Union in bytes divided by the size of the type
        /// `usize` in bytes. Otherwise, use its counterpart method
        /// [get_usize()](#method.get_usize) for safety.
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
        #[cfg(target_endian = "little")]
        #[inline] pub fn get_usize_(&self, i: usize) -> usize { unsafe { self.u_size[i] } }

        /// Returns i-th element of array `u_size` of type `usize`
        /// if `i` is less than the size of this Union in bytes divided by
        /// the size of the type `usize` in bytes. Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to the size of this Union
        /// in bytes divided by the size of the type `usize` in bytes,
        /// it will panic.
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_usize(). This method
        /// get_usize_() is considered to be slightly faster than the method
        /// get_usize().
        /// Use this method only when you are sure that `i` is less than
        /// the size of this Union in bytes divided by the size of the type
        /// `usize` in bytes. Otherwise, use its counterpart method
        /// [get_usize()](#method.get_usize) for safety.
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
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn get_usize_(&self, i: usize) -> usize { unsafe { self.u_size[Self::J-i] } }

        /// Returns i-th element of array `s_size` of type `isize`
        /// if `i` is less than the size of this Union in bytes divided by
        /// the size of the type `isize` in bytes. Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to the size of this Union
        /// in bytes divided by the size of the type `isize` in bytes,
        /// it will panic.
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_ssize(). This method
        /// get_ssize_() is considered to be slightly faster than the method
        /// get_ssize().
        /// Use this method only when you are sure that `i` is less than
        /// the size of this Union in bytes divided by the size of the type
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
        #[cfg(target_endian = "little")]
        #[inline] pub fn get_ssize_(&self, i: usize) -> isize { unsafe { self.s_size[i] } }

        /// Returns i-th element of array `s_size` of type `isize`
        /// if `i` is less than the size of this Union in bytes divided by
        /// the size of the type `isize` in bytes. Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Panics
        /// So, if `i` is greater than or equal to the size of this Union
        /// in bytes divided by the size of the type `isize` in bytes,
        /// it will panic.
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method get_ssize(). This method
        /// get_ssize_() is considered to be slightly faster than the method
        /// get_ssize().
        /// Use this method only when you are sure that `i` is less than
        /// the size of this Union in bytes divided by the size of the type
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
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn get_ssize_(&self, i: usize) -> isize { unsafe { self.s_size[Self::J-i] } }

        /// Returns i-th element of array `u_size` of type `usize` wrapped
        /// in Some of enum Option if `i` is less than the size of this Union
        /// in bytes divided by the size of the type `usize` in bytes.
        /// Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// the size of this Union in bytes divided by the size of the type
        /// `usize` in bytes. Otherwise, use its counterpart method
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
        #[cfg(target_endian = "little")]
        pub fn get_usize(&self, i: usize) -> Option<usize>
        {
            if i <= Self::J
                { unsafe { Some(self.u_size[i]) } }
            else
                { None }
        }

        /// Returns i-th element of array `u_size` of type `usize` wrapped
        /// in Some of enum Option if `i` is less than the size of this Union
        /// in bytes divided by the size of the type `usize` in bytes.
        /// Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// the size of this Union in bytes divided by the size of the type
        /// `usize` in bytes. Otherwise, use its counterpart method
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
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        pub fn get_usize(&self, i: usize) -> Option<usize>
        {
            if i <= Self::J
                { unsafe { Some(self.u_size[Self::J-i]) } }
            else
                { None }
        }

        /// Returns i-th element of array `s_size` of type `isize` wrapped
        /// in Some of enum Option if `i` is less than the size of this Union
        /// in bytes divided by the size of the type `isize` in bytes.
        /// Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// the size of this Union in bytes divided by the size of the type
        /// `isize` in bytes. Otherwise, use its counterpart method
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
        #[cfg(target_endian = "little")]
        pub fn get_ssize(&self, i: usize) -> Option<isize>
        {
            if i <= Self::J
                { unsafe { Some(self.s_size[i]) } }
            else
                { None }
        }

        /// Returns i-th element of array `s_size` of type `isize` wrapped
        /// in Some of enum Option if `i` is less than the size of this Union
        /// in bytes divided by the size of the type `isize` in bytes.
        /// Otherwise, it returns None of enum Option.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// the size of this Union in bytes divided by the size of the type
        /// `isize` in bytes. Otherwise, use its counterpart method
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
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        pub fn get_ssize(&self, i: usize) -> Option<isize>
        {
            if i <= Self::J
                { unsafe { Some(self.s_size[Self::J-i]) } }
            else
                { None }
        }

        /// Sets i-th element of its array `u_size` of type `usize`
        /// if `i` is less than the size of this Union in bytes divided
        /// by the size of the type `usize` in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_usize(). This method
        /// set_usize_() is considered to be slightly faster than the method
        /// set_usize().
        /// Use this method only when you are sure that `i` is less than
        /// the size of this Union in bytes divided by the size of the type
        /// `usize` in bytes. Otherwise, use its counterpart
        /// method [set_ssize()](#method.set_ssize) for safety.
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
        #[inline] pub fn set_usize_(&mut self, i: usize, val: usize)  { unsafe { self.u_size[i] = val; } }

        /// Sets i-th element of its array `u_size` of type `usize`
        /// if `i` is less than the size of this Union in bytes divided
        /// by the size of the type `usize` in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_usize(). This method
        /// set_usize_() is considered to be slightly faster than the method
        /// set_usize().
        /// Use this method only when you are sure that `i` is less than
        /// the size of this Union in bytes divided by the size of the type
        /// `usize` in bytes. Otherwise, use its counterpart
        /// method [set_ssize()](#method.set_ssize) for safety.
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
        #[inline] pub fn set_usize_(&mut self, i: usize, val: usize)  { unsafe { self.u_size[Self::J-i] = val; } }

        /// Sets i-th element of its array `s_size` of type `isize`
        /// if `i` is less than the size of this Union in bytes divided
        /// by the size of the type `isize` in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_ssize(). This method
        /// set_ssize_() is considered to be slightly faster than the method
        /// set_ssize().
        /// Use this method only when you are sure that `i` is less than
        /// the size of this Union in bytes divided by the size of the type
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
        #[cfg(target_endian = "little")]
        #[inline] pub fn set_ssize_(&mut self, i: usize, val: isize)  { unsafe { self.s_size[i] = val; } }

        /// Sets i-th element of its array `s_size` of type `isize`
        /// if `i` is less than the size of this Union in bytes divided
        /// by the size of the type `isize` in bytes.
        /// Otherwise, it will panic.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// It is performance-oriented and does not care for safety.
        /// It is virtually the same as the method set_ssize(). This method
        /// set_ssize_() is considered to be slightly faster than the method
        /// set_ssize().
        /// Use this method only when you are sure that `i` is less than
        /// the size of this Union in bytes divided by the size of the type
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
        /// encouraged to use it for serious purpose. Only use this crate for
        /// Big-endian CPUs with your own full responsibility.
        #[cfg(target_endian = "big")]
        #[inline] pub fn set_ssize_(&mut self, i: usize, val: isize)  { unsafe { self.s_size[Self::J-i] = val; } }

        /// Sets i-th element of its array `u_size` of type `usize` and returns
        /// true if `i` is less than the size of this Union in bytes divided
        /// by the size of the type `usize` in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// the size of this Union in bytes divided by the size of the type
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
        /// true if `i` is less than the size of this Union in bytes divided
        /// by the size of the type `usize` in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// the size of this Union in bytes divided by the size of the type
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
        /// true if `i` is less than the size of this Union in bytes divided
        /// by the size of the type `isize` in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// the size of this Union in bytes divided by the size of the type
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
        /// true if `i` is less than the size of this Union in bytes divided
        /// by the size of the type `isize` in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Argument i
        /// 0-th element contains LSB (Least Significant Bit), while (the size
        /// of this Union in bytes - 1)-th element contains MSB (Most
        /// Significant Bit) regardless endianness.
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// the size of this Union in bytes divided by the size of the type
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
