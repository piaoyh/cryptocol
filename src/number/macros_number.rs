// Copyright 2023 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![allow(non_camel_case_types)]
//! The module that contains macros for defining big unsigned integer data types. 

/// The macro that defines the types `u256`, `u512`, `u1024`, `u2048`, `u3072`,
/// `u4096`, `u5120`, `u6144`, `u7168`, `u8192`, and `u16384`. 
/// 
/// The types `u256`, `u512`, `u1024`, `u2048`, `u3072`, `u4096`, `u5120`,
/// `u6144`, `u7168`, `u8192`, and `u16384` are 256-bit, 512-bit, 1024-bit,
/// u2048-bit, 3072-bit, 4096-bit, 5120-bit, 6144-bit, 7168-bit, 8192-bit,
/// and 16384-bit big unsigned integer types, respectively. They are defined
/// based on `u8`, `u16`, `u32`, `u64` and `u128` according to the given parameter.
/// 
/// So, if you give `u128` as a parameter, it will define `u256`, `u512`,
/// `u1024`, `u2048`, `u3072`, `u4096`, `u5120`, `u6144`, `u7168`, `u8192`, and
/// `u16384` based on `u128`. That is, u256 and u1024 will be defined to be
/// `BigUInt<u128, 2>` and `BigUInt<u128, 8>`, respectively, for example.
/// 
/// Furthermore, `U32`, `U64`, `U128`, `U256`, `U384`, `U512`, `U640`, `U768`,
/// `U896`, `U1024`, and `U2048` will be also defined to be `u256`, `u512`, 
/// `u1024`, `u2048`, `u3072`, `u4096`, `u5120`, `u6144`, `u7168`, `u8192`, and
/// `u16384`, respectively. `U32` is 32-byte big unsigned integer type, and `U64`
/// is 64-byte big unsigned integer type, and `U128` is 128-byte big unsigned
/// integer type, and so on. That is, `U32` is a synonym of `u256`, `U64` is a
/// synonym of `u512`, and so on.
/// 
/// If you define big unsigned integer types with `define_utypes_with!(u128)`,
/// `u1024` and `U128` will be `BigUInt<u128, 8>`.
/// If you define big unsigned integer types with `define_utypes_with!(u64)`,
/// `u1024` and `U128` will be `BigUInt<u64, 16>`.
/// If you define big unsigned integer types with `define_utypes_with!(u32)`,
/// `u1024` and `U128` will be `BigUInt<u32, 32>`.
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
/// use Cryptocol::number::*;
/// 
/// fn main()
/// {
///     let num_txt = "1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_1111111111_";
///     let a_128 = u1024_with_u128::from_string(num_txt).unwrap();
///     let a_64 = u1024_with_u64::from_string(num_txt).unwrap();
///     let a_32 = u1024_with_u32::from_string(num_txt).unwrap();
///     let a_16 = u1024_with_u16::from_string(num_txt).unwrap();
///     let a_8 = u1024_with_u8::from_string(num_txt).unwrap();
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
/// use Cryptocol::define_utypes_with;
/// define_utypes_with!(u128);
/// let a = u256::from_string("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
/// let b = a << 1;
/// println!("b = {}", b);
/// assert_eq!(b, U32::from_string("24691342469135780246913578024691357802469135780246913578024691357802469135780").unwrap());
/// ```
/// Here, `u128` is used as a base type in the macro `define_utypes_with!(u128)`.
/// So, `u256` and `U32` are both `BigUInt<u128, 2>`, but you can choose a
/// different parameter such as `u64`. Then, `u256` and `U32` are both
/// `BigUInt<u64, 4>`.
/// 
/// The following example shows the different case that `u64`
/// is used as a base type in the macro `define_utypes_with!(u64)`.
/// ```
/// use Cryptocol::define_utypes_with;
/// define_utypes_with!(u64);
/// let a = u256::from_string("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
/// let b = a << 1;
/// println!("b = {}", b);
/// assert_eq!(b, U32::from_string("24691342469135780246913578024691357802469135780246913578024691357802469135780").unwrap());
/// ```
#[macro_export]
macro_rules! define_utypes_with
{
    (u128) =>
        {
            use Cryptocol::define_utypes_with_u128;
            define_utypes_with_u128!();
        };
    (u64) =>
        {
            use Cryptocol::define_utypes_with_u64;
            define_utypes_with_u64!();
        };
    (u32) =>
        {
            use Cryptocol::define_utypes_with_u32;
            define_utypes_with_u32!();
        };
    (u16) =>
        {
            use Cryptocol::define_utypes_with_u16;
            define_utypes_with_u16!();
        };
    (u8) =>
        {
            use Cryptocol::define_utypes_with_u8;
            define_utypes_with_u8!();
        };
}


