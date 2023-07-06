// Copyright 2023 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module that contains generic type of primitive number data types
//! for the counter part of uint and int.

#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

use std::fmt::Debug;
use std::ops::*;
use std::cmp::{Eq, Ord};

/// Trait Real is for generic type of primitive number data types
/// for the counter part of uint and int.
/// 
/// Trait Real is trait Float + trait Numeric. Here, the generic type of
/// primitive number data types includes: u8, u16, u32, u64, u128, usize, i8,
/// i16, i32, i64, i128, isize, f32 and f64. You will hardly use the trait Real
/// unless you improve the crate Cryptocol or create addional libraries that
/// works with the crate Cryptocol. But, if you only use the crate Cryptocol,
/// you can forget about this trait Real.
/// 
pub trait Real: Copy + Debug
            + Add + AddAssign + Sub + SubAssign + Mul + MulAssign + Div + DivAssign
{
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
    fn num(n: f64) -> Self;
}

impl Real for u8
{
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
    fn num(n: f64) -> Self      { n as Self }
}

impl Real for u16
{
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
    fn num(n: f64) -> Self      { n as Self }
}

impl Real for u32
{
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
    fn num(n: f64) -> Self      { n as Self }
}

impl Real for u64
{
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
    fn num(n: f64) -> Self      { n as Self }
}

impl Real for u128
{
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
    fn num(n: f64) -> Self      { n as Self }
}

impl Real for usize
{
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
    fn num(n: f64) -> Self      { n as Self }
}

impl Real for i8
{
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
    fn num(n: f64) -> Self      { n as Self }
}

impl Real for i16
{
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
    fn num(n: f64) -> Self      { n as Self }
}

impl Real for i32
{
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
    fn num(n: f64) -> Self      { n as Self }
}

impl Real for i64
{
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
    fn num(n: f64) -> Self      { n as Self }
}

impl Real for i128
{
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
    fn num(n: f64) -> Self      { n as Self }
}

impl Real for isize
{
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
    fn num(n: f64) -> Self      { n as Self }
}

impl Real for f32
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_bool(self) -> bool  { self != 0.0 }
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
    fn zero() -> Self           { 0.0 }
    fn num(n: f64) -> Self      { n as Self }
}

impl Real for f64
{
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
    fn into_bool(self) -> bool  { self != 0.0 }
    fn zero() -> Self           { 0.0 }
    fn num(n: f64) -> Self      { n as Self }
}
