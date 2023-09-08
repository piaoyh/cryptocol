// Copyright 2023 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module that contains a big unsigned integer struct
//! with user-defined fixed size and its methods.

//#![warn(missing_docs)]
//#![warn(missing_doc_code_examples)]
use std::fmt::{ self, Display, Formatter, Debug };
use std::mem::{ size_of, transmute };
use std::cmp::{ PartialEq, PartialOrd, Ordering };
use std::convert::{ From, Into };
use std::str::FromStr;
use std::ops::*;
use std::ops::Bound::*;

use rand::RngCore;
use rand::rngs::OsRng;

use super::trait_impl_for_big_uint::*;
use super::uint::*;
use super::int_unions::*;
use super::NumberErr;


/// 256-bit unsigned integer implemented by `BigUInt<u128, 2>` made with two `u128`s
pub type u256_with_u128 = BigUInt<u128, 2>;

/// 512-bit unsigned integer implemented by `BigUInt<u128, 4>` made with four `u128`s
pub type u512_with_u128 = BigUInt<u128, 4>;

/// 1024-bit unsigned integer implemented by `BigUInt<u128, 8>` made with eight `u128`s
pub type u1024_with_u128 = BigUInt<u128, 8>;

/// 2048-bit unsigned integer implemented by `BigUInt<u128, 4>` made with sixteen `u128`s
pub type u2048_with_u128 = BigUInt<u128, 16>;

/// 3072-bit unsigned integer implemented by `BigUInt<u128, 4>` made with twenty-four `u128`s
pub type u3072_with_u128 = BigUInt<u128, 24>;

/// 4096-bit unsigned integer implemented by `BigUInt<u128, 4>` made with thirty-two `u128`s
pub type u4096_with_u128 = BigUInt<u128, 32>;

/// 5120-bit unsigned integer implemented by `BigUInt<u128, 4>` made with forty `u128`s
pub type u5120_with_u128 = BigUInt<u128, 40>;

/// 6144-bit unsigned integer implemented by `BigUInt<u128, 4>` made with fory-eight `u128`s
pub type u6144_with_u128 = BigUInt<u128, 48>;

/// 7168-bit unsigned integer implemented by `BigUInt<u128, 4>` made with fifty-six `u128`s
pub type u7168_with_u128 = BigUInt<u128, 56>;

/// 8192-bit unsigned integer implemented by `BigUInt<u128, 4>` made with sixty-four `u128`s
pub type u8192_with_u128 = BigUInt<u128, 64>;

/// 16384-bit unsigned integer implemented by `BigUInt<u128, 4>` made with one hundred twenty-eight `u128`s
pub type u16384_with_u128 = BigUInt<u128, 128>;


/// 256-bit unsigned integer implemented by `BigUInt<u64, 4>` made with four `u64`s
pub type u256_with_u64 = BigUInt<u64, 4>;

/// 512-bit unsigned integer implemented by `BigUInt<u64, 8>` made with eight `u64`s
pub type u512_with_u64 = BigUInt<u64, 8>;

/// 1024-bit unsigned integer implemented by `BigUInt<u64, 16>` made with sixteen `u64`s
pub type u1024_with_u64 = BigUInt<u64, 16>;

/// 2048-bit unsigned integer implemented by `BigUInt<u64, 32>` made with thirty-two `u64`s
pub type u2048_with_u64 = BigUInt<u64, 32>;

/// 3072-bit unsigned integer implemented by `BigUInt<u64, 48>` made with fourty-eight `u64`s
pub type u3072_with_u64 = BigUInt<u64, 48>;

/// 4096-bit unsigned integer implemented by `BigUInt<u64, 64>` made with sixty-four `u64`s
pub type u4096_with_u64 = BigUInt<u64, 64>;

/// 5120-bit unsigned integer implemented by `BigUInt<u64, 80>` made with eighty `u64`s
pub type u5120_with_u64 = BigUInt<u64, 80>;

/// 6144-bit unsigned integer implemented by `BigUInt<u64, 96>` made with ninety-six `u64`s
pub type u6144_with_u64 = BigUInt<u64, 96>;

/// 7168-bit unsigned integer implemented by BigUInt<u64, 112> made with one hundred twelve `u64`s
pub type u7168_with_u64 = BigUInt<u64, 112>;

/// 8192-bit unsigned integer implemented by `BigUInt<u64, 128>` made with two hundred twenty-eight `u64`s
pub type u8192_with_u64 = BigUInt<u64, 128>;

/// 16384-bit unsigned integer implemented by `BigUInt<u64, 256>` made with two hundred fifty-six `u64`s
pub type u16384_with_u64 = BigUInt<u64, 256>;


/// 256-bit unsigned integer implemented by `BigUInt<u32, 8>` made with eight `u32`s
pub type u256_with_u32 = BigUInt<u32, 8>;

/// 512-bit unsigned integer implemented by `BigUInt<u32, 8>` made with sixteen `u32`s
pub type u512_with_u32 = BigUInt<u32, 16>;

/// 1024-bit unsigned integer implemented by `BigUInt<u32, 8>` made with thirty-two `u32`s
pub type u1024_with_u32 = BigUInt<u32, 32>;

/// 2048-bit unsigned integer implemented by `BigUInt<u32, 8>` made with sixty-four `u32`s
pub type u2048_with_u32 = BigUInt<u32, 64>;

/// 3072-bit unsigned integer implemented by `BigUInt<u32, 8>` made with ninety-six `u32`s
pub type u3072_with_u32 = BigUInt<u32, 96>;

/// 4096-bit unsigned integer implemented by `BigUInt<u32, 8>` made with one hundred twenty-eight `u32`s
pub type u4096_with_u32 = BigUInt<u32, 128>;

/// 5120-bit unsigned integer implemented by `BigUInt<u32, 8>` made with one hundred sixty `u32`s
pub type u5120_with_u32 = BigUInt<u32, 160>;

/// 6144-bit unsigned integer implemented by `BigUInt<u32, 8>` made with one hundred ninety-two `u32`s
pub type u6144_with_u32 = BigUInt<u32, 192>;

/// 7168-bit unsigned integer implemented by `BigUInt<u32, 8>` made with two hundred twenty-four `u32`s
pub type u7168_with_u32 = BigUInt<u32, 224>;

/// 8192-bit unsigned integer implemented by `BigUInt<u32, 8>` made with two hundred fifty-six `u32`s
pub type u8192_with_u32 = BigUInt<u32, 256>;

/// 16384-bit unsigned integer implemented by `BigUInt<u32, 8>` made with five hundred twelve `u32`s
pub type u16384_with_u32 = BigUInt<u32, 512>;


/// 256-bit unsigned integer implemented by `BigUInt<u16, 16>` made with sixteen `u16`s
pub type u256_with_u16 = BigUInt<u16, 16>;

/// 512-bit unsigned integer implemented by `BigUInt<u16, 32>` made with thirty-two `u16`s
pub type u512_with_u16 = BigUInt<u16, 32>;

/// 1024-bit unsigned integer implemented by `BigUInt<u16, 64>` made with sixty-four `u16`s
pub type u1024_with_u16 = BigUInt<u16, 64>;

/// 2048-bit unsigned integer implemented by `BigUInt<u16, 128>` made with one hundred twenty-eight `u16`s
pub type u2048_with_u16 = BigUInt<u16, 128>;

/// 3072-bit unsigned integer implemented by `BigUInt<u16, 192>` made with one hundred ninety-two `u16`s
pub type u3072_with_u16 = BigUInt<u16, 192>;

/// 4096-bit unsigned integer implemented by `BigUInt<u16, 256>` made with two hundred fifty-six `u16`s
pub type u4096_with_u16 = BigUInt<u16, 256>;

/// 5120-bit unsigned integer implemented by `BigUInt<u16, 320>` made with three hundred twenty `u16`s
pub type u5120_with_u16 = BigUInt<u16, 320>;

/// 6144-bit unsigned integer implemented by `BigUInt<u16, 384>` made with three hundred eighty-four `u16`s
pub type u6144_with_u16 = BigUInt<u16, 384>;

/// 7168-bit unsigned integer implemented by `BigUInt<u16, 448>` made with four hundred forty-eight `u16`s
pub type u7168_with_u16 = BigUInt<u16, 448>;

/// 8192-bit unsigned integer implemented by `BigUInt<u16, 512>` made with five hundred twelve `u16`s
pub type u8192_with_u16 = BigUInt<u16, 512>;

/// 16384-bit unsigned integer implemented by `BigUInt<u16, 1024>` made with one thousand twenty-four `u16`s
pub type u16384_with_u16 = BigUInt<u16, 1024>;


/// 256-bit unsigned integer implemented by `BigUInt<u8, 32>` made with thirty-two `u8`s
pub type u256_with_u8 = BigUInt<u8, 32>;

/// 512-bit unsigned integer implemented by `BigUInt<u8, 64>` made with sixty-four `u8`s
pub type u512_with_u8 = BigUInt<u8, 64>;

/// 1024-bit unsigned integer implemented by `BigUInt<u8, 128>` made with one hundred twenty-eight `u8`s
pub type u1024_with_u8 = BigUInt<u8, 128>;

/// 2048-bit unsigned integer implemented by `BigUInt<u8, 256>` made with two hundred fifty-six `u8`s
pub type u2048_with_u8 = BigUInt<u8, 256>;

/// 3072-bit unsigned integer implemented by `BigUInt<u8, 384>` made with three hundred eighty-four `u8`s
pub type u3072_with_u8 = BigUInt<u8, 384>;

/// 4096-bit unsigned integer implemented by `BigUInt<u8, 512>` made with five hundred twelve `u8`s
pub type u4096_with_u8 = BigUInt<u8, 512>;

/// 5120-bit unsigned integer implemented by `BigUInt<u8, 640>` made with six hundred forty-eight `u8`s
pub type u5120_with_u8 = BigUInt<u8, 640>;

/// 6144-bit unsigned integer implemented by `BigUInt<u8, 768>` made with seven hundred sixty-eight `u8`s
pub type u6144_with_u8 = BigUInt<u8, 768>;

/// 7168-bit unsigned integer implemented by `BigUInt<u8, 896>` made with eight hundred ninety-six `u8`s
pub type u7168_with_u8 = BigUInt<u8, 896>;

/// 8192-bit unsigned integer implemented by `BigUInt<u8, 1024>` made with one thousand twenty-four `u8`s
pub type u8192_with_u8 = BigUInt<u8, 1024>;

/// 16384-bit unsigned integer implemented by `BigUInt<u8, 2048>` made with two thousand forty-eight `u8`s
pub type u16384_with_u8 = BigUInt<u8, 2048>;


/// A struct that represents a big unsigned integer with user-defined fixed size.
/// 
/// The struct `BigUInt<T, const N: usize>` is a generic struct for which you
/// can decide the type of elements and its number. It is Little-endian.
/// - It consists of an array of type `T` with `N` elements and flags of type
///   `u8`.
/// - The generic data type `T` is supposed to be a _primitive unsigned integer_
///   type such as `u8`, `u16`, `u32`, `u64`, `u128` and `usize`. So, you are
///   supposed to choose one of `u8`, `u16`, `u32`, `u64`, `u128` and `usize`.
///   You cannot give a _signed integer_ such as `i8`, `i16`, `i32`, `i64`,
///   `i128` and `isize`. If you give a signed integer for generic type `T`, you
///   will get errors when you compile it. Of course, you can give the data type
///   other than `u8`, `u16`, `u32`, `u64`, `u128` and `usize` if the data type
///   that you give has the `UInt` trait. Then, you have to define all the
///   behaviors.
/// - If you give usize for the generic type `T`, the size of the type `usize`
///   depends on operating system. If your operating system is for 64-bit
///   machine, usize will be the same size of `u64` while your operating system
///   is for 32-bit machine, `usize` will be the same size of `u32`.
/// - The same sized `BigUInt` can be made in several ways.
///   For example, a 1024-bit unsigned integer can be implemented with
///   either `BigUInt<u128, 8>`, `BigUInt<u64, 16>`, `BigUInt<u32, 32>`,
///   `BigUInt<u16, 64>`, or `BigUInt<u8, 128>`. They are all 1024-bit
///   unsigned integers, but their performance will be different from 
///   one another depending on CPU.
/// - flags are OVERFLOW (0b0000_0001), UNDERFLOW (0b0000_0010),
///   INFINITY (0b0000_0100), and DIVIDED_BY_ZERO (== INFINITY)
/// 
/// # Panics
/// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
/// undefined though it may not panic.
/// 
/// # Examples
/// ```
/// use std::str::FromStr;
/// use Cryptocol::number::*;
/// type u1024 = BigUInt::<u128, 8>;
/// type U128 = BigUInt::<u64, 16>;
/// let a = u1024::from([1;8]);
/// let b = u1024::from_str_radix("00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001", 2).unwrap();
/// let mut c = u1024::from_str("1234567891234567879123456789111111111222222222333333333444444444555555555666666666777777777888888888999999999000000000").unwrap();
/// let d = U128::from_biguint(&c);
/// println!("a = {:?}", a);
/// println!("a = {}", a.to_string_with_radix(16).unwrap());
/// println!("b = {:?}", b);
/// println!("b = {}", b.to_string_with_radix(16).unwrap());
/// println!("c = {}", c);
/// println!("c + b = {}", c.wrapping_add(&b));
/// println!("c - b = {}", c.wrapping_sub(&b));
/// println!("c * b = {}", c.wrapping_mul(&b));
/// println!("b / c = {}", b.wrapping_div(&c));
/// println!("b % c = {}", b.wrapping_rem(&c));
/// println!("b + 1 = {}", b.wrapping_add_uint(1_u8));
/// println!("b - 1 = {}", b.wrapping_sub_uint(1_u8));
/// assert_eq!(a, b);
/// c.set_one();
/// assert_eq!(c, u1024::one());
/// ```
#[derive(Debug, Clone)]
pub struct BigUInt<T, const N: usize>
where T: Uint + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    number: [T; N],
    flag: u8,
}