/// The macro that defines the types `u256`, `u512`, `u1024`, `u2048`, `u3072`,
/// `u4096`, `u5120`, `u6144`, `u7168`, `u8192`, and `u16384` __based on `u128`__. 
/// 
/// The types `u256`, `u512`, `u1024`, `u2048`, `u3072`, `u4096`, `u5120`,
/// `u6144`, `u7168`, `u8192`, and `u16384` are 256-bit, 512-bit, 1024-bit,
/// u2048-bit, 3072-bit, 4096-bit, 5120-bit, 6144-bit, 7168-bit, 8192-bit,
/// and 16384-bit big unsigned integer types, respectively. They are all defined
/// __based on `u128`__.
/// 
/// So, it will define `u256`, `u512`, `u1024`, `u2048`, `u3072`, `u4096`,
/// `u5120`, `u6144`, `u7168`, `u8192`, and `u16384` __based on `u128`__.
/// That is, u256 and u1024 will be defined to be `BigUInt<u128, 2>` and
/// `BigUInt<u128, 8>`, respectively, for example.
/// 
/// Furthermore, `U32`, `U64`, `U128`, `U256`, `U384`, `U512`, `U640`, `U768`,
/// `U896`, `U1024`, and `U2048` will be also defined to be `u256`, `u512`, 
/// `u1024`, `u2048`, `u3072`, `u4096`, `u5120`, `u6144`, `u7168`, `u8192`, and
/// `u16384`, respectively. `U32` is 32-byte big unsigned integer type, and `U64`
/// is 64-byte big unsigned integer type, and `U128` is 128-byte big unsigned
/// integer type, and so on. That is, `U32` is a synonym of `u256`, `U64` is a
/// synonym of `u512`, and so on.
/// 
/// The following examples show how to use the macro `define_utypes_with_u128!()`.
/// 
/// # Example
/// ```
/// use Cryptocol::define_utypes_with_u128;
/// define_utypes_with_u128!();
/// let a = u256::from_string("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
/// let b = a << 1;
/// println!("b = {}", b);
/// assert_eq!(b, U32::from_string("24691342469135780246913578024691357802469135780246913578024691357802469135780").unwrap());
/// ```
/// Here, `u128` is used as a base type in the macro `define_utypes_with_u128!()`.
/// So, `u256` and `U32` are both `BigUInt<u128, 2>`. This macro
/// `define_utypes_with_u128!()` is virtually the same as `define_utypes_with!(u128)`.
/// Actually, this macro `define_utypes_with_u128!()` is used in the macro
/// `define_utypes_with!(...)`.
#[macro_export]
macro_rules! define_utypes_with_u128
{
    () =>
    {
        use Cryptocol::number::{ u256_with_u128, u512_with_u128, u1024_with_u128, u2048_with_u128,
                                u3072_with_u128, u4096_with_u128, u5120_with_u128, u6144_with_u128,
                                u7168_with_u128, u8192_with_u128, u16384_with_u128 };
        use Cryptocol::define_Utypes_with_utypes;

        /// 256-bit unsigned integer, Synonym of `u256_with_u128`
        pub type u256 = u256_with_u128;

        /// 512-bit unsigned integer for 128-bit machines, Synonym of `u512_with_u128`
        pub type u512 = u512_with_u128;

        /// 1024-bit unsigned integer for 128-bit machines, Synonym of `u1024_with_u128`
        pub type u1024 = u1024_with_u128;

        /// 2048-bit unsigned integer for 128-bit machines, Synonym of `u2048_with_u128`
        pub type u2048 = u2048_with_u128;

        /// 3072-bit unsigned integer for 128-bit machines, Synonym of `u3072_with_u128`
        pub type u3072 = u3072_with_u128;

        /// 4096-bit unsigned integer for 128-bit machines, Synonym of `u4096_with_u128`
        pub type u4096 = u4096_with_u128;

        /// 5120-bit unsigned integer for 128-bit machines, Synonym of `u5120_with_u128`
        pub type u5120 = u5120_with_u128;

        /// 6144-bit unsigned integer for 128-bit machines, Synonym of `u6144_with_u128`
        pub type u6144 = u6144_with_u128;

        /// 7168-bit unsigned integer for 128-bit machines, Synonym of `u7168_with_u128`
        pub type u7168 = u7168_with_u128;

        /// 8192-bit unsigned integer for 128-bit machines, Synonym of `u8192_with_u128`
        pub type u8192 = u8192_with_u128;

        /// 16384-bit unsigned integer for 128-bit machines, Synonym of `u16384_with_u128`
        pub type u16384 = u16384_with_u128;

        define_Utypes_with_utypes!();
    }
}


