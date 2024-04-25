// Copyright 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


use std::fmt::{ Debug, Display };
use std::ops::*;
use std::cmp::{ PartialEq, PartialOrd};

use crate::number::SmallUInt;

/// The supporting trait for `Random_Generic`
/// `Random_Generic` uses whatever object that has this trait for pseudo-random
/// number generator engine. So, if you plug in any hash algorithm that has
/// this trait to `Random_Generic`, `Random_Generic` will use the object
/// as its pseudo-random number generator engine.
/// You will hardly use the object that has this trait except the case
/// that you use it in order to plug it in the `Random_Generic`. 
#[allow(non_camel_case_types)]
pub trait Random_Engine
{
    // fn new() -> Self;
    /// Constructs the object.
    /// 
    /// # Example
    /// Refer to the souce codes of `Random` to see how to use this method.
    fn new() -> Self;

    // fn new_with<T, const N: usize>(message: &[T; N]) -> Self
    /// Constructs the object with the given `message`.
    /// 
    /// # Features
    /// How to use the argument `message` depends on the object.
    /// 
    /// # Example
    /// Refer to the souce codes of `Random` to see how to use this method.
    fn new_with<T, const N: usize>(message: &[T; N]) -> Self
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd;

    // fn sow_array<T, const N: usize>(&mut self, message: &[T; N])
    /// Provides new seeds for `self`.
    /// 
    /// # Argument
    /// `message` is the new seeds for `self`.
    /// 
    /// # Example
    /// Refer to the souce codes of `Random` to see how to use this method.
    fn sow_array<T, const N: usize>(&mut self, message: &[T; N])
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd;

    // fn harvest(&mut self, sugar: u64) -> [u64; 8]
    /// Outputs the pseudo-random number array.
    /// 
    /// # Argument
    /// `sugar` is `u64`-typed unsigned integer that changes the direction of
    /// its pseudo-random number sequence so that the period of the
    /// pseudo-random number sequence may not repeated.
    /// 
    /// # Example
    /// Refer to the souce codes of `Random` to see how to use this method.
    fn harvest(&mut self, sugar: u64) -> [u64; 8];
}
