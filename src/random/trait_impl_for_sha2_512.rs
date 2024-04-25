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
use crate::hash::SHA2_512_Generic;
use crate::random::Random_Engine;


impl<const H0: u64, const H1: u64, const H2: u64, const H3: u64,
    const H4: u64, const H5: u64, const H6: u64, const H7: u64, const ROUND: usize,
    const K00: u64, const K01: u64, const K02: u64, const K03: u64,
    const K04: u64, const K05: u64, const K06: u64, const K07: u64,
    const K08: u64, const K09: u64, const K10: u64, const K11: u64,
    const K12: u64, const K13: u64, const K14: u64, const K15: u64,
    const K16: u64, const K17: u64, const K18: u64, const K19: u64,
    const K20: u64, const K21: u64, const K22: u64, const K23: u64,
    const K24: u64, const K25: u64, const K26: u64, const K27: u64,
    const K28: u64, const K29: u64, const K30: u64, const K31: u64,
    const K32: u64, const K33: u64, const K34: u64, const K35: u64,
    const K36: u64, const K37: u64, const K38: u64, const K39: u64,
    const K40: u64, const K41: u64, const K42: u64, const K43: u64,
    const K44: u64, const K45: u64, const K46: u64, const K47: u64,
    const K48: u64, const K49: u64, const K50: u64, const K51: u64,
    const K52: u64, const K53: u64, const K54: u64, const K55: u64,
    const K56: u64, const K57: u64, const K58: u64, const K59: u64,
    const K60: u64, const K61: u64, const K62: u64, const K63: u64,
    const K64: u64, const K65: u64, const K66: u64, const K67: u64,
    const K68: u64, const K69: u64, const K70: u64, const K71: u64,
    const K72: u64, const K73: u64, const K74: u64, const K75: u64,
    const K76: u64, const K77: u64, const K78: u64, const K79: u64,
    const RR1: u32, const RR8: u32, const RR14: u32, const RR18: u32, 
    const RR19: u32, const RR28: u32, const RR34: u32, const RR39: u32, 
    const RR41: u32, const RR61: u32, const SR6: i32, const SR7: i32>
Random_Engine for SHA2_512_Generic<8, H0, H1, H2, H3, H4, H5, H6, H7, ROUND,
                            K00, K01, K02, K03, K04, K05, K06, K07,
                            K08, K09, K10, K11, K12, K13, K14, K15,
                            K16, K17, K18, K19, K20, K21, K22, K23,
                            K24, K25, K26, K27, K28, K29, K30, K31,
                            K32, K33, K34, K35, K36, K37, K38, K39,
                            K40, K41, K42, K43, K44, K45, K46, K47,
                            K48, K49, K50, K51, K52, K53, K54, K55,
                            K56, K57, K58, K59, K60, K61, K62, K63,
                            K64, K65, K66, K67, K68, K69, K70, K71,
                            K72, K73, K74, K75, K76, K77, K78, K79,
                            RR1, RR8, RR14, RR18, RR19, RR28, RR34,
                            RR39, RR41, RR61, SR6, SR7>
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
        self.get_hash_value_in_array()
    }
}
