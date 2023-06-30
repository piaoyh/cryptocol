// Copyright 2023 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! A big unsigned integer with user-defined fixed size.

//#![warn(missing_docs)]
//#![warn(missing_doc_code_examples)]
use std::fmt::{ self, Display, Formatter, Debug };
use std::mem::{ size_of, transmute };
use std::cmp::{ PartialEq, PartialOrd, Ordering };
use std::ops::*;

use super::uint::*;
use super::HugeInteger;
use super::BigInteger;


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


/// 256-bit unsigned integer, Synonym of `u256_with_u128`
#[cfg(target_pointer_width = "128")] pub type u256 = u256_with_u128;

/// 512-bit unsigned integer for 128-bit machines, Synonym of `u512_with_u128`
#[cfg(target_pointer_width = "128")] pub type u512 = u512_with_u128;

/// 1024-bit unsigned integer for 128-bit machines, Synonym of `u1024_with_u128`
#[cfg(target_pointer_width = "128")] pub type u1024 = u1024_with_u128;

/// 2048-bit unsigned integer for 128-bit machines, Synonym of `u2048_with_u128`
#[cfg(target_pointer_width = "128")] pub type u2048 = u2048_with_u128;

/// 3072-bit unsigned integer for 128-bit machines, Synonym of `u3072_with_u128`
#[cfg(target_pointer_width = "128")] pub type u3072 = u3072_with_u128;

/// 4096-bit unsigned integer for 128-bit machines, Synonym of `u4096_with_u128`
#[cfg(target_pointer_width = "128")] pub type u4096 = u4096_with_u128;

/// 5120-bit unsigned integer for 128-bit machines, Synonym of `u5120_with_u128`
#[cfg(target_pointer_width = "128")] pub type u5120 = u5120_with_u128;

/// 6144-bit unsigned integer for 128-bit machines, Synonym of `u6144_with_u128`
#[cfg(target_pointer_width = "128")] pub type u6144 = u6144_with_u128;

/// 7168-bit unsigned integer for 128-bit machines, Synonym of `u7168_with_u128`
#[cfg(target_pointer_width = "128")] pub type u7168 = u7168_with_u128;

/// 8192-bit unsigned integer for 128-bit machines, Synonym of `u8192_with_u128`
#[cfg(target_pointer_width = "128")] pub type u8192 = u8192_with_u128;

/// 16384-bit unsigned integer for 128-bit machines, Synonym of `u16384_with_u128`
#[cfg(target_pointer_width = "128")] pub type u16384 = u16384_with_u128;


/// 256-bit unsigned integer for 64-bit machines, Synonym of `u256_with_u64`
#[cfg(target_pointer_width = "64")] pub type u256 = u256_with_u64;

/// 512-bit unsigned integer for 64-bit machines, Synonym of `u512_with_u64`
#[cfg(target_pointer_width = "64")] pub type u512 = u512_with_u64;

/// 1024-bit unsigned integer for 64-bit machines, Synonym of `u1024_with_u64`
#[cfg(target_pointer_width = "64")] pub type u1024 = u1024_with_u64;

/// 2048-bit unsigned integer for 64-bit machines, Synonym of `u2048_with_u64`
#[cfg(target_pointer_width = "64")] pub type u2048 = u2048_with_u64;

/// 3072-bit unsigned integer for 64-bit machines, Synonym of `u3072_with_u64`
#[cfg(target_pointer_width = "64")] pub type u3072 = u3072_with_u64;

/// 4096-bit unsigned integer for 64-bit machines, Synonym of `u4096_with_u64`
#[cfg(target_pointer_width = "64")] pub type u4096 = u4096_with_u64;

/// 5120-bit unsigned integer for 64-bit machines, Synonym of `u5120_with_u64`
#[cfg(target_pointer_width = "64")] pub type u5120 = u5120_with_u64;

/// 6144-bit unsigned integer for 64-bit machines, Synonym of `u6144_with_u64`
#[cfg(target_pointer_width = "64")] pub type u6144 = u6144_with_u64;

/// 7168-bit unsigned integer for 64-bit machines, Synonym of `u7168_with_u64`
#[cfg(target_pointer_width = "64")] pub type u7168 = u7168_with_u64;

/// 8192-bit unsigned integer for 64-bit machines, Synonym of `u8192_with_u64`
#[cfg(target_pointer_width = "64")] pub type u8192 = u8192_with_u64;

/// 16384-bit unsigned integer for 64-bit machines, Synonym of `u16384_with_u64`
#[cfg(target_pointer_width = "64")] pub type u16384 = u16384_with_u64;


/// 256-bit unsigned integer for 32-bit machines, Synonym of `u256_with_u32`
#[cfg(target_pointer_width = "32")] pub type u256 = u256_with_u32;

/// 512-bit unsigned integer for 32-bit machines, Synonym of `u512_with_u32`
#[cfg(target_pointer_width = "32")] pub type u512 = u512_with_u32;

/// 1024-bit unsigned integer for 32-bit machines, Synonym of `u1024_with_u32`
#[cfg(target_pointer_width = "32")] pub type u1024 = u1024_with_u32;

/// 2048-bit unsigned integer for 32-bit machines, Synonym of `u2048_with_u32`
#[cfg(target_pointer_width = "32")] pub type u2048 = u2048_with_u32;

/// 3072-bit unsigned integer for 32-bit machines, Synonym of `u3072_with_u32`
#[cfg(target_pointer_width = "32")] pub type u3072 = u3072_with_u32;

/// 4096-bit unsigned integer for 32-bit machines, Synonym of `u4096_with_u32`
#[cfg(target_pointer_width = "32")] pub type u4096 = u4096_with_u32;

/// 5120-bit unsigned integer for 32-bit machines, Synonym of `u5120_with_u32`
#[cfg(target_pointer_width = "32")] pub type u5120 = u5120_with_u32;

