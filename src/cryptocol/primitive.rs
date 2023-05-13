use std::fmt::Debug;
use std::mem::size_of;
use std::f64::consts;
use std::ops::*;
use std::cmp::{Eq, Ord};

pub trait Real: Copy + Debug + Add + AddAssign + Sub + SubAssign + Mul + MulAssign + Div + DivAssign
{
    fn into_f64(self) -> f64;
    fn into_f32(self) -> f32;
    fn into_bool(self) -> bool;
    fn zero() -> Self;
    fn num(n: f64) -> Self;
}

pub trait Numeric: Copy + Eq + Ord + Debug + Add + AddAssign + Sub + SubAssign + Mul + MulAssign + Div + DivAssign
{
    fn wrapping_add(self, rhs: Self) -> Self;
    fn wrapping_sub(self, rhs: Self) -> Self;
    fn wrapping_mul(self, rhs: Self) -> Self;
    fn wrapping_div(self, rhs: Self) -> Self;
    fn into_f64(self) -> f64;
    fn into_f32(self) -> f32;
    fn into_u128(self) -> u128;
    fn into_i128(self) -> i128;
    fn into_usize(self) -> usize;
    fn into_bool(self) -> bool;
    fn zero() -> Self;
    fn one() -> Self;
    fn Max() -> Self;
    fn Min() -> Self;
    fn num(n: i128) -> Self;
}

pub trait Uint: Copy + Eq + Ord + Debug
                 + Add + AddAssign + Sub + SubAssign + Mul + MulAssign + Div + DivAssign
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
    fn into_usize(self) -> usize;
    fn into_bool(self) -> bool;
    fn zero() -> Self;
    fn one() -> Self;
    fn Max() -> Self;
    fn Min() -> Self;
    fn num(n: u128) -> Self;
}

pub trait Int: Copy + Eq + Ord + Debug + Add + AddAssign + Sub + SubAssign + Mul + MulAssign + Div + DivAssign
{
    fn wrapping_add(self, rhs: Self) -> Self;
    fn wrapping_sub(self, rhs: Self) -> Self;
    fn wrapping_mul(self, rhs: Self) -> Self;
    fn wrapping_div(self, rhs: Self) -> Self;
    fn into_f64(self) -> f64;
    fn into_f32(self) -> f32;
    fn into_i128(self) -> i128;
    fn into_bool(self) -> bool;
    fn zero() -> Self;
    fn one() -> Self;
    fn Max() -> Self;
    fn Min() -> Self;
    fn num(n: i128) -> Self;
}
pub trait Float: Copy + Debug + Add + AddAssign + Sub + SubAssign + Mul + MulAssign + Div + DivAssign
{
    fn into_f64(self) -> f64;
    fn into_f32(self) -> f32;
    fn into_bool(self) -> bool;
    fn zero() -> Self;
    fn Max() -> Self;
    fn Min() -> Self;
}

/*
pub trait Bool: Copy + Eq
{
    fn into_f64(self) -> f64;
    fn into_f32(self) -> f32;
    fn into_u128(self) -> u128;
    fn into_i128(self) -> i128;
    fn into_usize(self) -> usize;
    fn zero() -> Self;
    fn one() -> Self;
    fn Max() -> Self;
    fn Min() -> Self;
}
*/

impl Uint for u8
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
    fn into_usize(self) -> usize { self as usize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { Self::MAX }
    fn Min() -> Self            { Self::MIN }
    fn num(n: u128) -> Self     { n as Self }
}

impl Uint for u16
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
    fn into_usize(self) -> usize { self as usize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { Self::MAX }
    fn Min() -> Self            { Self::MIN }
    fn num(n: u128) -> Self     { n as Self }
}

impl Uint for u32
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
    fn into_usize(self) -> usize { self as usize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { Self::MAX }
    fn Min() -> Self            { Self::MIN }
    fn num(n: u128) -> Self     { n as Self }
}

impl Uint for u64
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
    fn into_usize(self) -> usize { self as usize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { Self::MAX }
    fn Min() -> Self            { Self::MIN }
    fn num(n: u128) -> Self     { n as Self }
}

