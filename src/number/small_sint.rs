// Copyright 2023, 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module that contains a trait Int for generic type of primitive signed
//! integral data types for the counter part of uint.

// #![warn(missing_docs)]
// #![warn(rustdoc::missing_doc_code_examples)]
#![allow(missing_docs)]
// #![allow(rustdoc::missing_doc_code_examples)]

use std::fmt::Debug;
use std::mem::size_of;
use std::ops::*;
use std::cmp::{Eq, Ord};

/// For generic type of primitive signed integral data types
/// for the counter part of uint.
///  
/// Here, the generic type of primitive signed integral data types includes:
/// i8, i16, i32, i64, i128, and isize. You will hardly use the trait Int unless
/// you improve the crate Cryptocol or create addional libraries that works with
/// the crate Cryptocol. But, if you only use the crate Cryptocol, you can
/// forget about this trait Int.
/// 
pub trait SmallSInt: Copy + Debug
            + Add + AddAssign + Sub + SubAssign + Mul + MulAssign + Div + DivAssign
            + Rem + RemAssign
            + Shl + ShlAssign + Shr + ShrAssign
            + Eq + Ord
{
    fn wrapping_add(self, rhs: Self) -> Self;
    fn wrapping_sub(self, rhs: Self) -> Self;
    fn wrapping_mul(self, rhs: Self) -> Self;
    fn wrapping_div(self, rhs: Self) -> Self;
    fn into_f64(self) -> f64;
    fn into_f32(self) -> f32;
    fn into_i128(self) -> i128;
    fn into_i64(self) -> i64;
    fn into_i32(self) -> i32;
    fn into_i16(self) -> i16;
    fn into_i8(self) -> i8;
    fn into_isize(self) -> isize;
    fn into_bool(self) -> bool;
    fn zero() -> Self;
    fn one() -> Self;
    fn max() -> Self;
    fn min() -> Self;
    fn num(n: i128) -> Self;
    fn size_in_bytes() -> usize;
    fn size_in_bits() -> usize;
}