/// 6144-bit unsigned integer for 32-bit machines, Synonym of `u6144_with_u32`
#[cfg(target_pointer_width = "32")] pub type u6144 = u6144_with_u32;

/// 7168-bit unsigned integer for 32-bit machines, Synonym of `u7168_with_u32`
#[cfg(target_pointer_width = "32")] pub type u7168 = u7168_with_u32;

/// 8192-bit unsigned integer for 32-bit machines, Synonym of `u8192_with_u32`
#[cfg(target_pointer_width = "32")] pub type u8192 = u8192_with_u32;

/// 16384-bit unsigned integer for 32-bit machines, Synonym of `u16384_with_u32`
#[cfg(target_pointer_width = "32")] pub type u16384 = u16384_with_u32;


/// 256-bit unsigned integer for 16-bit machines, Synonym of `u256_with_u16`
#[cfg(target_pointer_width = "16")] pub type u256 = u256_with_u16;

/// 512-bit unsigned integer for 16-bit machines, Synonym of `u512_with_u16`
#[cfg(target_pointer_width = "16")] pub type u512 = u512_with_u16;

/// 1024-bit unsigned integer for 16-bit machines, Synonym of `u1024_with_u16`
#[cfg(target_pointer_width = "16")] pub type u1024 = u1024_with_u16;

/// 2048-bit unsigned integer for 16-bit machines, Synonym of `u2048_with_u16`
#[cfg(target_pointer_width = "16")] pub type u2048 = u2048_with_u16;

/// 3072-bit unsigned integer for 16-bit machines, Synonym of `u3072_with_u16`
#[cfg(target_pointer_width = "16")] pub type u3072 = u3072_with_u16;

/// 4096-bit unsigned integer for 16-bit machines, Synonym of `u4096_with_u16`
#[cfg(target_pointer_width = "16")] pub type u4096 = u4096_with_u16;

/// 5120-bit unsigned integer for 16-bit machines, Synonym of `u5120_with_u16`
#[cfg(target_pointer_width = "16")] pub type u5120 = u5120_with_u16;

/// 6144-bit unsigned integer for 16-bit machines, Synonym of `u6144_with_u16`
#[cfg(target_pointer_width = "16")] pub type u6144 = u6144_with_u16;

/// 7168-bit unsigned integer for 16-bit machines, Synonym of `u7168_with_u16`
#[cfg(target_pointer_width = "16")] pub type u7168 = u7168_with_u16;

/// 8192-bit unsigned integer for 16-bit machines, Synonym of `u8192_with_u16`
#[cfg(target_pointer_width = "16")] pub type u8192 = u8192_with_u16;

/// 16384-bit unsigned integer for 16-bit machines, Synonym of `u16384_with_u16`
#[cfg(target_pointer_width = "16")] pub type u16384 = u16384_with_u16;


/// 256-bit unsigned integer for 8-bit machines, Synonym of `u256_with_u8`
#[cfg(target_pointer_width = "8")] pub type u256 = u256_with_u8;

/// 512-bit unsigned integer for 8-bit machines, Synonym of `u512_with_u8`
#[cfg(target_pointer_width = "8")] pub type u512 = u512_with_u8;

/// 1024-bit unsigned integer for 8-bit machines, Synonym of `u1024_with_u8`
#[cfg(target_pointer_width = "8")] pub type u1024 = u1024_with_u8;

/// 2048-bit unsigned integer for 8-bit machines, Synonym of `u2048_with_u8`
#[cfg(target_pointer_width = "8")] pub type u2048 = u2048_with_u8;

/// 3072-bit unsigned integer for 8-bit machines, Synonym of `u3072_with_u8`
#[cfg(target_pointer_width = "8")] pub type u3072 = u3072_with_u8;

/// 4096-bit unsigned integer for 8-bit machines, Synonym of `u4096_with_u8`
#[cfg(target_pointer_width = "8")] pub type u4096 = u4096_with_u8;

/// 5120-bit unsigned integer for 8-bit machines, Synonym of `u5120_with_u8`
#[cfg(target_pointer_width = "8")] pub type u5120 = u5120_with_u8;

/// 6144-bit unsigned integer for 8-bit machines, Synonym of `u6144_with_u8`
#[cfg(target_pointer_width = "8")] pub type u6144 = u6144_with_u8;

/// 7168-bit unsigned integer for 8-bit machines, Synonym of `u7168_with_u8`
#[cfg(target_pointer_width = "8")] pub type u7168 = u7168_with_u8;

/// 8192-bit unsigned integer for 8-bit machines, Synonym of `u8192_with_u8`
#[cfg(target_pointer_width = "8")] pub type u8192 = u8192_with_u8;

/// 16384-bit unsigned integer for 8-bit machines, Synonym of `u16384_with_u8`
#[cfg(target_pointer_width = "8")] pub type u16384 = u16384_with_u8;


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