impl Uint for u128
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
    fn into_usize(self) -> usize { self as usize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { Self::MAX }
    fn Min() -> Self            { Self::MIN }
    fn num(n: u128) -> Self     { n as Self }
}

impl Uint for usize
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
    fn into_usize(self) -> usize { self as usize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { Self::MAX }
    fn Min() -> Self            { Self::MIN }
    fn num(n: u128) -> Self     { n as Self }
}

impl Int for i8
{
    fn wrapping_add(self, rhs: Self) -> Self    { self.wrapping_add(rhs) }
    fn wrapping_sub(self, rhs: Self) -> Self    { self.wrapping_sub(rhs) }
    fn wrapping_mul(self, rhs: Self) -> Self    { self.wrapping_mul(rhs) }
    fn wrapping_div(self, rhs: Self) -> Self    { self.wrapping_div(rhs) }
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { Self::MAX }
    fn Min() -> Self            { Self::MIN }
    fn num(n: i128) -> Self     { n as Self }
}

impl Int for i16
{
    fn wrapping_add(self, rhs: Self) -> Self    { self.wrapping_add(rhs) }
    fn wrapping_sub(self, rhs: Self) -> Self    { self.wrapping_sub(rhs) }
    fn wrapping_mul(self, rhs: Self) -> Self    { self.wrapping_mul(rhs) }
    fn wrapping_div(self, rhs: Self) -> Self    { self.wrapping_div(rhs) }
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { Self::MAX }
    fn Min() -> Self            { Self::MIN }
    fn num(n: i128) -> Self     { n as Self }
}

impl Int for i32
{
    fn wrapping_add(self, rhs: Self) -> Self    { self.wrapping_add(rhs) }
    fn wrapping_sub(self, rhs: Self) -> Self    { self.wrapping_sub(rhs) }
    fn wrapping_mul(self, rhs: Self) -> Self    { self.wrapping_mul(rhs) }
    fn wrapping_div(self, rhs: Self) -> Self    { self.wrapping_div(rhs) }
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { Self::MAX }
    fn Min() -> Self            { Self::MIN }
    fn num(n: i128) -> Self     { n as Self }
}

impl Int for i64
{
    fn wrapping_add(self, rhs: Self) -> Self    { self.wrapping_add(rhs) }
    fn wrapping_sub(self, rhs: Self) -> Self    { self.wrapping_sub(rhs) }
    fn wrapping_mul(self, rhs: Self) -> Self    { self.wrapping_mul(rhs) }
    fn wrapping_div(self, rhs: Self) -> Self    { self.wrapping_div(rhs) }
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { Self::MAX }
    fn Min() -> Self            { Self::MIN }
    fn num(n: i128) -> Self     { n as Self }
}

impl Int for i128
{
    fn wrapping_add(self, rhs: Self) -> Self    { self.wrapping_add(rhs) }
    fn wrapping_sub(self, rhs: Self) -> Self    { self.wrapping_sub(rhs) }
    fn wrapping_mul(self, rhs: Self) -> Self    { self.wrapping_mul(rhs) }
    fn wrapping_div(self, rhs: Self) -> Self    { self.wrapping_div(rhs) }
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { Self::MAX }
    fn Min() -> Self            { Self::MIN }
    fn num(n: i128) -> Self     { n as Self }
}

impl Float for f32
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_bool(self) -> bool  { self != 0.0 }
    fn zero() -> Self           { 0.0 }
    fn Max() -> Self            { Self::MAX }
    fn Min() -> Self            { Self::MIN }
}

impl Float for f64
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_bool(self) -> bool  { self != 0.0 }
    fn zero() -> Self           { 0.0 }
    fn Max() -> Self            { Self::MAX }
    fn Min() -> Self            { Self::MIN }
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
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
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
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
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
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
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
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
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
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
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
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
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
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
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
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
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
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
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
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { Self::MAX }
    fn Min() -> Self            { Self::MIN }
    fn num(n: i128) -> Self     { n as Self }
}

impl Real for u8
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn num(n: f64) -> Self      { n as u8 }
}

impl Real for u16
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn num(n: f64) -> Self      { n as u16 }
}

impl Real for u32
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn num(n: f64) -> Self      { n as u32 }
}

impl Real for u64
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn num(n: f64) -> Self      { n as u64 }
}

