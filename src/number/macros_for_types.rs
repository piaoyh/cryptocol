// Copyright 2023, 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

// #![warn(missing_docs)]
// #![warn(rustdoc::missing_doc_code_examples)]
#![allow(non_camel_case_types)]
#![allow(missing_docs)]
#![allow(rustdoc::missing_doc_code_examples)]
//! The module that contains macros for defining big unsigned integer data types. 

/// The macro that defines the types `U256`, `U512`, `U1024`, `U2048`, `U3072`,
/// `U4096`, `U5120`, `U6144`, `U7168`, `U8192`, and `U16384`. 
/// 
/// The types `U256`, `U512`, `U1024`, `U2048`, `U3072`, `U4096`, `U5120`,
/// `U6144`, `U7168`, `U8192`, and `U16384` are 256-bit, 512-bit, 1024-bit,
/// U2048-bit, 3072-bit, 4096-bit, 5120-bit, 6144-bit, 7168-bit, 8192-bit,
/// and 16384-bit big unsigned integer types, respectively.
/// They are defined based on `u8`, `u16`, `u32`, `u64` and `u128` according
/// to the given parameter.
/// 
/// They are defined based on `u8`, `u16`, `u32`, `u64` and `u128` according
/// to the given parameter.
/// 
/// So, if you give `u128` as a parameter, it will define `U256`, `U512`,
/// `U1024`, `U2048`, `U3072`, `U4096`, `U5120`, `U6144`, `U7168`, `U8192`,
/// and `U16384`, based on `u128`. That is, U256, U256_, U1024, and U1024 will
/// be defined to be `BigUInt<u128, 2>` and `BigUInt<u128, 8>`, respectively,
/// for example.
/// 
/// Furthermore, `UU32`, `UU64`, `UU128`, `UU256`, `UU384`, `UU512`, `UU640`,
/// `UU768`, `UU896`, `UU1024`, and `UU2048` will be also defined to be `U256`,
/// `U512`, `U1024`, `U2048`, `U3072`, `U4096`, `U5120`, `U6144`, `U7168`,
/// `U8192`, and `U16384`, respectively. `UU32` is 32-byte big unsigned integer
/// type, and `UU64` is 64-byte big unsigned integer type, and `UU128` is
/// 128-byte big unsigned integer type, and so on.
/// That is, `UU32` is a synonym of `U256`, and `UU64` is a synonym of `U512`,
/// and so on.
/// 
/// If you define big unsigned integer types with `define_utypes_with!(u128)`,
/// `U1024` and `UU128` will be `BigUInt<u128, 8>`.
/// If you define big unsigned integer types with `define_utypes_with!(u64)`,
/// `U1024` and `UU128` will be `BigUInt<u64, 16>`.
/// If you define big unsigned integer types with `define_utypes_with!(u32)`,
/// `U1024` and `UU128` will be `BigUInt<u32, 32>`.
/// They are all the same sized, but their insides or structures are all
/// different from one another.
/// 
/// According to the performance test carried out on Samsung Laptop with Intel
/// I5 core and 32 GB RAM under Linux Mint 21.1 (Vera) on the date 5th of July,
/// 2023, the big unsigned integer type based on u128 showed the best
/// performance. So, you are highly recommended to use the big unsigned integer
/// types based on u128.
/// 
/// However, the performance results may be different under different conditions
/// such as on other machines, under other operating systems, etc. It means that
/// the big unsigned integer types based on u128 may or may not show the best
/// performance under certain conditions. You can test the performance by
/// yourself and find the best parameter for your system. The performance test
/// code used for test is as follows:
///
/// # Performance Test Code
/// ```
/// use std::time::SystemTime;
/// use std::fmt::{ Display, Debug };
/// use std::ops::*;
/// use cryptocol::number::*;
/// 
/// fn main()
/// {
///     let num_txt = "1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_";
///     let a_128 = U1024_with_u128::from_string(num_txt).unwrap();
///     let a_64 = U1024_with_u64::from_string(num_txt).unwrap();
///     let a_32 = U1024_with_u32::from_string(num_txt).unwrap();
///     let a_16 = U1024_with_u16::from_string(num_txt).unwrap();
///     let a_8 = U1024_with_u8::from_string(num_txt).unwrap();
/// 
///     calcAdd(&a_128);
///     calcAdd(&a_64);
///     calcAdd(&a_32);
///     calcAdd(&a_16);
///     calcAdd(&a_8);
///
///     calcMul(&a_128);
///     calcMul(&a_64);
///     calcMul(&a_32);
///     calcMul(&a_16);
///     calcMul(&a_8);
/// }
/// 
/// fn calcAdd<T, const N: usize>(a: &BigUInt<T, N>)
/// where T: SmallUInt + Display + Debug + ToString
///         + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
///         + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
///         + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
///         + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
///         + BitXor<Output=T> + BitXorAssign + Not<Output=T>
///         + PartialEq + PartialOrd
/// {
///     let mut sum = BigUInt::<T, N>::zero();
///     let now = SystemTime::now();
///     for _ in 0..1000
///     {
///         sum += *a;
///     }
///     let elapsed = now.elapsed().unwrap();
///     println!("{}-bit addition operation takes\t{}", T::size_in_bits(), elapsed.as_nanos());
/// }
///
/// fn calcMul<T, const N: usize>(a: &BigUInt<T, N>)
/// where T: SmallUInt + Display + Debug + ToString
///         + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
///         + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
///         + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
///         + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
///         + BitXor<Output=T> + BitXorAssign + Not<Output=T>
///         + PartialEq + PartialOrd
/// {
///     let mut sum = BigUInt::<T, N>::one();
///     let now = SystemTime::now();
///     for _ in 0..1000
///     {
///         sum *= *a;
///     }
///     let elapsed = now.elapsed().unwrap();
///     println!("{}-bit multiplication operation takes\t{}", T::size_in_bits(), elapsed.as_nanos());
/// }
/// ```
/// The following examples show how to use the macro `define_utypes_with!(...)`.
/// 
/// # Examples
/// ```
/// use cryptocol::define_utypes_with;
/// define_utypes_with!(u128);
/// let a = U256::from_string("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
/// let b = a << 1;
/// println!("b = {}", b);
/// assert_eq!(b, UU32::from_string("24691342469135780246913578024691357802469135780246913578024691357802469135780").unwrap());
/// ```
/// Here, `u128` is used as a base type in the macro `define_utypes_with!(u128)`.
/// So, `U256` and `UU32` are both `BigUInt<u128, 2>`, but you can choose a
/// different parameter such as `u64`. Then, `U256` and `UU32` are both
/// `BigUInt<u64, 4>`.
/// 
/// The following example shows the different case that `u64`
/// is used as a base type in the macro `define_utypes_with!(u64)`.
/// ```
/// use cryptocol::define_utypes_with;
/// define_utypes_with!(u64);
/// let a = U256::from_string("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
/// let b = a << 1;
/// println!("b = {}", b);
/// assert_eq!(b, UU32::from_string("24691342469135780246913578024691357802469135780246913578024691357802469135780").unwrap());
/// ```
#[macro_export]
macro_rules! define_utypes_with
{
    (u128) =>
        {
            use cryptocol::define_utypes_with_u128;
            define_utypes_with_u128!();
        };
    (u64) =>
        {
            use cryptocol::define_utypes_with_u64;
            define_utypes_with_u64!();
        };
    (u32) =>
        {
            use cryptocol::define_utypes_with_u32;
            define_utypes_with_u32!();
        };
    (u16) =>
        {
            use cryptocol::define_utypes_with_u16;
            define_utypes_with_u16!();
        };
    (u8) =>
        {
            use cryptocol::define_utypes_with_u8;
            define_utypes_with_u8!();
        };
}


