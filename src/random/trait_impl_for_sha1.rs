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

use crate::number::small_uint::*;
use crate::hash::SHA1_Generic;
use super::Random_Engine;


impl<const H0: u32, const H1: u32, const H2: u32, const H3: u32,
    const H4: u32, const ROUND: usize, const K0: u32, const K1: u32,
    const K2: u32, const K3: u32, const RL1: u32, const RL5: u32, const RL30: u32>
Random_Engine for SHA1_Generic<5, H0, H1, H2, H3, H4, ROUND, K0, K1, K2, K3, RL1, RL5, RL30>
{
    fn new() -> Self
    {
        Self::new()
    }

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
        res.sow_array(message);
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
        self.digest_array(message);
    }

    fn harvest(&mut self, tangling: u64) -> [u64; 8]
    {
        self.tangle(tangling);
        let a: [u32; 5] = self.get_hash_value_in_array();
        self.tangle(tangling);
        let b: [u32; 5] = self.get_hash_value_in_array();
        self.tangle(tangling);
        let c: [u32; 5] = self.get_hash_value_in_array();
        self.tangle(tangling);
        let d: [u32; 5] = self.get_hash_value_in_array();
        let mut res = [0_u64; 8];
        for i in 0..5
            { res[i] = ((a[i] as u64) << 32) | (b[i] as u64); }
        for i in 0..3
            { res[i+5] = ((c[i] as u64) << 32) | (d[i] as u64); }
        res
    }
}