/// A struct that represents a big unsigned integer with user-defined fixed size.
/// 
/// The struct `BigUInt<T, const N: usize>` is a generic struct for which you
/// can decide the type of elements and its number. It is Little Endian.
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
/// 
/// ```
/// use Cryptoology::number::*;
/// type BI = BigUInt::<u64, 16>;
/// type AI = BigUInt::<usize, 16>;
/// let bi = BI::from_array(&[1;N]);
/// let bb = BI::from_string_with_radix("0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__", 16).unwrap();
/// let b = BI::from_string("1234567891234567879123456789111111111222222222333333333444444444555555555666666666777777777888888888999999999000000000").unwrap();
/// println!("bi = {:?}", bi);
/// println!("bi = {}", bi.to_string_with_radix(16));
/// println!("bb = {:?}", bb);
/// println!("bb = {}", bb.to_string_with_radix(16));
/// println!("b = {}", b);
/// println!("b * bb = {}", b * bb);
/// println!("bb / b = {}", bb / b);
/// println!("bb % b = {}", bb % b);
/// println!("b + 1 = {}", b.add_uint(1));
/// println!("b - 1 = {}", b.sub_uint(1));
/// let a = AI::from_string("123654789147258369").unwrap();
/// println!("a = {}", a.to_strin());
/// ```
#[derive(Debug, Copy, Clone)]
pub struct BigUInt<T, const N: usize>
where T: Uint + Copy + Clone + Display + Debug + ToString
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
where T: Uint + Copy + Clone + Display + Debug + ToString
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
        //+ HugeInteger<T>
{
    /// Constructs a new `BigUInt<T, N>`.
    /// All the attributes of te constructed object will be initialized with `0`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use Cryptocol::number::BigUInt;
    /// let big_int = BigUInt::<u64,16>::new();
    /// assert_eq!(big_int, BigUInt::<u64,16>::from_uint(0));
    /// ```
    pub fn new() -> Self
    {
        Self { number: [T::zero(); N], flag: 0, }   // unsafe { zeroed::<Self>() }
    }

    /// Constructs a new `BigUInt<T, N>` which has the value of zero.
    /// This function calls `BigUInt<T, N>::new()`, so it is virtually exactly
    /// the same as the function `BigUInt<T, N>::new()`. Your source code will
    /// be better readable if you use `BigUInt<T, N>::zero()` instead of
    /// `BigUInt<T, N>::new()` especially when you create the big number zero.
    ///
    /// # Examples
    /// 
    /// ```
    /// use Cryptocol::number::BigUInt;
    /// let zero = BigUInt::<u64,16>::zero();
    /// assert_eq!(zero, BigUInt::<u64,16>::from_uint(0));
    /// ```
    #[inline]
    pub fn zero() -> Self
    {
        Self::new()   // unsafe { zeroed::<Self>() }
    }

    /// Constructs a new `BigUInt<T, N>` from an array of type `T`
    /// with `N` elements.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use Cryptocol::number::BigUInt;
    /// let big_num = BigUInt::<u8,32>::from_array(&[1;32]);
    /// assert_eq!(big_num, BigUInt::<u8,32>::from_string_with_radix("0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__0000_0000_0000_0001__", 16).unwrap(););
    /// ```
    pub fn from_array(val: &[T; N]) -> Self
    {
        let mut s = Self::new();
        s.set_number(val);
        s
    }

    /// Constructs a new `BigUInt<T, N>` from an unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, `u128` and `usize`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use Cryptocol::number::BigUInt;
    /// let cc = BigUInt::<u8,32>::from_uint(1004);
    /// assert_eq!(cc.into_u32(), 1004);
    /// ```
    /// 
    #[cfg(target_endian = "little")]
    pub fn from_uint<S>(val: S) -> Self
    where S: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=S> + AddAssign + Sub<Output=S> + SubAssign
            + Mul<Output=S> + MulAssign + Div<Output=S> + DivAssign
            + Shl<Output=S> + ShlAssign + Shr<Output=S> + ShrAssign
            + BitAnd<Output=S> + BitAndAssign + BitOr<Output=S> + BitOrAssign
            + BitXor<Output=S> + BitXorAssign + Not<Output=S>
            + PartialEq + PartialOrd
    {
        let TSIZE = size_of::<T>();
        let SSIZE = size_of::<S>();
        let mut me = Self::new();
        let mut share = Share::<T, S>::from_src(val);
        
        if TSIZE >= SSIZE
        {
            unsafe { me.set_num(0, share.des); }
        }
        else
        {
            let TSIZE_BIT = TSIZE * 8;
            for i in 0..SSIZE/TSIZE
            {
                unsafe { me.set_num(i, share.des); }
                unsafe { share.src >>= S::num(TSIZE_BIT as u128); }
            }
        }
        return me;
    }

    /// Constructs a new BigUInt<T, N> from an unsigned integer
    /// such as u8, u16, u32, u64, u128 and usize. The BigEndian version
    /// is so experimental that you may not want to use it for serious purpose.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use Cryptocol::number::BigUInt;
    /// let cc = BigUInt::<u8,32>::from_uint(1004);
    /// assert_eq!(cc.into_u32(), 1004);
    /// ```
    /// 
    #[cfg(target_endian = "big")]
    pub fn from_uint<S>(val: S) -> Self
    where S: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=S> + AddAssign + Sub<Output=S> + SubAssign
            + Mul<Output=S> + MulAssign + Div<Output=S> + DivAssign
            + Shl<Output=S> + ShlAssign + Shr<Output=S> + ShrAssign
            + BitAnd<Output=S> + BitAndAssign + BitOr<Output=S> + BitOrAssign
            + BitXor<Output=S> + BitXorAssign + Not<Output=S>
            + PartialEq + PartialOrd
    {
        let TSIZE = size_of::<T>();
        let SSIZE = size_of::<S>();
        let mut me = Self::new();
        let mut share = Share::<T, S>::from_src(val);
        
        if TSIZE >= SSIZE
        {
            unsafe { me.set_num(N-1, share.des); }
        }
        else    // if TSIZE < SSIZE
        {
            let TSIZE_BIT = TSIZE * 8;
            let LEN = SSIZE/TSIZE;
            if LEN <= N
            {
                for i in 0..LEN
                {
                    unsafe { me.set_num(N - LEN + i, share.des); }
                    unsafe { share.src <<= S::num(TSIZE_BIT as u128); }
                }    
            }
            else    // if LEN > N
            {
                unsafe { share.src <<= S::num(((LEN - N) * TSIZE_BIT) as u128); }
                for i in 0..N
                {
                    unsafe { me.set_num(i, share.des); }
                    unsafe { share.src <<= S::num(TSIZE_BIT as u128); }
                } 
            }
        }
        return me;
    }

    /// Constructs a new `BigUInt<T, N>` from a string with radix.
    /// The constructed object will be wrapped in `Some(BigUInt<T, N>)` if it is
    /// successfully created. Otherwise, this function returns None.
    /// 
    /// The radix can be from 2 up to 62 (= 10 + 26 + 26). Such a radix is 1 or
    /// more than 62 is not available, so that this function will return None.
    /// 
    /// If the radix is more than 10 and less than 37, the digit bigger than 9
    /// will be expressed with alphabets. The avaiable alphabets are
    /// case-insensitive. For example, the digit whose value is 10, 11, 15, 16,
    /// 35 and 36 are A or a, B or b, F or f, G or g, Y or y, and Z or z,
    /// respectively.
    /// 
    /// However, if the radix is more than 36 and less than 62, the digit bigger
    /// than 9 will be expressed with alphabets. The avaiable alphabets are
    /// case-sensitive, so A is different from a. For instance, the digit whose
    /// value is 10, 11, 35, 36, 37, 38, 60 and 61 are A, B, Y, Z, a, b, y and z,
    /// respectively.
    /// 
    /// In the string expressing a number, you can separate the digits with '_'
    /// in order to make it more readable. So, "10000" is the same as "1_0000".
    /// 
    /// # Examples
    /// 
    /// ```
    /// use Cryptocol::number::BigUInt;
    /// let bi = BigUInt::<u8,32>::from_string_with_radix("A16F", 16);
    /// assert_eq!(bi.into_u16(), 0xA16F_u16);
    /// ```
    pub fn from_string_with_radix(txt: &str, radix: usize) -> Option<Self>
    {
        if (radix < 2) || (radix > 10 + 26 + 26)
            { return None; }

        let mut bignum = Self::zero();
        for c in txt.chars()
        {
            if c == '_'
                { continue; }
            if !c.is_alphanumeric()
                { return None; }
            if radix <= 10
            {
                if c as usize >= '0' as usize + radix
                    { return None; }
            }
            else if radix <= 10 + 26
            {
                if (c as usize >= 'A' as usize + radix - 10) 
                        && (c as usize <= 'Z' as usize)
                    || (c as usize >= 'a' as usize + radix - 10)
                    { return None; }
            }
            else if c as usize >= 'a' as usize + radix - (10 + 26)  // radix <= 10 + 26 + 26
                { return None; }

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
        Some(bignum)
    }

    /// Constructs a new `BigUInt<T, N>` from a string with radix 10.
    /// The constructed object will be wrapped in `Some(BigUInt<T, N>)` if it is
    /// successfully created. Otherwise, this method returns `None`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use Cryptocol::number::BigUInt;
    /// let bi = BigUInt<u8,32>::from_string_with_radix("A16F", 16);
    /// ```
    #[inline]
    pub fn from_string(txt: &str) -> Option<Self>
    {
        Self::from_string_with_radix(txt, 10)
    }

    /// Returns how many bytes BigUInt contains. The return type is usize.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use Cryptocol::number::BigUInt;
    /// # fn main()
    /// # {
    /// let bi = u256::from_string_with_radix("A16F", 16).unwrap();
    /// assert_eq!(u256::size_in_bytes(), 256 / 8);
    /// # }
    /// ```
    pub fn size_in_bytes() -> usize { T::size_in_bytes() * N }

    pub fn size_in_bits() -> usize { Self::size_in_bytes() * 8 }

    /// Returns how many bytes the object of BigUInt type contains.
    /// The return type is usize.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # fn main()
    /// # {
    /// let bi = u256::from_string_with_radix("A16F", 16).unwrap();
    /// assert_eq!(bi.size_in_bytes(), 256 / 8);
    /// # }
    /// ```
    pub fn length_in_bytes(&self) -> usize { Self::size_in_bytes() }

    pub fn length_in_bits(&self) -> usize { Self::size_in_bits() }

    /// Constructs a new `BigUInt<T, N>` from another kind of `BigUInt<U, M>`.
    /// It copies not only long bit integer but also current flags from another
    /// kind of `BigUInt<U, M>`.
    /// 
    /// # Example
    /// 
    /// ```
    /// use std::mem::size_of;
    /// # use number::*;
    /// type T = u16;
    /// const N: usize = 16;
    /// const M: usize = size_of::<T>() * N;
    /// type BI = BigUInt::<T, N>;
    /// type AI = BigUInt::<u8, M>;
    /// let a = AI::from_string("123456789123456789123456789123456789123456789123456789").unwrap();
    /// let b = BI::from_biguint(&a);
    /// println!("a = {}", a);
    /// println!("b = {}", b);
    /// ```
    /// 
    #[cfg(target_endian = "little")]
    pub fn from_biguint<U, const M: usize>(biguint: &BigUInt<U, M>) -> Self
    where U: Uint + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let my_size = Self::size_in_bytes();
        let src_size = BigUInt::<U, M>::size_in_bytes();//size_of::<U>() * M;  //
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
    /// It copies not only long bit integer but also current flags from another
    /// kind of `BigUInt<U, M>`. It is so experimental for Big Endian CPU that
    /// you may not want to use it for serious purpose.
    /// 
    /// # Example
    /// 
    /// ```
    /// use std::mem::size_of;
    /// 
    /// type T = u16;
    /// const N: usize = 16;
    /// const M: usize = size_of::<T>() * N;
    /// type BI = BigUInt::<T, N>;
    /// type AI = BigUInt::<u8, M>;
    /// let a = AI::from_string("123456789123456789123456789123456789123456789123456789").unwrap();
    /// let b = BI::from_biguint(&a);
    /// println!("a = {}", a);
    /// println!("b = {}", b);
    /// ```
    /// 
    #[cfg(target_endian = "big")]
    pub fn from_biguint<U, const M: usize>(biguint: &BigUInt<U, M>) -> Self
    where U: Uint + Copy + Clone + Display + Debug + ToString
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

    /// Constucts a new `BigUInt<T, N>` which has the value zero
    /// and sets only the bit specified by bit_pos to be 1.
    /// 
    fn make_check_bits(bit_pos: usize) -> Self
    {
        let mut check_bits = Self::zero();
        check_bits.turn_check_bits(bit_pos);
        check_bits
    }

    /// Sets only the bit specified by bit_pos to be 1.
    /// 
    #[cfg(target_endian = "little")]
    fn turn_check_bits(&mut self, bit_pos: usize)
    {
        let TSIZE_BIT = size_of::<T>() * 8;
        let chunk_num = bit_pos / TSIZE_BIT;
        let piece_num = bit_pos % TSIZE_BIT;
        let mut val = T::one();
        val <<= T::num(piece_num as u128);
        self.set_zero();
        self.set_num(chunk_num, val);
    }

    /// Sets only the bit specified by bit_pos to be 1. It is so experimental
    /// for Big Endian CPU that you may be discouraged to use it for serious
    /// purpose.
    /// 
    #[cfg(target_endian = "big")]
    fn turn_check_bits(&mut self, bit_pos: usize)
    {
        let TSIZE_BIT = size_of::<T>() * 8;
        let chunk_num = N - 1 - bit_pos / TSIZE_BIT;
        let piece_num = bit_pos % TSIZE_BIT;
        let mut val = T::one();
        val <<= T::num(piece_num as u128);
        self.set_zero();
        self.set_num(chunk_num, val);
    }

    /// Adds a unsigned integer number of type `T` to `BigUInt`-typed unsigned
    /// integer and returns its result in a type of BigUInt.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use big_uint::*;
    /// let a = u1024::from_string("10000000000000000000000000000000000").unwrap();
    /// let sum = a.add_uint(35);
    /// println!("sum = {}", sum);
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
    /// use big_uint::*;
    /// let a = u1024::from_string("10000000000000000000000000000000000").unwrap();
    /// let sub = a.sub_uint(35);
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
    /// 
    /// ```
    /// use big_uint::*;
    /// let a = u1024::from_string("10000000000000000000000000000000000").unwrap();
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
    /// use big_uint::*;
    /// let a = u1024::from_string("10000000000000000000000000000000000").unwrap();
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
    /// 
    /// ```
    /// use big_uint::*;
    /// let a = u1024::from_string("10000000000000000000000000000000000").unwrap();
    /// let rem = a.rem_uint(35);
    /// println!("rem = {}", rem);
    /// ```
    pub fn rem_uint(&self, rhs: T) -> T
    {
        let (_, remainder) = self.divide_by_uint_fully(rhs);
        remainder
    }

    /// Sets `BigUInt`-typed number to be maximum value.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut a = u1024::new();
    /// a.set_max();
    /// println!("a = {}", a);
    /// ```
    pub fn set_max(&mut self)
    {
        for i in 0..N
            { self.set_num(i, T::Max()); }
    }

    /// Checks whether or not `BigUInt`-typed number to be maximum value.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut a = u1024::new();
    /// a.set_max();
    /// println!("Is a maximun? - {}", a.is_max());
    /// ```
    pub fn is_max(&self) -> bool
    {
        for i in 0..N
        {
            if self.get_num(i).unwrap() != T::Max()
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
        if self.get_num(0).unwrap() != val
        {
            false
        }
        else
        {
            for i in 1..N
            {
                if self.get_num(i).unwrap() != T::zero()
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

    pub fn into_biguint<U, const M: usize>(&self) -> BigUInt<U, M>
    where U: Uint + Copy + Clone + Display + Debug + ToString
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
}

impl<T, const N: usize> BigInteger<T, N> for BigUInt<T, N>
where T: Uint + Copy + Clone + Display + Debug + ToString
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
{
    /// Returns the reference of its array of T-type for borrowing instead
    /// of giving its ownership. BigUInt has an array of T in order
    /// to present long-sized unsigned integers.
    #[inline] fn get_number(&self) -> &[T; N]
    {
        &self.number
    }

    #[inline] fn set_number(&mut self, val: &[T; N])    { self.number.copy_from_slice(val); }

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
}


impl<T, const N: usize> HugeInteger<T> for BigUInt<T, N>
where T: Uint + Copy + Clone + Display + Debug + ToString
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
{
    /// Sets i-th element of its array of type T and return true if i < N.
    /// Otherwise, it sets none of the elements of its array of type T and
    /// returns false. BigUInt and BigInt have an array of T in order to
    /// present long-sized unsigned and signed integers, respectively.
    fn set_num(&mut self, i: usize, val: T) -> bool
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

    /// Reads the value of BigUInt<T, N> and write it into String
    /// with a certain radix.
    fn to_string_with_radix(&self, radix: usize) -> String
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

    /// Divides self which is of `BigUInt` type by rhs which is of `BigUInt`
    /// type, and returns quotient and remainder which are `BigUInt`type.
    /// If rhs is zero, the divided_by_zero and overflow flags of quotient will
    /// be set, and the quotient will be set to be max value of `BigUInt`type,
    /// and the remainder will be set to be zero of `BigUInt` type.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use Cryptocol::number::BigUInt;
    /// let dividend = u1024::from_string("1234567890157589425462369896");
    /// let divisor = u1024::from_string("1234567890");
    /// let (quotient, remainder) = dividend.divide_fully(divisor);
    /// ```
    fn divide_fully(&self, rhs: Self) -> (Self, Self)
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
        while self.get_num(n).unwrap() == zero && highest != 0
        {
            highest -= TSIZE_BITS;
            n -= 1;
        }
        let mut piece = one << T::num(TSIZE_BITS as u128 - 1);
        while self.get_num(n).unwrap() & piece == zero
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

    /// Accumulates or adds rhs of type `T` to self which is of `BigUInt` type.
    fn accumulate(&mut self, rhs: T)
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
    fn dissipate(&mut self, rhs: T)
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
    fn times(&mut self, rhs: T)
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
        bit_check <<= T::num((size_of::<T>() * 8 - 1).into_u128());
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
    /// use Cryptocol::number::BigUInt;
    /// let dividend = u1024::from_string("1234567890157589425462369896");
    /// let divisor = T::num(87_u128);
    /// let (quotient, remainder) = dividend.divide_by_uint_fully(divisor);
    /// ```
    fn divide_by_uint_fully(&self, rhs: T) -> (Self, T)
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
        while (highest != 0) && (self.get_num(n).unwrap() == zero)
        {
            highest -= TSIZE_IN_BITS;
            n -= 1;
        }
        let mut piece = one << T::num(TSIZE_IN_BITS as u128 - 1);
        while self.get_num(n).unwrap() & piece == zero
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
    fn quotient(&mut self, rhs: T) -> Self
    {
        let (quotient, _) = self.divide_by_uint_fully(rhs);
        quotient
    }

    /// Divides self which is of `BigUInt` type by rhs which is of type `T`,
    /// and returns remainder of type `T`. If you get both quotient and
    /// remainder, you'd better use the function divide_by_uint_fully() instead
    /// of calling the functions quotient() and remainder() in series because
    /// they call the function divide_by_uint_fully() internally.
    fn remainder(&mut self, rhs: T) -> T
    {
        let (_, remainder) = self.divide_by_uint_fully(rhs);
        remainder
    }

    #[inline] fn set_flag_bit(&mut self, flag: u8)      { self.flag |= flag; }
    #[inline] fn reset_flag_bit(&mut self, flag: u8)    { self.flag &= !flag; }
    #[inline] fn is_flag_bit_on(&self, flag: u8) -> bool    { (self.flag & flag) != 0 }
}


impl<T, const N: usize> Add for BigUInt<T, N>
where T: Uint + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self
    {
        let mut s = self.clone();
        s += rhs;
        s
    }
}

impl<T, const N: usize> AddAssign for BigUInt<T, N>
where T: Uint + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /// Adds and assign the result to it.
    /// 
    #[cfg(target_endian = "little")]
    fn add_assign(&mut self, rhs: Self)
    {
        let zero = T::zero();
        let mut	carry: T = zero;
        let mut midres: T;
        let mut c: bool;

        for i in 0..N
        {
            midres = self.number[i].wrapping_add(rhs.number[i]);
            c = midres < self.number[i];
            midres = midres.wrapping_add(carry);
            carry = if c || (midres < carry) { T::one() } else { zero };
            self.number[i] = midres;
        }
        if carry != zero
            { self.set_overflow(); }
    }

    #[cfg(target_endian = "big")]
    fn add_assign(&mut self, rhs: Self)
    {
        let zero = T::zero();
        let mut	carry: T = zero;
        let mut midres: T;
        let mut c: bool;

        let mut i = N - 1;
        loop
        {
            midres = self.number[i].wrapping_add(rhs.number[i]);
            c = midres < self.number[i];
            midres = midres.wrapping_add(carry);
            carry = if c || (midres < carry) { T::one() } else { zero };
            self.number[i] = midres;
            if i == 0
                { break; }
            i -= 1;
        }

        if carry != zero
            { self.set_overflow(); }
    }
}

impl<T, const N: usize> Sub for BigUInt<T, N>
where T: Uint + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self
    {
        let mut s = self.clone();
        s -= rhs;
        s
    }
}

impl<T, const N: usize> SubAssign for BigUInt<T, N>
where T: Uint + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /// Subtracts and assign the result to it.
    /// 
    #[cfg(target_endian = "little")]
    fn sub_assign(&mut self, rhs: Self)
    {
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
    }

    /// Subtracts and assign the result to it.
    /// 
    #[cfg(target_endian = "big")]
    fn sub_assign(&mut self, rhs: Self)
    {
        let zero = T::zero();
        let mut	carry: T = zero;
        let mut midres: T;
        let mut c: bool;
        let mut cc: T;
        let mut i = N;
        loop
        {
            i -= 1;
            midres = self.number[i].wrapping_sub(rhs.number[i]);
            c = midres > self.number[i];
            cc = midres;
            midres = midres.wrapping_sub(carry);
            carry = if c || (midres > cc) { T::one() } else { zero };
            self.number[i] = midres;
            if i == 0
                { break; }
        }
        if carry != zero
            { self.set_underflow(); }
    }
}

impl<T, const N: usize> Mul for BigUInt<T, N>
where T: Uint + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self
    {
        let mut s = self.clone();
        s *= rhs;
        s
    }
}