/// The macro that defines the types `U256`, `U512`, `U1024`, `U2048`,
/// `U3072`, `U4096`, `U5120`, `U6144`, `U7168`, `U8192`, and `U16384`
/// __based on `u128`__.
/// 
/// The types `U256`, `U512`, `U1024`, `U2048`, `U3072`, `U4096`, `U5120`,
/// `U6144`, `U7168`, `U8192`, and `U16384` are 256-bit, 512-bit, 1024-bit,
/// 2048-bit, 3072-bit, 4096-bit, 5120-bit, 6144-bit, 7168-bit, 8192-bit,
/// and 16384-bit big unsigned integer types, respectively.
/// They are all defined __based on `u128`__.
/// 
/// So, it will define `U256`, `U512`, `U1024`, `U2048`, `U3072`, `U4096`,
/// `U5120`, `U6144`, `U7168`, `U8192`, and  `U16384` __based on `u128`__.
/// That is, `U256` and `U1024` will be defined to be `BigUInt<u128, 2>`
/// and `BigUInt<u128, 8>`, respectively, for example.
/// 
/// Furthermore, `UU32`, `UU64`, `UU128`, `UU256`, `UU384`, `UU512`, `UU640`,
/// `UU768`, `UU896`, `UU1024`, and `UU2048` will be also defined to be `U256`,
/// `U512`, `U1024`, `U2048`, `U3072`, `U4096`, `U5120`, `U6144`, `U7168`,
/// `U8192`, and `U16384`, respectively.
/// `UU32` is 32-byte big unsigned integer type, and
/// `UU64` is 64-byte big unsigned integer type, and
/// `UU128` is 128-byte big unsigned integer type and so on.
/// That is, `UU32` is a synonym of `U256`, and
/// `UU64` is a synonym of `U512`, and so on.
/// 
/// The following examples show how to use the macro `define_utypes_with_u128!()`.
/// 
/// # Example
/// ```
/// use cryptocol::define_utypes_with_u128;
/// define_utypes_with_u128!();
/// let a = U256::from_string("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
/// let b = a << 1;
/// println!("b = {}", b);
/// assert_eq!(b, UU32::from_string("24691342469135780246913578024691357802469135780246913578024691357802469135780").unwrap());
/// ```
/// Here, `u128` is used as a base type in the macro `define_utypes_with_u128!()`.
/// So, `U256` and `UU32` are both `BigUInt<u128, 2>`. This macro
/// `define_utypes_with_u128!()` is virtually the same as `define_utypes_with!(u128)`.
/// Actually, this macro `define_utypes_with_u128!()` is used in the macro
/// `define_utypes_with!(...)`.
#[macro_export]
macro_rules! define_utypes_with_u128
{
    () =>
    {
        use cryptocol::number::{ U256_with_u128, U512_with_u128, U1024_with_u128, U2048_with_u128,
                                U3072_with_u128, U4096_with_u128, U5120_with_u128, U6144_with_u128,
                                U7168_with_u128, U8192_with_u128, U16384_with_u128 };

        /// 256-bit unsigned integer, Synonym of `U256_with_u128`
        pub type U256 = U256_with_u128;

        /// 512-bit unsigned integer for 128-bit machines, Synonym of `U512_with_u128`
        pub type U512 = U512_with_u128;

        /// 1024-bit unsigned integer for 128-bit machines, Synonym of `U1024_with_u128`
        pub type U1024 = U1024_with_u128;

        /// 2048-bit unsigned integer for 128-bit machines, Synonym of `U2048_with_u128`
        pub type U2048 = U2048_with_u128;

        /// 3072-bit unsigned integer for 128-bit machines, Synonym of `U3072_with_u128`
        pub type U3072 = U3072_with_u128;

        /// 4096-bit unsigned integer for 128-bit machines, Synonym of `U4096_with_u128`
        pub type U4096 = U4096_with_u128;

        /// 5120-bit unsigned integer for 128-bit machines, Synonym of `U5120_with_u128`
        pub type U5120 = U5120_with_u128;

        /// 6144-bit unsigned integer for 128-bit machines, Synonym of `U6144_with_u128`
        pub type U6144 = U6144_with_u128;

        /// 7168-bit unsigned integer for 128-bit machines, Synonym of `U7168_with_u128`
        pub type U7168 = U7168_with_u128;

        /// 8192-bit unsigned integer for 128-bit machines, Synonym of `U8192_with_u128`
        pub type U8192 = U8192_with_u128;

        /// 16384-bit unsigned integer for 128-bit machines, Synonym of `U16384_with_u128`
        pub type U16384 = U16384_with_u128;

        /// 32-byte unsigned integer, Synonym of `U256`
        pub type UU32 = U256;

        /// 64-byte unsigned integer, Synonym of `U512`
        pub type UU64 = U512;

        /// 128-byte unsigned integer, Synonym of `U1024`
        pub type UU128 = U1024;

        /// 256-byte unsigned integer, Synonym of `U2048`
        pub type UU256 = U2048;

        /// 384-byte unsigned integer, Synonym of `U3072`
        pub type UU384 = U3072;

        /// 512-byte unsigned integer, Synonym of `U4096`
        pub type UU512 = U4096;

        /// 640-byte unsigned integer, Synonym of `U5120`
        pub type UU640 = U5120;

        /// 760-byte unsigned integer, Synonym of `U6144`
        pub type UU768 = U6144;

        /// 896-byte unsigned integer, Synonym of `U7168`
        pub type UU896 = U7168;

        /// 1024-byte unsigned integer, Synonym of `U8192`
        pub type UU1024 = U8192;

        /// 2048-byte unsigned integer, Synonym of `U16384`
        pub type UU2048 = U16384;
    }
}


