// Copyright 2023 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

use std::fmt::Debug;
use std::ops::*;
use std::cmp::{Eq, Ord};

/// Trait Float is for generic type of primitive data types
/// for all modules of the crate Cryptocol.
/// 
/// Here, the generic type of primitive data types includes: f32 and f64. You
/// will hardly use the trait Float unless you improve the crate Cryptocol or
/// create addional libraries that works with the crate Cryptocol. But, if
/// you only use the crate Cryptocol, you can forget about this trait Float.
/// 
pub trait Float: Copy + Debug
            + Add + AddAssign + Sub + SubAssign + Mul + MulAssign + Div + DivAssign
            + Shl + ShlAssign + Shr + ShrAssign
            + Eq + Ord
{
    fn into_f64(self) -> f64;
    fn into_f32(self) -> f32;
    fn into_bool(self) -> bool;
    fn zero() -> Self;
    fn Max() -> Self;
    fn Min() -> Self;
    fn num(n: f64) -> Self;
}

impl Float for f32
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_bool(self) -> bool  { self != 0.0 }
    fn zero() -> Self           { 0.0 }
    fn Max() -> Self            { Self::MAX }
    fn Min() -> Self            { Self::MIN }
    fn num(n: f64) -> Self      { n as Self }
}

impl Float for f64
{
    fn into_f64(self) -> f64    { self as f64 }
    fn into_f32(self) -> f32    { self as f32 }
    fn into_bool(self) -> bool  { self != 0.0 }
    fn zero() -> Self           { 0.0 }
    fn Max() -> Self            { Self::MAX }
    fn Min() -> Self            { Self::MIN }
    fn num(n: f64) -> Self      { n as Self }
}
