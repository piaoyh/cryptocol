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
use std::ptr::copy_nonoverlapping;

use crate::number::small_uint::*;
use super::Random_Engine;
use super::AnyNumber;

impl Random_Engine for AnyNumber
{
    #[inline]
    fn new() -> Self
    {
        Self::new()
    }

    #[inline]
    fn new_with<T, const N: usize>(message: &[T; N]) -> Self
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
    {
        let mut res = Self::new();
        if N == 8
        {
            for i in 0..8
                { res.set_any_number_(i, message[i].into_u64()); }
        }
        else
        {
            let len = if 8 * 8 < T::size_in_bytes() * N {8 *8} else {T::size_in_bytes() * N};
            unsafe { copy_nonoverlapping(message.as_ptr() as *const u8, res.as_mut_ptr() as *mut u8, len); }
        }
        res
    }

    fn sow_array<T, const N: usize>(&mut self, message: &[T; N])
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
    {
        let n = if 8 <= N {8} else {N};
        for i in 0..n
        {
            self.set_any_number_(i, self.get_any_number_(i).wrapping_mul(1103515245));
            self.set_any_number_(i, self.get_any_number_(i).wrapping_add(12345));
        }
    }

    fn harvest(&mut self, tangling: u64) -> [u64; 8]
    {
        self.tangle(tangling);
        self.get_any_numbers()
    }
}
