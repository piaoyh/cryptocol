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
/// # Examples
/// ```
/// use std::str::FromStr;
/// use Cryptocol::number::*;
/// type u1024 = BigUInt::<u128, 8>;
/// type U128 = BigUInt::<usize, 16>;
/// let a = u1024::from([1;8]);
/// let b = u1024::from_str_radix("00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001", 2).unwrap();
/// let mut c = u1024::from_str("1234567891234567879123456789111111111222222222333333333444444444555555555666666666777777777888888888999999999000000000").unwrap();
/// let d = U128::from_biguint(&c);
/// println!("a = {:?}", a);
/// println!("a = {}", a.to_string_with_radix(16));
/// println!("b = {:?}", b);
/// println!("b = {}", b.to_string_with_radix(16));
/// println!("c = {}", c);
/// println!("c + b = {}", c + b);
/// println!("c - b = {}", c - b);
/// println!("c * b = {}", c * b);
/// println!("b / c = {}", b / c);
/// println!("b % c = {}", b % c);
/// println!("b + 1 = {}", b.add_uint(1));
/// println!("b - 1 = {}", b.sub_uint(1));
/// assert_eq!(a, b);
/// c.set_one();
/// assert_eq!(c, u1024::one());
/// ```
#[derive(Debug, Copy, Clone)]
pub struct BigUInt<T, const N: usize>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    number: [T; N],
    flag: u8,
}