/// The macro that defines the types `u256`, `u512`, `u1024`, `u2048`, `u3072`,
/// `u4096`, `u5120`, `u6144`, `u7168`, `u8192`, and `u16384` __based on `u64`__. 
/// 
/// The types `u256`, `u512`, `u1024`, `u2048`, `u3072`, `u4096`, `u5120`,
/// `u6144`, `u7168`, `u8192`, and `u16384` are 256-bit, 512-bit, 1024-bit,
/// u2048-bit, 3072-bit, 4096-bit, 5120-bit, 6144-bit, 7168-bit, 8192-bit,
/// and 16384-bit big unsigned integer types, respectively. They are all defined
/// __based on `u64`__.
/// 
/// So, it will define `u256`, `u512`, `u1024`, `u2048`, `u3072`, `u4096`,
/// `u5120`, `u6144`, `u7168`, `u8192`, and `u16384` __based on `u64`__.
/// That is, u256 and u1024 will be defined to be `BigUInt<u64, 4>` and
/// `BigUInt<u64, 16>`, respectively, for example.
/// 
/// Furthermore, `U32`, `U64`, `U128`, `U256`, `U384`, `U512`, `U640`, `U768`,
/// `U896`, `U1024`, and `U2048` will be also defined to be `u256`, `u512`, 
/// `u1024`, `u2048`, `u3072`, `u4096`, `u5120`, `u6144`, `u7168`, `u8192`, and
/// `u16384`, respectively. `U32` is 32-byte big unsigned integer type, and `U64`
/// is 64-byte big unsigned integer type, and `U128` is 128-byte big unsigned
/// integer type, and so on. That is, `U32` is a synonym of `u256`, `U64` is a
/// synonym of `u512`, and so on.
/// 
/// The following examples show how to use the macro `define_utypes_with_u64!()`.
/// 
/// # Example
/// ```
/// use Cryptocol::define_utypes_with_u64;
/// define_utypes_with_u64!();
/// let a = u256::from_string("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
/// let b = a << 1;
/// println!("b = {}", b);
/// assert_eq!(b, U32::from_string("24691342469135780246913578024691357802469135780246913578024691357802469135780").unwrap());
/// ```
/// Here, `u64` is used as a base type in the macro `define_utypes_with_u64!()`.
/// So, `u256` and `U32` are both `BigUInt<u64, 4>`. This macro
/// `define_utypes_with_u64!()` is virtually the same as `define_utypes_with!(u64)`.
/// Actually, this macro `define_utypes_with_u64!()` is used in the macro
/// `define_utypes_with!(...)`.
#[macro_export]
macro_rules! define_utypes_with_u64
{
    () =>
    {
        use Cryptocol::number::{ u256_with_u64, u512_with_u64, u1024_with_u64, u2048_with_u64,
                                u3072_with_u64, u4096_with_u64, u5120_with_u64, u6144_with_u64,
                                u7168_with_u64, u8192_with_u64, u16384_with_u64 };
        use Cryptocol::define_Utypes_with_utypes;

        /// 256-bit unsigned integer for 64-bit machines, Synonym of `u256_with_u64`
        pub type u256 = u256_with_u64;

        /// 512-bit unsigned integer for 64-bit machines, Synonym of `u512_with_u64`
        pub type u512 = u512_with_u64;

        /// 1024-bit unsigned integer for 64-bit machines, Synonym of `u1024_with_u64`
        pub type u1024 = u1024_with_u64;

        /// 2048-bit unsigned integer for 64-bit machines, Synonym of `u2048_with_u64`
        pub type u2048 = u2048_with_u64;

        /// 3072-bit unsigned integer for 64-bit machines, Synonym of `u3072_with_u64`
        pub type u3072 = u3072_with_u64;

        /// 4096-bit unsigned integer for 64-bit machines, Synonym of `u4096_with_u64`
        pub type u4096 = u4096_with_u64;

        /// 5120-bit unsigned integer for 64-bit machines, Synonym of `u5120_with_u64`
        pub type u5120 = u5120_with_u64;

        /// 6144-bit unsigned integer for 64-bit machines, Synonym of `u6144_with_u64`
        pub type u6144 = u6144_with_u64;

        /// 7168-bit unsigned integer for 64-bit machines, Synonym of `u7168_with_u64`
        pub type u7168 = u7168_with_u64;

        /// 8192-bit unsigned integer for 64-bit machines, Synonym of `u8192_with_u64`
        pub type u8192 = u8192_with_u64;

        /// 16384-bit unsigned integer for 64-bit machines, Synonym of `u16384_with_u64`
        pub type u16384 = u16384_with_u64;

        define_Utypes_with_utypes!();
    };
}