impl Real for u128
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn num(n: f64) -> Self      { n as u128 }
}

impl Real for i8
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn num(n: f64) -> Self      { n as i8 }
}

impl Real for i16
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn num(n: f64) -> Self      { n as i16 }
}

impl Real for i32
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn num(n: f64) -> Self      { n as i32 }
}

impl Real for i64
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn num(n: f64) -> Self      { n as i64 }
}

impl Real for i128
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_bool(self) -> bool  { self != 0 }
    fn zero() -> Self           { 0 }
    fn num(n: f64) -> Self      { n as i128 }
}

impl Real for f32
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_bool(self) -> bool  { self != 0.0 }
    fn zero() -> Self           { 0.0 }
    fn num(n: f64) -> Self      { n as f32 }
}

impl Real for f64
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_bool(self) -> bool  { self != 0.0 }
    fn zero() -> Self           { 0.0 }
    fn num(n: f64) -> Self      { n as f64 }
}

/*
impl Bool for u8
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { 1 }
    fn Min() -> Self            { 0 }
}

impl Bool for u16
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { 1 }
    fn Min() -> Self            { 0 }
}

impl Bool for u32
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { 1 }
    fn Min() -> Self            { 0 }
}

impl Bool for u64
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { 1 }
    fn Min() -> Self            { 0 }
}

impl Bool for u128
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { 1 }
    fn Min() -> Self            { 0 }
}

impl Bool for i8
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { 1 }
    fn Min() -> Self            { 0 }
}

impl Bool for i16
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { 1 }
    fn Min() -> Self            { 0 }
}

impl Bool for i32
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { 1 }
    fn Min() -> Self            { 0 }
}

impl Bool for i64
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { 1 }
    fn Min() -> Self            { 0 }
}

impl Bool for i128
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_u128(self) -> u128  { self as u128 }
    fn into_i128(self) -> i128  { self as i128 }
    fn into_usize(self) -> usize { self as usize }
    fn zero() -> Self           { 0 }
    fn one() -> Self            { 1 }
    fn Max() -> Self            { 1 }
    fn Min() -> Self            { 0 }
}
*/

#[derive(Copy, Clone)]
pub union UShort
{
    pub ushort: u16,
    pub byte: [u8; 2],
}

#[derive(Copy, Clone)]
pub union UInt
{
    pub uint: u32,
    pub ushort: [u16; 2],
    pub byte: [u8; 4],
}

#[derive(Copy, Clone)]
pub union ULong
{
    pub ulong: u64,
    pub uint: [u32; 2],
    pub ushort: [u16; 4],
    pub byte: [u8; 8],
}

#[derive(Copy, Clone)]
pub union ULonger
{
    pub ulonger: u128,
    pub ulong: [u64; 2],
    pub uint: [u32; 4],
    pub ushort: [u16; 8],
    pub byte: [u8; 16],
}

#[cfg(target_pointer_width = "64")]
#[derive(Copy, Clone)]
pub union USize
{
    pub size: usize,
    pub ulong: u64,
    pub uint: [u32; 2],
    pub ushort: [u16; 4],
    pub byte: [u8; 8],
}

#[cfg(target_pointer_width = "32")]
#[derive(Copy, Clone)]
pub union USize
{
    pub size: usize,
    pub uint: u32,
    pub ushort: [u16; 2],
    pub byte: [u8; 4],
}

impl UShort
{
    pub fn new() -> Self    { Self { ushort: 0 } }
    pub fn new_with(ushort: u16) -> Self   { Self { ushort } }
}

impl UInt
{
    pub fn new() -> Self    { Self { uint: 0 } }
    pub fn new_with(uint: u32) -> Self   { Self { uint } }
}

impl ULong
{
    pub fn new() -> Self    { Self { ulong: 0 } }
    pub fn new_with(ulong: u64) -> Self   { Self { ulong } }
}

impl ULonger
{
    pub fn new() -> Self    { Self { ulonger: 0 } }
    pub fn new_with(ulonger: u128) -> Self   { Self { ulonger } }
}

