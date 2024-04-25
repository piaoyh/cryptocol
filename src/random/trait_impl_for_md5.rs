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
use crate::hash::MD5_Generic;
use crate::random::Random_Engine;

impl<const H0: u32, const H1: u32, const H2: u32, const H3: u32, const ROUND: usize, 
    const K00: u32, const K01: u32, const K02: u32, const K03: u32,
    const K04: u32, const K05: u32, const K06: u32, const K07: u32,
    const K08: u32, const K09: u32, const K10: u32, const K11: u32,
    const K12: u32, const K13: u32, const K14: u32, const K15: u32,
    const K16: u32, const K17: u32, const K18: u32, const K19: u32,
    const K20: u32, const K21: u32, const K22: u32, const K23: u32,
    const K24: u32, const K25: u32, const K26: u32, const K27: u32,
    const K28: u32, const K29: u32, const K30: u32, const K31: u32,
    const K32: u32, const K33: u32, const K34: u32, const K35: u32,
    const K36: u32, const K37: u32, const K38: u32, const K39: u32,
    const K40: u32, const K41: u32, const K42: u32, const K43: u32,
    const K44: u32, const K45: u32, const K46: u32, const K47: u32,
    const K48: u32, const K49: u32, const K50: u32, const K51: u32,
    const K52: u32, const K53: u32, const K54: u32, const K55: u32,
    const K56: u32, const K57: u32, const K58: u32, const K59: u32,
    const K60: u32, const K61: u32, const K62: u32, const K63: u32,
    const R00: u32, const R01: u32, const R02: u32, const R03: u32,
    const R10: u32, const R11: u32, const R12: u32, const R13: u32,
    const R20: u32, const R21: u32, const R22: u32, const R23: u32,
    const R30: u32, const R31: u32, const R32: u32, const R33: u32>
Random_Engine for MD5_Generic<4, H0, H1, H2, H3, ROUND,
                    K00, K01, K02, K03, K04, K05, K06, K07,
                    K08, K09, K10, K11, K12, K13, K14, K15,
                    K16, K17, K18, K19, K20, K21, K22, K23,
                    K24, K25, K26, K27, K28, K29, K30, K31,
                    K32, K33, K34, K35, K36, K37, K38, K39,
                    K40, K41, K42, K43, K44, K45, K46, K47,
                    K48, K49, K50, K51, K52, K53, K54, K55,
                    K56, K57, K58, K59, K60, K61, K62, K63,
                    R00, R01, R02, R03, R10, R11, R12, R13,
                    R20, R21, R22, R23, R30, R31, R32, R33>
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

    fn sow_array<T, const M: usize>(&mut self, message: &[T; M])
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
        let a: [u32; 4] = self.get_hash_value_in_array();
        self.tangle(tangling);
        let b: [u32; 4] = self.get_hash_value_in_array();
        self.tangle(tangling);
        let c: [u32; 4] = self.get_hash_value_in_array();
        self.tangle(tangling);
        let d: [u32; 4] = self.get_hash_value_in_array();
        let mut res = [0_u64; 8];
        for i in 0..4
            { res[i] = ((a[i] as u64) << 32) | (b[i] as u64); }
        for i in 0..4
            { res[i+4] = ((c[i] as u64) << 32) | (d[i] as u64); }
        res
    }
}
