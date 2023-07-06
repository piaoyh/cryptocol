// Copyright 2023 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module that contains a trait Numeric for generic type of a primitive
//! signed integral data type and a primitive unsigned integral data type.

#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

use std::fmt::Debug;
use std::ops::*;
use std::cmp::{Eq, Ord};

/// Trait Numeric is for generic type of primitive signed/unsigned integral data
/// types for all modules of the crate Cryptocol.
/// 
/// Trait Numeric is trait Uint + trait Int. Here, the generic type of primitive
/// signed/unsigned integral data types includes: u8, u16, u32, u64, u128, usize,
/// i8, i16, i32, i64,i128, and isize. You will hardly use the trait Numeric
/// unless you improve the crate Cryptocol or create addional libraries that
/// works with the crate Cryptocol. But, if you only use the crate Cryptocol,
/// you can forget about this trait Numeric.
/// 
pub trait Numeric: Copy + Debug
                + Add + AddAssign + Sub + SubAssign + Mul + MulAssign + Div + DivAssign
                + Shl + ShlAssign + Shr + ShrAssign
                + Eq + Ord
{
    fn wrapping_add(self, rhs: Self) -> Self;
    fn wrapping_sub(self, rhs: Self) -> Self;
    fn wrapping_mul(self, rhs: Self) -> Self;
    fn wrapping_div(self, rhs: Self) -> Self;
    fn into_f64(self) -> f64;
    fn into_f32(self) -> f32;
    fn into_u128(self) -> u128;
    fn into_u64(self) -> u64;
    fn into_u32(self) -> u32;
    fn into_u16(self) -> u16;
    fn into_u8(self) -> u8;
    fn into_usize(self) -> usize;
    fn into_i128(self) -> i128;
    fn into_i64(self) -> i64;
    fn into_i32(self) -> i32;
    fn into_i16(self) -> i16;
    fn into_i8(self) -> i8;
    fn into_isize(self) -> isize;
    fn into_bool(self) -> bool;
    fn zero() -> Self;
    fn one() -> Self;
    fn Max() -> Self;
    fn Min() -> Self;
    fn num(n: i128) -> Self;
}