/// The macro that defines the types `u256`, `u512`, `u1024`, `u2048`, `u3072`,
/// `u4096`, `u5120`, `u6144`, `u7168`, `u8192`, and `u16384` __based on `u32`__. 
/// 
/// The types `u256`, `u512`, `u1024`, `u2048`, `u3072`, `u4096`, `u5120`,
/// `u6144`, `u7168`, `u8192`, and `u16384` are 256-bit, 512-bit, 1024-bit,
/// u2048-bit, 3072-bit, 4096-bit, 5120-bit, 6144-bit, 7168-bit, 8192-bit,
/// and 16384-bit big unsigned integer types, respectively. They are all defined
/// __based on `u32`__.
/// 
/// So, it will define `u256`, `u512`, `u1024`, `u2048`, `u3072`, `u4096`,
/// `u5120`, `u6144`, `u7168`, `u8192`, and `u16384` __based on `u32`__.
/// That is, u256 and u1024 will be defined to be `BigUInt<u32, 8>` and
/// `BigUInt<u32, 32>`, respectively, for example.
/// 
/// Furthermore, `U32`, `U64`, `U128`, `U256`, `U384`, `U512`, `U640`, `U768`,
/// `U896`, `U1024`, and `U2048` will be also defined to be `u256`, `u512`, 
/// `u1024`, `u2048`, `u3072`, `u4096`, `u5120`, `u6144`, `u7168`, `u8192`, and
/// `u16384`, respectively. `U32` is 32-byte big unsigned integer type, and `U64`
/// is 64-byte big unsigned integer type, and `U128` is 128-byte big unsigned
/// integer type, and so on. That is, `U32` is a synonym of `u256`, `U64` is a
/// synonym of `u512`, and so on.
/// 
/// The following examples show how to use the macro `define_utypes_with_u32!()`.
/// 
/// # Example
/// ```
/// use Cryptocol::define_utypes_with_u32;
/// define_utypes_with_u32!();
/// let a = u256::from_string("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
/// let b = a << 1;
/// println!("b = {}", b);
/// assert_eq!(b, U32::from_string("24691342469135780246913578024691357802469135780246913578024691357802469135780").unwrap());
/// ```
/// Here, `u16` is used as a base type in the macro `define_utypes_with_u32!()`.
/// So, `u256` and `U32` are both `BigUInt<u32, 8>`. This macro
/// `define_utypes_with_u32!()` is virtually the same as `define_utypes_with!(u32)`.
/// Actually, this macro `define_utypes_with_u32!()` is used in the macro
/// `define_utypes_with!(...)`.
#[macro_export]
macro_rules! define_utypes_with_u32
{
    () =>
    {
        use Cryptocol::number::{ u256_with_u32, u512_with_u32, u1024_with_u32, u2048_with_u32,
                                u3072_with_u32, u4096_with_u32, u5120_with_u32, u6144_with_u32,
                                u7168_with_u32, u8192_with_u32, u16384_with_u32 };
        use Cryptocol::define_Utypes_with_utypes;

        /// 256-bit unsigned integer for 32-bit machines, Synonym of `u256_with_u32`
        pub type u256 = u256_with_u32;
            
        /// 512-bit unsigned integer for 32-bit machines, Synonym of `u512_with_u32`
        pub type u512 = u512_with_u32;

        /// 1024-bit unsigned integer for 32-bit machines, Synonym of `u1024_with_u32`
        pub type u1024 = u1024_with_u32;

        /// 2048-bit unsigned integer for 32-bit machines, Synonym of `u2048_with_u32`
        pub type u2048 = u2048_with_u32;

        /// 3072-bit unsigned integer for 32-bit machines, Synonym of `u3072_with_u32`
        pub type u3072 = u3072_with_u32;

        /// 4096-bit unsigned integer for 32-bit machines, Synonym of `u4096_with_u32`
        pub type u4096 = u4096_with_u32;

        /// 5120-bit unsigned integer for 32-bit machines, Synonym of `u5120_with_u32`
        pub type u5120 = u5120_with_u32;

        /// 6144-bit unsigned integer for 32-bit machines, Synonym of `u6144_with_u32`
        pub type u6144 = u6144_with_u32;

        /// 7168-bit unsigned integer for 32-bit machines, Synonym of `u7168_with_u32`
        pub type u7168 = u7168_with_u32;

        /// 8192-bit unsigned integer for 32-bit machines, Synonym of `u8192_with_u32`
        pub type u8192 = u8192_with_u32;

        /// 16384-bit unsigned integer for 32-bit machines, Synonym of `u16384_with_u32`
        pub type u16384 = u16384_with_u32;

        define_Utypes_with_utypes!();
    };
}