impl USize
{
    pub fn new() -> Self    { Self { size: 0 } }
    pub fn new_with(size: usize) -> Self   { Self { size } }
}
/*
impl Shr<usize> for USize
{
    type Output = Self;
    fn shr(self, rhs: usize) -> Self   { let mut s = self; s.size >>= rhs; s}
}

impl ShrAssign<usize> for USize
{
    fn shr_assign(&mut self, rhs: usize)    { self.size >>= rhs; }
}
*/

pub trait Large_Integer<T, const N: usize>
where T: Uint,
    Self: Sized + Clone + AddAssign + SubAssign + MulAssign + DivAssign + RemAssign
{
    const OVERFLOW: u8  = 0b0000_0001;
    const UNDERFLOW: u8 = 0b0000_0010;
    const INFINITY: u8  = 0b0000_0100;
    const DIVIDED_BY_ZERO: u8 = Self::INFINITY;

    fn new() -> Self;
    fn from_array(val: &[T; N]) -> Self;
    fn from_uint<V: Copy>(val: V) -> Self;
    fn from_string_with_radix(txt: &str, radix: usize) -> Option<Self>;
    fn to_string_with_radix(&self, radix: usize) -> String;
    fn add_uint(&self, rhs: T) -> Self;
    fn sub_uint(&self, rhs: T) -> Self;
    fn mul_uint(&self, rhs: T) -> Self;
    fn div_uint(&self, rhs: T) -> Self;
    fn rem_uint(&self, rhs: T) -> Self;
    fn get_num(&self, i: usize) -> T;
    fn set_num(&mut self, i: usize, val: T);
    fn set_zero(&mut self);
    fn set_max(&mut self);
    fn set_flag_bit(&mut self, flag_bits: u8);
    fn reset_flag_bit(&mut self, flag_bits: u8);
    fn is_flag_bit_on(&self, flag_bits: u8) -> bool;

    fn from_string(txt: &str) -> Option<Self>
    {
        Self::from_string_with_radix(txt, 10)
    }

    fn accumulate(&mut self, rhs: T)
    {
        let bi = Self::from_uint(rhs);
        *self += bi;
    }

    fn dissipate(&mut self, rhs: T)
    {
        let bi = Self::from_uint(rhs);
        *self -= bi;
    }

    fn times(&mut self, rhs: T)
    {
        let bi = Self::from_uint(rhs);
        *self *= bi;
    }

    fn quotient(&mut self, rhs: T)
    {
        let bi = Self::from_uint(rhs);
        *self /= bi;
    }

    fn remainder(&mut self, rhs: T)
    {
        let bi = Self::from_uint(rhs);
        *self %= bi;
    }

    fn to_string(&self) -> String   { self.to_string_with_radix(10) }
    fn set_overflow(&mut self)      { self.set_flag_bit(Self::OVERFLOW); }
    fn reset_overflow(&mut self)    { self.reset_flag_bit(Self::OVERFLOW); }
    fn is_overflow(&self) -> bool   { self.is_flag_bit_on(Self::OVERFLOW) }
    fn set_underflow(&mut self)     { self.set_flag_bit(Self::UNDERFLOW) }
    fn reset_underflow(&mut self)   { self.reset_flag_bit(Self::UNDERFLOW) }
    fn is_underflow(&self) -> bool  { self.is_flag_bit_on(Self::UNDERFLOW) }
    fn set_inifinity(&mut self)     { self.set_flag_bit(Self::INFINITY); }
    fn reset_inifinity(&mut self)   { self.reset_flag_bit(Self::INFINITY); }
    fn is_inifinity(&self) -> bool  { self.is_flag_bit_on(Self::INFINITY) }
    fn set_untrustable(&mut self)   { self.set_flag_bit(Self::OVERFLOW | Self::UNDERFLOW); }
    fn reset_untrustable(&mut self) { self.reset_flag_bit(Self::OVERFLOW | Self::UNDERFLOW); }
    fn is_untrustable(&self) -> bool { self.is_flag_bit_on(Self::OVERFLOW | Self::UNDERFLOW)}
    fn set_divided_by_zero(&mut self)   { self.set_inifinity(); }
    fn reset_divided_by_zero(&mut self) { self.reset_inifinity(); }
    fn is_divided_by_zero(&self) -> bool { self.is_inifinity() }
}