impl Numeric for u8
{
    fn wrapping_add(self, rhs: Self) -> Self    { self.wrapping_add(rhs) }
    fn wrapping_sub(self, rhs: Self) -> Self    { self.wrapping_sub(rhs) }
    fn wrapping_mul(self, rhs: Self) -> Self    { self.wrapping_mul(rhs) }
    fn wrapping_div(self, rhs: Self) -> Self    { self.wrapping_div(rhs) }
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_u64(self) -> u64    { self as u64 }
    fn into_u32(self) -> u32    { self as u32 }
    fn into_u16(self) -> u16    { self as u16 }
    fn into_u8(self) -> u8      { self as u8 }
    fn into_usize(self) -> usize { self as usize }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_i64(self) -> i64    { self as i64 }
    fn into_i32(self) -> i32    { self as i32 }
    fn into_i16(self) -> i16    { self as i16 }
    fn into_i8(self) -> i8      { self as i8 }
    fn into_isize(self) -> isize { self as isize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { Self::MAX }
    fn Min() -> Self            { Self::MIN }
    fn num(n: i128) -> Self     { n as Self }
}

impl Numeric for u16
{
    fn wrapping_add(self, rhs: Self) -> Self    { self.wrapping_add(rhs) }
    fn wrapping_sub(self, rhs: Self) -> Self    { self.wrapping_sub(rhs) }
    fn wrapping_mul(self, rhs: Self) -> Self    { self.wrapping_mul(rhs) }
    fn wrapping_div(self, rhs: Self) -> Self    { self.wrapping_div(rhs) }
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_u64(self) -> u64    { self as u64 }
    fn into_u32(self) -> u32    { self as u32 }
    fn into_u16(self) -> u16    { self as u16 }
    fn into_u8(self) -> u8      { self as u8 }
    fn into_usize(self) -> usize { self as usize }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_i64(self) -> i64    { self as i64 }
    fn into_i32(self) -> i32    { self as i32 }
    fn into_i16(self) -> i16    { self as i16 }
    fn into_i8(self) -> i8      { self as i8 }
    fn into_isize(self) -> isize { self as isize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { Self::MAX }
    fn Min() -> Self            { Self::MIN }
    fn num(n: i128) -> Self     { n as Self }
}

impl Numeric for u32
{
    fn wrapping_add(self, rhs: Self) -> Self    { self.wrapping_add(rhs) }
    fn wrapping_sub(self, rhs: Self) -> Self    { self.wrapping_sub(rhs) }
    fn wrapping_mul(self, rhs: Self) -> Self    { self.wrapping_mul(rhs) }
    fn wrapping_div(self, rhs: Self) -> Self    { self.wrapping_div(rhs) }
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_u64(self) -> u64    { self as u64 }
    fn into_u32(self) -> u32    { self as u32 }
    fn into_u16(self) -> u16    { self as u16 }
    fn into_u8(self) -> u8      { self as u8 }
    fn into_usize(self) -> usize { self as usize }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_i64(self) -> i64    { self as i64 }
    fn into_i32(self) -> i32    { self as i32 }
    fn into_i16(self) -> i16    { self as i16 }
    fn into_i8(self) -> i8      { self as i8 }
    fn into_isize(self) -> isize { self as isize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { Self::MAX }
    fn Min() -> Self            { Self::MIN }
    fn num(n: i128) -> Self     { n as Self }
}

impl Numeric for u64
{
    fn wrapping_add(self, rhs: Self) -> Self    { self.wrapping_add(rhs) }
    fn wrapping_sub(self, rhs: Self) -> Self    { self.wrapping_sub(rhs) }
    fn wrapping_mul(self, rhs: Self) -> Self    { self.wrapping_mul(rhs) }
    fn wrapping_div(self, rhs: Self) -> Self    { self.wrapping_div(rhs) }
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_u64(self) -> u64    { self as u64 }
    fn into_u32(self) -> u32    { self as u32 }
    fn into_u16(self) -> u16    { self as u16 }
    fn into_u8(self) -> u8      { self as u8 }
    fn into_usize(self) -> usize { self as usize }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_i64(self) -> i64    { self as i64 }
    fn into_i32(self) -> i32    { self as i32 }
    fn into_i16(self) -> i16    { self as i16 }
    fn into_i8(self) -> i8      { self as i8 }
    fn into_isize(self) -> isize { self as isize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { Self::MAX }
    fn Min() -> Self            { Self::MIN }
    fn num(n: i128) -> Self     { n as Self }
}

impl Numeric for u128
{
    fn wrapping_add(self, rhs: Self) -> Self    { self.wrapping_add(rhs) }
    fn wrapping_sub(self, rhs: Self) -> Self    { self.wrapping_sub(rhs) }
    fn wrapping_mul(self, rhs: Self) -> Self    { self.wrapping_mul(rhs) }
    fn wrapping_div(self, rhs: Self) -> Self    { self.wrapping_div(rhs) }
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_u64(self) -> u64    { self as u64 }
    fn into_u32(self) -> u32    { self as u32 }
    fn into_u16(self) -> u16    { self as u16 }
    fn into_u8(self) -> u8      { self as u8 }
    fn into_usize(self) -> usize { self as usize }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_i64(self) -> i64    { self as i64 }
    fn into_i32(self) -> i32    { self as i32 }
    fn into_i16(self) -> i16    { self as i16 }
    fn into_i8(self) -> i8      { self as i8 }
    fn into_isize(self) -> isize { self as isize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { Self::MAX }
    fn Min() -> Self            { Self::MIN }
    fn num(n: i128) -> Self     { n as Self }
}

impl Numeric for usize
{
    fn wrapping_add(self, rhs: Self) -> Self    { self.wrapping_add(rhs) }
    fn wrapping_sub(self, rhs: Self) -> Self    { self.wrapping_sub(rhs) }
    fn wrapping_mul(self, rhs: Self) -> Self    { self.wrapping_mul(rhs) }
    fn wrapping_div(self, rhs: Self) -> Self    { self.wrapping_div(rhs) }
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_u64(self) -> u64    { self as u64 }
    fn into_u32(self) -> u32    { self as u32 }
    fn into_u16(self) -> u16    { self as u16 }
    fn into_u8(self) -> u8      { self as u8 }
    fn into_usize(self) -> usize { self as usize }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_i64(self) -> i64    { self as i64 }
    fn into_i32(self) -> i32    { self as i32 }
    fn into_i16(self) -> i16    { self as i16 }
    fn into_i8(self) -> i8      { self as i8 }
    fn into_isize(self) -> isize { self as isize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { Self::MAX }
    fn Min() -> Self            { Self::MIN }
    fn num(n: i128) -> Self     { n as Self }
}

impl Numeric for i8
{
    fn wrapping_add(self, rhs: Self) -> Self    { self.wrapping_add(rhs) }
    fn wrapping_sub(self, rhs: Self) -> Self    { self.wrapping_sub(rhs) }
    fn wrapping_mul(self, rhs: Self) -> Self    { self.wrapping_mul(rhs) }
    fn wrapping_div(self, rhs: Self) -> Self    { self.wrapping_div(rhs) }
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_u64(self) -> u64    { self as u64 }
    fn into_u32(self) -> u32    { self as u32 }
    fn into_u16(self) -> u16    { self as u16 }
    fn into_u8(self) -> u8      { self as u8 }
    fn into_usize(self) -> usize { self as usize }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_i64(self) -> i64    { self as i64 }
    fn into_i32(self) -> i32    { self as i32 }
    fn into_i16(self) -> i16    { self as i16 }
    fn into_i8(self) -> i8      { self as i8 }
    fn into_isize(self) -> isize { self as isize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { Self::MAX }
    fn Min() -> Self            { Self::MIN }
    fn num(n: i128) -> Self     { n as Self }
}

impl Numeric for i16
{
    fn wrapping_add(self, rhs: Self) -> Self    { self.wrapping_add(rhs) }
    fn wrapping_sub(self, rhs: Self) -> Self    { self.wrapping_sub(rhs) }
    fn wrapping_mul(self, rhs: Self) -> Self    { self.wrapping_mul(rhs) }
    fn wrapping_div(self, rhs: Self) -> Self    { self.wrapping_div(rhs) }
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_u64(self) -> u64    { self as u64 }
    fn into_u32(self) -> u32    { self as u32 }
    fn into_u16(self) -> u16    { self as u16 }
    fn into_u8(self) -> u8      { self as u8 }
    fn into_usize(self) -> usize { self as usize }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_i64(self) -> i64    { self as i64 }
    fn into_i32(self) -> i32    { self as i32 }
    fn into_i16(self) -> i16    { self as i16 }
    fn into_i8(self) -> i8      { self as i8 }
    fn into_isize(self) -> isize { self as isize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { Self::MAX }
    fn Min() -> Self            { Self::MIN }
    fn num(n: i128) -> Self     { n as Self }
}

impl Numeric for i32
{
    fn wrapping_add(self, rhs: Self) -> Self    { self.wrapping_add(rhs) }
    fn wrapping_sub(self, rhs: Self) -> Self    { self.wrapping_sub(rhs) }
    fn wrapping_mul(self, rhs: Self) -> Self    { self.wrapping_mul(rhs) }
    fn wrapping_div(self, rhs: Self) -> Self    { self.wrapping_div(rhs) }
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_u64(self) -> u64    { self as u64 }
    fn into_u32(self) -> u32    { self as u32 }
    fn into_u16(self) -> u16    { self as u16 }
    fn into_u8(self) -> u8      { self as u8 }
    fn into_usize(self) -> usize { self as usize }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_i64(self) -> i64    { self as i64 }
    fn into_i32(self) -> i32    { self as i32 }
    fn into_i16(self) -> i16    { self as i16 }
    fn into_i8(self) -> i8      { self as i8 }
    fn into_isize(self) -> isize { self as isize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { Self::MAX }
    fn Min() -> Self            { Self::MIN }
    fn num(n: i128) -> Self     { n as Self }
}

impl Numeric for i64
{
    fn wrapping_add(self, rhs: Self) -> Self    { self.wrapping_add(rhs) }
    fn wrapping_sub(self, rhs: Self) -> Self    { self.wrapping_sub(rhs) }
    fn wrapping_mul(self, rhs: Self) -> Self    { self.wrapping_mul(rhs) }
    fn wrapping_div(self, rhs: Self) -> Self    { self.wrapping_div(rhs) }
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_u64(self) -> u64    { self as u64 }
    fn into_u32(self) -> u32    { self as u32 }
    fn into_u16(self) -> u16    { self as u16 }
    fn into_u8(self) -> u8      { self as u8 }
    fn into_usize(self) -> usize { self as usize }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_i64(self) -> i64    { self as i64 }
    fn into_i32(self) -> i32    { self as i32 }
    fn into_i16(self) -> i16    { self as i16 }
    fn into_i8(self) -> i8      { self as i8 }
    fn into_isize(self) -> isize { self as isize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { Self::MAX }
    fn Min() -> Self            { Self::MIN }
    fn num(n: i128) -> Self     { n as Self }
}

impl Numeric for i128
{
    fn wrapping_add(self, rhs: Self) -> Self    { self.wrapping_add(rhs) }
    fn wrapping_sub(self, rhs: Self) -> Self    { self.wrapping_sub(rhs) }
    fn wrapping_mul(self, rhs: Self) -> Self    { self.wrapping_mul(rhs) }
    fn wrapping_div(self, rhs: Self) -> Self    { self.wrapping_div(rhs) }
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_u64(self) -> u64    { self as u64 }
    fn into_u32(self) -> u32    { self as u32 }
    fn into_u16(self) -> u16    { self as u16 }
    fn into_u8(self) -> u8      { self as u8 }
    fn into_usize(self) -> usize { self as usize }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_i64(self) -> i64    { self as i64 }
    fn into_i32(self) -> i32    { self as i32 }
    fn into_i16(self) -> i16    { self as i16 }
    fn into_i8(self) -> i8      { self as i8 }
    fn into_isize(self) -> isize { self as isize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { Self::MAX }
    fn Min() -> Self            { Self::MIN }
    fn num(n: i128) -> Self     { n as Self }
}

impl Numeric for isize
{
    fn wrapping_add(self, rhs: Self) -> Self    { self.wrapping_add(rhs) }
    fn wrapping_sub(self, rhs: Self) -> Self    { self.wrapping_sub(rhs) }
    fn wrapping_mul(self, rhs: Self) -> Self    { self.wrapping_mul(rhs) }
    fn wrapping_div(self, rhs: Self) -> Self    { self.wrapping_div(rhs) }
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_u64(self) -> u64    { self as u64 }
    fn into_u32(self) -> u32    { self as u32 }
    fn into_u16(self) -> u16    { self as u16 }
    fn into_u8(self) -> u8      { self as u8 }
    fn into_usize(self) -> usize { self as usize }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_i64(self) -> i64    { self as i64 }
    fn into_i32(self) -> i32    { self as i32 }
    fn into_i16(self) -> i16    { self as i16 }
    fn into_i8(self) -> i8      { self as i8 }
    fn into_isize(self) -> isize { self as isize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { Self::MAX }
    fn Min() -> Self            { Self::MIN }
    fn num(n: i128) -> Self     { n as Self }
}