/// The macro that defines the types `u256`, `u512`, `u1024`, `u2048`, `u3072`,
/// `u4096`, `u5120`, `u6144`, `u7168`, `u8192`, and `u16384` __based on `u16`__. 
/// 
/// The types `u256`, `u512`, `u1024`, `u2048`, `u3072`, `u4096`, `u5120`,
/// `u6144`, `u7168`, `u8192`, and `u16384` are 256-bit, 512-bit, 1024-bit,
/// u2048-bit, 3072-bit, 4096-bit, 5120-bit, 6144-bit, 7168-bit, 8192-bit,
/// and 16384-bit big unsigned integer types, respectively. They are all defined
/// __based on `u16`__.
/// 
/// So, it will define `u256`, `u512`, `u1024`, `u2048`, `u3072`, `u4096`,
/// `u5120`, `u6144`, `u7168`, `u8192`, and `u16384` __based on `u16`__.
/// That is, u256 and u1024 will be defined to be `BigUInt<u16, 16>` and
/// `BigUInt<u16, 64>`, respectively, for example.
/// 
/// Furthermore, `U32`, `U64`, `U128`, `U256`, `U384`, `U512`, `U640`, `U768`,
/// `U896`, `U1024`, and `U2048` will be also defined to be `u256`, `u512`, 
/// `u1024`, `u2048`, `u3072`, `u4096`, `u5120`, `u6144`, `u7168`, `u8192`, and
/// `u16384`, respectively. `U32` is 32-byte big unsigned integer type, and `U64`
/// is 64-byte big unsigned integer type, and `U128` is 128-byte big unsigned
/// integer type, and so on. That is, `U32` is a synonym of `u256`, `U64` is a
/// synonym of `u512`, and so on.
/// 
/// The following examples show how to use the macro `define_utypes_with_u16!()`.
/// 
/// # Example
/// ```
/// use Cryptocol::define_utypes_with_u16;
/// define_utypes_with_u16!();
/// let a = u256::from_string("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
/// let b = a << 1;
/// println!("b = {}", b);
/// assert_eq!(b, U32::from_string("24691342469135780246913578024691357802469135780246913578024691357802469135780").unwrap());
/// ```
/// Here, `u16` is used as a base type in the macro `define_utypes_with_u16!()`.
/// So, `u256` and `U32` are both `BigUInt<u16, 16>`. This macro
/// `define_utypes_with_u16!()` is virtually the same as `define_utypes_with!(u16)`.
/// Actually, this macro `define_utypes_with_u16!()` is used in the macro
/// `define_utypes_with!(...)`.
#[macro_export]
macro_rules! define_utypes_with_u16
{
    () =>
    {
        use Cryptocol::number::{ u256_with_u16, u512_with_u16, u1024_with_u16, u2048_with_u16,
                                u3072_with_u16, u4096_with_u16, u5120_with_u16, u6144_with_u16,
                                u7168_with_u16, u8192_with_u16, u16384_with_u16 };
        use Cryptocol::define_Utypes_with_utypes;

        /// 256-bit unsigned integer for 16-bit machines, Synonym of `u256_with_u16`
        pub type u256 = u256_with_u16;

        /// 512-bit unsigned integer for 16-bit machines, Synonym of `u512_with_u16`
        pub type u512 = u512_with_u16;

        /// 1024-bit unsigned integer for 16-bit machines, Synonym of `u1024_with_u16`
        pub type u1024 = u1024_with_u16;

        /// 2048-bit unsigned integer for 16-bit machines, Synonym of `u2048_with_u16`
        pub type u2048 = u2048_with_u16;

        /// 3072-bit unsigned integer for 16-bit machines, Synonym of `u3072_with_u16`
        pub type u3072 = u3072_with_u16;

        /// 4096-bit unsigned integer for 16-bit machines, Synonym of `u4096_with_u16`
        pub type u4096 = u4096_with_u16;

        /// 5120-bit unsigned integer for 16-bit machines, Synonym of `u5120_with_u16`
        pub type u5120 = u5120_with_u16;

        /// 6144-bit unsigned integer for 16-bit machines, Synonym of `u6144_with_u16`
        pub type u6144 = u6144_with_u16;

        /// 7168-bit unsigned integer for 16-bit machines, Synonym of `u7168_with_u16`
        pub type u7168 = u7168_with_u16;

        /// 8192-bit unsigned integer for 16-bit machines, Synonym of `u8192_with_u16`
        pub type u8192 = u8192_with_u16;

        /// 16384-bit unsigned integer for 16-bit machines, Synonym of `u16384_with_u16`
        pub type u16384 = u16384_with_u16;

        define_Utypes_with_utypes!();
    };
}