impl<T, const N: usize> BigUInt<T, N>
where T: Uint + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd,
    Self: Sized + Clone + Copy + Display + Debug + ToString
        + Add<Output = Self> + AddAssign + Sub<Output = Self> + SubAssign
        + Mul<Output = Self> + MulAssign + Div<Output = Self> + DivAssign
        + Rem<Output = Self> + RemAssign
        + Shl<i32, Output = Self> + ShlAssign<i32>
        + Shr<i32, Output = Self> + ShrAssign<i32>
        + BitAnd<Self, Output = Self> + BitAndAssign + BitOr<Output = Self> + BitOrAssign
        + BitXorAssign + Not<Output = Self>
        + From<T> + FromStr + From<[T; N]>
        //+ HugeInteger<T> + BigInteger<T, N>
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

    /// Constructs a new `BigUInt<T, N>`.
    /// 
    /// # Initialization
    /// All the attributes of te constructed object will be initialized with `0`.
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

    /// Constructs a new `BigUInt<T, N>` which has the value of zero.
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

    /// Constructs a new `BigUInt<T, N>` which has the value of one.
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for serious purpose. Only use this crate for Big-endian CPUs
    /// with your own full responsibility.
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
    pub fn one() -> Self
    {
        let mut me = Self::new();
        me.set_num(0, T::one());
        me
    }

    /// Constructs a new `BigUInt<T, N>` which has the value of maximum.
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

    /// Constructs a new `BigUInt<T, N>` from a string with radix.
    /// The constructed object will be wrapped in `Ok(BigUInt<T, N>)` if it is
    /// successfully created. Otherwise, this method returns one of
    /// `Err(NumberErr::OutOfValidRadixRange)`, `Err(NumberErr::NotAlphaNumeric)`,
    /// and `Err(NumberErr::NotFitToRadix)`.
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
            bignum.times(T::num(radix as u128));
            bignum.accumulate(T::num(num as u128));
        }
        Ok(bignum)
    }

    /// Constructs a new `BigUInt<T, N>` from an array of type `T` with `N`
    /// elements.
    /// 
    /// # Counter Part Method
    /// You can also use the method [from()](struct@BigUInt#impl-From<[T;+N]>-for-BigUInt<T,+N>)
    /// implemented by implementation of trait From<[T;N]>.
    /// 
    /// # Examples
    /// ```
    /// use Cryptocol::number::*;
    /// let big_num = BigUInt::<u8,32>::from_array(&[1_u8;32]);
    /// println!("big_num = {}", big_num.to_string_with_radix(2));
    /// assert_eq!(big_num, BigUInt::<u8,32>::from_str_radix("00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001", 2).unwrap());
    /// ```
    pub fn from_array(val: &[T; N]) -> Self
    {
        let mut s = Self::new();
        s.set_number(val);
        s
    }

    /// Constructs a new `BigUInt<T, N>` from another kind of `BigUInt<U, M>`.
    /// It copies not only long-bit integer but also current flags from another
    /// kind of `BigUInt<U, M>`.
    /// 
    /// # Example
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
    where U: Uint + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
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

    /// Constructs a new `BigUInt<T, N>` from another kind of `BigUInt<U, M>`.
    /// It copies not only long-bit integer but also current flags from another
    /// kind of `BigUInt<U, M>`.
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for serious purpose. Only use this crate for Big-endian CPUs
    /// with your own full responsibility.
    /// 
    /// # Example
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
    #[cfg(target_endian = "big")]
    pub fn from_biguint<U, const M: usize>(biguint: &BigUInt<U, M>) -> Self
    where U: Uint + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
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

    /// Constucts a new `BigUInt<T, N>` which has the random value.
    /// The random number that this method random() returns is a pure random
    /// number whose range is from 0 up to BigUInt::max() inclusively.
    /// 
    /// # Cryptographical Security
    /// The random number generated by this method random() is considered
    /// cryptographically secure. This method random() is based on the crate
    /// [rand](https://docs.rs/rand/latest/rand/index.html) (especially,
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Counter Part Method
    /// - If you want to be sure to use pure random number whose range is from 0
    /// up to BigUInt::max() inclusively, you are highly encouraged to use this
    /// method [random()](struct@BigUInt#method.random) rather than the method
    /// [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set).
    /// - If you want to use (N * 8)-bit long random number for cryptographic
    /// purpose, you are highly recommended to use the method [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set)
    /// instead of this method random() because this method random() does __NOT__
    /// guarrantee MSB (Most Significant Bit) is `1`. So, it is 50% that this
    /// method random() will generate a random number shorter than (N * 8)-bit.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// println!("Random number = {}", u1024::random());
    /// println!("Random number = {}", u1024::random_with_MSB_set());
    /// ```
    pub fn random() -> Self
    {
        let mut r = Self::new();
        match T::size_in_bytes()
        {
            1 => {
                    let mut common = UInt::new();
                    for i in 0..N
                    {
                        common.uint = OsRng.next_u32();
                        unsafe { r.set_num_(i, T::num(common.byte[0] as u128)); }
                    }
                },
            2 => {
                    let mut common = UInt::new();
                    for i in 0..N
                    {
                        common.uint = OsRng.next_u32();
                        unsafe { r.set_num_(i, T::num(common.ushort[0] as u128)); }
                    }
                },
            4 => {
                    for i in 0..N
                        { r.set_num_(i, T::num(OsRng.next_u32() as u128)); }
                },
            8 => {
                    for i in 0..N
                        { r.set_num_(i, T::num(OsRng.next_u64() as u128)); }
                },
            16 => {
                    for i in 0..N
                    {
                        let mut common = ULonger::new();
                        unsafe {
                            common.ulong[0] = OsRng.next_u64();
                            common.ulong[1] = OsRng.next_u64();
                            r.set_num_(i, T::num(common.ulonger));
                        }
                    }
                },
            _ => { r.set_zero() },
        }
        r
    }

    /// Constucts a new `BigUInt<T, N>` which has the random value
    /// with MSB (Most Significant Bit) is set. The random number that this
    /// method random_with_MSB_set() returns is a random number whose range
    /// is from !(BigUInt::max() >> 1) up to BigUInt::max() inclusively.
    /// 
    /// # Cryptographical Security
    /// The random number generated by this method random_with_MSB_set() is
    /// cryptographically secure because it uses the crate [rand](https://docs.rs/rand/latest/rand/index.html)
    /// ([rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Counter Part Method
    /// - If you want to be sure to use pure random number whose range is from 0
    /// up to BigUInt::max() inclusively, you are highly encouraged to use the
    /// method [random()](struct@BigUInt#method.random) rather than this method
    /// random_with_MSB_set().
    /// - If you want to be sure to use (N * 8)-bit long random number for
    /// cryptographic purpose, you are highly recommended to use the method
    /// random_with_MSB_set() instead of this method [random()](struct@BigUInt#method.random)
    /// because this method random_with_MSB_set() __DOES__ guarrantee MSB (Most
    /// Significant Bit) is `1`. So, it is __0 (zero) %__ that this method
    /// random_with_MSB_set() will generate a random number shorter than
    /// (N * 8)-bit.
    ///
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for serious purpose. Only use this crate for Big-endian CPUs
    /// with your own full responsibility.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with_u128;
    /// 
    /// define_utypes_with_u128!();
    /// println!("Random number = {}", u1024::random());
    /// println!("Random number = {}", u1024::random_with_MSB_set());
    /// ```
    pub fn random_with_MSB_set() -> Self
    {
        let mut r = Self::random();
        let highest = r.get_num_(N-1);
        let msb = !(T::max() >> T::one());
        r.set_num_(N-1, highest | msb);
        r
    }

    /// Constucts a new `BigUInt<T, N>` which has the value zero and sets only
    /// the bit specified by the argument bit_pos to be 1.
    /// 
    /// # Bit Position
    /// The bit positon bit_pos is zero-based and should be counted from LSB
    /// (Least Significant Bit) reguardless endian. So, if the bit_pos is `0`,
    /// only LSB is set to be `1` and all the other bits will be set to `0`.
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for serious purpose. Only use this crate for Big-endian CPUs
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with_u8;
    /// 
    /// define_utypes_with_u8!();
    /// let a = u256::make_check_bits(12);
    /// println!("a = {}", a.to_string_with_radix_and_stride(2, 8));
    /// assert_eq!(a, u256::from_str_radix("10000_00000000", 2).unwrap());
    /// ```
    pub fn make_check_bits(bit_pos: usize) -> Self
    {
        let mut check_bits = Self::zero();
        check_bits.turn_check_bits(bit_pos);
        check_bits
    }



    /***** METHODS TO GET SIZE BOTH IN BYTES AND BITS *****/

    /// Returns how many bytes long the long-bit number `BigUInt` is. It does
    /// not count how many bytes used for flags.
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

    /// Returns how many bits longz the long-bit number `BigUInt` is. It does
    /// not count how many bits used for flags.
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

    /// Returns how many bytes long the long-bit number, that is, the object of
    /// `BigUInt` is. It does not count how many bytes used for flags.
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

    /// Returns how many bits long the long-bit number, that is, the object of
    /// `BigUInt` is. It does not count how many bits used for flags.
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

    /// Changes a `BigUInt<T, N>` to have the value zero and sets only
    /// the bit specified by the argument bit_pos to be 1.
    /// 
    /// # Bit Position
    /// The bit positon bit_pos is zero-based and should be counted from LSB
    /// (Least Significant Bit) reguardless endian. So, if the bit_pos is `0`,
    /// only LSB is set to be `1` and all the other bits will be set to `0`.
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for serious purpose. Only use this crate for Big-endian CPUs
    /// with your own full responsibility.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with_u8;
    /// 
    /// define_utypes_with_u8!();
    /// let mut a = u256::random();
    /// a.turn_check_bits(12);
    /// println!("a = {}", a.to_string_with_radix_and_stride(2, 8));
    /// assert_eq!(a, u256::from_str_radix("10000_00000000", 2).unwrap());
    /// ```
    pub fn turn_check_bits(&mut self, bit_pos: usize)
    {
        let TSIZE_BITS = T::size_in_bits();
        let chunk_num = bit_pos / TSIZE_BITS;
        let piece_num = bit_pos % TSIZE_BITS;
        let mut val = T::one();
        val <<= T::num(piece_num as u128);
        self.set_zero();
        self.set_num_(chunk_num, val);
    }

    /// Returns i-th element of its array of type `T` wrapped in Some
    /// if `i` < `N`. Otherwise, it returns `None`. 
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
    /// # Counter Part Method
    /// When you are sure that `i` < `N`, you may want to use its counter part
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

    /// Returns i-th element of its array of type `T` wrapped in Some
    /// if `i` < `N`. Otherwise, it returns `None`. 
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
    /// # Counter Part Method
    /// When you are sure that `i` < `N`, you may want to use its counter part
    /// method [get_num_()](struct@BigUInt#method.get_num_) for performance.
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for serious purpose. Only use this crate for Big-endian CPUs
    /// with your own full responsibility.
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
    ///             assert_eq!(num, 40);
    ///         },
    ///     None => { println!("There is no third element."); },
    /// }
    /// ```
    #[cfg(target_endian = "big")]
    pub fn get_num(&self, i: usize) -> Option<T>
    {
        if i < N
            { Some(self.get_number()[N-1-i]) }
        else
            { None }
    }

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
    /// It is performance-oriented and does not care for safety. So, 
    /// if `i` >= `N`, it will panic.
    /// 
    /// # Counter Part Method
    /// Use this method only when you are sure that `i` < `N`.
    /// Otherwise, use its counter part method [get_num()](struct@BigUInt#method.get_num).
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with_u32;
    /// 
    /// define_utypes_with_u32!();
    /// let a = u256::from([0_u32, 10, 20, 30, 40, 50, 60, 70]);
    /// let e = a.get_num_(3);
    /// println!("a.get_num_(3) = {}", e);
    /// assert_eq!(e, 30);
    /// ```
    #[inline]
    #[cfg(target_endian = "little")]
    pub fn get_num_(&self, i: usize) -> T
    {
        self.number[i]
    }

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
    /// It is performance-oriented and does not care for safety. So, 
    /// if `i` >= `N`, it will panic.
    /// 
    /// # Counter Part Method
    /// Use this method only when you are sure that `i` < `N`.
    /// Otherwise, use its counter part method [get_num()](struct@BigUInt#method.get_num).
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for serious purpose. Only use this crate for Big-endian CPUs
    /// with your own full responsibility.
    /// 
    /// # Example
    /// ```
    /// use Cryptocol::number::*;
    /// use Cryptocol::define_utypes_with_u32;
    /// 
    /// define_utypes_with_u32!();
    /// let a = u256::from([0_u32, 10, 20, 30, 40, 50, 60, 70]);
    /// let e = a.get_num_(3);
    /// println!("a.get_num_(3) = {}", e);
    /// assert_eq!(e, 40);
    /// ```
    #[inline]
    #[cfg(target_endian = "big")]
    pub fn get_num_(&self, i: usize) -> T
    {
        self.number[N-1-i]
    }

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
    /// # Counter Part Method
    /// When you are sure that `i` < `N`, you may want to use its counter part
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
    /// # Counter Part Method
    /// When you are sure that `i` < `N`, you may want to use its counter part
    /// method [set_num_()](struct@BigUInt#method.set_num_) for performance.
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for serious purpose. Only use this crate for Big-endian CPUs
    /// with your own full responsibility.
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
    /// # Counter Part Method
    /// It is performance-oriented and does not care for safety.
    /// It is virtually the same as the method set_num(). This method set_num_()
    /// is considered to be slightly faster than the method set_num().
    /// Use this method set_num_() only when you are sure that `i` < `N`.
    /// Otherwise, use its counter part method [set_num()](struct@BigUInt#method.set_num).
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
    /// # Counter Part Method
    /// It is performance-oriented and does not care for safety.
    /// It is virtually the same as the method set_num(). This method set_num_()
    /// is considered to be slightly faster than the method set_num().
    /// Use this method set_num_() only when you are sure that `i` < `N`.
    /// Otherwise, use its counter part method [set_num()](struct@BigUInt#method.set_num).
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for serious purpose. Only use this crate for Big-endian CPUs
    /// with your own full responsibility.
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
    #[inline]
    #[cfg(target_endian = "big")]
    pub fn set_num_(&mut self, i: usize, val: T)
    {
        self.number[N-1-i] = val;
    }

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
    /// Returns the reference of its array of `T`-type for borrowing instead
    /// of giving its ownership. `BigUInt` has an array of `T` in order
    /// to present long-sized unsigned integers.
    /// 
    /// # Big-endian issue
    /// Only for big-endian compatible.
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for serious purpose. Only use this crate for Big-endian CPUs
    /// with your own full responsibility.
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
    #[inline]
    #[cfg(target_endian = "big")]
    pub fn get_number_mut(&self) -> &mut [T; N]
    {
        &mut self.number
    }

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
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for serious purpose. Only use this crate for Big-endian CPUs
    /// with your own full responsibility.
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
            Unbounded =>            { start = 0; }
        }
        match src.start_bound()
        {
            Excluded(s) =>  { end = (N - s); },
            Included(s) =>  { end = (N - 1 - s); },
            Unbounded =>            { end = N - 1; }
        }
        let new_src = Range::<&usize> { start: &start, end: &end };
        let new_dest = N - 1 - dest;
        self.number.copy_within(new_src, new_dest);
    }

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

    /// Checks whether `BigUInt` to be zero and returns true if it is
    /// zero and returns false if it is not zero.
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

    /// Sets `BigUInt` to be one.
    #[cfg(target_endian = "little")]
    pub fn set_one(&mut self)
    {
        for i in 1..N
            { self.set_num(i, T::zero()); }
        self.set_num(0, T::one());
    }

    /// Sets BigUInt to be one.
    #[cfg(target_endian = "big")]
    pub fn set_one(&mut self)
    {
        for i in 0..N-1
            { self.set_num(i, T::zero()); }
        self.set_num(N-1, T::one());
    }

    /// Checks whether `BigUInt` to be one and returns true if it is
    /// one, and returns false if it is not one. 
    #[cfg(target_endian = "little")]
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

    /// Checks whether `BigUInt` to be one and returns true if it is
    /// one, and returns false if it is not one. 
    #[cfg(target_endian = "big")]
    pub fn is_one(&self) -> bool
    {
        if self.get_num_(N-1) != T::one()
            { return false; }

        for i in 0..N-1
        {
            if self.get_num_(i) != T::zero()
                { return false; }
        }
        true
    }

    /// Sets `BigUInt`-typed number to be maximum value.
    /// 
    /// # Examples
    /// 
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

    /// Checks whether or not `BigUInt`-typed number to be maximum value.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let a = u256::max();
    /// println!("Is a maximun? - {}", a.is_max());
    /// assert_eq!(a.is_max(), true);
    /// ```
    pub fn is_max(&self) -> bool
    {
        for i in 0..N
        {
            if self.get_num_(i) != T::max()
                { return false; }
        }
        true
    }

    #[cfg(target_endian = "little")]
    pub fn set_uint(&mut self, val: T)
    {
        self.set_zero();
        self.set_num(0, val);
    }

    #[cfg(target_endian = "big")]
    pub fn set_uint(&mut self, val: T)
    {
        self.set_zero();
        self.set_num(N-1, val);
    }

    #[cfg(target_endian = "little")]
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

    #[cfg(target_endian = "big")]
    pub fn is_uint(&self, val: T) -> bool
    {
        if self.get_num(N-1) != val
        {
            false
        }
        else
        {
            for i in 0..N-1
            {
                if self.get_num(i) != T::zero()
                    { return false; }
            }
            true
        }
    }


    /***** METHODS FOR COMPARISON WITH UINT *****/

    /// Compares BigUInt or BigInt with a value of type T and returns the
    /// result of the comparison in the type `Option<Ordering>`. However, you'd
    /// better use the functions lt_uint(), gt_uint(), le_uint(), ge_uint(),
    /// and eq_uint(). Then, you don't have to use partial_cmp_uint() directly.
    fn partial_cmp_uint(&self, other: T) -> Option<Ordering>
    {
        if self.number[0] > other
        {
            return Some(Ordering::Greater);
        }
        else if self.number[0] < other
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

    /// Returns true if self is less than other. Otherwise, it returns false.
    fn lt_uint(&self, other: T) -> bool  { self.partial_cmp_uint(other).unwrap().is_lt() }

    /// Returns true if self is greater than other. Otherwise, it returns false.
    fn gt_uint(&self, other: T) -> bool  { self.partial_cmp_uint(other).unwrap().is_gt() }

    /// Returns true if self is less than or equal to other.
    /// Otherwise, it returns false.
    fn le_uint(&self, other: T) -> bool  { self.partial_cmp_uint(other).unwrap().is_le() }

    /// Returns true if self is greater than or equal to other.
    /// Otherwise, it returns false.
    fn ge_uint(&self, other: T) -> bool  { self.partial_cmp_uint(other).unwrap().is_ge() }

    /// Returns true if self is equal to other. Otherwise, it returns false.
    fn eq_uint(&self, other: T) -> bool  { self.partial_cmp_uint(other).unwrap().is_eq() }

    
    /***** ARITHMATIC OPERATIONS WITH UNSIGNED INTEGERS *****/

    /// Accumulates or adds rhs of type `T` to self which is of `BigUInt` type.
    pub fn accumulate(&mut self, rhs: T)
    {
        let zero = T::zero();
        let one = T::one();
        let mut midres = self.number[0].wrapping_add(rhs);
        let mut	carry = if midres < self.number[0] { one } else { zero };
        self.number[0] = midres;
        for i in 1..N
        {
            midres = self.number[i].wrapping_add(carry);
            carry = if midres < carry { one } else { zero };
            self.number[i] = midres;
            if carry == zero
                { break; }
        }
        if carry != T::zero()
            { self.set_overflow(); }
    }

    /// Dissipates or subtracts rhs of type `T` from self which is of
    /// `BigUInt` type.
    pub fn dissipate(&mut self, rhs: T)
    {
        let zero = T::zero();
        let one = T::one();
        let mut midres = self.number[0].wrapping_sub(rhs);
        let mut	carry= if midres > self.number[0] { one } else { zero };
        self.number[0] = midres;
        for i in 1..N
        {
            midres = self.number[i].wrapping_sub(carry);
            carry = if midres > self.number[i] { one } else { zero };
            self.number[i] = midres;
        }
        if carry != zero
        {
            if !self.is_untrustable()
                { self.set_underflow(); }
        }
    }

    /// Multiplies self which is of `BigUInt` type with rhs of type `T`.
    pub fn times(&mut self, rhs: T)
    {
        if self.is_zero()
            { return; }
        let zero = T::zero();
        let one = T::one();
        if rhs == zero
        {
            self.set_zero();
            return;
        }
        let adder = self.clone();
        let mut bit_check = one;
        bit_check <<= T::num((T::size_in_bits() - 1).into_u128());
        self.set_zero();
        while (bit_check != zero) && ((bit_check & rhs) == zero)
            { bit_check >>= one; }
        while bit_check != zero
        {
            *self <<= 1;
            if bit_check & rhs != zero
                { *self += adder; }
            bit_check >>= one;
        }
    }

    /// Divide BigUInt<T, N> by T so as to get quotient and remainder
    /// It returns tuple of quotient and remainder. quotient is Self and
    /// remainder is T. If rhs is zero, the divided_by_zero and overflow flags
    /// of quotient will be set, and the quotient and the remainder will be
    /// max value and zero, respectively.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use std::str::FromStr;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let dividend = u256::from_str("1234567890157589425462369896").unwrap();
    /// let (quotient, remainder) = dividend.divide_by_uint_fully(87_u128);
    /// ```
    pub fn divide_by_uint_fully(&self, rhs: T) -> (Self, T)
    {
        let mut quotient = Self::zero();
        let zero = T::zero();
        let one = T::one();
        if self.is_zero()
        {
            return (quotient, zero);
        }
        if rhs == zero
        {
            quotient.set_max();
            quotient.set_infinity();
            quotient.set_divided_by_zero();
            quotient.set_overflow();
            return (quotient, zero);
        }
        if self.lt_uint(rhs)
        {
            return (quotient, self.number[0]);
        }
        else if self.eq_uint(rhs)
        {
            quotient.set_uint(T::one());
            return (quotient, zero);
        }

        let mut adder = Self::zero();
        let mut res;
        let mut sum;
        let mut highest = Self::size_in_bits();
        let mut n = N - 1;
        let TSIZE_IN_BITS = T::size_in_bits();
        while (highest != 0) && (self.get_num_(n) == zero)
        {
            highest -= TSIZE_IN_BITS;
            n -= 1;
        }
        let mut piece = one << T::num(TSIZE_IN_BITS as u128 - 1);
        while self.get_num_(n) & piece == zero
        {
            highest -= 1;
            piece >>= one;
        }

        let mut high = highest;
        let mut low = 0;
        let mut mid = (high + low) >> 1;
        loop
        {
            high = highest;
            low = 0;
            if high == 0
            {
                return (quotient, (*self - quotient.mul_uint(rhs)).number[0]);
            }
            else    // if high > 0
            {
                loop
                {
                    mid = (high + low) >> 1;
                    adder.turn_check_bits(mid);
                    sum = quotient + adder;
                    res = sum.mul_uint(rhs);
                    if !res.is_overflow() && (*self > res)
                    {
                        if mid == highest - 1
                        {
                            quotient = sum;
                            highest = mid;
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
                    else if res.is_overflow() || res > *self
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
                        return (quotient + adder, zero);
                    }
                }
            }
        }
    }

    /// Divides self which is of `BigUInt` type by rhs which is of type `T`,
    /// and returns quotient of `BigUInt` type. If you get both quotient and
    /// remainder, you'd better use the function divide_by_uint_fully() instead
    /// of calling the functions quotient() and remainder() in series because
    /// they call the function divide_by_uint_fully() internally.
    pub fn quotient(&mut self, rhs: T) -> Self
    {
        let (quotient, _) = self.divide_by_uint_fully(rhs);
        quotient
    }

    /// Divides self which is of `BigUInt` type by rhs which is of type `T`,
    /// and returns remainder of type `T`. If you get both quotient and
    /// remainder, you'd better use the function divide_by_uint_fully() instead
    /// of calling the functions quotient() and remainder() in series because
    /// they call the function divide_by_uint_fully() internally.
    pub fn remainder(&mut self, rhs: T) -> T
    {
        let (_, remainder) = self.divide_by_uint_fully(rhs);
        remainder
    }

    /// Adds a unsigned integer number of type `T` to `BigUInt`-typed unsigned
    /// integer and returns its result in a type of BigUInt.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use std::str::FromStr;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let a = u256::from_str("10000000000000000000000000000000000").unwrap();
    /// let sum = a.add_uint(35);
    /// println!("sum = {}", sum);
    /// assert_eq!(sum, u256::from_str("10000000000000000000000000000000035").unwrap());
    /// ```
    pub fn add_uint(&self, rhs: T) -> Self
    {
        let mut bi = self.clone();
        bi.accumulate(rhs);
        bi
    }

    /// Subtracts a unsigned integer number of type `T` from `BigUInt`-typed
    /// unsigned integer and returns its result in a type of BigUInt.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use std::str::FromStr;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let a = u256::from_str("10000000000000000000000000000000000").unwrap();
    /// let sub = a.sub_uint(35_u128);
    /// println!("sub = {}", sub);
    /// ```
    pub fn sub_uint(&self, rhs: T) -> Self
    {
        let mut bi = self.clone();
        bi.dissipate(rhs);
        bi
    }

    /// Multiplies `BigUInt`-typed number with a unsigned integer number
    /// of type `T` and returns its result in a type of BigUInt.
    /// 
    /// # Examples
    /// ```
    /// use std::str::FromStr;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let a = u256::from_str("10000000000000000000000000000000000").unwrap();
    /// let mul = a.mul_uint(35);
    /// println!("mul = {}", mul);
    /// ```
    pub fn mul_uint(&self, rhs: T) -> Self
    {
        let mut bi = self.clone();
        bi.times(rhs);
        bi
    }

    /// Divides `BigUInt`-typed number with a unsigned integer number
    /// of type `T` and returns its quotient in a type of BigUInt.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use std::str::FromStr;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let a = u256::from_str("10000000000000000000000000000000000").unwrap();
    /// let div = a.div_uint(35);
    /// println!("div = {}", div);
    /// ```
    pub fn div_uint(&self, rhs: T) -> Self
    {
        let (quotient, _) = self.divide_by_uint_fully(rhs);
        quotient
    }

    /// Divides `BigUInt`-typed number with a unsigned integer number
    /// of type `T` and returns its remainder in a type of T.
    /// 
    /// # Examples
    /// ```
    /// use std::str::FromStr;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let a = u256::from_str("10000000000000000000000000000000000").unwrap();
    /// let rem = a.rem_uint(35_u128);
    /// println!("rem = {}", rem);
    /// ```
    pub fn rem_uint(&self, rhs: T) -> T
    {
        let (_, remainder) = self.divide_by_uint_fully(rhs);
        remainder
    }

    
    /***** SUPPLIMENTARY ARITHMATIC OPERATIONS WITH BigUInt *****/

    /// Divides self which is of `BigUInt` type by rhs which is of `BigUInt`
    /// type, and returns quotient and remainder which are `BigUInt`type.
    /// If rhs is zero, the divided_by_zero and overflow flags of quotient will
    /// be set, and the quotient will be set to be max value of `BigUInt`type,
    /// and the remainder will be set to be zero of `BigUInt` type.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use std::str::FromStr;
    /// use Cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let dividend = u256::from_str("1234567890157589425462369896").unwrap();
    /// let divisor = u256::from_str("1234567890").unwrap();
    /// let (quotient, remainder) = dividend.divide_fully(divisor);
    /// ```
    pub fn divide_fully(&self, rhs: Self) -> (Self, Self)
    {
        let mut quotient = Self::zero();
        let zero = T::zero();
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
        else if *self < rhs
        {
            return (quotient, self.clone());
        }
        else if *self == rhs
        {
            quotient.set_num(0, one);
            return (quotient, Self::zero());
        }

        let maximum = Self::size_in_bits() - 1;
        let mut adder = Self::zero();
        let mut res;
        let mut sum;
        let mut highest = Self::size_in_bits();
        let mut n = N - 1;
        let TSIZE_BITS = size_of::<T>() * 8;
        while self.get_num_(n) == zero && highest != 0
        {
            highest -= TSIZE_BITS;
            n -= 1;
        }
        let mut piece = one << T::num(TSIZE_BITS as u128 - 1);
        while self.get_num_(n) & piece == zero
        {
            highest -= 1;
            piece >>= one;
        }

        let mut high = highest;
        let mut low = 0;
        let mut mid = (high + low) >> 1;
        loop
        {
            high = highest;
            low = 0;
            if high == 0
            {
                return (quotient, *self - quotient * rhs);
            }
            else    // if high > 0
            {
                loop
                {
                    mid = (high + low) >> 1;
                    adder.turn_check_bits(mid);
                    sum = quotient + adder;
                    res = sum * rhs;
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


    /***** METHODS FOR CONVERTING INTO OTHER TYPES WITH/WITHOUT LOSS *****/

    pub fn into_biguint<U, const M: usize>(&self) -> BigUInt<U, M>
    where U: Uint + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        BigUInt::<U, M>::from_biguint(&self)
    }

    pub fn into_u128(&self) -> u128
    {
        let mut num = ULonger { ulonger: 0 };
        match T::size_in_bytes()
        {
            1 => {
                unsafe { num.byte[0] = self.number[0].into_u8(); }
                unsafe { if N > 1 { num.byte[1] = self.number[1].into_u8() } }
                unsafe { if N > 2 { num.byte[2] = self.number[2].into_u8() } }
                unsafe { if N > 3 { num.byte[3] = self.number[3].into_u8() } }
                unsafe { if N > 4 { num.byte[4] = self.number[4].into_u8() } }
                unsafe { if N > 5 { num.byte[5] = self.number[5].into_u8() } }
                unsafe { if N > 6 { num.byte[6] = self.number[6].into_u8() } }
                unsafe { if N > 7 { num.byte[7] = self.number[7].into_u8() } }
                unsafe { if N > 8 { num.byte[8] = self.number[8].into_u8() } }
                unsafe { if N > 9 { num.byte[9] = self.number[9].into_u8() } }
                unsafe { if N > 10 { num.byte[10] = self.number[10].into_u8() } }
                unsafe { if N > 11 { num.byte[11] = self.number[11].into_u8() } }
                unsafe { if N > 12 { num.byte[12] = self.number[12].into_u8() } }
                unsafe { if N > 13 { num.byte[13] = self.number[13].into_u8() } }
                unsafe { if N > 14 { num.byte[14] = self.number[14].into_u8() } }
                unsafe { if N > 15 { num.byte[15] = self.number[15].into_u8() } }
                },
            2 => {
                unsafe { num.ushort[0] = self.number[0].into_u16(); }
                unsafe { if N > 1 { num.ushort[1] = self.number[1].into_u16() } }
                unsafe { if N > 2 { num.ushort[2] = self.number[2].into_u16() } }
                unsafe { if N > 3 { num.ushort[3] = self.number[3].into_u16() } }
                unsafe { if N > 4 { num.ushort[4] = self.number[4].into_u16() } }
                unsafe { if N > 5 { num.ushort[5] = self.number[5].into_u16() } }
                unsafe { if N > 6 { num.ushort[6] = self.number[6].into_u16() } }
                unsafe { if N > 7 { num.ushort[7] = self.number[7].into_u16() } }
                },
            4 => {
                unsafe { num.uint[0] = self.number[0].into_u32(); }
                unsafe { if N > 2 { num.uint[1] = self.number[1].into_u32(); } }
                unsafe { if N > 3 { num.uint[2] = self.number[2].into_u32(); } }
                unsafe { if N > 4 { num.uint[3] = self.number[3].into_u32(); } }
                },
            8 => { 
                unsafe { num.ulong[0] = self.number[0].into_u64(); }
                unsafe { if N > 1 { num.ulong[1] = self.number[1].into_u64(); } }
                },
            _ => { return self.number[0].into_u128(); },
        }
        unsafe { num.ulonger }
    }

    pub fn into_u64(&self) -> u64
    {
        let mut num = ULonger { ulonger: 0 };
        match size_of::<T>()
        {
            1 => {
                unsafe { num.byte[0] = self.number[0].into_u8(); }
                unsafe { if N > 1 { num.byte[1] = self.number[1].into_u8() } }
                unsafe { if N > 2 { num.byte[2] = self.number[2].into_u8() } }
                unsafe { if N > 3 { num.byte[3] = self.number[3].into_u8() } }
                unsafe { if N > 4 { num.byte[4] = self.number[4].into_u8() } }
                unsafe { if N > 5 { num.byte[5] = self.number[5].into_u8() } }
                unsafe { if N > 6 { num.byte[6] = self.number[6].into_u8() } }
                unsafe { if N > 7 { num.byte[7] = self.number[7].into_u8() } }
                },
            2 => {
                unsafe { num.ushort[0] = self.number[0].into_u16(); }
                unsafe { if N > 1 { num.ushort[1] = self.number[1].into_u16() } }
                unsafe { if N > 2 { num.ushort[2] = self.number[2].into_u16() } }
                unsafe { if N > 3 { num.ushort[3] = self.number[3].into_u16() } }
                },
            4 => {
                unsafe { num.uint[0] = self.number[0].into_u32(); }
                unsafe { if N > 1 { num.uint[1] = self.number[1].into_u32(); } }
                },
            8 => { return self.number[0].into_u64(); },
            _ => { num.ulonger = self.number[0].into_u128(); },
        }
        unsafe { num.ulong[0] }
    }

    pub fn into_u32(&self) -> u32
    {
        let mut num = ULonger { ulonger: 0 };
        match size_of::<T>()
        {
            1 => {
                unsafe { num.byte[0] = self.number[0].into_u8(); }
                unsafe { if N > 1 { num.byte[1] = self.number[1].into_u8() } }
                unsafe { if N > 2 { num.byte[2] = self.number[2].into_u8() } }
                unsafe { if N > 3 { num.byte[3] = self.number[3].into_u8() } }
                },
            2 => {
                unsafe { num.ushort[0] = self.number[0].into_u16(); }
                unsafe { if N > 1 { num.ushort[1] = self.number[1].into_u16() } }
                },
            4 => { return self.number[0].into_u32(); },
            8 => { unsafe { num.ulong[0] = self.number[0].into_u64(); } },
            _ => { num.ulonger = self.number[0].into_u128(); },
        }
        unsafe { num.uint[0] }
    }

    /// little endian
    /// 
    #[cfg(target_endian = "little")]
    pub fn into_u16(&self) -> u16
    {
        let mut num = ULonger { ulonger: 0 };
        match size_of::<T>()
        {
            1 => {
                unsafe { num.byte[0] = self.number[0].into_u8(); }
                unsafe { if N > 1 { num.byte[1] = self.number[1].into_u8() } }
                },
            2 => { return self.number[0].into_u16(); },
            4 => { unsafe { num.uint[0] = self.number[0].into_u32(); } },
            8 => { unsafe { num.ulong[0] = self.number[0].into_u64(); } },
            _ => { num.ulonger = self.number[0].into_u128(); },
        }
        unsafe { num.ushort[0] }
    }

    /// big endian
    /// 
    #[cfg(target_endian = "big")]
    pub fn into_u16(&self) -> u16
    {
        let mut num = ULonger { ulonger: 0 };
        match size_of::<T>()
        {
            1 => {
                unsafe { num.byte[15] = self.number[N-1].into_u8(); }
                unsafe { if N > 1 { num.byte[14] = self.number[N-2].into_u8() } }
                },
            2 => { return self.number[N-1].into_u16(); },
            4 => { unsafe { num.uint[3] = self.number[N-1].into_u32(); } },
            8 => { unsafe { num.ulong[1] = self.number[N-1].into_u64(); } },
            _ => { num.ulonger = self.number[N-1].into_u128(); },
        }
        unsafe { num.ushort[7] }
    }

    /// little endian
    /// 
    #[cfg(target_endian = "little")]
    pub fn into_u8(&self) -> u8         { self.number[0].into_u8() }

    /// big endian
    /// 
    #[cfg(target_endian = "big")]
    pub fn into_u8(&self) -> u8         { self.number[N-1].into_u8() }

    pub fn to_string_with_radix_and_delimiter(&self, radix: usize, stride: usize, delimiter: &str) -> String
    {
        self.to_string_with_radix_and_stride(radix, stride).replace("_", delimiter)
    }

    pub fn to_string_with_radix_and_stride(&self, radix: usize, stride: usize) -> String
    {
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
                (dividend, remainder) = dividend.divide_by_uint_fully(T::num(radix as u128));
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
            num_str
        }
    }

    /// Reads the value of BigUInt<T, N> and write it into String
    /// with a certain radix.
    pub fn to_string_with_radix(&self, radix: usize) -> String
    {
        let mut txt = String::new();
        let zero = Self::zero();
        let mut dividend = self.clone();
        let mut remainder;
        loop
        {
            (dividend, remainder) = dividend.divide_by_uint_fully(T::num(radix as u128));
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
        num_str
    }


    /***** FLAG MANIPULATION *****/

    #[inline] pub fn set_flag_bit(&mut self, flag: u8)      { self.flag |= flag; }
    #[inline] pub fn reset_flag_bit(&mut self, flag: u8)    { self.flag &= !flag; }
    #[inline] pub fn is_flag_bit_on(&self, flag: u8) -> bool    { (self.flag & flag) != 0 }
    #[inline] pub fn reset_all_flags(&mut self)             { self.flag = 0; }
    #[inline] pub fn set_infinity(&mut self)      { self.set_flag_bit(Self::INFINITY); }
    #[inline] pub fn reset_inifinity(&mut self)   { self.reset_flag_bit(Self::INFINITY); }
    #[inline] pub fn is_inifinity(&self) -> bool  { self.is_flag_bit_on(Self::INFINITY) }
    #[inline] pub fn set_divided_by_zero(&mut self)   { self.set_flag_bit(Self::DIVIDED_BY_ZERO); }
    #[inline] pub fn reset_divided_by_zero(&mut self) { self.reset_flag_bit(Self::DIVIDED_BY_ZERO); }
    #[inline] pub fn is_divided_by_zero(&self) -> bool { self.is_flag_bit_on(Self::DIVIDED_BY_ZERO) }

    /// Sets overflow flag.
    #[inline] pub fn set_overflow(&mut self)
    {
        self.set_flag_bit(Self::OVERFLOW);
    }

    /// Resets overflow flag.
    #[inline] pub fn reset_overflow(&mut self)
    {
        self.reset_flag_bit(Self::OVERFLOW);
    }

    /// Checks whether or not overflow flag is set, and returns true
    /// if the overflow flag is set. Otherwise, it returns false.
    #[inline] pub fn is_overflow(&self) -> bool
    {
        self.is_flag_bit_on(Self::OVERFLOW)
    }

    /// Sets underflow flag.
    #[inline] pub fn set_underflow(&mut self)
    {
        self.set_flag_bit(Self::UNDERFLOW);
    }

    /// Reets underflow flag.
    #[inline] pub fn reset_underflow(&mut self)
    {
        self.reset_flag_bit(Self::UNDERFLOW);
    }

    /// Checks whether or not underflow flag is set, and returns true
    /// if the underflow flag is set. Otherwise, it returns false.
    #[inline] pub fn is_underflow(&self) -> bool
    {
        self.is_flag_bit_on(Self::UNDERFLOW)
    }

    /// Sets both overflow flag and underflow flag.
    #[inline] pub fn set_untrustable(&mut self)
    {
        self.set_flag_bit(Self::OVERFLOW | Self::UNDERFLOW);
    }

    /// Resets both overflow flag and underflow flag.
    #[inline] pub fn reset_untrustable(&mut self)
    {
        self.reset_flag_bit(Self::OVERFLOW | Self::UNDERFLOW);
    }

    /// Checks whether or not both overflow flag and underflow flag are all set,
    /// and returns true if both of the overflow flag and the underflow flag
    /// are all set. Otherwise, it returns false.
    #[inline] pub fn is_untrustable(&self) -> bool
    {
        self.is_flag_bit_on(Self::OVERFLOW | Self::UNDERFLOW)
    }
}