/// The macro that defines the types `U256`, `U512`, `U1024`, `U2048`,
/// `U3072`, `U4096`, `U5120`, `U6144`, `U7168`, `U8192`, and `U16384`
/// __based on `u64`__. 
/// 
/// The types `U256`, `U512`, `U1024`, `U2048`, `U3072`, `U4096`, `U5120`,
/// `U6144`, `U7168`, `U8192`, and `U16384` are 256-bit, 512-bit, 1024-bit,
/// 2048-bit, 3072-bit, 4096-bit, 5120-bit, 6144-bit, 7168-bit, 8192-bit,
/// and 16384-bit big unsigned integer types, respectively.
/// They are all defined __based on `u64`__.
/// 
/// So, it will define `U256`, `U512`, `U1024`, `U2048`, `U3072`, `U4096`,
/// `U5120`, `U6144`, `U7168`, `U8192`, and  `U16384` __based on `u64`__.
/// That is, `U256` and `U1024` will be defined to be `BigUInt<u64, 4>`
/// and `BigUInt<u64, 16>`, respectively, for example.
/// 
/// Furthermore, `UU32`, `UU64`, `UU128`, `UU256`, `UU384`, `UU512`, `UU640`,
/// `UU768`, `UU896`, `UU1024`, and `UU2048` will be also defined to be `U256`,
/// `U512`, `U1024`, `U2048`, `U3072`, `U4096`, `U5120`, `U6144`, `U7168`,
/// `U8192`, and `U16384`, respectively.
/// `UU32` is 32-byte big unsigned integer type, and
/// `UU64` is 64-byte big unsigned integer type, and
/// `UU128` is 128-byte big unsigned integer type and so on.
/// That is, `UU32` is a synonym of `U256`, and
/// `UU64` is a synonym of `U512`, and so on.
/// 
/// The following examples show how to use the macro `define_utypes_with_u64!()`.
/// 
/// # Example
/// ```
/// use cryptocol::define_utypes_with_u64;
/// define_utypes_with_u64!();
/// let a = U256::from_string("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
/// let b = a << 1;
/// println!("b = {}", b);
/// assert_eq!(b, UU32::from_string("24691342469135780246913578024691357802469135780246913578024691357802469135780").unwrap());
/// ```
/// Here, `u64` is used as a base type in the macro `define_utypes_with_u64!()`.
/// So, `U256` and `UU32` are both `BigUInt<u64, 4>`. This macro
/// `define_utypes_with_u64!()` is virtually the same as `define_utypes_with!(u64)`.
/// Actually, this macro `define_utypes_with_u64!()` is used in the macro
/// `define_utypes_with!(...)`.
#[macro_export]
macro_rules! define_utypes_with_u64
{
    () =>
    {
        use cryptocol::number::{ U256_with_u64, U512_with_u64, U1024_with_u64, U2048_with_u64,
                                U3072_with_u64, U4096_with_u64, U5120_with_u64, U6144_with_u64,
                                U7168_with_u64, U8192_with_u64, U16384_with_u64 };

        /// 256-bit unsigned integer for 64-bit machines, Synonym of `U256_with_u64`
        pub type U256 = U256_with_u64;

        /// 512-bit unsigned integer for 64-bit machines, Synonym of `U512_with_u64`
        pub type U512 = U512_with_u64;

        /// 1024-bit unsigned integer for 64-bit machines, Synonym of `U1024_with_u64`
        pub type U1024 = U1024_with_u64;

        /// 2048-bit unsigned integer for 64-bit machines, Synonym of `U2048_with_u64`
        pub type U2048 = U2048_with_u64;

        /// 3072-bit unsigned integer for 64-bit machines, Synonym of `U3072_with_u64`
        pub type U3072 = U3072_with_u64;

        /// 4096-bit unsigned integer for 64-bit machines, Synonym of `U4096_with_u64`
        pub type U4096 = U4096_with_u64;

        /// 5120-bit unsigned integer for 64-bit machines, Synonym of `U5120_with_u64`
        pub type U5120 = U5120_with_u64;

        /// 6144-bit unsigned integer for 64-bit machines, Synonym of `U6144_with_u64`
        pub type U6144 = U6144_with_u64;

        /// 7168-bit unsigned integer for 64-bit machines, Synonym of `U7168_with_u64`
        pub type U7168 = U7168_with_u64;

        /// 8192-bit unsigned integer for 64-bit machines, Synonym of `U8192_with_u64`
        pub type U8192 = U8192_with_u64;

        /// 16384-bit unsigned integer for 64-bit machines, Synonym of `U16384_with_u64`
        pub type U16384 = U16384_with_u64;

        /// 32-byte unsigned integer, Synonym of `U256`
        pub type UU32 = U256;

        /// 64-byte unsigned integer, Synonym of `U512`
        pub type UU64 = U512;

        /// 128-byte unsigned integer, Synonym of `U1024`
        pub type UU128 = U1024;

        /// 256-byte unsigned integer, Synonym of `U2048`
        pub type UU256 = U2048;

        /// 384-byte unsigned integer, Synonym of `U3072`
        pub type UU384 = U3072;

        /// 512-byte unsigned integer, Synonym of `U4096`
        pub type UU512 = U4096;

        /// 640-byte unsigned integer, Synonym of `U5120`
        pub type UU640 = U5120;

        /// 760-byte unsigned integer, Synonym of `U6144`
        pub type UU768 = U6144;

        /// 896-byte unsigned integer, Synonym of `U7168`
        pub type UU896 = U7168;

        /// 1024-byte unsigned integer, Synonym of `U8192`
        pub type UU1024 = U8192;

        /// 2048-byte unsigned integer, Synonym of `U16384`
        pub type UU2048 = U16384;
    };
}