/// The macro that defines the types `u256`, `u512`, `u1024`, `u2048`, `u3072`,
/// `u4096`, `u5120`, `u6144`, `u7168`, `u8192`, and `u16384` __based on `u8`__.
/// 
/// The types `u256`, `u512`, `u1024`, `u2048`, `u3072`, `u4096`, `u5120`,
/// `u6144`, `u7168`, `u8192`, and `u16384` are 256-bit, 512-bit, 1024-bit,
/// u2048-bit, 3072-bit, 4096-bit, 5120-bit, 6144-bit, 7168-bit, 8192-bit,
/// and 16384-bit big unsigned integer types, respectively. They are all defined
/// __based on `u8`__.
/// 
/// So, it will define `u256`, `u512`, `u1024`, `u2048`, `u3072`, `u4096`,
/// `u5120`, `u6144`, `u7168`, `u8192`, and `u16384` __based on `u8`__.
/// That is, u256 and u1024 will be defined to be `BigUInt<u8, 32>` and
/// `BigUInt<u8, 128>`, respectively, for example.
/// 
/// Furthermore, `U32`, `U64`, `U128`, `U256`, `U384`, `U512`, `U640`, `U768`,
/// `U896`, `U1024`, and `U2048` will be also defined to be `u256`, `u512`, 
/// `u1024`, `u2048`, `u3072`, `u4096`, `u5120`, `u6144`, `u7168`, `u8192`, and
/// `u16384`, respectively. `U32` is 32-byte big unsigned integer type, and `U64`
/// is 64-byte big unsigned integer type, and `U128` is 128-byte big unsigned
/// integer type, and so on. That is, `U32` is a synonym of `u256`, `U64` is a
/// synonym of `u512`, and so on.
/// 
/// The following examples show how to use the macro `define_utypes_with_u8!()`.
/// 
/// # Example
/// ```
/// use Cryptocol::define_utypes_with_u8;
/// define_utypes_with_u8!();
/// let a = u256::from_string("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
/// let b = a << 1;
/// println!("b = {}", b);
/// assert_eq!(b, U32::from_string("24691342469135780246913578024691357802469135780246913578024691357802469135780").unwrap());
/// ```
/// Here, `u8` is used as a base type in the macro `define_utypes_with_u8!()`.
/// So, `u256` and `U32` are both `BigUInt<u8, 32>`. This macro
/// `define_utypes_with_u8!()` is virtually the same as `define_utypes_with!(u8)`.
/// Actually, this macro `define_utypes_with_u8!()` is used in the macro
/// `define_utypes_with!(...)`.
#[macro_export]
macro_rules! define_utypes_with_u8
{
    () =>
    {
        use Cryptocol::number::{ u256_with_u8, u512_with_u8, u1024_with_u8, u2048_with_u8,
                                u3072_with_u8, u4096_with_u8, u5120_with_u8, u6144_with_u8,
                                u7168_with_u8, u8192_with_u8, u16384_with_u8 };
        use Cryptocol::define_Utypes_with_utypes;

        /// 256-bit unsigned integer for 8-bit machines, Synonym of `u256_with_u8`
        pub type u256 = u256_with_u8;

        /// 512-bit unsigned integer for 8-bit machines, Synonym of `u512_with_u8`
        pub type u512 = u512_with_u8;

        /// 1024-bit unsigned integer for 8-bit machines, Synonym of `u1024_with_u8`
        pub type u1024 = u1024_with_u8;

        /// 2048-bit unsigned integer for 8-bit machines, Synonym of `u2048_with_u8`
        pub type u2048 = u2048_with_u8;

        /// 3072-bit unsigned integer for 8-bit machines, Synonym of `u3072_with_u8`
        pub type u3072 = u3072_with_u8;

        /// 4096-bit unsigned integer for 8-bit machines, Synonym of `u4096_with_u8`
        pub type u4096 = u4096_with_u8;

        /// 5120-bit unsigned integer for 8-bit machines, Synonym of `u5120_with_u8`
        pub type u5120 = u5120_with_u8;

        /// 6144-bit unsigned integer for 8-bit machines, Synonym of `u6144_with_u8`
        pub type u6144 = u6144_with_u8;

        /// 7168-bit unsigned integer for 8-bit machines, Synonym of `u7168_with_u8`
        pub type u7168 = u7168_with_u8;

        /// 8192-bit unsigned integer for 8-bit machines, Synonym of `u8192_with_u8`
        pub type u8192 = u8192_with_u8;

        /// 16384-bit unsigned integer for 8-bit machines, Synonym of `u16384_with_u8`
        pub type u16384 = u16384_with_u8;

        define_Utypes_with_utypes!();
    };
}


