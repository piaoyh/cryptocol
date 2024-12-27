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
// #![warn(rustdoc::missing_doc_code_examples)]
#![allow(missing_docs)]
#![allow(rustdoc::missing_doc_code_examples)]

#[allow(unused_macros)]
macro_rules! new_with_small_uint {
    () => {
        // pub fn new_with_ubyte(ubyte: [u8; 2]) -> Self
        /// Constructs a new `SizeUnion` with initializing it with `ubyte`.
        /// 
        /// # Output
        /// A new object of `SizeUnion` initialized with the value `ubyte`.
        /// 
        /// # Initialization
        /// The field of the constructed object will be initialized with `ubyte`.
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// let a_sizeunion = SizeUnion::new_with_ubyte([172_u8, 216_u8]);
        /// println!("a_sizeunion = {}", a_sizeunion.get());
        /// assert_eq!(a_sizeunion.get(), 55468_u16);
        /// ```
        #[cfg(target_pointer_width = "16")]
        #[inline] pub fn new_with_ubytes(ubyte: [u8; 2]) -> Self    { Self { ubyte } }

        // pub fn new_with_ubytes(ubyte: [u8; 4]) -> Self
        /// Constructs a new `SizeUnion` with initializing it with `ubyte`.
        /// 
        /// # Output
        /// A new object of `SizeUnion` initialized with the value `ubyte`.
        /// 
        /// # Initialization
        /// The field of the constructed object will be initialized with `ubyte`.
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// let a_sizeunion = SizeUnion::new_with_ubytes([222_u8, 0_u8, 230_u8, 228_u8]);
        /// println!("a_sizeunion = {}", a_sizeunion.get());
        /// assert_eq!(a_sizeunion.get(), 3840278750_usize);
        /// ```
        #[cfg(target_pointer_width = "32")] 
        #[inline] pub fn new_with_ubytes(ubyte: [u8; 4]) -> Self    { Self { ubyte } }

        // pub fn new_with_ubytes(ubyte: [u8; 8]) -> Self
        /// Constructs a new `SizeUnion` with initializing it with `ubyte`.
        /// 
        /// # Output
        /// A new object of `SizeUnion` initialized with the value `ubyte`.
        /// 
        /// # Initialization
        /// The field of the constructed object will be initialized with `ubyte`.
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// let a_sizeunion = SizeUnion::new_with_ubytes([131_u8, 21_u8, 104_u8, 195_u8, 42_u8, 157_u8, 251_u8, 255_u8]);
        /// println!("a_sizeunion = {}", a_sizeunion.get());
        /// assert_eq!(a_sizeunion.get(), 18445509505818563971_usize);
        /// ```
        #[cfg(target_pointer_width = "64")] 
        #[inline] pub fn new_with_ubytes(ubyte: [u8; 8]) -> Self    { Self { ubyte } }

        // // pub fn new_with_ubytes(ubyte: [u8; 16]) -> Self
        // /// Constructs a new `SizeUnion` with initializing it with `ubyte`.
        // /// 
        // /// # Output
        // /// A new object of `SizeUnion` initialized with the value `ubyte`.
        // /// 
        // /// # Initialization
        // /// The field of the constructed object will be initialized with `ubyte`.
        // /// 
        // /// Example
        // /// ```
        // /// use cryptocol::number::SizeUnion;
        // /// let a_sizeunion = SizeUnion::new_with_ubytes([79_u8, 11_u8, 74_u8, 241_u8, 245_u8, 104_u8, 163_u8, 189_u8, 88_u8, 136_u8, 206_u8, 126_u8, 26_u8, 59_u8, 18_u8, 255_u8]);
        // /// println!("a_sizeunion = {}", a_sizeunion.get());
        // /// assert_eq!(a_sizeunion.get(), 339047799029950809142362261752780557135_usize);
        // /// ```
        // #[cfg(target_pointer_width = "128")] 
        // #[inline] pub fn new_with_ubytes(ubyte: [u8; 16]) -> Self    { Self { ubyte } }

        // pub fn new_with_ushorts(ushort: [u16; 2]) -> Self
        /// Constructs a new `SizeUnion` with initializing it with `ushort`.
        /// 
        /// # Output
        /// A new object of `SizeUnion` initialized with the value `ushort`.
        /// 
        /// # Initialization
        /// The field of the constructed object will be initialized with `ushort`.
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// let a_sizeunion = SizeUnion::new_with_ushorts([222_u16, 58598_u16]);
        /// println!("a_sizeunion = {}", a_sizeunion.get());
        /// assert_eq!(a_sizeunion.get(), 3840278750_usize);
        /// ```
        #[cfg(target_pointer_width = "32")]
        #[inline] pub fn new_with_ushorts(ushort: [u16; 2]) -> Self  { Self { ushort } }

        // pub fn new_with_ushorts(ushort: [u16; 4]) -> Self
        /// Constructs a new `SizeUnion` with initializing it with `ushort`.
        /// 
        /// # Output
        /// A new object of `SizeUnion` initialized with the value `ushort`.
        /// 
        /// # Initialization
        /// The field of the constructed object will be initialized with `ushort`.
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// let a_sizeunion = SizeUnion::new_with_ushorts([5507_u16, 50024_u16, 40234_u16, 65531_u16]);
        /// println!("a_sizeunion = {}", a_sizeunion.get());
        /// assert_eq!(a_sizeunion.get(), 18445509505818563971_usize);
        /// ```
        #[cfg(target_pointer_width = "64")] 
        #[inline] pub fn new_with_ushorts(ushort: [u16; 4]) -> Self  { Self { ushort } }

        // // pub fn new_with_ushorts(ushort: [u16; 8]) -> Self
        // /// Constructs a new `SizeUnion` with initializing it with `ushort`.
        // /// 
        // /// # Output
        // /// A new object of `SizeUnion` initialized with the value `ushort`.
        // /// 
        // /// # Initialization
        // /// The field of the constructed object will be initialized with `ushort`.
        // /// 
        // /// Example
        // /// ```
        // /// use cryptocol::number::SizeUnion;
        // /// let a_sizeunion = SizeUnion::new_with_ushorts([2895_u16, 61770_u16, 26869_u16, 48547_u16, 34904_u16, 5507_u16, 50024_u16, 40234_u16, 65531_u16]);
        // /// println!("a_sizeunion = {}", a_sizeunion.get());
        // /// assert_eq!(a_sizeunion.get(), 339047799029950809142362261752780557135_usize);
        // /// ```
        // #[cfg(target_pointer_width = "128")] 
        // #[inline] pub fn new_with_ushorts(ushort: [u16; 8]) -> Self  { Self { ushort } }

        // pub fn new_with_uints(uint: [u32; 2]) -> Self
        /// Constructs a new `SizeUnion` with initializing it with `uint`.
        /// 
        /// # Output
        /// A new object of `SizeUnion` initialized with the value `uint`.
        /// 
        /// # Initialization
        /// The field of the constructed object will be initialized with `uint`.
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// let a_sizeunion = SizeUnion::new_with_uints([3278378371_u32, 4294679850_u32]);
        /// println!("a_sizeunion = {}", a_sizeunion.get());
        /// assert_eq!(a_sizeunion.get(), 18445509505818563971_usize);
        /// ```
        #[cfg(target_pointer_width = "64")] 
        #[inline] pub fn new_with_uints(uint: [u32; 2]) -> Self     { Self { uint } }

        // // pub fn new_with_uints(uint: [u32; 4]) -> Self
        // /// Constructs a new `SizeUnion` with initializing it with `uint`.
        // /// 
        // /// # Output
        // /// A new object of `SizeUnion` initialized with the value `uint`.
        // /// 
        // /// # Initialization
        // /// The field of the constructed object will be initialized with `uint`.
        // /// 
        // /// Example
        // /// ```
        // /// use cryptocol::number::SizeUnion;
        // /// let a_sizeunion = SizeUnion::new_with_uints([4048161615_u32, 3181603061_u32, 2127464536_u32, 4279384858_u32]);
        // /// println!("a_sizeunion = {}", a_sizeunion.get());
        // /// assert_eq!(a_sizeunion.get(), 339047799029950809142362261752780557135_usize);
        // /// ```
        // #[cfg(target_pointer_width = "128")] 
        // #[inline] pub fn new_with_uints(uint: [u32; 4]) -> Self     { Self { uint } }

        // // pub fn new_with_ulongs(ulong: [u64; 4]) -> Self
        // /// Constructs a new `SizeUnion` with initializing it with `ulong`.
        // /// 
        // /// # Output
        // /// A new object of `SizeUnion` initialized with the value `ulong`.
        // /// 
        // /// # Initialization
        // /// The field of the constructed object will be initialized with `ulong`.
        // /// 
        // /// Example
        // /// ```
        // /// use cryptocol::number::SizeUnion;
        // /// let a_sizeunion = SizeUnion::new_with_ulongs([13664881099896654671_u64, 18379818014235068504_u64]);
        // /// println!("a_sizeunion = {}", a_sizeunion.get());
        // /// assert_eq!(a_sizeunion.get(), 339047799029950809142362261752780557135_usize);
        // /// ```
        // #[cfg(target_pointer_width = "128")] 
        // #[inline] pub fn new_with_ulongs(ulong: [u64; 4]) -> Self   { Self { ulong } }
    }
}
#[allow(unused_imports)] pub(super) use new_with_small_uint;

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
        /// let a_sizeunion = SizeUnion::new_with(250_usize);
        /// println!("a_sizeunion = {}", a_sizeunion.get_ubyte());
        /// assert_eq!(a_sizeunion.get_ubyte(), 250_usize);
        /// ```
        #[inline] pub fn get_ubyte(self) -> u8  { unsafe { self.ubyte } }

        // pub fn set_ubyte(&mut self, val: u8)
        /// Sets its value with `val` of type `u8`
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::SizeUnion;    
        /// let mut a_sizeunion = SizeUnion::new();
        /// a_sizeunion.set_ubyte(234_u8);
        /// println!("a_sizeunion = {}", a_sizeunion.get_ubyte());
        /// assert_eq!(a_sizeunion.get_ubyte(), 234_u8);
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
        /// let a_sizeunion = SizeUnion::new_with_signed(-123_i8);
        /// println!("a_sizeunion = {}", a_sizeunion.get_sbyte());
        /// assert_eq!(a_sizeunion.get_sbyte(), -123_i8);
        /// ```
        #[inline] pub fn get_sbyte(self) -> i8      { unsafe { self.sbyte } }

        // pub fn set_sbyte(&mut self, val: i8)
        /// Sets its value with `val` of type `isize`
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::SizeUnion;    
        /// let mut a_sizeunion = SizeUnion::new();
        /// a_sizeunion.set_sbyte(-123_isize);
        /// println!("a_sizeunion = {}", a_sizeunion.get_sbyte());
        /// assert_eq!(a_sizeunion.get_sbyte(), -123_isize);
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
        /// # Example 6 for SizeUnion for 32-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "32")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let a_sizeunion = SizeUnion::new_with(4048161615_usize);
        ///     for i in 0..4
        ///         { println!("a_sizeunion.get_ubyte_({}) = {}", i, a_sizeunion.get_ubyte_(i)); }
        ///     assert_eq!(a_sizeunion.get_ubyte_(0), 79_u8);
        ///     assert_eq!(a_sizeunion.get_ubyte_(1), 11_u8);
        ///     assert_eq!(a_sizeunion.get_ubyte_(2), 74_u8);
        ///     assert_eq!(a_sizeunion.get_ubyte_(3), 241_u8);
        /// 
        ///     // It will panic.
        ///     // println!("a_intunion.get_ubyte_(4) = {}", a_intunion.get_ubyte_(4));
        /// }
        /// ```
        /// 
        /// # Example 7 for SizeUnion for 16-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "16")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let a_sizeunion = SizeUnion::new_with(2895_usize);
        ///     for i in 0..2
        ///         { println!("a_sizeunion.get_ubyte_({}) = {}", i, a_sizeunion.get_ubyte_(i)); }
        ///     assert_eq!(a_sizeunion.get_ubyte_(0), 79_u8);
        ///     assert_eq!(a_sizeunion.get_ubyte_(1), 11_u8);
        /// 
        ///     // It will panic.
        ///     // println!("a_sizeunion.get_ubyte_(2) = {}", a_sizeunion.get_ubyte_(2));
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
        /// 
        /// // It will panic.
        /// // a_shortunion.set_ubyte_(2, 100_u8);
        /// 
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
        /// 
        /// // It will panic.
        /// // a_intunion.set_ubyte_(4, 100_u8);
        /// 
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
        /// 
        /// // It will panic.
        /// // a_longunion.set_ubyte_(8, 100_u8);
        /// 
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
        /// 
        /// // It will panic.
        /// // a_longerunion.set_ubyte_(16, 100_u8);
        /// 
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
        /// 
        ///     // It will panic.
        ///     // a_sizeunion.set_ubyte_(8, 100_u8);
        /// 
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
        /// # Example 6 for SizeUnion for 32-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "32")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let mut a_sizeunion = SizeUnion::new();
        ///     a_sizeunion.set_ubyte_(0, 79_u8);
        ///     a_sizeunion.set_ubyte_(1, 11_u8);
        ///     a_sizeunion.set_ubyte_(2, 74_u8);
        ///     a_sizeunion.set_ubyte_(3, 241_u8);
        ///     
        ///     // It will panic.
        ///     // a_sizeunion.set_ubyte_(4, 100_u8);
        ///     
        ///     println!("a_intunion.get() = {}", a_sizeunion.get());
        ///     for i in 0..4
        ///         { println!("a_sizeunion.get_ubyte_({}) = {}", i, a_sizeunion.get_ubyte_(i)); }
        ///     assert_eq!(a_sizeunion.get_ubyte_(0), 79_u8);
        ///     assert_eq!(a_sizeunion.get_ubyte_(1), 11_u8);
        ///     assert_eq!(a_sizeunion.get_ubyte_(2), 74_u8);
        ///     assert_eq!(a_sizeunion.get_ubyte_(3), 241_u8);
        ///     assert_eq!(a_sizeunion.get(), 4048161615_usize);
        /// }
        /// ```
        /// 
        /// # Example 7 for SizeUnion for 16-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "16")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let mut a_sizeunion = SizeUnion::new();
        ///     a_sizeunion.set_ubyte_(0, 79_u8);
        ///     a_sizeunion.set_ubyte_(1, 11_u8);
        ///     
        ///     // It will panic.
        ///     // a_sizeunion.set_ubyte_(2, 100_u8);
        ///     
        ///     println!("a_sizeunion.get() = {}", a_sizeunion.get());
        ///     for i in 0..2
        ///         { println!("a_sizeunion.get_ubyte_({}) = {}", i, a_sizeunion.get_ubyte_(i)); }
        ///     assert_eq!(a_sizeunion.get_ubyte_(0), 79_u8);
        ///     assert_eq!(a_sizeunion.get_ubyte_(1), 11_u8);
        ///     assert_eq!(a_sizeunion.get(), 2895_usize);
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
        /// # Example 6 for SizeUnion for 32-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "32")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let a_sizeunion = SizeUnion::new_with(4048161615_usize);
        ///     for i in 0..4
        ///         { println!("a_sizeunion.get_sbyte_({}) = {}", i, a_sizeunion.get_sbyte_(i)); }
        ///     assert_eq!(a_sizeunion.get_sbyte_(0), 79_i8);
        ///     assert_eq!(a_sizeunion.get_sbyte_(1), 11_i8);
        ///     assert_eq!(a_sizeunion.get_sbyte_(2), 74_i8);
        ///     assert_eq!(a_sizeunion.get_sbyte_(3), -15_i8);
        /// 
        ///     // It will panic.
        ///     // println!("a_sizeunion.get_sbyte_(4) = {}", a_sizeunion.get_sbyte_(4));
        /// }
        /// ```
        /// 
        /// # Example 7 for SizeUnion for 16-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "16")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let a_sizeunion = SizeUnion::new_with(2895_usize);
        ///     for i in 0..2
        ///         { println!("a_sizeunion.get_sbyte_({}) = {}", i, a_sizeunion.get_sbyte_(i)); }
        ///     assert_eq!(a_sizeunion.get_sbyte_(0), 79_i8);
        ///     assert_eq!(a_sizeunion.get_sbyte_(1), 11_i8);
        ///     
        ///     // It will panic.
        ///     // println!("a_sizeunion.get_sbyte_(2) = {}", a_sizeunion.get_sbyte_(2));
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
        /// 
        /// // It will panic.
        /// // a_shortunion.set_sbyte_(2, 100_i8);
        /// 
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
        /// 
        /// // It will panic.
        /// // a_intunion.set_sbyte_(4, 100_i8);
        /// 
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
        /// 
        /// // It will panic.
        /// // a_intunion.set_sbyte_(8, 100_i8);
        /// 
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
        /// 
        /// // It will panic.
        /// // a_longerunion.set_sbyte_(16, 100_i8);
        /// 
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
        /// 
        ///     // It will panic.
        ///     // a_sizeunion.set_sbyte_(8, 100_i8);
        /// 
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
        /// # Example 6 for SizeUnion for 32-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "32")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let mut a_sizeunion = SizeUnion::new();
        ///     a_sizeunion.set_sbyte_(0, 79_i8);
        ///     a_sizeunion.set_sbyte_(1, 11_i8);
        ///     a_sizeunion.set_sbyte_(2, 74_i8);
        ///     a_sizeunion.set_sbyte_(3, -15_i8);
        ///
        ///     // It will panic.
        ///     // a_sizeunion.set_sbyte_(4, 100_i8);
        ///
        ///     println!("a_sizeunion.get() = {}", a_sizeunion.get());
        ///     for i in 0..4
        ///         { println!("a_sizeunion.get_sbyte_({}) = {}", i, a_sizeunion.get_sbyte_(i)); }
        ///     assert_eq!(a_sizeunion.get_sbyte_(0), 79_i8);
        ///     assert_eq!(a_sizeunion.get_sbyte_(1), 11_i8);
        ///     assert_eq!(a_sizeunion.get_sbyte_(2), 74_i8);
        ///     assert_eq!(a_sizeunion.get_sbyte_(3), -15_i8);
        ///     assert_eq!(a_sizeunion.get(), 4048161615_usize);
        /// }
        /// ```
        /// 
        /// # Example 7 for SizeUnion for 16-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "16")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let mut a_sizeunion = SizeUnion::new();
        ///     a_sizeunion.set_sbyte_(0, 79_i8);
        ///     a_sizeunion.set_sbyte_(1, 11_i8);
        ///     
        ///     // It will panic.
        ///     // a_sizeunion.set_sbyte_(2, 100_i8);
        ///     
        ///     println!("a_sizeunion.get() = {}", a_sizeunion.get());
        ///     for i in 0..2
        ///         { println!("a_sizeunion.get_sbyte_({}) = {}", i, a_sizeunion.get_sbyte_(i)); }
        ///     assert_eq!(a_sizeunion.get_sbyte_(0), 79_i8);
        ///     assert_eq!(a_sizeunion.get_sbyte_(1), 11_i8);
        ///     assert_eq!(a_sizeunion.get(), 2895_usize);
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
        /// # Example 5 for SizeUnion for 32-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "32")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let a_sizeunion = SizeUnion::new_with(4048161615_usize);
        ///     for i in 0..4
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
        ///     assert_eq!(a_sizeunion.get_ubyte(4), None);
        /// }
        /// ```
        /// 
        /// # Example 7 for SizeUnion for 16-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "16")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let a_sizeunion = SizeUnion::new_with(2895_usize);
        ///     for i in 0..2
        ///     {
        ///         match a_sizeunion.get_ubyte(i)
        ///         {
        ///             Some(a) => { println!("a_sizeunion.get_ubyte({}) = {}", i, a); },
        ///             _ => {},
        ///         }
        ///     }
        ///     assert_eq!(a_sizeunion.get_ubyte(0), Some(79_u8));
        ///     assert_eq!(a_sizeunion.get_ubyte(1), Some(11_u8));
        ///     assert_eq!(a_sizeunion.get_ubyte(2), None);
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
        /// # Example 6 for SizeUnion for 32-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "32")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let mut a_sizeunion = SizeUnion::new();
        ///     let b0 = a_sizeunion.set_ubyte(0, 79_u8);
        ///     let b1 = a_sizeunion.set_ubyte(1, 11_u8);
        ///     let b2 = a_sizeunion.set_ubyte(2, 74_u8);
        ///     let b3 = a_sizeunion.set_ubyte(3, 241_u8);
        ///     let b4 = a_sizeunion.set_ubyte(4, 100_u8);
        ///     assert_eq!(b0, true);
        ///     assert_eq!(b1, true);
        ///     assert_eq!(b2, true);
        ///     assert_eq!(b3, true);
        ///     assert_eq!(b4, false);
        ///     println!("a_sizeunion.get() = {}", a_sizeunion.get());
        ///     for i in 0..4
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
        ///     assert_eq!(a_sizeunion.get(), 4048161615_usize);
        /// }
        /// ```
        /// 
        /// # Example 7 for SizeUnion for 16-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "16")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let mut a_sizeunion = SizeUnion::new();
        ///     let b0 = a_sizeunion.set_ubyte(0, 79_u8);
        ///     let b1 = a_sizeunion.set_ubyte(1, 11_u8);
        ///     let b2 = a_sizeunion.set_ubyte(2, 100_u8);  // Nothing will be done
        ///     assert_eq!(b0, true);
        ///     assert_eq!(b1, true);
        ///     assert_eq!(b2, false);
        ///     println!("a_sizeunion.get() = {}", a_sizeunion.get());
        ///     for i in 0..2
        ///     {
        ///         match a_sizeunion.get_ubyte(i)
        ///         {
        ///             Some(a) => { println!("a_sizeunion.get_ubyte({}) = {}", i, a); },
        ///             _ => {},
        ///         }
        ///     }
        ///     assert_eq!(a_sizeunion.get_ubyte(0), Some(79_u8));
        ///     assert_eq!(a_sizeunion.get_ubyte(1), Some(11_u8));
        ///     assert_eq!(a_sizeunion.get(), 2895_usize);
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
        /// # Example 6 for SizeUnion for 32-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "32")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let a_sizeunion = SizeUnion::new_with(4048161615_usize);
        ///     for i in 0..4
        ///     {
        ///         match a_intunion.get_sbyte(i)
        ///         {
        ///             Some(a) => { println!("a_sizeunion.get_sbyte({}) = {}", i, a); },
        ///             _ => {},
        ///         }
        ///     }
        ///     assert_eq!(a_sizeunion.get_sbyte(0), Some(79_i8));
        ///     assert_eq!(a_sizeunion.get_sbyte(1), Some(11_i8));
        ///     assert_eq!(a_sizeunion.get_sbyte(2), Some(74_i8));
        ///     assert_eq!(a_sizeunion.get_sbyte(3), Some(-15_i8));
        ///     assert_eq!(a_sizeunion.get_sbyte(4), None);
        /// }
        /// ```
        /// 
        /// # Example 7 for SizeUnion for 16-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "16")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let a_sizeunion = SizeUnion::new_with(2895_usize);
        ///     for i in 0..2
        ///     {
        ///         match a_sizeunion.get_sbyte(i)
        ///         {
        ///             Some(a) => { println!("a_sizeunion.get_sbyte({}) = {}", i, a); },
        ///             _ => {},
        ///         }
        ///     }
        ///     assert_eq!(a_sizeunion.get_sbyte(0), Some(79_i8));
        ///     assert_eq!(a_sizeunion.get_sbyte(1), Some(11_i8));
        ///     assert_eq!(a_sizeunion.get_sbyte(2), None);
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
        /// # Example 6 for SizeUnion for 32-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "32")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let mut a_sizeunion = SizeUnion::new();
        ///     let b0 = a_sizeunion.set_sbyte(0, 79_i8);
        ///     let b1 = a_sizeunion.set_sbyte(1, 11_i8);
        ///     let b2 = a_sizeunion.set_sbyte(2, 74_i8);
        ///     let b3 = a_sizeunion.set_sbyte(3, -15_i8);
        ///     let b4 = a_sizeunion.set_sbyte(4, 100_i8);
        ///     assert_eq!(b0, true);
        ///     assert_eq!(b1, true);
        ///     assert_eq!(b2, true);
        ///     assert_eq!(b3, true);
        ///     assert_eq!(b4, false);
        ///     println!("a_sizeunion.get() = {}", a_sizeunion.get());
        ///     for i in 0..4
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
        ///     assert_eq!(a_sizeunion.get(), 4048161615_usize);
        /// }
        /// ```
        /// 
        /// # Example 7 for SizeUnion for 16-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "16")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let mut a_sizeunion = SizeUnion::new();
        ///     let b0 = a_sizeunion.set_sbyte(0, 79_i8);
        ///     let b1 = a_sizeunion.set_sbyte(1, 11_i8);
        ///     let b2 = a_sizeunion.set_sbyte(2, 100_i8);  // Nothing will be done
        ///     assert_eq!(b0, true);
        ///     assert_eq!(b1, true);
        ///     assert_eq!(b2, false);
        ///     println!("a_sizeunion.get() = {}", a_sizeunion.get());
        ///     for i in 0..2
        ///     {
        ///         match a_sizeunion.get_sbyte(i)
        ///         {
        ///             Some(a) => { println!("a_sizeunion.get_sbyte({}) = {}", i, a); },
        ///             _ => {},
        ///         }
        ///     }
        ///     assert_eq!(a_sizeunion.get_sbyte(0), Some(79_i8));
        ///     assert_eq!(a_sizeunion.get_sbyte(1), Some(11_i8));
        ///     assert_eq!(a_sizeunion.get(), 2895_usize);
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
        /// let a_sizeunion = ShortUnion::new_with(55468_u16);
        /// println!("a_sizeunion = {}", a_sizeunion.get_ushort());
        /// assert_eq!(a_sizeunion.get_ushort(), 55468_u16);
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
        /// let mut a_sizeunion = ShortUnion::new();
        /// a_sizeunion.set_ushort(54321_u16);
        /// println!("a_sizeunion = {}", a_sizeunion.get_ushort());
        /// assert_eq!(a_sizeunion.get_ushort(), 54321_u16);
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
        /// let a_sizeunion = ShortUnion::new_with(54321_u16);
        /// println!("a_sizeunion = {}", a_sizeunion.get_sshort());
        /// assert_eq!(a_sizeunion.get_sshort(), -11215_i16);
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
        /// let mut a_sizeunion = ShortUnion::new();
        /// a_sizeunion.set_sshort(-11215_i16);
        /// println!("a_sizeunion = {}", a_sizeunion.get_sshort());
        /// assert_eq!(a_sizeunion.get_sshort(), -11215_i16);
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
        /// // println!("a_intunion.get_ushort_(2) = {}", a_intunion.get_ushort_(2));
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
        /// # Example 5 for SizeUnion for 32-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "32")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let a_sizeunion = SizeUnion::new_with(4048161615_usize);
        ///     for i in 0..2
        ///         { println!("a_sizeunion.get_ushort_({}) = {}", i, a_sizeunion.get_ushort_(i)); }
        ///     assert_eq!(a_sizeunion.get_ushort_(0), 2895_u16);
        ///     assert_eq!(a_sizeunion.get_ushort_(1), 61770_u16);
        /// 
        ///     // It will panic.
        ///     // println!("a_sizeunion.get_ushort_(2) = {}", a_sizeunion.get_ushort_(2));
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
        /// # Example 5 for SizeUnion for 32-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "32")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let mut a_sizeunion = SizeUnion::new();
        ///     a_sizeunion.set_ushort_(0, 2895_u16);
        ///     a_sizeunion.set_ushort_(1, 61770_u16);
        ///     
        ///     // It will panic.
        ///     // a_sizeunion.set_ushort_(2, 100_u16);
        ///     
        ///     println!("a_sizeunion.get() = {}", a_sizeunion.get());
        ///     for i in 0..2
        ///         { println!("a_sizeunion.get_ushort_({}) = {}", i, a_sizeunion.get_ushort_(i)); }
        ///     assert_eq!(a_sizeunion.get_ushort_(0), 2895_u16);
        ///     assert_eq!(a_sizeunion.get_ushort_(1), 61770_u16);
        ///     assert_eq!(a_sizeunion.get(), 4048161615_usize);
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
        /// # Example 5 for SizeUnion for 32-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "32")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let a_sizeunion = SizeUnion::new_with(4048161615_usize);
        ///     for i in 0..2
        ///         { println!("a_sizeunion.get_sshort_({}) = {}", i, a_sizeunion.get_sshort_(i)); }
        ///     assert_eq!(a_sizeunion.get_sshort_(0), 2895_i16);
        ///     assert_eq!(a_sizeunion.get_sshort_(1), -3766_i16);
        ///     
        ///     // It will panic.
        ///     // println!("a_sizeunion.get_sshort_(2) = {}", a_sizeunion.get_sshort_(2));
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
        /// # Example 5 for SizeUnion for 32-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "32")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     a_sizeunion.set_sshort_(0, 2895_i16);
        ///     a_sizeunion.set_sshort_(1, -3766_i16);
        ///     
        ///     // It will panic.
        ///     // a_sizeunion.set_sshort_(2, 100_i16);
        ///     
        ///     println!("a_sizeunion.get() = {}", a_sizeunion.get());
        ///     for i in 0..2
        ///         { println!("a_sizeunion.get_sshort_({}) = {}", i, a_sizeunion.get_sshort_(i)); }
        ///     assert_eq!(a_sizeunion.get_sshort_(0), 2895_i16);
        ///     assert_eq!(a_sizeunion.get_sshort_(1), -3766_i16);
        ///     assert_eq!(a_sizeunion.get(), 4048161615_usize);
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
        /// # Example 5 for SizeUnion for 32-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "32")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let a_sizeunion = IntUnion::new_with(4048161615_usize);
        ///     for i in 0..2
        ///     {
        ///         match a_sizeunion.get_ushort(i)
        ///         {
        ///             Some(a) => { println!("a_sizeunion.get_ushort({}) = {}", i, a); },
        ///             _ => {},
        ///         }
        ///     }
        ///     assert_eq!(a_sizeunion.get_ushort(0), Some(2895_u16));
        ///     assert_eq!(a_sizeunion.get_ushort(1), Some(61770_u16));
        ///     assert_eq!(a_sizeunion.get_ushort(2), None);
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
        /// # Example 5 for SizeUnion for 32-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "32")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let mut a_sizeunion = SizeUnion::new();
        ///     let b0 = a_sizeunion.set_ushort(0, 2895_u16);
        ///     let b1 = a_sizeunion.set_ushort(1, 61770_u16);
        ///     let b2 = a_sizeunion.set_ushort(2, 100_u16);
        ///     assert_eq!(b0, true);
        ///     assert_eq!(b1, true);
        ///     assert_eq!(b2, false);
        ///     println!("a_sizeunion.get() = {}", a_sizeunion.get());
        ///     for i in 0..2
        ///     {
        ///         match a_sizeunion.get_ushort(i)
        ///         {
        ///             Some(a) => { println!("a_sizeunion.get_ushort({}) = {}", i, a); },
        ///             _ => {},
        ///         }
        ///     }
        ///     assert_eq!(a_sizeunion.get_ushort(0), Some(2895_u16));
        ///     assert_eq!(a_sizeunion.get_ushort(1), Some(61770_u16));
        ///     assert_eq!(a_sizeunion.get(), 4048161615_usize);
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
        /// # Example 5 for SizeUnion for 32-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "32")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let a_sizeunion = IntUnion::new_with(4048161615_usize);
        ///     for i in 0..2
        ///     {
        ///         match a_sizeunion.get_sshort(i)
        ///         {
        ///             Some(a) => { println!("a_sizeunion.get_sshort({}) = {}", i, a); },
        ///             _ => {},
        ///         }
        ///     }
        ///     assert_eq!(a_sizeunion.get_sshort(0), Some(2895_i16));
        ///     assert_eq!(a_sizeunion.get_sshort(1), Some(-3766_i16));
        ///     assert_eq!(a_sizeunion.get_sshort(2), None);
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
        /// 
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
        /// 
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
        /// 
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
        /// # Example 5 for SizeUnion for 32-bit CPU
        /// ```
        /// #[cfg(target_pointer_width = "32")]
        /// {
        ///     use cryptocol::number::SizeUnion;
        ///     let mut a_sizeunion = SizeUnion::new();
        ///     let b0 = a_sizeunion.set_sshort(0, 2895_i16);
        ///     let b1 = a_sizeunion.set_sshort(1, -3766_i16);
        ///     let b2 = a_sizeunion.set_sshort(2, 100_i16);
        ///     assert_eq!(b0, true);
        ///     assert_eq!(b1, true);
        ///     assert_eq!(b2, false);
        ///     println!("a_sizeunion.get() = {}", a_sizeunion.get());
        ///     for i in 0..2
        ///     {
        ///         match a_sizeunion.get_sshort(i)
        ///         {
        ///             Some(a) => { println!("a_sizeunion.set_sshort({}) = {}", i, a); },
        ///             _ => {},
        ///         }
        ///     }
        ///     assert_eq!(a_sizeunion.get_sshort(0), Some(2895_i16));
        ///     assert_eq!(a_sizeunion.get_sshort(1), Some(-3766_i16));
        ///     assert_eq!(a_sizeunion.get(), 4048161615_usize);
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
        /// let a_intunion = IntUnion::new_with(987654321_u32);
        /// println!("a_intunion = {}", a_intunion.get_uint());
        /// assert_eq!(a_intunion.get_uint(), 987654321_u32);
        /// ```
        #[inline] pub fn get_uint(self) -> u32      { unsafe { self.uint } }

        // pub fn set_uint(&mut self, val: u32)
        /// Sets its value with `val` of type `u32`
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::IntUnion;
        /// let mut a_intunion = IntUnion::new();
        /// a_intunion.set_uint(987654321_u32);
        /// println!("a_intunion = {}", a_intunion.get_uint());
        /// assert_eq!(a_intunion.get_uint(), 987654321_u32);
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
        /// let a_intunion = IntUnion::new_with(2345678901_u32);
        /// println!("a_intunion = {}", a_intunion.get_sint());
        /// assert_eq!(a_intunion.get_sint(), -1949288395_i32);
        /// ```
        #[inline] pub fn get_sint(self) -> i32      { unsafe { self.sint } }

        // pub fn set_sint(&mut self, val: i32)
        /// Sets its value with `val` of type `i32`
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::IntUnion;
        /// let mut a_intunion = IntUnion::new();
        /// a_intunion.set_sint(-1949288395_i32);
        /// println!("a_intunion = {}", a_intunion.get_sint());
        /// assert_eq!(a_intunion.get_sint(), -1949288395_i32);
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
        /// let a_longunion = LongUnion::new_with(654321987654321_u64);
        /// println!("a_longunion = {}", a_longunion.get_ulong());
        /// assert_eq!(a_longunion.get_ulong(), 654321987654321_u64);
        /// ```
        #[inline] pub fn get_ulong(self) -> u64         { unsafe { self.ulong } }

        // pub fn set_ulong(&mut self, val: u64)
        /// Sets its value with `val` of type `u64`
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let mut a_longunion = LongUnion::new();
        /// a_longunion.set_ulong(654321987654321_u64);
        /// println!("a_longunion = {}", a_longunion.get_ulong());
        /// assert_eq!(a_longunion.get_ulong(), 654321987654321_u64);
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
        /// let a_longunion = LongUnion::new_with(12345678909876456789_u64);
        /// println!("a_longunion = {}", a_longunion.get_slong());
        /// assert_eq!(a_longunion.get_slong(), -6101065163833094827_i64);
        /// ```
        #[inline] pub fn get_slong(self) -> i64     { unsafe { self.slong } }

        // pub fn set_slong(&mut self, val: i64)
        /// Sets its value with `val` of type `i64`
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let mut a_longunion = LongUnion::new();
        /// a_longunion.set_slong(-6101065163833094827_i64);
        /// println!("a_longunion = {}", a_longunion.get_slong());
        /// assert_eq!(a_longunion.get_slong(), -6101065163833094827_i64);
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
        /// let a_longerunion = LongerUnion::new_with(98765432101234567898765432101234546789_u128);
        /// println!("a_longerunion = {}", a_longerunion.get_ulonger());
        /// assert_eq!(a_longerunion.get_ulonger(), 98765432101234567898765432101234546789_u128);
        /// ```
        #[inline] pub fn get_ulonger(self) -> u128      { unsafe { self.ulonger } }

        // pub fn set_ulonger(&mut self, val: u128)
        /// Sets its value with `val` of type `u128`
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::LongerUnion;    
        /// let mut a_longerunion = LongerUnion::new();
        /// a_longerunion.set_ulonger(98765432101234567898765432101234546789_u128);
        /// println!("a_longerunion = {}", a_longerunion.get_ulonger());
        /// assert_eq!(a_longerunion.get_ulonger(), 98765432101234567898765432101234546789_u128);
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
        /// let a_longerunion = LongerUnion::new_with(234567890987654321012345678987654321234_u128);
        /// println!("a_longerunion = {}", a_longerunion.get_slonger());
        /// assert_eq!(a_longerunion.get_slonger(), -105714475933284142451028928444113890222_i128);
        /// ```
        #[inline] pub fn get_slonger(self) -> i128      { unsafe { self.slonger } }

        // pub fn set_slonger(&mut self, val: i128)
        /// Sets its value with `val` of type `i128`
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::LongerUnion;    
        /// let mut a_longerunion = LongerUnion::new();
        /// a_longerunion.set_slonger(-105714475933284142451028928444113890222_i128);
        /// println!("a_longerunion = {}", a_longerunion.get_slonger());
        /// assert_eq!(a_longerunion.get_slonger(), -105714475933284142451028928444113890222_i128);
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
        /// let a_sizeunion = SizeUnion::new_with(250_usize);
        /// println!("a_sizeunion = {}", a_sizeunion.get_usize());
        /// assert_eq!(a_sizeunion.get_usize(), 250_usize);
        /// ```
        #[inline] pub fn get_usize(self) -> usize   { unsafe { self.u_size } }

        // pub fn set_usize(&mut self, val: usize)
        /// Sets its value with `val` of type `usize`
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::SizeUnion;    
        /// let mut a_sizeunion = SizeUnion::new();
        /// a_sizeunion.set_usize(234_usize);
        /// println!("a_sizeunion = {}", a_sizeunion.get_usize());
        /// assert_eq!(a_sizeunion.get_usize(), 234_usize);
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
        /// let a_sizeunion = SizeUnion::new_with_signed(-123_isize);
        /// println!("a_sizeunion = {}", a_sizeunion.get_ssize());
        /// assert_eq!(a_sizeunion.get_ssize(), -123_isize);
        /// ```
        #[inline] pub fn get_ssize(self) -> isize   { unsafe { self.s_size } }

        // pub fn set_ssize(&mut self, val: isize)
        /// Sets its value with `val` of type `isize`
        /// 
        /// Example
        /// ```
        /// use cryptocol::number::SizeUnion;    
        /// let mut a_sizeunion = SizeUnion::new();
        /// a_sizeunion.set_ssize(-123_isize);
        /// println!("a_sizeunion = {}", a_sizeunion.get_ssize());
        /// assert_eq!(a_sizeunion.get_ssize(), -123_isize);
        /// ```
        #[inline] pub fn set_ssize(&mut self, val: isize)   { self.s_size = val; }
    };
}
pub(super) use get_set_size_fit;

macro_rules! get_set_size {
    ($f:expr) => {
        const J: usize = $f - 1;

        // // pub fn get_usize_(&self, i: usize) -> usize
        // /// Returns i-th element of array `u_size` of type `usize`
        // /// if `i` is less than the size of this union in bytes divided by
        // /// the size of the type `usize` in bytes. Otherwise, it will panic.
        // /// 
        // /// # Arguments
        // /// `i` indicates `i`-th element. 0-th element contains LSB (Least
        // /// Significant Bit), while (the size of this union in bytes - 1)-th
        // /// element contains MSB (Most Significant Bit) regardless endianness.
        // /// 
        // /// # Output
        // /// i-th element of array `u_size` of type `usize`
        // /// if `i` is less than the size of this union in bytes divided by
        // /// the size of the type `usize` in bytes
        // /// 
        // /// # Panics
        // /// So, if `i` is greater than or equal to the size of this union
        // /// in bytes divided by the size of the type `usize` in bytes,
        // /// it will panic.
        // /// 
        // /// # Caution
        // /// Use this method only when you are sure that the argument `i` is
        // /// less than the size of this union in bytes divided by the size of
        // /// the type `usize` in bytes
        // /// 
        // /// # Counterpart Method
        // /// It is performance-oriented and does not care for safety.
        // /// It is virtually the same as the method get_usize(). This method
        // /// get_usize_() is considered to be slightly faster than the method
        // /// get_usize().
        // /// Use this method only when you are sure that `i` is less than
        // /// the size of this union in bytes divided by the size of the type
        // /// `usize` in bytes. Otherwise, use its counterpart method
        // /// [get_usize()](#method.get_usize) for safety.
        // /// 
        // /// # Example 1 for ShortUnion
        // /// ```
        // /// use cryptocol::number::ShortUnion;
        // /// let a_shortunion = ShortUnion::new_with(2895_u16);
        // /// for i in 0..2
        // ///     { println!("a_shortunion.get_usize_({}) = {}", i, a_shortunion.get_usize_(i)); }
        // /// assert_eq!(a_shortunion.get_usize_(0), 79_usize);
        // /// assert_eq!(a_shortunion.get_usize_(1), 11_usize);
        // /// 
        // /// // It will panic.
        // /// // println!("a_shortunion.get_usize_(2) = {}", a_shortunion.get_usize_(2));
        // /// ```
        // /// 
        // /// # Example 2 for IntUnion
        // /// ```
        // /// use cryptocol::number::IntUnion;
        // /// let a_intunion = IntUnion::new_with(4048161615_u32);
        // /// for i in 0..4
        // ///     { println!("a_intunion.get_usize_({}) = {}", i, a_intunion.get_usize_(i)); }
        // /// assert_eq!(a_intunion.get_usize_(0), 79_usize);
        // /// assert_eq!(a_intunion.get_usize_(1), 11_usize);
        // /// assert_eq!(a_intunion.get_usize_(2), 74_usize);
        // /// assert_eq!(a_intunion.get_usize_(3), 241_usize);
        // /// 
        // /// // It will panic.
        // /// // println!("a_intunion.get_usize_(4) = {}", a_intunion.get_usize_(4));
        // /// ```
        // /// 
        // /// # Example 3 for LongUnion
        // /// ```
        // /// use cryptocol::number::LongUnion;
        // /// let a_longunion = LongUnion::new_with(13664881099896654671_u64);
        // /// for i in 0..8
        // ///     { println!("a_longunion.get_usize_({}) = {}", i, a_longunion.get_usize_(i)); }
        // /// assert_eq!(a_longunion.get_usize_(0), 79_usize);
        // /// assert_eq!(a_longunion.get_usize_(1), 11_usize);
        // /// assert_eq!(a_longunion.get_usize_(2), 74_usize);
        // /// assert_eq!(a_longunion.get_usize_(3), 241_usize);
        // /// assert_eq!(a_longunion.get_usize_(4), 245_usize);
        // /// assert_eq!(a_longunion.get_usize_(5), 104_usize);
        // /// assert_eq!(a_longunion.get_usize_(6), 163_usize);
        // /// assert_eq!(a_longunion.get_usize_(7), 189_usize);
        // /// 
        // /// // It will panic.
        // /// // println!("a_longunion.get_usize_(8) = {}", a_longunion.get_usize_(8));
        // /// ```
        // /// 
        // /// # Example 4 for LongerUnion
        // /// ```
        // /// use cryptocol::number::LongerUnion;
        // /// let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        // /// for i in 0..16
        // ///     { println!("a_longerunion.get_usize_({}) = {}", i, a_longerunion.get_usize_(i)); }
        // /// assert_eq!(a_longerunion.get_usize_(0), 79_usize);
        // /// assert_eq!(a_longerunion.get_usize_(1), 11_usize);
        // /// assert_eq!(a_longerunion.get_usize_(2), 74_usize);
        // /// assert_eq!(a_longerunion.get_usize_(3), 241_usize);
        // /// assert_eq!(a_longerunion.get_usize_(4), 245_usize);
        // /// assert_eq!(a_longerunion.get_usize_(5), 104_usize);
        // /// assert_eq!(a_longerunion.get_usize_(6), 163_usize);
        // /// assert_eq!(a_longerunion.get_usize_(7), 189_usize);
        // /// assert_eq!(a_longerunion.get_usize_(8), 88_usize);
        // /// assert_eq!(a_longerunion.get_usize_(9), 136_usize);
        // /// assert_eq!(a_longerunion.get_usize_(10), 206_usize);
        // /// assert_eq!(a_longerunion.get_usize_(11), 126_usize);
        // /// assert_eq!(a_longerunion.get_usize_(12), 26_usize);
        // /// assert_eq!(a_longerunion.get_usize_(13), 59_usize);
        // /// assert_eq!(a_longerunion.get_usize_(14), 18_usize);
        // /// assert_eq!(a_longerunion.get_usize_(15), 255_usize);
        // /// 
        // /// // It will panic.
        // /// // println!("a_longerunion.get_usize_(16) = {}", a_longerunion.get_usize_(16));
        // /// ```
        // /// 
        // /// # Big-endian issue
        // /// It is just experimental for Big Endian CPUs. So, you are not
        // /// encouraged to use it for Big Endian CPUs for serious purpose.
        // /// Only use this crate for Big-endian CPUs with your own full
        // /// responsibility.
        // #[cfg(target_pointer_width = "8")]
        // #[inline] pub fn get_usize_(&self, i: usize) -> usize
        // {
        //     #[cfg(target_endian = "little")]    unsafe { self.u_size[i] }
        //     #[cfg(target_endian = "big")]       unsafe { self.u_size[Self::J-i] }
        // }

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
        /// # Example 1 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// let a_intunion = IntUnion::new_with(4048161615_u32);
        /// for i in 0..2
        ///     { println!("a_intunion.get_usize_({}) = {}", i, a_intunion.get_usize_(i)); }
        /// assert_eq!(a_intunion.get_usize_(0), 2895_usize);
        /// assert_eq!(a_intunion.get_usize_(1), 61770_usize);
        /// 
        /// // It will panic.
        /// // println!("a_intunion.get_usize_(2) = {}", a_intunion.get_usize_(2));
        /// ```
        /// 
        /// # Example 2 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let a_longunion = LongUnion::new_with(13664881099896654671_u64);
        /// for i in 0..4
        ///     { println!("a_longunion.get_usize_({}) = {}", i, a_longunion.get_usize_(i)); }
        /// assert_eq!(a_longunion.get_usize_(0), 2895_usize);
        /// assert_eq!(a_longunion.get_usize_(1), 61770_usize);
        /// assert_eq!(a_longunion.get_usize_(2), 26869_usize);
        /// assert_eq!(a_longunion.get_usize_(3), 48547_usize);
        /// 
        /// // It will panic.
        /// // println!("a_longunion.get_usize_(4) = {}", a_longunion.get_usize_(4));
        /// ```
        /// 
        /// # Example 3 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// for i in 0..8
        ///     { println!("a_longerunion.get_usize_({}) = {}", i, a_longerunion.get_usize_(i)); }
        /// assert_eq!(a_longerunion.get_usize_(0), 2895_usize);
        /// assert_eq!(a_longerunion.get_usize_(1), 61770_usize);
        /// assert_eq!(a_longerunion.get_usize_(2), 26869_usize);
        /// assert_eq!(a_longerunion.get_usize_(3), 48547_usize);
        /// assert_eq!(a_longerunion.get_usize_(4), 34904_usize);
        /// assert_eq!(a_longerunion.get_usize_(5), 32462_usize);
        /// assert_eq!(a_longerunion.get_usize_(6), 15130_usize);
        /// assert_eq!(a_longerunion.get_usize_(7), 65298_usize);
        /// 
        /// // It will panic.
        /// // println!("a_longerunion.get_usize_(8) = {}", a_longerunion.get_usize_(8));
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[cfg(target_pointer_width = "16")]
        #[inline] pub fn get_usize_(&self, i: usize) -> usize
        {
            #[cfg(target_endian = "little")]    unsafe { self.u_size[i] }
            #[cfg(target_endian = "big")]       unsafe { self.u_size[Self::J-i] }
        }


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
        /// # Example 1 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let a_longunion = LongUnion::new_with(13664881099896654671_u64);
        /// for i in 0..2
        ///     { println!("a_longunion.get_usize_({}) = {}", i, a_longunion.get_usize_(i)); }
        /// assert_eq!(a_longunion.get_usize_(0), 4048161615_usize);
        /// assert_eq!(a_longunion.get_usize_(1), 3181603061_usize);
        /// 
        /// // It will panic.
        /// // println!("a_longunion.get_usize_(2) = {}", a_longunion.get_usize_(2));
        /// ```
        /// 
        /// # Example 2 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// for i in 0..4
        ///     { println!("a_longerunion.get_usize_({}) = {}", i, a_longerunion.get_usize_(i)); }
        /// assert_eq!(a_longerunion.get_usize_(0), 4048161615_usize);
        /// assert_eq!(a_longerunion.get_usize_(1), 3181603061_usize);
        /// assert_eq!(a_longerunion.get_usize_(2), 2127464536_usize);
        /// assert_eq!(a_longerunion.get_usize_(3), 4279384858_usize);
        /// 
        /// // It will panic.
        /// // println!("a_longerunion.get_usize_(4) = {}", a_longerunion.get_usize_(4));
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[cfg(target_pointer_width = "32")]
        #[inline] pub fn get_usize_(&self, i: usize) -> usize
        {
            #[cfg(target_endian = "little")]    unsafe { self.u_size[i] }
            #[cfg(target_endian = "big")]       unsafe { self.u_size[Self::J-i] }
        }

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
        /// # Example
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let a_sizeerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// for i in 0..2
        ///     { println!("a_sizeerunion.get_usize_({}) = {}", i, a_sizeerunion.get_usize_(i)); }
        /// assert_eq!(a_sizeerunion.get_usize_(0), 13664881099896654671_usize);
        /// assert_eq!(a_sizeerunion.get_usize_(1), 18379818014235068504_usize);
        /// 
        /// // It will panic.
        /// // println!("a_sizeerunion.get_usize_(2) = {}", a_sizeerunion.get_usize_(2));
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[cfg(target_pointer_width = "64")]
        #[inline] pub fn get_usize_(&self, i: usize) -> usize
        {
            #[cfg(target_endian = "little")]    unsafe { self.u_size[i] }
            #[cfg(target_endian = "big")]       unsafe { self.u_size[Self::J-i] }
        }

        // // pub fn set_usize_(&mut self, i: usize, val: usize) 
        // /// Sets i-th element of its array `u_size` of type `usize`
        // /// if `i` is less than the size of this union in bytes divided
        // /// by the size of the type `usize` in bytes.
        // /// Otherwise, it will panic.
        // /// 
        // /// # Arguments
        // /// - `i` indicates `i`-th element. 0-th element contains LSB (Least
        // /// Significant Bit), while (the size of this union in bytes - 1)-th
        // /// element contains MSB (Most Significant Bit) regardless endianness.
        // /// - `val` is the value of type `usize` to set the i-th element of its
        // ///  array `u_size` of type `usize`.
        // /// 
        // /// # Panics
        // /// So, if `i` is greater than or equal to the size of this union
        // /// in bytes divided by the size of the type `usize` in bytes,
        // /// it will panic.
        // /// 
        // /// # Caution
        // /// Use this method only when you are sure that the argument `i` is
        // /// less than the size of this union in bytes divided by the size of
        // /// the type `usize` in bytes
        // /// 
        // /// # Counterpart Method
        // /// It is performance-oriented and does not care for safety.
        // /// It is virtually the same as the method set_usize(). This method
        // /// set_usize_() is considered to be slightly faster than the method
        // /// set_usize().
        // /// Use this method only when you are sure that `i` is less than
        // /// the size of this union in bytes divided by the size of the type
        // /// `usize` in bytes. Otherwise, use its counterpart
        // /// method [set_usize()](#method.set_usize) for safety.
        // /// 
        // /// # Example 1 for ShortUnion
        // /// ```
        // /// use cryptocol::number::ShortUnion;
        // /// let mut a_shortunion = ShortUnion::new();
        // /// a_shortunion.set_usize_(0, 79_usize);
        // /// a_shortunion.set_usize_(1, 11_usize);
        // /// 
        // /// // It will panic.
        // /// // a_shortunion.set_usize_(2, 100_usize);
        // /// 
        // /// println!("a_shortunion.get() = {}", a_shortunion.get());
        // /// for i in 0..2
        // ///     { println!("a_shortunion.get_usize_({}) = {}", i, a_shortunion.get_usize_(i)); }
        // /// assert_eq!(a_shortunion.get_usize_(0), 79_usize);
        // /// assert_eq!(a_shortunion.get_usize_(1), 11_usize);
        // /// assert_eq!(a_shortunion.get(), 2895_u16);
        // /// ```
        // /// 
        // /// # Example 2 for IntUnion
        // /// ```
        // /// use cryptocol::number::IntUnion;
        // /// let mut a_intunion = IntUnion::new();
        // /// a_intunion.set_usize_(0, 79_usize);
        // /// a_intunion.set_usize_(1, 11_usize);
        // /// a_intunion.set_usize_(2, 74_usize);
        // /// a_intunion.set_usize_(3, 241_usize);
        // /// 
        // /// // It will panic.
        // /// // a_intunion.set_usize_(4, 100_usize);
        // /// 
        // /// println!("a_intunion.get() = {}", a_intunion.get());
        // /// for i in 0..4
        // ///     { println!("a_intunion.get_usize_({}) = {}", i, a_intunion.get_usize_(i)); }
        // /// assert_eq!(a_intunion.get_usize_(0), 79_usize);
        // /// assert_eq!(a_intunion.get_usize_(1), 11_usize);
        // /// assert_eq!(a_intunion.get_usize_(2), 74_usize);
        // /// assert_eq!(a_intunion.get_usize_(3), 241_usize);
        // /// assert_eq!(a_intunion.get(), 4048161615_u32);
        // /// ```
        // /// 
        // /// # Example 3 for LongUnion
        // /// ```
        // /// use cryptocol::number::LongUnion;
        // /// let mut a_longunion = LongUnion::new();
        // /// a_longunion.set_usize_(0, 79_usize);
        // /// a_longunion.set_usize_(1, 11_usize);
        // /// a_longunion.set_usize_(2, 74_usize);
        // /// a_longunion.set_usize_(3, 241_usize);
        // /// a_longunion.set_usize_(4, 245_usize);
        // /// a_longunion.set_usize_(5, 104_usize);
        // /// a_longunion.set_usize_(6, 163_usize);
        // /// a_longunion.set_usize_(7, 189_usize);
        // /// 
        // /// // It will panic.
        // /// // a_longunion.set_usize_(8, 100_usize);
        // /// 
        // /// for i in 0..8
        // ///     { println!("a_longunion.get_usize_({}) = {}", i, a_longunion.get_usize_(i)); }
        // /// assert_eq!(a_longunion.get_usize_(0), 79_usize);
        // /// assert_eq!(a_longunion.get_usize_(1), 11_usize);
        // /// assert_eq!(a_longunion.get_usize_(2), 74_usize);
        // /// assert_eq!(a_longunion.get_usize_(3), 241_usize);
        // /// assert_eq!(a_longunion.get_usize_(4), 245_usize);
        // /// assert_eq!(a_longunion.get_usize_(5), 104_usize);
        // /// assert_eq!(a_longunion.get_usize_(6), 163_usize);
        // /// assert_eq!(a_longunion.get_usize_(7), 189_usize);
        // /// assert_eq!(a_longunion.get(), 13664881099896654671_u64);
        // /// ```
        // /// 
        // /// # Example 4 for LongerUnion
        // /// ```
        // /// use cryptocol::number::LongerUnion;
        // /// let mut a_longerunion = LongerUnion::new();
        // /// a_longerunion.set_usize_(0, 79_usize);
        // /// a_longerunion.set_usize_(1, 11_usize);
        // /// a_longerunion.set_usize_(2, 74_usize);
        // /// a_longerunion.set_usize_(3, 241_usize);
        // /// a_longerunion.set_usize_(4, 245_usize);
        // /// a_longerunion.set_usize_(5, 104_usize);
        // /// a_longerunion.set_usize_(6, 163_usize);
        // /// a_longerunion.set_usize_(7, 189_usize);
        // /// a_longerunion.set_usize_(8, 88_usize);
        // /// a_longerunion.set_usize_(9, 136_usize);
        // /// a_longerunion.set_usize_(10, 206_usize);
        // /// a_longerunion.set_usize_(11, 126_usize);
        // /// a_longerunion.set_usize_(12, 26_usize);
        // /// a_longerunion.set_usize_(13, 59_usize);
        // /// a_longerunion.set_usize_(14, 18_usize);
        // /// a_longerunion.set_usize_(15, 255_usize);
        // /// 
        // /// // It will panic.
        // /// // a_longerunion.set_usize_(16, 100_usize);
        // /// 
        // /// for i in 0..16
        // ///     { println!("a_longerunion.get_usize_({}) = {}", i, a_longerunion.get_usize_(i)); }
        // /// assert_eq!(a_longerunion.get_usize_(0), 79_usize);
        // /// assert_eq!(a_longerunion.get_usize_(1), 11_usize);
        // /// assert_eq!(a_longerunion.get_usize_(2), 74_usize);
        // /// assert_eq!(a_longerunion.get_usize_(3), 241_usize);
        // /// assert_eq!(a_longerunion.get_usize_(4), 245_usize);
        // /// assert_eq!(a_longerunion.get_usize_(5), 104_usize);
        // /// assert_eq!(a_longerunion.get_usize_(6), 163_usize);
        // /// assert_eq!(a_longerunion.get_usize_(7), 189_usize);
        // /// assert_eq!(a_longerunion.get_usize_(8), 88_usize);
        // /// assert_eq!(a_longerunion.get_usize_(9), 136_usize);
        // /// assert_eq!(a_longerunion.get_usize_(10), 206_usize);
        // /// assert_eq!(a_longerunion.get_usize_(11), 126_usize);
        // /// assert_eq!(a_longerunion.get_usize_(12), 26_usize);
        // /// assert_eq!(a_longerunion.get_usize_(13), 59_usize);
        // /// assert_eq!(a_longerunion.get_usize_(14), 18_usize);
        // /// assert_eq!(a_longerunion.get_usize_(15), 255_usize);
        // /// assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
        // /// ```
        // /// 
        // /// # Big-endian issue
        // /// It is just experimental for Big Endian CPUs. So, you are not
        // /// encouraged to use it for Big Endian CPUs for serious purpose.
        // /// Only use this crate for Big-endian CPUs with your own full
        // /// responsibility.
        // #[cfg(target_pointer_width = "8")]
        // #[inline] pub fn set_usize_(&mut self, i: usize, val: usize) 
        // {
        //     #[cfg(target_endian = "little")]    unsafe { self.u_size[i] = val; }
        //     #[cfg(target_endian = "big")]       unsafe { self.u_size[Self::J-i] = val; }
        // }

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
        /// # Example 1 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// let mut a_intunion = IntUnion::new();
        /// a_intunion.set_usize_(0, 2895_usize);
        /// a_intunion.set_usize_(1, 61770_usize);
        /// 
        /// // It will panic.
        /// // a_intunion.set_usize_(2, 100_usize);
        /// 
        /// println!("a_intunion.get() = {}", a_intunion.get());
        /// for i in 0..2
        ///     { println!("a_intunion.get_usize_({}) = {}", i, a_intunion.get_usize_(i)); }
        /// assert_eq!(a_intunion.get_usize_(0), 2895_usize);
        /// assert_eq!(a_intunion.get_usize_(1), 61770_usize);
        /// assert_eq!(a_intunion.get(), 4048161615_u32);
        /// ```
        /// 
        /// # Example 2 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let mut a_longunion = LongUnion::new();
        /// a_longunion.set_usize_(0, 2895_usize);
        /// a_longunion.set_usize_(1, 61770_usize);
        /// a_longunion.set_usize_(2, 26869_usize);
        /// a_longunion.set_usize_(3, 48547_usize);
        /// 
        /// // It will panic.
        /// // a_longunion.set_usize_(4, 100_usize);
        /// 
        /// for i in 0..4
        ///     { println!("a_longunion.get_usize_({}) = {}", i, a_longunion.get_usize_(i)); }
        /// assert_eq!(a_longunion.get_usize_(0), 2895_usize);
        /// assert_eq!(a_longunion.get_usize_(1), 61770_usize);
        /// assert_eq!(a_longunion.get_usize_(2), 26869_usize);
        /// assert_eq!(a_longunion.get_usize_(3), 48547_usize);
        /// assert_eq!(a_longunion.get(), 13664881099896654671_u64);
        /// ```
        /// 
        /// # Example 3 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let mut a_longerunion = LongerUnion::new();
        /// a_longerunion.set_usize_(0, 2895_usize);
        /// a_longerunion.set_usize_(1, 61770_usize);
        /// a_longerunion.set_usize_(2, 26869_usize);
        /// a_longerunion.set_usize_(3, 48547_usize);
        /// a_longerunion.set_usize_(4, 34904_usize);
        /// a_longerunion.set_usize_(5, 32462_usize);
        /// a_longerunion.set_usize_(6, 15130_usize);
        /// a_longerunion.set_usize_(7, 65298_usize);
        /// 
        /// // It will panic.
        /// // a_longerunion.set_usize_(8, 100_usize);
        /// 
        /// for i in 0..8
        ///     { println!("a_longerunion.get_usize_({}) = {}", i, a_longerunion.get_usize_(i)); }
        /// assert_eq!(a_longerunion.get_usize_(0), 2895_usize);
        /// assert_eq!(a_longerunion.get_usize_(1), 61770_usize);
        /// assert_eq!(a_longerunion.get_usize_(2), 26869_usize);
        /// assert_eq!(a_longerunion.get_usize_(3), 48547_usize);
        /// assert_eq!(a_longerunion.get_usize_(4), 34904_usize);
        /// assert_eq!(a_longerunion.get_usize_(5), 32462_usize);
        /// assert_eq!(a_longerunion.get_usize_(6), 15130_usize);
        /// assert_eq!(a_longerunion.get_usize_(7), 65298_usize);
        /// assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[cfg(target_pointer_width = "16")]
        #[inline] pub fn set_usize_(&mut self, i: usize, val: usize) 
        {
            #[cfg(target_endian = "little")]    unsafe { self.u_size[i] = val; }
            #[cfg(target_endian = "big")]       unsafe { self.u_size[Self::J-i] = val; }
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
        /// # Example 1 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let mut a_longunion = LongUnion::new();
        /// a_longunion.set_usize_(0, 4048161615_usize);
        /// a_longunion.set_usize_(1, 3181603061_usize);
        /// 
        /// // It will panic.
        /// // a_longunion.set_ushort_(2, 100_usize);
        /// 
        /// for i in 0..2
        ///     { println!("a_longunion.get_ushort_({}) = {}", i, a_longunion.get_ushort_(i)); }
        /// assert_eq!(a_longunion.get_usize_(0), 4048161615_usize);
        /// assert_eq!(a_longunion.get_usize_(1), 3181603061_usize);
        /// assert_eq!(a_longunion.get(), 13664881099896654671_u64);
        /// ```
        /// 
        /// # Example 2 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let mut a_longerunion = LongerUnion::new();
        /// a_longerunion.set_usize_(0, 4048161615_usize);
        /// a_longerunion.set_usize_(1, 3181603061_usize);
        /// a_longerunion.set_usize_(2, 2127464536_usize);
        /// a_longerunion.set_usize_(3, 4279384858_usize);
        /// 
        /// // It will panic.
        /// // a_longerunion.set_usize_(4, 100_usize);
        /// 
        /// for i in 0..4
        ///     { println!("a_longerunion.get_usize_({}) = {}", i, a_longerunion.get_usize_(i)); }
        /// assert_eq!(a_longerunion.get_usize_(0), 4048161615_usize);
        /// assert_eq!(a_longerunion.get_usize_(1), 3181603061_usize);
        /// assert_eq!(a_longerunion.get_usize_(2), 2127464536_usize);
        /// assert_eq!(a_longerunion.get_usize_(3), 4279384858_usize);
        /// assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[cfg(target_pointer_width = "32")]
        #[inline] pub fn set_usize_(&mut self, i: usize, val: usize) 
        {
            #[cfg(target_endian = "little")]    unsafe { self.u_size[i] = val; }
            #[cfg(target_endian = "big")]       unsafe { self.u_size[Self::J-i] = val; }
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
        /// # Example
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let mut a_sizeerunion = LongerUnion::new();
        /// a_sizeerunion.set_usize_(0, 13664881099896654671_usize);
        /// a_sizeerunion.set_usize_(1, 18379818014235068504_usize);
        /// 
        /// // It will panic.
        /// // a_sizeerunion.set_usize_(2, 100_usize);
        /// 
        /// for i in 0..2
        ///     { println!("a_sizeerunion.get_usize_({}) = {}", i, a_sizeerunion.get_usize_(i)); }
        /// assert_eq!(a_sizeerunion.get_usize_(0), 13664881099896654671_usize);
        /// assert_eq!(a_sizeerunion.get_usize_(1), 18379818014235068504_usize);
        /// assert_eq!(a_sizeerunion.get(), 339047799029950809142362261752780557135_u128);
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[cfg(target_pointer_width = "64")]
        #[inline] pub fn set_usize_(&mut self, i: usize, val: usize) 
        {
            #[cfg(target_endian = "little")]    unsafe { self.u_size[i] = val; }
            #[cfg(target_endian = "big")]       unsafe { self.u_size[Self::J-i] = val; }
        }

        // // pub fn get_ssize_(&self, i: usize) -> isize
        // /// Returns i-th element of array `s_size` of type `isize`
        // /// if `i` is less than the size of this union in bytes divided by
        // /// the size of the type `isize` in bytes. Otherwise, it will panic.
        // /// 
        // /// # Arguments
        // /// - `i` indicates `i`-th element. 0-th element contains LSB (Least
        // /// Significant Bit), while (the size of this union in bytes - 1)-th
        // /// element contains MSB (Most Significant Bit) regardless endianness.
        // /// 
        // /// # Output
        // /// i-th element of array `s_size` of type `isize`
        // /// if `i` is less than the size of this union in bytes divided by
        // /// the size of the type `isize` in bytes
        // /// 
        // /// # Panics
        // /// So, if `i` is greater than or equal to the size of this union
        // /// in bytes divided by the size of the type `isize` in bytes,
        // /// it will panic.
        // /// 
        // /// # Caution
        // /// Use this method only when you are sure that the argument `i` is
        // /// less than the size of this union in bytes divided by the size of
        // /// the type `usize` in bytes
        // /// 
        // /// # Counterpart Method
        // /// It is performance-oriented and does not care for safety.
        // /// It is virtually the same as the method get_ssize(). This method
        // /// get_ssize_() is considered to be slightly faster than the method
        // /// get_ssize().
        // /// Use this method only when you are sure that `i` is less than
        // /// the size of this union in bytes divided by the size of the type
        // /// `isize` in bytes. Otherwise, use its counterpart method
        // /// [get_ssize()](#method.get_ssize) for safety.
        // /// 
        // /// # Example 1 for ShortUnion
        // /// ```
        // /// use cryptocol::number::ShortUnion;
        // /// let a_shortunion = ShortUnion::new_with(2895_u16);
        // /// for i in 0..2
        // ///     { println!("a_shortunion.get_ssize_({}) = {}", i, a_shortunion.get_ssize_(i)); }
        // /// assert_eq!(a_shortunion.get_ssize_(0), 79__isize);
        // /// assert_eq!(a_shortunion.get_ssize_(1), 11__isize);
        // /// 
        // /// // It will panic.
        // /// // println!("a_shortunion.get_ssize_(2) = {}", a_shortunion.get_ssize_(2));
        // /// ```
        // /// 
        // /// # Example 2 for IntUnion
        // /// ```
        // /// use cryptocol::number::IntUnion;
        // /// let a_intunion = IntUnion::new_with(4048161615_u32);
        // /// for i in 0..4
        // ///     { println!("a_intunion.get_ssize_({}) = {}", i, a_intunion.get_ssize_(i)); }
        // /// assert_eq!(a_intunion.get_ssize_(0), 79__isize);
        // /// assert_eq!(a_intunion.get_ssize_(1), 11__isize);
        // /// assert_eq!(a_intunion.get_ssize_(2), 74__isize);
        // /// assert_eq!(a_intunion.get_ssize_(3), -15__isize);
        // /// 
        // /// // It will panic.
        // /// // println!("a_intunion.get_ssize_(4) = {}", a_intunion.get_ssize_(4));
        // /// ```
        // /// 
        // /// # Example 3 for LongUnion
        // /// ```
        // /// use cryptocol::number::LongUnion;
        // /// let a_longunion = LongUnion::new_with(13664881099896654671_u64);
        // /// for i in 0..8
        // ///     { println!("a_longunion.get_ssize_({}) = {}", i, a_longunion.get_ssize_(i)); }
        // /// assert_eq!(a_longunion.get_ssize_(0), 79__isize);
        // /// assert_eq!(a_longunion.get_ssize_(1), 11__isize);
        // /// assert_eq!(a_longunion.get_ssize_(2), 74__isize);
        // /// assert_eq!(a_longunion.get_ssize_(3), -15__isize);
        // /// assert_eq!(a_longunion.get_ssize_(4), -11__isize);
        // /// assert_eq!(a_longunion.get_ssize_(5), 104__isize);
        // /// assert_eq!(a_longunion.get_ssize_(6), -93__isize);
        // /// assert_eq!(a_longunion.get_ssize_(7), -67__isize);
        // /// 
        // /// // It will panic.
        // /// // println!("a_longunion.get_ssize_(8) = {}", a_longunion.get_ssize_(8));
        // /// ```
        // /// 
        // /// # Example 4 for LongerUnion
        // /// ```
        // /// use cryptocol::number::LongerUnion;
        // /// let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        // /// for i in 0..16
        // ///     { println!("a_longerunion.get_ssize_({}) = {}", i, a_longerunion.get_ssize_(i)); }
        // /// assert_eq!(a_longerunion.get_ssize_(0), 79__isize);
        // /// assert_eq!(a_longerunion.get_ssize_(1), 11__isize);
        // /// assert_eq!(a_longerunion.get_ssize_(2), 74__isize);
        // /// assert_eq!(a_longerunion.get_ssize_(3), -15__isize);
        // /// assert_eq!(a_longerunion.get_ssize_(4), -11__isize);
        // /// assert_eq!(a_longerunion.get_ssize_(5), 104__isize);
        // /// assert_eq!(a_longerunion.get_ssize_(6), -93__isize);
        // /// assert_eq!(a_longerunion.get_ssize_(7), -67__isize);
        // /// assert_eq!(a_longerunion.get_ssize_(8), 88__isize);
        // /// assert_eq!(a_longerunion.get_ssize_(9), -120__isize);
        // /// assert_eq!(a_longerunion.get_ssize_(10), -50__isize);
        // /// assert_eq!(a_longerunion.get_ssize_(11), 126__isize);
        // /// assert_eq!(a_longerunion.get_ssize_(12), 26__isize);
        // /// assert_eq!(a_longerunion.get_ssize_(13), 59__isize);
        // /// assert_eq!(a_longerunion.get_ssize_(14), 18__isize);
        // /// assert_eq!(a_longerunion.get_ssize_(15), -1__isize);
        // /// 
        // /// // It will panic.
        // /// // println!("a_longerunion.get_ssize_(16) = {}", a_longerunion.get_ssize_(16));
        // /// ```
        // /// 
        // /// # Big-endian issue
        // /// It is just experimental for Big Endian CPUs. So, you are not
        // /// encouraged to use it for Big Endian CPUs for serious purpose.
        // /// Only use this crate for Big-endian CPUs with your own full
        // /// responsibility.
        // #[cfg(target_pointer_width = "8")]
        // #[inline] pub fn get_ssize_(&self, i: usize) -> isize
        // {
        //     #[cfg(target_endian = "little")]    unsafe { self.s_size[i] }
        //     #[cfg(target_endian = "big")]       unsafe { self.s_size[Self::J-i] }
        // }

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
        /// # Example 1 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// let a_intunion = IntUnion::new_with(4048161615_u32);
        /// for i in 0..2
        ///     { println!("a_intunion.get_ssize_({}) = {}", i, a_intunion.get_ssize_(i)); }
        /// assert_eq!(a_intunion.get_ssize_(0), 2895_isize);
        /// assert_eq!(a_intunion.get_ssize_(1), -3766_isize);
        /// 
        /// // It will panic.
        /// // println!("a_intunion.get_ssize_(2) = {}", a_intunion.get_ssize_(2));
        /// ```
        /// 
        /// # Example 2 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let a_longunion = LongUnion::new_with(13664881099896654671_u64);
        /// for i in 0..4
        ///     { println!("a_longunion.get_ssize_({}) = {}", i, a_longunion.get_ssize_(i)); }
        /// assert_eq!(a_longunion.get_ssize_(0), 2895_isize);
        /// assert_eq!(a_longunion.get_ssize_(1), -3766_isize);
        /// assert_eq!(a_longunion.get_ssize_(2), 26869_isize);
        /// assert_eq!(a_longunion.get_ssize_(3), -16989_isize);
        /// 
        /// // It will panic.
        /// // println!("a_longunion.get_ssize_(4) = {}", a_longunion.get_ssize_(4));
        /// ```
        /// 
        /// # Example 3 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// for i in 0..8
        ///     { println!("a_longerunion.get_ssize_({}) = {}", i, a_longerunion.get_ssize_(i)); }
        /// assert_eq!(a_longerunion.get_ssize_(0), 2895_isize);
        /// assert_eq!(a_longerunion.get_ssize_(1), -3766_isize);
        /// assert_eq!(a_longerunion.get_ssize_(2), 26869_isize);
        /// assert_eq!(a_longerunion.get_ssize_(3), -16989_isize);
        /// assert_eq!(a_longerunion.get_ssize_(4), -30632_isize);
        /// assert_eq!(a_longerunion.get_ssize_(5), 32462_isize);
        /// assert_eq!(a_longerunion.get_ssize_(6), 15130_isize);
        /// assert_eq!(a_longerunion.get_ssize_(7), -238_isize);
        /// 
        /// // It will panic.
        /// // println!("a_longerunion.get_ssize_(8) = {}", a_longerunion.get_ssize_(8));
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[cfg(target_pointer_width = "16")]
        #[inline] pub fn get_ssize_(&self, i: usize) -> isize
        {
            #[cfg(target_endian = "little")]    unsafe { self.s_size[i] }
            #[cfg(target_endian = "big")]       unsafe { self.s_size[Self::J-i] }
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
        /// # Example 1 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let a_longunion = LongUnion::new_with(13664881099896654671_u64);
        /// for i in 0..2
        ///     { println!("a_longunion.get_ssize_({}) = {}", i, a_longunion.get_ssize_(i)); }
        /// assert_eq!(a_longunion.get_ssize_(0), -246805681_isize);
        /// assert_eq!(a_longunion.get_ssize_(1), -1113364235_isize);
        /// 
        /// // It will panic.
        /// // println!("a_longunion.get_ssize_(2) = {}", a_longunion.get_ssize_(2));
        /// ```
        /// 
        /// # Example 2 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// for i in 0..4
        ///     { println!("a_longerunion.get_ssize_({}) = {}", i, a_longerunion.get_ssize_(i)); }
        /// assert_eq!(a_longerunion.get_ssize_(0), -246805681_isize);
        /// assert_eq!(a_longerunion.get_ssize_(1), -1113364235_isize);
        /// assert_eq!(a_longerunion.get_ssize_(2), 2127464536_isize);
        /// assert_eq!(a_longerunion.get_ssize_(3), -15582438_isize);
        /// 
        /// // It will panic.
        /// // println!("a_longerunion.get_ssize_(4) = {}", a_longerunion.get_ssize_(4));
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[cfg(target_pointer_width = "32")]
        #[inline] pub fn get_ssize_(&self, i: usize) -> isize
        {
            #[cfg(target_endian = "little")]    unsafe { self.s_size[i] }
            #[cfg(target_endian = "big")]       unsafe { self.s_size[Self::J-i] }
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
        /// # Example
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// for i in 0..2
        ///     { println!("a_sizeerunion.get_ssize_({}) = {}", i, a_longerunion.get_ssize_(i)); }
        /// assert_eq!(a_longerunion.get_ssize_(0), -4781862973812896945_isize);
        /// assert_eq!(a_longerunion.get_ssize_(1), -66926059474483112_isize);
        /// 
        /// // It will panic.
        /// // println!("a_sizeerunion.get_ssize_(2) = {}", a_sizeerunion.get_ssize_(2));
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[cfg(target_pointer_width = "64")]
        #[inline] pub fn get_ssize_(&self, i: usize) -> isize
        {
            #[cfg(target_endian = "little")]    unsafe { self.s_size[i] }
            #[cfg(target_endian = "big")]       unsafe { self.s_size[Self::J-i] }
        }

        // // pub fn set_ssize_(&mut self, i: usize, val: isize) 
        // /// Sets i-th element of its array `s_size` of type `isize`
        // /// if `i` is less than the size of this union in bytes divided
        // /// by the size of the type `isize` in bytes.
        // /// Otherwise, it will panic.
        // /// 
        // /// # Arguments
        // /// - `i` indicates `i`-th element. 0-th element contains LSB (Least
        // /// Significant Bit), while (the size of this union in bytes - 1)-th
        // /// element contains MSB (Most Significant Bit) regardless endianness.
        // /// - `val` is the value of type `isize` to set the i-th element of its
        // ///  array `s_size` of type `isize`.
        // /// 
        // /// # Panics
        // /// So, if `i` is greater than or equal to the size of this union
        // /// in bytes divided by the size of the type `isize` in bytes,
        // /// it will panic.
        // /// 
        // /// # Caution
        // /// Use this method only when you are sure that the argument `i` is
        // /// less than the size of this union in bytes divided by the size of
        // /// the type `isize` in bytes
        // /// 
        // /// # Counterpart Method
        // /// It is performance-oriented and does not care for safety.
        // /// It is virtually the same as the method set_ssize(). This method
        // /// set_ssize_() is considered to be slightly faster than the method
        // /// set_ssize().
        // /// Use this method only when you are sure that `i` is less than
        // /// the size of this union in bytes divided by the size of the type
        // /// `isize` in bytes. Otherwise, use its counterpart method
        // /// [set_ssize()](#method.set_ssize) for safety.
        // /// 
        // /// # Example 1 for ShortUnion
        // /// ```
        // /// use cryptocol::number::ShortUnion;
        // /// let mut a_shortunion = ShortUnion::new();
        // /// a_shortunion.set_ssize_(0, 79_isize);
        // /// a_shortunion.set_ssize_(1, 11_isize);
        // /// 
        // /// // It will panic.
        // /// // a_shortunion.set_ssize_(2, 100_isize);
        // /// 
        // /// println!("a_shortunion.get() = {}", a_shortunion.get());
        // /// for i in 0..2
        // ///     { println!("a_shortunion.get_ssize_({}) = {}", i, a_shortunion.get_ssize_(i)); }
        // /// assert_eq!(a_shortunion.get_ssize_(0), 79_isize);
        // /// assert_eq!(a_shortunion.get_ssize_(1), 11_isize);
        // /// assert_eq!(a_shortunion.get(), 2895_u16);
        // /// ```
        // /// 
        // /// # Example 2 for IntUnion
        // /// ```
        // /// use cryptocol::number::IntUnion;
        // /// let mut a_intunion = IntUnion::new();
        // /// a_intunion.set_ssize_(0, 79_isize);
        // /// a_intunion.set_ssize_(1, 11_isize);
        // /// a_intunion.set_ssize_(2, 74_isize);
        // /// a_intunion.set_ssize_(3, -15_isize);
        // /// 
        // /// // It will panic.
        // /// // a_intunion.set_ssize_(4, 100_isize);
        // /// 
        // /// println!("a_intunion.get() = {}", a_intunion.get());
        // /// for i in 0..4
        // ///     { println!("a_intunion.get_ssize_({}) = {}", i, a_intunion.get_ssize_(i)); }
        // /// assert_eq!(a_intunion.get_ssize_(0), 79_isize);
        // /// assert_eq!(a_intunion.get_ssize_(1), 11_isize);
        // /// assert_eq!(a_intunion.get_ssize_(2), 74_isize);
        // /// assert_eq!(a_intunion.get_ssize_(3), -15_isize);
        // /// assert_eq!(a_intunion.get(), 4048161615_u32);
        // /// ```
        // /// 
        // /// # Example 3 for LongUnion
        // /// ```
        // /// use cryptocol::number::LongUnion;
        // /// let mut a_longunion = LongUnion::new();
        // /// a_longunion.set_ssize_(0, 79_isize);
        // /// a_longunion.set_ssize_(1, 11_isize);
        // /// a_longunion.set_ssize_(2, 74_isize);
        // /// a_longunion.set_ssize_(3, -15_isize);
        // /// a_longunion.set_ssize_(4, -11_isize);
        // /// a_longunion.set_ssize_(5, 104_isize);
        // /// a_longunion.set_ssize_(6, -93_isize);
        // /// a_longunion.set_ssize_(7, -67_isize);
        // /// 
        // /// // It will panic.
        // /// // a_intunion.set_ssize_(8, 100_isize);
        // /// 
        // /// println!("a_longunion.get() = {}", a_longunion.get());
        // /// for i in 0..8
        // ///     { println!("a_longunion.get_ssize_({}) = {}", i, a_longunion.get_ssize_(i)); }
        // /// assert_eq!(a_longunion.get_ssize_(0), 79_isize);
        // /// assert_eq!(a_longunion.get_ssize_(1), 11_isize);
        // /// assert_eq!(a_longunion.get_ssize_(2), 74_isize);
        // /// assert_eq!(a_longunion.get_ssize_(3), -15_isize);
        // /// assert_eq!(a_longunion.get_ssize_(4), -11_isize);
        // /// assert_eq!(a_longunion.get_ssize_(5), 104_isize);
        // /// assert_eq!(a_longunion.get_ssize_(6), -93_isize);
        // /// assert_eq!(a_longunion.get_ssize_(7), -67_isize);
        // /// assert_eq!(a_longunion.get(), 13664881099896654671_u64);
        // /// ```
        // /// 
        // /// # Example 4 for LongerUnion
        // /// ```
        // /// use cryptocol::number::LongerUnion;
        // /// let mut a_longerunion = LongerUnion::new();
        // /// a_longerunion.set_ssize_(0, 79_isize);
        // /// a_longerunion.set_ssize_(1, 11_isize);
        // /// a_longerunion.set_ssize_(2, 74_isize);
        // /// a_longerunion.set_ssize_(3, -15_isize);
        // /// a_longerunion.set_ssize_(4, -11_isize);
        // /// a_longerunion.set_ssize_(5, 104_isize);
        // /// a_longerunion.set_ssize_(6, -93_isize);
        // /// a_longerunion.set_ssize_(7, -67_isize);
        // /// a_longerunion.set_ssize_(8, 88_isize);
        // /// a_longerunion.set_ssize_(9, -120_isize);
        // /// a_longerunion.set_ssize_(10, -50_isize);
        // /// a_longerunion.set_ssize_(11, 126_isize);
        // /// a_longerunion.set_ssize_(12, 26_isize);
        // /// a_longerunion.set_ssize_(13, 59_isize);
        // /// a_longerunion.set_ssize_(14, 18_isize);
        // /// a_longerunion.set_ssize_(15, -1_isize);
        // /// 
        // /// // It will panic.
        // /// // a_longerunion.set_ssize_(16, 100_isize);
        // /// 
        // /// println!("a_longerunion.get() = {}", a_longerunion.get());
        // /// for i in 0..16
        // ///     { println!("a_longerunion.get_ssize_({}) = {}", i, a_longerunion.get_ssize_(i)); }
        // /// assert_eq!(a_longerunion.get_ssize_(0), 79_isize);
        // /// assert_eq!(a_longerunion.get_ssize_(1), 11_isize);
        // /// assert_eq!(a_longerunion.get_ssize_(2), 74_isize);
        // /// assert_eq!(a_longerunion.get_ssize_(3), -15_isize);
        // /// assert_eq!(a_longerunion.get_ssize_(4), -11_isize);
        // /// assert_eq!(a_longerunion.get_ssize_(5), 104_isize);
        // /// assert_eq!(a_longerunion.get_ssize_(6), -93_isize);
        // /// assert_eq!(a_longerunion.get_ssize_(7), -67_isize);
        // /// assert_eq!(a_longerunion.get_ssize_(8), 88_isize);
        // /// assert_eq!(a_longerunion.get_ssize_(9), -120_isize);
        // /// assert_eq!(a_longerunion.get_ssize_(10), -50_isize);
        // /// assert_eq!(a_longerunion.get_ssize_(11), 126_isize);
        // /// assert_eq!(a_longerunion.get_ssize_(12), 26_isize);
        // /// assert_eq!(a_longerunion.get_ssize_(13), 59_isize);
        // /// assert_eq!(a_longerunion.get_ssize_(14), 18_isize);
        // /// assert_eq!(a_longerunion.get_ssize_(15), -1_isize);
        // /// assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_usize);
        // /// ```
        // /// 
        // /// # Big-endian issue
        // /// It is just experimental for Big Endian CPUs. So, you are not
        // /// encouraged to use it for Big Endian CPUs for serious purpose.
        // /// Only use this crate for Big-endian CPUs with your own full
        // /// responsibility.
        // #[cfg(target_pointer_width = "8")]
        // #[inline] pub fn set_ssize_(&mut self, i: usize, val: isize) 
        // {
        //     #[cfg(target_endian = "little")]    unsafe { self.s_size[i] = val; }
        //     #[cfg(target_endian = "big")]       unsafe { self.s_size[Self::J-i] = val; }
        // }

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
        /// # Example 1 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// let mut a_intunion = IntUnion::new();
        /// a_intunion.set_ssize_(0, 2895_isize);
        /// a_intunion.set_ssize_(1, -3766_isize);
        /// 
        /// // It will panic.
        /// // a_intunion.set_ssize_(2, 100_isize);
        /// 
        /// println!("a_intunion.get() = {}", a_intunion.get());
        /// for i in 0..2
        ///     { println!("a_intunion.get_ssize_({}) = {}", i, a_intunion.get_ssize_(i)); }
        /// assert_eq!(a_intunion.get_ssize_(0), 2895_isize);
        /// assert_eq!(a_intunion.get_ssize_(1), -3766_isize);
        /// assert_eq!(a_intunion.get(), 4048161615_u32);
        /// ```
        /// 
        /// # Example 2 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let mut a_longunion = LongUnion::new();
        /// a_longunion.set_ssize_(0, 2895_isize);
        /// a_longunion.set_ssize_(1, -3766_isize);
        /// a_longunion.set_ssize_(2, 26869_isize);
        /// a_longunion.set_ssize_(3, -16989_isize);
        /// 
        /// // It will panic.
        /// // a_longunion.set_ssize_(4, 100_isize);
        /// 
        /// for i in 0..4
        ///     { println!("a_longunion.get_ssize_({}) = {}", i, a_longunion.get_ssize_(i)); }
        /// assert_eq!(a_longunion.get_ssize_(0), 2895_isize);
        /// assert_eq!(a_longunion.get_ssize_(1), -3766_isize);
        /// assert_eq!(a_longunion.get_ssize_(2), 26869_isize);
        /// assert_eq!(a_longunion.get_ssize_(3), -16989_isize);
        /// assert_eq!(a_longunion.get(), 13664881099896654671_u64);
        /// ```
        /// 
        /// # Example 3 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let mut a_longerunion = LongerUnion::new();
        /// a_longerunion.set_ssize_(0, 2895_isize);
        /// a_longerunion.set_ssize_(1, -3766_isize);
        /// a_longerunion.set_ssize_(2, 26869_isize);
        /// a_longerunion.set_ssize_(3, -16989_isize);
        /// a_longerunion.set_ssize_(4, -30632_isize);
        /// a_longerunion.set_ssize_(5, 32462_isize);
        /// a_longerunion.set_ssize_(6, 15130_isize);
        /// a_longerunion.set_ssize_(7, -238_isize);
        /// 
        /// // It will panic.
        /// // a_longerunion.set_ssize_(8, 100_isize);
        /// 
        /// for i in 0..8
        ///     { println!("a_longerunion.get_ssize_({}) = {}", i, a_longerunion.get_ssize_(i)); }
        /// assert_eq!(a_longerunion.get_ssize_(0), 2895_isize);
        /// assert_eq!(a_longerunion.get_ssize_(1), -3766_isize);
        /// assert_eq!(a_longerunion.get_ssize_(2), 26869_isize);
        /// assert_eq!(a_longerunion.get_ssize_(3), -16989_isize);
        /// assert_eq!(a_longerunion.get_ssize_(4), -30632_isize);
        /// assert_eq!(a_longerunion.get_ssize_(5), 32462_isize);
        /// assert_eq!(a_longerunion.get_ssize_(6), 15130_isize);
        /// assert_eq!(a_longerunion.get_ssize_(7), -238_isize);
        /// assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[cfg(target_pointer_width = "16")]
        #[inline] pub fn set_ssize_(&mut self, i: usize, val: isize) 
        {
            #[cfg(target_endian = "little")]    unsafe { self.s_size[i] = val; }
            #[cfg(target_endian = "big")]       unsafe { self.s_size[Self::J-i] = val; }
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
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let mut a_longunion = LongUnion::new();
        /// a_longunion.set_ssize_(0, -246805681_isize);
        /// a_longunion.set_ssize_(1, -1113364235_isize);
        /// 
        /// // It will panic.
        /// // a_longunion.set_ssize_(2, 100_isize);
        /// 
        /// for i in 0..2
        ///     { println!("a_longunion.get_ssize_({}) = {}", i, a_longunion.get_ssize_(i)); }
        /// assert_eq!(a_longunion.get_ssize_(0), -246805681_isize);
        /// assert_eq!(a_longunion.get_ssize_(1), -1113364235_isize);
        /// assert_eq!(a_longunion.get(), 13664881099896654671_u64);
        /// ```
        /// 
        /// # Example 2 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let mut a_longerunion = LongerUnion::new();
        /// a_longerunion.set_ssize_(0, -246805681_isize);
        /// a_longerunion.set_ssize_(1, -1113364235_isize);
        /// a_longerunion.set_ssize_(2, 2127464536_isize);
        /// a_longerunion.set_ssize_(3, -15582438_isize);
        /// 
        /// // It will panic.
        /// // a_longerunion.set_usize_(4, 100_usize);
        /// 
        /// for i in 0..4
        ///     { println!("a_longerunion.get_usize_({}) = {}", i, a_longerunion.get_usize_(i)); }
        /// assert_eq!(a_longerunion.get_ssize_(0), -246805681_isize);
        /// assert_eq!(a_longerunion.get_ssize_(1), -1113364235_isize);
        /// assert_eq!(a_longerunion.get_ssize_(2), 2127464536_isize);
        /// assert_eq!(a_longerunion.get_ssize_(3), -15582438_isize);
        /// assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[cfg(target_pointer_width = "32")]
        #[inline] pub fn set_ssize_(&mut self, i: usize, val: isize) 
        {
            #[cfg(target_endian = "little")]    unsafe { self.s_size[i] = val; }
            #[cfg(target_endian = "big")]       unsafe { self.s_size[Self::J-i] = val; }
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
        /// # Example
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let mut a_longerunion = LongerUnion::new();
        /// a_longerunion.set_ssize_(0, -4781862973812896945_isize);
        /// a_longerunion.set_ssize_(1, -66926059474483112_isize);
        /// 
        /// // It will panic.
        /// // a_longerunion.set_ssize_(2, 100_isize);
        /// 
        /// for i in 0..2
        ///     { println!("a_longerunion.get_ssize_({}) = {}", i, a_longerunion.get_ssize_(i)); }
        /// assert_eq!(a_longerunion.get_ssize_(0), -4781862973812896945_isize);
        /// assert_eq!(a_longerunion.get_ssize_(1), -66926059474483112_isize);
        /// assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[cfg(target_pointer_width = "64")]
        #[inline] pub fn set_ssize_(&mut self, i: usize, val: isize) 
        {
            #[cfg(target_endian = "little")]    unsafe { self.s_size[i] = val; }
            #[cfg(target_endian = "big")]       unsafe { self.s_size[Self::J-i] = val; }
        }

        // // pub fn get_usize(&self, i: usize) -> Option<usize>
        // /// Returns i-th element of array `u_size` of type `usize` wrapped
        // /// in Some of enum `Option` if `i` is less than the size of this union
        // /// in bytes divided by the size of the type `usize` in bytes.
        // /// Otherwise, it returns `None` of enum `Option`.
        // /// 
        // /// # Arguments
        // /// `i` indicates `i`-th element. 0-th element contains LSB (Least
        // /// Significant Bit), while (the size of this union in bytes - 1)-th
        // /// element contains MSB (Most Significant Bit) regardless endianness.
        // /// 
        // /// # Output
        // /// - i-th element of array `u_size` of type `usize` wrapped
        // /// in Some of enum `Option` if `i` is less than the size of this
        // /// union in bytes divided by the size of the type `usize` in bytes
        // /// - `None` if `i` is greater than or equal to the size of this
        // /// union in bytes divided by the size of the type `usize` in bytes
        // /// 
        // /// # Counterpart Method
        // /// Use this method when you are not sure that `i` is less than
        // /// the size of this union in bytes divided by the size of the type
        // /// `usize` in bytes. Otherwise, you can use its counterpart method
        // /// [get_usize_()](#method.get_usize_) for performance.
        // /// 
        // /// # Example 1 for ShortUnion
        // /// ```
        // /// use cryptocol::number::ShortUnion;
        // /// let a_shortunion = ShortUnion::new_with(2895_u16);
        // /// for i in 0..2
        // /// {
        // ///     match a_shortunion.get_usize(i)
        // ///     {
        // ///         Some(a) => { println!("a_shortunion.get_usize({}) = {}", i, a); },
        // ///         _ => {},
        // ///     }
        // /// }
        // /// assert_eq!(a_shortunion.get_usize(0), Some(79_usize));
        // /// assert_eq!(a_shortunion.get_usize(1), Some(11_usize));
        // /// assert_eq!(a_shortunion.get_usize(2), None);
        // /// ```
        // /// 
        // /// # Example 2 for IntUnion
        // /// ```
        // /// use cryptocol::number::IntUnion;
        // /// let a_intunion = IntUnion::new_with(4048161615_u32);
        // /// for i in 0..4
        // /// {
        // ///     match a_intunion.get_usize(i)
        // ///     {
        // ///         Some(a) => { println!("a_intunion.get_usize({}) = {}", i, a); },
        // ///         _ => {},
        // ///     }
        // /// }
        // /// assert_eq!(a_intunion.get_usize(0), Some(79_usize));
        // /// assert_eq!(a_intunion.get_usize(1), Some(11_usize));
        // /// assert_eq!(a_intunion.get_usize(2), Some(74_usize));
        // /// assert_eq!(a_intunion.get_usize(3), Some(241_usize));
        // /// assert_eq!(a_intunion.get_usize(4), None);
        // /// ```
        // /// 
        // /// # Example 3 for LongUnion
        // /// ```
        // /// use cryptocol::number::LongUnion;
        // /// let a_longunion = LongUnion::new_with(13664881099896654671_u64);
        // /// for i in 0..8
        // /// {
        // ///     match a_longunion.get_usize(i)
        // ///     {
        // ///         Some(a) => { println!("a_longunion.get_usize({}) = {}", i, a); },
        // ///         _ => {},
        // ///     }
        // /// }
        // /// assert_eq!(a_longunion.get_usize(0), Some(79_usize));
        // /// assert_eq!(a_longunion.get_usize(1), Some(11_usize));
        // /// assert_eq!(a_longunion.get_usize(2), Some(74_usize));
        // /// assert_eq!(a_longunion.get_usize(3), Some(241_usize));
        // /// assert_eq!(a_longunion.get_usize(4), Some(245_usize));
        // /// assert_eq!(a_longunion.get_usize(5), Some(104_usize));
        // /// assert_eq!(a_longunion.get_usize(6), Some(163_usize));
        // /// assert_eq!(a_longunion.get_usize(7), Some(189_usize));
        // /// assert_eq!(a_longunion.get_usize(8), None);
        // /// ```
        // /// 
        // /// # Example 4 for LongerUnion
        // /// ```
        // /// use cryptocol::number::LongerUnion;
        // /// let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_usize);
        // /// for i in 0..16
        // /// {
        // ///     match a_longerunion.get_usize(i)
        // ///     {
        // ///         Some(a) => { println!("a_longerunion.get_usize({}) = {}", i, a); },
        // ///         _ => {},
        // ///     }
        // /// }
        // /// assert_eq!(a_longerunion.get_usize(0), Some(79_usize));
        // /// assert_eq!(a_longerunion.get_usize(1), Some(11_usize));
        // /// assert_eq!(a_longerunion.get_usize(2), Some(74_usize));
        // /// assert_eq!(a_longerunion.get_usize(3), Some(241_usize));
        // /// assert_eq!(a_longerunion.get_usize(4), Some(245_usize));
        // /// assert_eq!(a_longerunion.get_usize(5), Some(104_usize));
        // /// assert_eq!(a_longerunion.get_usize(6), Some(163_usize));
        // /// assert_eq!(a_longerunion.get_usize(7), Some(189_usize));
        // /// assert_eq!(a_longerunion.get_usize(8), Some(88_usize));
        // /// assert_eq!(a_longerunion.get_usize(9), Some(136_usize));
        // /// assert_eq!(a_longerunion.get_usize(10), Some(206_usize));
        // /// assert_eq!(a_longerunion.get_usize(11), Some(126_usize));
        // /// assert_eq!(a_longerunion.get_usize(12), Some(26_usize));
        // /// assert_eq!(a_longerunion.get_usize(13), Some(59_usize));
        // /// assert_eq!(a_longerunion.get_usize(14), Some(18_usize));
        // /// assert_eq!(a_longerunion.get_usize(15), Some(255_usize));
        // /// assert_eq!(a_longerunion.get_usize(16), None);
        // /// ```
        // /// 
        // /// # Big-endian issue
        // /// It is just experimental for Big Endian CPUs. So, you are not
        // /// encouraged to use it for Big Endian CPUs for serious purpose.
        // /// Only use this crate for Big-endian CPUs with your own full
        // /// responsibility.
        // #[cfg(target_pointer_width = "8")]
        // pub fn get_usize(&self, i: usize) -> Option<usize>
        // {
        //     if i <= Self::J
        //     {
        //         #[cfg(target_endian = "little")]    unsafe { Some(self.u_size[i]) }
        //         #[cfg(target_endian = "big")]       unsafe { Some(self.u_size[Self::J-i]) }
        //     }
        //     else
        //     {
        //         None
        //     }
        // }

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
        /// # Example 1 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// let a_intunion = IntUnion::new_with(4048161615_u32);
        /// for i in 0..2
        /// {
        ///     match a_intunion.get_usize(i)
        ///     {
        ///         Some(a) => { println!("a_intunion.get_usize({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_intunion.get_usize(0), Some(2895_usize));
        /// assert_eq!(a_intunion.get_usize(1), Some(61770_usize));
        /// assert_eq!(a_intunion.get_usize(2), None);
        /// ```
        /// 
        /// # Example 2 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let a_longunion = LongUnion::new_with(13664881099896654671_u64);
        /// for i in 0..4
        /// {
        ///     match a_longunion.get_usize(i)
        ///     {
        ///         Some(a) => { println!("a_longunion.get_usize({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longunion.get_usize(0), Some(2895_usize));
        /// assert_eq!(a_longunion.get_usize(1), Some(61770_usize));
        /// assert_eq!(a_longunion.get_usize(2), Some(26869_usize));
        /// assert_eq!(a_longunion.get_usize(3), Some(48547_usize));
        /// assert_eq!(a_longunion.get_usize(4), None);
        /// ```
        /// 
        /// # Example 3 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// for i in 0..8
        /// {
        ///     match a_longerunion.get_usize(i)
        ///     {
        ///         Some(a) => { println!("a_longerunion.get_usize({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longerunion.get_usize(0), Some(2895_usize));
        /// assert_eq!(a_longerunion.get_usize(1), Some(61770_usize));
        /// assert_eq!(a_longerunion.get_usize(2), Some(26869_usize));
        /// assert_eq!(a_longerunion.get_usize(3), Some(48547_usize));
        /// assert_eq!(a_longerunion.get_usize(4), Some(34904_usize));
        /// assert_eq!(a_longerunion.get_usize(5), Some(32462_usize));
        /// assert_eq!(a_longerunion.get_usize(6), Some(15130_usize));
        /// assert_eq!(a_longerunion.get_usize(7), Some(65298_usize));
        /// assert_eq!(a_longerunion.get_usize(8), None);
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[cfg(target_pointer_width = "16")]
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
        /// # Example 1 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let a_longunion = LongUnion::new_with(13664881099896654671_u64);
        /// for i in 0..2
        /// {
        ///     match a_longunion.get_usize(i)
        ///     {
        ///         Some(a) => { println!("a_longunion.get_usize({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longunion.get_usize(0), Some(4048161615_usize));
        /// assert_eq!(a_longunion.get_usize(1), Some(3181603061_usize));
        /// assert_eq!(a_longunion.get_usize(2), None);
        /// ```
        /// 
        /// # Example 2 for LongerUnion
        /// use cryptocol::number::LongerUnion;
        /// let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// for i in 0..4
        /// {
        ///     match a_longerunion.get_usize(i)
        ///     {
        ///         Some(a) => { println!("a_longerunion.get_usize({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longerunion.get_usize(0), Some(4048161615_usize));
        /// assert_eq!(a_longerunion.get_usize(1), Some(3181603061_usize));
        /// assert_eq!(a_longerunion.get_usize(2), Some(2127464536_usize));
        /// assert_eq!(a_longerunion.get_usize(3), Some(4279384858_usize));
        /// assert_eq!(a_longerunion.get_usize(4), None);
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[cfg(target_pointer_width = "32")]
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
        /// # Example
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// for i in 0..2
        /// {
        ///     match a_longerunion.get_usize(i)
        ///     {
        ///         Some(a) => { println!("a_longerunion.get_usize({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longerunion.get_usize(0), Some(13664881099896654671_usize));
        /// assert_eq!(a_longerunion.get_usize(1), Some(18379818014235068504_usize));
        /// assert_eq!(a_longerunion.get_usize(4), None);
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[cfg(target_pointer_width = "64")]
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

        // // pub fn set_usize(&mut self, i: usize, val:usize) -> bool
        // /// Sets i-th element of its array `u_size` of type `usize` and returns
        // /// true if `i` is less than the size of this union in bytes divided
        // /// by the size of the type `usize` in bytes.
        // /// Otherwise, it will set nothing amd return false.
        // /// 
        // /// # Arguments
        // /// - `i` indicates `i`-th element. 0-th element contains LSB (Least
        // /// Significant Bit), while (the size of this union in bytes - 1)-th
        // /// element contains MSB (Most Significant Bit) regardless endianness.
        // /// - `val` is the value of type `usize` to set the i-th element of its
        // ///  array `u_size` of type `usize`.
        // /// 
        // /// # Output
        // /// - `true` if `i` is less than the size of this union in bytes
        // /// divided by the size of the type `usize` in bytes
        // /// - `false` if `i` is equal to or greater than the size of this
        // /// union in bytes divided by the size of the type `usize` in bytes
        // /// 
        // /// # Counterpart Method
        // /// Use this method when you are not sure that `i` is less than
        // /// the size of this union in bytes divided by the size of the type
        // /// `usize` in bytes. Otherwise, you can use its counterpart
        // /// method [set_usize_()](#method.set_usize_) for performance.
        // /// 
        // /// # Example 1 for ShortUnion
        // /// ```
        // /// use cryptocol::number::ShortUnion;
        // /// let mut a_shortunion = ShortUnion::new();
        // /// let b0 = a_shortunion.set_usize(0, 79_usize);
        // /// let b1 = a_shortunion.set_usize(1, 11_usize);
        // /// let b2 = a_shortunion.set_usize(2, 100_usize);  // Nothing will be done
        // /// assert_eq!(b0, true);
        // /// assert_eq!(b1, true);
        // /// assert_eq!(b2, false);
        // /// println!("a_shortunion.get() = {}", a_shortunion.get());
        // /// for i in 0..2
        // /// {
        // ///     match a_shortunion.get_usize(i)
        // ///     {
        // ///         Some(a) => { println!("a_shortunion.get_usize({}) = {}", i, a); },
        // ///         _ => {},
        // ///     }
        // /// }
        // /// assert_eq!(a_shortunion.get_usize(0), Some(79_usize));
        // /// assert_eq!(a_shortunion.get_usize(1), Some(11_usize));
        // /// assert_eq!(a_shortunion.get(), 2895_u16);
        // /// ```
        // /// 
        // /// # Example 2 for IntUnion
        // /// ```
        // /// use cryptocol::number::IntUnion;
        // /// let mut a_intunion = IntUnion::new();
        // /// let b0 = a_intunion.set_usize(0, 79_usize);
        // /// let b1 = a_intunion.set_usize(1, 11_usize);
        // /// let b2 = a_intunion.set_usize(2, 74_usize);
        // /// let b3 = a_intunion.set_usize(3, 241_usize);
        // /// let b4 = a_intunion.set_usize(4, 100_usize);
        // /// assert_eq!(b0, true);
        // /// assert_eq!(b1, true);
        // /// assert_eq!(b2, true);
        // /// assert_eq!(b3, true);
        // /// assert_eq!(b4, false);
        // /// println!("a_intunion.get() = {}", a_intunion.get());
        // /// for i in 0..4
        // /// {
        // ///     match a_intunion.get_usize(i)
        // ///     {
        // ///         Some(a) => { println!("a_intunion.get_usize({}) = {}", i, a); },
        // ///         _ => {},
        // ///     }
        // /// }
        // /// assert_eq!(a_intunion.get_usize(0), Some(79_usize));
        // /// assert_eq!(a_intunion.get_usize(1), Some(11_usize));
        // /// assert_eq!(a_intunion.get_usize(2), Some(74_usize));
        // /// assert_eq!(a_intunion.get_usize(3), Some(241_usize));
        // /// assert_eq!(a_intunion.get(), 4048161615_u32);
        // /// ```
        // /// 
        // /// # Example 3 for LongUnion
        // /// ```
        // /// use cryptocol::number::LongUnion;
        // /// let mut a_longunion = LongUnion::new();
        // /// let b0 = a_longunion.set_usize(0, 79_usize);
        // /// let b1 = a_longunion.set_usize(1, 11_usize);
        // /// let b2 = a_longunion.set_usize(2, 74_usize);
        // /// let b3 = a_longunion.set_usize(3, 241_usize);
        // /// let b4 = a_longunion.set_usize(4, 245_usize);
        // /// let b5 = a_longunion.set_usize(5, 104_usize);
        // /// let b6 = a_longunion.set_usize(6, 163_usize);
        // /// let b7 = a_longunion.set_usize(7, 189_usize);
        // /// let b8 = a_longunion.set_usize(8, 100_usize);
        // /// assert_eq!(b0, true);
        // /// assert_eq!(b1, true);
        // /// assert_eq!(b2, true);
        // /// assert_eq!(b3, true);
        // /// assert_eq!(b4, true);
        // /// assert_eq!(b5, true);
        // /// assert_eq!(b6, true);
        // /// assert_eq!(b7, true);
        // /// assert_eq!(b8, false);
        // /// println!("a_longunion.get() = {}", a_longunion.get());
        // /// for i in 0..8
        // /// {
        // ///     match a_longunion.get_usize(i)
        // ///     {
        // ///         Some(a) => { println!("a_longunion.get_usize({}) = {}", i, a); },
        // ///         _ => {},
        // ///     }
        // /// }
        // /// assert_eq!(a_longunion.get_usize(0), Some(79_usize));
        // /// assert_eq!(a_longunion.get_usize(1), Some(11_usize));
        // /// assert_eq!(a_longunion.get_usize(2), Some(74_usize));
        // /// assert_eq!(a_longunion.get_usize(3), Some(241_usize));
        // /// assert_eq!(a_longunion.get_usize(4), Some(245_usize));
        // /// assert_eq!(a_longunion.get_usize(5), Some(104_usize));
        // /// assert_eq!(a_longunion.get_usize(6), Some(163_usize));
        // /// assert_eq!(a_longunion.get_usize(7), Some(189_usize));
        // /// assert_eq!(a_longunion.get(), 13664881099896654671_u64);
        // /// ```
        // /// 
        // /// # Example 4 for LongerUnion
        // /// ```
        // /// use cryptocol::number::LongerUnion;
        // /// let mut a_longerunion = LongerUnion::new();
        // /// let b0 = a_longerunion.set_usize(0, 79_usize);
        // /// let b1 = a_longerunion.set_usize(1, 11_usize);
        // /// let b2 = a_longerunion.set_usize(2, 74_usize);
        // /// let b3 = a_longerunion.set_usize(3, 241_usize);
        // /// let b4 = a_longerunion.set_usize(4, 245_usize);
        // /// let b5 = a_longerunion.set_usize(5, 104_usize);
        // /// let b6 = a_longerunion.set_usize(6, 163_usize);
        // /// let b7 = a_longerunion.set_usize(7, 189_usize);
        // /// let b8 = a_longerunion.set_usize(8, 88_usize);
        // /// let b9 = a_longerunion.set_usize(9, 136_usize);
        // /// let b10 = a_longerunion.set_usize(10, 206_usize);
        // /// let b11 = a_longerunion.set_usize(11, 126_usize);
        // /// let b12 = a_longerunion.set_usize(12, 26_usize);
        // /// let b13 = a_longerunion.set_usize(13, 59_usize);
        // /// let b14 = a_longerunion.set_usize(14, 18_usize);
        // /// let b15 = a_longerunion.set_usize(15, 255_usize);
        // /// let b16 = a_longerunion.set_usize(16, 100_usize);
        // /// assert_eq!(b0, true);
        // /// assert_eq!(b1, true);
        // /// assert_eq!(b2, true);
        // /// assert_eq!(b3, true);
        // /// assert_eq!(b4, true);
        // /// assert_eq!(b5, true);
        // /// assert_eq!(b6, true);
        // /// assert_eq!(b7, true);
        // /// assert_eq!(b8, true);
        // /// assert_eq!(b9, true);
        // /// assert_eq!(b10, true);
        // /// assert_eq!(b11, true);
        // /// assert_eq!(b12, true);
        // /// assert_eq!(b13, true);
        // /// assert_eq!(b14, true);
        // /// assert_eq!(b15, true);
        // /// assert_eq!(b16, false);
        // /// println!("a_longerunion.get() = {}", a_longerunion.get());
        // /// for i in 0..16
        // /// {
        // ///     match a_longerunion.get_usize(i)
        // ///     {
        // ///         Some(a) => { println!("a_longerunion.get_usize({}) = {}", i, a); },
        // ///         _ => {},
        // ///     }
        // /// }
        // /// assert_eq!(a_longerunion.get_usize(0), Some(79_usize));
        // /// assert_eq!(a_longerunion.get_usize(1), Some(11_usize));
        // /// assert_eq!(a_longerunion.get_usize(2), Some(74_usize));
        // /// assert_eq!(a_longerunion.get_usize(3), Some(241_usize));
        // /// assert_eq!(a_longerunion.get_usize(4), Some(245_usize));
        // /// assert_eq!(a_longerunion.get_usize(5), Some(104_usize));
        // /// assert_eq!(a_longerunion.get_usize(6), Some(163_usize));
        // /// assert_eq!(a_longerunion.get_usize(7), Some(189_usize));
        // /// assert_eq!(a_longerunion.get_usize(8), Some(88_usize));
        // /// assert_eq!(a_longerunion.get_usize(9), Some(136_usize));
        // /// assert_eq!(a_longerunion.get_usize(10), Some(206_usize));
        // /// assert_eq!(a_longerunion.get_usize(11), Some(126_usize));
        // /// assert_eq!(a_longerunion.get_usize(12), Some(26_usize));
        // /// assert_eq!(a_longerunion.get_usize(13), Some(59_usize));
        // /// assert_eq!(a_longerunion.get_usize(14), Some(18_usize));
        // /// assert_eq!(a_longerunion.get_usize(15), Some(255_usize));
        // /// assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_usize);
        // /// ```
        // /// 
        // /// # Big-endian issue
        // /// It is just experimental for Big Endian CPUs. So, you are not
        // /// encouraged to use it for Big Endian CPUs for serious purpose.
        // /// Only use this crate for Big-endian CPUs with your own full
        // /// responsibility.
        // #[cfg(target_pointer_width = "8")]
        // pub fn set_usize(&mut self, i: usize, val:usize) -> bool
        // {
        //     if i <= Self::J
        //     { 
        //         #[cfg(target_endian = "little")]    unsafe { self.u_size[i] = val; }
        //         #[cfg(target_endian = "big")]       unsafe { self.u_size[Self::J-i] = val; }
        //         true
        //     }
        //     else
        //     {
        //         false
        //     }
        // }

        // pub fn set_usize(&mut self, i: usize, val:usize) -> bool
        /// Sets i-th element of its array `u_size` of type `usize` and returns
        /// true if `i` is less than the size of this union in bytes divided
        /// by the size of the type `usize` in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Arguments
        /// - `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// - `val` is the value of type `usize` to set the i-th element of its
        ///  array `u_size` of type `usize`.
        /// 
        /// # Output
        /// - `true` if `i` is less than the size of this union in bytes
        /// divided by the size of the type `usize` in bytes
        /// - `false` if `i` is equal to or greater than the size of this
        /// union in bytes divided by the size of the type `usize` in bytes
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// the size of this union in bytes divided by the size of the type
        /// `usize` in bytes. Otherwise, you can use its counterpart
        /// method [set_usize_()](#method.set_usize_) for performance.
        /// 
        /// # Example 1 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// let mut a_intunion = IntUnion::new();
        /// let b0 = a_intunion.set_usize(0, 2895_usize);
        /// let b1 = a_intunion.set_usize(1, 61770_usize);
        /// let b2 = a_intunion.set_usize(2, 100_usize);
        /// assert_eq!(b0, true);
        /// assert_eq!(b1, true);
        /// assert_eq!(b2, false);
        /// println!("a_intunion.get() = {}", a_intunion.get());
        /// for i in 0..2
        /// {
        ///     match a_intunion.get_usize(i)
        ///     {
        ///         Some(a) => { println!("a_intunion.get_usize({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_intunion.get_usize(0), Some(2895_usize));
        /// assert_eq!(a_intunion.get_usize(1), Some(61770_usize));
        /// assert_eq!(a_intunion.get(), 4048161615_u32);
        /// ```
        /// 
        /// # Example 2 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let mut a_longunion = LongUnion::new();
        /// let b0 = a_longunion.set_usize(0, 2895_usize);
        /// let b1 = a_longunion.set_usize(1, 61770_usize);
        /// let b2 = a_longunion.set_usize(2, 26869_usize);
        /// let b3 = a_longunion.set_usize(3, 48547_usize);
        /// let b4 = a_longunion.set_usize(4, 100_usize);
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
        /// assert_eq!(a_longunion.get_usize(0), Some(2895_usize));
        /// assert_eq!(a_longunion.get_usize(1), Some(61770_usize));
        /// assert_eq!(a_longunion.get_usize(2), Some(26869_usize));
        /// assert_eq!(a_longunion.get_usize(3), Some(48547_usize));
        /// assert_eq!(a_longunion.get(), 13664881099896654671_u64);
        /// ```
        /// 
        /// # Example 3 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let mut a_longerunion = LongerUnion::new();
        /// let b0 = a_longerunion.set_usize(0, 2895_usize);
        /// let b1 = a_longerunion.set_usize(1, 61770_usize);
        /// let b2 = a_longerunion.set_usize(2, 26869_usize);
        /// let b3 = a_longerunion.set_usize(3, 48547_usize);
        /// let b4 = a_longerunion.set_usize(4, 34904_usize);
        /// let b5 = a_longerunion.set_usize(5, 32462_usize);
        /// let b6 = a_longerunion.set_usize(6, 15130_usize);
        /// let b7 = a_longerunion.set_usize(7, 65298_usize);
        /// let b8 = a_longerunion.set_usize(8, 100_usize);
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
        ///     match a_longerunion.get_usize(i)
        ///     {
        ///         Some(a) => { println!("a_longerunion.get_usize({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longerunion.get_usize(0), Some(2895_usize));
        /// assert_eq!(a_longerunion.get_usize(1), Some(61770_usize));
        /// assert_eq!(a_longerunion.get_usize(2), Some(26869_usize));
        /// assert_eq!(a_longerunion.get_usize(3), Some(48547_usize));
        /// assert_eq!(a_longerunion.get_usize(4), Some(34904_usize));
        /// assert_eq!(a_longerunion.get_usize(5), Some(32462_usize));
        /// assert_eq!(a_longerunion.get_usize(6), Some(15130_usize));
        /// assert_eq!(a_longerunion.get_usize(7), Some(65298_usize));
        /// assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[cfg(target_pointer_width = "16")]
        pub fn set_usize(&mut self, i: usize, val:usize) -> bool
        {
            if i <= Self::J
            { 
                #[cfg(target_endian = "little")]    unsafe { self.u_size[i] = val; }
                #[cfg(target_endian = "big")]       unsafe { self.u_size[Self::J-i] = val; }
                true
            }
            else
            {
                false
            }
        }

        // pub fn set_usize(&mut self, i: usize, val:usize) -> bool
        /// Sets i-th element of its array `u_size` of type `usize` and returns
        /// true if `i` is less than the size of this union in bytes divided
        /// by the size of the type `usize` in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Arguments
        /// - `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// - `val` is the value of type `usize` to set the i-th element of its
        ///  array `u_size` of type `usize`.
        /// 
        /// # Output
        /// - `true` if `i` is less than the size of this union in bytes
        /// divided by the size of the type `usize` in bytes
        /// - `false` if `i` is equal to or greater than the size of this
        /// union in bytes divided by the size of the type `usize` in bytes
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// the size of this union in bytes divided by the size of the type
        /// `usize` in bytes. Otherwise, you can use its counterpart
        /// method [set_usize_()](#method.set_usize_) for performance.
        /// 
        /// # Example 1 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let mut a_longunion = LongUnion::new();
        /// let b0 = a_longunion.set_usize(0, 4048161615_usize);
        /// let b1 = a_longunion.set_usize(1, 3181603061_usize);
        /// let b2 = a_longunion.set_usize(2, 100_usize);
        /// assert_eq!(b0, true);
        /// assert_eq!(b1, true);
        /// assert_eq!(b2, false);
        /// for i in 0..2
        /// {
        ///     match a_longunion.get_usize(i)
        ///     {
        ///         Some(a) => { println!("a_longunion.get_usize({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longunion.get_usize(0), Some(4048161615_usize));
        /// assert_eq!(a_longunion.get_usize(1), Some(3181603061_usize));
        /// assert_eq!(a_longunion.get(), 13664881099896654671_u64);
        /// ```
        /// 
        /// # Example 2 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let mut a_longerunion = LongerUnion::new();
        /// let b0 = a_longerunion.set_usize(0, 4048161615_usize);
        /// let b1 = a_longerunion.set_usize(1, 3181603061_usize);
        /// let b2 = a_longerunion.set_usize(2, 2127464536_usize);
        /// let b3 = a_longerunion.set_usize(3, 4279384858_usize);
        /// let b4 = a_longerunion.set_usize(4, 100_usize);
        /// assert_eq!(b0, true);
        /// assert_eq!(b1, true);
        /// assert_eq!(b2, true);
        /// assert_eq!(b3, true);
        /// assert_eq!(b4, false);
        /// 
        /// for i in 0..4
        /// {
        ///     match a_longerunion.get_usize(i)
        ///     {
        ///         Some(a) => { println!("a_longerunion.get_usize({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longerunion.get_usize(0), Some(4048161615_usize));
        /// assert_eq!(a_longerunion.get_usize(1), Some(3181603061_usize));
        /// assert_eq!(a_longerunion.get_usize(2), Some(2127464536_usize));
        /// assert_eq!(a_longerunion.get_usize(3), Some(4279384858_usize));
        /// assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[cfg(target_pointer_width = "32")]
        pub fn set_usize(&mut self, i: usize, val:usize) -> bool
        {
            if i <= Self::J
            { 
                #[cfg(target_endian = "little")]    unsafe { self.u_size[i] = val; }
                #[cfg(target_endian = "big")]       unsafe { self.u_size[Self::J-i] = val; }
                true
            }
            else
            {
                false
            }
        }

        // pub fn set_usize(&mut self, i: usize, val:usize) -> bool
        /// Sets i-th element of its array `u_size` of type `usize` and returns
        /// true if `i` is less than the size of this union in bytes divided
        /// by the size of the type `usize` in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Arguments
        /// - `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// - `val` is the value of type `usize` to set the i-th element of its
        ///  array `u_size` of type `usize`.
        /// 
        /// # Output
        /// - `true` if `i` is less than the size of this union in bytes
        /// divided by the size of the type `usize` in bytes
        /// - `false` if `i` is equal to or greater than the size of this
        /// union in bytes divided by the size of the type `usize` in bytes
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// the size of this union in bytes divided by the size of the type
        /// `usize` in bytes. Otherwise, you can use its counterpart
        /// method [set_usize_()](#method.set_usize_) for performance.
        /// 
        /// # Example
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let mut a_longerunion = LongerUnion::new();
        /// let b0 = a_longerunion.set_usize(0, 13664881099896654671_usize);
        /// let b1 = a_longerunion.set_usize(1, 18379818014235068504_usize);
        /// let b2 = a_longerunion.set_usize(4, 100_usize);
        /// assert_eq!(b0, true);
        /// assert_eq!(b1, true);
        /// assert_eq!(b2, false);
        /// 
        /// for i in 0..2
        /// {
        ///     match a_longerunion.get_usize(i)
        ///     {
        ///         Some(a) => { println!("a_sizeunion.get_usize({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longerunion.get_usize(0), Some(13664881099896654671_usize));
        /// assert_eq!(a_longerunion.get_usize(1), Some(18379818014235068504_usize));
        /// assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[cfg(target_pointer_width = "64")]
        pub fn set_usize(&mut self, i: usize, val:usize) -> bool
        {
            if i <= Self::J
            { 
                #[cfg(target_endian = "little")]    unsafe { self.u_size[i] = val; }
                #[cfg(target_endian = "big")]       unsafe { self.u_size[Self::J-i] = val; }
                true
            }
            else
            {
                false
            }
        }

        // // pub fn get_ssize(&self, i: usize) -> Option<isize>
        // /// Returns i-th element of array `s_size` of type `isize` wrapped
        // /// in Some of enum `Option` if `i` is less than the size of this union
        // /// in bytes divided by the size of the type `isize` in bytes.
        // /// Otherwise, it returns None of enum `Option`.
        // /// 
        // /// # Arguments
        // /// `i` indicates `i`-th element. 0-th element contains LSB (Least
        // /// Significant Bit), while (the size of this union in bytes - 1)-th
        // /// element contains MSB (Most Significant Bit) regardless endianness.
        // /// 
        // /// # Output
        // /// i-th element of array `s_size` of type `isize` wrapped
        // /// in Some of enum `Option` if `i` is less than the size of this union
        // /// in bytes divided by the size of the type `isize` in bytes
        // /// 
        // /// # Counterpart Method
        // /// Use this method when you are not sure that `i` is less than
        // /// the size of this union in bytes divided by the size of the type
        // /// `isize` in bytes. Otherwise, you can use its counterpart method
        // /// [get_ssize_()](#method.get_ssize_) for performance.
        // /// 
        // /// # Example 1 for ShortUnion
        // /// ```
        // /// use cryptocol::number::ShortUnion;
        // /// let a_shortunion = ShortUnion::new_with(2895_u16);
        // /// for i in 0..2
        // /// {
        // ///     match a_shortunion.get_ssize(i)
        // ///     {
        // ///         Some(a) => { println!("a_shortunion.get_ssize({}) = {}", i, a); },
        // ///         _ => {},
        // ///     }
        // /// }
        // /// assert_eq!(a_shortunion.get_ssize(0), Some(79_isize));
        // /// assert_eq!(a_shortunion.get_ssize(1), Some(11_isize));
        // /// assert_eq!(a_shortunion.get_ssize(2), None);
        // /// ```
        // /// 
        // /// # Example 2 for IntUnion
        // /// ```
        // /// use cryptocol::number::IntUnion;
        // /// let a_intunion = IntUnion::new_with(4048161615_u32);
        // /// for i in 0..4
        // /// {
        // ///     match a_intunion.get_ssize(i)
        // ///     {
        // ///         Some(a) => { println!("a_intunion.get_ssize({}) = {}", i, a); },
        // ///         _ => {},
        // ///     }
        // /// }
        // /// assert_eq!(a_intunion.get_ssize(0), Some(79_isize));
        // /// assert_eq!(a_intunion.get_ssize(1), Some(11_isize));
        // /// assert_eq!(a_intunion.get_ssize(2), Some(74_isize));
        // /// assert_eq!(a_intunion.get_ssize(3), Some(-15_isize));
        // /// assert_eq!(a_intunion.get_ssize(4), None);
        // /// ```
        // /// 
        // /// # Example 3 for LongUnion
        // /// ```
        // /// use cryptocol::number::LongUnion;
        // /// let a_longunion = LongUnion::new_with(13664881099896654671_u64);
        // /// for i in 0..8
        // /// {
        // ///     match a_longunion.get_ssize(i)
        // ///     {
        // ///         Some(a) => { println!("a_longunion.get_ssize({}) = {}", i, a); },
        // ///         _ => {},
        // ///     }
        // /// }
        // /// assert_eq!(a_longunion.get_ssize(0), Some(79_isize));
        // /// assert_eq!(a_longunion.get_ssize(1), Some(11_isize));
        // /// assert_eq!(a_longunion.get_ssize(2), Some(74_isize));
        // /// assert_eq!(a_longunion.get_ssize(3), Some(-15_isize));
        // /// assert_eq!(a_longunion.get_ssize(4), Some(-11_isize));
        // /// assert_eq!(a_longunion.get_ssize(5), Some(104_isize));
        // /// assert_eq!(a_longunion.get_ssize(6), Some(-93_isize));
        // /// assert_eq!(a_longunion.get_ssize(7), Some(-67_isize));
        // /// assert_eq!(a_longunion.get_ssize(8), None);
        // /// ```
        // /// 
        // /// # Example 4 for LongerUnion
        // /// ```
        // /// use cryptocol::number::LongerUnion;
        // /// let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_usize);
        // /// for i in 0..16
        // /// {
        // ///     match a_longerunion.get_ssize(i)
        // ///     {
        // ///         Some(a) => { println!("a_longerunion.get_ssize({}) = {}", i, a); },
        // ///         _ => {},
        // ///     }
        // /// }
        // /// assert_eq!(a_longerunion.get_ssize(0), Some(79_isize));
        // /// assert_eq!(a_longerunion.get_ssize(1), Some(11_isize));
        // /// assert_eq!(a_longerunion.get_ssize(2), Some(74_isize));
        // /// assert_eq!(a_longerunion.get_ssize(3), Some(-15_isize));
        // /// assert_eq!(a_longerunion.get_ssize(4), Some(-11_isize));
        // /// assert_eq!(a_longerunion.get_ssize(5), Some(104_isize));
        // /// assert_eq!(a_longerunion.get_ssize(6), Some(-93_isize));
        // /// assert_eq!(a_longerunion.get_ssize(7), Some(-67_isize));
        // /// assert_eq!(a_longerunion.get_ssize(8), Some(88_isize));
        // /// assert_eq!(a_longerunion.get_ssize(9), Some(-120_isize));
        // /// assert_eq!(a_longerunion.get_ssize(10), Some(-50_isize));
        // /// assert_eq!(a_longerunion.get_ssize(11), Some(126_isize));
        // /// assert_eq!(a_longerunion.get_ssize(12), Some(26_isize));
        // /// assert_eq!(a_longerunion.get_ssize(13), Some(59_isize));
        // /// assert_eq!(a_longerunion.get_ssize(14), Some(18_isize));
        // /// assert_eq!(a_longerunion.get_ssize(15), Some(-1_isize));
        // /// assert_eq!(a_longerunion.get_ssize(16), None);
        // /// ```
        // /// 
        // /// # Big-endian issue
        // /// It is just experimental for Big Endian CPUs. So, you are not
        // /// encouraged to use it for Big Endian CPUs for serious purpose.
        // /// Only use this crate for Big-endian CPUs with your own full
        // /// responsibility.
        // #[cfg(target_pointer_width = "8")]
        // pub fn get_ssize(&self, i: usize) -> Option<isize>
        // {
        //     if i <= Self::J
        //     {
        //         #[cfg(target_endian = "little")]    unsafe { Some(self.s_size[i]) }
        //         #[cfg(target_endian = "big")]       unsafe { Some(self.s_size[Self::J-i]) }
        //     }
        //     else
        //     {
        //         None
        //     }
        // }

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
        /// # Example 1 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// let a_intunion = IntUnion::new_with(4048161615_u32);
        /// for i in 0..2
        /// {
        ///     match a_intunion.get_ssize(i)
        ///     {
        ///         Some(a) => { println!("a_intunion.get_ssize({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_intunion.get_ssize(0), Some(2895_isize));
        /// assert_eq!(a_intunion.get_ssize(1), Some(-3766_isize));
        /// assert_eq!(a_intunion.get_ssize(2), None);
        /// ```
        /// 
        /// # Example 2 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let a_longunion = LongUnion::new_with(13664881099896654671_u64);
        /// for i in 0..4
        /// {
        ///     match a_longunion.get_ssize(i)
        ///     {
        ///         Some(a) => { println!("a_longunion.get_ssize({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longunion.get_ssize(0), Some(2895_isize));
        /// assert_eq!(a_longunion.get_ssize(1), Some(-3766_isize));
        /// assert_eq!(a_longunion.get_ssize(2), Some(26869_isize));
        /// assert_eq!(a_longunion.get_ssize(3), Some(-16989_isize));
        /// assert_eq!(a_longunion.get_ssize(4), None);
        /// ```
        /// 
        /// # Example 3 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// for i in 0..8
        /// {
        ///     match a_longerunion.get_ssize(i)
        ///     {
        ///         Some(a) => { println!("a_longerunion.get_ssize({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longerunion.get_ssize(0), Some(2895_isize));
        /// assert_eq!(a_longerunion.get_ssize(1), Some(-3766_isize));
        /// assert_eq!(a_longerunion.get_ssize(2), Some(26869_isize));
        /// assert_eq!(a_longerunion.get_ssize(3), Some(-16989_isize));
        /// assert_eq!(a_longerunion.get_ssize(4), Some(-30632_isize));
        /// assert_eq!(a_longerunion.get_ssize(5), Some(32462_isize));
        /// assert_eq!(a_longerunion.get_ssize(6), Some(15130_isize));
        /// assert_eq!(a_longerunion.get_ssize(7), Some(-238_isize));
        /// assert_eq!(a_longerunion.get_ssize(8), None);
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[cfg(target_pointer_width = "16")]
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
        /// # Example 1 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let a_longunion = LongUnion::new_with(13664881099896654671_u64);
        /// for i in 0..2
        /// {
        ///     match a_longunion.get_ssize(i)
        ///     {
        ///         Some(a) => { println!("a_longunion.get_ssize({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longunion.get_ssize(0), Some(-246805681_isize));
        /// assert_eq!(a_longunion.get_ssize(1), Some(-1113364235_isize));
        /// assert_eq!(a_longunion.get_ssize(2), None);
        /// ```
        /// 
        /// # Example 2 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// for i in 0..4
        /// {
        ///     match a_longerunion.get_ssize(i)
        ///     {
        ///         Some(a) => { println!("a_longerunion.get_ssize({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longerunion.get_ssize(0), Some(-246805681_isize));
        /// assert_eq!(a_longerunion.get_ssize(1), Some(-1113364235_isize));
        /// assert_eq!(a_longerunion.get_ssize(2), Some(2127464536_isize));
        /// assert_eq!(a_longerunion.get_ssize(3), Some(-15582438_isize));
        /// assert_eq!(a_longerunion.get_ssize(4), None);
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[cfg(target_pointer_width = "32")]
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
        /// # Example
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let a_longerunion = LongerUnion::new_with(339047799029950809142362261752780557135_u128);
        /// for i in 0..2
        /// {
        ///     match a_longerunion.get_ssize(i)
        ///     {
        ///         Some(a) => { println!("a_longerunion.get_ssize({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longerunion.get_ssize(0), Some(-4781862973812896945_isize));
        /// assert_eq!(a_longerunion.get_ssize(1), Some(-66926059474483112_isize));
        /// assert_eq!(a_longerunion.get_sint(4), None);
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[cfg(target_pointer_width = "64")]
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

        // // pub fn set_ssize(&mut self, i: usize, val: isize) -> bool
        // /// Sets i-th element of its array `ssize` of type `isize` and returns
        // /// true if `i` is less than the size of this union in bytes divided
        // /// by the size of the type `isize` in bytes.
        // /// Otherwise, it will set nothing amd return false.
        // /// 
        // /// # Arguments i
        // /// `i` indicates `i`-th element. 0-th element contains LSB (Least
        // /// Significant Bit), while (the size of this union in bytes - 1)-th
        // /// element contains MSB (Most Significant Bit) regardless endianness.
        // /// - `val` is the value of type `isize` to set the i-th element of its
        // ///  array `s_size` of type `isize`.
        // /// 
        // /// # Output
        // /// - `true` if `i` is less than the size of this union in bytes
        // /// divided by the size of the type `isize` in bytes
        // /// - `false` if `i` is equal to or greater than the size of this
        // /// union in bytes divided by the size of the type `isize` in bytes
        // /// 
        // /// # Counterpart Method
        // /// Use this method when you are not sure that `i` is less than
        // /// the size of this union in bytes divided by the size of the type
        // /// `isize` in bytes. Otherwise, use its counterpart
        // /// method [set_ssize_()](#method.set_ssize_) for performance.
        // /// 
        // /// # Example 1 for ShortUnion
        // /// ```
        // /// use cryptocol::number::ShortUnion;
        // /// let mut a_shortunion = ShortUnion::new();
        // /// let b0 = a_shortunion.set_ssize(0, 79_isize);
        // /// let b1 = a_shortunion.set_ssize(1, 11_isize);
        // /// let b2 = a_shortunion.set_ssize(2, 100_isize);  // Nothing will be done
        // /// assert_eq!(b0, true);
        // /// assert_eq!(b1, true);
        // /// assert_eq!(b2, false);
        // /// println!("a_shortunion.get() = {}", a_shortunion.get());
        // /// for i in 0..2
        // /// {
        // ///     match a_shortunion.get_ssize(i)
        // ///     {
        // ///         Some(a) => { println!("a_shortunion.get_ssize({}) = {}", i, a); },
        // ///         _ => {},
        // ///     }
        // /// }
        // /// assert_eq!(a_shortunion.get_ssize(0), Some(79_isize));
        // /// assert_eq!(a_shortunion.get_ssize(1), Some(11_isize));
        // /// assert_eq!(a_shortunion.get_ssize(2), None);
        // /// ```
        // /// 
        // /// # Example 2 for IntUnion
        // /// ```
        // /// use cryptocol::number::IntUnion;
        // /// let mut a_intunion = IntUnion::new();
        // /// let b0 = a_intunion.set_ssize(0, 79_isize);
        // /// let b1 = a_intunion.set_ssize(1, 11_isize);
        // /// let b2 = a_intunion.set_ssize(2, 74_isize);
        // /// let b3 = a_intunion.set_ssize(3, -15_isize);
        // /// let b4 = a_intunion.set_ssize(4, 100_isize);
        // /// assert_eq!(b0, true);
        // /// assert_eq!(b1, true);
        // /// assert_eq!(b2, true);
        // /// assert_eq!(b3, true);
        // /// assert_eq!(b4, false);
        // /// println!("a_intunion.get() = {}", a_intunion.get());
        // /// for i in 0..4
        // /// {
        // ///     match a_intunion.get_ssize(i)
        // ///     {
        // ///         Some(a) => { println!("a_intunion.get_ssize({}) = {}", i, a); },
        // ///         _ => {},
        // ///     }
        // /// }
        // /// assert_eq!(a_intunion.get_ssize(0), Some(79_isize));
        // /// assert_eq!(a_intunion.get_ssize(1), Some(11_isize));
        // /// assert_eq!(a_intunion.get_ssize(2), Some(74_isize));
        // /// assert_eq!(a_intunion.get_ssize(3), Some(-15_isize));
        // /// assert_eq!(a_intunion.get(), 4048161615_u32);
        // /// ```
        // /// 
        // /// # Example 3 for LongUnion
        // /// ```
        // /// use cryptocol::number::LongUnion;
        // /// let mut a_longunion = LongUnion::new();
        // /// let b0 = a_longunion.set_ssize(0, 79_isize);
        // /// let b1 = a_longunion.set_ssize(1, 11_isize);
        // /// let b2 = a_longunion.set_ssize(2, 74_isize);
        // /// let b3 = a_longunion.set_ssize(3, -15_isize);
        // /// let b4 = a_longunion.set_ssize(4, -11_isize);
        // /// let b5 = a_longunion.set_ssize(5, 104_isize);
        // /// let b6 = a_longunion.set_ssize(6, -93_isize);
        // /// let b7 = a_longunion.set_ssize(7, -67_isize);
        // /// let b8 = a_longunion.set_ssize(8, 100_isize);
        // /// assert_eq!(b0, true);
        // /// assert_eq!(b1, true);
        // /// assert_eq!(b2, true);
        // /// assert_eq!(b3, true);
        // /// assert_eq!(b4, true);
        // /// assert_eq!(b5, true);
        // /// assert_eq!(b6, true);
        // /// assert_eq!(b7, true);
        // /// assert_eq!(b8, false);
        // /// println!("a_longunion.get() = {}", a_longunion.get());
        // /// for i in 0..8
        // /// {
        // ///     match a_longunion.get_usize(i)
        // ///     {
        // ///         Some(a) => { println!("a_longunion.get_usize({}) = {}", i, a); },
        // ///         _ => {},
        // ///     }
        // /// }
        // /// assert_eq!(a_longunion.get_ssize(0), Some(79_isize));
        // /// assert_eq!(a_longunion.get_ssize(1), Some(11_isize));
        // /// assert_eq!(a_longunion.get_ssize(2), Some(74_isize));
        // /// assert_eq!(a_longunion.get_ssize(3), Some(-15_isize));
        // /// assert_eq!(a_longunion.get_ssize(4), Some(-11_isize));
        // /// assert_eq!(a_longunion.get_ssize(5), Some(104_isize));
        // /// assert_eq!(a_longunion.get_ssize(6), Some(-93_isize));
        // /// assert_eq!(a_longunion.get_ssize(7), Some(-67_isize));
        // /// assert_eq!(a_longunion.get(), 13664881099896654671_u64);
        // /// ```
        // /// 
        // /// # Example 4 for LongerUnion
        // /// ```
        // /// use cryptocol::number::LongerUnion;
        // /// let mut a_longerunion = LongerUnion::new();
        // /// let b0 = a_longerunion.set_ssize(0, 79_isize);
        // /// let b1 = a_longerunion.set_ssize(1, 11_isize);
        // /// let b2 = a_longerunion.set_ssize(2, 74_isize);
        // /// let b3 = a_longerunion.set_ssize(3, -15_isize);
        // /// let b4 = a_longerunion.set_ssize(4, -11_isize);
        // /// let b5 = a_longerunion.set_ssize(5, 104_isize);
        // /// let b6 = a_longerunion.set_ssize(6, -93_isize);
        // /// let b7 = a_longerunion.set_ssize(7, -67_isize);
        // /// let b8 = a_longerunion.set_ssize(8, 88_isize);
        // /// let b9 = a_longerunion.set_ssize(9, -120_isize);
        // /// let b10 = a_longerunion.set_ssize(10, -50_isize);
        // /// let b11 = a_longerunion.set_ssize(11, 126_isize);
        // /// let b12 = a_longerunion.set_ssize(12, 26_isize);
        // /// let b13 = a_longerunion.set_ssize(13, 59_isize);
        // /// let b14 = a_longerunion.set_ssize(14, 18_isize);
        // /// let b15 = a_longerunion.set_ssize(15, -1_isize);
        // /// let b16 = a_longerunion.set_ssize(16, 100_isize);
        // /// assert_eq!(b0, true);
        // /// assert_eq!(b1, true);
        // /// assert_eq!(b2, true);
        // /// assert_eq!(b3, true);
        // /// assert_eq!(b4, true);
        // /// assert_eq!(b5, true);
        // /// assert_eq!(b6, true);
        // /// assert_eq!(b7, true);
        // /// assert_eq!(b8, true);
        // /// assert_eq!(b9, true);
        // /// assert_eq!(b10, true);
        // /// assert_eq!(b11, true);
        // /// assert_eq!(b12, true);
        // /// assert_eq!(b13, true);
        // /// assert_eq!(b14, true);
        // /// assert_eq!(b15, true);
        // /// assert_eq!(b16, false);
        // /// println!("a_longerunion.get() = {}", a_longerunion.get());
        // /// for i in 0..16
        // /// {
        // ///     match a_longerunion.get_usize(i)
        // ///     {
        // ///         Some(a) => { println!("a_longerunion.get_usize({}) = {}", i, a); },
        // ///         _ => {},
        // ///     }
        // /// }
        // /// assert_eq!(a_longerunion.get_ssize(0), Some(79_isize));
        // /// assert_eq!(a_longerunion.get_ssize(1), Some(11_isize));
        // /// assert_eq!(a_longerunion.get_ssize(2), Some(74_isize));
        // /// assert_eq!(a_longerunion.get_ssize(3), Some(-15_isize));
        // /// assert_eq!(a_longerunion.get_ssize(4), Some(-11_isize));
        // /// assert_eq!(a_longerunion.get_ssize(5), Some(104_isize));
        // /// assert_eq!(a_longerunion.get_ssize(6), Some(-93_isize));
        // /// assert_eq!(a_longerunion.get_ssize(7), Some(-67_isize));
        // /// assert_eq!(a_longerunion.get_ssize(8), Some(88_isize));
        // /// assert_eq!(a_longerunion.get_ssize(9), Some(-120_isize));
        // /// assert_eq!(a_longerunion.get_ssize(10), Some(-50_isize));
        // /// assert_eq!(a_longerunion.get_ssize(11), Some(126_isize));
        // /// assert_eq!(a_longerunion.get_ssize(12), Some(26_isize));
        // /// assert_eq!(a_longerunion.get_ssize(13), Some(59_isize));
        // /// assert_eq!(a_longerunion.get_ssize(14), Some(18_isize));
        // /// assert_eq!(a_longerunion.get_ssize(15), Some(-1_isize));
        // /// assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_usize);
        // /// ```
        // /// 
        // /// # Big-endian issue
        // /// It is just experimental for Big Endian CPUs. So, you are not
        // /// encouraged to use it for Big Endian CPUs for serious purpose.
        // /// Only use this crate for Big-endian CPUs with your own full
        // /// responsibility.
        // #[cfg(target_pointer_width = "8")]
        // pub fn set_ssize(&mut self, i: usize, val: isize) -> bool
        // {
        //     if i <= Self::J
        //     { 
        //         #[cfg(target_endian = "little")]    unsafe { self.s_size[i] = val; }
        //         #[cfg(target_endian = "big")]       unsafe { self.s_size[Self::J-i] = val; }
        //         true
        //     }
        //     else
        //     {
        //         false
        //     }
        // }

        // pub fn set_ssize(&mut self, i: usize, val: isize) -> bool
        /// Sets i-th element of its array `ssize` of type `isize` and returns
        /// true if `i` is less than the size of this union in bytes divided
        /// by the size of the type `isize` in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Arguments i
        /// `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// - `val` is the value of type `isize` to set the i-th element of its
        ///  array `s_size` of type `isize`.
        /// 
        /// # Output
        /// - `true` if `i` is less than the size of this union in bytes
        /// divided by the size of the type `isize` in bytes
        /// - `false` if `i` is equal to or greater than the size of this
        /// union in bytes divided by the size of the type `isize` in bytes
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// the size of this union in bytes divided by the size of the type
        /// `isize` in bytes. Otherwise, use its counterpart
        /// method [set_ssize_()](#method.set_ssize_) for performance.
        /// 
        /// # Example 1 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// let mut a_intunion = IntUnion::new();
        /// let b0 = a_intunion.set_ssize(0, 2895_isize);
        /// let b1 = a_intunion.set_ssize(1, -3766_isize);
        /// let b2 = a_intunion.set_ssize(2, 100_isize);
        /// assert_eq!(b0, true);
        /// assert_eq!(b1, true);
        /// assert_eq!(b2, false);
        /// println!("a_intunion.get() = {}", a_intunion.get());
        /// for i in 0..2
        /// {
        ///     match a_intunion.get_ssize(i)
        ///     {
        ///         Some(a) => { println!("a_intunion.set_ssize({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_intunion.get_ssize(0), Some(2895_isize));
        /// assert_eq!(a_intunion.get_ssize(1), Some(-3766_isize));
        /// assert_eq!(a_intunion.get(), 4048161615_u32);
        /// ```
        /// 
        /// # Example 2 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let mut a_longunion = LongUnion::new();
        /// let b0 = a_longunion.set_ssize(0, 2895_isize);
        /// let b1 = a_longunion.set_ssize(1, -3766_isize);
        /// let b2 = a_longunion.set_ssize(2, 26869_isize);
        /// let b3 = a_longunion.set_ssize(3, -16989_isize);
        /// let b4 = a_longunion.set_ssize(4, 100_isize);
        /// assert_eq!(b0, true);
        /// assert_eq!(b1, true);
        /// assert_eq!(b2, true);
        /// assert_eq!(b3, true);
        /// assert_eq!(b4, false);
        /// println!("a_longunion.get() = {}", a_longunion.get());
        /// for i in 0..4
        /// {
        ///     match a_longunion.get_ssize(i)
        ///     {
        ///         Some(a) => { println!("a_longunion.get_ssize({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longunion.get_ssize(0), Some(2895_isize));
        /// assert_eq!(a_longunion.get_ssize(1), Some(-3766_isize));
        /// assert_eq!(a_longunion.get_ssize(2), Some(26869_isize));
        /// assert_eq!(a_longunion.get_ssize(3), Some(-16989_isize));
        /// assert_eq!(a_longunion.get(), 13664881099896654671_u64);
        /// ```
        /// 
        /// # Example 3 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let mut a_longerunion = LongerUnion::new();
        /// let b0 = a_longerunion.set_ssize(0, 2895_isize);
        /// let b1 = a_longerunion.set_ssize(1, -3766_isize);
        /// let b2 = a_longerunion.set_ssize(2, 26869_isize);
        /// let b3 = a_longerunion.set_ssize(3, -16989_isize);
        /// let b4 = a_longerunion.set_ssize(4, -30632_isize);
        /// let b5 = a_longerunion.set_ssize(5, 32462_isize);
        /// let b6 = a_longerunion.set_ssize(6, 15130_isize);
        /// let b7 = a_longerunion.set_ssize(7, -238_isize);
        /// let b8 = a_longerunion.set_ssize(8, 100_isize);
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
        ///     match a_longerunion.get_usize(i)
        ///     {
        ///         Some(a) => { println!("a_longerunion.get_usize({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longerunion.get_ssize(0), Some(2895_isize));
        /// assert_eq!(a_longerunion.get_ssize(1), Some(-3766_isize));
        /// assert_eq!(a_longerunion.get_ssize(2), Some(26869_isize));
        /// assert_eq!(a_longerunion.get_ssize(3), Some(-16989_isize));
        /// assert_eq!(a_longerunion.get_ssize(4), Some(-30632_isize));
        /// assert_eq!(a_longerunion.get_ssize(5), Some(32462_isize));
        /// assert_eq!(a_longerunion.get_ssize(6), Some(15130_isize));
        /// assert_eq!(a_longerunion.get_ssize(7), Some(-238_isize));
        /// assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[cfg(target_pointer_width = "16")]
        pub fn set_ssize(&mut self, i: usize, val: isize) -> bool
        {
            if i <= Self::J
            { 
                #[cfg(target_endian = "little")]    unsafe { self.s_size[i] = val; }
                #[cfg(target_endian = "big")]       unsafe { self.s_size[Self::J-i] = val; }
                true
            }
            else
            {
                false
            }
        }

        // pub fn set_ssize(&mut self, i: usize, val: isize) -> bool
        /// Sets i-th element of its array `ssize` of type `isize` and returns
        /// true if `i` is less than the size of this union in bytes divided
        /// by the size of the type `isize` in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Arguments i
        /// `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// - `val` is the value of type `isize` to set the i-th element of its
        ///  array `s_size` of type `isize`.
        /// 
        /// # Output
        /// - `true` if `i` is less than the size of this union in bytes
        /// divided by the size of the type `isize` in bytes
        /// - `false` if `i` is equal to or greater than the size of this
        /// union in bytes divided by the size of the type `isize` in bytes
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// the size of this union in bytes divided by the size of the type
        /// `isize` in bytes. Otherwise, use its counterpart
        /// method [set_ssize_()](#method.set_ssize_) for performance.
        /// 
        /// # Example 1 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let mut a_longunion = LongUnion::new();
        /// let b0 = a_longunion.set_ssize(0, -246805681_isize);
        /// let b1 = a_longunion.set_ssize(1, -1113364235_isize);
        /// let b2 = a_longunion.set_ssize(2, 100_isize);
        /// assert_eq!(b0, true);
        /// assert_eq!(b1, true);
        /// assert_eq!(b2, false);
        /// for i in 0..2
        /// {
        ///     match a_longunion.get_ssize(i)
        ///     {
        ///         Some(a) => { println!("a_longunion.set_ssize({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longunion.get_ssize(0), Some(-246805681_isize));
        /// assert_eq!(a_longunion.get_ssize(1), Some(-1113364235_isize));
        /// assert_eq!(a_longunion.get(), 13664881099896654671_u64);
        /// ```
        /// 
        /// # Example 2 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let mut a_longerunion = LongerUnion::new();
        /// let b0 = a_longerunion.set_ssize(0, -246805681_isize);
        /// let b1 = a_longerunion.set_ssize(1, -1113364235_isize);
        /// let b2 = a_longerunion.set_ssize(2, 2127464536_isize);
        /// let b3 = a_longerunion.set_ssize(3, -15582438_isize);
        /// let b4 = a_longerunion.set_ssize(4, 100_isize);
        /// assert_eq!(b0, true);
        /// assert_eq!(b1, true);
        /// assert_eq!(b2, true);
        /// assert_eq!(b3, true);
        /// assert_eq!(b4, false);
        /// 
        /// for i in 0..4
        /// {
        ///     match a_longerunion.get_usize(i)
        ///     {
        ///         Some(a) => { println!("a_longerunion.get_usize({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longerunion.get_ssize(0), Some(-246805681_isize));
        /// assert_eq!(a_longerunion.get_ssize(1), Some(-1113364235_isize));
        /// assert_eq!(a_longerunion.get_ssize(2), Some(2127464536_isize));
        /// assert_eq!(a_longerunion.get_ssize(3), Some(-15582438_isize));
        /// assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[cfg(target_pointer_width = "32")]
        pub fn set_ssize(&mut self, i: usize, val: isize) -> bool
        {
            if i <= Self::J
            { 
                #[cfg(target_endian = "little")]    unsafe { self.s_size[i] = val; }
                #[cfg(target_endian = "big")]       unsafe { self.s_size[Self::J-i] = val; }
                true
            }
            else
            {
                false
            }
        }

        // pub fn set_ssize(&mut self, i: usize, val: isize) -> bool
        /// Sets i-th element of its array `ssize` of type `isize` and returns
        /// true if `i` is less than the size of this union in bytes divided
        /// by the size of the type `isize` in bytes.
        /// Otherwise, it will set nothing amd return false.
        /// 
        /// # Arguments i
        /// `i` indicates `i`-th element. 0-th element contains LSB (Least
        /// Significant Bit), while (the size of this union in bytes - 1)-th
        /// element contains MSB (Most Significant Bit) regardless endianness.
        /// - `val` is the value of type `isize` to set the i-th element of its
        ///  array `s_size` of type `isize`.
        /// 
        /// # Output
        /// - `true` if `i` is less than the size of this union in bytes
        /// divided by the size of the type `isize` in bytes
        /// - `false` if `i` is equal to or greater than the size of this
        /// union in bytes divided by the size of the type `isize` in bytes
        /// 
        /// # Counterpart Method
        /// Use this method when you are not sure that `i` is less than
        /// the size of this union in bytes divided by the size of the type
        /// `isize` in bytes. Otherwise, use its counterpart
        /// method [set_ssize_()](#method.set_ssize_) for performance.
        /// 
        /// # Example
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let mut a_longerunion = LongerUnion::new();
        /// let b0 = a_longerunion.set_ssize(0, -4781862973812896945_isize);
        /// let b1 = a_longerunion.set_ssize(1, -66926059474483112_isize);
        /// let b2 = a_longerunion.set_ssize(4, 100_isize);
        /// assert_eq!(b0, true);
        /// assert_eq!(b1, true);
        /// assert_eq!(b2, false);
        /// 
        /// for i in 0..2
        /// {
        ///     match a_longerunion.get_ssize(i)
        ///     {
        ///         Some(a) => { println!("a_longerunion.get_ssize({}) = {}", i, a); },
        ///         _ => {},
        ///     }
        /// }
        /// assert_eq!(a_longerunion.get_ssize(0), Some(-4781862973812896945_isize));
        /// assert_eq!(a_longerunion.get_ssize(1), Some(-66926059474483112_isize));
        /// assert_eq!(a_longerunion.get(), 339047799029950809142362261752780557135_u128);
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not
        /// encouraged to use it for Big Endian CPUs for serious purpose.
        /// Only use this crate for Big-endian CPUs with your own full
        /// responsibility.
        #[cfg(target_pointer_width = "64")]
        pub fn set_ssize(&mut self, i: usize, val: isize) -> bool
        {
            if i <= Self::J
            { 
                #[cfg(target_endian = "little")]    unsafe { self.s_size[i] = val; }
                #[cfg(target_endian = "big")]       unsafe { self.s_size[Self::J-i] = val; }
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
        /***** ADDITION *****/

        // pub fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool)
        /// Calculates `self` + `rhs` + `carry`,
        /// wrapping around at the boundary of the type.
        /// 
        /// # Arguments
        /// - `rhs` is the operand of `Self` type.
        /// - `carry` is the carry overflowed from the previous operation.
        /// If there is no overflowed carry from the previous operation,
        /// `carry` is `false`. Otherwise, it is `true`.
        /// 
        /// # Features
        /// - This allows chaining together multiple additions to create a wider
        /// addition, and can be useful for big integer type addition.
        /// - This can be thought of as a 8-bit “full adder”, in the electronics
        /// sense.
        /// - If `carry` is `false`, this method is equivalent to
        /// `overflowing_add()`.
        /// 
        /// # Outputs
        /// It returns a tuple containing the sum and the output `carry`.
        /// It performs “ternary addition” of two integer operands and a
        /// carry-in bit, and returns an output integer and a carry-out bit.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// // a_intunion: IntUnion === (a_high_shortunion, a_low_shortunion) === (10000_u16, 10100_u16) === 655370100_u32
        /// let a_high_shortunion = ShortUnion::new_with(10000_u16);
        /// let a_low_shortunion = ShortUnion::new_with(10100_u16);
        /// // b_shortunion: IntUnion === (b_high_shortunion, b_low_shortunion) === (10000_u16, 20000_u16) === 3276830000_u32
        /// let b_high_shortunion = ShortUnion::new_with(50000_u16);
        /// let b_low_shortunion = ShortUnion::new_with(30000_u16);
        /// 
        /// // (10000_u16, 10100_u16) + (50000_u16, 30000_u16) == 655370100_u32 + 3276830000_u32 == 3932200100_u32
        /// //    655370100_u32 == (10000_u16, 10100_u16)
        /// // + 3276830000_u32 == (50000_u16, 30000_u16)
        /// // ------------------------------------------
        /// //   3932200100_u32 == (60000_u16, 40100_u16)
        /// 
        /// // c_u32: u32 === (c_high_shortunion, c_low_shortunion)
        /// let (c_low_shortunion, carry) = a_low_shortunion.carrying_add(b_low_shortunion, false);
        /// let (c_high_shortunion, carry) = a_high_shortunion.carrying_add(b_high_shortunion, carry);
        /// println!("{}-{}, {}", c_high_shortunion, c_low_shortunion, carry);
        /// assert_eq!(c_high_shortunion.get(), 60000_u16);
        /// assert_eq!(c_low_shortunion.get(), 40100_u16);
        /// assert_eq!(carry, false);
        /// 
        /// // (10000_u16, 10100_u16) + (50000_u16, 30000_u16) == 3932200100_u32 + 3276830000_u32 == 51501_u16
        /// //   3932200100_u32 == (60000_u16, 40100_u16)
        /// // + 3276830000_u32 == (50000_u16, 30000_u16)
        /// // ------------------------------------------
        /// //   2914062804_u32 == (44465_u16,  4564_u16)
        /// 
        /// // d: u32 === (d_high_shortunion, d_low_shortunion)
        /// let (d_low_shortunion, carry) = c_low_shortunion.carrying_add(b_low_shortunion, false);
        /// let (d_high_shortunion, carry) = c_high_shortunion.carrying_add(b_high_shortunion, carry);
        /// println!("{}-{}, {}", d_high_shortunion, d_low_shortunion, carry);
        /// assert_eq!(d_high_shortunion.get(), 44465_u16);
        /// assert_eq!(d_low_shortunion.get(), 4564_u16);
        /// assert_eq!(carry, true);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// //  1234567890123456789_u64 == ( 287445236_u16, 2112454933_u16)
        /// //+ 9876543210123456789_u64 == (2299561912_u16, 2956226837_u16)
        /// //-------------------------------------------------------------
        /// // 11111111100246913578_u64 == (2587007149_u16,  773714474_u16)
        /// 
        /// // a: u256 === (a_high_longerunion, a_low_longerunion)
        /// let (a_low_intunion, carry) = IntUnion::new_with(2112454933_u32).carrying_add(IntUnion::new_with(2956226837_u32), false);
        /// let (a_high_intunion, carry) = IntUnion::new_with(287445236_u32).carrying_add(IntUnion::new_with(2299561912_u32), carry);
        /// println!("{}-{}, {}", a_high_intunion, a_low_intunion, carry);
        /// assert_eq!(a_high_intunion.get(), 2587007149_u32);
        /// assert_eq!(a_low_intunion.get(), 773714474_u32);
        /// assert_eq!(carry, false);
        /// 
        /// //  11111111100246913578_u64 == (2587007149_u32,  773714474_u32)
        /// //+  9876543210123456789_u64 == (2299561912_u32, 2956226837_u32)
        /// //--------------------------------------------------------------
        /// //   2540910236660818751_u64 == ( 591601765_u32, 3729941311_u32)
        /// 
        /// // b: u256 === (b_high_longerunion, b_low_longerunion)
        /// let (b_low_intunion, carry) = IntUnion::new_with(773714474_u32).carrying_add(IntUnion::new_with(2956226837_u32), false);
        /// let (b_high_intunion, carry) = IntUnion::new_with(2587007149_u32).carrying_add(IntUnion::new_with(2299561912_u32), carry);
        /// println!("{}-{}, {}", b_high_intunion, b_low_intunion, carry);
        /// assert_eq!(b_high_intunion.get(), 591601765_u32);
        /// assert_eq!(b_low_intunion.get(), 3729941311_u32);
        /// assert_eq!(carry, true);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// // a_longerunion: LongerUnion === (a_high_longunion, a_low_longunion) === (6692605942763486917_u64, 12312739301371248917_u64) === 322222221211111111100000000088888888987_u128
        /// let a_high_longunion = LongUnion::new_with(6692605942763486917_u64);
        /// let a_low_longunion = LongUnion::new_with(12312739301371248917_u64);
        /// // b_longunion: LongerUnion === (b_high_longunion, b_low_longunion) === (10775095670246085798_u64, 7681743649119882630_u64) === 198765432198765432198765432198765432198_u128
        /// let b_high_longunion = LongUnion::new_with(10775095670246085798_u64);
        /// let b_low_longunion = LongUnion::new_with(7681743649119882630_u64);
        /// 
        /// // (6692605942763486917_u64, 12312739301371248917_u64) + (10775095670246085798_u64, 7681743649119882630_u64) == 123456789012345678901234567890123456789_u128 + 198765432198765432198765432198765432198_u128 == 322222221211111111100000000088888888987_u128
        /// //   123456789012345678901234567890123456789_u128 == (6692605942763486917_u64, 12312739301371248917_u64)
        /// // + 198765432198765432198765432198765432198_u128 == (10775095670246085798_u64, 7681743649119882630_u64)
        /// // -----------------------------------------------------------------------------------------------------
        /// //   322222221211111111100000000088888888987_u128 == (17467701613009572716_u64, 1547738876781579931_u64)
        /// 
        /// // c_u128: u128 === (c_high_longunion, c_low_longunion)
        /// let (c_low_longunion, carry) = a_low_longunion.carrying_add(b_low_longunion, false);
        /// let (c_high_longunion, carry) = a_high_longunion.carrying_add(b_high_longunion, carry);
        /// println!("{}-{}, {}", c_high_longunion, c_low_longunion, carry);
        /// assert_eq!(c_high_longunion.get(), 17467701613009572716_u64);
        /// assert_eq!(c_low_longunion.get(), 1547738876781579931_u64);
        /// assert_eq!(carry, false);
        /// 
        /// // (17467701613009572716_u64, 1547738876781579931_u64) + (10775095670246085798_u64, 7681743649119882630_u64) == 322222221211111111100000000088888888987_u128 + 198765432198765432198765432198765432198_u128 == 180705286488938079835390824855886109729_u64
        /// //   322222221211111111100000000088888888987_u128 == (17467701613009572716_u64, 1547738876781579931_u64)
        /// // + 198765432198765432198765432198765432198_u128 == (10775095670246085798_u64, 7681743649119882630_u64)
        /// // -----------------------------------------------------------------------------------------------------
        /// //   180705286488938079835390824855886109729_u128 == ( 9796053209546106898_u64, 9229482525901462561_u64)
        /// 
        /// // d: u128 === (d_high_longunion, d_low_longunion)
        /// let (d_low_longunion, carry) = c_low_longunion.carrying_add(b_low_longunion, false);
        /// let (d_high_longunion, carry) = c_high_longunion.carrying_add(b_high_longunion, carry);
        /// println!("{}-{}, {}", d_high_longunion, d_low_longunion, carry);
        /// assert_eq!(d_high_longunion.get(), 9796053209546106898_u64);
        /// assert_eq!(d_low_longunion.get(), 9229482525901462561_u64);
        /// assert_eq!(carry, true);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// //  4201016837757989640311993609423984479246482890531986660185_u256 == (12345678901234567890_u128, 6789012345678912345_u128)
        /// //+                 419908440780438063913804265570801972943493_u256 == (                1234_u128,                6789_u128)
        /// //--------------------------------------------------------------------------------------------------------------------------
        /// //  4201016837757990060220434389862048393050748461333959603678_u256 == (12345678901234569124_u128, 6789012345678919134_u128)
        /// 
        /// // a: u256 === (a_high_longerunion, a_low_longerunion)
        /// let (a_low_longerunion, carry) = LongerUnion::new_with(6789012345678912345_u128).carrying_add(LongerUnion::new_with(6789_u128), false);
        /// let (a_high_longerunion, carry) = LongerUnion::new_with(12345678901234567890_u128).carrying_add(LongerUnion::new_with(1234_u128), carry);
        /// println!("{}-{}, {}", a_high_longerunion, a_low_longerunion, carry);
        /// assert_eq!(a_high_longerunion.get(), 12345678901234569124_u128);
        /// assert_eq!(a_low_longerunion.get(), 6789012345678919134_u128);
        /// assert_eq!(carry, false);
        /// 
        /// //  308778904632843187796189293356501087608549893209439890708590319850715068122315 == (226854911280625642308916404954512140970_u128, 56789012345678912345678901234567890123_u128)
        /// //+  57896044618658097711785492504343953926307055644800578124155540853313808954190 == (170141183460469231731687303715884105727_u128, 12345678901234567890123456789012345678_u128)
        /// //-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
        /// //   19298681539552699237261830834781317975046994857318776714373108680289488156697 == ( 56713727820156410577229101238628035241_u128, 69134691246913480235802358023580235801_u128)
        /// 
        /// // b: u256 === (b_high_longerunion, b_low_longerunion)
        /// let (b_low_longerunion, carry) = LongerUnion::new_with(56789012345678912345678901234567890123_u128).carrying_add(LongerUnion::new_with(12345678901234567890123456789012345678_u128), false);
        /// let (b_high_longerunion, carry) = LongerUnion::new_with(226854911280625642308916404954512140970_u128).carrying_add(LongerUnion::new_with(170141183460469231731687303715884105727_u128), carry);
        /// println!("{}-{}, {}", b_high_longerunion, b_low_longerunion, carry);
        /// assert_eq!(b_high_longerunion.get(), 56713727820156410577229101238628035241_u128);
        /// assert_eq!(b_low_longerunion.get(), 69134691246913480235802358023580235801_u128);
        /// assert_eq!(carry, true);
        /// ```
        /// 
        /// # Example 5 for SizeUnion for 64-bitn CPU
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// // a_longerunion: LongerUnion === (a_high_sizeunion, a_low_sizeunion) === (6692605942763486917_usize, 12312739301371248917_usize) === 322222221211111111100000000088888888987_u128
        /// let a_high_sizeunion = SizeUnion::new_with(6692605942763486917_usize);
        /// let a_low_sizeunion = SizeUnion::new_with(12312739301371248917_usize);
        /// // b_sizeunion: LongerUnion === (b_high_sizeunion, b_low_sizeunion) === (10775095670246085798_usize, 7681743649119882630_usize) === 198765432198765432198765432198765432198_u128
        /// let b_high_sizeunion = SizeUnion::new_with(10775095670246085798_usize);
        /// let b_low_sizeunion = SizeUnion::new_with(7681743649119882630_usize);
        /// 
        /// // (6692605942763486917_usize, 12312739301371248917_usize) + (10775095670246085798_usize, 7681743649119882630_usize) == 123456789012345678901234567890123456789_u128 + 198765432198765432198765432198765432198_u128 == 322222221211111111100000000088888888987_u128
        /// //   123456789012345678901234567890123456789_u128 == (6692605942763486917_usize, 12312739301371248917_usize)
        /// // + 198765432198765432198765432198765432198_u128 == (10775095670246085798_usize, 7681743649119882630_usize)
        /// // ---------------------------------------------------------------------------------------------------------
        /// //   322222221211111111100000000088888888987_u128 == (17467701613009572716_usize, 1547738876781579931_usize)
        /// 
        /// // c_u128: u128 === (c_high_sizeunion, c_low_sizeunion)
        /// let (c_low_sizeunion, carry) = a_low_sizeunion.carrying_add(b_low_sizeunion, false);
        /// let (c_high_sizeunion, carry) = a_high_sizeunion.carrying_add(b_high_sizeunion, carry);
        /// println!("{}-{}, {}", c_high_sizeunion, c_low_sizeunion, carry);
        /// assert_eq!(c_high_sizeunion.get(), 17467701613009572716_usize);
        /// assert_eq!(c_low_sizeunion.get(), 1547738876781579931_usize);
        /// assert_eq!(carry, false);
        /// 
        /// // (17467701613009572716_usize, 1547738876781579931_usize) + (10775095670246085798_usize, 7681743649119882630_usize) == 322222221211111111100000000088888888987_u128 + 198765432198765432198765432198765432198_u128 == 180705286488938079835390824855886109729_usize
        /// //   322222221211111111100000000088888888987_u128 == (17467701613009572716_usize, 1547738876781579931_usize)
        /// // + 198765432198765432198765432198765432198_u128 == (10775095670246085798_usize, 7681743649119882630_usize)
        /// // ---------------------------------------------------------------------------------------------------------
        /// //   180705286488938079835390824855886109729_u128 == ( 9796053209546106898_usize, 9229482525901462561_usize)
        /// 
        /// // d: u128 === (d_high_sizeunion, d_low_sizeunion)
        /// let (d_low_sizeunion, carry) = c_low_sizeunion.carrying_add(b_low_sizeunion, false);
        /// let (d_high_sizeunion, carry) = c_high_sizeunion.carrying_add(b_high_sizeunion, carry);
        /// println!("{}-{}, {}", d_high_sizeunion, d_low_sizeunion, carry);
        /// assert_eq!(d_high_sizeunion.get(), 9796053209546106898_usize);
        /// assert_eq!(d_low_sizeunion.get(), 9229482525901462561_usize);
        /// assert_eq!(carry, true);
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
        pub fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool)
        {
            let (r_this, c1) = self.get().overflowing_add(rhs.get());
            let cc = if carry { 1 as $f } else { 0 as $f };
            let (res_this, c2) = r_this.overflowing_add(cc);
            let res = Self::new_with(res_this);
            (res, c1 || c2)
        }

        // pub fn wrapping_add(self, rhs: Self) -> Self
        /// Computes `self` + `rhs`, wrapping around at the boundary of the type.
        /// 
        /// # Arguments
        /// - `rhs` is the operand of `Self` type.
        /// 
        /// # Features
        /// It adds two numbers with wrapping (modular) addition.
        /// 
        /// # Output
        /// It returns the `self` + `rhs` in the type of `Self`.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, ShortUnion };
        /// let a_shortunion = ShortUnion::new_with(u16::MAX - 55_u16);
        /// let b_shortunion = ShortUnion::new_with(55);
        /// let c_shortunion = a_shortunion.wrapping_add(b_shortunion);
        /// println!("{} + {} = {}", a_shortunion, b_shortunion, c_shortunion);
        /// assert_eq!(c_shortunion.get(), u16::MAX);
        /// 
        /// let d_shortunion = c_shortunion.wrapping_add(1_u16.into_shortunion());
        /// println!("{} + 1 = {}", a_shortunion, d_shortunion);
        /// assert_eq!(d_shortunion.get(), 0_u16);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, IntUnion };
        /// let a_intunion = IntUnion::new_with(u32::MAX - 55_u32);
        /// let b_intunion = IntUnion::new_with(55);
        /// let c_intunion = a_intunion.wrapping_add(b_intunion);
        /// println!("{} + {} = {}", a_intunion, b_intunion, c_intunion);
        /// assert_eq!(c_intunion.get(), u32::MAX);
        /// 
        /// let d_intunion = c_intunion.wrapping_add(1_u32.into_intunion());
        /// println!("{} + 1 = {}", a_intunion, d_intunion);
        /// assert_eq!(d_intunion.get(), 0_u32);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, LongUnion };
        /// let a_longunion = LongUnion::new_with(u64::MAX - 55_u64);
        /// let b_longunion = LongUnion::new_with(55);
        /// let c_longunion = a_longunion.wrapping_add(b_longunion);
        /// println!("{} + {} = {}", a_longunion, b_longunion, c_longunion);
        /// assert_eq!(c_longunion.get(), u64::MAX);
        /// 
        /// let d_longunion = c_longunion.wrapping_add(1_u32.into_longunion());
        /// println!("{} + 1 = {}", a_longunion, d_longunion);
        /// assert_eq!(d_longunion.get(), 0_u64);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, LongerUnion };
        /// let a_longerunion = LongerUnion::new_with(u128::MAX - 55_u128);
        /// let b_longerunion = LongerUnion::new_with(55);
        /// let c_longerunion = a_longerunion.wrapping_add(b_longerunion);
        /// println!("{} + {} = {}", a_longerunion, b_longerunion, c_longerunion);
        /// assert_eq!(c_longerunion.get(), u128::MAX);
        /// 
        /// let d_longerunion = c_longerunion.wrapping_add(1_u128.into_longerunion());
        /// println!("{} + 1 = {}", a_longerunion, d_longerunion);
        /// assert_eq!(d_longerunion.get(), 0_u128);
        /// ```
        /// 
        /// # Example 5 for SizeUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, SizeUnion };
        /// let a_sizeunion = SizeUnion::new_with(usize::MAX - 55_usize);
        /// let b_sizeunion = SizeUnion::new_with(55);
        /// let c_sizeunion = a_sizeunion.wrapping_add(b_sizeunion);
        /// println!("{} + {} = {}", a_sizeunion, b_sizeunion, c_sizeunion);
        /// assert_eq!(c_sizeunion.get(), usize::MAX);
        /// 
        /// let d_sizeunion = c_sizeunion.wrapping_add(1_usize.into_sizeunion());
        /// println!("{} + 1 = {}", a_sizeunion, d_sizeunion);
        /// assert_eq!(d_sizeunion.get(), 0_usize);
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
        #[inline] pub fn wrapping_add(self, rhs: Self) -> Self      { Self::new_with( self.get().wrapping_add(rhs.get()) ) }

        // pub fn overflowing_add(self, rhs: Self) -> (Self, bool)
        /// Calculates `self` + `rhs`, wrapping around at the boundary of the type.
        /// 
        /// # Arguments
        /// - `rhs` is the operand of `Self` type.
        /// 
        /// # Features
        /// It adds two numbers with wrapping (modular) addition. It is the same
        /// as the method carrying_add() with the imput carry which is false.
        /// 
        /// # Output
        /// It returns a tuple of the addition along with a boolean indicating
        /// whether an arithmetic overflow would occur. If an overflow would
        /// have occurred then the wrapped value is returned.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, ShortUnion };
        /// let a_shortunion = ShortUnion::new_with(u16::MAX - 55_u16);
        /// let (b_shortunion, overflow) = a_shortunion.overflowing_add(55_u16.into_shortunion());
        /// println!("{} + 55 = {}\nOverflow = {}", a_shortunion, b_shortunion, overflow);
        /// assert_eq!(b_shortunion.get(), u16::MAX);
        /// assert_eq!(overflow, false);
        /// 
        /// let (c_shortunion, overflow) = b_shortunion.overflowing_add(1_u16.into_shortunion());
        /// println!("{} + 1 = {}\nOverflow = {}", b_shortunion, c_shortunion, overflow);
        /// assert_eq!(c_shortunion.get(), 0_u16);
        /// assert_eq!(overflow, true);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, IntUnion };
        /// let a_intunion = IntUnion::new_with(u32::MAX - 55_u32);
        /// let (b_intunion, overflow) = a_intunion.overflowing_add(55_u32.into_intunion());
        /// println!("{} + 55 = {}\nOverflow = {}", a_intunion, b_intunion, overflow);
        /// assert_eq!(b_intunion.get(), u32::MAX);
        /// assert_eq!(overflow, false);
        ///    
        /// let (c_intunion, overflow) = b_intunion.overflowing_add(1_u32.into_intunion());
        /// println!("{} + 1 = {}\nOverflow = {}", b_intunion, c_intunion, overflow);
        /// assert_eq!(c_intunion.get(), 0_u32);
        /// assert_eq!(overflow, true);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, LongUnion };
        /// let a_longunion = LongUnion::new_with(u64::MAX - 55_u64);
        /// let (b_longunion, overflow) = a_longunion.overflowing_add(55_u64.into_longunion());
        /// println!("{} + 55 = {}\nOverflow = {}", a_longunion, b_longunion, overflow);
        /// assert_eq!(b_longunion.get(), u64::MAX);
        /// assert_eq!(overflow, false);
        ///    
        /// let (c_longunion, overflow) = b_longunion.overflowing_add(1_u64.into_longunion());
        /// println!("{} + 1 = {}\nOverflow = {}", b_longunion, c_longunion, overflow);
        /// assert_eq!(c_longunion.get(), 0_u64);
        /// assert_eq!(overflow, true);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, LongerUnion };
        /// let a_longerunion = LongerUnion::new_with(u128::MAX - 55_u128);
        /// let (b_longerunion, overflow) = a_longerunion.overflowing_add(55_u128.into_longerunion());
        /// println!("{} + 55 = {}\nOverflow = {}", a_longerunion, b_longerunion, overflow);
        /// assert_eq!(b_longerunion.get(), u128::MAX);
        /// assert_eq!(overflow, false);
        ///    
        /// let (c_longerunion, overflow) = b_longerunion.overflowing_add(1_u128.into_longerunion());
        /// println!("{} + 1 = {}\nOverflow = {}", b_longerunion, c_longerunion, overflow);
        /// assert_eq!(c_longerunion.into_u128(), 0_u128);
        /// assert_eq!(overflow, true);
        /// ```
        /// 
        /// # Example 5 for SizeUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, SizeUnion };
        /// let a_sizeunion = SizeUnion::new_with(usize::MAX - 55_usize);
        /// let (b_sizeunion, overflow) = a_sizeunion.overflowing_add(55_usize.into_sizeunion());
        /// println!("{} + 55 = {}\nOverflow = {}", a_sizeunion, b_sizeunion, overflow);
        /// assert_eq!(b_sizeunion.get(), usize::MAX);
        /// assert_eq!(overflow, false);
        ///    
        /// let (c_sizeunion, overflow) = b_sizeunion.overflowing_add(1_usize.into_sizeunion());
        /// println!("{} + 1 = {}\nOverflow = {}", b_sizeunion, c_sizeunion, overflow);
        /// assert_eq!(c_sizeunion.get(), 0_usize);
        /// assert_eq!(overflow, true);
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
        pub fn overflowing_add(self, rhs: Self) -> (Self, bool)
        {
            let (res_this, carry) = self.get().overflowing_add(rhs.get());
            (Self::new_with(res_this), carry)
        }

        // fn checked_add(self, rhs: Self) -> Option<Self>;
        /// Computes `self` + `rhs`.
        /// 
        /// # Arguments
        /// - `rhs` is the operand of `Self` type.
        /// 
        /// # Output
        /// It returns self + rhs in the type `Self` wrapped by `Some`
        /// of enum `Option` if overflow did not occur.
        /// And, it returns `None` if overflow occurred.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, ShortUnion };
        /// let a_shortunion = ShortUnion::new_with(u16::MAX - 55_u16);
        /// let b_shortunion = 55_u16.into_shortunion();
        /// let c_shortunion = a_shortunion.checked_add(b_shortunion);
        /// match c_shortunion
        /// {
        ///     Some(c) => {
        ///             println!("{} + 55 = {}", a_shortunion, c);
        ///             assert_eq!(c.get(), u16::MAX);
        ///         },
        ///     None => { println!("Overflow happened."); },
        /// }
        /// 
        /// let d_shortunion = c_shortunion.unwrap().checked_add(1_u16.into_shortunion());
        /// match d_shortunion
        /// {
        ///     Some(d) => { println!("{} + 1 = {}", a_shortunion, d); },
        ///     None => {
        ///             println!("Overflow happened.");
        ///             assert_eq!(d_shortunion, None);
        ///         },
        /// }
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, IntUnion };
        /// let a_intunion = IntUnion::new_with(u32::MAX - 55_u32);
        /// let b_intunion = 55_u32.into_intunion();
        /// let c_intunion = a_intunion.checked_add(b_intunion);
        /// match c_intunion
        /// {
        ///     Some(c) => {
        ///             println!("{} + 55 = {}", a_intunion, c);
        ///             assert_eq!(c.get(), u32::MAX);
        ///         },
        ///     None => { println!("Overflow happened."); },
        /// }
        /// 
        /// let d_intunion = c_intunion.unwrap().checked_add(1_u32.into_intunion());
        /// match d_intunion
        /// {
        ///     Some(d) => { println!("{} + 1 = {}", a_intunion, d); },
        ///     None => {
        ///             println!("Overflow happened.");
        ///             assert_eq!(d_intunion, None);
        ///         },
        /// }
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, LongUnion };
        /// let a_longunion = LongUnion::new_with(u64::MAX - 55_u64);
        /// let b_longunion = 55_u64.into_longunion();
        /// let c_longunion = a_longunion.checked_add(b_longunion);
        /// match c_longunion
        /// {
        ///     Some(c) => {
        ///             println!("{} + 55 = {}", a_longunion, c);
        ///             assert_eq!(c.get(), u64::MAX);
        ///         },
        ///     None => { println!("Overflow happened."); },
        /// }
        /// 
        /// let d_longunion = c_longunion.unwrap().checked_add(1_u64.into_longunion());
        /// match d_longunion
        /// {
        ///     Some(d) => { println!("{} + 1 = {}", a_longunion, d); },
        ///     None => {
        ///             println!("Overflow happened.");
        ///             assert_eq!(d_longunion, None);
        ///         },
        /// }
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, LongerUnion };
        /// let a_longerunion = LongerUnion::new_with(u128::MAX - 55_u128);
        /// let b_longerunion = 55_u128.into_longerunion();
        /// let c_longerunion = a_longerunion.checked_add(b_longerunion);
        /// match c_longerunion
        /// {
        ///     Some(c) => {
        ///             println!("{} + 55 = {}", a_longerunion, c);
        ///             assert_eq!(c.get(), u128::MAX);
        ///         },
        ///     None => { println!("Overflow happened."); },
        /// }
        /// 
        /// let d_longerunion = c_longerunion.unwrap().checked_add(1_u128.into_longerunion());
        /// match d_longerunion
        /// {
        ///     Some(d) => { println!("{} + 1 = {}", a_longerunion, d); },
        ///     None => {
        ///             println!("Overflow happened.");
        ///             assert_eq!(d_longerunion, None);
        ///         },
        /// }
        /// ```
        /// 
        /// # Example 5 for SizeUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, SizeUnion };
        /// let a_sizeunion = SizeUnion::new_with(usize::MAX - 55_usize);
        /// let b_sizeunion = 55_usize.into_sizeunion();
        /// let c_sizeunion = a_sizeunion.checked_add(b_sizeunion);
        /// match c_sizeunion
        /// {
        ///     Some(c) => {
        ///             println!("{} + 55 = {}", a_sizeunion, c);
        ///             assert_eq!(c.get(), usize::MAX);
        ///         },
        ///     None => { println!("Overflow happened."); },
        /// }
        /// 
        /// let d_sizeunion = c_sizeunion.unwrap().checked_add(1_usize.into_sizeunion());
        /// match d_sizeunion
        /// {
        ///     Some(d) => { println!("{} + 1 = {}", a_sizeunion, d); },
        ///     None => {
        ///             println!("Overflow happened.");
        ///             assert_eq!(d_sizeunion, None);
        ///         },
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
        pub fn checked_add(self, rhs: Self) -> Option<Self>
        {
            match self.get().checked_add(rhs.get())
            {
                Some(res_this) =>   { Some(Self::new_with(res_this)) },
                None =>             { None },
            }
        }

        // fn unchecked_add(self, rhs: Self) -> Self;
        /// Computes `self` + `rhs`, assuming overflow cannot occur.
        /// 
        /// # Arguments
        /// `rhs` is the operand of `Self` type.
        /// 
        /// # Features
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
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// let a_shortunion = ShortUnion::new_with(u16::MAX - 55_u16);
        /// let b_shortunion = ShortUnion::new_with(55_u16);
        /// let c_shortunion = a_shortunion.saturating_add(b_shortunion);
        /// println!("{} + 55 = {}", a_shortunion, c_shortunion);
        /// assert_eq!(c_shortunion.get(), u16::MAX);
        /// 
        /// // It will panic
        /// // let d_shortunion = small_uint_unchecked_add_func(c_shortunion, 1_u16);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// let a_intunion = IntUnion::new_with(u32::MAX - 55_u32);
        /// let b_intunion = IntUnion::new_with(55_u32);
        /// let c_intunion = a_intunion.saturating_add(b_intunion);
        /// println!("{} + 55 = {}", a_intunion, c_intunion);
        /// assert_eq!(c_intunion.get(), u32::MAX);
        /// 
        /// // It will panic
        /// // let d_intunion = small_uint_unchecked_add_func(c_intunion, 1_u32);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let a_longunion = LongUnion::new_with(u64::MAX - 55_u64);
        /// let b_longunion = LongUnion::new_with(55_u64);
        /// let c_longunion = a_longunion.saturating_add(b_longunion);
        /// println!("{} + 55 = {}", a_longunion, c_longunion);
        /// assert_eq!(c_longunion.get(), u64::MAX);
        /// 
        /// // It will panic
        /// // let d_longunion = small_uint_unchecked_add_func(c_longunion, 1_u64);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let a_longerunion = LongerUnion::new_with(u128::MAX - 55_u128);
        /// let b_longerunion = LongerUnion::new_with(55_u128);
        /// let c_longerunion = a_longerunion.saturating_add(b_longerunion);
        /// println!("{} + 55 = {}", a_longerunion, c_longerunion);
        /// assert_eq!(c_longerunion.get(), u128::MAX);
        /// 
        /// // It will panic
        /// // let d_longerunion = small_uint_unchecked_add_func(c_longerunion, 1_u128);
        /// ```
        /// 
        /// # Example 5 for SizeUnion
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// let a_sizeunion = SizeUnion::new_with(usize::MAX - 55_usize);
        /// let b_sizeunion = SizeUnion::new_with(55_usize);
        /// let c_sizeunion = a_sizeunion.saturating_add(b_sizeunion);
        /// println!("{} + 55 = {}", a_sizeunion, c_sizeunion);
        /// assert_eq!(c_sizeunion.get(), usize::MAX);
        /// 
        /// // It will panic
        /// // let d_sizeunion = small_uint_unchecked_add_func(c_sizeunion, 1_usize);
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
        #[inline] pub fn unchecked_add(self, rhs: Self) -> Self     { Self::new_with( self.get().checked_add(rhs.get()).unwrap() ) }

        // fn saturating_add(self, rhs: Self) -> Self;
        /// Computes `self` + `rhs`, saturating at the numeric bounds
        /// instead of overflowing.
        /// 
        /// # Arguments
        /// `rhs` is the operand of `Self` type.
        /// 
        /// # Features
        /// It adds two numbers with saturating integer addition
        /// 
        /// # Output
        /// It returns the smaller one of `self` + `rhs` and the maxium
        /// of the type of `Self`.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// let a_shortunion = ShortUnion::new_with(u16::MAX - 55_u16);
        /// let b_shortunion = ShortUnion::new_with(55_u16);
        /// let c_shortunion = a_shortunion.saturating_add(b_shortunion);
        /// println!("{} + 55 = {}", a_shortunion, c_shortunion);
        /// assert_eq!(c_shortunion.get(), u16::MAX);
        /// 
        /// let d_shortunion = c_shortunion.saturating_add(b_shortunion);
        /// println!("{} + 55 = {}", c_shortunion, d_shortunion);
        /// assert_eq!(d_shortunion.get(), u16::MAX);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// let a_intunion = IntUnion::new_with(u32::MAX - 55_u32);
        /// let b_intunion = IntUnion::new_with(55_u32);
        /// let c_intunion = a_intunion.saturating_add(b_intunion);
        /// println!("{} + 55 = {}", a_intunion, c_intunion);
        /// assert_eq!(c_intunion.get(), u32::MAX);
        /// 
        /// let d_intunion = c_intunion.saturating_add(b_intunion);
        /// println!("{} + 55 = {}", c_intunion, d_intunion);
        /// assert_eq!(d_intunion.get(), u32::MAX);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let a_longunion = LongUnion::new_with(u64::MAX - 55_u64);
        /// let b_longunion = LongUnion::new_with(55_u64);
        /// let c_longunion = a_longunion.saturating_add(b_longunion);
        /// println!("{} + 55 = {}", a_longunion, c_longunion);
        /// assert_eq!(c_longunion.get(), u64::MAX);
        /// 
        /// let d_longunion = c_longunion.saturating_add(b_longunion);
        /// println!("{} + 55 = {}", c_longunion, d_longunion);
        /// assert_eq!(d_longunion.get(), u64::MAX);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let a_longerunion = LongerUnion::new_with(u128::MAX - 55_u128);
        /// let b_longerunion = LongerUnion::new_with(55_u128);
        /// let c_longerunion = a_longerunion.saturating_add(b_longerunion);
        /// println!("{} + 55 = {}", a_longerunion, c_longerunion);
        /// assert_eq!(c_longerunion.get(), u128::MAX);
        /// 
        /// let d_longerunion = c_longerunion.saturating_add(b_longerunion);
        /// println!("{} + 55 = {}", c_longerunion, d_longerunion);
        /// assert_eq!(d_longerunion.get(), u128::MAX);
        /// ```
        /// 
        /// # Example 5 for SizeUnion
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// let a_sizeunion = SizeUnion::new_with(usize::MAX - 55_usize);
        /// let b_sizeunion = SizeUnion::new_with(55_usize);
        /// let c_sizeunion = a_sizeunion.saturating_add(b_sizeunion);
        /// println!("{} + 55 = {}", a_sizeunion, c_sizeunion);
        /// assert_eq!(c_sizeunion.get(), usize::MAX);
        /// 
        /// let d_sizeunion = c_sizeunion.saturating_add(b_sizeunion);
        /// println!("{} + 55 = {}", c_sizeunion, d_sizeunion);
        /// assert_eq!(d_sizeunion.get(), usize::MAX);
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
        #[inline] pub fn saturating_add(self, rhs: Self) -> Self    { Self::new_with( self.get().saturating_add(rhs.get()) ) }


        /***** SUBTRACTION *****/

        // fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool);
        /// Calculates `self` - `rhs` - `borrow`,
        /// wrapping around at the boundary of the type.
        /// 
        /// # Arguments
        /// - `rhs` is the operand of `Self` type.
        /// - `borrow` is the borrow overflowed from the previous operation.
        /// If there is no overflowed borrow from the previous operation,
        /// `borrow` is `false`. Otherwise, it is `true`.
        /// 
        /// # Features
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
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// // a_u32: u32 === (a_high_shortunion, a_low_shortunion) == (50000_u16, 30000_u16) == 3276830000_u32
        /// let a_high_shortunion = ShortUnion::new_with(50000_u16);
        /// let a_low_shortunion = ShortUnion::new_with(30000_u16);
        /// // b_u32: u32 === (b_high_shortunion, b_low_shortunion) == (10000_u16, 10100_u16) == 655370100_u32
        /// let b_high_shortunion = ShortUnion::new_with(10000_u16);
        /// let b_low_shortunion = ShortUnion::new_with(10100_u16);
        /// 
        /// // (50000_u16, 30000_u16) - (10000_u16, 10100_u16) == 3276830000_u32 - 655370100_u32 == 99_u16
        /// //   3276830000_u32 == (50000_u16, 30000_u16)
        /// // -  655370100_u32 == (10000_u16, 10100_u16)
        /// // ------------------------------------------
        /// //   2621459900_u32 == (40000_u16, 19900_u16)
        /// 
        /// // c: u32 === (c_high_shortunion, c_low_shortunion)
        /// let (c_low_shortunion, borrow) = a_low_shortunion.borrowing_sub(b_low_shortunion, false);
        /// let (c_high_shortunion, borrow) = a_high_shortunion.borrowing_sub(b_high_shortunion, borrow);
        /// println!("{}-{}, {}", c_high_shortunion, c_low_shortunion, borrow);
        /// assert_eq!(c_high_shortunion.get(), 40000_u16);
        /// assert_eq!(c_low_shortunion.get(), 19900_u16);
        /// assert_eq!(borrow, false);
        /// 
        /// // (10000_u16, 10100_u16) - (50000_u16, 30000_u16) == 655370100_u32 - 3276830000_u32 == 51501_u16
        /// //    655370100_u32 == (10000_u16, 10100_u16)
        /// // - 3276830000_u32 == (50000_u16, 30000_u16)
        /// // ------------------------------------------
        /// //   1673507396_u32 == (25535_u16, 45636_u16)
        /// 
        /// // d: u32 === (d_high_shortunion, d_low_shortunion)
        /// let (d_low_shortunion, borrow) = b_low_shortunion.borrowing_sub(a_low_shortunion, false);
        /// let (d_high_shortunion, borrow) = b_high_shortunion.borrowing_sub(a_high_shortunion, borrow);
        /// println!("{}-{}, {}", d_high_shortunion, d_low_shortunion, borrow);
        /// assert_eq!(d_high_shortunion.get(), 25535_u16);
        /// assert_eq!(d_low_shortunion.get(), 45636_u16);
        /// assert_eq!(borrow, true);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// // a_u64: u64 === (a_high_intunion, a_low_intunion) == (2299561912_u32, 2956226837_u32) == 9876543210123456789_u64
        /// let a_high_intunion = IntUnion::new_with(2299561912_u32);
        /// let a_low_intunion = IntUnion::new_with(2956226837_u32);
        /// // b_u64: u64 === (b_high_intunion, b_low_intunion) == (1782160508_u32, 682685733_u32) == 7654321098765432101_u64
        /// let b_high_intunion = IntUnion::new_with(1782160508_u32);
        /// let b_low_intunion = IntUnion::new_with(682685733_u32);
        /// 
        /// // (2299561912_u32, 2956226837_u32) - (1782160508_u32, 682685733_u32) == 9876543210123456789_u64 - 7654321098765432101_u64 == 2222222111358024688_u64
        /// //   9876543210123456789_u64 == (2299561912_u32, 2956226837_u32)
        /// // - 7654321098765432101_u64 == (1782160508_u32,  682685733_u32)
        /// // -------------------------------------------------------------
        /// //   2222222111358024688_u64 == ( 517401404_u32, 2273541104_u32)
        /// 
        /// // c: u64 === (c_high_intunion, c_low_intunion)
        /// let (c_low_intunion, borrow) = a_low_intunion.borrowing_sub(b_low_intunion, false);
        /// let (c_high_intunion, borrow) = a_high_intunion.borrowing_sub(b_high_intunion, borrow);
        /// println!("{}-{}, {}", c_high_intunion, c_low_intunion, borrow);
        /// assert_eq!(c_high_intunion.get(), 517401404_u32);
        /// assert_eq!(c_low_intunion.get(), 2273541104_u32);
        /// assert_eq!(borrow, false);
        /// 
        /// // (517401404_u32, 2273541104_u32) - (1782160508_u32,  682685733_u32) == 2222222111358024688_u32 - 7654321098765432101_u32 == 13014645086302144203_u16
        /// //   2222222111358024688_u64 == ( 517401404_u32, 2273541104_u32)
        /// // - 7654321098765432101_u64 == (1782160508_u32,  682685733_u32)
        /// // -------------------------------------------------------------
        /// //  13014645086302144203_u64 == (3030208192_u32, 1590855371_u32)
        /// 
        /// // d: u64 === (d_high_intunion, d_low_intunion)
        /// let (d_low_intunion, borrow) = c_low_intunion.borrowing_sub(b_low_intunion, false);
        /// let (d_high_intunion, borrow) = c_high_intunion.borrowing_sub(b_high_intunion, borrow);
        /// println!("{}-{}, {}", d_high_intunion, d_low_intunion, borrow);
        /// assert_eq!(d_high_intunion.get(), 3030208192_u32);
        /// assert_eq!(d_low_intunion.get(), 1590855371_u32);
        /// assert_eq!(borrow, true);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// // a_u128: u128 === (a_high_longunion, a_low_longunion) == (10775095670246085798_u64, 7681743649119882630_u64) == 198765432198765432198765432198765432198_u128
        /// let a_high_longunion = LongUnion::new_with(10775095670246085798_u64);
        /// let a_low_longunion = LongUnion::new_with(7681743649119882630_u64);
        /// // b_u128: u128 === (b_high_longunion, b_low_longunion) == (6692605942763486917_u64, 12312739301371248917_u64) == 123456789012345678901234567890123456789_u128
        /// let b_high_longunion = LongUnion::new_with(6692605942763486917_u64);
        /// let b_low_longunion = LongUnion::new_with(12312739301371248917_u64);
        /// 
        /// // (10775095670246085798_u64, 7681743649119882630_u64) - (6692605942763486917_u64, 12312739301371248917_u64) == 198765432198765432198765432198765432198_u128 - 123456789012345678901234567890123456789_u128 == 75308643186419753297530864308641975409_u128
        /// //   198765432198765432198765432198765432198_u128 == (10775095670246085798_u64,  7681743649119882630_u64)
        /// // - 123456789012345678901234567890123456789_u128 == ( 6692605942763486917_u64, 12312739301371248917_u64)
        /// // ------------------------------------------------------------------------------------------------------
        /// //    75308643186419753297530864308641975409_u128 == (10775095670246085798_u64,  7681743649119882630_u64)
        /// 
        /// // c: u32 === (c_high_u16, c_low_u16)
        /// let (c_low_longunion, borrow) = a_low_longunion.borrowing_sub(b_low_longunion, false);
        /// let (c_high_longunion, borrow) = a_high_longunion.borrowing_sub(b_high_longunion, borrow);
        /// println!("{}-{}, {}", c_high_longunion, c_low_longunion, borrow);
        /// assert_eq!(c_high_longunion.get(), 4082489727482598880_u64);
        /// assert_eq!(c_low_longunion.get(), 13815748421458185329_u64);
        /// assert_eq!(borrow, false);
        /// 
        /// // (10775095670246085798_u64, 7681743649119882630_u64) - (6692605942763486917_u64, 12312739301371248917_u64) == 75308643186419753297530864308641975409_u128 - 123456789012345678901234567890123456789_u128 == 292134221095012537859670903850286730076_u128
        /// //    75308643186419753297530864308641975409_u128 == (10775095670246085798_u64,  7681743649119882630_u64)
        /// // - 123456789012345678901234567890123456789_u128 == ( 6692605942763486917_u64, 12312739301371248917_u64)
        /// // ------------------------------------------------------------------------------------------------------
        /// //   292134221095012537859670903850286730076_u128 == (15836627858428663579_u64,  1503009120086936412_u64)
        /// 
        /// // d: u128 === (d_high_u64, d_low_u64)
        /// let (d_low_longunion, borrow) = c_low_longunion.borrowing_sub(b_low_longunion, false);
        /// let (d_high_longunion, borrow) = c_high_longunion.borrowing_sub(b_high_longunion, borrow);
        /// println!("{}-{}, {}", d_high_longunion, d_low_longunion, borrow);
        /// assert_eq!(d_high_longunion.get(), 15836627858428663579_u64);
        /// assert_eq!(d_low_longunion.get(), 1503009120086936412_u64);
        /// assert_eq!(borrow, true);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// //   4201016837757989640311993609423984479246482890531986660185 == (12345678901234567890_u128, 6789012345678912345_u128)
        /// // -                 419908440780438063913804265570801972943493 == (                1234_u128,                6789_u128)
        /// // ---------------------------------------------------------------------------------------------------------------------
        /// //   4201016837757989220403552828985920565442217319730013716692 == (12345678901234566656_u128, 6789012345678905556_u128)
        /// 
        /// // a_u256: u256 === (a_high_longerunion, a_low_longerunion)
        /// let (a_low_longerunion, borrow) = LongerUnion::new_with(6789012345678912345_u128).borrowing_sub(LongerUnion::new_with(6789_u128), false);
        /// let (a_high_longerunion, borrow) = LongerUnion::new_with(12345678901234567890_u128).borrowing_sub(LongerUnion::new_with(1234_u128), borrow);
        /// println!("{}-{}, {}", a_low_longerunion, a_high_longerunion, borrow);
        /// assert_eq!(a_high_longerunion.get(), 12345678901234566656_u128);
        /// assert_eq!(a_low_longerunion.get(), 6789012345678905556_u128);
        /// assert_eq!(borrow, false);
        /// 
        /// //    57896044618658097711785492504343953926307055644800578124155540853313808954190 == (170141183460469231731687303715884105727_u128,  12345678901234567890123456789012345678_u128)
        /// // - 308778904632843187796189293356501087608549893209439890708590319850715068122315 == (226854911280625642308916404954512140970_u128,  56789012345678912345678901234567890123_u128)
        /// // --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
        /// //   328077586172395887033451124191282405584107085763563507612853141042164389031555 == (283568639100782052886145506193140176212_u128, 295839033476494119007819162986212667011_u128)
        /// 
        /// // b_u256: u256 === (b_high_longerunion, b_low_longerunion)
        /// let (b_low_longerunion, borrow) = LongerUnion::new_with(12345678901234567890123456789012345678_u128).borrowing_sub(LongerUnion::new_with(56789012345678912345678901234567890123_u128), false);
        /// let (b_high_longerunion, borrow) = LongerUnion::new_with(170141183460469231731687303715884105727_u128).borrowing_sub(LongerUnion::new_with(226854911280625642308916404954512140970_u128), borrow);
        /// println!("{}-{}, {}", b_high_longerunion, b_low_longerunion, borrow);
        /// assert_eq!(b_high_longerunion.get(), 283568639100782052886145506193140176212_u128);
        /// assert_eq!(b_low_longerunion.get(), 295839033476494119007819162986212667011_u128);
        /// assert_eq!(borrow, true);
        /// ```
        /// 
        /// # Example 5 for SizeUnion for 64-bit CPU
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// // a_u128: u128 === (a_high_usize, a_low_usize) == (10775095670246085798_usize, 7681743649119882630_usize) == 198765432198765432198765432198765432198_u128
        /// let a_high_sizeunion = SizeUnion::new_with(10775095670246085798_usize);
        /// let a_low_sizeunion = SizeUnion::new_with(7681743649119882630_usize);
        /// // b_u128: u128 === (b_high_usize, b_low_usize) == (6692605942763486917_usize, 12312739301371248917_usize) == 123456789012345678901234567890123456789_u128
        /// let b_high_sizeunion = SizeUnion::new_with(6692605942763486917_usize);
        /// let b_low_sizeunion = SizeUnion::new_with(12312739301371248917_usize);
        /// 
        /// // (10775095670246085798_usize, 7681743649119882630_usize) - (6692605942763486917_usize, 12312739301371248917_usize) == 198765432198765432198765432198765432198_u128 - 123456789012345678901234567890123456789_u128 == 75308643186419753297530864308641975409_u128
        /// //   198765432198765432198765432198765432198_u128 == (10775095670246085798_usize,  7681743649119882630_usize)
        /// // - 123456789012345678901234567890123456789_u128 == ( 6692605942763486917_usize, 12312739301371248917_usize)
        /// // ------------------------------------------------------------------------------------------------------
        /// //    75308643186419753297530864308641975409_u128 == ( 4082489727482598880_usize, 13815748421458185329_usize)
        /// 
        /// // c: u128 === (c_high_usize, c_low_usize)
        /// let (c_low_sizeunion, borrow) = a_low_sizeunion.borrowing_sub(b_low_sizeunion, false);
        /// let (c_high_sizeunion, borrow) = a_high_sizeunion.borrowing_sub(b_high_sizeunion, borrow);
        /// println!("{}-{}, {}", c_high_sizeunion, c_low_sizeunion, borrow);
        /// assert_eq!(c_high_sizeunion.get(), 4082489727482598880_usize);
        /// assert_eq!(c_low_sizeunion.get(), 13815748421458185329_usize);
        /// assert_eq!(borrow, false);
        /// 
        /// // (4082489727482598880_usize, 13815748421458185329_usize) - (6692605942763486917_usize, 12312739301371248917_usize) == 75308643186419753297530864308641975409_u128 - 123456789012345678901234567890123456789_u128 == 292134221095012537859670903850286730076_u128
        /// //    75308643186419753297530864308641975409_u128 == ( 4082489727482598880_usize, 13815748421458185329_usize)
        /// // - 123456789012345678901234567890123456789_u128 == ( 6692605942763486917_usize, 12312739301371248917_usize)
        /// // ------------------------------------------------------------------------------------------------------
        /// //   292134221095012537859670903850286730076_u128 == (14364254346226952735_usize,  4630995652251366287_usize)
        /// 
        /// // d: u128 === (d_high_usize, d_low_usize)
        /// let (d_low_sizeunion, borrow) = c_low_sizeunion.borrowing_sub(b_low_sizeunion, false);
        /// let (d_high_sizeunion, borrow) = c_high_sizeunion.borrowing_sub(b_high_sizeunion, borrow);
        /// println!("{}-{}, {}", d_high_sizeunion, d_low_sizeunion, borrow);
        /// assert_eq!(d_high_sizeunion.get(), 15836627858428663579_usize);
        /// assert_eq!(d_low_sizeunion.get(), 1503009120086936412_usize);
        /// assert_eq!(borrow, true);
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
        pub fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool)
        {
            let (r_this, b1) = self.get().overflowing_sub(rhs.get());
            let (res_this, b2) = r_this.overflowing_sub(borrow as $f);
            (Self::new_with(res_this), b1 || b2)
        }

        // fn wrapping_sub(self, rhs: Self) -> Self;
        /// Computes `self` - `rhs`, wrapping around at the boundary of the type.
        /// 
        /// # Arguments
        /// `rhs` is the subtractor from `self`.
        /// 
        /// # Features
        /// It subtracts rhs from self with wrapping (modular) subtraction.
        /// 
        /// # Output
        /// It returns the `self` - `rhs` in the type of `Self`.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// 
        /// let a_shortunion = ShortUnion::new_with(55_u16).wrapping_sub(ShortUnion::new_with(55_u16));
        /// println!("55 - 55 = {}", a_shortunion);
        /// assert_eq!(a_shortunion.get(), 0_u16);
        /// 
        /// let b_shortunion = a_shortunion.wrapping_sub(ShortUnion::new_with(1_u16));
        /// println!("{} - 1 = {}", a_shortunion, b_shortunion);
        /// assert_eq!(b_shortunion.get(), u16::MAX);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// 
        /// let a_intunion = IntUnion::new_with(55_u32).wrapping_sub(IntUnion::new_with(55_u32));
        /// println!("55 - 55 = {}", a_intunion);
        /// assert_eq!(a_intunion.get(), 0_u32);
        /// 
        /// let b_intunion = a_intunion.wrapping_sub(IntUnion::new_with(1_u32));
        /// println!("{} - 1 = {}", a_intunion, b_intunion);
        /// assert_eq!(b_intunion.get(), u32::MAX);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// 
        /// let a_longunion = LongUnion::new_with(55_u64).wrapping_sub(LongUnion::new_with(55_u64));
        /// println!("55 - 55 = {}", a_longunion);
        /// assert_eq!(a_longunion.get(), 0_u64);
        /// 
        /// let b_longunion = a_longunion.wrapping_sub(LongUnion::new_with(1_u64));
        /// println!("{} - 1 = {}", a_longunion, b_longunion);
        /// assert_eq!(b_longunion.get(), u64::MAX);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// 
        /// let a_longerunion = LongerUnion::new_with(55_u128).wrapping_sub(LongerUnion::new_with(55_u128));
        /// println!("55 - 55 = {}", a_longerunion);
        /// assert_eq!(a_longerunion.get(), 0_u128);
        /// 
        /// let b_longerunion = a_longerunion.wrapping_sub(LongerUnion::new_with(1_u128));
        /// println!("{} - 1 = {}", a_longerunion, b_longerunion);
        /// assert_eq!(b_longerunion.get(), u128::MAX);
        /// ```
        /// 
        /// # Example 5 for SizeUnion
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// 
        /// let a_sizeunion = SizeUnion::new_with(55_usize).wrapping_sub(SizeUnion::new_with(55_usize));
        /// println!("55 - 55 = {}", a_sizeunion);
        /// assert_eq!(a_sizeunion.get(), 0_usize);
        /// 
        /// let b_sizeunion = a_sizeunion.wrapping_sub(SizeUnion::new_with(1_usize));
        /// println!("{} - 1 = {}", a_sizeunion, b_sizeunion);
        /// assert_eq!(b_sizeunion.get(), usize::MAX);
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
        #[inline] pub fn wrapping_sub(self, rhs: Self) -> Self      { Self::new_with( self.get().wrapping_sub(rhs.get()) ) }
        // #[inline] pub fn wrapping_sub_assign(&mut self, rhs: Self)  { self.set(self.get().wrapping_sub(rhs.get())); }


        // fn overflowing_sub(self, rhs: Self) -> (Self, bool);
        /// Calculates `self` - `rhs`, wrapping around at the boundary of the type.
        /// 
        /// # Arguments
        /// `rhs` is the subtractor from `self`.
        /// 
        /// # Features
        /// It subtracts rhs from self with wrapping (modular) subtraction.
        /// It is the same as the method borrowing_sub() with the imput carry which
        /// is false.
        /// 
        /// # Output
        /// It returns a tuple of the subtraction along with a boolean indicating
        /// whether an arithmetic underflow would occur. If an underflow would
        /// have occurred then the wrapped value is returned.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// 
        /// let (a_shortunion, overflow) = ShortUnion::new_with(55_u16).overflowing_sub(ShortUnion::new_with(55_u16));
        /// println!("55 - 55 = {}\nUnderflow = {}", a_shortunion, overflow);
        /// assert_eq!(a_shortunion.get(), 0_u16);
        /// assert_eq!(overflow, false);
        ///    
        /// let (b_shortunion, overflow) = a_shortunion.overflowing_sub(ShortUnion::new_with(1_u16));
        /// println!("{} - 1 = {}\nUnderflow = {}", b_shortunion, b_shortunion, overflow);
        /// assert_eq!(b_shortunion.get(), u16::MAX);
        /// assert_eq!(overflow, true);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// 
        /// let (a_intunion, overflow) = IntUnion::new_with(55_u32).overflowing_sub(IntUnion::new_with(55_u32));
        /// println!("55 - 55 = {}\nUnderflow = {}", a_intunion, overflow);
        /// assert_eq!(a_intunion.get(), 0_u32);
        /// assert_eq!(overflow, false);
        ///    
        /// let (b_intunion, overflow) = a_intunion.overflowing_sub(IntUnion::new_with(1_u32));
        /// println!("{} - 1 = {}\nUnderflow = {}", a_intunion, b_intunion, overflow);
        /// assert_eq!(b_intunion.get(), u32::MAX);
        /// assert_eq!(overflow, true);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// 
        /// let (a_longunion, overflow) = LongUnion::new_with(55_u64).overflowing_sub(LongUnion::new_with(55_u64));
        /// println!("55 - 55 = {}\nUnderflow = {}", a_longunion, overflow);
        /// assert_eq!(a_longunion.get(), 0_u64);
        /// assert_eq!(overflow, false);
        ///    
        /// let (b_longunion, overflow) = a_longunion.overflowing_sub(LongUnion::new_with(1_u64));
        /// println!("{} - 1 = {}\nUnderflow = {}", a_longunion, b_longunion, overflow);
        /// assert_eq!(b_longunion.get(), u64::MAX);
        /// assert_eq!(overflow, true);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// 
        /// let (a_longerunion, overflow) = LongerUnion::new_with(55_u128).overflowing_sub(LongerUnion::new_with(55_u128));
        /// println!("55 - 55 = {}\nUnderflow = {}", a_longerunion, a_longerunion);
        /// assert_eq!(a_longerunion.get(), 0_u128);
        /// assert_eq!(overflow, false);
        ///    
        /// let (b_longerunion, overflow) = a_longerunion.overflowing_sub(LongerUnion::new_with(1_u128));
        /// println!("{} - 1 = {}\nUnderflow = {}", a_longerunion, b_longerunion, overflow);
        /// assert_eq!(b_longerunion.get(), u128::MAX);
        /// assert_eq!(overflow, true);
        /// ```
        /// 
        /// # Example 5 for SizeUnion
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// 
        /// let (a_sizeunion, overflow) = SizeUnion::new_with(55_usize).overflowing_sub(SizeUnion::new_with(55_usize));
        /// println!("55 - 55 = {}\nUnderflow = {}", a_sizeunion, overflow);
        /// assert_eq!(a_sizeunion.get(), 0_usize);
        /// assert_eq!(overflow, false);
        ///    
        /// let (b_sizeunion, overflow) = a_sizeunion.overflowing_sub(SizeUnion::new_with(1_usize));
        /// println!("{} - 1 = {}\nUnderflow = {}", a_sizeunion, b_sizeunion, overflow);
        /// assert_eq!(b_sizeunion.get(), usize::MAX);
        /// assert_eq!(overflow, true);
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
        pub fn overflowing_sub(self, rhs: Self) -> (Self, bool)
        {
            let (res_this, borrow) = self.get().overflowing_sub(rhs.get());
            (Self::new_with(res_this), borrow)
        }

        // fn checked_sub(self, rhs: Self) -> Option<Self>;
        /// Computes `self` - `rhs`.
        /// 
        /// # Arguments
        /// `rhs` is the subtractor from `self`.
        /// 
        /// # Output
        /// It returns `self` - `rhs` in the type `Self` wrapped by `Some`
        /// of enum `Option` if overflow did not occur.
        /// And, it returns `None` if overflow occurred.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// 
        /// let a_shortunion = ShortUnion::new_with(55_u16).checked_sub(ShortUnion::new_with(55_u16));
        /// match a_shortunion
        /// {
        ///     Some(a) => {
        ///             println!("55 - 55 = {}", a);
        ///             assert_eq!(a.get(), 0_u16);
        ///         },
        ///     None => { println!("Underflow happened."); },
        /// }
        /// 
        /// let b_shortunion = a_shortunion.unwrap().checked_sub(ShortUnion::new_with(1_u16));
        /// match b_shortunion
        /// {
        ///     Some(b) => { println!("{} - 1 = {}", a_shortunion.unwrap(), b); },
        ///     None => {
        ///             println!("Underflow happened.");
        ///             assert_eq!(b_shortunion, None);
        ///         },
        /// }
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// 
        /// let a_intunion = IntUnion::new_with(55_u32).checked_sub(IntUnion::new_with(55_u32));
        /// match a_intunion
        /// {
        ///     Some(a) => {
        ///             println!("55 - 55 = {}", a);
        ///             assert_eq!(a.get(), 0_u32);
        ///         },
        ///     None => { println!("Underflow happened."); },
        /// }
        /// 
        /// let b_intunion = a_intunion.unwrap().checked_sub(IntUnion::new_with(1_u32));
        /// match b_intunion
        /// {
        ///     Some(b) => { println!("{} - 1 = {}", a_intunion.unwrap(), b); },
        ///     None => {
        ///             println!("Underflow happened.");
        ///             assert_eq!(b_intunion, None);
        ///         },
        /// }
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// 
        /// let a_longunion = LongUnion::new_with(55_u64).checked_sub(LongUnion::new_with(55_u64));
        /// match a_longunion
        /// {
        ///     Some(a) => {
        ///             println!("55 - 55 = {}", a);
        ///             assert_eq!(a.get(), 0_u64);
        ///         },
        ///     None => { println!("Underflow happened."); },
        /// }
        /// 
        /// let b_longunion = a_longunion.unwrap().checked_sub(LongUnion::new_with(1_u64));
        /// match b_longunion
        /// {
        ///     Some(b) => { println!("{} - 1 = {}", a_longunion.unwrap(), b); },
        ///     None => {
        ///             println!("Underflow happened.");
        ///             assert_eq!(b_longunion, None);
        ///         },
        /// }
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// 
        /// let a_longerunion = LongerUnion::new_with(55_u128).checked_sub(LongerUnion::new_with(55_u128));
        /// match a_longerunion
        /// {
        ///     Some(a) => {
        ///             println!("55 - 55 = {}", a);
        ///             assert_eq!(a.get(), 0_u128);
        ///         },
        ///     None => { println!("Underflow happened."); },
        /// }
        /// 
        /// let b_longerunion = a_longerunion.unwrap().checked_sub(LongerUnion::new_with(1_u128));
        /// match b_longerunion
        /// {
        ///     Some(b) => { println!("{} - 1 = {}", a_longerunion.unwrap(), b); },
        ///     None => {
        ///             println!("Underflow happened.");
        ///             assert_eq!(b_longerunion, None);
        ///         },
        /// }
        /// ```
        /// 
        /// # Example 5 for SizeUnion
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// 
        /// let a_sizeunion = SizeUnion::new_with(55_usize).checked_sub(SizeUnion::new_with(55_usize));
        /// match a_sizeunion
        /// {
        ///     Some(a) => {
        ///             println!("55 - 55 = {}", a);
        ///             assert_eq!(a.get(), 0_usize);
        ///         },
        ///     None => { println!("Underflow happened."); },
        /// }
        /// 
        /// let b_sizeunion = a_sizeunion.unwrap().checked_sub(SizeUnion::new_with(1_usize));
        /// match b_sizeunion
        /// {
        ///     Some(b) => { println!("{} - 1 = {}", a_sizeunion.unwrap(), b); },
        ///     None => {
        ///             println!("Underflow happened.");
        ///             assert_eq!(b_sizeunion, None);
        ///         },
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
        pub fn checked_sub(self, rhs: Self) -> Option<Self>
        {
            match self.get().checked_sub(rhs.get())
            {
                Some(res_this) =>   { Some(Self::new_with(res_this)) },
                None =>             { None },
            }
        }

        /// fn unchecked_sub(self, rhs: Self) -> Self;
        /// Computes `self` - `rhs`, assuming overflow cannot occur.
        /// 
        /// # Arguments
        /// `rhs` is the subtractor from `self`.
        /// 
        /// # Features
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
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// 
        /// let a_ushortunion = ShortUnion::new_with(55_u16).unchecked_sub(ShortUnion::new_with(55_u16));
        /// println!("55 - 55 = {}", a_ushortunion);
        /// assert_eq!(a_ushortunion.get(), 0_u16);
        /// 
        /// // It will panic
        /// // let b_ushortunion = ShortUnion::new_with(a_ushortunion.unchecked_sub(ShortUnion::new_with(1_u16);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// 
        /// let a_intunion = IntUnion::new_with(55_u32).unchecked_sub(IntUnion::new_with(55_u32));
        /// println!("55 - 55 = {}", a_intunion);
        /// assert_eq!(a_intunion.get(), 0_u32);
        /// 
        /// // It will panic
        /// // let b_intunion = IntUnion::new_with(a_intunion.unchecked_sub(IntUnion::new_with(1_u32));
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// 
        /// let a_longunion = LongUnion::new_with(55_u64).unchecked_sub(LongUnion::new_with(55_u64));
        /// println!("55 - 55 = {}", a_longunion);
        /// assert_eq!(a_longunion.get(), 0_u64);
        /// 
        /// // It will panic
        /// // let b_u64 = LongUnion::new_with(a_longunion.unchecked_sub(LongUnion::new_with(1_u64);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// 
        /// let a_longerunion = LongerUnion::new_with(55_u128).unchecked_sub(LongerUnion::new_with(55_u128));
        /// println!("55 - 55 = {}", a_longerunion);
        /// assert_eq!(a_longerunion.get(), 0_u128);
        /// 
        /// // It will panic
        /// // let b_longerunion = LongerUnion::new_with(a_longerunion.unchecked_sub(LongerUnion::new_with(1_u128));
        /// ```
        /// 
        /// # Example 5 for SizeUnion
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// 
        /// let a_sizeunion = SizeUnion::new_with(55_usize).unchecked_sub(SizeUnion::new_with(55_usize));
        /// println!("55 - 55 = {}", a_sizeunion);
        /// assert_eq!(a_sizeunion.get(), 0_usize);
        /// 
        /// // It will panic
        /// // let b_sizeunion = SizeUnion::new_with(a_sizeunion.unchecked_sub(SizeUnion::new_with(1_usize));
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
        #[inline] pub fn unchecked_sub(self, rhs: Self) -> Self     { Self::new_with( self.get().checked_sub(rhs.get()).unwrap() ) }

        // fn saturating_sub(self, rhs: Self) -> Self;
        /// Computes `self` - `rhs`, saturating at the numeric bounds
        /// instead of underflowing.
        /// 
        /// # Arguments
        /// `rhs` is the subtractor from `self`.
        /// 
        /// # Features
        /// It subtracts rhs from self with saturating integer subtraction.
        /// 
        /// # Output
        /// It returns the bigger one of `self` - `rhs` and the zero
        /// of the type of `Self`.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// 
        /// let a_shortunion = ShortUnion::new_with(55_u16).saturating_sub(ShortUnion::new_with(50_u16));
        /// println!("55 - 50 = {}", a_shortunion);
        /// assert_eq!(a_shortunion.get(), 5_u16);
        /// 
        /// let b_u16 = a_shortunion.saturating_sub(ShortUnion::new_with(55_u16));
        /// println!("5 - 55 = {}", b_u16);
        /// assert_eq!(b_u16.get(), 0_u16);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// 
        /// let a_intunion = IntUnion::new_with(55_u32).saturating_sub(IntUnion::new_with(50_u32));
        /// println!("55 - 50 = {}", a_intunion);
        /// assert_eq!(a_intunion.get(), 5_u32);
        /// 
        /// let b_intunion = a_intunion.saturating_sub(IntUnion::new_with(55_u32));
        /// println!("{} - 55 = {}", a_intunion, b_intunion);
        /// assert_eq!(b_intunion.get(), 0_u32);
        /// ```
        /// 
        /// # Example 3 for IntUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// 
        /// let a_longunion = LongUnion::new_with(55_u64).saturating_sub(LongUnion::new_with(50_u64));
        /// println!("55 - 50 = {}", a_longunion);
        /// assert_eq!(a_longunion.get(), 5_u64);
        /// 
        /// let b_longunion = a_longunion.saturating_sub(LongUnion::new_with(55_u64));
        /// println!("{} - 55 = {}", a_longunion, b_longunion);
        /// assert_eq!(b_longunion.get(), 0_u64);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let a_longerunion = LongerUnion::new_with(55_u128).saturating_sub(LongerUnion::new_with(50_u128));
        /// println!("55 - 50 = {}", a_longerunion);
        /// assert_eq!(a_longerunion.get(), 5_u128);
        /// 
        /// let b_longerunion = a_longerunion.saturating_sub(LongerUnion::new_with(55_u128));
        /// println!("{} - 55 = {}", a_longerunion, b_longerunion);
        /// assert_eq!(b_longerunion.get(), 0_u128);
        /// ```
        /// 
        /// # Example 5 for SizeUnion
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// 
        /// let a_sizeunion = SizeUnion::new_with(55_usize).saturating_sub(SizeUnion::new_with(50_usize));
        /// println!("55 - 50 = {}", a_sizeunion);
        /// assert_eq!(a_sizeunion.get(), 5_usize);
        /// 
        /// let b_sizeunion = a_sizeunion.saturating_sub(SizeUnion::new_with(55_usize));
        /// println!("{} - 55 = {}", a_sizeunion, b_sizeunion);
        /// assert_eq!(b_sizeunion.get(), 0_usize);
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
        #[inline] pub fn saturating_sub(self, rhs: Self) -> Self    { Self::new_with( self.get().saturating_sub(rhs.get()) ) }

        // fn abs_diff(self, other: Self) -> Self
        /// Computes the absolute difference between `self` and `other`.
        /// 
        /// # Arguments
        /// `other` is of type `Self` 
        /// 
        /// # Output
        /// It returns the absolute difference between `self` and `other`.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// let a_shortunion = ShortUnion::new_with(5050_u16);
        /// let b_shortunion = ShortUnion::new_with(5000_u16);
        /// let c_shortunion = a_shortunion.abs_diff(b_shortunion);
        /// let d_shortunion = b_shortunion.abs_diff(a_shortunion);
        /// println!("{} <-> {} = {}", a_shortunion, b_shortunion, c_shortunion);
        /// assert_eq!(c_shortunion.get(), 50_u16);
        /// println!("{} <-> {} = {}", b_shortunion, a_shortunion, d_shortunion);
        /// assert_eq!(d_shortunion.get(), 50_u16);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// let a_intunion = IntUnion::new_with(500500_u32);
        /// let b_intunion = IntUnion::new_with(500000_u32);
        /// let c_intunion = a_intunion.abs_diff(b_intunion);
        /// let d_intunion = b_intunion.abs_diff(a_intunion);
        /// println!("{} <-> {} = {}", a_intunion, b_intunion, c_intunion);
        /// assert_eq!(c_intunion.get(), 500_u32);
        /// println!("{} <-> {} = {}", b_intunion, a_intunion, d_intunion);
        /// assert_eq!(d_intunion.get(), 500_u32);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let a_longunion = LongUnion::new_with(5000050000_u64);
        /// let b_longunion = LongUnion::new_with(5000000000_u64);
        /// let c_longunion = a_longunion.abs_diff(b_longunion);
        /// let d_longunion = b_longunion.abs_diff(a_longunion);
        /// println!("{} <-> {} = {}", a_longunion, b_longunion, c_longunion);
        /// assert_eq!(c_longunion.get(), 50000_u64);
        /// println!("{} <-> {} = {}", b_longunion, a_longunion, d_longunion);
        /// assert_eq!(d_longunion.get(), 50000_u64);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let a_longerunion = LongerUnion::new_with(500000000500000000_u128);
        /// let b_longerunion = LongerUnion::new_with(500000000000000000_u128);
        /// let c_longerunion = a_longerunion.abs_diff(b_longerunion);
        /// let d_longerunion = b_longerunion.abs_diff(a_longerunion);
        /// println!("{} <-> {} = {}", a_longerunion, b_longerunion, c_longerunion);
        /// assert_eq!(c_longerunion.get(), 500000000_u128);
        /// println!("{} <-> {} = {}", b_longerunion, a_longerunion, d_longerunion);
        /// assert_eq!(d_longerunion.get(), 500000000_u128);
        /// ```
        /// 
        /// # Example 5 for SizeUnion
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// let a_sizeunion = SizeUnion::new_with(105_usize);
        /// let b_sizeunion = SizeUnion::new_with(100_usize);
        /// let c_sizeunion = a_sizeunion.abs_diff(b_sizeunion);
        /// let d_sizeunion = b_sizeunion.abs_diff(a_sizeunion);
        /// println!("{} <-> {} = {}", a_sizeunion, b_sizeunion, c_sizeunion);
        /// assert_eq!(c_sizeunion.get(), 5_usize);
        /// println!("{} <-> {} = {}", b_sizeunion, a_sizeunion, d_sizeunion);
        /// assert_eq!(d_sizeunion.get(), 5_usize);
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
        #[inline] pub fn abs_diff(self, other: Self) -> Self    { Self::new_with( self.get().abs_diff(other.get()) ) }

        // pub fn carrying_mul(self, rhs: Self, carry: Self) -> (Self, Self)
        /// Calculates the “full multiplication” `self` * `rhs` + `carry` without
        /// the possibility to overflow.
        /// 
        /// # Arguments
        /// - `rhs` is a multiplier of the type `Self`
        /// - `carry` is a carry overflowed from a previous multiplication operation.
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
        /// # Example 1 for ShortUnion for Little Endian
        /// ```
        /// use cryptocol::number::{ SmallUInt, ShortUnion, LongUnion };
        /// 
        /// // a_u64: u32 === (a_high_u16, a_low_u16) == (10000_u16, 10100_u16) == 257010000_u32
        /// let a_high_shortunion = ShortUnion::new_with(10000_u16);
        /// let a_low_shortunion = ShortUnion::new_with(10100_u16);
        /// // b_u32: u32 === (b_high_u16, b_low_u16) == (10000_u16, 20000_u16) == 258000000_u32
        /// let b_high_shortunion = ShortUnion::new_with(10000_u16);
        /// let b_low_shortunion = ShortUnion::new_with(20000_u16);
        /// 
        /// // (10000_u16, 10100_u16) X (10000_u16, 20000_u16) == 257010000_u32 X 258000000_u32 == 66308580000000000_u32
        /// //
        /// //                        (10000_u16, 10100_u16) == 655370100_u32
        /// // X                      (10000_u16, 20000_u16) == 655380000_u32
        /// // ---------------------------------------------
        /// //                       (  3082_u16, 18048_u16)
        /// //            (  3051_u16, 49664_u16)
        /// //            (  1541_u16,  9024_u16)
        /// // + (1525_u16, 57600_u16)
        /// // ---------------------------------
        /// //   (1525_u16, 62192_u16, 61770_u16, 18048_u16) == 429516456138000000_u64
        /// let zero_shortunion = ShortUnion::zero();
        /// let one_shortunion = ShortUnion::one();
        /// let (c_lower_shortunion, c_tmp_shortunion) = b_low_shortunion.carrying_mul(a_low_shortunion, zero_shortunion);
        /// let (d_low_shortunion, d_high_shortunion) = b_low_shortunion.carrying_mul(a_high_shortunion, c_tmp_shortunion);
        /// let (mut c_low_shortunion, e_high_shortunion) = b_high_shortunion.carrying_mul(a_low_shortunion, zero_shortunion);
        /// let (mut c_high_shortunion, mut c_higher_shortunion) = b_high_shortunion.carrying_mul(a_high_shortunion, e_high_shortunion);
        /// 
        /// let mut overflow: bool;
        /// (c_low_shortunion, overflow) = c_low_shortunion.overflowing_add(d_low_shortunion);
        /// if overflow
        ///     { (c_high_shortunion, overflow) = c_high_shortunion.overflowing_add(one_shortunion); }
        /// if overflow
        ///     { c_higher_shortunion = c_higher_shortunion.wrapping_add(one_shortunion); }
        /// 
        /// (c_high_shortunion, overflow) = c_high_shortunion.overflowing_add(d_high_shortunion);
        /// if overflow
        ///     { c_higher_shortunion = c_higher_shortunion.wrapping_add(one_shortunion); }
        /// println!("{}-{}-{}-{}", c_higher_shortunion, c_high_shortunion, c_low_shortunion, c_lower_shortunion);
        /// assert_eq!(c_higher_shortunion.get(), 1525_u16);
        /// assert_eq!(c_high_shortunion.get(), 62192_u16);
        /// assert_eq!(c_low_shortunion.get(), 61770_u16);
        /// assert_eq!(c_lower_shortunion.get(), 18048_u16);
        /// 
        /// let a_longunion = LongUnion::new_with_ushorts([a_low_shortunion.get(), a_high_shortunion.get(), 0, 0]);
        /// let b_longunion = LongUnion::new_with_ushorts([b_low_shortunion.get(), b_high_shortunion.get(), 0, 0]);
        /// let c_longunion = a_longunion * b_longunion;
        /// println!("{} * {} = {}", a_longunion.get(), b_longunion.get(), c_longunion.get());
        /// assert_eq!(c_higher_shortunion.get(), c_longunion.get_ushort_(3));
        /// assert_eq!(c_high_shortunion.get(), c_longunion.get_ushort_(2));
        /// assert_eq!(c_low_shortunion.get(), c_longunion.get_ushort_(1));
        /// assert_eq!(c_lower_shortunion.get(), c_longunion.get_ushort_(0));
        /// ```
        /// 
        /// # Example 2 for IntUnion for Little Endian
        /// ```
        /// use cryptocol::number::{ SmallUInt, IntUnion, LongerUnion };
        /// 
        /// // a_u64: u64 === (a_high_u32, a_low_u32) == (2299561912_u32, 2956226837_u32) == 9876543210123456789_u64
        /// let a_high_intunion = IntUnion::new_with(2299561912_u32);
        /// let a_low_intunion = IntUnion::new_with(2956226837_u32);
        /// // b_u64: u64 === (b_high_u32, b_low_u32) == (1782160508_u32, 682685733_u32) == 7654321098765432101_u64
        /// let b_high_intunion = IntUnion::new_with(1782160508_u32);
        /// let b_low_intunion = IntUnion::new_with(682685733_u32);
        /// 
        /// // (2299561912_u32, 2956226837_u32) X (1782160508_u32, 682685733_u32) == 9876543210123456789_u64 X 7654321098765432101_u64 == (4098188426859548455_u64, 17997868695111430409_u64) == 75598233076116445704676116321386983689_u128
        /// //
        /// //                                  (2299561912_u32, 2956226837_u32) == 9876543210123456789_u64
        /// // X                                (1782160508_u32,  682685733_u32) == 7654321098765432101_u64
        /// // -----------------------------------------------------------------
        /// //                                  ( 469892724_u32, 2923262217_u32)
        /// //                  ( 365515730_u32, 2949035416_u32)
        /// //                  (1226661429_u32,  771527212_u32)
        /// // + (954183848_u32, 3735936288_u32)
        /// // -----------------------------------------------------------------
        /// //   (954183849_u32, 1033146151_u32, 4190455352_u32, 2923262217_u32) == 429516456138000000_u64
        /// let zero_intunion = IntUnion::zero();
        /// let one_intunion = IntUnion::one();
        /// let (c_lower_intunion, c_tmp_intunion) = b_low_intunion.carrying_mul(a_low_intunion, zero_intunion);
        /// let (d_low_intunion, d_high_intunion) = b_low_intunion.carrying_mul(a_high_intunion, c_tmp_intunion);
        /// let (mut c_low_intunion, e_high_intunion) = b_high_intunion.carrying_mul(a_low_intunion, zero_intunion);
        /// let (mut c_high_intunion, mut c_higher_intunion) = b_high_intunion.carrying_mul(a_high_intunion, e_high_intunion);
        /// 
        /// let mut overflow: bool;
        /// (c_low_intunion, overflow) = c_low_intunion.overflowing_add(d_low_intunion);
        /// if overflow
        ///     { (c_high_intunion, overflow) = c_high_intunion.overflowing_add(one_intunion); }
        /// if overflow
        ///     { c_higher_intunion = c_higher_intunion.wrapping_add(one_intunion); }
        /// 
        /// (c_high_intunion, overflow) = c_high_intunion.overflowing_add(d_high_intunion);
        /// if overflow
        ///     { c_higher_intunion = c_higher_intunion.wrapping_add(one_intunion); }
        /// println!("{}-{}-{}-{}", c_higher_intunion, c_high_intunion, c_low_intunion, c_lower_intunion);
        /// assert_eq!(c_higher_intunion.get(), 954183849_u32);
        /// assert_eq!(c_high_intunion.get(), 1033146151_u32);
        /// assert_eq!(c_low_intunion.get(), 4190455352_u32);
        /// assert_eq!(c_lower_intunion.get(), 2923262217_u32);
        /// 
        /// let a_longerunion = LongerUnion::new_with_uints([a_low_intunion.get(), a_high_intunion.get(), 0, 0]);
        /// let b_longerunion = LongerUnion::new_with_uints([b_low_intunion.get(), b_high_intunion.get(), 0, 0]);
        /// let c_longerunion = a_longerunion * b_longerunion;
        /// println!("{} * {} = {}", a_longerunion.get(), b_longerunion.get(), c_longerunion.get());
        /// assert_eq!(c_higher_intunion.get(), c_longerunion.get_uint_(3));
        /// assert_eq!(c_high_intunion.get(), c_longerunion.get_uint_(2));
        /// assert_eq!(c_low_intunion.get(), c_longerunion.get_uint_(1));
        /// assert_eq!(c_lower_intunion.get(), c_longerunion.get_uint_(0));
        /// ```
        /// 
        /// # Example 3 for LongUnion for Little Endian
        /// ```
        /// use cryptocol::number::{ SmallUInt, LongUnion };
        /// 
        /// // a_u128: u128 === (a_high_u64, a_low_u64) == (10775095670246085798_u64, 7681743649119882630_u64) == 198765432198765432198765432198765432198_u128
        /// let a_high_longunion = LongUnion::new_with(10775095670246085798_u64);
        /// let a_low_longunion = LongUnion::new_with(7681743649119882630_u64);
        /// // b_u64: u64 === (b_high_u64, b_low_u64) == (6692605942763486917_u64, 12312739301371248917_u64) == 123456789012345678901234567890123456789_u128
        /// let b_high_longunion = LongUnion::new_with(6692605942763486917_u64);
        /// let b_low_longunion = LongUnion::new_with(12312739301371248917_u64);
        /// 
        /// // (10775095670246085798_u64, 7681743649119882630_u64) X (6692605942763486917_u64, 12312739301371248917_u64) == 198765432198765432198765432198765432198_u128 X 123456789012345678901234567890123456789_u128 == (72113469316534070997571940237811086202_u128, 284839445932509422190795104397182362110_u128) == 24538942025910684226047858446061575867965995914594253912457079712243362292222_u256
        /// //
        /// //                                                      (10775095670246085798_u64,  7681743649119882630_u64) == 198765432198765432198765432198765432198_u128
        /// // X                                                    ( 6692605942763486917_u64, 12312739301371248917_u64) == 123456789012345678901234567890123456789_u128
        /// // ---------------------------------------------------------------------------------------------------------
        /// //                                                      ( 5127371342803972846_u64,  9393535397455192574_u64)
        /// //                             (7192106282005498115_u64,  3473120370613376926_u64)
        /// //                             (2786989562573083321_u64,  6840685591062354974_u64)
        /// // + (3909279004922650219_u64,  1464703988338300862_u64)
        /// // ---------------------------------------------------------------------------------------------------------
        /// //   (3909279004922650219_u64, 11443799832916882298_u64, 15441177304479704746_u64,  9393535397455192574_u64) == 24538942025910684226047858446061575867965995914594253912457079712243362292222_u256
        /// let zero_longunion = LongUnion::zero();
        /// let one_longunion = LongUnion::one();
        /// let (c_lower_longunion, c_tmp_longunion) = b_low_longunion.carrying_mul(a_low_longunion, zero_longunion);
        /// let (d_low_longunion, d_high_longunion) = b_low_longunion.carrying_mul(a_high_longunion, c_tmp_longunion);
        /// let (mut c_low_longunion, e_high_longunion) = b_high_longunion.carrying_mul(a_low_longunion, zero_longunion);
        /// let (mut c_high_longunion, mut c_higher_longunion) = b_high_longunion.carrying_mul(a_high_longunion, e_high_longunion);
        /// 
        /// let mut overflow: bool;
        /// (c_low_longunion, overflow) = c_low_longunion.overflowing_add(d_low_longunion);
        /// if overflow
        ///     { (c_high_longunion, overflow) = c_high_longunion.overflowing_add(one_longunion); }
        /// if overflow
        ///     { c_higher_longunion = c_higher_longunion.wrapping_add(one_longunion); }
        /// 
        /// (c_high_longunion, overflow) = c_high_longunion.overflowing_add(d_high_longunion);
        /// if overflow
        ///     { c_higher_longunion = c_higher_longunion.wrapping_add(one_longunion); }
        /// println!("{}-{}-{}-{}", c_higher_longunion, c_high_longunion, c_low_longunion, c_lower_longunion);
        /// assert_eq!(c_higher_longunion.get(), 3909279004922650219_u64);
        /// assert_eq!(c_high_longunion.get(), 11443799832916882298_u64);
        /// assert_eq!(c_low_longunion.get(), 15441177304479704746_u64);
        /// assert_eq!(c_lower_longunion.get(), 9393535397455192574_u64);
        /// ```
        /// 
        /// # Example 4 for LongerUnion for Little Endian
        /// ```
        /// use cryptocol::number::{ SmallUInt, LongerUnion };
        /// 
        /// // a_u256: u256 === (a_high_u128, a_low_u128) == (123456789012345678901234567890123456789_u128, 198765432198765432198765432198765432198_u128) == 42010168377579896403540037778015643756626903575004241358522734820017396206982_u256
        /// let a_high_longerunion = LongerUnion::new_with(123456789012345678901234567890123456789_u128);
        /// let a_low_longerunion = LongerUnion::new_with(198765432198765432198765432198765432198_u128);
        /// // b_u256: u256 === (b_high_u128, b_low_u128) == (75318642097531864209753186420975318642_u128, 135792468013579246801357924680135792468_u128) == 25629605806219180037134371884461041203042609997744073457419340831856170555220_u256
        /// let b_high_longerunion = LongerUnion::new_with(75318642097531864209753186420975318642_u128);
        /// let b_low_longerunion = LongerUnion::new_with(135792468013579246801357924680135792468_u128);
        /// 
        /// // (123456789012345678901234567890123456789_u128, 198765432198765432198765432198765432198_u128) X (75318642097531864209753186420975318642_u128 - 135792468013579246801357924680135792468_u128) == 42010168377579896403540037778015643756626903575004241358522734820017396206982_u256 X 25629605806219180037134371884461041203042609997744073457419340831856170555220_u256 = 1076704055370267103358067448344494207403929951418850598311166733254725709101675518708273284527051744761749874770306207984521811586513200762632500980546040_u512
        /// //
        /// //                                                                                              (123456789012345678901234567890123456789_u128, 198765432198765432198765432198765432198_u128) == 42010168377579896403540037778015643756626903575004241358522734820017396206982_u256
        /// // X                                                                                            ( 75318642097531864209753186420975318642_u128, 135792468013579246801357924680135792468_u128) == 25629605806219180037134371884461041203042609997744073457419340831856170555220_u256
        /// // -----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
        /// //                                                                                              ( 79318975115531594676802389315672824709_u128, 305933135181961371815664194362919418360_u128)
        /// //                                                ( 49266443702953415606417933871327680361_u128, 301235724958848324675382352967843249636_u128)
        /// //                                                ( 43995057941448862830514490586650222101_u128,  35386202970580104685103432753963846060_u128)
        /// // + (27326122685316262062508597076325453266_u128, 184240100967607654057575481238459345242_u128)
        /// // -----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
        /// //   (27326122685316262062508597076325453266_u128, 277501602612009932494507905696437247705_u128,  75658536124021560573913567605711708949_u128, 305933135181961371815664194362919418360_u128) == 1076704055370267103358067448344494207403929951418850598311166733254725709101675518708273284527051744761749874770306207984521811586513200762632500980546040_u512
        /// let zero_longerunion = LongerUnion::zero();
        /// let one_longerunion = LongerUnion::one();
        /// let (c_lower_longerunion, c_tmp_longerunion) = b_low_longerunion.carrying_mul(a_low_longerunion, zero_longerunion);
        /// let (d_low_longerunion, d_high_longerunion) = b_low_longerunion.carrying_mul(a_high_longerunion, c_tmp_longerunion);
        /// let (mut c_low_longerunion, e_high_longerunion) = b_high_longerunion.carrying_mul(a_low_longerunion, zero_longerunion);
        /// let (mut c_high_longerunion, mut c_higher_longerunion) = b_high_longerunion.carrying_mul(a_high_longerunion, e_high_longerunion);
        /// 
        /// let mut overflow: bool;
        /// (c_low_longerunion, overflow) = c_low_longerunion.overflowing_add(d_low_longerunion);
        /// if overflow
        ///     { (c_high_longerunion, overflow) = c_high_longerunion.overflowing_add(one_longerunion); }
        /// if overflow
        ///     { c_higher_longerunion = c_higher_longerunion.wrapping_add(one_longerunion); }
        /// 
        /// (c_high_longerunion, overflow) = c_high_longerunion.overflowing_add(d_high_longerunion);
        /// if overflow
        ///     { c_higher_longerunion = c_higher_longerunion.wrapping_add(one_longerunion); }
        /// println!("{}-{}-{}-{}", c_higher_longerunion, c_high_longerunion, c_low_longerunion, c_lower_longerunion);
        /// assert_eq!(c_higher_longerunion.get(), 27326122685316262062508597076325453266_u128);
        /// assert_eq!(c_high_longerunion.get(), 277501602612009932494507905696437247705_u128);
        /// assert_eq!(c_low_longerunion.get(), 75658536124021560573913567605711708949_u128);
        /// assert_eq!(c_lower_longerunion.get(), 305933135181961371815664194362919418360_u128);
        /// ```
        /// 
        /// # Example 5 for SizeUnion for Little Endian
        /// ```
        /// use cryptocol::number::{ SmallUInt, SizeUnion };
        /// 
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     // a_u128: u128 === (a_high_usize, a_low_usize) == (10775095670246085798_usize, 7681743649119882630_usize) == 198765432198765432198765432198765432198_u128
        ///     let a_high_sizeunion = SizeUnion::new_with(10775095670246085798_usize);
        ///     let a_low_sizeunion = SizeUnion::new_with(7681743649119882630_usize);
        ///     // b_usize: usize === (b_high_usize, b_low_usize) == (6692605942763486917_usize, 12312739301371248917_usize) == 123456789012345678901234567890123456789_u128
        ///     let b_high_sizeunion = SizeUnion::new_with(6692605942763486917_usize);
        ///     let b_low_sizeunion = SizeUnion::new_with(12312739301371248917_usize);
        /// 
        ///     // (10775095670246085798_usize, 7681743649119882630_usize) X (6692605942763486917_usize, 12312739301371248917_usize) == 198765432198765432198765432198765432198_u128 X 123456789012345678901234567890123456789_u128 == (72113469316534070997571940237811086202_u128, 284839445932509422190795104397182362110_u128) == 24538942025910684226047858446061575867965995914594253912457079712243362292222_u256
        ///     //
        ///     //                                                          (10775095670246085798_usize,  7681743649119882630_usize) == 198765432198765432198765432198765432198_u128
        ///     // X                                                        ( 6692605942763486917_usize, 12312739301371248917_usize) == 123456789012345678901234567890123456789_u128
        ///     // -----------------------------------------------------------------------------------------------------------------
        ///     //                                                          ( 5127371342803972846_usize,  9393535397455192574_usize)
        ///     //                               (7192106282005498115_usize,  3473120370613376926_usize)
        ///     //                               (2786989562573083321_usize,  6840685591062354974_usize)
        ///     // + (3909279004922650219_usize,  1464703988338300862_usize)
        ///     // -----------------------------------------------------------------------------------------------------------------
        ///     //   (3909279004922650219_usize, 11443799832916882298_usize, 15441177304479704746_usize,  9393535397455192574_usize) == 24538942025910684226047858446061575867965995914594253912457079712243362292222_u256
        ///     let zero_sizeunion = SizeUnion::zero();
        ///     let one_sizeunion = SizeUnion::one();
        ///     let (c_lower_sizeunion, c_tmp_sizeunion) = b_low_sizeunion.carrying_mul(a_low_sizeunion, zero_sizeunion);
        ///     let (d_low_sizeunion, d_high_sizeunion) = b_low_sizeunion.carrying_mul(a_high_sizeunion, c_tmp_sizeunion);
        ///     let (mut c_low_sizeunion, e_high_sizeunion) = b_high_sizeunion.carrying_mul(a_low_sizeunion, zero_sizeunion);
        ///     let (mut c_high_sizeunion, mut c_higher_sizeunion) = b_high_sizeunion.carrying_mul(a_high_sizeunion, e_high_sizeunion);
        /// 
        ///     let mut overflow: bool;
        ///     (c_low_sizeunion, overflow) = c_low_sizeunion.overflowing_add(d_low_sizeunion);
        ///     if overflow
        ///         { (c_high_sizeunion, overflow) = c_high_sizeunion.overflowing_add(one_sizeunion); }
        ///     if overflow
        ///         { c_higher_sizeunion = c_higher_sizeunion.wrapping_add(one_sizeunion); }
        /// 
        ///     (c_high_sizeunion, overflow) = c_high_sizeunion.overflowing_add(d_high_sizeunion);
        ///     if overflow
        ///         { c_higher_sizeunion = c_higher_sizeunion.wrapping_add(one_sizeunion); }
        ///     println!("{}-{}-{}-{}", c_higher_sizeunion, c_high_sizeunion, c_low_sizeunion, c_lower_sizeunion);
        ///     assert_eq!(c_higher_sizeunion.get(), 3909279004922650219_usize);
        ///     assert_eq!(c_high_sizeunion.get(), 11443799832916882298_usize);
        ///     assert_eq!(c_low_sizeunion.get(), 15441177304479704746_usize);
        ///     assert_eq!(c_lower_sizeunion.get(), 9393535397455192574_usize);
        /// }
        /// ```
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
        #[inline] pub fn carrying_mul(self, rhs: Self, carry: Self) -> (Self, Self)
        {
            let (low, high) = SmallUInt::carrying_mul(self.get(), rhs.get(), carry.get());
            (Self::new_with(low), Self::new_with(high))
        }

        // pub fn widening_mul(self, rhs: Self) -> (Self, Self)
        /// Calculates the complete product `self` * `rhs` without the possibility
        /// to overflow.
        /// 
        /// # Arguments
        /// - `rhs` is a multiplier of the type `Self`
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
        /// # Example 1 for ShortUnion for Little Endian
        /// ```
        /// use cryptocol::number::{ SmallUInt, ShortUnion, LongUnion };
        /// 
        /// // a_u64: u32 === (a_high_u16, a_low_u16) == (10000_u16, 10100_u16) == 257010000_u32
        /// let a_high_shortunion = ShortUnion::new_with(10000_u16);
        /// let a_low_shortunion = ShortUnion::new_with(10100_u16);
        /// // b_u32: u32 === (b_high_u16, b_low_u16) == (10000_u16, 20000_u16) == 258000000_u32
        /// let b_high_shortunion = ShortUnion::new_with(10000_u16);
        /// let b_low_shortunion = ShortUnion::new_with(20000_u16);
        /// 
        /// // (10000_u16, 10100_u16) X (10000_u16, 20000_u16) == 257010000_u32 X 258000000_u32 == 66308580000000000_u32
        /// //
        /// //                        (10000_u16, 10100_u16) == 655370100_u32
        /// // X                      (10000_u16, 20000_u16) == 655380000_u32
        /// // ---------------------------------------------
        /// //                       (  3082_u16, 18048_u16)
        /// //            (  3051_u16, 49664_u16)
        /// //            (  1541_u16,  9024_u16)
        /// // + (1525_u16, 57600_u16)
        /// // ---------------------------------
        /// //   (1525_u16, 62192_u16, 61770_u16, 18048_u16) == 429516456138000000_u64
        /// let one_shortunion = ShortUnion::one();
        /// let (c_lower_shortunion, c_temp_shortunion) = b_low_shortunion.widening_mul(a_low_shortunion);
        /// let (d_low_shortunion, d_high_shortunion) = b_low_shortunion.widening_mul(a_high_shortunion);
        /// let (mut c_low_shortunion, e_high_shortunion) = b_high_shortunion.widening_mul(a_low_shortunion);
        /// let (mut c_high_shortunion, mut c_higher_shortunion) = b_high_shortunion.widening_mul(a_high_shortunion);
        /// 
        /// let mut overflow: bool;
        /// (c_low_shortunion, overflow) = c_low_shortunion.overflowing_add(d_low_shortunion);
        /// if overflow
        ///     { (c_high_shortunion, overflow) = c_high_shortunion.overflowing_add(one_shortunion); }
        /// if overflow
        ///     { c_higher_shortunion = c_higher_shortunion.wrapping_add(one_shortunion); }
        /// (c_low_shortunion, overflow) = c_low_shortunion.overflowing_add(c_temp_shortunion);
        /// if overflow
        ///     { (c_high_shortunion, overflow) = c_high_shortunion.overflowing_add(one_shortunion); }
        /// if overflow
        ///     { c_higher_shortunion = c_higher_shortunion.wrapping_add(one_shortunion); }
        /// 
        /// (c_high_shortunion, overflow) = c_high_shortunion.overflowing_add(d_high_shortunion);
        /// if overflow
        ///     { c_higher_shortunion = c_higher_shortunion.wrapping_add(one_shortunion); }
        /// (c_high_shortunion, overflow) = c_high_shortunion.overflowing_add(e_high_shortunion);
        /// if overflow
        ///     { c_higher_shortunion = c_higher_shortunion.wrapping_add(one_shortunion); }
        /// println!("{}-{}-{}-{}", c_higher_shortunion, c_high_shortunion, c_low_shortunion, c_lower_shortunion);
        /// assert_eq!(c_higher_shortunion.get(), 1525_u16);
        /// assert_eq!(c_high_shortunion.get(), 62192_u16);
        /// assert_eq!(c_low_shortunion.get(), 61770_u16);
        /// assert_eq!(c_lower_shortunion.get(), 18048_u16);
        /// 
        /// let a_longunion = LongUnion::new_with_ushorts([a_low_shortunion.get(), a_high_shortunion.get(), 0, 0]);
        /// let b_longunion = LongUnion::new_with_ushorts([b_low_shortunion.get(), b_high_shortunion.get(), 0, 0]);
        /// let c_longunion = a_longunion * b_longunion;
        /// println!("{} * {} = {}", a_longunion.get(), b_longunion.get(), c_longunion.get());
        /// assert_eq!(c_higher_shortunion.get(), c_longunion.get_ushort_(3));
        /// assert_eq!(c_high_shortunion.get(), c_longunion.get_ushort_(2));
        /// assert_eq!(c_low_shortunion.get(), c_longunion.get_ushort_(1));
        /// assert_eq!(c_lower_shortunion.get(), c_longunion.get_ushort_(0));
        /// ```
        /// 
        /// # Example 2 for IntUnion for Little Endian
        /// ```
        /// use cryptocol::number::{ SmallUInt, IntUnion, LongerUnion };
        /// 
        /// // a_u64: u64 === (a_high_u32, a_low_u32) == (2299561912_u32, 2956226837_u32) == 9876543210123456789_u64
        /// let a_high_intunion = IntUnion::new_with(2299561912_u32);
        /// let a_low_intunion = IntUnion::new_with(2956226837_u32);
        /// // b_u64: u64 === (b_high_u32, b_low_u32) == (1782160508_u32, 682685733_u32) == 7654321098765432101_u64
        /// let b_high_intunion = IntUnion::new_with(1782160508_u32);
        /// let b_low_intunion = IntUnion::new_with(682685733_u32);
        /// 
        /// // (2299561912_u32, 2956226837_u32) X (1782160508_u32, 682685733_u32) == 9876543210123456789_u64 X 7654321098765432101_u64 == (4098188426859548455_u64, 17997868695111430409_u64) == 75598233076116445704676116321386983689_u128
        /// //
        /// //                                  (2299561912_u32, 2956226837_u32) == 9876543210123456789_u64
        /// // X                                (1782160508_u32,  682685733_u32) == 7654321098765432101_u64
        /// // -----------------------------------------------------------------
        /// //                                  ( 469892724_u32, 2923262217_u32)
        /// //                  ( 365515730_u32, 2949035416_u32)
        /// //                  (1226661429_u32,  771527212_u32)
        /// // + (954183848_u32, 3735936288_u32)
        /// // -----------------------------------------------------------------
        /// //   (954183849_u32, 1033146151_u32, 4190455352_u32, 2923262217_u32) == 429516456138000000_u64
        /// let one_intunion = IntUnion::one();
        /// let (c_lower_intunion, c_temp_intunion) = b_low_intunion.widening_mul(a_low_intunion);
        /// let (d_low_intunion, d_high_intunion) = b_low_intunion.widening_mul(a_high_intunion);
        /// let (mut c_low_intunion, e_high_intunion) = b_high_intunion.widening_mul(a_low_intunion);
        /// let (mut c_high_intunion, mut c_higher_intunion) = b_high_intunion.widening_mul(a_high_intunion);
        /// 
        /// let mut overflow: bool;
        /// (c_low_intunion, overflow) = c_low_intunion.overflowing_add(d_low_intunion);
        /// if overflow
        ///     { (c_high_intunion, overflow) = c_high_intunion.overflowing_add(one_intunion); }
        /// if overflow
        ///     { c_higher_intunion = c_higher_intunion.wrapping_add(one_intunion); }
        /// (c_low_intunion, overflow) = c_low_intunion.overflowing_add(c_temp_intunion);
        /// if overflow
        ///     { (c_high_intunion, overflow) = c_high_intunion.overflowing_add(one_intunion); }
        /// if overflow
        ///     { c_higher_intunion = c_higher_intunion.wrapping_add(one_intunion); }
        /// 
        /// (c_high_intunion, overflow) = c_high_intunion.overflowing_add(d_high_intunion);
        /// if overflow
        ///     { c_higher_intunion = c_higher_intunion.wrapping_add(one_intunion); }
        /// (c_high_intunion, overflow) = c_high_intunion.overflowing_add(e_high_intunion);
        /// if overflow
        ///     { c_higher_intunion = c_higher_intunion.wrapping_add(one_intunion); }
        /// println!("{}-{}-{}-{}", c_higher_intunion, c_high_intunion, c_low_intunion, c_lower_intunion);
        /// assert_eq!(c_higher_intunion.get(), 954183849_u32);
        /// assert_eq!(c_high_intunion.get(), 1033146151_u32);
        /// assert_eq!(c_low_intunion.get(), 4190455352_u32);
        /// assert_eq!(c_lower_intunion.get(), 2923262217_u32);
        /// 
        /// let a_longerunion = LongerUnion::new_with_uints([a_low_intunion.get(), a_high_intunion.get(), 0, 0]);
        /// let b_longerunion = LongerUnion::new_with_uints([b_low_intunion.get(), b_high_intunion.get(), 0, 0]);
        /// let c_longerunion = a_longerunion * b_longerunion;
        /// println!("{} * {} = {}", a_longerunion.get(), b_longerunion.get(), c_longerunion.get());
        /// assert_eq!(c_higher_intunion.get(), c_longerunion.get_uint_(3));
        /// assert_eq!(c_high_intunion.get(), c_longerunion.get_uint_(2));
        /// assert_eq!(c_low_intunion.get(), c_longerunion.get_uint_(1));
        /// assert_eq!(c_lower_intunion.get(), c_longerunion.get_uint_(0));
        /// ```
        /// 
        /// # Example 3 for LongUnion for Little Endian
        /// ```
        /// use cryptocol::number::{ SmallUInt, LongUnion };
        /// 
        /// // a_u128: u128 === (a_high_u64, a_low_u64) == (10775095670246085798_u64, 7681743649119882630_u64) == 198765432198765432198765432198765432198_u128
        /// let a_high_longunion = LongUnion::new_with(10775095670246085798_u64);
        /// let a_low_longunion = LongUnion::new_with(7681743649119882630_u64);
        /// // b_u64: u64 === (b_high_u64, b_low_u64) == (6692605942763486917_u64, 12312739301371248917_u64) == 123456789012345678901234567890123456789_u128
        /// let b_high_longunion = LongUnion::new_with(6692605942763486917_u64);
        /// let b_low_longunion = LongUnion::new_with(12312739301371248917_u64);
        /// 
        /// // (10775095670246085798_u64, 7681743649119882630_u64) X (6692605942763486917_u64, 12312739301371248917_u64) == 198765432198765432198765432198765432198_u128 X 123456789012345678901234567890123456789_u128 == (72113469316534070997571940237811086202_u128, 284839445932509422190795104397182362110_u128) == 24538942025910684226047858446061575867965995914594253912457079712243362292222_u256
        /// //
        /// //                                                      (10775095670246085798_u64,  7681743649119882630_u64) == 198765432198765432198765432198765432198_u128
        /// // X                                                    ( 6692605942763486917_u64, 12312739301371248917_u64) == 123456789012345678901234567890123456789_u128
        /// // ---------------------------------------------------------------------------------------------------------
        /// //                                                      ( 5127371342803972846_u64,  9393535397455192574_u64)
        /// //                             (7192106282005498115_u64,  3473120370613376926_u64)
        /// //                             (2786989562573083321_u64,  6840685591062354974_u64)
        /// // + (3909279004922650219_u64,  1464703988338300862_u64)
        /// // ---------------------------------------------------------------------------------------------------------
        /// //   (3909279004922650219_u64, 11443799832916882298_u64, 15441177304479704746_u64,  9393535397455192574_u64) == 24538942025910684226047858446061575867965995914594253912457079712243362292222_u256
        /// let one_longunion = LongUnion::one();
        /// let (c_lower_longunion, c_temp_longunion) = b_low_longunion.widening_mul(a_low_longunion);
        /// let (d_low_longunion, d_high_longunion) = b_low_longunion.widening_mul(a_high_longunion);
        /// let (mut c_low_longunion, e_high_longunion) = b_high_longunion.widening_mul(a_low_longunion);
        /// let (mut c_high_longunion, mut c_higher_longunion) = b_high_longunion.widening_mul(a_high_longunion);
        /// 
        /// let mut overflow: bool;
        /// (c_low_longunion, overflow) = c_low_longunion.overflowing_add(d_low_longunion);
        /// if overflow
        ///     { (c_high_longunion, overflow) = c_high_longunion.overflowing_add(one_longunion); }
        /// if overflow
        ///     { c_higher_longunion = c_higher_longunion.wrapping_add(one_longunion); }
        /// (c_low_longunion, overflow) = c_low_longunion.overflowing_add(c_temp_longunion);
        /// if overflow
        ///     { (c_high_longunion, overflow) = c_high_longunion.overflowing_add(one_longunion); }
        /// if overflow
        ///     { c_higher_longunion = c_higher_longunion.wrapping_add(one_longunion); }
        /// 
        /// (c_high_longunion, overflow) = c_high_longunion.overflowing_add(d_high_longunion);
        /// if overflow
        ///     { c_higher_longunion = c_higher_longunion.wrapping_add(one_longunion); }
        /// (c_high_longunion, overflow) = c_high_longunion.overflowing_add(e_high_longunion);
        /// if overflow
        ///     { c_higher_longunion = c_higher_longunion.wrapping_add(one_longunion); }
        /// println!("{}-{}-{}-{}", c_higher_longunion, c_high_longunion, c_low_longunion, c_lower_longunion);
        /// assert_eq!(c_higher_longunion.get(), 3909279004922650219_u64);
        /// assert_eq!(c_high_longunion.get(), 11443799832916882298_u64);
        /// assert_eq!(c_low_longunion.get(), 15441177304479704746_u64);
        /// assert_eq!(c_lower_longunion.get(), 9393535397455192574_u64);
        /// ```
        /// 
        /// # Example 4 for LongerUnion for Little Endian
        /// ```
        /// use cryptocol::number::{ SmallUInt, LongerUnion };
        /// 
        /// // a_u256: u256 === (a_high_u128, a_low_u128) == (123456789012345678901234567890123456789_u128, 198765432198765432198765432198765432198_u128) == 42010168377579896403540037778015643756626903575004241358522734820017396206982_u256
        /// let a_high_longerunion = LongerUnion::new_with(123456789012345678901234567890123456789_u128);
        /// let a_low_longerunion = LongerUnion::new_with(198765432198765432198765432198765432198_u128);
        /// // b_u256: u256 === (b_high_u128, b_low_u128) == (75318642097531864209753186420975318642_u128, 135792468013579246801357924680135792468_u128) == 25629605806219180037134371884461041203042609997744073457419340831856170555220_u256
        /// let b_high_longerunion = LongerUnion::new_with(75318642097531864209753186420975318642_u128);
        /// let b_low_longerunion = LongerUnion::new_with(135792468013579246801357924680135792468_u128);
        /// 
        /// // (123456789012345678901234567890123456789_u128, 198765432198765432198765432198765432198_u128) X (75318642097531864209753186420975318642_u128 - 135792468013579246801357924680135792468_u128) == 42010168377579896403540037778015643756626903575004241358522734820017396206982_u256 X 25629605806219180037134371884461041203042609997744073457419340831856170555220_u256 = 1076704055370267103358067448344494207403929951418850598311166733254725709101675518708273284527051744761749874770306207984521811586513200762632500980546040_u512
        /// //
        /// //                                                                                              (123456789012345678901234567890123456789_u128, 198765432198765432198765432198765432198_u128) == 42010168377579896403540037778015643756626903575004241358522734820017396206982_u256
        /// // X                                                                                            ( 75318642097531864209753186420975318642_u128, 135792468013579246801357924680135792468_u128) == 25629605806219180037134371884461041203042609997744073457419340831856170555220_u256
        /// // -----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
        /// //                                                                                              ( 79318975115531594676802389315672824709_u128, 305933135181961371815664194362919418360_u128)
        /// //                                                ( 49266443702953415606417933871327680361_u128, 301235724958848324675382352967843249636_u128)
        /// //                                                ( 43995057941448862830514490586650222101_u128,  35386202970580104685103432753963846060_u128)
        /// // + (27326122685316262062508597076325453266_u128, 184240100967607654057575481238459345242_u128)
        /// // -----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
        /// //   (27326122685316262062508597076325453266_u128, 277501602612009932494507905696437247705_u128,  75658536124021560573913567605711708949_u128, 305933135181961371815664194362919418360_u128) == 1076704055370267103358067448344494207403929951418850598311166733254725709101675518708273284527051744761749874770306207984521811586513200762632500980546040_u512
        /// let one_longerunion = LongerUnion::one();
        /// let (c_lower_longerunion, c_temp_longerunion) = b_low_longerunion.widening_mul(a_low_longerunion);
        /// let (d_low_longerunion, d_high_longerunion) = b_low_longerunion.widening_mul(a_high_longerunion);
        /// let (mut c_low_longerunion, e_high_longerunion) = b_high_longerunion.widening_mul(a_low_longerunion);
        /// let (mut c_high_longerunion, mut c_higher_longerunion) = b_high_longerunion.widening_mul(a_high_longerunion);
        /// 
        /// let mut overflow: bool;
        /// (c_low_longerunion, overflow) = c_low_longerunion.overflowing_add(d_low_longerunion);
        /// if overflow
        ///     { (c_high_longerunion, overflow) = c_high_longerunion.overflowing_add(one_longerunion); }
        /// if overflow
        ///     { c_higher_longerunion = c_higher_longerunion.wrapping_add(one_longerunion); }
        /// (c_low_longerunion, overflow) = c_low_longerunion.overflowing_add(c_temp_longerunion);
        /// if overflow
        ///     { (c_high_longerunion, overflow) = c_high_longerunion.overflowing_add(one_longerunion); }
        /// if overflow
        ///     { c_higher_longerunion = c_higher_longerunion.wrapping_add(one_longerunion); }
        /// 
        /// (c_high_longerunion, overflow) = c_high_longerunion.overflowing_add(d_high_longerunion);
        /// if overflow
        ///     { c_higher_longerunion = c_higher_longerunion.wrapping_add(one_longerunion); }
        /// (c_high_longerunion, overflow) = c_high_longerunion.overflowing_add(e_high_longerunion);
        /// if overflow
        ///     { c_higher_longerunion = c_higher_longerunion.wrapping_add(one_longerunion); }
        /// println!("{}-{}-{}-{}", c_higher_longerunion, c_high_longerunion, c_low_longerunion, c_lower_longerunion);
        /// assert_eq!(c_higher_longerunion.get(), 27326122685316262062508597076325453266_u128);
        /// assert_eq!(c_high_longerunion.get(), 277501602612009932494507905696437247705_u128);
        /// assert_eq!(c_low_longerunion.get(), 75658536124021560573913567605711708949_u128);
        /// assert_eq!(c_lower_longerunion.get(), 305933135181961371815664194362919418360_u128);
        /// ```
        /// 
        /// # Example 5 for SizeUnion for Little Endian
        /// ```
        /// use cryptocol::number::{ SmallUInt, SizeUnion };
        /// 
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     // a_u128: u128 === (a_high_usize, a_low_usize) == (10775095670246085798_usize, 7681743649119882630_usize) == 198765432198765432198765432198765432198_u128
        ///     let a_high_sizeunion = SizeUnion::new_with(10775095670246085798_usize);
        ///     let a_low_sizeunion = SizeUnion::new_with(7681743649119882630_usize);
        ///     // b_usize: usize === (b_high_usize, b_low_usize) == (6692605942763486917_usize, 12312739301371248917_usize) == 123456789012345678901234567890123456789_u128
        ///     let b_high_sizeunion = SizeUnion::new_with(6692605942763486917_usize);
        ///     let b_low_sizeunion = SizeUnion::new_with(12312739301371248917_usize);
        /// 
        ///     // (10775095670246085798_usize, 7681743649119882630_usize) X (6692605942763486917_usize, 12312739301371248917_usize) == 198765432198765432198765432198765432198_u128 X 123456789012345678901234567890123456789_u128 == (72113469316534070997571940237811086202_u128, 284839445932509422190795104397182362110_u128) == 24538942025910684226047858446061575867965995914594253912457079712243362292222_u256
        ///     //
        ///     //                                                          (10775095670246085798_usize,  7681743649119882630_usize) == 198765432198765432198765432198765432198_u128
        ///     // X                                                        ( 6692605942763486917_usize, 12312739301371248917_usize) == 123456789012345678901234567890123456789_u128
        ///     // -----------------------------------------------------------------------------------------------------------------
        ///     //                                                          ( 5127371342803972846_usize,  9393535397455192574_usize)
        ///     //                               (7192106282005498115_usize,  3473120370613376926_usize)
        ///     //                               (2786989562573083321_usize,  6840685591062354974_usize)
        ///     // + (3909279004922650219_usize,  1464703988338300862_usize)
        ///     // -----------------------------------------------------------------------------------------------------------------
        ///     //   (3909279004922650219_usize, 11443799832916882298_usize, 15441177304479704746_usize,  9393535397455192574_usize) == 24538942025910684226047858446061575867965995914594253912457079712243362292222_u256
        ///     let one_sizeunion = SizeUnion::one();
        ///     let (c_lower_sizeunion, c_temp_sizeunion) = b_low_sizeunion.widening_mul(a_low_sizeunion);
        ///     let (d_low_sizeunion, d_high_sizeunion) = b_low_sizeunion.widening_mul(a_high_sizeunion);
        ///     let (mut c_low_sizeunion, e_high_sizeunion) = b_high_sizeunion.widening_mul(a_low_sizeunion);
        ///     let (mut c_high_sizeunion, mut c_higher_sizeunion) = b_high_sizeunion.widening_mul(a_high_sizeunion);
        /// 
        ///     let mut overflow: bool;
        ///     (c_low_sizeunion, overflow) = c_low_sizeunion.overflowing_add(d_low_sizeunion);
        ///     if overflow
        ///         { (c_high_sizeunion, overflow) = c_high_sizeunion.overflowing_add(one_sizeunion); }
        ///     if overflow
        ///         { c_higher_sizeunion = c_higher_sizeunion.wrapping_add(one_sizeunion); }
        ///     (c_low_sizeunion, overflow) = c_low_sizeunion.overflowing_add(c_temp_sizeunion);
        ///     if overflow
        ///         { (c_high_sizeunion, overflow) = c_high_sizeunion.overflowing_add(one_sizeunion); }
        ///     if overflow
        ///         { c_higher_sizeunion = c_higher_sizeunion.wrapping_add(one_sizeunion); }
        /// 
        ///     (c_high_sizeunion, overflow) = c_high_sizeunion.overflowing_add(d_high_sizeunion);
        ///     if overflow
        ///         { c_higher_sizeunion = c_higher_sizeunion.wrapping_add(one_sizeunion); }
        ///     (c_high_sizeunion, overflow) = c_high_sizeunion.overflowing_add(e_high_sizeunion);
        ///     if overflow
        ///         { c_higher_sizeunion = c_higher_sizeunion.wrapping_add(one_sizeunion); }
        ///     println!("{}-{}-{}-{}", c_higher_sizeunion, c_high_sizeunion, c_low_sizeunion, c_lower_sizeunion);
        ///     assert_eq!(c_higher_sizeunion.get(), 3909279004922650219_usize);
        ///     assert_eq!(c_high_sizeunion.get(), 11443799832916882298_usize);
        ///     assert_eq!(c_low_sizeunion.get(), 15441177304479704746_usize);
        ///     assert_eq!(c_lower_sizeunion.get(), 9393535397455192574_usize);
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
        pub fn widening_mul(self, rhs: Self) -> (Self, Self)
        {
            let zero = Self::zero();
            let one = Self::one();
            if rhs.is_zero() || self.is_zero()
                { return (zero, zero); }
    
            let mut low = zero;
            let mut high = zero;
            let mut overflow: bool;
            let mut bit_check = one << (Self::size_in_bits() - 1 - rhs.leading_zeros() as usize);
            let adder = self;
            while !bit_check.is_zero()
            {
                high <<= 1;
                if low.is_msb_set()
                    { high.set_lsb(); }
                low <<= 1;
                if (bit_check.get() & rhs.get()) != 0
                {
                    (low, overflow) = low.overflowing_add(adder);
                    if overflow
                        { high = high.wrapping_add(one); }
                }
                bit_check >>= 1;
            }
            (low, high)
        }

        // fn wrapping_mul(self, rhs: Self) -> Self
        /// Computes `self` * `rhs`, wrapping around at the boundary of the type.
        /// 
        /// # Arguments
        /// - `rhs` is a multiplier of the type `Self`
        /// 
        /// # Features
        /// It multiplies two numbers with wrapping (modular) multiplication.
        /// 
        /// # Output
        /// It returns the `self` * `rhs` in the type of `Self`.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// 
        /// let a_shortunion = ShortUnion::new_with(u16::MAX / 3).wrapping_mul(ShortUnion::new_with(2_u16));
        /// println!("{} * 2 = {}", ShortUnion::new_with(u16::MAX / 3), a_shortunion);
        /// assert_eq!(a_shortunion.get(), 43690_u16);
        /// 
        /// let b_shortunion = a_shortunion.wrapping_mul(ShortUnion::new_with(2_u16));
        /// println!("{} * 2 = {}", a_shortunion, b_shortunion);
        /// assert_eq!(b_shortunion.get(), 21844_u16);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// 
        /// let a_intunion = IntUnion::new_with(u32::MAX / 3).wrapping_mul(IntUnion::new_with(2_u32));
        /// println!("{} * 2 = {}", IntUnion::new_with(u32::MAX / 3), a_intunion);
        /// assert_eq!(a_intunion.get(), 2863311530_u32);
        /// 
        /// let b_intunion = a_intunion.wrapping_mul(IntUnion::new_with(2_u32));
        /// println!("{} * 2 = {}", a_intunion, b_intunion);
        /// assert_eq!(b_intunion.get(), 1431655764_u32);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// 
        /// let a_longunion = LongUnion::new_with(u64::MAX / 3).wrapping_mul(LongUnion::new_with(2_u64));
        /// println!("{} * 2 = {}", LongUnion::new_with(u64::MAX / 3), a_longunion);
        /// assert_eq!(a_longunion.get(), 12297829382473034410_u64);
        /// 
        /// let b_longunion = a_longunion.wrapping_mul(LongUnion::new_with(2_u64));
        /// println!("{} * 2 = {}", a_longunion, b_longunion);
        /// assert_eq!(b_longunion.get(), 6148914691236517204_u64);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// 
        /// let a_longerunion = LongerUnion::new_with(u128::MAX / 3).wrapping_mul(LongerUnion::new_with(2_u128));
        /// println!("{} * 2 = {}", LongerUnion::new_with(u128::MAX / 3), a_longerunion);
        /// assert_eq!(a_longerunion.get(), 226854911280625642308916404954512140970_u128);
        /// 
        /// let b_longerunion = a_longerunion.wrapping_mul(LongerUnion::new_with(2_u128));
        /// println!("{} * 2 = {}", a_longerunion, b_longerunion);
        /// assert_eq!(b_longerunion.get(), 113427455640312821154458202477256070484_u128);
        /// ```
        /// 
        /// # Example 4 for SizeUnion for 64-bit CPUs
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// 
        /// let a_sizeunion = SizeUnion::new_with(usize::MAX / 3).wrapping_mul(SizeUnion::new_with(2_usize));
        /// println!("{} * 2 = {}", SizeUnion::new_with(usize::MAX / 3), a_sizeunion);
        /// #[cfg(target_pointer_width = "64")] assert_eq!(a_sizeunion.get(), 12297829382473034410_usize);
        /// 
        /// let b_sizeunion = a_sizeunion.wrapping_mul(SizeUnion::new_with(2_usize));
        /// println!("{} * 2 = {}", a_sizeunion, b_sizeunion);
        /// #[cfg(target_pointer_width = "64")] assert_eq!(b_sizeunion.get(), 6148914691236517204_usize);
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
        #[inline] pub fn wrapping_mul(self, rhs: Self) -> Self      { Self::new_with( self.get().wrapping_mul(rhs.get()) ) }
        // #[inline] pub fn wrapping_mul_assign(&mut self, rhs: Self)  { self.set(self.get().wrapping_mul(rhs.get())); }
    
        // fn overflowing_mul(self, rhs: Self) -> (Self, bool);
        /// Calculates `self` * `rhs`, wrapping around at the boundary of the type.
        /// 
        /// # Arguments
        /// - `rhs` is a multiplier of the type `Self`
        /// 
        /// # Features
        /// It multiplies two numbers with wrapping (modular) multiplication.
        /// 
        /// # Output
        /// It returns a tuple of the multiplication along with a boolean indicating
        /// whether an arithmetic overflow would occur. If an overflow would
        /// have occurred then the wrapped value is returned.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// 
        /// let (a_shortunion, overflow) = ShortUnion::new_with(u16::MAX / 3).overflowing_mul(ShortUnion::new_with(2_u16));
        /// println!("{} * 2 = {}\nOverflow = {}", ShortUnion::new_with(u16::MAX / 3), a_shortunion, overflow);
        /// assert_eq!(a_shortunion.get(), 43690_u16);
        /// assert_eq!(overflow, false);
        /// 
        /// let (b_shortunion, overflow) = a_shortunion.overflowing_mul(ShortUnion::new_with(2_u16));
        /// println!("{} * 2 = {}\nOverflow = {}", a_shortunion, b_shortunion, overflow);
        /// assert_eq!(b_shortunion.get(), 21844_u16);
        /// assert_eq!(overflow, true);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// 
        /// let (a_intunion, overflow) = IntUnion::new_with(u32::MAX / 3).overflowing_mul(IntUnion::new_with(2_u32));
        /// println!("{} * 2 = {}\nOverflow = {}", IntUnion::new_with(u32::MAX / 3), a_intunion, overflow);
        /// assert_eq!(a_intunion.get(), 2863311530_u32);
        /// assert_eq!(overflow, false);
        /// 
        /// let (b_intunion, overflow) = a_intunion.overflowing_mul(IntUnion::new_with(2_u32));
        /// println!("{} * 2 = {}\nOverflow = {}", a_intunion, b_intunion, overflow);
        /// assert_eq!(b_intunion.get(), 1431655764_u32);
        /// assert_eq!(overflow, true);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// 
        /// let (a_longunion, overflow) = LongUnion::new_with(u64::MAX / 3).overflowing_mul(LongUnion::new_with(2_u64));
        /// println!("{} * 2 = {}\nOverflow = {}", LongUnion::new_with(u64::MAX / 3), a_longunion, overflow);
        /// assert_eq!(a_longunion.get(), 12297829382473034410_u64);
        /// assert_eq!(overflow, false);
        /// 
        /// let (b_longunion, overflow) = a_longunion.overflowing_mul(LongUnion::new_with(2_u64));
        /// println!("{} * 2 = {}\nOverflow = {}", a_longunion, b_longunion, overflow);
        /// assert_eq!(b_longunion.get(), 6148914691236517204_u64);
        /// assert_eq!(overflow, true);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// 
        /// let (a_longerunion, overflow) = LongerUnion::new_with(u128::MAX / 3).overflowing_mul(LongerUnion::new_with(2_u128));
        /// println!("{} * 2 = {}\nOverflow = {}", LongerUnion::new_with(u128::MAX / 3), a_longerunion, overflow);
        /// assert_eq!(a_longerunion.get(), 226854911280625642308916404954512140970_u128);
        /// assert_eq!(overflow, false);
        /// 
        /// let (b_longerunion, overflow)= a_longerunion.overflowing_mul(LongerUnion::new_with(2_u128));
        /// println!("{} * 2 = {}\nOverflow = {}", a_longerunion, b_longerunion, overflow);
        /// assert_eq!(b_longerunion.get(), 113427455640312821154458202477256070484_u128);
        /// assert_eq!(overflow, true);
        /// ```
        /// 
        /// # Example 5 for SizeUnion
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// 
        /// let (a_sizeunion, overflow) = SizeUnion::new_with(usize::MAX / 3).overflowing_mul(SizeUnion::new_with(2_usize));
        /// println!("{} * 2 = {}\nOverflow = {}", SizeUnion::new_with(usize::MAX / 3), a_sizeunion, overflow);
        /// #[cfg(target_pointer_width = "64")] assert_eq!(a_sizeunion.get(), 12297829382473034410_usize);
        /// assert_eq!(overflow, false);
        /// 
        /// let (b_sizeunion, overflow) = a_sizeunion.overflowing_mul(SizeUnion::new_with(2_usize));
        /// println!("{} * 2 = {}\nOverflow = {}", a_sizeunion, b_sizeunion, overflow);
        /// #[cfg(target_pointer_width = "64")] assert_eq!(b_sizeunion.get(), 6148914691236517204_usize);
        /// assert_eq!(overflow, true);
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
        pub fn overflowing_mul(self, rhs: Self) -> (Self, bool)
        {
            let (res_this, carry) = self.get().overflowing_mul(rhs.get());
            (Self::new_with(res_this), carry)
        }

        // fn checked_mul(self, rhs: Self) -> Option<Self>;
        /// Computes `self` * `rhs`.
        /// 
        /// # Arguments
        /// - `rhs` is a multiplier of the type `Self`
        /// 
        /// # Output
        /// It returns `self` * `rhs` in the type `Self` wrapped by `Some`
        /// of enum `Option` if overflow did not occur.
        /// And, it returns `None` if overflow occurred.
        /// 
        /// # Example 1 for u8
        /// ```
        /// use cryptocol::number::{ ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };
        /// 
        /// // Example for ShortUnion
        /// let a_shortunion = ShortUnion::new_with(u16::MAX / 3).checked_mul(ShortUnion::new_with(2_u16));
        /// match a_shortunion
        /// {
        ///     Some(a) => {
        ///             println!("{} * 2 = {}", ShortUnion::new_with(u16::MAX / 3), a);
        ///             assert_eq!(a.get(), 43690_u16);
        ///         },
        ///     None => { println!("Overflow happened."); },
        /// }
        /// 
        /// let b_shortunion = a_shortunion.unwrap().checked_mul(ShortUnion::new_with(2_u16));
        /// match b_shortunion
        /// {
        ///     Some(b) => { println!("{} * 2 = {}", a_shortunion.unwrap(), b); },
        ///     None => {
        ///             println!("Overflow happened.");
        ///             assert_eq!(b_shortunion, None);
        ///         },
        /// }
        /// 
        /// // Example for IntUnion
        /// let a_intunion = IntUnion::new_with(u32::MAX / 3).checked_mul(IntUnion::new_with(2_u32));
        /// match a_intunion
        /// {
        ///     Some(a) => {
        ///             println!("{} * 2 = {}", IntUnion::new_with(u32::MAX / 3), a);
        ///             assert_eq!(a.get(), 2863311530_u32);
        ///         },
        ///     None => { println!("Overflow happened."); },
        /// }
        /// 
        /// let b_intunion = a_intunion.unwrap().checked_mul(IntUnion::new_with(2_u32));
        /// match b_intunion
        /// {
        ///     Some(b) => { println!("{} * 2 = {}", a_intunion.unwrap(), b); },
        ///     None => {
        ///             println!("Overflow happened.");
        ///             assert_eq!(b_intunion, None);
        ///         },
        /// }
        /// 
        /// // Example for LongUnion
        /// let a_longunion = LongUnion::new_with(u64::MAX / 3).checked_mul(LongUnion::new_with(2_u64));
        /// match a_longunion
        /// {
        ///     Some(a) => {
        ///             println!("{} * 2 = {}", LongUnion::new_with(u64::MAX / 3), a);
        ///             assert_eq!(a.get(), 12297829382473034410_u64);
        ///         },
        ///     None => { println!("Overflow happened."); },
        /// }
        /// 
        /// let b_longunion = a_longunion.unwrap().checked_mul(LongUnion::new_with(2_u64));
        /// match b_longunion
        /// {
        ///     Some(b) => { println!("{} * 2 = {}", a_longunion.unwrap(), b); },
        ///     None => {
        ///             println!("Overflow happened.");
        ///             assert_eq!(b_longunion, None);
        ///         },
        /// }
        /// 
        /// // Example for LongerUnion
        /// let a_longerunion = LongerUnion::new_with(u128::MAX / 3).checked_mul(LongerUnion::new_with(2_u128));
        /// match a_longerunion
        /// {
        ///     Some(a) => {
        ///             println!("{} * 2 = {}", LongerUnion::new_with(u128::MAX / 3), a);
        ///             assert_eq!(a.get(), 226854911280625642308916404954512140970_u128);
        ///         },
        ///     None => { println!("Overflow happened."); },
        /// }
        /// 
        /// let b_longerunion = a_longerunion.unwrap().checked_mul(LongerUnion::new_with(2_u128));
        /// match b_longerunion
        /// {
        ///     Some(b) => { println!("{} * 2 = {}", a_longerunion.unwrap(), b); },
        ///     None => {
        ///             println!("Overflow happened.");
        ///             assert_eq!(b_longerunion, None);
        ///         },
        /// }
        /// 
        /// // Example for SizeUnion
        /// let a_sizeunion = SizeUnion::new_with(usize::MAX / 3).checked_mul(SizeUnion::new_with(2_usize));
        /// match a_sizeunion
        /// {
        ///     Some(a) => {
        ///             println!("{} * 2 = {}", SizeUnion::new_with(usize::MAX / 3), a);
        ///             #[cfg(target_pointer_width = "64")] assert_eq!(a.get(), 12297829382473034410_usize);
        ///         },
        ///     None => { println!("Overflow happened."); },
        /// }
        /// 
        /// let b_sizeunion = a_sizeunion.unwrap().checked_mul(SizeUnion::new_with(2_usize));
        /// match b_sizeunion
        /// {
        ///     Some(b) => { println!("{} * 2 = {}", a_sizeunion.unwrap(), b); },
        ///     None => {
        ///             println!("Overflow happened.");
        ///             assert_eq!(b_sizeunion, None);
        ///         },
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
        pub fn checked_mul(self, rhs: Self) -> Option<Self>
        {
            match self.get().checked_mul(rhs.get())
            {
                Some(res_this) =>   { Some(Self::new_with(res_this)) },
                None =>             { None },
            }
        }

        // fn unchecked_mul(self, rhs: Self) -> Self;
        /// Computes `self` + `rhs`, assuming overflow cannot occur.
        /// 
        /// # Arguments
        /// - `rhs` is a multiplier of the type `Self`
        /// 
        /// # Features
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
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// 
        /// let a_shortunion = ShortUnion::new_with(u16::MAX / 3).unchecked_mul(ShortUnion::new_with(2_u16));
        /// println!("{} * 2 = {}", ShortUnion::new_with(u16::MAX / 3), a_shortunion);
        /// assert_eq!(a_shortunion.get(), 43690_u16);
        /// 
        /// // It will panic
        /// // let b_shortunion = a_shortunion.unchecked_mul(ShortUnion::new_with(2_u16));
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// 
        /// let a_intunion = IntUnion::new_with(u32::MAX / 3).unchecked_mul(IntUnion::new_with(2_u32));
        /// println!("{} * 2 = {}", IntUnion::new_with(u32::MAX / 3), a_intunion);
        /// assert_eq!(a_intunion.get(), 2863311530_u32);
        /// 
        /// // It will panic
        /// // let b_intunion = a_intunion.unchecked_mul(IntUnion::new_with(2_u32));
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// 
        /// let a_longunion = LongUnion::new_with(u64::MAX / 3).unchecked_mul(LongUnion::new_with(2_u64));
        /// println!("{} * 2 = {}", LongUnion::new_with(u64::MAX / 3), a_longunion);
        /// assert_eq!(a_longunion.get(), 12297829382473034410_u64);
        /// 
        /// // It will panic
        /// // let b_longunion = a_longunion.unchecked_mul(LongUnion::new_with(2_u64));
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// 
        /// let a_longerunion = LongerUnion::new_with(u128::MAX / 3).unchecked_mul(LongerUnion::new_with(2_u128));
        /// println!("{} * 2 = {}", LongerUnion::new_with(u128::MAX / 3), a_longerunion);
        /// assert_eq!(a_longerunion.get(), 226854911280625642308916404954512140970_u128);
        /// 
        /// // It will panic
        /// // let b_longerunion = a_longerunion.unchecked_mul(LongerUnion::new_with(2_u128));
        /// ```
        /// 
        /// # Example 5 for SizeUnion
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// 
        /// let a_sizeunion = SizeUnion::new_with(usize::MAX / 3).unchecked_mul(SizeUnion::new_with(2_usize));
        /// println!("{} * 2 = {}", SizeUnion::new_with(usize::MAX / 3), a_sizeunion);
        /// #[cfg(target_pointer_width = "64")] assert_eq!(a_sizeunion.get(), 12297829382473034410_usize);
        /// 
        /// // It will panic
        /// // let b_sizeunion = a_sizeunion.unchecked_mul(SizeUnion::new_with(2_usize));
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
        #[inline] pub fn unchecked_mul(self, rhs: Self) -> Self     { Self::new_with( self.get().checked_mul(rhs.get()).unwrap() ) }

        // fn saturating_mul(self, rhs: Self) -> Self
        /// Computes `self` * `rhs`, saturating at the numeric bounds
        /// instead of overflowing.
        /// 
        /// # Arguments
        /// - `rhs` is a multiplier of the type `Self`
        /// 
        /// # Features
        /// It multiplies two numbers with saturating integer multiplication
        /// 
        /// # Output
        /// It returns the smaller one of `self` * `rhs` and the maxium
        /// of the type of `Self`.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// 
        /// let a_shortunion = ShortUnion::new_with(u16::MAX / 3).saturating_mul(ShortUnion::new_with(2_u16));
        /// println!("{} * 2 = {}", ShortUnion::new_with(u16::MAX / 3), a_shortunion);
        /// assert_eq!(a_shortunion.get(), 43690_u16);
        /// 
        /// let b_shortunion = a_shortunion.saturating_mul(ShortUnion::new_with(2_u16));
        /// println!("{} * 2 = {}", a_shortunion, b_shortunion);
        /// assert_eq!(b_shortunion.get(), u16::MAX);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// 
        /// let a_intunion = IntUnion::new_with(u32::MAX / 3).saturating_mul(IntUnion::new_with(2_u32));
        /// println!("{} * 2 = {}", IntUnion::new_with(u32::MAX / 3), a_intunion);
        /// assert_eq!(a_intunion.get(), 2863311530_u32);
        /// 
        /// let b_intunion = a_intunion.saturating_mul(IntUnion::new_with(2_u32));
        /// println!("{} * 2 = {}", a_intunion, b_intunion);
        /// assert_eq!(b_intunion.get(), u32::MAX);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// 
        /// let a_longunion = LongUnion::new_with(u64::MAX / 3).saturating_mul(LongUnion::new_with(2_u64));
        /// println!("{} * 2 = {}", LongUnion::new_with(u64::MAX / 3), a_longunion);
        /// assert_eq!(a_longunion.get(), 12297829382473034410_u64);
        /// 
        /// let b_longunion = a_longunion.saturating_mul(LongUnion::new_with(2_u64));
        /// println!("{} * 2 = {}", a_longunion, b_longunion);
        /// assert_eq!(b_longunion.get(), u64::MAX);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// 
        /// let a_longerunion = LongerUnion::new_with(u128::MAX / 3).saturating_mul(LongerUnion::new_with(2_u128));
        /// println!("{} * 2 = {}", LongerUnion::new_with(u128::MAX / 3), a_longerunion);
        /// assert_eq!(a_longerunion.get(), 226854911280625642308916404954512140970_u128);
        /// 
        /// let b_longerunion = a_longerunion.saturating_mul(LongerUnion::new_with(2_u128));
        /// println!("{} * 2 = {}", a_longerunion, b_longerunion);
        /// assert_eq!(b_longerunion.get(), u128::MAX);
        /// ```
        /// 
        /// # Example 5 for SizeUnion
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// 
        /// let a_sizeunion = SizeUnion::new_with(usize::MAX / 3).saturating_mul(SizeUnion::new_with(2_usize));
        /// println!("{} * 2 = {}", SizeUnion::new_with(usize::MAX / 3), a_sizeunion);
        /// #[cfg(target_pointer_width = "64")] assert_eq!(a_sizeunion.get(), 12297829382473034410_usize);
        /// 
        /// let b_sizeunion = a_sizeunion.saturating_mul(SizeUnion::new_with(2_usize));
        /// println!("{} * 2 = {}", a_sizeunion, b_sizeunion);
        /// assert_eq!(b_sizeunion.get(), usize::MAX);
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
        #[inline] pub fn saturating_mul(self, rhs: Self) -> Self    { Self::new_with( self.get().saturating_mul(rhs.get()) ) }

        // fn wrapping_div(self, rhs: Self) -> Self
        /// Computes `self`/ `rhs`.
        /// 
        /// # Arguments
        /// `rhs` is a divisor of the type `Self`.
        /// 
        /// # Features
        /// Wrapped division on unsigned types is just normal division. There’s no
        /// way wrapping could ever happen. This function exists, so that all
        /// operations are accounted for in the wrapping operations.
        /// 
        /// # Output
        /// It returns the `self` / `rhs` in the type of `Self`.
        /// 
        /// # Panics
        /// It will panic if rhs is zero.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, ShortUnion };
        /// 
        /// let a_shortunion = ShortUnion::new_with(u16::MAX / 3).wrapping_div(ShortUnion::new_with(2_u16));
        /// println!("{} / 2 = {}", ShortUnion::new_with(u16::MAX / 3), a_shortunion);
        /// assert_eq!(a_shortunion.get(), 10922_u16);
        /// 
        /// // It will panic.
        /// // let a_panic = ShortUnion::new_with(u16::MAX / 3).wrapping_div(ShortUnion::zero());
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, IntUnion };
        /// 
        /// let a_intunion = IntUnion::new_with(u32::MAX / 3).wrapping_div(IntUnion::new_with(2_u32));
        /// println!("{} / 2 = {}", IntUnion::new_with(u32::MAX / 3), a_intunion);
        /// assert_eq!(a_intunion.get(), 715827882_u32);
        /// 
        /// // It will panic.
        /// // let a_panic = IntUnion::new_with(u32::MAX / 3).wrapping_div(IntUnion::zero());
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, LongUnion };
        /// 
        /// let a_longunion = LongUnion::new_with(u64::MAX / 3).wrapping_div(LongUnion::new_with(2_u64));
        /// println!("{} / 2 = {}", LongUnion::new_with(u64::MAX / 3), a_longunion);
        /// assert_eq!(a_longunion.get(), 3074457345618258602_u64);
        /// 
        /// // It will panic.
        /// // let a_panic = LongUnion::new_with(u64::MAX / 3).wrapping_div(LongUnion::zero());
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, LongerUnion };
        /// 
        /// let a_longerunion = LongerUnion::new_with(u128::MAX / 3).wrapping_div(LongerUnion::new_with(2_u128));
        /// println!("{} / 2 = {}", LongerUnion::new_with(u128::MAX / 3), a_longerunion);
        /// assert_eq!(a_longerunion.get(), 56713727820156410577229101238628035242_u128);
        /// 
        /// // It will panic.
        /// // let a_panic = LongUnion::new_with(u128::MAX / 3).wrapping_div(LongerUnion::zero());
        /// ```
        /// 
        /// # Example 5 for SizeUnion for 64-bit CPUs
        /// ```
        /// use cryptocol::number::{ SmallUInt, SizeUnion };
        /// 
        /// let a_sizeunion = SizeUnion::new_with(usize::MAX / 3).wrapping_div(SizeUnion::new_with(2_usize));
        /// println!("{} / 2 = {}", SizeUnion::new_with(usize::MAX / 3), a_sizeunion);
        /// #[cfg(target_pointer_width = "64")] assert_eq!(a_sizeunion.get(), 3074457345618258602_usize);
        /// 
        /// // It will panic.
        /// // let a_panic = SizeUnion::new_with(usize::MAX / 3).wrapping_div(SizeUnion::zero());
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
        #[inline] pub fn wrapping_div(self, rhs: Self) -> Self      { Self::new_with( self.get().wrapping_div(rhs.get()) ) }
        // #[inline] pub fn wrapping_div_assign(&mut self, rhs: Self)  { self.this = self.get().wrapping_div(rhs.get()); }

        // fn overflowing_div(self, rhs: Self) -> (Self, bool)
        /// Calculates `self` / `rhs`.
        /// 
        /// # Arguments
        /// `rhs` is a divisor of the type `Self`.
        /// 
        /// # Features
        /// It divides `self` by `rhs`.
        /// 
        /// # Output
        /// It returns a tuple of the quotient along with a boolean indicating
        /// whether an arithmetic overflow would occur. Note that for unsigned
        /// integers overflow never occurs, so the second value is always `false`.
        /// 
        /// # Panics
        /// It will panic if `rhs` is zero.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, ShortUnion };
        /// 
        /// let (a_shortunion, overflow) = ShortUnion::new_with(u16::MAX / 3).overflowing_div(ShortUnion::new_with(2_u16));
        /// println!("{} / 2 = {}\nOverflow = {}", ShortUnion::new_with(u16::MAX / 3), a_shortunion, overflow);
        /// assert_eq!(a_shortunion.get(), 10922_u16);
        /// assert_eq!(overflow, false);
        /// 
        /// // It will panic.
        /// // let a_panic = a_shortunion.overflowing_div(ShortUnion::zero());
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, IntUnion };
        /// 
        /// let (a_intunion, overflow) = IntUnion::new_with(u32::MAX / 3).overflowing_div(IntUnion::new_with(2_u32));
        /// println!("{} / 2 = {}\nOverflow = {}", IntUnion::new_with(u32::MAX / 3), a_intunion, overflow);
        /// assert_eq!(a_intunion.get(), 715827882_u32);
        /// assert_eq!(overflow, false);
        /// 
        /// // It will panic.
        /// // let a_panic = a_intunion.overflowing_div(IntUnion::zero());
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, LongUnion };
        /// 
        /// let (a_longunion, overflow) = LongUnion::new_with(u64::MAX / 3).overflowing_div(LongUnion::new_with(2_u64));
        /// println!("{} / 2 = {}\nOverflow = {}", LongUnion::new_with(u64::MAX / 3), a_longunion, overflow);
        /// assert_eq!(a_longunion.get(), 3074457345618258602_u64);
        /// assert_eq!(overflow, false);
        /// 
        /// // It will panic.
        /// // let a_panic = a_longunion.overflowing_div(LongUnion::zero());
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, LongerUnion };
        /// 
        /// let (a_longerunion, overflow) = LongerUnion::new_with(u128::MAX / 3).overflowing_div(LongerUnion::new_with(2_u128));
        /// println!("{} / 2 = {}\nOverflow = {}", LongerUnion::new_with(u128::MAX / 3), a_longerunion, overflow);
        /// assert_eq!(a_longerunion.get(), 56713727820156410577229101238628035242_u128);
        /// assert_eq!(overflow, false);
        /// 
        /// // It will panic.
        /// // let a_panic = a_longerunion.overflowing_div(LongerUnion::zero());
        /// ```
        /// 
        /// # Example 5 for SizeUnion for 64-bit CPUs
        /// ```
        /// use cryptocol::number::{ SmallUInt, SizeUnion };
        /// 
        /// let (a_sizeunion, overflow) = SizeUnion::new_with(usize::MAX / 3).overflowing_div(SizeUnion::new_with(2_usize));
        /// println!("{} / 2 = {}\nOverflow = {}", SizeUnion::new_with(usize::MAX / 3), a_sizeunion, overflow);
        /// #[cfg(target_pointer_width = "64")] assert_eq!(a_sizeunion.get(), 3074457345618258602_usize);
        /// assert_eq!(overflow, false);
        /// 
        /// // It will panic.
        /// // let a_panic = a_sizeunion.overflowing_div(SizeUnion::zero());
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
        pub fn overflowing_div(self, rhs: Self) -> (Self, bool)
        {
            let (res_this, carry) = self.get().overflowing_div(rhs.get());
            (Self::new_with(res_this), carry)
        }

        // fn checked_div(self, rhs: Self) -> Option<Self>;
        /// Computes `self` / `rhs`.
        /// 
        /// # Arguments
        /// `rhs` is a divisor of the type `Self`.
        /// 
        /// # Output
        /// It returns `self` / `rhs` in the type `Self` wrapped by `Some`
        /// of enum `Option`. And, it returns `None` if rhs is zero.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, ShortUnion };
        /// 
        /// let a_shortunion = ShortUnion::new_with(u16::MAX / 3).checked_div(ShortUnion::new_with(2_u16));
        /// match a_shortunion
        /// {
        ///     Some(a) => {
        ///             println!("{} / 2 = {}", ShortUnion::new_with(u16::MAX / 3), a);
        ///             assert_eq!(a.get(), 10922_u16);
        ///         },
        ///     None => { println!("Divided by zero."); },
        /// }
        /// 
        /// let b_shortunion = ShortUnion::new_with(u16::MAX / 3).checked_div(ShortUnion::zero());
        /// match b_shortunion
        /// {
        ///     Some(b) => { println!("{} / 2 = {}", ShortUnion::new_with(u16::MAX / 3), b); },
        ///     None => {
        ///             println!("Divided by zero.");
        ///             assert_eq!(b_shortunion, None);
        ///         },
        /// }
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, IntUnion };
        /// 
        /// let a_intunion = IntUnion::new_with(u32::MAX / 3).checked_div(IntUnion::new_with(2_u32));
        /// match a_intunion
        /// {
        ///     Some(a) => {
        ///             println!("{} / 2 = {}", IntUnion::new_with(u32::MAX / 3), a);
        ///             assert_eq!(a.get(), 715827882_u32);
        ///         },
        ///     None => { println!("Divided by zero."); },
        /// }
        /// 
        /// let b_intunion = IntUnion::new_with(u32::MAX / 3).checked_div(IntUnion::zero());
        /// match b_intunion
        /// {
        ///     Some(b) => { println!("{} / 2 = {}", IntUnion::new_with(u32::MAX / 3), b); },
        ///     None => {
        ///             println!("Divided by zero.");
        ///             assert_eq!(b_intunion, None);
        ///         },
        /// }
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, LongUnion };
        /// 
        /// let a_longunion = LongUnion::new_with(u64::MAX / 3).checked_div(LongUnion::new_with(2_u64));
        /// match a_longunion
        /// {
        ///     Some(a) => {
        ///             println!("{} / 2 = {}", LongUnion::new_with(u64::MAX / 3), a);
        ///             assert_eq!(a.get(), 3074457345618258602_u64);
        ///         },
        ///     None => { println!("Divided by zero."); },
        /// }
        /// 
        /// let b_longunion = LongUnion::new_with(u64::MAX / 3).checked_div(LongUnion::zero());
        /// match b_longunion
        /// {
        ///     Some(b) => { println!("{} / 2 = {}", LongUnion::new_with(u64::MAX / 3), b); },
        ///     None => {
        ///             println!("Divided by zero.");
        ///             assert_eq!(b_longunion, None);
        ///         },
        /// }
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, LongerUnion };
        /// 
        /// let a_longerunion = LongerUnion::new_with(u128::MAX / 3).checked_div(LongerUnion::new_with(2_u128));
        /// match a_longerunion
        /// {
        ///     Some(a) => {
        ///             println!("{} / 2 = {}", LongerUnion::new_with(u128::MAX / 3), a);
        ///             assert_eq!(a.get(), 56713727820156410577229101238628035242_u128);
        ///         },
        ///     None => { println!("Divided by zero."); },
        /// }
        /// 
        /// let b_longerunion = LongerUnion::new_with(u128::MAX / 3).checked_div(LongerUnion::zero());
        /// match b_longerunion
        /// {
        ///     Some(b) => { println!("{} / 2 = {}", LongerUnion::new_with(u128::MAX / 3), b); },
        ///     None => {
        ///             println!("Divided by zero.");
        ///             assert_eq!(b_longerunion, None);
        ///         },
        /// }
        /// ```
        /// 
        /// # Example 5 for SizeUnion for 64-bit CPUs
        /// ```
        /// use cryptocol::number::{ SmallUInt, SizeUnion };
        /// 
        /// let a_sizeunion = SizeUnion::new_with(usize::MAX / 3).checked_div(SizeUnion::new_with(2_usize));
        /// match a_sizeunion
        /// {
        ///     Some(a) => {
        ///             println!("{} / 2 = {}", SizeUnion::new_with(usize::MAX / 3), a);
        ///             #[cfg(target_pointer_width = "64")] assert_eq!(a.get(), 3074457345618258602_usize);
        ///         },
        ///     None => { println!("Divided by zero."); },
        /// }
        /// 
        /// let b_sizeunion = SizeUnion::new_with(usize::MAX / 3).checked_div(SizeUnion::zero());
        /// match b_sizeunion
        /// {
        ///     Some(b) => { println!("{} / 2 = {}", SizeUnion::new_with(usize::MAX / 3), b); },
        ///     None => {
        ///             println!("Divided by zero.");
        ///             assert_eq!(b_sizeunion, None);
        ///         },
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
        pub fn checked_div(self, rhs: Self) -> Option<Self>
        {
            match self.get().checked_div(rhs.get())
            {
                Some(res_this) =>   { Some(Self::new_with(res_this)) },
                None =>             { None },
            }
        }

        // fn saturating_div(self, rhs: Self) -> Self
        /// Computes `self` / `rhs`.
        /// 
        /// # Features
        /// 
        /// # Output
        /// It returns `self` / `rhs` in the type `Self`.
        /// 
        /// # Panics
        /// It will panic if rhs is zero.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, ShortUnion };
        /// 
        /// let a_shortunion = ShortUnion::new_with(u16::MAX / 3).saturating_div(ShortUnion::new_with(2_u16));
        /// println!("{} / 2 = {}", ShortUnion::new_with(u16::MAX / 3), a_shortunion);
        /// assert_eq!(a_shortunion.get(), 10922_u16);
        /// 
        /// // It will panic.
        /// // let a_panic = ShortUnion::new_with(u16::MAX / 3).saturating_div(ShortUnion::zero());
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, IntUnion };
        /// 
        /// let a_intunion = IntUnion::new_with(u32::MAX / 3).saturating_div(IntUnion::new_with(2_u32));
        /// println!("{} / 2 = {}", IntUnion::new_with(u32::MAX / 3), a_intunion);
        /// assert_eq!(a_intunion.get(), 715827882_u32);
        /// 
        /// // It will panic.
        /// // let a_panic = IntUnion::new_with(u32::MAX / 3).saturating_div(IntUnion::zero()));
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, LongUnion };
        /// 
        /// let a_longunion = LongUnion::new_with(u64::MAX / 3).saturating_div(LongUnion::new_with(2_u64));
        /// println!("{} / 2 = {}", LongUnion::new_with(u64::MAX / 3), a_longunion);
        /// assert_eq!(a_longunion.get(), 3074457345618258602_u64);
        /// 
        /// // It will panic.
        /// // let a_panic = LongUnion::new_with(u64::MAX / 3).saturating_div(LongUnion::zero());
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, LongerUnion };
        /// 
        /// let a_longerunion = LongerUnion::new_with(u128::MAX / 3).saturating_div(LongerUnion::new_with(2_u128));
        /// println!("{} / 2 = {}", LongerUnion::new_with(u128::MAX / 3), a_longerunion);
        /// assert_eq!(a_longerunion.get(), 56713727820156410577229101238628035242_u128);
        /// 
        /// // It will panic.
        /// // let a_panic = LongerUnion::new_with(u128::MAX / 3).saturating_div(LongerUnion::zero());
        /// ```
        /// 
        /// # Example 5 for SizeUnion for 64-bit CPUs
        /// ```
        /// use cryptocol::number::{ SmallUInt, SizeUnion };
        /// 
        /// let a_sizeunion = SizeUnion::new_with(usize::MAX / 3).saturating_div(SizeUnion::new_with(2_usize));
        /// println!("{} / 2 = {}", SizeUnion::new_with(usize::MAX / 3), a_sizeunion);
        /// #[cfg(target_pointer_width = "64")] assert_eq!(a_sizeunion.get(), 3074457345618258602_usize);
        /// 
        /// // It will panic.
        /// // let a_panic = SizeUnion::new_with(usize::MAX / 3).saturating_div(SizeUnion::zero());
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
        #[inline] pub fn saturating_div(self, rhs: Self) -> Self    { Self::new_with( self.get().saturating_div(rhs.get()) ) }

        // fn wrapping_rem(self, rhs: Self) -> Self
        /// Computes `self` % `rhs`.
        /// 
        /// # Arguments
        /// `rhs` is divisor of the type `Self`.
        /// 
        /// # Features
        /// Wrapped remainder calculation on unsigned types is just the regular
        /// remainder calculation. There’s no way wrapping could ever happen.
        /// This function exists, so that all operations are accounted for in the
        /// wrapping operations.
        /// 
        /// # Output
        /// It returns the `self` % `rhs` in the type of `Self`.
        /// 
        /// # Panics
        /// It will panic if rhs is zero.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, ShortUnion };
        /// 
        /// let a_shortunion = ShortUnion::new_with(u16::MAX / 3).wrapping_rem(ShortUnion::new_with(3_u16));
        /// println!("{} % 3 = {}", ShortUnion::new_with(u16::MAX / 3), a_shortunion);
        /// assert_eq!(a_shortunion.get(), 2_u16);
        /// 
        /// // It will panic.
        /// // let a_panic = ShortUnion::new_with(u16::MAX / 3).wrapping_rem(ShortUnion::zero());
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, IntUnion };
        /// 
        /// let a_intunion = IntUnion::new_with(u32::MAX / 3).wrapping_rem(IntUnion::new_with(3_u32));
        /// println!("{} % 3 = {}", IntUnion::new_with(u32::MAX / 3), a_intunion);
        /// assert_eq!(a_intunion.get(), 1_u32);
        /// 
        /// // It will panic.
        /// // let a_panic = IntUnion::new_with(u32::MAX / 3).wrapping_rem(IntUnion::zero());
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, LongUnion };
        /// 
        /// let a_longunion = LongUnion::new_with(u64::MAX / 3).wrapping_rem(LongUnion::new_with(3_u64));
        /// println!("{} % 3 = {}", LongUnion::new_with(u64::MAX / 3), a_longunion);
        /// assert_eq!(a_longunion.get(), 2_u64);
        /// 
        /// // It will panic.
        /// // let a_panic = LongUnion::new_with(u64::MAX / 3).wrapping_rem(IntUnion::zero());
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, LongerUnion };
        /// 
        /// let a_longerunion = LongerUnion::new_with(u128::MAX / 3).wrapping_rem(LongerUnion::new_with(3_u128));
        /// println!("{} % 3 = {}", LongerUnion::new_with(u128::MAX / 3), a_longerunion);
        /// assert_eq!(a_longerunion.get(), 1_u128);
        /// 
        /// // It will panic.
        /// // let a_panic = LongerUnion::new_with(u128::MAX / 3).wrapping_rem(LongerUnion::zero());
        /// ```
        /// 
        /// # Example 5 for SizeUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, SizeUnion };
        /// 
        /// let a_sizeunion = SizeUnion::new_with(usize::MAX / 3).wrapping_rem(SizeUnion::new_with(3_usize));
        /// println!("{} % 3 = {}", SizeUnion::new_with(usize::MAX / 3), a_sizeunion);
        /// assert_eq!(a_sizeunion.get(), 2_usize);
        /// 
        /// // It will panic.
        /// // let a_panic = sSizeUnion::new_with(usize::MAX / 3).wrapping_rem(SizeUnion::zero());
        /// ```
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

        #[inline] pub fn wrapping_rem(self, rhs: Self) -> Self      { Self::new_with( self.get().wrapping_rem(rhs.get()) ) }
        // #[inline] pub fn wrapping_rem_assign(&mut self, rhs: Self)  { self.set(self.get().wrapping_rem(rhs.get())); }

        // fn overflowing_rem(self, rhs: Self) -> (Self, bool)
        /// Calculates `self` % `rhs`.
        /// 
        /// # Arguments
        /// `rhs` is divisor of the type `Self`.
        /// 
        /// # Features
        /// It calculates the remainder when self is divided by rhs.
        /// 
        /// # Output
        /// It returns a tuple of the remainder along with a boolean indicating
        /// whether an arithmetic overflow would occur. Note that for unsigned
        /// integers overflow never occurs, so the second value is always `false`.
        /// 
        /// # Panics
        /// It will panic if rhs is zero.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, ShortUnion };
        /// 
        /// let (a_shortunion, overflow) = ShortUnion::new_with(u16::MAX / 3).overflowing_rem(ShortUnion::new_with(3_u16));
        /// println!("{} % 3 = {}\nOverflow = {}", ShortUnion::new_with(u16::MAX / 3), a_shortunion, overflow);
        /// assert_eq!(a_shortunion.get(), 2_u16);
        /// assert_eq!(overflow, false);
        /// 
        /// // It will panic.
        /// // let a_panic = ShortUnion::new_with(u16::MAX / 3).overflowing_rem(ShortUnion::zero());
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, IntUnion };
        /// 
        /// let (a_intunion, overflow) = IntUnion::new_with(u32::MAX / 3).overflowing_rem(IntUnion::new_with(3_u32));
        /// println!("{} % 3 = {}\nOverflow = {}", IntUnion::new_with(u32::MAX / 3), a_intunion, overflow);
        /// assert_eq!(a_intunion.get(), 1_u32);
        /// assert_eq!(overflow, false);
        /// 
        /// // It will panic.
        /// // let a_panic = IntUnion::new_with(u32::MAX / 3).overflowing_rem(IntUnion::zero());
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, LongUnion };
        /// 
        /// let (a_longunion, overflow) = LongUnion::new_with(u64::MAX / 3).overflowing_rem(LongUnion::new_with(3_u64));
        /// println!("{} % 3 = {}\nOverflow = {}", LongUnion::new_with(u64::MAX / 3), a_longunion, overflow);
        /// assert_eq!(a_longunion.get(), 2_u64);
        /// assert_eq!(overflow, false);
        /// 
        /// // It will panic.
        /// // let a_panic = LongUnion::new_with(u64::MAX / 3).overflowing_rem(LongUnion::zero());
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, LongerUnion };
        /// 
        /// let (a_longerunion, overflow) = LongerUnion::new_with(u128::MAX / 3).overflowing_rem(LongerUnion::new_with(3_u128));
        /// println!("{} % 3 = {}\nOverflow = {}", LongerUnion::new_with(u128::MAX / 3), a_longerunion, overflow);
        /// assert_eq!(a_longerunion.get(), 1_u128);
        /// assert_eq!(overflow, false);
        /// 
        /// // It will panic.
        /// // let a_panic = LongerUnion::new_with(u128::MAX / 3).overflowing_rem(LongerUnion::zero());
        /// ```
        /// 
        /// # Example 5 for SizeUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, SizeUnion };
        /// 
        /// let (a_sizeunion, overflow) = SizeUnion::new_with(usize::MAX / 3).overflowing_rem(SizeUnion::new_with(3_usize));
        /// println!("{} % 3 = {}\nOverflow = {}", SizeUnion::new_with(usize::MAX / 3), a_sizeunion, overflow);
        /// assert_eq!(a_sizeunion.get(), 2_usize);
        /// assert_eq!(overflow, false);
        /// 
        /// // It will panic.
        /// // let a_panic = SizeUnion::new_with(usize::MAX / 3).overflowing_rem(SizeUnion::zero());
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
        pub fn overflowing_rem(self, rhs: Self) -> (Self, bool)
        {
            let (res_this, carry) = self.get().overflowing_rem(rhs.get());
            (Self::new_with(res_this), carry)
        }

        // fn checked_rem(self, rhs: Self) -> Option<Self>
        /// Computes `self` % `rhs`.
        /// 
        /// # Arguments
        /// `rhs` is divisor of the type `Self`.
        /// 
        /// # Output
        /// It returns `self` % `rhs` in the type `Self` wrapped by `Some`
        /// of enum `Option`. And, it returns `None` if rhs is zero.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, ShortUnion };
        /// 
        /// let a_shortunion = ShortUnion::new_with(u16::MAX / 3).checked_rem(ShortUnion::new_with(3_u16));
        /// match a_shortunion
        /// {
        ///     Some(a) => {
        ///             println!("{} % 3 = {}", ShortUnion::new_with(u16::MAX / 3), a);
        ///             assert_eq!(a.get(), 2_u16);
        ///         },
        ///     None => { println!("Divided by zero."); },
        /// }
        /// 
        /// let b_shortunion = ShortUnion::new_with(u16::MAX / 3).checked_rem(ShortUnion::zero());
        /// match b_shortunion
        /// {
        ///     Some(b) => { println!("{} % 3 = {}", ShortUnion::new_with(u16::MAX / 3), b); },
        ///     None => {
        ///             println!("Divided by zero.");
        ///             assert_eq!(b_shortunion, None);
        ///         },
        /// }
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, IntUnion };
        /// 
        /// let a_intunion = IntUnion::new_with(u32::MAX / 3).checked_rem(IntUnion::new_with(3_u32));
        /// match a_intunion
        /// {
        ///     Some(a) => {
        ///             println!("{} % 3 = {}", IntUnion::new_with(u32::MAX / 3), a);
        ///             assert_eq!(a.get(), 1_u32);
        ///         },
        ///     None => { println!("Divided by zero."); },
        /// }
        /// 
        /// let b_intunion = IntUnion::new_with(u32::MAX / 3).checked_rem(IntUnion::zero());
        /// match b_intunion
        /// {
        ///     Some(b) => { println!("{} % 3 = {}", IntUnion::new_with(u32::MAX / 3), b); },
        ///     None => {
        ///             println!("Divided by zero.");
        ///             assert_eq!(b_intunion, None);
        ///         },
        /// }
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, LongUnion };
        /// 
        /// let a_longunion = LongUnion::new_with(u64::MAX / 3).checked_rem(LongUnion::new_with(3_u64));
        /// match a_longunion
        /// {
        ///     Some(a) => {
        ///             println!("{} % 3 = {}", LongUnion::new_with(u64::MAX / 3), a);
        ///             assert_eq!(a.get(), 2_u64);
        ///         },
        ///     None => { println!("Divided by zero."); },
        /// }
        /// 
        /// let b_longunion = LongUnion::new_with(u64::MAX / 3).checked_rem(LongUnion::zero());
        /// match b_longunion
        /// {
        ///     Some(b) => { println!("{} % 3 = {}", LongUnion::new_with(u64::MAX / 3), b); },
        ///     None => {
        ///             println!("Divided by zero.");
        ///             assert_eq!(b_longunion, None);
        ///         },
        /// }
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, LongerUnion };
        /// 
        /// let a_longerunion = LongerUnion::new_with(u128::MAX / 3).checked_rem(LongerUnion::new_with(3_u128));
        /// match a_longerunion
        /// {
        ///     Some(a) => {
        ///             println!("{} % 3 = {}", LongerUnion::new_with(u128::MAX / 3), a);
        ///             assert_eq!(a.get(), 1_u128);
        ///         },
        ///     None => { println!("Divided by zero."); },
        /// }
        /// 
        /// let b_longerunion = LongerUnion::new_with(u128::MAX / 3).checked_rem(LongerUnion::zero());
        /// match b_longerunion
        /// {
        ///     Some(b) => { println!("{} % 3 = {}", LongerUnion::new_with(u128::MAX / 3), b); },
        ///     None => {
        ///             println!("Divided by zero.");
        ///             assert_eq!(b_longerunion, None);
        ///         },
        /// }
        /// ```
        /// 
        /// # Example 5 for SizeUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, SizeUnion };
        /// 
        /// let a_sizeunion = SizeUnion::new_with(usize::MAX / 3).checked_rem(SizeUnion::new_with(3_usize));
        /// match a_sizeunion
        /// {
        ///     Some(a) => {
        ///             println!("{} % 3 = {}", SizeUnion::new_with(usize::MAX / 3), a);
        ///             assert_eq!(a.get(), 2_usize);
        ///         },
        ///     None => { println!("Divided by zero."); },
        /// }
        /// 
        /// let b_sizeunion = SizeUnion::new_with(usize::MAX / 3).checked_rem(SizeUnion::zero());
        /// match b_sizeunion
        /// {
        ///     Some(b) => { println!("{} % 3 = {}", SizeUnion::new_with(usize::MAX / 3), b); },
        ///     None => {
        ///             println!("Divided by zero.");
        ///             assert_eq!(b_sizeunion, None);
        ///         },
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
        pub fn checked_rem(self, rhs: Self) -> Option<Self>
        {
            match self.get().checked_rem(rhs.get())
            {
                Some(res_this) =>   { Some(Self::new_with(res_this)) },
                None =>             { None },
            }
        }

        // pub const fn wrapping_neg(self) -> Self;
        /// Computes `-self`, wrapping around at the boundary of the type.
        /// 
        /// # Feature
        /// - Wrapping (modular) negation. Since unsigned types do not have negative
        /// equivalents all applications of this function will wrap (except for -0).
        /// - For values smaller than the corresponding signed type’s maximum the
        /// result is the same as casting the corresponding signed value.
        /// - Any larger values are equivalent to MAX + 1 - (val - MAX - 1)
        /// where MAX is the corresponding signed type’s maximum.
        /// 
        /// # Output
        /// It returns `-self`, wrapping around at the boundary of the type.
        /// 
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// let a_shortunion = ShortUnion::new_with(12345_u16);
        /// let b_shortunion = a_shortunion.wrapping_neg();
        /// println!("-{} = {}", a_shortunion, b_shortunion);
        /// assert_eq!(b_shortunion.get(), 53191_u16);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// let a_intunion = IntUnion::new_with(1234567890_u32);
        /// let b_intunion = a_intunion.wrapping_neg();
        /// println!("-{} = {}", a_intunion, b_intunion);
        /// assert_eq!(b_intunion.get(), 3060399406_u32);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let a_longunion = LongUnion::new_with(12345678901234567890_u64);
        /// let b_longunion = a_longunion.wrapping_neg();
        /// println!("-{} = {}", a_longunion, b_longunion);
        /// assert_eq!(b_longunion.get(), 6101065172474983726_u64);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let a_longerunion = LongerUnion::new_with(123456789012345678901234567890123456789_u128);
        /// let b_longerunion = a_longerunion.wrapping_neg();
        /// println!("-{} = {}", a_longerunion, b_longerunion);
        /// assert_eq!(b_longerunion.get(), 216825577908592784562140039541644754667_u128);
        /// ```
        /// 
        /// # Example 5 for SizeUnion for 64-bit CPU
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// let a_sizeunion = SizeUnion::new_with(1234567890123456789_usize);
        /// let b_sizeunion = a_sizeunion.wrapping_neg();
        /// println!("-{} = {}", a_sizeunion, a_sizeunion);
        /// assert_eq!(b_sizeunion.get(), 17212176183586094827_usize);
        /// ```
        /// 
        /// # Plagiarism in descryption
        /// It calls the method wrapping_neg() of implementation of the primitive
        /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
        /// `usize` directly. So, all the description of this method is mainly the
        /// same as that of the method wrapping_neg() of implementation of the
        /// primitive unsigned integer types except example codes. Confer to the
        /// descryptions that are linked to in the section _Reference_. This
        /// plagiarism is not made maliciously but is made for the reason of
        /// effectiveness and efficiency so that users may understand better and
        /// easily how to use this method with simiilarity to the method
        /// wrapping_neg() of implementation of the primitive unsigned integer types.
        #[inline] pub fn wrapping_neg(self) -> Self             { Self::new_with( self.get().wrapping_neg() ) }

        // pub const fn overflowing_neg(self) -> (Self, bool);
        /// Negates `self` in an overflowing fashion.
        /// 
        /// # Feature
        /// - Note that for positive unsigned values overflow always occurs,
        /// but negating 0 does not overflow.
        /// 
        /// # Output
        /// A pair of `!self + 1` using wrapping operations to return the value
        /// that represents the negation of this unsigned value,
        /// and whether it overflows
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// let a_shortunion = ShortUnion::new_with(0_u16);
        /// let (b_shortunion, overflow) = a_shortunion.overflowing_neg();
        /// println!("-{} = {}, {}", a_shortunion, b_shortunion, overflow);
        /// assert_eq!(b_shortunion.get(), 0_u16);
        /// assert_eq!(overflow, false);
        /// 
        /// let c_shortunion = ShortUnion::new_with(12345_u16);
        /// let (d_shortunion, overflow) = c_shortunion.overflowing_neg();
        /// println!("-{} = {}, {}", c_shortunion, d_shortunion, overflow);
        /// assert_eq!(d_shortunion.get(), 53191_u16);
        /// assert_eq!(overflow, true);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// let a_intunion = IntUnion::new_with(0_u32);
        /// let (b_intunion, overflow) = a_intunion.overflowing_neg();
        /// println!("-{} = {}, {}", a_intunion, b_intunion, overflow);
        /// assert_eq!(b_intunion.get(), 0_u32);
        /// assert_eq!(overflow, false);
        /// 
        /// let c_intunion = IntUnion::new_with(1234567890_u32);
        /// let (d_intunion, overflow) = c_intunion.overflowing_neg();
        /// println!("-{} = {}, {}", c_intunion, d_intunion, overflow);
        /// assert_eq!(d_intunion.get(), 3060399406_u32);
        /// assert_eq!(overflow, true);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// let a_longunion = LongUnion::new_with(0_u64);
        /// let (b_longunion , overflow) = a_longunion.overflowing_neg();
        /// println!("-{} = {}, {}", a_longunion, b_longunion, overflow);
        /// assert_eq!(b_longunion.get(), 0_u64);
        /// assert_eq!(overflow, false);
        /// 
        /// let c_longunion = LongUnion::new_with(12345678901234567890_u64);
        /// let (d_longunion, overflow) = c_longunion.overflowing_neg();
        /// println!("-{} = {}, {}", c_longunion, d_longunion, overflow);
        /// assert_eq!(d_longunion.get(), 6101065172474983726_u64);
        /// assert_eq!(overflow, true);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// let a_longerunion = LongerUnion::new_with(0_u128);
        /// let (b_longerunion, overflow) = a_longerunion.overflowing_neg();
        /// println!("-{} = {}, {}", a_longerunion, b_longerunion, overflow);
        /// assert_eq!(b_longerunion.get(), 0_u128);
        /// assert_eq!(overflow, false);
        /// 
        /// let c_longerunion = LongerUnion::new_with(123456789012345678901234567890123456789_u128);
        /// let (d_longerunion, overflow) = c_longerunion.overflowing_neg();
        /// println!("-{} = {}, {}", c_longerunion, d_longerunion, overflow);
        /// assert_eq!(d_longerunion.get(), 216825577908592784562140039541644754667_u128);
        /// assert_eq!(overflow, true);
        /// ```
        /// 
        /// # Example 5 for SizeUnion for 64-bit CPU
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// let a_sizeunion = SizeUnion::new_with(0_usize);
        /// let (b_sizeunion, overflow) = a_sizeunion.overflowing_neg();
        /// println!("-{} = {}, {}", a_sizeunion, a_sizeunion, overflow);
        /// assert_eq!(b_sizeunion.get(), 0_usize);
        /// assert_eq!(overflow, false);
        /// 
        /// let c_sizeunion = SizeUnion::new_with(1234567890123456789_usize);
        /// let (d_sizeunion, overflow) = c_sizeunion.overflowing_neg();
        /// println!("-{} = {}, {}", c_sizeunion, d_sizeunion, overflow);
        /// assert_eq!(d_sizeunion.get(), 17212176183586094827_usize);
        /// assert_eq!(overflow, true);
        /// ```
        /// 
        /// # Plagiarism in descryption
        /// It calls the method overflowing_neg() of implementation of the primitive
        /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
        /// `usize` directly. So, all the description of this method is mainly the
        /// same as that of the method overflowing_neg() of implementation of the
        /// primitive unsigned integer types except example codes. Confer to the
        /// descryptions that are linked to in the section _Reference_. This
        /// plagiarism is not made maliciously but is made for the reason of
        /// effectiveness and efficiency so that users may understand better and
        /// easily how to use this method with simiilarity to the method
        /// overflowing_neg() of implementation of the primitive unsigned integer types.
        #[inline] pub fn overflowing_neg(self) -> (Self, bool)  { let a = self.get().overflowing_neg(); (Self::new_with(a.0), a.1) }

        // fn pow(self, exp: u32) -> Self;
        /// Raises `self` to the power of `exp`, using exponentiation by squaring.
        /// 
        /// # Arguments
        /// `exp` is the exponential of the type of `u32`.
        /// 
        /// # Features
        /// In release mode, it does not panic but works with wrapping (modular)
        /// exponentiation in the same way of wrapping_pow().
        /// 
        /// # Panics
        /// In debug mode, it will panic if the result of this method is more
        /// than the possible maximum value.
        /// 
        /// # Output
        /// It returns the self raised to the power of exp, in the type of `Self`.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// 
        /// let a_shortunion = ShortUnion::new_with(9);
        /// let b_shortunion = a_shortunion.pow(5_u32);
        /// println!("9 ** 5 = {}, where ** is the power operator", b_shortunion);
        /// assert_eq!(b_shortunion.get(), 59049_u16);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// 
        /// let a_intunion = IntUnion::new_with(81);
        /// let b_intunion = a_intunion.pow(5_u32);
        /// println!("81 ** 5 = {}, where ** is the power operator", b_intunion);
        /// assert_eq!(b_intunion.get(), 3486784401_u32);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// 
        /// let a_longunion = LongUnion::new_with(6561);
        /// let b_longunion = a_longunion.pow(5_u32);
        /// println!("6561 ** 5 = {}, where ** is the power operator", b_longunion);
        /// assert_eq!(b_longunion.get(), 12157665459056928801_u64);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// 
        /// let a_longerunion = LongerUnion::new_with(43046721);
        /// let b_longerunion = a_longerunion.pow(5_u32);
        /// println!("43046721 ** 5 = {}, where ** is the power operator", b_longerunion);
        /// assert_eq!(b_longerunion.get(), 147808829414345923316083210206383297601_u128);
        /// ```
        /// 
        /// # Example 5 for SizeUnion
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// 
        /// let a_sizeunion = SizeUnion::new_with(3);
        /// let b_sizeunion = a_sizeunion.pow(5_u32);
        /// println!("3 ** 5 = {}, where ** is the power operator", b_sizeunion);
        /// assert_eq!(b_sizeunion.get(), 243_usize);
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
        #[inline] pub fn pow(self, exp: u32) -> Self    { Self::new_with( self.get().pow(exp) ) }

        // fn wrapping_pow(self, exp: u32) -> Self;
        /// Computes self.pow(exp) with wrapping around at the boundary of the type.
        /// 
        /// # Arguments
        /// `exp` is the exponential of the type of `u32`.
        /// 
        /// # Features
        /// Wrapping (modular) exponentiation.
        /// 
        /// # Output
        /// It returns the self raised to the power of exp, in the type of `Self`.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// 
        /// let a_shortunion = ShortUnion::new_with(9);
        /// let b_shortunion = a_shortunion.wrapping_pow(5_u32);
        /// println!("9 ** 5 = {}, where ** is the power operator", b_shortunion);
        /// assert_eq!(b_shortunion.get(), 59049_u16);
        /// 
        /// let c_shortunion = a_shortunion.wrapping_pow(6_u32);
        /// println!("9 ** 6 = {}, where ** is the power operator", c_shortunion);
        /// assert_eq!(c_shortunion.get(), 7153_u16);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// 
        /// let a_intunion = IntUnion::new_with(81);
        /// let b_intunion = a_intunion.wrapping_pow(5_u32);
        /// println!("81 ** 5 = {}, where ** is the power operator", b_intunion);
        /// assert_eq!(b_intunion.get(), 3486784401_u32);
        /// 
        /// let c_intunion = a_intunion.wrapping_pow(6_u32);
        /// println!("81 ** 6 = {}, where ** is the power operator", c_intunion);
        /// assert_eq!(c_intunion.get(), 3256662241_u32);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// 
        /// let a_longunion = LongUnion::new_with(6561);
        /// let b_longunion = a_longunion.wrapping_pow(5_u32);
        /// println!("6561 ** 5 = {}, where ** is the power operator", b_longunion);
        /// assert_eq!(b_longunion.get(), 12157665459056928801_u64);
        /// 
        /// let c_longunion = a_longunion.wrapping_pow(6_u32);
        /// println!("6561 ** 6 = {}, where ** is the power operator", c_longunion);
        /// assert_eq!(c_longunion.get(), 2721702152408675777_u64);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// 
        /// let a_longerunion = LongerUnion::new_with(43046721_u128);
        /// let b_longerunion = a_longerunion.wrapping_pow(5_u32);
        /// println!("43046721 ** 5 = {}, where ** is the power operator", b_longerunion);
        /// assert_eq!(b_longerunion.get(), 147808829414345923316083210206383297601_u128);
        /// 
        /// let c_longerunion = a_longerunion.wrapping_pow(6_u32);
        /// println!("43046721 ** 6 = {}, where ** is the power operator", c_longerunion);
        /// assert_eq!(c_longerunion.get(), 333574137813082321045752866839264852865_u128);
        /// ```
        /// 
        /// # Example 5 for SizeUnion
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// 
        /// let a_sizeunion = SizeUnion::new_with(3);
        /// let b_sizeunion = a_sizeunion.wrapping_pow(5_u32);
        /// println!("3 ** 5 = {}, where ** is the power operator", b_sizeunion);
        /// assert_eq!(b_sizeunion.get(), 243_usize);
        /// 
        /// let c_sizeunion = a_sizeunion.wrapping_pow(128_u32);
        /// println!("3 ** 128 = {}, where ** is the power operator", c_sizeunion);
        /// // #[cfg(target_pointer_width = "8")] assert_eq!(c_sizeunion.get(), 1_usize);
        /// #[cfg(target_pointer_width = "16")] assert_eq!(c_sizeunion.get(), 31233_usize);
        /// #[cfg(target_pointer_width = "32")] assert_eq!(c_sizeunion.get(), 2324068865_usize);
        /// #[cfg(target_pointer_width = "64")] assert_eq!(c_sizeunion.get(), 9241971931925084673_usize);
        /// // #[cfg(target_pointer_width = "128")] assert_eq!(c_sizeunion.get(), 303523815449207866983105381828026333697_usize);
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
        #[inline] pub fn wrapping_pow(self, exp: u32) -> Self   { Self::new_with( self.get().wrapping_pow(exp) ) }

        // fn overflowing_pow(self, exp: u32) -> (Self, bool);
        /// Raises self to the power of exp, using exponentiation by squaring.
        /// 
        /// # Arguments
        /// `exp` is the exponential of the type of `u32`.
        /// 
        /// # Features
        /// Exponentiation by squaring.
        /// 
        /// # Output
        /// It returns a tuple of the exponentiation along with a bool indicating
        /// whether an overflow happened.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// 
        /// let (a_shortunion, overflow) = ShortUnion::new_with(12_u16).overflowing_pow(4);
        /// println!("{} ** 4 = {}, where ** is the power operator\nOverflow = {}", 12_u16, a_shortunion, overflow);
        /// assert_eq!(a_shortunion.get(), 20736_u16);
        /// assert_eq!(overflow, false);
        ///    
        /// let (b_shortunion, overflow) = ShortUnion::new_with(12_u16).overflowing_pow(5);
        /// println!("{} ** 5 = {}, where ** is the power operator\nOverflow = {}", 12_u16, b_shortunion, overflow);
        /// assert_eq!(b_shortunion.get(), 52224_u16);
        /// assert_eq!(overflow, true);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// 
        /// let (a_intunion, overflow) = IntUnion::new_with(38_u32).overflowing_pow(6);
        /// println!("{} ** 6 = {}, where ** is the power operator\nOverflow = {}", 38_u32, a_intunion, overflow);
        /// assert_eq!(a_intunion.get(), 3010936384_u32);
        /// assert_eq!(overflow, false);
        ///    
        /// let (b_intunion, overflow) = IntUnion::new_with(38_u32).overflowing_pow(7);
        /// println!("{} ** 7 = {}, where ** is the power operator\nOverflow = {}", 38_u32, b_intunion, overflow);
        /// assert_eq!(b_intunion.get(), 2746432896_u32);
        /// assert_eq!(overflow, true);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// 
        /// let (a_longunion, overflow) = LongUnion::new_with(1004_u64).overflowing_pow(6);
        /// println!("{} ** 6 = {}, where ** is the power operator\nOverflow = {}", 1004_u64, a_longunion, overflow);
        /// assert_eq!(a_longunion.get(), 1024241283846148096_u64);
        /// assert_eq!(overflow, false);
        ///    
        /// let (b_longunion, overflow) = LongUnion::new_with(1004_u64).overflowing_pow(7);
        /// println!("{} ** 7 = {}, where ** is the power operator\nOverflow = {}", 1004_u64, b_longunion, overflow);
        /// assert_eq!(b_longunion.get(), 13767324927507349504_u64);
        /// assert_eq!(overflow, true);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// 
        /// let (a_longerunion, overflow) = LongerUnion::new_with(10003_u128).overflowing_pow(9);
        /// println!("{} ** 9 = {}, where ** is the power operator\nOverflow = {}", 10003_u128, a_longerunion, overflow);
        /// assert_eq!(a_longerunion.get(), 1002703242269020906241243873790509683_u128);
        /// assert_eq!(overflow, false);
        ///    
        /// let (b_longerunion, overflow) = LongerUnion::new_with(10003_u128).overflowing_pow(10);
        /// println!("{} ** 10 = {}, where ** is the power operator\nOverflow = {}", 10003_u128, b_longerunion, overflow);
        /// assert_eq!(b_longerunion.get(), 161851891709800684693298854005190226825_u128);
        /// assert_eq!(overflow, true);
        /// ```
        /// 
        /// # Example 5 for SizeUnion
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// 
        /// let (a_sizeunion, overflow) = SizeUnion::new_with(3_usize).overflowing_pow(5);
        /// println!("{} ** 5 = {}, where ** is the power operator\nOverflow = {}", 3_usize, a_sizeunion, overflow);
        /// assert_eq!(a_sizeunion.get(), 243_usize);
        /// assert_eq!(overflow, false);
        ///    
        /// let (b_sizeunion, overflow) = SizeUnion::new_with(3_usize).overflowing_pow(128);
        /// println!("{} ** 128 = {}, where ** is the power operator\nOverflow = {}", 3_u64, b_sizeunion, overflow);
        /// // #[cfg(target_pointer_width = "8")] assert_eq!(b_sizeunion.get(), 1_usize);
        /// #[cfg(target_pointer_width = "16")] assert_eq!(b_sizeunion.get(), 31233_usize);
        /// #[cfg(target_pointer_width = "32")] assert_eq!(b_sizeunion.get(), 2324068865_usize);
        /// #[cfg(target_pointer_width = "64")] assert_eq!(b_sizeunion.get(), 9241971931925084673_usize);
        /// // #[cfg(target_pointer_width = "128")] assert_eq!(b_sizeunion.get(), 303523815449207866983105381828026333697_usize);
        /// assert_eq!(overflow, true);
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
        pub fn overflowing_pow(self, exp: u32) -> (Self, bool)
        {
            let (res_this, carry) = self.get().overflowing_pow(exp);
            (Self::new_with(res_this), carry)
        }

        // fn checked_pow(self, exp: u32) -> Option<Self>;
        /// Computes self.pow(exp), returning None if overflow occurred.
        /// 
        /// # Arguments
        /// `exp` is the exponential of the type of `u32`.
        /// 
        /// # Output
        /// It returns None if overflow occurred. Otherwise, it returns 'self
        /// raised to the power of exp' wrapped by `Some` of enum `Option`.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// 
        /// let a_shortunion = ShortUnion::new_with(12_u16).checked_pow(4);
        /// match a_shortunion
        /// {
        ///     Some(a) => {
        ///             println!("{} ** 4 = {}, where ** is the power operator", ShortUnion::new_with(12_u16), a);
        ///             assert_eq!(a.get(), 20736_u16);
        ///         },
        ///     None => { println!("Overflow happened."); },
        /// }
        /// 
        /// let b_shortunion = ShortUnion::new_with(12_u16).checked_pow(5);
        /// match b_shortunion
        /// {
        ///     Some(b) => { println!("{} ** 5 = {}, where ** is the power operator", ShortUnion::new_with(12_u16), b); },
        ///     None => {
        ///             println!("Overflow happened.");
        ///             assert_eq!(b_shortunion, None);
        ///         },
        /// }
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// 
        /// let a_intunion = IntUnion::new_with(38_u32).checked_pow(6);
        /// match a_intunion
        /// {
        ///     Some(a) => {
        ///             println!("{} ** 6 = {}, where ** is the power operator", IntUnion::new_with(38_u32), a);
        ///             assert_eq!(a.get(), 3010936384_u32);
        ///         },
        ///     None => { println!("Overflow happened."); },
        /// }
        /// 
        /// let b_intunion = IntUnion::new_with(38_u32).checked_pow(7);
        /// match b_intunion
        /// {
        ///     Some(b) => { println!("{} ** 7 = {}, where ** is the power operator", IntUnion::new_with(38_u32), b); },
        ///     None => {
        ///             println!("Overflow happened.");
        ///             assert_eq!(b_intunion, None);
        ///         },
        /// }
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// 
        /// let a_longunion = LongUnion::new_with(1004_u64).checked_pow(6);
        /// match a_longunion
        /// {
        ///     Some(a) => {
        ///             println!("{} ** 6 = {}, where ** is the power operator", LongUnion::new_with(1004_u64), a);
        ///             assert_eq!(a.get(), 1024241283846148096_u64);
        ///         },
        ///     None => { println!("Overflow happened."); },
        /// }
        /// 
        /// let b_longunion = LongUnion::new_with(1004_u64).checked_pow(7);
        /// match b_longunion
        /// {
        ///     Some(b) => { println!("{} ** 7 = {}, where ** is the power operator", LongUnion::new_with(1004_u64), b); },
        ///     None => {
        ///             println!("Overflow happened.");
        ///             assert_eq!(b_longunion, None);
        ///         },
        /// }
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// 
        /// let a_longerunion = LongerUnion::new_with(10003_u128).checked_pow(9);
        /// match a_longerunion
        /// {
        ///     Some(a) => {
        ///             println!("{} ** 9 = {}, where ** is the power operator", LongerUnion::new_with(10003_u128), a);
        ///             assert_eq!(a.get(), 1002703242269020906241243873790509683_u128);
        ///         },
        ///     None => { println!("Overflow happened."); },
        /// }
        /// 
        /// let b_longerunion = LongerUnion::new_with(10003_u128).checked_pow(10);
        /// match b_longerunion
        /// {
        ///     Some(b) => { println!("{} ** 10 = {}, where ** is the power operator", LongerUnion::new_with(10003_u128), b); },
        ///     None => {
        ///             println!("Overflow happened.");
        ///             assert_eq!(b_longerunion, None);
        ///         },
        /// }
        /// ```
        /// 
        /// # Example 5 for SizeUnion
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// 
        /// let a_sizeunion = SizeUnion::new_with(3_usize).checked_pow(5);
        /// match a_sizeunion
        /// {
        ///     Some(a) => {
        ///             println!("{} ** 6 = {}, where ** is the power operator", SizeUnion::new_with(3_usize), a);
        ///             assert_eq!(a.get(), 243_usize);
        ///         },
        ///     None => { println!("Overflow happened."); },
        /// }
        /// 
        /// let b_sizeunion = SizeUnion::new_with(3_usize).checked_pow(128);
        /// match b_sizeunion
        /// {
        ///     Some(b) => { println!("{} ** 128 = {}, where ** is the power operator", SizeUnion::new_with(3_usize), b); },
        ///     None => {
        ///             println!("Overflow happened.");
        ///             assert_eq!(b_sizeunion, None);
        ///         },
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
        pub fn checked_pow(self, exp: u32) -> Option<Self>
        {
            match self.get().checked_pow(exp)
            {
                Some(res_this) =>   { Some(Self::new_with(res_this)) },
                None =>             { None },
            }
        }

        // fn unchecked_pow(self, exp: u32) -> Self
        /// Computes self.pow(exp), unless overflow does not occcurred.
        /// Otherwise, it will panic.
        /// 
        /// # Arguments
        /// `exp` is the exponential of the type of `u32`.
        /// 
        /// # Panics
        /// This method will be panic if overflow occurred.
        /// Use this method only when you are sure that overflow won't occur.
        /// 
        /// # Output
        /// It returns the result of `self` raised to the power of `exp`
        /// if overflow does not occur.
        /// 
        /// # Arguments
        /// The argument `exp` is the primitive unsigned integer type u32.
        /// 
        /// # Features
        /// Wrapping (modular) exponentiation.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// 
        /// let a_shortunion = ShortUnion::new_with(12_u16).unchecked_pow(4);
        /// println!("{} ** 4 = {}, where ** is the power operator", ShortUnion::new_with(12_u16), a_shortunion);
        /// assert_eq!(a_shortunion.get(), 20736_u16);
        /// // It will panic.
        /// // let b_shortunion = ShortUnion::new_with(12_u16).unchecked_pow(5);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// 
        /// let a_intunion = IntUnion::new_with(38_u32).unchecked_pow(6);
        /// println!("{} ** 6 = {}, where ** is the power operator", IntUnion::new_with(38_u32), a_intunion);
        /// assert_eq!(a_intunion.get(), 3010936384_u32);
        /// // It will panic.
        /// // let b_intunion = IntUnion::new_with(38_u32).unchecked_pow(7);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// 
        /// let a_longunion = LongUnion::new_with(1004_u64).unchecked_pow(6);
        /// println!("{} ** 6 = {}, where ** is the power operator", LongUnion::new_with(1004_u64), a_longunion);
        /// assert_eq!(a_longunion.get(), 1024241283846148096_u64);
        /// // It will panic.
        /// // let b_longunion = LongUnion::new_with(1004_u64).unchecked_pow(7);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// 
        /// let a_longerunion = LongerUnion::new_with(10003_u128).unchecked_pow(9);
        /// println!("{} ** 9 = {}, where ** is the power operator", LongerUnion::new_with(10003_u128), a_longerunion);
        /// assert_eq!(a_longerunion.get(), 1002703242269020906241243873790509683_u128);
        /// // It will panic.
        /// // let b_longerunion = LongerUnion::new_with(10003_u128).unchecked_pow(10);
        /// ```
        /// 
        /// # Example 5 for SizeUnion
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// 
        /// let a_sizeunion = SizeUnion::new_with(3_usize).unchecked_pow(5);
        /// println!("{} ** 5 = {}, where ** is the power operator", SizeUnion::new_with(3_usize), a_sizeunion);
        /// assert_eq!(a_sizeunion.get(), 243_usize);
        /// // It will panic.
        /// // let b_sizeunion = SizeUnion::new_with(3_usize).unchecked_pow(128);
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
        pub fn unchecked_pow(self, exp: u32) -> Self    { self.checked_pow(exp).unwrap() }

        // fn saturating_pow(self, exp: u32) -> Self;
        /// Computes self.pow(exp), saturating at the numeric bounds instead of overflowing.
        /// 
        /// # Arguments
        /// `exp` is the exponential of the type of `u32`.
        /// 
        /// # Features
        /// Saturating integer exponentiation.
        /// 
        /// # Output
        /// It returns 'self raised to the power of exp' if overflow does not happen.
        /// Otherwise, it returns the maximum value.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// 
        /// let a_shortunion = ShortUnion::new_with(12_u16).saturating_pow(4);
        /// println!("{} ** 4 = {}, where ** is the power operator", ShortUnion::new_with(12_u16), a_shortunion);
        /// assert_eq!(a_shortunion.get(), 20736_u16);
        /// 
        /// let b_shortunion = ShortUnion::new_with(12_u16).saturating_pow(5);
        /// println!("{} ** 5 = {}, where ** is the power operator", ShortUnion::new_with(12_u16), b_shortunion);
        /// assert_eq!(b_shortunion.get(), u16::MAX);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// 
        /// let a_intunion = IntUnion::new_with(38_u32).saturating_pow(6);
        /// println!("{} ** 6 = {}, where ** is the power operator", IntUnion::new_with(38_u32), a_intunion);
        /// assert_eq!(a_intunion.get(), 3010936384_u32);
        /// 
        /// let b_intunion = IntUnion::new_with(38_u32).saturating_pow(7);
        /// println!("{} ** 7 = {}, where ** is the power operator", IntUnion::new_with(38_u32), b_intunion);
        /// assert_eq!(b_intunion.get(), u32::MAX);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// 
        /// let a_longunion = LongUnion::new_with(1004_u64).saturating_pow(6);
        /// println!("{} ** 6 = {}, where ** is the power operator", LongUnion::new_with(1004_u64), a_longunion);
        /// assert_eq!(a_longunion.get(), 1024241283846148096_u64);
        /// 
        /// let b_longunion =  LongUnion::new_with(1004_u64).saturating_pow(7);
        /// println!("{} ** 7 = {}, where ** is the power operator", LongUnion::new_with(1004_u64), b_longunion);
        /// assert_eq!(b_longunion.get(), u64::MAX);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// 
        /// let a_longerunion = LongerUnion::new_with(10003_u128).saturating_pow(9);
        /// println!("{} ** 9 = {}, where ** is the power operator", LongerUnion::new_with(10003_u128), a_longerunion);
        /// assert_eq!(a_longerunion.get(), 1002703242269020906241243873790509683_u128);
        /// 
        /// let b_longerunion = LongerUnion::new_with(10003_u128).saturating_pow(10);
        /// println!("{} ** 10 = {}, where ** is the power operator", LongerUnion::new_with(10003_u128), b_longerunion);
        /// assert_eq!(b_longerunion.get(), u128::MAX);
        /// ```
        /// 
        /// # Example 5 for SizeUnion
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// 
        /// let a_sizeunion = SizeUnion::new_with(3_usize).saturating_pow(5);
        /// println!("{} ** 5 = {}, where ** is the power operator", SizeUnion::new_with(3_usize), a_sizeunion);
        /// assert_eq!(a_sizeunion.get(), 243_usize);
        /// 
        /// let b_sizeunion = SizeUnion::new_with(3_usize).saturating_pow(128);
        /// println!("{} ** 128 = {}, where ** is the power operator", SizeUnion::new_with(3_usize), b_sizeunion);
        /// assert_eq!(b_sizeunion.get(), usize::MAX);
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
        #[inline] pub fn saturating_pow(self, exp: u32) -> Self { Self::new_with( self.get().saturating_pow(exp) ) }


        // fn ilog(self, base: Self) -> u32;
        /// Returns the logarithm of the number with respect to an arbitrary base.
        /// 
        /// # Arguments
        /// `base` is the base of the logarithm.
        /// 
        /// # Output
        /// The logarithm of the number with respect to an arbitrary base,
        /// rounded down
        /// 
        /// # Features
        /// - Usually the result of logarithm is float point number. So, it rounds
        /// down the logarithm result if it is not fit to interger.
        /// - This method might not be optimized owing to implementation details;
        /// ilog2 can produce results more efficiently for base 2, and
        /// ilog10 can produce results more efficiently for base 10.
        /// 
        /// # Panics
        /// This function will panic if `self` is zero, or if `base` is less than 2.
        /// So, use this method only when `self` is non-zero and `base` is not less
        /// than 2.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, ShortUnion };
        /// 
        /// let a_shortunion = ShortUnion::new_with(10000_u16);
        /// let base_shortunion = ShortUnion::new_with(5_u16);
        /// let res = a_shortunion.ilog(base_shortunion);
        /// println!("log_{} ({}) = {}", base_shortunion, a_shortunion, res);
        /// assert_eq!(res, 5_u32);
        /// 
        /// // It will panic.
        /// // let res =ShortUnion::zero().ilog(base_shortunion);
        /// 
        /// // It will panic.
        /// // let res = a_shortunion.ilog(ShortUnion::zero());
        /// 
        /// // It will panic.
        /// // let res = ShortUnion::zero().ilog(ShortUnion::zero());
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, IntUnion };
        /// 
        /// let a_intunion = IntUnion::new_with(1000000000_u32);
        /// let base_intunion = IntUnion::new_with(7_u32);
        /// let res = a_intunion.ilog(base_intunion);
        /// println!("log_{} ({}) = {}", base_intunion, a_intunion, res);
        /// assert_eq!(res, 10_u32);
        /// 
        /// // It will panic.
        /// // let res = IntUnion::zero().ilog(base_intunion);
        /// 
        /// // It will panic.
        /// // let res = a_intunion.ilog(IntUnion::zero());
        /// 
        /// // It will panic.
        /// // let res = IntUnion::zero().ilog(IntUnion::zero());
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, LongUnion };
        /// 
        /// let a_longunion = LongUnion::new_with(10000000000000000000_u64);
        /// let base_longunion = LongUnion::new_with(11_u64);
        /// let res = a_longunion.ilog(base_longunion);
        /// println!("log_{} ({}) = {}", base_longunion, a_longunion, res);
        /// assert_eq!(res, 18_u32);
        /// 
        /// // It will panic.
        /// // let res = LongUnion::zero().ilog(base_longunion);
        /// 
        /// // It will panic.
        /// // let res = a_longunion.ilog(LongUnion::zero());
        /// 
        /// // It will panic.
        /// // let res = LongUnion::zero().ilog(LongUnion::zero());
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, LongerUnion };
        /// 
        /// let a_longerunion = LongerUnion::new_with(100000000000000000000000000000000000000_u128);
        /// let base_longerunion = LongerUnion::new_with(13_u128);
        /// let res = a_longerunion.ilog(base_longerunion);
        /// println!("log_{} ({}) = {}", base_longerunion, a_longerunion, res);
        /// assert_eq!(res, 34_u32);
        /// 
        /// // It will panic.
        /// // let res = LongerUnion::zero().ilog(base_longerunion);
        /// 
        /// // It will panic.
        /// // let res = a_longerunion.ilog(LongerUnion::zero());
        /// 
        /// // It will panic.
        /// // let res = LongerUnion::zero().ilog(LongerUnion::zero());
        /// ```
        /// 
        /// # Example 5 for SizeUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, SizeUnion };
        /// 
        /// let a_sizeunion = SizeUnion::new_with(100_usize);
        /// let base_sizeunion = SizeUnion::new_with(3_usize);
        /// let res = a_sizeunion.ilog(base_sizeunion);
        /// println!("log_{} ({}) = {}", base_sizeunion, a_sizeunion, res);
        /// assert_eq!(res, 4_u32);
        /// 
        /// // It will panic.
        /// // let res = SizeUnion::zero().ilog(base_sizeunion);
        /// 
        /// // It will panic.
        /// // let res = a_sizeunion.ilog(SizeUnion::zero());
        /// 
        /// // It will panic.
        /// // let res = SizeUnion::zero().ilog(SizeUnion::zero());
        /// ```
        /// 
        /// # Plagiarism in descryption
        /// It calls the method ilog() of implementation of the primitive
        /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
        /// `usize` directly. So, all the description of this method is mainly the
        /// same as that of the method ilog() of implementation of the
        /// primitive unsigned integer types except example codes. Confer to the
        /// descryptions that are linked to in the section _Reference_. This
        /// plagiarism is not made maliciously but is made for the reason of
        /// effectiveness and efficiency so that users may understand better and
        /// easily how to use this method with simiilarity to the method
        /// ilog() of implementation of the primitive unsigned integer types.
        #[inline] pub fn ilog(self, base: Self) -> u32  { self.get().ilog(base.get()) }

        // fn ilog10(self) -> u32;
        /// Returns the base 10 logarithm of the number.
        /// 
        /// # Output
        /// The logarithm of the number with respect to an arbitrary base,
        /// rounded down.
        /// 
        /// # Features
        /// - Usually the result of logarithm is float point number.
        /// So, it rounds down the logarithm result if it is not fit to interger.
        /// 
        /// # Panics
        /// This function will panic if `self` is zero.
        /// So, use this method only when `self` is non-zero.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, ShortUnion };
        /// 
        /// let a_shortunion = ShortUnion::new_with(10000_u16);
        /// let res = a_shortunion.ilog10();
        /// println!("log_10 ({}) = {}", a_shortunion, res);
        /// assert_eq!(res, 4_u32);
        /// 
        /// // It will panic.
        /// // let res = ShortUnion::zero().ilog10();
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, IntUnion };
        /// 
        /// let a_intunion = IntUnion::new_with(1000000000_u32);
        /// let res = a_intunion.ilog10();
        /// println!("log_10 ({}) = {}", a_intunion, res);
        /// assert_eq!(res, 9_u32);
        /// 
        /// // It will panic.
        /// // let res = IntUnion::zero().ilog10();
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, LongUnion };
        /// 
        /// let a_longunion = LongUnion::new_with(10000000000000000000_u64);
        /// let res = a_longunion.ilog10();
        /// println!("log_10 ({}) = {}", a_longunion, res);
        /// assert_eq!(res, 19_u32);
        /// 
        /// // It will panic.
        /// // let res = LongUnion::zero().ilog10();
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, LongerUnion };
        /// 
        /// let a_longerunion = LongerUnion::new_with(100000000000000000000000000000000000000_u128);
        /// let res = a_longerunion.ilog10();
        /// println!("log_10 ({}) = {}", a_longerunion, res);
        /// assert_eq!(res, 38_u32);
        /// 
        /// // It will panic.
        /// // let res = LongerUnion::zero().ilog10();
        /// ```
        /// 
        /// # Example 5 for SizeUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, SizeUnion };
        /// 
        /// let a_sizeunion = SizeUnion::new_with(100_usize);
        /// let res = a_sizeunion.ilog10();
        /// println!("log_10 ({}) = {}", a_sizeunion, res);
        /// assert_eq!(res, 2_u32);
        /// 
        /// // It will panic.
        /// // let res = SizeUnion::zero().ilog10();
        /// ```
        /// 
        /// # Plagiarism in descryption
        /// It calls the method ilog10() of implementation of the primitive
        /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
        /// `usize` directly. So, all the description of this method is mainly the
        /// same as that of the method ilog10() of implementation of the
        /// primitive unsigned integer types except example codes. Confer to the
        /// descryptions that are linked to in the section _Reference_. This
        /// plagiarism is not made maliciously but is made for the reason of
        /// effectiveness and efficiency so that users may understand better and
        /// easily how to use this method with simiilarity to the method
        /// ilog10() of implementation of the primitive unsigned integer types.
        #[inline] pub fn ilog10(self) -> u32            { self.get().ilog10() }

        // fn ilog2(self) -> u32;
        /// Returns the base 2 logarithm of the number, rounded down.
        /// 
        /// # Output
        /// The logarithm of the number with respect to an arbitrary base,
        /// rounded down
        /// 
        /// # Features
        /// - Usually the result of logarithm is float point number. So, it rounds
        /// down the logarithm result if it is not fit to interger.
        /// 
        /// # Panics
        /// This function will panic if `self` is zero.
        /// So, use this method only when `self` is non-zero.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, ShortUnion };
        /// 
        /// let a_shortunion = ShortUnion::new_with(10000_u16);
        /// let res = a_shortunion.ilog2();
        /// println!("log_2 ({}) = {}", a_shortunion, res);
        /// assert_eq!(res, 13_u32);
        /// 
        /// // It will panic.
        /// // let res = ShortUnion::zero().ilog2();
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, IntUnion };
        /// 
        /// let a_intunion = IntUnion::new_with(1000000000_u32);
        /// let res = a_intunion.ilog2();
        /// println!("log_2 ({}) = {}", a_intunion, res);
        /// assert_eq!(res, 29_u32);
        /// 
        /// // It will panic.
        /// // let res = IntUnion::zero().ilog2();
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, LongUnion };
        /// 
        /// let a_longunion = LongUnion::new_with(10000000000000000000_u64);
        /// let res = a_longunion.ilog2();
        /// println!("log_2 ({}) = {}", a_longunion, res);
        /// assert_eq!(res, 63_u32);
        /// 
        /// // It will panic.
        /// // let res = LongUnion::zero().ilog2();
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, LongerUnion };
        /// 
        /// let a_longerunion = LongerUnion::new_with(100000000000000000000000000000000000000_u128);
        /// let res = a_longerunion.ilog2();
        /// println!("log_2 ({}) = {}", a_longerunion, res);
        /// assert_eq!(res, 126_u32);
        /// 
        /// // It will panic.
        /// // let res = LongerUnion::zero().ilog2();
        /// ```
        /// 
        /// # Example 5 for SizeUnion
        /// ```
        /// use cryptocol::number::{ SmallUInt, SizeUnion };
        /// 
        /// let a_sizeunion = SizeUnion::new_with(100_usize);
        /// let res = a_sizeunion.ilog2();
        /// println!("log_2 ({}) = {}", a_sizeunion, res);
        /// assert_eq!(res, 6_u32);
        /// 
        /// // It will panic.
        /// // let res = SizeUnion::zero().ilog2();
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not encouraged
        /// to use it for Big Endian CPUs for serious purpose. Only use this crate
        /// for Big-endian CPUs with your own full responsibility.
        /// Tests a `SmallUInt`-type object to find whether or not it is a
        /// prime number.
        /// 
        /// # Plagiarism in descryption
        /// It calls the method ilog2() of implementation of the primitive
        /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
        /// `usize` directly. So, all the description of this method is mainly the
        /// same as that of the method ilog2() of implementation of the
        /// primitive unsigned integer types except example codes. Confer to the
        /// descryptions that are linked to in the section _Reference_. This
        /// plagiarism is not made maliciously but is made for the reason of
        /// effectiveness and efficiency so that users may understand better and
        /// easily how to use this method with simiilarity to the method
        /// ilog2() of implementation of the primitive unsigned integer types.
        #[inline] pub fn ilog2(self) -> u32             { self.get().ilog2() }

        // fn isqrt(self) -> Self;
        /// Returns the square root of the number.
        /// 
        /// # Output
        /// The square root of the number, rounded down
        /// 
        /// # Features
        /// - Usually the result of square root is float point number.
        /// So, it rounds down the square root result if it is not fit to interger.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// 
        /// let a_shortunion = ShortUnion::new_with(10000_u16);
        /// let res_shortunion = a_shortunion.isqrt();
        /// println!("isqrt( {} ) = {}", a_shortunion, res_shortunion);
        /// assert_eq!(res_shortunion.get(), 100_u16);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// 
        /// let a_intunion = IntUnion::new_with(1000000000_u32);
        /// let res_intunion = a_intunion.isqrt();
        /// println!("isqrt( {} ) = {}", a_intunion, res_intunion);
        /// assert_eq!(res_intunion.get(), 31622_u32);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// 
        /// let a_longunion = LongUnion::new_with(10000000000000000000_u64);
        /// let res_longunion = a_longunion.isqrt();
        /// println!("isqrt( {} ) = {}", a_longunion, res_longunion);
        /// assert_eq!(res_longunion.get(), 3162277660_u64);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// 
        /// let a_longerunion = LongerUnion::new_with(100000000000000000000000000000000000000_u128);
        /// let res_longerunion = a_longerunion.isqrt();
        /// println!("isqrt( {} ) = {}", a_longerunion, res_longerunion);
        /// assert_eq!(res_longerunion.get(), 10000000000000000000_u128);
        /// ```
        /// 
        /// # Example 5 for SizeUnion
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// 
        /// let a_sizeunion = SizeUnion::new_with(100_usize);
        /// let res_sizeunion = a_sizeunion.isqrt();
        /// println!("isqrt( {} ) = {}", a_sizeunion, res_sizeunion);
        /// assert_eq!(res_sizeunion.get(), 10_usize);
        /// ```
        /// 
        /// # Big-endian issue
        /// It is just experimental for Big Endian CPUs. So, you are not encouraged
        /// to use it for Big Endian CPUs for serious purpose. Only use this crate
        /// for Big-endian CPUs with your own full responsibility.
        /// Tests a `SmallUInt`-type object to find whether or not it is a
        /// prime number.
        /// 
        /// # Plagiarism in descryption
        /// It calls the method isqrt() of implementation of the primitive
        /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
        /// `usize` directly. So, all the description of this method is mainly the
        /// same as that of the method isqrt() of implementation of the
        /// primitive unsigned integer types except example codes. Confer to the
        /// descryptions that are linked to in the section _Reference_. This
        /// plagiarism is not made maliciously but is made for the reason of
        /// effectiveness and efficiency so that users may understand better and
        /// easily how to use this method with simiilarity to the method
        /// isqrt() of implementation of the primitive unsigned integer types.
        #[inline] pub fn isqrt(self) -> Self            { Self::new_with( SmallUInt::isqrt(self.get()) ) }

        // fn reverse_bits(self) -> Self;
        /// Reverses the order of bits in the integer.
        /// 
        /// # Features
        /// The (LSB) least significant bit becomes the most significant bit,
        /// secondly least significant bit becomes secondly most significant bit,
        /// etc.
        /// 
        /// # Output
        /// the number that has the bit order opposite to that of `self`.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// 
        /// let c_shortunion = ShortUnion::new_with(0b1011001110001111_u16);
        /// let d_shortunion = c_shortunion.reverse_bits();
        /// println!("{:016b} -> {:016b}", c_shortunion.get(), d_shortunion.get());
        /// assert_eq!(d_shortunion.get(), 0b1111000111001101_u16);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// 
        /// let c_intunion = IntUnion::new_with(0b10110011100011110000111110000011_u32);
        /// let d_intunion = c_intunion.reverse_bits();
        /// println!("{:032b} -> {:032b}", c_intunion.get(), d_intunion.get());
        /// assert_eq!(d_intunion.get(), 0b11000001111100001111000111001101_u32);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// 
        /// let c_longunion = LongUnion::new_with(0b1011001110001111000011111000001111110000001111111000000011111111_u64);
        /// let d_longunion = c_longunion.reverse_bits();
        /// println!("{:064b} -> {:064b}", c_longunion.get(), d_longunion.get());
        /// assert_eq!(d_longunion.get(), 0b1111111100000001111111000000111111000001111100001111000111001101_u64);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// 
        /// let c_longerunion = LongerUnion::new_with(0b10110011100011110000111110000011111100000011111110000000111111110000000011111111100000000011111111110000000000111111111110000000_u128);
        /// let d_longerunion = c_longerunion.reverse_bits();
        /// println!("{:0128b} -> {:0128b}", c_longerunion.get(), d_longerunion.get());
        /// assert_eq!(d_longerunion.get(), 0b00000001111111111100000000001111111111000000000111111111000000001111111100000001111111000000111111000001111100001111000111001101_u128);
        /// ```
        /// 
        /// # Example 5 for SizeUnion for 64-bit CPUs
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// 
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     let c_sizeunion = SizeUnion::new_with(0b1011001110001111000011111000001111110000001111111000000011111111_usize);
        ///     let d_sizeunion = c_sizeunion.reverse_bits();
        ///     println!("{:064b} -> {:064b}", c_sizeunion.get(), d_sizeunion.get());
        ///     assert_eq!(d_sizeunion.get(), 0b1111111100000001111111000000111111000001111100001111000111001101_usize);
        /// }
        /// ```
        /// 
        /// # Plagiarism in descryption
        /// It calls the method reverse_bits() of implementation of the primitive
        /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
        /// `usize` directly. So, all the description of this method is mainly the
        /// same as that of the method reverse_bits() of implementation of the
        /// primitive unsigned integer types except example codes. Confer to the
        /// descryptions that are linked to in the section _Reference_. This
        /// plagiarism is not made maliciously but is made for the reason of
        /// effectiveness and efficiency so that users may understand better and
        /// easily how to use this method with simiilarity to the method
        /// reverse_bits() of implementation of the primitive unsigned integer types.
        #[inline] pub fn reverse_bits(self) -> Self     { Self::new_with( self.get().reverse_bits() ) }

        // fn rotate_left(self, n: u32) -> Self
        /// Shifts the bits to the left by a specified amount, `n`, wrapping the
        /// truncated bits to the end of the resulting integer.
        /// 
        /// # Output
        /// The number whose bits are shifted to the left by a specified amount,
        /// `n`, wrapping the truncated bits to the end of the resulting integer
        /// 
        /// # Arguments
        /// `n` indicates how many bits will be shifted to the left
        /// 
        /// # Caution
        /// Please note this method does not perform the same operation as the
        /// `<<` shifting operator!
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// 
        /// let a_shortunion = ShortUnion::new_with(0b1011001110001111_u16);
        /// let b_shortunion = a_shortunion.rotate_left(4);
        /// println!("{:016b} -> {:016b}", a_shortunion.get(), b_shortunion.get());
        /// assert_eq!(b_shortunion.get(), 0b0011100011111011_u16);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// 
        /// let a_intunion = IntUnion::new_with(0b10110011100011110000111110000011_u32);
        /// let b_intunion = a_intunion.rotate_left(8);
        /// println!("{:032b} -> {:032b}", a_intunion.get(), b_intunion.get());
        /// assert_eq!(b_intunion.get(), 0b10001111000011111000001110110011_u32);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// 
        /// let a_longunion = LongUnion::new_with(0b1011001110001111000011111000001111110000001111111000000011111111_u64);
        /// let b_longunion = a_longunion.rotate_left(16);
        /// println!("{:064b} -> {:064b}", a_longunion.get(), b_longunion.get());
        /// assert_eq!(b_longunion.get(), 0b0000111110000011111100000011111110000000111111111011001110001111_u64);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// 
        /// let a_longerunion = LongerUnion::new_with(0b10110011100011110000111110000011111100000011111110000000111111110000000011111111100000000011111111110000000000111111111110000000_u128);
        /// let b_longerunion = a_longerunion.rotate_left(32);
        /// println!("{:0128b} -> {:0128b}", a_longerunion.get(), b_longerunion.get());
        /// assert_eq!(b_longerunion.get(), 0b11110000001111111000000011111111000000001111111110000000001111111111000000000011111111111000000010110011100011110000111110000011_u128);
        /// ```
        /// 
        /// # Example 5 for SizeUnion for 64-bit CPUs
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// 
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     let a_sizeunion = SizeUnion::new_with(0b1011001110001111000011111000001111110000001111111000000011111111_usize);
        ///     let b_sizeunion = a_sizeunion.rotate_left(16);
        ///     println!("{:064b} -> {:064b}", a_sizeunion.get(), b_sizeunion.get());
        ///     assert_eq!(b_sizeunion.get(), 0b0000111110000011111100000011111110000000111111111011001110001111_usize);
        /// }
        /// ```
        /// 
        /// # Plagiarism in descryption
        /// It calls the method rotate_left() of implementation of the primitive
        /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
        /// `usize` directly. So, all the description of this method is mainly the
        /// same as that of the method rotate_left() of implementation of the
        /// primitive unsigned integer types except example codes. Confer to the
        /// descryptions that are linked to in the section _Reference_. This
        /// plagiarism is not made maliciously but is made for the reason of
        /// effectiveness and efficiency so that users may understand better and
        /// easily how to use this method with simiilarity to the method
        /// rotate_left() of implementation of the primitive unsigned integer types.
        #[inline] pub fn rotate_left(self, n: u32) -> Self  { Self::new_with( self.get().rotate_left(n) ) }

        // fn rotate_right(self, n: u32) -> Self
        /// Shifts the bits to the right by a specified amount, `n`, wrapping the
        /// truncated bits to the end of the resulting integer.
        /// 
        /// # Output
        /// The number whose bits are shifted to the right by a specified amount,
        /// `n`, wrapping the truncated bits to the end of the resulting integer
        /// 
        /// # Arguments
        /// `n` indicates how many bits will be shifted to the right
        /// 
        /// # Caution
        /// Please note this method does not perform the same operation as the
        /// `>>` shifting operator!
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// 
        /// let a_shortunion = ShortUnion::new_with(0b1011001110001111_u16);
        /// let b_shortunion = a_shortunion.rotate_right(4);
        /// println!("{:016b} -> {:016b}", a_shortunion.get(), b_shortunion.get());
        /// assert_eq!(b_shortunion.get(), 0b1111101100111000_u16);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// 
        /// let a_intunion = IntUnion::new_with(0b10110011100011110000111110000011_u32);
        /// let b_intunion = a_intunion.rotate_right(8);
        /// println!("{:032b} -> {:032b}", a_intunion.get(), b_intunion.get());
        /// assert_eq!(b_intunion.get(), 0b10000011101100111000111100001111_u32);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// 
        /// let a_longunion = LongUnion::new_with(0b1011001110001111000011111000001111110000001111111000000011111111_u64);
        /// let b_longunion = a_longunion.rotate_right(16);
        /// println!("{:064b} -> {:064b}", a_longunion.get(), b_longunion.get());
        /// assert_eq!(b_longunion.get(), 0b1000000011111111101100111000111100001111100000111111000000111111_u64);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// 
        /// let a_longerunion = LongerUnion::new_with(0b10110011100011110000111110000011111100000011111110000000111111110000000011111111100000000011111111110000000000111111111110000000_u128);
        /// let b_longerunion = a_longerunion.rotate_right(32);
        /// println!("{:0128b} -> {:0128b}", a_longerunion.get(), b_longerunion.get());
        /// assert_eq!(b_longerunion.get(), 0b11110000000000111111111110000000101100111000111100001111100000111111000000111111100000001111111100000000111111111000000000111111_u128);
        /// ```
        /// 
        /// # Example 5 for SizeUnion for 64-bit CPUs
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// 
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     let a_sizeunion = SizeUnion::new_with(0b1011001110001111000011111000001111110000001111111000000011111111_usize);
        ///     let b_sizeunion = a_sizeunion.rotate_right(16);
        ///     println!("{:064b} -> {:064b}", a_sizeunion.get(), b_sizeunion.get());
        ///     assert_eq!(b_sizeunion.get(), 0b1000000011111111101100111000111100001111100000111111000000111111_usize);
        /// }
        /// ```
        /// 
        /// # Plagiarism in descryption
        /// It calls the method rotate_right() of implementation of the primitive
        /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
        /// `usize` directly. So, all the description of this method is mainly the
        /// same as that of the method rotate_right() of implementation of the
        /// primitive unsigned integer types except example codes. Confer to the
        /// descryptions that are linked to in the section _Reference_. This
        /// plagiarism is not made maliciously but is made for the reason of
        /// effectiveness and efficiency so that users may understand better and
        /// easily how to use this method with simiilarity to the method
        /// rotate_right() of implementation of the primitive unsigned integer
        /// types.
        #[inline] pub fn rotate_right(self, n: u32) -> Self { Self::new_with( self.get().rotate_right(n) ) }

        // fn count_ones(self) -> u32;
        /// Returns the number of ones in the binary representation of `self`.
        /// 
        /// # Output
        /// The total number of the bits that are set to be one in the binary
        /// representation of `self`.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// 
        /// let a_shortunion = ShortUnion::new_with(0b1011001110001111_u16);
        /// let ones = a_shortunion.count_ones();
        /// println!("The number of ones of {:016b} is {}.", a_shortunion.get(), ones);
        /// assert_eq!(ones, 10_u32);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// 
        /// let a_intunion = IntUnion::new_with(0b10110011100011110000111110000011_u32);
        /// let ones = a_intunion.count_ones();
        /// println!("The number of ones of {:032b} is {}.", a_intunion.get(), ones);
        /// assert_eq!(ones, 17_u32);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// 
        /// let a_longunion = LongUnion::new_with(0b1011001110001111000011111000001111110000001111111000000011111111_u64);
        /// let ones = a_longunion.count_ones();
        /// println!("The number of ones of {:064b} is {}.", a_longunion.get(), ones);
        /// assert_eq!(ones, 36_u32);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// 
        /// let a_longerunion = LongerUnion::new_with(0b10110011100011110000111110000011111100000011111110000000111111110000000011111111100000000011111111110000000000111111111110000000_u128);
        /// let ones = a_longerunion.count_ones();
        /// println!("The number of ones of {:0128b} is {}.", a_longerunion.get(), ones);
        /// assert_eq!(ones, 66_u32);
        /// ```
        /// 
        /// # Example 5 for SizeUnion
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// 
        /// let a_sizeunion = SizeUnion::new_with(0b10110011_usize);
        /// let ones = a_sizeunion.count_ones();
        /// println!("The number of ones of {:064b} is {}.", a_sizeunion.get(), ones);
        /// assert_eq!(ones, 5_u32);
        /// ```
        /// 
        /// # Plagiarism in descryption
        /// It calls the method count_ones() of implementation of the primitive
        /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
        /// `usize` directly. So, all the description of this method is mainly the
        /// same as that of the method count_ones() of implementation of the
        /// primitive unsigned integer types except example codes. Confer to the
        /// descryptions that are linked to in the section _Reference_. This
        /// plagiarism is not made maliciously but is made for the reason of
        /// effectiveness and efficiency so that users may understand better and
        /// easily how to use this method with simiilarity to the method
        /// count_ones() of implementation of the primitive unsigned integer
        /// types.
        #[inline] pub fn count_ones(self) -> u32        { self.get().count_ones() }

        // fn count_zeros(&self) -> u32
        /// Returns the number of zeros in the binary representation of `self`.
        /// 
        /// # Output
        /// The total number of the bits that are set to be zero in the binary
        /// representation of `self`.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// 
        /// let a_shortunion = ShortUnion::new_with(0b1011001110001111_u16);
        /// let zeros = a_shortunion.count_zeros();
        /// println!("The number of zeros of {:016b} is {}.", a_shortunion.get(), zeros);
        /// assert_eq!(zeros, 6_u32);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// 
        /// let a_intunion = IntUnion::new_with(0b10110011100011110000111110000011_u32);
        /// let zeros = a_intunion.count_zeros();
        /// println!("The number of zeros of {:032b} is {}.", a_intunion.get(), zeros);
        /// assert_eq!(zeros, 15_u32);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// 
        /// let a_longunion = LongUnion::new_with(0b1011001110001111000011111000001111110000001111111000000011111111_u64);
        /// let zeros = a_longunion.count_zeros();
        /// println!("The number of zeros of {:064b} is {}.", a_longunion.get(), zeros);
        /// assert_eq!(zeros, 28_u32);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// 
        /// let a_longerunion = LongerUnion::new_with(0b10110011100011110000111110000011111100000011111110000000111111110000000011111111100000000011111111110000000000111111111110000000_u128);
        /// let zeros = a_longerunion.count_zeros();
        /// println!("The number of zeros of {:0128b} is {}.", a_longerunion.get(), zeros);
        /// assert_eq!(zeros, 62_u32);
        /// ```
        /// 
        /// # Example 5 for SizeUnion for 64-bit CPUs
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// 
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     let a_sizeunion = SizeUnion::new_with(0b1011001110001111000011111000001111110000001111111000000011111111_usize);
        ///     let zeros = a_sizeunion.count_zeros();
        ///     println!("The number of zeros of {:064b} is {}.", a_sizeunion.get(), zeros);
        ///     assert_eq!(zeros, 28_u32);
        /// }
        /// ```
        /// 
        /// # Plagiarism in descryption
        /// It calls the method count_zeros() of implementation of the primitive
        /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
        /// `usize` directly. So, all the description of this method is mainly the
        /// same as that of the method count_zeros() of implementation of the
        /// primitive unsigned integer types except example codes. Confer to the
        /// descryptions that are linked to in the section _Reference_. This
        /// plagiarism is not made maliciously but is made for the reason of
        /// effectiveness and efficiency so that users may understand better and
        /// easily how to use this method with simiilarity to the method
        /// count_zeros() of implementation of the primitive unsigned integer
        /// types.
        #[inline] pub fn count_zeros(self) -> u32       { self.get().count_zeros() }

        // fn leading_ones(&self) -> u32
        /// Returns the number of leading ones
        /// in the binary representation of `self`.
        /// 
        /// # Output
        /// The total number of the leading bits that are set to be one
        /// in the binary representation of `self`.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// 
        /// let a_shortunion = ShortUnion::new_with(0b1100111000111110_u16);
        /// let ones = a_shortunion.leading_ones();
        /// println!("The number of leading ones of {:016b} is {}.", a_shortunion.get(), ones);
        /// assert_eq!(ones, 2_u32);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// 
        /// let a_intunion = IntUnion::new_with(0b11110000111110000011101100111000_u32);
        /// let ones = a_intunion.leading_ones();
        /// println!("The number of leading ones of {:032b} is {}.", a_intunion.get(), ones);
        /// assert_eq!(ones, 4_u32);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// 
        /// let a_longunion = LongUnion::new_with(0b1111100000111111000000111111100000001111111110110011100011110000_u64);
        /// let ones = a_longunion.leading_ones();
        /// println!("The number of leading ones of {:064b} is {}.", a_longunion.get(), ones);
        /// assert_eq!(ones, 5_u32);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// 
        /// let a_longerunion = LongerUnion::new_with(0b11111111111000000010110011100011110000111110000011111100000011111110000000111111110000000011111111100000000011111111110000000000_u128);
        /// let ones = a_longerunion.leading_ones();
        /// println!("The number of leading ones of {:0128b} is {}.", a_longerunion.get(), ones);
        /// assert_eq!(ones, 11_u32);
        /// ```
        /// 
        /// # Example 5 for SizeUnion for 64-bit CPUs
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// 
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     let a_sizeunion = SizeUnion::new_with(0b1111111000000011111111101100111000111100001111100000111111000000_usize);
        ///     let ones = a_sizeunion.leading_ones();
        ///     println!("The number of leading ones of {:064b} is {}.", a_sizeunion.get(), ones);
        ///     assert_eq!(ones, 7_u32);
        /// }
        /// ```
        /// 
        /// # Plagiarism in descryption
        /// It calls the method leading_ones() of implementation of the primitive
        /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
        /// `usize` directly. So, all the description of this method is mainly the
        /// same as that of the method leading_ones() of implementation of the
        /// primitive unsigned integer types except example codes. Confer to the
        /// descryptions that are linked to in the section _Reference_. This
        /// plagiarism is not made maliciously but is made for the reason of
        /// effectiveness and efficiency so that users may understand better and
        /// easily how to use this method with simiilarity to the method
        /// leading_ones() of implementation of the primitive unsigned integer
        /// types.
        #[inline] pub fn leading_ones(self) -> u32      { self.get().leading_ones() }
    
        // fn leading_zeros(&self) -> u32
        /// Returns the number of leading zeros
        /// in the binary representation of `self`.
        /// 
        /// # Output
        /// The total number of the leading bits that are set to be zero
        /// in the binary representation of `self`.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// 
        /// let a_shortunion = ShortUnion::new_with(0b0011100011111011_u16);
        /// let zeros = a_shortunion.leading_zeros();
        /// println!("The number of leading zeros of {:016b} is {}.", a_shortunion.get(), zeros);
        /// assert_eq!(zeros, 2_u32);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// 
        /// let a_intunion = IntUnion::new_with(0b01100111000111100001111100000111_u32);
        /// let zeros = a_intunion.leading_zeros();
        /// println!("The number of leading zeros of {:032b} is {}.", a_intunion.get(), zeros);
        /// assert_eq!(zeros, 1_u32);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// 
        /// let a_longunion = LongUnion::new_with(0b0000011111100000011111110000000111111111011001110001111000011111_u64);
        /// let zeros = a_longunion.leading_zeros();
        /// println!("The number of leading zeros of {:064b} is {}.", a_longunion.get(), zeros);
        /// assert_eq!(zeros, 5_u32);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// 
        /// let a_longerunion = LongerUnion::new_with(0b00000001111111100000000111111111000000000111111111100000000001111111111100000001011001110001111000011111000001111110000001111111_u128);
        /// let zeros = a_longerunion.leading_zeros();
        /// println!("The number of leading zeros of {:0128b} is {}.", a_longerunion.get(), zeros);
        /// assert_eq!(zeros, 7_u32);
        /// ```
        /// 
        /// # Example 5 for SizeUnion for 64-bit CPUs
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// 
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     let a_sizeunion = SizeUnion::new_with(0b0000111110000011111100000011111110000000111111111011001110001111_usize);
        ///     let zeros = a_sizeunion.leading_zeros();
        ///     println!("The number of leading zeros of {:064b} is {}.", a_sizeunion.get(), zeros);
        ///     assert_eq!(zeros, 4_u32);
        /// }
        /// ```
        /// 
        /// # Plagiarism in descryption
        /// It calls the method leading_zeros() of implementation of the primitive
        /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
        /// `usize` directly. So, all the description of this method is mainly the
        /// same as that of the method leading_zeros() of implementation of the
        /// primitive unsigned integer types except example codes. Confer to the
        /// descryptions that are linked to in the section _Reference_. This
        /// plagiarism is not made maliciously but is made for the reason of
        /// effectiveness and efficiency so that users may understand better and
        /// easily how to use this method with simiilarity to the method
        /// leading_zeros() of implementation of the primitive unsigned integer
        /// types.
        #[inline] pub fn leading_zeros(self) -> u32     { self.get().leading_zeros() }

        // fn trailing_ones(self) -> u32;
        /// Returns the number of trailing ones
        /// in the binary representation of `self`.
        /// 
        /// # Output
        /// The total number of the trailing bits that are set to be one
        /// in the binary representation of `self`.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// 
        /// let a_shortunion = ShortUnion::new_with(0b1111011001110001_u16);
        /// let ones = a_shortunion.trailing_ones();
        /// println!("The number of trailing ones of {:016b} is {}.", a_shortunion.get(), ones);
        /// assert_eq!(ones, 1_u32);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// 
        /// let a_intunion = IntUnion::new_with(0b00000111011001110001111000011111_u32);
        /// let ones = a_intunion.trailing_ones();
        /// println!("The number of trailing ones of {:032b} is {}.", a_intunion.get(), ones);
        /// assert_eq!(ones, 5_u32);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// 
        /// let a_longunion = LongUnion::new_with(0b1011001110001111000011111000001111110000001111111000000011111111_u64);
        /// let ones = a_longunion.trailing_ones();
        /// println!("The number of trailing ones of {:064b} is {}.", a_longunion.get(), ones);
        /// assert_eq!(ones, 8_u32);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// 
        /// let a_longerunion = LongerUnion::new_with(0b00000000001111111111100000001011001110001111000011111000001111110000001111111000000011111111000000001111111110000000001111111111_u128);
        /// let ones = a_longerunion.trailing_ones();
        /// println!("The number of trailing ones of {:0128b} is {}.", a_longerunion.get(), ones);
        /// assert_eq!(ones, 10_u32);
        /// ```
        /// 
        /// # Example 5 for SizeUnion
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// 
        /// let a_sizeunion = SizeUnion::new_with(0b10110011_usize);
        /// let ones = a_sizeunion.trailing_ones();
        /// println!("The number of trailing ones of {:064b} is {}.", a_sizeunion.get(), ones);
        /// assert_eq!(ones, 2_u32);
        /// ```
        /// 
        /// # Plagiarism in descryption
        /// It calls the method trailing_ones() of implementation of the primitive
        /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
        /// `usize` directly. So, all the description of this method is mainly the
        /// same as that of the method trailing_ones() of implementation of the
        /// primitive unsigned integer types except example codes. Confer to the
        /// descryptions that are linked to in the section _Reference_. This
        /// plagiarism is not made maliciously but is made for the reason of
        /// effectiveness and efficiency so that users may understand better and
        /// easily how to use this method with simiilarity to the method
        /// trailing_ones() of implementation of the primitive unsigned integer
        /// types.
        #[inline] pub fn trailing_ones(self) -> u32     { self.get().trailing_ones() }

        // fn trailing_zeros(self) -> u32;
        /// Returns the number of trailing zeros
        /// in the binary representation of `self`.
        /// 
        /// # Output
        /// The total number of the trailing bits that are set to be zero
        /// in the binary representation of `self`.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// 
        /// let a_shortunion = ShortUnion::new_with(0b1111101100111000_u16);
        /// let zeros = a_shortunion.trailing_zeros();
        /// println!("The number of trailing zeros of {:016b} is {}.", a_shortunion.get(), zeros);
        /// assert_eq!(zeros, 3_u32);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// 
        /// let a_intunion = IntUnion::new_with(0b11101100111000111100001111100000_u32);
        /// let zeros = a_intunion.trailing_zeros();
        /// println!("The number of trailing zeros of {:032b} is {}.", a_intunion.get(), zeros);
        /// assert_eq!(zeros, 5_u32);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// 
        /// let a_longunion = LongUnion::new_with(0b1111111000000011111111101100111000111100001111100000111111000000_u64);
        /// let zeros = a_longunion.trailing_zeros();
        /// println!("The number of trailing zeros of {:064b} is {}.", a_longunion.get(), zeros);
        /// assert_eq!(zeros, 6_u32);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// 
        /// let a_longerunion = LongerUnion::new_with(0b10110011100011110000111110000011111100000011111110000000111111110000000011111111100000000011111111110000000000111111111110000000_u128);
        /// let zeros = a_longerunion.trailing_zeros();
        /// println!("The number of trailing zeros of {:0128b} is {}.", a_longerunion.get(), zeros);
        /// assert_eq!(zeros, 7_u32);
        /// ```
        /// 
        /// # Example 5 for SizeUnion
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// 
        /// let a_sizeunion = SizeUnion::new_with(0b10110000_usize);
        /// let zeros = a_sizeunion.trailing_zeros();
        /// println!("The number of trailing zeros of {:064b} is {}.", a_sizeunion.get(), zeros);
        /// assert_eq!(zeros, 4_u32);
        /// ```
        /// 
        /// # Plagiarism in descryption
        /// It calls the method trailing_zeros() of implementation of the primitive
        /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
        /// `usize` directly. So, all the description of this method is mainly the
        /// same as that of the method trailing_zeros() of implementation of the
        /// primitive unsigned integer types except example codes. Confer to the
        /// descryptions that are linked to in the section _Reference_. This
        /// plagiarism is not made maliciously but is made for the reason of
        /// effectiveness and efficiency so that users may understand better and
        /// easily how to use this method with simiilarity to the method
        /// trailing_zeros() of implementation of the primitive unsigned integer
        /// types.
        #[inline] pub fn trailing_zeros(self) -> u32    { self.get().trailing_zeros() }

        // fn from_be(x: Self) -> Self;
        /// Converts an integer from big endian to the target’s endianness.
        /// 
        /// # Arguments
        /// `x` is an integer stored in the form of big endianness.
        /// 
        /// # Features
        /// On big endian machine, this function does nothing, which is a no-op.
        /// On little endian machine, the bytes are swapped.
        /// 
        /// # Output
        /// An integer conveted from big endian to the target’s endianness.
        /// 
        /// # Example 1 for ShortUnion for Little Endianness
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// 
        /// let a_shortunionbe = ShortUnion::new_with(0x1234_u16);
        /// let b_shortunionle = ShortUnion::from_be(a_shortunionbe);
        /// println!("{:04x} -> {:04x}", a_shortunionbe.get(), b_shortunionle.get());
        /// assert_eq!(b_shortunionle.get(), 0x3412_u16);
        /// ```
        /// 
        /// # Example 2 for IntUnion for Little Endianness
        /// ```
        /// use cryptocol::number::IntUnion;
        /// 
        /// let a_intunionbe = IntUnion::new_with(0x12345678_u32);
        /// let b_intunionle = IntUnion::from_be(a_intunionbe);
        /// println!("{:08x} -> {:08x}", a_intunionbe.get(), b_intunionle.get());
        /// assert_eq!(b_intunionle.get(), 0x78563412_u32);
        /// ```
        /// 
        /// # Example 3 for LongUnion for Little Endianness
        /// ```
        /// use cryptocol::number::LongUnion;
        /// 
        /// let a_longunionbe = LongUnion::new_with(0x123456789ABCDEF0_u64);
        /// let b_longunionle = LongUnion::from_be(a_longunionbe);
        /// println!("{:016x} -> {:016x}", a_longunionbe.get(), b_longunionle.get());
        /// assert_eq!(b_longunionle.get(), 0xf0debc9a78563412_u64);
        /// ```
        /// 
        /// # Example 4 for LongerUnion for Little Endianness
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// 
        /// let a_longerunionbe = LongerUnion::new_with(0x123456789ABCDEF0123456789ABCDEF0_u128);
        /// let b_longerunionle = LongerUnion::from_be(a_longerunionbe);
        /// println!("{:032x} -> {:032x}", a_longerunionbe.get(), b_longerunionle.get());
        /// assert_eq!(b_longerunionle.get(), 0xf0debc9a78563412f0debc9a78563412_u128);
        /// ```
        /// 
        /// # Example 5 for SizeUnion for 64-bit Little Endian CPUs
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// 
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     let a_sizeunionbe = SizeUnion::new_with(0x123456789ABCDEF0_usize);
        ///     let b_sizeunionle = SizeUnion::from_be(a_sizeunionbe);
        ///     println!("{:016x} -> {:016x}", a_sizeunionbe.get(), b_sizeunionle.get());
        ///     assert_eq!(b_sizeunionle.get(), 0xf0debc9a78563412_usize);
        /// }
        /// ```
        /// 
        /// # Plagiarism in descryption
        /// It calls the method from_be() of implementation of the primitive
        /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
        /// `usize` directly. So, all the description of this method is mainly the
        /// same as that of the method from_be() of implementation of the
        /// primitive unsigned integer types except example codes. Confer to the
        /// descryptions that are linked to in the section _Reference_. This
        /// plagiarism is not made maliciously but is made for the reason of
        /// effectiveness and efficiency so that users may understand better and
        /// easily how to use this method with simiilarity to the method
        /// from_be() of implementation of the primitive unsigned integer
        /// types.
        #[inline] pub fn from_be(x: Self) -> Self   { Self::new_with( <$f>::from_be(x.get()) ) }

        // fn from_le(x: Self) -> Self;
        /// Converts an integer from little endian to the target’s endianness.
        /// 
        /// # Arguments
        /// `x` is an integer stored in the form of big endianness.
        /// 
        /// # Features
        /// On little endian machine, this function does nothing, which is a no-op.
        /// On big endian machine, the bytes are swapped.
        /// 
        /// # Output
        /// An integer conveted from little endian to the target’s endianness.
        /// 
        /// # Example 1 for ShortUnion for Little Endianness
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// 
        /// let a_shortunionle = ShortUnion::new_with(0x1234_u16);
        /// let b_shortunionle = ShortUnion::from_le(a_shortunionle);
        /// println!("{:04x} -> {:04x}", a_shortunionle.get(), b_shortunionle.get());
        /// assert_eq!(b_shortunionle.get(), 0x1234_u16);
        /// ```
        /// 
        /// # Example 2 for IntUnion for Little Endianness
        /// ```
        /// use cryptocol::number::IntUnion;
        /// 
        /// let a_intunionle = IntUnion::new_with(0x12345678_u32);
        /// let b_intunionle = IntUnion::from_le(a_intunionle);
        /// println!("{:08x} -> {:08x}", a_intunionle.get(), b_intunionle.get());
        /// assert_eq!(b_intunionle.get(), 0x12345678_u32);
        /// ```
        /// 
        /// # Example 3 for LongUnion for Little Endianness
        /// ```
        /// use cryptocol::number::LongUnion;
        /// 
        /// let a_longunionle = LongUnion::new_with(0x123456789ABCDEF0_u64);
        /// let b_longunionle = LongUnion::from_le(a_longunionle);
        /// println!("{:016x} -> {:016x}", a_longunionle.get(), b_longunionle.get());
        /// assert_eq!(b_longunionle.get(), 0x123456789ABCDEF0_u64);
        /// ```
        /// 
        /// # Example 4 for LongerUnion for Little Endianness
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// 
        /// let a_longerunionle = LongerUnion::new_with(0x123456789ABCDEF0123456789ABCDEF0_u128);
        /// let b_longerunionle = LongerUnion::from_le(a_longerunionle);
        /// println!("{:032x} -> {:032x}", a_longerunionle.get(), b_longerunionle.get());
        /// assert_eq!(b_longerunionle.get(), 0x123456789ABCDEF0123456789ABCDEF0_u128);
        /// ```
        /// 
        /// # Example 5 for SizeUnion for Little Endianness
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// 
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     let a_sizeunionle = SizeUnion::new_with(0x123456789ABCDEF0_usize);
        ///     let b_sizeunionle = SizeUnion::from_le(a_sizeunionle);
        ///     println!("{:016x} -> {:016x}", a_sizeunionle.get(), b_sizeunionle.get());
        ///     assert_eq!(b_sizeunionle.get(), 0x123456789ABCDEF0_usize);
        /// }
        /// ```
        /// 
        /// # Plagiarism in descryption
        /// It calls the method from_le() of implementation of the primitive
        /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
        /// `usize` directly. So, all the description of this method is mainly the
        /// same as that of the method from_le() of implementation of the
        /// primitive unsigned integer types except example codes. Confer to the
        /// descryptions that are linked to in the section _Reference_. This
        /// plagiarism is not made maliciously but is made for the reason of
        /// effectiveness and efficiency so that users may understand better and
        /// easily how to use this method with simiilarity to the method
        /// from_le() of implementation of the primitive unsigned integer
        /// types.
        #[inline] pub fn from_le(x: Self) -> Self   { Self::new_with( <$f>::from_le(x.get()) ) }

        // fn to_be(self) -> Self;
        /// Converts `self` to big endian from the target’s endianness.
        /// 
        /// # Features
        /// On big endian machine, this method does nothing, which is a no-op.
        /// On little endian the bytes are swapped.
        /// 
        /// # Output
        /// An integer converted to big endian from the target’s endianness
        /// 
        /// # Example 1 for ShortUnion for Little Endianness
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// 
        /// let a_shortunionle = ShortUnion::new_with(0x1234_u16);
        /// let b_shortunionbe = ShortUnion::to_be(a_shortunionle);
        /// println!("{:04x} -> {:04x}", a_shortunionle.get(), b_shortunionbe.get());
        /// assert_eq!(b_shortunionbe.get(), 0x3412_u16);
        /// ```
        /// 
        /// # Example 2 for IntUnion for Little Endianness
        /// ```
        /// use cryptocol::number::IntUnion;
        /// 
        /// let a_intunionle = IntUnion::new_with(0x12345678_u32);
        /// let b_intunionbe = IntUnion::to_be(a_intunionle);
        /// println!("{:08x} -> {:08x}", a_intunionle.get(), b_intunionbe.get());
        /// assert_eq!(b_intunionbe.get(), 0x78563412_u32);
        /// ```
        /// 
        /// # Example 3 for LongUnion for Little Endianness
        /// ```
        /// use cryptocol::number::LongUnion;
        /// 
        /// let a_longunionle = LongUnion::new_with(0x123456789ABCDEF0_u64);
        /// let b_longunionbe = LongUnion::to_be(a_longunionle);
        /// println!("{:016x} -> {:016x}", a_longunionle.get(), b_longunionbe.get());
        /// assert_eq!(b_longunionbe.get(), 0xf0debc9a78563412_u64);
        /// ```
        /// 
        /// # Example 4 for LongerUnion for Little Endianness
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// 
        /// let a_longerunionle = LongerUnion::new_with(0x123456789ABCDEF0123456789ABCDEF0_u128);
        /// let b_longerunionbe = LongerUnion::to_be(a_longerunionle);
        /// println!("{:032x} -> {:032x}", a_longerunionle.get(), b_longerunionbe.get());
        /// assert_eq!(b_longerunionbe.get(), 0xf0debc9a78563412f0debc9a78563412_u128);
        /// ```
        /// 
        /// # Example 4 for SizeUnion for 64-bit Little Endian CPUs
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// 
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     let a_sizeunionle = SizeUnion::new_with(0x123456789ABCDEF0_usize);
        ///     let b_sizeunionbe = SizeUnion::to_be(a_sizeunionle);
        ///     println!("{:016x} -> {:016x}", a_sizeunionle.get(), b_sizeunionbe.get());
        ///     assert_eq!(b_sizeunionbe.get(), 0xf0debc9a78563412_usize);
        /// }
        /// ```
        /// 
        /// # Plagiarism in descryption
        /// It calls the method to_be() of implementation of the primitive
        /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
        /// `usize` directly. So, all the description of this method is mainly the
        /// same as that of the method to_be() of implementation of the
        /// primitive unsigned integer types except example codes. Confer to the
        /// descryptions that are linked to in the section _Reference_. This
        /// plagiarism is not made maliciously but is made for the reason of
        /// effectiveness and efficiency so that users may understand better and
        /// easily how to use this method with simiilarity to the method
        /// to_be() of implementation of the primitive unsigned integer
        /// types.
        #[inline] pub fn to_be(self) -> Self        { Self::new_with( self.get().to_be() ) }

        // fn to_le(self) -> Self;
        /// Converts self to little endian from the target’s endianness.
        /// 
        /// # Features
        /// On little endian machine, this method does nothing, which is a no-op.
        /// On big endian the bytes are swapped.
        /// 
        /// # Output
        /// An integer converted to little endian from the target’s endianness
        /// 
        /// # Example 1 for ShortUnion for Little Endianness
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// 
        /// let a_shortunionle = ShortUnion::new_with(0x1234_u16);
        /// let b_shortunionle = ShortUnion::to_le(a_shortunionle);
        /// println!("{:04x} -> {:04x}", a_shortunionle.get(), b_shortunionle.get());
        /// assert_eq!(b_shortunionle.get(), 0x1234_u16);
        /// ```
        /// 
        /// # Example 2 for IntUnion for Little Endianness
        /// ```
        /// use cryptocol::number::IntUnion;
        /// 
        /// let a_intunionle = IntUnion::new_with(0x12345678_u32);
        /// let b_intunionle = IntUnion::to_le(a_intunionle);
        /// println!("{:08x} -> {:08x}", a_intunionle.get(), b_intunionle.get());
        /// assert_eq!(b_intunionle.get(), 0x12345678_u32);
        /// ```
        /// 
        /// # Example 3 for LongUnion for Little Endianness
        /// ```
        /// use cryptocol::number::LongUnion;
        /// 
        /// let a_longunionle = LongUnion::new_with(0x123456789ABCDEF0_u64);
        /// let b_longunionle = LongUnion::to_le(a_longunionle);
        /// println!("{:016x} -> {:016x}", a_longunionle.get(), b_longunionle.get());
        /// assert_eq!(b_longunionle.get(), 0x123456789ABCDEF0_u64);
        /// ```
        /// 
        /// # Example 4 for LongerUnion for Little Endianness
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// 
        /// let a_longerunionle = LongerUnion::new_with(0x123456789ABCDEF0123456789ABCDEF0_u128);
        /// let b_longerunionle = LongerUnion::to_le(a_longerunionle);
        /// println!("{:032x} -> {:032x}", a_longerunionle.get(), b_longerunionle.get());
        /// assert_eq!(b_longerunionle.get(), 0x123456789ABCDEF0123456789ABCDEF0_u128);
        /// ```
        /// 
        /// # Example 5 for SizeUnion for 64-bit Little Endian CPUs
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// 
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     let a_sizeunionle = SizeUnion::new_with(0x123456789ABCDEF0_usize);
        ///     let b_sizeunionle = SizeUnion::to_le(a_sizeunionle);
        ///     println!("{:016x} -> {:016x}", a_sizeunionle.get(), b_sizeunionle.get());
        ///     assert_eq!(b_sizeunionle.get(), 0x123456789ABCDEF0_usize);
        /// }
        /// ```
        /// 
        /// # Plagiarism in descryption
        /// It calls the method to_le() of implementation of the primitive
        /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
        /// `usize` directly. So, all the description of this method is mainly the
        /// same as that of the method to_le() of implementation of the
        /// primitive unsigned integer types except example codes. Confer to the
        /// descryptions that are linked to in the section _Reference_. This
        /// plagiarism is not made maliciously but is made for the reason of
        /// effectiveness and efficiency so that users may understand better and
        /// easily how to use this method with simiilarity to the method
        /// to_le() of implementation of the primitive unsigned integer
        /// types.
        #[inline] pub fn to_le(self) -> Self        { Self::new_with( self.get().to_le() ) }

        // fn swap_bytes(self) -> Self;
        /// Reverses the byte order of the integer.
        /// 
        /// # Features
        /// The least significant byte becomes the most significant byte, second
        /// least-significant byte becomes second most-significant byte, etc.
        /// 
        /// # Output
        /// An integer whose byte order was reversed to opposite endian.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// 
        /// let a_shortunion = ShortUnion::new_with(0x1234_u16);
        /// let b_shortunion = ShortUnion::swap_bytes(a_shortunion);
        /// println!("{:04x} -> {:04x}", a_shortunion.get(), b_shortunion.get());
        /// assert_eq!(b_shortunion.get(), 0x3412_u16);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// 
        /// let a_intunion = IntUnion::new_with(0x12345678_u32);
        /// let b_intunion = IntUnion::swap_bytes(a_intunion);
        /// println!("{:08x} -> {:08x}", a_intunion.get(), b_intunion.get());
        /// assert_eq!(b_intunion.get(), 0x78563412_u32);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// 
        /// let a_longunion = LongUnion::new_with(0x123456789ABCDEF0_u64);
        /// let b_longunion = LongUnion::swap_bytes(a_longunion);
        /// println!("{:016x} -> {:016x}", a_longunion.get(), b_longunion.get());
        /// assert_eq!(b_longunion.get(), 0xf0debc9a78563412_u64);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// 
        /// let a_longerunion = LongerUnion::new_with(0x123456789ABCDEF0123456789ABCDEF0_u128);
        /// let b_longerunion = LongerUnion::swap_bytes(a_longerunion);
        /// println!("{:032x} -> {:032x}", a_longerunion.get(), b_longerunion.get());
        /// assert_eq!(b_longerunion.get(), 0xf0debc9a78563412f0debc9a78563412_u128);
        /// ```
        /// 
        /// # Example 5 for SizeUnion for 64-bit CPUs
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// 
        /// #[cfg(target_pointer_width = "64")]
        /// {
        ///     let a_sizeunion = SizeUnion::new_with(0x123456789ABCDEF0_usize);
        ///     let b_sizeunion = SizeUnion::swap_bytes(a_sizeunion);
        ///     println!("{:016x} -> {:016x}", a_sizeunion.get(), b_sizeunion.get());
        ///     assert_eq!(b_sizeunion.get(), 0xf0debc9a78563412_usize);
        /// }
        /// ```
        /// 
        /// # Plagiarism in descryption
        /// It calls the method swap_bytes() of implementation of the primitive
        /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
        /// `usize` directly. So, all the description of this method is mainly the
        /// same as that of the method swap_bytes() of implementation of the
        /// primitive unsigned integer types except example codes. Confer to the
        /// descryptions that are linked to in the section _Reference_. This
        /// plagiarism is not made maliciously but is made for the reason of
        /// effectiveness and efficiency so that users may understand better and
        /// easily how to use this method with simiilarity to the method
        /// swap_bytes() of implementation of the primitive unsigned integer
        /// types.
        #[inline] pub fn swap_bytes(self) -> Self   { Self::new_with( self.get().swap_bytes() ) }

        // fn is_power_of_two(self) -> bool;
        /// Returns `true` if and only if `self` == `2^k` for some `k`.
        /// 
        /// # Output
        /// If and only if `self` == `2^k` for some `k`, `true` is returned.
        /// Otherwise, it returns `false`.
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// 
        /// let a_shortunion = ShortUnion::new_with(32768_u16);
        /// let a_two = a_shortunion.is_power_of_two();
        /// println!("{} is {}power of two..", a_shortunion, if a_two {""} else {"not "});
        /// assert!(a_two);
        /// 
        /// let b_shortunion = ShortUnion::new_with(65432_u16);
        /// let b_two = b_shortunion.is_power_of_two();
        /// println!("{} is {}power of two..", b_shortunion, if b_two {""} else {"not "});
        /// assert!(!b_two);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// 
        /// let a_intunion = IntUnion::new_with(2147483648_u32);
        /// let a_two = a_intunion.is_power_of_two();
        /// println!("{} is {}power of two..", a_intunion, if a_two {""} else {"not "});
        /// assert!(a_two);
        /// 
        /// let b_intunion = IntUnion::new_with(876543210_u32);
        /// let b_two = b_intunion.is_power_of_two();
        /// println!("{} is {}power of two..", b_intunion, if b_two {""} else {"not "});
        /// assert!(!b_two);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// 
        /// let a_longunion = LongUnion::new_with(9223372036854775808_u64);
        /// let a_two = a_longunion.is_power_of_two();
        /// println!("{} is {}power of two..", a_longunion, if a_two {""} else {"not "});
        /// assert!(a_two);
        /// 
        /// let b_longunion = LongUnion::new_with(2468135791234567892_u64);
        /// let b_two = b_longunion.is_power_of_two();
        /// println!("{} is {}power of two..", b_longunion, if b_two {""} else {"not "});
        /// assert!(!b_two);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// 
        /// let a_longerunion = LongerUnion::new_with(170141183460469231731687303715884105728_u128);
        /// let a_two = a_longerunion.is_power_of_two();
        /// println!("{} is {}power of two..", a_longerunion, if a_two {""} else {"not "});
        /// assert!(a_two);
        /// 
        /// let b_longerunion = LongerUnion::new_with(200000000000000000000000000000000000000_u128);
        /// let b_two = b_longerunion.is_power_of_two();
        /// println!("{} is {}power of two..", b_longerunion, if b_two {""} else {"not "});
        /// assert!(!b_two);
        /// ```
        /// 
        /// # Example 5 for SizeUnion
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// 
        /// let a_sizeunion = SizeUnion::new_with(128_usize);
        /// let a_two = a_sizeunion.is_power_of_two();
        /// println!("{} is {}power of two..", a_sizeunion, if a_two {""} else {"not "});
        /// assert!(a_two);
        /// 
        /// let b_sizeunion = SizeUnion::new_with(200_usize);
        /// let b_two = b_sizeunion.is_power_of_two();
        /// println!("{} is {}power of two..", b_sizeunion, if b_two {""} else {"not "});
        /// assert!(!b_two);
        /// ```
        /// 
        /// # Plagiarism in descryption
        /// It calls the method is_power_of_two() of implementation of the primitive
        /// unsigned integer types such as`u8`, `u16`, `u32`, `u64`, `u128` and
        /// `usize` directly. So, all the description of this method is mainly the
        /// same as that of the method is_power_of_two() of implementation of the
        /// primitive unsigned integer types except example codes. Confer to the
        /// descryptions that are linked to in the section _Reference_. This
        /// plagiarism is not made maliciously but is made for the reason of
        /// effectiveness and efficiency so that users may understand better and
        /// easily how to use this method with simiilarity to the method
        /// is_power_of_two() of implementation of the primitive unsigned integer
        /// types.
        #[inline] pub fn is_power_of_two(self) -> bool    { self.get().is_power_of_two() }

        // fn next_power_of_two(self) -> Self;
        /// Returns the smallest power of two greater than or equal to `self`.
        /// 
        /// # Output
        /// The smallest power of two greater than or equal to `self`
        /// 
        /// # Features
        /// When return value overflows i.e., `self > (1 << (N-1) for type uN`,
        /// it panics in debug mode and the return value is wrapped to 0 in release
        /// mode (the only situation in which method can return 0).
        /// 
        /// # Example 1 for ShortUnion
        /// ```
        /// use cryptocol::number::ShortUnion;
        /// 
        /// // Example for ShortUnion
        /// let a_shortunion = ShortUnion::new_with(400_u16);
        /// let b_shortunion = a_shortunion.next_power_of_two();
        /// println!("{} => {}", a_shortunion.get(), b_shortunion.get());
        /// assert_eq!(b_shortunion.get(), 512_u16);
        /// ```
        /// 
        /// # Example 2 for IntUnion
        /// ```
        /// use cryptocol::number::IntUnion;
        /// 
        /// let a_intunion = IntUnion::new_with(400000_u32);
        /// let b_intunion = a_intunion.next_power_of_two();
        /// println!("{} => {}", a_intunion.get(), b_intunion.get());
        /// assert_eq!(b_intunion.get(), 524288_u32);
        /// ```
        /// 
        /// # Example 3 for LongUnion
        /// ```
        /// use cryptocol::number::LongUnion;
        /// 
        /// let a_longunion = LongUnion::new_with(400000000000_u64);
        /// let b_longunion = a_longunion.next_power_of_two();
        /// println!("{} => {}", a_longunion.get(), b_longunion.get());
        /// assert_eq!(b_longunion.get(), 549755813888_u64);
        /// ```
        /// 
        /// # Example 4 for LongerUnion
        /// ```
        /// use cryptocol::number::LongerUnion;
        /// 
        /// let a_longerunion = LongerUnion::new_with(4000000000000000000000000000_u128);
        /// let b_longerunion = a_longerunion.next_power_of_two();
        /// println!("{} => {}", a_longerunion.get(), b_longerunion.get());
        /// assert_eq!(b_longerunion.get(), 4951760157141521099596496896_u128);
        /// ```
        /// 
        /// # Example 5 for SizeUnion
        /// ```
        /// use cryptocol::number::SizeUnion;
        /// 
        /// let a_sizeunion = SizeUnion::new_with(4000000000000000_usize);
        /// let b_sizeunion = a_sizeunion.next_power_of_two();
        /// println!("{} => {}", a_sizeunion.get(), b_sizeunion.get());
        /// assert_eq!(b_sizeunion.get(), 4503599627370496_usize);
        /// ```
        /// 
        /// # Plagiarism in descryption
        /// It calls the method next_power_of_two() of implementation of the
        /// primitive unsigned integer types such as`u8`, `u16`, `u32`, `u64`,
        /// `u128` and `usize` directly. So, all the description of this method
        /// is mainly the same as that of the method next_power_of_two() of
        /// implementation of the primitive unsigned integer types except example
        /// codes. Confer to the descryptions that are linked to in the section
        /// _Reference_. This plagiarism is not made maliciously but is made for
        /// the reason of effectiveness and efficiency so that users may understand
        /// better and easily how to use this method with simiilarity to the method
        /// next_power_of_two() of implementation of the primitive unsigned integer
        /// types.
        #[inline] pub fn next_power_of_two(self) -> Self  { Self::new_with( self.get().next_power_of_two() ) }
    }
}
pub(super) use integer_union_methods;

//================
macro_rules! operators_for_integer_unions_impl {
    ($f:ty) => {
        impl Add for $f
        {
            type Output = Self;

            /// The addition operator +.
            /// 
            /// # Example 1 for ShortUinion
            /// ```
            /// 
            /// ```
            /// 
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
            /// This method tests for `self`` and other values to be equal,
            /// and is used by operator `==`.
            /// [Read more](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html#tymethod.eq)
            /// 
            /// # Example 1
            /// ```
            /// use cryptocol::number::IntUnion;
            /// let a_intunion = IntUnion::new_with_signed(454688546_i32);
            /// let b_intunion = IntUnion::new_with_signed(454688546_i32);
            /// if a_intunion == b_intunion
            ///     { println!("Equal"); }
            /// else
            ///     { println!("Different"); }
            /// assert!(a_intunion == b_intunion);
            /// ````
            /// 
            /// # Example 2
            /// ```
            /// use cryptocol::number::IntUnion;
            /// let a_intunion = IntUnion::new_with_signed(454688546_i32);
            /// let b_intunion = IntUnion::new_with_signed(-454688546_i32);
            /// if a_intunion == b_intunion
            ///     { println!("Equal"); }
            /// else
            ///     { println!("Different"); }
            /// assert!(a_intunion != b_intunion);
            /// ````
            /// 
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
                    { Some(Ordering::Greater) }
                else if self.get() < other.get()
                    { Some(Ordering::Less) }
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
            /// The right shift assignment operator >>=.
            /// 
            /// # Example 1 for ShortUnion
            /// ```
            /// let mut a_shortunion = ShortUnion::new_with(1234_u16);
            /// let right = 
            /// ```
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
            /// let a_shortunion = ShortUnion::new_with(60521_u16);
            /// println!("{}", a_shortunion);
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

        // /// Make a `SmallUInt`-type object to have a random value.
        // /// [Read more in detail](trait@SmallUInt#tymethod.random)
        // #[cfg(target_pointer_width = "8")]
        // #[inline] fn random() -> Self   { Self::new_with(OsRng.next_u32() as usize) }

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
        
        // /// Make a `SmallUInt`-type object to have a random value.
        // /// [Read more in detail](trait@SmallUInt#tymethod.random)
        // #[cfg(target_pointer_width = "128")]
        // fn random() -> Self
        // {
        //     let mut common = SizeUnion::new();
        //     common.set_ulong_(0, OsRng.next_u64());
        //     common.set_ulong_(1, OsRng.next_u64());
        //     common
        // }
    };
}
*/