impl<'a, T, const N: usize> BigUInt<T, N>
where T: Uint + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd,
    Self: Sized + Clone + Display + Debug + ToString + 'a
        + Add<Output = Self> + AddAssign
        + Add<&'a Self, Output = Self> + AddAssign<&'a Self>
        + Sub<Output = Self> + SubAssign
        + Sub<&'a Self, Output = Self> + SubAssign<&'a Self>
        + Mul<Output = Self> + MulAssign
        + Mul<&'a Self, Output = Self> + MulAssign<&'a Self>
        + Div<Output = Self> + DivAssign
        + Div<&'a Self, Output = Self> + DivAssign<&'a Self>
        + Rem<Output = Self> + RemAssign
        + Rem<&'a Self, Output = Self> + RemAssign<&'a Self>
        + Shl<i32, Output = Self> + ShlAssign<i32>
        + Shr<i32, Output = Self> + ShrAssign<i32>
        + BitAnd<Self, Output = Self> + BitAndAssign
        + BitAnd<&'a Self, Output = Self> + BitAndAssign<&'a Self>
        + BitOr<Self, Output = Self> + BitOrAssign
        + BitOr<&'a Self, Output = Self> + BitOrAssign<&'a Self>
        + BitXor<Self, Output = Self> + BitXorAssign
        + BitXor<&'a Self, Output = Self> + BitXorAssign<&'a Self>
        + Not<Output = Self>
        + From<T> + FromStr + From<[T; N]> + From<u32>
{
    /***** CONSTANTS FOR FLAGS *****/

    /// A flag to represent whether or not overflow happened
    /// during previous operations. When divided-by-zero happens,
    /// the flags `DIVIDED_BY_ZERO`, `INFINITY` and `OVERFLOW` will be set.
    const OVERFLOW: u8          = 0b0000_0001;

    /// A flag to represent whether or not underflow happened
    /// during previous operations.
    const UNDERFLOW: u8         = 0b0000_0010;
    
    /// A flag to represent whether or not the value became extremely big
    /// for some reasons such as "divided by zero" during previous operations.
    /// When divided-by-zero happens,
    /// the flags `DIVIDED_BY_ZERO`, `INFINITY` and `OVERFLOW` will be set.
    const INFINITY: u8          = 0b0000_0100;

    /// A flag to represent whether or not divided-by-zero happened
    /// during previous operations. When divided-by-zero happens,
    /// the flags `DIVIDED_BY_ZERO`, `INFINITY` and `OVERFLOW` will be set.
    const DIVIDED_BY_ZERO: u8   = 0b0000_1000;


    /***** CONSTRUCTORS *****/

    // pub fn new() -> Self
    /// Constructs a new `BigUInt<T, N>`.
    /// 
    /// # Output
    /// A new object of `BigUInt<T, N>`.
    /// 
    /// # Initialization
    /// All the attributes of the constructed object will be
    /// initialized with `0`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let a = u256::new();
    /// println!("a = {}", a);
    /// assert_eq!(a, u256::from(0_u128));
    /// ```
    pub fn new() -> Self
    {
        Self { number: [T::zero(); N], flag: 0, }   // unsafe { zeroed::<Self>() }
    }

    // pub fn zero() -> Self
    /// Constructs a new `BigUInt<T, N>` which has the value of zero.
    /// 
    /// # Output
    /// A new object of `BigUInt<T, N>` that represents zero.
    /// 
    /// # Features
    /// This function calls `BigUInt<T, N>::new()`, so it is virtually exactly
    /// the same as the function `BigUInt<T, N>::new()`.
    /// 
    /// # Benefit
    /// Your source code will be better readable if you use
    /// `BigUInt<T, N>::zero()` instead of `BigUInt<T, N>::new()` especially
    /// when you create the big number zero.
    ///
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let zero = u256::zero();
    /// println!("zero = {}", zero);
    /// assert_eq!(zero, u256::new());
    /// ```
    #[inline]
    pub fn zero() -> Self
    {
        Self::new()   // unsafe { zeroed::<Self>() }
    }

    // pub fn one() -> Self
    /// Constructs a new `BigUInt<T, N>` which has the value of one.
    /// 
    /// # Output
    /// A new object of `BigUInt<T, N>` that represents one.
    /// 
    /// # Benefit
    /// Your source code will be better readable if you use
    /// `BigUInt<T, N>::one()` instead of `BigUInt<T, N>::new()` and then
    /// `set_uint(1)` especially when you create the big number one.
    /// 
    /// # Example
    /// ```
    /// use std::convert::From;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// let one = u256::one();
    /// println!("one = {}", one);
    /// assert_eq!(one, u256::from(1_u64));
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn one() -> Self
    {
        let mut me = Self::new();
        me.set_num_(0, T::one());
        me
    }

    // pub fn max() -> Self
    /// Constructs a new `BigUInt<T, N>` which has the value of maximum.
    /// 
    /// # Output
    /// A new object of `BigUInt<T, N>` that represents maximum value.
    /// 
    /// # Features
    /// All bits are set to be `1`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// let maximum = u256::max();
    /// println!("maximum = {}", maximum);
    /// assert_eq!(maximum, !u256::zero());
    /// ```
    pub fn max() -> Self
    {
        Self { number: [T::max(); N], flag: 0, }
    }

    // pub fn submax(size_in_bits: usize) -> Self
    /// Constructs a new `BigUInt<T, N>`-type object which has the value of
    /// `size_in_bits`-bit long maximum value in which all bits are set to
    /// be `1`.
    /// 
    /// # Output
    /// A new object of `BigUInt<T, N>` that represents `size_in_bits`-bit
    /// long maximum value.
    /// 
    /// # Features
    /// This method will make all the `size_in_bits` bits of `number[T;N]` of
    /// `self` from LSB (Least Significant Bit) to be `1` and the rest of the
    /// bits up to MSB (Most Significant Bit) to be `0`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// use Cryptocol::number::*;
    /// use std::str::FromStr;
    /// define_utypes_with!(u64);
    /// let maximum = u256::max();
    /// let half = u256::submax(128_usize);
    /// println!("maximum =\t{}\nhalf maximum = \t{}", maximum, half);
    /// assert_eq!(maximum, u256::from_str("347376267711948586270712955026063723559809953996921692118372752023739388919807").unwrap());
    /// assert_eq!(half, u256::from_str("340282366920938463463374607431768211455").unwrap());
    /// ```
    pub fn submax(size_in_bits: usize) -> Self
    {
        let mut res = Self::max();
        res.set_submax(size_in_bits);
        res
    }

    // pub fn halfmax() -> Self
    /// Constructs a new `BigUInt<T, N>`-type object which has the value of
    /// half-length maximum value in which all bits are set to be `1`.
    /// 
    /// # Output
    /// A new object of `BigUInt<T, N>` that represents a half-length
    /// maximum value.
    /// 
    /// # Features
    /// This method will make all the lower half bits of `number[T;N]` of
    /// `self` from LSB (Least Significant Bit) to be `1` and the rest of the
    /// bits up to MSB (Most Significant Bit) to be `0`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// use Cryptocol::number::*;
    /// use std::str::FromStr;
    /// define_utypes_with!(u64);
    /// let maximum = u256::max();
    /// let half = u256::halfmax();
    /// println!("maximum =\t{}\nhalf maximum = \t{}", maximum, half);
    /// assert_eq!(maximum, u256::from_str("347376267711948586270712955026063723559809953996921692118372752023739388919807").unwrap());
    /// assert_eq!(half, u256::from_str("340282366920938463463374607431768211455").unwrap());
    /// ```
    #[inline]
    pub fn halfmax() -> Self
    {
        Self::submax(Self::size_in_bits() >> 1)
    }

    // pub fn from_uint<U>(val: U) -> Self
    /// Constructs a new `BigUInt<T, N>`-type object from an unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, `u128` and `usize`.
    /// 
    /// # Output
    /// A new object of `BigUInt<T, N>` that represents the same value of `val`.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, this method may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Examples for u8
    /// ```
    /// use Cryptocol::number::BigUInt;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let aa = u512::from_uint(123_u8);
    /// println!("aa = {}", aa);
    /// assert_eq!(aa.into_u8(), 123_u8);
    /// ```
    /// 
    /// # Examples for u16
    /// ```
    /// use Cryptocol::number::BigUInt;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let bb = u512::from_uint(12345_u16);
    /// println!("bb = {}", bb);
    /// assert_eq!(bb.into_u16(), 12345_u16);
    /// ```
    /// 
    /// # Examples for u32
    /// ```
    /// use Cryptocol::number::BigUInt;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let cc = u512::from_uint(1234567890_u32);
    /// println!("cc = {}", cc);
    /// assert_eq!(cc.into_u32(), 1234567890_u32);
    /// ```
    /// 
    /// # Examples for u64
    /// ```
    /// use Cryptocol::number::BigUInt;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let dd = u512::from_uint(12345678901234567890_u64);
    /// println!("dd = {}", dd);
    /// assert_eq!(dd.into_u64(), 12345678901234567890_u64);
    /// ```
    /// 
    /// # Examples for u128
    /// ```
    /// use Cryptocol::number::BigUInt;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let ee = u512::from_uint(123456789012345678901234567890123456789_u128);
    /// println!("ee = {}", ee);
    /// assert_eq!(ee.into_u128(), 123456789012345678901234567890123456789_u128);
    /// ```
    /// 
    /// # Examples for usize
    /// ```
    /// use Cryptocol::number::BigUInt;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let ff = u512::from_uint(12345678901234567890_usize);
    /// println!("ff = {}", ff);
    /// assert_eq!(ff.into_usize(), 12345678901234567890_usize);
    /// ```
    pub fn from_uint<U>(val: U) -> Self
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let TSIZE = T::size_in_bytes();
        let SSIZE = U::size_in_bytes();
        let mut me = Self::new();
        let mut share = Share::<T, U>::from_src(val);
        
        if TSIZE >= SSIZE
        {
            unsafe { me.set_num_(0, share.des); }
        }
        else
        {
            let TSIZE_BITS = TSIZE * 8;
            for i in 0..SSIZE/TSIZE
            {
                unsafe { me.set_num_(i, share.des); }
                unsafe { share.src >>= U::num(TSIZE_BITS as u128); }
            }
        }
        return me;
    }

    // pub fn from_array(val: &[T; N]) -> Self
    /// Constructs a new `BigUInt<T, N>` from an array of type `T` with `N`
    /// elements.
    /// 
    /// # Output
    /// A new object of `BigUInt<T, N>` that represents the same value of array `val`.
    /// 
    /// # Counterpart Method
    /// You can also use the method [from()](struct@BigUInt#impl-From<[T;+N]>-for-BigUInt<T,+N>)
    /// implemented by implementation of trait From<[T;N]>.
    /// 
    /// # Examples
    /// ```
    /// use Cryptocol::number::*;
    /// let big_num = BigUInt::<u8,32>::from_array(&[1_u8;32]);
    /// println!("big_num = {}", big_num.to_string_with_radix(2).unwrap());
    /// assert_eq!(big_num, BigUInt::<u8,32>::from_str_radix("00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001", 2).unwrap());
    /// ```
    pub fn from_array(val: &[T; N]) -> Self
    {
        let mut s = Self::new();
        s.set_number(val);
        s
    }

    // pub fn from_biguint<U, const M: usize>(biguint: &BigUInt<U, M>) -> Self
    /// Constructs a new `BigUInt<T, N>` from another kind of `BigUInt<U, M>`.
    /// 
    /// # Output
    /// A new object of `BigUInt<T, N>` that represents the same value of another
    /// kind of `BigUInt<U, M>`.
    /// 
    /// # Feature
    /// It copies not only long-bit integer but also current flags from another
    /// kind of `BigUInt<U, M>`.
    /// 
    /// # Example for the same length
    /// ```
    /// use std::str::FromStr;
    /// use std::mem::size_of;
    /// use Cryptocol::number::*;
    /// type u8_32 = BigUInt::<u8, 32>;
    /// type u16_16 = BigUInt::<u16, 16>;
    /// let a = u8_32::from_str("123456789123456789123456789123456789123456789123456789").unwrap();
    /// let b = u16_16::from_biguint(&a);
    /// println!("a = {}", a);
    /// println!("b = {}", b);
    /// ```
    #[cfg(target_endian = "little")]
    pub fn from_biguint<U, const M: usize>(biguint: &BigUInt<U, M>) -> Self
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let my_size = Self::size_in_bytes();
        let src_size = BigUInt::<U, M>::size_in_bytes();
        if my_size <= src_size
        {
            let mut me = Self::new();
            let src = unsafe { transmute::<&[U;M], &[T;N]>(&biguint.number) };
            me.number.copy_from_slice(src);
            me
        }
        else //if my_size > src_size
        {
            let common = Common::<T, N, U, M>::from_src(&biguint.number);
            unsafe { Self::from_array(&common.des) }
        }
    }

    // pub fn from_biguint<U, const M: usize>(biguint: &BigUInt<U, M>) -> Self
    /// Constructs a new `BigUInt<T, N>` from another kind of `BigUInt<U, M>`.
    /// 
    /// # Output
    /// A new object of `BigUInt<T, N>` that represents the same value of another
    /// kind of `BigUInt<U, M>`.
    /// 
    /// # Example for the same length
    /// ```
    /// use std::str::FromStr;
    /// use std::mem::size_of;
    /// use Cryptocol::number::*;
    /// type u8_32 = BigUInt::<u8, 32>;
    /// type u16_16 = BigUInt::<u16, 16>;
    /// let a = u8_32::from_str("123456789123456789123456789123456789123456789123456789").unwrap();
    /// let b = u16_16::from_biguint(&a);
    /// println!("a = {}", a);
    /// println!("b = {}", b);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[cfg(target_endian = "big")]
    pub fn from_biguint<U, const M: usize>(biguint: &BigUInt<U, M>) -> Self
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let my_size = Self::size_in_bytes();
        let src_size = BigUInt::<U, M>::size_in_bytes();
        let mut me = Self::new();
        if my_size == src_size
        {
            let src = unsafe { transmute::<&[U;M], &[T;N]>(&biguint.number) };
            me.number.copy_from_slice(src);
        }
        else
        {
            let mut common = Common::<T, N, U, M>::from_src(&biguint.number);
            common.into_des(&mut me.number);
        }
        me
    }

    // pub fn from_be(be: &Self) -> Self
    /// Converts a big integer from big endian to the target’s endianness.
    /// 
    /// # Feature
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[cfg(target_endian = "little")]
    #[inline]
    pub fn from_be(be: &Self) -> Self
    {
        be.swap_bytes()
    }

    // pub fn from_be(be: &Self) -> Self
    /// Converts a big integer from big endian to the target’s endianness.
    /// 
    /// # Feature
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[cfg(target_endian = "big")]
    #[inline]
    pub fn from_be(be: &Self) -> Self
    {
        be.clone()
    }

    // pub fn from_be_bytes(be_bytes: &[T; N]) -> Self
    /// Create a native endian integer value from its representation
    /// as a byte array in big endian.
    /// 
    /// # Feature
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[cfg(target_endian = "little")]
    #[inline]
    pub fn from_be_bytes(be_bytes: &[T; N]) -> Self
    {
        Self::from_array(be_bytes).swap_bytes()
    }

    // pub fn from_be_bytes(be_bytes: &[T; N]) -> Self
    /// Create a native endian integer value from its representation
    /// as a byte array in big endian.
    /// 
    /// # Feature
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[cfg(target_endian = "big")]
    #[inline]
    pub fn from_be_bytes(be_bytes: &[T; N]) -> Self
    {
        Self::from_array(be_bytes)
    }

    // pub fn from_le(le: &Self) -> Self
    /// Converts a little integer from little endian to the target’s endianness.
    /// 
    /// # Feature
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[cfg(target_endian = "little")]
    #[inline]
    pub fn from_le(le: &Self) -> Self
    {
        le.clone()
    }

    // pub fn from_le(le: &Self) -> Self
    /// Converts a little integer from little endian to the target’s endianness.
    /// 
    /// # Feature
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[cfg(target_endian = "big")]
    #[inline]
    pub fn from_le(le: &Self) -> Self
    {
        le.swap_bytes()
    }

    // pub fn from_le_bytes(le_bytes: &[T; N]) -> Self
    /// Create a native endian integer value from its representation
    /// as a byte array in little endian.
    /// 
    /// # Feature
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[cfg(target_endian = "little")]
    #[inline]
    pub fn from_le_bytes(le_bytes: &[T; N]) -> Self
    {
        Self::from_array(le_bytes)
    }

    // pub fn from_le_bytes(le_bytes: &[T; N]) -> Self
    /// Create a native endian integer value from its representation
    /// as a byte array in little endian.
    /// 
    /// # Feature
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[cfg(target_endian = "big")]
    #[inline]
    pub fn from_le_bytes(le_bytes: &[T; N]) -> Self
    {
        Self::from_array(le_bytes).swap_bytes()
    }

    //  pub fn from_str_radix(txt: &str, radix: usize) -> Result<Self, NumberErr>
    /// Constructs a new `BigUInt<T, N>` from a string with radix.
    /// 
    /// # Output
    /// The constructed object will be wrapped in `Ok(BigUInt<T, N>)` if it is
    /// successfully created. Otherwise, this method returns one of
    /// `Err(NumberErr::OutOfValidRadixRange)`, `Err(NumberErr::NotAlphaNumeric)`,
    /// and `Err(NumberErr::NotFitToRadix)` according to its failure reason.
    /// 
    /// # Errors
    /// - If the argument `txt` of this method includes any alphanumeric
    /// letter(s), it will return `Err(NumberErr::NotAlphaNumeric)`.
    /// - If the argument `radix` of this method is out of the valid range from
    /// `2` up to `62` inclusively, it will return `Err(NumberErr::OutOfValidRadixRange)`.
    /// - If the argument `txt` of this method includes any letter(s) out of the
    /// valid letter range even if they are alphanumeric, it will return
    /// `Err(NumberErr::NotFitToRadix)`. For example, in the case of hexadecimal
    /// number system which means that the argument radix is `16`, if the
    /// argument `txt` includes 'g', it will return `Err(NumberErr::NotFitToRadix)`.
    /// 
    /// # Valid Radix Range
    /// The radix can be from `2` up to `62` (= 10 + 26 + 26). Such radices that
    /// are less than `2` or more than `62` are not available. In this case,
    /// this method will return `Err(NumberErr::OutOfValidRadixRange)`.
    /// 
    /// # Radix more than `10` and less than `37`
    /// If the radix is more than `10` and less than `37`, the digit bigger than
    /// `9` will be expressed with alphabets. The avaiable alphabets are
    /// _case-insensitive_. For example, in the case of hexadecimal number
    /// system, the digit whose value is `10`, `11`, `12`, `13`, `14`, and `15`
    /// are represented as `A` or `a`, `B` or `b`, `C` or `c`, `D` or `d`, `E`
    /// or `e`, and `F` or `f`, respectively. And, in the case of 37-ary number
    /// system, the values `16`, `35` and `36` are represented as `G` or `g`,
    /// `Y` or `y`, and `Z` or `z`, respectively.
    /// 
    /// # Radix more than `36` and less than `63`
    /// However, if the radix is more than `36` and less than `63`, the digit
    /// bigger than `9` will be expressed with alphabets. The avaiable alphabets
    /// are _case-sensitive_, so `A` is different from `a`. For instance, in the
    /// case of 62-ary number system, the digit whose value is `10`, `11`, `35`,
    /// `36`, `37`, `38`, `60` and `61` are represented as `A`, `B`, `Y`, `Z`,
    /// `a`, `b`, `y` and `z`, respectively.
    /// 
    /// # Delimiter _
    /// In the number expression in a string, you can separate the digits with
    /// '_' in order to make it more readable. So, "10000" is the same as
    /// "1_0000".
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use Cryptocol::number::{ BigUInt, NumberErr };
    /// let a = BigUInt::<u8,32>::from_str_radix("1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0", 16).unwrap();
    /// println!("a = {}", a);
    /// assert_eq!(a.to_string(), "8234104123542484900769178205574010627627573691361805720124810878238590820080");
    /// ```
    pub fn from_str_radix(txt: &str, radix: usize) -> Result<Self, NumberErr>
    {
        if (radix < 2) || (radix > 10 + 26 + 26)
            { return Err(NumberErr::OutOfValidRadixRange); }

        let mut bignum = Self::zero();
        for c in txt.chars()
        {
            if c == '_'
                { continue; }
            if !c.is_alphanumeric()
                { return Err(NumberErr::NotAlphaNumeric); }
            if radix <= 10
            {
                if c as usize >= '0' as usize + radix
                    { return Err(NumberErr::NotFitToRadix); }
            }
            else if radix <= 10 + 26
            {
                if (c as usize >= 'A' as usize + radix - 10) 
                        && (c as usize <= 'Z' as usize)
                    || (c as usize >= 'a' as usize + radix - 10)
                    { return Err(NumberErr::NotFitToRadix); }
            }
            else if c as usize >= 'a' as usize + radix - (10 + 26)  // radix <= 10 + 26 + 26
                { return Err(NumberErr::NotFitToRadix); }

            let num: usize = if radix <= 10    { c as usize - '0' as usize }
                        else if radix <= 10 + 26
                        {
                            if c as usize <= '9' as usize
                                { c as usize - '0' as usize }
                            else if c as usize <= 'Z' as usize
                                { c as usize - 'A' as usize + 10 }
                            else
                                { c as usize - 'a' as usize + 10 }
                        }
                        else    // if radix <= 10 + 26 + 26
                        {
                            if c as usize <= '9' as usize
                                { c as usize - '0' as usize }
                            else if c as usize <= 'Z' as usize
                                { c as usize - 'A' as usize + 10 }
                            else
                                { c as usize - 'a' as usize + 10 + 26 }
                        };
            bignum.wrapping_mul_assign_uint(T::usize_as_Uint(radix));
            bignum.wrapping_add_assign_uint(T::usize_as_Uint(num));
        }
        if bignum.is_overflow()
            { Err(NumberErr::TooBigNumber) }
        else
            { Ok(bignum) }
    }

    // pub fn generate_check_bits(bit_pos: usize) -> Self
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
    /// # Example
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with_u8;
    /// 
    /// define_utypes_with_u8!();
    /// let a = u256::generate_check_bits(12);
    /// println!("a = {}", a.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a, u256::from_str_radix("10000_00000000", 2).unwrap());
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn generate_check_bits(bit_pos: usize) -> Self
    {
        let mut check_bits = Self::zero();
        check_bits.turn_check_bits(bit_pos);
        check_bits
    }



    /***** METHODS FOR GENERATING RANDOM PRIME NUMBERS *****/

    // pub fn random() -> Self
    /// Constucts a new `BigUInt<T, N>`-type object which has the random value.
    /// 
    /// # Output
    /// The random number that this method random() returns is a pure random
    /// number whose range is from 0 up to BigUInt::max() inclusively.
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
    /// 
    /// define_utypes_with!(u128);
    /// println!("Random number = {}", u1024::random());
    /// ```
    pub fn random() -> Self
    {
        let mut r = Self::new();
        r.randomize();
        r
    }

    // pub fn random_odd() -> Self
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
    /// random_odd() can be considered cryptographically unsecure though This
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
    /// 
    /// define_utypes_with!(u128);
    /// println!("Random number = {}", u1024::random_odd());
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn random_odd() -> Self
    {
        let mut r = Self::random();
        r.set_LSB();
        r
    }

    // pub fn random_less_than(ceiling: &Self) -> Self
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
    /// random_less_than() can be considered cryptographically unsecure though
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
    /// define_utypes_with!(u128);
    /// let ceiling = u1024::max() / u1024::from_uint::<u32>(3);
    /// println!("Random number = {}", u1024::random_less_than(&ceiling));
    /// ```
    #[inline]
    pub fn random_less_than(ceiling: &Self) -> Self
    {
        Self::random().wrapping_rem(ceiling)
    }

    // pub fn random_odd_less_than(ceiling: &Self) -> Self
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
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let ceiling = u1024::max() / u1024::from_uint::<u32>(3);
    /// println!("Random number = {}", u1024::random_odd_less_than(&ceiling));
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn random_odd_less_than(ceiling: &Self) -> Self
    {
        let mut r = Self::random_less_than(ceiling);
        r.set_LSB();
        r
    }

    // pub fn random_with_MSB_set() -> Self
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
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with_u128;
    /// 
    /// define_utypes_with_u128!();
    /// let num = u1024::random_with_MSB_set();
    /// println!("Random number = {}", u1024::random());
    /// println!("Random number = {}", num);
    /// assert!(num > u1024::max() >> 1);
    /// ```
    ///
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn random_with_MSB_set() -> Self
    {
        let mut r = Self::random();
        r.set_MSB();
        r
    }

    // pub fn random_odd_with_MSB_set() -> Self
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
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with_u128;
    /// 
    /// define_utypes_with_u128!();
    /// let num = u1024::random_odd_with_MSB_set();
    /// println!("Random number = {}", u1024::random());
    /// println!("Random number = {}", num);
    /// let yes = num.is_odd();
    /// if yes  { assert!(yes); }
    /// else    { assert!(!yes); }
    /// ```
    pub fn random_odd_with_MSB_set() -> Self
    {
        let mut r = Self::random_with_MSB_set();
        r.set_LSB();
        r
    }

    // pub fn random_prime_Miller_Rabin(repetition: usize) -> Self
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
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with_u128;
    /// 
    /// define_utypes_with_u128!();
    /// let num = u256::random_prime_Miller_Rabin(5);
    /// println!("Random number = {}", num);
    /// let yes = num.is_prime_Miller_Rabin(5);
    /// if yes  { assert!(yes) }
    /// else    { assert!(!yes) }
    /// ```
    pub fn random_prime_Miller_Rabin(repetition: usize) -> Self
    {
        let mut complete = false;
        let mut res = Self::new();
        while !complete
        {
            res.randomize();
            res.set_MSB();
            res.set_LSB();
            complete = res.is_prime_Miller_Rabin(repetition);
        }
        res
    }

    // pub fn randomize(&mut self)
    /// Make a `BigUInt<T, N>`-type object to have a random value.
    /// The random number that this method randomize() makes is a pure random
    /// number whose range is from 0 up to BigUInt::max() inclusively.
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
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with_u128;
    /// define_utypes_with_u128!();
    /// 
    /// let mut r = u256::new();
    /// println!("original number = {}", r);
    /// assert_eq!(r, u256::zero());
    /// r.randomize();
    /// println!("random number = {}", r);
    /// let yes = r > u256::max() >> 1;
    /// if yes  { assert!(yes) }
    /// else    { assert!(!yes) }
    /// ```
    pub fn randomize(&mut self)
    {
        match T::size_in_bytes()
        {
            1 => {
                    let mut common = IntUnion::new();
                    for i in 0..N
                    {
                        common.set(OsRng.next_u32());
                        self.set_num_(i, T::u8_as_Uint(common.get_ubyte_(0)));
                    }
                },
            2 => {
                    let mut common = IntUnion::new();
                    for i in 0..N
                    {
                        common.set(OsRng.next_u32());
                        self.set_num_(i, T::u16_as_Uint(common.get_ushort_(0)));
                    }
                },
            4 => {
                    for i in 0..N
                        { self.set_num_(i, T::u32_as_Uint(OsRng.next_u32())); }
                },
            8 => {
                    for i in 0..N
                        { self.set_num_(i, T::u64_as_Uint(OsRng.next_u64())); }
                },
            16 => {
                    for i in 0..N
                    {
                        let mut common = LongerUnion::new();
                        common.set_ulong_(0, OsRng.next_u64());
                        common.set_ulong_(1, OsRng.next_u64());
                        self.set_num_(i, T::u128_as_Uint(common.get()));
                    }
                },
            _ => { self.set_zero() },
        }
    }

    // pub fn is_prime_Miller_Rabin(&self, repetition: usize) -> bool
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
    /// # Example
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with_u128;
    /// 
    /// define_utypes_with_u128!();
    /// let num = u1024::random();
    /// let yes = num.is_prime_Miller_Rabin(5);
    /// println!("Is {} a random number? => {}", num, yes);
    /// if yes  { assert!(yes); }
    /// else    { assert!(!yes); }
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn is_prime_Miller_Rabin(&self, repetition: usize) -> bool
    {
        if self.is_zero_or_one()
            { return false; }
        
        if self.is_uint(T::u8_as_Uint(2)) ||  self.is_uint(T::u8_as_Uint(3))
            { return true; }

        // n-1 = (2^s) * d 로 표현하기 위한 과정
        let mut d = self.wrapping_sub_uint(T::one());
        let self_minus_one = self.wrapping_sub_uint(T::one());
        d.shift_right_assign(d.trailing_zeros());
        for _ in 0..repetition
        {
            let mut rand_num = Self::random_less_than(&self.wrapping_sub_uint(T::u8_as_Uint(4))).wrapping_add_uint(T::u8_as_Uint(2));
            let mut x = rand_num.pow(&d).wrapping_rem(self);
            if x.is_one() || x == self_minus_one
                { continue; }

            while d != self_minus_one
            {
                x = x.wrapping_mul(&x);
                x.wrapping_rem_assign(self);
                d.wrapping_mul_assign_uint(T::u8_as_Uint(2));

                if x.is_one()
                    { return false; }
                if x == self_minus_one
                    { break; }
            }
            if x != self_minus_one
                { return false; }
        }
        true
    }



    /***** METHODS TO GET SIZE BOTH IN BYTES AND BITS *****/

    // pub fn size_in_bytes() -> usize
    /// Returns how many bytes long the number `BigUInt` is.
    /// 
    /// # Output
    /// It returns its size in bytes.
    /// 
    /// # Feature
    /// It does not count how many bytes are used for flags.
    /// 
    /// # Examples
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// println!("u256 is {}-byte integer.", u256::size_in_bytes());
    /// assert_eq!(u256::size_in_bytes(), 256 / 8);
    /// ```
    #[inline]
    pub fn size_in_bytes() -> usize
    {
        T::size_in_bytes() * N
    }

    // pub fn size_in_bits() -> usize
    /// Returns how many bits long the number `BigUInt` is.
    /// 
    /// # Output
    /// It returns its size in bits.
    /// 
    /// # Feature
    /// It does not count how many bytes are used for flags.
    /// 
    /// # Examples
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// println!("u256 is {}-bit integer.", u256::size_in_bits());
    /// assert_eq!(u256::size_in_bits(), 256);
    /// ```
    #[inline]
    pub fn size_in_bits() -> usize
    {
        Self::size_in_bytes() * 8
    }

    // pub fn length_in_bytes(&self) -> usize
    /// Returns how many bytes long the number i.e. the object of
    /// `BigUInt` is.
    /// 
    /// # Output
    /// It returns its size in bytes.
    /// 
    /// # Feature
    /// It does not count how many bytes are used for flags.
    /// 
    /// # Examples
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let bi = u256::from_str_radix("A16F", 16).unwrap();
    /// println!("bi is {}-byte integer.", bi.length_in_bytes());
    /// assert_eq!(bi.length_in_bytes(), 256 / 8);
    /// ```
    #[inline]
    pub fn length_in_bytes(&self) -> usize
    {
        Self::size_in_bytes()
    }

    // pub fn length_in_bits(&self) -> usize
    /// Returns how many bits long the number i.e. the object of
    /// `BigUInt` is.
    /// 
    /// # Output
    /// It returns its size in bits.
    /// 
    /// # Feature
    /// It does not count how many bytes are used for flags.
    /// 
    /// # Examples
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let bi = u256::from_str_radix("A16F", 16).unwrap();
    /// println!("bi is {}-bit integer.", bi.length_in_bits());
    /// assert_eq!(bi.length_in_bits(), 256);
    /// ```
    #[inline]
    pub fn length_in_bits(&self) -> usize
    {
        Self::size_in_bits()
    }


    /***** METHODS TO GET, SET, AND CHECK *****/

    // pub fn turn_check_bits(&mut self, bit_pos: usize)
    /// Changes a `BigUInt<T, N>` to have the value zero and sets only
    /// the bit specified by the argument bit_pos to be 1.
    /// 
    /// # Bit Position
    /// The bit positon bit_pos is zero-based and should be counted from LSB
    /// (Least Significant Bit) reguardless endian. So, if the bit_pos is `0`,
    /// only LSB is set to be `1` and all the other bits will be set to `0`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with_u8;
    /// 
    /// define_utypes_with_u8!();
    /// let mut a = u256::random();
    /// a.turn_check_bits(12);
    /// println!("a = {}", a.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a, u256::from_str_radix("10000_00000000", 2).unwrap());
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn turn_check_bits(&mut self, bit_pos: usize)
    {
        let TSIZE_BITS = T::size_in_bits();
        let chunk_num = bit_pos / TSIZE_BITS;
        let piece_num = bit_pos % TSIZE_BITS;
        let mut val = T::one();
        val <<= T::usize_as_Uint(piece_num);
        self.set_zero();
        self.set_num_(chunk_num, val);
    }

    // pub fn get_num(&self, i: usize) -> Option<T>
    /// Returns i-th element of its array of type `T` wrapped in Some
    /// of enum Option if `i` < `N`. Otherwise, it returns `None`.
    /// 
    /// # Argument i
    /// 0-th element contains LSB (Least Significant Bit), while (N-1)-th
    /// element contains MSB (Most Significant Bit) regardless endianness.
    /// `BigUInt` have an array of type `T` in order to present long-sized
    /// unsigned integer.
    /// 
    /// # Error
    /// If `i` >= `N`, it returns `None`.
    /// 
    /// # Counterpart Method
    /// When you are sure that `i` < `N`, you may want to use its counterpart
    /// method [get_num_()](struct@BigUInt#method.get_num_) for performance.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with_u32;
    /// 
    /// define_utypes_with_u32!();
    /// let a = u256::from([0_u32, 10, 20, 30, 40, 50, 60, 70]);
    /// let e = a.get_num(3);
    /// match e
    /// {
    ///     Some(num) => {
    ///             println!("a.get_num(3).unwrap() = {}", num);
    ///             assert_eq!(num, 30);
    ///         },
    ///     None => { println!("There is no third element."); },
    /// }
    /// ```
    #[cfg(target_endian = "little")]
    pub fn get_num(&self, i: usize) -> Option<T>
    {
        if i < N
            { Some(self.get_number()[i]) }
        else
            { None }
    }

    // pub fn get_num(&self, i: usize) -> Option<T>
    /// Returns i-th element of its array of type `T` wrapped in Some
    /// of enum Option if `i` < `N`. Otherwise, it returns `None`. 
    /// 
    /// # Argument i
    /// 0-th element contains LSB (Least Significant Bit), while (N-1)-th
    /// element contains MSB (Most Significant Bit) regardless endianness.
    /// `BigUInt` have an array of type `T` in order to present long-sized
    /// unsigned integer.
    /// 
    /// # Error
    /// If `i` >= `N`, it returns `None`.
    /// 
    /// # Counterpart Method
    /// When you are sure that `i` < `N`, you may want to use its counterpart
    /// method [get_num_()](struct@BigUInt#method.get_num_) for performance.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with_u32;
    /// 
    /// define_utypes_with_u32!();
    /// let a = u256::from([0_u32, 10, 20, 30, 40, 50, 60, 70]);
    /// let e = a.get_num(3);
    /// match e
    /// {
    ///     Some(num) => {
    ///             println!("a.get_num(3).unwrap() = {}", num);
    ///             assert_eq!(num, 30);
    ///         },
    ///     None => { println!("There is no third element."); },
    /// }
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[cfg(target_endian = "big")]
    pub fn get_num(&self, i: usize) -> Option<T>
    {
        if i < N
            { Some(self.get_number()[N-1-i]) }
        else
            { None }
    }

    // pub fn get_num_(&self, i: usize) -> T
    /// Returns i-th element of its array of type `T` if `i` < `N`.
    /// Otherwise, it will panic.
    /// 
    /// # Argument i
    /// 0-th element contains LSB (Least Significant Bit), while (N-1)-th
    /// element contains MSB (Most Significant Bit) regardless endianness.
    /// `BigUInt` have an array of type `T` in order to present long-sized
    /// unsigned integer.
    /// 
    /// # Panics
    /// This method is performance-oriented and does not care for safety.
    /// So, if `i` >= `N`, it will panic.
    /// 
    /// # Counterpart Method
    /// Use this method only when you are sure that `i` < `N`.
    /// Otherwise, use its Counterpart method
    /// [get_num()](struct@BigUInt#method.get_num) for safety.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with_u32;
    /// 
    /// define_utypes_with_u32!();
    /// let a = u256::from([0_u32, 10, 20, 30, 40, 50, 60, 70]);
    /// let b = a.get_num_(3);
    /// println!("a.get_num_(3) = {}", b);
    /// assert_eq!(b, 30);
    /// // It will panic.
    /// // let c = a.get_num_(8);
    /// ```
    #[inline]
    #[cfg(target_endian = "little")]
    pub fn get_num_(&self, i: usize) -> T
    {
        self.number[i]
    }

    // pub fn get_num_(&self, i: usize) -> T
    /// Returns i-th element of its array of type `T` if `i` < `N`.
    /// Otherwise, it will panic.
    /// 
    /// # Argument i
    /// 0-th element contains LSB (Least Significant Bit), while (N-1)-th
    /// element contains MSB (Most Significant Bit) regardless endianness.
    /// `BigUInt` have an array of type `T` in order to present long-sized
    /// unsigned integer.
    /// 
    /// # Panics
    /// This method is performance-oriented and does not care for safety.
    /// So, if `i` >= `N`, it will panic.
    /// 
    /// # Counterpart Method
    /// Use this method only when you are sure that `i` < `N`.
    /// Otherwise, use its Counterpart method
    /// [get_num()](struct@BigUInt#method.get_num) for safety.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with_u32;
    /// 
    /// define_utypes_with_u32!();
    /// let a = u256::from([0_u32, 10, 20, 30, 40, 50, 60, 70]);
    /// let b = a.get_num_(3);
    /// println!("a.get_num_(3) = {}", b);
    /// assert_eq!(b, 30);
    /// // It will panic.
    /// // let c = a.get_num_(8);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    #[cfg(target_endian = "big")]
    pub fn get_num_(&self, i: usize) -> T
    {
        self.number[N-1-i]
    }

    // pub fn set_num(&mut self, i: usize, val: T) -> bool
    /// Sets i-th element of its array of type `T`, and return `true`
    /// if `i` < `N`. Otherwise, it sets none of the elements of its
    /// array of type `T`, and returns `false`.
    ///  
    /// # Argument i
    /// 0-th element contains LSB (Least Significant Bit), while (N-1)-th
    /// element contains MSB (Most Significant Bit) regardless endianess.
    /// `BigUInt` have an array of type `T` in order to present long-sized
    /// unsigned integer.
    /// 
    /// # Error
    /// If `i` >= `N`, it will return `false`.
    /// 
    /// # Counterpart Method
    /// When you are sure that `i` < `N`, you may want to use its Counterpart
    /// method [set_num_()](struct@BigUInt#method.set_num_) for performance.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with_u32;
    /// 
    /// define_utypes_with_u32!();
    /// let mut a = u256::from([0_u32, 10, 20, 30, 40, 50, 60, 70]);
    /// let mut e = a.get_num(3);
    /// match e
    /// {
    ///     Some(num) => {
    ///             println!("a.get_num(3).unwrap() = {}", num);
    ///             assert_eq!(num, 30);
    ///         },
    ///     None => { println!("There is no third element."); },
    /// }
    /// let succ = a.set_num(3, 300);
    /// if succ
    /// {
    ///     let num = a.get_num_(3);
    ///     println!("a.get_num_(3) = {}", num);
    ///     assert_eq!(num, 300);
    /// }
    /// else
    /// {
    ///     println!("There is no third element.");
    /// }
    /// ```
    #[cfg(target_endian = "little")]
    pub fn set_num(&mut self, i: usize, val: T) -> bool
    {
        if i < N
        {
            self.number[i] = val;
            true
        }
        else
        {
            false
        }
    }

    /// Sets i-th element of its array of type `T`, and return `true`
    /// if `i` < `N`. Otherwise, it sets none of the elements of its
    /// array of type `T`, and returns `false`.
    ///  
    /// # Argument i
    /// 0-th element contains LSB (Least Significant Bit), while (N-1)-th
    /// element contains MSB (Most Significant Bit) regardless endianness.
    /// `BigUInt` have an array of type `T` in order to present long-sized
    /// unsigned integer.
    /// 
    /// # Error
    /// If `i` >= `N`, it will return `false`.
    /// 
    /// # Counterpart Method
    /// When you are sure that `i` < `N`, you may want to use its Counterpart
    /// method [set_num_()](struct@BigUInt#method.set_num_) for performance.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with_u32;
    /// 
    /// define_utypes_with_u32!();
    /// let mut a = u256::from([0_u32, 10, 20, 30, 40, 50, 60, 70]);
    /// let mut e = a.get_num(3);
    /// match e
    /// {
    ///     Some(num) => {
    ///             println!("a.get_num(3).unwrap() = {}", num);
    ///             assert_eq!(num, 30);
    ///         },
    ///     None => { println!("There is no third element."); },
    /// }
    /// let succ = a.set_num(3, 300);
    /// if succ
    /// {
    ///     let num = a.get_num_(3);
    ///     println!("a.get_num_(3) = {}", num);
    ///     assert_eq!(num, 300);
    /// }
    /// else
    /// {
    ///     println!("There is no third element.");
    /// }
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[cfg(target_endian = "big")]
    pub fn set_num(&mut self, i: usize, val: T) -> bool
    {
        if i < N
        {
            self.number[N-1-i] = val;
            true
        }
        else
        {
            false
        }
    }

    // pub fn set_num_(&mut self, i: usize, val: T)
    /// Sets i-th element of its array of type `T` if `i` < `N`.
    /// Otherwise, it will panic.
    ///  
    /// # Argument i
    /// 0-th element contains LSB (Least Significant Bit), while (N-1)-th
    /// element contains MSB (Most Significant Bit) regardless endianness.
    /// `BigUInt` have an array of type `T` in order to present long-sized
    /// unsigned integer.
    /// 
    /// # Panics
    /// If `i` >= `N`, it will panic.
    /// 
    /// # Counterpart Method
    /// It is performance-oriented and does not care for safety.
    /// It is virtually the same as the method set_num(). This method set_num_()
    /// is considered to be slightly faster than the method set_num().
    /// Use this method set_num_() only when you are sure that `i` < `N`.
    /// Otherwise, use its Counterpart method [set_num()](struct@BigUInt#method.set_num).
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with_u32;
    /// 
    /// define_utypes_with_u32!();
    /// let mut a = u256::from([0_u32, 10, 20, 30, 40, 50, 60, 70]);
    /// let mut num = a.get_num_(3);
    /// println!("a.get_num_(3) = {}", num);
    /// assert_eq!(num, 30);
    /// 
    /// a.set_num_(3, 300);
    /// num = a.get_num_(3);
    /// println!("a.get_num_(3) = {}", num);
    /// assert_eq!(num, 300);
    /// ```
    #[inline]
    #[cfg(target_endian = "little")]
    pub fn set_num_(&mut self, i: usize, val: T)
    {
        self.number[i] = val;
    }

    // pub fn set_num_(&mut self, i: usize, val: T)
    /// Sets i-th element of its array of type `T` if `i` < `N`.
    /// Otherwise, it will panic.
    ///  
    /// # Argument i
    /// 0-th element contains LSB (Least Significant Bit), while (N-1)-th
    /// element contains MSB (Most Significant Bit) regardless endianness.
    /// `BigUInt` have an array of type `T` in order to present long-sized
    /// unsigned integer.
    /// 
    /// # Panics
    /// If `i` >= `N`, it will panic.
    /// 
    /// # Counterpart Method
    /// It is performance-oriented and does not care for safety.
    /// It is virtually the same as the method set_num(). This method set_num_()
    /// is considered to be slightly faster than the method set_num().
    /// Use this method set_num_() only when you are sure that `i` < `N`.
    /// Otherwise, use its Counterpart method [set_num()](struct@BigUInt#method.set_num).
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with_u32;
    /// 
    /// define_utypes_with_u32!();
    /// let mut a = u256::from([0_u32, 10, 20, 30, 40, 50, 60, 70]);
    /// let mut num = a.get_num_(3);
    /// println!("a.get_num_(3) = {}", num);
    /// assert_eq!(num, 40);
    /// 
    /// a.set_num_(3, 400);
    /// num = a.get_num_(3);
    /// println!("a.get_num_(3) = {}", num);
    /// assert_eq!(num, 400);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    #[cfg(target_endian = "big")]
    pub fn set_num_(&mut self, i: usize, val: T)
    {
        self.number[N-1-i] = val;
    }

    // pub fn get_number(&self) -> &[T; N]
    /// Returns the reference of its array of `T`-type for borrowing instead
    /// of giving its ownership. `BigUInt` has an array of `T` in order
    /// to present long-sized unsigned integers.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// let a = "12345678909876543210123456789098765432101234567890987654321012345678909876543210".parse::<u256>().unwrap();
    /// let arr = a.get_number();
    /// println!("arr = {:?}", arr);
    /// assert_eq!(arr, &[1524178666_u32, 777431351, 1787851831, 3605297539, 2895800654, 97228082, 1118990153, 2660148093]);
    /// ```
    #[inline]
    pub fn get_number(&self) -> &[T; N]
    {
        &self.number
    }

    // pub fn get_number_mut(&self) -> &mut [T; N]
    /// Returns the reference of its array of `T`-type for borrowing instead
    /// of giving its ownership. `BigUInt` has an array of `T` in order
    /// to present long-sized unsigned integers.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// let a = "12345678909876543210123456789098765432101234567890987654321012345678909876543210".parse::<u256>().unwrap();
    /// let arr = a.get_number_mut();
    /// println!("arr = {:?}", arr);
    /// assert_eq!(arr, &[1524178666_u32, 777431351, 1787851831, 3605297539, 2895800654, 97228082, 1118990153, 2660148093]);
    /// ```
    /// 
    /// # Big-endian issue
    /// Only for big-endian compatible.
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for serious purpose. Only use this crate for Big-endian CPUs
    /// with your own full responsibility.
    #[inline]
    #[cfg(target_endian = "big")]
    pub fn get_number_mut(&self) -> &mut [T; N]
    {
        &mut self.number
    }

    // pub fn set_number(&mut self, val: &[T; N])
    /// Sets the contents of its array of `T`-type. The argument val is the
    /// reference of array of type `T` with the length `N`. `BigUInt` have an
    /// array of `T` in order to present long-sized unsigned integer.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// let mut a = u256::new();
    /// a.set_number(&[1_u16, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    /// println!("arr = {:?}", a);
    /// assert_eq!(a.get_number(), &[1_u16, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    /// ```
    #[inline]
    pub fn set_number(&mut self, val: &[T; N])
    {
        self.number.copy_from_slice(val);
    }

    // pub fn copy_within<R>(&mut self, src: R, dest: usize)
    /// Copies elements from one part of the slice `T`-array of BigUInt to
    /// another part of itself, using a memmove.
    /// 
    /// # Arguments
    /// - src is the range within self.number to copy from. Regardless
    /// endianness, the index is counted from LSB (Least Significant Bit) to MSB
    /// (Most Significant Bit). So, index 0 is LSB and index N-1 is MSB.
    /// - dest is the starting index of the range within self.number to copy to,
    /// which will have the same length as src.
    /// - The two ranges may overlap.
    /// - The ends of the two ranges must be less than or equal to self.len().
    /// 
    /// # Panics
    /// This method copy_within() will panic if either range exceeds the end
    /// of the slice, or if the end of src is before the start.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// use Cryptocol::number::BigUInt;
    /// define_utypes_with!(u16);
    /// let mut a = u256::new();
    /// a.set_number(&[0_u16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    /// println!("a = {:?}", a);
    /// a.copy_within(3..10, 6);
    /// println!("a = {:?}", a);
    /// assert_eq!(a.get_number(), &[0, 1, 2, 3, 4, 5, 3, 4, 5, 6, 7, 8, 9, 13, 14, 15]);
    /// ```
    #[inline]
    #[cfg(target_endian = "little")]
    pub fn copy_within<R>(&mut self, src: R, dest: usize)
    where R: RangeBounds<usize>
    {
        self.number.copy_within(src, dest);
    }

    // pub fn copy_within<R>(&mut self, src: R, dest: usize)
    /// Copies elements from one part of the slice `T`-array of BigUInt to
    /// another part of itself, using a memmove.
    /// 
    /// # Arguments
    /// - src is the range within self.number to copy from. Regardless
    /// endianness, the index is counted from LSB (Least Significant Bit) to MSB
    /// (Most Significant Bit). So, index 0 is LSB and index N-1 is MSB. if you
    /// want to use index Big-endian way, in other words, index 0 is MSB and
    /// index N-1 is LSB, use the method [_copy_within()](struct@BigUInt#method._copy_within)
    /// instead of this method copy_within().
    /// - dest is the starting index of the range within self.number to copy to,
    /// which will have the same length as src.
    /// - The two ranges may overlap.
    /// - The ends of the two ranges must be less than or equal to self.len().
    /// 
    /// # Panics
    /// This method copy_within() will panic if either range exceeds the end
    /// of the slice, or if the end of src is before the start.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// use Cryptocol::number::BigUInt;
    /// define_utypes_with!(u16);
    /// let mut a = u256::new();
    /// a.set_number(&[0_u16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    /// println!("a = {:?}", a);
    /// a.copy_within(3..10, 6);
    /// println!("a = {:?}", a);
    /// assert_eq!(a.get_number(), &[0, 1, 2, 6, 7, 8, 9, 10, 11, 12, 10, 11, 12, 13, 14, 15]);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    #[cfg(target_endian = "big")]
    pub fn copy_within<R>(&mut self, src: R, dest: usize)
    where R: RangeBounds<usize>
    {
        let mut start: usize;
        let mut end: usize;

        match src.end_bound()
        {
            Excluded(s) =>  { start = (N - s); },
            Included(s) =>  { start = (N - 1 - s); },
            Unbounded =>    { start = 0; }
        }
        match src.start_bound()
        {
            Excluded(s) =>  { end = (N - s); },
            Included(s) =>  { end = (N - 1 - s); },
            Unbounded =>    { end = N - 1; }
        }
        let new_src = Range::<&usize> { start: &start, end: &end };
        let new_dest = N - 1 - dest;
        self.number.copy_within(new_src, new_dest);
    }

    // pub fn set_zero(&mut self)
    /// Sets `BigUInt` to be zero.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::BigUInt;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// let mut a = u256::new();
    /// a.set_number(&[1_u16, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    /// println!("a = {}", a);
    /// a.set_zero();
    /// println!("a = {}", a);
    /// assert_eq!(a, u256::zero());
    /// ```
    pub fn set_zero(&mut self)
    {
        for i in 0..N
            { self.set_num(i, T::zero()); }
    }

    // pub fn is_zero(&self) -> bool
    /// Checks whether `BigUInt` to be zero and returns true if it is
    /// zero and returns false if it is not zero.
    /// 
    /// # Output
    /// It returns true if it is zero. Otherwise, it returns false.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::BigUInt;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let a = u1024::zero();
    /// if a.is_zero()
    ///     { println!("a is Zero"); }
    /// else
    ///     { println!("a is Not Zero"); }
    /// assert!(a.is_zero());
    /// ```
    pub fn is_zero(&self) -> bool
    {
        for i in 0..N
        {
            if self.get_num_(i) != T::zero()
                { return false; }
        }
        true
    }

    // pub fn set_one(&mut self)
    /// Sets `BigUInt` to be one.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::BigUInt;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// let mut a = u256::new();
    /// a.set_number(&[1_u16, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    /// println!("a = {}", a);
    /// a.set_one();
    /// println!("a = {}", a);
    /// assert_eq!(a, u256::one());
    /// ```
    #[cfg(target_endian = "little")]
    pub fn set_one(&mut self)
    {
        for i in 1..N
            { self.set_num(i, T::zero()); }
        self.set_num(0, T::one());
    }

    // pub fn set_one(&mut self)
    /// Sets BigUInt to be one.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::BigUInt;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// let mut a = u256::new();
    /// a.set_number(&[1_u16, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    /// println!("a = {}", a);
    /// a.set_one();
    /// println!("a = {}", a);
    /// assert_eq!(a, u256::one());
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[cfg(target_endian = "big")]
    pub fn set_one(&mut self)
    {
        for i in 0..N-1
            { self.set_num(i, T::zero()); }
        self.set_num(N-1, T::one());
    }

    // pub fn is_one(&self) -> bool
    /// Checks whether `BigUInt` to be one and returns true if it is
    /// one, and returns false if it is not one.
    /// 
    /// # Output
    /// It returns `true` if it is one. Otherwise, it returns `false`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::BigUInt;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let a = u1024::one();
    /// if a.is_one()
    ///     { println!("a is One"); }
    /// else
    ///     { println!("a is Not One"); }
    /// assert!(a.is_one());
    /// ```
    pub fn is_one(&self) -> bool
    {
        if self.get_num_(0) != T::one()
            { return false; }

        for i in 1..N
        {
            if self.get_num_(i) != T::zero()
                { return false; }
        }
        true
    }

    // pub fn is_zero_or_one(&self) -> bool
    /// Checks whether `BigUInt` to be either zero or one and returns true if it
    /// is either zero or one. Otherwise, it returns false.
    /// 
    /// # Output
    /// It returns true if it is either zero or one. Otherwise, it returns false.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::BigUInt;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let a = u1024::one();
    /// if a.is_zero_or_one()
    ///     { println!("a is One or Zero."); }
    /// else
    ///     { println!("a is Neither One nor Zero."); }
    /// assert!(a.is_zero_or_one());
    /// ```
    pub fn is_zero_or_one(&self) -> bool
    {
        if self.get_num_(0) > T::one()
            { return false; }

        for i in 1..N
        {
            if self.get_num_(i) != T::zero()
                { return false; }
        }
        true
    }

    // pub fn set_max(&mut self)
    /// Sets `BigUInt`-type number to be maximum value in which all bits are
    /// set to be `1`.
    /// 
    /// # Examples
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let mut a = u256::new();
    /// a.set_max();
    /// println!("a = {}", a);
    /// ```
    pub fn set_max(&mut self)
    {
        for i in 0..N
            { self.set_num(i, T::max()); }
    }

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
    /// define_utypes_with!(u128);
    /// let mut a = u256::new();
    /// a.set_max();
    /// println!("a = {}", a);
    /// assert_eq!(a, u256::max());
    /// a.set_submax(128_usize);
    /// println!("a = {}", a);
    /// assert_eq!(a, u256::submax(128_usize));
    /// ```
    pub fn set_submax(&mut self, size_in_bits: usize)
    {
        let TSIZE_IN_BITS = T::size_in_bits();
        if size_in_bits >= self.length_in_bits()
        {
            self.set_max();
            return;
        }
        else if size_in_bits == 0
        {
            self.set_zero();
            return;
        }

        let chunk_num = size_in_bits / TSIZE_IN_BITS;
        let piece_num = size_in_bits % TSIZE_IN_BITS;
        let zero = T::zero();
        let max = T::max();
        self.reset_all_flags();
        for i in 0..chunk_num
            { self.set_num_(i, max); }
        for i in chunk_num..N
            { self.set_num_(i, zero); }
        if piece_num != 0
            { self.set_num_(chunk_num, max >> T::usize_as_Uint(TSIZE_IN_BITS - piece_num)); }
    }

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
    /// define_utypes_with!(u128);
    /// let mut a = u256::new();
    /// a.set_max();
    /// println!("a = {}", a);
    /// assert_eq!(a, u256::max());
    /// a.set_halfmax();
    /// println!("a = {}", a);
    /// assert_eq!(a, u256::halfmax());
    /// ```
    #[inline]
    pub fn set_halfmax(&mut self)
    {
        self.set_submax(self.length_in_bits() >> 1);
    }

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
    /// let a = u256::max();
    /// println!("Is a maximun? - {}", a.is_max());
    /// assert_eq!(a.is_max(), true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn is_max(&self) -> bool
    {
        for i in 0..N
        {
            if self.get_num_(i) != T::max()
                { return false; }
        }
        true
    }

    // pub fn set_MSB(&mut self)
    /// Sets the MSB (Most Significant Bit) of `BigUInt`-type number with `1`.
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn set_MSB(&mut self)
    {
        let highest = self.get_num_(N-1);
        let msb = !(T::max() >> T::one());
        self.set_num_(N-1, highest | msb);
    }

    // pub fn set_LSB(&mut self)
    /// Sets the LSB (Least Significant Bit) of `BigUInt`-type number with `1`.
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn set_LSB(&mut self)
    {
        let lowest = self.get_num_(0);
        let lsb = T::one();
        self.set_num_(0, lowest | lsb);
    }

    // pub fn set_uint(&mut self, val: T)
    /// Sets `BigUInt`-type number with `T`-type small value such as `u8`,
    /// `u16`, `u32`, `u64`, and `u128` type value. This mathod set_uint()
    /// is useful especially when you initialize `BigUInt`-type big
    /// unsigned integer with a small value.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// let mut a = u1024::new();
    /// a.set_uint(25);
    /// let b = u1024::from(25_u8);
    /// println!("a = {}", a);
    /// assert_eq!(a, b);
    /// ```
    pub fn set_uint(&mut self, val: T)
    {
        self.set_zero();
        self.set_num_(0, val);
    }

    // pub fn set_uint(&mut self, val: T)
    /// Sets `BigUInt`-type number with `T`-type small value such as `u8`,
    /// `u16`, `u32`, `u64`, and `u128` type value. This mathod set_uint()
    /// is useful especially when you initialize `BigUInt`-type big
    /// unsigned integer with a small value.
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// let mut a = u1024::new();
    /// a.set_uint(25);
    /// let b = u1024::from(25_u8);
    /// println!("a = {}", a);
    /// assert_eq!(a, b);
    /// ```
    #[cfg(target_endian = "big")]
    pub fn _set_uint(&mut self, val: T)
    {
        self.set_zero();
        self._set_num_(N-1, val);
    }

    // pub fn is_uint(&self, val: T) -> bool
    /// Check whether the `BigUInt`-type number is equal to `T`-type number.
    /// It will return `true`, if it is equal to the `T`-type number. Otherwise,
    /// it will return `false`.
    /// 
    /// # Output
    /// It will return `true`, if it is equal to val.
    /// Otherwise, it will return `false`.
    /// 
    /// # Counterpart Method
    /// This method is_uint() is virtually the same the method [eq_uint()](struct@BigUInt#method.eq_uint).
    /// However, you may want to use this method is_uint() rather than [eq_uint()](struct@BigUInt#method.eq_uint),
    /// if you know that this method is_uint() is a bit faster than [eq_uint()](struct@BigUInt#method.eq_uint),
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let mut a = u1024::new();
    /// a.set_uint(25);
    /// if a.is_uint(25u128)   { println!("They are the same."); }
    /// else                   { println!("They are differnt."); }
    /// assert!(a.is_uint(25_u128));
    /// ```
    pub fn is_uint(&self, val: T) -> bool
    {
        if self.get_num_(0) != val
        {
            false
        }
        else
        {
            for i in 1..N
            {
                if self.get_num_(i) != T::zero()
                    { return false; }
            }
            true
        }
    }

    // pub fn is_odd(&self) -> bool
    /// Checks whether the `BigUInt`-type number is an odd number.
    /// 
    /// # Output
    /// It will return `true`, if it is odd. Otherwise, it will return `false`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let mut a = u1024::new();
    /// a.set_uint(25);
    /// if a.is_odd()
    ///     { println!("a is odd"); }
    /// else
    ///     { println!("a is even"); }
    /// assert!(a.is_odd());
    /// ```
    #[inline]
    pub fn is_odd(&self) -> bool
    {
        self.get_num_(0).is_odd()
    }

    // pub fn is_even(&self) -> bool
    /// Checks whether the `BigUInt`-type number is an even number.
    /// 
    /// # Output
    /// It will return `true`, if it is even. Otherwise, it will return `false`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let mut a = u1024::new();
    /// a.set_uint(24);
    /// if a.is_even()
    ///     { println!("a is even"); }
    /// else
    ///     { println!("a is odd"); }
    /// assert!(a.is_even());
    /// ```
    #[inline]
    pub fn is_even(&self) -> bool
    {
        !self.is_odd()
    }


    /***** METHODS TO CHECK BITS *****/

    // pub fn count_ones(&self) -> u32
    /// Returns the number of ones in the binary representation
    /// of the array number[T;N] of `self`.
    /// 
    /// # Output
    /// It returns the total number of the bits that are set to be one.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn count_ones(&self) -> u32
    {
        let mut res = 0_u32;
        for i in 0..N
            { res += self.number[i].count_ones(); }
        res
    }

    // pub fn count_zeros(&self) -> u32
    /// Returns the number of zeros in the binary representation
    /// of the array number[T;N] of `self`.
    /// 
    /// # Output
    /// It returns the total number of the bits that are set to be zero.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn count_zeros(&self) -> u32
    {
        let mut res = 0_u32;
        for i in 0..N
            { res += self.number[i].count_zeros(); }
        res
    }

    // pub fn leading_ones(&self) -> u32
    /// Returns the number of leading ones in the binary representation
    /// of the array number[T;N] of `self`.
    /// 
    /// # Output
    /// It returns the total number of the leading bits that are set to be one.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn leading_ones(&self) -> u32
    {
        let mut res = 0_u32;
        let mut i = N-1;
        while i != 0
        {
            if self.get_num_(i) == T::max()
                { res += T::size_in_bits().into_u32(); }
            else
                { return res + self.get_num_(i).leading_ones(); }
            i -= 1;
        }
        res + self.get_num_(0).leading_ones()
    }

    // pub fn leading_zeros(&self) -> u32
    /// Returns the number of leading zeros in the binary representation
    /// of the array number[T;N] of `self`.
    /// 
    /// # Output
    /// It returns the total number of the leading bits that are set to be zero.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn leading_zeros(&self) -> u32
    {
        let mut res = 0_u32;
        let mut i = N-1;
        while i != 0
        {
            if self.get_num_(i) == T::zero()
                { res += T::size_in_bits().into_u32(); }
            else
                { return res + self.get_num_(i).leading_zeros(); }
            i -= 1;
        }
        res + self.get_num_(0).leading_zeros()
    }

    // pub fn trailing_ones(&self) -> u32
    /// Returns the number of trailing ones in the binary representation
    /// of the array number[T;N] of `self`.
    /// 
    /// # Output
    /// It returns the total number of the trailing bits that are set to be one.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn trailing_ones(&self) -> u32
    {
        let mut res = 0_u32;
        for i in 0..N
        {
            if self.get_num_(i) == T::max()
            {
                res += T::size_in_bits().into_u32();
            }
            else
            {
                res += self.get_num_(i).trailing_ones();
                break;
            }
        }
        res
    }

    // pub fn trailing_zeros(&self) -> u32
    /// Returns the number of trailing zeros in the binary representation
    /// of the array number[T;N] of `self`.
    /// 
    /// # Output
    /// It returns the total number of the trailing bits that are set to be zero.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn trailing_zeros(&self) -> u32
    {
        let mut res = 0_u32;
        for i in 0..N
        {
            if self.get_num_(i) == T::zero()
            {
                res += T::size_in_bits().into_u32();
            }
            else
            {
                res += self.get_num_(i).trailing_zeros();
                break;
            }
        }
        res
    }

    // pub fn leading_max_elements(&self) -> u32
    /// Returns the number of leading maximum elements in the binary
    /// representation of the array number[T;N] of `self`.
    /// Here, 'maximum element' means the element that has all bits to be one.
    /// 
    /// # Output
    /// It returns the total number of the leading maximum elements
    /// that has all bits set to be one.
    /// Here, 'maximum element' means the element that has all bits to be one. 
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn leading_max_elements(&self) -> u32
    {
        let mut res = 0_u32;
        let mut i = N-1;
        while i != 0
        {
            if self.get_num_(i) == T::max()
                { res += 1; }
            else
                { return res; }
            i -= 1;
        }
        if self.get_num_(0) == T::max()
            { res + 1 }
        else
            { res }
    }

    // pub fn leading_zero_elements(&self) -> u32
    /// Returns the number of leading zero elements in the binary
    /// representation of the array number[T;N] of `self`.
    /// 
    /// # Output
    /// It returns the total number of the leading zero elemments
    /// that are set to be zero.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn leading_zero_elements(&self) -> u32
    {
        let mut res = 0_u32;
        let mut i = N-1;
        while i != 0
        {
            if self.get_num_(i) == T::zero()
                { res += 1; }
            else
                { return res; }
            i -= 1;
        }
        if self.get_num_(0) == T::zero()
            { res + 1 }
        else
            { res }
    }

    // pub fn trailing_max_elements(&self) -> u32
    /// Returns the number of trailing maximum elements in the binary
    /// representation of the array number[T;N] of `self`.
    /// Here, 'maximum element' means the element that has all bits to be one.
    /// 
    /// # Output
    /// It returns the total number of the trailing maximum elemeents
    /// that have all bits set to be one.
    /// Here, 'maximum element' means the element that has all bits to be one.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn trailing_max_elements(&self) -> u32
    {
        let mut res = 0_u32;
        for i in 0..N
        {
            if self.get_num_(i) == T::max()
                { res += 1; }
            else
                { return res; }
        }
        res
    }

    // pub fn trailing_zero_elements(&self) -> u32
    /// Returns the number of trailing zeros in the binary representation
    /// of the array number[T;N] of `self`.
    /// 
    /// # Output
    /// It returns the total number of the trailing bits that are set to be zero.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn trailing_zero_elements(&self) -> u32
    {
        let mut res = 0_u32;
        for i in 0..N
        {
            if self.get_num_(i) == T::zero()
                { res += 1; }
            else
                { return res; }
        }
        res
    }


    /***** METHODS FOR COMPARISON WITH UINT *****/

    // pub fn partial_cmp_uint<U>(&self, other: U) -> Option<Ordering>
    /// Compares BigUInt with a value of type `U` and returns the
    /// result of the comparison in the type `Option<Ordering>`. However,
    /// you'd better use the functions `lt_uint()`, `gt_uint()`, `le_uint()`,
    /// `ge_uint()`, and `eq_uint()`.
    /// Then, you don't have to use `partial_cmp_uint()` directly.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `Ordering::Greater` wrapped by `Some` of enum `Option`
    /// if `self` is greater than `other`.
    /// It returns `Ordering::Less` wrapped by `Some` of enum `Option`
    /// if `self` is less than `other`.
    /// It returns `Ordering::Equal` wrapped by `Some` of enum `Option`
    /// if `self` is equal to `other`.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn partial_cmp_uint<U>(&self, other: U) -> Option<Ordering>
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let t_other: T;
        if other.length_in_bytes() > T::size_in_bytes()
        {
            return self.partial_cmp(&Self::from_uint(other));
        }
        else    // if rhs.length_in_bytes() <= T::size_in_bytes()
        {
            t_other = T::num::<U>(other);
        }

        if self.number[0] > t_other
        {
            return Some(Ordering::Greater);
        }
        else if self.number[0] < t_other
        {
            for idx in 1..N
            {
                if self.number[idx] != T::zero()
                    { return Some(Ordering::Greater); }
            }
            return Some(Ordering::Less);
        }
        else    // if self.number[0] == other
        {
            for idx in 1..N
            {
                if self.number[idx] != T::zero()
                    { return Some(Ordering::Greater); }
            }
        }
        Some(Ordering::Equal)
    }

    // pub fn lt_uint<U>(&self, other: T) -> bool
    /// Returns `true` if `self` is less than `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `true` if `self` is less than `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    pub fn lt_uint<U>(&self, other: U) -> bool
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    { self.partial_cmp_uint(other).unwrap().is_lt() }

    // pub fn gt_uint<U>(&self, other: U) -> bool
    /// Returns `true` if `self` is greater than `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `true` if `self` is greater than `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    pub fn gt_uint<U>(&self, other: U) -> bool
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    { self.partial_cmp_uint(other).unwrap().is_gt() }

    // pub fn le_uint<U>(&self, other: U) -> bool
    /// Returns `true` if `self` is less than or equal to `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `true` if `self` is less than or equal to `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    pub fn le_uint<U>(&self, other: U) -> bool
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    { self.partial_cmp_uint(other).unwrap().is_le() }

    // pub fn ge_uint<U>(&self, other: U) -> bool 
    /// Returns `true` if self is greater than or equal to `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `true` if `self` is greater than or equal to `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    pub fn ge_uint<U>(&self, other: U) -> bool
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    { self.partial_cmp_uint(other).unwrap().is_ge() }

    // pub fn eq_uint<U>(&self, other: U) -> bool
    /// Returns `true` if `self` is equal to `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `true` if `self` is equal to `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    pub fn eq_uint<U>(&self, other: U) -> bool
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    { self.partial_cmp_uint(other).unwrap().is_eq() }

    

    /***** ARITHMATIC OPERATIONS WITH UNSIGNED INTEGERS *****/

    /*** ADDITION ***/

    // pub fn carrying_add_uint<U>(&self, rhs: U, carry: bool) -> (Self, bool)
    /// Calculates `self` + `rhs` + `carry`,
    /// wrapping around at the boundary of the type.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Features
    /// This allows chaining together multiple additions to create even a wider
    /// addition. This can be thought of as a big integer “full adder”,
    /// in the electronics sense.
    /// 
    /// If the input carry is `false`, this method is equivalent to
    /// `overflowing_add_uint()`, and the output carry is equal to
    /// the overflow flag.
    /// 
    /// # Outputs
    /// It returns a tuple containing the sum and the output carry. It performs
    /// “ternary addition” of one big integer operand, a primitive unsigned
    /// integer, and a carry-in bit, and returns an output big integer and a
    /// carry-out bit.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use Cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// // Todo
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn carrying_add_uint<U>(&self, rhs: U, carry: bool) -> (Self, bool)
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut res = self.clone();
        let c = res.carrying_add_assign_uint(rhs, carry);
        (res, c)
    }

    // pub fn carrying_add_assign_uint<U>(&mut self, rhs: U, carry: bool) -> bool
    /// Accumulate `rhs` + `carry` to `self`, wrapping around at the boundary
    /// of the type, and return the resulting carry.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Features
    /// This allows chaining together multiple additions to create even a wider
    /// addition. This can be thought of as a big integer “full adder”,
    /// in the electronics sense.
    /// 
    /// If the input carry is false, this method is equivalent to
    /// `overflowing_add_assign_uint()`, and the output carry is equal to
    /// the overflow flag.
    /// 
    /// # Outputs
    /// It returns the output carry. It performs “ternary addition” of a big
    /// integer operands, primitive unsigned integer, and a carry-in bit,
    /// and returns a carry-out bit.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use Cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// // Todo
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn carrying_add_assign_uint<U>(&mut self, rhs: U, carry: bool) -> bool
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let trhs: T;
        if rhs.length_in_bytes() > T::size_in_bytes()
        {
            return self.carrying_add_assign(&Self::from_uint(rhs), carry);
        }
        else    // if rhs.length_in_bytes() <= T::size_in_bytes()
        {
            trhs = T::num::<U>(rhs);
        }
        
        let mut c: bool;
        let mut num: T;
        (num, c) = self.get_num_(0).carrying_add(trhs, carry);
        self.set_num_(0, num);
        if c
        {
            for i in 1..N
            {
                (num, c) = self.get_num_(i).carrying_add(T::zero(), c);
                self.set_num_(i, num);
                if !c
                    { break; }
            }
        }
        if c
            { self.set_overflow(); }
        c
    }

    // pub fn wrapping_add_uint<U>(&self, rhs: U) -> Self
    /// Computes `self` + `rhs`, wrapping around at the boundary of the type.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `self` + `rhs` with wrapping (modular) addition.
    /// 
    /// # Feature
    /// Wrapping (modular) addition.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    ///  
    /// let zero = u512::zero();
    /// let one = u512::one();
    /// let two = u512::from(2_u8);
    /// let three = u512::from(3_u8);
    /// let a = u512::max().wrapping_sub(&one);
    /// let b = a.wrapping_add_uint(1_u128);
    /// let c = a.wrapping_add_uint(2_u128);
    /// let d = a.wrapping_add_uint(3_u128);
    /// 
    /// println!("{} + 1 = {}", a, b);
    /// assert_eq!(b, u512::max());
    /// 
    /// println!("{} + 2 = {}", a, c);
    /// assert_eq!(c, zero);
    /// println!("{} + 3 = {}", a, d);
    /// assert_eq!(d, one);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn wrapping_add_uint<U>(&self, rhs: U) -> Self
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let trhs: T;
        if rhs.length_in_bytes() > T::size_in_bytes()
        {
            return self.wrapping_add(&Self::from_uint(rhs));
        }
        else    // if rhs.length_in_bytes() <= T::size_in_bytes()
        {
            trhs = T::num::<U>(rhs);
        }
        let (res, _) = self.carrying_add_uint(trhs, false);
        res
    }

    // pub fn wrapping_add_assign_uint<U>(&mut self, rhs: U)
    /// Computes `self` + `rhs`, wrapping around at the boundary of the type,
    /// and assign the result to `self` back.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Feature
    /// Wrapping (modular) addition.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let zero = u512::zero();
    /// let one = u512::one();
    /// 
    /// let mut a = u512::max().wrapping_sub(&one);
    /// println!("Originally,\ta = {}", a);
    /// assert_eq!(a.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    /// 
    /// a.wrapping_add_assign_uint(1_u128);
    /// println!("After a += 1,\ta = {}", a);
    /// assert_eq!(a, u512::max());
    /// 
    /// a.wrapping_add_assign_uint(1_u128);
    /// println!("After a += 1,\ta = {}", a);
    /// assert_eq!(a, zero);
    /// 
    /// a.wrapping_add_assign_uint(1_u128);
    /// println!("After a += 1,\ta = {}", a);
    /// assert_eq!(a, one);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn wrapping_add_assign_uint<U>(&mut self, rhs: U)
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.carrying_add_assign_uint(rhs, false);
    }

    // pub fn overflowing_add_uint<U>(&self, rhs: U) -> (Self, bool)
    /// Calculates `self` + `rhs`.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Output
    /// It returns a tuple of the addition `self` + `rhs` along with a boolean
    /// indicating whether an arithmetic overflow would occur. If an overflow
    /// would have occurred then the wrapped (modular) value is returned.
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
    #[inline]
    pub fn overflowing_add_uint<U>(&self, rhs: U) -> (Self, bool)
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.carrying_add_uint(rhs, false)
    }

    // pub fn overflowing_add_assign_uint<U>(&mut self, rhs: U) -> bool
    /// Calculates `self` + `rhs`, and assigns the result to `self` back.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Output
    /// It returns true if an arithmetic overflow would occur.
    /// Otherwise, it returns `false`.
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
    #[inline]
    pub fn overflowing_add_assign_uint<U>(&mut self, rhs: U) -> bool
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.carrying_add_assign_uint(rhs, false)
    }

    // pub fn checked_add_uint<U>(&self, rhs: U) -> Option<Self>
    /// Computes `self` + `rhs`.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the sum `self` + `rhs` wrapped by `Some` of enum `Option`
    /// if overflow did not occur. Otherwise, it returns `None` of enum Option.
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
    pub fn checked_add_uint<U>(&self, rhs: U) -> Option<Self>
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let trhs: T;
        if rhs.length_in_bytes() > T::size_in_bytes()
        {
            return self.checked_add(&Self::from_uint(rhs));
        }
        else    // if rhs.length_in_bytes() <= T::size_in_bytes()
        {
            trhs = T::num::<U>(rhs);
        }
        let mut res = self.clone();
        let overflow = res.overflowing_add_assign_uint(trhs);
        if overflow
            { None }
        else
            { Some(res) }
    }

    // pub fn unchecked_add_uint<U>(&self, rhs: U) -> Self
    /// Computes `self` + `rhs`, assuming overflow cannot occur.
    /// 
    /// # Panics
    /// If overflow occurred, it will panic. So, use this method only when you
    /// are sure that overflow will not occur. 
    /// 
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the sum `self` + `rhs` if overflow did not occur.
    /// Otherwise, it will panic.
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
    #[inline]
    pub fn unchecked_add_uint<U>(&self, rhs: U) -> Self
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.checked_add_uint(rhs).unwrap()
    }

    // pub fn saturating_add_uint<U>(&self, rhs: U) -> Self
    /// Computes `self` + `rhs`, saturating at the numeric bounds
    /// instead of overflowing.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the sum `self` + `rhs` if overflow did not occur.
    /// Otherwise, it returns the maximum value.
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
    pub fn saturating_add_uint<U>(&self, rhs: U) -> Self
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut res = self.clone();
        res.saturating_add_assign_uint(rhs);
        res
    }

    // pub fn saturating_add_assign_uint<U>(&mut self, rhs: T)
    /// Computes `self` + `rhs`, saturating at the numeric bounds
    /// instead of overflowing, and assigns the result to `self` back.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
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
    pub fn saturating_add_assign_uint<U>(&mut self, rhs: U)
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if self.overflowing_add_assign_uint(rhs)
            { self.set_max(); }
    }


    /*** Subtraction ***/

    // pub fn borrowing_sub_uint<U>(&self, rhs: U, borrow: bool) -> (Self, bool)
    /// Calculates self − rhs − borrow and returns a tuple containing the
    /// difference and the output borrow.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Features
    /// It performs “ternary subtraction” by subtracting a primitive unsigned
    /// integer operand and a borrow-in bit from `self`, and returns an output
    /// integer and a borrow-out bit. This allows chaining together multiple
    /// subtractions to create a wider subtraction.
    /// 
    /// If the input borrow is `false`, this method is equivalent to
    /// `overflowing_sub_uint()`, and the output carry is equal to
    /// the underflow flag.
    /// 
    /// # Outputs
    /// It returns a tuple containing an output big integer and a carry-out bit.
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
    pub fn borrowing_sub_uint<U>(&self, rhs: U, borrow: bool) -> (Self, bool)
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut res = self.clone();
        let b = res.borrowing_sub_assign_uint(rhs, borrow);
        (res, b)
    }

    // pub fn borrowing_sub_assign_uint<U>(&mut self, rhs: U, borrow: bool) -> bool
    /// Calculates self − rhs − borrow, and assigns difference to `self` back,
    /// and returns the output borrow.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Features
    /// It performs “ternary subtraction” by subtracting an primitive unsiged
    /// integer operand and a borrow-in bit from `self`, and a borrow-out bit.
    /// This allows chaining together multiple subtractions to create a wider
    /// subtraction.
    /// 
    /// If the input borrow is `false`, this method is equivalent to
    /// `overflowing_sub_assign_uint()`, and the output carry is equal to
    /// the underflow flag.
    /// 
    /// # Outputs
    /// It returns a tuple containing an output big integer and a carry-out bit.
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
    pub fn borrowing_sub_assign_uint<U>(&mut self, rhs: U, borrow: bool) -> bool
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let trhs: T;
        if rhs.length_in_bytes() > T::size_in_bytes()
        {
            return self.borrowing_sub_assign(&Self::from_uint(rhs), borrow);
        }
        else    // if rhs.length_in_bytes() <= T::size_in_bytes()
        {
            trhs = T::num::<U>(rhs);
        }

        let mut num: T;
        let mut b: bool;
        (num, b) = self.get_num_(0).borrowing_sub(trhs, borrow);
        self.set_num_(0, num);
        if b
        {
            for i in 1..N
            {
                (num, b) = self.get_num_(i).borrowing_sub(T::zero(), b);
                self.set_num_(i, num);
                if !b
                    { break; }
            }
        }
        if b
            { self.set_underflow(); }
        b
    }

    // pub fn wrapping_sub_uint<U>(&self, rhs: U) -> Self
    /// Subtracts a unsigned integer number of type `U` from `BigUInt`-type
    /// unsigned integer and returns its result in a type of BigUInt.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the subtraction of `rhs` from `self`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use std::str::FromStr;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let a = u256::from_str("10000000000000000000000000000000000").unwrap();
    /// let sub = a.wrapping_sub_uint(35_u128);
    /// println!("sub = {}", sub);
    /// assert_eq!(sub.to_string(), "9999999999999999999999999999999965");
    /// ```
    pub fn wrapping_sub_uint<U>(&self, rhs: U) -> Self
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let (res, _) = self.borrowing_sub_uint(rhs, false);
        res
    }

    // pub fn wrapping_sub_assign_uint<U>(&mut self, rhs: U)
    /// Subtracts rhs of type `U` from self which is of `BigUInt` type,
    /// and returns the result to `self` back.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    pub fn wrapping_sub_assign_uint<U>(&mut self, rhs: U)
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.borrowing_sub_assign_uint(rhs, false);
    }

    // pub fn overflowing_sub_uint<U>(&self, rhs: U) -> (Self, bool)
    /// Calculates `self` - `rhs`.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Output
    /// It returns a tuple of the subtraction `self` - `rhs` along with a boolean
    /// indicating whether an arithmetic unerflow would occur. If an unerflow
    /// would have occurred then the wrapped (modular) value is returned.
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
    #[inline]
    pub fn overflowing_sub_uint<U>(&self, rhs: U) -> (Self, bool)
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.borrowing_sub_uint(rhs, false)
    }

    // pub fn overflowing_sub_assign<U>(&mut self, rhs: U) -> bool
    /// Calculates `self` - `rhs`, and assigns the result to `self` back.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Output
    /// It returns true if an arithmetic unerflow would occur.
    /// Otherwise, it returns `false`.
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
    #[inline]
    pub fn overflowing_sub_assign_uint<U>(&mut self, rhs: U) -> bool
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.borrowing_sub_assign_uint(rhs, false)
    }

    // pub fn checked_sub_uint<U>(&self, rhs: U) -> Option<Self>
    /// Computes `self` - `rhs`.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the difference `self` - `rhs` wrapped by `Some`
    /// of enum `Option` if unerflow did not occur.
    /// Otherwise, it returns `None` of enum Option.
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
    pub fn checked_sub_uint<U>(&self, rhs: U) -> Option<Self>
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut res = self.clone();
        let underflow = res.overflowing_sub_assign_uint(rhs);
        if underflow
            { None }
        else
            { Some(res) }
    }

    // pub fn unchecked_sub_uint<U>(&self, rhs: U) -> Self
    /// Computes `self` - `rhs`, assuming underflow cannot occur.
    /// 
    /// # Panics
    /// - If underflow occurred, it will panic. So, use this method only when
    /// you are sure that underflow will not occur.
    /// - If size_of::<T>() * N <= 128, some methods may panic or its behavior
    /// may undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the difference `self` - `rhs` if underflow did not occur.
    /// Otherwise, it will panic.
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
    #[inline]
    pub fn unchecked_sub_uint<U>(&self, rhs: U) -> Self
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.checked_sub_uint(rhs).unwrap()
    }

    // pub fn saturating_sub_uint<U>(&self, rhs: U) -> Self
    /// Computes `self` - `rhs`, saturating at the numeric bounds
    /// instead of underflowing.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the difference `self` - `rhs` if underflowing did not occur.
    /// Otherwise, it returns `0`.
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
    pub fn saturating_sub_uint<U>(&self, rhs: U) -> Self
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut res = self.clone();
        res.saturating_sub_assign_uint(rhs);
        res
    }

    // pub fn saturating_sub_assign_uint<U>(&mut self, rhs: U)
    /// Computes `self` - `rhs`, saturating at the numeric bounds
    /// instead of underflowing, and assigns the result to `self` back.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Feature
    /// `self` will be the difference `self` - `rhs` if underflowing
    /// did not occur. Otherwise, it returns `0`.
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
    pub fn saturating_sub_assign_uint<U>(&mut self, rhs: U)
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if self.overflowing_sub_assign_uint(rhs)
            { self.set_zero(); }
    }

    // pub fn abs_diff<U>(&self, other: U) -> Self
    /// Computes the absolute difference between `self` and `other`.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the absolute difference between `self` and `other`.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use Cryptocol::number::BigUInt;
    /// use Cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// // Todo
    /// ```
    pub fn abs_diff_uint<U>(&self, other: U) -> Self
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let t_other: T;
        if other.length_in_bytes() > T::size_in_bytes()
        {
            return self.abs_diff(&Self::from_uint(other));
        }
        else    // if rhs.length_in_bytes() <= T::size_in_bytes()
        {
            t_other = T::num::<U>(other);
        }

        if self.lt_uint(t_other)
            { Self::from_uint(t_other - self.get_num_(0)) }
        else
            { self.wrapping_add_uint(t_other) }
    }

    /*** Multiplication ***/

    // pub fn carrying_mul_uint<U>(&self, rhs: U, carry: Self) -> (Self, Self)
    /// Calculates the “full multiplication” `self` * `rhs` + `carry` without
    /// the possibility to overflow.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `self` * `rhs` + `carry` in the form of a tuple of the
    /// low-order (wrapping) bits and the high-order (overflow) bits of the
    /// result as two separate values, in that order.
    /// 
    /// # Feature
    /// It performs “long multiplication” which takes in an extra amount to add,
    /// and may return an additional amount of overflow. This allows for
    /// chaining together multiple multiplications to create “bigger integers”
    /// which represent larger values.
    /// 
    /// # Counterpart Methods
    /// If you don’t need the carry, then you can use `widening_mul()` instead.
    /// 
    /// The value of the first field in the returned tuple matches what you’d
    /// get by combining the `wrapping_mul_uint()` and `wrapping_add_uint()`
    /// methods: `self.wrapping_mul_uint(rhs).wrapping_add_uint(carry)`. So,
    /// `self.carrying_mul_uint(rhs, carry).0`
    /// == `self.wrapping_mul_uint(rhs).wrapping_add_uint(carry)`
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
    pub fn carrying_mul_uint<U>(&self, rhs: U, carry: Self) -> (Self, Self)
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut low = self.clone();
        let high = low.carrying_mul_assign_uint(rhs, carry);
        (low, high)
    }

    // pub fn carrying_mul_assign_uint<U>(&mut self, rhs: U, carry: Self) -> Self
    /// Calculates the “full multiplication” `self` * `rhs` + `carry` without
    /// the possibility to overflow, and assigs the low-order bits of the result
    /// to `self` back and returns the high-order bits of the result.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the high-order (overflow) bits of `self` * `rhs` + `carry`
    /// of the result.
    /// 
    /// # Feature
    /// It performs “long multiplication” which takes in an extra amount to add,
    /// and may return an additional amount of overflow. This allows for
    /// chaining together multiple multiplications to create “bigger integers”
    /// which represent larger values.
    /// 
    /// # Counterpart Methods
    /// If you don’t need the carry, then you can use `widening_mul_assign_uint()`
    /// instead.
    /// 
    /// The value of `self` after calculation matches what you’d get by
    /// combining the `wrapping_mul_uint()` and `wrapping_add_assign_uint()` methods:
    /// `self.wrapping_mul_uint(rhs).wrapping_add_assign(_uintcarry)`.
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
    pub fn carrying_mul_assign_uint<U>(&mut self, rhs: U, carry: Self) -> Self
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let trhs: T;
        if rhs.length_in_bytes() > T::size_in_bytes()
        {
            return self.carrying_mul_assign(&Self::from_uint(rhs), carry);
        }
        else    // if rhs.length_in_bytes() <= T::size_in_bytes()
        {
            trhs = T::num::<U>(rhs);
        }
        let zero = T::zero();
        let mut high = Self::zero();
        if trhs == zero
        {
            self.set_zero();
            return high;
        }
        if self.is_zero()
            { return high; }

        let one = T::one();
        let adder = self.clone();
        let mut bit_check = one << T::usize_as_Uint(T::size_in_bits() - 1 - trhs.leading_zeros() as usize);
        self.set_zero();
        while bit_check != zero
        {
            *self <<= 1;
            high <<= 1;
            if bit_check & trhs != zero
            {
                self.wrapping_add_assign(&adder);
                if self.is_overflow()
                    { high.wrapping_add_assign_uint(1_u8); }
            }
            bit_check >>= one;
        }
        if self.overflowing_add_assign(&carry)
            { high.wrapping_add_assign_uint(1_u8); }
        high
    }

    // pub fn widening_mul<U>(&self, rhs: U) -> (Self, Self)
    /// Calculates the complete product `self` * `rhs` without the possibility
    /// to overflow.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `self` * `rhs` in the form of a tuple of the low-order
    /// (wrapping) bits and the high-order (overflow) bits of the result as
    /// two separate values, in that order.
    /// 
    /// # Feature
    /// It performs “long multiplication” which takes in an extra amount to add,
    /// and may return an additional amount of overflow. This allows for
    /// chaining together multiple multiplications to create “bigger integers”
    /// which represent larger values.
    /// 
    /// # Counterpart Methods
    /// If you also need to add a carry to the wide result, then you want to use
    /// `carrying_mul_uint()` instead.
    ///     
    /// The value of the first field in the returned tuple matches what you’d
    /// get the `wrapping_mul_uint()` methods.
    /// `self.widening_mul_uint(rhs).0` == `self.wrapping_mul_uint(rhs)`
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
    #[inline]
    pub fn widening_mul_uint<U>(&self, rhs: U) -> (Self, Self)
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.carrying_mul_uint(rhs, Self::zero())
    }

    // pub fn widening_mul_assign_uint<U>(&mut self, rhs: U) -> Self
    /// Calculates the complete product `self` * `rhs` without the possibility
    /// to overflow.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the high-order (overflow) bits of the result `self` * `rhs`.
    /// 
    /// # Feature
    /// It performs “long multiplication” which takes in an extra amount to add,
    /// and may return an additional amount of overflow. This allows for
    /// chaining together multiple multiplications to create “bigger integers”
    /// which represent larger values.
    /// 
    /// # Counterpart Methods
    /// If you also need to add a carry to the wide result, then you want to use
    /// `carrying_mul_assign_uint()` instead.
    ///     
    /// The value of `self` after calculation matches what you’d get the
    /// `wrapping_mul_uint()` methods.
    /// `self` == `self.wrapping_mul_uint(rhs)`
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
    #[inline]
    pub fn widening_mul_assign_uint<U>(&mut self, rhs: U) -> Self
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.carrying_mul_assign_uint(rhs, Self::zero())
    }

    // pub fn wrapping_mul_uint<U>(&self, rhs: U) -> Self
    /// Multiplies `BigUInt`-type number with a unsigned integer number
    /// of type `T` and returns its result in a type of BigUInt.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the multiplication of `self` and `rhs`.
    /// 
    /// # Examples
    /// ```
    /// use std::str::FromStr;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let a = u256::from_str("10000000000000000000000000000000000").unwrap();
    /// let mul = a.wrapping_mul_uint(35_u128);
    /// println!("mul = {}", mul);
    /// assert_eq!(mul.to_string(), "350000000000000000000000000000000000");
    /// ```
    pub fn wrapping_mul_uint<U>(&self, rhs: U) -> Self
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut bi = self.clone();
        bi.wrapping_mul_assign_uint(rhs);
        bi
    }

    // pub fn wrapping_mul_assign_uint<U>(&mut self, rhs: U)
    /// Multiplies self which is of `BigUInt` type with rhs of type `U`.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn wrapping_mul_assign_uint<U>(&mut self, rhs: U)
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let trhs: T;
        if rhs.length_in_bytes() > T::size_in_bytes()
        {
            return self.wrapping_mul_assign(&Self::from_uint(rhs));
        }
        else    // if rhs.length_in_bytes() <= T::size_in_bytes()
        {
            trhs = T::num::<U>(rhs);
        }
        if self.is_zero()
            { return; }
        let zero = T::zero();
        let one = T::one();
        if trhs == zero
        {
            self.set_zero();
            return;
        }
        let adder = self.clone();
        let mut bit_check = one << T::usize_as_Uint(T::size_in_bits() - 1 - trhs.leading_zeros() as usize);
        self.set_zero();

        while bit_check != zero
        {
            *self <<= 1;
            if bit_check & trhs != zero
                { self.wrapping_add_assign(&adder); }
            bit_check >>= one;
        }
    }

    // pub fn overflowing_mul_uint<U>(&self, rhs: U) -> (Self, bool)
    /// Calculates `self` * `rhs`.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Output
    /// It returns a tuple of the multiplication `self` * `rhs` along
    /// with a boolean indicating whether an arithmetic overflow would occur.
    /// If an overflow would have occurred then the wrapped (modular) value
    /// is returned.
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
    pub fn overflowing_mul_uint<U>(&self, rhs: U) -> (Self, bool)
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut res = self.clone();
        let overflow = res.overflowing_mul_assign_uint(rhs);
        (res, overflow)
    }

    // pub fn overflowing_mul_assign_uint<U>(&mut self, rhs: U) -> bool
    /// Calculates `self` * `rhs`, and assigns the result to `self` back.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Output
    /// It returns true if an arithmetic overflow would occur.
    /// Otherwise, it returns `false`.
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
    pub fn overflowing_mul_assign_uint<U>(&mut self, rhs: U) -> bool
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.wrapping_mul_assign_uint(rhs);
        self.is_overflow()
    }

    // pub fn checked_mul_uint<U>(&self, rhs: U) -> Option<Self>
    /// Computes `self` * `rhs`.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the sum `self` * `rhs` wrapped by `Some` of enum `Option`
    /// if overflow did not occur. Otherwise, it returns `None` of enum Option.
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
    pub fn checked_mul_uint<U>(&self, rhs: U) -> Option<Self>
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut res = self.clone();
        let overflow = res.overflowing_mul_assign_uint(rhs);
        if overflow
            { None }
        else
            { Some(res) }
    }

    // pub fn unchecked_mul_uint<U>(&self, rhs: U) -> Self
    /// Computes `self` * `rhs`, assuming overflow cannot occur.
    /// 
    /// # Panics
    /// - If overflow occurred, it will panic. So, use this method only when
    /// you are sure that overflow will not occur.
    /// - If size_of::<T>() * N <= 128, some methods may panic or its behavior
    /// may undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the sum `self` * `rhs` if overflow did not occur.
    /// Otherwise, it will panic.
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
    #[inline]
    pub fn unchecked_mul_uint<U>(&self, rhs: U) -> Self
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.checked_mul_uint(rhs).unwrap()
    }

    // pub fn saturating_mul_uint<U>(&self, rhs: U) -> Self
    /// Computes `self` * `rhs`, saturating at the numeric bounds
    /// instead of overflowing.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the sum `self` + `rhs` if overflow did not occur.
    /// Otherwise, it returns the maximum value.
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
    pub fn saturating_mul_uint<U>(&self, rhs: U) -> Self
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut res = self.clone();
        res.saturating_mul_assign_uint(rhs);
        res
    }

    // pub fn saturating_mul_assign_uint<U>(&mut self, rhs: U)
    /// Computes `self` * `rhs`, saturating at the numeric bounds
    /// instead of overflowing, and assigns the result to `self` back.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
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
    pub fn saturating_mul_assign_uint<U>(&mut self, rhs: U)
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if self.overflowing_mul_assign_uint(rhs)
            { self.set_max(); }
    }


    /*** Division ***/

    // pub fn divide_fully_uint<U>t(&self, rhs: U) -> (Self, T)
    /// Divide `BigUInt<T, N>` by `rhs` so as to get quotient and remainder
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Output
    /// It returns tuple of quotient and remainder. quotient is `Self` type
    /// and remainder is `T` type.
    /// 
    /// # Feature
    /// If `rhs` is zero, the divided_by_zero and overflow flags of quotient
    /// will be set, and the quotient and the remainder will be max value and
    /// zero, respectively.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use std::str::FromStr;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let dividend = u256::from_str("1234567890157589425462369896").unwrap();
    /// let (quotient, remainder) = dividend.divide_fully_uint(87_u128);
    /// ```
    pub fn divide_fully_uint<U>(&self, rhs: U) -> (Self, U)
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let trhs: T;
        if rhs.length_in_bytes() > T::size_in_bytes()
        {
            let (q, r) = self.divide_fully(&Self::from_uint(rhs));
            match U::size_in_bytes()
            {
                2 => {
                    let mut rr = ShortUnion::new();
                    for i in 0..2
                    {
                        rr.set_ubyte_(i, r.get_num_(i).into_u8());
                    }
                    return (q, U::u16_as_Uint(rr.get()));
                },
                4 => {
                    let mut rr = IntUnion::new();
                    match T::size_in_bytes()
                    {
                        1 => {
                            for i in 0..4
                            {
                                rr.set_ubyte_(i, r.get_num_(i).into_u8());
                            }
                        },
                        _ => {
                            for i in 0..2
                            {
                                rr.set_ushort_(i, r.get_num_(i).into_u16());
                            }
                        },
                    }
                    return (q, U::u32_as_Uint(rr.get()));
                },
                8 => {
                    let mut rr = LongUnion::new();
                    match T::size_in_bytes()
                    {
                        1 => {
                            for i in 0..8
                            {
                                rr.set_ubyte_(i, r.get_num_(i).into_u8());
                            }
                        },
                        2 => {
                            for i in 0..4
                            {
                                rr.set_ushort_(i, r.get_num_(i).into_u16());
                            }
                        },
                        _ => {
                            for i in 0..2
                            {
                                rr.set_uint_(i, r.get_num_(i).into_u32());
                            }
                        },
                    }
                    return (q, U::u64_as_Uint(rr.get()));
                },
                _ => {
                    let mut rr = LongerUnion::new();
                    match T::size_in_bytes()
                    {
                        1 => {
                            for i in 0..16
                            {
                                rr.set_ubyte_(i, r.get_num_(i).into_u8());
                            }
                        },
                        2 => {
                            for i in 0..8
                            {
                                rr.set_ushort_(i, r.get_num_(i).into_u16());
                            }
                        },
                        4 => {
                            for i in 0..4
                            {
                                rr.set_uint_(i, r.get_num_(i).into_u32());
                            }
                        },
                        _ => {
                            for i in 0..2
                            {
                                rr.set_ulong_(i, r.get_num_(i).into_u64());
                            }
                        },
                    }
                    return (q, U::u128_as_Uint(rr.get()));
                }
            }
        }
        else    // if rhs.length_in_bytes() <= T::size_in_bytes()
        {
            trhs = T::num::<U>(rhs);
        }
        let mut quotient = Self::zero();
        let tzero = T::zero();
        let uzero = U::zero();
        if self.is_zero()
        {
            return (quotient, uzero);
        }
        if rhs == uzero
        {
            quotient.set_max();
            quotient.set_infinity();
            quotient.set_divided_by_zero();
            quotient.set_overflow();
            return (quotient, uzero);
        }
        if self.lt_uint(rhs)
        {
            return (quotient, U::num(self.get_num_(0)));
        }
        else if self.eq_uint(rhs)
        {
            quotient.set_uint(T::one());
            return (quotient, uzero);
        }

        let mut adder = Self::zero();
        let mut highest = self.length_in_bits() - self.leading_zeros() as usize;
        let mut high = highest;
        let mut low = 0;
        let mut mid = (high + low) >> 1;
        let mut res;
        let mut sum;
        let maximum = Self::size_in_bits() - 1;
        loop
        {
            high = highest;
            low = 0;
            if high == 0
            {
                return (quotient.clone(), U::num(self.wrapping_sub(&quotient.wrapping_mul_uint(rhs)).get_num_(0)));
            }
            else    // if high > 0
            {
                loop
                {
                    mid = (high + low) >> 1;
                    adder.turn_check_bits(mid);
                    sum = quotient.wrapping_add(&adder);
                    res = sum.wrapping_mul_uint(rhs);
                    if !res.is_overflow() && (*self > res)
                    {
                        if mid == maximum
                        {
                            quotient = sum;
                            break;
                        }
                        else if mid == low 
                        {
                            quotient = sum;
                            if mid == 0
                                { highest = 0; }
                            break;
                        }
                        low = mid;
                    }
                    else if res.is_overflow() || (res > *self)
                    {
                        if mid == low
                        {
                            highest = mid;
                            break;
                        }
                        high = mid;
                    }
                    else    // if res == *self
                    {
                        return (quotient + adder, uzero);
                    }
                }
            }
        }
    }

    // pub fn wrapping_div_uint(&self, rhs: T) -> Self
    /// Calculates the quotient when `self` is divided by `rhs`,
    /// which is `self` / `rhs`.
    ///
    /// # Output
    /// It returns the quotient of when `self` is divided by `rhs`,
    /// which is `self` / `rhs`. 
    /// 
    /// # Feature
    /// Wrapped division on `BigUInt` types is just normal division.
    /// There’s no way wrapping could ever happen. This function exists,
    /// so that all operations are accounted for in the wrapping operations.
    /// 
    /// If `rhs` is zero, the quotient will have maximum value of `BigUInt`
    /// type, and the flags of quotient such as `OVERFLOW`, `INFINITY`, and
    /// `DIVIDED_BY_ZERO` will be set. __It does not panic__ while the same
    /// named methods `wrapping_div()` for primitive integer data type such
    /// as u8, u16, u32, u64, etc. will panic if `rhs` is zero.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let a = u256::from_str("10000000000000000000000000000000000").unwrap();
    /// let div = a.wrapping_div_uint(35);
    /// println!("div = {}", div);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn wrapping_div_uint(&self, rhs: T) -> Self
    {
        let (quotient, _) = self.divide_fully_uint(rhs);
        quotient
    }

    // pub fn wrapping_div_assign_uint(&mut self, rhs: T)
    /// Calculates the quotient when `self` is divided by `rhs`,
    /// which is `self` / `rhs`, and assign the result to `self` back.
    /// 
    /// # Feature
    /// Wrapped division on `BigUInt` types is just normal division.
    /// There’s no way wrapping could ever happen. This function exists,
    /// so that all operations are accounted for in the wrapping operations.
    /// 
    /// If `rhs` is zero, the `self` will have maximum value of `BigUInt`
    /// type, and the flags of `self` such as `OVERFLOW`, `INFINITY`, and
    /// `DIVIDED_BY_ZERO` will be set. __It does not panic__ while the same
    /// kind methods `wrapping_div()` for primitive integer data type such
    /// as u8, u16, u32, u64, etc. will panic if `rhs` is zero.
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
    pub fn wrapping_div_assign_uint(&mut self, rhs: T)
    {
        let (quotient, _) = self.divide_fully_uint(rhs);
        *self = quotient;
    }

    // pub fn checked_div_uint(&self, rhs: T) -> Option<Self>
    /// Calculates the quotient when `self` is divided by `rhs`,
    /// which is `self` / `rhs`.
    /// 
    /// # Output
    /// It returns `None` if `rhs` is zero. Otherwise, it returns the quotient
    /// of when `self` is divided by `rhs`, which is `self` / `rhs`,
    /// wrapped by `Some` of enum `Option`.
    /// 
    /// # Feature
    /// Wrapped division on `BigUInt` types is just normal division.
    /// There’s no way wrapping could ever happen.
    /// 
    /// If `rhs` is zero, the quotient will have maximum value of `BigUInt`
    /// type, and the flags of quotient such as `OVERFLOW`, `INFINITY`, and
    /// `DIVIDED_BY_ZERO` will be set.
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
    pub fn checked_div_uint(&self, rhs: T) -> Option<Self>
    {
        let res = self.wrapping_div_uint(rhs);
        if res.is_divided_by_zero()
            { None }
        else
            { Some(res) }
    }

    // pub fn unchecked_div_uint(&self, rhs: T) -> Self
    /// Calculates the quotient when `self` is divided by `rhs`,
    /// which is `self` / `rhs`, assuming that `rhs` cannot be zero.
    /// 
    /// # Panics
    /// If `rhs` is zero, it will panic. So, use this method only when you
    /// are sure that `rhs` is not zero. 
    /// 
    /// # Output
    /// It returns the quotient of when `self` is divided by `rhs`,
    /// which is `self` / `rhs` if `rhs` is not zero.
    /// Otherwise, it will panic.
    /// 
    /// # Feature
    /// Wrapped division on `BigUInt` types is just normal division.
    /// There’s no way wrapping could ever happen.
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
    #[inline]
    pub fn unchecked_div_uint(&self, rhs: T) -> Self
    {
        self.checked_div_uint(rhs).unwrap()
    }

    // pub fn saturating_div_uint(&self, rhs: T) -> Self
    /// Calculates the quotient when `self` is divided by `rhs`,
    /// which is `self` / `rhs`, saturating at the numeric bounds
    /// instead of overflowing.
    /// 
    /// # Output
    /// It returns the quotient of when `self` is divided by `rhs`,
    /// which is `self` / `rhs` if `rhs` is not zero.
    /// Otherwise, it returns the maximum value.
    /// 
    /// # Feature
    /// Overflow will not happen unless `rhs` is zero. __It does not panic__
    /// while the same named methods `saturating_div()` for primitive integer
    /// data type such as u8, u16, u32, u64, etc. will panic if `rhs` is zero.
    /// 
    /// If `rhs` is zero, the quotient will have maximum value of `BigUInt`
    /// type, and the flags of quotient such as `OVERFLOW` and `DIVIDED_BY_ZERO`
    /// will be set.
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
    pub fn saturating_div_uint(&self, rhs: T) -> Self
    {
        let (mut quotient, _) = self.divide_fully_uint(rhs);
        quotient.reset_inifinity();
        quotient
    }

    // pub fn saturating_div_assign_uint(&mut self, rhs: T) -> Self
    /// Calculates the quotient when `self` is divided by `rhs`,
    /// which is `self` / `rhs`, saturating at the numeric bounds
    /// instead of overflowing, and assigns the quotient to `self` back.
    /// 
    /// # Feature
    /// Overflow will not happen unless `rhs` is zero. __It does not panic__
    /// while the similar methods `saturating_div()` for primitive integer
    /// data type such as u8, u16, u32, u64, etc. will panic if `rhs` is zero.
    /// 
    /// If `rhs` is zero, `self` will have maximum value of `BigUInt`
    /// type, and the flags of `self` such as `OVERFLOW` and `DIVIDED_BY_ZERO`
    /// will be set.
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
    #[inline]
    pub fn saturating_div_assign_uint(&mut self, rhs: T)
    {
        *self = self.saturating_div_uint(rhs);
    }

    // pub fn wrapping_rem_uint(&self, rhs: T) -> T
    /// Calculates the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`.
    /// 
    /// # Output
    /// It returns the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`, with wrapping (modular) addition.
    /// 
    /// # Feature
    /// Wrapped remainder calculation on `BigUInt` types is just the regular
    /// remainder calculation. There’s no way wrapping could ever happen. This
    /// function exists, so that all operations are accounted for in the
    /// wrapping operations.
    /// 
    /// If `rhs` is zero, the remainder is zero and its `DIVIDED_BY_ZERO`
    /// is set. __It does not panic__ while the same named methods
    /// `wrapping_rem()` for primitive integer data type such as u8, u16,
    /// u32, u64, etc. will panic if `rhs` is zero.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let a = u256::from_str("10000000000000000000000000000000000").unwrap();
    /// let rem = a.wrapping_rem_uint(35_u128);
    /// println!("rem = {}", rem);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn wrapping_rem_uint(&self, rhs: T) -> T
    {
        let (_, remainder) = self.divide_fully_uint(rhs);
        remainder
    }

    // pub fn wrapping_rem_assign_uint(&mut self, rhs: T)
    /// Calculates the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`, and returns the remainder to `self` back.
    /// 
    /// # Feature
    /// Wrapped remainder calculation on `BigUInt` types is just the regular
    /// remainder calculation. There’s no way wrapping could ever happen. This
    /// function exists, so that all operations are accounted for in the
    /// wrapping operations.
    /// 
    /// If `rhs` is zero, the `self` is zero and its `DIVIDED_BY_ZERO`
    /// is set. __It does not panic__ while the similar methods
    /// `wrapping_rem()` for primitive integer data type such as 
    /// u8, u16, u32, u64, etc. will panic if `rhs` is zero.
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
    pub fn wrapping_rem_assign_uint(&mut self, rhs: T)
    {
        let (_, remainder) = self.divide_fully_uint(rhs);
        self.set_uint(remainder);
        if rhs == T::zero()
            { self.set_divided_by_zero(); }
    }

    // pub fn overflowing_rem_uint(&self, rhs: T) -> (T, bool)
    /// Calculates the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`.
    /// 
    /// # Output
    /// It returns a tuple of the remainder after dividing,
    /// which is `self` % `rhs` along with a boolean indicating whether an
    /// arithmetic overflow would occur.
    /// 
    /// # Feature
    /// Note that overflow never occurs, so the second value is always false.
    /// 
    /// If `rhs` is zero, the remainder is zero and its `DIVIDED_BY_ZERO`
    /// is set. __It does not panic__ while the same named methods
    /// `overflowing_rem()` for primitive integer data type such as u8, u16,
    /// u32, u64, etc. will panic if `rhs` is zero.
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
    pub fn overflowing_rem_uint(&self, rhs: T) -> (T, bool)
    {
        let (_, remainder) = self.divide_fully_uint(rhs);
        (remainder, false)
    }

    // pub fn overflowing_rem_assign_uint(&mut self, rhs: T) -> bool
    /// Calculates the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`, and returns the remainder to `self` back.
    /// 
    /// # Output
    /// It returns a boolean indicating whether an arithmetic overflow
    /// would occur.
    /// 
    /// # Feature
    /// Note that overflow never occurs, so the outtput is always false.
    /// 
    /// If `rhs` is zero, `self` is zero and its `DIVIDED_BY_ZERO`
    /// is set. __It does not panic__ while the similar methods
    /// `overflowing_rem()` for primitive integer data type such as
    /// u8, u16, u32, u64, etc. will panic if `rhs` is zero.
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
    pub fn overflowing_rem_assign_uint(&mut self, rhs: T) -> bool
    {
        let (_, remainder) = self.divide_fully_uint(rhs);
        self.set_uint(remainder);
        if rhs == T::zero()
            { self.set_divided_by_zero(); }
        false
    }

    // pub fn checked_rem_uint(&self, rhs: T) -> Option<T>
    /// Calculates the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`.
    /// 
    /// # Output
    /// It returns the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`, wrapped by `Some` of enum `Option`
    /// if `rhs` is not zero. Otherwise, it returns `None` of enum Option.
    /// 
    /// # Feature
    /// Note that overflow never occurs.
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
    pub fn checked_rem_uint(&self, rhs: T) -> Option<T>
    {
        let (_, remainder) = self.divide_fully_uint(rhs);
        if rhs == T::zero()
            { None }
        else
            { Some(remainder) }
    }

    // pub fn unchecked_rem_uint(&self, rhs: T) -> T
    /// Calculates the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`, assuming `rhs` cannot be zero.
    /// 
    /// # Output
    /// It returns the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`, if `rhs` is not zero.
    /// Otherwise, it returns zero.
    /// 
    /// # Feature
    /// Note that overflow never occurs.
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
    #[inline]
    pub fn unchecked_rem_uint(&self, rhs: T) -> T
    {
        self.checked_rem_uint(rhs).unwrap()
    }

    // pub fn saturating_rem_uint(&self, rhs: T) -> T
    /// Calculates the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`
    /// 
    /// # Output
    /// It returns the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`, if `rhs` is not zero.
    /// Otherwise, it returns zero.
    /// 
    /// # Feature
    /// If `rhs` is zero, the remainder will have zero of`BigUInt` type,
    /// and `DIVIDED_BY_ZERO` flag of the remainder will be set, and
    /// the remainder will be set to be zero of `BigUInt` type.
    /// 
    /// Note that overflow never occurs.
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
    pub fn saturating_rem_uint(&self, rhs: T) -> T
    {
        let (_, remainder) = self.divide_fully_uint(rhs);
        remainder
    }

    // pub fn saturating_rem_assign_uint(&mut self, rhs: T)
    /// Calculates the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`, and assigns the remainder to `self` back.
    /// 
    /// # Feature
    /// If `rhs` is zero, `self` will have zero of`BigUInt` type,
    /// and `DIVIDED_BY_ZERO` flag of `self` will be set, and
    /// `self` will be set to be zero of `BigUInt` type.
    /// 
    /// Note that overflow never occurs.
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
    pub fn saturating_rem_assign_uint(&mut self, rhs: T)
    {
        let (_, remainder) = self.divide_fully_uint(rhs);
        self.set_uint(remainder);
        if rhs == T::zero()
            { self.set_divided_by_zero(); }
    }

    // pub fn next_multiple_of_uint<U>(&self, rhs: U) -> Self
    /// Calculates the smallest value greater than or equal to `self` that is
    /// a multiple of `rhs`.
    /// 
    /// # Panics
    /// - This function will panic if rhs is zero.
    /// - If size_of::<T>() * N <= 128, This method may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the smallest value greater than or equal to self that is
    /// a multiple of `rhs`. However, if overflow occurs, it returns the value
    /// wrapped around.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn next_multiple_of_uint<U>(&self, rhs: U) -> Self
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut res = self.clone();
        res.next_multiple_of_assign_uint(rhs);
        res
    }

    // pub fn next_multiple_of_assign_uint<U>(&mut self, rhs: U)
    /// Calculates the smallest value greater than or equal to `self` that is
    /// a multiple of `rhs`, and assigns the result to `self` back.
    /// 
    /// # Panics
    /// - This function will panic if rhs is zero.
    /// - If size_of::<T>() * N <= 128, This method may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Feature
    /// `self` will be the smallest value greater than or equal to self that is
    /// a multiple of `rhs`. However, if overflow occurs, `self` will be the
    /// value wrapped around.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn next_multiple_of_assign_uint<U>(&mut self, rhs: U)
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if rhs == U::zero()
            { panic!(); }

        if U::size_in_bytes() > T::size_in_bytes()
        {
            self.next_multiple_of_assign(&Self::from_uint(rhs));
        }
        else
        {
            let trhs = T::num(rhs);
            let r = self.wrapping_rem_uint(trhs);
            if r != T::zero()
            {
                self.wrapping_add_assign_uint(trhs - r);
            }
        }
    }

    


    /***** METHODS FOR EXPONENTIATION AND LOGARITHM WITH UNSIGNED INTEGERS *****/

    // pub fn pow_uint<U>(&self, exp: U) -> Self
    /// Raises `BigUInt` type number to the power of exp, using exponentiation
    /// of type `BigUInt` by squaring. The type `U` has the trait `Uint`.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the result of `self` raised to the power of `exp`.
    /// 
    /// # Argument
    /// The argument `exp` is the primitive unsigned integer type.
    /// 
    /// # Counterpart Method
    /// If `rhs` is `BigUInt` type number, use the mentod `pow()` instead.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a = u256::from_uint(123_u8);
    /// 
    /// // normal exponentiation
    /// let b = a.pow_uint(36_u8);
    /// println!("123 ** 36 = {}", b);
    /// assert_eq!(b.to_string(), "1724185592748300222303045014791251528772289498837076631331177393773983461361");
    /// 
    /// // wrapping (modular) exponentiation
    /// let c = a.pow_uint(37_u8);
    /// println!("123 ** 37 = {}", c);
    /// assert_eq!(c.to_string(), "96282738670724731919703551810636030185721623691319861614277235426286836107467");
    /// 
    /// // evidence of wrapping (modular) exponentiation
    /// assert_eq!(b.is_overflow(), false);
    /// assert_eq!(c.is_overflow(), true);
    /// ```
    #[inline]
    pub fn pow_uint<U>(&self, exp: U) -> Self
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.wrapping_pow_uint(exp)
    }

    // pub fn pow_assign_uint<U>(&mut self, exp: U)
    /// Raises `BigUInt` type number to the power of exp,
    /// using exponentiation of primitive unsigned integer type by squaring,
    /// and assign the result to `self` back.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Argument
    /// The argument `exp` is the primitive unsigned integer type.
    /// 
    /// # Counterpart Method
    /// If `rhs` is the `BigUInt` type number, use the mentod `pow_assign()`
    /// instead.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = u256::from_uint(234_u8);
    /// 
    /// // normal exponentiation
    /// a.pow_assign_uint(34_u8);
    /// println!("234 ** 34 = {}", a);
    /// assert_eq!(a.to_string(), "101771369680718065636717400052436696519017873276976456689251925337442881634304");
    /// 
    /// // wrapping (modular) exponentiation
    /// let old = a.clone();
    /// a = u256::from_uint(234_u8);
    /// a.pow_assign_uint(35_u8);
    /// println!("234 ** 35 = {}", a);
    /// assert_eq!(a.to_string(), "77122211638207297159819685489165875529835490356175237196145807339442726240256");
    /// 
    /// // evidence of wrapping (modular) exponentiation
    /// assert!(old > a);
    /// ```
    #[inline]
    pub fn pow_assign_uint<U>(&mut self, exp: U)
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.wrapping_pow_assign_uint(exp);
    }

    // pub fn wrapping_pow_uint<U>(&self, exp: U) -> Self
    /// Raises `BigUInt` type number to the power of exp, using exponentiation
    /// of type `BigUInt` by squaring, wrapping around at the boundary of the
    /// type. The type `U` has the trait `Uint`.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the result of `self` raised to the power of `exp`.
    /// 
    /// # Argument
    /// The argument `exp` is the primitive unsigned integer type.
    /// 
    /// # Feature
    /// Wrapping (modular) exponentiation. It calls wrapping_pow_uint()
    /// internally.
    /// 
    /// # Counterpart Method
    /// If `rhs` is `BigUInt` type number, use the mentod `pow()` instead.
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
    pub fn wrapping_pow_uint<U>(&self, exp: U) -> Self
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut res = self.clone();
        res.wrapping_pow_assign_uint(exp);
        res
    }

    // pub fn wrapping_pow_assign_uint<U>(&mut self, exp: U)
    /// Raises `BigUInt` type number to the power of `exp`, using exponentiation
    /// of primitive unsigned integer type by squaring, wrapping around at the
    /// boundary of the type, and assign the result to `self` back.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Argument
    /// The argument `exp` is the primitive unsigned integer type.
    /// 
    /// # Feature
    /// Wrapping (modular) exponentiation. It calls wrapping_pow_assign_uint()
    /// internally.
    /// 
    /// # Counterpart Method
    /// If `rhs` is the `BigUInt` type number, use the mentod `pow_assign()`
    /// instead.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = u256::from_uint(234_u8);
    /// let mut exp = u256::from_uint(34_u8);
    /// 
    /// // normal exponentiation
    /// a.wrapping_pow_assign(&exp);
    /// println!("234 ** 34 = {}", a);
    /// assert_eq!(a.to_string(), "101771369680718065636717400052436696519017873276976456689251925337442881634304");
    /// 
    /// // wrapping (modular) exponentiation
    /// let old = a.clone();
    /// a = u256::from_uint(234_u8);
    /// exp += 1;
    /// a.wrapping_pow_assign(&exp);
    /// println!("234 ** 35 = {}", a);
    /// assert_eq!(a.to_string(), "77122211638207297159819685489165875529835490356175237196145807339442726240256");
    /// 
    /// // evidence of wrapping (modular) exponentiation
    /// assert!(old > a);
    /// ```
    pub fn wrapping_pow_assign_uint<U>(&mut self, exp: U)
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if self.is_zero() || self.is_one()
            { return; }

        let zero = U::zero();
        let one = U::one();
        let multiplier = self.clone();
        self.set_one();
        if exp == zero
            { return; }

        let mut bit_check = one << U::usize_as_Uint(exp.length_in_bits() - 1 - exp.leading_zeros() as usize);
        if bit_check != zero
        {
            self.wrapping_mul_assign(&multiplier);
            bit_check >>= one;
        }
        while bit_check != zero
        {
            *self = self.wrapping_mul(self);
            if (bit_check & exp) != zero
                { self.wrapping_mul_assign(&multiplier); }
            bit_check >>= one;
        }
    }


    // pub fn ilog_uint<U>(&self, base: U) -> Self
    /// Calculates the logarithm of the number with respect to a `base`.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the logarithm of the number with respect to an arbitrary
    /// `base`, rounded down.
    /// 
    /// # Counterpart Methods
    /// This method might not be optimized owing to implementation details;
    /// `ilog2()` can produce results more efficiently for base 2,
    /// and `ilog10()` can produce results more efficiently for base 10.
    /// 
    /// # Panics
    /// This function will panic if `self` is zero, or if `base` is less than 2.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn ilog_uint<U>(&self, base: U) -> Self
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if self.is_zero() || (base <= U::one())
            { panic!() }
        let mut count = 0_usize;
        let mut quotient = self.clone();
        quotient.wrapping_div_assign_uint(T::num::<U>(base));
        while !quotient.is_zero()
        {
            count += 1;
            quotient.wrapping_div_assign_uint(T::num::<U>(base))
        }
        Self::from_uint(count)
    }

    // pub fn checked_ilog_uint<U>(&self, base: U) -> Option<Self>
    /// Calculates the logarithm of the number with respect to a `base`.
    /// 
    /// # Panics
    /// If size_of::<T>() * N <= 128, some methods may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the logarithm of the number with respect to an arbitrary
    /// `base`, rounded down, wrapped by `Some` of enum `Option`.
    /// It returns `None` if `self` is zero, or if `base` is less than 2.
    /// 
    /// # Counterpart Methods
    /// This method might not be optimized owing to implementation details;
    /// `checked_ilog2()` can produce results more efficiently for base 2,
    /// and `checked_ilog10` can produce results more efficiently for base 10.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn checked_ilog_uint<U>(&self, base: U) -> Option<Self>
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if self.is_zero() || (base <= U::one())
            { return None }
        let mut count = 0_usize;
        let mut quotient = self.clone();
        quotient.wrapping_div_assign_uint(T::num::<U>(base));
        while !quotient.is_zero()
        {
            count += 1;
            quotient.wrapping_div_assign_uint(T::num::<U>(base))
        }
        Some(Self::from_uint(count))
    }

    // pub fn overflowing_pow_uint() -> (Self, bool)
    // pub fn overflowing_pow_assign_uint() -> bool
    // pub fn checked_pow_uint() -> Option<Self>
    // pub fn unchecked_pow_uint() -> Self
    // pub fn saturating_pow_uint() -> Self
    // pub fn saturating_pow_assign_uint()

    // pub fn wrapping_pow_uint() -> Self
    // pub fn wrapping_pow_assign_uint()
    // pub fn overflowing_pow_uint() -> (Self, bool)
    // pub fn overflowing_pow_assign_uint() -> bool
    // pub fn checked_pow_uint() -> Option<Self>
    // pub fn unchecked_pow_uint() -> Self
    // pub fn saturating_pow_uint() -> Self
    // pub fn saturating_pow_assign_uint()

    /***** ARITHMATIC OPERATIONS WITH BigUInt *****/

    /*** ADDITION ***/

    // pub fn carrying_add(&self, rhs: &Self, carry: bool) -> (Self, bool)
    /// Calculates `self` + `rhs` + `carry`,
    /// wrapping around at the boundary of the type.
    /// 
    /// # Features
    /// This allows chaining together multiple additions to create even a wider
    /// addition. This can be thought of as a big integer “full adder”,
    /// in the electronics sense.
    /// 
    /// If the input carry is `false`, this method is equivalent to
    /// `overflowing_add()`, and the output carry is equal to the overflow flag.
    /// 
    /// # Outputs
    /// It returns a tuple containing the sum and the output carry. It performs
    /// “ternary addition” of two big integer operands and a carry-in bit, and
    /// returns an output big integer and a carry-out bit.
    /// 
    /// # Counterpart Method
    /// The method `carrying_add_uint()` is a bit faster than this method
    /// `carrying_add()`. If `rhs` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, u128 and usize. use the mentod `pow_uint()`.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use Cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let a_hi = u256::from_str("9876543210987654321098765432109876543210987654321098765432109876543210987654").unwrap();
    /// let a_lo = u256::from_str("91234567890123456789012345678901234567890123456789012345678901234567890123456").unwrap();
    /// let b_hi = u256::from_str("1111111101111111110111111111011111111101111111110111111111011111111101111110").unwrap();
    /// let b_lo = u256::from_str("101111111101111111110111111111011111111101111111110111111111011111111101111110").unwrap();
    /// 
    /// let (c_lo, carry) = a_lo.carrying_add(&b_lo, false);
    /// let (c_hi, overflow) = a_hi.carrying_add(&b_hi, carry);
    ///  
    /// println!("{}:{} + {}:{} = {}:{}", a_hi, a_lo, b_hi, b_lo, c_hi, c_lo);
    /// println!("carry = {}, overflow = {}", carry, overflow);
    /// 
    /// assert_eq!(c_hi.to_string(), "10987654312098765431209876543120987654312098765431209876543120987654312098765");
    /// assert_eq!(c_lo.to_string(), "76553589753918372475552471781224437825721249902258559417332328337765861594630");
    /// assert_eq!(carry, true);
    /// assert_eq!(overflow, false);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn carrying_add(&self, rhs: &Self, carry: bool) -> (Self, bool)
    {
        let mut res = self.clone();
        let c = res.carrying_add_assign(rhs, carry);
        (res, c)
    }

    // pub fn carrying_add_assign(&self, rhs: &Self, carry: bool) -> bool
    /// Accumulate `rhs` + `carry` to `self`, wrapping around at the boundary
    /// of the type, and return the resulting carry.
    /// 
    /// # Features
    /// This allows chaining together multiple additions to create even a wider
    /// addition. This can be thought of as a big integer “full adder”,
    /// in the electronics sense.
    /// 
    /// If the input carry is false, this method is equivalent to
    /// `overflowing_add_assign()`, and the output carry is equal to
    /// the overflow flag.
    /// 
    /// # Outputs
    /// It returns the output carry. It performs “ternary addition” of two big
    /// integer operands and a carry-in bit, and returns a carry-out bit.
    /// 
    /// # Counterpart Method
    /// The method `carrying_add_assign_uint()` is a bit faster than this
    /// method `carrying_add_assign()`. If `rhs` is primitive unsigned integral
    /// data type such as u8, u16, u32, u64, u128 and usize. use the mentod
    /// `carrying_add_assign_uint()`.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use Cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_hi = u256::from_str("9876543210987654321098765432109876543210987654321098765432109876543210987654").unwrap();
    /// let mut a_lo = u256::from_str("91234567890123456789012345678901234567890123456789012345678901234567890123456").unwrap();
    /// let b_hi = u256::from_str("1111111101111111110111111111011111111101111111110111111111011111111101111110").unwrap();
    /// let b_lo = u256::from_str("101111111101111111110111111111011111111101111111110111111111011111111101111110").unwrap();
    /// 
    /// let carry = a_lo.carrying_add_assign(&b_lo, false);
    /// let overflow = a_hi.carrying_add_assign(&b_hi, carry);
    /// 
    /// println!("9876543210987654321098765432109876543210987654321098765432109876543210987654:91234567890123456789012345678901234567890123456789012345678901234567890123456 + {}:{} = {}:{}", b_hi, b_lo, a_hi, a_lo);
    /// println!("carry = {}, overflow = {}", carry, overflow);
    /// 
    /// assert_eq!(a_hi.to_string(), "10987654312098765431209876543120987654312098765431209876543120987654312098765");
    /// assert_eq!(a_lo.to_string(), "76553589753918372475552471781224437825721249902258559417332328337765861594630");
    /// assert_eq!(carry, true);
    /// assert_eq!(overflow, false);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn carrying_add_assign(&mut self, rhs: &Self, carry: bool) -> bool
    {
        let mut c = carry;
        let mut num: T;
        for i in 0..N
        {
            (num, c) = self.get_num_(i).carrying_add(rhs.get_num_(i), c);
            self.set_num_(i, num);
        }
        if c
            { self.set_overflow(); }
        c
    }

    // pub fn wrapping_add(&self, rhs: &Self) -> Self
    /// Computes `self` + `rhs`, wrapping around at the boundary of the type.
    /// 
    /// # Output
    /// It returns `self` + `rhs` with wrapping (modular) addition.
    /// 
    /// # Feature
    /// Wrapping (modular) addition.
    /// 
    /// # Counterpart Method
    /// The method `wrapping_add_uint()` is a bit faster than this
    /// method `wrapping_add()`. If `rhs` is primitive unsigned integral
    /// data type such as u8, u16, u32, u64, u128 and usize. use the mentod
    /// `wrapping_add_uint()`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    ///  
    /// let zero = u512::zero();
    /// let one = u512::one();
    /// let two = u512::from(2_u8);
    /// let three = u512::from(3_u8);
    /// let a = u512::max() - &one;
    /// let b = a.wrapping_add(&one);
    /// let c = a.wrapping_add(&two);
    /// let d = a.wrapping_add(&three);
    /// 
    /// println!("{} + 1 = {}", a, b);
    /// assert_eq!(b, u512::max());
    /// 
    /// println!("{} + 2 = {}", a, c);
    /// assert_eq!(c, zero);
    /// println!("{} + 3 = {}", a, d);
    /// assert_eq!(d, one);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn wrapping_add(&self, rhs: &Self) -> Self
    {
        let (res, _) = self.carrying_add(rhs, false); 
        res
    }

    // pub fn wrapping_add_assign(&mut self, rhs: &Self)
    /// Computes `self` + `rhs`, wrapping around at the boundary of the type,
    /// and assign the result to `self` back.
    /// 
    /// # Feature
    /// Wrapping (modular) addition.
    /// 
    /// # Counterpart Method
    /// The method `wrapping_add_assign_uint()` is a bit faster than this
    /// method `wrapping_add_assign()`. If `rhs` is primitive unsigned integral
    /// data type such as u8, u16, u32, u64, u128 and usize. use the mentod
    /// `wrapping_add_assign_uint()`.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let zero = u512::zero();
    /// let one = u512::one();
    /// 
    /// let mut a = u512::max().wrapping_sub(&one);
    /// println!("Originally,\ta = {}", a);
    /// assert_eq!(a.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    /// 
    /// a.wrapping_add_assign(&one);
    /// println!("After a += 1,\ta = {}", a);
    /// assert_eq!(a, u512::max());
    /// 
    /// a.wrapping_add_assign(&one);
    /// println!("After a += 1,\ta = {}", a);
    /// assert_eq!(a, zero);
    /// 
    /// a.wrapping_add_assign(&one);
    /// println!("After a += 1,\ta = {}", a);
    /// assert_eq!(a, one);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn wrapping_add_assign(&mut self, rhs: &Self)
    {
        self.carrying_add_assign(rhs, false);
    /*
        let zero = T::zero();
        let mut midres: T;
        let mut cc = zero;
        let mut c: bool;
        for i in 0..N
        {
            midres = self.number[i].wrapping_add(rhs.number[i]);
            c = midres < self.number[i];
            midres = midres.wrapping_add(carry);
            cc = if c || (midres < cc) { T::one() } else { zero };
            self.number[i] = midres;
        }
        if cc != zero
            { self.set_overflow(); }
    */
    }

    // pub fn overflowing_add(&self, rhs: &Self) -> (Self, bool)
    /// Calculates `self` + `rhs`.
    /// 
    /// # Output
    /// It returns a tuple of the addition `self` + `rhs` along with a boolean
    /// indicating whether an arithmetic overflow would occur. If an overflow
    /// would have occurred then the wrapped (modular) value is returned.
    /// 
    /// # Counterpart Method
    /// The method `overflowing_add_uint()` is a bit faster than this
    /// method `overflowing_add()`. If `rhs` is primitive unsigned integral
    /// data type such as u8, u16, u32, u64, u128 and usize. use the mentod
    /// `overflowing_add_uint()`.
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
    pub fn overflowing_add(&self, rhs: &Self) -> (Self, bool)
    {
        let mut res = self.clone();
        res.carrying_add(rhs, false)
    }

    // pub fn overflowing_add_assign(&mut self, rhs: &Self) -> bool
    /// Calculates `self` + `rhs`, and assigns the result to `self` back.
    /// 
    /// # Output
    /// It returns true if an arithmetic overflow would occur.
    /// Otherwise, it returns `false`.
    /// 
    /// # Counterpart Method
    /// The method `overflowing_add_assign_uint()` is a bit faster than this
    /// method `overflowing_add_assign()`. If `rhs` is primitive unsigned integral
    /// data type such as u8, u16, u32, u64, u128 and usize. use the mentod
    /// `overflowing_add_assign_uint()`.
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
    #[inline]
    pub fn overflowing_add_assign(&mut self, rhs: &Self) -> bool
    {
        self.carrying_add_assign(rhs, false)
    }

    // pub fn checked_add(&self, rhs: &Self) -> Option<Self>
    /// Computes `self` + `rhs`.
    /// 
    /// # Output
    /// It returns the sum `self` + `rhs` wrapped by `Some` of enum `Option`
    /// if overflow did not occur. Otherwise, it returns `None` of enum Option.
    /// 
    /// # Counterpart Method
    /// The method `checked_add_uint()` is a bit faster than this
    /// method `checked_add()`. If `rhs` is primitive unsigned integral
    /// data type such as u8, u16, u32, u64, u128 and usize. use the mentod
    /// `checked_add_uint()`.
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
    pub fn checked_add(&self, rhs: &Self) -> Option<Self>
    {
        let mut res = self.clone();
        let overflow = res.overflowing_add_assign(rhs);
        if overflow
            { None }
        else
            { Some(res) }
    }

    // pub fn unchecked_add(&self, rhs: &Self) -> Self
    /// Computes `self` + `rhs`, assuming overflow cannot occur.
    /// 
    /// # Panics
    /// If overflow occurred, it will panic. So, use this method only when you
    /// are sure that overflow will not occur. 
    /// 
    /// # Output
    /// It returns the sum `self` + `rhs` if overflow did not occur.
    /// Otherwise, it will panic.
    /// 
    /// # Counterpart Method
    /// The method `unchecked_add_uint()` is a bit faster than this
    /// method `unchecked_add()`. If `rhs` is primitive unsigned integral
    /// data type such as u8, u16, u32, u64, u128 and usize. use the mentod
    /// `unchecked_add_uint()`.
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
    #[inline]
    pub fn unchecked_add(&self, rhs: &Self) -> Self
    {
        self.checked_add(rhs).unwrap()
    }

    // pub fn saturating_add(&self, rhs: &Self) -> Self
    /// Computes `self` + `rhs`, saturating at the numeric bounds
    /// instead of overflowing.
    /// 
    /// # Output
    /// It returns the sum `self` + `rhs` if overflow did not occur.
    /// Otherwise, it returns the maximum value.
    /// 
    /// # Counterpart Method
    /// The method `saturating_add_uint()` is a bit faster than this
    /// method `saturating_add()`. If `rhs` is primitive unsigned integral
    /// data type such as u8, u16, u32, u64, u128 and usize. use the mentod
    /// `saturating_add_uint()`.
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
    pub fn saturating_add(&self, rhs: &Self) -> Self
    {
        let mut res = self.clone();
        res.saturating_add_assign(rhs);
        res
    }

    // pub fn saturating_add_assign(&mut self, rhs: &Self)
    /// Computes `self` + `rhs`, saturating at the numeric bounds
    /// instead of overflowing, and assigns the result to `self` back.
    /// 
    /// # Counterpart Method
    /// The method `saturating_add_assign_uint()` is a bit faster than this
    /// method `saturating_add_assign()`. If `rhs` is primitive unsigned integral
    /// data type such as u8, u16, u32, u64, u128 and usize. use the mentod
    /// `saturating_add_assign_uint()`.
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
    pub fn saturating_add_assign(&mut self, rhs: &Self)
    {
        if self.overflowing_add_assign(rhs)
            { self.set_max(); }
    }


    /*** Subtraction ***/

    // pub fn borrowing_sub(&self, rhs: &Self, borrow: bool) -> (Self, bool)
    /// Calculates self − rhs − borrow and returns a tuple containing the
    /// difference and the output borrow.
    /// 
    /// # Features
    /// It performs “ternary subtraction” by subtracting both an integer operand
    /// and a borrow-in bit from `self`, and returns an output integer and a
    /// borrow-out bit. This allows chaining together multiple subtractions to
    /// create a wider subtraction.
    /// 
    /// If the input carry is `false`, this method is equivalent to
    /// `overflowing_sub()`, and the output carry is equal to
    /// the underflow flag.
    /// 
    /// # Outputs
    /// It returns a tuple containing an output big integer and a carry-out bit.
    /// 
    /// # Counterpart Method
    /// The method `borrowing_sub_uint()` is a bit faster than this
    /// method `borrowing_sub()`. If `rhs` is primitive unsigned integral
    /// data type such as u8, u16, u32, u64, u128 and usize. use the mentod
    /// `borrowing_sub_uint()`.
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
    pub fn borrowing_sub(&self, rhs: &Self, borrow: bool) -> (Self, bool)
    {
        let mut res = self.clone();
        let b = res.borrowing_sub_assign(rhs, borrow);
        (res, b)
    }

    // pub fn borrowing_sub_assign(&mut self, rhs: &Self, borrow: bool) -> bool
    /// Calculates self − rhs − borrow, and assigns difference to `self` back,
    /// and returns the output borrow.
    /// 
    /// # Features
    /// It performs “ternary subtraction” by subtracting both an integer operand
    /// and a borrow-in bit from `self`, and a borrow-out bit. This allows
    /// chaining together multiple subtractions to create a wider subtraction.
    /// 
    /// If the input carry is `false`, this method is equivalent to
    /// `overflowing_sub_assign()`, and the output carry is equal to
    /// the underflow flag.
    /// 
    /// # Outputs
    /// It returns a carry-out bit.
    /// 
    /// # Counterpart Method
    /// The method `borrowing_sub_assign_uint()` is a bit faster than this
    /// method `borrowing_sub_assign()`. If `rhs` is primitive unsigned integral
    /// data type such as u8, u16, u32, u64, u128 and usize. use the mentod
    /// `borrowing_sub_assign_uint()`.
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
    pub fn borrowing_sub_assign(&mut self, rhs: &Self, borrow: bool) -> bool
    {
        let mut num: T;
        let mut	b = borrow;
        for i in 0..N
        {
            (num, b) = self.get_num_(i).borrowing_sub(rhs.get_num_(i), borrow);
            self.set_num_(i, num);
        }
        if b
            { self.set_underflow(); }
        b
    }

    // pub fn wrapping_sub(&self, rhs: &Self) -> Self
    /// Computes `self` - `rhs`, wrapping around at the boundary of the type.
    /// 
    /// # Output
    /// It returns `self` - `rhs` with wrapping (modular) subtraction.
    /// 
    /// # Feature
    /// Wrapping (modular) subtraction.
    /// 
    /// # Counterpart Method
    /// The method `wrapping_sub_uint()` is a bit faster than this
    /// method `wrapping_sub()`. If `rhs` is primitive unsigned integral
    /// data type such as u8, u16, u32, u64, u128 and usize. use the mentod
    /// `wrapping_sub_uint()`.
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
    pub fn wrapping_sub(&self, rhs: &Self) -> Self
    {
        let (res, _) = self.borrowing_sub(rhs, false);
        res
    }

    // pub fn wrapping_sub_assign(&mut self, rhs: &Self)
    /// Computes `self` - `rhs`, wrapping around at the boundary of the type,
    /// and assign the result to `self` back.
    /// 
    /// # Feature
    /// Wrapping (modular) subtraction.
    /// 
    /// # Counterpart Method
    /// The method `wwrapping_sub_assign_uint()` is a bit faster than this
    /// method `wrapping_sub_assign()`. If `rhs` is primitive unsigned integral
    /// data type such as u8, u16, u32, u64, u128 and usize. use the mentod
    /// `wwrapping_sub_assign_uint()`.
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
    #[inline]
    pub fn wrapping_sub_assign(&mut self, rhs: &Self)
    {
        self.borrowing_sub_assign(rhs, false);
    /*
       let zero = T::zero();
        let mut	carry: T = zero;
        let mut midres: T;
        let mut c: bool;
        let mut cc: T;

        for i in 0..N
        {
            midres = self.number[i].wrapping_sub(rhs.number[i]);
            c = midres > self.number[i];
            cc = midres;
            midres = midres.wrapping_sub(carry);
            carry = if c || (midres > cc) { T::one() } else { zero };
            self.number[i] = midres;
        }
        if carry != zero
            { self.set_underflow(); }
    */
    }

    // pub fn overflowing_sub(&self, rhs: &Self) -> (Self, bool)
    /// Calculates `self` - `rhs`.
    /// 
    /// # Output
    /// It returns a tuple of the subtraction `self` - `rhs` along with a boolean
    /// indicating whether an arithmetic unerflow would occur. If an unerflow
    /// would have occurred then the wrapped (modular) value is returned.
    /// 
    /// # Counterpart Method
    /// The method `overflowing_sub_uint()` is a bit faster than this
    /// method `overflowing_sub()`. If `rhs` is primitive unsigned integral
    /// data type such as u8, u16, u32, u64, u128 and usize. use the mentod
    /// `overflowing_sub_uint()`.
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
    #[inline]
    pub fn overflowing_sub(&self, rhs: &Self) -> (Self, bool)
    {
        self.borrowing_sub(rhs, false)
    }

    // pub fn overflowing_sub_assign(&mut self, rhs: &Self) -> bool
    /// Calculates `self` - `rhs`, and assigns the result to `self` back.
    /// 
    /// # Output
    /// It returns true if an arithmetic unerflow would occur.
    /// Otherwise, it returns `false`.
    /// 
    /// # Counterpart Method
    /// The method `overflowing_sub_assign_uint()` is a bit faster than this
    /// method `overflowing_sub_assign()`. If `rhs` is primitive unsigned
    /// integral data type such as u8, u16, u32, u64, u128 and usize.
    /// use the mentod `overflowing_sub_assign_uint()`.
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
    #[inline]
    pub fn overflowing_sub_assign(&mut self, rhs: &Self) -> bool
    {
        self.borrowing_sub_assign(rhs, false)
    }

    // pub fn checked_sub(&self, rhs: &Self) -> Option<Self>
    /// Computes `self` - `rhs`.
    /// 
    /// # Output
    /// It returns the difference `self` - `rhs` wrapped by `Some`
    /// of enum `Option` if unerflow did not occur.
    /// Otherwise, it returns `None` of enum Option.
    /// 
    /// # Counterpart Method
    /// The method `checked_sub_uint()` is a bit faster than this
    /// method `checked_sub()`. If `rhs` is primitive unsigned
    /// integral data type such as u8, u16, u32, u64, u128 and usize.
    /// use the mentod `checked_sub_uint()`.
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
    pub fn checked_sub(&self, rhs: &Self) -> Option<Self>
    {
        let mut res = self.clone();
        let underflow = res.overflowing_sub_assign(rhs);
        if underflow
            { None }
        else
            { Some(res) }
    }

    // pub fn unchecked_sub(&self, rhs: &Self) -> Self
    /// Computes `self` - `rhs`, assuming underflow cannot occur.
    /// 
    /// # Panics
    /// If underflow occurred, it will panic. So, use this method only when you
    /// are sure that underflow will not occur. 
    /// 
    /// # Output
    /// It returns the difference `self` - `rhs` if underflow did not occur.
    /// Otherwise, it will panic.
    /// 
    /// # Counterpart Method
    /// The method `unchecked_sub_uint()` is a bit faster than this
    /// method `unchecked_sub()`. If `rhs` is primitive unsigned
    /// integral data type such as u8, u16, u32, u64, u128 and usize.
    /// use the mentod `unchecked_sub_uint()`.
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
    #[inline]
    pub fn unchecked_sub(&self, rhs: &Self) -> Self
    {
        self.checked_sub(rhs).unwrap()
    }

    // pub fn saturating_sub(&self, rhs: &Self) -> Self
    /// Computes `self` - `rhs`, saturating at the numeric bounds
    /// instead of underflowing.
    /// 
    /// # Output
    /// It returns the difference `self` - `rhs` if underflowing did not occur.
    /// Otherwise, it returns `0`.
    /// 
    /// # Counterpart Method
    /// The method `saturating_sub_uint()` is a bit faster than this
    /// method `saturating_sub()`. If `rhs` is primitive unsigned
    /// integral data type such as u8, u16, u32, u64, u128 and usize.
    /// use the mentod `saturating_sub()`.
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
    pub fn saturating_sub(&self, rhs: &Self) -> Self
    {
        let mut res = self.clone();
        res.saturating_sub_assign(rhs);
        res
    }

    // pub fn saturating_sub_assign(&mut self, rhs: &Self)
    /// Computes `self` - `rhs`, saturating at the numeric bounds
    /// instead of underflowing, and assigns the result to `self` back.
    /// 
    /// # Feature
    /// `self` will be the difference `self` - `rhs` if underflowing
    /// did not occur. Otherwise, it returns `0`.
    /// 
    /// # Counterpart Method
    /// The method `saturating_sub_assign_uint()` is a bit faster than this
    /// method `saturating_sub_assign()`. If `rhs` is primitive unsigned
    /// integral data type such as u8, u16, u32, u64, u128 and usize.
    /// use the mentod `saturating_sub_assign_uint()`.
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
    pub fn saturating_sub_assign(&mut self, rhs: &Self)
    {
        if self.overflowing_sub_assign(rhs)
            { self.set_zero(); }
    }

    // pub fn abs_diff(&self, other: &Self) -> Self
    /// Computes the absolute difference between `self` and `other`.
    /// 
    /// # Output
    /// It returns the absolute difference between `self` and `other`.
    /// 
    /// # Counterpart Method
    /// The method `abs_diff_uint()` is a bit faster than this
    /// method `abs_diff()`. If `rhs` is primitive unsigned
    /// integral data type such as u8, u16, u32, u64, u128 and usize.
    /// use the mentod `abs_diff_uint()`.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use Cryptocol::number::BigUInt;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a = u256::from_str("500000000000000000500000000500000000500000000500000000").unwrap();
    /// let b = u256::from_str("500000000000000000000000000000000000000000000000000000").unwrap();
    /// let c = a.abs_diff(&b);
    /// let d = b.abs_diff(&a);
    /// println!("500000000000000000500000000500000000500000000500000000 <-> 500000000000000000000000000000000000000000000000000000 = {}", c);
    /// println!("500000000000000000000000000000000000000000000000000000 <-> 500000000000000000500000000500000000500000000500000000 = {}", d);
    /// assert_eq!(c, u256::from_str("500000000500000000500000000500000000").unwrap());
    /// assert_eq!(d, u256::from_str("500000000500000000500000000500000000").unwrap());
    /// ```
    pub fn abs_diff(&self, other: &Self) -> Self
    {
        if self < other
            { other.wrapping_sub(self) }
        else
            { self.wrapping_sub(other) }
    }


    /*** Multiplication ***/

    // pub fn carrying_mul(&self, rhs: &Self, carry: Self) -> (Self, Self)
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
    /// chaining together multiple multiplications to create “bigger integers”
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
    /// The method `carrying_mul_uint()` is a bit faster than this
    /// method `carrying_mul()`. If `rhs` is primitive unsigned
    /// integral data type such as u8, u16, u32, u64, u128 and usize.
    /// use the mentod `carrying_mul_uint()`.
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
    pub fn carrying_mul(&self, rhs: &Self, carry: Self) -> (Self, Self)
    {
        let mut low = self.clone();
        let high = low.carrying_mul_assign(rhs, carry);
        (low, high)
    }

    // pub fn carrying_mul_assign(&mut self, rhs: &Self, carry: Self) -> Self
    /// Calculates the “full multiplication” `self` * `rhs` + `carry` without
    /// the possibility to overflow, and assigs the low-order bits of the result
    /// to `self` back and returns the high-order bits of the result.
    /// 
    /// # Output
    /// It returns the high-order (overflow) bits of `self` * `rhs` + `carry`
    /// of the result.
    /// 
    /// # Feature
    /// It performs “long multiplication” which takes in an extra amount to add,
    /// and may return an additional amount of overflow. This allows for
    /// chaining together multiple multiplications to create “bigger integers”
    /// which represent larger values.
    /// 
    /// # Counterpart Methods
    /// If you don’t need the carry, then you can use `widening_mul_assign()`
    /// instead.
    /// 
    /// The value of `self` after calculation matches what you’d get by
    /// combining the `wrapping_mul()` and `wrapping_add_assign()` methods:
    /// `self.wrapping_mul(rhs).wrapping_add_assign(carry)`.
    /// 
    /// The method `carrying_mul_assign_uint()` is a bit faster than this
    /// method `carrying_mul_assign()`. If `rhs` is primitive unsigned
    /// integral data type such as u8, u16, u32, u64, u128 and usize.
    /// use the mentod `carrying_mul_assign_uint()`.
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
    pub fn carrying_mul_assign(&mut self, rhs: &Self, carry: Self) -> Self
    {
        let mut high = Self::zero();
        if rhs.is_zero()
        {
            self.set_zero();
            return high;
        }
        if self.is_zero()
            { return high; }

        let zero = T::zero();
        let one = T::one();
        let adder = self.clone();
        let TSIZE_BITS = T::size_in_bits();
        let mut multiply_first = |num: T| {
            let mut bit_check = one;
            bit_check <<= T::usize_as_Uint(TSIZE_BITS - 1);
            while (bit_check != zero) && (bit_check & num == zero)
                { bit_check >>= one; }

            self.set_zero();
            while bit_check != zero
            {
                *self <<= 1;
                high <<= 1;
                if bit_check & num != zero
                {
                    self.wrapping_add_assign(&adder);
                    if self.is_overflow()
                        { high.wrapping_add_uint(T::u8_as_Uint(1)); }
                }
                bit_check >>= one;
            }
        };

        let mut n = N - 1;
        while rhs.get_num_(n) == zero
            { n -= 1; }
        multiply_first(rhs.get_num_(n));
        if n == 0
            { return high; }
        n -= 1;

        let mut multiply = |num: T| {
            if num == T::zero()
            {
                *self <<= TSIZE_BITS as i32;
                high <<= TSIZE_BITS as i32;
                return;
            }
            let mut bit_check = one;
            bit_check <<= T::usize_as_Uint(TSIZE_BITS - 1);
            while bit_check != zero
            {
                *self <<= 1;
                high <<= 1;
                if self.is_overflow()
                    { high.set_num_(0, high.get_num_(0) | T::u8_as_Uint(1)) ; }
                if bit_check & num != zero
                {
                    self.wrapping_add_assign(&adder);
                    high.wrapping_add_uint(T::bool_as_Uint(self.is_overflow()));
                }
                bit_check >>= one;
            }
        };

        loop
        {
            multiply(rhs.get_num_(n));
            if n == 0
                { return high; }
            n = n.wrapping_sub(1);
        }
    }

    // pub fn widening_mul(&self, rhs: &Self) -> (Self, Self)
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
    /// chaining together multiple multiplications to create “bigger integers”
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
    /// The method `widening_mul_uint()` is a bit faster than this
    /// method `widening_mul()`. If `rhs` is primitive unsigned
    /// integral data type such as u8, u16, u32, u64, u128 and usize.
    /// use the mentod `widening_mul_uint()`.
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
    #[inline]
    pub fn widening_mul(&self, rhs: &Self) -> (Self, Self)
    {
        self.carrying_mul(rhs, Self::zero())
    }

    // pub fn widening_mul_assign(&mut self, rhs: &Self) -> Self
    /// Calculates the complete product `self` * `rhs` without the possibility
    /// to overflow.
    /// 
    /// # Output
    /// It returns the high-order (overflow) bits of the result `self` * `rhs`.
    /// 
    /// # Feature
    /// It performs “long multiplication” which takes in an extra amount to add,
    /// and may return an additional amount of overflow. This allows for
    /// chaining together multiple multiplications to create “bigger integers”
    /// which represent larger values.
    /// 
    /// # Counterpart Methods
    /// If you also need to add a carry to the wide result, then you want to use
    /// `carrying_mul_assign()` instead.
    ///     
    /// The value of `self` after calculation matches what you’d get the
    /// `wrapping_mul()` methods.
    /// `self` == `self.wrapping_mul(rhs)`
    /// 
    /// The method `widening_mul_assign_uint()` is a bit faster than this
    /// method `widening_mul_assign()`. If `rhs` is primitive unsigned
    /// integral data type such as u8, u16, u32, u64, u128 and usize.
    /// use the mentod `widening_mul_assign_uint()`.
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
    #[inline]
    pub fn widening_mul_assign(&mut self, rhs: &Self) -> Self
    {
        self.carrying_mul_assign(rhs, Self::zero())
    }

    // pub fn wrapping_mul(&self, rhs: &Self) -> Self
    /// Computes `self` * `rhs`, wrapping around at the boundary of the type.
    /// 
    /// # Output
    /// It returns `self` * `rhs` with wrapping (modular) addition.
    /// 
    /// # Feature
    /// Wrapping (modular) addition.
    /// 
    /// # Counterpart Method
    /// The method `wrapping_mul_uint()` is a bit faster than this
    /// method `wrapping_mul()`. If `rhs` is primitive unsigned
    /// integral data type such as u8, u16, u32, u64, u128 and usize.
    /// use the mentod `wrapping_mul_uint()`.
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
    pub fn wrapping_mul(&self, rhs: &Self) -> Self
    {
        let mut res = self.clone();
        res.wrapping_mul_assign(rhs);
        res
    }

    // pub fn wrapping_mul_assign(&mut self, rhs: &Self)
    /// Computes self * rhs, wrapping around at the boundary of the type,
    /// and assign the result to `self` back.
    /// 
    /// # Feature
    /// Wrapping (modular) multiplication.
    /// 
    /// # Counterpart Method
    /// The method `wrapping_mul_assign_uint()` is a bit faster than this
    /// method `wrapping_mul_assign()`. If `rhs` is primitive unsigned
    /// integral data type such as u8, u16, u32, u64, u128 and usize.
    /// use the mentod `wrapping_mul_assign_uint()`.
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
    pub fn wrapping_mul_assign(&mut self, rhs: &Self)
    {
        if rhs.is_zero()
        {
            self.set_zero();
            return;
        }
        if self.is_zero()
            { return; }

        let zero = T::zero();
        let one = T::one();
        let adder = self.clone();
        let TSIZE_BITS = size_of::<T>() * 8;
        let mut multiply_first = |num: T| {
            let mut bit_check = one;
            bit_check <<= T::usize_as_Uint(TSIZE_BITS - 1);
            while (bit_check != zero) && (bit_check & num == zero)
                { bit_check >>= one; }

            self.set_zero();
            while bit_check != zero
            {
                *self <<= 1;
                if bit_check & num != zero
                    { self.wrapping_add_assign(&adder); }
                bit_check >>= one;
            }
        };

        let mut n = N - 1;
        while rhs.get_num_(n) == zero
            { n -= 1; }
        multiply_first(rhs.get_num_(n));
        if n == 0
            { return; }
        n -= 1;

        let mut multiply = |num: T| {
            if num == T::zero()
            {
                *self <<= TSIZE_BITS as i32;
                return;
            }
            let mut bit_check = one;
            bit_check <<= T::usize_as_Uint(TSIZE_BITS - 1);
            while bit_check != zero
            {
                *self <<= 1;
                if bit_check & num != zero
                    { self.wrapping_add_assign(&adder); }
                bit_check >>= one;
            }
        };

        loop
        {
            multiply(rhs.get_num_(n));
            if n == 0
                { break; }
            n = n.wrapping_sub(1);
        }
    }

    // pub fn overflowing_mul(&self, rhs: &Self) -> (Self, bool)
    /// Calculates `self` * `rhs`.
    /// 
    /// # Output
    /// It returns a tuple of the multiplication `self` * `rhs` along
    /// with a boolean indicating whether an arithmetic overflow would occur.
    /// If an overflow would have occurred then the wrapped (modular) value
    /// is returned.
    /// 
    /// # Counterpart Method
    /// The method `overflowing_mul_uint()` is a bit faster than this
    /// method `overflowing_mul()`. If `rhs` is primitive unsigned
    /// integral data type such as u8, u16, u32, u64, u128 and usize.
    /// use the mentod `overflowing_mul_uint()`.
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
    pub fn overflowing_mul(&self, rhs: &Self) -> (Self, bool)
    {
        let mut res = self.clone();
        let overflow = res.overflowing_mul_assign(rhs);
        (res, overflow)
    }

    // pub fn overflowing_mul_assign(&mut self, rhs: &Self) -> bool
    /// Calculates `self` * `rhs`, and assigns the result to `self` back.
    /// 
    /// # Output
    /// It returns true if an arithmetic overflow would occur.
    /// Otherwise, it returns `false`.
    /// 
    /// # Counterpart Method
    /// The method `overflowing_mul_assign_uint()` is a bit faster than this
    /// method `overflowing_mul_assign()`. If `rhs` is primitive unsigned
    /// integral data type such as u8, u16, u32, u64, u128 and usize.
    /// use the mentod `overflowing_mul_assign_uint()`.
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
    pub fn overflowing_mul_assign(&mut self, rhs: &Self) -> bool
    {
        self.wrapping_mul_assign(rhs);
        self.is_overflow()
    }

    // pub fn checked_mul(&self, rhs: &Self) -> Option<Self>
    /// Computes `self` * `rhs`.
    /// 
    /// # Output
    /// It returns the sum `self` * `rhs` wrapped by `Some` of enum `Option`
    /// if overflow did not occur. Otherwise, it returns `None` of enum Option.
    /// 
    /// # Counterpart Method
    /// The method `checked_mul_uint()` is a bit faster than this
    /// method `checked_mul()`. If `rhs` is primitive unsigned
    /// integral data type such as u8, u16, u32, u64, u128 and usize.
    /// use the mentod `checked_mul_uint()`.
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
    pub fn checked_mul(&self, rhs: &Self) -> Option<Self>
    {
        let mut res = self.clone();
        let overflow = res.overflowing_mul_assign(rhs);
        if overflow
            { None }
        else
            { Some(res) }
    }

    // pub fn unchecked_mul(&self, rhs: &Self) -> Self
    /// Computes `self` * `rhs`, assuming overflow cannot occur.
    /// 
    /// # Panics
    /// If overflow occurred, it will panic. So, use this method only when you
    /// are sure that overflow will not occur. 
    /// 
    /// # Output
    /// It returns the sum `self` * `rhs` if overflow did not occur.
    /// Otherwise, it will panic.
    /// 
    /// # Counterpart Method
    /// The method `unchecked_mul_uint()` is a bit faster than this
    /// method `unchecked_mul()`. If `rhs` is primitive unsigned
    /// integral data type such as u8, u16, u32, u64, u128 and usize.
    /// use the mentod `unchecked_mul_uint()`.
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
    #[inline]
    pub fn unchecked_mul(&self, rhs: &Self) -> Self
    {
        self.checked_mul(rhs).unwrap()
    }

    // pub fn saturating_mul(&self, rhs: &Self) -> Self
    /// Computes `self` * `rhs`, saturating at the numeric bounds
    /// instead of overflowing.
    /// 
    /// # Output
    /// It returns the sum `self` + `rhs` if overflow did not occur.
    /// Otherwise, it returns the maximum value.
    /// 
    /// # Counterpart Method
    /// The method `saturating_mul_uint()` is a bit faster than this
    /// method `saturating_mul()`. If `rhs` is primitive unsigned
    /// integral data type such as u8, u16, u32, u64, u128 and usize.
    /// use the mentod `saturating_mul_uint()`.
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
    pub fn saturating_mul(&self, rhs: &Self) -> Self
    {
        let mut res = self.clone();
        res.saturating_mul_assign(rhs);
        res
    }

    // pub fn saturating_mul_assign(&mut self, rhs: &Self)
    /// Computes `self` * `rhs`, saturating at the numeric bounds
    /// instead of overflowing, and assigns the result to `self` back.
    /// 
    /// # Counterpart Method
    /// The method `saturating_mul_assign_uint()` is a bit faster than this
    /// method `saturating_mul_assign()`. If `rhs` is primitive unsigned
    /// integral data type such as u8, u16, u32, u64, u128 and usize.
    /// use the mentod `saturating_mul_assign_uint()`.
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
    pub fn saturating_mul_assign(&mut self, rhs: &Self)
    {
        if self.overflowing_mul_assign(rhs)
            { self.set_max(); }
    }


    /*** Division ***/

    // pub fn divide_fully(&self, rhs: &Self) -> (Self, Self)
    /// Divides `self` which is of `BigUInt` type by `rhs` which is of `BigUInt`
    /// type, and returns a tuple of quotient and remainder.
    /// 
    /// # Output
    /// It returns a tuple of quotient and remainder which are `BigUInt`type.
    /// 
    /// # Feature
    /// If `rhs` is zero, the quotient will have maximum value of `BigUInt`
    /// type, and the flags of quotient such as `OVERFLOW`, `INFINITY`, and
    /// `DIVIDED_BY_ZERO` will be set, and the remainder will be set to be
    /// zero of `BigUInt` type, the `DIVIDED_BY_ZERO` flag of remainder
    /// will be set.
    /// 
    /// # Counterpart Method
    /// The method `divide_fully_uint()` is a bit faster than this
    /// method `divide_fully()`. If `rhs` is primitive unsigned
    /// integral data type such as u8, u16, u32, u64, u128 and usize.
    /// use the mentod `divide_fully_uint()`.
    /// 
    /// # Examples
    /// ```
    /// use std::str::FromStr;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let dividend = u256::from_str("1234567890157589425462369896").unwrap();
    /// let divisor = u256::from_str("1234567890").unwrap();
    /// let (quotient, remainder) = dividend.divide_fully(&divisor);
    /// ```
    pub fn divide_fully(&self, rhs: &Self) -> (Self, Self)
    {
        let mut quotient = Self::zero();
        let one = T::one();
        if self.is_zero()
        {
            return (quotient, Self::zero());
        }
        else if rhs.is_zero()
        {
            quotient.set_max();
            quotient.set_infinity();
            quotient.set_divided_by_zero();
            quotient.set_overflow();
            let mut remainder = Self::zero();
            remainder.set_divided_by_zero();
            return (quotient, remainder);
        }
        else if *self < *rhs
        {
            return (quotient, self.clone());
        }
        else if *self == *rhs
        {
            quotient.set_num(0, one);
            return (quotient, Self::zero());
        }

        let mut adder = Self::zero();
        let mut highest = self.length_in_bits() - self.leading_zeros() as usize;
        let mut high = highest;
        let mut low = 0;
        let mut mid = (high + low) >> 1;
        let mut res;
        let mut sum;
        let maximum = Self::size_in_bits() - 1;
        loop
        {
            high = highest;
            low = 0;
            if high == 0
            {
                return (quotient.clone(), self.wrapping_sub(&quotient.wrapping_mul(rhs)));
            }
            else    // if high > 0
            {
                loop
                {
                    mid = (high + low) >> 1;
                    adder.turn_check_bits(mid);
                    sum = quotient.wrapping_add(&adder);
                    res = sum.wrapping_mul(rhs);
                    if !res.is_overflow() && (*self > res)
                    {
                        if mid == maximum
                        {
                            quotient = sum;
                            break;
                        }
                        else if mid == low
                        { 
                            quotient = sum;
                            if mid == 0
                                { highest = 0; }
                            break;
                        }
                        low = mid;
                    }
                    else if res.is_overflow() || (res > *self)
                    {
                        if mid == low
                        { 
                            highest = mid;
                            break;
                        }
                        high = mid;
                    }
                    else    // if res == *self
                        { return (quotient + adder, Self::zero()); }
                }
            }
        }
    }

    // pub fn wrapping_div(&self, rhs: &Self) -> Self
    /// Calculates the quotient when `self` is divided by `rhs`,
    /// which is `self` / `rhs`.
    ///
    /// # Output
    /// It returns the quotient of when `self` is divided by `rhs`,
    /// which is `self` / `rhs`. 
    /// 
    /// # Feature
    /// Wrapped division on `BigUInt` types is just normal division.
    /// There’s no way wrapping could ever happen. This function exists,
    /// so that all operations are accounted for in the wrapping operations.
    /// 
    /// If `rhs` is zero, the quotient will have maximum value of `BigUInt`
    /// type, and the flags of quotient such as `OVERFLOW`, `INFINITY`, and
    /// `DIVIDED_BY_ZERO` will be set. __It does not panic__ while the same
    /// named methods `wrapping_div()` for primitive integer data type such
    /// as u8, u16, u32, u64, etc. will panic if `rhs` is zero.
    /// 
    /// # Counterpart Method
    /// The method `wrapping_div_uint()` is a bit faster than this
    /// method `wrapping_div()`. If `rhs` is primitive unsigned
    /// integral data type such as u8, u16, u32, u64, u128 and usize.
    /// use the mentod `wrapping_div_uint()`.
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
    pub fn wrapping_div(&self, rhs: &Self) -> Self
    {
        let (quotient, _) = self.divide_fully(rhs);
        quotient
    }

    // pub fn wrapping_div_assign(&mut self, rhs: &Self)
    /// Calculates the quotient when `self` is divided by `rhs`,
    /// which is `self` / `rhs`, and assign the result to `self` back.
    /// 
    /// # Feature
    /// Wrapped division on `BigUInt` types is just normal division.
    /// There’s no way wrapping could ever happen. This function exists,
    /// so that all operations are accounted for in the wrapping operations.
    /// 
    /// If `rhs` is zero, the `self` will have maximum value of `BigUInt`
    /// type, and the flags of `self` such as `OVERFLOW`, `INFINITY`, and
    /// `DIVIDED_BY_ZERO` will be set. __It does not panic__ while the same
    /// kind methods `wrapping_div()` for primitive integer data type such
    /// as u8, u16, u32, u64, etc. will panic if `rhs` is zero.
    /// 
    /// # Counterpart Method
    /// The method `wrapping_div_assign_uint()` is a bit faster than this
    /// method `wrapping_div_assign()`. If `rhs` is primitive unsigned
    /// integral data type such as u8, u16, u32, u64, u128 and usize.
    /// use the mentod `wrapping_div_assign_uint()`.
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
    pub fn wrapping_div_assign(&mut self, rhs: &Self)
    {
        let (quotient, _) = self.divide_fully(rhs);
        *self = quotient;
    }

    // pub fn checked_div(&self, rhs: &Self) -> Option<Self>
    /// Calculates the quotient when `self` is divided by `rhs`,
    /// which is `self` / `rhs`.
    /// 
    /// # Output
    /// It returns `None` if `rhs` is zero. Otherwise, it returns the quotient
    /// of when `self` is divided by `rhs`, which is `self` / `rhs`,
    /// wrapped by `Some` of enum `Option`.
    /// 
    /// # Feature
    /// Wrapped division on `BigUInt` types is just normal division.
    /// There’s no way wrapping could ever happen.
    /// 
    /// If `rhs` is zero, the quotient will have maximum value of `BigUInt`
    /// type, and the flags of quotient such as `OVERFLOW`, `INFINITY`, and
    /// `DIVIDED_BY_ZERO` will be set.
    /// 
    /// # Counterpart Method
    /// The method `checked_div_uint()` is a bit faster than this
    /// method `checked_div()`. If `rhs` is primitive unsigned
    /// integral data type such as u8, u16, u32, u64, u128 and usize.
    /// use the mentod `checked_div_uint()`.
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
    pub fn checked_div(&self, rhs: &Self) -> Option<Self>
    {
        let res = self.wrapping_div(rhs);
        if res.is_divided_by_zero()
            { None }
        else
            { Some(res) }
    }

    // pub fn unchecked_div(&self, rhs: &Self) -> Self
    /// Calculates the quotient when `self` is divided by `rhs`,
    /// which is `self` / `rhs`, assuming that `rhs` cannot be zero.
    /// 
    /// # Panics
    /// If `rhs` is zero, it will panic. So, use this method only when you
    /// are sure that `rhs` is not zero. 
    /// 
    /// # Output
    /// It returns the quotient of when `self` is divided by `rhs`,
    /// which is `self` / `rhs` if `rhs` is not zero.
    /// Otherwise, it will panic.
    /// 
    /// # Feature
    /// Wrapped division on `BigUInt` types is just normal division.
    /// There’s no way wrapping could ever happen.
    /// 
    /// # Counterpart Method
    /// The method `unchecked_div_uint()` is a bit faster than this
    /// method `unchecked_div()`. If `rhs` is primitive unsigned
    /// integral data type such as u8, u16, u32, u64, u128 and usize.
    /// use the mentod `unchecked_div_uint()`.
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
    #[inline]
    pub fn unchecked_div(&self, rhs: &Self) -> Self
    {
        self.checked_div(rhs).unwrap()
    }

    // pub fn saturating_div(&self, rhs: &Self) -> Self
    /// Calculates the quotient when `self` is divided by `rhs`,
    /// which is `self` / `rhs`, saturating at the numeric bounds
    /// instead of overflowing.
    /// 
    /// # Output
    /// It returns the quotient of when `self` is divided by `rhs`,
    /// which is `self` / `rhs` if `rhs` is not zero.
    /// Otherwise, it returns the maximum value.
    /// 
    /// # Feature
    /// Overflow will not happen unless `rhs` is zero. __It does not panic__
    /// while the same named methods `saturating_div()` for primitive integer
    /// data type such as u8, u16, u32, u64, etc. will panic if `rhs` is zero.
    /// 
    /// If `rhs` is zero, the quotient will have maximum value of `BigUInt`
    /// type, and the flags of quotient such as `OVERFLOW` and `DIVIDED_BY_ZERO`
    /// will be set.
    /// 
    /// # Counterpart Method
    /// The method `saturating_div_uint()` is a bit faster than this
    /// method `saturating_div()`. If `rhs` is primitive unsigned
    /// integral data type such as u8, u16, u32, u64, u128 and usize.
    /// use the mentod `saturating_div_uint()`.
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
    pub fn saturating_div(&self, rhs: &Self) -> Self
    {
        let (mut quotient, _) = self.divide_fully(rhs);
        quotient.reset_inifinity();
        quotient
    }

    // pub fn saturating_div(&self, rhs: &Self) -> Self
    /// Calculates the quotient when `self` is divided by `rhs`,
    /// which is `self` / `rhs`, saturating at the numeric bounds
    /// instead of overflowing, and assigns the quotient to `self`.
    /// 
    /// # Feature
    /// Overflow will not happen unless `rhs` is zero. __It does not panic__
    /// while the similar methods `saturating_div()` for primitive integer
    /// data type such as u8, u16, u32, u64, etc. will panic if `rhs` is zero.
    /// 
    /// If `rhs` is zero, `self` will have maximum value of `BigUInt`
    /// type, and the flags of `self` such as `OVERFLOW` and `DIVIDED_BY_ZERO`
    /// will be set.
    /// 
    /// # Counterpart Method
    /// The method `saturating_div_assign_uint()` is a bit faster than this
    /// method `saturating_div_assign()`. If `rhs` is primitive unsigned
    /// integral data type such as u8, u16, u32, u64, u128 and usize.
    /// use the mentod `saturating_div_assign_uint()`.
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
    #[inline]
    pub fn saturating_div_assign(&mut self, rhs: &Self)
    {
        *self = self.saturating_div(rhs);
    }

    // pub fn wrapping_rem(&self, rhs: &Self) -> Self
    /// Calculates the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`.
    /// 
    /// # Output
    /// It returns the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`, with wrapping (modular) addition.
    /// 
    /// # Feature
    /// Wrapped remainder calculation on `BigUInt` types is just the regular
    /// remainder calculation. There’s no way wrapping could ever happen. This
    /// function exists, so that all operations are accounted for in the
    /// wrapping operations.
    /// 
    /// If `rhs` is zero, the remainder is zero and its `DIVIDED_BY_ZERO`
    /// is set. __It does not panic__ while the same named methods
    /// `wrapping_rem()` for primitive integer data type such as u8, u16,
    /// u32, u64, etc. will panic if `rhs` is zero.
    /// 
    /// # Counterpart Method
    /// The method `wrapping_rem_uint()` is a bit faster than this
    /// method `wrapping_rem()`. If `rhs` is primitive unsigned
    /// integral data type such as u8, u16, u32, u64, u128 and usize.
    /// use the mentod `wrapping_rem_uint()`.
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
    pub fn wrapping_rem(&self, rhs: &Self) -> Self
    {
        let (_, remainder) = self.divide_fully(rhs);
        remainder
    }

    // pub fn wrapping_rem_assign(&mut self, rhs: &Self)
    /// Calculates the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`, and returns the remainder to `self` back.
    /// 
    /// # Feature
    /// Wrapped remainder calculation on `BigUInt` types is just the regular
    /// remainder calculation. There’s no way wrapping could ever happen. This
    /// function exists, so that all operations are accounted for in the
    /// wrapping operations.
    /// 
    /// If `rhs` is zero, the `self` is zero and its `DIVIDED_BY_ZERO`
    /// is set. __It does not panic__ while the similar methods
    /// `wrapping_rem()` for primitive integer data type such as 
    /// u8, u16, u32, u64, etc. will panic if `rhs` is zero.
    /// 
    /// # Counterpart Method
    /// The method `wrapping_rem_assign_uint()` is a bit faster than this
    /// method `wrapping_rem_assign()`. If `rhs` is primitive unsigned
    /// integral data type such as u8, u16, u32, u64, u128 and usize.
    /// use the mentod `wrapping_rem_assign_uint()`.
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
    pub fn wrapping_rem_assign(&mut self, rhs: &Self)
    {
        let (_, remainder) = self.divide_fully(rhs);
        *self = remainder;
    }

    // pub fn overflowing_rem(self, rhs: &Self) -> (Self, bool)
    /// Calculates the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`.
    /// 
    /// # Output
    /// It returns a tuple of the remainder after dividing,
    /// which is `self` % `rhs` along with a boolean indicating whether an
    /// arithmetic overflow would occur.
    /// 
    /// # Feature
    /// Note that overflow never occurs, so the second value is always false.
    /// 
    /// If `rhs` is zero, the remainder is zero and its `DIVIDED_BY_ZERO`
    /// is set. __It does not panic__ while the same named methods
    /// `overflowing_rem()` for primitive integer data type such as u8, u16,
    /// u32, u64, etc. will panic if `rhs` is zero.
    /// 
    /// # Counterpart Method
    /// The method `overflowing_rem_uint()` is a bit faster than this
    /// method `overflowing_rem()`. If `rhs` is primitive unsigned
    /// integral data type such as u8, u16, u32, u64, u128 and usize.
    /// use the mentod `overflowing_rem_uint()`.
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
    pub fn overflowing_rem(&self, rhs: &Self) -> (Self, bool)
    {
        let (_, remainder) = self.divide_fully(rhs);
        (remainder, false)
    }

    // pub fn overflowing_rem_assign(&mut self, rhs: &Self) -> bool
    /// Calculates the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`, and returns the remainder to `self` back.
    /// 
    /// # Output
    /// It returns a boolean indicating whether an arithmetic overflow
    /// would occur.
    /// 
    /// # Feature
    /// Note that overflow never occurs, so the outtput is always false.
    /// 
    /// If `rhs` is zero, `self` is zero and its `DIVIDED_BY_ZERO`
    /// is set. __It does not panic__ while the similar methods
    /// `overflowing_rem()` for primitive integer data type such as
    /// u8, u16, u32, u64, etc. will panic if `rhs` is zero.
    /// 
    /// # Counterpart Method
    /// The method `overflowing_rem_assign_uint()` is a bit faster than this
    /// method `overflowing_rem_assign()`. If `rhs` is primitive unsigned
    /// integral data type such as u8, u16, u32, u64, u128 and usize.
    /// use the mentod `overflowing_rem_assign_uint()`.
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
    pub fn overflowing_rem_assign(&mut self, rhs: &Self) -> bool
    {
        let (_, remainder) = self.divide_fully(rhs);
        *self = remainder;
        false
    }

    // pub fn checked_rem(&self, rhs: &Self) -> Option<Self>
    /// Calculates the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`.
    /// 
    /// # Output
    /// It returns the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`, wrapped by `Some` of enum `Option`
    /// if `rhs` is not zero. Otherwise, it returns `None` of enum Option.
    /// 
    /// # Feature
    /// Note that overflow never occurs.
    /// 
    /// # Counterpart Method
    /// The method `checked_rem_uint()` is a bit faster than this
    /// method `checked_rem()`. If `rhs` is primitive unsigned
    /// integral data type such as u8, u16, u32, u64, u128 and usize.
    /// use the mentod `checked_rem_uint()`.
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
    pub fn checked_rem(&self, rhs: &Self) -> Option<Self>
    {
        let (_, remainder) = self.divide_fully(rhs);
        if remainder.is_divided_by_zero()
            { None }
        else
            { Some(remainder) }
    }

    // pub fn unchecked_rem(&self, rhs: &Self) -> Self
    /// Calculates the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`, assuming `rhs` cannot be zero.
    /// 
    /// # Output
    /// It returns the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`, if `rhs` is not zero.
    /// Otherwise, it returns zero.
    /// 
    /// # Feature
    /// Note that overflow never occurs.
    /// 
    /// # Counterpart Method
    /// The method `unchecked_rem_uint()` is a bit faster than this
    /// method `unchecked_rem()`. If `rhs` is primitive unsigned
    /// integral data type such as u8, u16, u32, u64, u128 and usize.
    /// use the mentod `unchecked_rem_uint()`.
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
    #[inline]
    pub fn unchecked_rem(&self, rhs: &Self) -> Self
    {
        self.checked_rem(rhs).unwrap()
    }

    // pub fn saturating_rem(&self, rhs: &Self) -> Self
    /// Calculates the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`
    /// 
    /// # Output
    /// It returns the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`, if `rhs` is not zero.
    /// Otherwise, it returns zero.
    /// 
    /// # Feature
    /// If `rhs` is zero, the remainder will have zero of`BigUInt` type,
    /// and `DIVIDED_BY_ZERO` flag of the remainder will be set, and
    /// the remainder will be set to be zero of `BigUInt` type.
    /// 
    /// Note that overflow never occurs.
    /// 
    /// # Counterpart Method
    /// The method `saturating_rem_uint()` is a bit faster than this
    /// method `saturating_rem()`. If `rhs` is primitive unsigned
    /// integral data type such as u8, u16, u32, u64, u128 and usize.
    /// use the mentod `saturating_rem_uint()`.
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
    pub fn saturating_rem(&self, rhs: &Self) -> Self
    {
        let (_, remainder) = self.divide_fully(&rhs);
        remainder
    }

    // pub fn saturating_rem_assign(&mut self, rhs: &Self)
    /// Calculates the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`, and assigns the remainder to `self` back.
    /// 
    /// # Feature
    /// If `rhs` is zero, `self` will have zero of`BigUInt` type,
    /// and `DIVIDED_BY_ZERO` flag of `self` will be set, and
    /// `self` will be set to be zero of `BigUInt` type.
    /// 
    /// Note that overflow never occurs.
    /// 
    /// # Counterpart Method
    /// The method `saturating_rem_assign_uint()` is a bit faster than this
    /// method `saturating_rem_assign()`. If `rhs` is primitive unsigned
    /// integral data type such as u8, u16, u32, u64, u128 and usize.
    /// use the mentod `saturating_rem_assign_uint()`.
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
    #[inline]
    pub fn saturating_rem_assign(&mut self, rhs: &Self)
    {
        *self = self.saturating_rem(rhs);
    }


    // pub fn next_multiple_of(&self, rhs: &Self) -> Self
    /// Calculates the smallest value greater than or equal to `self` that is
    /// a multiple of `rhs`.
    /// 
    /// # Panics
    /// - This function will panic if rhs is zero.
    /// - If size_of::<T>() * N <= 128, This method may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the smallest value greater than or equal to self that is
    /// a multiple of `rhs`. However, if overflow occurs, it returns the value
    /// wrapped around.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn next_multiple_of(&self, rhs: &Self) -> Self
    {
        let mut res = self.clone();
        res.next_multiple_of_assign(&rhs);
        res
    }

    // pub fn next_multiple_of_assign(&mut self, rhs: &Self)
    /// Calculates the smallest value greater than or equal to `self` that is
    /// a multiple of `rhs`, and assigns the result to `self` back.
    /// 
    /// # Panics
    /// - This function will panic if rhs is zero.
    /// - If size_of::<T>() * N <= 128, This method may panic or its behavior may
    /// undefined though it may not panic.
    /// 
    /// # Feature
    /// `self` will be the smallest value greater than or equal to self that is
    /// a multiple of `rhs`. However, if overflow occurs, `self` will be the
    /// value wrapped around.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn next_multiple_of_assign(&mut self, rhs: &Self)
    {
        if rhs.is_zero()
            { panic!(); }

        let r = self.wrapping_rem(rhs);
        if !r.is_zero()
        {
            self.wrapping_add_assign(&rhs.wrapping_sub(&r));
        }
    }

    // pub fn next_power_of_two(&self) -> Self
    /// Returns the smallest power of two greater than or equal to `self`.
    /// 
    /// # Output
    /// It returns the smallest power of two greater than or equal to `self`.
    /// 
    /// # Feature
    /// When return value overflows (i.e., self > (1 << (size_of::<T>() * N - 1))),
    /// it returns value wrapped around.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn next_power_of_two(&self) -> Self
    {
        let mut res = self.clone();
        res.next_power_of_two_assign();
        res
    }

    // pub fn next_power_of_two_assign(&mut self)
    /// Finds the smallest power of two greater than or equal to `self`,
    /// and returns the result to `self` back.
    /// 
    /// # Feature
    /// When the result overflows
    /// (i.e., self > (1 << (size_of::<T>() * N - 1))),
    /// it `self` will be the value wrapped around.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn next_power_of_two_assign(&mut self)
    {
        if !self.is_power_of_two()
        {
            let bit_pos = Self::size_in_bits() - 1 - self.leading_zeros() as usize;
            self.turn_check_bits(bit_pos);
            self.shift_left(1_u8);
        }
    }

    // pub fn is_power_of_two(&self) -> bool
    /// Returns true if and only if self == 2 ** k for some k.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    pub fn is_power_of_two(&self) -> bool
    {
        self.count_ones() == 1
    }

    // pub fn midpoint(&self, rhs: &Self) -> Self
    /// Calculates the middle point of `self` and `rhs`.
    /// 
    /// # Feature
    /// midpoint(&a, &b) is (a + b) >> 1 as if it were performed in
    /// a sufficiently-large signed integral type.
    /// 
    /// This implies that the result is always rounded towards negative infinity
    /// and that no overflow will ever occur.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    pub fn midpoint(&self, rhs: &Self) -> Self
    {
        self.wrapping_add(&rhs) >> 1
    }

    pub fn gcd(&self, other: &Self) -> Self
    {
        if self.is_zero() || other.is_zero()
            { panic!(); }
        let mut x = self.clone();
        let mut y = Self::from_biguint(other);
        let mut t: Self;
        while !y.is_zero()
        {
            t = y.clone();
            y = x.wrapping_rem(&y);
            x = t;
        }
        x
     }



    /***** METHODS FOR EXPONENTIATION AND LOGARITHM *****/

    // pub fn pow(&mut self, exp: &Self) -> Self
    /// Raises `BigUInt` type number to the power of exp, using exponentiation
    /// of type `BigUInt` by squaring, wrapping around at the boundary of the
    /// type.
    /// 
    /// # Output
    /// It returns the result of `self` raised to the power of `exp`.
    /// 
    /// # Argument
    /// The argument `exp` is the type `BigUInt`.
    /// 
    /// # Feature
    /// Wrapping (modular) exponentiation. It calls wrapping_pow() internally.
    /// 
    /// # Counterpart Method
    /// The method `pow_uint()` is more efficient than this method `pow()`
    /// when the exponent `exp` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, u128 and usize. If `rhs` is the primitive
    /// unsigned integral data type number, use the mentod `pow_uint()`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a = u256::from_uint(234_u8);
    /// let mut exp = u256::from_uint(34_u8);
    /// 
    /// // normal exponentiation
    /// let b = a.pow(&exp);
    /// println!("234 ** 34 = {}", b);
    /// assert_eq!(b.to_string(), "101771369680718065636717400052436696519017873276976456689251925337442881634304");
    /// 
    /// // wrapping (modular) exponentiation
    /// exp += 1;
    /// let c = a.pow(&exp);
    /// println!("234 ** 35 = {}", c);
    /// assert_eq!(c.to_string(), "77122211638207297159819685489165875529835490356175237196145807339442726240256");
    /// 
    /// // evidence of wrapping (modular) exponentiation
    /// assert!(b > c);
    /// ```
    #[inline]
    pub fn pow(&self, exp: &Self) -> Self
    {
        self.wrapping_pow(exp)
    }

    // pub fn pow_assign(&mut self, exp: &Self)
    /// Raises `BigUInt` type number to the power of exp, using exponentiation
    /// of type `BigUInt` by squaring, wrapping around at the boundary of the
    /// type, and assign the result to `self` back.
    /// 
    /// # Argument
    /// The argument `exp` is the type `BigUInt`.
    /// 
    /// # Feature
    /// Wrapping (modular) exponentiation. It calls wrapping_pow() internally.
    /// 
    /// # Counterpart Method
    /// The method `pow_assign_uint()` is more efficient than this method
    /// `pow_assign()` when the exponent `exp` is primitive unsigned integral
    /// data type such as u8, u16, u32, u64, u128 and usize. If `rhs` is the
    /// primitive unsigned integral data type number, use the mentod
    /// `pow_assign_uint()`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = u256::from_uint(234_u8);
    /// let mut exp = u256::from_uint(34_u8);
    /// 
    /// // normal exponentiation
    /// a.pow_assign(&exp);
    /// println!("234 ** 34 = {}", a);
    /// assert_eq!(a.to_string(), "101771369680718065636717400052436696519017873276976456689251925337442881634304");
    /// 
    /// // wrapping (modular) exponentiation
    /// let old = a.clone();
    /// a = u256::from_uint(234_u8);
    /// exp += 1;
    /// a.pow_assign(&exp);
    /// println!("234 ** 35 = {}", a);
    /// assert_eq!(a.to_string(), "77122211638207297159819685489165875529835490356175237196145807339442726240256");
    /// 
    /// // evidence of wrapping (modular) exponentiation
    /// assert!(old > a);
    /// ```
    #[inline]
    pub fn pow_assign(&mut self, exp: &Self)
    {
        self.wrapping_pow_assign(exp);
    }

    // pub fn wrapping_pow(&mut self, exp: &Self) -> Self
    /// Raises `BigUInt` type number to the power of exp, using exponentiation
    /// of type `BigUInt` by squaring, wrapping around at the boundary of the
    /// type.
    /// 
    /// # Output
    /// It returns the result of `self` raised to the power of `exp`.
    /// 
    /// # Argument
    /// The argument `exp` is the type `BigUInt`.
    /// 
    /// # Feature
    /// Wrapping (modular) exponentiation.
    /// 
    /// # Counterpart Method
    /// The method `wrapping_pow_uint()` is a bit faster than this method
    /// `wrapping_pow()` when the exponent `exp` is primitive unsigned integral
    /// data type such as u8, u16, u32, u64, u128 and usize. If `rhs` is the
    /// primitive unsigned integral data type number, use the mentod
    /// `wrapping_pow_uint()`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a = u256::from_uint(234_u8);
    /// let mut exp = u256::from_uint(34_u8);
    /// 
    /// // normal exponentiation
    /// let b = a.wrapping_pow(&exp);
    /// println!("234 ** 34 = {}", b);
    /// assert_eq!(b.to_string(), "101771369680718065636717400052436696519017873276976456689251925337442881634304");
    /// 
    /// // wrapping (modular) exponentiation
    /// exp += 1;
    /// let c = a.wrapping_pow(&exp);
    /// println!("234 ** 35 = {}", c);
    /// assert_eq!(c.to_string(), "77122211638207297159819685489165875529835490356175237196145807339442726240256");
    /// 
    /// // evidence of wrapping (modular) exponentiation
    /// assert!(b > c);
    /// ```
    pub fn wrapping_pow(&self, exp: &Self) -> Self
    {
        let mut res = self.clone();
        res.wrapping_pow_assign(exp);
        res
    }

    // pub fn wrapping_pow_assign(&mut self, exp: &Self)
    /// Raises `BigUInt` type number to the power of exp, using exponentiation
    /// of type `BigUInt` by squaring, wrapping around at the boundary of the
    /// type, and assign the result to `self` back.
    /// 
    /// # Argument
    /// The argument `exp` is the type `BigUInt`.
    /// 
    /// # Feature
    /// Wrapping (modular) exponentiation.
    /// 
    /// # Counterpart Method
    /// The method `wrapping_pow_assign_uint()` is a bit faster than this method
    /// `wrapping_pow_assign()` when the exponent `exp` is primitive unsigned
    /// integral data type such as u8, u16, u32, u64, u128 and usize. If `rhs`
    /// is the primitive unsigned integral data type number, use the mentod
    /// `wrapping_pow_assign_uint()`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = u256::from_uint(234_u8);
    /// let mut exp = u256::from_uint(34_u8);
    /// 
    /// // normal exponentiation
    /// a.wrapping_pow_assign(&exp);
    /// println!("234 ** 34 = {}", a);
    /// assert_eq!(a.to_string(), "101771369680718065636717400052436696519017873276976456689251925337442881634304");
    /// 
    /// // wrapping (modular) exponentiation
    /// let old = a.clone();
    /// a = u256::from_uint(234_u8);
    /// exp += 1;
    /// a.wrapping_pow_assign(&exp);
    /// println!("234 ** 35 = {}", a);
    /// assert_eq!(a.to_string(), "77122211638207297159819685489165875529835490356175237196145807339442726240256");
    /// 
    /// // evidence of wrapping (modular) exponentiation
    /// assert!(old > a);
    /// ```
    pub fn wrapping_pow_assign(&mut self, exp: &Self)
    {
        if self.is_zero() || self.is_one()
            { return; }

        let multiplier = self.clone();
        self.set_one();
        if exp.is_zero()
            { return; }

        let mut bit_check = Self::one();
        bit_check.shift_left_assign(exp.length_in_bits() - exp.leading_zeros() as usize - 1);
        if !bit_check.is_zero()
        {
            self.wrapping_mul_assign(&multiplier); 
            bit_check.shift_right_assign(1_u8);
        }
        while !bit_check.is_zero()
        {
            *self = self.wrapping_mul(self);
            if !(bit_check.and(exp).is_zero())
                { self.wrapping_mul_assign(&multiplier); }
            bit_check.shift_right_assign(1_u8);
        }
    }

    // pub fn overflowing_pow(&self, exp: &Self) -> (Self, bool)
    /// Raises `BigUInt` type number to the power of exp, using exponentiation
    /// of type `BigUInt` by squaring, wrapping around at the boundary of the
    /// type.
    /// 
    /// # Output
    /// It returns the result of `self` raised to the power of `exp`.
    /// 
    /// # Argument
    /// The argument `exp` is the type `BigUInt`.
    /// 
    /// # Feature
    /// Wrapping (modular) exponentiation.
    /// 
    /// # Counterpart Method
    /// The method `overflowing_pow_uint()` is a bit faster than this method
    /// `overflowing_pow()` when the exponent `exp` is primitive unsigned
    /// integral data type such as u8, u16, u32, u64, u128 and usize. If `rhs`
    /// is the primitive unsigned integral data type number,
    /// use the mentod `overflowing_pow_uint()`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a = u256::from_uint(234_u8);
    /// let mut exp = u256::from_uint(34_u8);
    /// 
    /// // normal exponentiation
    /// let (b, bb) = a.overflowing_pow(&exp);
    /// println!("234 ** 34 = {}, overflow = {}", b, bb);
    /// assert_eq!(b.to_string(), "101771369680718065636717400052436696519017873276976456689251925337442881634304");
    /// assert_eq!(bb, false);
    /// 
    /// // wrapping (modular) exponentiation
    /// exp += 1;
    /// let (c, cc) = a.overflowing_pow(&exp);
    /// println!("234 ** 35 = {}, overflow = {}", c, cc);
    /// assert_eq!(c.to_string(), "77122211638207297159819685489165875529835490356175237196145807339442726240256");
    /// assert_eq!(cc, true);
    /// 
    /// // evidence of wrapping (modular) exponentiation
    /// assert!(b > c);
    /// ```
    pub fn overflowing_pow(&self, exp: &Self) -> (Self, bool)
    {
        let mut res = self.clone();
        let overflow = res.overflowing_pow_assign(exp);
        (res, overflow)
    }

    // pub fn overflowing_pow_assign(&mut self, exp: &Self) -> bool
    /// Raises `BigUInt` type number to the power of exp, using exponentiation
    /// of type `BigUInt` by squaring, wrapping around at the boundary of the
    /// type, and assign the result to `self` back, and returns a tuple of the
    /// exponentiation along with a bool indicating whether an overflow happened.
    /// 
    /// # Output
    /// It returns bool indicating whether an overflow happened.
    /// It returns `true` if overflow happened. Otherwise, it returns `false`.
    /// 
    /// # Argument
    /// The argument `exp` is the type `BigUInt`.
    /// 
    /// # Feature
    /// Wrapping (modular) exponentiation.
    /// 
    /// # Counterpart Method
    /// The method `overflow_pow_assign_uint()` is a bit faster than this method
    /// `overflow_pow_assign()` when the exponent `exp` is primitive unsigned
    /// integral data type such as u8, u16, u32, u64, u128 and usize. If `rhs`
    /// is the primitive unsigned integral data type number, use the mentod
    /// `overflow_pow_assign_uint()`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = u256::from_uint(234_u8);
    /// let mut exp = u256::from_uint(34_u8);
    /// 
    /// // normal exponentiation
    /// let mut aa = a.overflowing_pow_assign(&exp);
    /// println!("234 ** 34 = {}, overflow = {}", a, aa);
    /// assert_eq!(a.to_string(), "101771369680718065636717400052436696519017873276976456689251925337442881634304");
    /// assert_eq!(aa, false);
    /// 
    /// // wrapping (modular) exponentiation
    /// let old = a.clone();
    /// a = u256::from_uint(234_u8);
    /// exp += 1;
    /// aa = a.overflowing_pow_assign(&exp);
    /// println!("234 ** 35 = {}, overflow = {}", a, aa);
    /// assert_eq!(a.to_string(), "77122211638207297159819685489165875529835490356175237196145807339442726240256");
    /// assert_eq!(aa, true);
    /// 
    /// // evidence of wrapping (modular) exponentiation
    /// assert!(old > a);
    /// ```
    pub fn overflowing_pow_assign(&mut self, exp: &Self) -> bool
    {
        self.wrapping_pow_assign(exp);
        self.is_overflow()
    }

    // pub fn checked_pow(&self, exp: &Self) -> Option<Self>
    /// Raises `BigUInt` type number to the power of exp, using exponentiation
    /// of type `BigUInt` by squaring, wrapping around at the boundary of the
    /// type, returning None if overflow occurred.
    /// 
    /// # Output
    /// It returns the result of `self` raised to the power of `exp`, wrapped in
    /// `Some` of enum `Option` if overflow did not happen.
    /// Otherwise, it returns `None`.
    /// 
    /// # Argument
    /// The argument `exp` is the type `BigUInt`.
    /// 
    /// # Feature
    /// Checked wrapping (modular) exponentiation. 
    /// 
    /// # Counterpart Method
    /// The method `checked_pow_uint()` is a bit faster than this method
    /// `checked_pow()` when the exponent `exp` is primitive unsigned integral
    /// data type such as u8, u16, u32, u64, u128 and usize. If `rhs` is the
    /// primitive unsigned integral data type number, use the mentod
    /// `checked_pow_uint()`.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a = u256::from_uint(234_u8);
    /// let mut exp = u256::from_uint(34_u8);
    /// 
    /// // normal exponentiation
    /// let b = a.checked_pow(&exp);
    /// println!("234 ** 34 = {}", b.as_ref().unwrap());
    /// assert_eq!(b.unwrap().to_string(), "101771369680718065636717400052436696519017873276976456689251925337442881634304");
    /// 
    /// // wrapping (modular) exponentiation
    /// exp += 1;
    /// let c = a.checked_pow(&exp);
    /// println!("234 ** 35 = {}", c.as_ref().unwrap());
    /// assert_eq!(c, None);
    /// ```
    pub fn checked_pow(&self, exp: &Self) -> Option<Self>
    {
        let mut res = self.clone();
        let overflow = res.overflowing_pow_assign(exp);
        if overflow
            { None }
        else
            { Some(res) }
    }

    #[inline]
    pub fn unchecked_pow(&self, exp: &Self) -> Self
    {
        self.checked_pow(&exp).unwrap()
    }

    pub fn saturating_pow(&self, exp: &Self) -> Self
    {
        let mut res = self.clone();
        res.saturating_pow_assign(exp);
        res
    }

    pub fn saturating_pow_assign(&mut self, exp: &Self)
    {
        if self.overflowing_pow_assign(&exp)
            { self.set_max(); }
    }

    // pub fn ilog(&self, base: Self) -> Self
    /// Calculates the logarithm of the number with respect to a `base`.
    /// 
    /// # Output
    /// It returns the logarithm of the number with respect to an arbitrary
    /// `base`, rounded down.
    /// 
    /// # Counterpart Methods
    /// This method might not be optimized owing to implementation details;
    /// `ilog2()` can produce results more efficiently for base 2,
    /// and `ilog10` can produce results more efficiently for base 10.
    /// 
    /// # Panics
    /// This function will panic if `self` is zero, or if `base` is less than 2.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn ilog(&self, base: Self) -> Self
    {
        if self.is_zero() || (base < Self::from_uint(2_u8))
            { panic!() }
        let mut count = 0_usize;
        let mut quotient = self.clone();
        quotient.wrapping_div_assign(&base);
        while !quotient.is_zero()
        {
            count += 1;
            quotient.wrapping_div_assign(&base)
        }
        Self::from_uint(count)
    }

    // pub fn ilog2(&self) -> Self
    /// Calculates Returns the base 2 logarithm of the number.
    /// 
    /// # Output
    /// It returns the base 2 logarithm of the number, rounded down.
    /// 
    /// # Panics
    /// This function will panic if `self` is zero.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a = u256::from_uint(2_u8);
    /// let b = a.ilog2();
    /// println!("log_2(2) = {}", b);
    /// assert_eq!(b, u256::from_uint(1_u8));
    /// ```
    pub fn ilog2(&self) -> Self
    {
        if self.is_zero()
            { panic!() }
        Self::from_uint(self.length_in_bits() - self.leading_zeros() as usize - 1)
    }

    #[inline]
    pub fn ilog10(&self) -> Self
    {
        self.ilog_uint(10_u8)
    }

    // pub fn checked_ilog(&self, base: Self) -> Self
    /// Calculates the logarithm of the number with respect to a `base`.
    /// 
    /// # Output
    /// It returns the logarithm of the number with respect to an arbitrary
    /// `base`, rounded down, wrapped by `Some` of enum `Option`.
    /// It returns `None` if `self` is zero, or if `base` is less than 2.
    /// 
    /// # Counterpart Methods
    /// This method might not be optimized owing to implementation details;
    /// `checked_ilog2()` can produce results more efficiently for base 2,
    /// and `checked_ilog10` can produce results more efficiently for base 10.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn checked_ilog(&self, base: Self) -> Option<Self>
    {
        if self.is_zero() || (base < Self::from_uint(2_u8))
            { return None; }
        let mut count = 0_usize;
        let mut quotient = self.clone();
        quotient.wrapping_div_assign(&base);
        while !quotient.is_zero()
        {
            count += 1;
            quotient.wrapping_div_assign(&base)
        }
        Some(Self::from_uint(count))
    }

    // pub fn checked_ilog2(&self) -> Self
    /// Calculates Returns the base 2 logarithm of the number.
    /// 
    /// # Output
    /// It returns the base 2 logarithm of the number, rounded down,
    /// wrapped by `Some` of enum `Option`. It returns `None` if `self` is zero.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a = u256::from_uint(2_u8);
    /// let b = a.ilog2();
    /// println!("log_2(2) = {}", b);
    /// assert_eq!(b, u256::from_uint(1_u8));
    /// ```
    pub fn checked_ilog2(&self) -> Option<Self>
    {
        if self.is_zero()
            { return None; }
        Some(Self::from_uint(self.length_in_bits() - self.leading_zeros() as usize - 1))
    }

    #[inline]
    pub fn checked_ilog10(&self) -> Option<Self>
    {
        self.checked_ilog_uint(10_u8)
    }

    // pub fn wrapping_pow() -> Self
    // pub fn wrapping_pow_assign()
    // pub fn overflowing_pow() -> (Self, bool)
    // pub fn overflowing_pow_assign() -> bool
    // pub fn checked_pow() -> Option<Self>
    // pub fn unchecked_pow() -> Self
    // pub fn saturating_pow() -> Self
    // pub fn saturating_pow_assign()


    /***** METHODS FOR BIT OPERATION *****/


    // pub fn shift_left<U>(&self, n: U) -> Self
    /// Shift left the field `number: [T;N]` to the left by `n`.
    /// 
    /// # Feature
    /// 'Shift left' means 'move left all bits'. So, if `10011010` is shifted
    /// left by 2, it will be `01101000`, for example.
    /// 
    /// # Output
    /// It returns the left-shifted version of `self`. which is shifted to the
    /// left by `n` bits.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn shift_left<U>(&self, n: U) -> Self
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut res = self.clone();
        res.shift_left_assign(n);
        res
    }

    // pub fn shift_left_assign<U>(&mut self, n: U)
    /// shifts the field `number: [T;N]` to the left by `n`, and assign it
    /// to `self` back.
    /// 
    /// # Feature
    /// 'Shift left' means 'move left all bits'. So, if `10011010` is shifted
    /// left by 2, it will be `01101000`, for example.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn shift_left_assign<U>(&mut self, n: U)
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let TSIZE_IN_BITS = T::size_in_bits();
        let chunk_num = (n / U::num(TSIZE_IN_BITS as u128)).into_usize();
        let piece_num = (n % U::num(TSIZE_IN_BITS as u128)).into_usize();
        let zero = T::zero();
        if chunk_num > 0
        {
            for i in N-chunk_num..N
            {
                if self.get_num_(i) > zero
                {
                    self.set_overflow();
                    break;
                }
            }
            self.copy_within(0..N-chunk_num, chunk_num);
            for idx in 0..chunk_num
                { self.set_num_(idx, zero); }
        }
        if piece_num == 0
            { return; }
        if (self.get_num_(N-1).leading_zeros() as usize) < piece_num
            { self.set_overflow(); }

        let mut num: T;
        let mut carry = zero;
        for idx in chunk_num..N
        {
            num = (self.get_num_(idx) << T::usize_as_Uint(piece_num)) | carry;
            carry = self.get_num_(idx) >> T::usize_as_Uint(TSIZE_IN_BITS - piece_num);
            self.set_num_(idx, num);
        }
        if carry != zero
            { self.set_overflow(); }
    }

    // pub fn shift_right<U>(&self, n: U) -> Self
    /// Shift right the field `number: [T;N]` to the right by `n`.
    /// 
    /// # Feature
    /// 'Shift right' means 'move right all bits'. So, if `10011010` is shifted
    /// right by 2, it will be `00100110`, for example.
    /// 
    /// # Output
    /// It returns the right-shifted version of `self`. which is shifted to the
    /// right by `n` bits.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn shift_right<U>(&self, n: U) -> Self
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut res = self.clone();
        res.shift_right_assign(n);
        res
    }

    // pub fn shift_right_assign<U>(&mut self, n: U)
    /// shifts the field `number: [T;N]` to the right by `n`, and assign it
    /// to `self` back.
    /// 
    /// # Feature
    /// 'Shift right' means 'move right all bits'. So, if `10011010` is shifted
    /// right by 2, it will be `00100110`, for example.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn shift_right_assign<U>(&mut self, n: U)
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let TSIZE_IN_BITS = T::size_in_bits();
        let chunk_num = (n / U::usize_as_Uint(TSIZE_IN_BITS)).into_usize();
        let piece_num = (n % U::usize_as_Uint(TSIZE_IN_BITS)).into_usize();
        let zero = T::zero();
        self.reset_all_flags();
        if chunk_num > 0
        {
            for i in 0..chunk_num
            {
                if self.get_num_(i) > zero
                {
                    self.set_underflow();
                    break;
                }
            }
            self.copy_within(chunk_num..N, 0);
            for idx in N-chunk_num..N
                { self.set_num_(idx, zero); }
        }
        if piece_num == 0
            { return; }
        if (self.get_num_(0) << T::usize_as_Uint(TSIZE_IN_BITS - piece_num)) != zero
            { self.set_underflow(); }

        let mut num: T;
        let mut carry = T::zero();
        let mut idx = N - 1 - chunk_num;
        loop
        {
            num = (self.get_num_(idx) >> T::usize_as_Uint(piece_num)) | carry;
            carry = self.get_num_(idx) << T::usize_as_Uint(TSIZE_IN_BITS - piece_num);
            self.set_num_(idx, num);
            if idx == 0
                { break; }
            idx -= 1;
        }
        if carry != zero
            { self.set_underflow(); }
    }

    // pub fn rotate_left<U>(&self, n: U) -> Self
    /// Rotates the field `number: [T;N]` to the left by `n`.
    /// 
    /// # Feature
    /// 'Rotate left' means 'shift left' with filling the left-pushed-out bits
    /// to the empty rightmost bits. So, if `10011010` is rotated left by 2,
    /// it will be `01101010`, for example.
    /// 
    /// # Output
    /// It returns the left-rotated version of `self`. which is rotated to the
    /// left by `n` bits.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    pub fn rotate_left<U>(&self, n: U) -> Self
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
       self.shift_left(n) | self.shift_right(U::num((N * 8) as u128 - n.into_u128()))
    }

    // pub fn rotate_left_assign<U>(&mut self, n: U)
    /// Rotates the field `number: [T;N]` to the left by `n`, and assign it
    /// to `self` back.
    /// 
    /// # Feature
    /// 'Rotate left' means 'shift left' with filling the left-pushed-out bits
    /// to the empty rightmost bits. So, if `10011010` is rotated left by 2,
    /// it will be `01101010`, for example.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    pub fn rotate_left_assign<U>(&mut self, n: U)
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.set_number(self.rotate_left(n).get_number());
    }

    // pub fn rotate_right<U>(&self, n: U) -> Self
    /// Rotates the field `number: [T;N]` to the right by `n`.
    /// 
    /// # Feature
    /// 'Rotate right' means 'shift right' with filling the right-pushed-out bits
    /// to the empty leftmost bits. So, if `10011010` is rotated right by 2,
    /// it will be `10100110`, for example.
    /// 
    /// # Output
    /// It returns the right-rotated version of `self`. which is rotated to the
    /// right by `n` bits.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    pub fn rotate_right<U>(&self, n: U) -> Self
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
       self.shift_right(n) | self.shift_left(U::num((N * 8) as u128 - n.into_u128()))
    }

    // pub fn rotate_right_assign<U>(&mut self, n: U)
    /// Rotates the field `number: [T;N]` to the right by `n`, and assign it
    /// to `self` back.
    /// 
    /// # Feature
    /// 'Rotate right' means 'shift right' with filling the right-pushed-out bits
    /// to the empty leftmost bits. So, if `10011010` is rotated right by 2,
    /// it will be `10100110`, for example.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    pub fn rotate_right_assign<U>(&mut self, n: U)
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.set_number(self.rotate_right(n).get_number());
    }

    // pub fn bitand(&self, rhs: &Self) -> Self
    /// Performs the bitwise AND (&) operation.
    /// 
    /// # Output
    /// It returns the result after applying the bitwise AND operation.
    ///  
    /// # Example 1
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::from_str_radix("11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000", 2).unwrap();
    /// let c = a.and(&b);
    /// 
    /// println!("a = {}", a.to_string_with_radix(2).unwrap());
    /// println!("b = {}", b.to_string_with_radix(2).unwrap());
    /// println!("a & b = {}", c.to_string_with_radix(2).unwrap());
    /// 
    /// assert_eq!(c, a & b);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::zero();
    /// let c = a.and(&b);
    /// 
    /// println!("a = {}", a.to_string_with_radix(2).unwrap());
    /// println!("b = {}", b.to_string_with_radix(2).unwrap());
    /// println!("a & b = {}", c.to_string_with_radix(2).unwrap());
    /// 
    /// assert_eq!(c, a & b);
    /// ```
    pub fn and(&self, rhs: &Self) -> Self
    {
        let mut s = self.clone();
        s.and_assign(rhs);
        s
    }

    // pub fn bitand_assign(&mut self, rhs: &Self)
    /// Performs the bitwise AND (&) operation, and then assigns the result
    /// to `self` back after applying the bitwise AND operation.
    /// 
    /// # Example 1
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::from_str_radix("11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000", 2).unwrap();
    /// a.and_assign(&b);
    /// 
    /// println!("a = {}", a.to_string_with_radix(2).unwrap());
    /// 
    /// assert_eq!(a, u256::from_str_radix("1111000000000000110000000000001110001000000100011010101000000000111100000000000011000000000000111000100000010001101010100000000011110000000000001100000000000011100010000001000110101010000000001111000000000000110000000000001110001000000100011010101000000000", 2).unwrap());
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::zero();
    /// a.and_assign(&b);
    /// 
    /// println!("a = {}", a.to_string_with_radix(2).unwrap());
    /// 
    /// assert_eq!(a, u256::zero());
    /// ```
    pub fn and_assign(&mut self, rhs: &Self)
    {
        let mut n: T;
        for idx in 0..N
        {
            n = self.get_num_(idx) & rhs.get_num_(idx);
            self.set_num_(idx, n);
        }
    }

    // pub fn or(self, rhs: &Self) -> Self
    /// Performs the bitwise OR (|) operation.
    /// 
    /// # Output
    /// It returns the result after applying the bitwise OR (|) operation.
    /// 
    /// # Example 1
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::from_str_radix("11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000", 2).unwrap();
    /// let c = a.or(&b);
    /// 
    /// println!("a = {}", a.to_string_with_radix(2).unwrap());
    /// println!("b = {}", b.to_string_with_radix(2).unwrap());
    /// println!("a | b = {}", c.to_string_with_radix(2).unwrap());
    /// 
    /// assert_eq!(c, a.or(&b));
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::max();
    /// let c = a.or(&b);
    /// 
    /// println!("a = {}", a.to_string_with_radix(2).unwrap());
    /// println!("b = {}", b.to_string_with_radix(2).unwrap());
    /// println!("a | b = {}", c.to_string_with_radix(2).unwrap());
    /// 
    /// assert_eq!(c, u256::max());
    /// ```
    pub fn or(&self, rhs: &Self) -> Self
    {
        let mut s = self.clone();
        s.or_assign(&rhs);
        s
    }

    // pub fn or_assign(&mut self, rhs: &Self)
    /// Performs the bitwise OR (|) operation, and then assigns the result
    /// to `self` after applying the bitwise OR (|) operation.
    /// 
    /// # Example 1
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::from_str_radix("11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000", 2).unwrap();
    /// a.or_assign(&b);
    /// 
    /// println!("a = {}", a.to_string_with_radix(2).unwrap());
    /// 
    /// assert_eq!(a, u256::from_str_radix("1111111100001111111111000011111111101110011101111111111101010101111111110000111111111100001111111110111001110111111111110101010111111111000011111111110000111111111011100111011111111111010101011111111100001111111111000011111111101110011101111111111101010101", 2).unwrap());
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::max();
    /// a.or_assign(&b);
    /// 
    /// println!("a = {}", a.to_string_with_radix(2).unwrap());
    /// 
    /// assert_eq!(a, u256::max());
    /// ```
    pub fn or_assign(&mut self, rhs: &Self)
    {
        let mut n: T;
        for idx in 0..N
        {
            n = self.get_num_(idx) | rhs.get_num_(idx);
            self.set_num_(idx, n);
        }
    }


    // pub fn xor(self, rhs: &Self) -> Self
    /// Performs the bitwise XOR (^) operation.
    /// 
    /// # Output
    /// It returns the result after applying the bitwise XOR (^) operation.
    /// 
    /// # Example 1
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::from_str_radix("11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000", 2).unwrap();
    /// let c = a.xor(&b);
    /// 
    /// println!("a = {}", a.to_string_with_radix(2).unwrap());
    /// println!("b = {}", b.to_string_with_radix(2).unwrap());
    /// println!("a ^ b = {}", c.to_string_with_radix(2).unwrap());
    /// 
    /// assert_eq!(c, a.or(&b));
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::max();
    /// let c = a.xor(&b);
    /// 
    /// println!("a = {}", a.to_string_with_radix(2).unwrap());
    /// println!("b = {}", b.to_string_with_radix(2).unwrap());
    /// println!("a ^ b = {}", c.to_string_with_radix(2).unwrap());
    /// 
    /// assert_eq!(c, u256::max());
    /// ```
    pub fn xor(&self, rhs: &Self) -> Self
    {
        let mut s = self.clone();
        s.or_assign(&rhs);
        s
    }

    // pub fn xor_assign(&mut self, rhs: &Self)
    /// Performs the bitwise XOR (^) operation, and then assigns the result
    /// to `self` after applying the bitwise XOR (^) operation.
    /// 
    /// # Example 1
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::from_str_radix("11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000", 2).unwrap();
    /// a.xor_assign(&b);
    /// 
    /// println!("a = {}", a.to_string_with_radix(2).unwrap());
    /// 
    /// assert_eq!(a, u256::from_str_radix("1111111100001111111111000011111111101110011101111111111101010101111111110000111111111100001111111110111001110111111111110101010111111111000011111111110000111111111011100111011111111111010101011111111100001111111111000011111111101110011101111111111101010101", 2).unwrap());
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::max();
    /// a.xor_assign(&b);
    /// 
    /// println!("a = {}", a.to_string_with_radix(2).unwrap());
    /// 
    /// assert_eq!(a, u256::max());
    /// ```
    pub fn xor_assign(&mut self, rhs: &Self)
    {
        let mut n: T;
        for idx in 0..N
        {
            n = self.get_num_(idx) | rhs.get_num_(idx);
            self.set_num_(idx, n);
        }
    }

    // pub fn flip(&self) -> Self
    /// Performs the unary ! operation.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn flip(&self) -> Self
    {
        let mut res = self.clone();
        res.flip_assign();
        res
    }

    // pub fn flip_assign(&mut self)
    /// Performs the unary ! operation, and assigns the result to `self`.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn flip_assign(&mut self)
    {
        for idx in 0..N
            { self.set_num_(idx, !self.get_num_(idx)); }
    }

    // pub fn reverse_bits(&self) -> Self
    /// Reverses the order of bits in the integer.
    /// 
    /// # Output
    /// It returns the reversed order of bits in the integer.
    /// 
    /// # Feature
    /// The least significant bit becomes the most significant bit,
    /// second least-significant bit becomes second most-significant bit, etc.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn reverse_bits(&self) -> Self
    {
        let mut res = self.clone();
        res.reverse_bits_assign();
        res
    }

    // pub fn reverse_bits_assign(&mut self)
    /// Reverses the order of bits in the integer,
    /// and assigns the result to `self`.
    /// 
    /// # Output
    /// It returns the reversed order of bits in the integer.
    /// 
    /// # Feature
    /// The least significant bit becomes the most significant bit,
    /// second least-significant bit becomes second most-significant bit, etc.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn reverse_bits_assign(&mut self)
    {
        let mut low: T;
        let mut high: T;
        for i in 0..N/2
        {
            low = self.get_num_(i).reverse_bits();
            high = self.get_num_(N-1-i).reverse_bits();
            self.set_num_(N-1-i, low);
            self.set_num_(i, high);
        }
        if N.is_odd()
            { self.set_num_(N/2+1, self.get_num_(N/2+1).reverse_bits()); }
    }

    // pub fn swap_bytes(&self) -> Self
    /// Reverses the byte order of the integer.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn swap_bytes(&self) -> Self
    {
        let mut res = self.clone();
        res.swap_bytes_assign();
        res
    }

    // pub fn swap_bytes_assign(&mut self)
    /// Reverses the byte order of the integer, and assign the result to `self`.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn swap_bytes_assign(&mut self)
    {
        let mut tmp: T;
        for i in 0..N/2
        {
            tmp = self.get_num_(i);
            self.set_num_(i, self.get_num_(N-1-i));
            self.set_num_(N-1-i, tmp);
        }
    }



    /***** METHODS FOR COMPARISON *****/

    pub fn eq(&self, other: &Self) -> bool
    {
        for idx in 0..N
        {
            if self.get_num_(idx) != other.get_num_(idx)
                { return false; }
        }
        true
    }

    #[cfg(target_endian = "little")]
    pub fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        let mut idx = N - 1;
        loop
        {
            if self.get_num_(idx) > other.get_num_(idx)
                { return Some(Ordering::Greater); }
            else if self.get_num_(idx) < other.get_num_(idx)
                { return Some(Ordering::Less); }
            if idx == 0
                { break; }
            idx -= 1;
        }
        Some(Ordering::Equal)
    }

    #[cfg(target_endian = "big")]
    pub fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        for idx in 0..N
        {
            if self.get_num_(idx) > other.get_num_(idx)
                { return Some(Ordering::Greater); }
            else if self.get_num_(idx) < other.get_num_(idx)
                { return Some(Ordering::Less); }
        }
        Some(Ordering::Equal)
    }


    /***** METHODS FOR CONVERTING INTO OTHER TYPES WITH/WITHOUT LOSS *****/

    // pub fn into_biguint<U, const M: usize>(&self) -> BigUInt<U, M>
    /// Converts `self` into another kind of `BigUInt<U, M>`.
    /// 
    /// # Feature
    /// It copies the contents of its field `number[T;N]` to the field
    /// `number: [U;M]` of `BigUInt<U, M>`. If the size of `number: [T;N]`
    /// of `self` is bigger than the size of `number: [U;M]` of `BigUInt<U, M>`,
    /// that is, `size_of::<T>() * N` > `size_of::<U>() * M`, it is lossy
    /// conversion. Otherwise, no contents of the field `number: [T;N]` of
    /// `self` is lost. Always, the field `flag` is not copied.
    /// 
    /// # Output
    /// It returns another kind of `BigUInt<U, M>` with keeping the contents
    /// of the field `number: [T;N]` as much as possible.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    pub fn into_biguint<U, const M: usize>(&self) -> BigUInt<U, M>
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        BigUInt::<U, M>::from_biguint(&self)
    }

    // pub fn into_u128(&self) -> u128
    /// Converts `self` into `u128`.
    /// 
    /// # Feature
    /// It takes the lowest `u128`-sized bytes, that is, the lowest sixteen
    /// bytes from `self`, and return then as `u128` data type.
    /// It is usually lossy conversion.
    /// 
    /// # Output
    /// It returns the lowest sixteen bytes of `self`.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn into_u128(&self) -> u128
    {
        let mut num = LongerUnion::new();
        match T::size_in_bytes()
        {
            1 => {
                    num.set_ubyte_(0, self.get_num_(0).into_u8());
                    if N > 1 { num.set_ubyte_(1, self.get_num_(1).into_u8()) }
                    if N > 2 { num.set_ubyte_(2, self.get_num_(2).into_u8()) }
                    if N > 3 { num.set_ubyte_(3, self.get_num_(3).into_u8()) }
                    if N > 4 { num.set_ubyte_(4, self.get_num_(4).into_u8()) }
                    if N > 5 { num.set_ubyte_(5, self.get_num_(5).into_u8()) }
                    if N > 6 { num.set_ubyte_(6, self.get_num_(6).into_u8()) }
                    if N > 7 { num.set_ubyte_(7, self.get_num_(7).into_u8()) }
                    if N > 8 { num.set_ubyte_(8, self.get_num_(8).into_u8()) }
                    if N > 9 { num.set_ubyte_(9, self.get_num_(9).into_u8()) }
                    if N > 10 { num.set_ubyte_(10, self.get_num_(10).into_u8()) }
                    if N > 11 { num.set_ubyte_(11, self.get_num_(11).into_u8()) }
                    if N > 12 { num.set_ubyte_(12, self.get_num_(12).into_u8()) }
                    if N > 13 { num.set_ubyte_(13, self.get_num_(13).into_u8()) }
                    if N > 14 { num.set_ubyte_(14, self.get_num_(14).into_u8()) }
                    if N > 15 { num.set_ubyte_(15, self.get_num_(15).into_u8()) }
                },
            2 => {
                    num.set_ushort_(0, self.get_num_(0).into_u16());
                    if N > 1 { num.set_ushort_(1, self.get_num_(1).into_u16()); }
                    if N > 2 { num.set_ushort_(2, self.get_num_(2).into_u16()); }
                    if N > 3 { num.set_ushort_(3, self.get_num_(3).into_u16()); }
                    if N > 4 { num.set_ushort_(4, self.get_num_(4).into_u16()); }
                    if N > 5 { num.set_ushort_(5, self.get_num_(5).into_u16()); }
                    if N > 6 { num.set_ushort_(6, self.get_num_(6).into_u16()); }
                    if N > 7 { num.set_ushort_(7, self.get_num_(7).into_u16()); }
                },
            4 => {
                    num.set_uint_(0, self.get_num_(0).into_u32());
                    if N > 2 { num.set_uint_(1, self.get_num_(1).into_u32()); }
                    if N > 3 { num.set_uint_(2, self.get_num_(2).into_u32()); }
                    if N > 4 { num.set_uint_(3, self.get_num_(3).into_u32()); }
                },
            8 => { 
                    num.set_ulong_(0, self.get_num_(0).into_u64());
                    if N > 1 { num.set_ulong_(1, self.get_num_(1).into_u64()); }
                },
            _ => { return self.get_num_(0).into_u128(); },
        }
        num.get()
    }

    // pub fn into_u64(&self) -> u64
    /// Converts `self` into `u64`.
    /// 
    /// # Feature
    /// It takes the lowest `u64`-sized bytes, that is, the lowest eight
    /// bytes from `self`, and return then as `u64` data type.
    /// It is usually lossy conversion.
    /// 
    /// # Output
    /// It returns the lowest eight bytes of `self`.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn into_u64(&self) -> u64
    {
        let mut num = LongerUnion::new();
        match size_of::<T>()
        {
            1 => {
                    num.set_ubyte_(0, self.get_num_(0).into_u8());
                    if N > 1 { num.set_ubyte_(1, self.get_num_(1).into_u8()); }
                    if N > 2 { num.set_ubyte_(2, self.get_num_(2).into_u8()); }
                    if N > 3 { num.set_ubyte_(3, self.get_num_(3).into_u8()); }
                    if N > 4 { num.set_ubyte_(4, self.get_num_(4).into_u8()); }
                    if N > 5 { num.set_ubyte_(5, self.get_num_(5).into_u8()); }
                    if N > 6 { num.set_ubyte_(6, self.get_num_(6).into_u8()); }
                    if N > 7 { num.set_ubyte_(7, self.get_num_(7).into_u8()); }
                },
            2 => {
                    num.set_ushort_(0, self.get_num_(0).into_u16());
                    if N > 1 { num.set_ushort_(1, self.get_num_(1).into_u16()); }
                    if N > 2 { num.set_ushort_(2, self.get_num_(2).into_u16()); }
                    if N > 3 { num.set_ushort_(3, self.get_num_(3).into_u16()); }
                },
            4 => {
                    num.set_uint_(0, self.get_num_(0).into_u32());
                    if N > 1 { num.set_uint_(1, self.get_num_(1).into_u32()); }
                },
            8 => { return self.get_num_(0).into_u64(); },
            _ => { num.set(self.number[0].into_u128()); },
        }
        num.get_ulong_(0)
    }

    // pub fn into_u32(&self) -> u32
    /// Converts `self` into `u32`.
    /// 
    /// # Feature
    /// It takes the lowest `u32`-sized bytes, that is, the lowest four
    /// bytes from `self`, and return then as `u32` data type.
    /// It is usually lossy conversion.
    /// 
    /// # Output
    /// It returns the lowest four bytes of `self`.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn into_u32(&self) -> u32
    {
        let mut num = LongerUnion::new();
        match size_of::<T>()
        {
            1 => {
                    num.set_ubyte_(0, self.get_num_(0).into_u8());
                    if N > 1 { num.set_ubyte_(1, self.get_num_(1).into_u8()); }
                    if N > 2 { num.set_ubyte_(2, self.get_num_(2).into_u8()); }
                    if N > 3 { num.set_ubyte_(3, self.get_num_(3).into_u8()); }
                },
            2 => {
                    num.set_ushort_(0, self.get_num_(0).into_u16());
                    if N > 1 { num.set_ushort_(1, self.get_num_(1).into_u16()); }
                },
            4 => { return self.get_num_(0).into_u32(); },
            8 => { num.set_ulong_(0, self.get_num_(0).into_u64()); },
            _ => { num.set(self.get_num_(0).into_u128()); },
        }
        num.get_uint_(0)
    }

    // pub fn into_u16(&self) -> u16
    /// Converts `self` into `u16`.
    /// 
    /// # Feature
    /// It takes the lowest `u16`-sized bytes, that is, the lowest two
    /// bytes from `self`, and return then as `u16` data type.
    /// It is usually lossy conversion.
    /// 
    /// # Output
    /// It returns the lowest two bytes of `self`.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn into_u16(&self) -> u16
    {
        let mut num = LongerUnion::new();
        match size_of::<T>()
        {
            1 => {
                    num.set_ubyte_(0, self.get_num_(0).into_u8());
                    if N > 1 { num.set_ubyte_(1, self.get_num_(1).into_u8()) }
                },
            2 => { return self.get_num_(0).into_u16(); },
            4 => { num.set_uint_(0, self.get_num_(0).into_u32()); },
            8 => { num.set_ulong_(0, self.get_num_(0).into_u64()); },
            _ => { num.set(self.get_num_(0).into_u128()); },
        }
        num.get_ushort_(0)
    }

    // pub fn into_u8(&self) -> u8
    /// Converts `self` into `u8`.
    /// 
    /// # Feature
    /// It takes the lowest `u8`-sized byte, that is, the lowest one
    /// byte from `self`, and return it as `u8` data type.
    /// It is usually lossy conversion.
    /// 
    /// # Output
    /// It returns the lowest byte of `self`.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    pub fn into_u8(&self) -> u8
    {
        self.get_num_(0).into_u8()
    }

    // pub fn into_usize(&self) -> usize
    /// Converts `self` into `usize`.
    /// 
    /// # Feature
    /// It takes the lowest `usize`-sized bytes from `self`,
    /// and return them as `usize` data type. It is usually lossy conversion.
    /// 
    /// # Output
    /// It returns the lowest `usize` long part of `self`.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    pub fn into_usize(&self) -> usize
    {
        #[cfg(target_pointer_width = "128")]    return self.into_u128().into_usize();
        #[cfg(target_pointer_width = "64")]     return self.into_u64().into_usize();
        #[cfg(target_pointer_width = "32")]     return self.into_u32().into_usize();
        #[cfg(target_pointer_width = "16")]     return self.into_u16().into_usize();
        #[cfg(target_pointer_width = "16")]     return self.into_u16().into_usize();
    }

    // pub fn to_be(&self) -> Self
    /// Converts `self` to big endian from the target’s endianness.
    /// 
    /// # Feature
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[cfg(target_endian = "little")]
    #[inline]
    pub fn to_be(&self) -> Self
    {
        self.swap_bytes()
    }

    // pub fn to_be(&self) -> Self
    /// Converts `self` to big endian from the target’s endianness.
    /// 
    /// # Feature
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[cfg(target_endian = "big")]
    #[inline]
    pub fn to_be(&self) -> Self
    {
        self.clone()
    }

    // pub fn to_be_bytes(&self) -> [T; N]
    /// Return the memory representation of this integer as a byte array
    /// in big-endian (network) byte order.
    /// # Feature
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[cfg(target_endian = "little")]
    #[inline]
    pub fn to_be_bytes(&self) -> [T; N]
    {
        self.swap_bytes().get_number().clone()
    }

    // pub fn to_be_bytes(&self) -> [T; N]
    /// Return the memory representation of this integer as a byte array
    /// in big-endian (network) byte order.
    /// # Feature
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[cfg(target_endian = "big")]
    #[inline]
    pub fn to_be_bytes(&self) -> [T; N]
    {
        self.get_number().clone()
    }

    // pub fn from_le(le: &Self) -> Self
    /// Converts `self` to little endian from the target’s endianness.
    /// 
    /// # Feature
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[cfg(target_endian = "little")]
    #[inline]
    pub fn to_le(&self) -> Self
    {
        self.clone()
    }

    // pub fn from_le(le: &Self) -> Self
    /// Converts `self` to little endian from the target’s endianness.
    /// 
    /// # Feature
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[cfg(target_endian = "big")]
    #[inline]
    pub fn to_le(&self) -> Self
    {
        self.swap_bytes()
    }

    // pub fn to_le_bytes(&self) -> [T; N]
    /// Return the memory representation of this integer as a byte array
    /// in little-endian byte order.
    /// 
    /// # Feature
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[cfg(target_endian = "little")]
    #[inline]
    pub fn to_le_bytes(&self) -> [T; N]
    {
        self.get_number().clone()
    }

    // pub fn to_le_bytes(&self) -> [T; N]
    /// Return the memory representation of this integer as a byte array
    /// in little-endian byte order.
    /// 
    /// # Feature
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[cfg(target_endian = "big")]
    #[inline]
    pub fn to_le_bytes(&self) -> [T; N]
    {
        self.swap_bytes().get_number().clone()
    }

    // pub fn to_string_with_radix_and_delimiter(&self, radix: usize, stride: usize, delimiter: &str) -> Result<String, NumberErr>
    /// Reads the value of `BigUInt<T, N>` and write it into String in a 
    /// certain radix indicated by `radix`, with certain stride steps
    /// indicated by `stride`, and with a delimiter indicated by `delimiter`.
    /// 
    /// # Output
    /// It returns a sring that shows the value of `BigUInt<T, N>` in a 
    /// certain radix indicated by `radix`, with certain stride steps
    /// indicated by `stride`, and with a delimiter indicated by `delimiter`.
    /// 
    /// # Valid Radix Range
    /// The radix can be from `2` up to `62` (= 10 + 26 + 26). Such radices that
    /// are less than `2` or more than `62` are not available. In this case,
    /// this method will return `Err(NumberErr::OutOfValidRadixRange)`.
    /// 
    /// # Radix more than `10` and less than `37`
    /// If the radix is more than `10` and less than `37`, the digit bigger than
    /// `9` will be expressed with alphabets. The avaiable alphabets are
    /// _case-insensitive_. For example, in the case of hexadecimal number
    /// system, the digit whose value is `10`, `11`, `12`, `13`, `14`, and `15`
    /// are represented as `A` or `a`, `B` or `b`, `C` or `c`, `D` or `d`, `E`
    /// or `e`, and `F` or `f`, respectively. And, in the case of 37-ary number
    /// system, the values `16`, `35` and `36` are represented as `G` or `g`,
    /// `Y` or `y`, and `Z` or `z`, respectively.
    /// 
    /// # Radix more than `36` and less than `63`
    /// However, if the radix is more than `36` and less than `63`, the digit
    /// bigger than `9` will be expressed with alphabets. The avaiable alphabets
    /// are _case-sensitive_, so `A` is different from `a`. For instance, in the
    /// case of 62-ary number system, the digit whose value is `10`, `11`, `35`,
    /// `36`, `37`, `38`, `60` and `61` are represented as `A`, `B`, `Y`, `Z`,
    /// `a`, `b`, `y` and `z`, respectively.
    /// 
    /// # Stride
    /// In the number expression in a string, you can separate the digits every
    /// certain number of digits which is called stride. For example, if
    /// `stride` is 4, the delimeter will be added every four digits. So,
    /// `100000000` will be written as "1_0000_0000".
    /// 
    /// # Replaceable Delimiter
    /// In the number expression in a string, you can replace the default
    /// delimiter with any `str` such as "," or "--" in order to make it more
    /// readable. So, `100000000` will be written as "1,0000,0000" or
    /// "1--0000--0000", for example.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn to_string_with_radix_and_delimiter(&self, radix: usize, stride: usize, delimiter: &str) -> Result<String, NumberErr>
    {
        let res = self.to_string_with_radix_and_stride(radix, stride);
        match res
        {
            Ok(txt) =>  { Ok(txt.replace("_", delimiter)) },
            Err(_) =>   { res },
        }
    }

    // pub fn to_string_with_radix_and_stride(&self, radix: usize, stride: usize) -> Result<String, NumberErr>
    /// Reads the value of `BigUInt<T, N>` and write it into String
    /// with a certain radix.
    /// 
    /// # Output
    /// It returns a sring that shows the value of `BigUInt<T, N>`.
    /// 
    /// # Valid Radix Range
    /// The radix can be from `2` up to `62` (= 10 + 26 + 26). Such radices that
    /// are less than `2` or more than `62` are not available. In this case,
    /// this method will return `Err(NumberErr::OutOfValidRadixRange)`.
    /// 
    /// # Radix more than `10` and less than `37`
    /// If the radix is more than `10` and less than `37`, the digit bigger than
    /// `9` will be expressed with alphabets. The avaiable alphabets are
    /// _case-insensitive_. For example, in the case of hexadecimal number
    /// system, the digit whose value is `10`, `11`, `12`, `13`, `14`, and `15`
    /// are represented as `A` or `a`, `B` or `b`, `C` or `c`, `D` or `d`, `E`
    /// or `e`, and `F` or `f`, respectively. And, in the case of 37-ary number
    /// system, the values `16`, `35` and `36` are represented as `G` or `g`,
    /// `Y` or `y`, and `Z` or `z`, respectively.
    /// 
    /// # Radix more than `36` and less than `63`
    /// However, if the radix is more than `36` and less than `63`, the digit
    /// bigger than `9` will be expressed with alphabets. The avaiable alphabets
    /// are _case-sensitive_, so `A` is different from `a`. For instance, in the
    /// case of 62-ary number system, the digit whose value is `10`, `11`, `35`,
    /// `36`, `37`, `38`, `60` and `61` are represented as `A`, `B`, `Y`, `Z`,
    /// `a`, `b`, `y` and `z`, respectively.
    /// 
    /// # Stride
    /// In the number expression in a string, you can separate the digits every
    /// certain number of digits which is called stride. For example, if
    /// `stride` is 4, the delimeter will be added every four digits. So,
    /// `100000000` will be written as "1_0000_0000".
    /// 
    /// # Default Delimiter _
    /// In the number expression in a string, you can separate the digits with
    /// the default delimiter '_' in order to make it more readable. So, "10000"
    /// is the same as "1_0000".
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn to_string_with_radix_and_stride(&self, radix: usize, stride: usize) -> Result<String, NumberErr>
    {
        if (radix < 2) || (radix > 10 + 26 + 26)
            { return Err(NumberErr::OutOfValidRadixRange); }

        if stride == 0 
        {
            self.to_string_with_radix(radix)
        }
        else
        {
            let mut stride_num = stride;
            let mut txt = String::new();
            let zero = Self::zero();
            let mut dividend = self.clone();
            let mut remainder;
            loop
            {
                (dividend, remainder) = dividend.divide_fully_uint(T::usize_as_Uint(radix));
                let r = remainder.into_u32();
                let c = if r < 10     { ('0' as u32 + r) as u8 as char }
                        else if r < 10 + 26 { ('A' as u32 - 10 + r) as u8 as char }
                        else                { ('a' as u32 - 10 - 26 + r) as u8 as char };  // if r < 10 + 26 + 26
                txt.push(c);
                stride_num -= 1;
                if dividend == zero
                    { break; }
                if stride_num == 0
                {
                    txt.push('_');
                    stride_num = stride;
                }
            }
            if txt.len() == 0
                { txt.push('0'); }
            let mut num_str = String::new();
            while let Some(ch) = txt.pop()
                { num_str.push(ch); }
            Ok(num_str)
        }
    }

    // pub fn to_string_with_radix(&self, radix: usize) -> Result<String, NumberErr>
    /// Reads the value of `BigUInt<T, N>` and write it into String
    /// with a certain radix.
    /// 
    /// # Output
    /// It returns a sring that shows the value of `BigUInt<T, N>`.
    /// 
    /// # Valid Radix Range
    /// The radix can be from `2` up to `62` (= 10 + 26 + 26). Such radices that
    /// are less than `2` or more than `62` are not available. In this case,
    /// this method will return `Err(NumberErr::OutOfValidRadixRange)`.
    /// 
    /// # Radix more than `10` and less than `37`
    /// If the radix is more than `10` and less than `37`, the digit bigger than
    /// `9` will be expressed with alphabets. The avaiable alphabets are
    /// _case-insensitive_. For example, in the case of hexadecimal number
    /// system, the digit whose value is `10`, `11`, `12`, `13`, `14`, and `15`
    /// are represented as `A` or `a`, `B` or `b`, `C` or `c`, `D` or `d`, `E`
    /// or `e`, and `F` or `f`, respectively. And, in the case of 37-ary number
    /// system, the values `16`, `35` and `36` are represented as `G` or `g`,
    /// `Y` or `y`, and `Z` or `z`, respectively.
    /// 
    /// # Radix more than `36` and less than `63`
    /// However, if the radix is more than `36` and less than `63`, the digit
    /// bigger than `9` will be expressed with alphabets. The avaiable alphabets
    /// are _case-sensitive_, so `A` is different from `a`. For instance, in the
    /// case of 62-ary number system, the digit whose value is `10`, `11`, `35`,
    /// `36`, `37`, `38`, `60` and `61` are represented as `A`, `B`, `Y`, `Z`,
    /// `a`, `b`, `y` and `z`, respectively.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn to_string_with_radix(&self, radix: usize) -> Result<String, NumberErr>
    {
        if (radix < 2) || (radix > 10 + 26 + 26)
            { return Err(NumberErr::OutOfValidRadixRange); }

        let mut txt = String::new();
        let zero = Self::zero();
        let mut dividend = self.clone();
        let mut remainder;
        loop
        {
            (dividend, remainder) = dividend.divide_fully_uint(T::usize_as_Uint(radix));
            let r = remainder.into_u32();
            let c = if r < 10     { ('0' as u32 + r) as u8 as char }
                    else if r < 10 + 26 { ('A' as u32 - 10 + r) as u8 as char }
                    else                { ('a' as u32 - 10 - 26 + r) as u8 as char };  // if r < 10 + 26 + 26
            txt.push(c);
            if dividend == zero
                { break; }
        }
        if txt.len() == 0
            { txt.push('0'); }
        let mut num_str = String::new();
        while let Some(ch) = txt.pop()
            { num_str.push(ch); }
        Ok(num_str)
    }



    /***** FLAG MANIPULATION *****/

    // pub fn set_flag_bit(&mut self, flag: u8)
    /// Sets flag bits that `flag`indicates to be `1`.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    pub fn set_flag_bit(&mut self, flag: u8)
    {
        self.flag |= flag;
    }

    // pub fn reset_flag_bit(&mut self, flag: u8)
    /// Resets flag bits that `flag`indicates to be `0`.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    pub fn reset_flag_bit(&mut self, flag: u8)
    {
        self.flag &= !flag;
    }

    // pub fn is_flag_bit_on(&self) -> bool
    /// Checks whether or not the flag bits that `flag`indicates are set to be `1.
    /// 
    /// # Output
    /// It returns `true` if the flag bits that `flag`indicates are set.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    pub fn is_flag_bit_on(&self, flag: u8) -> bool
    {
        (self.flag & flag) != 0
    }

    // pub fn reset_all_flags(&mut self)
    /// Resets all flag bits to be `0`.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    pub fn reset_all_flags(&mut self)
    {
        self.flag = 0;
    }

    // pub fn set_infinity(&mut self)
    /// Sets infinity flag.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    pub fn set_infinity(&mut self)
    {
        self.set_flag_bit(Self::INFINITY);
    }

    // pub fn reset_inifinity(&mut self)
    /// Resets infinity flag.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    pub fn reset_inifinity(&mut self)
    {
        self.reset_flag_bit(Self::INFINITY);
    }

    // pub fn is_inifinity(&self) -> bool
    /// Checks whether or not inifinity flag is set.
    /// 
    /// # Output
    /// It returns `true` if the inifinity flag is set.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    pub fn is_inifinity(&self) -> bool
    {
        self.is_flag_bit_on(Self::INFINITY)
    }

    // pub fn set_divided_by_zero(&mut self)
    /// Sets divided_by_zero flag.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    pub fn set_divided_by_zero(&mut self)
    {
        self.set_flag_bit(Self::DIVIDED_BY_ZERO);
    }

    // pub fn reset_divided_by_zero(&mut self)
    /// Resets divided_by_zero flag.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    pub fn reset_divided_by_zero(&mut self) { self.reset_flag_bit(Self::DIVIDED_BY_ZERO); }

    // pub fn is_overflow(&self) -> bool
    /// Checks whether or not divided_by_zero flag is set.
    /// 
    /// # Output
    /// It returns `true` if the divided_by_zero flag is set.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    pub fn is_divided_by_zero(&self) -> bool
    {
        self.is_flag_bit_on(Self::DIVIDED_BY_ZERO)
    }

    // pub fn set_overflow(&mut self)
    /// Sets overflow flag.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    pub fn set_overflow(&mut self)
    {
        self.set_flag_bit(Self::OVERFLOW);
    }

    // pub fn reset_overflow(&mut self)
    /// Resets overflow flag.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    pub fn reset_overflow(&mut self)
    {
        self.reset_flag_bit(Self::OVERFLOW);
    }

    // pub fn is_overflow(&self) -> bool
    /// Checks whether or not overflow flag is set.
    /// 
    /// # Output
    /// It returns `true` if the overflow flag is set.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    pub fn is_overflow(&self) -> bool
    {
        self.is_flag_bit_on(Self::OVERFLOW)
    }

    // pub fn set_underflow(&mut self)
    /// Sets underflow flag.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    pub fn set_underflow(&mut self)
    {
        self.set_flag_bit(Self::UNDERFLOW);
    }

    // pub fn reset_underflow(&mut self)
    /// Reets underflow flag.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    pub fn reset_underflow(&mut self)
    {
        self.reset_flag_bit(Self::UNDERFLOW);
    }

    // pub fn is_underflow(&self) -> bool
    /// Checks whether or not underflow flag is set.
    /// 
    /// # Output
    /// It returns `true` if the underflow flag is set.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    pub fn is_underflow(&self) -> bool
    {
        self.is_flag_bit_on(Self::UNDERFLOW)
    }

    // pub fn set_untrustable(&mut self)
    /// Sets both overflow flag and underflow flag.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    pub fn set_untrustable(&mut self)
    {
        self.set_flag_bit(Self::OVERFLOW | Self::UNDERFLOW);
    }

    // pub fn reset_untrustable(&mut self)
    /// Resets both overflow flag and underflow flag.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    pub fn reset_untrustable(&mut self)
    {
        self.reset_flag_bit(Self::OVERFLOW | Self::UNDERFLOW);
    }

    // pub fn is_untrustable(&self) -> bool
    /// Checks whether or not both overflow flag and underflow flag are all set.
    /// 
    /// # Output
    /// It returns `true` if both of the overflow flag and the underflow flag
    /// are all set. Otherwise, it returns `false`.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    pub fn is_untrustable(&self) -> bool
    {
        self.is_flag_bit_on(Self::OVERFLOW | Self::UNDERFLOW)
    }
}
