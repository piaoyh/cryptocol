// Copyright 2023, 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module that contains the enumerator NumberErr for indicating number
//! operation.

/// In operation of BigUInt, BigInt, and LargeInt, errors can occur. In this
/// case, the enumerator `NumberErr` will indicate what kind of error occurred.
#[derive(Debug, Copy, Clone)]
pub enum NumberErr
{
    /// Indicates that the given radix is out of the valid range.
    /// It should be in the range from binary up to 62-ary, inclusively.
    OutOfValidRadixRange,

    /// Indicates that the string or the character is not alphanumeric.
    NotAlphaNumeric,

    /// Indicates that the string or the character is not fit to the given radix.
    NotFitToRadix,

    /// Indicates that the number that the string represents is too big
    /// for the created object to contain.
    TooBigNumber,
}