/// The macro that defines the types `U256`, `U512`, `U1024`, `U2048`,
/// `U3072`, `U4096`, `U5120`, `U6144`, `U7168`, `U8192`, and `U16384`
/// __based on `u32`__. 
/// 
/// The types `U256`, `U512`, `U1024`, `U2048`, `U3072`, `U4096`, `U5120`,
/// `U6144`, `U7168`, `U8192`, and `U16384` are 256-bit, 512-bit, 1024-bit,
/// 2048-bit, 3072-bit, 4096-bit, 5120-bit, 6144-bit, 7168-bit, 8192-bit,
/// and 16384-bit big unsigned integer types, respectively.
/// They are all defined __based on `u32`__.
/// 
/// So, it will define `U256`, `U256_`, `U512`, `U512_`, `U1024`, `U1024_`,
/// `U2048`, `U2048_`, `U3072`, `U3072_`,`U4096`, `U4096_`, `U5120`, `U5120_`,
/// `U6144`, `U6144_`, `U7168`, `U7168_`, `U8192`, `U8192_`,
/// `U16384`, and `U16384_` __based on `u32`__.
/// That is, `U256` and `U1024` will be defined to be `BigUInt<u32, 8>`
/// and `BigUInt<<u32, 32>`, respectively, for example.
/// 
/// Furthermore, `UU32`, `UU64`, `UU128`, `UU256`, `UU384`, `UU512`, `UU640`,
/// `UU768`, `UU896`, `UU1024`, and `UU2048` will be also defined to be `U256`,
/// `U512`, `U1024`, `U2048`, `U3072`, `U4096`, `U5120`, `U6144`, `U7168`,
/// `U8192`, and `U16384`, respectively.
/// `UU32` is 32-byte big unsigned integer type, and
/// `UU64` is 64-byte big unsigned integer type, and
/// `UU128` is 128-byte big unsigned integer type and so on.
/// That is, `UU32` is a synonym of `U256`, and
/// `UU64` is a synonym of `U512`, and so on.
/// 
/// The following examples show how to use the macro `define_utypes_with_u32!()`.
/// 
/// # Example
/// ```
/// use cryptocol::define_utypes_with_u32;
/// define_utypes_with_u32!();
/// let a = U256::from_string("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
/// let b = a << 1;
/// println!("b = {}", b);
/// assert_eq!(b, UU32::from_string("24691342469135780246913578024691357802469135780246913578024691357802469135780").unwrap());
/// ```
/// Here, `u16` is used as a base type in the macro `define_utypes_with_u32!()`.
/// So, `U256` and `UU32` are both `BigUInt<u32, 8>`. This macro
/// `define_utypes_with_u32!()` is virtually the same as `define_utypes_with!(u32)`.
/// Actually, this macro `define_utypes_with_u32!()` is used in the macro
/// `define_utypes_with!(...)`.
#[macro_export]
macro_rules! define_utypes_with_u32
{
    () =>
    {
        use cryptocol::number::{ U256_with_u32, U512_with_u32, U1024_with_u32, U2048_with_u32,
                                U3072_with_u32, U4096_with_u32, U5120_with_u32, U6144_with_u32,
                                U7168_with_u32, U8192_with_u32, U16384_with_u32 };

        /// 256-bit unsigned integer for 32-bit machines, Synonym of `U256_with_u32`
        pub type U256 = U256_with_u32;
            
        /// 512-bit unsigned integer for 32-bit machines, Synonym of `U512_with_u32`
        pub type U512 = U512_with_u32;

        /// 1024-bit unsigned integer for 32-bit machines, Synonym of `U1024_with_u32`
        pub type U1024 = U1024_with_u32;

        /// 2048-bit unsigned integer for 32-bit machines, Synonym of `U2048_with_u32`
        pub type U2048 = U2048_with_u32;

        /// 3072-bit unsigned integer for 32-bit machines, Synonym of `U3072_with_u32`
        pub type U3072 = U3072_with_u32;

        /// 4096-bit unsigned integer for 32-bit machines, Synonym of `U4096_with_u32`
        pub type U4096 = U4096_with_u32;

        /// 5120-bit unsigned integer for 32-bit machines, Synonym of `U5120_with_u32`
        pub type U5120 = U5120_with_u32;

        /// 6144-bit unsigned integer for 32-bit machines, Synonym of `U6144_with_u32`
        pub type U6144 = U6144_with_u32;

        /// 7168-bit unsigned integer for 32-bit machines, Synonym of `U7168_with_u32`
        pub type U7168 = U7168_with_u32;

        /// 8192-bit unsigned integer for 32-bit machines, Synonym of `U8192_with_u32`
        pub type U8192 = U8192_with_u32;

        /// 16384-bit unsigned integer for 32-bit machines, Synonym of `U16384_with_u32`
        pub type U16384 = U16384_with_u32;

        /// 32-byte unsigned integer, Synonym of `U256`
        pub type UU32 = U256;

        /// 64-byte unsigned integer, Synonym of `U512`
        pub type UU64 = U512;

        /// 128-byte unsigned integer, Synonym of `U1024`
        pub type UU128 = U1024;

        /// 256-byte unsigned integer, Synonym of `U2048`
        pub type UU256 = U2048;

        /// 384-byte unsigned integer, Synonym of `U3072`
        pub type UU384 = U3072;

        /// 512-byte unsigned integer, Synonym of `U4096`
        pub type UU512 = U4096;

        /// 640-byte unsigned integer, Synonym of `U5120`
        pub type UU640 = U5120;

        /// 760-byte unsigned integer, Synonym of `U6144`
        pub type UU768 = U6144;

        /// 896-byte unsigned integer, Synonym of `U7168`
        pub type UU896 = U7168;

        /// 1024-byte unsigned integer, Synonym of `U8192`
        pub type UU1024 = U8192;

        /// 2048-byte unsigned integer, Synonym of `U16384`
        pub type UU2048 = U16384;
    };
}