impl<T, const N: usize> MulAssign for BigUInt<T, N>
where T: Uint + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /// Multiplies and assign the result to it.
    /// 
    #[cfg(target_endian = "little")]
    fn mul_assign(&mut self, rhs: Self)
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
        let TSIZE_BIT = size_of::<T>() * 8;
        let mut multiply_first = |num: T| {
            let mut bit_check = one;
            bit_check <<= T::num((TSIZE_BIT - 1).into_u128());
            while (bit_check != zero) && (bit_check & num == zero)
                { bit_check >>= one; }

            self.set_zero();
            while bit_check != zero
            {
                *self <<= 1;
                if bit_check & num != zero
                    { *self += adder; }
                bit_check >>= one;
            }
        };

        let mut n = N - 1;
        while rhs.number[n] == zero
            { n -= 1; }
        multiply_first(rhs.number[n]);
        if n == 0
            { return; }
        n -= 1;

        let mut multiply = |num: T| {
            if num == T::zero()
            {
                *self <<= TSIZE_BIT as i32;
                return;
            }
            let mut bit_check = one;
            bit_check <<= T::num((TSIZE_BIT - 1).into_u128());
            while bit_check != zero
            {
                *self <<= 1;
                if bit_check & num != zero
                    { *self += adder; }
                bit_check >>= one;
            }
        };

        loop
        {
            multiply(rhs.number[n]);
            if n == 0
                { break; }
            n = n.wrapping_sub(1);
        }
    }

    /// Multiplies and assign the result to it.
    /// 
    #[cfg(target_endian = "big")]
    fn mul_assign(&mut self, rhs: Self)
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
        let TSIZE_BIT = size_of::<T>() * 8;
        let mut multiply_first = |num: T| {
            let mut bit_check = one;
            bit_check <<= T::num((TSIZE_BIT - 1).into_u128());
            while (bit_check != zero) && (bit_check & num == zero)
                { bit_check >>= one; }

            self.set_zero();
            while bit_check != zero
            {
                *self <<= 1;
                if bit_check & num != zero
                    { *self += adder; }
                bit_check >>= one;
            }
        };

        let mut n = 0;
        while rhs.number[n] == zero
            { n += 1; }
        multiply_first(rhs.number[n]);
        n += 1;

        let mut multiply = |num: T| {
            if num == T::zero()
            {
                *self <<= TSIZE_BIT as i32;
                return;
            }
            let mut bit_check = one;
            bit_check <<= T::num((TSIZE_BIT - 1).into_u128());
            while bit_check != zero
            {
                *self <<= 1;
                if bit_check & num != zero
                    { *self += adder; }
                bit_check >>= one;
            }
        };
        while n < N
        {
            multiply(rhs.number[n]);
            n += 1;
        }
    }
}

