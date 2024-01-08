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
use crate::hash::{ SHA1, SHA0 };
use super::PRNG;


macro_rules! hash_for_hashes_impl {
    ($f:ty) => {
        impl PRNG for $f
        {

            fn new() -> Self
            {
                <$f>::new()
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
                let a = self.get_HashValue_in_array();
                self.tangle(tangling);
                let b = self.get_HashValue_in_array();
                self.tangle(tangling);
                let c = self.get_HashValue_in_array();
                self.tangle(tangling);
                let d = self.get_HashValue_in_array();
                let mut res = [0_u64; 8];
                for i in 0..5
                    { res[i] = ((a[i] as u64) << 32) | (b[i] as u64); }
                for i in 0..3
                    { res[i+5] = ((c[i] as u64) << 32) | (d[i] as u64); }
                res
            }
        }
    }
}

hash_for_hashes_impl! { SHA1 }
hash_for_hashes_impl! { SHA0 }