/// The macro that defines the types `U256`, `U512`, `U1024`, `U2048`,
/// `U3072`, `U4096`, `U5120`, `U6144`, `U7168`, `U8192`, and `U16384`
/// __based on `u16`__. 
/// 
/// The types `U256`, `U512`, `U1024`, `U2048`, `U3072`, `U4096`, `U5120`,
/// `U6144`, `U7168`, `U8192`, and `U16384` are 256-bit, 512-bit, 1024-bit,
/// 2048-bit, 3072-bit, 4096-bit, 5120-bit, 6144-bit, 7168-bit, 8192-bit,
/// and 16384-bit big unsigned integer types, respectively.
/// They are all defined __based on `u16`__.
/// 
/// So, it will define `U256`, `U512`, `U1024`, `U2048`, `U3072`, `U4096`,
/// `U5120`, `U6144`, `U7168`, `U8192`, and  `U16384` __based on `u16`__.
/// That is, `U256` and `U1024` will be defined to be `BigUInt<u16, 16>`
/// and `BigUInt<u16, 64>`, respectively, for example.
/// 
/// Furthermore, `UU32`, `UU64`, `UU128`, `UU256`, `UU384`, `UU512`, `UU640`,
/// `UU768`, `UU896`, `UU1024`, and `UU2048` will be also defined to be `U256`,
/// `U512`, `U1024`, `U2048`, `U3072`, `U4096`, `U5120`, `U6144`, `U7168`,
/// `U8192`, and `U16384`, respectively.
/// `UU32` is 32-byte big unsigned integer type, and
/// `UU64` is 64-byte big unsigned integer type, and
/// `UU128` is 128-byte big unsigned integer type and so on.
/// That is, `UU32` is a synonym of `U256`, and
/// `UU64` is a synonym of `U512`, and so on.
/// 
/// The following examples show how to use the macro `define_utypes_with_u16!()`.
/// 
/// # Example
/// ```
/// use cryptocol::define_utypes_with_u16;
/// define_utypes_with_u16!();
/// let a = U256::from_string("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
/// let b = a << 1;
/// println!("b = {}", b);
/// assert_eq!(b, UU32::from_string("24691342469135780246913578024691357802469135780246913578024691357802469135780").unwrap());
/// ```
/// Here, `u16` is used as a base type in the macro `define_utypes_with_u16!()`.
/// So, `U256` and `UU32` are both `BigUInt<u16, 16>`. This macro
/// `define_utypes_with_u16!()` is virtually the same as `define_utypes_with!(u16)`.
/// Actually, this macro `define_utypes_with_u16!()` is used in the macro
/// `define_utypes_with!(...)`.
#[macro_export]
macro_rules! define_utypes_with_u16
{
    () =>
    {
        use cryptocol::number::{ U256_with_u16, U512_with_u16, U1024_with_u16, U2048_with_u16,
                                U3072_with_u16, U4096_with_u16, U5120_with_u16, U6144_with_u16,
                                U7168_with_u16, U8192_with_u16, U16384_with_u16 };
                                
        /// 256-bit unsigned integer for 16-bit machines, Synonym of `U256_with_u16`
        pub type U256 = U256_with_u16;

        /// 512-bit unsigned integer for 16-bit machines, Synonym of `U512_with_u16`
        pub type U512 = U512_with_u16;

        /// 1024-bit unsigned integer for 16-bit machines, Synonym of `U1024_with_u16`
        pub type U1024 = U1024_with_u16;

        /// 2048-bit unsigned integer for 16-bit machines, Synonym of `U2048_with_u16`
        pub type U2048 = U2048_with_u16;

        /// 3072-bit unsigned integer for 16-bit machines, Synonym of `U3072_with_u16`
        pub type U3072 = U3072_with_u16;

        /// 4096-bit unsigned integer for 16-bit machines, Synonym of `U4096_with_u16`
        pub type U4096 = U4096_with_u16;

        /// 5120-bit unsigned integer for 16-bit machines, Synonym of `U5120_with_u16`
        pub type U5120 = U5120_with_u16;

        /// 6144-bit unsigned integer for 16-bit machines, Synonym of `U6144_with_u16`
        pub type U6144 = U6144_with_u16;

        /// 7168-bit unsigned integer for 16-bit machines, Synonym of `U7168_with_u16`
        pub type U7168 = U7168_with_u16;

        /// 8192-bit unsigned integer for 16-bit machines, Synonym of `U8192_with_u16`
        pub type U8192 = U8192_with_u16;

        /// 16384-bit unsigned integer for 16-bit machines, Synonym of `U16384_with_u16`
        pub type U16384 = U16384_with_u16;

        /// 32-byte unsigned integer, Synonym of `U256`
        pub type UU32 = U256;

        /// 64-byte unsigned integer, Synonym of `U512`
        pub type UU64 = U512;

        /// 128-byte unsigned integer, Synonym of `U1024`
        pub type UU128 = U1024;

        /// 256-byte unsigned integer, Synonym of `U2048`
        pub type UU256 = U2048;

        /// 384-byte unsigned integer, Synonym of `U3072`
        pub type UU384 = U3072;

        /// 512-byte unsigned integer, Synonym of `U4096`
        pub type UU512 = U4096;

        /// 640-byte unsigned integer, Synonym of `U5120`
        pub type UU640 = U5120;

        /// 760-byte unsigned integer, Synonym of `U6144`
        pub type UU768 = U6144;

        /// 896-byte unsigned integer, Synonym of `U7168`
        pub type UU896 = U7168;

        /// 1024-byte unsigned integer, Synonym of `U8192`
        pub type UU1024 = U8192;

        /// 2048-byte unsigned integer, Synonym of `U16384`
        pub type UU2048 = U16384;
    };
}


