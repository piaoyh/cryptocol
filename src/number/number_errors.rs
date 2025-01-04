// Copyright 2023, 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module that contains the enumerator NumberErr for indicating number
//! operation.

#![allow(missing_docs)]
#![allow(rustdoc::missing_doc_code_examples)]

use std::fmt::{ self, Display, Formatter, Debug };
use NumberErr::{OutOfValidRadixRange, NotAlphaNumeric, NotFitToRadix, TooBigNumber};

/// In operation of BigUInt, BigInt, and LargeInt, errors can occur. In this
/// case, the enumerator `NumberErr` will indicate what kind of error occurred.
#[derive(Debug, Copy, Clone, PartialEq)]
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



impl Display for NumberErr
{
    // fn fmt(&self, f: &mut Formatter) -> fmt::Result
    /// Formats the value using the given formatter.
    /// 
    /// # Arguments
    /// `f` is a buffer, this method must write the formatted string into it,
    /// and is of the type `&mut Formatter`.
    /// 
    /// # Features
    /// Automagically the function `to_string()` will be implemented. So, you
    /// can use the function `to_string()` and the macro `println!()`.
    /// [Read more](https://doc.rust-lang.org/core/fmt/trait.Display.html#tymethod.fmt)
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "69743176821145534028236692093846345739169743176821145534028236692093846345739");
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
    /// let txt = a_biguint.to_string();
    /// println!("{}", txt);
    /// assert_eq!(txt, "12345671234567890123456789012345678901234567890123456789012345678901234567890");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        let txt: &str;
        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)
        match *self
        {
            OutOfValidRadixRange => { txt = "The given radix is out of the valid range. It should be in the range from binary up to 62-ary, inclusively."; },
            NotAlphaNumeric => { txt = "The string or the character is not alphanumeric."; },
            NotFitToRadix => { txt = "The string or the character is not fit to the given radix."; },
            TooBigNumber => { txt = "The number that the string represents is too big for the created object to contain."; },
        }
        write!(f, "{}", txt)
    }
}