impl<T, const N: usize> Div for BigUInt<T, N>
where T: Uint + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self
    {
        let (quotient, _) = self.divide_fully(rhs);
        quotient
    }
}

impl<T, const N: usize> DivAssign for BigUInt<T, N>
where T: Uint + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    fn div_assign(&mut self, rhs: Self) { *self = *self / rhs; }
}

impl<T, const N: usize> Rem for BigUInt<T, N>
where T: Uint + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;
    fn rem(self, rhs: Self) -> Self
    {
        let (_, remainder) = self.divide_fully(rhs);
        remainder
    }
}

impl<T, const N: usize> RemAssign for BigUInt<T, N>
where T: Uint + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    fn rem_assign(&mut self, rhs: Self) { *self = *self % rhs; }
}

impl<T, const N: usize> Shl<i32> for BigUInt<T, N>
where T: Uint + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;
    fn shl(self, rhs: i32) -> Self
    {
        let mut s = self.clone();
        s <<= rhs;
        s
    }
}

impl<T, const N: usize> ShlAssign<i32> for BigUInt<T, N>
where T: Uint + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /// for Little endian 
    #[cfg(target_endian = "little")]
    fn shl_assign(&mut self, rhs: i32)
    {
        if rhs < 0
        {
		    *self >>= -rhs;
            return;
        }
        let TSIZE_BIT = size_of::<T>() * 8;
        let chunk_num = rhs as usize / TSIZE_BIT as usize;
        let piece_num = rhs as usize % TSIZE_BIT as usize;
        let zero = T::zero();
        self.flag = 0;
        if chunk_num > 0
        {
            for i in N-chunk_num..N
            {
                if self.number[i] > zero
                {
                    self.set_overflow();
                    break;
                }
            }
            self.number.copy_within(0..N-chunk_num, chunk_num);
            for idx in 0..chunk_num
                { self.number[idx] = zero; }
        }
        if piece_num == 0
            { return; }
        if (self.number[N-1] >> T::num((TSIZE_BIT - piece_num).into_u128())) != zero
            { self.set_overflow(); }

        let mut num: T;
        let mut carry = zero;
        for idx in chunk_num..N
        {
            num = (self.number[idx] << T::num(piece_num.into_u128())) | carry;
            carry = self.number[idx] >> T::num((TSIZE_BIT - piece_num).into_u128());
            self.number[idx] = num;
        }
        if carry != zero
            { self.set_overflow(); }
    }

    /// for Big endian 
    #[cfg(target_endian = "big")]
    fn shl_assign(&mut self, rhs: i32)
    {
        if rhs < 0
        {
            *self >>= -rhs;
            return;
        }
        let TSIZE_BIT = size_of::<T>() * 8;
        let chunk_num = rhs as usize / TSIZE_BIT as usize;
        let piece_num = rhs as usize % TSIZE_BIT as usize;
        let zero = T::zero();
        self.flag = 0;
        if chunk_num > 0
        {
            for i in 0..N-chunk_num
            {
                if self.number[i] > zero
                {
                    self.set_overflow();
                    break;
                }
            }
            self.number.copy_within(chunk_num..N, 0);
            for idx in N-chunk_num..N
                { self.number[idx] = zero; }
        }
        if piece_num == 0
            { return; }
        if (self.number[0] >> T::num((TSIZE_BIT - piece_num).into_u128())) != zero
            { self.set_overflow(); }

        let mut num: T;
        let mut carry = zero;
        let mut idx = N - 1 - chunk_num;
        loop
        {
            num = (self.number[idx] << T::num(piece_num.into_u128())) | carry;
            carry = self.number[idx] >> T::num((TSIZE_BIT - piece_num).into_u128());
            self.number[idx] = num;
            if idx == 0
                { break; }
            idx -= 1;
        }
        if carry != zero
            { self.set_overflow(); }
    }
}