/// The macro that defines the types `U256`, `U512`, `U1024`, `U2048`,
/// `U3072`, `U4096`, `U5120`, `U6144`, `U7168`, `U8192`, and `U16384`
/// __based on `u8`__.
/// 
/// The types `U256`, `U512`, `U1024`, `U2048`, `U3072`, `U4096`, `U5120`,
/// `U6144`, `U7168`, `U8192`, and `U16384` are 256-bit, 512-bit, 1024-bit,
/// 2048-bit, 3072-bit, 4096-bit, 5120-bit, 6144-bit, 7168-bit, 8192-bit,
/// and 16384-bit big unsigned integer types, respectively.
/// They are all defined __based on `u8`__.
/// 
/// So, it will define `U256`, `U512`, `U1024`, `U2048`, `U3072`, `U4096`,
/// `U5120`, `U6144`, `U7168`, `U8192`, and  `U16384` __based on `u8`__.
/// That is, `U256` and `U1024` will be defined to be `BigUInt<u8, 32>`
/// and `BigUInt<u8, 128>`, respectively, for example.
/// 
/// Furthermore, `UU32`, `UU64`, `UU128`, `UU256`, `UU384`, `UU512`, `UU640`,
/// `UU768`, `UU896`, `UU1024`, and `UU2048` will be also defined to be `U256`,
/// `U512`, `U1024`, `U2048`, `U3072`, `U4096`, `U5120`, `U6144`, `U7168`,
/// `U8192`, and `U16384`, respectively.
/// `UU32` is 32-byte big unsigned integer type, and
/// `UU64` is 64-byte big unsigned integer type, and
/// `UU128` is 128-byte big unsigned integer type and so on.
/// That is, `UU32` is a synonym of `U256`, and
/// `UU64` is a synonym of `U512`, and so on.
/// 
/// The following examples show how to use the macro `define_utypes_with_u8!()`.
/// 
/// # Example
/// ```
/// use cryptocol::define_utypes_with_u8;
/// define_utypes_with_u8!();
/// let a = U256::from_string("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
/// let b = a << 1;
/// println!("b = {}", b);
/// assert_eq!(b, UU32::from_string("24691342469135780246913578024691357802469135780246913578024691357802469135780").unwrap());
/// ```
/// Here, `u8` is used as a base type in the macro `define_utypes_with_u8!()`.
/// So, `U256` and `UU32` are both `BigUInt<u8, 32>`. This macro
/// `define_utypes_with_u8!()` is virtually the same as `define_utypes_with!(u8)`.
/// Actually, this macro `define_utypes_with_u8!()` is used in the macro
/// `define_utypes_with!(...)`.
#[macro_export]
macro_rules! define_utypes_with_u8
{
    () =>
    {
        use cryptocol::number::{ U256_with_u8, U512_with_u8, U1024_with_u8, U2048_with_u8,
                                U3072_with_u8, U4096_with_u8, U5120_with_u8, U6144_with_u8,
                                U7168_with_u8, U8192_with_u8, U16384_with_u8 };

        /// 256-bit unsigned integer for 8-bit machines, Synonym of `U256_with_u8`
        pub type U256 = U256_with_u8;

        /// 512-bit unsigned integer for 8-bit machines, Synonym of `U512_with_u8`
        pub type U512 = U512_with_u8;

        /// 1024-bit unsigned integer for 8-bit machines, Synonym of `U1024_with_u8`
        pub type U1024 = U1024_with_u8;

        /// 2048-bit unsigned integer for 8-bit machines, Synonym of `U2048_with_u8`
        pub type U2048 = U2048_with_u8;

        /// 3072-bit unsigned integer for 8-bit machines, Synonym of `U3072_with_u8`
        pub type U3072 = U3072_with_u8;

        /// 4096-bit unsigned integer for 8-bit machines, Synonym of `U4096_with_u8`
        pub type U4096 = U4096_with_u8;

        /// 5120-bit unsigned integer for 8-bit machines, Synonym of `U5120_with_u8`
        pub type U5120 = U5120_with_u8;

        /// 6144-bit unsigned integer for 8-bit machines, Synonym of `U6144_with_u8`
        pub type U6144 = U6144_with_u8;

        /// 7168-bit unsigned integer for 8-bit machines, Synonym of `U7168_with_u8`
        pub type U7168 = U7168_with_u8;

        /// 8192-bit unsigned integer for 8-bit machines, Synonym of `U8192_with_u8`
        pub type U8192 = U8192_with_u8;

        /// 16384-bit unsigned integer for 8-bit machines, Synonym of `U16384_with_u8`
        pub type U16384 = U16384_with_u8;

        /// 32-byte unsigned integer, Synonym of `U256`
        pub type UU32 = U256;

        /// 64-byte unsigned integer, Synonym of `U512`
        pub type UU64 = U512;

        /// 128-byte unsigned integer, Synonym of `U1024`
        pub type UU128 = U1024;

        /// 256-byte unsigned integer, Synonym of `U2048`
        pub type UU256 = U2048;

        /// 384-byte unsigned integer, Synonym of `U3072`
        pub type UU384 = U3072;

        /// 512-byte unsigned integer, Synonym of `U4096`
        pub type UU512 = U4096;

        /// 640-byte unsigned integer, Synonym of `U5120`
        pub type UU640 = U5120;

        /// 760-byte unsigned integer, Synonym of `U6144`
        pub type UU768 = U6144;

        /// 896-byte unsigned integer, Synonym of `U7168`
        pub type UU896 = U7168;

        /// 1024-byte unsigned integer, Synonym of `U8192`
        pub type UU1024 = U8192;

        /// 2048-byte unsigned integer, Synonym of `U16384`
        pub type UU2048 = U16384;
    };
}


