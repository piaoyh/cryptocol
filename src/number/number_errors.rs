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
/// 
/// # Example 1
/// ```
/// use std::str::FromStr;
/// use cryptocol::number::NumberErr;
/// use cryptocol::define_utypes_with;
/// define_utypes_with!(u128);
/// 
/// let a_biguint_wrapped = U256::from_str_radix("1234567890_ABCDEF_GHIJKLMN", 100);
/// match a_biguint_wrapped
/// {
///     Ok(a_biguint) => { println!("a_biguint = {}", a_biguint); },
///     Err(e) => {
///             println!("Error: {}", e);
///             assert_eq!(e.to_string(), "The given radix is out of the valid range. It should be in the range from binary up to 62-ary, inclusively.");
///             assert_eq!(e, NumberErr::OutOfValidRadixRange);
///         }
/// }
/// ```
/// 
/// # Example 2
/// ```
/// use std::str::FromStr;
/// use cryptocol::number::NumberErr;
/// use cryptocol::define_utypes_with;
/// define_utypes_with!(u128);
/// 
/// let a_biguint_wrapped = U256::from_str("@!#$%^&*()_+=-|-/?><`~");
/// match a_biguint_wrapped
/// {
///     Ok(a_biguint) => { println!("a_biguint = {}", a_biguint); },
///     Err(e) => {
///             println!("Error: {}", e);
///             assert_eq!(e.to_string(), "The string or the character is not alphanumeric.");
///             assert_eq!(e, NumberErr::NotAlphaNumeric);
///         }
/// }
/// ```
/// 
/// # Example 3
/// ```
/// use std::str::FromStr;
/// use cryptocol::number::NumberErr;
/// use cryptocol::define_utypes_with;
/// define_utypes_with!(u128);
/// 
/// let a_biguint_wrapped = U256::from_str_radix("1234567890_ABCDEF_GHIJKLMN", 16);
/// match a_biguint_wrapped
/// {
///     Ok(a_biguint) => { println!("a_biguint = {}", a_biguint); },
///     Err(e) => {
///             println!("Error: {}", e);
///             assert_eq!(e.to_string(), "The string or the character is not fit to the given radix.");
///             assert_eq!(e, NumberErr::NotFitToRadix);
///         }
/// }
/// ```
/// 
/// # Example 4
/// ```
/// use std::str::FromStr;
/// use cryptocol::number::NumberErr;
/// use cryptocol::define_utypes_with;
/// define_utypes_with!(u128);
/// 
/// let a_biguint_wrapped = U256::from_str("1234567891234567879123456789111111111222222222333333333444444444555555555666666666777777777888888888999999999000000000");
/// match a_biguint_wrapped
/// {
///     Ok(a_biguint) => { println!("a_biguint = {}", a_biguint); },
///     Err(e) => {
///             println!("Error: {}", e);
///             assert_eq!(e.to_string(), "The number that the string represents is too big for the created object to contain.");
///             assert_eq!(e, NumberErr::TooBigNumber);
///         }
/// }
/// ```
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
    /// use cryptocol::number::NumberErr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint_wrapped = U256::from_str_radix("1234567890_ABCDEF_GHIJKLMN", 100);
    /// match a_biguint_wrapped
    /// {
    ///     Ok(a_biguint) => { println!("a_biguint = {}", a_biguint); },
    ///     Err(e) => {
    ///             println!("Error: {}", e);
    ///             assert_eq!(e.to_string(), "The given radix is out of the valid range. It should be in the range from binary up to 62-ary, inclusively.");
    ///             assert_eq!(e, NumberErr::OutOfValidRadixRange);
    ///         }
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::number::NumberErr;
    /// 
    /// println!("NumberErr::NotAlphaNumeric: {}", NumberErr::NotAlphaNumeric);
    /// assert_eq!(NumberErr::NotAlphaNumeric.to_string(), "The string or the character is not alphanumeric.");
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::number::NumberErr;
    /// 
    /// let txt = NumberErr::TooBigNumber.to_string();
    /// println!("Error: {}", txt);
    /// assert_eq!(txt, "The number that the string represents is too big for the created object to contain.");
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use cryptocol::number::NumberErr;
    /// 
    /// let error = NumberErr::NotFitToRadix;
    /// println!("NumberErr::NotFitToRadix: {}", error);
    /// assert_eq!(NumberErr::NotFitToRadix.to_string(), "The string or the character is not fit to the given radix.");
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