impl SmallSInt for i8
{
    fn wrapping_add(self, rhs: Self) -> Self    { self.wrapping_add(rhs) }
    fn wrapping_sub(self, rhs: Self) -> Self    { self.wrapping_sub(rhs) }
    fn wrapping_mul(self, rhs: Self) -> Self    { self.wrapping_mul(rhs) }
    fn wrapping_div(self, rhs: Self) -> Self    { self.wrapping_div(rhs) }
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_i64(self) -> i64    { self as i64 }
    fn into_i32(self) -> i32    { self as i32 }
    fn into_i16(self) -> i16    { self as i16 }
    fn into_i8(self) -> i8      { self as i8 }
    fn into_isize(self) -> isize { self as isize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn max() -> Self            { Self::MAX }
    fn min() -> Self            { Self::MIN }
    fn num(n: i128) -> Self     { n as Self }
    fn size_in_bytes() -> usize { size_of::<Self>() }
    fn size_in_bits() -> usize  { size_of::<Self>() * 8 }
}

impl SmallSInt for i16
{
    fn wrapping_add(self, rhs: Self) -> Self    { self.wrapping_add(rhs) }
    fn wrapping_sub(self, rhs: Self) -> Self    { self.wrapping_sub(rhs) }
    fn wrapping_mul(self, rhs: Self) -> Self    { self.wrapping_mul(rhs) }
    fn wrapping_div(self, rhs: Self) -> Self    { self.wrapping_div(rhs) }
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_i64(self) -> i64    { self as i64 }
    fn into_i32(self) -> i32    { self as i32 }
    fn into_i16(self) -> i16    { self as i16 }
    fn into_i8(self) -> i8      { self as i8 }
    fn into_isize(self) -> isize { self as isize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn max() -> Self            { Self::MAX }
    fn min() -> Self            { Self::MIN }
    fn num(n: i128) -> Self     { n as Self }
    fn size_in_bytes() -> usize { size_of::<Self>() }
    fn size_in_bits() -> usize  { size_of::<Self>() * 8 }
}

impl SmallSInt for i32
{
    fn wrapping_add(self, rhs: Self) -> Self    { self.wrapping_add(rhs) }
    fn wrapping_sub(self, rhs: Self) -> Self    { self.wrapping_sub(rhs) }
    fn wrapping_mul(self, rhs: Self) -> Self    { self.wrapping_mul(rhs) }
    fn wrapping_div(self, rhs: Self) -> Self    { self.wrapping_div(rhs) }
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_i64(self) -> i64    { self as i64 }
    fn into_i32(self) -> i32    { self as i32 }
    fn into_i16(self) -> i16    { self as i16 }
    fn into_i8(self) -> i8      { self as i8 }
    fn into_isize(self) -> isize { self as isize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn max() -> Self            { Self::MAX }
    fn min() -> Self            { Self::MIN }
    fn num(n: i128) -> Self     { n as Self }
    fn size_in_bytes() -> usize { size_of::<Self>() }
    fn size_in_bits() -> usize  { size_of::<Self>() * 8 }
}

impl SmallSInt for i64
{
    fn wrapping_add(self, rhs: Self) -> Self    { self.wrapping_add(rhs) }
    fn wrapping_sub(self, rhs: Self) -> Self    { self.wrapping_sub(rhs) }
    fn wrapping_mul(self, rhs: Self) -> Self    { self.wrapping_mul(rhs) }
    fn wrapping_div(self, rhs: Self) -> Self    { self.wrapping_div(rhs) }
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_i64(self) -> i64    { self as i64 }
    fn into_i32(self) -> i32    { self as i32 }
    fn into_i16(self) -> i16    { self as i16 }
    fn into_i8(self) -> i8      { self as i8 }
    fn into_isize(self) -> isize { self as isize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn max() -> Self            { Self::MAX }
    fn min() -> Self            { Self::MIN }
    fn num(n: i128) -> Self     { n as Self }
    fn size_in_bytes() -> usize { size_of::<Self>() }
    fn size_in_bits() -> usize  { size_of::<Self>() * 8 }
}

impl SmallSInt for i128
{
    fn wrapping_add(self, rhs: Self) -> Self    { self.wrapping_add(rhs) }
    fn wrapping_sub(self, rhs: Self) -> Self    { self.wrapping_sub(rhs) }
    fn wrapping_mul(self, rhs: Self) -> Self    { self.wrapping_mul(rhs) }
    fn wrapping_div(self, rhs: Self) -> Self    { self.wrapping_div(rhs) }
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_i64(self) -> i64    { self as i64 }
    fn into_i32(self) -> i32    { self as i32 }
    fn into_i16(self) -> i16    { self as i16 }
    fn into_i8(self) -> i8      { self as i8 }
    fn into_isize(self) -> isize { self as isize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn max() -> Self            { Self::MAX }
    fn min() -> Self            { Self::MIN }
    fn num(n: i128) -> Self     { n as Self }
    fn size_in_bytes() -> usize { size_of::<Self>() }
    fn size_in_bits() -> usize  { size_of::<Self>() * 8 }
}

impl SmallSInt for isize
{
    fn wrapping_add(self, rhs: Self) -> Self    { self.wrapping_add(rhs) }
    fn wrapping_sub(self, rhs: Self) -> Self    { self.wrapping_sub(rhs) }
    fn wrapping_mul(self, rhs: Self) -> Self    { self.wrapping_mul(rhs) }
    fn wrapping_div(self, rhs: Self) -> Self    { self.wrapping_div(rhs) }
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_i64(self) -> i64    { self as i64 }
    fn into_i32(self) -> i32    { self as i32 }
    fn into_i16(self) -> i16    { self as i16 }
    fn into_i8(self) -> i8      { self as i8 }
    fn into_isize(self) -> isize { self as isize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn max() -> Self            { Self::MAX }
    fn min() -> Self            { Self::MIN }
    fn num(n: i128) -> Self     { n as Self }
    fn size_in_bytes() -> usize { size_of::<Self>() }
    fn size_in_bits() -> usize  { size_of::<Self>() * 8 }
}