impl<T, const N: usize> Shr<i32> for BigUInt<T, N>
where T: Uint + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;
    fn shr(self, rhs: i32) -> Self
    {
        let mut s = self.clone();
        s >>= rhs;
        s
    }
}

impl<T, const N: usize> ShrAssign<i32> for BigUInt<T, N>
where T: Uint + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /// for Little endian 
    #[cfg(target_endian = "little")]
    fn shr_assign(&mut self, rhs: i32)
    {
        if rhs < 0
        {
		    *self <<= -rhs;
            return;
        }
        let TSIZE_BIT = size_of::<T>() * 8;
        let chunk_num = rhs as usize / TSIZE_BIT as usize;
        let piece_num = rhs as usize % TSIZE_BIT as usize;
        let zero = T::zero();
        self.flag = 0;
        if chunk_num > 0
        {
            for i in 0..chunk_num
            {
                if self.number[i] > zero
                {
                    self.set_underflow();
                    break;
                }
            }
            self.number.copy_within(chunk_num..N, 0);
            for idx in N-chunk_num..N
                { self.number[idx] = zero; }
        }
        if piece_num == 0
            { return; }
        if (self.number[0] << T::num((TSIZE_BIT - piece_num).into_u128())) != zero
            { self.set_underflow(); }

        let mut num: T;
        let mut carry = T::zero();
        let mut idx = N - 1 - chunk_num;
        loop
        {
            num = (self.number[idx] >> T::num(piece_num.into_u128())) | carry;
            carry = self.number[idx] << T::num((TSIZE_BIT - piece_num).into_u128());
            self.number[idx] = num;
            if idx == 0
                { break; }
            idx -= 1;
        }
        if carry != zero
            { self.set_underflow(); }
    }

    /// for Big endian 
    #[cfg(target_endian = "big")]
    fn shr_assign(&mut self, rhs: i32)
    {
        use std::slice::Chunks;

        if rhs < 0
        {
		    *self <<= -rhs;
            return;
        }
        let TSIZE_BIT = size_of::<T>() * 8;
        let chunk_num = rhs as usize / TSIZE_BIT as usize;
        let piece_num = rhs as usize % TSIZE_BIT as usize;
        let zero = T::zero();
        self.flag = 0;
        if chunk_num > 0
        {
            for i in N-chunk_num..N
            {
                if self.number[i] > zero
                {
                    self.set_underflow();
                    break;
                }
            }
            self.number.copy_within(0..N-chunk_num, chunk_num);
            for idx in 0..chunk_num
                { self.number[idx] = zero; }
        }
        if piece_num == 0
            { return; }
        if (self.number[N-1] << T::num((TSIZE_BIT - piece_num).into_u128())) != zero
            { self.set_underflow(); }

        let mut num: T;
        let mut carry = T::zero();
        let mut idx = 0;
        for idx in 0..N-chunk_num
        {
            num = (self.number[idx] >> T::num(piece_num.into_u128())) | carry;
            carry = self.number[idx] << T::num((TSIZE_BIT - piece_num).into_u128());
            self.number[idx] = num;
        }
        if carry != zero
            { self.set_underflow(); }
    }
}

