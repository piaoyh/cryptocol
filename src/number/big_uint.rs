// Copyright 2023, 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module that contains a big unsigned integer struct
//! with user-defined fixed size and its methods.

// #![warn(missing_docs)]
// #![warn(missing_doc_code_examples)]
#![allow(missing_docs)]
#![allow(missing_doc_code_examples)]

use std::fmt::{ Display, Debug };
use std::mem::size_of;
use std::cmp::{ PartialEq, PartialOrd, Ordering };
use std::convert::From;
use std::str::FromStr;
use std::ops::*;

use crate::number::{ SmallUInt, LongerUnion, SharedValues, SharedArrays, NumberErr };


/// 256-bit unsigned integer implemented by `BigUInt<u128, 2>` made with two `u128`s
#[allow(non_camel_case_types)] pub type U256_with_u128 = BigUInt<u128, 2>;

/// 512-bit unsigned integer implemented by `BigUInt<u128, 4>` made with four `u128`s
#[allow(non_camel_case_types)] pub type U512_with_u128 = BigUInt<u128, 4>;

/// 1024-bit unsigned integer implemented by `BigUInt<u128, 8>` made with eight `u128`s
#[allow(non_camel_case_types)] pub type U1024_with_u128 = BigUInt<u128, 8>;

/// 2048-bit unsigned integer implemented by `BigUInt<u128, 4>` made with sixteen `u128`s
#[allow(non_camel_case_types)] pub type U2048_with_u128 = BigUInt<u128, 16>;

/// 3072-bit unsigned integer implemented by `BigUInt<u128, 4>` made with twenty-four `u128`s
#[allow(non_camel_case_types)] pub type U3072_with_u128 = BigUInt<u128, 24>;

/// 4096-bit unsigned integer implemented by `BigUInt<u128, 4>` made with thirty-two `u128`s
#[allow(non_camel_case_types)] pub type U4096_with_u128 = BigUInt<u128, 32>;

/// 5120-bit unsigned integer implemented by `BigUInt<u128, 4>` made with forty `u128`s
#[allow(non_camel_case_types)] pub type U5120_with_u128 = BigUInt<u128, 40>;

/// 6144-bit unsigned integer implemented by `BigUInt<u128, 4>` made with fory-eight `u128`s
#[allow(non_camel_case_types)] pub type U6144_with_u128 = BigUInt<u128, 48>;

/// 7168-bit unsigned integer implemented by `BigUInt<u128, 4>` made with fifty-six `u128`s
#[allow(non_camel_case_types)] pub type U7168_with_u128 = BigUInt<u128, 56>;

/// 8192-bit unsigned integer implemented by `BigUInt<u128, 4>` made with sixty-four `u128`s
#[allow(non_camel_case_types)] pub type U8192_with_u128 = BigUInt<u128, 64>;

/// 16384-bit unsigned integer implemented by `BigUInt<u128, 4>` made with one hundred twenty-eight `u128`s
#[allow(non_camel_case_types)] pub type U16384_with_u128 = BigUInt<u128, 128>;


/// 256-bit unsigned integer implemented by `BigUInt<u64, 4>` made with four `u64`s
#[allow(non_camel_case_types)] pub type U256_with_u64 = BigUInt<u64, 4>;

/// 512-bit unsigned integer implemented by `BigUInt<u64, 8>` made with eight `u64`s
#[allow(non_camel_case_types)] pub type U512_with_u64 = BigUInt<u64, 8>;

/// 1024-bit unsigned integer implemented by `BigUInt<u64, 16>` made with sixteen `u64`s
#[allow(non_camel_case_types)] pub type U1024_with_u64 = BigUInt<u64, 16>;

/// 2048-bit unsigned integer implemented by `BigUInt<u64, 32>` made with thirty-two `u64`s
#[allow(non_camel_case_types)] pub type U2048_with_u64 = BigUInt<u64, 32>;

/// 3072-bit unsigned integer implemented by `BigUInt<u64, 48>` made with fourty-eight `u64`s
#[allow(non_camel_case_types)] pub type U3072_with_u64 = BigUInt<u64, 48>;

/// 4096-bit unsigned integer implemented by `BigUInt<u64, 64>` made with sixty-four `u64`s
#[allow(non_camel_case_types)] pub type U4096_with_u64 = BigUInt<u64, 64>;

/// 5120-bit unsigned integer implemented by `BigUInt<u64, 80>` made with eighty `u64`s
#[allow(non_camel_case_types)] pub type U5120_with_u64 = BigUInt<u64, 80>;

/// 6144-bit unsigned integer implemented by `BigUInt<u64, 96>` made with ninety-six `u64`s
#[allow(non_camel_case_types)] pub type U6144_with_u64 = BigUInt<u64, 96>;

/// 7168-bit unsigned integer implemented by BigUInt<u64, 112> made with one hundred twelve `u64`s
#[allow(non_camel_case_types)] pub type U7168_with_u64 = BigUInt<u64, 112>;

/// 8192-bit unsigned integer implemented by `BigUInt<u64, 128>` made with two hundred twenty-eight `u64`s
#[allow(non_camel_case_types)] pub type U8192_with_u64 = BigUInt<u64, 128>;

/// 16384-bit unsigned integer implemented by `BigUInt<u64, 256>` made with two hundred fifty-six `u64`s
#[allow(non_camel_case_types)] pub type U16384_with_u64 = BigUInt<u64, 256>;


/// 256-bit unsigned integer implemented by `BigUInt<u32, 8>` made with eight `u32`s
#[allow(non_camel_case_types)] pub type U256_with_u32 = BigUInt<u32, 8>;

/// 512-bit unsigned integer implemented by `BigUInt<u32, 8>` made with sixteen `u32`s
#[allow(non_camel_case_types)] pub type U512_with_u32 = BigUInt<u32, 16>;

/// 1024-bit unsigned integer implemented by `BigUInt<u32, 8>` made with thirty-two `u32`s
#[allow(non_camel_case_types)] pub type U1024_with_u32 = BigUInt<u32, 32>;

/// 2048-bit unsigned integer implemented by `BigUInt<u32, 8>` made with sixty-four `u32`s
#[allow(non_camel_case_types)] pub type U2048_with_u32 = BigUInt<u32, 64>;

/// 3072-bit unsigned integer implemented by `BigUInt<u32, 8>` made with ninety-six `u32`s
#[allow(non_camel_case_types)] pub type U3072_with_u32 = BigUInt<u32, 96>;

/// 4096-bit unsigned integer implemented by `BigUInt<u32, 8>` made with one hundred twenty-eight `u32`s
#[allow(non_camel_case_types)] pub type U4096_with_u32 = BigUInt<u32, 128>;

/// 5120-bit unsigned integer implemented by `BigUInt<u32, 8>` made with one hundred sixty `u32`s
#[allow(non_camel_case_types)] pub type U5120_with_u32 = BigUInt<u32, 160>;

/// 6144-bit unsigned integer implemented by `BigUInt<u32, 8>` made with one hundred ninety-two `u32`s
#[allow(non_camel_case_types)] pub type U6144_with_u32 = BigUInt<u32, 192>;

/// 7168-bit unsigned integer implemented by `BigUInt<u32, 8>` made with two hundred twenty-four `u32`s
#[allow(non_camel_case_types)] pub type U7168_with_u32 = BigUInt<u32, 224>;

/// 8192-bit unsigned integer implemented by `BigUInt<u32, 8>` made with two hundred fifty-six `u32`s
#[allow(non_camel_case_types)] pub type U8192_with_u32 = BigUInt<u32, 256>;

/// 16384-bit unsigned integer implemented by `BigUInt<u32, 8>` made with five hundred twelve `u32`s
#[allow(non_camel_case_types)] pub type U16384_with_u32 = BigUInt<u32, 512>;


/// 256-bit unsigned integer implemented by `BigUInt<u16, 16>` made with sixteen `u16`s
#[allow(non_camel_case_types)] pub type U256_with_u16 = BigUInt<u16, 16>;

/// 512-bit unsigned integer implemented by `BigUInt<u16, 32>` made with thirty-two `u16`s
#[allow(non_camel_case_types)] pub type U512_with_u16 = BigUInt<u16, 32>;

/// 1024-bit unsigned integer implemented by `BigUInt<u16, 64>` made with sixty-four `u16`s
#[allow(non_camel_case_types)] pub type U1024_with_u16 = BigUInt<u16, 64>;

/// 2048-bit unsigned integer implemented by `BigUInt<u16, 128>` made with one hundred twenty-eight `u16`s
#[allow(non_camel_case_types)] pub type U2048_with_u16 = BigUInt<u16, 128>;

/// 3072-bit unsigned integer implemented by `BigUInt<u16, 192>` made with one hundred ninety-two `u16`s
#[allow(non_camel_case_types)] pub type U3072_with_u16 = BigUInt<u16, 192>;

/// 4096-bit unsigned integer implemented by `BigUInt<u16, 256>` made with two hundred fifty-six `u16`s
#[allow(non_camel_case_types)] pub type U4096_with_u16 = BigUInt<u16, 256>;

/// 5120-bit unsigned integer implemented by `BigUInt<u16, 320>` made with three hundred twenty `u16`s
#[allow(non_camel_case_types)] pub type U5120_with_u16 = BigUInt<u16, 320>;

/// 6144-bit unsigned integer implemented by `BigUInt<u16, 384>` made with three hundred eighty-four `u16`s
#[allow(non_camel_case_types)] pub type U6144_with_u16 = BigUInt<u16, 384>;

/// 7168-bit unsigned integer implemented by `BigUInt<u16, 448>` made with four hundred forty-eight `u16`s
#[allow(non_camel_case_types)] pub type U7168_with_u16 = BigUInt<u16, 448>;

/// 8192-bit unsigned integer implemented by `BigUInt<u16, 512>` made with five hundred twelve `u16`s
#[allow(non_camel_case_types)] pub type U8192_with_u16 = BigUInt<u16, 512>;

/// 16384-bit unsigned integer implemented by `BigUInt<u16, 1024>` made with one thousand twenty-four `u16`s
#[allow(non_camel_case_types)] pub type U16384_with_u16 = BigUInt<u16, 1024>;


/// 256-bit unsigned integer implemented by `BigUInt<u8, 32>` made with thirty-two `u8`s
#[allow(non_camel_case_types)] pub type U256_with_u8 = BigUInt<u8, 32>;

/// 512-bit unsigned integer implemented by `BigUInt<u8, 64>` made with sixty-four `u8`s
#[allow(non_camel_case_types)] pub type U512_with_u8 = BigUInt<u8, 64>;

/// 1024-bit unsigned integer implemented by `BigUInt<u8, 128>` made with one hundred twenty-eight `u8`s
#[allow(non_camel_case_types)] pub type U1024_with_u8 = BigUInt<u8, 128>;

/// 2048-bit unsigned integer implemented by `BigUInt<u8, 256>` made with two hundred fifty-six `u8`s
#[allow(non_camel_case_types)] pub type U2048_with_u8 = BigUInt<u8, 256>;

/// 3072-bit unsigned integer implemented by `BigUInt<u8, 384>` made with three hundred eighty-four `u8`s
#[allow(non_camel_case_types)] pub type U3072_with_u8 = BigUInt<u8, 384>;

/// 4096-bit unsigned integer implemented by `BigUInt<u8, 512>` made with five hundred twelve `u8`s
#[allow(non_camel_case_types)] pub type U4096_with_u8 = BigUInt<u8, 512>;

/// 5120-bit unsigned integer implemented by `BigUInt<u8, 640>` made with six hundred forty-eight `u8`s
#[allow(non_camel_case_types)] pub type U5120_with_u8 = BigUInt<u8, 640>;

/// 6144-bit unsigned integer implemented by `BigUInt<u8, 768>` made with seven hundred sixty-eight `u8`s
#[allow(non_camel_case_types)] pub type U6144_with_u8 = BigUInt<u8, 768>;

/// 7168-bit unsigned integer implemented by `BigUInt<u8, 896>` made with eight hundred ninety-six `u8`s
#[allow(non_camel_case_types)] pub type U7168_with_u8 = BigUInt<u8, 896>;

/// 8192-bit unsigned integer implemented by `BigUInt<u8, 1024>` made with one thousand twenty-four `u8`s
#[allow(non_camel_case_types)] pub type U8192_with_u8 = BigUInt<u8, 1024>;

/// 16384-bit unsigned integer implemented by `BigUInt<u8, 2048>` made with two thousand forty-eight `u8`s
#[allow(non_camel_case_types)] pub type U16384_with_u8 = BigUInt<u8, 2048>;


//////////////////////////////////////////
/// # Introduction
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
/// If `size_of::<T>() * N` <= `128`, some methods may panic
/// or its behavior may be undefined though it may not panic.
/// 
/// # Quick Start
/// It is generic data type. So, you can define 1024-bit unsigned integer
/// as follows. Note that `u1024` is not keyword of Rust while `u128` is one
/// of Rust keywords. So, you can use `u1024` as a user-defined datatype.
/// 
/// ## Example 1
/// ```
/// use cryptocol::number::*;
/// type u1024 = BigUInt::<u128, 8>;
/// ```
/// Then, you can use `u1024` in the similar way to how to use `u8` or `u64`.
/// And, `u1024` is composed of eight `u128`s.
/// Of course, you can define the same `u1024` as follows.
/// 
/// ## Example 2
/// ```
/// use cryptocol::number::*;
/// type u1024 = BigUInt::<u64, 16>;
/// ```
/// Then, it is composed of sixteen `u64`s though it is the same size of
/// type `BigUInt::<u128, 8>`. As you've already understood, you can define
/// `u1024` to be `BigUInt::<u8, 128>` too. It is totally up to you.
/// 
/// ## Performance
/// However, in both release mode and debug mode, if you consider performance
/// for addition and subtraction, you are highly encouraged to use
/// `BigUInt::<u128, 8>` rather than `BigUInt::<u64, 16>` or
/// `BigUInt::<u8, 128>` or any other ones.
/// 
/// And, in release mode, if you consider performance for multiplication and
/// division, you are highly encouraged to use `BigUInt::<u16, 16>` for
/// multiplication and `BigUInt::<u64, 4>` for division rather than
/// `BigUInt::<u128, 16>` or `BigUInt::<u32, 128>` or any other ones.
/// In debug mode, `BigUInt::<u128, 8>` showed the best performance.
/// Later, you will see the performance test code, and you may want to run the
/// performance test code on your own computer to compare your test results
/// by yourself.
/// 
/// ## Modules and Constructors for Convenience
/// If you use (import) `std::str::FromStr`, you can create BigUInt integer
/// intuitively and conveniently. Of course, you have to use (import) the
/// necessary stuffs in the module `cryptocol::number`. So, for your
/// convenience, use (import) `cryptocol::number::*`. It is no harm for you.
/// 
/// Look into the following examples so that you may get some more ideas
/// about how to use BigUInt.
/// 
/// ## Example 3
/// ```
/// use std::str::FromStr;
/// use cryptocol::number::*;
/// 
/// type U1024 = BigUInt::<u128, 8>;
/// 
/// let a = U1024::from([1_u128; 8]);
/// let b = U1024::from_str_radix("00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001", 2).unwrap();
/// let c = U1024::from_str("1234567891234567879123456789111111111222222222333333333444444444555555555666666666777777777888888888999999999000000000").unwrap();
/// 
/// println!("a = {:?}\nOverflow: {}\nUnderflow: {}\nInfiniity: {}\nDivided_by_Zero: {}", a.get_number(), a.is_overflow(), a.is_underflow(), a.is_inifinity(), a.is_divided_by_zero());
/// assert_eq!(*a.get_number(), [1, 1, 1, 1, 1, 1, 1, 1]);
/// assert_eq!(a.is_overflow(), false);
/// assert_eq!(a.is_underflow(), false);
/// assert_eq!(a.is_inifinity(), false);
/// assert_eq!(a.is_divided_by_zero(), false);
/// 
/// println!("a = {}", a.to_string_with_radix(16).unwrap());
/// assert_eq!(a.to_string_with_radix(16).unwrap(), "100000000000000000000000000000001000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001");
/// 
/// println!("b = {:?}\nOverflow: {}\nUnderflow: {}\nInfiniity: {}\nDivided_by_Zero: {}", b.get_number(), b.is_overflow(), b.is_underflow(), b.is_inifinity(), b.is_divided_by_zero());
/// assert_eq!(*b.get_number(), [1, 1, 1, 1, 1, 1, 1, 1]);
/// assert_eq!(b.is_overflow(), false);
/// assert_eq!(b.is_underflow(), false);
/// assert_eq!(b.is_inifinity(), false);
/// assert_eq!(b.is_divided_by_zero(), false);
/// println!("b = {}", b.to_string());
/// println!("b = {}", b.to_string_with_radix(16).unwrap());
/// assert_eq!(b.to_string_with_radix(16).unwrap(), "100000000000000000000000000000001000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001");
/// 
/// println!("c = {}", c);
/// assert_eq!(c.to_string(), "1234567891234567879123456789111111111222222222333333333444444444555555555666666666777777777888888888999999999000000000");
/// 
/// let mut d = b.clone() + c.clone();
/// println!("b + c = {}", d);
/// assert_eq!(d.to_string(), "528294531135665246352339784916516606520399844128422231063109688515136405111986307932151574694014881104306146237268412201528404470859010781743924190173408080836782200210173329257227380279796317564269871499347888467433181081526414901390421871123571433486060157564694902273");
/// 
/// d = b.clone() - c.clone();
/// println!("b - c = {}", d);
/// assert_eq!(d.to_string(), "528294531135665246352339784916516606520399844128422231063109688515136405111986307932151574694014881104306146237268412201528404470859010781743924190173405611700999731074415082343649158057573873119825204832680999578544069970415081568056866315567793655708060157566694902273");
/// 
/// d = c.clone() - b.clone();
/// println!("c - b = {}", d);
/// assert_eq!(d.to_string(), "179769313486231590772930519078902473361269403363094992027077741372816159198980563288580055091344426332604977474759407049726638194120401741388541284402205712176239488954006474494558295411072688507752083221010590686494501524284889008354087905708146237584806440714171216671890379622911922649127296172057529234943");
/// 
/// d = c.clone() * b.clone();
/// println!("c * b = {}", d);
/// assert_eq!(d.to_string(), "59830717854030867758075123183163555064720825939616846267926369121354707541167863856429897315021801292311343603281484761713479005341939688693125073345149826515313989515871501605159397439048630578377892313876159164289859563003628270426845234033215692532247483706885131175507859004610238546564083383732338767360");
/// 
/// d = b.clone() / c.clone();
/// println!("b / c = {}", d);
/// assert_eq!(d.to_string(), "427918573686029304066254243786715892164567464161173266402914429285403265969001177679575353202952599315891695262671719654199608368852942773933951103642477");
/// 
/// d = b.clone() % c.clone();
/// println!("b % c = {}", d);
/// assert_eq!(d.to_string(), "974831854472745921484474959642423157588012401465652792186214606232572248263942179693215574222740495163800042694902273");
/// 
/// d = b.clone() + 5_u128;
/// println!("b + 5 = {}", d);
/// assert_eq!(d.to_string(), "528294531135665246352339784916516606520399844128422231063109688515136405111986307932151574694014881104306146237268412201528404470859010781743924190173406846268890965642294205800438269168685095342047538166014444022988625525970748234723644093345682544597060157565694902278");
/// 
/// d = b.clone() - 1_u128;
/// println!("b - 1 = {}", d);
/// assert_eq!(d.to_string(), "528294531135665246352339784916516606520399844128422231063109688515136405111986307932151574694014881104306146237268412201528404470859010781743924190173406846268890965642294205800438269168685095342047538166014444022988625525970748234723644093345682544597060157565694902272");
/// 
/// d = b.clone() * 42_u128;
/// println!("b * 42 = {}", d);
/// assert_eq!(d.to_string(), "22188370307697940346798270966493697473856793453393733704650606917635729014703424933150366137148625006380858141965273312464192987776078452833244815987283087543293420556976356643618407305084774004365996602972606648965522272090771425858393051920518666873076526617759185895466");
/// 
/// d = b.clone() / 5_u128;
/// println!("b / 5 = {}", d);
/// assert_eq!(d.to_string(), "105658906227133049270467956983303321304079968825684446212621937703027281022397261586430314938802976220861229247453682440305680894171802156348784838034681369253778193128458841160087653833737019068409507633202888804597725105194149646944728818669136508919412031513138980454");
/// 
/// let e = b.clone() % 5_u128;
/// println!("b % 5 = {}", e);
/// assert_eq!(e, 3);
/// ```
/// 
/// ## Operators and its Disadvantages for Performance
/// If you use operators such as `+`, `-`, `*`, `/`, `%`, `+=`, `-=`, `*=`,
/// `/=`, `%=`, `&`, `|`, `^`, `&=`, `|=`, `^=`, and `!`, they don't use
/// `&self` and `&rhs` but `self` and `rhs` so that they may swallow (move)
/// `self` and `rhs`. This means that you cannot use `self` and `rhs` again.
/// In order to use `self` and `rhs` again, their `clones` were used by clone()
/// methods in Example 3 shown above. It is a bit disadvantageous. So, you are
/// highly encouraged to use methods instead of operators. See the following
/// example. Example 4 is a better version of Example 3 in the viewpoint of
/// performance though Example 4 looks less easy to read or uglier than
/// Example 3.
/// 
/// ## Predefined Datatypes for Convenience
/// You can use predefiend datatypes such as `U256`, `U512`, `U1024`, etc.,
/// or `UU32`, `UU64`, `UU128`, etc. All you have to do is to use (import)
/// `cryptocol::define_utypes_with`, and use a `define_utypes_with!()` macro.
/// The macro `define_utypes_with!()` requires base unsigned integer type such
/// as `u128`, `u64`, `u32`, `u16`, and `u8`. `usize` is not supported as base
/// type because `usize` and `isize` have different size according to
/// CPU and operating system. So, Example 3 can be rewritten as Example 4.
/// 
/// ## Example 4
/// ```
/// use std::str::FromStr;
/// use cryptocol::define_utypes_with;
/// 
/// define_utypes_with!(u128);
/// 
/// let a = U1024::from([1; 8]);
/// let b = U1024::from_str_radix("00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001", 2).unwrap();
/// let c = UU128::from_str("1234567891234567879123456789111111111222222222333333333444444444555555555666666666777777777888888888999999999000000000").unwrap();
/// 
/// println!("a = {:?}\nOverflow: {}\nUnderflow: {}\nInfiniity: {}\nDivided_by_Zero: {}", a.get_number(), a.is_overflow(), a.is_underflow(), a.is_inifinity(), a.is_divided_by_zero());
/// assert_eq!(*a.get_number(), [1, 1, 1, 1, 1, 1, 1, 1]);
/// assert_eq!(a.is_overflow(), false);
/// assert_eq!(a.is_underflow(), false);
/// assert_eq!(a.is_inifinity(), false);
/// assert_eq!(a.is_divided_by_zero(), false);
/// 
/// println!("a = {}", a.to_string_with_radix(16).unwrap());
/// assert_eq!(a.to_string_with_radix(16).unwrap(), "100000000000000000000000000000001000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001");
/// 
/// println!("b = {:?}\nOverflow: {}\nUnderflow: {}\nInfiniity: {}\nDivided_by_Zero: {}", b.get_number(), b.is_overflow(), b.is_underflow(), b.is_inifinity(), b.is_divided_by_zero());
/// assert_eq!(*b.get_number(), [1, 1, 1, 1, 1, 1, 1, 1]);
/// assert_eq!(b.is_overflow(), false);
/// assert_eq!(b.is_underflow(), false);
/// assert_eq!(b.is_inifinity(), false);
/// assert_eq!(b.is_divided_by_zero(), false);
/// 
/// println!("b = {}", b.to_string_with_radix(16).unwrap());
/// assert_eq!(b.to_string_with_radix(16).unwrap(), "100000000000000000000000000000001000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001");
/// 
/// println!("c = {}", c);
/// assert_eq!(c.to_string(), "1234567891234567879123456789111111111222222222333333333444444444555555555666666666777777777888888888999999999000000000");
/// 
/// let mut d = c.wrapping_add(&b);
/// println!("b + c = {}", d);
/// assert_eq!(d.to_string(), "528294531135665246352339784916516606520399844128422231063109688515136405111986307932151574694014881104306146237268412201528404470859010781743924190173408080836782200210173329257227380279796317564269871499347888467433181081526414901390421871123571433486060157564694902273");
/// 
/// d = b.wrapping_sub(&c);
/// println!("b - c = {}", d);
/// assert_eq!(d.to_string(), "528294531135665246352339784916516606520399844128422231063109688515136405111986307932151574694014881104306146237268412201528404470859010781743924190173405611700999731074415082343649158057573873119825204832680999578544069970415081568056866315567793655708060157566694902273");
/// 
/// d = c.wrapping_sub(&b);
/// println!("c - b = {}", d);
/// assert_eq!(d.to_string(), "179769313486231590772930519078902473361269403363094992027077741372816159198980563288580055091344426332604977474759407049726638194120401741388541284402205712176239488954006474494558295411072688507752083221010590686494501524284889008354087905708146237584806440714171216671890379622911922649127296172057529234943");
/// 
/// d = c.wrapping_mul(&b);
/// println!("c * b = {}", d);
/// assert_eq!(d.to_string(), "59830717854030867758075123183163555064720825939616846267926369121354707541167863856429897315021801292311343603281484761713479005341939688693125073345149826515313989515871501605159397439048630578377892313876159164289859563003628270426845234033215692532247483706885131175507859004610238546564083383732338767360");
/// 
/// d = b.wrapping_div(&c);
/// println!("b / c = {}", d);
/// assert_eq!(d.to_string(), "427918573686029304066254243786715892164567464161173266402914429285403265969001177679575353202952599315891695262671719654199608368852942773933951103642477");
/// 
/// d = b.wrapping_rem(&c);
/// println!("b % c = {}", d);
/// assert_eq!(d.to_string(), "974831854472745921484474959642423157588012401465652792186214606232572248263942179693215574222740495163800042694902273");
/// 
/// d = b.wrapping_add_uint(5_u128);
/// println!("b + 5 = {}", d);
/// assert_eq!(d.to_string(), "528294531135665246352339784916516606520399844128422231063109688515136405111986307932151574694014881104306146237268412201528404470859010781743924190173406846268890965642294205800438269168685095342047538166014444022988625525970748234723644093345682544597060157565694902278");
/// 
/// d = b.wrapping_sub_uint(1_u128);
/// println!("b - 1 = {}", d);
/// assert_eq!(d.to_string(), "528294531135665246352339784916516606520399844128422231063109688515136405111986307932151574694014881104306146237268412201528404470859010781743924190173406846268890965642294205800438269168685095342047538166014444022988625525970748234723644093345682544597060157565694902272");
/// 
/// d = b.wrapping_mul_uint(42_u128);
/// println!("b * 42 = {}", d);
/// assert_eq!(d.to_string(), "22188370307697940346798270966493697473856793453393733704650606917635729014703424933150366137148625006380858141965273312464192987776078452833244815987283087543293420556976356643618407305084774004365996602972606648965522272090771425858393051920518666873076526617759185895466");
/// 
/// d = b.wrapping_div_uint(5_u128);
/// println!("b / 5 = {}", d);
/// assert_eq!(d.to_string(), "105658906227133049270467956983303321304079968825684446212621937703027281022397261586430314938802976220861229247453682440305680894171802156348784838034681369253778193128458841160087653833737019068409507633202888804597725105194149646944728818669136508919412031513138980454");
/// 
/// let e = b.wrapping_rem_uint(5_u128);
/// println!("b % 5 = {}", e);
/// assert_eq!(e, 3);
/// ```
/// 
/// However, if you want to use any datatypes that are not predefined
/// such as `u136`, `U144`, `U192`, `U320`, `U384`, etc. or `UU17`, `UU18`,
/// `UU24`, `UU40`, `UU48`, etc., you can define them for yourself as Example 5.
/// 
/// ## Example 5
/// ```
/// use cryptocol::number::*;
/// type U136 = BigUInt::<u8, 17>;
/// type U144 = BigUInt::<u16, 9>;
/// type U192 = BigUInt::<u32, 6>;
/// type U320 = BigUInt::<u64, 5>;
/// type U348 = BigUInt::<u128, 3>;
/// type UU17 = BigUInt::<u8, 17>;
/// type UU18 = BigUInt::<u16, 9>;
/// type UU24 = BigUInt::<u32, 6>;
/// type UU40 = BigUInt::<u64, 5>;
/// type UU48 = BigUInt::<u128, 3>;
/// ```
/// 
/// ## Performance Test
/// Which base type will achieve best performance? According to the result of
/// performance test on author's 64-bit computer, `u128` as base type showed
/// the best performannce for addition and subtraction in both release mode
/// and debug mode all the time, while `u16` and `u64` as base type showed the
/// best performannce for multiplication and division respectively in release
/// mode most of the time. Rarely, however, `u32` as base type showed the best
/// performannce for multiplication and/or division in release mode.
/// In debug mode, `u128` as base type showed the best performannce for
/// multiplication and division most of the time. More rarely than in release
/// mode, however, `u32` as base type showed the best performannce for
/// multiplication and division. The result is obtained from 64-bit machine.
/// If the test was done on 32-bit machine, the result might be different.
/// 
/// | Operation      | Best base type in Release mode         | Best base type in Debug mode           |
/// |----------------|----------------------------------------|----------------------------------------|
/// | Addition       | `u128` most of the time / `u64` rarely | `u128` all the time                    |
/// | Subtraction    | `u128` all the time                    | `u128` all the time                    |
/// | Multiplication | `u16`  all the time                    | `u128` most of the time / `u64` rarely |
/// | Division       | `u64` all the time                     | `u128` most of the time / `u64` rarely |
/// 
/// The following is the code used for the Performance Test.
/// performance!() is a macro. And, Rust Playground may not run this code
/// correctly because of using a user-defined macro. You are recommended to
/// build and run the performance test code on your own computer by yourself
/// rather than on Rust Playground.
/// 
/// ## Performance Test Code
/// ```
/// macro_rules! performance
/// {
///     ($t:ty, $b:expr, $ti:expr, $f:expr) => {
///         let mut sum = <$t>::zero();
///         let a = <$t>::from_str_radix("00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001", 2).unwrap();
///         let now: SystemTime;
///         match $f
///         {
///             0 => {
///                 now = SystemTime::now();
///                 for _ in 0..100_0000
///                     { sum.wrapping_add_assign(&a); }
///             },
///             1 => {
///                 now = SystemTime::now();
///                 for _ in 0..100_0000
///                     { sum.wrapping_sub_assign(&a); }
///             },
///             2 => {
///                 now = SystemTime::now();
///                 for _ in 0..100_0000
///                     { sum.wrapping_mul_assign(&a); }
///             },
///             _ => {
///                 now = SystemTime::now();
///                 for _ in 0..100_0000
///                     { sum.wrapping_div_assign(&a); }
///             },
///         }
///         match now.elapsed()
///         {
///             Ok(elapsed) => {
///                 $ti = elapsed.as_micros();
///                 println!("{}-based: {} usec", $b, $ti);
///             },
///             Err(e) => { println!("{}", e); },
///         }
///     }
/// }
/// 
/// fn main()
/// {
///     use std::time::SystemTime;
///     use cryptocol::number::*;
/// 
///     let mut ti = [0_u128; 5];   // How many microseconds
///     let dt = ["u128", "u64", "u32", "u16", "u8"];
///     let op = ["addition", "subtraction", "multplication", "division" ];
/// 
///     for operator in 0..4
///     {
///         println!("=== {} ===", op[operator]);
///         performance!(U1024_with_u128, dt[0], ti[0], operator);
///         performance!(U1024_with_u64, dt[1], ti[1], operator);
///         performance!(U1024_with_u32, dt[2], ti[2], operator);
///         performance!(U1024_with_u16, dt[3], ti[3], operator);
///         performance!(U1024_with_u8, dt[4], ti[4], operator);
///     
///         let mut fastest = 0;
///         for i in 1..5
///         {
///             if ti[fastest] > ti[i]
///                 { fastest = i; }
///         }
///         println!("The fastest one is {}.\n", dt[fastest]);
///         
///         if cfg!(debug)
///         {
///             assert_eq!(fastest, 0); // It means u128 shows the best performance most of the time.
///         }
///         else
///         {
///             if operator < 2
///                 { assert_eq!(fastest, 0); } // It means u128 shows the best performance.
///             else
///                 { assert_eq!(fastest, 1); } // It means u64 shows the best performance most of the time.
///         }
///     }
/// }
/// ```
/// `U1024_with_u128`, `U1024_with_u64`, `U1024_with_u32`, `U1024_with_u16`,
/// and `U1024_with_u128` are all predefined datatypes too.
/// 
/// So, if ypu calculate addition and subtraction mainly, you'd better use
/// `u128`-based datatype such as `BigUInt<u128, N>`. Or use predefined
/// datatype as follows.
/// 
/// ## Example 6
/// ```
/// use std::str::FromStr;
/// use cryptocol::define_utypes_with;
/// define_utypes_with!(u128);
/// let a = U1024::new();
/// ```
/// If you calculate multiplication and division mainly, you'd better use
/// `u16`-based datatype for multiplication and `u64`-based datatype for
/// division such as `BigUInt<u16, N>` and `BigUInt<u64, N>`, respectively.
/// Or use predefined datatype as follows.
/// 
/// ## Example 7
/// ```
/// use std::str::FromStr;
/// use cryptocol::define_utypes_with;
/// define_utypes_with!(u16);
/// let a = U1024::new();
/// ```
/// 
/// # Big-endian issue
/// It is just experimental for Big Endian CPUs. So, you are not encouraged
/// to use it for Big Endian CPUs for serious purpose. Only use this crate
/// for Big-endian CPUs with your own full responsibility.
#[derive(Debug, Clone)]
pub struct BigUInt<T, const N: usize>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // method_widening_mul_assign_uint: fn(&mut Self, T) -> Self,
    // method_wrapping_mul_assign_uint: fn(&mut Self, T),
    // method_widening_mul_assign: fn(&mut Self, &Self) -> Self,
    // method_wrapping_mul_assign: fn(&mut Self, &Self),
    number: [T; N],
    flag: u8,
}

impl<T, const N: usize> BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd,
    Self: Sized + Clone + Display + Debug + ToString
        + Add<Output = Self> + AddAssign
        + Sub<Output = Self> + SubAssign
        + Mul<Output = Self> + MulAssign
        + Div<Output = Self> + DivAssign
        + Rem<Output = Self> + RemAssign
        + Shl<i32, Output = Self> + ShlAssign<i32>
        + Shr<i32, Output = Self> + ShrAssign<i32>
        + BitAnd<Self, Output = Self> + BitAndAssign
        + BitOr<Self, Output = Self> + BitOrAssign
        + BitXor<Self, Output = Self> + BitXorAssign
        + Not<Output = Self>
        + From<T> + FromStr + From<[T; N]> + From<u32>
        // + 'a
        // + Add<&'a Self, Output = Self> + AddAssign<&'a Self>
        // + Sub<&'a Self, Output = Self> + SubAssign<&'a Self>
        // + Mul<&'a Self, Output = Self> + MulAssign<&'a Self>
        // + Div<&'a Self, Output = Self> + DivAssign<&'a Self>
        // + Rem<&'a Self, Output = Self> + RemAssign<&'a Self>
        // + BitAnd<&'a Self, Output = Self> + BitAndAssign<&'a Self>
        // + BitOr<&'a Self, Output = Self> + BitOrAssign<&'a Self>
        // + BitXor<&'a Self, Output = Self> + BitXorAssign<&'a Self>

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

    #[allow(non_upper_case_globals)]
    const method_widening_mul_assign_uint: fn(&mut Self, T) -> Self
            =   if N > 16
                    { Self::widening_mul_assign_uint_1 }
                else
                    { Self::widening_mul_assign_uint_2 };

    #[allow(non_upper_case_globals)]
    const method_wrapping_mul_assign_uint: fn(&mut Self, T)
            =   if N > 16
                    { Self::wrapping_mul_assign_uint_1 }
                else
                    { Self::wrapping_mul_assign_uint_2 };

    #[allow(non_upper_case_globals)]
    const method_widening_mul_assign: fn(&mut Self, &Self) -> Self
            =   if N > 16
                    { Self::widening_mul_assign_1 }
                else
                    { Self::widening_mul_assign_2 };

    #[allow(non_upper_case_globals)]
    const method_wrapping_mul_assign: fn(&mut Self, &Self)
            =   if N > 16
                    { Self::wrapping_mul_assign_1 }
                else
                    { Self::wrapping_mul_assign_2 };

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
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let obj = U256::new();
    /// println!("obj = {}", obj);
    /// assert_eq!(obj.to_string(), "0");
    /// ```
    pub fn new() -> Self
    {
        Self
        {
            number: [T::zero(); N],
            flag: 0,
        }
    }

    // pub fn zero() -> Self
    /// Constructs a new `BigUInt<T, N>` which has the value of `0`.
    /// 
    /// # Output
    /// A new object of `BigUInt<T, N>` that represents `0`.
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
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let zero = U256::zero();
    /// println!("zero = {}", zero);
    /// assert_eq!(zero.to_string(), "0");
    /// ```
    #[inline]
    pub fn zero() -> Self
    {
        Self::new()   // unsafe { zeroed::<Self>() }
    }

    // pub fn one() -> Self
    /// Constructs a new `BigUInt<T, N>` which has the value of `1`.
    /// 
    /// # Output
    /// A new object of `BigUInt<T, N>` that represents `1`.
    /// 
    /// # Benefit
    /// Your source code will be better readable if you use
    /// `BigUInt<T, N>::one()` instead of `BigUInt<T, N>::new()` and then
    /// `set_uint(1)` especially when you create the big number `1`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let one = U256::one();
    /// println!("one = {}", one);
    /// assert_eq!(one.to_string(), "1");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn one() -> Self
    {
        let mut me = Self::zero();
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
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let maximum = U256::max();
    /// assert_eq!(maximum.to_string(), "115792089237316195423570985008687907853269984665640564039457584007913129639935");
    /// assert_eq!(maximum.wrapping_add_uint(1_u16), U256::zero());
    /// ```
    pub fn max() -> Self
    {
        let mut me = Self::new();
        me.set_max();
        me
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
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let half = U256::submax(128_usize);
    /// println!("half maximum = \t{}", half);
    /// println!("half maximum = \t{}", half.to_string_with_radix_and_stride(16, 4).unwrap());
    /// assert_eq!(half.to_string(), "340282366920938463463374607431768211455");
    /// assert_eq!(half.to_string_with_radix_and_stride(16, 4).unwrap(), "FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF");
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
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let half = U256::halfmax();
    /// println!("half maximum = \t{}", half);
    /// println!("half maximum = \t{}", half.to_string_with_radix_and_stride(16, 4).unwrap());
    /// assert_eq!(half.to_string(), "340282366920938463463374607431768211455");
    /// assert_eq!(half.to_string_with_radix_and_stride(16, 4).unwrap(), "FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF");
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
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with_u16;
    /// define_utypes_with_u16!();
    /// 
    /// let a_from_u8 = U512::from_uint(123_u8);
    /// let b_from_u16 = U512::from_uint(12345_u16);
    /// let c_from_u32 = U512::from_uint(1234567890_u32);
    /// let d_from_u64 = U512::from_uint(12345678901234567890_u64);
    /// let e_from_u128 = U512::from_uint(123456789012345678901234567890123456789_u128);
    /// let f_from_usize = U512::from_uint(123_usize);
    /// 
    /// println!("a_from_u8 = {}", a_from_u8);
    /// println!("b_from_u16 = {}", b_from_u16);
    /// println!("c_from_u32 = {}", c_from_u32);
    /// println!("d_from_u64 = {}", d_from_u64);
    /// println!("e_from_u128 = {}", e_from_u128);
    /// println!("f_from_usize = {}", f_from_usize);
    /// 
    /// assert_eq!(a_from_u8.into_u8(), 123_u8);
    /// assert_eq!(b_from_u16.into_u16(), 12345_u16);
    /// assert_eq!(c_from_u32.into_u32(), 1234567890_u32);
    /// assert_eq!(d_from_u64.into_u64(), 12345678901234567890_u64);
    /// assert_eq!(e_from_u128.into_u128(), 123456789012345678901234567890123456789_u128);
    /// assert_eq!(f_from_usize.into_usize(), 123_usize);
    /// ```
    pub fn from_uint<U>(val: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let size_t = T::size_in_bytes();
        let size_u = U::size_in_bytes();
        let mut me = Self::zero();
        let mut share = SharedValues::<T, U>::from_src(val);
//        unsafe { copy_nonoverlapping(val.as_ptr() as *const u8, me.number.as_mut_ptr() as *mut u8, size_u); }
        if size_t >= size_u
        {
            unsafe { me.set_num_(0, share.des); }
        }
        else    // if size_t < size_u
        {
            let size_t_bits = size_t * 8;
            for i in 0..size_u/size_t
            {
                unsafe { me.set_num_(i, share.des); }
                unsafe { share.src >>= U::usize_as_smalluint(size_t_bits); }
            }
        }
        return me;
    }

    // pub fn from_array(val: [T; N]) -> Self
    /// Constructs a new `BigUInt<T, N>` from an array of type `T` with `N`
    /// elements.
    /// 
    /// # Output
    /// A new object of `BigUInt<T, N>` that represents the same value of array `val`.
    /// 
    /// # Counterpart Method
    /// You can also use the method [from()](struct@BigUInt#impl-From<[T;+N]>-for-BigUInt<T,+N>)
    /// implemented by implementation of trait `From<[T;N]>`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let big_num = U256::from_array([10_u8;32]);
    /// println!("big_num = {}", big_num.to_string_with_radix(16).unwrap());
    /// assert_eq!(big_num.to_string_with_radix(16).unwrap(), "A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A");
    /// ```
    pub fn from_array(val: [T; N]) -> Self
    {
        Self { number: val, flag: 0 }
    }

    // pub fn from_biguint<U, const M: usize>(biguint: &BigUInt<U, M>) -> Self
    /// Constructs a new `BigUInt<T, N>` from another kind of `BigUInt<U, M>`.
    /// 
    /// # Output
    /// A new object of `BigUInt<T, N>` that represents the same value of another
    /// kind of `BigUInt<U, M>`.
    /// 
    /// # Features
    /// It copies not only long-bit integer but also current flags from another
    /// kind of `BigUInt<U, M>`.
    /// 
    /// # Example 1 for the same length
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::*;
    /// 
    /// let a_u512_with_u8 = U512_with_u8::from_str("123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789").unwrap();
    /// let b_u512_with_u8 = U512_with_u8::from_biguint(&a_u512_with_u8);
    /// println!("a_u512_with_u8 = {}", a_u512_with_u8);
    /// println!("b_u512_with_u8 = {}", b_u512_with_u8);
    /// assert_eq!(a_u512_with_u8.to_string(), "123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789");
    /// assert_eq!(b_u512_with_u8.to_string(), "123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789");
    /// ```
    /// 
    /// # Example 2 for the shorter length
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::*;
    /// 
    /// let a_u512_with_u8 = U512_with_u8::from_str("123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789").unwrap();
    /// let b_u256_with_u8 = U256_with_u16::from_biguint(&a_u512_with_u8);
    /// println!("a_u512_with_u8 = {}", a_u512_with_u8);
    /// println!("b_u256_with_u8 = {}", b_u256_with_u8);
    /// assert_eq!(a_u512_with_u8.to_string(), "123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789");
    /// assert_eq!(b_u256_with_u8.to_string(), "98633800081229720571026865697976779988382011787853764870844783447569204535061");
    /// ```
    /// 
    /// # Example 3 for the longer length
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::*;
    /// 
    /// let a_u512_with_u8 = U512_with_u8::from_str("123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789").unwrap();
    /// let b_u1024_with_u8 = U1024_with_u16::from_biguint(&a_u512_with_u8);
    /// println!("a_u512_with_u8 = {}", a_u512_with_u8);
    /// println!("b_u1024_with_u8 = {}", b_u1024_with_u8);
    /// assert_eq!(a_u512_with_u8.to_string(), "123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789");
    /// assert_eq!(b_u1024_with_u8.to_string(), "123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789");
    /// ```
    #[inline]
    pub fn from_biguint<U, const M: usize>(biguint: &BigUInt<U, M>) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        Self::from_array(unsafe {SharedArrays::<T, N, U, M>::from_src(biguint.get_number()).des})
    }

    // pub fn from_be(be: Self) -> Self
    /// Converts a big unsigned integer from big endian to the target’s
    /// endianness.
    /// 
    /// # Features
    /// - On big endian machine, this is a no-op.
    /// - On little endian machine, the bytes are swapped.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let be = U256::from_array([0x1234, 0x5678, 0x90ab, 0xcdef,
    ///                             0x1122, 0x3344, 0x5566, 0x7788,
    ///                             0x9900, 0xaabb, 0xccdd, 0xeeff,
    ///                             0x1f2e, 0x3d4c, 0x5b6a, 0x7089]);
    /// let le = U256::from_be(be.clone());
    /// println!("be = 0x{}", be.to_string_with_radix(16).unwrap());
    /// println!("le = 0x{}", le.to_string_with_radix(16).unwrap());
    /// #[cfg(target_endian = "little")]
    /// {
    ///     assert_eq!(be.to_string_with_radix(16).unwrap(), "70895B6A3D4C1F2EEEFFCCDDAABB99007788556633441122CDEF90AB56781234");
    ///     assert_eq!(le.to_string_with_radix(16).unwrap(), "34127856AB90EFCD22114433665588770099BBAADDCCFFEE2E1F4C3D6A5B8970");        
    /// }
    /// #[cfg(target_endian = "big")]
    /// {
    ///     assert_eq!(be.to_string_with_radix(16).unwrap(), "1234567890ABCDEF11223344556677889900AABBCCDDEEFF1F2E3D4C5B6A7089");
    ///     assert_eq!(le.to_string_with_radix(16).unwrap(), "1234567890ABCDEF11223344556677889900AABBCCDDEEFF1F2E3D4C5B6A7089");        
    /// }
    /// ```
    #[inline]
    pub fn from_be(be: Self) -> Self
    {
        #[cfg(target_endian = "little")]    return be.swap_bytes();
        #[cfg(target_endian = "big")]       return be.clone();
    }

    // pub fn from_be_bytes(be_bytes: [T; N]) -> Self
    /// Create a native endian unsigned integer value from its representation
    /// as a byte array in big endian.
    /// 
    /// # Features
    /// - On big endian machine, this is a no-op.
    /// - On little endian machine, the bytes are swapped.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let be_array = [0x12345678, 0x90abcdef, 0x11223344, 0x55667788,
    ///                 0x9900aabb, 0xccddeeff, 0x1f2e3d4c, 0x5b6a7089];
    /// let le = U256::from_be_bytes(be_array.clone());
    /// print!("be_array = ");
    /// for elem in be_array
    ///     { print!("0x{:8x} ", elem); }
    /// println!();
    /// println!("le = 0x{}", le.to_string_with_radix_and_delimiter(16, 8, " 0x").unwrap());
    /// #[cfg(target_endian = "little")]    assert_eq!(le.to_string_with_radix_and_stride(16, 8).unwrap(), "78563412_EFCDAB90_44332211_88776655_BBAA0099_FFEEDDCC_4C3D2E1F_89706A5B");
    /// #[cfg(target_endian = "big")]       assert_eq!(le.to_string_with_radix(16).unwrap(), "12345678_90ABCDEF_11223344_55667788_9900AABB_CCDDEEFF_1F2E3D4C_5B6A7089");
    /// ```
    pub fn from_be_bytes(be_bytes: [T; N]) -> Self
    {
        #[cfg(target_endian = "little")]
        {
            let mut me = Self::from_array(be_bytes);
            me.swap_bytes_assign();
            me
        } 
        #[cfg(target_endian = "big")]       return Self::from_array(be_bytes);
    }

    // pub fn from_le(le: Self) -> Self
    /// Converts a big unsigned integer from little endian to the target’s
    /// endianness.
    /// 
    /// # Features
    /// - On little endian this is a no-op.
    /// - On big endian the bytes are swapped.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let le1 = U256::from_array([0x1234, 0x5678, 0x90ab, 0xcdef,
    ///                 0x1122, 0x3344, 0x5566, 0x7788,
    ///                 0x9900, 0xaabb, 0xccdd, 0xeeff,
    ///                 0x1f2e, 0x3d4c, 0x5b6a, 0x7089]);
    /// let le2 = U256::from_le(le1.clone());
    /// println!("le1 = 0x{}", le1.to_string_with_radix(16).unwrap());
    /// println!("le2 = 0x{}", le2.to_string_with_radix(16).unwrap());
    /// #[cfg(target_endian = "little")]
    /// {
    ///     assert_eq!(le1.to_string_with_radix(16).unwrap(), "70895B6A3D4C1F2EEEFFCCDDAABB99007788556633441122CDEF90AB56781234");
    ///     assert_eq!(le2.to_string_with_radix(16).unwrap(), "70895B6A3D4C1F2EEEFFCCDDAABB99007788556633441122CDEF90AB56781234");
    /// }
    /// #[cfg(target_endian = "big")]
    /// {
    ///     assert_eq!(le1.to_string_with_radix(16).unwrap(), "1234567890ABCDEF11223344556677889900AABBCCDDEEFF1F2E3D4C5B6A7089");
    ///     assert_eq!(le2.to_string_with_radix(16).unwrap(), "34127856AB90EFCD22114433665588770099BBAADDCCFFEE2E1F4C3D6A5B8970");
    /// }
    /// ```
    #[inline]
    pub fn from_le(le: Self) -> Self
    {
        #[cfg(target_endian = "little")]    return le;
        #[cfg(target_endian = "big")]       return le.swap_bytes();
    }
    
    // pub fn from_le_bytes(le_bytes: [T; N]) -> Self
    /// Create a native endian integer value from its representation
    /// as a byte array in little endian.
    /// 
    /// # Features
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let le_array = [0x12345678, 0x90abcdef, 0x11223344, 0x55667788,
    ///                 0x9900aabb, 0xccddeeff, 0x1f2e3d4c, 0x5b6a7089];
    /// let le = U256::from_le_bytes(le_array.clone());
    /// print!("le_array = ");
    /// for elem in le_array
    ///     { print!("0x{:8x} ", elem); }
    /// println!();
    /// println!("le = 0x{}", le.to_string_with_radix_and_delimiter(16, 8, " 0x").unwrap());
    /// #[cfg(target_endian = "little")]    assert_eq!(le.to_string_with_radix_and_stride(16, 8).unwrap(), "5B6A7089_1F2E3D4C_CCDDEEFF_9900AABB_55667788_11223344_90ABCDEF_12345678");
    /// #[cfg(target_endian = "big")]       assert_eq!(le.to_string_with_radix(16).unwrap(), "12345678_90ABCDEF_11223344_55667788_9900AABB_CCDDEEFF_1F2E3D4C_5B6A7089");
    /// ```
    #[inline]
    pub fn from_le_bytes(le_bytes: [T; N]) -> Self
    {
        #[cfg(target_endian = "little")]    return Self::from_array(le_bytes);
        #[cfg(target_endian = "big")]
        {
            let mut me = Self::from_array(le_bytes);
            me.swap_bytes_assign();
            me
        } 
    }

    //  pub fn from_string(txt: &str) -> Result<Self, NumberErr>
    /// Constructs a new `BigUInt<T, N>` from a string of decimal number.
    /// 
    /// # Output
    /// The constructed object will be wrapped in `Ok(BigUInt<T, N>)` if it is
    /// successfully created. Otherwise, this method returns one of
    /// `Err(NumberErr::NotAlphaNumeric)`, and `Err(NumberErr::NotFitToRadix)`
    /// according to its failure reason.
    /// 
    /// # Delimiter _
    /// In the number expression in a string, you can separate the digits with
    /// '_' in order to make it more readable. So, "1_0000" and "10_000" are all
    /// the same as"10000".
    /// 
    /// # Errors
    /// | priority | argument | value                                           | Caused Error                 |
    /// |----------|----------|-------------------------------------------------|------------------------------|
    /// | 1st      | `txt`    | contains any non-alphanumeric letter except '_' | `NumberErr::NotAlphaNumeric` |
    /// | 2nd      | `txt`    | contains any alphabet                           | `NumberErr::NotFitToRadix`   |
    /// | 3rd      | `txt`    | expresses bigger number than maximum value      | `NumberErr::TooBigNumber`    |
    /// 
    /// When multiple errors were caused, only the error with higher priority is
    /// issued. `1st` is higher than `2nd`, and so on.
    /// 
    /// # Example 1 for correct case
    /// ```
    /// use std::fmt::Write as _;
    /// use cryptocol::number::NumberErr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_correct = U256::from_string("1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890");
    /// match a_correct
    /// {
    ///     Ok(n) => {
    ///             println!("a_correct = {}", n);
    ///             assert_eq!(n.to_string(), "1234567890123456789012345678901234567890123456789012345678901234567890");
    ///         },
    ///     Err(e) => {
    ///             match e
    ///             {
    ///                 NumberErr::NotAlphaNumeric =>  { println!("Failed: Not alphanumeric!") },
    ///                 NumberErr::NotFitToRadix =>    { println!("Failed: Not decimal number!") },
    ///                 NumberErr::TooBigNumber =>     { println!("Failed: Too big number!") },
    ///                 _ => {},
    ///             }
    ///         },
    /// }
    /// ```
    /// 
    /// # Example 2 for NumberErr::NotAlphaNumeric case
    /// ```
    /// use std::fmt::Write as _;
    /// use cryptocol::number::NumberErr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let b_contains_non_alphanumeric = U256::from_string("12345+67890");
    /// match b_contains_non_alphanumeric
    /// {
    ///     Ok(n) =>  { println!("a_correct = {}", n); },
    ///     Err(e) => {
    ///         match e
    ///         {
    ///             NumberErr::NotAlphaNumeric => {
    ///                     println!("Failed: Not alphanumeric!");
    ///                     let mut txt = String::new();
    ///                     write!(&mut txt, "{:?}", b_contains_non_alphanumeric).unwrap();
    ///                     assert_eq!(txt, "Err(NotAlphaNumeric)");
    ///                 },
    ///             NumberErr::NotFitToRadix =>    { println!("Failed: Not decimal number!"); },
    ///             NumberErr::TooBigNumber =>     { println!("Failed: Too big number!"); },
    ///             _ => {},
    ///         }
    ///     },
    /// }
    /// ```
    /// 
    /// # Example 3 for NumberErr::NotFitToRadix case
    /// ```
    /// use std::fmt::Write as _;
    /// use cryptocol::number::NumberErr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let c_constains_not_fit_to_radix = U256::from_string("1234567890a");
    /// match c_constains_not_fit_to_radix
    /// {
    ///     Ok(n) =>  { println!("c_constains_not_fit_to_radix = {}", n); },
    ///     Err(e) => {
    ///         match e
    ///         {
    ///             NumberErr::NotAlphaNumeric =>  { println!("Failed: Not alphanumeric!"); },
    ///             NumberErr::NotFitToRadix => {
    ///                     println!("Failed: Not decimal number!");
    ///                     let mut txt = String::new();
    ///                     write!(&mut txt, "{:?}", c_constains_not_fit_to_radix).unwrap();
    ///                     assert_eq!(txt, "Err(NotFitToRadix)");
    ///                 },
    ///                 NumberErr::TooBigNumber =>     { println!("Failed: Too big number!"); },
    ///             _ => {},
    ///         }
    ///     },
    /// }
    /// ```
    /// 
    /// # Example 4 for NumberErr::TooBigNumber case
    /// ```
    /// use std::fmt::Write as _;
    /// use cryptocol::number::NumberErr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let d_constains_too_big_number = U256::from_string("1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890");
    /// match d_constains_too_big_number
    /// {
    ///     Ok(n) =>  { println!("d_constains_too_big_number = {}", n); },
    ///     Err(e) => {
    ///         match e
    ///         {
    ///             NumberErr::NotAlphaNumeric =>  { println!("Failed: Not alphanumeric!"); },
    ///             NumberErr::NotFitToRadix =>    { println!("Failed: Not decimal number!"); },
    ///             NumberErr::TooBigNumber => {
    ///                     println!("Failed: Too big number!");
    ///                     let mut txt = String::new();
    ///                     write!(&mut txt, "{:?}", d_constains_too_big_number).unwrap();
    ///                     assert_eq!(txt, "Err(TooBigNumber)");
    ///                 },
    ///             _ => {},
    ///         }
    ///     },
    /// }
    /// ```
    /// 
    /// # Example 5 for NumberErr::NotAlphaNumeric and NumberErr::NotFitToRadix case
    /// ```
    /// use std::fmt::Write as _;
    /// use cryptocol::number::NumberErr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let e_contains_non_alphanumeric_not_fit_to_radix = U256::from_string("F12345+67890");
    /// match e_contains_non_alphanumeric_not_fit_to_radix
    /// {
    ///     Ok(n) =>  { println!("e_contains_non_alphanumeric_not_fit_to_radix = {}", n); },
    ///     Err(e) => {
    ///         match e
    ///         {
    ///             NumberErr::NotAlphaNumeric => {
    ///                     println!("Failed: Not alphanumeric!");
    ///                     let mut txt = String::new();
    ///                     write!(&mut txt, "{:?}", e_contains_non_alphanumeric_not_fit_to_radix).unwrap();
    ///                     assert_eq!(txt, "Err(NotAlphaNumeric)");
    ///                 },
    ///             NumberErr::NotFitToRadix =>    { println!("Failed: Not decimal number!"); },
    ///             NumberErr::TooBigNumber =>     { println!("Failed: Too big number!"); },
    ///             _ => {},
    ///         }
    ///     },
    /// }
    /// ```
    #[inline]
    pub fn from_string(txt: &str) -> Result<Self, NumberErr>
    {
        Self::from_str_radix(txt, 10)
    }

    //  pub fn from_str_radix(txt: &str, radix: u32) -> Result<Self, NumberErr>
    /// Constructs a new `BigUInt<T, N>` from a string with `radix`.
    /// 
    /// # Output
    /// The constructed object will be wrapped in `Ok(BigUInt<T, N>)` if it is
    /// successfully created. Otherwise, this method returns one of
    /// `Err(NumberErr::OutOfValidRadixRange)`, `Err(NumberErr::NotAlphaNumeric)`,
    /// and `Err(NumberErr::NotFitToRadix)` according to its failure reason.
    /// 
    /// # Errors
    /// - If the argument `txt` of this method includes any letters other than
    /// alphanumeric letter(s),
    /// it will return`Err(NumberErr::NotAlphaNumeric)`.
    /// - If the argument `radix` of this method is out of the valid range from
    /// `2` up to `62` inclusively,
    /// it will return `Err(NumberErr::OutOfValidRadixRange)`.
    /// - If the argument `txt` of this method includes any letter(s) out of
    /// the valid letter range even if they are alphanumeric, it will return
    /// `Err(NumberErr::NotFitToRadix)`. For example, in the case of hexadecimal
    /// number system which means that the argument radix is `16`, if the
    /// argument `txt` includes 'g',
    /// it will return `Err(NumberErr::NotFitToRadix)`.
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
    /// # Errors
    /// | priority | argument | value                                           | Caused Error                      |
    /// |----------|----------|-------------------------------------------------|-----------------------------------|
    /// | 1st      | `radix`  | less than `2` or greater than `62`              | `NumberErr::OutOfValidRadixRange` |
    /// | 2nd      | `txt`    | contains any non-alphanumeric letter except '_' | `NumberErr::NotAlphaNumeric`      |
    /// | 3rd      | `txt`    | contains any letter or number out of `radix`    | `NumberErr::NotFitToRadix`        |
    /// | 4th      | `txt`    | expresses bigger number than maximum value      | `NumberErr::TooBigNumber`         |
    /// 
    /// When multiple errors were caused, only the error with higher priority is
    /// issued. `1st` is higher than `2nd`, and so on.
    /// 
    /// # Example 1 for correct case
    /// ```
    /// use std::fmt::Write as _;
    /// use cryptocol::number::NumberErr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_correct = U512::from_str_radix("1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0", 16);
    /// match a_correct
    /// {
    ///     Ok(n) => {
    ///             println!("a_correct = {}", n);
    ///             assert_eq!(n.to_string_with_radix_and_stride(16, 4).unwrap(), "1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0");
    ///         },
    ///     Err(e) => {
    ///             match e
    ///             {
    ///                 NumberErr::OutOfValidRadixRange =>  { println!("Failed: Out of Valid Radix Range!") },
    ///                 NumberErr::NotAlphaNumeric =>       { println!("Failed: Not alphanumeric!") },
    ///                 NumberErr::NotFitToRadix =>         { println!("Failed: Not decimal number!") },
    ///                 NumberErr::TooBigNumber =>          { println!("Failed: Too big number!") },
    ///             }
    ///         },
    /// }
    /// ```
    /// 
    /// # Example 2 for NumberErr::OutOfValidRadixRange case
    /// ```
    /// use std::fmt::Write as _;
    /// use cryptocol::number::NumberErr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let b_contains_out_of_valid_radix_range = U512::from_str_radix("1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0", 63);
    /// match b_contains_out_of_valid_radix_range
    /// {
    ///     Ok(n) =>  { println!("a_correct = {}", n); },
    ///     Err(e) => {
    ///         match e
    ///         {
    ///             NumberErr::OutOfValidRadixRange => {
    ///                     println!("Failed: Out of Valid Radix Range!");
    ///                     let mut txt = String::new();
    ///                     write!(&mut txt, "{:?}", b_contains_out_of_valid_radix_range).unwrap();
    ///                     assert_eq!(txt, "Err(OutOfValidRadixRange)");
    ///                 },
    ///             NumberErr::NotAlphaNumeric =>   { println!("Failed: Not alphanumeric!"); },
    ///             NumberErr::NotFitToRadix =>     { println!("Failed: Not decimal number!"); },
    ///             NumberErr::TooBigNumber =>      { println!("Failed: Too big number!"); },
    ///         }
    ///     },
    /// }
    /// ```
    /// 
    /// # Example 3 for NumberErr::NotAlphaNumeric case
    /// ```
    /// use std::fmt::Write as _;
    /// use cryptocol::number::NumberErr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let c_contains_non_alphanumeric = U512::from_str_radix("1234-5678-9ABC-DEF0-1234-5678-9ABC-DEF0-1234-5678-9ABC-DEF0-1234-5678-9ABC-DEF0-1234-5678-9ABC-DEF0-1234-5678-9ABC-DEF0-1234-5678-9ABC-DEF0-1234-5678-9ABC-DEF0", 16);
    /// match c_contains_non_alphanumeric
    /// {
    ///     Ok(n) =>  { println!("a_correct = {}", n); },
    ///     Err(e) => {
    ///         match e
    ///         {
    ///             NumberErr::OutOfValidRadixRange => { println!("Failed: Out of Valid Radix Range!") },
    ///             NumberErr::NotAlphaNumeric => {
    ///                     println!("Failed: Not alphanumeric!");
    ///                     let mut txt = String::new();
    ///                     write!(&mut txt, "{:?}", c_contains_non_alphanumeric).unwrap();
    ///                     assert_eq!(txt, "Err(NotAlphaNumeric)");
    ///                 },
    ///             NumberErr::NotFitToRadix => { println!("Failed: Not decimal number!"); },
    ///             NumberErr::TooBigNumber =>  { println!("Failed: Too big number!"); },
    ///         }
    ///     },
    /// }
    /// ```
    /// 
    /// # Example 4 for NumberErr::NotFitToRadix case
    /// ```
    /// use std::fmt::Write as _;
    /// use cryptocol::number::NumberErr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let d_constains_not_fit_to_radix = U512::from_str_radix("1234_5678_9ABC_DEFG_1234_5678_9ABC_DEFG_1234_5678_9ABC_DEFG_1234_5678_9ABC_DEFG_1234_5678_9ABC_DEFG_1234_5678_9ABC_DEFG_1234_5678_9ABC_DEFG_1234_5678_9ABC_DEFG", 16);
    /// match d_constains_not_fit_to_radix
    /// {
    ///     Ok(n) =>  { println!("d_constains_not_fit_to_radix = {}", n); },
    ///     Err(e) => {
    ///         match e
    ///         {
    ///             NumberErr::OutOfValidRadixRange =>  { println!("Failed: Out of Valid Radix Range!") },
    ///             NumberErr::NotAlphaNumeric =>       { println!("Failed: Not alphanumeric!"); },
    ///             NumberErr::NotFitToRadix => {
    ///                     println!("Failed: Not hexadecimal number!");
    ///                     let mut txt = String::new();
    ///                     write!(&mut txt, "{:?}", d_constains_not_fit_to_radix).unwrap();
    ///                     assert_eq!(txt, "Err(NotFitToRadix)");
    ///                 },
    ///                 NumberErr::TooBigNumber =>     { println!("Failed: Too big number!"); },
    ///         }
    ///     },
    /// }
    /// ```
    /// 
    /// # Example 5 for NumberErr::TooBigNumber case
    /// ```
    /// use std::fmt::Write as _;
    /// use cryptocol::number::NumberErr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let e_constains_too_big_number = U512::from_str_radix("1_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0", 16);
    /// match e_constains_too_big_number
    /// {
    ///     Ok(n) =>  { println!("c_constains_too_big_number = {}", n); },
    ///     Err(e) => {
    ///         match e
    ///         {
    ///             NumberErr::OutOfValidRadixRange =>  { println!("Failed: Out of Valid Radix Range!") },
    ///             NumberErr::NotAlphaNumeric =>       { println!("Failed: Not alphanumeric!"); },
    ///             NumberErr::NotFitToRadix =>         { println!("Failed: Not decimal number!"); },
    ///             NumberErr::TooBigNumber => {
    ///                     println!("Failed: Too big number!");
    ///                     let mut txt = String::new();
    ///                     write!(&mut txt, "{:?}", e_constains_too_big_number).unwrap();
    ///                     assert_eq!(txt, "Err(TooBigNumber)");
    ///                 },
    ///         }
    ///     },
    /// }
    /// ```
    /// 
    /// # Example 6 for NumberErr::NotAlphaNumeric, NumberErr::NotFitToRadix, and NumberErr::TooBigNumber case
    /// ```
    /// use std::fmt::Write as _;
    /// use cryptocol::number::NumberErr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let f_contains_non_alphanumeric_not_fit_to_radix = U512::from_str_radix("1,1234,5678,9ABC,DEFG,1234,5678,9ABC,DEFG,1234,5678,9ABC,DEFG,1234,5678,9ABC,DEFG,1234,5678,9ABC,DEFG,1234,5678,9ABC,DEFG,1234,5678,9ABC,DEFG,1234,5678,9ABC,DEFG", 16);
    /// match f_contains_non_alphanumeric_not_fit_to_radix
    /// {
    ///     Ok(n) =>  { println!("f_contains_non_alphanumeric_not_fit_to_radix = {}", n); },
    ///     Err(e) => {
    ///         match e
    ///         {
    ///             NumberErr::OutOfValidRadixRange =>  { println!("Failed: Out of Valid Radix Range!") },
    ///             NumberErr::NotAlphaNumeric => {
    ///                     println!("Failed: Not alphanumeric!");
    ///                     let mut txt = String::new();
    ///                     write!(&mut txt, "{:?}", f_contains_non_alphanumeric_not_fit_to_radix).unwrap();
    ///                     assert_eq!(txt, "Err(NotAlphaNumeric)");
    ///                 },
    ///             NumberErr::NotFitToRadix =>    { println!("Failed: Not decimal number!"); },
    ///             NumberErr::TooBigNumber =>     { println!("Failed: Too big number!"); },
    ///         }
    ///     },
    /// }
    /// ```
    pub fn from_str_radix(txt: &str, radix: u32) -> Result<Self, NumberErr>
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
                if c as u32 >= '0' as u32 + radix
                    { return Err(NumberErr::NotFitToRadix); }
            }
            else if radix <= 10 + 26
            {
                if (c as u32 >= 'A' as u32 + radix - 10) 
                        && (c as u32 <= 'Z' as u32)
                    || (c as u32 >= 'a' as u32 + radix - 10)
                    { return Err(NumberErr::NotFitToRadix); }
            }
            else if c as u32 >= 'a' as u32 + radix - (10 + 26)  // radix <= 10 + 26 + 26
                { return Err(NumberErr::NotFitToRadix); }

            let num: u32 = if radix <= 10    { c as u32 - '0' as u32 }
                        else if radix <= 10 + 26
                        {
                            if c <= '9'
                                { c as u32 - '0' as u32 }
                            else if c <= 'Z'
                                { c as u32 - 'A' as u32 + 10 }
                            else
                                { c as u32 - 'a' as u32 + 10 }
                        }
                        else    // if radix <= 10 + 26 + 26
                        {
                            if c <= '9'
                                { c as u32 - '0' as u32 }
                            else if c <= 'Z'
                                { c as u32 - 'A' as u32 + 10 }
                            else
                                { c as u32 - 'a' as u32 + 10 + 26 }
                        };
            bignum.wrapping_mul_assign_uint(T::u32_as_smalluint(radix));
            bignum.wrapping_add_assign_uint(T::u32_as_smalluint(num));
        }
        if bignum.is_overflow()
            { Err(NumberErr::TooBigNumber) }
        else
            { Ok(bignum) }
    }

    // pub fn generate_check_bits(bit_pos: usize) -> Option<Self>
    /// Constucts a new `BigUInt<T, N>` which has the value zero and sets only
    /// the bit specified by the argument bit_pos to be 1.
    /// 
    /// # Output
    /// It returns a big unsigned integer `BigUInt<T, N>` whose bit specified
    /// by the argument bit_posvalue is set to be 1, wrapped by enum
    /// `Some(self)` of `Option<Self>` if the bit positon `bit_pos` is less
    /// than `size_of::<T>() * N * 8`. It returns `None` if the bit positon
    /// `bit_pos` is greater than or equal to `size_of::<T>() * N * 8`.
    /// 
    /// # Bit Position
    /// The bit positon bit_pos is zero-based and should be counted from LSB
    /// (Least Significant Bit) reguardless endian. So, if the bit_pos is `0`,
    /// only LSB is set to be `1` and all the other bits will be set to `0`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with_u32;
    /// define_utypes_with_u32!();
    /// 
    /// let a_0 = U256::generate_check_bits(0).unwrap();
    /// println!("a_0 = {}", a_0.to_string_with_radix_and_stride(2, 10).unwrap());
    /// assert_eq!(a_0.to_string_with_radix_and_stride(2, 10).unwrap(), "1");
    /// 
    /// let a_12 = U256::generate_check_bits(12).unwrap();
    /// println!("a_12 = {}", a_12.to_string_with_radix_and_stride(2, 10).unwrap());
    /// assert_eq!(a_12.to_string_with_radix_and_stride(2, 10).unwrap(), "100_0000000000");
    /// 
    /// let a_255 = U256::generate_check_bits(255).unwrap();
    /// println!("a_255 = {}", a_255.to_string_with_radix_and_stride(2, 10).unwrap());
    /// assert_eq!(a_255.to_string_with_radix_and_stride(2, 10).unwrap(), "100000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000");
    /// 
    /// let a_256 = U256::generate_check_bits(256);
    /// println!("a_256 = {:?}", a_256);
    /// assert_eq!(a_256, None);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn generate_check_bits(bit_pos: usize) -> Option<Self>
    {
        if bit_pos < Self::size_in_bits()
            { Some(Self::generate_check_bits_(bit_pos)) }
        else
            { None }
    }

    // pub fn generate_check_bits_(bit_pos: usize) -> Self
    /// Constucts a new `BigUInt<T, N>` which has the value zero and sets only
    /// the bit specified by the argument bit_pos to be 1.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    /// - If the bit positon `bit_pos` is greater than or equal to
    /// `size_of::<T>() * N * 8`, this method will panic.
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
    /// use cryptocol::define_utypes_with_u32;
    /// define_utypes_with_u32!();
    /// 
    /// let a_0 = U256::generate_check_bits_(0);
    /// println!("a_0 = {}", a_0.to_string_with_radix_and_stride(2, 10).unwrap());
    /// assert_eq!(a_0.to_string_with_radix_and_stride(2, 10).unwrap(), "1");
    /// 
    /// let a_12 = U256::generate_check_bits_(12);
    /// println!("a_12 = {}", a_12.to_string_with_radix_and_stride(2, 10).unwrap());
    /// assert_eq!(a_12.to_string_with_radix_and_stride(2, 10).unwrap(), "100_0000000000");
    /// 
    /// let a_255 = U256::generate_check_bits_(255);
    /// println!("a_255 = {}", a_255.to_string_with_radix_and_stride(2, 10).unwrap());
    /// assert_eq!(a_255.to_string_with_radix_and_stride(2, 10).unwrap(), "100000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000");
    /// 
    /// // It will panic!
    /// // let a_256 = U256::generate_check_bits_(256);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn generate_check_bits_(bit_pos: usize) -> Self
    {
        let mut check_bits = Self::zero();
        check_bits.turn_check_bits(bit_pos);
        check_bits
    }


    
    /***** METHODS TO GET SIZE BOTH IN BYTES AND BITS *****/

    // pub fn size_in_bytes() -> usize
    /// Returns how many bytes long the number `BigUInt` is.
    /// 
    /// # Output
    /// It returns its size in bytes.
    /// 
    /// # Features
    /// It does not count how many bytes are used for flags.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// println!("U256 is {}-byte integer.", U256::size_in_bytes());
    /// assert_eq!(U256::size_in_bytes(), 32);
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
    /// # Features
    /// It does not count how many bytes are used for flags.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// println!("U256 is {}-bit integer.", U256::size_in_bits());
    /// assert_eq!(U256::size_in_bits(), 256);
    /// ```
    #[inline]
    pub fn size_in_bits() -> usize
    {
        T::size_in_bits() * N
    }

    // pub fn length_in_bytes(&self) -> usize
    /// Returns how many bytes long the number i.e. the object of
    /// `BigUInt` is.
    /// 
    /// # Output
    /// It returns its size in bytes.
    /// 
    /// # Features
    /// It does not count how many bytes are used for flags.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a = U256::from_str_radix("A16F", 16).unwrap();
    /// println!("a is {}-byte integer.", a.length_in_bytes());
    /// assert_eq!(a.length_in_bytes(), 32);
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
    /// # Features
    /// It does not count how many bytes are used for flags.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a = U256::from_str_radix("A16F", 16).unwrap();
    /// println!("a is {}-bit integer.", a.length_in_bits());
    /// assert_eq!(a.length_in_bits(), 256);
    /// ```
    #[inline]
    pub fn length_in_bits(&self) -> usize
    {
        Self::size_in_bits()
    }



    /***** METHODS TO GET, SET, AND CHECK *****/

    // pub fn turn_check_bits(&mut self, bit_pos: usize)
    /// Changes a `BigUInt<T, N>` to have the value zero and sets only
    /// the bit specified by the argument `bit_pos` to be 1.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    /// - If the bit positon `bit_pos` is greater than or equal to
    /// `size_of::<T>() * N * 8`, this method will panic.
    /// 
    /// # Bit Position
    /// The bit positon `bit_pos` is zero-based and should be counted from LSB
    /// (Least Significant Bit) reguardless endian. So, if the `bit_pos` is `0`,
    /// only LSB is set to be `1` and all the other bits will be set to `0`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = U256::from_string("256487951236974125896345564889974258").unwrap();
    /// println!("a = {}", a.to_string_with_radix_and_stride(2, 8).unwrap());
    /// a.turn_check_bits(102);
    /// println!("a = {}", a.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a, U256::from_str_radix("1000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000", 2).unwrap());
    /// 
    /// // It will panic.
    /// // a.turn_check_bits(256);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn turn_check_bits(&mut self, bit_pos: usize)
    {
        let size_t_bits = T::size_in_bits();
        let chunk_num = bit_pos / size_t_bits;
        let piece_num = bit_pos % size_t_bits;
        let mut val = T::one();
        val <<= T::usize_as_smalluint(piece_num);
        self.set_zero();
        self.set_num_(chunk_num, val);
    }

    // pub fn is_bit_set(&self, bit_pos: usize) -> Option<bool>
    /// Check a `self` to know whether or not the bit specified by the argument
    /// `bit_pos` to be 1.
    /// 
    /// # Bit Position
    /// The bit positon `bit_pos` is zero-based and should be counted from LSB
    /// (Least Significant Bit) reguardless endianness. So, if the `bit_pos`
    /// is `0`, only LSB is set to be `1` and all the other bits will be set
    /// to `0`.
    /// 
    /// # Output
    /// If the bit specified by `bit_pos` is set to be one, this method returns
    /// `Some(true)` of enum `Option<bool>`. If the bit specified by `bit_pos`
    /// is set to be zero, this method returns `Some(true)` of enum
    /// `Option<bool>`. If the bit positon `bit_pos` is greater than or equal
    /// to `size_of::<T>() * N * 8`, this method returns `None`.
    /// 
    /// # Counterpart method
    /// If you are sure that `bit_pos` is less than `size_of::<T>() * N * 8`,
    /// you can use the method `is_bit_set_()` for better performance.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a = U256::from_string("12345678912345678912345678912345678912345678912345678912345678912345678912345").unwrap();
    /// println!("a = {}_U256", a.to_string_with_radix_and_stride(2, 10).unwrap());
    /// let mut res = a.is_bit_set(151);
    /// match res
    /// {
    ///     Some(r) => {
    ///         println!("The {}th bit is set: {}", 151, r);
    ///         assert_eq!(a.is_bit_set_(151), true);
    ///     },
    ///     None => {
    ///         println!("{}_U256 does not have the {}th bit.", a, 151);
    ///     }
    /// }
    /// 
    /// res = a.is_bit_set(200);
    /// match res
    /// {
    ///     Some(r) => {
    ///         println!("The {}th bit is set: {}", 200, r);
    ///         assert_eq!(a.is_bit_set_(200), false);
    ///     },
    ///     None => {
    ///         println!("{}_U256 does not have the {}th bit.", a, 200);
    ///     }
    /// }
    /// 
    /// res = a.is_bit_set(300);
    /// match res
    /// {
    ///     Some(r) => {
    ///         println!("The {}th bit is set: {}", 300, r);
    ///     },
    ///     None => {
    ///         println!("{}_U256 does not have the {}th bit.", a, 300);
    ///         assert_eq!(a.is_bit_set(300), None);
    ///     }
    /// }
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn is_bit_set(&self, bit_pos: usize) -> Option<bool>
    {
        if (bit_pos / T::size_in_bits()) >= N
            { None }
        else
            { Some(self.is_bit_set_(bit_pos)) }
    }

    // pub fn is_bit_set_(&self, bit_pos: usize) -> bool
    /// Check whether or not the bit specified by the argument
    /// `bit_pos` in `self` to be 1.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    /// - If the bit positon `bit_pos` is greater than or equal to
    /// `size_of::<T>() * N * 8`, this method will panic. So, you are highly
    /// recommended to use only when you are sure that `bit_pos` is neither
    /// greater than nor equal to `size_of::<T>() * N * 8`. Otherwise, use
    /// the method `is_bit_set()`.
    /// 
    /// # Bit Position
    /// The bit positon `bit_pos` is zero-based and should be counted from LSB
    /// (Least Significant Bit) reguardless endianness. So, if the `bit_pos`
    ///  is `0`, only LSB is set to be `1` and all the other bits will be set
    /// to `0`.
    /// 
    /// # Output
    /// If the bit specified by `bit_pos` is set to be one, this method returns
    /// `true`. If the bit specified by `bit_pos` is set to be zero, this
    /// method returns `false`.
    /// 
    /// # Counterpart method
    /// If you are not sure that `bit_pos` is less than `size_of::<T>() * N * 8`,
    /// you are highly encouraged to use the method `is_bit_set()`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a = U256::from_string("12345678912345678912345678912345678912345678912345678912345678912345678912345").unwrap();
    /// println!("a = {}_U256", a.to_string_with_radix_and_stride(2, 10).unwrap());
    /// println!("The {}th bit is set: {}", 151, a.is_bit_set_(151));
    /// assert_eq!(a.is_bit_set_(151), true);
    /// println!("The {}th bit is set: {}", 200, a.is_bit_set_(200));
    /// assert_eq!(a.is_bit_set_(200), false);
    /// // It will panic!!!
    /// // println!("The {}th bit is set: {}", 300, a.is_bit_set_(300));
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn is_bit_set_(&self, bit_pos: usize) -> bool
    {
        let size_t_bits = T::size_in_bits();
        let chunk_num = bit_pos / size_t_bits;
        let piece_num = bit_pos % size_t_bits;
        self.get_num_(chunk_num).is_bit_set_(piece_num)
    }

    // pub fn get_upper_portion(portion: usize) -> Self
    /// Get the non-zero upper portion (high order part) from `self`.
    /// 
    /// # Argument
    /// The argument `portion` specifies the length of the high order part to
    /// take in bits.
    /// 
    /// # Output
    /// - If `portion` is bigger than or equal to the length of the non-zero
    /// part of `self`, this method returns `self`. Here, non-zero part of
    /// `00101100` is not `1011` but `101100` for example.
    /// - If `portion` is less than the length of the non-zero part of `self`,
    /// this method returns the high order part of `self` as many bits as
    /// specified by `portion`. Here, non-zero part of `00101100` is not `1011`
    /// but `101100` for example.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a = U256::from_string("12345678912345678912345678912345678912345678912345678912345678912345678912345").unwrap();
    /// let b = a.get_upper_portion(10);
    /// println!("The 10-bit upper portion of {}_U256 is {}_U256", a.to_string_with_radix_and_stride(2, 10).unwrap(), b.to_string_with_radix_and_stride(2, 10).unwrap());
    /// assert_eq!(b.to_string_with_radix_and_stride(2, 10).unwrap(), "1101101001");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn get_upper_portion(&self, portion: usize) -> Self
    {
        let leading = self.leading_zeros();
        let size = self.length_in_bits();
        let available = size - leading as usize;
        if portion >= available
            { self.clone() }
        else
            { self.shift_right(available - portion) }
    }

    // pub fn get_lower_portion(portion: usize) -> Self
    /// Get the lower portion (low order part) from `self`.
    /// 
    /// # Argument
    /// The argument `portion` specifies the length of the low order part to
    /// take in bits.
    /// 
    /// # Output
    /// - If `portion` is bigger than or equal to the length of the non-zero
    /// part of `self`, this method returns `self`. Here, non-zero part of
    /// `00101100` is not `1011` but `101100` for example.
    /// - If `portion` is less than the length of the non-zero part of `self`,
    /// this method returns the low order part of `self` as many bits as
    /// specified by `portion`. Here, non-zero part of `00101100` is not `1011`
    /// but `101100` for example.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a = U256::from_string("12345678912345678912345678912345678912345678912345678912345678912345678912340").unwrap();
    /// let b = a.get_lower_portion(10);
    /// println!("The 10-bit lower portion of {}_U256 is {}_U256", a.to_string_with_radix_and_stride(2, 10).unwrap(), b.to_string_with_radix_and_stride(2, 10).unwrap());
    /// assert_eq!(b.to_string_with_radix_and_stride(2, 10).unwrap(), "1101010100");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn get_lower_portion(&self, portion: usize) -> Self
    {
        let leading = self.leading_zeros();
        let size = self.length_in_bits();
        let available = size - leading as usize;
        let mut ret = self.clone();
        if portion == 0
        {
            return Self::zero();
        }
        else if portion < available
        {
            let size_t_bits = T::size_in_bits();
            let chunk_num = (portion - 1) / size_t_bits;
            let piece_num = portion % size_t_bits;
            if piece_num != 0
            {
                let mut thing = ret.get_num_(chunk_num);
                thing <<= T::usize_as_smalluint(T::size_in_bits() - piece_num);
                thing >>= T::usize_as_smalluint(T::size_in_bits() - piece_num);
                ret.set_num_(chunk_num, thing);
            }
            for i in (chunk_num + 1)..(N - leading as usize / size_t_bits)
                { ret.set_num_(i, T::zero()); }
        }
        ret
    }

    // pub fn get_num(&self, i: usize) -> Option<T>
    /// Returns i-th element of its array of type `T` wrapped in Some
    /// of enum Option if `i` < `N`. Otherwise, it returns `None`.
    /// 
    /// # Arguments
    /// - `i` is zero-based.
    /// - 0-th element contains LSB (Least Significant Bit), while (N-1)-th
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
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a = U256::from([0_u32, 10, 20, 30, 40, 50, 60, 70]);
    /// match a.get_num(3)
    /// {
    ///     Some(num) => {
    ///         println!("a.get_num(3).unwrap() = {}", num);
    ///         assert_eq!(num, 30);
    ///     },
    ///     None => {
    ///         println!("There is no third element.");
    ///     },
    /// }
    /// 
    /// let f = a.get_num(8);
    /// match f
    /// {
    ///     Some(num) => {
    ///         println!("a.get_num(3).unwrap() = {}", num);
    ///     },
    ///     None => {
    ///         println!("There is no third element.");
    ///         assert_eq!(f, None);
    ///     },
    /// }
    /// ```
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn get_num(&self, i: usize) -> Option<T>
    {
        if i < N
        {
            #[cfg(target_endian = "little")]    { Some(self.get_number()[i]) }
            #[cfg(target_endian = "big")]       { Some(self.get_number()[N-1-i]) }
        }
        else
        {
            None
        }
    }

    // pub fn get_num_(&self, i: usize) -> T
    /// Returns i-th element of its array of type `T` if `i` < `N`.
    /// Otherwise, it will panic.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    /// - This method is performance-oriented and does not care for safety.
    /// So, if `i` >= `N`, it will panic.
    /// 
    /// # Arguments
    /// - `i` is zero-based.
    /// - 0-th element contains LSB (Least Significant Bit), while (N-1)-th
    /// element contains MSB (Most Significant Bit) regardless endianness.
    /// `BigUInt` have an array of type `T` in order to present long-sized
    /// unsigned integer.
    /// 
    /// # Counterpart Method
    /// Use this method only when you are sure that `i` < `N`.
    /// Otherwise, use its Counterpart method
    /// [get_num()](struct@BigUInt#method.get_num) for safety.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a = U256::from([0_u32, 10, 20, 30, 40, 50, 60, 70]);
    /// let b = a.get_num_(3);
    /// println!("a.get_num_(3) = {}", b);
    /// assert_eq!(b, 30);
    /// // It will panic.
    /// // let c = a.get_num_(8);
    /// ```
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn get_num_(&self, i: usize) -> T
    {
        #[cfg(target_endian = "little")]    { self.number[i] }
        #[cfg(target_endian = "big")]       { self.number[N-1-i] }
    }

    // pub fn set_num(&mut self, i: usize, val: T) -> bool
    /// Sets i-th element of its array of type `T`, and return `true`
    /// if `i` < `N`. Otherwise, it sets none of the elements of its
    /// array of type `T`, and returns `false`.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    ///  
    /// # Arguments
    /// - `i` is zero-based.
    /// - 0-th element contains LSB (Least Significant Bit), while (N-1)-th
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
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a = U256::from([0_u64, 10, 20, 30]);
    /// let mut num = a.get_num_(3);
    /// println!("a.get_num(3).unwrap() = {}", num);
    /// let b = a.set_num(3, 0);
    /// assert!(b);
    /// num = a.get_num_(3);
    /// println!("a.get_num(3).unwrap() = {}", num);
    /// assert_eq!(num, 0);
    /// 
    /// let c = a.set_num(4, 0);
    /// if !b
    ///     { println!("There is no fourth element."); }
    /// assert!(!c);
    /// ```
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn set_num(&mut self, i: usize, val: T) -> bool
    {
        if i < N
        {
            #[cfg(target_endian = "little")]    { self.number[i] = val; }
            #[cfg(target_endian = "big")]       { self.number[N-1-i] = val; }
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
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    /// - If `i` >= `N`, it will panic.
    /// 
    /// # Arguments
    /// - `i` is zero-based.
    /// - 0-th element contains LSB (Least Significant Bit), while (N-1)-th
    /// element contains MSB (Most Significant Bit) regardless endianness.
    /// `BigUInt` have an array of type `T` in order to present long-sized
    /// unsigned integer.
    /// 
    /// # Counterpart Method
    /// It is performance-oriented and does not care for safety.
    /// It is virtually the same as the method set_num(). This method set_num_()
    /// is considered to be slightly faster than the method set_num().
    /// Use this method set_num_() only when you are sure that `i` < `N`.
    /// Otherwise, use its Counterpart method
    /// [set_num()](struct@BigUInt#method.set_num).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = U256::from([10_u128, 20]);
    /// let mut num = a.get_num_(1);
    /// println!("a.get_num_(1) = {}", num);
    /// a.set_num_(1, 0);
    /// num = a.get_num_(1);
    /// println!("a.get_num_(1) = {}", num);
    /// assert_eq!(num, 0);
    /// 
    /// // It will panic.
    /// // let c = a.set_num_(4, 0);
    /// ```
    #[inline]
    pub fn set_num_(&mut self, i: usize, val: T)
    {
        #[cfg(target_endian = "little")]    { self.number[i] = val; }
        #[cfg(target_endian = "big")]       { self.number[N-1-i] = val; }
    }

    // pub fn get_number(&self) -> &[T; N]
    /// Returns the reference of its array of `T`-type for borrowing instead
    /// of giving its ownership. `BigUInt` has an array of `T` in order
    /// to present long-sized unsigned integers.
    /// 
    /// # Features
    /// The output of this method is immutable.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// if let Ok(a) = "12345678909876543210123456789098765432101234567890987654321012345678909876543".parse::<U256>()
    /// {
    ///     let arr = a.get_number();
    ///     println!("arr = {:?}", arr);
    ///     assert_eq!(arr, &[169027903, 1302152522, 3897323189, 3259190507, 1179716839, 4196280276, 2015458651, 457926681]);
    /// }
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
    /// # Features
    /// The output of this method is mutable.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// if let Ok(a) = "12345678909876543210123456789098765432101234567890987654321012345678909876543".parse::<U256>()
    /// {
    ///     let arr = a.get_number_mut();
    ///     println!("arr = {:?}", arr);
    ///     assert_eq!(arr, &[169027903, 1302152522, 3897323189, 3259190507, 1179716839, 4196280276, 2015458651, 457926681]);
    /// }
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
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a = U256::new();
    /// println!("arr = {:?}", a);
    /// let arr = [1_u16, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    /// a.set_number(&arr);
    /// println!("arr = {:?}", a);
    /// assert_eq!(a.get_number(), &arr);
    /// ```
    #[inline]
    pub fn set_number(&mut self, val: &[T; N])
    {
        self.number.copy_from_slice(val);
    }

    // fn copy_within<R>(&mut self, src: R, dest: usize)
    /// Copies elements from one part of the slice `T`-array of BigUInt to
    /// another part of itself, using a memmove.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    /// - This method copy_within() will panic if either range exceeds the end
    /// of the slice, or if the end of src is before the start.
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
    // / # Example
    // / ```
    // / use cryptocol::define_utypes_with;
    // / define_utypes_with!(u16);
    // / let mut a = U256::new();
    // / a.set_number(&[0_u16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    // / println!("a = {:?}", a);
    // / a.copy_within(3..10, 6);
    // / println!("a = {:?}", a);
    // / assert_eq!(a.get_number(), &[0, 1, 2, 3, 4, 5, 3, 4, 5, 6, 7, 8, 9, 13, 14, 15]);
    // / ```
    #[cfg(target_endian = "little")]
    #[inline]
    fn copy_within<R>(&mut self, src: R, dest: usize)
    where R: RangeBounds<usize>
    {
        self.number.copy_within(src, dest);
    }

    // fn copy_within<R>(&mut self, src: R, dest: usize)
    /// Copies elements from one part of the slice `T`-array of BigUInt to
    /// another part of itself, using a memmove.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    /// - This method copy_within() will panic if either range exceeds the end
    /// of the slice, or if the end of src is before the start.
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
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// let mut a = U256::new();
    /// a.set_number(&[0_u16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    /// println!("a = {:?}", a);
    /// a.copy_within(3..10, 6);
    /// println!("a = {:?}", a);
    /// assert_eq!(a.get_number(), &[0, 1, 2, 3, 4, 5, 3, 4, 5, 6, 7, 8, 9, 13, 14, 15]);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[cfg(target_endian = "big")]
    #[inline]
    fn copy_within<R>(&mut self, src: R, dest: usize)
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
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a = U256::new();
    /// a.set_number(&[1_u16, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    /// println!("a = {}", a);
    /// a.set_zero();
    /// println!("a = {}", a);
    /// assert_eq!(a, U256::zero());
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
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = U1024::zero();
    /// let mut b_zero = a.is_zero();
    /// if b_zero
    /// {
    ///     println!("a is Zero");
    ///     assert_eq!(b_zero, true);
    /// }
    /// else
    /// {
    ///     println!("a is Not Zero");
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = U1024::zero();
    /// let mut b_zero = a.is_zero();
    /// a.set_one();
    /// b_zero = a.is_zero();
    /// if b_zero
    /// {
    ///     println!("a is Zero");
    /// }
    /// else
    /// {
    ///     println!("a is Not Zero");
    ///     assert_eq!(b_zero, false);
    /// }
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
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a = U256::new();
    /// a.set_number(&[1_u16, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    /// println!("a = {}", a);
    /// a.set_one();
    /// println!("a = {}", a);
    /// assert_eq!(a, U256::one());
    /// ```
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn set_one(&mut self)
    {
        self.set_zero();
        #[cfg(target_endian = "little")]    { self.set_num(0, T::one()); }
        #[cfg(target_endian = "big")]       { self.set_num(N-1, T::one()); }
    }

    // pub fn is_one(&self) -> bool
    /// Checks whether `BigUInt` to be one and returns true if it is
    /// one, and returns false if it is not one.
    /// 
    /// # Output
    /// It returns `true` if it is one. Otherwise, it returns `false`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = U256::one();
    /// let mut b_one = a.is_one();
    /// if b_one
    /// {
    ///     println!("a is One");
    ///     assert_eq!(b_one, true);
    /// }
    /// else
    /// {
    ///     println!("a is Not One");
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = U256::one();
    /// let mut b_one = a.is_one();
    /// 
    /// a.set_max();
    /// b_one = a.is_one();
    /// if b_one
    /// {
    ///     println!("a is One");
    /// }
    /// else
    /// {
    ///     println!("a is Not One");
    ///     assert_eq!(b_one, false);
    /// }
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
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a = U256::zero();
    /// println!("a = {}", a);
    /// let b_zero_or_one = a.is_zero_or_one();
    /// if b_zero_or_one
    /// {
    ///     println!("a is One or Zero.");
    ///     assert_eq!(b_zero_or_one, true);
    /// }
    /// else
    /// {
    ///     println!("a is Neither One nor Zero.");
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a = U256::one();
    /// println!("a = {}", a);
    /// let b_zero_or_one = a.is_zero_or_one();
    /// if b_zero_or_one
    /// {
    ///     println!("a is One or Zero.");
    /// }
    /// else
    /// {
    ///     println!("a is Neither One nor Zero.");
    ///     assert_eq!(b_zero_or_one, true);
    /// }
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a = U256::one();
    /// a.wrapping_add_assign_uint(1_u8);
    /// println!("a = {}", a);
    /// let b_zero_or_one = a.is_zero_or_one();
    /// if b_zero_or_one
    /// {
    ///     println!("a is One or Zero.");
    /// }
    /// else
    /// {
    ///     println!("a is Neither One nor Zero.");
    ///     assert_eq!(b_zero_or_one, false);
    /// }
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
    /// Sets `self` to be maximum value in which all bits are
    /// set to be `1`.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a = U256::new();
    /// println!("a = {}", a);
    /// a.set_max();
    /// println!("a = {}", a.to_string_with_radix_and_stride(16, 8).unwrap());
    /// assert_eq!(a.to_string_with_radix_and_stride(16, 8).unwrap(), "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF");
    /// ```
    pub fn set_max(&mut self)
    {
        for i in 0..N
            { self.set_num(i, T::max()); }
    }

    // pub fn set_submax(&mut self, size_in_bits: usize)
    /// Sets `set` to be `size_in_bits`-bit long maximum value
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
    /// let mut a = U256::new();
    /// println!("a = {}", a);
    /// a.set_submax(200_usize);
    /// println!("a = {}", a.to_string_with_radix_and_stride(16, 8).unwrap());
    /// assert_eq!(a.to_string_with_radix_and_stride(16, 8).unwrap(), "FF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF");
    /// ```
    pub fn set_submax(&mut self, size_in_bits: usize)
    {
        let size_t_bits = T::size_in_bits();
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

        let chunk_num = size_in_bits / size_t_bits;
        let piece_num = size_in_bits % size_t_bits;
        let zero = T::zero();
        let max = T::max();
        self.reset_all_flags();
        for i in 0..chunk_num
            { self.set_num_(i, max); }
        for i in chunk_num..N
            { self.set_num_(i, zero); }
        if piece_num != 0
            { self.set_num_(chunk_num, max >> T::usize_as_smalluint(size_t_bits - piece_num)); }
    }

    // pub fn set_halfmax(&mut self)
    /// Sets `self` to be half long maximum value
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
    /// let mut a = U256::new();
    /// println!("a = {}", a);
    /// a.set_halfmax();
    /// println!("a = {}", a.to_string_with_radix_and_stride(16, 8).unwrap());
    /// assert_eq!(a.to_string_with_radix_and_stride(16, 8).unwrap(), "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF");
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
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a = U256::max();
    /// println!("Is {} a 256-bit maximum? - {}", a, a.is_max());
    /// assert_eq!(a.is_max(), true);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let b = U256::max().wrapping_sub_uint(1_u8);
    /// println!("Is {} a 256-bit maximum? - {}", b, b.is_max());
    /// assert_eq!(b.is_max(), false);
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

    // pub fn set_msb(&mut self)
    /// Sets the MSB (Most Significant Bit) of `BigUInt`-type number with `1`.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a = U256::new();
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
    pub fn set_msb(&mut self)
    {
        let highest = self.get_num_(N-1);
        let msb = !(T::max() >> T::one());
        self.set_num_(N-1, highest | msb);
    }

    // pub fn set_lsb(&mut self)
    /// Sets the LSB (Least Significant Bit) of `BigUInt`-type number with `1`.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a = U256::new();
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
    pub fn set_lsb(&mut self)
    {
        let lowest = self.get_num_(0);
        let lsb = T::one();
        self.set_num_(0, lowest | lsb);
    }

    // pub fn set_uint<U>(&mut self, val: U)
    /// Sets `BigUInt`-type number with `U`-type small value such as `u8`,
    /// `u16`, `u32`, `u64`, and `u128` type value. This mathod set_uint()
    /// is useful especially when you initialize `BigUInt`-type big
    /// unsigned integer with a small value.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    /// - If `size_of::<T>() * N` < `size_of::<U>()`, this method may panic or
    /// its behavior may be undefined though it may not panic.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a = U1024::new();
    /// println!("a = {}", a);
    /// a.set_uint(340282366920938463453374607431768211455_u128);
    /// println!("a = {}", a);
    /// assert_eq!(a.to_string(), "340282366920938463453374607431768211455");
    /// ```
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn set_uint<U>(&mut self, val: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let size_t = T::size_in_bytes();
        let size_v = U::size_in_bytes();
        let mut share = SharedValues::<T, U>::from_src(val);
        
        self.set_zero();
        if size_t >= size_v
        {
            unsafe { self.set_num_(0, share.des); }
        }
        else    // size_v is multiple of size_t.
        {
            let size_t_bits = size_t * 8;
            #[cfg(target_endian = "little")]
            for i in 0..size_v/size_t
            {
                unsafe { self.set_num_(i, share.des); }
                unsafe { share.src >>= U::usize_as_smalluint(size_t_bits); }
            }
            #[cfg(target_endian = "big")]
            {
                let mut i = size_v/size_t - 1;
                loop
                {
                    unsafe { self.set_num_(i, share.des); }
                    unsafe { share.src <<= U::usize_as_smalluint(size_t_bits); }
                    if i == 0
                        { break; }
                    i -= 1;
                }
            }
        }
    }

    // pub fn is_uint<U>(&self, val: U) -> bool
    /// Check whether the `BigUInt`-type number is equal to `U`-type number.
    /// It will return `true`, if it is equal to the `U`-type number. Otherwise,
    /// it will return `false`.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    /// - If `size_of::<T>() * N` < `size_of::<U>()`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
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
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a = U1024::one() + 50_u16;
    /// println!("Question: Is a 51?\nAnswer: {}", a.is_uint(51_u32));
    /// assert_eq!(a.is_uint(51_u16), true);
    /// assert_eq!(a.is_uint(50_u16), false);
    /// ```
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn is_uint<U>(&self, val: U) -> bool
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let size_t = T::size_in_bytes();
        let size_v = U::size_in_bytes();
        let mut share = SharedValues::<T, U>::from_src(val);
        
        if size_t >= size_v
        {
            if unsafe { self.get_num_(0) != share.des }
                { return false; }
            for i in 1..N
            {
                if self.get_num_(i) != T::zero()
                    { return false; }
            }
        }
        else    // size_v is multiple of size_t.
        {
            let size_t_bits = size_t * 8;
            #[cfg(target_endian = "little")]
            for i in 0..size_v/size_t
            {
                if unsafe { self.get_num_(i) != share.des }
                    { return false; }
                unsafe { share.src >>= U::usize_as_smalluint(size_t_bits); }
            }
            #[cfg(target_endian = "big")]
            {
                let mut i = size_v/size_t - 1;
                loop
                {
                    if unsafe { self.get_num_(i) != share.des }
                        { return false; }
                    if i == 0
                        { break; }
                    unsafe { share.src <<= U::usize_as_smalluint(size_t_bits); }     
     
                    i -= 1;          
                }
            }
            for i in size_v/size_t..N
            {
                if self.get_num_(i) != T::zero()
                    { return false; }
            }
        }
        true
    }

    // pub fn is_odd(&self) -> bool
    /// Checks whether the `BigUInt`-type number is an odd number.
    /// 
    /// # Output
    /// It will return `true`, if it is odd. Otherwise, it will return `false`.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a = U256::new();
    /// a.set_uint(340282366920938463453374697431768211455_u128);
    /// if a.is_odd()
    ///     { println!("{} is odd", a); }
    /// else
    ///     { println!("{} is even", a); }
    /// assert_eq!(a.is_odd(), true);
    /// 
    /// a <<= 1;
    /// if a.is_odd()
    ///     { println!("{} is odd", a); }
    /// else
    ///     { println!("{} is even", a); }
    /// assert_eq!(a.is_odd(), false);
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
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = U256::new();
    /// a.set_uint(169743176821145534028236692093846345739_u128);
    /// if a.is_even()
    ///     { println!("{} is even", a); }
    /// else
    ///     { println!("{} is odd", a); }
    /// assert_eq!(a.is_even(), false);
    /// 
    /// a <<= 1;
    /// if a.is_even()
    ///     { println!("{} is even", a); }
    /// else
    ///     { println!("{} is odd", a); }
    /// assert_eq!(a.is_even(), true);
    /// ```
    #[inline]
    pub fn is_even(&self) -> bool
    {
        !self.is_odd()
    }

    // pub fn is_msb_set(&self) -> bool
    /// Checks whether the MSB (Most Segnificant Bit) of `self` is set to be
    /// one.
    /// 
    /// # Output
    /// It will return `true`, if the MSB of `self` is set to be one.
    /// Otherwise, it will return `false`.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = U256::new();
    /// a.set_uint(169743176821145534028236692093846345739_u128);
    /// if a.is_msb_set()
    ///     { println!("{} is greater than halfmax ({}).", a, U256::halfmax()); }
    /// else
    ///     { println!("{} is less than or equal to halfmax ({}).", a, U256::halfmax()); }
    /// assert_eq!(a.is_msb_set(), false);
    /// 
    /// a.set_msb();
    /// if a.is_msb_set()
    ///     { println!("{} is greater than halfmax ({}).", a, U256::halfmax()); }
    /// else
    ///     { println!("{} is less than or equal to halfmax ({}).", a, U256::halfmax()); }
    /// assert_eq!(a.is_msb_set(), true);
    /// ```
    #[inline]
    pub fn is_msb_set(&self) -> bool
    {
        self.get_num_(N-1).is_msb_set()
    }


///////////////

    /***** METHODS TO CHECK BITS *****/

    // pub fn count_ones(&self) -> u32
    /// Returns the number of ones in the binary representation
    /// of the array `number[T;N]` of `self`.
    /// 
    /// # Output
    /// It returns the total number of the bits that are set to be one.
    /// 
    /// # Examples
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a = U256::from_str("100000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap();
    /// println!("{} has {} ones in binary.", a, a.count_ones());
    /// assert_eq!(a.count_ones(), 107);
    /// ```
    pub fn count_ones(&self) -> u32
    {
        let mut res = 0_u32;
        for i in 0..N
            { res += self.get_num_(i).count_ones(); }
        res
    }

    // pub fn count_zeros(&self) -> u32
    /// Returns the number of zeros in the binary representation
    /// of the array `number[T;N]` of `self`.
    /// 
    /// # Output
    /// It returns the total number of the bits that are set to be zero.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a = "100000000000000000000000000000000000000000000000000000000000000000000000000000".parse::<U256>().unwrap();
    /// println!("{} has {} zeros in binary.", a, a.count_zeros());
    /// assert_eq!(a.count_zeros(), 149);
    /// ```
    pub fn count_zeros(&self) -> u32
    {
        let mut res = 0_u32;
        for i in 0..N
            { res += self.get_num_(i).count_zeros(); }
        res
    }

    // pub fn leading_ones(&self) -> u32
    /// Returns the number of leading ones in the binary representation
    /// of the array `number[T;N]` of `self`.
    /// 
    /// # Output
    /// It returns the total number of the leading bits that are set to be one.
    /// 
    /// # Examples
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a = U256::from_str("100000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap();
    /// println!("{} has {} leading ones in binary.", a, a.leading_ones());
    /// assert_eq!(a.leading_ones(), 2);
    /// ```
    pub fn leading_ones(&self) -> u32
    {
        let mut res = 0_u32;
        let mut i = N-1;
        while i != 0
        {
            if self.get_num_(i).is_max()
                { res += T::size_in_bits().into_u32(); }
            else
                { return res + self.get_num_(i).leading_ones(); }
            i -= 1;
        }
        res + self.get_num_(0).leading_ones()
    }

    // pub fn leading_zeros(&self) -> u32
    /// Returns the number of leading zeros in the binary representation
    /// of the array `number[T;N]` of `self`.
    /// 
    /// # Output
    /// It returns the total number of the leading bits that are set to be zero.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a = "100000000000000000000000000000000000000000000000000000000000000000000000000000".parse::<U256>().unwrap();
    /// println!("{} has {} leading zeros in binary.", a, a.leading_zeros());
    /// assert_eq!(a.leading_zeros(), 0);
    /// ```
    pub fn leading_zeros(&self) -> u32
    {
        let mut res = 0_u32;
        let mut i = N-1;
        while i != 0
        {
            if self.get_num_(i).is_zero()
                { res += T::size_in_bits().into_u32(); }
            else
                { return res + self.get_num_(i).leading_zeros(); }
            i -= 1;
        }
        res + self.get_num_(0).leading_zeros()
    }

    // pub fn trailing_ones(&self) -> u32
    /// Returns the number of trailing ones in the binary representation
    /// of the array `number[T;N]` of `self`.
    /// 
    /// # Output
    /// It returns the total number of the trailing bits that are set to be one.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use std::str::FromStr;
    /// define_utypes_with!(u128);
    /// 
    /// let a = U256::from_str("111111111111111111111111111111111111111111111111111111111111111111111111111111").unwrap();
    /// println!("{} has {} trailing ones in binary.", a, a.trailing_ones());
    /// assert_eq!(a.trailing_ones(), 3);
    /// ```
    pub fn trailing_ones(&self) -> u32
    {
        let mut res = 0_u32;
        for i in 0..N
        {
            if self.get_num_(i).is_max()
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
    /// of the array `number[T;N]` of `self`.
    /// 
    /// # Output
    /// It returns the total number of the trailing bits that are set to be zero.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a = "111111111111111111111111111111111111111111111111111111111111111111111111111111".parse::<U256>().unwrap();
    /// println!("{} has {} trailing zeros in binary.", a, a.trailing_zeros());
    /// assert_eq!(a.trailing_zeros(), 0);
    /// ```
    pub fn trailing_zeros(&self) -> u32
    {
        let mut res = 0_u32;
        for i in 0..N
        {
            if self.get_num_(i).is_zero()
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
    /// representation of the array `number[T;N]` of `self`.
    /// Here, 'maximum element' means the element that has all bits to be one.
    /// 
    /// # Output
    /// It returns the total number of the leading maximum elements
    /// that has all bits set to be one.
    /// Here, 'maximum element' means the element that has all bits to be one. 
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a = U256::from_str_radix("FFFFFFFF_EEEEEEEE_DDDDDDDD_CCCCCCCC_BBBBBBBB_AAAAAAAA_99999999_88888888", 16).unwrap();
    /// println!("{} has {} leading max elements in array.", a, a.leading_max_elements());
    /// assert_eq!(a.leading_max_elements(), 4);
    /// ```
    pub fn leading_max_elements(&self) -> u32
    {
        let mut res = 0_u32;
        let mut i = N-1;
        while i != 0
        {
            if self.get_num_(i).is_max()
                { res += 1; }
            else
                { return res; }
            i -= 1;
        }
        if self.get_num_(0).is_max()
            { res + 1 }
        else
            { res }
    }

    // pub fn leading_zero_elements(&self) -> u32
    /// Returns the number of leading zero elements in the binary
    /// representation of the array `number[T;N]` of `self`.
    /// 
    /// # Output
    /// It returns the total number of the leading zero elemments
    /// that are set to be zero.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a = U256::from_str_radix("00000000_FFFFFFFF_EEEEEEEE_DDDDDDDD_CCCCCCCC_BBBBBBBB_AAAAAAAA_99999999", 16).unwrap();
    /// println!("{} has {} leading zero elements in array.", a, a.leading_zero_elements());
    /// assert_eq!(a.leading_zero_elements(), 1);
    /// ```
    pub fn leading_zero_elements(&self) -> u32
    {
        let mut res = 0_u32;
        let mut i = N-1;
        while i != 0
        {
            if self.get_num_(i).is_zero()
                { res += 1; }
            else
                { return res; }
            i -= 1;
        }
        if self.get_num_(0).is_zero()
            { res + 1 }
        else
            { res }
    }

    // pub fn trailing_max_elements(&self) -> u32
    /// Returns the number of trailing maximum elements in the binary
    /// representation of the array `number[T;N]` of `self`.
    /// Here, 'maximum element' means the element that has all bits to be one.
    /// 
    /// # Output
    /// It returns the total number of the trailing maximum elemeents
    /// that have all bits set to be one.
    /// Here, 'maximum element' means the element that has all bits to be one.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a = U256::from_str_radix("88888888_99999999_AAAAAAAA_BBBBBBBB_CCCCCCCC_DDDDDDDD_EEEEEEEE_FFFFFFFF", 16).unwrap();
    /// println!("{} has {} leading max elements in array.", a, a.trailing_max_elements());
    /// assert_eq!(a.trailing_max_elements(), 2);
    /// ```
    pub fn trailing_max_elements(&self) -> u32
    {
        let mut res = 0_u32;
        for i in 0..N
        {
            if self.get_num_(i).is_max()
                { res += 1; }
            else
                { return res; }
        }
        res
    }

    // pub fn trailing_zero_elements(&self) -> u32
    /// Returns the number of trailing zeros in the binary representation
    /// of the array `number[T;N]` of `self`.
    /// 
    /// # Output
    /// It returns the total number of the trailing bits that are set to be zero.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a = U256::from_str_radix("FFFFFFFF_EEEEEEEE_DDDDDDDD_CCCCCCCC_BBBBBBBB_AAAAAAAA_9999999_900000000", 16).unwrap();
    /// println!("{} has {} leading zero elements in array.", a, a.trailing_zero_elements());
    /// assert_eq!(a.trailing_zero_elements(), 4);
    /// ```
    pub fn trailing_zero_elements(&self) -> u32
    {
        let mut res = 0_u32;
        for i in 0..N
        {
            if self.get_num_(i).is_zero()
                { res += 1; }
            else
                { return res; }
        }
        res
    }



    /***** METHODS FOR COMPARISON WITH UINT *****/

    // pub fn partial_cmp_uint<U>(&self, other: U) -> Option<Ordering>
    /// Compares `self` and a value of type `U` and returns the
    /// result of the comparison in the type `Option<Ordering>`.
    /// However, if the datatype `U` is the same datatype `T`, it will be
    /// more convenient for you to use the operators `<`, `>`, `<=`, `>=`,
    /// `==`, and `!=`. Then, you don't have to use `partial_cmp_uint()`
    /// directly. But, if the datatype `U` is not the same datatype `T`, you
    /// can use the methods `lt_uint()`, `gt_uint()`, `le_uint()`,
    /// `ge_uint()`, and `eq_uint()` for your convenience. Then, you don't
    /// have to use `partial_cmp_uint()` directly too.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    /// - If `size_of::<T>() * N` < `size_of::<U>()`, This method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `Ordering::Greater` wrapped by `Some` of enum `Option`
    /// if `self` is greater than `other`.
    /// It returns `Ordering::Less` wrapped by `Some` of enum `Option`
    /// if `self` is less than `other`.
    /// It returns `Ordering::Equal` wrapped by `Some` of enum `Option`
    /// if `self` is equal to `other`.
    /// 
    /// # Example 1
    /// ```
    /// use std::cmp::Ordering;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let res = UU32::from_uint(100_u8).partial_cmp_uint(90_u128).unwrap();
    /// match res
    /// {
    ///     Ordering::Greater => { println!("100 > 90"); }
    ///     Ordering::Less => { println!("100 < 90"); }
    ///     Ordering::Equal => { println!("100 = 90"); }
    /// }
    /// assert_eq!(res, Ordering::Greater);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::cmp::Ordering;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let res = UU32::from_uint(100_u8).partial_cmp_uint(110_u128).unwrap();
    /// match res
    /// {
    ///     Ordering::Greater => { println!("100 > 110"); }
    ///     Ordering::Less => { println!("100 < 110"); }
    ///     Ordering::Equal => { println!("100 = 110"); }
    /// }
    /// assert_eq!(res, Ordering::Less);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use std::cmp::Ordering;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let res = UU32::from_uint(100_u8).partial_cmp_uint(100_u128).unwrap();
    /// match res
    /// {
    ///     Ordering::Greater => { println!("100 > 100"); }
    ///     Ordering::Less => { println!("100 < 100"); }
    ///     Ordering::Equal => { println!("100 = 100"); }
    /// }
    /// assert_eq!(res, Ordering::Equal);
    /// ```
    pub fn partial_cmp_uint<U>(&self, other: U) -> Option<Ordering>
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if other.length_in_bytes() > T::size_in_bytes()
        {
            return self.partial_cmp(&Self::from_uint(other));
        }

        // if rhs.length_in_bytes() <= T::size_in_bytes()
        let t_other = T::num::<U>(other);
        if self.get_num_(0) > t_other
        {
            return Some(Ordering::Greater);
        }
        else if self.get_num_(0) < t_other
        {
            for idx in 1..N
            {
                if self.get_num_(idx) != T::zero()
                    { return Some(Ordering::Greater); }
            }
            return Some(Ordering::Less);
        }
        else    // if self.number[0] == other
        {
            for idx in 1..N
            {
                if self.get_num_(idx) != T::zero()
                    { return Some(Ordering::Greater); }
            }
        }
        Some(Ordering::Equal)
    }

    // pub fn lt_uint<U>(&self, other: U) -> bool
    /// Compares `self` and `other` to find whether `self` is less than `other`.
    /// However, if the datatype `U` is the same datatype `T`, it will be
    /// more convenient for you to use the operator `<`.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    /// - If `size_of::<T>() * N` < `size_of::<U>()`, This method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `true` if `self` is less than `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let res = UU32::from_uint(100_u16).gt_uint(90_u64);
    /// if res
    ///     { println!("100 > 90"); }
    /// else
    ///     { println!("100 <= 90"); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let res = UU32::from_uint(100_u16).gt_uint(110_u64);
    /// if res
    ///     { println!("100 > 110"); }
    /// else
    ///     { println!("100 <= 110"); }
    /// assert_eq!(res, false);
    /// ```
    #[inline]
    pub fn lt_uint<U>(&self, other: U) -> bool
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    { self.partial_cmp_uint(other).unwrap().is_lt() }

    // pub fn gt_uint<U>(&self, other: U) -> bool
    /// Compares `self` and `other` to find whether `self` is greater
    /// than `other`. However, if the datatype `U` is the same datatype `T`,
    /// it will be more convenient for you to use the operator `>`.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    /// - If `size_of::<T>() * N` < `size_of::<U>()`, This method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `true` if `self` is greater than `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let res = UU32::from_uint(100_u32).gt_uint(90_u32);
    /// if res
    ///     { println!("100 > 90"); }
    /// else
    ///     { println!("100 <= 90"); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let res = UU32::from_uint(100_u32).gt_uint(110_u32);
    /// if res
    ///     { println!("100 > 110"); }
    /// else
    ///     { println!("100 <= 110"); }
    /// assert_eq!(res, false);
    /// ```
    #[inline]
    pub fn gt_uint<U>(&self, other: U) -> bool
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    { self.partial_cmp_uint(other).unwrap().is_gt() }

    // pub fn le_uint<U>(&self, other: U) -> bool
    /// Compares `self` and `other` to find whether `self` is less than or
    /// equal to `other`. However, if the datatype `U` is the same datatype
    /// `T`, it will be more convenient for you to use the operator `<=`.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    /// - If `size_of::<T>() * N` < `size_of::<U>()`, This method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `true` if `self` is less than or equal to `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let res = UU32::from_uint(100_u64).le_uint(110_u16);
    /// if res
    ///     { println!("100 <= 110"); }
    /// else
    ///     { println!("100 > 110"); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let res = UU32::from_uint(100_u64).lt_uint(90_u16);
    /// if res
    ///     { println!("100 < 90"); }
    /// else
    ///     { println!("100 >= 90"); }
    /// assert_eq!(res, false);
    /// ```
    #[inline]
    pub fn le_uint<U>(&self, other: U) -> bool
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    { self.partial_cmp_uint(other).unwrap().is_le() }

    // pub fn ge_uint<U>(&self, other: U) -> bool 
    /// Compares `self` and `other` to find whether `self` is greater than
    /// or equal to `other`. However, if the datatype `U` is the same datatype
    /// `T`, it will be more convenient for you to use the operator `>=`.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    /// - If `size_of::<T>() * N` < `size_of::<U>()`, This method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `true` if `self` is greater than or equal to `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let res = UU32::from_uint(100_u128).gt_uint(90_u8);
    /// if res
    ///     { println!("100 >= 90"); }
    /// else
    ///     { println!("100 <= 90"); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let res = UU32::from_uint(100_u128).gt_uint(110_u8);
    /// if res
    ///     { println!("100 > 110"); }
    /// else
    ///     { println!("100 <= 110"); }
    /// assert_eq!(res, false);
    /// ```
    #[inline]
    pub fn ge_uint<U>(&self, other: U) -> bool
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    { self.partial_cmp_uint(other).unwrap().is_ge() }

    // pub fn eq_uint<U>(&self, other: U) -> bool
    /// Compares `self` and `other` to find whether `self` is equal to `other`.
    /// However, if the datatype `U` is the same datatype `T`, it will be
    /// more convenient for you to use the operator `==`.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    /// - If `size_of::<T>() * N` < `size_of::<U>()`, This method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `true` if `self` is equal to `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let res = UU32::from_uint(100_u32).eq_uint(100_u8);
    /// if res
    ///     { println!("100 = 100"); }
    /// else
    ///     { println!("100 != 100"); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let res = UU32::from_uint(100_u64).eq_uint(200_u16);
    /// if res
    ///     { println!("100 = 200"); }
    /// else
    ///     { println!("100 != 200"); }
    /// assert_eq!(res, false);
    /// ```
    #[inline]
    pub fn eq_uint<U>(&self, other: U) -> bool
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    { self.partial_cmp_uint(other).unwrap().is_eq() }



    /***** METHODS FOR COMPARISON WITH BIGUINT *****/

    // pub fn eq(&self, other: &Self) -> bool
    /// Compare `self` with `other` to find whether `self` is equal to `other`.
    /// However, it will be more convenient to you if you use use the operator
    /// `==`. Then, you don't have to use `partial_cmp()` directly.
    /// 
    /// # Output
    /// It returns `true` if `self` is equal to `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let res = UU32::from_string(num_str).unwrap().eq(&UU32::from_string(num_str).unwrap());
    /// if res
    ///     { println!("{0} = {0}", num_str); }
    /// else
    ///     { println!("{0} != {0}", num_str); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let res = UU32::from_string(num_str).unwrap().eq(&UU32::from_uint(100_u8));
    /// if res
    ///     { println!("{} = 100", num_str); }
    /// else
    ///     { println!("{} != 100", num_str); }
    /// assert_eq!(res, false);
    /// ```
    pub fn eq(&self, other: &Self) -> bool
    {
        for idx in 0..N
        {
            if self.get_num_(idx) != other.get_num_(idx)
                { return false; }
        }
        true
    }

    // pub fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    /// Compares `self` and a value of `other` and returns the result of the
    /// comparison in the type `Option<Ordering>`.
    /// However, it will be more convenient to you if you use use the operators
    /// `<`, `>`, `<=`, `>=`,  `==`, and `!=`. Then, you don't have to use
    /// `partial_cmp()` directly.
    /// 
    /// # Output
    /// It returns `Ordering::Greater` wrapped by `Some` of enum `Option`
    /// if `self` is greater than `other`.
    /// It returns `Ordering::Less` wrapped by `Some` of enum `Option`
    /// if `self` is less than `other`.
    /// It returns `Ordering::Equal` wrapped by `Some` of enum `Option`
    /// if `self` is equal to `other`.
    /// 
    /// # Example 1
    /// ```
    /// use std::cmp::Ordering;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let num_str1 = "70000000000000000000000000000000000000000000000000000000000000000000000000000";
    /// let num_str2 = "60000000000000000000000000000000000000000000000000000000000000000000000000000";
    /// let num1 = num_str1.parse::<UU32>().unwrap();
    /// let num2 = num_str2.parse::<UU32>().unwrap();
    /// 
    /// let res = num1.partial_cmp(&num2).unwrap();
    /// match res
    /// {
    ///     Ordering::Greater => { println!("{} > {}", num1, num2); }
    ///     Ordering::Less => { println!("{} < {}", num1, num2); }
    ///     Ordering::Equal => { println!("{} = {}", num1, num2); }
    /// }
    /// assert_eq!(res, Ordering::Greater);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::cmp::Ordering;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let num_str1 = "70000000000000000000000000000000000000000000000000000000000000000000000000000";
    /// let num_str3 = "80000000000000000000000000000000000000000000000000000000000000000000000000000";
    /// let num1 = num_str1.parse::<UU32>().unwrap();
    /// let num3 = num_str3.parse::<UU32>().unwrap();
    /// let res = num1.partial_cmp(&num3).unwrap();
    /// match res
    /// {
    ///     Ordering::Greater => { println!("{} > {}", num1, num3); }
    ///     Ordering::Less => { println!("{} < {}", num1, num3); }
    ///     Ordering::Equal => { println!("{} = {}", num1, num3); }
    /// }
    /// assert_eq!(res, Ordering::Less);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use std::cmp::Ordering;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let num_str1 = "70000000000000000000000000000000000000000000000000000000000000000000000000000";
    /// let num1 = num_str1.parse::<UU32>().unwrap();
    /// 
    /// let res = num1.partial_cmp(&num1).unwrap();
    /// match res
    /// {
    ///     Ordering::Greater => { println!("{0} > {0}", num1); }
    ///     Ordering::Less => { println!("{0} < {0}", num1); }
    ///     Ordering::Equal => { println!("{0} = {0}", num1); }
    /// }
    /// assert_eq!(res, Ordering::Equal);
    /// ```
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

    

    /***** ARITHMATIC OPERATIONS WITH UINT *****/

    /*** ADDITION ***/

    // pub fn carrying_add_uint<U>(&self, rhs: U, carry: bool) -> (Self, bool)
    /// Calculates `self` + `rhs` + `carry`,
    /// wrapping around at the boundary of the type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Outputs
    /// It returns a tuple containing the sum and the output carry. It performs
    /// “ternary addition” of one big integer operand, a primitive unsigned
    /// integer, and a carry-in bit, and returns an output big integer and a
    /// carry-out bit.
    /// 
    /// # Features
    /// - This allows chaining together multiple additions to create even a
    /// wider addition. This can be thought of as a big integer “full adder”,
    /// in the electronics sense.
    /// - If the input carry is `false`, this method is equivalent to
    /// `overflowing_add_uint()`, and the output carry reflect current overflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method [carrying_add()](struct@BigUInt#method.carrying_add)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let num_str1 = "FFEEDDBB_AA998877_66554433_221100FF_EEDDBBAA_99887766_55443322_1100FFEE";
    /// let num1 = UU32::from_str_radix(num_str1, 16).unwrap();
    /// let num_uint = 0x11223344_55667788_9900AABB_CCDDEEFF_u128;
    /// 
    /// let (sum, carry) = num1.carrying_add_uint(num_uint, false);
    /// println!("{} + {} = {}\ncarry = {}", num1, num_uint, sum, carry);
    /// assert_eq!(sum.to_string(), "115761816335569101403435733562708448393664880666628652711615198738168793722605");
    /// assert_eq!(carry, false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let num_str1 = "FFEEDDBB_AA998877_66554433_221100FF_EEDDBBAA_99887766_55443322_1100FFEE";
    /// let num1 = UU32::from_str_radix(num_str1, 16).unwrap();
    /// let num_uint = 0x11223344_55667788_9900AABB_CCDDEEFF_u128;
    /// 
    /// let (sum, carry) = num1.carrying_add_uint(num_uint, true);
    /// println!("{} + {} = {}\ncarry = {}", num1, num_uint, sum, carry);
    /// assert_eq!(sum.to_string(), "115761816335569101403435733562708448393664880666628652711615198738168793722606");
    /// assert_eq!(carry, false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let num_str2 = "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF";
    /// let num2 = UU32::from_str_radix(num_str2, 16).unwrap();
    /// let num_uint = 0x11223344_55667788_9900AABB_CCDDEEFF_u128;
    /// 
    /// let (sum, carry) = num2.carrying_add_uint(num_uint, false);
    /// println!("{} + {} = {}\ncarry = {}", num2, num_uint, sum, carry);
    /// assert_eq!(sum.to_string(), "22774453838368691933710012711845097214");
    /// assert_eq!(carry, true);
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let num_str2 = "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF";
    /// let num2 = UU32::from_str_radix(num_str2, 16).unwrap();
    /// let num_uint = 0x11223344_55667788_9900AABB_CCDDEEFF_u128;
    /// 
    /// let (sum, carry) = num2.carrying_add_uint(num_uint, true);
    /// println!("{} + {} = {}\ncarry = {}", num2, num_uint, sum, carry);
    /// assert_eq!(sum.to_string(), "22774453838368691933710012711845097215");
    /// assert_eq!(carry, true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn carrying_add_uint<U>(&self, rhs: U, carry: bool) -> (Self, bool)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut res = Self::from_array(self.get_number().clone());
        let c = res.carrying_add_assign_uint(rhs, carry);
        (res, c)
    }

///////////////////////////
    // pub fn carrying_add_assign_uint<U>(&mut self, rhs: U, carry: bool) -> bool
    /// Accumulate `rhs` + `carry` to `self`, wrapping around at the boundary
    /// of the type, and return the resulting carry.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Outputs
    /// It returns the output carry. It performs “ternary addition” of a big
    /// integer operands, primitive unsigned integer, and a carry-in bit,
    /// and returns a carry-out bit.
    /// 
    /// # Features
    /// - This allows chaining together multiple additions to create even a
    /// wider addition. This can be thought of as a big integer “full adder”,
    /// in the electronics sense.
    /// - If the input carry is false, this method is equivalent to
    /// `overflowing_add_assign_uint()`, and the output carry reflect current
    /// overflow.
    /// - All the flags are historical, which means, for example, if an overflow
    /// occurred even once before this current operation or `OVERFLOW`
    /// flag is already set before this current operation, the `OVERFLOW` flag
    /// is not changed even if this current operation does not cause overflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method [carrying_add_assign()](struct@BigUInt#method.carrying_add_assign)
    /// is proper rather than this method.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let num_str1 = "FFEEDDBB_AA998877_66554433_221100FF_EEDDBBAA_99887766_55443322_1100FFEE";
    /// let num_str2 = "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF";
    /// let mut num1 = U256::from_str_radix(num_str1, 16).unwrap();
    /// let mut num2 = U256::from_str_radix(num_str2, 16).unwrap();
    /// let num_uint = 0x9900AABB_CCDDEEFF_u64;
    /// 
    /// println!("Originally,\tnum1 = {}", num1);
    /// let mut num3 = num1.clone();
    /// let mut carry = num1.carrying_add_assign_uint(num_uint, false);
    /// println!("After num1 += {},\tnum1 = {}\tcarry = {}", num_uint, num1, carry);
    /// assert_eq!(num1.to_string(), "115761816335569101403435733562708448393642106212790284019692513725068324302573");
    /// assert_eq!(carry, false);
    /// 
    /// num1 = num3;
    /// println!("Originally,\tnum1 = {}", num1);
    /// carry = num1.carrying_add_assign_uint(num_uint, true);
    /// println!("After num1 += {},\tnum1 = {}\tcarry = {}", num_uint, num1, carry);
    /// assert_eq!(num1.to_string(), "115761816335569101403435733562708448393642106212790284019692513725068324302574");
    /// assert_eq!(carry, false);
    /// 
    /// num3 = num2.clone();
    /// println!("Originally,\tnum2 = {}", num2);
    /// carry = num2.carrying_add_assign_uint(num_uint, false);
    /// println!("After num2 += {},\tnum2 = {}\tcarry = {}", num_uint, num2, carry);
    /// assert_eq!(num2.to_string(), "11024999611375677182");
    /// assert_eq!(carry, true);
    /// 
    /// num2 = num3;
    /// println!("Originally,\tnum2 = {}", num2);
    /// carry = num2.carrying_add_assign_uint(num_uint, true);
    /// println!("After num2 += {},\tnum2 = {}\tcarry = {}", num_uint, num2, carry);
    /// assert_eq!(num2.to_string(), "11024999611375677183");
    /// assert_eq!(carry, true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn carrying_add_assign_uint<U>(&mut self, rhs: U, carry: bool) -> bool
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if U::size_in_bytes() > T::size_in_bytes()
        {
            return self.carrying_add_assign(&Self::from_uint(rhs), carry);
        }

        // if rhs.length_in_bytes() <= T::size_in_bytes()
        let zero = T::zero();
        let (mut num, mut c) = self.get_num_(0).carrying_add(T::num::<U>(rhs), carry);
        self.set_num_(0, num);
        if c
        {
            for i in 1..N
            {
                (num, c) = self.get_num_(i).carrying_add(zero, c);
                self.set_num_(i, num);
                if !c
                    { break; }
            }
            if c
                { self.set_overflow(); }
        }
        c
    }

    // pub fn wrapping_add_uint<U>(&self, rhs: U) -> Self
    /// Computes `self` + `rhs`, wrapping around at the boundary of the type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `self` + `rhs` with wrapping (modular) addition.
    /// 
    /// # Features
    /// Wrapping (modular) addition.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method [wrapping_add()](struct@BigUInt#method.wrapping_add)
    /// is proper rather than this method.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a = U512::max().wrapping_sub_uint(1_u8);
    /// let b = a.wrapping_add_uint(1_u8);
    /// let c = a.wrapping_add_uint(2_u8);
    /// let d = a.wrapping_add_uint(3_u8);
    /// 
    /// println!("{} + 1 = {}", a, b);
    /// assert_eq!(b.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    /// 
    /// println!("{} + 2 = {}", a, c);
    /// assert_eq!(c.to_string(), "0");
    /// 
    /// println!("{} + 3 = {}", a, d);
    /// assert_eq!(d.to_string(), "1");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn wrapping_add_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if U::size_in_bytes() > T::size_in_bytes()
        {
            return self.wrapping_add(&Self::from_uint(rhs));
        }
        // if U::size_in_bytes() <= T::size_in_bytes()
        let (res, _) = self.carrying_add_uint(rhs, false);
        res
    }

    // pub fn wrapping_add_assign_uint<U>(&mut self, rhs: U)
    /// Computes `self` + `rhs`, wrapping around at the boundary of the type,
    /// and assign the result to `self` back.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - Wrapping (modular) addition.
    /// - All the flags are historical, which means, for example, if an overflow
    /// occurred even once before this current operation or `OVERFLOW`
    /// flag is already set before this current operation, the `OVERFLOW` flag
    /// is not changed even if this current operation does not cause overflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method [wrapping_add_assign()](struct@BigUInt#method.wrapping_add_assign)
    /// is proper rather than this method.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a = UU64::max().wrapping_sub_uint(1_u8);
    /// println!("Originally,\ta = {}", a);
    /// assert_eq!(a.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    /// 
    /// a.wrapping_add_assign_uint(1_u8);
    /// println!("After a += 1,\ta = {}", a);
    /// assert_eq!(a.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    /// 
    /// a.wrapping_add_assign_uint(1_u8);
    /// println!("After a += 1,\ta = {}", a);
    /// assert_eq!(a.to_string(), "0");
    /// 
    /// a.wrapping_add_assign_uint(1_u8);
    /// println!("After a += 1,\ta = {}", a);
    /// assert_eq!(a.to_string(), "1");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn wrapping_add_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
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
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns a tuple of the addition `self` + `rhs` along with a boolean
    /// indicating whether an arithmetic overflow would occur. If an overflow
    /// would have occurred then the wrapped (modular) value is returned.
    /// 
    /// # Features
    /// The output overflow reflects current overflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method [overflowing_add()](struct@BigUInt#method.overflowing_add)
    /// is proper rather than this method.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a = U512::max().wrapping_sub_uint(1_u8);
    /// let (b, overflow) = a.overflowing_add_uint(1_u8);
    /// println!("{} + 1 = {}\noverflow = {}", a, b, overflow);
    /// assert_eq!(b.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    /// assert_eq!(overflow, false);
    /// 
    /// let (c, overflow) = a.overflowing_add_uint(2_u8);
    /// println!("{} + 2 = {}\noverflow = {}", a, c, overflow);
    /// assert_eq!(c.to_string(), "0");
    /// assert_eq!(overflow, true);
    /// 
    /// let (d, overflow) = a.overflowing_add_uint(3_u8);
    /// println!("{} + 3 = {}\noverflow = {}", a, d, overflow);
    /// assert_eq!(d.to_string(), "1");
    /// assert_eq!(overflow, true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn overflowing_add_uint<U>(&self, rhs: U) -> (Self, bool)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut res = Self::from_array(self.get_number().clone());
        let current_overflow = res.overflowing_add_assign_uint(rhs);
        (res, current_overflow)
    }

    // pub fn overflowing_add_assign_uint<U>(&mut self, rhs: U) -> bool
    /// Calculates `self` + `rhs`, and assigns the result to `self` back.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns true if an arithmetic overflow would occur.
    /// Otherwise, it returns `false`.
    /// 
    /// # Features
    /// - The output overflow reflects current overflow.
    /// - All the flags are historical, which means, for example, if an overflow
    /// occurred even once before this current operation or `OVERFLOW`
    /// flag is already set before this current operation, the `OVERFLOW` flag
    /// is not changed even if this current operation does not cause overflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method [overflowing_add_assign()](struct@BigUInt#method.overflowing_add_assign)
    /// is proper rather than this method.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a = UU64::max().wrapping_sub_uint(1_u8);
    /// println!("Originally,\ta = {}", a);
    /// assert_eq!(a.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    /// 
    /// let mut overflow = a.overflowing_add_assign_uint(1_u8);
    /// println!("After a += 1,\ta = {}\noverflow = {}", a, overflow);
    /// assert_eq!(a.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    /// assert_eq!(overflow, false);
    /// 
    /// overflow = a.overflowing_add_assign_uint(1_u8);
    /// println!("After a += 1,\ta = {}\noverflow = {}", a, overflow);
    /// assert_eq!(a.to_string(), "0");
    /// assert_eq!(overflow, true);
    /// 
    /// overflow = a.overflowing_add_assign_uint(1_u8);
    /// println!("After a += 1,\ta = {}\noverflow = {}", a, overflow);
    /// assert_eq!(a.to_string(), "1");
    /// assert_eq!(overflow, false);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn overflowing_add_assign_uint<U>(&mut self, rhs: U) -> bool
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let old = self.get_all_flags();
        self.reset_all_flags();
        self.wrapping_add_assign_uint(rhs);
        let current_overflow = self.is_overflow();
        self.set_flag_bit(old);
        current_overflow
    }

    // pub fn checked_add_uint<U>(&self, rhs: U) -> Option<Self>
    /// Computes `self` + `rhs`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the sum `self` + `rhs` wrapped by `Some` of enum `Option`
    /// if overflow did not occur at current operation.
    /// Otherwise, it returns `None` of enum Option.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method [checked_add()](struct@BigUInt#method.checked_add)
    /// is proper rather than this method.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a = U512::max().wrapping_sub_uint(1_u8);
    /// let b = a.checked_add_uint(1_u8);
    /// match b
    /// {
    ///     Some(num) => {
    ///         println!("{} + 1 = {}", a, num);
    ///         assert_eq!(num.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    ///     },
    ///     None => {
    ///         println!("{} + 1 = overflow", a);
    ///     }
    /// }
    /// 
    /// let c = a.checked_add_uint(2_u8);
    /// match c
    /// {
    ///     Some(num) => {
    ///         println!("{} + 2 = {}", a, num);
    ///     },
    ///     None => {
    ///         println!("{} + 2 = overflow", a);
    ///         assert_eq!(c, None);
    ///     }
    /// }
    /// 
    /// let d = a.checked_add_uint(3_u8);
    /// match d
    /// {
    ///     Some(num) => {
    ///         println!("{} + 2 = {}", a, num);
    ///     },
    ///     None => {
    ///         println!("{} + 2 = overflow", a);
    ///         assert_eq!(d, None);
    ///     }
    /// }
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn checked_add_uint<U>(&self, rhs: U) -> Option<Self>
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
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
        let mut res = Self::from_array(self.get_number().clone());
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
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// - If overflow occurred, it will panic. So, use this method
    /// only when you are sure that overflow will not occur.
    /// 
    /// # Output
    /// It returns the sum `self` + `rhs` if overflow did not occur at current
    /// operation. Otherwise, it will panic.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method [unchecked_add()](struct@BigUInt#method.unchecked_add)
    /// is proper rather than this method.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a = UU64::max().wrapping_sub_uint(1_u8);
    /// let b = a.unchecked_add_uint(1_u8);
    /// println!("{} + 1 = {}", a, b);
    /// assert_eq!(b.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    /// 
    /// // It will panic.
    /// // let c = a.unchecked_add_uint(2_u8);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn unchecked_add_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
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
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the sum `self` + `rhs` if overflow did not occur.
    /// Otherwise, it returns the maximum value.
    /// 
    /// # Features
    /// - This method saturates when it reaches maximum value.
    /// - It does not set `OVERFLOW` flag.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method [saturating_add()](struct@BigUInt#method.saturating_add)
    /// is proper rather than this method.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a = U512::max().wrapping_sub_uint(2_u8);
    /// let b = a.saturating_add_uint(1_u8);
    /// let c = a.saturating_add_uint(2_u8);
    /// let d = a.saturating_add_uint(3_u8);
    /// 
    /// println!("{} + 1 = {}", a, b);
    /// assert_eq!(b.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    /// 
    /// println!("{} + 2 = {}", a, c);
    /// assert_eq!(c.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    /// 
    /// println!("{} + 3 = {}", a, d);
    /// assert_eq!(d.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    ///  ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn saturating_add_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut res = Self::from_array(self.get_number().clone());
        res.saturating_add_assign_uint(rhs);
        res
    }

    // pub fn saturating_add_assign_uint<U>(&mut self, rhs: T)
    /// Computes `self` + `rhs`, saturating at the numeric bounds
    /// instead of overflowing, and assigns the result to `self` back.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - This method saturates when it reaches maximum value.
    /// - It does not set `OVERFLOW` flag.
    /// - All the flags are historical, which means, for example, if an overflow
    /// occurred even once before this current operation or `OVERFLOW`
    /// flag is already set before this current operation, the `OVERFLOW` flag
    /// is not changed even if this current operation does not cause overflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method [saturating_add_assign()](struct@BigUInt#method.saturating_add_assign)
    /// is proper rather than this method.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = UU64::max().wrapping_sub_uint(2_u8);
    /// println!("Originally,\ta = {}", a);
    /// assert_eq!(a.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084093");
    /// 
    /// a.saturating_add_assign_uint(1_u8);
    /// println!("After a += 1,\ta = {}", a);
    /// assert_eq!(a.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    /// 
    /// a.saturating_add_assign_uint(1_u8);
    /// println!("After a += 1,\ta = {}", a);
    /// assert_eq!(a.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    /// 
    /// a.saturating_add_assign_uint(1_u8);
    /// println!("After a += 1,\ta = {}", a);
    /// assert_eq!(a.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn saturating_add_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let overflow = self.is_overflow();
        if self.overflowing_add_assign_uint(rhs)
        {
            self.set_max();
            if !overflow
                { self.reset_overflow(); }
        }
    }

    // pub fn modular_add_uint<U>(&self, rhs: U, modulo: &Self) -> Self
    /// Computes (`self` + `rhs`) % `modulo`, wrapping around at `modulo` of
    /// the type `Self` instead of overflowing.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the modulo-sum (`self` + `rhs`) % `modulo` with wrapping
    /// (modular) addition at `modulo`.
    /// 
    /// # Features
    /// - Wrapping (modular) addition at `modulo`.
    /// - The differences between this method `modular_add_uint()` and the
    /// method `wrapping_add_uint()` are, first, where wrapping around happens,
    /// and, second, whether or not `OVERFLOW` flag is set. First, this method
    /// wraps araound at `modulo` while the method `wrapping_add_uint()` wraps
    /// araound at maximum value + 1. Second, this method does not set
    /// `OVERFLOW` flag even if wrapping around happens at `modulo` while the
    /// method `wrapping_add_uint()` sets `OVERFLOW` flag when wrapping around
    /// happens.
    /// - If `modulo` is `zero`, the flags such as `OVERFLOW`, `DIVIDED_BY_ZERO`,
    /// and `INFINITY` will be set.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method [modular_add()](struct@BigUInt#method.modular_add)
    /// is proper rather than this method `modular_add_uint()`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let m = a.wrapping_add_uint(2_u8);
    /// let b = a.modular_add_uint(1_u8, &m);
    /// let c = a.modular_add_uint(2_u8, &m);
    /// let d = a.modular_add_uint(3_u8, &m);
    /// 
    /// println!("{} + 1 = {}", a, b);
    /// assert_eq!(b.to_string(), "76801874298166903427690031858186486050853753882811946569946433649007");
    /// 
    /// println!("{} + 2 = {}", a, c);
    /// assert_eq!(c.to_string(), "0");
    /// 
    /// println!("{} + 3 = {}", a, d);
    /// assert_eq!(d.to_string(), "1");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn modular_add_uint<U>(&self, rhs: U, modulo: &Self) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd

    {
        let mut res = Self::from_array(self.get_number().clone());
        res.modular_add_assign_uint(rhs, modulo);
        res
    }

    // pub fn modular_add_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    /// Computes (`self` + `rhs`) % `modulo`, wrapping around at `modulo`
    /// of the type `Self` instead of overflowing, and then, assign the result
    /// back to `self`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - Wrapping (modular) addition at `modulo`.
    /// - The differences between this method `modular_add_assign_uint()` and
    /// the method `wrapping_add_assign_uint()` are, first, where wrapping
    /// around happens, and, second, whether or not `OVERFLOW` flag is set.
    /// First, this method `modular_add_assign_uint()` wraps araound at `modulo`
    /// while the method `wrapping_add_assign_uint()` wraps araound at maximum
    /// value + 1. Second, this method `modular_add_assign_uint()` does not set
    /// `OVERFLOW` flag even if wrapping around happens at `modulo` while the
    /// method `wrapping_add_assign_uint()` sets `OVERFLOW` flag when wrapping
    /// around happens.
    /// - If `modulo` is `zero`, the flags such as `OVERFLOW`, `DIVIDED_BY_ZERO`,
    /// and `INFINITY` will be set.
    /// - All the flags are historical, which means, for example, if an overflow
    /// occurred even once before this current operation or `OVERFLOW`
    /// flag is already set before this current operation, the `OVERFLOW` flag
    /// is not changed even if this current operation does not cause overflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [modular_add_assign()](struct@BigUInt#method.modular_add_assign)
    /// is proper rather than this method.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a = U256::from_string("768018742981669034276900318581864860508537538828119465699464336490060").unwrap();
    /// let m = a.wrapping_add_uint(2_u8);
    /// println!("Originally,\ta = {}", a);
    /// assert_eq!(a.to_string(), "768018742981669034276900318581864860508537538828119465699464336490060");
    /// 
    /// a.modular_add_assign_uint(1_u8, &m);
    /// println!("After a += 1,\ta = {}", a);
    /// assert_eq!(a.to_string(), "768018742981669034276900318581864860508537538828119465699464336490061");
    /// 
    /// a.modular_add_assign_uint(1_u8, &m);
    /// println!("After a += 1,\ta = {}", a);
    /// assert_eq!(a.to_string(), "0");
    /// 
    /// a.modular_add_assign_uint(1_u8, &m);
    /// println!("After a += 1,\ta = {}", a);
    /// assert_eq!(a.to_string(), "1");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn modular_add_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if modulo.is_zero()
        {
            self.set_max();
            self.set_overflow();
            self.set_divided_by_zero();
            self.set_infinity();
        }
        else if *self >= *modulo
        {
            self.wrapping_rem_assign(modulo);
        }
        if modulo.gt_uint(rhs)
        {
            let diff = modulo.wrapping_sub_uint(rhs);
            if *self >= diff
                { self.wrapping_sub_assign(&diff); }
            else
                { self.wrapping_add_assign_uint(rhs); }
        }
        else if rhs.length_in_bytes() > T::size_in_bytes()  // && (module <= rhs)
        {
            self.modular_add_assign(&Self::from_uint(rhs), modulo);
        }
        else    // if (U::size_in_bytes() <= T::size_in_bytes()) && (module <= rhs)
        {
            let mrhs = rhs.into_u128();
            let modu = modulo.into_u128();
            let mut mself = self.into_u128();
            mself = mself.modular_add(mrhs, modu);
            self.set_uint(mself);
        }
    }


    /*** Subtraction ***/

    // pub fn borrowing_sub_uint<U>(&self, rhs: U, borrow: bool) -> (Self, bool)
    /// Calculates self − rhs − borrow and returns a tuple containing the
    /// difference and the output borrow.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Outputs
    /// It returns a tuple containing an output big integer and a carry-out bit.
    /// 
    /// # Features
    /// - It performs “ternary subtraction” by subtracting a primitive unsigned
    /// integer operand and a borrow-in bit from `self`, and returns an output
    /// integer and a borrow-out bit. This allows chaining together multiple
    /// subtractions to create a wider subtraction.
    /// - If the input borrow is `false`, this method is equivalent to
    /// `overflowing_sub_uint()`, and the output carry is equal to
    /// the `UNDERFLOW` flag.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method [borrowing_sub()](struct@BigUInt#method.borrowing_sub)
    /// is proper rather than this method `borrowing_sub_uint()`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let num_str1 = "FFEEDDBB_AA998877_66554433_221100FF_EEDDBBAA_99887766_55443322_1100FFEE";
    /// let num_str2 = "11223344_55667788_9900AABB_CCDDEEEe";
    /// let num1 = UU32::from_str_radix(num_str1, 16).unwrap();
    /// let num2 = UU32::from_str_radix(num_str2, 16).unwrap();
    /// let num_uint = 0x11223344_55667788_9900AABB_CCDDEEFf_u128;
    /// 
    /// let (mut dif, mut carry) = num1.borrowing_sub_uint(num_uint, false);
    /// println!("{} - {} = {}\ncarry = {}", num1, num_uint, dif, carry);
    /// assert_eq!(dif.to_string(), "115761816335569101403435733562708448393619331758951915327747778712745103528175");
    /// assert_eq!(carry, false);
    /// 
    /// (dif, carry) = num1.borrowing_sub_uint(num_uint, true);
    /// println!("{} - {} = {}\ncarry = {}", num1, num_uint, dif, carry);
    /// assert_eq!(dif.to_string(), "115761816335569101403435733562708448393619331758951915327747778712745103528174");
    /// assert_eq!(carry, false);
    /// 
    /// (dif, carry) = num2.borrowing_sub_uint(num_uint, false);
    /// println!("{} - {} = {}\ncarry = {}", num2, num_uint, dif, carry);
    /// assert_eq!(dif.to_string(), "115792089237316195423570985008687907853269984665640564039457584007913129639919");
    /// assert_eq!(carry, true);
    /// 
    /// (dif, carry) = num2.borrowing_sub_uint(num_uint, true);
    /// println!("{} - {} = {}\ncarry = {}", num2, num_uint, dif, carry);
    /// assert_eq!(dif.to_string(), "115792089237316195423570985008687907853269984665640564039457584007913129639918");
    /// assert_eq!(carry, true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn borrowing_sub_uint<U>(&self, rhs: U, borrow: bool) -> (Self, bool)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut res = Self::from_array(self.get_number().clone());
        let b = res.borrowing_sub_assign_uint(rhs, borrow);
        (res, b)
    }

    // pub fn borrowing_sub_assign_uint<U>(&mut self, rhs: U, borrow: bool) -> bool
    /// Calculates self − rhs − borrow, and assigns difference to `self` back,
    /// and returns the output borrow.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Outputs
    /// It returns a tuple containing an output big integer and a carry-out bit.
    /// 
    /// # Features
    /// -It performs “ternary subtraction” by subtracting an primitive unsiged
    /// integer operand and a borrow-in bit from `self`, and a borrow-out bit.
    /// This allows chaining together multiple subtractions to create a wider
    /// subtraction.
    /// - If the input borrow is `false`, this method is equivalent to
    /// `overflowing_sub_assign_uint()`, and the output carry reflect current
    /// underflow.
    /// - All the flags are historical, which means, for example, if an
    /// underflow occurred even once before this current operation or
    /// `UNDERFLOW` flag is already set before this current operation,
    /// the `UNDERFLOW` flag is not changed even if this current operation
    /// does not cause overflow.
    /// - All the flags are historical, which means, for example, if an
    /// overflow occurred even once before this current operation or
    /// `OVERFLOW` flag is already set before this current operation,
    /// the `OVERFLOW` flag is not changed even if this current operation
    /// does not cause overflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method [borrowing_sub_assign()](struct@BigUInt#method.borrowing_sub_assign)
    /// is proper rather than this method `borrowing_sub_assign_uint()`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let num_str1 = "FFEEDDBB_AA998877_66554433_221100FF_EEDDBBAA_99887766_55443322_1100FFEE";
    /// let num_str2 = "9900AABB_CCDDEEFe";
    /// let mut num1 = U256::from_str_radix(num_str1, 16).unwrap();
    /// let mut num2 = U256::from_str_radix(num_str2, 16).unwrap();
    /// let num_uint = 0x9900AABB_CCDDEEFf_u64;
    /// 
    /// println!("Originally,\tnum1 = {}", num1);
    /// let mut num3 = num1.clone();
    /// let mut carry = num1.borrowing_sub_assign_uint(num_uint, false);
    /// println!("After num1 -= {},\tnum1 = {}\tcarry = {}", num_uint, num1, carry);
    /// assert_eq!(num1.to_string(), "115761816335569101403435733562708448393642106212790284019670463725845572948207");
    /// assert_eq!(carry, false);
    /// 
    /// num1 = num3;
    /// println!("Originally,\tnum1 = {}", num1);
    /// carry = num1.borrowing_sub_assign_uint(num_uint, true);
    /// println!("After num1 -= {},\tnum1 = {}\tcarry = {}", num_uint, num1, carry);
    /// assert_eq!(num1.to_string(), "115761816335569101403435733562708448393642106212790284019670463725845572948206");
    /// assert_eq!(carry, false);
    /// 
    /// num3 = num2.clone();
    /// println!("Originally,\tnum2 = {}", num2);
    /// carry = num2.borrowing_sub_assign_uint(num_uint, false);
    /// println!("After num2 -= {},\tnum2 = {}\tcarry = {}", num_uint, num2, carry);
    /// assert_eq!(num2.to_string(), "115792089237316195423570985008687907853269984665640564039457584007913129639935");
    /// assert_eq!(carry, true);
    /// 
    /// num2 = num3;
    /// println!("Originally,\tnum2 = {}", num2);
    /// carry = num2.borrowing_sub_assign_uint(num_uint, true);
    /// println!("After num2 -= {},\tnum2 = {}\tcarry = {}", num_uint, num2, carry);
    /// assert_eq!(num2.to_string(), "115792089237316195423570985008687907853269984665640564039457584007913129639934");
    /// assert_eq!(carry, true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn borrowing_sub_assign_uint<U>(&mut self, rhs: U, borrow: bool) -> bool
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if U::size_in_bytes() > T::size_in_bytes()
        {
            return self.borrowing_sub_assign(&Self::from_uint(rhs), borrow);
        }
        // if U::size_in_bytes() <= T::size_in_bytes()
        let (mut num, mut b) = self.get_num_(0).borrowing_sub(T::num::<U>(rhs), borrow);
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
            if b
                { self.set_underflow(); }
        }
        b
    }

    // pub fn wrapping_sub_uint<U>(&self, rhs: U) -> Self
    /// Subtracts a unsigned integer number of type `U` from `BigUInt`-type
    /// unsigned integer and returns its result in a type of `BigUInt`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the subtraction of `rhs` from `self`.
    /// 
    /// # Features
    /// - Wrapping (modular) subtraction.
    /// - It computes `self`` - `rhs`, wrapping around at the boundary
    /// of the type.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method [wrapping_sub()](struct@BigUInt#method.wrapping_sub)
    /// is proper rather than this method `wrapping_sub_uint()`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a = U512::one();
    /// let b = a.wrapping_sub_uint(1_u8);
    /// let c = a.wrapping_sub_uint(2_u8);
    /// let d = a.wrapping_sub_uint(3_u8);
    /// 
    /// println!("{} - 1 = {}", a, b);
    /// assert_eq!(b.to_string(), "0");
    /// 
    /// println!("{} - 2 = {}", a, c);
    /// assert_eq!(c.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    /// 
    /// println!("{} - 3 = {}", a, d);
    /// assert_eq!(d.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn wrapping_sub_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
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
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - Wrapping (modular) subtraction.
    /// - It computes `self`` - `rhs`, wrapping around at the boundary
    /// of the type.
    /// - All the flags are historical, which means, for example, if an
    /// underflow occurred even once before this current operation or
    /// `UNDERFLOW` flag is already set before this current operation,
    /// the `UNDERFLOW` flag is not changed even if this current operation
    /// does not cause underflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [wrapping_sub_assign()](struct@BigUInt#method.wrapping_sub_assign)
    /// is proper rather than this method `wrapping_sub_assign_uint()`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a = UU64::one();
    /// println!("Originally,\ta = {}", a);
    /// assert_eq!(a.to_string(), "1");
    /// 
    /// a.wrapping_sub_assign_uint(1_u8);
    /// println!("After a -= 1,\ta = {}", a);
    /// assert_eq!(a.to_string(), "0");
    /// 
    /// a.wrapping_sub_assign_uint(1_u8);
    /// println!("After a -= 1,\ta = {}", a);
    /// assert_eq!(a.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    /// 
    /// a.wrapping_sub_assign_uint(1_u8);
    /// println!("After a -= 1,\ta = {}", a);
    /// assert_eq!(a.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn wrapping_sub_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
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
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns a tuple of the subtraction `self` - `rhs` along with a
    /// boolean indicating whether an arithmetic unerflow would occur. If an
    /// unerflow would have occurred then the wrapped (modular) value is
    /// returned.
    /// 
    /// # Features
    /// - It returns a tuple of the subtraction along with a boolean indicating
    /// whether an arithmetic overflow would occur.
    /// - If an overflow would have occurred then the wrapped value is returned.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [overflowing_sub()](struct@BigUInt#method.overflowing_sub) is proper
    /// rather than this method `overflowing_sub_uint()`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a = U512::one();
    /// let (b, underflow) = a.overflowing_sub_uint(1_u8);
    /// println!("{} - 1 = {}\nunderflow = {}", a, b, underflow);
    /// assert_eq!(b.to_string(), "0");
    /// assert_eq!(underflow, false);
    /// 
    /// let (c, underflow) = a.overflowing_sub_uint(2_u8);
    /// println!("{} - 2 = {}\nunderflow = {}", a, c, underflow);
    /// assert_eq!(c.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    /// assert_eq!(underflow, true);
    /// 
    /// let (d, underflow) = a.overflowing_sub_uint(3_u8);
    /// println!("{} - 3 = {}\nunderflow = {}", a, d, underflow);
    /// assert_eq!(d.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    /// assert_eq!(underflow, true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn overflowing_sub_uint<U>(&self, rhs: U) -> (Self, bool)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut res = Self::from_array(self.get_number().clone());
        let current_underflow = res.overflowing_sub_assign_uint(rhs);
        (res, current_underflow)
    }

    // pub fn overflowing_sub_assign<U>(&mut self, rhs: U) -> bool
    /// Calculates `self` - `rhs`, and assigns the result to `self` back.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns true if an arithmetic unerflow would occur.
    /// Otherwise, it returns `false`.
    /// 
    /// # Features
    /// - If an overflow would have occurred then the wrapped value is returned
    /// back to `self`.
    /// - All the flags are historical, which means, for example, if an
    /// underflow occurred even once before this current operation or
    /// `UNDERFLOW` flag is already set before this current operation,
    /// the `UNDERFLOW` flag is not changed even if this current operation
    /// does not cause underflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [overflowing_sub_assign()](struct@BigUInt#method.overflowing_sub_assign)
    /// is proper rather than this method `overflowing_sub_assign_uint()`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a = UU64::one();
    /// println!("Originally,\ta = {}", a);
    /// assert_eq!(a.to_string(), "1");
    /// 
    /// let mut underflow = a.overflowing_sub_assign_uint(1_u8);
    /// println!("After a -= 1,\ta = {}\nunderflow = {}", a, underflow);
    /// assert_eq!(a.to_string(), "0");
    /// assert_eq!(underflow, false);
    /// 
    /// underflow = a.overflowing_sub_assign_uint(1_u8);
    /// println!("After a -= 1,\ta = {}\nunderflow = {}", a, underflow);
    /// assert_eq!(a.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    /// assert_eq!(underflow, true);
    /// 
    /// underflow = a.overflowing_sub_assign_uint(1_u8);
    /// println!("After a -= 1,\ta = {}\nunderflow = {}", a, underflow);
    /// assert_eq!(a.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    /// assert_eq!(underflow, false);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn overflowing_sub_assign_uint<U>(&mut self, rhs: U) -> bool
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let old = self.get_all_flags();
        self.reset_all_flags();
        self.wrapping_sub_assign_uint(rhs);
        let current_underflow = self.is_underflow();
        self.set_flag_bit(old);
        current_underflow
    }

    // pub fn checked_sub_uint<U>(&self, rhs: U) -> Option<Self>
    /// Computes `self` - `rhs`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the difference `self` - `rhs` wrapped by `Some`
    /// of enum `Option` if unerflow did not occur.
    /// Otherwise, it returns `None` of enum Option.
    /// 
    /// # Features
    /// - Checked integer subtraction.
    /// - It computes `self` - `rhs, returning None if underflow occurred.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [checked_sub()](struct@BigUInt#method.checked_sub) is proper
    /// rather than this method `checked_sub_uint()`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a = U512::one();
    /// let b = a.checked_sub_uint(1_u8);
    /// match b
    /// {
    ///     Some(num) => {
    ///         println!("{} - 1 = {}", a, num);
    ///         assert_eq!(num.to_string(), "0");
    ///     },
    ///     None => {
    ///         println!("{} - 1 = overflow", a);
    ///     }
    /// }
    /// 
    /// let c = a.checked_sub_uint(2_u8);
    /// match c
    /// {
    ///     Some(num) => {
    ///         println!("{} - 2 = {}", a, num);
    ///     },
    ///     None => {
    ///         println!("{} - 2 = overflow", a);
    ///         assert_eq!(c, None);
    ///     }
    /// }
    /// 
    /// let d = a.checked_sub_uint(3_u8);
    /// match d
    /// {
    ///     Some(num) => {
    ///         println!("{} - 3 = {}", a, num);
    ///     },
    ///     None => {
    ///         println!("{} - 3 = overflow", a);
    ///         assert_eq!(d, None);
    ///     }
    /// }
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn checked_sub_uint<U>(&self, rhs: U) -> Option<Self>
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut res = Self::from_array(self.get_number().clone());
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
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// - If underflow occurred, it will panic. So, use this method only when
    /// you are sure that underflow will not occur.
    /// 
    /// # Output
    /// It returns the difference `self` - `rhs` if underflow did not occur.
    /// Otherwise, it will panic.
    /// 
    /// # Features
    /// - Unchecked integer subtraction.
    /// - It computes `self` - `rhs`, assuming overflow cannot occur.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [unchecked_sub()](struct@BigUInt#method.unchecked_sub) is proper
    /// rather than this method `unchecked_sub_uint()`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a = UU64::one();
    /// let b = a.unchecked_sub_uint(1_u8);
    /// println!("{} - 1 = {}", a, b);
    /// assert_eq!(b.to_string(), "0");
    /// // It will panic.
    /// // let c = a.unchecked_add_uint(2_u8);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn unchecked_sub_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
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
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the difference `self` - `rhs` if underflowing did not occur.
    /// Otherwise, it returns `0`.
    /// 
    /// # Features
    /// - Saturating integer subtraction.
    /// - This method saturates when it reaches `zero`.
    /// - It does not set `UNDERFLOW` flag.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [saturating_sub()](struct@BigUInt#method.saturating_sub) is proper
    /// rather than this method `saturating_sub_uint()`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a = U512::zero().wrapping_add_uint(2_u8);
    /// let b = a.saturating_sub_uint(1_u8);
    /// let c = a.saturating_sub_uint(2_u8);
    /// let d = a.saturating_sub_uint(3_u8);
    /// 
    /// println!("{} - 1 = {}", a, b);
    /// assert_eq!(b.to_string(), "1");
    /// 
    /// println!("{} - 2 = {}", a, c);
    /// assert_eq!(c.to_string(), "0");
    /// 
    /// println!("{} - 3 = {}", a, d);
    /// assert_eq!(d.to_string(), "0");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn saturating_sub_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut res = Self::from_array(self.get_number().clone());
        res.saturating_sub_assign_uint(rhs);
        res
    }

    // pub fn saturating_sub_assign_uint<U>(&mut self, rhs: U)
    /// Computes `self` - `rhs`, saturating at the numeric bounds
    /// instead of underflowing, and assigns the result to `self` back.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - `self` will be the difference `self` - `rhs` if underflowing
    /// did not occur. Otherwise, it returns `0`.
    /// - This method saturates when it reaches `zero`.
    /// - It does not set `UNDERFLOW` flag.
    /// - All the flags are historical, which means, for example, if an
    /// underflow occurred even once before this current operation or
    /// `UNDERFLOW` flag is already set before this current operation,
    /// the `UNDERFLOW` flag is not changed even if this current operation
    /// does not cause underflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [saturating_sub_assign()](struct@BigUInt#method.saturating_sub_assign)
    /// is proper rather than this method `saturating_sub_assign_uint()`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = UU64::zero().wrapping_add_uint(2_u8);
    /// println!("Originally,\ta = {}", a);
    /// assert_eq!(a.to_string(), "2");
    /// 
    /// a.saturating_sub_assign_uint(1_u8);
    /// println!("After a -= 1,\ta = {}", a);
    /// assert_eq!(a.to_string(), "1");
    /// 
    /// a.saturating_sub_assign_uint(1_u8);
    /// println!("After a -= 1,\ta = {}", a);
    /// assert_eq!(a.to_string(), "0");
    /// 
    /// a.saturating_sub_assign_uint(1_u8);
    /// println!("After a -= 1,\ta = {}", a);
    /// assert_eq!(a.to_string(), "0");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn saturating_sub_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let underflow = self.is_underflow();
        if self.overflowing_sub_assign_uint(rhs)
        {
            self.set_zero();
            if !underflow
                { self.reset_underflow(); }
        }
    }

    // pub fn modular_sub_uint<U>(&self, rhs: U, modulo: &Self) -> Self
    /// Computes (`self` - `rhs`) % `modulo`, wrapping around at `modulo` of the
    /// type `Self` instead of underflowing.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the modulo-difference (`self` - `rhs`) % `modulo` with
    /// wrapping (modular) subtraction at `modulo`.
    /// 
    /// # Features
    /// - Wrapping (modular) subtraction at `modulo`.
    /// - The differences between this method `modular_sub_uint()` and the
    /// method `wrapping_sub_uint()` are, first, where wrapping around happens,
    /// and, second, whether or not `UNDERFLOW` flag is set. First, this method
    /// wraps araound at `modulo` while the method `wrapping_sub_uint()` wraps
    /// araound at maximum value. Second, this method does not set `UNDERFLOW`
    /// flag even if wrapping around happens while the method
    /// `wrapping_sub_uint()` sets `UNDERFLOW` flag when wrapping around
    /// happens.
    /// - If `modulo` is `zero`, the flags such as `OVERFLOW`, `DIVIDED_BY_ZERO`,
    /// and `INFINITY` will be set.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [modular_sub()](struct@BigUInt#method.modular_sub) is
    /// proper rather than this method `modular_sub_uint()`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let m = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let a = U256::from_uint(2_u8);
    /// let b = a.modular_sub_uint(1_u8, &m);
    /// let c = a.modular_sub_uint(2_u8, &m);
    /// let d = a.modular_sub_uint(3_u8, &m);
    /// 
    /// println!("{} - 1 = {}", a, b);
    /// assert_eq!(b.to_string(), "1");
    /// 
    /// println!("{} - 2 = {}", a, c);
    /// assert_eq!(c.to_string(), "0");
    /// 
    /// println!("{} - 3 = {}", a, d);
    /// assert_eq!(d.to_string(), "76801874298166903427690031858186486050853753882811946569946433649006084093");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn modular_sub_uint<U>(&self, rhs: U, modulo: &Self) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut res = Self::from_array(self.get_number().clone());
        res.modular_sub_assign_uint(rhs, modulo);
        res
    }

    // pub fn modular_sub_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    /// Computes (`self` - `rhs`) % `modulo`, wrapping around at `modulo`
    /// of the type `Self` instead of underflowing, and then, assign the result
    /// back to `self`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - Wrapping (modular) subtraction at `modulo`.
    /// - The differences between this method `modular_sub_assign()` and the
    /// method `wrapping_sub_assign()` are, first, where wrapping around
    /// happens, and, second, whether or not `UNDERFLOW` flag is set. First,
    /// this method wraps araound at `modulo` while the method
    /// `wrapping_sub_assign()` wraps araound at maximum value. Second, this
    /// method does not set `UNDERFLOW` flag even if wrapping around happens,
    /// while the method `wrapping_sub_assign()` sets `UNDERFLOW` flag when
    /// wrapping around happens.
    /// - If `modulo` is `zero`, the flags such as `OVERFLOW`, `DIVIDED_BY_ZERO`,
    /// and `INFINITY` will be set.
    /// - All the flags are historical, which means, for example, if an
    /// underflow occurred even once before this current operation or
    /// `UNDERFLOW` flag is already set before this current operation,
    /// the `UNDERFLOW` flag is not changed even if this current operation
    /// does not cause underflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128, the method
    /// [modular_sub_assign()](struct@BigUInt#method.modular_sub_assign) is
    /// proper rather than this method `modular_sub_assign_uint()`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let mut a = UU32::from_uint(2_u8);
    /// 
    /// println!("Originally,\ta = {}", a);
    /// assert_eq!(a.to_string(), "2");
    /// 
    /// a.modular_sub_assign_uint(1_u8, &m);
    /// println!("After a -= 1,\ta = {}", a);
    /// assert_eq!(a.to_string(), "1");
    /// 
    /// a.modular_sub_assign_uint(1_u8, &m);
    /// println!("After a -= 1,\ta = {}", a);
    /// assert_eq!(a.to_string(), "0");
    /// 
    /// a.modular_sub_assign_uint(1_u8, &m);
    /// println!("After a -= 1,\ta = {}", a);
    /// assert_eq!(a.to_string(), "76801874298166903427690031858186486050853753882811946569946433649006084093");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn modular_sub_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if *self >= *modulo
            { self.wrapping_rem_assign(modulo); }
        if modulo.gt_uint(rhs)
        {
            if self.ge_uint(rhs)    // if *self >= rhs
            {
                self.wrapping_sub_assign_uint(rhs);
            }
            else    // if *self < rhs
            {
                let diff = modulo.wrapping_sub_uint(rhs);
                self.wrapping_add_assign(&diff);
            }
        }
        else if rhs.length_in_bytes() > T::size_in_bytes()  // && (module <= rhs)
        {
            self.modular_sub_assign(&Self::from_uint(rhs), modulo);
        }
        else    // if (U::size_in_bytes() <= T::size_in_bytes()) && (module <= rhs)
        {
            let modu = modulo.into_u128();
            let mrhs = rhs.into_u128().wrapping_rem(modu);
            let mut mself = self.into_u128();
            mself = mself.modular_sub(mrhs, modu);
            self.set_uint(mself);
        }
    }

    // pub fn abs_diff_uint<U>(&self, other: U) -> Self
    /// Computes the absolute difference between `self` and `other`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the absolute difference between `self` and `other`.
    /// 
    /// # Features
    /// It computes the absolute difference between `self` and other.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [abs_diff()](struct@BigUInt#method.abs_diff) is proper rather than
    /// this method `abs_diff_uint()`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let num_str1 = "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF";
    /// let num_str2 = "12345678_9ABCDEF0_12345678_9ABCDEF0";
    /// let num_str3 = "9900AABB_CCDDEEFF_9900AABB_CCDDEEFF";
    /// let num1 = U256::from_str_radix(num_str1, 16).unwrap();
    /// let num2 = U256::from_str_radix(num_str2, 16).unwrap();
    /// let num3 = U256::from_str_radix(num_str3, 16).unwrap();
    /// let num_uint = 0x9900AABB_CCDDEEFF_9900AABB_CCDDEEFF_u128;
    /// 
    /// let a = num1.abs_diff_uint(num_uint);
    /// let b = num2.abs_diff_uint(num_uint);
    /// let c = num3.abs_diff_uint(num_uint);
    /// 
    /// println!("| {} - {} | = {}", num1, num_uint, a);
    /// assert_eq!(a.to_string(), "115792089237316195423570985008687907853066609319396769656704041438214461985024");
    /// 
    /// println!("| {} - {} | = {}", num2, num_uint, b);
    /// assert_eq!(b.to_string(), "179177489040527647888749252028162707471");
    /// 
    /// println!("| {} - {} | = {}", num3, num_uint, c);
    /// assert_eq!(c.to_string(), "0");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn abs_diff_uint<U>(&self, other: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if U::size_in_bytes() > T::size_in_bytes()
        {
            self.abs_diff(&Self::from_uint(other))
        }
        else // if rhs.length_in_bytes() <= T::size_in_bytes()
        {
            let t_other = T::num::<U>(other);
            if self.lt_uint(t_other)
                { Self::from_uint(t_other - self.get_num_(0)) }
            else
                { self.wrapping_sub_uint(t_other) }
        }
    }


    /*** Multiplication ***/

    // pub fn carrying_mul_uint<U>(&self, rhs: U, carry: Self) -> (Self, Self)
    /// Calculates the “full multiplication” `self` * `rhs` + `carry`,
    /// without the possibility to overflow.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `self` * `rhs` + `carry` in the form of a tuple of the
    /// low-order (wrapping) bits and the high-order (overflow) bits of the
    /// result as two separate values, in that order, that is, (`low`, `high`).
    /// 
    /// # Features
    /// - It performs “long multiplication” which takes in an extra amount to
    /// add, and may return an additional amount of overflow. This allows for
    /// chaining together multiple multiplications to create “bigger integers”
    /// which represent larger values.
    /// - If the high-order part of the return value is not zero, the
    /// `OVERFOLOW` flag will be set though the output tuple is free from
    /// overflow. It is because the `OVERFOLOW` flag is about `self`,
    /// and not about the result of multiplication.
    /// 
    /// # Counterpart Methods
    /// - If you don’t need the carry, then you can use
    /// [widening_mul_uint()](struct@BigUInt#method.widening_mul_uint) instead.
    /// - The value of the first field in the returned tuple matches what you’d
    /// get by combining the methods
    /// [wrapping_mul_uint()](struct@BigUInt#method.wrapping_mul_uint) and
    /// [wrapping_add_uint()](struct@BigUInt#method.wrapping_add_uint):
    /// `self.wrapping_mul_uint(rhs).wrapping_add_uint(carry)`. So,
    /// `self.carrying_mul_uint(rhs, carry).0`
    /// == `self.wrapping_mul_uint(rhs).wrapping_add_uint(carry)`
    /// - If `rhs` is bigger than `u128`, the method [carrying_mul()](struct@BigUInt#method.carrying_mul)
    /// is proper rather than this method `carrying_mul_uint()`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_low = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let a_high = UU32::from_string("75388281194656994643364900608409476801874298166903427690031858186486050853").unwrap();
    /// let b_uint = 225_u8;
    /// let (mut res_low, mut res_high) = a_low.carrying_mul_uint(b_uint, UU32::zero());
    /// let (mut res_high, mut res_higher) = a_high.carrying_mul_uint(b_uint, res_high);
    /// 
    /// println!("{}:{} X {} = {}:{}:{}", a_high, a_low, b_uint, res_higher, res_high, res_low);
    /// assert_eq!(res_higher.to_string(), "0");
    /// assert_eq!(res_high.to_string(), "16962363268797823794757102636892132280421717087553271230257168091959361441925");
    /// assert_eq!(res_low.to_string(), "17280421717087553271230257168091959361442094623632687978237947571026368921150");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn carrying_mul_uint<U>(&self, rhs: U, carry: Self) -> (Self, Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut low = Self::from_array(self.get_number().clone());
        let high = low.carrying_mul_assign_uint(rhs, carry);
        (low, high)
    }

    // pub fn carrying_mul_assign_uint<U>(&mut self, rhs: U, carry: Self) -> Self
    /// Calculates the “full multiplication” `self` * `rhs` + `carry`,
    /// without the possibility to overflow, and assigs the low-order bits of
    /// the result to `self` back and returns the high-order bits of the result.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the high-order (overflow) bits of `self` * `rhs` + `carry`
    /// of the result.
    /// 
    /// # Features
    /// - It performs “long multiplication” which takes in an extra amount to
    /// add, and may return an additional amount of overflow. This allows for
    /// chaining together multiple multiplications to create “bigger integers”
    /// which represent larger values.
    /// - If the return value is not zero, the `OVERFOLOW` flag will be set
    /// though the output tuple is free from overflow. It is because the
    /// `OVERFOLOW` flag is about `self`, and not about the result of
    /// multiplication.
    /// - All the flags are historical, which means, for example, if an
    /// overflow occurred even once before this current operation or
    /// `OVERFLOW` flag is already set before this current operation,
    /// the `OVERFLOW` flag is not changed even if this current operation
    /// does not cause overflow.
    /// 
    /// # Counterpart Methods
    /// - If you don’t need the carry, then you can use
    /// [widening_mul_assign_uint()](struct@BigUInt#method.widening_mul_assign_uint)
    /// instead.
    /// - The value of `self` after calculation matches what you’d get by
    /// combining the mehtods
    /// [wrapping_mul_uint()](struct@BigUInt#method.wrapping_mul_uint) and
    /// [wrapping_add_assign_uint()](struct@BigUInt#method.wrapping_add_assign_uint):
    /// `self.wrapping_mul_uint(rhs).wrapping_add_assign_uint(carry)`.
    /// - If `rhs` is bigger than `u128`, the method [carrying_mul_assign()](struct@BigUInt#method.carrying_mul_assign)
    /// is proper rather than this method `carrying_mul_assign_uint()`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_low = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let mut a_high = U256::from_string("75388281194656994643364900608409476801874298166903427690031858186486050853").unwrap();
    /// let b_uint = 225_u8;
    /// 
    /// println!("Originally,\ta_low = {}", a_low);
    /// assert_eq!(a_low.to_string(), "76801874298166903427690031858186486050853753882811946569946433649006084094");
    /// println!("Originally,\ta_high = {}\n", a_high);
    /// assert_eq!(a_high.to_string(), "75388281194656994643364900608409476801874298166903427690031858186486050853");
    /// 
    /// let mut res_high = a_low.carrying_mul_assign_uint(b_uint, UU32::zero());
    /// println!("After a_low.carrying_mul_assign_uint(225_u8, 0),\na_low = {}\n", a_low);
    /// assert_eq!(a_low.to_string(), "17280421717087553271230257168091959361442094623632687978237947571026368921150");
    /// 
    /// let mut res_higher = a_high.carrying_mul_assign_uint(b_uint, res_high);
    /// println!("After a_high.carrying_mul_assign_uint(225_u8, res_higher),\na_high = {}\nres_higher = {}", a_high, res_higher);
    /// assert_eq!(a_high.to_string(), "16962363268797823794757102636892132280421717087553271230257168091959361441925");
    /// assert_eq!(res_higher.to_string(), "0");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn carrying_mul_assign_uint<U>(&mut self, rhs: U, carry: Self) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut high = self.widening_mul_assign_uint(rhs);
        if self.overflowing_add_assign(&carry)
            { high.wrapping_add_assign_uint(1_u8); }
        high
    }

    // pub fn widening_mul_uint<U>(&self, rhs: U) -> (Self, Self)
    /// Calculates the complete product `self` * `rhs` without the possibility
    /// to overflow.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `self` * `rhs` in the form of a tuple of the low-order
    /// (wrapping) bits and the high-order (overflow) bits of the result as
    /// two separate values, in that order.
    /// 
    /// # Features
    /// - It performs “long multiplication” which takes in an extra amount to add,
    /// and may return an additional amount of overflow. This allows for
    /// chaining together multiple multiplications to create “bigger integers”
    /// which represent larger values.
    /// - If the high-order part of the return value is not zero, the
    /// `OVERFOLOW` flag will be set though the output tuple is free from
    /// overflow. It is because the `OVERFOLOW` flag is about `self`,
    /// and not about the result of multiplication.
    /// 
    /// # Counterpart Methods
    /// - If you also need to add a carry to the wide result, then you want to
    /// use [carrying_mul_uint()](struct@BigUInt#method.carrying_mul_uint)
    /// instead.
    /// - The value of the first field in the returned tuple matches what
    /// you’d get the method
    /// [wrapping_mul_uint()](struct@BigUInt#method.wrapping_mul_uint).
    /// `self.widening_mul_uint(rhs).0` == `self.wrapping_mul_uint(rhs)`
    /// - If `rhs` is bigger than `u128`, the method [widening_mul()](struct@BigUInt#method.widening_mul)
    /// is proper rather than this method `widening_mul_uint()`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_uint = 248_u8;
    /// let (res_low, res_high) = a_biguint.widening_mul_uint(b_uint);
    /// 
    /// println!("{} X {} = {}:{}", a_biguint, b_uint, res_high, res_low);
    /// assert_eq!(res_high.to_string(), "1");
    /// assert_eq!(res_low.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn widening_mul_uint<U>(&self, rhs: U) -> (Self, Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut low = Self::from_array(self.get_number().clone());
        let high = low.widening_mul_assign_uint(rhs);
        (low, high)
    }

    // pub fn widening_mul_assign_uint<U>(&mut self, rhs: U) -> Self
    /// Calculates the complete product `self` * `rhs` without the possibility
    /// to overflow.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the high-order (overflow) bits of the result `self` * `rhs`.
    /// 
    /// # Features
    /// - It performs “long multiplication” which takes in an extra amount to add,
    /// and may return an additional amount of overflow. This allows for
    /// chaining together multiple multiplications to create “bigger integers”
    /// which represent larger values.
    /// - If the return value is not zero, the `OVERFOLOW` flag will be set
    /// though the output tuple is free from overflow. It is because the
    /// `OVERFOLOW` flag is about `self`, and not about the result of
    /// multiplication.
    /// - All the flags are historical, which means, for example, if an
    /// overflow occurred even once before this current operation or
    /// `OVERFLOW` flag is already set before this current operation,
    /// the `OVERFLOW` flag is not changed even if this current operation
    /// does not cause overflow.
    /// 
    /// # Counterpart Methods
    /// - If you also need to add a carry to the wide result, then you want to
    /// use
    /// [widening_mul_assign_uint()](struct@BigUInt#method.carrying_mul_assign_uint)
    /// instead.
    /// - The value of `self` after calculation matches what you’d get the
    /// method [wrapping_mul_uint()](struct@BigUInt#method.wrapping_mul_uint)
    /// `self` == `self.wrapping_mul_uint(rhs)`.
    /// - If `rhs` is bigger than `u128`, the method [widening_mul_assign()](struct@BigUInt#method.widening_mul_assign)
    /// is proper rather than this method `widening_mul_assign_uint()`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = UU32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_uint = 248_u8;
    /// 
    /// println!("Originally,\ta_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "876801874298166903427690031858186486050853753882811946569946433649006084094");
    /// 
    /// let mut res_high = a_biguint.widening_mul_assign_uint(b_uint);
    /// println!("After a_biguint.widening_mul_assign_uint(248_u8),\na_biguint = {}\nres_high = {}", a_biguint, res_high);
    /// assert_eq!(a_biguint.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    /// assert_eq!(res_high.to_string(), "1");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn widening_mul_assign_uint<U>(&mut self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if U::size_in_bytes() > T::size_in_bytes()
            { self.widening_mul_assign(&Self::from_uint(rhs)) }
        else // if U::size_in_bytes() <= T::size_in_bytes()
            { (Self::method_widening_mul_assign_uint)(self, T::num::<U>(rhs)) }
    }

    // Using carrying_mul()
    fn widening_mul_assign_uint_1(&mut self, rhs: T) -> Self
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
        let i_n = N-self.leading_zero_elements() as usize;
        let mut lower;
        let mut higher = zero;
        for i in 0..i_n
        {
            (lower, higher) = self.get_num_(i).carrying_mul(rhs, higher);
            self.set_num_(i, lower);
        }
        if !higher.is_zero()
        {
            if i_n < N
            {
                self.set_num_(i_n, higher);
            }
            else
            {
                self.set_overflow();
                high.set_num_(0, higher);
            }
        }
        high
    }

    // Using shift_left()
    fn widening_mul_assign_uint_2(&mut self, rhs: T) -> Self
    {
        let mut high = Self::zero();
        if rhs.is_zero()
        {
            self.set_zero();
            return high;
        }
        if self.is_zero()
            { return high; }

        let mut rhs = rhs;
        let mut adder = self.clone();
        self.set_zero();
        loop
        {
            if rhs.is_odd()
            {
                if self.overflowing_add_assign(&adder)
                    { high.wrapping_add_assign_uint(1_u8); }
            }
            rhs >>= T::one();
            if rhs == T::zero()
                { break; }
            adder.shift_left_assign(1_u8);
        }
        high
    }

    // pub fn wrapping_mul_uint<U>(&self, rhs: U) -> Self
    /// Multiplies `BigUInt`-type number with a unsigned integer number
    /// of type `T` and returns its result in a type of `BigUInt`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the multiplication of `self` and `rhs`, wrapping around
    /// at the boundary of the type.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method [wrapping_mul_uint()](struct@BigUInt#method.wrapping_mul_uint)
    /// is proper rather than this method `wrapping_mul_uint()`.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_uint = 248_u16;
    /// let res = a_biguint.wrapping_mul_uint(b_uint);
    /// println!("{} X {} = {}", a_biguint, b_uint, res);
    /// assert_eq!(res.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    /// assert_eq!(res.is_overflow(), true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn wrapping_mul_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut res = Self::from_array(self.get_number().clone());
        res.wrapping_mul_assign_uint(rhs);
        res
    }

    // pub fn wrapping_mul_assign_uint<U>(&mut self, rhs: U)
    /// Multiplies self which is of `BigUInt` type with rhs of type `U`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - All the flags are historical, which means, for example, if an
    /// overflow occurred even once before this current operation or
    /// `OVERFLOW` flag is already set before this current operation,
    /// the `OVERFLOW` flag is not changed even if this current operation
    /// does not cause overflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method [wrapping_mul_assign()](struct@BigUInt#method.wrapping_mul_assign)
    /// is proper rather than this method `wrapping_mul_assign_uint()`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = UU32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_uint = 248_u16;
    /// 
    /// println!("Originally,\ta_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "876801874298166903427690031858186486050853753882811946569946433649006084094");
    /// 
    /// a_biguint.wrapping_mul_assign_uint(b_uint);
    /// println!("After a_biguint.wrapping_mul_assign_uint(248_u16), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn wrapping_mul_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if U::size_in_bytes() > T::size_in_bytes()
            { self.wrapping_mul_assign(&Self::from_uint(rhs)) }
        else // if U::size_in_bytes() <= T::size_in_bytes()
            { (Self::method_wrapping_mul_assign_uint)(self, T::num::<U>(rhs)) }
    }

    // Using carrying_mul()
    fn wrapping_mul_assign_uint_1(&mut self, rhs: T)
    {
        if rhs.is_zero()
        {
            self.set_zero();
            return;
        }
        if self.is_zero()
            { return; }

        let zero = T::zero();
        let mut lower;
        let mut higher = zero;
        let i_n = N - self.leading_zero_elements() as usize;
        for i in 0..i_n
        {
            (lower, higher) = self.get_num_(i).carrying_mul(rhs, higher);
            self.set_num_(i, lower);
        }
        if !higher.is_zero()
        {
            if i_n < N
                { self.set_num_(i_n, higher); }
            else
                { self.set_overflow(); }
        }
    }

    // Using shift_left()
    fn wrapping_mul_assign_uint_2(&mut self, rhs: T)
    {
        if rhs.is_zero()
        {
            self.set_zero();
            return;
        }
        if self.is_zero()
            { return; }

        let mut trhs = rhs;
        let mut adder = Self::from_array(self.get_number().clone());
        self.set_zero();
        loop
        {
            if trhs.is_odd()
                { self.wrapping_add_assign(&adder); }
            trhs >>= T::one();
            if trhs.is_zero()
                { break; }
            adder.shift_left_assign(1_u8);
        }
        if adder.is_overflow()
            { self.set_overflow(); }
    }

    // pub fn overflowing_mul_uint<U>(&self, rhs: U) -> (Self, bool)
    /// Calculates `self` * `rhs`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns a tuple of the multiplication `self` * `rhs` along
    /// with a boolean indicating whether an arithmetic overflow would occur.
    /// If an overflow would have occurred then the wrapped (modular) value
    /// is returned.
    /// 
    /// # Features
    /// - If the second element of the output tuple is false, the `OVERFOLOW`
    /// flag will be set.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [overflowing_mul()](struct@BigUInt#method.overflowing_mul)
    /// is proper rather than this method `overflowing_mul_uint()`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_uint = 248_u8;
    /// let (res, overflow) = a_biguint.overflowing_mul_uint(b_uint);
    /// println!("{} X {} = {}, {}", a_biguint, b_uint, res, overflow);
    /// assert_eq!(res.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    /// assert_eq!(overflow, true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn overflowing_mul_uint<U>(&self, rhs: U) -> (Self, bool)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut res = Self::from_array(self.get_number().clone());
        let current_overflow = res.overflowing_mul_assign_uint(rhs);
        (res, current_overflow)
    }

    // pub fn overflowing_mul_assign_uint<U>(&mut self, rhs: U) -> bool
    /// Calculates `self` * `rhs`, and assigns the result to `self` back.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns true if an arithmetic overflow would occur.
    /// Otherwise, it returns `false`.
    /// 
    /// # Features
    /// - If the output is false, the `OVERFOLOW` flag will be set.
    /// - All the flags are historical, which means, for example, if an
    /// overflow occurred even once before this current operation or
    /// `OVERFLOW` flag is already set before this current operation,
    /// the `OVERFLOW` flag is not changed even if this current operation
    /// does not cause overflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [overflowing_mul_assign()](struct@BigUInt#method.overflowing_mul_assign)
    /// is proper rather than this method `overflowing_mul_assign_uint()`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = UU32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_uint = 248_u16;
    /// 
    /// println!("Originally,\ta_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "876801874298166903427690031858186486050853753882811946569946433649006084094");
    /// 
    /// let overflow = a_biguint.overflowing_mul_assign_uint(b_uint);
    /// println!("After a_biguint.overflowing_mul_assign_uint(248_u16), a_biguint = {}, {}", a_biguint, overflow);
    /// assert_eq!(a_biguint.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    /// assert_eq!(overflow, true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn overflowing_mul_assign_uint<U>(&mut self, rhs: U) -> bool
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let old = self.get_all_flags();
        self.reset_all_flags();
        self.wrapping_mul_assign_uint(rhs);
        let current_overflow = self.is_overflow();
        self.set_flag_bit(old);
        current_overflow
    }

    // pub fn checked_mul_uint<U>(&self, rhs: U) -> Option<Self>
    /// Computes `self` * `rhs`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the sum `self` * `rhs` wrapped by `Some` of enum `Option`
    /// if overflow did not occur. Otherwise, it returns `None` of enum Option.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [checked_mul()](struct@BigUInt#method.checked_mul)
    /// is proper rather than this method `checked_mul_uint()`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let mut b_uint = 248_u8;
    /// let mut res = a_biguint.checked_mul_uint(b_uint);
    /// match &res
    /// {
    ///     Some(r) => { println!("{} X {} = {}", a_biguint, b_uint, r); },
    ///     None => { println!("Overflow happend!"); },
    /// }
    /// assert_eq!(res, None);
    /// 
    /// b_uint = 5_u8;
    /// res = a_biguint.checked_mul_uint(b_uint);
    /// match &res
    /// {
    ///     Some(r) => { println!("{} X {} = {}", a_biguint, b_uint, r); },
    ///     None => { println!("Overflow happend!"); },
    /// }
    /// assert_eq!(res.unwrap().to_string(), "4384009371490834517138450159290932430254268769414059732849732168245030420470");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn checked_mul_uint<U>(&self, rhs: U) -> Option<Self>
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut res = Self::from_array(self.get_number().clone());
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
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// - If overflow occurred, it will panic. So, use this method only when
    /// you are sure that overflow will not occur.
    /// 
    /// # Output
    /// It returns the sum `self` * `rhs` if overflow did not occur.
    /// Otherwise, it will panic.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [unchecked_mul()](struct@BigUInt#method.unchecked_mul)
    /// is proper rather than this method `unchecked_mul_uint()`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = UU32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let mut res = a_biguint.unchecked_mul_uint(5_u8);
    /// println!("{} X {} = {}", a_biguint, 5_u8, res);
    /// assert_eq!(res.to_string(), "4384009371490834517138450159290932430254268769414059732849732168245030420470");
    /// 
    /// // It will panic.
    /// // res = a_biguint.unchecked_mul_uint(248_u8);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn unchecked_mul_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
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
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the sum `self` + `rhs` if overflow did not occur.
    /// Otherwise, it returns the maximum value.
    /// 
    /// # Features
    /// - This method saturates when it reaches maximum value.
    /// - It does not set `OVERFLOW` flag.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [saturating_mul()](struct@BigUInt#method.saturating_mul)
    /// is proper rather than this method `saturating_mul_uint()`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let mut res = a_biguint.saturating_mul_uint(5_u8);
    /// println!("{} X {} = {}", a_biguint, 5_u8, res);
    /// assert_eq!(res.to_string(), "4384009371490834517138450159290932430254268769414059732849732168245030420470");
    /// 
    /// res = a_biguint.saturating_mul_uint(248_u8);
    /// println!("{} X {} = {}", a_biguint, 248_u8, res);
    /// assert_eq!(res.to_string(), "115792089237316195423570985008687907853269984665640564039457584007913129639935");
    /// assert_eq!(res, UU32::max());
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn saturating_mul_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut res = Self::from_array(self.get_number().clone());
        res.saturating_mul_assign_uint(rhs);
        res
    }

    // pub fn saturating_mul_assign_uint<U>(&mut self, rhs: U)
    /// Computes `self` * `rhs`, saturating at the numeric bounds
    /// instead of overflowing, and assigns the result to `self` back.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - This method saturates when it reaches maximum value.
    /// - It does not set `OVERFLOW` flag.
    /// - All the flags are historical, which means, for example, if an
    /// overflow occurred even once before this current operation or
    /// `OVERFLOW` flag is already set before this current operation,
    /// the `OVERFLOW` flag is not changed even if this current operation
    /// does not cause overflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [saturating_mul_assign()](struct@BigUInt#method.saturating_mul_assign)
    /// is proper rather than this method `saturating_mul_assign_uint()`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let mut b_biguint = a_biguint.clone();
    /// 
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// a_biguint.saturating_mul_assign_uint(5_u8);
    /// println!("After a_biguint.saturating_mul_assign_uint(5_u8), a_biguint = {}", a_biguint);
    /// 
    /// println!("Originally, b_biguint = {}", b_biguint);
    /// b_biguint.saturating_mul_assign_uint(248_u8);
    /// println!("After b_biguint.saturating_mul_assign_uint(248_u8), b_biguint = {}", b_biguint);
    /// assert_eq!(b_biguint.to_string(), "115792089237316195423570985008687907853269984665640564039457584007913129639935");
    /// assert_eq!(b_biguint, UU32::max());
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn saturating_mul_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let overflow = self.is_overflow();
        if self.overflowing_mul_assign_uint(rhs)
        {
            self.set_max();
            if !overflow
                { self.reset_overflow(); }
        }
    }

    /*
    pub fn expanded_mul<U, const M: usize>(&self, rhs: U) -> BigUInt<T, M>
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {

        let (low, high) = self.widening_mul_uint(rhs);
        low.into_biguint::<T, M>()
        BigUInt::<T, M>::new()
    }
    */

    // pub fn modular_mul_uint<U>(&self, rhs: U, modulo: &Self) -> Self
    /// Computes (`self` * `rhs`) % `modulo`, wrapping around at `modulo`
    /// of the type `Self`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns (`self` * `rhs`) % `modulo`.
    /// 
    /// # Features
    /// - Wrapping (modular) multiplication at `modulo`.
    /// - The differences between this method `modular_mul_uint()` and the
    /// method `wrapping_mul_uint()` are, first, where wrapping around happens,
    /// and, second, whether or not `OVERFLOW` flag is set. First, this method
    /// wraps araound at `modulo` while the method `wrapping_mul_uint()` wraps
    /// araound at maximum value. Second, this method does not set `OVERFLOW`
    /// flag even if wrapping around happens, while the method
    /// `wrapping_mul_uint()` sets `OVERFLOW` flag when wrapping around happens.
    /// - If `modulo` is `zero`, the flags such as `OVERFLOW`, `DIVIDED_BY_ZERO`,
    /// and `INFINITY` will be set.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [modular_mul()](struct@BigUInt#method.modular_mul) is proper
    /// rather than this method `modular_mul_uint()`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let a_biguint = U256::from_string("31858186486050853753882811946768018742981669034276900586487291375468285").unwrap();
    /// let mut mul_uint = 5_u8;
    /// 
    /// let mut res = a_biguint.modular_mul_uint(mul_uint, &m);
    /// println!("{} * {} = {}", a_biguint, mul_uint, res);
    /// assert_eq!(res.to_string(), "159290932430254268769414059733840093714908345171384502932436456877341425");
    /// 
    /// mul_uint = 248_u8;
    /// res = a_biguint.modular_mul_uint(mul_uint, &m);
    /// println!("{} * {} = {}", a_biguint, mul_uint, res);
    /// assert_eq!(res.to_string(), "7900830248540611730962937362798468648259453920500671345448848261116134680");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn modular_mul_uint<U>(&self, rhs: U, modulo: &Self) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut res = Self::from_array(self.get_number().clone());
        res.modular_mul_assign_uint(rhs, modulo);
        res
    }

    // pub fn modular_mul_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    /// Computes (`self` * `rhs`) % `modulo`, wrapping around at `modulo`
    /// of the type `Self`, and assign the result to `self` back.
    /// 
    /// # Features
    /// - Wrapping (modular) multiplication at `modulo`. The differences between
    /// this method `modular_mul_assign_uint()` and the method
    /// `wrapping_mul_assign_uint()` are, first, where wrapping around happens,
    /// and, second, whether or not `OVERFLOW` flag is set. First, this method
    /// wraps araound at `modulo` while the method `wrapping_mul_assign_uint()`
    /// wraps araound at maximum value. Second, this method does not set
    /// `OVERFLOW` flag even if wrapping around happens, while the method
    /// `wrapping_mul_assign_uint()` sets `OVERFLOW` flag when wrapping around
    /// happens.
    /// - If `modulo` is `zero`, the flags such as `OVERFLOW`, `DIVIDED_BY_ZERO`,
    /// and `INFINITY` will be set.
    /// - All the flags are historical, which means, for example, if an
    /// overflow occurred even once before this current operation or
    /// `OVERFLOW` flag is already set before this current operation,
    /// the `OVERFLOW` flag is not changed even if this current operation
    /// does not cause overflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [modular_mul_assign()](struct@BigUInt#method.modular_mul_assign)
    /// is proper rather than this method `modular_mul_assign_uint()`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let mut a_biguint = U256::from_string("31858186486050853753882811946768018742981669034276900586487291375468285").unwrap();
    /// let mut b_biguint = a_biguint.clone();
    /// let mut mul_uint = 5_u8;
    /// 
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// a_biguint.modular_mul_assign_uint(mul_uint, &m);
    /// println!("After a_biguint.modular_mul_assign_uint(mul_uint, &m), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "159290932430254268769414059733840093714908345171384502932436456877341425");
    /// 
    /// mul_uint = 248_u8;
    /// println!("Originally, b_biguint = {}", b_biguint);
    /// b_biguint.modular_mul_assign_uint(mul_uint, &m);
    /// println!("After b_biguint.modular_mul_assign_uint(mul_uint, &m), b_biguint = {}", b_biguint);
    /// assert_eq!(b_biguint.to_string(), "7900830248540611730962937362798468648259453920500671345448848261116134680");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn modular_mul_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if *self >= *modulo
            { self.wrapping_rem_assign(modulo); }
        if modulo.gt_uint(rhs)
        {
            let mut mrhs = T::num::<U>(rhs);
            let mut res = Self::zero();
            while mrhs > T::zero()
            {
                if mrhs.is_odd()
                    { res.modular_add_assign(self, modulo); }
                self.modular_add_assign(&self.clone(), modulo);
                mrhs >>= T::one();
            }
            *self = res;
        }
        else if U::size_in_bytes() > T::size_in_bytes()  // && (module <= rhs)
        {
            self.modular_mul_assign(&Self::from_uint(rhs), modulo);
        }
        else    // if (U::size_in_bytes() <= T::size_in_bytes()) && (module <= rhs)
        {
            let modu = modulo.into_u128();
            let mrhs = rhs.into_u128().wrapping_rem(modu);
            let mut mself = self.into_u128();
            mself = mself.modular_mul(mrhs, modu);
            self.set_uint(mself);
        }
    }


    /*** Division ***/

    // pub fn divide_fully_uint<U>(&self, rhs: U) -> (Self, U)
    /// Divide `BigUInt<T, N>` by `rhs` so as to get quotient and remainder
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns tuple of quotient and remainder. quotient is `Self` type
    /// and remainder is `U` type.
    /// 
    /// # Features
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// - If `rhs` is zero, the quotient will have maximum value of `BigUInt`
    /// type, and the flags of quotient such as `OVERFLOW`, `INFINITY`, and
    /// `DIVIDED_BY_ZERO` will be set.
    /// - If `rhs` is zero, the remainder will be `zero` of `U` type,
    /// and the `DIVIDED_BY_ZERO` flag of remainder will be set.
    /// - __It does not panic__ even if `rhs` is zero.
    /// - This function is the base function for all the methods *_div_uint(),
    /// and *_div_assign_uint().
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [divide_fully()](struct@BigUInt#method.divide_fully)
    /// is proper rather than this method `divide_fully_uint()`.
    /// 
    /// # Examples
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let mut divisor = 87_u8;
    /// let (mut quotient, mut remainder) = dividend.divide_fully_uint(divisor);
    /// println!("{} / {} => quotient = {} , remainder = {}", dividend, divisor, quotient, remainder);
    /// assert_eq!(quotient.to_string(), "1419043551905275201680884938348044216837079832");
    /// assert_eq!(remainder.to_string(), "8");
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_inifinity(), false);
    /// assert_eq!(quotient.is_divided_by_zero(), false);
    /// 
    /// divisor = 0_u8;
    /// (quotient, remainder) = dividend.divide_fully_uint(divisor);
    /// println!("{} / {} => quotient = {} , remainder = {}", dividend, divisor, quotient, remainder);
    /// assert_eq!(quotient, U256::max());
    /// assert_eq!(remainder.to_string(), "0");
    /// assert_eq!(quotient.is_overflow(), true);
    /// assert_eq!(quotient.is_inifinity(), true);
    /// assert_eq!(quotient.is_divided_by_zero(), true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn divide_fully_uint<U>(&self, rhs: U) -> (Self, U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if self.is_zero()
        {
            (Self::zero(), U::zero())
        }
        else if rhs.is_zero()
        {
            let mut quotient = Self::max();
            quotient.set_infinity();
            quotient.set_divided_by_zero();
            quotient.set_overflow();
            (quotient, U::zero())
        }
        else if self.lt_uint(rhs)
        {
            (Self::zero(), SharedValues::<U, T>::from_src(self.get_num_(0)).get_des())
        }
        else if self.eq_uint(rhs)
        {
            (Self::one(), U::zero())
        }
        else if U::size_in_bytes() <= T::size_in_bytes()
        {
            let mut quotient = Self::zero();
            let size_rhs = rhs.length_in_bits() - rhs.leading_zeros() as usize;
            let size_self = self.length_in_bits() - self.leading_zeros() as usize;
            let mut remainder = SharedValues::<U, T>::from_src(self.get_upper_portion(size_rhs).get_num_(0)).get_des();
            let mut position = size_self - size_rhs;
            loop
            {
                if remainder >= rhs
                {
                    quotient.set_lsb();
                    remainder = remainder.wrapping_sub(rhs);
                }
                if position == 0
                    { break; }
                position -= 1;
                quotient.shift_left_assign(1_u8);
                remainder <<= U::one();
                if self.is_bit_set_(position)
                    { remainder.set_lsb(); }
            }
            (quotient, remainder)
        }
        else    // if U::size_in_bytes() > T::size_in_bytes()
        {
            let (quotient, remainder) = self.divide_fully(&Self::from_uint(rhs));
            (quotient, remainder.into_uint::<U>())
        }
    }

    // pub fn wrapping_div_uint<U>(&self, rhs: U) -> Self
    /// Calculates the quotient when `self` is divided by `rhs`,
    /// which is `self` / `rhs`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    ///
    /// # Output
    /// It returns the quotient of when `self` is divided by `rhs`,
    /// which is `self` / `rhs`. 
    /// 
    /// # Features
    /// - Wrapped division on `BigUInt` types is just normal division.
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// - This function exists, so that all operations are accounted for
    /// in the wrapping operations.
    /// - If `rhs` is zero, the quotient will have maximum value of `BigUInt`
    /// type, and the flags of quotient such as `OVERFLOW`, `INFINITY`, and
    /// `DIVIDED_BY_ZERO` will be set
    /// - __It does not panic__ while the counterpart method `wrapping_div()`
    /// for primitive integer data type such as u8, u16, u32, u64, etc.
    /// will panic if `rhs` is zero.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [wrapping_div()](struct@BigUInt#method.wrapping_div)
    /// is proper rather than this method `wrapping_div_uint()`.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let mut divisor = 87_u8;
    /// let mut quotient = dividend.wrapping_div_uint(divisor);
    /// println!("{} / {} = {}", dividend, divisor, quotient);
    /// assert_eq!(quotient.to_string(), "1419043551905275201680884938348044216837079832");
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_inifinity(), false);
    /// assert_eq!(quotient.is_divided_by_zero(), false);
    /// 
    /// divisor = 0_u8;
    /// quotient = dividend.wrapping_div_uint(divisor);
    /// println!("{} / {} = {}", dividend, divisor, quotient);
    /// assert_eq!(quotient, U256::max());
    /// assert_eq!(quotient.is_overflow(), true);
    /// assert_eq!(quotient.is_inifinity(), true);
    /// assert_eq!(quotient.is_divided_by_zero(), true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn wrapping_div_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let (quotient, _) = self.divide_fully_uint(rhs);
        quotient
    }

    // pub fn wrapping_div_assign_uint<U>(&mut self, rhs: U)
    /// Calculates the quotient when `self` is divided by `rhs`,
    /// which is `self` / `rhs`, and assigns the result to `self` back.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    ///
    /// # Features
    /// - Wrapped division on `BigUInt` types is just normal division.
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// - This function exists, so that all operations are accounted for
    /// in the wrapping operations.
    /// - If `rhs` is zero, the quotient will have maximum value of `BigUInt`
    /// type, and the flags of quotient such as `OVERFLOW`, `INFINITY`, and
    /// `DIVIDED_BY_ZERO` will be set
    /// - __It does not panic__ while the counterpart method `wrapping_div()`
    /// for primitive integer data type such as u8, u16, u32, u64, etc.
    /// will panic if `rhs` is zero.
    /// - All the flags are historical, which means, for example, if an
    /// divided_by_zero occurred even once before this current operation or
    /// `DIVIDED_BY_ZERO` flag is already set before this current operation,
    /// the `DIVIDED_BY_ZERO` flag is not changed even if this current operation
    /// does not cause divided_by_zero.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [wrapping_div_assign()](struct@BigUInt#method.wrapping_div_assign)
    /// is proper rather than this method `wrapping_div_assign_uint()`.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let mut divisor = 87_u8;
    /// println!("Originally,\na_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_inifinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// divisor = 0_u8;
    /// a_biguint.wrapping_div_assign_uint(divisor);
    /// println!("After a_biguint.wrapping_div_assign_uint({}),\na_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint, UU32::max());
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_inifinity(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn wrapping_div_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let flags = self.get_all_flags();
        *self = self.wrapping_div_uint(rhs);
        self.set_flag_bit(flags);
    }

    // pub fn overflowing_div_uint<U>(&self, rhs: U) -> (Self, bool)
    /// Calculates the quotient when `self` is divided by `rhs`,
    /// which is `self` / `rhs`.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    ///
    /// # Output
    /// It returns a tuple of the quotient along with a boolean indicating
    /// whether an arithmetic overflow would occur. Note that when `rhs` is
    /// a non-zero unsigned integer, overflow never occurs, so the second
    /// value is always `false` in that case. However, if `rhs` is zero,
    /// the second value of the output tuple will be `true`.
    ///
    /// # Features
    /// - Overflowing division on `BigUInt` types is just normal division.
    /// - There’s no way overflowing could ever happen unless `rhs` is zero.
    /// - This function exists, so that all operations are accounted for
    /// in the wrapping operations.
    /// - If `rhs` is zero, the quotient will have maximum value of `BigUInt`
    /// type, and the flags of quotient such as `OVERFLOW`, `INFINITY`, and
    /// `DIVIDED_BY_ZERO` will be set
    /// - __It does not panic__ while the counterpart method `overflowing_div()`
    /// for primitive integer data type such as u8, u16, u32, u64, etc.
    /// will panic if `rhs` is zero.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [overflowing_div()](struct@BigUInt#method.overflowing_div)
    /// is proper rather than this method `overflowing_div_uint()`.
    ///
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let mut divisor = 87_u8;
    /// let (mut quotient, mut overflow) = dividend.overflowing_div_uint(divisor);
    /// println!("{} / {} = {}", dividend, divisor, quotient);
    /// assert_eq!(quotient.to_string(), "1419043551905275201680884938348044216837079832");
    /// assert_eq!(overflow, false);
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_inifinity(), false);
    /// assert_eq!(quotient.is_divided_by_zero(), false);
    /// 
    /// divisor = 0_u8;
    /// (quotient, overflow) = dividend.overflowing_div_uint(divisor);
    /// println!("{} / {} = {}", dividend, divisor, quotient);
    /// assert_eq!(quotient, U256::max());
    /// assert_eq!(overflow, true);
    /// assert_eq!(quotient.is_overflow(), true);
    /// assert_eq!(quotient.is_inifinity(), true);
    /// assert_eq!(quotient.is_divided_by_zero(), true);
    /// ```
    ///
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn overflowing_div_uint<U>(&self, rhs: U) -> (Self, bool)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let (quotient, _) = self.divide_fully_uint(rhs);
        let overflow = quotient.is_overflow();
        (quotient, overflow)
    }

    // pub fn overflowing_div_assign_uint<U>(&mut self, rhs: U) -> bool
    /// Calculates the quotient when `self` is divided by `rhs`,
    /// which is `self` / `rhs`, and assigns the result to `self` back.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    ///
    /// # Output
    /// It returns a boolean indicating whether an arithmetic overflow would
    /// occur. Note that when `rhs` is a non-zero unsigned integer, overflow
    /// never occurs, so the output is always `false` in that case. However,
    /// if `rhs` is zero, the output will be `true`.
    ///
    /// # Features
    /// - Overflowing division on `BigUInt` types is just normal division.
    /// - There’s no way overflowing could ever happen unless `rhs` is zero.
    /// - This function exists, so that all operations are accounted for
    /// in the wrapping operations.
    /// - If `rhs` is zero, the quotient will have maximum value of `BigUInt`
    /// type, and the flags of quotient such as `OVERFLOW`, `INFINITY`, and
    /// `DIVIDED_BY_ZERO` will be set
    /// - __It does not panic__ while the counterpart method `overflowing_div()`
    /// for primitive integer data type such as u8, u16, u32, u64, etc.
    /// will panic if `rhs` is zero.
    /// - All the flags are historical, which means, for example, if an
    /// divided_by_zero occurred even once before this current operation or
    /// `DIVIDED_BY_ZERO` flag is already set before this current operation,
    /// the `DIVIDED_BY_ZERO` flag is not changed even if this current operation
    /// does not cause divided_by_zero.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [overflowing_div_assign()](struct@BigUInt#method.overflowing_div_assign)
    /// is proper rather than this method `overflowing_div_assign_uint()`.
    ///
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let mut divisor = 87_u16;
    /// println!("Originally,\na_biguint = {}", a_biguint);
    /// let mut overflow = a_biguint.overflowing_div_assign_uint(divisor);
    /// println!("After a_biguint.overflowing_div_assign_uint({}),\na_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1419043551905275201680884938348044216837079832");
    /// assert_eq!(overflow, false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_inifinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// divisor = 0_u16;
    /// overflow = a_biguint.overflowing_div_assign_uint(divisor);
    /// println!("After a_biguint.overflowing_div_assign_uint({}),\na_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint, UU32::max());
    /// assert_eq!(overflow, true);
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_inifinity(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// ```
    ///
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn overflowing_div_assign_uint<U>(&mut self, rhs: U) -> bool
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let old = self.get_all_flags();
        let (quotient, _) = self.divide_fully_uint(rhs);
        *self = quotient;
        let overflow = self.is_overflow();
        self.set_flag_bit(old);
        overflow
    }

    // pub fn checked_div_uint<U>(&self, rhs: U) -> Option<Self>
    /// Calculates the quotient when `self` is divided by `rhs`,
    /// which is `self` / `rhs`.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `None` if `rhs` is zero. Otherwise, it returns the quotient
    /// of when `self` is divided by `rhs`, which is `self` / `rhs`,
    /// wrapped by `Some` of enum `Option`.
    /// 
    /// # Features
    /// - Checked integer division on `BigUInt` types is just normal division.
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [checked_div()](struct@BigUInt#method.checked_div)
    /// is proper rather than this method `checked_div_uint()`.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let mut divisor = 87_u8;
    /// let mut quotient = dividend.checked_div_uint(divisor);
    /// match quotient.clone()
    /// {
    ///     Some(q) =>
    ///         {
    ///             println!("{} / {} = {}", dividend, divisor, q);
    ///             assert_eq!(q.to_string(), "1419043551905275201680884938348044216837079832");
    ///             assert_eq!(quotient.clone().unwrap().is_overflow(), false);
    ///             assert_eq!(quotient.clone().unwrap().is_inifinity(), false);
    ///             assert_eq!(quotient.clone().unwrap().is_divided_by_zero(), false);
    ///         },
    ///     None => { println!("Divided By Zero"); },
    /// }
    /// 
    /// divisor = 0_u8;
    /// quotient = dividend.checked_div_uint(divisor);
    /// match quotient
    /// {
    ///     Some(q) => { println!("{} / {} = {}", dividend, divisor, q); },
    ///     None =>
    ///         {
    ///             println!("Divided By Zero");
    ///             assert_eq!(quotient, None);
    ///         },
    /// }
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn checked_div_uint<U>(&self, rhs: U) -> Option<Self>
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let res = self.wrapping_div_uint(rhs);
        if res.is_divided_by_zero()
            { None }
        else
            { Some(res) }
    }

    // pub fn unchecked_div_uint<U>(&self, rhs: U) -> Self
    /// Calculates the quotient when `self` is divided by `rhs`,
    /// which is `self` / `rhs`, assuming that `rhs` cannot be zero.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// - If `rhs` is zero, it will panic. So, use this method only when you
    /// are sure that `rhs` is not zero. 
    /// 
    /// # Output
    /// It returns the quotient of when `self` is divided by `rhs`,
    /// which is `self` / `rhs` if `rhs` is not zero.
    /// If `rhs` is zero, it will panic.
    /// 
    /// # Features
    /// - Unchecked integer division on `BigUInt` types is just normal division.
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// - If `rhs` is zero, it will panic.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [unchecked_div()](struct@BigUInt#method.unchecked_div)
    /// is proper rather than this method `unchecked_div_uint()`.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let mut divisor = 87_u8;
    /// let mut quotient = dividend.unchecked_div_uint(divisor);
    /// println!("{} / {} = {}", dividend, divisor, quotient);
    /// assert_eq!(quotient.to_string(), "1419043551905275201680884938348044216837079832");
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_inifinity(), false);
    /// assert_eq!(quotient.is_divided_by_zero(), false);
    /// 
    /// divisor = 0_u8;
    /// // It will panic.
    /// // quotient = dividend.uchecked_div_uint(divisor);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn unchecked_div_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.checked_div_uint(rhs).unwrap()
    }

    // pub fn saturating_div_uint<U>(&self, rhs: U) -> Self
    /// Calculates the quotient when `self` is divided by `rhs`,
    /// which is `self` / `rhs`, saturating at the numeric bounds
    /// instead of overflowing.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the quotient of when `self` is divided by `rhs`,
    /// which is `self` / `rhs` if `rhs` is not zero.
    /// If `rhs` is zero, it returns the maximum value.
    /// 
    /// # Features
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// - This method saturates when it reaches maximum value.
    /// - It does not set `OVERFLOW` flag.
    /// - If `rhs` is zero, the quotient will have maximum value of `BigUInt`
    /// type, and `DIVIDED_BY_ZERO` flag of quotient will be set.
    /// - __It does not panic__ while the counterpart method `saturating_div()`
    /// for primitive integer data type such as u8, u16, u32, u64, etc.
    /// will panic if `rhs` is zero.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [saturating_div()](struct@BigUInt#method.saturating_div)
    /// is proper rather than this method `saturating_div_uint()`.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let mut divisor = 87_u8;
    /// let mut quotient = dividend.saturating_div_uint(divisor);
    /// println!("{} / {} = {}", dividend, divisor, quotient);
    /// assert_eq!(quotient.to_string(), "1419043551905275201680884938348044216837079832");
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_inifinity(), false);
    /// assert_eq!(quotient.is_divided_by_zero(), false);
    /// 
    /// divisor = 0_u8;
    /// quotient = dividend.saturating_div_uint(divisor);
    /// println!("{} / {} = {}", dividend, divisor, quotient);
    /// assert_eq!(quotient, U256::max());
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_inifinity(), false);
    /// assert_eq!(quotient.is_divided_by_zero(), true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn saturating_div_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let (mut quotient, _) = self.divide_fully_uint(rhs);
        if rhs.is_zero()
        {
            quotient.set_max();
            quotient.reset_overflow();
            quotient.reset_inifinity();
            quotient.set_divided_by_zero();
        }
        quotient
    }

    // pub fn saturating_div_assign_uint<U>(&mut self, rhs: U) -> Self
    /// Calculates the quotient when `self` is divided by `rhs`,
    /// which is `self` / `rhs`, saturating at the numeric bounds
    /// instead of overflowing, and assigns the quotient to `self` back.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// - This method saturates when it reaches maximum value.
    /// - It does not set `OVERFLOW` flag.
    /// - If `rhs` is zero, the quotient will have maximum value of `BigUInt`
    /// type, and `DIVIDED_BY_ZERO` flag of quotient will be set.
    /// - __It does not panic__ while the counterpart method `saturating_div()`
    /// for primitive integer data type such as u8, u16, u32, u64, etc.
    /// will panic if `rhs` is zero.
    /// - All the flags are historical, which means, for example, if an
    /// divided_by_zero occurred even once before this current operation or
    /// `DIVIDED_BY_ZERO` flag is already set before this current operation,
    /// the `DIVIDED_BY_ZERO` flag is not changed even if this current operation
    /// does not cause divided_by_zero.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [saturating_div_assign()](struct@BigUInt#method.saturating_div_assign)
    /// is proper rather than this method `saturating_div_assign_uint()`.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let mut divisor = 87_u16;
    /// println!("Originally,\na_biguint = {}", a_biguint);
    /// a_biguint.saturating_div_assign_uint(divisor);
    /// println!("After a_biguint.saturating_div_assign_uint({}),\na_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1419043551905275201680884938348044216837079832");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_inifinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// divisor = 0_u16;
    /// a_biguint.saturating_div_assign_uint(divisor);
    /// println!("After a_biguint.saturating_div_assign_uint({}),\na_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint, UU32::max());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_inifinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn saturating_div_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let flags = self.get_all_flags();
        *self = self.saturating_div_uint(rhs);
        self.set_flag_bit(flags);
    }

    // pub fn modular_div_uint<U>(&self, rhs: U, modulo: &Self) -> Self
    /// Calculates the quotient when `self` % `modulo` is divided by
    /// `rhs` % `modulo`.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the quotient of when `self` % `modulo` is divided by
    /// `rhs` % `modulo` if `rhs` % `modulo` is not zero.
    /// If `rhs` % `modulo` is zero, the quotient will have maximum value
    /// of `BigUInt` type.
    /// 
    /// # Features
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// - If `rhs` % `modulo` is zero, the quotient will have maximum value
    /// of `BigUInt` type, and the flags of quotient such as `OVERFLOW`,
    /// `INFINITY`, and `DIVIDED_BY_ZERO` will be set.
    /// - __It does not panic__ even if `rhs` % `modulo` is zero.
    /// - If `modulo` is `zero`, the flags such as `OVERFLOW`, `DIVIDED_BY_ZERO`,
    /// and `INFINITY` will be set.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [modular_div()](struct@BigUInt#method.modular_div)
    /// is proper rather than this method `modular_div_uint()`.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let mut divisor = 128_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// let mut quotient = dividend.modular_div_uint(divisor, &modulo);
    /// println!("{} / {} = {}", dividend, divisor, quotient);
    /// assert_eq!(quotient.to_string(), "3");
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_inifinity(), false);
    /// assert_eq!(quotient.is_divided_by_zero(), false);
    /// 
    /// divisor = 200_u8;
    /// quotient = dividend.modular_div_uint(divisor, &modulo);
    /// println!("{} / {} = {}", dividend, divisor, quotient);
    /// assert_eq!(quotient, U256::max());
    /// assert_eq!(quotient.is_overflow(), true);
    /// assert_eq!(quotient.is_inifinity(), true);
    /// assert_eq!(quotient.is_divided_by_zero(), true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn modular_div_uint<U>(&self, rhs: U, modulo: &Self) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.modular_div(&Self::from_uint(rhs), modulo)
    }

    // pub fn modular_div_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    /// Calculates the quotient when `self` % `modulo` is divided by
    /// `rhs` % `modulo`, and assigns the quotient to `self` back.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// - If `rhs` % `modulo` is zero, the quotient will have maximum value
    /// of `BigUInt` type, and the flags of quotient such as `OVERFLOW`,
    /// `INFINITY`, and `DIVIDED_BY_ZERO` will be set.
    /// - __It does not panic__ even if `rhs` % `modulo` is zero.
    /// - If `modulo` is `zero`, the flags such as `OVERFLOW`, `DIVIDED_BY_ZERO`,
    /// and `INFINITY` will be set.
    /// - All the flags are historical, which means, for example, if an
    /// divided_by_zero occurred even once before this current operation or
    /// `DIVIDED_BY_ZERO` flag is already set before this current operation,
    /// the `DIVIDED_BY_ZERO` flag is not changed even if this current operation
    /// does not cause divided_by_zero.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [modular_div_assign()](struct@BigUInt#method.modular_div_assign)
    /// is proper rather than this method `modular_div_assign_uint()`.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let mut divisor = 128_u8;
    /// let modulo = UU32::from_uint(100_u8);
    /// println!("Originally,\na_biguint = {}", a_biguint);
    /// a_biguint.modular_div_assign_uint(divisor, &modulo);
    /// println!("After a_biguint.modular_div_assign_uint({}),\na_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "3");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_inifinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// divisor = 200_u8;
    /// a_biguint.modular_div_assign_uint(divisor, &modulo);
    /// println!("After a_biguint.modular_div_assign_uint({}),\na_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint, U256::max());
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_inifinity(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn modular_div_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.modular_div_assign(&Self::from_uint(rhs), modulo);
    }

    // pub fn wrapping_rem_uint<U>(&self, rhs: U) -> U
    /// Calculates the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// - If `rhs` is not zero, it returns the remainder when `self` is divided
    /// by `rhs`, which is `self` % `rhs`, with wrapping (modular) addition.
    /// - If `rhs` is zero, it returns `zero` of `U` type.
    /// 
    /// # Features
    /// - Wrapped remainder calculation on `BigUInt` types is just the regular
    /// remainder calculation.
    /// - There’s no way wrapping could ever happen.
    /// - This function exists, so that all operations are accounted for in the
    /// wrapping operations.
    /// - If `rhs` is `zero`, the remainder is zero.
    /// -__It does not panic__ while the same named methods `wrapping_rem()`
    /// for primitive integer data type such as u8, u16, u32, u64, etc. will
    /// panic if `rhs` is zero.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [wrapping_rem()](struct@BigUInt#method.wrapping_rem)
    /// is proper rather than this method `wrapping_rem_uint()`.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let mut divisor = 87_u8;
    /// let mut remainder = dividend.wrapping_rem_uint(divisor);
    /// println!("{} % {} = {}", dividend, divisor, remainder);
    /// assert_eq!(remainder.to_string(), "8");
    /// 
    /// divisor = 0_u8;
    /// remainder = dividend.wrapping_rem_uint(divisor);
    /// println!("{} % {} = {}", dividend, divisor, remainder);
    /// assert_eq!(remainder, 0);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn wrapping_rem_uint<U>(&self, rhs: U) -> U
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let (_, remainder) = self.divide_fully_uint(rhs);
        remainder
    }

    // pub fn wrapping_rem_assign_uint<U>(&mut self, rhs: U)
    /// Calculates the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`, and returns the remainder to `self` back.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - Wrapped remainder calculation on `BigUInt` types is just the regular
    /// remainder calculation.
    /// - There’s no way wrapping could ever happen.
    /// - This function exists, so that all operations are accounted for in the
    /// wrapping operations.
    /// - If `rhs` is zero, the `self` is zero and its `DIVIDED_BY_ZERO`
    /// is set.
    /// - __It does not panic__ while the counterpart method
    /// `wrapping_rem()` for primitive integer data type such as 
    /// u8, u16, u32, u64, etc. will panic if `rhs` is zero.
    /// - All the flags are historical, which means, for example, if an
    /// divided_by_zero occurred even once before this current operation or
    /// `DIVIDED_BY_ZERO` flag is already set before this current operation,
    /// the `DIVIDED_BY_ZERO` flag is not changed even if this current operation
    /// does not cause divided_by_zero.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [wrapping_rem_assign()](struct@BigUInt#method.wrapping_rem_assign)
    /// is proper rather than this method `wrapping_rem_assign_uint()`.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let mut divisor = 87_u8;
    /// println!("Originally,\na_biguint = {}", a_biguint);
    /// a_biguint.wrapping_rem_assign_uint(divisor);
    /// println!("After a_biguint.wrapping_rem_assign_uint({}),\na_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "8");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_inifinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// divisor = 0_u8;
    /// a_biguint.wrapping_rem_assign_uint(divisor);
    /// println!("After a_biguint.wrapping_rem_assign_uint({}),\na_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint, 0);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_inifinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn wrapping_rem_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let (_, remainder) = self.divide_fully_uint(rhs);
        self.set_uint(remainder);
        if rhs == U::zero()
            { self.set_divided_by_zero(); }
    }

    // pub fn overflowing_rem_uint<U>(&self, rhs: U) -> (U, bool)
    /// Calculates the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns a tuple of the remainder after dividing,
    /// which is `self` % `rhs` along with a boolean indicating whether an
    /// arithmetic overflow would occur.
    /// 
    /// # Features
    /// - Note that overflow never occurs, so the second value is always false.
    /// - If `rhs` is zero, the remainder is zero and the second output
    /// indicating whether or not an arithmetic overflow would occur is `false`.
    /// - __It does not panic__ while the counterpart method `overflowing_rem()`
    /// for primitive integer data type such as u8, u16, u32, u64, etc. will
    /// panic if `rhs` is zero.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [overflowing_rem()](struct@BigUInt#method.overflowing_rem)
    /// is proper rather than this method `overflowing_rem_uint()`.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let mut divisor = 87_u8;
    /// let (mut remainder, mut overflow) = dividend.overflowing_rem_uint(divisor);
    /// println!("{} % {} = {}", dividend, divisor, remainder);
    /// assert_eq!(remainder, 8);
    /// assert_eq!(overflow, false);
    /// 
    /// divisor = 0_u8;
    /// (remainder, overflow) = dividend.overflowing_rem_uint(divisor);
    /// println!("{} % {} = {}", dividend, divisor, remainder);
    /// assert_eq!(remainder, 0);
    /// assert_eq!(overflow, false);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn overflowing_rem_uint<U>(&self, rhs: U) -> (U, bool)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let (_, remainder) = self.divide_fully_uint(rhs);
        (remainder, false)
    }

    // pub fn overflowing_rem_assign_uint<U>(&mut self, rhs: U) -> bool
    /// Calculates the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`, and returns the remainder to `self` back.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns a boolean indicating whether an arithmetic overflow
    /// would occur.
    /// 
    /// # Features
    /// - Note that overflow never occurs, so the outtput is always false.
    /// - If `rhs` is zero, `self` is zero and its `DIVIDED_BY_ZERO`
    /// is set. __It does not panic__ while the similar methods
    /// `overflowing_rem()` for primitive integer data type such as
    /// u8, u16, u32, u64, etc. will panic if `rhs` is zero.
    /// - All the flags are historical, which means, for example, if an
    /// divided_by_zero occurred even once before this current operation or
    /// `DIVIDED_BY_ZERO` flag is already set before this current operation,
    /// the `DIVIDED_BY_ZERO` flag is not changed even if this current operation
    /// does not cause divided_by_zero.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [overflowing_rem_assign()](struct@BigUInt#method.overflowing_rem_assign)
    /// is proper rather than this method `overflowing_rem_assign_uint()`.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let mut divisor = 87_u16;
    /// println!("Originally,\na_biguint = {}", a_biguint);
    /// let mut overflow = a_biguint.overflowing_rem_assign_uint(divisor);
    /// println!("After a_biguint.overflowing_rem_assign_uint({}),\na_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "8");
    /// assert_eq!(overflow, false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_inifinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// divisor = 0_u16;
    /// overflow = a_biguint.overflowing_rem_assign_uint(divisor);
    /// println!("After a_biguint.overflowing_rem_assign_uint({}),\na_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint, U256::zero());
    /// assert_eq!(overflow, false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_inifinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn overflowing_rem_assign_uint<U>(&mut self, rhs: U) -> bool
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let (_, remainder) = self.divide_fully_uint(rhs);
        self.set_uint(remainder);
        if rhs == U::zero()
            { self.set_divided_by_zero(); }
        false
    }

    // pub fn checked_rem_uint<U>(&self, rhs: U) -> Option<U>
    /// Calculates the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`, wrapped by `Some` of enum `Option`
    /// if `rhs` is not zero. Otherwise, it returns `None` of enum `Option`.
    /// 
    /// # Features
    /// - Note that overflow never occurs.
    /// - If `rhs` is zero, the output of this method
    /// is `None` of enum `Option`.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [checked_rem()](struct@BigUInt#method.checked_rem)
    /// is proper rather than this method `checked_rem_uint()`.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let mut divisor = 87_u8;
    /// let mut remainder = dividend.checked_rem_uint(divisor);
    /// match remainder
    /// {
    ///     Some(r) =>
    ///         {
    ///             println!("{} % {} = {}", dividend, divisor, r);
    ///             assert_eq!(r.to_string(), "8");
    ///         },
    ///     None => { println!("Divided By Zero"); },
    /// }
    /// 
    /// divisor = 0_u8;
    /// remainder = dividend.checked_rem_uint(divisor);
    /// match remainder
    /// {
    ///     Some(r) => { println!("{} % {} = {}", dividend, divisor, r); },
    ///     None =>
    ///         {
    ///             println!("Divided By Zero");
    ///             assert_eq!(remainder, None);
    ///         },
    /// }
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn checked_rem_uint<U>(&self, rhs: U) -> Option<U>
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let (_, remainder) = self.divide_fully_uint(rhs);
        if rhs == U::zero()
            { None }
        else
            { Some(remainder) }
    }

    // pub fn unchecked_rem_uint<U>(&self, rhs: U) -> U
    /// Calculates the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`, assuming `rhs` cannot be zero.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// - If `rhs` is `zero`, it will panic.
    /// 
    /// # Output
    /// It returns the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`, if `rhs` is not zero.
    /// Otherwise, it will panic.
    /// 
    /// # Features
    /// Note that overflow never occurs.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [unchecked_rem()](struct@BigUInt#method.unchecked_rem)
    /// is proper rather than this method `unchecked_rem_uint()`.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let mut divisor = 87_u8;
    /// let mut remainder = dividend.unchecked_rem_uint(divisor);
    /// println!("{} % {} = {}", dividend, divisor, remainder);
    /// assert_eq!(remainder.to_string(), "8");
    /// 
    /// divisor = 0_u8;
    /// // It will panic.
    /// // remainder = dividend.unchecked_rem_uint(divisor);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn unchecked_rem_uint<U>(&self, rhs: U) -> U
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.checked_rem_uint(rhs).unwrap()
    }

    // pub fn saturating_rem_uint<U>(&self, rhs: U) -> U
    /// Calculates the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`, if `rhs` is not zero.
    /// If `rhs` is zero, it returns zero.
    /// 
    /// # Features
    /// - Note that overflow never occurs.
    /// - There’s no way wrapping could ever happen.
    /// - If `rhs` is `zero`, the remainder will have zero of`BigUInt` type.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [saturating_rem()](struct@BigUInt#method.saturating_rem)
    /// is proper rather than this method `saturating_rem_uint()`.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let mut divisor = 87_u8;
    /// let mut remainder = dividend.saturating_rem_uint(divisor);
    /// println!("{} % {} = {}", dividend, divisor, remainder);
    /// assert_eq!(remainder.to_string(), "8");
    /// 
    /// divisor = 0_u8;
    /// remainder = dividend.saturating_rem_uint(divisor);
    /// println!("{} % {} = {}", dividend, divisor, remainder);
    /// assert_eq!(remainder, 0);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn saturating_rem_uint<U>(&self, rhs: U) -> U
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let (_, remainder) = self.divide_fully_uint(rhs);
        remainder
    }

    // pub fn saturating_rem_assign_uint<U>(&mut self, rhs: U)
    /// Calculates the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`, and assigns the remainder to `self` back.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - If `rhs` is zero, `self` will have the value `zero`` of`BigUInt` type,
    /// and `DIVIDED_BY_ZERO` flag of `self` will be set.
    /// - Note that overflow never occurs.
    /// - All the flags are historical, which means, for example, if an
    /// divided_by_zero occurred even once before this current operation or
    /// `DIVIDED_BY_ZERO` flag is already set before this current operation,
    /// the `DIVIDED_BY_ZERO` flag is not changed even if this current operation
    /// does not cause divided_by_zero.
    /// 
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [saturating_rem_assign()](struct@BigUInt#method.saturating_rem_assign)
    /// is proper rather than this method `saturating_rem_assign_uint()`.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let mut divisor = 87_u16;
    /// println!("Originally,\na_biguint = {}", a_biguint);
    /// a_biguint.saturating_rem_assign_uint(divisor);
    /// println!("After a_biguint.saturating_rem_assign_uint({}),\na_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "8");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_inifinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// divisor = 0_u16;
    /// a_biguint.saturating_rem_assign_uint(divisor);
    /// println!("After a_biguint.saturating_rem_assign_uint({}),\na_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint, UU32::zero());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_inifinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn saturating_rem_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let (_, remainder) = self.divide_fully_uint(rhs);
        self.set_uint(remainder);
        if rhs == U::zero()
            { self.set_divided_by_zero(); }
    }

    // pub fn modular_rem_uint<U>(&self, rhs: U, modulo: &Self) -> U
    /// Calculates the remainder when `self` % `modulo` is divided by
    /// `rhs` % `modulo`.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// - It returns the remainder of when `self` % `modulo` is divided by
    /// `rhs` % `modulo` if `rhs` % `modulo` is not zero.
    /// - If `rhs` % `modulo` is zero, it returns `zero`.
    /// 
    /// # Features
    /// - Overflow will not happen unless `rhs` % `modulo` is zero.
    /// - If `rhs` % `modulo` is zero, the remaindere will be `zero` of `U`
    /// type.
    /// - __It does not panic__ even if `rhs` % `modulo` is zero.
    /// - If `modulo` is `zero`, the flags such as `OVERFLOW`,
    /// `DIVIDED_BY_ZERO`, and `INFINITY` will be set.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [modular_rem()](struct@BigUInt#method.modular_rem)
    /// is proper rather than this method `modular_rem_uint()`.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let mut divisor = 128_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// let mut remainder = dividend.modular_rem_uint(divisor, &modulo);
    /// println!("{} % {} = {}", dividend, divisor, remainder);
    /// assert_eq!(remainder.to_string(), "8");
    /// 
    /// divisor = 200_u8;
    /// remainder = dividend.modular_rem_uint(divisor, &modulo);
    /// println!("{} % {} = {}", dividend, divisor, remainder);
    /// assert_eq!(remainder.to_string(), "0");
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn modular_rem_uint<U>(&self, rhs: U, modulo: &Self) -> U
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.modular_rem(&Self::from_uint(rhs), modulo).into_uint::<U>()
    }

    // pub fn modular_rem_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    /// Calculates the remainder when `self` % `modulo` is divided by
    /// `rhs` % `modulo`.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - Overflow will not happen unless `rhs` is zero.
    /// - If `rhs` % `modulo` is zero, `self` which is remainder will be `zero`,
    /// and its flag `DIVIDED_BY_ZERO` will be set.
    /// - __It does not panic__ even if `rhs` % `modulo` is zero.
    /// - If `modulo` is `zero`, the flags such as `OVERFLOW`, `DIVIDED_BY_ZERO`,
    /// and `INFINITY` will be set.
    /// - All the flags are historical, which means, for example, if an
    /// divided_by_zero occurred even once before this current operation or
    /// `DIVIDED_BY_ZERO` flag is already set before this current operation,
    /// the `DIVIDED_BY_ZERO` flag is not changed even if this current operation
    /// does not cause divided_by_zero.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [modular_rem_assign()](struct@BigUInt#method.modular_rem_assign)
    /// is proper rather than this method `modular_rem_assign_uint()`.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let mut divisor = 128_u8;
    /// let modulo = UU32::from_uint(100_u8);
    /// println!("Originally,\na_biguint = {}", a_biguint);
    /// a_biguint.modular_rem_assign_uint(divisor, &modulo);
    /// println!("After a_biguint.modular_rem_assign_uint({}),\na_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "8");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_inifinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// divisor = 200_u8;
    /// a_biguint.modular_rem_assign_uint(divisor, &modulo);
    /// println!("After a_biguint.modular_rem_assign_uint({}),\na_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_inifinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn modular_rem_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.modular_rem_assign(&Self::from_uint(rhs), modulo);
    }

    // pub fn next_multiple_of_uint<U>(&self, rhs: U) -> Self
    /// Calculates the smallest value greater than or equal to `self` that is
    /// a multiple of `rhs`.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// - This function will panic if rhs is zero.
    /// 
    /// # Output
    /// It returns the smallest value greater than or equal to self that is
    /// a multiple of `rhs`. However, if overflow occurs, it returns the value
    /// wrapped around.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [next_multiple_of()](struct@BigUInt#method.next_multiple_of)
    /// is proper rather than this method `next_multiple_of_uint()`.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// let mut num = 586478_u32;
    /// let mut multiple = a_biguint.next_multiple_of_uint(num);
    /// println!("The next multiple of {} is {}", a_biguint, multiple);
    /// assert_eq!(multiple.to_string(), "123456789012345678901234567890123697594");
    /// assert_eq!(multiple.is_overflow(), false);
    /// 
    /// a_biguint = U256::from_str_radix("FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF", 16).unwrap();
    /// multiple = a_biguint.next_multiple_of_uint(num);
    /// println!("The next multiple of {} is {}", a_biguint, multiple);
    /// assert_eq!(multiple.to_string(), "448670");
    /// assert_eq!(multiple.is_overflow(), true);
    /// 
    /// num = 0_u32;
    /// // It will panic.
    /// // multiple = a_biguint.next_multiple_of_uint(num);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn next_multiple_of_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
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
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// - This function will panic if rhs is zero.
    /// 
    /// # Features
    /// - `self` will be the smallest value greater than or equal to self that is
    /// a multiple of `rhs`. However, if overflow occurs, `self` will be the
    /// value wrapped around.
    /// - All the flags are historical, which means, for example, if an
    /// overflow occurred even once before this current operation or
    /// `OVERFLOW` flag is already set before this current operation,
    /// the `OVERFLOW` flag is not changed even if this current operation
    /// does not cause overflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [next_multiple_of_assign()](struct@BigUInt#method.next_multiple_of_assign)
    /// is proper rather than this method `next_multiple_of_assign_uint()`.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    /// let mut num = 586478_u32;
    /// 
    /// println!("Originally,\na_biguint = {}", a_biguint);
    /// a_biguint.next_multiple_of_assign_uint(num);
    /// println!("After a_biguint.next_multiple_of_assign_uint({}),\na_biguint = {}", num, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "123456789012345678901234567890123697594");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// 
    /// a_biguint = UU32::from_str_radix("FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF", 16).unwrap();
    /// println!("Originally,\na_biguint = {}", a_biguint);
    /// a_biguint.next_multiple_of_assign_uint(num);
    /// println!("After a_biguint.next_multiple_of_assign_uint({}),\na_biguint = {}", num, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "448670");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// 
    /// num = 0_u32;
    /// // It will panic.
    /// // a_biguint.next_multiple_of_assign_uint(num);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn next_multiple_of_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
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
        else    // if U::size_in_bytes() <= T::size_in_bytes()
        {
            let trhs = T::num(rhs);
            let r = self.wrapping_rem_uint(trhs);
            if r != T::zero()
            {
                self.wrapping_add_assign_uint(trhs - r);
            }
        }
    }

    

    /***** METHODS FOR EXPONENTIATION AND LOGARITHM WITH UINT *****/

    // pub fn pow_uint<U>(&self, exp: U) -> Self
    /// Raises `self` to the power of exp, using exponentiation
    /// of type `BigUInt` by squaring. The type `U` has the trait `SmallUInt`.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the result of `self` raised to the power of `exp`.
    /// 
    /// # Argument
    /// The argument `exp` is the primitive unsigned integer type.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation.
    /// - Even if overflowing happens, the `OVERFLOW` flag will not be set for
    /// either `self` or `output`.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [pow()](struct@BigUInt#method.pow)
    /// is proper rather than this method `pow_uint()`.
    /// - If you need to know whether or not overflow occurs, use the method
    /// [overflowing_pow_uint()](struct@BigUInt#method.overflowing_pow_uint).
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = UU32::from_str("10").unwrap();
    /// let mut exp = 30_u16;
    /// let mut res = a_biguint.pow_uint(exp);
    /// println!("{} ** {} = {}", a_biguint, exp, res);
    /// assert_eq!(res.to_string(), "1000000000000000000000000000000");
    /// assert_eq!(res.is_overflow(), false);
    /// 
    /// exp = 100_u16;
    /// res = a_biguint.pow_uint(exp);
    /// println!("{} ** {} = {}", a_biguint, exp, res);
    /// assert_eq!(res.to_string(), "60053020119642567005817971699943807522652027577520184704273238430174760927232");
    /// assert_eq!(res.is_overflow(), true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn pow_uint<U>(&self, exp: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
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
    /// The type `U` has the trait `SmallUInt`.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Argument
    /// The argument `exp` is the primitive unsigned integer type.
    /// 
    /// # Features
    /// - It calls wrapping_pow_assign_uint() internally.
    /// - If overflowing happens, the `OVERFLOW` flag will be set.
    /// - All the flags are historical, which means, for example, if an
    /// overflow occurred even once before this current operation or
    /// `OVERFLOW` flag is already set before this current operation,
    /// the `OVERFLOW` flag is not changed even if this current operation
    /// does not cause overflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [pow_assign()](struct@BigUInt#method.pow_assign)
    /// is proper rather than this method `pow_assign_uint()`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_uint(10_u8);
    /// let exp = 10_u8;
    /// 
    /// println!("Originally,\na_biguint = {}", a_biguint);
    /// a_biguint.pow_assign_uint(exp);
    /// println!("After a_biguint.pow_assign_uint({}),\na_biguint = {}", exp, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "10000000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// 
    /// a_biguint.pow_assign_uint(exp);
    /// println!("After a_biguint.pow_assign_uint({}),\na_biguint = {}", exp, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "60053020119642567005817971699943807522652027577520184704273238430174760927232");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn pow_assign_uint<U>(&mut self, exp: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
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
    /// type. The type `U` has the trait `SmallUInt`.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the result of `self` raised to the power of `exp`.
    /// 
    /// # Argument
    /// The argument `exp` is the primitive unsigned integer type.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation.
    /// - Even if overflowing happens, the `OVERFLOW` flag will not be set for
    /// either `self` or `output`.
    /// 
    /// # Counterpart Method
    /// - If `rhs` is bigger than `u128`, the method
    /// [wrapping_pow_uint()](struct@BigUInt#method.wrapping_pow_uint)
    /// is proper rather than this method `wrapping_pow()`.
    /// - If you need to know whether or not overflow occurs, use the method
    /// [overflowing_pow_uint()](struct@BigUInt#method.overflowing_pow_uint).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let mut exp = 30_u32;
    /// let mut res = a_biguint.wrapping_pow_uint(exp);
    /// println!("{} ** {} = {}", a_biguint, exp, res);
    /// assert_eq!(res.to_string(), "1000000000000000000000000000000");
    /// assert_eq!(res.is_overflow(), false);
    /// 
    /// exp = 100_u32;
    /// res = a_biguint.wrapping_pow_uint(exp);
    /// println!("{} ** {} = {}", a_biguint, exp, res);
    /// assert_eq!(res.to_string(), "60053020119642567005817971699943807522652027577520184704273238430174760927232");
    /// assert_eq!(res.is_overflow(), true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn wrapping_pow_uint<U>(&self, exp: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut res = Self::from_array(self.get_number().clone());
        res.wrapping_pow_assign_uint(exp);
        res.reset_all_flags();
        res
    }

    // pub fn wrapping_pow_assign_uint<U>(&mut self, exp: U)
    /// Raises `self` to the power of `exp`, using exponentiation
    /// of primitive unsigned integer type by squaring, wrapping around at the
    /// boundary of the type, and assign the result to `self` back.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Argument
    /// The argument `exp` is the primitive unsigned integer type.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation.
    /// - If overflowing happens, the `OVERFLOW` flag will be set.
    /// - All the flags are historical, which means, for example, if an
    /// overflow occurred even once before this current operation or
    /// `OVERFLOW` flag is already set before this current operation,
    /// the `OVERFLOW` flag is not changed even if this current operation
    /// does not cause overflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [wrapping_pow_assign()](struct@BigUInt#method.wrapping_pow_assign)
    /// is proper rather than this method `wrapping_pow_assign_uint()`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_uint(10_u8);
    /// let exp = 10_u8;
    /// 
    /// println!("Originally,\na_biguint = {}", a_biguint);
    /// a_biguint.wrapping_pow_assign_uint(exp);
    /// println!("After a_biguint.wrapping_pow_assign_uint({}),\na_biguint = {}", exp, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "10000000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// 
    /// a_biguint.wrapping_pow_assign_uint(exp);
    /// println!("After a_biguint.wrapping_pow_assign_uint({}),\na_biguint = {}", exp, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "60053020119642567005817971699943807522652027577520184704273238430174760927232");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn wrapping_pow_assign_uint<U>(&mut self, exp: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if self.is_zero_or_one()
            { return; }

        let zero = U::zero();
        let one = U::one();
        let multiplier = self.clone();
        self.set_one();
        if exp == zero
            { return; }

        let mut bit_check = one << U::usize_as_smalluint(exp.length_in_bits() - 1 - exp.leading_zeros() as usize);
        if bit_check != zero
        {
            self.wrapping_mul_assign(&multiplier);
            bit_check >>= one;
        }
        while bit_check != zero
        {
            self.wrapping_mul_assign(&self.clone());
            if (bit_check & exp) != zero
                { self.wrapping_mul_assign(&multiplier); }
            bit_check >>= one;
        }
    }

    // pub fn overflowing_pow_uint<U>(&self, exp: U) -> (Self, bool)
    /// Raises `BigUInt` type number to the power of `exp`, using exponentiation
    /// of primitive unsigned integer type by squaring.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns a tuple of the exponentiation along with a bool indicating
    /// whether an overflow happened. The second term of the tuple output
    /// has flags reset.
    ///
    /// # Argument
    /// The argument `exp` is the primitive unsigned integer type.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation.
    /// - Even if overflowing happens, the `OVERFLOW` flag will not be set for
    /// either `self` or `output`.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [overflowing_pow()](struct@BigUInt#method.overflowing_pow)
    /// is proper rather than this method `overflowing_pow_uint()`.
    /// - If you do not need to know whether or not overflow occurs, use the
    /// method [wrapping_pow_uint()](struct@BigUInt#method.wrapping_pow_uint).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let mut exp = 30_u32;
    /// let mut res = a_biguint.overflowing_pow_uint(exp);
    /// println!("{} ** {} = {}\noverflow = {}", a_biguint, exp, res.0, res.1);
    /// assert_eq!(res.0.to_string(), "1000000000000000000000000000000");
    /// assert_eq!(res.1, false);
    /// assert_eq!(res.0.is_overflow(), false);
    /// 
    /// exp = 100_u32;
    /// res = a_biguint.overflowing_pow_uint(exp);
    /// println!("{} ** {} = {}\noverflow = {}", a_biguint, exp, res.0, res.1);
    /// assert_eq!(res.0.to_string(), "60053020119642567005817971699943807522652027577520184704273238430174760927232");
    /// assert_eq!(res.1, true);
    /// assert_eq!(res.0.is_overflow(), true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn overflowing_pow_uint<U>(&self, exp: U) -> (Self, bool)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut res = Self::from_array(self.get_number().clone());
        let overflow = res.overflowing_pow_assign_uint(exp);
        res.reset_all_flags();
        (res, overflow)
    }
    
    // pub fn overflowing_pow_assign_uint<U>(&mut self, exp: U) -> bool
    /// Raises `BigUInt` type number to the power of `exp`, using exponentiation
    /// of primitive unsigned integer type by squaring, and returns the result
    /// to `self` back.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns a bool indicating whether or not an overflow happened.
    /// It is the current overflow which has nothing to do with historical
    /// ovrerflow of `self`.
    ///
    /// # Argument
    /// The argument `exp` is the primitive unsigned integer type.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation.
    /// - If overflowing happens, the `OVERFLOW` flag will be set.
    /// - If overflowing did not happen in the current operation, the output
    /// will be false even if the `OVERFLOW` flag of `self` was already set
    /// because of previous operation of `self`.
    /// - All the flags are historical, which means, for example, if an
    /// overflow occurred even once before this current operation or
    /// `OVERFLOW` flag is already set before this current operation,
    /// the `OVERFLOW` flag is not changed even if this current operation
    /// does not cause overflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [overflowing_pow_assign()](struct@BigUInt#method.overflowing_pow_assign)
    /// is proper rather than this method `overflowing_pow_assign_uint()`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U256::from_uint(10_u8);
    /// let mut exp = 30_u32;
    /// 
    /// println!("Originally,\na_biguint = {}", a_biguint);
    /// let mut overflow = a_biguint.overflowing_pow_assign_uint(exp);
    /// println!("After a_biguint.overflowing_pow_assign_uint({}),\na_biguint = {}\noverflow = {}", exp, a_biguint, overflow);
    /// assert_eq!(a_biguint.to_string(), "1000000000000000000000000000000");
    /// assert_eq!(overflow, false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// 
    /// exp = 3_u32;
    /// overflow = a_biguint.overflowing_pow_assign_uint(exp);
    /// println!("After a_biguint.overflowing_pow_assign_uint({}),\na_biguint = {}\noverflow = {}", exp, a_biguint, overflow);
    /// assert_eq!(a_biguint.to_string(), "51484102413631087777415798035541167055393351402420714880745735202410401366016");
    /// assert_eq!(overflow, true);
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// 
    /// exp = 0_u32;
    /// overflow = a_biguint.overflowing_pow_assign_uint(exp);
    /// println!("After a_biguint.overflowing_pow_assign_uint({}),\na_biguint = {}\noverflow = {}", exp, a_biguint, overflow);
    /// assert_eq!(a_biguint.to_string(), "1");
    /// assert_eq!(overflow, false);
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn overflowing_pow_assign_uint<U>(&mut self, exp: U) -> bool
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let old_flags = self.get_all_flags();
        self.reset_overflow();
        self.wrapping_pow_assign_uint(exp);
        let current_overflow = self.is_overflow();
        let current_flag = self.get_all_flags();
        self.set_flag_bit(old_flags | current_flag);
        current_overflow
    }

    // pub fn checked_pow_uint<U>(&self, exp: U) -> Option<Self>
    /// Raises `BigUInt` type number to the power of exp, using exponentiation
    /// of type `BigUInt` by squaring, wrapping around at the boundary of the
    /// type. The type `U` has the trait `SmallUInt`.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the result of `self` raised to the power of `exp`,
    /// wrapped by `Some` of enum `Option` if overflow does not occur.
    /// If overflow occurs, it returns `None` of enum `Option`.
    /// 
    /// # Argument
    /// The argument `exp` is the primitive unsigned integer type.
    /// 
    /// # Features
    /// Wrapping (modular) exponentiation.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [checked_pow()](struct@BigUInt#method.checked_pow)
    /// is proper rather than this method `checked_pow_uint()`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let mut exp = 30_u32;
    /// let mut res = a_biguint.checked_pow_uint(exp);
    /// match res
    /// {
    ///     Some(raised) => {
    ///             println!("{} ** {} = {}", a_biguint, exp, raised);
    ///             assert_eq!(raised.to_string(), "1000000000000000000000000000000");
    ///             assert_eq!(raised.is_overflow(), false);
    ///         },
    ///     None => { println!("Overflow"); }
    /// }
    /// 
    /// exp = 100_u32;
    /// res = a_biguint.checked_pow_uint(exp);
    /// match res
    /// {
    ///     Some(raised) => { println!("{} ** {} = {}", a_biguint, exp, raised); },
    ///     None => {
    ///             println!("Overflow");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn checked_pow_uint<U>(&self, exp: U) -> Option<Self>
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let (res, overflow) = self.overflowing_pow_uint(exp);
        if overflow
            { None }
        else
            { Some(res) }
    }
   
    // pub fn unchecked_pow_uint<U>(&self, exp: U) -> Self
    /// Raises `BigUInt` type number to the power of exp, using exponentiation
    /// of type `BigUInt` by squaring, wrapping around at the boundary of the
    /// type. The type `U` has the trait `SmallUInt`.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// - If overflow occurs, it will panic.
    /// 
    /// # Output
    /// It returns the result of `self` raised to the power of `exp`
    /// if overflow does not occur. If overflow occurs, it will panic.
    /// 
    /// # Argument
    /// The argument `exp` is the primitive unsigned integer type.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [unchecked_pow()](struct@BigUInt#method.unchecked_pow)
    /// is proper rather than this method `unchecked_pow_uint()`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let mut exp = 30_u32;
    /// let mut res = a_biguint.unchecked_pow_uint(exp);
    /// println!("{} ** {} = {}", a_biguint, exp, res);
    /// assert_eq!(res.to_string(), "1000000000000000000000000000000");
    /// assert_eq!(res.is_overflow(), false);
    /// 
    /// exp = 100_u32;
    // It will panic.
    // res = a_biguint.unchecked_pow_uint(exp);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn unchecked_pow_uint<U>(&self, exp: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.checked_pow_uint(exp).unwrap()
    }

    // pub fn saturating_pow_uint<U>(&self, exp: U) -> Self
    /// Raises `BigUInt` type number to the power of exp, using exponentiation
    /// of type `BigUInt` by squaring, saturating at the numeric bounds
    /// instead of overflowing. The type `U` has the trait `SmallUInt`.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the result of `self` raised to the power of `exp`.
    /// It returns the maximum value instead of overflowing.
    /// 
    /// # Argument
    /// The argument `exp` is the primitive unsigned integer type.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation.
    /// - Overflowing never happens.
    /// - This method saturates when it reaches maximum value.
    /// - It does not set `OVERFLOW` flag.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [saturating_pow()](struct@BigUInt#method.saturating_pow)
    /// is proper rather than this method `saturating_pow_uint()`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_uint(10_u8);
    /// let mut exp = 30_u32;
    /// let mut res = a_biguint.saturating_pow_uint(exp);
    /// println!("{} ** {} = {}", a_biguint, exp, res);
    /// assert_eq!(res.to_string(), "1000000000000000000000000000000");
    /// assert_eq!(res.is_overflow(), false);
    /// 
    /// exp = 100_u32;
    /// res = a_biguint.saturating_pow_uint(exp);
    /// println!("{} ** {} = {}", a_biguint, exp, res);
    /// assert_eq!(res, UU32::max());
    /// assert_eq!(res.is_overflow(), false);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn saturating_pow_uint<U>(&self, exp: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut res = Self::from_array(self.get_number().clone());
        res.saturating_pow_assign_uint(exp);
        res
    }

    // pub fn saturating_pow_assign_uint<U>(&self, exp: U) -> Self
    /// Raises `BigUInt` type number to the power of exp, using exponentiation
    /// of type `BigUInt` by squaring, saturating at the numeric bounds
    /// instead of overflowing, and returns the result to `self` back.
    /// The type `U` has the trait `SmallUInt`.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Argument
    /// The argument `exp` is the primitive unsigned integer type.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation.
    /// - Overflowing never happens.
    /// - `self` will be the maximum value instead of overflowing.
    /// - This method saturates when it reaches maximum value.
    /// - It does not set `OVERFLOW` flag.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [saturating_pow_assign()](struct@BigUInt#method.saturating_pow_assign)
    /// is proper rather than this method `saturating_pow_assign_uint()`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = UU32::from_uint(10_u8);
    /// let mut exp = 30_u32;
    /// println!("Originally,\na_biguint = {}", a_biguint);
    /// a_biguint.saturating_pow_assign_uint(exp);
    /// println!("After a_biguint.overflowing_pow_assign_uint({}),\na_biguint = {}", exp, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1000000000000000000000000000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// 
    /// a_biguint.saturating_pow_assign_uint(exp);
    /// println!("After a_biguint.overflowing_pow_assign_uint({}),\na_biguint = {}", exp, a_biguint);
    /// assert_eq!(a_biguint, UU32::max());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn saturating_pow_assign_uint<U>(&mut self, exp: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let overflow = self.is_overflow();
        if self.overflowing_pow_assign_uint(exp)
        {
            self.set_max();
            if !overflow
                { self.reset_overflow(); }
        }
    }

    // pub fn modular_pow_uint<U>(&self, exp: U, modulo: &Self) -> Self
    /// Raises `BigUInt` type number to the power of exp, using exponentiation
    /// of type `U` by squaring, wrapping around at `modulo` of the
    /// type `U`. The type `U` has the trait `SmallUInt`.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the result of `self` raised to the power of `exp`, wrapping
    /// around at `modulo`. If `modulo` is `zero`, it returns maximum value.
    /// 
    /// # Argument
    /// The argument `exp` is the primitive unsigned integer type.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation, wrapping around at `modulo`.
    /// - If `modulo` is `zero`, the flags such as `OVERFLOW`, `DIVIDED_BY_ZERO`,
    /// and `INFINITY` will be set.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, use the method
    /// [modular_pow()](struct@BigUInt#method.modular_pow) instead.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(10_u8);
    /// let mut exp = 30_u32;
    /// let mut modulo = U256::halfmax();
    /// let mut res = a_biguint.modular_pow_uint(exp, &modulo);
    /// println!("{} ** {} (mod {}) = {}", a_biguint, exp, modulo, res);
    /// assert_eq!(res.to_string(), "1000000000000000000000000000000");
    /// assert_eq!(res.is_overflow(), false);
    /// 
    /// exp = 100_u32;
    /// res = a_biguint.modular_pow_uint(exp, &modulo);
    /// println!("{} ** {} (mod {}) = {}", a_biguint, exp, modulo, res);
    /// assert_eq!(res.to_string(), "59749648429786538521694772865754025520");
    /// assert_eq!(res.is_overflow(), false);
    /// 
    /// modulo.set_zero();
    /// res = a_biguint.modular_pow_uint(exp, &modulo);
    /// println!("{} ** {} (mod {}) = {}", a_biguint, exp, modulo, res);
    /// assert_eq!(res, U256::max());
    /// assert_eq!(res.is_overflow(), true);
    /// assert_eq!(res.is_inifinity(), true);
    /// assert_eq!(res.is_divided_by_zero(), true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn modular_pow_uint<U>(&self, exp: U, modulo: &Self) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut res = Self::from_array(self.get_number().clone());
        res.modular_pow_assign_uint(exp, modulo);
        res
    }

    // pub fn modular_pow_assign_uint<U>(&mut self, exp: U, modulo: &Self)
    /// Raises `BigUInt` type number to the power of `exp`, using exponentiation
    /// of primitive unsigned integer type by squaring, wrapping around at
    /// `modulo`, and assign the result to `self` back.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Argument
    /// The argument `exp` is the primitive unsigned integer type.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation, wrapping around at `modulo`.
    /// - If `modulo` is `zero`, `self` will have the maximum value and
    /// the flags such as `OVERFLOW`, `DIVIDED_BY_ZERO`,
    /// and `INFINITY` will be set.
    /// - All the flags are historical, which means, for example, if an
    /// overflow occurred even once before this current operation or
    /// `OVERFLOW` flag is already set before this current operation,
    /// the `OVERFLOW` flag is not changed even if this current operation
    /// does not cause overflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [modular_pow_assign()](struct@BigUInt#method.modular_pow_assign)
    /// is proper rather than this method.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_uint(10_u8);
    /// let mut exp = 30_u32;
    /// let mut modulo = U256::halfmax();
    /// println!("Originally,\na_biguint = {}", a_biguint);
    /// a_biguint.modular_pow_assign_uint(exp, &modulo);
    /// println!("After a_biguint.modular_pow_assign_uint({}),\na_biguint = {}", exp, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1000000000000000000000000000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// 
    /// exp = 100_u32;
    /// a_biguint.modular_pow_assign_uint(exp, &modulo);
    /// println!("After a_biguint.modular_pow_assign_uint({}),\na_biguint = {}", exp, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "52266245075570873327294567809656160090");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// 
    /// modulo.set_zero();
    /// a_biguint.modular_pow_assign_uint(exp, &modulo);
    /// println!("After a_biguint.modular_pow_assign_uint({}),\na_biguint = {}", exp, a_biguint);
    /// assert_eq!(a_biguint, U256::max());
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_inifinity(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn modular_pow_assign_uint<U>(&mut self, exp: U, modulo: &Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if modulo.is_zero()
        {
            self.set_max();
            self.set_overflow();
            self.set_infinity();
            self.set_divided_by_zero();
            return;
        }
        if *self >= *modulo
            { self.wrapping_rem_assign(modulo); }
        let mut acc = Self::from_array(self.get_number().clone());
        self.set_one();
        let mut mexp = exp;
        while mexp > U::zero()
        {
            if mexp.is_odd()
                { self.modular_mul_assign(&acc, modulo); }
                acc.modular_mul_assign(&acc.clone(), modulo);
            mexp >>= U::one();
        }
    }

    // pub fn root_uint<U>(&self, exp: U) -> Self
    /// 
    #[inline]
    pub fn root_uint<U>(&self, exp: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.wrapping_root_uint(exp)
    }

    pub fn root_assign_uint<U>(&mut self, _exp: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        
    }

    pub fn wrapping_root_uint<U>(&self, exp: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut adder;
        let mut highest = ((Self::size_in_bits() as u128 - self.leading_zeros() as u128).wrapping_div(exp.into_u128())) as usize;
        let mut high;
        let mut low;
        let mut mid;
        let mut res = Self::zero();
        let mut sum;
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
                    adder = Self::generate_check_bits_(mid);
                    sum = res.wrapping_add(&adder);
                    let (sq, b_overflow) = sum.overflowing_pow_uint(exp);
                    if !b_overflow && (sq < *self)
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
                    else if b_overflow || (sq > *self)
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

    pub fn wrapping_root_assign_uint<U>(&mut self, _exp: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        
    }

    pub fn overflowing_root_uint<U>(&self, exp: U) -> (Self, bool)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        (self.wrapping_root_uint(exp), false)
    }
    
    pub fn overflowing_root_assign_uint<U>(&mut self, _exp: U) -> bool
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        false
    }

    pub fn checked_root_uint<U>(&self, exp: U) -> Option<Self>
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        Some(self.root_uint(exp))
    }

    pub fn unchecked_root_uint<U>(&self, exp: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.root_uint(exp)
    }

    pub fn saturating_root_uint<U>(&self, exp: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.root_uint(exp)
    }

    pub fn saturating_root_assign_uint<U>(&mut self, _exp: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        
    }

    // pub fn ilog_uint<U>(&self, base: U) -> Self
    /// Calculates the logarithm of the number with respect to a `base`.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// - This function will panic if `self` is zero,
    /// or if `base` is less than 2.
    /// 
    /// # Output
    /// It returns the logarithm of the number with respect to an arbitrary
    /// `base`, rounded down.
    /// 
    /// # Counterpart Methods
    /// This method might not be optimized owing to implementation details;
    /// [ilog2()](struct@BigUInt#method.ilog2) can produce results more
    /// efficiently for base 2, and [ilog10()](struct@BigUInt#method.ilog10)
    /// can produce results more efficiently for base 10.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn ilog_uint<U>(&self, base: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
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

    pub fn ilog_assign_uint<U>(&mut self, _base: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {

    }

    pub fn wrapping_ilog_uint<U>(&self, base: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.ilog_uint(base)
    }

    pub fn wrapping_ilog_assign_uint<U>(&mut self, _base: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        
    }

    pub fn overflowing_ilog_uint<U>(&self, base: U) -> (Self, bool)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut res = Self::from_array(self.get_number().clone());
        let overflow = res.overflowing_ilog_assign_uint(base);
        (res, overflow)
    }

    pub fn overflowing_ilog_assign_uint<U>(&mut self, _base: U) -> bool
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        true
    }

    // pub fn checked_ilog_uint<U>(&self, base: U) -> Option<Self>
    /// Calculates the logarithm of the number with respect to a `base`.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the logarithm of the number with respect to an arbitrary
    /// `base`, rounded down, wrapped by `Some` of enum `Option`.
    /// It returns `None` if `self` is zero, or if `base` is less than 2.
    /// 
    /// # Counterpart Methods
    /// This method might not be optimized owing to implementation details;
    /// [checked_ilog2()](struct@BigUInt#method.checked_ilog2) can produce
    /// results more efficiently for base 2, and
    /// [checked_ilog10()](struct@BigUInt#method.checked_ilog10) can produce
    /// results more efficiently for base 10.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn checked_ilog_uint<U>(&self, base: U) -> Option<Self>
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
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

    pub fn unchecked_ilog_uint<U>(&self, base: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.ilog_uint(base)
    }

    pub fn saturating_ilog_uint<U>(&self, base: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.ilog_uint(base)
    }

    pub fn saturating_ilog_assign_uint<U>(&mut self, _base: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        
    }

    pub fn ilog2_uint(&self) -> Self
    {
        self.wrapping_ilog2_uint()
    }

    pub fn wrapping_ilog2_uint(&self) -> Self
    {
        self.wrapping_ilog_uint(2_u8)
    }


    pub fn wrapping_ilog2_assign_uint(&mut self)
    {
        self.wrapping_ilog_assign_uint(2_u8)
    }

    pub fn overflowing_ilog2_uint(&self) -> (Self, bool)
    {
        let mut res = Self::from_array(self.get_number().clone());
        let overflow = res.overflowing_ilog2_assign_uint();
        (res, overflow)
    }

    pub fn overflowing_ilog2_assign_uint(&mut self) -> bool
    {
        let flags = self.get_all_flags();
        self.reset_all_flags();
        self.wrapping_ilog2_assign_uint();
        let underflow = self.is_underflow();
        self.set_flag_bit(flags);
        underflow
    }

    // pub fn checked_ilog2_uint<U>(&self) -> Option<Self>
    /// Calculates the logarithm of the number with respect to a `base`.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the logarithm of the number with respect to an arbitrary
    /// `base`, rounded down, wrapped by `Some` of enum `Option`.
    /// It returns `None` if `self` is zero, or if `base` is less than 2.
    /// 
    /// # Counterpart Methods
    /// This method might not be optimized owing to implementation details;
    /// [checked_ilog2()](struct@BigUInt#method.checked_ilog2) can produce
    /// results more efficiently for base 2, and
    /// [checked_ilog10()](struct@BigUInt#method.checked_ilog10) can produce
    /// results more efficiently for base 10.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn checked_ilog2_uint(&self) -> Option<Self>
    {
        self.checked_ilog_uint(2_u8)
    }

    pub fn unchecked_ilog2_uint(&self) -> Self
    {
        self.unchecked_ilog_uint(2_u8)
    }

    pub fn saturating_ilog2_uint(&self) -> Self
    {
        self.saturating_ilog_uint(2_u8)
    }

    pub fn saturating_ilog2_assign_uint(&mut self)
    {
        self.saturating_ilog_assign_uint(2_u8)
    }

    pub fn ilog10_uint(&self) -> Self
    {
        self.ilog_uint(10_u8)
    }

    pub fn wrapping_ilog10_uint(&self) -> Self
    {
        self.wrapping_ilog_uint(10_u8)
    }


    pub fn wrapping_ilog10_assign_uint(&mut self)
    {
        self.wrapping_ilog_assign_uint(10_u8)
    }

    pub fn overflowing_ilog10_uint(&self) -> (Self, bool)
    {
        let mut res = Self::from_array(self.get_number().clone());
        let overflow = res.overflowing_ilog10_assign_uint();
        (res, overflow)
    }

    pub fn overflowing_ilog10_assign_uint(&mut self) -> bool
    {
        self.overflowing_ilog_assign_uint(10_u8)
    }

    // pub fn checked_ilog2_uint<U>(&self, base: U) -> Option<Self>
    /// Calculates the logarithm of the number with respect to a `base`.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the logarithm of the number with respect to an arbitrary
    /// `base`, rounded down, wrapped by `Some` of enum `Option`.
    /// It returns `None` if `self` is zero, or if `base` is less than 2.
    /// 
    /// # Counterpart Methods
    /// This method might not be optimized owing to implementation details;
    /// [checked_ilog2()](struct@BigUInt#method.checked_ilog2) can produce
    /// results more efficiently for base 2, and
    /// [checked_ilog10()](struct@BigUInt#method.checked_ilog10) can produce
    /// results more efficiently for base 10.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn checked_ilog10_uint(&self) -> Option<Self>
    {
        self.checked_ilog_uint(10_u8)
    }

    pub fn unchecked_ilog10_uint(&self) -> Self
    {
        self.unchecked_ilog_uint(10_u8)
    }

    pub fn saturating_ilog10_uint(&self) -> Self
    {
        self.saturating_ilog_uint(10_u8)
    }

    pub fn saturating_ilog10_assign_uint(&mut self)
    {
        self.saturating_ilog_assign_uint(10_u8);
    }



    /***** ARITHMATIC OPERATIONS WITH BIGUINT *****/

    /*** ADDITION ***/

    // pub fn carrying_add(&self, rhs: &Self, carry: bool) -> (Self, bool)
    /// Calculates `self` + `rhs` + `carry`,
    /// wrapping around at the boundary of the type.
    /// 
    /// # Features
    /// - This allows chaining together multiple additions to create even a wider
    /// addition. This can be thought of as a big integer “full adder”,
    /// in the electronics sense.
    /// - If the input carry is `false`, this method is equivalent to
    /// `overflowing_add()`.
    /// - The output carry is equal to the `OVERFLOW` flag.
    /// 
    /// # Outputs
    /// It returns a tuple containing the sum and the output carry. It performs
    /// “ternary addition” of two big integer operands and a carry-in bit, and
    /// returns an output big integer and a carry-out bit.
    /// 
    /// # Counterpart Method
    /// - The method
    /// [carrying_add_uint()](struct@BigUInt#method.carrying_add_uint)
    /// is a bit faster than this method `carrying_add()`.
    /// - If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, you are highly encouraged to use the method
    /// [carrying_add_uint()](struct@BigUInt#method.carrying_add_uint).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_hi = U256::from_str_radix("15D5_ECE4_41DB_7709_BA44_8C40_0DCF_7160_3CD4_F7FF_F0CF_476F_33FD_438B_0E1D_2086", 16).unwrap();
    /// let a_lo = U256::from_str_radix("C9B4_EF7B_BBC9_F60E_45CB_EE41_B567_A641_7D69_A0EC_05F7_65A7_F81B_5C91_72DC_BAC0", 16).unwrap();
    /// let b_hi = U256::from_str_radix("274_DDD9_4DAA_9405_B621_6BCA_AF43_78E3_0FA6_1D7D_86F4_0D17_2C18_A01C_80F9_DB46", 16).unwrap();
    /// let b_lo = U256::from_str_radix("DF8A_DC5F_FDA5_6D18_0010_7A81_C337_17A1_BA3E_98EB_F6C6_AD17_2C18_A01C_80F9_DB46", 16).unwrap();
    /// 
    /// let (c_lo, carry) = a_lo.carrying_add(&b_lo, false);
    /// let (c_hi, overflow) = a_hi.carrying_add(&b_hi, carry);
    /// 
    /// println!("{}:{} + {}:{} = {}:{}", a_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_lo.to_string_with_radix_and_stride(16, 4).unwrap(), b_hi.to_string_with_radix_and_stride(16, 4).unwrap(), b_lo.to_string_with_radix_and_stride(16, 4).unwrap(), c_hi.to_string_with_radix_and_stride(16, 4).unwrap(), c_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    /// println!("carry = {}, overflow = {}", carry, overflow);
    /// 
    /// assert_eq!(c_hi.to_string_with_radix_and_stride(16, 4).unwrap(), "184A_CABD_8F86_0B0F_7065_F80A_BD12_EA43_4C7B_157D_77C3_5486_6015_E3A7_8F16_FBCD");
    /// assert_eq!(c_lo.to_string_with_radix_and_stride(16, 4).unwrap(), "A93F_CBDB_B96F_6326_45DC_68C3_789E_BDE3_37A8_39D7_FCBE_12BF_2433_FCAD_F3D6_9606");
    /// assert_eq!(carry, true);
    /// assert_eq!(c_lo.is_overflow(), true);
    /// assert_eq!(overflow, false);
    /// assert_eq!(c_hi.is_overflow(), false);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn carrying_add(&self, rhs: &Self, carry: bool) -> (Self, bool)
    {
        let mut res = Self::from_array(self.get_number().clone());
        let c = res.carrying_add_assign(rhs, carry);
        (res, c)
    }

    // pub fn carrying_add_assign(&self, rhs: &Self, carry: bool) -> bool
    /// Accumulate `rhs` + `carry` to `self`, wrapping around at the boundary
    /// of the type, and return the resulting carry.
    /// 
    /// # Features
    /// - This allows chaining together multiple additions to create even a
    /// wider addition. This can be thought of as a big integer “full adder”,
    /// in the electronics sense.
    /// - If the input carry is `false`, this method is equivalent to
    /// `overflowing_add_assign()`.
    /// - All the flags are historical, which means, for example, if an
    /// overflow occurred even once before this current operation or
    /// `OVERFLOW` flag is already set before this current operation,
    /// the `OVERFLOW` flag is not changed even if this current operation
    /// does not cause overflow.
    /// 
    /// # Output
    /// It returns the output carry. It performs “ternary addition” of two big
    /// integer operands and a carry-in bit, and returns a carry-out bit.
    /// 
    /// # Counterpart Method
    /// - The method
    /// [carrying_add_assign_uint()](struct@BigUInt#method.carrying_add_assign_uint)
    /// is a bit faster than this method `carrying_add_assign()`.
    /// - If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, you are highly encouraged to use the method
    /// [carrying_add_assign_uint()](struct@BigUInt#method.carrying_add_assign_uint).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_hi = U256::from_str_radix("15D5_ECE4_41DB_7709_BA44_8C40_0DCF_7160_3CD4_F7FF_F0CF_476F_33FD_438B_0E1D_2086", 16).unwrap();
    /// let mut a_lo = U256::from_str_radix("C9B4_EF7B_BBC9_F60E_45CB_EE41_B567_A641_7D69_A0EC_05F7_65A7_F81B_5C91_72DC_BAC0", 16).unwrap();
    /// let b_hi = U256::from_str_radix("274_DDD9_4DAA_9405_B621_6BCA_AF43_78E3_0FA6_1D7D_86F4_0D17_2C18_A01C_80F9_DB46", 16).unwrap();
    /// let b_lo = U256::from_str_radix("DF8A_DC5F_FDA5_6D18_0010_7A81_C337_17A1_BA3E_98EB_F6C6_AD17_2C18_A01C_80F9_DB46", 16).unwrap();
    /// let c_hi = U256::from(1_u8);
    /// let c_lo = U256::from(1_u8);
    /// 
    /// print!("{}:{} + {}:{}", a_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_lo.to_string_with_radix_and_stride(16, 4).unwrap(), b_hi.to_string_with_radix_and_stride(16, 4).unwrap(), b_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    /// let mut carry = a_lo.carrying_add_assign(&b_lo, false);
    /// let mut overflow = a_hi.carrying_add_assign(&b_hi, carry);
    /// println!(" = {}:{}", a_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    /// println!("carry = {}, overflow = {}", carry, overflow);
    /// 
    /// assert_eq!(a_hi.to_string_with_radix_and_stride(16, 4).unwrap(), "184A_CABD_8F86_0B0F_7065_F80A_BD12_EA43_4C7B_157D_77C3_5486_6015_E3A7_8F16_FBCD");
    /// assert_eq!(a_lo.to_string_with_radix_and_stride(16, 4).unwrap(), "A93F_CBDB_B96F_6326_45DC_68C3_789E_BDE3_37A8_39D7_FCBE_12BF_2433_FCAD_F3D6_9606");
    /// assert_eq!(carry, true);
    /// assert_eq!(a_lo.is_overflow(), true);
    /// assert_eq!(overflow, false);
    /// assert_eq!(a_hi.is_overflow(), false);
    /// 
    /// print!("{}:{} + {}:{}", a_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_lo.to_string_with_radix_and_stride(16, 4).unwrap(), c_hi.to_string_with_radix_and_stride(16, 4).unwrap(), c_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    /// carry = a_lo.carrying_add_assign(&c_lo, false);
    /// overflow = a_hi.carrying_add_assign(&c_hi, carry);
    /// println!(" = {}:{}", a_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    /// println!("carry = {}, overflow = {}", carry, overflow);
    /// assert_eq!(a_hi.to_string_with_radix_and_stride(16, 4).unwrap(), "184A_CABD_8F86_0B0F_7065_F80A_BD12_EA43_4C7B_157D_77C3_5486_6015_E3A7_8F16_FBCE");
    /// assert_eq!(a_lo.to_string_with_radix_and_stride(16, 4).unwrap(), "A93F_CBDB_B96F_6326_45DC_68C3_789E_BDE3_37A8_39D7_FCBE_12BF_2433_FCAD_F3D6_9607");
    /// assert_eq!(carry, false);
    /// assert_eq!(a_lo.is_overflow(), true);
    /// assert_eq!(overflow, false);
    /// assert_eq!(a_hi.is_overflow(), false);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn carrying_add_assign(&mut self, rhs: &Self, carry: bool) -> bool
    {
        let mut c = carry;
        let mut num;
        let i_l = self.leading_zero_elements() as usize;
        let j_l = rhs.leading_zero_elements() as usize;
        let ij_n = N - if i_l < j_l {i_l} else {j_l};
        for i in 0..ij_n
        {
            (num, c) = self.get_num_(i).carrying_add(rhs.get_num_(i), c);
            self.set_num_(i, num);
        }
        if c
        {
            if ij_n < N
            {
                self.set_num_(ij_n, T::one());
                c = false;
            }
            else
            {
                self.set_overflow();
            }
        }
        c
    }

    // pub fn wrapping_add(&self, rhs: &Self) -> Self
    /// Computes `self` + `rhs`, wrapping around at the boundary of the type.
    /// 
    /// # Output
    /// It returns `self` + `rhs` with wrapping (modular) addition.
    /// 
    /// # Features
    /// Wrapping (modular) addition.
    /// 
    /// # Counterpart Method
    /// - The method
    /// [wrapping_add_uint()](struct@BigUInt#method.wrapping_add_uint)
    /// is a bit faster than this method `wrapping_add()`.
    /// - If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [wrapping_add_uint()](struct@BigUInt#method.wrapping_add_uint).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    ///  
    /// let zero = U512::zero();
    /// let one = U512::one();
    /// let two = U512::from(2_u8);
    /// let three = U512::from(3_u8);
    /// let a = U512::max() - one.clone();
    /// let b = a.wrapping_add(&one);
    /// let c = a.wrapping_add(&two);
    /// let d = a.wrapping_add(&three);
    /// 
    /// println!("{} + 1 = {}", a, b);
    /// assert_eq!(b, U512::max());
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
    /// # Features
    /// Wrapping (modular) addition.
    /// 
    /// # Counterpart Method
    /// - The method
    /// [wrapping_add_assign_uint()](struct@BigUInt#method.wrapping_add_assign_uint)
    /// is a bit faster than this method `wrapping_add_assign()`.
    /// - If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [wrapping_add_assign_uint()](struct@BigUInt#method.wrapping_add_assign_uint).
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let zero = U512::zero();
    /// let one = U512::one();
    /// 
    /// let mut a = U512::max().wrapping_sub(&one);
    /// println!("Originally,\ta = {}", a);
    /// assert_eq!(a.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    /// 
    /// a.wrapping_add_assign(&one);
    /// println!("After a += 1,\ta = {}", a);
    /// assert_eq!(a, U512::max());
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
    /// - The method
    /// [overflowing_add_uint()](struct@BigUInt#method.overflowing_add_uint)
    /// is a bit faster than this method `overflowing_add()`.
    /// - If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [overflowing_add_uint()](struct@BigUInt#method.overflowing_add_uint).
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
        let mut res = Self::from_array(self.get_number().clone());
        let current_overflow = res.overflowing_add_assign(rhs);
        (res, current_overflow)
    }

    // pub fn overflowing_add_assign(&mut self, rhs: &Self) -> bool
    /// Calculates `self` + `rhs`, and assigns the result to `self` back.
    /// 
    /// # Output
    /// It returns true if an arithmetic overflow would occur.
    /// Otherwise, it returns `false`.
    /// 
    /// # Counterpart Method
    /// - The method
    /// [overflowing_add_assign_uint()](struct@BigUInt#method.overflowing_add_assign_uint)
    /// is a bit faster than this method `overflowing_add_assign()`.
    /// - If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [overflowing_add_assign_uint()](struct@BigUInt#method.overflowing_add_assign_uint).
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
        let old_overflow = self.is_overflow();
        self.reset_overflow();
        self.wrapping_add_assign(rhs);
        let current_overflow = self.is_overflow();
        if old_overflow || current_overflow
            { self.set_overflow(); }
        current_overflow
    }

    // pub fn checked_add(&self, rhs: &Self) -> Option<Self>
    /// Computes `self` + `rhs`.
    /// 
    /// # Output
    /// It returns the sum `self` + `rhs` wrapped by `Some` of enum `Option`
    /// if overflow did not occur. Otherwise, it returns `None` of enum Option.
    /// 
    /// # Counterpart Method
    /// - The method
    /// [checked_add_uint()](struct@BigUInt#method.checked_add_uint)
    /// is a bit faster than this method `checked_add()`.
    /// - If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [checked_add_uint()](struct@BigUInt#method.checked_add_uint).
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
        let mut res = Self::from_array(self.get_number().clone());
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
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// - If overflow occurred, it will panic. So, use this method
    /// only when you are sure that overflow will not occur. 
    /// 
    /// # Output
    /// It returns the sum `self` + `rhs` if overflow did not occur.
    /// Otherwise, it will panic.
    /// 
    /// # Counterpart Method
    /// - The method
    /// [unchecked_add_uint()](struct@BigUInt#method.unchecked_add_uint)
    /// is a bit faster than this method `unchecked_add()`.
    /// - If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [unchecked_add_uint()](struct@BigUInt#method.unchecked_add_uint).
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
    /// - The method
    /// [saturating_add_uint()](struct@BigUInt#method.saturating_add_uint)
    /// is a bit faster than this method `saturating_add()`.
    /// - If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [saturating_add_uint()](struct@BigUInt#method.saturating_add_uint).
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
        let mut res = Self::from_array(self.get_number().clone());
        res.saturating_add_assign(rhs);
        res
    }

    // pub fn saturating_add_assign(&mut self, rhs: &Self)
    /// Computes `self` + `rhs`, saturating at the numeric bounds
    /// instead of overflowing, and assigns the result to `self` back.
    /// 
    /// # Counterpart Method
    /// - The method
    /// [saturating_add_assign_uint()](struct@BigUInt#method.saturating_add_assign_uint)
    /// is a bit faster than this method `saturating_add_assign()`.
    /// - If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [saturating_add_assign_uint()](struct@BigUInt#method.saturating_add_assign_uint).
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

    // pub fn modular_add(&self, rhs: &Self, modulo: &Self) -> Self
    /// Computes (`self` + `rhs`) % `modulo`, wrapping around at `modulo` of the
    /// type `Self` instead of overflowing.
    /// 
    /// # Output
    /// It returns the modulo-sum (`self` + `rhs`) % `modulo` with wrapping
    /// (modular) addition at `modulo`.
    /// 
    /// # Features
    /// - Wrapping (modular) addition at `modulo`. The differences between this
    /// method `modular_add()` and the method `wrapping_add()` are, first,
    /// where wrapping around happens, and, second, whether or not `OVERFLOW`
    /// flag is set. First, this method wraps araound at `modulo` while the
    /// method `wrapping_add()` wraps araound at maximum value + 1. Second,
    /// this method does not set `OVERFLOW` flag even if wrapping around
    /// happens while the method `wrapping_add()` sets `OVERFLOW` flag when
    /// wrapping around happens.
    /// - If `modulo` is `zero`, the flags such as `OVERFLOW`, `DIVIDED_BY_ZERO`,
    /// and `INFINITY` will be set.
    /// 
    /// # Counterpart Method
    /// - The method
    /// [modular_add_uint()](struct@BigUInt#method.modular_add_uint)
    /// is a bit faster than this method `modular_add()`.
    /// - If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [modular_add_uint()](struct@BigUInt#method.modular_add_uint).
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
    pub fn modular_add(&self, rhs: &Self, modulo: &Self) -> Self
    {
        let mut res = Self::from_array(self.get_number().clone());
        res.modular_add_assign(rhs, modulo);
        res
    }

    // pub fn modular_add_assign(&mut self, rhs: &Self, modulo: &Self)
    /// Computes (`self` + `rhs`) % `modulo`, wrapping around at `modulo`
    /// of the type `Self` instead of overflowing, and then, assign the result
    /// back to `self`.
    /// 
    /// # Features
    /// - Wrapping (modular) addition at `modulo`. The differences between this
    /// method `modular_add_assign()` and the method `wrapping_add_assign()`
    /// are, first, where wrapping around happens, and, second, whether or not
    /// `OVERFLOW` flag is set. First, this method wraps araound at `modulo`
    /// while the method `wrapping_add_assign()` wraps araound at maximum value.
    /// Second, this method does not set `OVERFLOW` flag even if wrapping around
    /// happens, while the method `wrapping_add_assign()` sets `OVERFLOW` flag
    /// when wrapping around happens.
    /// - If `modulo` is `zero`, the flags such as `OVERFLOW`, `DIVIDED_BY_ZERO`,
    /// and `INFINITY` will be set.
    /// - All the flags are historical, which means, for example, if an
    /// overflow occurred even once before this current operation or
    /// `OVERFLOW` flag is already set before this current operation,
    /// the `OVERFLOW` flag is not changed even if this current operation
    /// does not cause overflow.
    /// 
    /// # Counterpart Method
    /// - The method
    /// [modular_add_assign_uint()](struct@BigUInt#method.modular_add_assign_uint)
    /// is a bit faster than this method `modular_add_assign()`.
    /// - If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [modular_add_assign_uint()](struct@BigUInt#method.modular_add_assign_uint).
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
    pub fn modular_add_assign(&mut self, rhs: &Self, modulo: &Self)
    {
        self.wrapping_rem_assign(modulo);
        if *rhs < *modulo    // In this case, it does not cause cloning for performance.
        {
            // let mrhs = rhs.clone(); // Does not clone for performance
            let diff = modulo.wrapping_sub(rhs);
            if *self >= diff
                { self.wrapping_sub_assign(&diff); }
            else
                { self.wrapping_add_assign(rhs); }
        }
        else    // if *rhs >= *modulo
        {
            let mrhs = rhs.wrapping_rem(modulo);
            let diff = modulo.wrapping_sub(&mrhs);
            if *self >= diff
                { self.wrapping_sub_assign(&diff); }
            else
                { self.wrapping_add_assign(&mrhs); }
        }
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
    /// the `UNDERFLOW` flag.
    /// 
    /// # Outputs
    /// It returns a tuple containing an output big integer and a carry-out bit.
    /// 
    /// # Counterpart Method
    /// - The method
    /// [borrowing_sub_uint()](struct@BigUInt#method.borrowing_sub_uint)
    /// is a bit faster than this method `borrowing_sub()`.
    /// - If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [borrowing_sub_uint()](struct@BigUInt#method.borrowing_sub_uint).
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
        let mut res = Self::from_array(self.get_number().clone());
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
    /// the `UNDERFLOW` flag.
    /// 
    /// # Outputs
    /// It returns a carry-out bit.
    /// 
    /// # Counterpart Method
    /// The method
    /// [borrowing_sub_assign_uint()](struct@BigUInt#method.borrowing_sub_assign_uint)
    /// is a bit faster than this method `borrowing_sub_assign()`.
    /// If `rhs` is primitive unsigned integral
    /// data type such as u8, u16, u32, u64, and u128, use the method
    /// [borrowing_sub_assign_uint()](struct@BigUInt#method.borrowing_sub_assign_uint).
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
        let mut	b = borrow;
        let mut num;
        let i_l = self.leading_zero_elements() as usize;
        let j_l = rhs.leading_zero_elements() as usize;
        let ij_n = if i_l < j_l {N-i_l} else {N};
        for i in 0..ij_n
        {
            (num, b) = self.get_num_(i).borrowing_sub(rhs.get_num_(i), b);
            self.set_num_(i, num);
        }
        if b
        {
            for i in ij_n..N
                { self.set_num_(i, T::max()); }
            self.set_underflow();
        }
        b
    }

    // pub fn wrapping_sub(&self, rhs: &Self) -> Self
    /// Computes `self` - `rhs`, wrapping around at the boundary of the type.
    /// 
    /// # Output
    /// It returns `self` - `rhs` with wrapping (modular) subtraction.
    /// 
    /// # Features
    /// Wrapping (modular) subtraction.
    /// 
    /// # Counterpart Method
    /// The method
    /// [wrapping_sub_uint()](struct@BigUInt#method.wrapping_sub_uint)
    /// is a bit faster than this method `wrapping_sub()`.
    /// If `rhs` is primitive unsigned integral
    /// data type such as u8, u16, u32, u64, and u128, use the method
    /// [wrapping_sub_uint()](struct@BigUInt#method.wrapping_sub_uint).
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
    /// # Features
    /// Wrapping (modular) subtraction.
    /// 
    /// # Counterpart Method
    /// The method
    /// [wrapping_sub_assign_uint()](struct@BigUInt#method.wrapping_sub_assign_uint)
    /// is a bit faster than this method `wrapping_sub_assign()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [wrapping_sub_assign_uint()](struct@BigUInt#method.wrapping_sub_assign_uint).
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
    /// The method
    /// [overflowing_sub_uint()](struct@BigUInt#method.overflowing_sub_uint)
    /// is a bit faster than this method `overflowing_sub()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [overflowing_sub_uint()](struct@BigUInt#method.overflowing_sub_uint).
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
        let mut res = Self::from_array(self.get_number().clone());
        let current_overflow = res.overflowing_sub_assign(rhs);
        (res, current_overflow)
    }

    // pub fn overflowing_sub_assign(&mut self, rhs: &Self) -> bool
    /// Calculates `self` - `rhs`, and assigns the result to `self` back.
    /// 
    /// # Output
    /// It returns true if an arithmetic unerflow would occur.
    /// Otherwise, it returns `false`.
    /// 
    /// # Counterpart Method
    /// The method
    /// [overflowing_sub_assign_uint()](struct@BigUInt#method.overflowing_sub_assign_uint)
    /// is a bit faster than this method `overflowing_sub_assign()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [overflowing_sub_assign_uint()](struct@BigUInt#method.overflowing_sub_assign_uint).
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
        let old_overflow = self.is_overflow();
        self.reset_overflow();
        self.wrapping_sub_assign(rhs);
        let current_overflow = self.is_overflow();
        if old_overflow || current_overflow
            { self.set_overflow(); }
        current_overflow
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
    /// The method
    /// [checked_sub_uint()](struct@BigUInt#method.checked_sub_uint)
    /// is a bit faster than this method `checked_sub()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [checked_sub_uint()](struct@BigUInt#method.checked_sub_uint).
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
        let mut res = Self::from_array(self.get_number().clone());
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
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// - If underflow occurred, it will panic. So, use this method
    /// only when you are sure that underflow will not occur. 
    /// 
    /// # Output
    /// It returns the difference `self` - `rhs` if underflow did not occur.
    /// Otherwise, it will panic.
    /// 
    /// # Counterpart Method
    /// The method
    /// [unchecked_sub_uint()](struct@BigUInt#method.unchecked_sub_uint)
    /// is a bit faster than this method `unchecked_sub()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [unchecked_sub_uint()](struct@BigUInt#method.unchecked_sub_uint).
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
    /// The method
    /// [saturating_sub_uint()](struct@BigUInt#method.saturating_sub_uint)
    /// is a bit faster than this method `saturating_sub()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [saturating_sub()](struct@BigUInt#method.saturating_sub).
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
        let mut res = Self::from_array(self.get_number().clone());
        res.saturating_sub_assign(rhs);
        res
    }

    // pub fn saturating_sub_assign(&mut self, rhs: &Self)
    /// Computes `self` - `rhs`, saturating at the numeric bounds
    /// instead of underflowing, and assigns the result to `self` back.
    /// 
    /// # Features
    /// `self` will be the difference `self` - `rhs` if underflowing
    /// did not occur. Otherwise, it returns `0`.
    /// 
    /// # Counterpart Method
    /// The method
    /// [saturating_sub_assign_uint()](struct@BigUInt#method.saturating_sub_assign_uint)
    /// is a bit faster than this method `saturating_sub_assign()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [saturating_sub_assign_uint()](struct@BigUInt#method.saturating_sub_assign_uint).
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
    /// The method [abs_diff_uint()](struct@BigUInt#method.abs_diff_uint)
    /// is a bit faster than this method `abs_diff()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [abs_diff_uint()](struct@BigUInt#method.abs_diff_uint).
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigUInt;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a = U256::from_str("500000000000000000500000000500000000500000000500000000").unwrap();
    /// let b = U256::from_str("500000000000000000000000000000000000000000000000000000").unwrap();
    /// let c = a.abs_diff(&b);
    /// let d = b.abs_diff(&a);
    /// println!("500000000000000000500000000500000000500000000500000000 <-> 500000000000000000000000000000000000000000000000000000 = {}", c);
    /// println!("500000000000000000000000000000000000000000000000000000 <-> 500000000000000000500000000500000000500000000500000000 = {}", d);
    /// assert_eq!(c, U256::from_str("500000000500000000500000000500000000").unwrap());
    /// assert_eq!(d, U256::from_str("500000000500000000500000000500000000").unwrap());
    /// ```
    pub fn abs_diff(&self, other: &Self) -> Self
    {
        if self < other
            { other.wrapping_sub(self) }
        else
            { self.wrapping_sub(other) }
    }

    // pub fn modular_sub(&self, rhs: &Self, modulo: &Self) -> Self
    /// Computes (`self` - `rhs`) % `modulo`, wrapping around at `modulo` of the
    /// type `Self` instead of underflowing.
    /// 
    /// # Output
    /// It returns the modulo-difference (`self` - `rhs`) % `modulo` with
    /// wrapping (modular) subtraction at `modulo`.
    /// 
    /// # Features
    /// - Wrapping (modular) subtraction at `modulo`. The differences between
    /// this method `modular_sub()` and the method `wrapping_sub()` are, first,
    /// where wrapping around happens, and, second, whether or not `UNDERFLOW`
    /// flag is set. First, this method wraps araound at `modulo` while the
    /// method `wrapping_sub()` wraps araound at maximum value. Second, this
    /// method does not set `UNDERFLOW` flag even if wrapping around
    /// happens while the method `wrapping_sub()` sets `UNDERFLOW` flag when
    /// wrapping around happens.
    /// - If `modulo` is `zero`, the flags such as `OVERFLOW`, `DIVIDED_BY_ZERO`,
    /// and `INFINITY` will be set.
    /// 
    /// # Counterpart Method
    /// The method
    /// [modular_sub_uint()](struct@BigUInt#method.modular_sub_uint)
    /// is a bit faster than this method `modular_sub()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [modular_sub_uint()](struct@BigUInt#method.modular_sub_uint).
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
    pub fn modular_sub(&self, rhs: &Self, modulo: &Self) -> Self
    {
        let mut res = Self::from_array(self.get_number().clone());
        res.modular_sub_assign(rhs, modulo);
        res
    }

    // pub fn modular_sub_assign(&mut self, rhs: &Self, modulo: &Self)
    /// Computes (`self` - `rhs`) % `modulo`, wrapping around at `modulo`
    /// of the type `Self` instead of underflowing, and then, assign the result
    /// back to `self`.
    /// 
    /// # Features
    /// Wrapping (modular) subtraction at `modulo`. The differences between this
    /// method `modular_sub_assign()` and the method `wrapping_sub_assign()`
    /// are, first, where wrapping around happens, and, second, whether or not
    /// `UNDERFLOW` flag is set. First, this method wraps araound at `modulo`
    /// while the method `wrapping_sub_assign()` wraps araound at maximum value.
    /// Second, this method does not set `UNDERFLOW` flag even if wrapping
    /// around happens, while the method `wrapping_sub_assign()` sets
    /// `UNDERFLOW` flag when wrapping around happens.
    /// - If `modulo` is `zero`, the flags such as `OVERFLOW`, `DIVIDED_BY_ZERO`,
    /// and `INFINITY` will be set.
    /// - All the flags reflect historical underflow, which means, for example,
    /// if an overflow occurred even once before this current operation or
    /// `OVERFOLOW` flag is already set before this current operation, the
    /// `OVERFOLOW` flag is not changed even though this current operation does
    /// not cause overflow.
    /// 
    /// # Counterpart Method
    /// The method
    /// [modular_sub_assign_uint()](struct@BigUInt#method.modular_sub_assign_uint)
    /// is a bit faster than this method `modular_sub_assign()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [modular_sub_assign_uint()](struct@BigUInt#method.modular_sub_assign_uint).
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
    pub fn modular_sub_assign(&mut self, rhs: &Self, modulo: &Self)
    {
        self.wrapping_rem_assign(modulo);
        if *rhs < *modulo    // In this case, it does not cause cloning for performance.
        {
            // let mrhs = rhs.clone(); // Does not clone for performance
            if *self >= *rhs
            {
                self.wrapping_sub_assign(rhs);
            }
            else    // if *self < *rhs
            {
                let diff = modulo.wrapping_sub(rhs);
                self.wrapping_add_assign(&diff);
            }
        }
        else    // if *rhs >= *modulo
        {
            let mrhs = rhs.wrapping_rem(modulo);
            if *self >= mrhs
            {
                self.wrapping_sub_assign(&mrhs);
            }
            else
            {
                let diff = modulo.wrapping_sub(&mrhs);
                self.wrapping_add_assign(&diff);
            }
        }
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
    /// # Features
    /// It performs “long multiplication” which takes in an extra amount to add,
    /// and may return an additional amount of overflow. This allows for
    /// chaining together multiple multiplications to create “bigger integers”
    /// which represent larger values.
    /// 
    /// # Counterpart Methods
    /// - If you don’t need the carry, then you can use
    /// [widening_mul()](struct@BigUInt#method.widening_mul) instead.
    /// - The value of the first field in the returned tuple matches
    /// what you’d get by combining the methods
    /// [wrapping_mul()](struct@BigUInt#method.wrapping_mul) and
    /// [wrapping_add()](struct@BigUInt#method.wrapping_add):
    /// `self.wrapping_mul(rhs).wrapping_add(carry)`. So,
    /// `self.carrying_mul(rhs, carry).0` == `self.wrapping_mul(rhs).wrapping_add(carry)`
    /// - The method
    /// [carrying_mul_uint()](struct@BigUInt#method.carrying_mul_uint)
    /// is a bit faster than this method `carrying_mul()`. If `rhs` is
    /// primitive unsigned integral data type such as u8, u16, u32, u64, and
    /// u128, use the method
    /// [carrying_mul_uint()](struct@BigUInt#method.carrying_mul_uint).
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
        let mut low = Self::from_array(self.get_number().clone());
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
    /// # Features
    /// It performs “long multiplication” which takes in an extra amount to add,
    /// and may return an additional amount of overflow. This allows for
    /// chaining together multiple multiplications to create “bigger integers”
    /// which represent larger values.
    /// 
    /// # Counterpart Methods
    /// - If you don’t need the carry, then you can use
    /// [widening_mul_assign()](struct@BigUInt#method.widening_mul_assign)
    /// instead.
    /// - The value of `self` after calculation matches what you’d get by
    /// combining the methods
    /// [wrapping_mul()](struct@BigUInt#method.wrapping_mul) and
    /// [wrapping_add_assign()](struct@BigUInt#method.wrapping_add_assign):
    /// `self.wrapping_mul(rhs).wrapping_add_assign(carry)`.
    /// - The method
    /// [carrying_mul_assign_uint()](struct@BigUInt#method.carrying_mul_assign_uint)
    /// is a bit faster than this method `carrying_mul_assign()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128,
    /// use the method
    /// [carrying_mul_assign_uint()](struct@BigUInt#method.carrying_mul_assign_uint).
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
        let mut high = self.widening_mul_assign(rhs);
        if self.overflowing_add_assign(&carry)
            { high.wrapping_add_assign_uint(1_u8); }
        high


/*
        let zero = T::zero();
        let one = T::one();
        let adder = self.clone();
        let size_t_bits = T::size_in_bits();
        let mut multiply_first = |num: T| {
            let mut bit_check = one;
            bit_check <<= T::usize_as_smalluint(size_t_bits - 1);
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
                        { high.wrapping_add_uint(T::u8_as_smalluint(1)); }
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
                *self <<= size_t_bits as i32;
                high <<= size_t_bits as i32;
                return;
            }
            let mut bit_check = one;
            bit_check <<= T::usize_as_smalluint(size_t_bits - 1);
            while bit_check != zero
            {
                *self <<= 1;
                high <<= 1;
                if self.is_overflow()
                    { high.set_num_(0, high.get_num_(0) | T::u8_as_smalluint(1)) ; }
                if bit_check & num != zero
                {
                    self.wrapping_add_assign(&adder);
                    high.wrapping_add_uint(T::bool_as_smalluint(self.is_overflow()));
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
        */
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
    /// # Features
    /// It performs “long multiplication” which takes in an extra amount to add,
    /// and may return an additional amount of overflow. This allows for
    /// chaining together multiple multiplications to create “bigger integers”
    /// which represent larger values.
    /// 
    /// # Counterpart Methods
    /// - If you also need to add a carry to the wide result, then you want to use
    /// [carrying_mul()](struct@BigUInt#method.carrying_mul) instead.
    /// - The value of the first field in the returned tuple matches what you’d
    /// get the method [wrapping_mul()](struct@BigUInt#method.wrapping_mul).
    /// `self.widening_mul(rhs).0` == `self.wrapping_mul(rhs)`
    /// - The method
    /// [widening_mul_uint()](struct@BigUInt#method.widening_mul_uint)
    /// is a bit faster than this method `widening_mul()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [widening_mul_uint()](struct@BigUInt#method.widening_mul_uint).
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
        let mut low = Self::from_array(self.get_number().clone());
        let high = low.widening_mul_assign(rhs);
        (low, high)
    }

    // pub fn widening_mul_assign(&mut self, rhs: &Self) -> Self
    /// Calculates the complete product `self` * `rhs` without the possibility
    /// to overflow.
    /// 
    /// # Output
    /// It returns the high-order (overflow) bits of the result `self` * `rhs`.
    /// 
    /// # Features
    /// It performs “long multiplication” which takes in an extra amount to add,
    /// and may return an additional amount of overflow. This allows for
    /// chaining together multiple multiplications to create “bigger integers”
    /// which represent larger values.
    /// 
    /// # Counterpart Methods
    /// - If you also need to add a carry to the wide result, then you want
    /// to use
    /// [carrying_mul_assign()](struct@BigUInt#method.carrying_mul_assign)
    /// instead.
    /// - The value of `self` after calculation matches what you’d get
    /// the method [wrapping_mul()](struct@BigUInt#method.wrapping_mul).
    /// `self` == `self.wrapping_mul(rhs)`
    /// - The method
    /// [widening_mul_assign_uint()](struct@BigUInt#method.widening_mul_assign_uint)
    /// is a bit faster than this method `widening_mul_assign()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [widening_mul_assign_uint()](struct@BigUInt#method.widening_mul_assign_uint).
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
        (Self::method_widening_mul_assign)(self, rhs)
    }

    fn widening_mul_assign_1(&mut self, rhs: &Self) -> Self
    {
        if rhs.is_zero()
        {
            self.set_zero();
            return Self::zero();
        }
        if self.is_zero()
            { return Self::zero(); }

        let operand = self.clone();
        let zero = T::zero();
        let mut high = Self::zero();
        let mut lower = zero;
        let mut higher = zero;
        let i_n = N - rhs.leading_zero_elements() as usize;
        let j_n = N - operand.leading_zero_elements() as usize;
        for i in 0..i_n
        {
            for j in 0..j_n
            {
                (lower, higher) = operand.get_num_(j).carrying_mul(rhs.get_num_(i), higher);
                let ij = i + j;
                if ij < N
                    { self.set_num_(ij, lower); }
                else
                    { high.set_num_(ij - N, lower); }
            }
            let c = i + j_n;
            if c < N
                { self.set_num_(c, lower); }
            else
                { high.set_num_(c - N, lower); }
        }
        if !high.is_zero()
            { self.set_overflow(); }
        high
    }

    fn widening_mul_assign_2(&mut self, rhs: &Self) -> Self
    {
        if rhs.is_zero()
        {
            self.set_zero();
            return Self::zero();
        }
        if self.is_zero()
            { return Self::zero(); }

        let adder = self.clone();
        let size_t_bits_minus_one = T::size_in_bits() - 1;
        let mut high = Self::zero();
        let mut chunk = N - 1 - rhs.leading_zero_elements() as usize;
        let mut piece = T::size_in_bits() - 1 - rhs.get_num_(chunk).leading_zeros() as usize;
        self.set_zero();
        loop
        {
            let num = rhs.get_num_(chunk);
            if num.is_zero()
            {
                self.shift_left_assign(size_t_bits_minus_one);
            }
            else
            {
                loop
                {
                    if num.is_bit_set_(piece)
                    {
                        if self.overflowing_add_assign(&adder)
                            { high.wrapping_add_assign_uint(1_u8); }
                    }
                    if piece == 0
                        { break; }
                    piece -= 1;
                    high.shift_left_assign(1_u8);
                    if self.is_msb_set()
                        { high.set_lsb(); }
                    self.shift_left_assign(1_u8);
                }
            }
            if chunk == 0
                { break; }
            chunk -= 1;
            high.shift_left_assign(1_u8);
            if self.is_msb_set()
                { high.set_lsb(); }
            self.shift_left_assign(1_u8);
            piece = T::size_in_bits() - 1;
        }
        high
    }

    // pub fn wrapping_mul(&self, rhs: &Self) -> Self
    /// Computes `self` * `rhs`, wrapping around at the boundary of the type.
    /// 
    /// # Output
    /// It returns `self` * `rhs` with wrapping (modular) addition.
    /// 
    /// # Features
    /// Wrapping (modular) addition.
    /// 
    /// # Counterpart Method
    /// The method
    /// [wrapping_mul_uint()](struct@BigUInt#method.wrapping_mul_uint)
    /// is a bit faster than this method `wrapping_mul()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [wrapping_mul_uint()](struct@BigUInt#method.wrapping_mul_uint).
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
        let mut res = Self::from_array(self.get_number().clone());
        res.wrapping_mul_assign(rhs);
        res
    }

    // pub fn wrapping_mul_assign(&mut self, rhs: &Self)
    /// Computes self * rhs, wrapping around at the boundary of the type,
    /// and assign the result to `self` back.
    /// 
    /// # Features
    /// Wrapping (modular) multiplication.
    /// 
    /// # Counterpart Method
    /// The method
    /// [wrapping_mul_assign_uint()](struct@BigUInt#method.wrapping_mul_assign_uint)
    /// is a bit faster than this method `wrapping_mul_assign()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [wrapping_mul_assign_uint()](struct@BigUInt#method.wrapping_mul_assign_uint).
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
    pub fn wrapping_mul_assign(&mut self, rhs: &Self)
    {
        (Self::method_wrapping_mul_assign)(self, rhs);
    }

    fn wrapping_mul_assign_1(&mut self, rhs: &Self)
    {
        if rhs.is_zero()
        {
            self.set_zero();
            return;
        }
        if self.is_zero()
            { return; }

        let operand = Self::from_array(self.get_number().clone());
        let zero = T::zero();
        let one = T::one();
        let i_n = N - rhs.leading_zero_elements() as usize;
        let j_n = N - operand.leading_zero_elements() as usize;
        let mut lower;
        let mut higher;
        let mut sum;
        let mut overflow;
        let mut ij = 0_usize;
        self.set_zero();
        for i in 0..i_n
        {
            higher = zero;
            for j in 0..j_n
            {
                ij = i + j;
                if ij >= N
                {
                    self.set_overflow();
                    ij -= 1;
                    break;
                }
                (lower, higher) = operand.get_num_(j).carrying_mul(rhs.get_num_(i), higher);
                (sum, overflow) = self.get_num_(ij).overflowing_add(lower);
                self.set_num_(ij, sum);
                if overflow
                    { higher += one; }
            }

            ij += 1;
            if !higher.is_zero()
            {
                if ij < N
                {
                    (sum, overflow) = self.get_num_(ij).overflowing_add(higher);
                    self.set_num_(ij, sum);
                    while overflow
                    {
                        ij += 1;
                        if ij >= N
                        {
                            self.set_overflow();
                            break;
                        }
                        (sum, overflow) = self.get_num_(ij).overflowing_add(one);
                        self.set_num_(ij, sum);
                    }
                }
                else
                {
                    self.set_overflow();
                }
            }
        }
    }

    fn wrapping_mul_assign_2(&mut self, rhs: &Self)
    {
        if rhs.is_zero()
        {
            self.set_zero();
            return;
        }
        if self.is_zero()
            { return; }

        let adder = Self::from_array(self.get_number().clone());
        let size_t_bits_minus_one = T::size_in_bits()-1;
        let mut chunk = N - 1 - rhs.leading_zero_elements() as usize;
        let mut piece = T::size_in_bits() - 1 - rhs.get_num_(chunk).leading_zeros() as usize;
        self.set_zero();
        loop
        {
            let num = rhs.get_num_(chunk);
            if num.is_zero()
            {
                self.shift_left_assign(size_t_bits_minus_one);
            }
            else
            {
                loop
                {
                    if num.is_bit_set_(piece)
                        { self.wrapping_add_assign(&adder); }
                    if piece == 0
                        { break; }
                    piece -= 1;
                    self.shift_left_assign(1_u8);
                }
            }
            if chunk == 0
                { break; }
            chunk -= 1;
            self.shift_left_assign(1_u8);
            piece = T::size_in_bits() - 1;
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
    /// The method
    /// [overflowing_mul_uint()](struct@BigUInt#method.overflowing_mul_uint)
    /// is a bit faster than this method `overflowing_mul()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [overflowing_mul_uint()](struct@BigUInt#method.overflowing_mul_uint).
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
        let mut res = Self::from_array(self.get_number().clone());
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
    /// The method
    /// [overflowing_mul_assign_uint()](struct@BigUInt#method.overflowing_mul_assign_uint)
    /// is a bit faster than this method `overflowing_mul_assign()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [overflowing_mul_assign_uint()](struct@BigUInt#method.overflowing_mul_assign_uint).
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
        let old_overflow = self.is_overflow();
        self.reset_overflow();
        self.wrapping_mul_assign(rhs);
        let current_overflow = self.is_overflow();
        if old_overflow || current_overflow
            { self.set_overflow(); }
        current_overflow
    }

    // pub fn checked_mul(&self, rhs: &Self) -> Option<Self>
    /// Computes `self` * `rhs`.
    /// 
    /// # Output
    /// It returns the sum `self` * `rhs` wrapped by `Some` of enum `Option`
    /// if overflow did not occur. Otherwise, it returns `None` of enum Option.
    /// 
    /// # Counterpart Method
    /// The method [checked_mul_uint()](struct@BigUInt#method.checked_mul_uint)
    /// is a bit faster than this method `checked_mul()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [checked_mul_uint()](struct@BigUInt#method.checked_mul_uint).
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
        let mut res = Self::from_array(self.get_number().clone());
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
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// - If overflow occurred, it will panic. So, use this method
    /// only when you are sure that overflow will not occur. 
    /// 
    /// # Output
    /// It returns the sum `self` * `rhs` if overflow did not occur.
    /// Otherwise, it will panic.
    /// 
    /// # Counterpart Method
    /// The method
    /// [unchecked_mul_uint()](struct@BigUInt#method.unchecked_mul_uint)
    /// is a bit faster than this method `unchecked_mul()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [unchecked_mul_uint()](struct@BigUInt#method.unchecked_mul_uint).
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
    /// The method
    /// [saturating_mul_uint()](struct@BigUInt#method.saturating_mul_uint)
    /// is a bit faster than this method `saturating_mul()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [saturating_mul_uint()](struct@BigUInt#method.saturating_mul_uint).
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
        let mut res = Self::from_array(self.get_number().clone());
        res.saturating_mul_assign(rhs);
        res
    }

    // pub fn saturating_mul_assign(&mut self, rhs: &Self)
    /// Computes `self` * `rhs`, saturating at the numeric bounds
    /// instead of overflowing, and assigns the result to `self` back.
    /// 
    /// # Counterpart Method
    /// The method
    /// [saturating_mul_assign_uint()](struct@BigUInt#method.saturating_mul_assign_uint)
    /// is a bit faster than this method `saturating_mul_assign()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [saturating_mul_assign_uint()](struct@BigUInt#method.saturating_mul_assign_uint).
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

    // pub fn modular_mul(&self, rhs: &Self, modulo: &Self) -> Self
    /// Computes (`self` * `rhs`) % `modulo`, wrapping around at `modulo`
    /// of the type `Self`.
    /// 
    /// # Features
    /// - Wrapping (modular) multiplication at `modulo`. The differences between
    /// this method `modular_mul()` and the method `wrapping_mul()` are, first,
    /// where wrapping around happens, and, second, whether or not `OVERFLOW`
    /// flag is set. First, this method wraps araound at `modulo` while the
    /// method `wrapping_mul()` wraps araound at maximum value. Second, this
    /// method does not set `OVERFLOW` flag even if wrapping around happens,
    /// while the method `wrapping_mul()` sets `OVERFLOW` flag when wrapping
    /// around happens.
    /// - If `modulo` is `zero`, the flags such as `OVERFLOW`, `DIVIDED_BY_ZERO`,
    /// and `INFINITY` will be set.
    /// 
    /// # Counterpart Method
    /// The method [modular_mul_uint()](struct@BigUInt#method.modular_mul_uint)
    /// is a bit faster than this method `modular_mul()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [modular_mul_uint()](struct@BigUInt#method.modular_mul_uint).
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
    pub fn modular_mul(&self, rhs: &Self, modulo: &Self) -> Self
    {
        let mut res = Self::from_array(self.get_number().clone());
        res.modular_mul_assign(rhs, modulo);
        res
    }

    // pub fn modular_mul_assign(&self, rhs: &Self, modulo: &Self)
    /// Computes (`self` * `rhs`) % `modulo`, wrapping around at `modulo`
    /// of the type `Self`, and assign the result to `self` back.
    /// 
    /// # Features
    /// - Wrapping (modular) multiplication at `modulo`. The differences between
    /// this method `modular_mul_assign()` and the method
    /// `wrapping_mul_assign()` are, first, where wrapping around happens, and,
    /// second, whether or not `OVERFLOW` flag is set. First, this method wraps
    /// araound at `modulo` while the method `wrapping_mul_assign()` wraps
    /// araound at maximum value. Second, this method does not set `OVERFLOW`
    /// flag even if wrapping around happens, while the method
    /// `wrapping_mul_assign()` sets `OVERFLOW` flag when wrapping around
    /// happens.
    /// - If `modulo` is `zero`, the flags such as `OVERFLOW`, `DIVIDED_BY_ZERO`,
    /// and `INFINITY` will be set.
    /// - All the flags reflect historical underflow, which means, for example,
    /// if an overflow occurred even once before this current operation or
    /// `OVERFOLOW` flag is already set before this current operation, the
    /// `OVERFOLOW` flag is not changed even though this current operation does
    /// not cause overflow.
    /// 
    /// # Counterpart Method
    /// The method
    /// [modular_mul_assign_uint()](struct@BigUInt#method.modular_mul_assign_uint)
    /// is a bit faster than this method `modular_mul_assign()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [modular_mul_assign_uint()](struct@BigUInt#method.modular_mul_assign_uint).
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
    pub fn modular_mul_assign(&mut self, rhs: &Self, modulo: &Self)
    {
        self.wrapping_rem_assign(modulo);
        let mut mrhs = rhs.wrapping_rem(modulo);
        let mut res = Self::zero();
        let zero = Self::zero();
        while mrhs > zero
        {
            if mrhs.is_odd()
                { res.modular_add_assign(self, modulo); }
            self.modular_add_assign(&self.clone(), modulo);
            mrhs.shift_right_assign(1_u8);
        }
        self.set_number(res.get_number());
    }


    /*** Division ***/

    // pub fn divide_fully(&self, rhs: &Self) -> (Self, Self)
    /// Divides `self` which is of `BigUInt` type by `rhs` which is of `BigUInt`
    /// type, and returns a tuple of quotient and remainder.
    /// 
    /// # Output
    /// It returns a tuple of quotient and remainder which are `BigUInt`type.
    /// 
    /// # Features
    /// If `rhs` is zero, the quotient will have maximum value of `BigUInt`
    /// type, and the flags of quotient such as `OVERFLOW`, `INFINITY`, and
    /// `DIVIDED_BY_ZERO` will be set, and the remainder will be set to be
    /// zero of `BigUInt` type, the `DIVIDED_BY_ZERO` flag of remainder
    /// will be set.
    /// 
    /// # Counterpart Method
    /// The method
    /// [divide_fully_uint()](struct@BigUInt#method.divide_fully_uint)
    /// is a bit faster than this method `divide_fully()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [divide_fully_uint()](struct@BigUInt#method.divide_fully_uint).
    /// 
    /// # Examples
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let dividend = U256::from_str("1234567890157589425462369896").unwrap();
    /// let divisor = U256::from_str("1234567890").unwrap();
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

        let size_rhs = rhs.length_in_bits() - rhs.leading_zeros() as usize;
        let size_self = self.length_in_bits() - self.leading_zeros() as usize;
        let mut remainder = self.get_upper_portion(size_rhs);
        let mut position = size_self - size_rhs;
        loop
        {
            if *rhs <= remainder
            {
                quotient.set_lsb();
                remainder.wrapping_sub_assign(rhs); 
            }
            if position == 0
                { break; }
            position -= 1;
            quotient.shift_left_assign(1_u8);
            remainder.shift_left_assign(1_u8);
            if self.is_bit_set_(position)
                { remainder.set_lsb(); }
        }

        (quotient, remainder)
    }

    // pub fn wrapping_div(&self, rhs: &Self) -> Self
    /// Calculates the quotient when `self` is divided by `rhs`,
    /// which is `self` / `rhs`.
    ///
    /// # Output
    /// It returns the quotient of when `self` is divided by `rhs`,
    /// which is `self` / `rhs`. 
    /// 
    /// # Features
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
    /// The method
    /// [wrapping_div_uint()](struct@BigUInt#method.wrapping_div_uint)
    /// is a bit faster than this method `wrapping_div()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [wrapping_div_uint()](struct@BigUInt#method.wrapping_div_uint).
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
    /// # Features
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
    /// The method
    /// [wrapping_div_assign_uint()](struct@BigUInt#method.wrapping_div_assign_uint)
    /// is a bit faster than this method `wrapping_div_assign()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [wrapping_div_assign_uint()](struct@BigUInt#method.wrapping_div_assign_uint).
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
    /// # Features
    /// Wrapped division on `BigUInt` types is just normal division.
    /// There’s no way wrapping could ever happen.
    /// 
    /// If `rhs` is zero, the quotient will have maximum value of `BigUInt`
    /// type, and the flags of quotient such as `OVERFLOW`, `INFINITY`, and
    /// `DIVIDED_BY_ZERO` will be set.
    /// 
    /// # Counterpart Method
    /// The method [checked_div_uint()](struct@BigUInt#method.checked_div_uint)
    /// is a bit faster than this method `checked_div()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [checked_div_uint()](struct@BigUInt#method.checked_div_uint).
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
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// - If `rhs` is zero, it will panic. So, use this method
    /// only when you are sure that `rhs` is not zero. 
    /// 
    /// # Output
    /// It returns the quotient of when `self` is divided by `rhs`,
    /// which is `self` / `rhs` if `rhs` is not zero.
    /// Otherwise, it will panic.
    /// 
    /// # Features
    /// Wrapped division on `BigUInt` types is just normal division.
    /// There’s no way wrapping could ever happen.
    /// 
    /// # Counterpart Method
    /// The method
    /// [unchecked_div_uint()](struct@BigUInt#method.unchecked_div_uint)
    /// is a bit faster than this method `unchecked_div()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [unchecked_div_uint()](struct@BigUInt#method.unchecked_div_uint).
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
    /// # Features
    /// Overflow will not happen unless `rhs` is zero. __It does not panic__
    /// while the same named methods `saturating_div()` for primitive integer
    /// data type such as u8, u16, u32, u64, etc. will panic if `rhs` is zero.
    /// 
    /// If `rhs` is zero, the quotient will have maximum value of `BigUInt`
    /// type, and the flags of quotient such as `OVERFLOW` and `DIVIDED_BY_ZERO`
    /// will be set.
    /// 
    /// # Counterpart Method
    /// The method
    /// [saturating_div_uint()](struct@BigUInt#method.saturating_div_uint)
    /// is a bit faster than this method `saturating_div()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [saturating_div_uint()](struct@BigUInt#method.saturating_div_uint).
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
        let (quotient, _) = self.divide_fully(rhs);
        quotient
    }

    // pub fn saturating_div(&self, rhs: &Self) -> Self
    /// Calculates the quotient when `self` is divided by `rhs`,
    /// which is `self` / `rhs`, saturating at the numeric bounds
    /// instead of overflowing, and assigns the quotient to `self`.
    /// 
    /// # Features
    /// Overflow will not happen unless `rhs` is zero. __It does not panic__
    /// while the similar methods `saturating_div()` for primitive integer
    /// data type such as u8, u16, u32, u64, etc. will panic if `rhs` is zero.
    /// 
    /// If `rhs` is zero, `self` will have maximum value of `BigUInt`
    /// type, and the flags of `self` such as `OVERFLOW` and `DIVIDED_BY_ZERO`
    /// will be set.
    /// 
    /// # Counterpart Method
    /// The method
    /// [saturating_div_assign_uint()](struct@BigUInt#method.saturating_div_assign_uint)
    /// is a bit faster than this method `saturating_div_assign()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [saturating_div_assign_uint()](struct@BigUInt#method.saturating_div_assign_uint).
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
        let old_overflow = self.is_overflow();
        *self = self.saturating_div(rhs);
        if old_overflow
            { self.set_overflow(); }
    }

    // pub fn modular_div(&self, rhs: &Self, modulo: &Self) -> Self
    /// Calculates the quotient when `self` % `modulo` is divided by
    /// `rhs` % `modulo`.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the quotient of when `self` % `modulo` is divided by
    /// `rhs` % `modulo` if `rhs` % `modulo` is not zero.
    /// If `rhs` % `modulo` is zero, it returns the maximum value.
    /// 
    /// # Features
    /// - Overflow will not happen unless `rhs` % `modulo` is zero.
    /// - __It does not panic__ even if `rhs` % `modulo` is `zero`
    /// - If `rhs` % `modulo` is zero, the quotient will have maximum value
    /// of `BigUInt` type, and the flags of quotient such as `OVERFLOW`,
    /// `INFINITY`, and `DIVIDED_BY_ZERO` will be set.
    /// - If `modulo` is `zero`, the remainder will be `zero` and its
    /// flags such as `OVERFLOW`, `DIVIDED_BY_ZERO`, and `INFINITY` will be set.
    /// 
    /// # Counterpart Method
    /// If `rhs` is smaller than or equal to `u128`, the method
    /// [modular_div_uint()](struct@BigUInt#method.modular_div_uint)
    /// is proper rather than this method `modular_div()`.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let mut divisor = 87_u8;
    /// let mut quotient = dividend.saturating_div_uint(divisor);
    /// println!("{} / {} = {}", dividend, divisor, quotient);
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_inifinity(), false);
    /// assert_eq!(quotient.is_divided_by_zero(), false);
    /// 
    /// divisor = 0_u8;
    /// quotient = dividend.saturating_div_uint(divisor);
    /// println!("{} / {} = {}", dividend, divisor, quotient);
    /// assert_eq!(quotient, U256::max());
    /// assert_eq!(quotient.is_overflow(), true);
    /// assert_eq!(quotient.is_inifinity(), true);
    /// assert_eq!(quotient.is_divided_by_zero(), true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn modular_div(&self, rhs: &Self, modulo: &Self) -> Self
    {
        let mut lhs = Self::from_array(self.get_number().clone());
        lhs.modular_div_assign(rhs, modulo);
        lhs
    }

    // pub fn modular_div_assign(&mut self, rhs: &Self, modulo: &Self)
    /// Calculates the quotient when `self` % `modulo` is divided by
    /// `rhs` % `modulo`, and assigns the quotient to `self` back.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - Overflow will not happen unless `rhs` % `modulo` is zero.
    /// - If `rhs` % `modulo` is zero, the quotient will have maximum value
    /// of `BigUInt` type, and the flags of quotient such as `OVERFLOW`,
    /// `INFINITY`, and `DIVIDED_BY_ZERO` will be set.
    /// - __It does not panic__ even if `rhs` % `modulo` is `zero`.
    /// - If `modulo` is `zero`, the flags such as `OVERFLOW`, `DIVIDED_BY_ZERO`,
    /// and `INFINITY` will be set.
    /// - If `modulo` is `zero`, the flags such as `OVERFLOW`, `DIVIDED_BY_ZERO`,
    /// and `INFINITY` will be set.
    /// - All the flags reflect historical flags, which means, for example,
    /// if an overflow occurred even once before this current operation or
    /// `OVERFOLOW` flag is already set before this current operation, the
    /// `OVERFOLOW` flag is not changed even though this current operation does
    /// not cause overflow.
    /// - All the flags reflect historical underflow, which means, for example,
    /// if an overflow occurred even once before this current operation or
    /// `OVERFOLOW` flag is already set before this current operation, the
    /// `OVERFOLOW` flag is not changed even though this current operation does
    /// not cause overflow.
    /// - If `modulo` is `zero`, the flags such as `OVERFLOW`, `DIVIDED_BY_ZERO`,
    /// and `INFINITY` will be set.
    /// - The `OVERFLOW`, `INFINITY`, and `DIVIDED_BY_ZERO` flags reflect
    /// historical overflow, which means if an overflow and/or divided-by-zero
    /// occurred even once before this current operation or `OVERFLOW`,
    /// `INFINITY`, and/or `DIVIDED_BY_ZERO` flag(s) is/are already set before
    /// this current operation, the `OVERFLOW`, `INFINITY`, and/or
    /// `DIVIDED_BY_ZERO` flag(s) is/are not changed even though this current
    /// operation does not cause overflow and/or divided-by-zero.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [saturating_div_assign()](struct@BigUInt#method.saturating_div_assign)
    /// is proper rather than this method `saturating_div_assign_uint()`.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let mut divisor = 87_u16;
    /// println!("Originally,\na_biguint = {}", a_biguint);
    /// a_biguint.saturating_div_assign_uint(divisor);
    /// println!("After a_biguint.saturating_div_assign_uint({}),\na_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1419043551905275201680884938348044216837079832");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_inifinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// divisor = 0_u16;
    /// a_biguint.saturating_div_assign_uint(divisor);
    /// println!("After a_biguint.saturating_div_assign_uint({}),\na_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint, UU32::max());
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_inifinity(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn modular_div_assign(&mut self, rhs: &Self, modulo: &Self)
    {
        self.wrapping_rem_assign(modulo);
        self.wrapping_div_assign(&rhs.wrapping_rem(modulo));
    }

    // pub fn wrapping_rem(&self, rhs: &Self) -> Self
    /// Calculates the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`.
    /// 
    /// # Output
    /// It returns the remainder when `self` is divided by `rhs`,
    /// which is `self` % `rhs`, with wrapping (modular) addition.
    /// 
    /// # Features
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
    /// The method
    /// [wrapping_rem_uint()](struct@BigUInt#method.wrapping_rem_uint)
    /// is a bit faster than this method `wrapping_rem()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [wrapping_rem_uint()](struct@BigUInt#method.wrapping_rem_uint).
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
    /// # Features
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
    /// The method
    /// [wrapping_rem_assign_uint()](struct@BigUInt#method.wrapping_rem_assign_uint)
    /// is a bit faster than this method `wrapping_rem_assign()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [wrapping_rem_assign_uint()](struct@BigUInt#method.wrapping_rem_assign_uint).
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
        let old_overflow = self.is_overflow();
        let (_, remainder) = self.divide_fully(rhs);
        *self = remainder;
        if old_overflow
            { self.set_overflow(); }
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
    /// # Features
    /// Note that overflow never occurs, so the second value is always false.
    /// 
    /// If `rhs` is zero, the remainder is zero and its `DIVIDED_BY_ZERO`
    /// is set. __It does not panic__ while the same named methods
    /// `overflowing_rem()` for primitive integer data type such as u8, u16,
    /// u32, u64, etc. will panic if `rhs` is zero.
    /// 
    /// # Counterpart Method
    /// The method
    /// [overflowing_rem_uint()](struct@BigUInt#method.overflowing_rem_uint)
    /// is a bit faster than this method `overflowing_rem()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [overflowing_rem_uint()](struct@BigUInt#method.overflowing_rem_uint).
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
    /// # Features
    /// Note that overflow never occurs, so the outtput is always false.
    /// 
    /// If `rhs` is zero, `self` is zero and its `DIVIDED_BY_ZERO`
    /// is set. __It does not panic__ while the similar methods
    /// `overflowing_rem()` for primitive integer data type such as
    /// u8, u16, u32, u64, etc. will panic if `rhs` is zero.
    /// 
    /// # Counterpart Method
    /// The method
    /// [overflowing_rem_assign_uint()](struct@BigUInt#method.overflowing_rem_assign_uint)
    /// is a bit faster than this method `overflowing_rem_assign()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [overflowing_rem_assign_uint()](struct@BigUInt#method.overflowing_rem_assign_uint).
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
        let old_overflow = self.is_overflow();
        let (_, remainder) = self.divide_fully(rhs);
        *self = remainder;
        if old_overflow
            { self.set_overflow(); }
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
    /// # Features
    /// Note that overflow never occurs.
    /// 
    /// # Counterpart Method
    /// The method
    /// [checked_rem_uint()](struct@BigUInt#method.checked_rem_uint)
    /// is a bit faster than this method `checked_rem()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [checked_rem_uint()](struct@BigUInt#method.checked_rem_uint).
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
    /// # Features
    /// Note that overflow never occurs.
    /// 
    /// # Counterpart Method
    /// The method
    /// [unchecked_rem_uint()](struct@BigUInt#method.unchecked_rem_uint)
    /// is a bit faster than this method `unchecked_rem()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [unchecked_rem_uint()](struct@BigUInt#method.unchecked_rem_uint).
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
    /// # Features
    /// If `rhs` is zero, the remainder will have zero of`BigUInt` type,
    /// and `DIVIDED_BY_ZERO` flag of the remainder will be set, and
    /// the remainder will be set to be zero of `BigUInt` type.
    /// 
    /// Note that overflow never occurs.
    /// 
    /// # Counterpart Method
    /// The method
    /// [saturating_rem_uint()](struct@BigUInt#method.saturating_rem_uint)
    /// is a bit faster than this method `saturating_rem()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [saturating_rem_uint()](struct@BigUInt#method.saturating_rem_uint).
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
    /// # Features
    /// If `rhs` is zero, `self` will have zero of`BigUInt` type,
    /// and `DIVIDED_BY_ZERO` flag of `self` will be set, and
    /// `self` will be set to be zero of `BigUInt` type.
    /// 
    /// Note that overflow never occurs.
    /// 
    /// # Counterpart Method
    /// The method
    /// [saturating_rem_assign_uint()](struct@BigUInt#method.saturating_rem_assign_uint)
    /// is a bit faster than this method `saturating_rem_assign()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [saturating_rem_assign_uint()](struct@BigUInt#method.saturating_rem_assign_uint).
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
        let old_overflow = self.is_overflow();
        *self = self.saturating_rem(rhs);
        if old_overflow
            { self.set_overflow(); }
    }


    // pub fn modular_rem(&self, rhs: &Self, modulo: &Self) -> Self
    /// Calculates the remainder when `self` % `modulo` is divided by 
    /// rhs` % `modulo`.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// - It returns the remainder of when `self` % `modulo` is divided by
    /// `rhs` % `modulo`, if `rhs` % `modulo` is not zero.
    /// - If `rhs` % `modulo` is zero, it returns `zero`.
    /// 
    /// # Features
    /// - If `rhs` % `modulo` is zero, the remainder will be `zero`
    /// of `BigUInt` type, and the flag of remainder `DIVIDED_BY_ZERO`
    /// will be set.
    /// - __It does not panic__ even if `rhs` % `modulo` is `zero`.
    /// - If `modulo` is `zero`, the flags such as `OVERFLOW`, `DIVIDED_BY_ZERO`,
    /// and `INFINITY` will be set.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [saturating_div()](struct@BigUInt#method.saturating_div)
    /// is proper rather than this method `saturating_div_uint()`.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let mut divisor = 87_u8;
    /// let mut quotient = dividend.saturating_div_uint(divisor);
    /// println!("{} / {} = {}", dividend, divisor, quotient);
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_inifinity(), false);
    /// assert_eq!(quotient.is_divided_by_zero(), false);
    /// 
    /// divisor = 0_u8;
    /// quotient = dividend.saturating_div_uint(divisor);
    /// println!("{} / {} = {}", dividend, divisor, quotient);
    /// assert_eq!(quotient, U256::max());
    /// assert_eq!(quotient.is_overflow(), true);
    /// assert_eq!(quotient.is_inifinity(), true);
    /// assert_eq!(quotient.is_divided_by_zero(), true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    pub fn modular_rem(&self, rhs: &Self, modulo: &Self) -> Self
    {
        let mut lhs = Self::from_array(self.get_number().clone());
        lhs.modular_rem_assign(rhs, modulo);
        lhs
    }

    // pub fn modular_rem_assign(&mut self, rhs: &Self, modulo: &Self)
    /// Calculates the quotient when `self` % `modulo` is divided by
    /// `rhs` % `modulo`, and assigns the remainder to `self` back.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - If `rhs` % `modulo` is zero, `self` which is the remainder will be
    /// `zero`, and its flag `DIVIDED_BY_ZERO` will be set.
    /// - __It does not panic__ even if `rhs` % `modulo` is `zero`.
    /// - If `modulo` is `zero`, the flags such as `OVERFLOW`, `DIVIDED_BY_ZERO`,
    /// and `INFINITY` will be set.
    /// - 
    /// The `OVERFLOW`, `INFINITY`, and `DIVIDED_BY_ZERO` flags reflect
    /// historical overflow, which means if an overflow and/or divided-by-zero
    /// occurred even once before this current operation or `OVERFLOW`,
    /// `INFINITY`, and/or `DIVIDED_BY_ZERO` flag(s) is/are already set before
    /// this current operation, the `OVERFLOW`, `INFINITY`, and/or
    /// `DIVIDED_BY_ZERO` flag(s) is/are not changed even though this current
    /// operation does not cause overflow and/or divided-by-zero.
    /// - Overflow will not happen unless `rhs` is zero. __It does not panic__
    /// while the similar methods `saturating_div()` for primitive integer
    /// data type such as u8, u16, u32, u64, etc. will panic if `rhs` is zero.
    /// - If `rhs` is zero, the quotient will have maximum value of `BigUInt`
    /// type, and the flags of quotient such as `OVERFLOW`, `INFINITY`, and
    /// `DIVIDED_BY_ZERO` will be set. __It does not panic__ while the
    /// counterpart method `wrapping_div()` for primitive integer data type
    /// such as u8, u16, u32, u64, etc. will panic if `rhs` is zero.
    /// - The `OVERFLOW`, `INFINITY`, and `DIVIDED_BY_ZERO` flags reflect
    /// historical overflow, which means if an overflow and/or divided-by-zero
    /// occurred even once before this current operation or `OVERFLOW`,
    /// `INFINITY`, and/or `DIVIDED_BY_ZERO` flag(s) is/are already set before
    /// this current operation, the `OVERFLOW`, `INFINITY`, and/or
    /// `DIVIDED_BY_ZERO` flag(s) is/are not changed even though this current
    /// operation does not cause overflow and/or divided-by-zero.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [saturating_div_assign()](struct@BigUInt#method.saturating_div_assign)
    /// is proper rather than this method `saturating_div_assign_uint()`.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let mut divisor = 87_u16;
    /// println!("Originally,\na_biguint = {}", a_biguint);
    /// a_biguint.saturating_div_assign_uint(divisor);
    /// println!("After a_biguint.saturating_div_assign_uint({}),\na_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1419043551905275201680884938348044216837079832");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_inifinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// divisor = 0_u16;
    /// a_biguint.saturating_div_assign_uint(divisor);
    /// println!("After a_biguint.saturating_div_assign_uint({}),\na_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint, UU32::max());
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_inifinity(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    pub fn modular_rem_assign(&mut self, rhs: &Self, modulo: &Self)
    {
        self.wrapping_rem_assign(modulo);
        self.wrapping_rem_assign(&rhs.wrapping_rem(modulo));
    }

    // pub fn next_multiple_of(&self, rhs: &Self) -> Self
    /// Calculates the smallest value greater than or equal to `self` that is
    /// a multiple of `rhs`.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// - This function will panic if rhs is zero.
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
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// - This function will panic if rhs is zero.
    /// 
    /// # Features
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
    /// # Features
    /// When return value overflows
    /// (i.e., self > `(1 << (size_of::<T>() * N - 1))`),
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
    /// # Features
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



    /***** METHODS FOR EXPONENTIATION AND LOGARITHM WITH BIGUINT *****/

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
    /// # Features
    /// - Wrapping (modular) exponentiation.
    /// - It calls wrapping_pow() internally.
    /// - If overflowing happens, the `OVERFLOW` flag will be set.
    /// - All the flags are historical, which means, for example, if an
    /// overflow occurred even once before this current operation or
    /// `OVERFLOW` flag is already set before this current operation,
    /// the `OVERFLOW` flag is not changed even if this current operation
    /// does not cause overflow.
    /// 
    /// # Counterpart Method
    /// The method [pow_uint()](struct@BigUInt#method.pow_uint) is more
    /// efficient than this method `pow()` when the exponent `exp` is primitive
    /// unsigned integral data type such as u8, u16, u32, u64, and u128.
    /// If `rhs` is the primitive unsigned integral data type number,
    /// use the method [pow_uint()](struct@BigUInt#method.pow_uint).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a = U256::from_uint(234_u8);
    /// let mut exp = U256::from_uint(34_u8);
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
    /// # Features
    /// - Wrapping (modular) exponentiation.
    /// - It calls wrapping_pow() internally.
    /// - If overflowing happens, the `OVERFLOW` flag will be set.
    /// - All the flags are historical, which means, for example, if an
    /// overflow occurred even once before this current operation or
    /// `OVERFLOW` flag is already set before this current operation,
    /// the `OVERFLOW` flag is not changed even if this current operation
    /// does not cause overflow.
    /// 
    /// # Counterpart Method
    /// The method [pow_assign_uint()](struct@BigUInt#method.pow_assign_uint)
    /// is more efficient than this method `pow_assign()` when the exponent 
    /// exp` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128.
    /// If `rhs` is the primitive unsigned integral data type number, use the
    /// method [pow_assign_uint()](struct@BigUInt#method.pow_assign_uint).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = U256::from_uint(234_u8);
    /// let mut exp = U256::from_uint(34_u8);
    /// 
    /// // normal exponentiation
    /// a.pow_assign(&exp);
    /// println!("234 ** 34 = {}", a);
    /// assert_eq!(a.to_string(), "101771369680718065636717400052436696519017873276976456689251925337442881634304");
    /// 
    /// // wrapping (modular) exponentiation
    /// let old = a.clone();
    /// a = U256::from_uint(234_u8);
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
    /// # Features
    /// Wrapping (modular) exponentiation.
    /// 
    /// # Counterpart Method
    /// The method
    /// [wrapping_pow_uint()](struct@BigUInt#method.wrapping_pow_uint)
    /// is a bit faster than this method `wrapping_pow()` when the exponent
    /// `exp` is primitive unsigned integral data type such as u8, u16, u32,
    /// u64, and u128. If `rhs` is the primitive unsigned integral data type
    /// number, use the method
    /// [wrapping_pow_uint()](struct@BigUInt#method.wrapping_pow_uint).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a = U256::from_uint(234_u8);
    /// let mut exp = U256::from_uint(34_u8);
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
        let mut res = Self::from_array(self.get_number().clone());
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
    /// # Features
    /// Wrapping (modular) exponentiation.
    /// 
    /// # Counterpart Method
    /// The method
    /// [wrapping_pow_assign_uint()](struct@BigUInt#method.wrapping_pow_assign_uint)
    /// is a bit faster than this method `wrapping_pow_assign()` when the
    /// exponent `exp` is primitive unsigned integral data type such as u8,
    /// u16, u32, u64, and u128. If `rhs` is the primitive unsigned integral
    /// data type number, use the method
    /// [wrapping_pow_assign_uint()](struct@BigUInt#method.wrapping_pow_assign_uint).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = U256::from_uint(234_u8);
    /// let mut exp = U256::from_uint(34_u8);
    /// 
    /// // normal exponentiation
    /// a.wrapping_pow_assign(&exp);
    /// println!("234 ** 34 = {}", a);
    /// assert_eq!(a.to_string(), "101771369680718065636717400052436696519017873276976456689251925337442881634304");
    /// 
    /// // wrapping (modular) exponentiation
    /// let old = a.clone();
    /// a = U256::from_uint(234_u8);
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
    /// # Features
    /// Wrapping (modular) exponentiation.
    /// 
    /// # Counterpart Method
    /// The method
    /// [overflowing_pow_uint()](struct@BigUInt#method.overflowing_pow_uint)
    /// is a bit faster than this method `overflowing_pow()` when the exponent
    /// `exp` is primitive unsigned integral data type such as u8, u16, u32,
    /// u64, and u128. If `rhs` is the primitive unsigned integral data type
    /// number, use the method
    /// [overflowing_pow_uint()](struct@BigUInt#method.overflowing_pow_uint).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a = U256::from_uint(234_u8);
    /// let mut exp = U256::from_uint(34_u8);
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
        let mut res = Self::from_array(self.get_number().clone());
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
    /// # Features
    /// Wrapping (modular) exponentiation.
    /// 
    /// # Counterpart Method
    /// The method
    /// [overflow_pow_assign_uint()](struct@BigUInt#method.overflow_pow_assign_uint)
    /// is a bit faster than this method `overflow_pow_assign()` when the
    /// exponent `exp` is primitive unsigned integral data type such as u8,
    /// u16, u32, u64, and u128. If `rhs` is the primitive unsigned integral
    /// data type number, use the method
    /// [overflow_pow_assign_uint()](struct@BigUInt#method.overflow_pow_assign_uint).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = U256::from_uint(234_u8);
    /// let mut exp = U256::from_uint(34_u8);
    /// 
    /// // normal exponentiation
    /// let mut aa = a.overflowing_pow_assign(&exp);
    /// println!("234 ** 34 = {}, overflow = {}", a, aa);
    /// assert_eq!(a.to_string(), "101771369680718065636717400052436696519017873276976456689251925337442881634304");
    /// assert_eq!(aa, false);
    /// 
    /// // wrapping (modular) exponentiation
    /// let old = a.clone();
    /// a = U256::from_uint(234_u8);
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
        let old_overflow = self.is_overflow();
        self.reset_overflow();
        self.wrapping_pow_assign(exp);
        let current_overflow = self.is_overflow();
        if old_overflow || current_overflow
            { self.set_overflow(); }
        current_overflow
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
    /// # Features
    /// Checked wrapping (modular) exponentiation. 
    /// 
    /// # Counterpart Method
    /// The method
    /// [checked_pow_uint()](struct@BigUInt#method.checked_pow_uint) is a bit
    /// faster than this method `checked_pow()` when the exponent `exp` is
    /// primitive unsigned integral data type such as u8, u16, u32, u64, and
    /// u128. If `rhs` is the primitive unsigned integral data type number,
    /// use the method
    /// [checked_pow_uint()](struct@BigUInt#method.checked_pow_uint).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a = U256::from_uint(234_u8);
    /// let mut exp = U256::from_uint(34_u8);
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
        let mut res = Self::from_array(self.get_number().clone());
        res.saturating_pow_assign(exp);
        res
    }

    pub fn saturating_pow_assign(&mut self, exp: &Self)
    {
        if self.overflowing_pow_assign(&exp)
            { self.set_max(); }
    }

    pub fn modular_pow(&self, exp: &Self, modulo: &Self) -> Self
    {
        let mut res = Self::from_array(self.get_number().clone());
        res.modular_pow_assign(exp, modulo);
        res
    }

    pub fn modular_pow_assign(&mut self, exp: &Self, modulo: &Self)
    {
        if *self >= *modulo
            { self.wrapping_rem_assign(modulo); }
        let mut res = Self::one();
        let mut mexp = exp.clone();
        let zero = Self::zero();

        while mexp > zero
        {
            if mexp.is_odd()
                { res.modular_mul_assign(self, modulo); }
            self.modular_mul_assign(&self.clone(), modulo);
            mexp.shift_right_assign(1_u8);
        }
        *self = res;
    }


    // pub fn ilog(&self, base: Self) -> Self
    /// Calculates the logarithm of the number with respect to a `base`.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// - This function will panic if `self` is zero,
    /// or if `base` is less than 2.
    /// 
    /// # Output
    /// It returns the logarithm of the number with respect to an arbitrary
    /// `base`, rounded down.
    /// 
    /// # Counterpart Methods
    /// This method might not be optimized owing to implementation details;
    /// [ilog2()](struct@BigUInt#method.ilog2) can produce results more
    /// efficiently for base 2, and [ilog10()](struct@BigUInt#method.ilog10)
    /// can produce results more efficiently for base 10.
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
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// - This function will panic if `self` is zero.
    /// 
    /// # Output
    /// It returns the base 2 logarithm of the number, rounded down.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a = U256::from_uint(2_u8);
    /// let b = a.ilog2();
    /// println!("log_2(2) = {}", b);
    /// assert_eq!(b, U256::from_uint(1_u8));
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
    /// [checked_ilog2()](struct@BigUInt#method.checked_ilog2) can produce
    /// results more efficiently for base 2, and
    /// [checked_ilog10()](struct@BigUInt#method.checked_ilog10) can produce
    /// results more efficiently for base 10.
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
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a = U256::from_uint(2_u8);
    /// let b = a.ilog2();
    /// println!("log_2(2) = {}", b);
    /// assert_eq!(b, U256::from_uint(1_u8));
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

    pub fn sqrt(&self) -> Self
    {
        let mut adder;
        let mut highest = (Self::size_in_bits() - self.leading_zeros() as usize) >> 1;
        let mut high;
        let mut low;
        let mut mid;
        let mut res = Self::zero();
        let mut sum;
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
                    adder = Self::generate_check_bits_(mid);
                    sum = res.wrapping_add(&adder);
                    let (sq, b_overflow) = sum.overflowing_mul(&sum);
                    if !b_overflow && (sq < *self)
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
                    else if b_overflow || (sq > *self)
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

    pub fn root(self, exp: Self) -> Self
    {
        let mut adder;
        let mut highest = Self::from_uint(Self::size_in_bits() - self.leading_zeros() as usize).wrapping_add(&exp).into_usize();
        let mut high;
        let mut low;
        let mut mid;
        let mut res = Self::zero();
        let mut sum;
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
                    adder = Self::generate_check_bits_(mid);
                    sum = res.wrapping_add(&adder);
                    let (sq, b_overflow) = sum.overflowing_pow(&exp);
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


    // pub fn midpoint(&self, rhs: &Self) -> Self
    /// Calculates the middle point of `self` and `rhs`.
    /// 
    /// # Features
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



    /***** METHODS FOR TESTING PRIME NUMBER *****/
    pub fn is_prime_using_miller_rabin(&self, repetition: usize) -> bool
    {
        if self.is_zero_or_one() || self.is_even()
            { return false; }
        
        if self.is_uint(2_u8) ||  self.is_uint(3_u8)
            { return true; }

        if self.le_uint(u64::MAX)
        {
            let small_self = self.into_u64();
            return small_self.is_prime_using_miller_rabin(repetition);
        }

        let a_list = [2_u64, 7, 61, 325, 9375, 28178, 450775, 9780504, 1795265022];
        let len = a_list.len();
        let common = if len < repetition { len } else { repetition };
        let mut i = 0;
        while i < common
        {
            if !self.test_miller_rabin(&Self::from_uint(a_list[i]))
                { return false; }
            i += 1;
        }

        let mut a = a_list[len-1] + 1;
        for _ in i..repetition
        {
            if !self.test_miller_rabin(&Self::from_uint(a))
                { return false; }
            a += 1;
        }
        true
     }
 
     /// Performs Millar Rabin method with a number less than `self`.
     fn test_miller_rabin(&self, a: &Self) -> bool
     {
         let self_minus_one = self.wrapping_sub_uint(1_u8);
         let mut d = self_minus_one.clone();
         while d.is_even()
         {
             if a.modular_pow(&d, self) == self_minus_one
                 { return true; }
             d.shift_right_assign(1_u8);
         }
         let tmp = a.modular_pow(&d, self);
         return tmp == self_minus_one || tmp.is_one();
    }



    /***** METHODS FOR BIT OPERATION *****/

    // pub fn shift_left<U>(&self, n: U) -> Self
    /// Shift left the field `number: [T;N]` to the left by `n`.
    /// 
    /// # Features
    /// 'Shift left' means 'move left all bits'. So, if `10011010` is shifted
    /// left by 2, it will be `01101000`, for example.
    /// 
    /// # Output
    /// It returns the left-shifted version of `self`, which is shifted to the
    /// left by `n` bits.
    /// 
    /// # Overflow
    /// For BigUInt, 'overflow' does not have the same meaning as that of
    /// primitive unsigned integer data type. For BigUInt, overflow means that
    /// carry occurs, while overflow means that all the bits are pushed outside
    /// for primitive unsigned integer data type.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn shift_left<U>(&self, n: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut res = Self::from_array(self.get_number().clone());
        res.shift_left_assign(n);
        res
    }

    // pub fn shift_left_assign<U>(&mut self, n: U)
    /// shifts the field `number: [T;N]` to the left by `n`, and assign it
    /// to `self` back.
    /// 
    /// # Features
    /// 'Shift left' means 'move left all bits'. So, if `10011010` is shifted
    /// left by 2, it will be `01101000`, for example.
    /// 
    /// # Overflow
    /// For BigUInt, 'overflow' does not have the same meaning as that of
    /// primitive unsigned integer data type. For BigUInt, overflow means that
    /// carry occurs, while overflow means that all the bits are pushed outside
    /// for primitive unsigned integer data type.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn shift_left_assign<U>(&mut self, n: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let size_t_bits = T::size_in_bits();
        let chunk_num = n.wrapping_div(U::usize_as_smalluint(size_t_bits)).into_usize();
        let piece_num = n.wrapping_rem(U::usize_as_smalluint(size_t_bits)).into_usize();
        let zero = T::zero();
        if chunk_num > 0
        {
            for i in N-chunk_num..N
            {
                if !self.get_num_(i).is_zero()
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
        // if (self.get_num_(N-1).leading_zeros() as usize) < piece_num
        //     { self.set_overflow(); }

        let mut num: T;
        let mut carry = zero;
        for idx in chunk_num..N
        {
            num = (self.get_num_(idx) << T::usize_as_smalluint(piece_num)) | carry;
            carry = self.get_num_(idx) >> T::usize_as_smalluint(size_t_bits - piece_num);
            self.set_num_(idx, num);
        }
        if !carry.is_zero()
            { self.set_overflow(); }
    }

    // pub fn checked_shift_left<U>(&self, n: U) -> Option<Self>
    /// Shift left the field `number: [T;N]` to the left by `n`.
    /// 
    /// # Features
    /// 'Shift left' means 'move left all bits'. So, if `10011010` is shifted
    /// left by 2, it will be `01101000`, for example.
    /// 
    /// # Output
    /// It returns the left-shifted version of `self`, which is shifted to the
    /// left by `n` bits, wrapped by `some` of enum `Option`.
    /// If n is greater than or equal to the size of the type `T` * N * 8,
    /// it returns `None`.
    /// 
    /// # Overflow
    /// For BigUInt, 'overflow' does not have the same meaning as that of
    /// primitive unsigned integer data type. For BigUInt, overflow means that
    /// carry occurs, while overflow means that all the bits are pushed outside
    /// for primitive unsigned integer data type.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn checked_shift_left<U>(&self, n: U) -> Option<Self>
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if Self::size_in_bits().into_u128() < n.into_u128()
        {
            return None;
        }
        let mut res = self.clone();
        res.shift_left_assign(n);
        Some(res)
    }

    // pub fn unchecked_shift_left<U>(&self, n: U) -> Self
    /// Shift left the field `number: [T;N]` to the left by `n`.
    /// 
    /// # Features
    /// 'Shift left' means 'move left all bits'. So, if `10011010` is shifted
    /// left by 2, it will be `01101000`, for example.
    /// 
    /// # Output
    /// It returns the left-shifted version of `self`, which is shifted to the
    /// left by `n` bits.
    /// 
    /// # Overflow
    /// For BigUInt, 'overflow' does not have the same meaning as that of
    /// primitive unsigned integer data type. For BigUInt, overflow means that
    /// carry occurs, while overflow means that all the bits are pushed outside
    /// for primitive unsigned integer data type.
    /// 
    /// # Panic
    /// If n is greater than or equal to the size of the type `T` * N * 8,
    /// it will panic.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    pub fn unchecked_shift_left<U>(&self, n: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.checked_shift_left(n).unwrap()
    }

    // pub fn shift_right<U>(&self, n: U) -> Self
    /// Shift right the field `number: [T;N]` to the right by `n`.
    /// 
    /// # Features
    /// 'Shift right' means 'move right all bits'. So, if `10011010` is shifted
    /// right by 2, it will be `00100110`, for example.
    /// 
    /// # Output
    /// It returns the right-shifted version of `self`. which is shifted to the
    /// right by `n` bits.
    /// 
    /// # Underflow
    /// For BigUInt, 'underflow' does not have the same meaning as that of
    /// primitive unsigned integer data type. For BigUInt, underflow means that
    /// carry occurs, while underflow means that all the bits are pushed outside
    /// for primitive unsigned integer data type.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn shift_right<U>(&self, n: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut res = Self::from_array(self.get_number().clone());
        res.shift_right_assign(n);
        res
    }

    // pub fn shift_right_assign<U>(&mut self, n: U)
    /// shifts the field `number: [T;N]` to the right by `n`, and assign it
    /// to `self` back.
    /// 
    /// # Features
    /// 'Shift right' means 'move right all bits'. So, if `10011010` is shifted
    /// right by 2, it will be `00100110`, for example.
    /// 
    /// # Underflow
    /// For BigUInt, 'underflow' does not have the same meaning as that of
    /// primitive unsigned integer data type. For BigUInt, underflow means that
    /// carry occurs, while underflow means that all the bits are pushed outside
    /// for primitive unsigned integer data type.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn shift_right_assign<U>(&mut self, n: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let size_t_bits = T::size_in_bits();
        let chunk_num = (n / U::usize_as_smalluint(size_t_bits)).into_usize();
        let piece_num = (n % U::usize_as_smalluint(size_t_bits)).into_usize();
        let zero = T::zero();
        if chunk_num > 0
        {
            for i in 0..chunk_num
            {
                if self.get_num_(i) != zero
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
        if (self.get_num_(0) << T::usize_as_smalluint(size_t_bits - piece_num)) != zero
            { self.set_underflow(); }

        let mut num: T;
        let mut carry = T::zero();
        let mut idx = N - 1 - chunk_num;
        loop
        {
            num = (self.get_num_(idx) >> T::usize_as_smalluint(piece_num)) | carry;
            carry = self.get_num_(idx) << T::usize_as_smalluint(size_t_bits - piece_num);
            self.set_num_(idx, num);
            if idx == 0
                { break; }
            idx -= 1;
        }
        if carry != zero
            { self.set_underflow(); }
    }

    // pub fn checked_shift_right<U>(&self, n: U) -> Option<Self>
    /// Shift right the field `number: [T;N]` to the right by `n`.
    /// 
    /// # Features
    /// 'Shift right' means 'move right all bits'. So, if `10011010` is shifted
    /// right by 2, it will be `00100110`, for example.
    /// 
    /// # Output
    /// It returns the right-shifted version of `self`. which is shifted to the
    /// right by `n` bits, wrapped by `some` of enum `Option`.
    /// If n is greater than or equal to the size of the type `T` * N * 8,
    /// it returns `None`.
    /// 
    /// # Underflow
    /// For BigUInt, 'underflow' does not have the same meaning as that of
    /// primitive unsigned integer data type. For BigUInt, underflow means that
    /// carry occurs, while underflow means that all the bits are pushed outside
    /// for primitive unsigned integer data type.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn checked_shift_right<U>(&self, n: U) -> Option<Self>
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if Self::size_in_bits().into_u128() < n.into_u128()
        {
            return None;
        }
        let mut res = self.clone();
        res.shift_right_assign(n);
        Some(res)
    }

    // pub fn unchecked_shift_right<U>(&mut self, n: U) -> Self
    /// shifts the field `number: [T;N]` to the right by `n`, and assign it
    /// to `self` back.
    /// 
    /// # Features
    /// 'Shift right' means 'move right all bits'. So, if `10011010` is shifted
    /// right by 2, it will be `00100110`, for example.
    /// 
    /// # Output
    /// It returns the right-shifted version of `self`. which is shifted to the
    /// right by `n` bits.
    /// 
    /// # Underflow
    /// For BigUInt, 'underflow' does not have the same meaning as that of
    /// primitive unsigned integer data type. For BigUInt, underflow means that
    /// carry occurs, while underflow means that all the bits are pushed outside
    /// for primitive unsigned integer data type.
    /// 
    /// # Panic
    /// If n is greater than or equal to the size of the type `T` * N * 8,
    /// it will panic.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    pub fn unchecked_shift_right<U>(&mut self, n: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.checked_shift_right(n).unwrap()
    }

    // pub fn rotate_left<U>(&self, n: U) -> Self
    /// Rotates the field `number: [T;N]` to the left by `n`.
    /// 
    /// # Features
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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
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
    /// # Features
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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
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
    /// # Features
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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
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
    /// # Features
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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
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
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let a = U512::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = U512::from_str_radix("11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000", 2).unwrap();
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
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let a = U512::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = U512::zero();
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
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = U512::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = U512::from_str_radix("11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000", 2).unwrap();
    /// a.and_assign(&b);
    /// println!("a = {}", a.to_string_with_radix(2).unwrap());
    /// assert_eq!(a, U512::from_str_radix("11110000000000001100000000000011100010000001000110101010000000001111000000000000110000000000001110001000000100011010101000000000111100000000000011000000000000111000100000010001101010100000000011110000000000001100000000000011100010000001000110101010000000001111000000000000110000000000001110001000000100011010101000000000", 2).unwrap());
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = U512::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = U512::zero();
    /// a.and_assign(&b);
    /// println!("a = {}", a.to_string_with_radix(2).unwrap());
    /// assert_eq!(a, U512::zero());
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
    /// use cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let a = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = U256::from_str_radix("11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000", 2).unwrap();
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
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let a = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = U256::max();
    /// let c = a.or(&b);
    /// 
    /// println!("a = {}", a.to_string_with_radix(2).unwrap());
    /// println!("b = {}", b.to_string_with_radix(2).unwrap());
    /// println!("a | b = {}", c.to_string_with_radix(2).unwrap());
    /// 
    /// assert_eq!(c, U256::max());
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
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = U256::from_str_radix("11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000", 2).unwrap();
    /// a.or_assign(&b);
    /// 
    /// println!("a = {}", a.to_string_with_radix(2).unwrap());
    /// 
    /// assert_eq!(a, U256::from_str_radix("1111111100001111111111000011111111101110011101111111111101010101111111110000111111111100001111111110111001110111111111110101010111111111000011111111110000111111111011100111011111111111010101011111111100001111111111000011111111101110011101111111111101010101", 2).unwrap());
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = U256::max();
    /// a.or_assign(&b);
    /// 
    /// println!("a = {}", a.to_string_with_radix(2).unwrap());
    /// 
    /// assert_eq!(a, U256::max());
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
    /// use cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let a = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = U256::from_str_radix("11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000", 2).unwrap();
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
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let a = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = U256::max();
    /// let c = a.xor(&b);
    /// 
    /// println!("a = {}", a.to_string_with_radix(2).unwrap());
    /// println!("b = {}", b.to_string_with_radix(2).unwrap());
    /// println!("a ^ b = {}", c.to_string_with_radix(2).unwrap());
    /// 
    /// assert_eq!(c, U256::max());
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
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = U256::from_str_radix("11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000", 2).unwrap();
    /// a.xor_assign(&b);
    /// 
    /// println!("a = {}", a.to_string_with_radix(2).unwrap());
    /// 
    /// assert_eq!(a, U256::from_str_radix("1111111100001111111111000011111111101110011101111111111101010101111111110000111111111100001111111110111001110111111111110101010111111111000011111111110000111111111011100111011111111111010101011111111100001111111111000011111111101110011101111111111101010101", 2).unwrap());
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = U256::max();
    /// a.xor_assign(&b);
    /// 
    /// println!("a = {}", a.to_string_with_radix(2).unwrap());
    /// 
    /// assert_eq!(a, U256::max());
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
    /// # Features
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
    /// # Features
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
        for i in 0..N/2
        {
            let tmp = self.get_num_(i).swap_bytes();
            self.set_num_(i, self.get_num_(N-1-i).swap_bytes());
            self.set_num_(N-1-i, tmp);
        }
    }



    /***** METHODS FOR CONVERTING INTO OTHER TYPES WITH/WITHOUT LOSS *****/

    // pub fn into_biguint<U, const M: usize>(&self) -> BigUInt<U, M>
    /// Converts `self` into another kind of `BigUInt<U, M>`.
    /// 
    /// # Features
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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
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

    // pub fn into_uint<U>(&self) -> U
    /// Converts `self` into `U`-type small value such as `u8`, `u16`, `u32`,
    /// `u64`, and `u128` type value.
    /// This mathod get_uint() is useful especially when `self` has `U`-type
    /// small unsigned integer sized value and you want to cast `self` into
    /// `U`-type small unsigned integer with a small value.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the `U`-type small unsigned integer
    /// which are its lowest elements.
    /// 
    /// # Features
    /// If the size of the value that `self` has is bigger than
    /// the size of `U`, the higher-bit portion will be truncated.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn into_uint<U>(&self) -> U
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if T::size_in_bytes() >= U::size_in_bytes()
        {
            SharedValues::<U, T>::from_src(self.get_num_(0)).get_des()
        }
        else
        {
            match U::size_in_bytes()
            {
                2 => { U::u16_as_smalluint(self.into_u16()) },
                4 => { U::u32_as_smalluint(self.into_u32()) },
                8 => { U::u64_as_smalluint(self.into_u64()) },
                _ => { U::u128_as_smalluint(self.into_u128()) },
            }
        }
    }

    // pub fn into_u128(&self) -> u128
    /// Converts `self` into `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the lowest sixteen bytes of `self` as `u128`.
    /// 
    /// # Features
    /// It takes the lowest `u128`-sized bytes, that is, the lowest sixteen
    /// bytes from `self`, and return then as `u128` data type.
    /// It is usually lossy conversion.
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
                    for i in 0..if 16 < N {16} else {N}
                        { num.set_ubyte_(i, self.get_num_(i).into_u8()); }
                },
            2 => {
                    for i in 0..if 8 < N {8} else {N}
                        { num.set_ushort_(i, self.get_num_(i).into_u16()); }
                },
            4 => {
                    for i in 0..if 4 < N {4} else {N}
                        { num.set_uint_(i, self.get_num_(i).into_u32()); }
                },
            8 => {
                    for i in 0..if 2 < N {2} else {N}
                        { num.set_ulong_(i, self.get_num_(i).into_u64()); }
                },
            _ => { return self.get_num_(0).into_u128(); },
        }
        num.get()
    }

    // pub fn into_u64(&self) -> u64
    /// Converts `self` into `u64`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the lowest eight bytes of `self` as `u64`.
    /// 
    /// # Features
    /// It takes the lowest `u64`-sized bytes, that is, the lowest eight
    /// bytes from `self`, and return then as `u64` data type.
    /// It is usually lossy conversion.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn into_u64(&self) -> u64
    {
        let mut num = LongerUnion::new();
        match T::size_in_bytes()
        {
            1 => {
                    for i in 0..if 8 < N {8} else {N}
                        { num.set_ubyte_(i, self.get_num_(i).into_u8()); }
                },
            2 => {
                    for i in 0..if 4 < N {4} else {N}
                    { num.set_ushort_(i, self.get_num_(i).into_u16()); }
                },
            4 => {
                    for i in 0..if 2 < N {2} else {N}
                        { num.set_uint_(i, self.get_num_(i).into_u32()); }
                },
            8 => { return self.get_num_(0).into_u64(); },
            _ => { num.set(self.number[0].into_u128()); },
        }
        num.get_ulong_(0)
    }

    // pub fn into_u32(&self) -> u32
    /// Converts `self` into `u32`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the lowest four bytes of `self` as `u32`.
    /// 
    /// # Features
    /// It takes the lowest `u32`-sized bytes, that is, the lowest four
    /// bytes from `self`, and return then as `u32` data type.
    /// It is usually lossy conversion.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    pub fn into_u32(&self) -> u32
    {
        let mut num = LongerUnion::new();
        match T::size_in_bytes()
        {
            1 => {
                    for i in 0..if 4 < N {4} else {N}
                        { num.set_ubyte_(i, self.get_num_(i).into_u8()); }
                },
            2 => {
                    for i in 0..if 2 < N {2} else {N}
                        { num.set_ushort_(i, self.get_num_(i).into_u16()); }
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
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the lowest two bytes of `self` as `u16`.
    /// 
    /// # Features
    /// It takes the lowest `u16`-sized bytes, that is, the lowest two
    /// bytes from `self`, and return then as `u16` data type.
    /// It is usually lossy conversion.
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
                    for i in 0..if 2 < N {2} else {N}
                        { num.set_ubyte_(i, self.get_num_(i).into_u8()); }
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
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the lowest one byte of `self` as `u32`.
    /// 
    /// # Features
    /// It takes the lowest `u8`-sized byte, that is, the lowest one
    /// byte from `self`, and return it as `u8` data type.
    /// It is usually lossy conversion.
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
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the lowest `usize` long part of `self` as `usize`.
    /// 
    /// # Features
    /// It takes the lowest `usize`-sized bytes from `self`,
    /// and return them as `usize` data type. It is usually lossy conversion.
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
    /// # Features
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
    /// # Features
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
    /// # Features
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
    /// # Features
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

    // pub fn to_le(&self) -> Self
    /// Converts `self` to little endian from the target’s endianness.
    /// 
    /// # Features
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

    // pub fn to_le(&self) -> Self
    /// Converts `self` to little endian from the target’s endianness.
    /// 
    /// # Features
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
    /// # Features
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
    /// # Features
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
    /// # Error
    /// | argument | value                              | Caused Error                      |
    /// |----------|------------------------------------|-----------------------------------|
    /// | `radix`  | less than `2` or greater than `62` | `NumberErr::OutOfValidRadixRange` |
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
    /// # Error
    /// | argument | value                              | Caused Error                      |
    /// |----------|------------------------------------|-----------------------------------|
    /// | `radix`  | less than `2` or greater than `62` | `NumberErr::OutOfValidRadixRange` |
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
                (dividend, remainder) = dividend.divide_fully_uint(T::usize_as_smalluint(radix));
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
    /// # Error
    /// | argument | value                              | Caused Error                      |
    /// |----------|------------------------------------|-----------------------------------|
    /// | `radix`  | less than `2` or greater than `62` | `NumberErr::OutOfValidRadixRange` |
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
            (dividend, remainder) = dividend.divide_fully_uint(T::usize_as_smalluint(radix));
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

    // pub fn get_all_flags(&self) -> u8
    /// Gets all the flag bits.
    ///
    /// # Output
    /// It returns all the flag bits.
    ///
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    fn get_all_flags(&self) -> u8
    {
        self.flag
    }

    // pub fn reset_all_flags(&mut self)
    /// Resets all flag bits to be `0`.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    #[inline]
    fn reset_all_flags(&mut self)
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
    /// Sets `OVERFLOW` flag.
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
    /// Resets `OVERFLOW` flag.
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
    /// Checks whether or not `OVERFLOW` flag is set.
    /// 
    /// # Output
    /// It returns `true` if the `OVERFLOW` flag is set.
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
    /// Sets `UNDERFLOW` flag.
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
    /// Reets `UNDERFLOW` flag.
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
    /// Checks whether or not `UNDERFLOW` flag is set.
    /// 
    /// # Output
    /// It returns `true` if the `UNDERFLOW` flag is set.
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
    /// Sets both `OVERFLOW` flag and `UNDERFLOW` flag.
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
    /// Resets both `OVERFLOW` flag and `UNDERFLOW` flag.
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
    /// Checks whether or not both `OVERFLOW` flag and `UNDERFLOW` flag are all set.
    /// 
    /// # Output
    /// It returns `true` if both of the `OVERFLOW` flag and the `UNDERFLOW` flag
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