/// The macro that defines the types `U32`, `U64`, `U128`, `U256`, `U384`,
/// `U512`, `U640`, `U768`, `U896`, `U1024`, and `U2048`.
/// 
/// In order to use this macro `define_Utypes_with_utypes!()`, the types `u256`,
/// `u512`, `u1024`, `u2048`, `u3072`, `u4096`, `u5120`, `u6144`, `u7168`,
/// `u8192`, and `u16384` should have been defined beforehand.
/// 
/// The types `U32`, `U64`, `U128`, `U256`, `U384`, `U512`, `U640`, `U768`,
/// `U896`, `U1024`, and `U2048` are 32-byte, 64-byte, 128-byte, 256-byte,
/// 384-byte, 512-byte, 640-byte, 768-byte, 896-byte, 1024-byte, and 2048-byte
/// big unsigned integer types, respectively.
/// 
/// `U32`, `U64`, `U128`, `U256`, `U384`, `U512`, `U640`, `U768`, `U896`,
/// `U1024`, and `U2048` will be defined to be `u256`, `u512`, `u1024`,
/// `u2048`, `u3072`, `u4096`, `u5120`, `u6144`, `u7168`, `u8192`, and `u16384`,
/// respectively. That is, `U32` is a synonym of `u256`, `U64` is a synonym of
/// `u512`, and so on.
/// 
/// Actually, this macro `define_Utypes_with_utypes!()` is
/// used in the macros `define_utypes_with_u8!()`, `define_utypes_with_u16!()`,
/// `define_utypes_with_u32!()`, `define_utypes_with_u64!()`, and
/// `define_utypes_with_u128!()` which are used in the macro 
/// `define_utypes_with!(...)`.
/// 
/// You are highly recommended not use this macro `define_Utypes_with_utypes!()`
/// unless you really need to use it in your code. Instead, use
/// `define_utypes_with_u8!()`, `define_utypes_with_u16!()`,
/// `define_utypes_with_u32!()`, `define_utypes_with_u64!()`,
/// `define_utypes_with_u128!()` or `define_utypes_with!(...)` if you need.
#[macro_export]
macro_rules! define_Utypes_with_utypes
{
    () =>
    {
        /// 32-byte unsigned integer, Synonym of `u256`
        pub type U32 = u256;

        /// 64-byte unsigned integer, Synonym of `u512`
        pub type U64 = u512;

        /// 128-byte unsigned integer, Synonym of `u1024`
        pub type U128 = u1024;

        /// 256-byte unsigned integer, Synonym of `u2048`
        pub type U256 = u2048;

        /// 384-byte unsigned integer, Synonym of `u3072`
        pub type U384 = u3072;

        /// 512-byte unsigned integer, Synonym of `u4096`
        pub type U512 = u4096;

        /// 640-byte unsigned integer, Synonym of `u5120`
        pub type U640 = u5120;

        /// 760-byte unsigned integer, Synonym of `u6144`
        pub type U768 = u6144;

        /// 896-byte unsigned integer, Synonym of `u7168`
        pub type U896 = u7168;

        /// 1024-byte unsigned integer, Synonym of `u8192`
        pub type U1024 = u8192;

        /// 2048-byte unsigned integer, Synonym of `u16384`
        pub type U2048 = u16384;
    };
}