impl<T, const N: usize> BitAnd for BigUInt<T, N>
where T: Uint + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self
    {
        let mut s = self.clone();
        s &= rhs;
        s
    }
}

impl<T, const N: usize> BitAndAssign for BigUInt<T, N>
where T: Uint + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    fn bitand_assign(&mut self, rhs: Self)
    {
        for idx in 0..N
            { self.number[idx] &= rhs.number[idx]; }
    }
}

impl<T, const N: usize> BitOr for BigUInt<T, N>
where T: Uint + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self
    {
        let mut s = self.clone();
        s |= rhs;
        s
    }
}

impl<T, const N: usize> BitOrAssign for BigUInt<T, N>
where T: Uint + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    fn bitor_assign(&mut self, rhs: Self)
    {
        for idx in 0..N
            { self.number[idx] |= rhs.number[idx]; }
    }
}

impl<T, const N: usize> BitXor for BigUInt<T, N>
where T: Uint + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self
    {
        let mut s = self.clone();
        s ^= rhs;
        s
    }
}

impl<T, const N: usize> BitXorAssign for BigUInt<T, N>
where T: Uint + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    fn bitxor_assign(&mut self, rhs: Self)
    {
        for idx in 0..N
            { self.number[idx] ^= rhs.number[idx]; }
    }
}