// /// The macro that defines the types `UU32`, `UU64`, `UU128`, `UU256`, `UU384`,
// /// `UU512`, `UU640`, `UU768`, `UU896`, `UU1024`, and `UU2048`.
// /// 
// /// In order to use this macro `define_uutypes_with_utypes!()`, the types `U256`,
// /// `U512`, `U1024`, `U2048`, `U3072`, `U4096`, `U5120`, `U6144`, `U7168`,
// /// `U8192`, and `U16384` should have been defined beforehand.
// /// 
// /// The types `UU32`, `UU64`, `UU128`, `UU256`, `UU384`, `UU512`, `UU640`, `UU768`,
// /// `UU896`, `UU1024`, and `UU2048` are 32-byte, 64-byte, 128-byte, 256-byte,
// /// 384-byte, 512-byte, 640-byte, 768-byte, 896-byte, 1024-byte, and 2048-byte
// /// big unsigned integer types, respectively.
// /// 
// /// `UU32`, `UU64`, `UU128`, `UU256`, `UU384`, `UU512`, `UU640`, `UU768`, `UU896`,
// /// `UU1024`, and `UU2048` will be defined to be `U256`, `U512`, `U1024`,
// /// `U2048`, `U3072`, `U4096`, `U5120`, `U6144`, `U7168`, `U8192`, and `U16384`,
// /// respectively. That is, `UU32` is a synonym of `U256`, `UU64` is a synonym of
// /// `U512`, and so on.
// /// 
// /// Actually, this macro `define_uutypes_with_utypes!()` is
// /// used in the macros `define_utypes_with_u8!()`, `define_utypes_with_u16!()`,
// /// `define_utypes_with_u32!()`, `define_utypes_with_u64!()`, and
// /// `define_utypes_with_u128!()` which are used in the macro 
// /// `define_utypes_with!(...)`.
// /// 
// /// You are highly recommended not use this macro `define_uutypes_with_utypes!()`
// /// unless you really need to use it in your code. Instead, use
// /// `define_utypes_with_u8!()`, `define_utypes_with_u16!()`,
// /// `define_utypes_with_u32!()`, `define_utypes_with_u64!()`,
// /// `define_utypes_with_u128!()` or `define_utypes_with!(...)` if you need.
// #[macro_export]
// macro_rules! define_uutypes_with_utypes
// {
//     () =>
//     {
//         /// 32-byte unsigned integer, Synonym of `U256`
//         pub type UU32 = U256;

//         /// 64-byte unsigned integer, Synonym of `U512`
//         pub type UU64 = U512;

//         /// 128-byte unsigned integer, Synonym of `U1024`
//         pub type UU128 = U1024;

//         /// 256-byte unsigned integer, Synonym of `U2048`
//         pub type UU256 = U2048;

//         /// 384-byte unsigned integer, Synonym of `U3072`
//         pub type UU384 = U3072;

//         /// 512-byte unsigned integer, Synonym of `U4096`
//         pub type UU512 = U4096;

//         /// 640-byte unsigned integer, Synonym of `U5120`
//         pub type UU640 = U5120;

//         /// 760-byte unsigned integer, Synonym of `U6144`
//         pub type UU768 = U6144;

//         /// 896-byte unsigned integer, Synonym of `U7168`
//         pub type UU896 = U7168;

//         /// 1024-byte unsigned integer, Synonym of `U8192`
//         pub type UU1024 = U8192;

//         /// 2048-byte unsigned integer, Synonym of `U16384`
//         pub type UU2048 = U16384;
//     };
// }