impl<T, const N: usize> Not for BigUInt<T, N>
where T: Uint + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;
    fn not(self) -> Self
    {
        let mut s = self.clone();
        for idx in 0..N
            { s.number[idx] = !s.number[idx]; }
        s
    }
}

impl<T, const N: usize> PartialEq for BigUInt<T, N>
where T: Uint + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    fn eq(&self, other: &Self) -> bool
    {
        for idx in 0..N
        {
            if self.number[idx] != other.number[idx]
                { return false; }
        }
        true
    }

    fn ne(&self, other: &Self) -> bool  { !(self == other) }
}

impl<T, const N: usize> PartialOrd for BigUInt<T, N>
where T: Uint + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /// for little endian 
    #[cfg(target_endian = "little")]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        let mut idx = N - 1;
        loop
        {
            if self.number[idx] > other.number[idx]
                { return Some(Ordering::Greater); }
            else if self.number[idx] < other.number[idx]
                { return Some(Ordering::Less); }
            if idx == 0
                { break; }
            idx -= 1;
        }
        Some(Ordering::Equal)
    }

    /// for Big endian 
    #[cfg(target_endian = "big")]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        for idx in 0..N
        {
            if self.number[idx] > other.number[idx]
                { return Some(Ordering::Greater); }
            else if self.number[idx] < other.number[idx]
                { return Some(Ordering::Less); }
        }
        Some(Ordering::Equal)
    }

    fn lt(&self, other: &Self) -> bool  { self.partial_cmp(&other).unwrap().is_lt() }
    fn gt(&self, other: &Self) -> bool  { self.partial_cmp(&other).unwrap().is_gt() }
    fn le(&self, other: &Self) -> bool  { self.partial_cmp(&other).unwrap().is_le() }
    fn ge(&self, other: &Self) -> bool  { self.partial_cmp(&other).unwrap().is_ge() }
}

impl<T, const N: usize> Display for BigUInt<T, N>
where T: Uint + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // Automatically to_string() will be implemented.
    // `f` is a buffer, this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)
        write!(f, "{}", self.to_string_with_radix(10))
    }
}
