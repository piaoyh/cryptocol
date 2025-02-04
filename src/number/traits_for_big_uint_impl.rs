// Copyright 2023, 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module that contains implementation of external traits.

//#![warn(missing_docs)]
//#![warn(rustdoc::missing_doc_code_examples)]
// #![allow(missing_docs)]
// #![allow(rustdoc::missing_doc_code_examples)]

use std::str::FromStr;
use std::convert::From;
use std::mem::size_of_val;
use std::fmt::{ Alignment, Error, Formatter, Display, Debug, Pointer,
                Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp };
use std::cmp::{ PartialEq, PartialOrd, Ordering };
use std::ops::{ BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
                Shl, ShlAssign, Shr, ShrAssign, 
                Add, AddAssign, Sub, SubAssign, Mul, MulAssign,
                Div, DivAssign, Rem, RemAssign };

use crate::number::{ SmallUInt, BigUInt, NumberErr };


macro_rules! calc_assign_to_calc
{
    ($me:expr, $op:tt, $rhs:expr) => {
        let mut res = Self::from_array($me.get_number().clone());
        res $op $rhs;
        return res;
    };
    // calc_assign_to_calc!(self, <<=, rhs);
    //
    // let mut s = Self::from_array(self.get_number().clone());
    // s <<= rhs;
    // s

    // ($me:expr, $func:expr, $rhs:expr, $modulo:expr) => {
    //     let mut res = Self::from_array(Self::get_number($me).clone());
    //     $func(&mut res, $rhs, $modulo);
    //     return res;
    // }
    // calc_assign_to_calc!(self, Self::modular_add_assign_uint, rhs, modulo);
    //
    // let mut res = Self::from_array(self.get_number().clone());
    // res.modular_add_assign_uint(rhs, modulo);
    // res
}

macro_rules! fmt_with_radix
{
    ($me:expr, $f:expr, $radix:expr, $prefix:expr, $lowerhex:expr, $pointer:expr) => {
        return match $me.to_string_with_radix($radix)
        {
            Ok(txt) => {
                let txt = if $lowerhex {txt.to_lowercase()} else {txt};
                let len = txt.len();
                let mut space = match $f.width()
                {
                    Some(n) => if len >= n {0} else {n - len},
                    None => 0_usize,
                };
                let mut prefix = String::new();
                if $f.sign_plus()
                {
                    prefix.push('+');
                    space = if space > 1 {space-1} else {0};
                }
                if $f.sign_minus()
                {
                    prefix.push('-');
                    space = if space > 1 {space-1} else {0};
                }
                if $f.alternate() || $pointer
                {
                    prefix.push_str($prefix);
                    space = if space > 2 {space-2} else {0};
                }
                if $f.sign_aware_zero_pad()
                {
                    for _ in 0..space
                        { prefix.push('0'); }
                    write!($f, "{}{}", prefix, txt)
                }
                else
                {
                    let ch = $f.fill();
                    if let Some(s) = $f.align()
                    {
                        match s
                        {
                        Alignment::Left => {
                                let mut trailing = String::new();
                                for _ in 0..space
                                    { trailing.push(ch); }
                                write!($f, "{}{}{}", prefix, txt, trailing)
                            },
                        Alignment::Right => {
                                let mut leading = String::new();
                                for _ in 0..space
                                    { leading.push(ch); }
                                write!($f, "{}{}{}", leading, prefix, txt)
                            },
                        Alignment::Center => {
                                let mut leading = String::new();
                                for _ in 0..space>>1
                                    { leading.push(ch); }
                                let mut trailing = String::new();
                                for _ in 0..(space+1)>>1
                                    { trailing.push(ch); }
                                write!($f, "{}{}{}{}", leading, prefix, txt, trailing)
                            },
                        }
                    }
                    else
                    {
                        let mut trailing = String::new();
                        for _ in 0..space
                            { trailing.push(ch); }
                        write!($f, "{}{}{}", prefix, txt, trailing)
                    }
                }
            },
            Err(_) => { Err(Error) },
        };
    };
    // fmt_with_radix!(self, f, 16, "0X", false, false);
    //
    // return match self.to_string_with_radix(16)
    // {
    //     Ok(txt) => {
    //         let len = txt.len();
    //         let mut space = match f.width()
    //         {
    //             Some(n) => if len >= n {0} else {n - len},
    //             None => 0_usize,
    //         };
    //         let mut prefix = String::new();
    //         if f.sign_plus()
    //         {
    //             prefix.push('+');
    //             space = if space > 1 {space-1} else {0};
    //         }
    //         if f.sign_minus()
    //         {
    //             prefix.push('-');
    //             space = if space > 1 {space-1} else {0};
    //         }
    //         if f.alternate()
    //         {
    //             prefix.push_str("0x");
    //             space = if space > 2 {space-2} else {0};
    //         }
    //         if f.sign_aware_zero_pad()
    //         {
    //             for _ in 0..space
    //                 { prefix.push('0'); }
    //             write!(f, "{}{}", prefix, txt)
    //         }
    //         else
    //         {
    //             let ch = f.fill();
    //             if let Some(s) = f.align()
    //             {
    //                 match s
    //                 {
    //                 Alignment::Left => {
    //                         let mut trailing = String::new();
    //                         for _ in 0..space
    //                             { trailing.push(ch); }
    //                         write!(f, "{}{}{}", prefix, txt, trailing)
    //                     },
    //                 Alignment::Right => {
    //                         let mut leading = String::new();
    //                         for _ in 0..space
    //                             { leading.push(ch); }
    //                         write!(f, "{}{}{}", leading, prefix, txt)
    //                     },
    //                 Alignment::Center => {
    //                         let mut leading = String::new();
    //                         for _ in 0..space>>1
    //                             { leading.push(ch); }
    //                         let mut trailing = String::new();
    //                         for _ in 0..(space+1)>>1
    //                             { trailing.push(ch); }
    //                         write!(f, "{}{}{}{}", leading, prefix, txt, trailing)
    //                     },
    //                 }
    //             }
    //             else
    //             {
    //                 let mut trailing = String::new();
    //                 for _ in 0..space
    //                     { trailing.push(ch); }
    //                 write!(f, "{}{}{}", prefix, txt, trailing)
    //             }
    //         }
    //     },
    //     Err(_) => { Err(Error) },
    // }

    ($me:expr, $f:expr, $radix:expr, $prefix:expr, $lowerhex:expr) => {
        fmt_with_radix!($me, $f, $radix, $prefix, $lowerhex, false);
    };
    ($me:expr, $f:expr, $radix:expr, $prefix:expr) => {
        fmt_with_radix!($me, $f, $radix, $prefix, false, false);
    };
    ($me:expr, $f:expr) => {
        fmt_with_radix!($me, $f, 16, "0x", true, true);
    };
}

macro_rules! fmt_with_exponent
{
    ($me:expr, $f:expr, $exponent:expr) => {
        return match $me.to_string_with_radix(10)
        {
            Ok(mut txt) => {
                let exp = txt.len() - 1;
                let exp_txt = exp.to_string();
                if let Some(precision) = $f.precision()
                {
                    if exp > precision
                    {
                        let roundup = txt.as_bytes()[precision + 1] as u32 >= '5' as u32;
                        txt.truncate(precision + 1);
                        if roundup
                        {
                            let num = Self::from_str(txt.as_str()).unwrap();
                            txt = num.wrapping_add_uint(1_u8).to_string();
                        }
                    }
                }
                if exp != 0
                    { txt.insert(1, '.'); }
                txt.push($exponent);
                txt.push_str(exp_txt.as_str());
                let len = txt.len();
                let mut space = match $f.width()
                {
                    Some(n) => if len >= n {0} else {n - len},
                    None => 0_usize,
                };
                let mut prefix = String::new();
                if $f.sign_plus()
                {
                    prefix.push('+');
                    space = if space > 1 {space-1} else {0};
                }
                if $f.sign_minus()
                {
                    prefix.push('-');
                    space = if space > 1 {space-1} else {0};
                }
                if $f.sign_aware_zero_pad()
                {
                    for _ in 0..space
                        { prefix.push('0'); }
                    write!($f, "{}{}", prefix, txt)
                }
                else
                {
                    let ch = $f.fill();
                    if let Some(s) = $f.align()
                    {
                        match s
                        {
                        Alignment::Left => {
                                let mut trailing = String::new();
                                for _ in 0..space
                                    { trailing.push(ch); }
                                write!($f, "{}{}{}", prefix, txt, trailing)
                            },
                        Alignment::Right => {
                                let mut leading = String::new();
                                for _ in 0..space
                                    { leading.push(ch); }
                                write!($f, "{}{}{}", leading, prefix, txt)
                            },
                        Alignment::Center => {
                                let mut leading = String::new();
                                for _ in 0..space>>1
                                    { leading.push(ch); }
                                let mut trailing = String::new();
                                for _ in 0..(space+1)>>1
                                    { trailing.push(ch); }
                                write!($f, "{}{}{}{}", leading, prefix, txt, trailing)
                            },
                        }
                    }
                    else
                    {
                        let mut trailing = String::new();
                        for _ in 0..space
                            { trailing.push(ch); }
                        write!($f, "{}{}{}", prefix, txt, trailing)
                    }
                }
            },
            Err(_) => { Err(Error) },
        };
    };
    // fmt_with_exponent!(self, f, 'e');
    // 
    // return match self.to_string_with_radix(10)
    // {
    //     Ok(mut txt) => {
    //         let exp = txt.len() - 1;
    //         let exp_txt = exp.to_string();
    //         if let Some(precision) = f.precision()
    //         {
    //             if exp > precision
    //             {
    //                 let roundup = txt.as_bytes()[precision + 1] as u32 >= '5' as u32;
    //                 txt.truncate(precision + 1);
    //                 if roundup
    //                 {
    //                     let num = Self::from_str(txt.as_str()).unwrap();
    //                     txt = num.wrapping_add_uint(1_u8).to_string();
    //                 }
    //             }
    //         }
    //         if exp != 0
    //             { txt.insert(1, '.'); }
    //         txt.push('e');
    //         txt.push_str(exp_txt.as_str());
    //         let len = txt.len();
    //         let mut space = match $f.width()
    //         {
    //             Some(n) => if len >= n {0} else {n - len},
    //             None => 0_usize,
    //         };
    //         let mut prefix = String::new();
    //         if f.sign_plus()
    //         {
    //             prefix.push('+');
    //             space = if space > 1 {space-1} else {0};
    //         }
    //         if f.sign_minus()
    //         {
    //             prefix.push('-');
    //             space = if space > 1 {space-1} else {0};
    //         }
    //         if f.sign_aware_zero_pad()
    //         {
    //             for _ in 0..space
    //                 { prefix.push('0'); }
    //             write!(f, "{}{}", prefix, txt)
    //         }
    //         else
    //         {
    //             let ch = f.fill();
    //             if let Some(s) = f.align()
    //             {
    //                 match s
    //                 {
    //                 Alignment::Left => {
    //                         let mut trailing = String::new();
    //                         for _ in 0..space
    //                             { trailing.push(ch); }
    //                         write!(f, "{}{}{}", prefix, txt, trailing)
    //                     },
    //                 Alignment::Right => {
    //                         let mut leading = String::new();
    //                         for _ in 0..space
    //                             { leading.push(ch); }
    //                         write!(f, "{}{}{}", leading, prefix, txt)
    //                     },
    //                 Alignment::Center => {
    //                         let mut leading = String::new();
    //                         for _ in 0..space>>1
    //                             { leading.push(ch); }
    //                         let mut trailing = String::new();
    //                         for _ in 0..(space+1)>>1
    //                             { trailing.push(ch); }
    //                         write!(f, "{}{}{}{}", leading, prefix, txt, trailing)
    //                     },
    //                 }
    //             }
    //             else
    //             {
    //                 let mut trailing = String::new();
    //                 for _ in 0..space
    //                     { trailing.push(ch); }
    //                 write!(f, "{}{}{}", prefix, txt, trailing)
    //             }
    //         }
    //     },
    //     Err(_) => { Err(Error) },
    // };
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `+` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after addition
/// operation. In order to prevent this, the operands should be cloned or
/// copied before addition operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait Add<Rhs = Self> {
///     type Output;
///     // Required method
///     fn add(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait Add<Rhs = Self> {
///     type Output;
///     // Required method
///     fn add(self, rhs: Rhs) -> Self::Output;
///   or
///     fn add(&self, rhs: Rhs) -> Self::Output;
///   or
///     fn add(self, rhs: &Rhs) -> Self::Output;
///   or
///     fn add(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility and because the function(s) of the trait should be fixed,
/// trait Add2, Add3, and Add4 are provided and the developer
/// implements none or only one of traits Add, Add2, Add3, and Add4.
/// 
/// ```text
/// pub trait Add<Rhs = Self> {
///     type Output;
///     // Required method
///     fn add(self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait Add2<Rhs = Self> {
///     type Output;
///     // Required method
///     fn add(&self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait Add2<Rhs = Self> {
///     type Output;
///     // Required method
///     fn add(self, rhs: &Rhs) -> Self::Output;
/// }
/// 
/// pub trait Add2<Rhs = Self> {
///     type Output;
///     // Required method
///     fn add(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// Unlike trait Add, the trait PartialEq makes the operators `==` and `!=` take
/// not `&Self` but `Self` as its first operand and not `&Rhs` (or `&Self`) but
/// `Rhs` (or `Self`) as its second operand but makes the functions eq() and
/// ne() take not `self` but `&self` as its first argument and not `Rhs` but
/// `&Rhs` as its second argument. So, I think the third suggestion is possible.
/// The prototype of trait PartialEq is as follows:
/// 
/// ```text
/// pub trait PartialEq<Rhs = Self>
/// where
/// Rhs: ?Sized,
/// {
///     // Required method
///     fn eq(&self, other: &Rhs) -> bool;
/// 
///     // Provided method
///     fn ne(&self, other: &Rhs) -> bool { ... }
/// }
/// ```
impl<T, const N: usize> Add for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    // fn add(self, rhs: Self) -> Self
    /// Calculates `self` + `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and returns an addition result `self` + `rhs`.
    /// 
    /// # Arguments
    /// `rhs` is to be added to `self`, and is of `Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `self` + `rhs` with wrapping (modular) addition.
    /// 
    /// # Features
    /// - Wrapping (modular) addition.
    /// - If overflow happened, the flag `OVERFLOW` of the return value
    ///   will be set.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U512::max() - 1_u8;
    /// let one_biguint = U512::one();
    /// let res = a_biguint.clone() + one_biguint.clone();
    /// println!("{} + {} = {}", a_biguint, one_biguint, res);
    /// assert_eq!(res, U512::max());
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U512::max() - 1_u8;
    /// let two_biguint = U512::from_uint(2_u8);
    /// let res = a_biguint.clone() + two_biguint.clone();
    /// println!("{} + {} = {}", a_biguint, two_biguint, res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), true);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U512::max() - 1_u8;
    /// let three_biguint = U512::from_uint(3_u8);
    /// let res = a_biguint.clone() + three_biguint.clone();
    /// println!("{} + {} = {}", a_biguint, three_biguint, res);
    /// assert_eq!(res.to_string(), "1");
    /// assert_eq!(res.is_overflow(), true);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// ```
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U512::max() - 1_u8;
    /// let one_biguint = U512::one();
    /// let _res = a_biguint + one_biguint;
    /// println!("{} + {} = {}", a_biguint, one_biguint, _res);
    /// // The operator '+' swallowed (took the ownership of) a_biguint and one_biguint.
    /// 
    /// let a_biguint = U512::max() - 1_u8;
    /// let two_biguint = U512::from_uint(2_u8);
    /// let _res = a_biguint + two_biguint;
    /// println!("{} + {} = {}", a_biguint, two_biguint, _res);
    /// // The operator '+' swallowed (took the ownership of) a_biguint and two_biguint.
    /// 
    /// let a_biguint = U512::max() - 1_u8;
    /// let three_biguint = U512::from_uint(3_u8);
    /// let _res = a_biguint + three_biguint;
    /// println!("{} + {} = {}", a_biguint, three_biguint, _res);
    /// // The operator '+' swallowed (took the ownership of) a_biguint and three_biguint.
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    fn add(self, rhs: Self) -> Self
    {
        self.wrapping_add(&rhs)
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `+` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after addition
/// operation. In order to prevent this, the operands should be cloned or
/// copied before addition operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait Add<Rhs = Self> {
///     type Output;
///     // Required method
///     fn add(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait Add<Rhs = Self> {
///     type Output;
///     // Required method
///     fn add(self, rhs: Rhs) -> Self::Output;
///   or
///     fn add(&self, rhs: Rhs) -> Self::Output;
///   or
///     fn add(self, rhs: &Rhs) -> Self::Output;
///   or
///     fn add(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait Add2, Add3, and Add4 are provided and the developer
/// implements none or only one of traits Add, Add2, Add3, and Add4.
/// 
/// ```text
/// pub trait Add<Rhs = Self> {
///     type Output;
///     // Required method
///     fn add(self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait Add2<Rhs = Self> {
///     type Output;
///     // Required method
///     fn add(&self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait Add3<Rhs = Self> {
///     type Output;
///     // Required method
///     fn add(self, rhs: &Rhs) -> Self::Output;
/// }
/// 
/// pub trait Add4<Rhs = Self> {
///     type Output;
///     // Required method
///     fn add(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// Unlike trait Add, the trait PartialEq makes the operators `==` and `!=` take
/// not `&Self` but `Self` as its first operand and not `&Rhs` (or `&Self`) but
/// `Rhs` (or `Self`) as its second operand but makes the functions eq() and
/// ne() take not `self` but `&self` as its first argument and not `Rhs` but
/// `&Rhs` as its second argument. So, I think the third suggestion is possible.
/// The prototype of trait PartialEq is as follows:
/// 
/// ```text
/// pub trait PartialEq<Rhs = Self>
/// where
/// Rhs: ?Sized,
/// {
///     // Required method
///     fn eq(&self, other: &Rhs) -> bool;
/// 
///     // Provided method
///     fn ne(&self, other: &Rhs) -> bool { ... }
/// }
/// ```
impl<T, const N: usize> Add<T> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    // fn add(self, rhs: T)-> Self
    /// Calculates `self` + `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and returns an addition result `self` + `rhs`.
    /// 
    /// # Arguments
    /// `rhs` is to be added to `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `self` + `rhs` with wrapping (modular) addition.
    /// 
    /// # Features
    /// - Wrapping (modular) addition.
    /// - If overflow happened, the flag `OVERFLOW` of the return value
    ///   will be set.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U512::max() - 1_u16;
    /// let one_uint = 1_u16;
    /// let res = a_biguint.clone() + one_uint;
    /// println!("{} + {} = {}", a_biguint, one_uint, res);
    /// assert_eq!(res.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U512::max() - 1_u16;
    /// let two_uint = 2_u16;
    /// let res = a_biguint.clone() + two_uint;
    /// println!("{} + {} = {}", a_biguint, two_uint, res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), true);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U512::max() - 1_u16;
    /// let three_uint = 3_u16;
    /// let res = a_biguint.clone() + three_uint;
    /// println!("{} + {} = {}", a_biguint, three_uint, res);
    /// assert_eq!(res.to_string(), "1");
    /// assert_eq!(res.is_overflow(), true);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// ```
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U512::max() - 1_u16;
    /// let one_uint = 1_u16;
    /// let res = a_biguint + one_uint;
    /// println!("{} + {} = {}", a_biguint, one_uint, res);
    /// // The operator '+' swallowed (took the ownership of) a_biguint.
    /// 
    /// let a_biguint = U512::max() - 1_u16;
    /// let two_uint = 2_u16;
    /// let res = a_biguint + two_uint;
    /// println!("{} + {} = {}", a_biguint, two_uint, res);
    /// // The operator '+' swallowed (took the ownership of) a_biguint.
    /// 
    /// let a_biguint = U512::max() - 1_u16;
    /// let three_uint = 3_u16;
    /// let res = a_biguint + three_uint;
    /// println!("{} + {} = {}", a_biguint, three_uint, res);
    /// // The operator '+' swallowed (took the ownership of) a_biguint.
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    fn add(self, rhs: T)-> Self
    {
        self.wrapping_add_uint(rhs)
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `+=` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after addition
/// operation. In order to prevent this, the operands should be cloned or
/// copied before addition operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait AddAssign<Rhs = Self> {
///     // Required method
///     fn add_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait AddAssign<Rhs = Self> {
///     // Required method
///     fn add_assign(&mut self, rhs: Rhs);
///   or
///     fn add_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait AddAssign2 is provided and the developer
/// implements none or only one of traits AddAssign and AddAssign2.
/// 
/// ```text
/// pub trait AddAssign<Rhs = Self> {
///     // Required method
///     fn add_assign(&mut self, rhs: Rhs);
/// }
/// 
/// pub trait AddAssign2<Rhs = Self> {
///     // Required method
///     fn add_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// Unlike trait AddAssign, the trait PartialEq makes the operators
/// `==` and `!=` take not `&Self` but `Self` as its first operand and not
/// `&Rhs` (or `&Self`) but `Rhs` (or `Self`) as its second operand but makes
/// the functions eq() and ne() take not `self` but `&self` as its first
/// argument and not `Rhs` but `&Rhs` as its second argument.
/// So, I think the third suggestion is possible.
/// The prototype of trait PartialEq is as follows:
/// 
/// ```text
/// pub trait PartialEq<Rhs = Self>
/// where
/// Rhs: ?Sized,
/// {
///     // Required method
///     fn eq(&self, other: &Rhs) -> bool;
/// 
///     // Provided method
///     fn ne(&self, other: &Rhs) -> bool { ... }
/// }
/// ```
impl<T, const N: usize> AddAssign for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // fn add_assign(&mut self, rhs: Self)
    /// Calculates `self` + `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and assign the addition result `self` + `rhs` to `self` back.
    /// 
    /// # Arguments
    /// `rhs` is to be added to `self`, and is of `Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - Wrapping (modular) addition.
    /// - If overflow happened, the flag `OVERFLOW` of `self` will be set.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U512::max() - 1_u32;
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// 
    /// let one_biguint = U512::one();
    /// a_biguint += one_biguint.clone();
    /// println!("After a_biguint += {}, a_biguint = {}", one_biguint, a_biguint);
    /// assert_eq!(a_biguint, U512::max());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// ```
    /// 
    /// # Example 2 
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U512::max() - 1_u32;
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// 
    /// let two_biguint = U512::from_uint(2_u8);
    /// a_biguint += two_biguint.clone();
    /// println!("After a_biguint += {}, a_biguint = {}", two_biguint, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U512::max() - 1_u32;
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// 
    /// let three_biguint = U512::from_uint(3_u8);
    /// a_biguint += three_biguint.clone();
    /// println!("After a_biguint += {},\ta_biguint = {}", three_biguint, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// ```
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U512::max() - 1_u32;
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// let one_biguint = U512::one();
    /// a_biguint += one_biguint;
    /// println!("After a_biguint += {}, a_biguint = {}", one_biguint, a_biguint);
    /// // The operator '+=' swallowed (took the ownership of) one_biguint.
    /// 
    /// let mut a_biguint = U512::max() - 1_u32;
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// let two_biguint = U512::from_uint(2_u8);
    /// a_biguint += two_biguint.clone();
    /// println!("After a_biguint += {}, a_biguint = {}", two_biguint, a_biguint);
    /// // The operator '+=' swallowed (took the ownership of) two_biguint.
    /// 
    /// let mut a_biguint = U512::max() - 1_u32;
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// let three_biguint = U512::from_uint(3_u8);
    /// a_biguint += three_biguint.clone();
    /// println!("After a_biguint += {}, a_biguint = {}", three_biguint, a_biguint);
    /// // The operator '+=' swallowed (took the ownership of) three_biguint.
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    fn add_assign(&mut self, rhs: Self)
    {
        self.wrapping_add_assign(&rhs);
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `+=` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after addition
/// operation. In order to prevent this, the operands should be cloned or
/// copied before addition operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait AddAssign<Rhs = Self> {
///     // Required method
///     fn add_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait AddAssign<Rhs = Self> {
///     // Required method
///     fn add_assign(&mut self, rhs: Rhs);
///   or
///     fn add_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait AddAssign2 is provided and the developer
/// implements none or only one of traits AddAssign and AddAssign2.
/// 
/// ```text
/// pub trait AddAssign<Rhs = Self> {
///     // Required method
///     fn add_assign(&mut self, rhs: Rhs);
/// }
/// 
/// pub trait AddAssign2<Rhs = Self> {
///     // Required method
///     fn add_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// Unlike trait AddAssign, the trait PartialEq makes the operators
/// `==` and `!=` take not `&Self` but `Self` as its first operand and not
/// `&Rhs` (or `&Self`) but `Rhs` (or `Self`) as its second operand but makes
/// the functions eq() and ne() take not `self` but `&self` as its first
/// argument and not `Rhs` but `&Rhs` as its second argument.
/// So, I think the third suggestion is possible.
/// The prototype of trait PartialEq is as follows:
/// 
/// ```text
/// pub trait PartialEq<Rhs = Self>
/// where
/// Rhs: ?Sized,
/// {
///     // Required method
///     fn eq(&self, other: &Rhs) -> bool;
/// 
///     // Provided method
///     fn ne(&self, other: &Rhs) -> bool { ... }
/// }
/// ```
impl<T, const N: usize> AddAssign<T> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // fn add_assign(&mut self, rhs: T)
    /// Calculates `self` + `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and assigns the addition result `self` + `rhs` to `self` back.
    /// 
    /// # Arguments
    /// `rhs` is to be added to `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - Wrapping (modular) addition.
    /// - If overflow happened, the flag `OVERFLOW` of `self` will be set.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = UU64::max() - 1_u64;
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// 
    /// let one_uint = 1_u64;
    /// a_biguint += one_uint;
    /// println!("After a_biguint += {}, a_biguint = {}", one_uint, a_biguint);
    /// assert_eq!(a_biguint, UU64::max());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = UU64::max() - 1_u64;
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// 
    /// let two_uint = 2_u64;
    /// a_biguint += two_uint;
    /// println!("After a_biguint += {}, a_biguint = {}", two_uint, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U512::max() - 1_u64;
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// 
    /// let three_uint = 3_u64;
    /// a_biguint += three_uint;
    /// println!("After a_biguint += {}, a_biguint = {}", three_uint, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    fn add_assign(&mut self, rhs: T)
    {
        self.wrapping_add_assign_uint(rhs);
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `-` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after addition
/// operation. In order to prevent this, the operands should be cloned or
/// copied before addition operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait Sub<Rhs = Self> {
///     type Output;
///     // Required method
///     fn sub(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait Sub<Rhs = Self> {
///     type Output;
///     // Required method
///     fn sub(self, rhs: Rhs) -> Self::Output;
///   or
///     fn sub(&self, rhs: Rhs) -> Self::Output;
///   or
///     fn sub(self, rhs: &Rhs) -> Self::Output;
///   or
///     fn sub(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait Sub2, Sub3, and Sub4 are provided and the developer
/// implements none or only one of traits Sub, Sub2, Sub3, and Sub4.
/// 
/// ```text
/// pub trait Sub<Rhs = Self> {
///     type Output;
///     // Required method
///     fn sub(self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait Sub2<Rhs = Self> {
///     type Output;
///     // Required method
///     fn sub(&self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait Sub3<Rhs = Self> {
///     type Output;
///     // Required method
///     fn sub(self, rhs: &Rhs) -> Self::Output;
/// }
/// 
/// pub trait Sub4<Rhs = Self> {
///     type Output;
///     // Required method
///     fn sub(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// Unlike trait Sub, the trait PartialEq makes the operators `==` and `!=` take
/// not `&Self` but `Self` as its first operand and not `&Rhs` (or `&Self`) but
/// `Rhs` (or `Self`) as its second operand but makes the functions eq() and
/// ne() take not `self` but `&self` as its first argument and not `Rhs` but
/// `&Rhs` as its second argument. So, I think the third suggestion is possible.
/// The prototype of trait PartialEq is as follows:
/// 
/// ```text
/// pub trait PartialEq<Rhs = Self>
/// where
/// Rhs: ?Sized,
/// {
///     // Required method
///     fn eq(&self, other: &Rhs) -> bool;
/// 
///     // Provided method
///     fn ne(&self, other: &Rhs) -> bool { ... }
/// }
/// ```
impl<T, const N: usize> Sub for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    // fn sub(self, rhs: Self) -> Self
    /// Calculates `self` - `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and returns a subtraction result `self` - `rhs`.
    /// 
    /// # Arguments
    /// `rhs` is to be subtracted from `self`, and is of `Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `self` - `rhs` with wrapping (modular) subtraction.
    /// 
    /// # Features
    /// - Wrapping (modular) subtraction.
    /// - If underflow happened, the flag `UNDERFLOW` of the return value
    ///   will be set.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U512::one();
    /// let one_biguint = U512::one();
    /// let res = a_biguint.clone() - one_biguint.clone();
    /// println!("{} - {} = {}", a_biguint, one_biguint, res);
    /// assert_eq!(res, U512::zero());
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U512::one();
    /// let two_biguint = U512::from_uint(2_u8);
    /// let res = a_biguint.clone() - two_biguint.clone();
    /// println!("{} - {} = {}", a_biguint, two_biguint, res);
    /// assert_eq!(res, U512::max());
    /// assert_eq!(res.is_underflow(), true);
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U512::one();
    /// let three_biguint = U512::from_uint(3_u8);
    /// let res = a_biguint.clone() - three_biguint.clone();
    /// println!("{} - {} = {}", a_biguint, three_biguint, res);
    /// assert_eq!(res.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    /// assert_eq!(res.is_underflow(), true);
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// ```
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U512::one();
    /// let one_biguint = U512::one();
    /// let _res = a_biguint - one_biguint;
    /// println!("{} - {} = {}", a_biguint, one_biguint, _res);
    /// // The operator '-' swallowed (took the ownership of) a_biguint and one_biguint.
    /// 
    /// let a_biguint = U512::one();
    /// let two_biguint = U512::from_uint(2_u8);
    /// let _res = a_biguint - two_biguint;
    /// println!("{} - {} = {}", a_biguint, one_biguint, _res);
    /// // The operator '-' swallowed (took the ownership of) a_biguint and two_biguint.
    /// 
    /// let a_biguint = U512::one();
    /// let three_biguint = U512::from_uint(3_u8);
    /// let _res = a_biguint - three_biguint;
    /// println!("{} - {} = {}", a_biguint, one_biguint, _res);
    /// // The operator '-' swallowed (took the ownership of) a_biguint and three_biguint.
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    fn sub(self, rhs: Self) -> Self
    {
        self.wrapping_sub(&rhs)
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `-` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after addition
/// operation. In order to prevent this, the operands should be cloned or
/// copied before addition operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait Sub<Rhs = Self> {
///     type Output;
///     // Required method
///     fn sub(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait Sub<Rhs = Self> {
///     type Output;
///     // Required method
///     fn sub(self, rhs: Rhs) -> Self::Output;
///   or
///     fn sub(&self, rhs: Rhs) -> Self::Output;
///   or
///     fn sub(self, rhs: &Rhs) -> Self::Output;
///   or
///     fn sub(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait Sub2, Sub3, and Sub4 are provided and the developer
/// implements none or only one of traits Sub, Sub2, Sub3, and Sub4.
/// 
/// ```text
/// pub trait Sub<Rhs = Self> {
///     type Output;
///     // Required method
///     fn sub(self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait Sub2<Rhs = Self> {
///     type Output;
///     // Required method
///     fn sub(&self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait Sub3<Rhs = Self> {
///     type Output;
///     // Required method
///     fn sub(self, rhs: &Rhs) -> Self::Output;
/// }
/// 
/// pub trait Sub4<Rhs = Self> {
///     type Output;
///     // Required method
///     fn sub(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// Unlike trait Sub, the trait PartialEq makes the operators `==` and `!=` take
/// not `&Self` but `Self` as its first operand and not `&Rhs` (or `&Self`) but
/// `Rhs` (or `Self`) as its second operand but makes the functions eq() and
/// ne() take not `self` but `&self` as its first argument and not `Rhs` but
/// `&Rhs` as its second argument. So, I think the third suggestion is possible.
/// The prototype of trait PartialEq is as follows:
/// 
/// ```text
/// pub trait PartialEq<Rhs = Self>
/// where
/// Rhs: ?Sized,
/// {
///     // Required method
///     fn eq(&self, other: &Rhs) -> bool;
/// 
///     // Provided method
///     fn ne(&self, other: &Rhs) -> bool { ... }
/// }
/// ```
impl<T, const N: usize> Sub<T> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    // fn sub(self, rhs: T) -> Self
    /// Calculates `self` - `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and returns a subtraction result `self` - `rhs`.
    /// 
    /// # Arguments
    /// `rhs` is to be subtracted from `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `self` - `rhs` with wrapping (modular) subtraction.
    /// 
    /// # Features
    /// - Wrapping (modular) subtraction.
    /// - If underflow happened, the flag `UNDERFLOW` of the return value
    ///   will be set.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U512::one();
    /// let one_uint = 1_u8;
    /// let res = a_biguint.clone() - one_uint.clone();
    /// println!("{} - {} = {}", a_biguint, one_uint, res);
    /// assert_eq!(res, U512::zero());
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U512::one();
    /// let two_uint = 2_u8;
    /// let res = a_biguint.clone() - two_uint.clone();
    /// println!("{} - {} = {}", a_biguint, two_uint, res);
    /// assert_eq!(res, U512::max());
    /// assert_eq!(res.is_underflow(), true);
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U512::one();
    /// let three_uint = 3_u8;
    /// let res = a_biguint.clone() - three_uint.clone();
    /// println!("{} - {} = {}", a_biguint, three_uint, res);
    /// assert_eq!(res.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    /// assert_eq!(res.is_underflow(), true);
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// ```
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U512::one();
    /// let one_uint = 1_8;
    /// let _res = a_biguint - one_uint;
    /// println!("{} - {} = {}", a_biguint, one_uint, _res);
    /// // The operator '-' swallowed (took the ownership of) a_biguint.
    /// 
    /// let a_biguint = U512::one();
    /// let two_uint = 2_u8;
    /// let _res = a_biguint - two_uint;
    /// println!("{} - {} = {}", a_biguint, one_uint, _res);
    /// // The operator '-' swallowed (took the ownership of) a_biguint.
    /// 
    /// let a_biguint = U512::one();
    /// let three_uint = 3_u8;
    /// let _res = a_biguint - three_uint;
    /// println!("{} - {} = {}", a_biguint, one_uint, _res);
    /// // The operator '-' swallowed (took the ownership of) a_biguint.
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    fn sub(self, rhs: T) -> Self
    {
        self.wrapping_sub_uint(rhs)
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `-=` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after subtraction
/// operation. In order to prevent this, the operands should be cloned or
/// copied before subtraction operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait SubAssign<Rhs = Self> {
///     // Required method
///     fn sub_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait SubAssign<Rhs = Self> {
///     // Required method
///     fn sub_assign(&mut self, rhs: Rhs);
///   or
///     fn sub_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait SubAssign2 is provided and the developer
/// implements none or only one of traits SubAssign and SubAssign2.
/// ```
/// pub trait SubAssign<Rhs = Self> {
///     // Required method
///     fn sub_assign(&mut self, rhs: Rhs);
/// }
/// 
/// pub trait SubAssign2<Rhs = Self> {
///     // Required method
///     fn sub_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// Unlike trait SubAssign, the trait PartialEq makes the operators
/// `==` and `!=` take not `&Self` but `Self` as its first operand and not
/// `&Rhs` (or `&Self`) but `Rhs` (or `Self`) as its second operand but makes
/// the functions eq() and ne() take not `self` but `&self` as its first
/// argument and not `Rhs` but `&Rhs` as its second argument.
/// So, I think the third suggestion is possible.
/// The prototype of trait PartialEq is as follows:
/// 
/// ```text
/// pub trait PartialEq<Rhs = Self>
/// where
/// Rhs: ?Sized,
/// {
///     // Required method
///     fn eq(&self, other: &Rhs) -> bool;
/// 
///     // Provided method
///     fn ne(&self, other: &Rhs) -> bool { ... }
/// }
/// ```
impl<T, const N: usize> SubAssign for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // fn sub_assign(&mut self, rhs: Self)
    /// Calculates `self` - `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and assign the subtraction result `self` - `rhs` to `self` back.
    /// 
    /// # Arguments
    /// `rhs` is to be subtracted from `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - Wrapping (modular) subtraction.
    /// - If underflow happened, the flag `UNDERFLOW` of `self` will be set.
    /// - All the flags are historical, which means, for example, if an underflow
    ///   occurred even once before this current operation or `UNDERFLOW`
    ///   flag is already set before this current operation, the `UNDERFLOW` flag
    ///   is not changed even if this current operation does not cause underflow.
    ///
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U512::one();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// 
    /// let one_biguint = U512::one();
    /// a_biguint -= one_biguint.clone();
    /// println!("After a_biguint -= {}, a_biguint = {}", one_biguint, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// ```
    ///
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U512::one();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// 
    /// let two_biguint = U512::from_uint(2_u8);
    /// a_biguint -= two_biguint.clone();
    /// println!("After a_biguint -= {}, a_biguint = {}", two_biguint, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    /// assert_eq!(a_biguint.is_underflow(), true);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// ```
    ///
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U512::one();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// 
    /// let three_biguint = U512::from_uint(3_u8);
    /// a_biguint -= three_biguint.clone();
    /// println!("After a_biguint -= {}, a_biguint = {}", three_biguint, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    /// assert_eq!(a_biguint.is_underflow(), true);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// ```
    ///
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U512::one();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// let one_biguint = U512::one();
    /// a_biguint -= one_biguint;
    /// println!("After a_biguint -= {}, a_biguint = {}", one_biguint, a_biguint);
    /// // The operator '-=' swallowed (took the ownership of) one_biguint.
    /// 
    /// let mut a_biguint = U512::one();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// let two_biguint = U512::from_uint(2_u8);
    /// a_biguint -= two_biguint.clone();
    /// println!("After a_biguint -= {}, a_biguint = {}", two_biguint, a_biguint);
    /// // The operator '-=' swallowed (took the ownership of) two_biguint.
    /// 
    /// let mut a_biguint = U512::one();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// let three_biguint = U512::from_uint(3_u8);
    /// a_biguint -= three_biguint.clone();
    /// println!("After a_biguint -= {}, a_biguint = {}", three_biguint, a_biguint);
    /// // The operator '-=' swallowed (took the ownership of) three_biguint.
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    fn sub_assign(&mut self, rhs: Self)
    {
        self.wrapping_sub_assign(&rhs);
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `-=` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after subtraction
/// operation. In order to prevent this, the operands should be cloned or
/// copied before subtraction operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait SubAssign<Rhs = Self> {
///     // Required method
///     fn sub_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait SubAssign<Rhs = Self> {
///     // Required method
///     fn sub_assign(&mut self, rhs: Rhs);
///   or
///     fn sub_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait SubAssign2 is provided and the developer
/// implements none or only one of traits SubAssign and SubAssign2.
/// 
/// ```
/// pub trait SubAssign<Rhs = Self> {
///     // Required method
///     fn sub_assign(&mut self, rhs: Rhs);
/// }
/// 
/// pub trait SubAssign2<Rhs = Self> {
///     // Required method
///     fn sub_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// Unlike trait SubAssign, the trait PartialEq makes the operators
/// `==` and `!=` take not `&Self` but `Self` as its first operand and not
/// `&Rhs` (or `&Self`) but `Rhs` (or `Self`) as its second operand but makes
/// the functions eq() and ne() take not `self` but `&self` as its first
/// argument and not `Rhs` but `&Rhs` as its second argument.
/// So, I think the third suggestion is possible.
/// The prototype of trait PartialEq is as follows:
/// 
/// ```text
/// pub trait PartialEq<Rhs = Self>
/// where
/// Rhs: ?Sized,
/// {
///     // Required method
///     fn eq(&self, other: &Rhs) -> bool;
/// 
///     // Provided method
///     fn ne(&self, other: &Rhs) -> bool { ... }
/// }
/// ```
impl<T, const N: usize> SubAssign<T> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // fn sub_assign(&mut self, rhs: T)
    /// Calculates `self` - `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and assigns the subtraction result `self` - `rhs` to `self` back.
    /// 
    /// # Arguments
    /// `rhs` is to be subtracted from `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - Wrapping (modular) subtraction.
    /// - If underflow happened, the flag `UNDERFLOW` of `self` will be set.
    /// - All the flags are historical, which means, for example, if an underflow
    ///   occurred even once before this current operation or `UNDERFLOW`
    ///   flag is already set before this current operation, the `UNDERFLOW` flag
    ///   is not changed even if this current operation does not cause underflow.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U512::one();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// 
    /// let one_uint = 1_u32;
    /// a_biguint -= one_uint;
    /// println!("After a_biguint -= {}, a_biguint = {}", one_uint, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U512::one();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// 
    /// let two_uint = 2_u32;
    /// a_biguint -= two_uint;
    /// println!("After a_biguint -= {}, a_biguint = {}", two_uint, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    /// assert_eq!(a_biguint.is_underflow(), true);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U512::one();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// 
    /// let three_uint = 3_u32;
    /// a_biguint -= three_uint;
    /// println!("After a_biguint -= {}, a_biguint = {}", three_uint, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    /// assert_eq!(a_biguint.is_underflow(), true);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    fn sub_assign(&mut self, rhs: T)
    {
        self.wrapping_sub_assign_uint(rhs);
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `*` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after multiplication
/// operation. In order to prevent this, the operands should be cloned or
/// copied before multiplication operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait Mul<Rhs = Self> {
///     type Output;
///     // Required method
///     fn mul(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait Mul<Rhs = Self> {
///     type Output;
///     // Required method
///     fn mul(self, rhs: Rhs) -> Self::Output;
///   or
///     fn mul(&self, rhs: Rhs) -> Self::Output;
///   or
///     fn mul(self, rhs: &Rhs) -> Self::Output;
///   or
///     fn mul(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait Mul2, Mul3, and Mul4 are provided and the developer
/// implements none or only one of traits Mul, Mul2, Mul3, and Mul4.
/// ```
/// pub trait Mul<Rhs = Self> {
///     type Output;
///     // Required method
///     fn mul(self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait Mul2<Rhs = Self> {
///     type Output;
///     // Required method
///     fn mul(&self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait Mul3<Rhs = Self> {
///     type Output;
///     // Required method
///     fn mul(self, rhs: &Rhs) -> Self::Output;
/// }
/// 
/// pub trait Mul4<Rhs = Self> {
///     type Output;
///     // Required method
///     fn mul(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// Unlike trait Mul, the trait PartialEq makes the operators `==` and `!=` take
/// not `&Self` but `Self` as its first operand and not `&Rhs` (or `&Self`) but
/// `Rhs` (or `Self`) as its second operand but makes the functions eq() and
/// ne() take not `self` but `&self` as its first argument and not `Rhs` but
/// `&Rhs` as its second argument. So, I think the third suggestion is possible.
/// The prototype of trait PartialEq is as follows:
/// 
/// ```text
/// pub trait PartialEq<Rhs = Self>
/// where
/// Rhs: ?Sized,
/// {
///     // Required method
///     fn eq(&self, other: &Rhs) -> bool;
/// 
///     // Provided method
///     fn ne(&self, other: &Rhs) -> bool { ... }
/// }
/// ```
impl<T, const N: usize> Mul for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    // fn mul(self, rhs: Self) -> Self
    /// Calculates `self` * `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and returns a multiplication result `self` * `rhs`.
    /// 
    /// # Arguments
    /// `rhs` is to be added to `self`, and is of `Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the multiplication result `self` * `rhs` with wrapping
    /// (modular) multiplication.
    /// 
    /// # Features
    /// - Wrapping (modular) multiplication.
    /// - If overflow happened, the flag `OVERFLOW` of the return value
    ///   will be set.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    /// let b_biguint = U256::from_uint(248_u8);
    /// let res = a_biguint.clone() * b_biguint.clone();
    /// println!("{} X {} = {}", a_biguint, b_biguint, res);
    /// assert_eq!(res.to_string(), "3070286482594539205006712790083024854061173096293736274934671554495350784");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_biguint = U256::from_uint(248_u8);
    /// let res = a_biguint.clone() * b_biguint.clone();
    /// println!("{} X {} = {}", a_biguint, b_biguint, res);
    /// assert_eq!(res.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    /// assert_eq!(res.is_overflow(), true);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// ```
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    /// let b_biguint = U256::from_uint(248_u8);
    /// let _res = a_biguint * b_biguint;
    /// println!("{} X {} = {}", a_biguint, b_biguint, res);
    /// // The operator '*' swallowed (took the ownership of) a_biguint and b_biguint.
    /// 
    /// let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_biguint = U256::from_uint(248_u8);
    /// let _res = a_biguint * b_biguint;
    /// println!("{} X {} = {}", a_biguint, b_biguint, res);
    /// // The operator '*' swallowed (took the ownership of) a_biguint and b_biguint.
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    fn mul(self, rhs: Self) -> Self
    {
        self.wrapping_mul(&rhs)
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `*` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after multiplication
/// operation. In order to prevent this, the operands should be cloned or
/// copied before multiplication operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait Mul<Rhs = Self> {
///     type Output;
///     // Required method
///     fn mul(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait Mul<Rhs = Self> {
///     type Output;
///     // Required method
///     fn mul(self, rhs: Rhs) -> Self::Output;
///   or
///     fn mul(&self, rhs: Rhs) -> Self::Output;
///   or
///     fn mul(self, rhs: &Rhs) -> Self::Output;
///   or
///     fn mul(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait Mul2, Mul3, and Mul4 are provided and the developer
/// implements none or only one of traits Mul, Mul2, Mul3, and Mul4.
/// ```
/// pub trait Mul<Rhs = Self> {
///     type Output;
///     // Required method
///     fn mul(self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait Mul2<Rhs = Self> {
///     type Output;
///     // Required method
///     fn mul(&self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait Mul3<Rhs = Self> {
///     type Output;
///     // Required method
///     fn mul(self, rhs: &Rhs) -> Self::Output;
/// }
/// 
/// pub trait Mul4<Rhs = Self> {
///     type Output;
///     // Required method
///     fn mul(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// Unlike trait Mul, the trait PartialEq makes the operators `==` and `!=` take
/// not `&Self` but `Self` as its first operand and not `&Rhs` (or `&Self`) but
/// `Rhs` (or `Self`) as its second operand but makes the functions eq() and
/// ne() take not `self` but `&self` as its first argument and not `Rhs` but
/// `&Rhs` as its second argument. So, I think the third suggestion is possible.
/// The prototype of trait PartialEq is as follows:
/// 
/// ```text
/// pub trait PartialEq<Rhs = Self>
/// where
/// Rhs: ?Sized,
/// {
///     // Required method
///     fn eq(&self, other: &Rhs) -> bool;
/// 
///     // Provided method
///     fn ne(&self, other: &Rhs) -> bool { ... }
/// }
/// ```
impl<T, const N: usize> Mul<T> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    // fn mul(self, rhs: T) -> Self
    /// Calculates `self` * `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and returns a multiplication result `self` * `rhs`.
    /// 
    /// # Arguments
    /// `rhs` is to be multiplied to `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the multiplication result `self` * `rhs` with wrapping
    /// (modular) multiplication.
    /// 
    /// # Features
    /// - Wrapping (modular) multiplication.
    /// - If overflow happened, the flag `OVERFLOW` of the return value
    ///   will be set.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    /// let b_uint = 248_u128;
    /// let res = a_biguint.clone() * b_uint;
    /// println!("{} X {} = {}", a_biguint, b_uint, res);
    /// assert_eq!(res.to_string(), "3070286482594539205006712790083024854061173096293736274934671554495350784");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_uint = 248_u128;
    /// let res = a_biguint.clone() * b_uint;
    /// println!("{} X {} = {}", a_biguint, b_uint, res);
    /// assert_eq!(res.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    /// assert_eq!(res.is_overflow(), true);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// ```
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    /// let b_uint = 248_u128;
    /// let _res = a_biguint * b_uint;
    /// println!("{} X {} = {}", a_biguint, b_uint, _res);
    /// // The operator '*' swallowed (took the ownership of) a_biguint.
    /// 
    /// let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_uint = 248_u128;
    /// let _res = a_biguint * b_uint;
    /// println!("{} X {} = {}", b_biguint, b_uint, _res);
    /// // The operator '*' swallowed (took the ownership of) a_biguint.
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    fn mul(self, rhs: T) -> Self
    {
        self.wrapping_mul_uint(rhs)
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `*=` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after mutltiplication
/// operation. In order to prevent this, the operands should be cloned or
/// copied before mutltiplication operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait MulAssign<Rhs = Self> {
///     // Required method
///     fn mul_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait MulAssign<Rhs = Self> {
///     // Required method
///     fn mul_assign(&mut self, rhs: Rhs);
///   or
///     fn mul_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait MulAssign2 is provided and the developer
/// implements none or only one of traits MulAssign and MulAssign2.
/// ```
/// pub trait MulAssign<Rhs = Self> {
///     // Required method
///     fn mul_assign(&mut self, rhs: Rhs);
/// }
/// 
/// pub trait MulAssign2<Rhs = Self> {
///     // Required method
///     fn mul_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// Unlike trait MulAssign, the trait PartialEq makes the operators
/// `==` and `!=` take not `&Self` but `Self` as its first operand and not
/// `&Rhs` (or `&Self`) but `Rhs` (or `Self`) as its second operand but makes
/// the functions eq() and ne() take not `self` but `&self` as its first
/// argument and not `Rhs` but `&Rhs` as its second argument.
/// So, I think the third suggestion is possible.
/// The prototype of trait PartialEq is as follows:
/// 
/// ```text
/// pub trait PartialEq<Rhs = Self>
/// where
/// Rhs: ?Sized,
/// {
///     // Required method
///     fn eq(&self, other: &Rhs) -> bool;
/// 
///     // Provided method
///     fn ne(&self, other: &Rhs) -> bool { ... }
/// }
/// ```
impl<T, const N: usize> MulAssign for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // fn mul_assign(&mut self, rhs: Self)
    /// Calculates `self` * `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and assigns a multiplication result `self` * `rhs` to `self` back.
    /// 
    /// # Arguments
    /// `rhs` is to be added to `self`, and is of `Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - Wrapping (modular) multiplication.
    /// - If overflow happened, the flag `OVERFLOW` of `self` will be set.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = UU32::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// 
    /// let b_biguint = U256::from_uint(248_u8);
    /// a_biguint *= b_biguint.clone();
    /// println!("After a_biguint *= {}, a_biguint = {}", b_biguint, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "3070286482594539205006712790083024854061173096293736274934671554495350784");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = UU32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// 
    /// let b_biguint = U256::from_uint(248_u8);
    /// a_biguint *= b_biguint.clone();
    /// println!("After a_biguint *= {}, a_biguint = {}", b_biguint, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// ```
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U256::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// let b_biguint = U256::from_uint(248_u8);
    /// a_biguint *= b_biguint;
    /// println!("After a_biguint *= {}, a_biguint = {}", b_biguint, a_biguint);
    /// // The operator '*' swallowed (took the ownership of) a_biguint and b_biguint.
    /// 
    /// let mut a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// let b_biguint = U256::from_uint(248_u8);
    /// a_biguint *= b_biguint;
    /// println!("After a_biguint *= {}, a_biguint = {}", b_biguint, a_biguint);
    /// // The operator '*' swallowed (took the ownership of) a_biguint and b_biguint.
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline] 
    fn mul_assign(&mut self, rhs: Self)
    {
        self.wrapping_mul_assign(&rhs);
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `*=` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after mutltiplication
/// operation. In order to prevent this, the operands should be cloned or
/// copied before mutltiplication operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait MulAssign<Rhs = Self> {
///     // Required method
///     fn mul_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait MulAssign<Rhs = Self> {
///     // Required method
///     fn mul_assign(&mut self, rhs: Rhs);
///   or
///     fn mul_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait MulAssign2 is provided and the developer
/// implements none or only one of traits MulAssign and MulAssign2.
/// 
/// ```
/// pub trait MulAssign<Rhs = Self> {
///     // Required method
///     fn mul_assign(&mut self, rhs: Rhs);
/// }
/// 
/// pub trait MulAssign2<Rhs = Self> {
///     // Required method
///     fn mul_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// Unlike trait MulAssign, the trait PartialEq makes the operators
/// `==` and `!=` take not `&Self` but `Self` as its first operand and not
/// `&Rhs` (or `&Self`) but `Rhs` (or `Self`) as its second operand but makes
/// the functions eq() and ne() take not `self` but `&self` as its first
/// argument and not `Rhs` but `&Rhs` as its second argument.
/// So, I think the third suggestion is possible.
/// The prototype of trait PartialEq is as follows:
/// 
/// ```text
/// pub trait PartialEq<Rhs = Self>
/// where
/// Rhs: ?Sized,
/// {
///     // Required method
///     fn eq(&self, other: &Rhs) -> bool;
/// 
///     // Provided method
///     fn ne(&self, other: &Rhs) -> bool { ... }
/// }
/// ```
impl<T, const N: usize> MulAssign<T> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // fn mul_assign(&mut self, rhs: T)
    /// Calculates `self` * `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and assigns a multiplication result `self` * `rhs` to `self` back.
    /// 
    /// # Arguments
    /// `rhs` is to be multiplied to `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - Wrapping (modular) multiplication.
    /// - If overflow happened, the flag `OVERFLOW` of `self` will be set.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = UU32::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "12380187429816690342769003185818648605085375388281194656994643364900608");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// 
    /// let b_uint = 248_u16;
    /// a_biguint *= b_uint;
    /// println!("After a_biguint *= {}, a_biguint = {}", b_uint, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "3070286482594539205006712790083024854061173096293736274934671554495350784");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = UU32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "876801874298166903427690031858186486050853753882811946569946433649006084094");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// 
    /// let b_uint = 248_u16;
    /// a_biguint *= b_uint;
    /// println!("After a_biguint *= {}, a_biguint = {}", b_uint, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    fn mul_assign(&mut self, rhs: T)
    {
        self.wrapping_mul_assign_uint(rhs);
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `/` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after division
/// operation. In order to prevent this, the operands should be cloned or
/// copied before division operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait Div<Rhs = Self> {
///     type Output;
///     // Required method
///     fn div(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait Div<Rhs = Self> {
///     type Output;
///     // Required method
///     fn div(self, rhs: Rhs) -> Self::Output;
///   or
///     fn div(&self, rhs: Rhs) -> Self::Output;
///   or
///     fn div(self, rhs: &Rhs) -> Self::Output;
///   or
///     fn div(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait Div2, Div3, and Div4 are provided and the developer
/// implements none or only one of traits Div, Div2, Div3, and Div4.
/// 
/// ```
/// pub trait Div<Rhs = Self> {
///     type Output;
///     // Required method
///     fn div(self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait Div2<Rhs = Self> {
///     type Output;
///     // Required method
///     fn div(&self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait Div3<Rhs = Self> {
///     type Output;
///     // Required method
///     fn div(self, rhs: &Rhs) -> Self::Output;
/// }
/// 
/// pub trait Div4<Rhs = Self> {
///     type Output;
///     // Required method
///     fn div(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// Unlike trait Div, the trait PartialEq makes the operators `==` and `!=` take
/// not `&Self` but `Self` as its first operand and not `&Rhs` (or `&Self`) but
/// `Rhs` (or `Self`) as its second operand but makes the functions eq() and
/// ne() take not `self` but `&self` as its first argument and not `Rhs` but
/// `&Rhs` as its second argument. So, I think the third suggestion is possible.
/// The prototype of trait PartialEq is as follows:
/// 
/// ```text
/// pub trait PartialEq<Rhs = Self>
/// where
/// Rhs: ?Sized,
/// {
///     // Required method
///     fn eq(&self, other: &Rhs) -> bool;
/// 
///     // Provided method
///     fn ne(&self, other: &Rhs) -> bool { ... }
/// }
/// ```
impl<T, const N: usize> Div for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    // fn div(self, rhs: Self) -> Self
    /// Divides `self` by `rhs`, and returns the quotient.
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of `Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `rhs` is zero, this method will panic.
    ///
    /// # Output
    /// It returns a quotient of `BigUInt` type,
    /// and the quotient would never overflow. 
    /// 
    /// # Features
    /// - Wrapped division on `BigUInt` types is just normal division.
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// - If `rhs` is zero, this method will panic.
    /// - This function exists, so that all operations are accounted for
    ///   in the wrapping operations.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = U256::from_uint(87_u8);
    /// let quotient = dividend.clone() / divisor.clone();
    /// println!("{} / {} = {}", dividend, divisor, quotient);
    /// assert_eq!(quotient.to_string(), "1419043551905275201680884938348044216837079832");
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_underflow(), false);
    /// assert_eq!(quotient.is_infinity(), false);
    /// assert_eq!(quotient.is_undefined(), false);
    /// assert_eq!(quotient.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = U256::zero();
    /// let divisor = U256::from_uint(87_u8);
    /// let quotient = dividend.clone() / divisor.clone();
    /// println!("{} / {} = {}", dividend, divisor, quotient);
    /// assert_eq!(quotient.to_string(), "0");
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_underflow(), false);
    /// assert_eq!(quotient.is_infinity(), false);
    /// assert_eq!(quotient.is_undefined(), false);
    /// assert_eq!(quotient.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = U256::zero();
    /// // It will panic!
    /// // let quotient = _dividend.clone() / _divisor.clone();
    /// 
    /// let _dividend = U256::zero();
    /// let _divisor = U256::zero();
    /// // It will panic!
    /// // let quotient = _dividend.clone() / _divisor.clone();
    /// ```
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = U256::from_uint(87_u8);
    /// let _quotient = dividend / divisor;
    /// // It cannot be compiled!
    /// println!("{} / {} = {}", dividend, divisor, _quotient);
    /// // The operator '/' swallowed (took the ownership of) dividend and divisor.
    /// 
    /// let dividend = U256::zero();
    /// let divisor = U256::from_uint(87_u8);
    /// let _quotient = dividend / divisor;
    /// // It cannot be compiled!
    /// println!("{} / {} = {}", dividend, divisor, _quotient);
    /// // The operator '/' swallowed (took the ownership of) dividend and divisor.
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    fn div(self, rhs: Self) -> Self
    {
        self.wrapping_div(&rhs)
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `/` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after division
/// operation. In order to prevent this, the operands should be cloned or
/// copied before division operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait Div<Rhs = Self> {
///     type Output;
///     // Required method
///     fn div(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait Div<Rhs = Self> {
///     type Output;
///     // Required method
///     fn div(self, rhs: Rhs) -> Self::Output;
///   or
///     fn div(&self, rhs: Rhs) -> Self::Output;
///   or
///     fn div(self, rhs: &Rhs) -> Self::Output;
///   or
///     fn div(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait Div2, Div3, and Div4 are provided and the developer
/// implements none or only one of traits Div, Div2, Div3, and Div4.
/// 
/// ```
/// pub trait Div<Rhs = Self> {
///     type Output;
///     // Required method
///     fn div(self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait Div2<Rhs = Self> {
///     type Output;
///     // Required method
///     fn div(&self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait Div3<Rhs = Self> {
///     type Output;
///     // Required method
///     fn div(self, rhs: &Rhs) -> Self::Output;
/// }
/// 
/// pub trait Div4<Rhs = Self> {
///     type Output;
///     // Required method
///     fn div(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// Unlike trait Div, the trait PartialEq makes the operators `==` and `!=` take
/// not `&Self` but `Self` as its first operand and not `&Rhs` (or `&Self`) but
/// `Rhs` (or `Self`) as its second operand but makes the functions eq() and
/// ne() take not `self` but `&self` as its first argument and not `Rhs` but
/// `&Rhs` as its second argument. So, I think the third suggestion is possible.
/// The prototype of trait PartialEq is as follows:
/// 
/// ```text
/// pub trait PartialEq<Rhs = Self>
/// where
/// Rhs: ?Sized,
/// {
///     // Required method
///     fn eq(&self, other: &Rhs) -> bool;
/// 
///     // Provided method
///     fn ne(&self, other: &Rhs) -> bool { ... }
/// }
/// ```
impl<T, const N: usize> Div<T> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    // fn div(self, rhs: T) -> Self
    /// Divides `self` by `rhs`, and returns the quotient.
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of primitive unsigned integral data type
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `rhs` is zero, this method will panic.
    ///
    /// # Output
    /// It returns a quotient of `BigUInt` type,
    /// and the quotient would never overflow.
    /// 
    /// # Features
    /// - Wrapped division on `BigUInt` types is just normal division.
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// - If `rhs` is zero, this method will panic.
    /// - This function exists, so that all operations are accounted for
    ///   in the wrapping operations.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 87_u64;
    /// let quotient = dividend.clone() / divisor;
    /// println!("{} / {} = {}", dividend, divisor, quotient);
    /// assert_eq!(quotient.to_string(), "1419043551905275201680884938348044216837079832");
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_underflow(), false);
    /// assert_eq!(quotient.is_infinity(), false);
    /// assert_eq!(quotient.is_undefined(), false);
    /// assert_eq!(quotient.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let dividend = U256::zero();
    /// let divisor = 87_u64;
    /// let quotient = dividend.clone() / divisor;
    /// println!("{} / {} = {}", dividend, divisor, quotient);
    /// assert_eq!(quotient.to_string(), "0");
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_underflow(), false);
    /// assert_eq!(quotient.is_infinity(), false);
    /// assert_eq!(quotient.is_undefined(), false);
    /// assert_eq!(quotient.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = 0_u64;
    /// // It will panic!
    /// let quotient = _dividend.clone() / _divisor;
    /// 
    /// let _dividend = U256::zero();
    /// let _divisor = 0_u64;
    /// // It will panic!
    /// let quotient = _dividend.clone() / _divisor;
    /// ```
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 87_u64;
    /// let _quotient = dividend / divisor;
    /// // It cannot be compiled!
    /// println!("{} / {} = {}", dividend, divisor, _quotient);
    /// // The operator '/' swallowed (took the ownership of) dividend.
    /// 
    /// let dividend = U256::zero();
    /// let divisor = 87_u64;
    /// let _quotient = dividend / divisor;
    /// // It cannot be compiled!
    /// println!("{} / {} = {}", dividend, divisor, _quotient);
    /// // The operator '/' swallowed (took the ownership of) dividend.
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    fn div(self, rhs: T) -> Self
    {
        self.wrapping_div_uint(rhs)
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `/=` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after division
/// operation. In order to prevent this, the operands should be cloned or
/// copied before division operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait DivAssign<Rhs = Self> {
///     // Required method
///     fn div_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait DivAssign<Rhs = Self> {
///     // Required method
///     fn div_assign(&mut self, rhs: Rhs);
///   or
///     fn div_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait DivAssign2 is provided and the developer
/// implements none or only one of traits DivAssign and DivAssign2.
/// 
/// ```
/// pub trait DivAssign<Rhs = Self> {
///     // Required method
///     fn div_assign(&mut self, rhs: Rhs);
/// }
/// 
/// pub trait DivAssign2<Rhs = Self> {
///     // Required method
///     fn div_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// Unlike trait DivAssign, the trait PartialEq makes the operators
/// `==` and `!=` take not `&Self` but `Self` as its first operand and not
/// `&Rhs` (or `&Self`) but `Rhs` (or `Self`) as its second operand but makes
/// the functions eq() and ne() take not `self` but `&self` as its first
/// argument and not `Rhs` but `&Rhs` as its second argument.
/// So, I think the third suggestion is possible.
/// The prototype of trait PartialEq is as follows:
/// 
/// ```text
/// pub trait PartialEq<Rhs = Self>
/// where
/// Rhs: ?Sized,
/// {
///     // Required method
///     fn eq(&self, other: &Rhs) -> bool;
/// 
///     // Provided method
///     fn ne(&self, other: &Rhs) -> bool { ... }
/// }
/// ```
impl<T, const N: usize> DivAssign for BigUInt<T, N>
where T: SmallUInt + Copy + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // fn div_assign(&mut self, rhs: Self)
    /// Divides `self` by `rhs`, and assigns the quotient to `self` back.
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of `Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `rhs` is zero, this method will panic.
    /// 
    /// # Features
    /// - Wrapped division on `BigUInt` types is just normal division.
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// - If `rhs` is zero, this method will panic.
    /// - This function exists, so that all operations are accounted for
    ///   in the wrapping operations.
    /// - All the flags are historical, which means, for example, if an
    ///   divided_by_zero occurred even once before this current operation or
    ///   `DIVIDED_BY_ZERO` flag is already set before this current operation,
    ///   the `DIVIDED_BY_ZERO` flag is not changed even if this current operation
    ///   does not cause divided_by_zero.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let divisor = UU32::from_uint(87_u8);
    /// a_biguint /= divisor.clone();
    /// println!("After a_biguint /= {}, a_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1419043551905275201680884938348044216837079832");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = UU32::zero();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let divisor = UU32::from_uint(87_u8);
    /// a_biguint /= divisor.clone();
    /// println!("After a_biguint /= {}, a_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut _a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// let _divisor = UU32::zero();
    /// // It will panic!
    /// // _a_biguint /= _divisor.clone();
    /// 
    /// let mut _a_biguint = UU32::zero();
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// let _divisor = UU32::zero();
    /// // It will panic!
    /// // _a_biguint /= _divisor.clone();
    /// ```
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// let divisor = UU32::from_uint(87_u8);
    /// a_biguint /= divisor;
    /// // It cannot be compiled!
    /// println!("After a_biguint /= {}, a_biguint = {}", divisor, a_biguint);
    /// // The operator '/=' swallowed (took the ownership of) divisor.
    /// 
    /// let mut a_biguint = UU32::zero();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// let divisor = UU32::from_uint(87_u8);
    /// a_biguint /= divisor;
    /// // It cannot be compiled!
    /// println!("After a_biguint /= {}, a_biguint = {}", divisor, a_biguint);
    /// // The operator '/=' swallowed (took the ownership of) divisor.
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    fn div_assign(&mut self, rhs: Self)
    {
        self.wrapping_div_assign(&rhs);
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `/=` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after division
/// operation. In order to prevent this, the operands should be cloned or
/// copied before division operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait DivAssign<Rhs = Self> {
///     // Required method
///     fn div_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait DivAssign<Rhs = Self> {
///     // Required method
///     fn div_assign(&mut self, rhs: Rhs);
///   or
///     fn div_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait DivAssign2 is provided and the developer
/// implements none or only one of traits DivAssign and DivAssign2.
/// 
/// ```
/// pub trait DivAssign<Rhs = Self> {
///     // Required method
///     fn div_assign(&mut self, rhs: Rhs);
/// }
/// 
/// pub trait DivAssign2<Rhs = Self> {
///     // Required method
///     fn div_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// Unlike trait DivAssign, the trait PartialEq makes the operators
/// `==` and `!=` take not `&Self` but `Self` as its first operand and not
/// `&Rhs` (or `&Self`) but `Rhs` (or `Self`) as its second operand but makes
/// the functions eq() and ne() take not `self` but `&self` as its first
/// argument and not `Rhs` but `&Rhs` as its second argument.
/// So, I think the third suggestion is possible.
/// The prototype of trait PartialEq is as follows:
/// 
/// ```text
/// pub trait PartialEq<Rhs = Self>
/// where
/// Rhs: ?Sized,
/// {
///     // Required method
///     fn eq(&self, other: &Rhs) -> bool;
/// 
///     // Provided method
///     fn ne(&self, other: &Rhs) -> bool { ... }
/// }
/// ```
impl<T, const N: usize> DivAssign<T> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // fn div_assign(&mut self, rhs: T)
    /// Divides `self` by `rhs`, and assigns the quotient to `self` back..
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of primitive unsigned integral data type
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `rhs` is zero, this method will panic.
    /// 
    /// # Features
    /// - Wrapped division on `BigUInt` types is just normal division.
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// - If `rhs` is zero, this method will panic.
    /// - This function exists, so that all operations are accounted for
    ///   in the wrapping operations.
    /// - All the flags are historical, which means, for example, if an
    ///   divided_by_zero occurred even once before this current operation or
    ///   `DIVIDED_BY_ZERO` flag is already set before this current operation,
    ///   the `DIVIDED_BY_ZERO` flag is not changed even if this current operation
    ///   does not cause divided_by_zero.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 87_u8;
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// 
    /// a_biguint.wrapping_div_assign_uint(divisor);
    /// println!("After a_biguint.wrapping_div_assign_uint(&divisor),\na_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = UU32::zero();
    /// let divisor = 87_u8;
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// 
    /// a_biguint.wrapping_div_assign_uint(divisor);
    /// println!("After a_biguint.wrapping_div_assign_uint(&divisor),\na_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// ```
    /// 
    /// # Panic Exmaples
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut _a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = 0_u8;
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// // It will panic!
    /// // a_biguint.wrapping_div_assign_uint(_divisor);
    /// 
    /// let mut _a_biguint = UU32::zero();
    /// let _divisor = 0_u8;
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// // It will panic!
    /// // a_biguint.wrapping_div_assign_uint(_divisor);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    fn div_assign(&mut self, rhs: T)
    {
        self.wrapping_div_assign_uint(rhs);
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `%` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after division
/// operation. In order to prevent this, the operands should be cloned or
/// copied before division operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait Rem<Rhs = Self> {
///     type Output;
///     // Required method
///     fn rem(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait Rem<Rhs = Self> {
///     type Output;
///     // Required method
///     fn rem(self, rhs: Rhs) -> Self::Output;
///   or
///     fn rem(&self, rhs: Rhs) -> Self::Output;
///   or
///     fn rem(self, rhs: &Rhs) -> Self::Output;
///   or
///     fn rem(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait Rem2, Rem3, and Rem4 are provided and the developer
/// implements none or only one of traits Rem, Rem2, Rem3, and Rem4.
/// 
/// ```
/// pub trait Rem<Rhs = Self> {
///     type Output;
///     // Required method
///     fn rem(self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait Rem2<Rhs = Self> {
///     type Output;
///     // Required method
///     fn rem(&self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait Rem3<Rhs = Self> {
///     type Output;
///     // Required method
///     fn rem(self, rhs: &Rhs) -> Self::Output;
/// }
/// 
/// pub trait Rem4<Rhs = Self> {
///     type Output;
///     // Required method
///     fn rem(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// Unlike trait Rem, the trait PartialEq makes the operators `==` and `!=` take
/// not `&Self` but `Self` as its first operand and not `&Rhs` (or `&Self`) but
/// `Rhs` (or `Self`) as its second operand but makes the functions eq() and
/// ne() take not `self` but `&self` as its first argument and not `Rhs` but
/// `&Rhs` as its second argument. So, I think the third suggestion is possible.
/// The prototype of trait PartialEq is as follows:
/// 
/// ```text
/// pub trait PartialEq<Rhs = Self>
/// where
/// Rhs: ?Sized,
/// {
///     // Required method
///     fn eq(&self, other: &Rhs) -> bool;
/// 
///     // Provided method
///     fn ne(&self, other: &Rhs) -> bool { ... }
/// }
/// ```
impl<T, const N: usize> Rem for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    // fn rem(self, rhs: Self) -> Self
    /// Divides `self` by `rhs`, and returns the remainder.
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of `Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `rhs` is zero, this method will panic.
    ///
    /// # Output
    /// It returns a remainder of `BigUInt` type,
    /// and the remainder would never overflow. 
    /// 
    /// # Features
    /// - Wrapped division on `BigUInt` types is just normal division.
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// - If `rhs` is zero, this method will panic.
    /// - This function exists, so that all operations are accounted for
    ///   in the wrapping operations.
    /// 
    /// # Counterpart Method
    /// The method
    /// [wrapping_rem_uint()](struct@BigUInt#method.wrapping_rem_uint)
    /// is a bit faster than this method `wrapping_rem()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [wrapping_rem_uint()](struct@BigUInt#method.wrapping_rem_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = UU32::from_uint(87_u8);
    /// let remainder = dividend.clone() % divisor.clone();
    /// println!("{} % {} = {}", dividend, divisor, remainder);
    /// assert_eq!(remainder.to_string(), "8");
    /// assert_eq!(remainder.is_overflow(), false);
    /// assert_eq!(remainder.is_underflow(), false);
    /// assert_eq!(remainder.is_infinity(), false);
    /// assert_eq!(remainder.is_undefined(), false);
    /// assert_eq!(remainder.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let dividend = UU32::zero();
    /// let divisor = UU32::from_uint(87_u8);
    /// let remainder = dividend.clone() % divisor.clone();
    /// println!("{} % {} = {}", dividend, divisor, remainder);
    /// assert_eq!(remainder.to_string(), "0");
    /// assert_eq!(remainder.is_overflow(), false);
    /// assert_eq!(remainder.is_underflow(), false);
    /// assert_eq!(remainder.is_infinity(), false);
    /// assert_eq!(remainder.is_undefined(), false);
    /// assert_eq!(remainder.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let _dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = UU32::zero();
    /// // It will panic!
    /// // let remainder = _dividend.wrapping_rem(&_divisor);
    /// 
    /// let _dividend = UU32::zero();
    /// let _divisor = UU32::zero();
    /// // It will panic!
    /// // let remainder = _dividend.wrapping_rem(&_divisor);
    /// ```
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = UU32::from_uint(87_u8);
    /// let _remainder = dividend % divisor;
    /// // It cannot be compiled!
    /// println!("{} % {} = {}", dividend, divisor, _remainder);
    /// // The operator '%' swallowed (took the ownership of) dividend and divisor.
    /// 
    /// let dividend = UU32::zero();
    /// let divisor = UU32::from_uint(87_u8);
    /// let _remainder = dividend % divisor;
    /// // It cannot be compiled!
    /// println!("{} % {} = {}", dividend, divisor, _remainder);
    /// // The operator '%' swallowed (took the ownership of) dividend and divisor.
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    fn rem(self, rhs: Self) -> Self
    {
        self.wrapping_rem(&rhs)
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `%` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after division
/// operation. In order to prevent this, the operands should be cloned or
/// copied before division operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait Rem<Rhs = Self> {
///     type Output;
///     // Required method
///     fn rem(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait Rem<Rhs = Self> {
///     type Output;
///     // Required method
///     fn rem(self, rhs: Rhs) -> Self::Output;
///   or
///     fn rem(&self, rhs: Rhs) -> Self::Output;
///   or
///     fn rem(self, rhs: &Rhs) -> Self::Output;
///   or
///     fn rem(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait Rem2, Rem3, and Rem4 are provided and the developer
/// implements none or only one of traits Rem, Rem2, Rem3, and Rem4.
/// 
/// ```
/// pub trait Rem<Rhs = Self> {
///     type Output;
///     // Required method
///     fn rem(self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait Rem2<Rhs = Self> {
///     type Output;
///     // Required method
///     fn rem(&self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait Rem3<Rhs = Self> {
///     type Output;
///     // Required method
///     fn rem(self, rhs: &Rhs) -> Self::Output;
/// }
/// 
/// pub trait Rem4<Rhs = Self> {
///     type Output;
///     // Required method
///     fn rem(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// Unlike trait Rem, the trait PartialEq makes the operators `==` and `!=` take
/// not `&Self` but `Self` as its first operand and not `&Rhs` (or `&Self`) but
/// `Rhs` (or `Self`) as its second operand but makes the functions eq() and
/// ne() take not `self` but `&self` as its first argument and not `Rhs` but
/// `&Rhs` as its second argument. So, I think the third suggestion is possible.
/// The prototype of trait PartialEq is as follows:
/// 
/// ```text
/// pub trait PartialEq<Rhs = Self>
/// where
/// Rhs: ?Sized,
/// {
///     // Required method
///     fn eq(&self, other: &Rhs) -> bool;
/// 
///     // Provided method
///     fn ne(&self, other: &Rhs) -> bool { ... }
/// }
/// ```
impl<T, const N: usize> Rem<T> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = T;

    // fn rem(self, rhs: T) -> T
    /// Divides `self` by `rhs`, and returns the remainder.
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of primitive unsigned integral data type
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `rhs` is zero, this method will panic.
    ///
    /// # Output
    /// It returns a remainder of `BigUInt` type,
    /// and the remainder would never overflow.
    /// 
    /// # Features
    /// - Wrapped division on `BigUInt` types is just normal division.
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// - If `rhs` is zero, this method will panic.
    /// - This function exists, so that all operations are accounted for
    ///   in the wrapping operations.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 87_u32;
    /// let remainder = dividend.clone() % divisor;
    /// println!("{} % {} = {}", dividend, divisor, remainder);
    /// assert_eq!(remainder.to_string(), "8");
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = UU32::zero();
    /// let divisor = 87_u32;
    /// let remainder = dividend.clone() % divisor;
    /// println!("{} % {} = {}", dividend, divisor, remainder);
    /// assert_eq!(remainder.to_string(), "0");
    /// ```
    /// 
    /// # Panic Examples
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let _dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = 0_u32;
    /// // It will panic!
    /// // let remainder = _dividend.clone() % _divisor;
    /// 
    /// let _dividend = UU32::zero();
    /// let _divisor = 0_u32;
    /// // It will panic!
    /// // let remainder = _dividend.clone() % _divisor;
    /// ```
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 87_u32;
    /// let _remainder = dividend % divisor;
    /// // It cannot be compiled!
    /// println!("{} % {} = {}", dividend, divisor, _remainder);
    /// // The operator '%' swallowed (took the ownership of) dividend.
    /// 
    /// let dividend = UU32::zero();
    /// let divisor = 87_u32;
    /// let _remainder = dividend % divisor;
    /// // It cannot be compiled!
    /// println!("{} % {} = {}", dividend, divisor, _remainder);
    /// // The operator '%' swallowed (took the ownership of) dividend.
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    fn rem(self, rhs: T) -> T
    {
        self.wrapping_rem_uint(rhs)
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `%=` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after division
/// operation. In order to prevent this, the operands should be cloned or
/// copied before division operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait RemAssign<Rhs = Self> {
///     // Required method
///     fn rem_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait RemAssign<Rhs = Self> {
///     // Required method
///     fn rem_assign(&mut self, rhs: Rhs);
///   or
///     fn rem_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait RemAssign2 is provided and the developer
/// implements none or only one of traits RemAssign and RemAssign2.
/// 
/// ```
/// pub trait RemAssign<Rhs = Self> {
///     // Required method
///     fn rem_assign(&mut self, rhs: Rhs);
/// }
/// 
/// pub trait RemAssign2<Rhs = Self> {
///     // Required method
///     fn rem_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// Unlike trait RemAssign, the trait PartialEq makes the operators
/// `==` and `!=` take not `&Self` but `Self` as its first operand and not
/// `&Rhs` (or `&Self`) but `Rhs` (or `Self`) as its second operand but makes
/// the functions eq() and ne() take not `self` but `&self` as its first
/// argument and not `Rhs` but `&Rhs` as its second argument.
/// So, I think the third suggestion is possible.
/// The prototype of trait PartialEq is as follows:
/// 
/// ```text
/// pub trait PartialEq<Rhs = Self>
/// where
/// Rhs: ?Sized,
/// {
///     // Required method
///     fn eq(&self, other: &Rhs) -> bool;
/// 
///     // Provided method
///     fn ne(&self, other: &Rhs) -> bool { ... }
/// }
/// ```
impl<T, const N: usize> RemAssign for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // fn rem_assign(&mut self, rhs: Self)
    /// Divides `self` by `rhs`, and assigns the remainder to `self` back.
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of `Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `rhs` is zero, this method will panic.
    /// 
    /// # Features
    /// - Wrapped division on `BigUInt` types is just normal division.
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// - If `rhs` is zero, this method will panic.
    /// - This function exists, so that all operations are accounted for
    ///   in the wrapping operations.
    /// - All the flags are historical, which means, for example, if an
    ///   divided_by_zero occurred even once before this current operation or
    ///   `DIVIDED_BY_ZERO` flag is already set before this current operation,
    ///   the `DIVIDED_BY_ZERO` flag is not changed even if this current operation
    ///   does not cause divided_by_zero.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let divisor = U256::from_uint(87_u8);
    /// a_biguint %= divisor.clone();
    /// println!("After a_biguint %= {}, a_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "8");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::zero();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let divisor = U256::from_uint(87_u8);
    /// a_biguint %= divisor.clone();
    /// println!("After a_biguint %= {}, a_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// println!("Originally, a_biguint = {}", _a_biguint);
    /// let _divisor = U256::zero();
    /// // It will panic!
    /// // _a_biguint %= _divisor.clone();
    /// 
    /// let mut _a_biguint = U256::zero();
    /// println!("Originally, a_biguint = {}", _a_biguint);
    /// let _divisor = U256::zero();
    /// // It will panic!
    /// // _a_biguint %= _divisor.clone();
    /// ```
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// let divisor = U256::from_uint(87_u8);
    /// a_biguint %= divisor;
    /// // It cannot be compiled!
    /// println!("After a_biguint =/ {}, a_biguint = {}", divisor, a_biguint);
    /// // The operator %= swallowed (took the ownership of) divisor.
    /// 
    /// let mut a_biguint = U256::zero();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// let divisor = U256::from_uint(87_u8);
    /// a_biguint %= divisor;
    /// // It cannot be compiled!
    /// println!("After a_biguint =/ {}, a_biguint = {}", divisor, a_biguint);
    /// // The operator %= swallowed (took the ownership of) divisor.
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    fn rem_assign(&mut self, rhs: Self)
    {
        self.wrapping_rem_assign(&rhs);
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `%=` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after division
/// operation. In order to prevent this, the operands should be cloned or
/// copied before division operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait RemAssign<Rhs = Self> {
///     // Required method
///     fn rem_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait RemAssign<Rhs = Self> {
///     // Required method
///     fn rem_assign(&mut self, rhs: Rhs);
///   or
///     fn rem_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait RemAssign2 is provided and the developer
/// implements none or only one of traits RemAssign and RemAssign2.
/// 
/// ```
/// pub trait RemAssign<Rhs = Self> {
///     // Required method
///     fn rem_assign(&mut self, rhs: Rhs);
/// }
/// 
/// pub trait RemAssign2<Rhs = Self> {
///     // Required method
///     fn rem_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// Unlike trait RemAssign, the trait PartialEq makes the operators
/// `==` and `!=` take not `&Self` but `Self` as its first operand and not
/// `&Rhs` (or `&Self`) but `Rhs` (or `Self`) as its second operand but makes
/// the functions eq() and ne() take not `self` but `&self` as its first
/// argument and not `Rhs` but `&Rhs` as its second argument.
/// So, I think the third suggestion is possible.
/// The prototype of trait PartialEq is as follows:
/// 
/// ```text
/// pub trait PartialEq<Rhs = Self>
/// where
/// Rhs: ?Sized,
/// {
///     // Required method
///     fn eq(&self, other: &Rhs) -> bool;
/// 
///     // Provided method
///     fn ne(&self, other: &Rhs) -> bool { ... }
/// }
/// ```
impl<T, const N: usize> RemAssign<T> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // fn rem_assign(&mut self, rhs: T)
    /// Divides `self` by `rhs`, and assigns the remainder to `self` back..
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of primitive unsigned integral data type
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `rhs` is zero, this method will panic.
    /// 
    /// # Features
    /// - Wrapped division on `BigUInt` types is just normal division.
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// - If `rhs` is zero, this method will panic.
    /// - This function exists, so that all operations are accounted for
    ///   in the wrapping operations.
    /// - All the flags are historical, which means, for example, if an
    ///   divided_by_zero occurred even once before this current operation or
    ///   `DIVIDED_BY_ZERO` flag is already set before this current operation,
    ///   the `DIVIDED_BY_ZERO` flag is not changed even if this current operation
    ///   does not cause divided_by_zero.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let divisor = 87_u128;
    /// a_biguint %= divisor;
    /// println!("After a_biguint %= {}, a_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "8");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::zero();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let divisor = 87_u128;
    /// a_biguint %= divisor;
    /// println!("After a_biguint %= {}, a_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// println!("Originally, a_biguint = {}", _a_biguint);
    /// let _divisor = 0_u128;
    /// // It will panic!
    /// // _a_biguint %= _divisor;
    /// 
    /// let mut _a_biguint = U256::zero();
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// let _divisor = 0_u128;
    /// // It will panic!
    /// // _a_biguint %= _divisor;
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    fn rem_assign(&mut self, rhs: T)
    {
        self.wrapping_rem_assign_uint(rhs);
    }
}



macro_rules! shl_for_BigUInt_impl {
    ($f:ty) => {
        /// I would like to suggest the modification of Rust grammar because the
        /// operator `<<` swallows (takes the ownership of) two operands which are
        /// left-hand side operand `self` and right-hand side operand `rhs` so that
        /// the two operands `self` and `rhs` cannot be used again after shift-left
        /// operation. In order to prevent this, the operands should be cloned or
        /// copied before shift-left operation. This adds the unnecessary overhead.
        /// The heavier the operand object is, the more the overhead is.
        /// 
        /// So, I would like to suggest one of the following three as follows:
        /// 
        /// # First suggestion
        /// Changing the types of the parameters as follows:
        /// 
        /// ```text
        /// pub trait Shl<Rhs = Self> {
        ///     type Output;
        ///     // Required method
        ///     fn shl(&self, rhs: &Rhs) -> Self::Output;
        /// }
        /// ```
        /// 
        /// # Second suggestion
        /// If the first suggestion is impossible because of backward compatibility,
        /// grammar allows the developer to choose the types of parameters but make
        /// only one function.
        /// 
        /// ```text
        /// pub trait Shl<Rhs = Self> {
        ///     type Output;
        ///     // Required method
        ///     fn shl(self, rhs: Rhs) -> Self::Output;
        ///   or
        ///     fn shl(&self, rhs: Rhs) -> Self::Output;
        ///   or
        ///     fn shl(self, rhs: &Rhs) -> Self::Output;
        ///   or
        ///     fn shl(&self, rhs: &Rhs) -> Self::Output;
        /// }
        /// ```
        /// 
        /// # Third suggestion
        /// If the first and second suggestions are impossible because of backward
        /// compatibility, trait Shl2, Shl3, and Shl4 are provided and the developer
        /// implements none or only one of traits Shl, Shl2, Shl3, and Shl4.
        /// 
        /// ```
        /// pub trait Shl<Rhs = Self> {
        ///     type Output;
        ///     // Required method
        ///     fn shl(self, rhs: Rhs) -> Self::Output;
        /// }
        /// 
        /// pub trait Shl2<Rhs = Self> {
        ///     type Output;
        ///     // Required method
        ///     fn shl(&self, rhs: Rhs) -> Self::Output;
        /// }
        /// 
        /// pub trait Shl3<Rhs = Self> {
        ///     type Output;
        ///     // Required method
        ///     fn shl(self, rhs: &Rhs) -> Self::Output;
        /// }
        /// 
        /// pub trait Shl4<Rhs = Self> {
        ///     type Output;
        ///     // Required method
        ///     fn shl(&self, rhs: &Rhs) -> Self::Output;
        /// }
        /// ```
        /// 
        /// Unlike trait Shl, the trait PartialEq makes the operators `==` and `!=` take
        /// not `&Self` but `Self` as its first operand and not `&Rhs` (or `&Self`) but
        /// `Rhs` (or `Self`) as its second operand but makes the functions eq() and
        /// ne() take not `self` but `&self` as its first argument and not `Rhs` but
        /// `&Rhs` as its second argument. So, I think the third suggestion is possible.
        /// The prototype of trait PartialEq is as follows:
        /// 
        /// ```text
        /// pub trait PartialEq<Rhs = Self>
        /// where
        /// Rhs: ?Sized,
        /// {
        ///     // Required method
        ///     fn eq(&self, other: &Rhs) -> bool;
        /// 
        ///     // Provided method
        ///     fn ne(&self, other: &Rhs) -> bool { ... }
        /// }
        /// ```
        impl<T, const N: usize> Shl<$f> for BigUInt<T, N>
        where T: SmallUInt + Copy + Clone + Display + Debug + ToString
                + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
                + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
                + Rem<Output=T> + RemAssign
                + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
                + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
                + BitXor<Output=T> + BitXorAssign + Not<Output=T>
                + PartialEq + PartialOrd
        {
            type Output = Self;

            // fn shl(self, rhs: $f) -> Self
            /// Shift left the field `number: [T;N]` to the left by `n`,
            /// and returns the result.
            /// 
            /// # Arguments
            /// `rhs` indicates how many bits this method shift `self` left by,
            /// and can be any primitive integer such as `i8`, `i16`, `i32`,
            /// `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, and
            /// `usize`.
            /// 
            /// # Output
            /// It returns the left-shifted version of `self`, which is shifted
            /// to the left by `rhs` bits.
            /// 
            /// # Left Carry
            /// 'A left-carry occurs' means that a bit `1` is pushed out
            /// during shift-left operation.
            ///
            /// # Panics
            /// If `size_of::<T>() * N` <= `128`, this method may panic
            /// or its behavior may be undefined though it may not panic.
            /// 
            /// # Features
            /// - 'Shift left' means 'move left all bits'. So, if `10011010`
            ///   is shifted left by 2, it will be `01101000`, for example.
            /// - Unlike for primitive integer data types such as `i8`, `i16`,
            ///   `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`,
            ///   `u128`, and `usize`, and for Union data types such as
            ///   ShortUnion, IntUnion, LongUnion, LongerUnion, and SizeUnion,
            ///   the negative value of `rhs` makes the operator `<<` work as
            ///   `>>` for BigUInt.
            ///   So, `self` << -2 means `self` >> 2 for example. 
            /// - Unlike for primitive integer data types such as `i8`, `i16`,
            ///   `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`,
            ///   `u128`, and `usize` and for Union data types such as
            ///   ShortUnion, IntUnion, LongUnion, LongerUnion, and SizeUnion,
            ///   the value of `rhs` greater than or equal to `size_of::<T>() * N` 
            ///   pushes all the bits out so that the return value will be zero.
            ///   For example, `self` << `size_of::<T>() * N` will return zero.
            /// - If `rhs` is a positive integer, this operation may cause carry-left.
            /// - If `rhs` is a negative integer, this operation may cause carry-right.
            /// - If `1` is pushed out to the left during the << operation,
            ///   `LEFT_CARRY` flag will be set and the method is_left_carry()
            ///   will return true.
            /// - If `1` is pushed out to the right during the << operation,
            ///   `RIGHT_CARRY` flag will be set and the method is_right_carry()
            ///   will return true.
            /// 
            /// # Example 1 for u8
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 3_u8;
            /// let res = a_biguint.clone() << n;
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101000");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), true);
            /// assert_eq!(res.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 2 for u16
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 4_u16;
            /// let res = a_biguint.clone() << n;
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01010000");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), false);
            /// assert_eq!(res.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 3 for u32
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 128_u32;
            /// let res = a_biguint.clone() << n;
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), true);
            /// assert_eq!(res.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 4 for u64
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 256_u64;
            /// let res = a_biguint.clone() << n;
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string(), "0");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), true);
            /// assert_eq!(res.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 5 for u128
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 512_u128;
            /// let res = a_biguint.clone() << n;
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string(), "0");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), true);
            /// assert_eq!(res.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 6 for usize
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 1024_usize;
            /// let res = a_biguint.clone() << n;
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string(), "0");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), true);
            /// assert_eq!(res.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 7 for positive i8
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 3_i8;
            /// let res = a_biguint.clone() << n;
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101000");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), true);
            /// assert_eq!(res.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 8 for positive i16
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 4_i16;
            /// let res = a_biguint.clone() << n;
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01010000");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), false);
            /// assert_eq!(res.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 9 for positive i32
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 128_i32;
            /// let res = a_biguint.clone() << n;
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), true);
            /// assert_eq!(res.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 10 for positive i64
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 256_i64;
            /// let res = a_biguint.clone() << n;
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string(), "0");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), true);
            /// assert_eq!(res.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 11 for positive i128
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 512_i128;
            /// let res = a_biguint.clone() << n;
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string(), "0");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), true);
            /// assert_eq!(res.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 12 for positive isize
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 1024_isize;
            /// let res = a_biguint.clone() << n;
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string(), "0");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), true);
            /// assert_eq!(res.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 13 for negative i8
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
            /// let n = -3_i8;
            /// let res = a_biguint.clone() << n;
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01100000_00011111");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), false);
            /// assert_eq!(res.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 14 for negative i16
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_11110000", 2).unwrap();
            /// let n = -4_i16;
            /// let res = a_biguint.clone() << n;
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "1111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10101111");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), false);
            /// assert_eq!(res.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 15 for negative i32
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
            /// let n = -128_i32;
            /// let res = a_biguint.clone() << n;
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), false);
            /// assert_eq!(res.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 16 for negative i64
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = -256_i64;
            /// let res = a_biguint.clone() << n;
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string(), "0");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), false);
            /// assert_eq!(res.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 17 for negative i128
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = -512_i128;
            /// let res = a_biguint.clone() << n;
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string(), "0");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), false);
            /// assert_eq!(res.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 18 for negative isize
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = -1024_isize;
            /// let res = a_biguint.clone() << n;
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string(), "0");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), false);
            /// assert_eq!(res.is_right_carry(), true);
            /// ```
            /// 
            /// # Compile-fail Examples
            /// ```compile_fail
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 3_u8;
            /// let _res = a_biguint << n;
            /// // It cannot be compiled!
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator << swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 4_u16;
            /// let _res = a_biguint << n;
            /// // It cannot be compiled!
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator << swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 128_u32;
            /// let _res = a_biguint << n;
            /// // It cannot be compiled!
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator << swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 256_u64;
            /// let _res = a_biguint << n;
            /// // It cannot be compiled!
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator << swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 512_u128;
            /// let _res = a_biguint << n;
            /// // It cannot be compiled!
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator << swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 1024_usize;
            /// let _res = a_biguint << n;
            /// // It cannot be compiled!
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator << swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 3_i8;
            /// let _res = a_biguint << n;
            /// // It cannot be compiled!
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator << swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 4_i16;
            /// let _res = a_biguint << n;
            /// // It cannot be compiled!
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator << swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 128_i32;
            /// let _res = a_biguint << n;
            /// // It cannot be compiled!
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator << swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 256_i64;
            /// let _res = a_biguint << n;
            /// // It cannot be compiled!
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator << swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 512_i128;
            /// let _res = a_biguint << n;
            /// // It cannot be compiled!
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator << swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 1024_isize;
            /// let _res = a_biguint << n;
            /// // It cannot be compiled!
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator << swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
            /// let n = -3_i8;
            /// let _res = a_biguint << n;
            /// // It cannot be compiled!
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator << swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_11110000", 2).unwrap();
            /// let n = -4_i16;
            /// let _res = a_biguint << n;
            /// // It cannot be compiled!
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator << swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
            /// let n = -128_i32;
            /// let _res = a_biguint << n;
            /// // It cannot be compiled!
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator << swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = -256_i64;
            /// let _res = a_biguint << n;
            /// // It cannot be compiled!
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator << swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = -512_i128;
            /// let _res = a_biguint << n;
            /// // It cannot be compiled!
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator << swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = -1024_isize;
            /// let _res = a_biguint << n;
            /// // It cannot be compiled!
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator << swallowed (took the ownership of) a_biguint.
            /// ```
            /// 
            /// # Big-endian issue
            /// It is just experimental for Big Endian CPUs. So, you are not encouraged
            /// to use it for Big Endian CPUs for serious purpose. Only use this crate
            /// for Big-endian CPUs with your own full responsibility.
            fn shl(self, rhs: $f) -> Self
            {
                calc_assign_to_calc!(self, <<=, rhs);
            }
        }
    }
}



macro_rules! shlassign_i_for_BigUInt_impl {
    ($f:ty) => {
        /// I would like to suggest the modification of Rust grammar because the
        /// operator `<<=` swallows (takes the ownership of) two operands which are
        /// left-hand side operand `self` and right-hand side operand `rhs` so that
        /// the two operands `self` and `rhs` cannot be used again after shift-left
        /// operation. In order to prevent this, the operands should be cloned or
        /// copied before shift-left operation. This adds the unnecessary overhead.
        /// The heavier the operand object is, the more the overhead is.
        /// 
        /// So, I would like to suggest one of the following three as follows:
        /// 
        /// # First suggestion
        /// Changing the types of the parameters as follows:
        /// 
        /// ```text
        /// pub trait ShlAssign<Rhs = Self> {
        ///     // Required method
        ///     fn shl_assign(&mut self, rhs: &Rhs);
        /// }
        /// ```
        /// 
        /// # Second suggestion
        /// If the first suggestion is impossible because of backward compatibility,
        /// grammar allows the developer to choose the types of parameters but make
        /// only one function.
        /// 
        /// ```text
        /// pub trait ShlAssign<Rhs = Self> {
        ///     // Required method
        ///     fn shl_assign(&mut self, rhs: Rhs);
        ///   or
        ///     fn shl_assign(&mut self, rhs: &Rhs);
        /// }
        /// ```
        /// 
        /// # Third suggestion
        /// If the first and second suggestions are impossible because of backward
        /// compatibility, trait ShlAssign2 is provided and the developer
        /// implements none or only one of traits ShlAssign and ShlAssign2.
        /// 
        /// ```
        /// pub trait ShlAssign<Rhs = Self> {
        ///     // Required method
        ///     fn shl_assign(&mut self, rhs: Rhs);
        /// }
        /// 
        /// pub trait ShlAssign2<Rhs = Self> {
        ///     // Required method
        ///     fn shl_assign(&mut self, rhs: &Rhs);
        /// }
        /// ```
        /// 
        /// Unlike trait ShlAssign, the trait PartialEq makes the operators
        /// `==` and `!=` take not `&Self` but `Self` as its first operand and not
        /// `&Rhs` (or `&Self`) but `Rhs` (or `Self`) as its second operand but makes
        /// the functions eq() and ne() take not `self` but `&self` as its first
        /// argument and not `Rhs` but `&Rhs` as its second argument.
        /// So, I think the third suggestion is possible.
        /// The prototype of trait PartialEq is as follows:
        /// 
        /// ```text
        /// pub trait PartialEq<Rhs = Self>
        /// where
        /// Rhs: ?Sized,
        /// {
        ///     // Required method
        ///     fn eq(&self, other: &Rhs) -> bool;
        /// 
        ///     // Provided method
        ///     fn ne(&self, other: &Rhs) -> bool { ... }
        /// }
        /// ```
        impl<T, const N: usize> ShlAssign<$f> for BigUInt<T, N>
        where T: SmallUInt + Copy + Clone + Display + Debug + ToString
                + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
                + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
                + Rem<Output=T> + RemAssign
                + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
                + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
                + BitXor<Output=T> + BitXorAssign + Not<Output=T>
                + PartialEq + PartialOrd
        {
            // fn shl_assign(&mut self, rhs: $f)
            /// shifts the field `number: [T;N]` to the left by `n`,
            /// and assigns the result to `self` back.
            /// 
            /// # Arguments
            /// `rhs` indicates how many bits this method shift `self` left by,
            /// and can be any primitive integer such as `i8`, `i16`, `i32`,
            /// `i64`, `i128`, and `isize`.
            /// 
            /// # Left Carry
            /// 'A left-carry occurs' means that a bit `1` is pushed out
            /// during shift-left operation.
            ///
            /// # Panics
            /// If `size_of::<T>() * N` <= `128`, this method may panic
            /// or its behavior may be undefined though it may not panic.
            /// 
            /// # Features
            /// - 'Shift left' means 'move left all bits'. So, if `10011010`
            ///   is shifted left by 2, it will be `01101000`, for example.
            /// - Unlike for primitive integer data types such as `i8`, `i16`,
            ///   `i32`, `i64`, `i128`, and `isize`, and for Union data types
            ///   such as ShortUnion, IntUnion, LongUnion, LongerUnion, and
            ///   SizeUnion, the negative value of `rhs` makes the operator
            ///   `<<=` work as `>>=` for BigUInt.
            ///   So, `self` <<= -2 means `self` >>= 2 for example. 
            /// - Unlike for primitive integer data types such as `i8`, `i16`,
            ///   `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`,
            ///   `u128`, and `usize` and for Union data types such as
            ///   ShortUnion, IntUnion, LongUnion, LongerUnion, and SizeUnion,
            ///   the value of `rhs` greater than or equal to `size_of::<T>() * N` 
            ///   pushes all the bits out so that the return value will be zero.
            ///   For example, `self` <<= `size_of::<T>() * N` will return zero.
            /// - If `rhs` is a positive integer, this operation may cause carry-left.
            /// - If `rhs` is a negative integer, this operation may cause carry-right.
            /// - If `1` is pushed out to the left during the << operation,
            ///   `LEFT_CARRY` flag will be set and the method is_left_carry()
            ///   will return true.
            /// - If `1` is pushed out to the right during the << operation,
            ///   `RIGHT_CARRY` flag will be set and the method is_right_carry()
            ///   will return true.
            /// 
            /// # Example 1 for positive i8
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u16);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 3_i8;
            /// a_biguint <<= n;
            /// println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101000");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), true);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 2 for positive i16
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u16);
            /// 
            /// let mut a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 4_i16;
            /// a_biguint <<= n;
            /// println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01010000");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 3 for positive i32
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u16);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 128_i32;
            /// a_biguint <<= n;
            /// println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), true);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 4 for positive i64
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u16);
            /// 
            /// let mut a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 256_i64;
            /// a_biguint <<= n;
            /// println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string(), "0");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), true);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 5 for positive i128
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u16);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 512_i128;
            /// a_biguint <<= n;
            /// println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string(), "0");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), true);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 6 for positive isize
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u16);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 1024_isize;
            /// a_biguint <<= n;
            /// println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string(), "0");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), true);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 7 for negative i8
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u16);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = -3_i8;
            /// a_biguint <<= n;
            /// println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01100000_00011111");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 8 for negative i16
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u16);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11110000", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = -4_i16;
            /// a_biguint <<= n;
            /// println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "1111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00110000_00001111");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 9 for negative i32
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u16);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = -128_i32;
            /// a_biguint <<= n;
            /// println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 10 for negative i64
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u16);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = -256_i64;
            /// a_biguint <<= n;
            /// println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string(), "0");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 11 for negative i128
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u16);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = -512_i128;
            /// a_biguint <<= n;
            /// println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string(), "0");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 12 for negative isize
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u16);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = -1024_isize;
            /// a_biguint <<= n;
            /// println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string(), "0");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), true);
            /// ```
            /// 
            /// # Big-endian issue
            /// It is just experimental for Big Endian CPUs. So, you are not encouraged
            /// to use it for Big Endian CPUs for serious purpose. Only use this crate
            /// for Big-endian CPUs with your own full responsibility.
            fn shl_assign(&mut self, rhs: $f)
            {
                if rhs < 0
                {
                    match size_of_val(&rhs)
                    {
                        1 => { *self >>= (-rhs as u8); },
                        2 => { *self >>= (-rhs as u16); },
                        4 => { *self >>= (-rhs as u32); },
                        8 => { *self >>= (-rhs as u64); },
                        16 => { *self >>= (-rhs as u128); },
                        _ => {},
                    }
                }
                else
                {
                    match size_of_val(&rhs)
                    {
                        1 => { *self <<= (rhs as u8); },
                        2 => { *self <<= (rhs as u16); },
                        4 => { *self <<= (rhs as u32); },
                        8 => { *self <<= (rhs as u64); },
                        16 => { *self <<= (rhs as u128); },
                        _ => {},
                    }
                }
            }
        }
    }
}



macro_rules! shlassign_u_for_BigUInt_impl {
    ($f:ty) => {
        /// I would like to suggest the modification of Rust grammar because the
        /// operator `<<=` swallows (takes the ownership of) two operands which are
        /// left-hand side operand `self` and right-hand side operand `rhs` so that
        /// the two operands `self` and `rhs` cannot be used again after shift-left
        /// operation. In order to prevent this, the operands should be cloned or
        /// copied before shift-left operation. This adds the unnecessary overhead.
        /// The heavier the operand object is, the more the overhead is.
        /// 
        /// So, I would like to suggest one of the following three as follows:
        /// 
        /// # First suggestion
        /// Changing the types of the parameters as follows:
        /// 
        /// ```text
        /// pub trait ShlAssign<Rhs = Self> {
        ///     // Required method
        ///     fn shl_assign(&mut self, rhs: &Rhs);
        /// }
        /// ```
        /// 
        /// # Second suggestion
        /// If the first suggestion is impossible because of backward compatibility,
        /// grammar allows the developer to choose the types of parameters but make
        /// only one function.
        /// 
        /// ```text
        /// pub trait ShlAssign<Rhs = Self> {
        ///     // Required method
        ///     fn shl_assign(&mut self, rhs: Rhs);
        ///   or
        ///     fn shl_assign(&mut self, rhs: &Rhs);
        /// }
        /// ```
        /// 
        /// # Third suggestion
        /// If the first and second suggestions are impossible because of backward
        /// compatibility, trait ShlAssign2 is provided and the developer
        /// implements none or only one of traits ShlAssign and ShlAssign2.
        /// 
        /// ```
        /// pub trait ShlAssign<Rhs = Self> {
        ///     // Required method
        ///     fn shl_assign(&mut self, rhs: Rhs);
        /// }
        /// 
        /// pub trait ShlAssign2<Rhs = Self> {
        ///     // Required method
        ///     fn shl_assign(&mut self, rhs: &Rhs);
        /// }
        /// ```
        /// 
        /// Unlike trait ShlAssign, the trait PartialEq makes the operators
        /// `==` and `!=` take not `&Self` but `Self` as its first operand and not
        /// `&Rhs` (or `&Self`) but `Rhs` (or `Self`) as its second operand but makes
        /// the functions eq() and ne() take not `self` but `&self` as its first
        /// argument and not `Rhs` but `&Rhs` as its second argument.
        /// So, I think the third suggestion is possible.
        /// The prototype of trait PartialEq is as follows:
        /// 
        /// ```text
        /// pub trait PartialEq<Rhs = Self>
        /// where
        /// Rhs: ?Sized,
        /// {
        ///     // Required method
        ///     fn eq(&self, other: &Rhs) -> bool;
        /// 
        ///     // Provided method
        ///     fn ne(&self, other: &Rhs) -> bool { ... }
        /// }
        /// ```
        impl<T, const N: usize> ShlAssign<$f> for BigUInt<T, N>
        where T: SmallUInt + Copy + Clone + Display + Debug + ToString
                + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
                + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
                + Rem<Output=T> + RemAssign
                + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
                + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
                + BitXor<Output=T> + BitXorAssign + Not<Output=T>
                + PartialEq + PartialOrd
        {
            // fn shl_assign(&mut self, rhs: $f)
            /// shifts the field `number: [T;N]` to the left by `n`,
            /// and assigns the result to `self` back.
            /// 
            /// # Arguments
            /// `rhs` indicates how many bits this method shift `self` left by,
            /// and can be any primitive integer such as `u8`, `u16`, `u32`,
            /// `u64`, `u128`, and `usize`.
            /// 
            /// # Left Carry
            /// 'A left-carry occurs' means that a bit `1` is pushed out
            /// during shift-left operation.
            ///
            /// # Panics
            /// If `size_of::<T>() * N` <= `128`, this method may panic
            /// or its behavior may be undefined though it may not panic.
            /// 
            /// # Features
            /// - 'Shift left' means 'move left all bits'. So, if `10011010`
            ///   is shifted left by 2, it will be `01101000`, for example.
            /// - Unlike for primitive unsigned integer data types such as
            ///   `u8`, `u16`, `u32`, `u64`, `u128`, and `usize` and for Union
            ///   data types such as ShortUnion, IntUnion, LongUnion,
            ///   LongerUnion, and SizeUnion, the value of `rhs` greater than
            ///   or equal to `size_of::<T>() * N` pushes all the bits out so
            ///   that the return value will be zero.
            ///   For example, `self` <<= `size_of::<T>() * N` will return zero.
            /// - If `rhs` is a positive integer, this operation may cause carry-left.
            /// - If `1` is pushed out to the left during the << operation,
            ///   `LEFT_CARRY` flag will be set and the method is_left_carry()
            ///   will return true.
            /// 
            /// # Example 1 for u8
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u16);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 3_u8;
            /// a_biguint <<= n;
            /// println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101000");
             /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), true);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 2 for u16
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u16);
            /// 
            /// let mut a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 4_u16;
            /// a_biguint <<= n;
            /// println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01010000");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 3 for u32
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u16);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 128_u32;
            /// a_biguint <<= n;
            /// println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), true);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 4 for u64
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u16);
            /// 
            /// let mut a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 256_u64;
            /// a_biguint <<= n;
            /// println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string(), "0");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), true);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 5 for u128
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u16);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 512_u128;
            /// a_biguint <<= n;
            /// println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string(), "0");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), true);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 6 for usize
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u16);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 1024_usize;
            /// a_biguint <<= n;
            /// println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string(), "0");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), true);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Big-endian issue
            /// It is just experimental for Big Endian CPUs. So, you are not encouraged
            /// to use it for Big Endian CPUs for serious purpose. Only use this crate
            /// for Big-endian CPUs with your own full responsibility.
            #[inline]
            fn shl_assign(&mut self, rhs: $f)
            {
                self.shift_left_assign(rhs);
            }
        }
    }
}



shl_for_BigUInt_impl! { i8 }
shl_for_BigUInt_impl! { i16 }
shl_for_BigUInt_impl! { i32 }
shl_for_BigUInt_impl! { i64 }
shl_for_BigUInt_impl! { i128 }
shl_for_BigUInt_impl! { isize }
shl_for_BigUInt_impl! { u8 }
shl_for_BigUInt_impl! { u16 }
shl_for_BigUInt_impl! { u32 }
shl_for_BigUInt_impl! { u64 }
shl_for_BigUInt_impl! { u128 }
shl_for_BigUInt_impl! { usize }
shlassign_i_for_BigUInt_impl! { i8 }
shlassign_i_for_BigUInt_impl! { i16 }
shlassign_i_for_BigUInt_impl! { i32 }
shlassign_i_for_BigUInt_impl! { i64 }
shlassign_i_for_BigUInt_impl! { i128 }
shlassign_i_for_BigUInt_impl! { isize }
shlassign_u_for_BigUInt_impl! { u8 }
shlassign_u_for_BigUInt_impl! { u16 }
shlassign_u_for_BigUInt_impl! { u32 }
shlassign_u_for_BigUInt_impl! { u64 }
shlassign_u_for_BigUInt_impl! { u128 }
shlassign_u_for_BigUInt_impl! { usize }



macro_rules! shr_for_BigUInt_impl {
    ($f:ty) => {
        /// I would like to suggest the modification of Rust grammar because the
        /// operator `>>` swallows (takes the ownership of) two operands which are
        /// left-hand side operand `self` and right-hand side operand `rhs` so that
        /// the two operands `self` and `rhs` cannot be used again after shift-right
        /// operation. In order to prevent this, the operands should be cloned or
        /// copied before shift-right operation. This adds the unnecessary overhead.
        /// The heavier the operand object is, the more the overhead is.
        /// 
        /// So, I would like to suggest one of the following three as follows:
        /// 
        /// # First suggestion
        /// Changing the types of the parameters as follows:
        /// 
        /// ```text
        /// pub trait Shr<Rhs = Self> {
        ///     type Output;
        ///     // Required method
        ///     fn shr(&self, rhs: &Rhs) -> Self::Output;
        /// }
        /// ```
        /// 
        /// # Second suggestion
        /// If the first suggestion is impossible because of backward compatibility,
        /// grammar allows the developer to choose the types of parameters but make
        /// only one function.
        /// 
        /// ```text
        /// pub trait Shr<Rhs = Self> {
        ///     type Output;
        ///     // Required method
        ///     fn shr(self, rhs: Rhs) -> Self::Output;
        ///   or
        ///     fn shr(&self, rhs: Rhs) -> Self::Output;
        ///   or
        ///     fn shr(self, rhs: &Rhs) -> Self::Output;
        ///   or
        ///     fn shr(&self, rhs: &Rhs) -> Self::Output;
        /// }
        /// ```
        /// 
        /// # Third suggestion
        /// If the first and second suggestions are impossible because of backward
        /// compatibility, trait Shr2, Shr3, and Shr4 are provided and the developer
        /// implements none or only one of traits Shr, Shr2, Shr3, and Shr4.
        /// 
        /// ```text
        /// pub trait Shr<Rhs = Self> {
        ///     type Output;
        ///     // Required method
        ///     fn shr(self, rhs: Rhs) -> Self::Output;
        /// }
        /// 
        /// pub trait Shr2<Rhs = Self> {
        ///     type Output;
        ///     // Required method
        ///     fn shr(&self, rhs: Rhs) -> Self::Output;
        /// }
        /// 
        /// pub trait Shr3<Rhs = Self> {
        ///     type Output;
        ///     // Required method
        ///     fn shr(self, rhs: &Rhs) -> Self::Output;
        /// }
        /// 
        /// pub trait Shr4<Rhs = Self> {
        ///     type Output;
        ///     // Required method
        ///     fn shr(&self, rhs: &Rhs) -> Self::Output;
        /// }
        /// ```
        /// 
        /// Unlike trait Shr, the trait PartialEq makes the operators `==` and `!=` take
        /// not `&Self` but `Self` as its first operand and not `&Rhs` (or `&Self`) but
        /// `Rhs` (or `Self`) as its second operand but makes the functions eq() and
        /// ne() take not `self` but `&self` as its first argument and not `Rhs` but
        /// `&Rhs` as its second argument. So, I think the third suggestion is possible.
        /// The prototype of trait PartialEq is as follows:
        /// 
        /// ```text
        /// pub trait PartialEq<Rhs = Self>
        /// where
        /// Rhs: ?Sized,
        /// {
        ///     // Required method
        ///     fn eq(&self, other: &Rhs) -> bool;
        /// 
        ///     // Provided method
        ///     fn ne(&self, other: &Rhs) -> bool { ... }
        /// }
        /// ```
        impl<T, const N: usize> Shr<$f> for BigUInt<T, N>
        where T: SmallUInt + Copy + Clone + Display + Debug + ToString
                + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
                + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
                + Rem<Output=T> + RemAssign
                + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
                + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
                + BitXor<Output=T> + BitXorAssign + Not<Output=T>
                + PartialEq + PartialOrd
        {
            type Output = Self;

            // fn shr(self, rhs: $f) -> Self
            /// Shift right the field `number: [T;N]` to the right by `n`,
            /// and returns the result.
            /// 
            /// # Arguments
            /// `rhs` indicates how many bits this method shift `self` right by,
            /// and can be any primitive integer such as `i8`, `i16`, `i32`,
            /// `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, and
            /// `usize`.
            /// 
            /// # Output
            /// It returns the right-shifted version of `self`,
            /// which is shifted to the right by `rhs` bits.
            /// 
            /// # Right Carry
            /// 'A right-carry occurs' means that a bit `1` is pushed out
            /// during shift-right operation.
            ///
            /// # Panics
            /// If `size_of::<T>() * N` <= `128`, this method may panic
            /// or its behavior may be undefined though it may not panic.
            /// 
            /// # Features
            /// - 'Shift right' means 'move right all bits'. So, if `10011010`
            ///   is shifted right by 2, it will be `00100110`, for example.
            /// - Unlike for primitive integer data types such as `i8`, `i16`,
            ///   `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`,
            ///   `u128`, and `usize`, and for Union data types such as
            ///   ShortUnion, IntUnion, LongUnion, LongerUnion, and SizeUnion,
            ///   the negative value of `rhs` makes the operator `>>` work as
            ///   `<<` for BigUInt.
            ///   So, `self` >> -2 means `self` << 2 for example. 
            /// - Unlike for primitive integer data types such as `i8`, `i16`,
            ///   `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`,
            ///   `u128`, and `usize` and for Union data types such as
            ///   ShortUnion, IntUnion, LongUnion, LongerUnion, and SizeUnion,
            ///   the value of `rhs` greater than or equal to `size_of::<T>() * N` 
            ///   pushes all the bits out so that the return value will be zero.
            ///   For example, `self` >> `size_of::<T>() * N` will return zero.
            /// - If `rhs` is a positive integer, this operation may cause carry-right.
            /// - If `rhs` is a negative integer, this operation may cause carry-left.
            /// - If `1` is pushed out to the right during the >> operation,
            ///   `RIGHT_CARRY` flag will be set and the method is_right_carry()
            ///   will return true.
            /// - If `1` is pushed out to the left during the >> operation,
            ///   `LEFT_CARRY` flag will be set and the method is_left_carry()
            ///   will return true.
            /// 
            /// # Example 1 for u8
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
            /// let n = 3_u8;
            /// let res = a_biguint.clone() >> n;
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01100000_00011111");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), false);
            /// assert_eq!(res.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 2 for u16
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_11110000", 2).unwrap();
            /// let n = 4_u16;
            /// let res = a_biguint.clone() >> n;
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "1111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10101111");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), false);
            /// assert_eq!(res.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 3 for u32
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
            /// let n = 128_u32;
            /// let res = a_biguint.clone() >> n;
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), false);
            /// assert_eq!(res.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 4 for u64
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 256_u64;
            /// let res = a_biguint.clone() >> n;
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string(), "0");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), false);
            /// assert_eq!(res.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 5 for u128
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 512_u128;
            /// let res = a_biguint.clone() >> n;
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string(), "0");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), false);
            /// assert_eq!(res.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 6 for usize
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 1024_usize;
            /// let res = a_biguint.clone() >> n;
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string(), "0");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), false);
            /// assert_eq!(res.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 7 for positive i8
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
            /// let n = 3_i8;
            /// let res = a_biguint.clone() >> n;
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01100000_00011111");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), false);
            /// assert_eq!(res.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 8 for positive i16
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_11110000", 2).unwrap();
            /// let n = 4_i16;
            /// let res = a_biguint.clone() >> n;
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "1111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10101111");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), false);
            /// assert_eq!(res.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 9 for positive i32
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
            /// let n = 128_i32;
            /// let res = a_biguint.clone() >> n;
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), false);
            /// assert_eq!(res.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 10 for positive i64
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 256_i64;
            /// let res = a_biguint.clone() >> n;
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string(), "0");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), false);
            /// assert_eq!(res.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 11 for positive i128
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 512_i128;
            /// let res = a_biguint.clone() >> n;
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string(), "0");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), false);
            /// assert_eq!(res.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 12 for positive isize
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 1024_isize;
            /// let res = a_biguint.clone() >> n;
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string(), "0");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), false);
            /// assert_eq!(res.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 13 for negative i8
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = -3_i8;
            /// let res = a_biguint.clone() >> n;
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101000");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), true);
            /// assert_eq!(res.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 14 for negative i16
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = -4_i16;
            /// let res = a_biguint.clone() >> n;
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01010000");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), false);
            /// assert_eq!(res.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 15 for negative i32
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = -128_i32;
            /// let res = a_biguint.clone() >> n;
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), true);
            /// assert_eq!(res.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 16 for negative i64
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = -256_i64;
            /// let res = a_biguint.clone() >> n;
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string(), "0");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), true);
            /// assert_eq!(res.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 17 for negative i128
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = -512_i128;
            /// let res = a_biguint.clone() >> n;
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string(), "0");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), true);
            /// assert_eq!(res.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 18 for negative isize
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = -1024_isize;
            /// let res = a_biguint.clone() >> n;
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string(), "0");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), true);
            /// assert_eq!(res.is_right_carry(), false);
            /// ```
            /// 
            /// # Compile-fail Examples
            /// ```compile_fail
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 3_u8;
            /// let _res = a_biguint >> n;
            /// // It cannot be compiled!
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator >> swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 4_u16;
            /// let _res = a_biguint >> n;
            /// // It cannot be compiled!
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator >> swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 128_u32;
            /// let _res = a_biguint >> n;
            /// // It cannot be compiled!
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator >> swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 256_u64;
            /// let _res = a_biguint >> n;
            /// // It cannot be compiled!
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator >> swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 512_u128;
            /// let _res = a_biguint >> n;
            /// // It cannot be compiled!
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator >> swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 1024_usize;
            /// let _res = a_biguint >> n;
            /// // It cannot be compiled!
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator >> swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 3_i8;
            /// let _res = a_biguint >> n;
            /// // It cannot be compiled!
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator >> swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 4_i16;
            /// let _res = a_biguint >> n;
            /// // It cannot be compiled!
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator >> swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 128_i32;
            /// let _res = a_biguint >> n;
            /// // It cannot be compiled!
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator >> swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 256_i64;
            /// let _res = a_biguint >> n;
            /// // It cannot be compiled!
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator >> swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 512_i128;
            /// let _res = a_biguint >> n;
            /// // It cannot be compiled!
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator >> swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 1024_isize;
            /// let _res = a_biguint >> n;
            /// // It cannot be compiled!
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator >> swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
            /// let n = -3_i8;
            /// let _res = a_biguint >> n;
            /// // It cannot be compiled!
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator >> swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_11110000", 2).unwrap();
            /// let n = -4_i16;
            /// let _res = a_biguint >> n;
            /// // It cannot be compiled!
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator >> swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
            /// let n = -128_i32;
            /// let _res = a_biguint >> n;
            /// // It cannot be compiled!
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator >> swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = -256_i64;
            /// let _res = a_biguint >> n;
            /// // It cannot be compiled!
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator >> swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = -512_i128;
            /// let _res = a_biguint >> n;
            /// // It cannot be compiled!
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator >> swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = -1024_isize;
            /// let _res = a_biguint >> n;
            /// // It cannot be compiled!
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator >> swallowed (took the ownership of) a_biguint.
            /// ```
            /// 
            /// # Big-endian issue
            /// It is just experimental for Big Endian CPUs. So, you are not encouraged
            /// to use it for Big Endian CPUs for serious purpose. Only use this crate
            /// for Big-endian CPUs with your own full responsibility.
            fn shr(self, rhs: $f) -> Self
            {
                calc_assign_to_calc!(self, >>=, rhs);
            }
        }
    }
}



macro_rules! shrassign_i_for_BigUInt_impl {
    ($f:ty) => {
        /// I would like to suggest the modification of Rust grammar because the
        /// operator `>>=` swallows (takes the ownership of) two operands which are
        /// left-hand side operand `self` and right-hand side operand `rhs` so that
        /// the two operands `self` and `rhs` cannot be used again after shift-right
        /// operation. In order to prevent this, the operands should be cloned or
        /// copied before shift-right operation. This adds the unnecessary overhead.
        /// The heavier the operand object is, the more the overhead is.
        /// 
        /// So, I would like to suggest one of the following three as follows:
        /// 
        /// # First suggestion
        /// Changing the types of the parameters as follows:
        /// 
        /// ```text
        /// pub trait ShrAssign<Rhs = Self> {
        ///     // Required method
        ///     fn shr_assign(&mut self, rhs: &Rhs);
        /// }
        /// ```
        /// 
        /// # Second suggestion
        /// If the first suggestion is impossible because of backward compatibility,
        /// grammar allows the developer to choose the types of parameters but make
        /// only one function.
        /// 
        /// ```text
        /// pub trait ShrAssign<Rhs = Self> {
        ///     // Required method
        ///     fn shr_assign(&mut self, rhs: Rhs);
        ///   or
        ///     fn shr_assign(&mut self, rhs: &Rhs);
        /// }
        /// ```
        /// 
        /// # Third suggestion
        /// If the first and second suggestions are impossible because of backward
        /// compatibility, trait ShrAssign2 is provided and the developer
        /// implements none or only one of traits ShrAssign and ShrAssign2.
        /// 
        /// ```text
        /// pub trait ShrAssign<Rhs = Self> {
        ///     // Required method
        ///     fn shr_assign(&mut self, rhs: Rhs);
        /// }
        /// 
        /// pub trait ShrAssign2<Rhs = Self> {
        ///     // Required method
        ///     fn shr_assign(&mut self, rhs: &Rhs);
        /// }
        /// ```
        /// 
        /// Unlike trait ShrAssign, the trait PartialEq makes the operators
        /// `==` and `!=` take not `&Self` but `Self` as its first operand and not
        /// `&Rhs` (or `&Self`) but `Rhs` (or `Self`) as its second operand but makes
        /// the functions eq() and ne() take not `self` but `&self` as its first
        /// argument and not `Rhs` but `&Rhs` as its second argument.
        /// So, I think the third suggestion is possible.
        /// The prototype of trait PartialEq is as follows:
        /// 
        /// ```text
        /// pub trait PartialEq<Rhs = Self>
        /// where
        /// Rhs: ?Sized,
        /// {
        ///     // Required method
        ///     fn eq(&self, other: &Rhs) -> bool;
        /// 
        ///     // Provided method
        ///     fn ne(&self, other: &Rhs) -> bool { ... }
        /// }
        /// ```
        impl<T, const N: usize> ShrAssign<$f> for BigUInt<T, N>
        where T: SmallUInt + Copy + Clone + Display + Debug + ToString
                + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
                + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
                + Rem<Output=T> + RemAssign
                + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
                + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
                + BitXor<Output=T> + BitXorAssign + Not<Output=T>
                + PartialEq + PartialOrd
        {
            // fn shr_assign(&mut self, rhs: $f)
            /// shifts the field `number: [T;N]` to the right by `n`,
            /// and assigns the result to `self` back.
            /// 
            /// # Arguments
            /// `rhs` indicates how many bits this method shift `self` right by,
            /// and can be any primitive integer such as `i8`, `i16`, `i32`,
            /// `i64`, `i128`, and `isize`.
            /// 
            /// # Right Carry
            /// 'A right-carry occurs' means that a bit `1` is pushed out
            /// during shift-right operation.
            ///
            /// # Panics
            /// If `size_of::<T>() * N` <= `128`, this method may panic
            /// or its behavior may be undefined though it may not panic.
            /// 
            /// # Features
            /// - 'Shift right' means 'move right all bits'. So, if `10011010`
            ///   is shifted right by 2, it will be `00100110`, for example.
            /// - Unlike for primitive integer data types such as `i8`, `i16`,
            ///   `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`,
            ///   `u128`, and `usize`, and for Union data types such as
            ///   ShortUnion, IntUnion, LongUnion, LongerUnion, and SizeUnion,
            ///   the negative value of `rhs` makes the operator `>>=` work as
            ///   `<<=` for BigUInt.
            ///   So, `self` >>= -2 means `self` <<= 2 for example. 
            /// - Unlike for primitive integer data types such as `i8`, `i16`,
            ///   `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`,
            ///   `u128`, and `usize` and for Union data types such as
            ///   ShortUnion, IntUnion, LongUnion, LongerUnion, and SizeUnion,
            ///   the value of `rhs` greater than or equal to `size_of::<T>() * N` 
            ///   pushes all the bits out so that the return value will be zero.
            ///   For example, `self` >>= `size_of::<T>() * N`
            ///   will make `self` zero.
            /// - If `rhs` is a positive integer, this operation may cause carry-right.
            /// - If `rhs` is a negative integer, this operation may cause carry-left.
            /// - If `1` is pushed out to the right during the >> operation,
            ///   `RIGHT_CARRY` flag will be set and the method is_right_carry()
            ///   will return true.
            /// - If `1` is pushed out to the left during the >> operation,
            ///   `LEFT_CARRY` flag will be set and the method is_left_carry()
            ///   will return true.
            /// 
            /// # Example 1 for positive i8
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u64);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 3_i8;
            /// a_biguint >>= n;
            /// println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01100000_00011111");
             /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 2 for positive i16
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u64);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11110000", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 4_i16;
            /// a_biguint >>= n;
            /// println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "1111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00110000_00001111");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 3 for positive i32
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u64);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 128_i32;
            /// a_biguint >>= n;
            /// println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 4 for positive i64
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u64);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 256_i64;
            /// a_biguint >>= n;
            /// println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string(), "0");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 5 for positive i128
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u64);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 512_i128;
            /// a_biguint >>= n;
            /// println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string(), "0");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 6 for positive isize
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u64);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 1024_isize;
            /// a_biguint >>= n;
            /// println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string(), "0");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 7 for negative i8
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u64);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = -3_i8;
            /// a_biguint >>= n;
            /// println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101000");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), true);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 8 for negative i16
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u64);
            /// 
            /// let mut a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = -4_i16;
            /// a_biguint >>= n;
            /// println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01010000");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 9 for negative i32
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u64);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = -128_i32;
            /// a_biguint >>= n;
            /// println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), true);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 10 for negative i64
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u64);
            /// let mut a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = -256_i64;
            /// a_biguint >>= n;
            /// println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string(), "0");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), true);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 11 for negative i128
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u64);
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = -512_i128;
            /// a_biguint >>= n;
            /// println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string(), "0");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), true);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 12 for negative isize
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u64);
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = -1024_isize;
            /// a_biguint >>= n;
            /// println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string(), "0");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), true);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Big-endian issue
            /// It is just experimental for Big Endian CPUs. So, you are not encouraged
            /// to use it for Big Endian CPUs for serious purpose. Only use this crate
            /// for Big-endian CPUs with your own full responsibility.
            fn shr_assign(&mut self, rhs: $f)
            {
                if rhs < 0
                {
                    match size_of_val(&rhs)
                    {
                        1 => { *self <<= (-rhs as u8); },
                        2 => { *self <<= (-rhs as u16); },
                        4 => { *self <<= (-rhs as u32); },
                        8 => { *self <<= (-rhs as u64); },
                        16 => { *self <<= (-rhs as u128); },
                        _ => {},
                    }
                }
                else
                {
                    match size_of_val(&rhs)
                    {
                        1 => { *self >>= (rhs as u8); },
                        2 => { *self >>= (rhs as u16); },
                        4 => { *self >>= (rhs as u32); },
                        8 => { *self >>= (rhs as u64); },
                        16 => { *self >>= (rhs as u128); },
                        _ => {},
                    }
                }
            }
        }
    }
}


macro_rules! shrassign_u_for_BigUInt_impl {
    ($f:ty) => {
        /// I would like to suggest the modification of Rust grammar because the
        /// operator `>>=` swallows (takes the ownership of) two operands which are
        /// left-hand side operand `self` and right-hand side operand `rhs` so that
        /// the two operands `self` and `rhs` cannot be used again after shift-right
        /// operation. In order to prevent this, the operands should be cloned or
        /// copied before shift-right operation. This adds the unnecessary overhead.
        /// The heavier the operand object is, the more the overhead is.
        /// 
        /// So, I would like to suggest one of the following three as follows:
        /// 
        /// # First suggestion
        /// Changing the types of the parameters as follows:
        /// 
        /// ```text
        /// pub trait ShrAssign<Rhs = Self> {
        ///     // Required method
        ///     fn shr_assign(&mut self, rhs: &Rhs);
        /// }
        /// ```
        /// 
        /// # Second suggestion
        /// If the first suggestion is impossible because of backward compatibility,
        /// grammar allows the developer to choose the types of parameters but make
        /// only one function.
        /// 
        /// ```text
        /// pub trait ShrAssign<Rhs = Self> {
        ///     // Required method
        ///     fn shr_assign(&mut self, rhs: Rhs);
        ///   or
        ///     fn shr_assign(&mut self, rhs: &Rhs);
        /// }
        /// ```
        /// 
        /// # Third suggestion
        /// If the first and second suggestions are impossible because of backward
        /// compatibility, trait ShrAssign2 is provided and the developer
        /// implements none or only one of traits ShrAssign and ShrAssign2.
        /// 
        /// ```text
        /// pub trait ShrAssign<Rhs = Self> {
        ///     // Required method
        ///     fn shr_assign(&mut self, rhs: Rhs);
        /// }
        /// 
        /// pub trait ShrAssign2<Rhs = Self> {
        ///     // Required method
        ///     fn shr_assign(&mut self, rhs: &Rhs);
        /// }
        /// ```
        /// 
        /// Unlike trait ShrAssign, the trait PartialEq makes the operators
        /// `==` and `!=` take not `&Self` but `Self` as its first operand and not
        /// `&Rhs` (or `&Self`) but `Rhs` (or `Self`) as its second operand but makes
        /// the functions eq() and ne() take not `self` but `&self` as its first
        /// argument and not `Rhs` but `&Rhs` as its second argument.
        /// So, I think the third suggestion is possible.
        /// The prototype of trait PartialEq is as follows:
        /// 
        /// ```text
        /// pub trait PartialEq<Rhs = Self>
        /// where
        /// Rhs: ?Sized,
        /// {
        ///     // Required method
        ///     fn eq(&self, other: &Rhs) -> bool;
        /// 
        ///     // Provided method
        ///     fn ne(&self, other: &Rhs) -> bool { ... }
        /// }
        /// ```
        impl<T, const N: usize> ShrAssign<$f> for BigUInt<T, N>
        where T: SmallUInt + Copy + Clone + Display + Debug + ToString
                + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
                + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
                + Rem<Output=T> + RemAssign
                + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
                + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
                + BitXor<Output=T> + BitXorAssign + Not<Output=T>
                + PartialEq + PartialOrd
        {
            // fn shr_assign(&mut self, rhs: $f)
            /// shifts the field `number: [T;N]` to the right by `n`,
            /// and assigns the result to `self` back.
            /// 
            /// # Arguments
            /// `rhs` indicates how many bits this method shift `self` right by,
            /// and can be any primitive integer such as `u8`, `u16`, `u32`,
            /// `u64`, `u128`, and `usize`.
            /// 
            /// # Right Carry
            /// 'A right-carry occurs' means that a bit `1` is pushed out
            /// during shift-right operation.
            ///
            /// # Panics
            /// If `size_of::<T>() * N` <= `128`, this method may panic
            /// or its behavior may be undefined though it may not panic.
            /// 
            /// # Features
            /// - 'Shift right' means 'move right all bits'. So, if `10011010`
            ///   is shifted right by 2, it will be `00100110`, for example.
            ///   So, `self` >> -2 means `self` << 2 for example. 
            /// - Unlike for primitive integer data types such as `i8`, `i16`,
            ///   `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`,
            ///   `u128`, and `usize` and for Union data types such as
            ///   ShortUnion, IntUnion, LongUnion, LongerUnion, and SizeUnion,
            ///   the value of `rhs` greater than or equal to `size_of::<T>() * N` 
            ///   pushes all the bits out so that the return value will be zero.
            ///   For example, `self` >>= `size_of::<T>() * N`
            ///   will make `self` zero.
            /// - If underflow happens during the >>= operation, `UNDERFLOW` flag
            ///   will be set and the method is_underflow() will return true.
            /// - If overflow happens during the >>= operation, `OVERFLOW` flag
            ///   will be set and the method is_overflow() will return true.
            /// 
            /// # Example 1 for u8
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u64);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 3_u8;
            /// a_biguint >>= n;
            /// println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01100000_00011111");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 2 for u16
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u64);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11110000", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 4_u16;
            /// a_biguint >>= n;
            /// println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "1111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00110000_00001111");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 3 for u32
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u64);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 128_u32;
            /// a_biguint >>= n;
            /// println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 4 for u64
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u64);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 256_u64;
            /// a_biguint >>= n;
            /// println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string(), "0");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 5 for u128
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u64);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 512_u128;
            /// a_biguint >>= n;
            /// println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string(), "0");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 6 for usize
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u64);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 1024_usize;
            /// a_biguint >>= n;
            /// println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string(), "0");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), true);
            /// ```
            /// 
            /// # Big-endian issue
            /// It is just experimental for Big Endian CPUs. So, you are not encouraged
            /// to use it for Big Endian CPUs for serious purpose. Only use this crate
            /// for Big-endian CPUs with your own full responsibility.
            #[inline]
            fn shr_assign(&mut self, rhs: $f)
            {
                self.shift_right_assign(rhs);
            }
        }
    }
}



shr_for_BigUInt_impl! { i8 }
shr_for_BigUInt_impl! { i16 }
shr_for_BigUInt_impl! { i32 }
shr_for_BigUInt_impl! { i64 }
shr_for_BigUInt_impl! { i128 }
shr_for_BigUInt_impl! { isize }
shr_for_BigUInt_impl! { u8 }
shr_for_BigUInt_impl! { u16 }
shr_for_BigUInt_impl! { u32 }
shr_for_BigUInt_impl! { u64 }
shr_for_BigUInt_impl! { u128 }
shr_for_BigUInt_impl! { usize }
shrassign_i_for_BigUInt_impl! { i8 }
shrassign_i_for_BigUInt_impl! { i16 }
shrassign_i_for_BigUInt_impl! { i32 }
shrassign_i_for_BigUInt_impl! { i64 }
shrassign_i_for_BigUInt_impl! { i128 }
shrassign_i_for_BigUInt_impl! { isize }
shrassign_u_for_BigUInt_impl! { u8 }
shrassign_u_for_BigUInt_impl! { u16 }
shrassign_u_for_BigUInt_impl! { u32 }
shrassign_u_for_BigUInt_impl! { u64 }
shrassign_u_for_BigUInt_impl! { u128 }
shrassign_u_for_BigUInt_impl! { usize }



/// I would like to suggest the modification of Rust grammar because the
/// operator `&` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after AND (&)
/// operation. In order to prevent this, the operands should be cloned or
/// copied before AND (&) operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait BitAnd<Rhs = Self> {
///     type Output;
///     // Required method
///     fn bitand(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait BitAnd<Rhs = Self> {
///     type Output;
///     // Required method
///     fn bitand(self, rhs: Rhs) -> Self::Output;
///   or
///     fn bitand(&self, rhs: Rhs) -> Self::Output;
///   or
///     fn bitand(self, rhs: &Rhs) -> Self::Output;
///   or
///     fn bitand(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait BitAnd2, BitAnd3, and BitAnd4 are provided and
/// the developer implements none or only one of traits BitAnd, BitAnd2,
/// BitAnd3, and BitAnd4.
/// 
/// ```text
/// pub trait BitAnd<Rhs = Self> {
///     type Output;
///     // Required method
///     fn bitand(self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait BitAnd2<Rhs = Self> {
///     type Output;
///     // Required method
///     fn bitand(&self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait BitAnd3<Rhs = Self> {
///     type Output;
///     // Required method
///     fn bitand(self, rhs: &Rhs) -> Self::Output;
/// }
/// 
/// pub trait BitAnd4<Rhs = Self> {
///     type Output;
///     // Required method
///     fn bitand(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// Unlike trait BitAnd, the trait PartialEq makes the operators
/// `==` and `!=` take not `&Self` but `Self` as its first operand and not
/// `&Rhs` (or `&Self`) but `Rhs` (or `Self`) as its second operand but makes
/// the functions eq() and ne() take not `self` but `&self` as its first
/// argument and not `Rhs` but `&Rhs` as its second argument.
/// So, I think the third suggestion is possible.
/// The prototype of trait PartialEq is as follows:
/// 
/// ```text
/// pub trait PartialEq<Rhs = Self>
/// where
/// Rhs: ?Sized,
/// {
///     // Required method
///     fn eq(&self, other: &Rhs) -> bool;
/// 
///     // Provided method
///     fn ne(&self, other: &Rhs) -> bool { ... }
/// }
/// ```
impl<T, const N: usize> BitAnd for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    // fn bitand(self, rhs: Self) -> Self
    /// Performs the bitwise AND (&) operation, and then returns the result.
    /// 
    /// # Arguments
    /// - `rhs` is the reference of another object that AND (&) operation is
    ///   performed with.
    /// - `rhs` is of `Self` type.
    /// 
    /// # Output
    /// It returns the result after applying the bitwise AND operation.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    /// let c_biguint = a_biguint.clone() & b_biguint.clone();
    /// 
    /// println!("{} & {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(c_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_00000000_00000000_00000000_11111111_00000000_00000000_11111111_00000000_00000000_00000000_00000000_00000000_10001111_00001111_10000011_11110000_00000000_00000000_00000000_00000000_10100010_10001100_00000000_10000011_00000000_00111111_10000000_00000000_00000000");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::max();
    /// let c_biguint = a_biguint.clone() & b_biguint.clone();
    /// 
    /// println!("{} & {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(c_biguint, a_biguint);
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::zero();
    /// let c_biguint = a_biguint.clone() & b_biguint.clone();
    /// 
    /// println!("{} & {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(c_biguint.to_string(), "0");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    /// let _c_biguint = a_biguint & b_biguint;
    /// // It cannot be compiled!
    /// println!("{} & {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), _c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator & swallowed (took the ownership of) a_biguint and b_biguint.
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::max();
    /// let _c_biguint = a_biguint & b_biguint;
    /// // It cannot be compiled!
    /// println!("{} & {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), _c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator & swallowed (took the ownership of) a_biguint and b_biguint.
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::zero();
    /// let _c_biguint = a_biguint & b_biguint;
    /// // It cannot be compiled!
    /// println!("{} & {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), _c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator & swallowed (took the ownership of) a_biguint and b_biguint.
    /// ```
    #[inline]
    fn bitand(self, rhs: Self) -> Self
    {
        self.and(&rhs)
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `&=` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after AND (&=)
/// operation. In order to prevent this, the operands should be cloned or
/// copied before AND (&=) operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait BitAndAssign<Rhs = Self> {
///     // Required method
///     fn bitand_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait BitAndAssign<Rhs = Self> {
///     // Required method
///     fn bitand_assign(&mut self, rhs: Rhs);
///   or
///     fn bitand_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait BitAndAssign2 is provided and the developer
/// implements none or only one of traits BitAndAssign and BitAndAssign2.
/// 
/// ```
/// pub trait BitAndAssign<Rhs = Self> {
///     // Required method
///     fn bitand_assign(&mut self, rhs: Rhs);
/// }
/// 
/// pub trait ShrAssign2<Rhs = Self> {
///     // Required method
///     fn bitand_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// Unlike trait BitAndAssign, the trait PartialEq makes the operators
/// `==` and `!=` take not `&Self` but `Self` as its first operand and not
/// `&Rhs` (or `&Self`) but `Rhs` (or `Self`) as its second operand but makes
/// the functions eq() and ne() take not `self` but `&self` as its first
/// argument and not `Rhs` but `&Rhs` as its second argument.
/// So, I think the third suggestion is possible.
/// The prototype of trait PartialEq is as follows:
/// 
/// ```text
/// pub trait PartialEq<Rhs = Self>
/// where
/// Rhs: ?Sized,
/// {
///     // Required method
///     fn eq(&self, other: &Rhs) -> bool;
/// 
///     // Provided method
///     fn ne(&self, other: &Rhs) -> bool { ... }
/// }
/// ```
impl<T, const N: usize> BitAndAssign for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // fn bitand_assign(&mut self, rhs: Self)
    /// Performs the bitwise AND (&) operation,
    /// and then assigns the result to `self` back.
    /// 
    /// # Arguments
    /// - `rhs` is the reference to another object that the AND (&) operation
    ///   is performed with.
    /// - `rhs` is of `Self` type.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    /// a_biguint &= b_biguint.clone();
    /// println!("After a_biguint &= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_00000000_00000000_00000000_11111111_00000000_00000000_11111111_00000000_00000000_00000000_00000000_00000000_10001111_00001111_10000011_11110000_00000000_00000000_00000000_00000000_10100010_10001100_00000000_10000011_00000000_00111111_10000000_00000000_00000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let b_biguint = U256::max();
    /// a_biguint &= b_biguint.clone();
    /// println!("After a_biguint &= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let b_biguint = U256::zero();
    /// a_biguint &= b_biguint.clone();
    /// println!("After a_biguint &= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    /// a_biguint &= b_biguint;
    /// // It cannot be compiled!
    /// println!("After a_biguint &= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator &= swallowed (took the ownership of) b_biguint.
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// let b_biguint = U256::max();
    /// a_biguint &= b_biguint;
    /// // It cannot be compiled!
    /// println!("After a_biguint &= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator &= swallowed (took the ownership of) b_biguint.
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// let b_biguint = U256::zero();
    /// a_biguint &= b_biguint;
    /// // It cannot be compiled!
    /// println!("After a_biguint &= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator &= swallowed (took the ownership of) b_biguint.
    /// ```
    #[inline]
    fn bitand_assign(&mut self, rhs: Self)
    {
        self.and_assign(&rhs);
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `|` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after OR (|)
/// operation. In order to prevent this, the operands should be cloned or
/// copied before OR (|) operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait BitOr<Rhs = Self> {
///     type Output;
///     // Required method
///     fn bitor(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait BitOr<Rhs = Self> {
///     type Output;
///     // Required method
///     fn bitor(self, rhs: Rhs) -> Self::Output;
///   or
///     fn bitor(&self, rhs: Rhs) -> Self::Output;
///   or
///     fn bitor(self, rhs: &Rhs) -> Self::Output;
///   or
///     fn bitor(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait BitOr2, BitOr3, and BitOr4 are provided and
/// the developer implements none or only one of traits BitOr2, BitOr3,
/// and BitOr4.
/// 
/// ```
/// pub trait BitOr<Rhs = Self> {
///     type Output;
///     // Required method
///     fn bitor(self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait BitOr2<Rhs = Self> {
///     type Output;
///     // Required method
///     fn bitor(&self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait BitOr3<Rhs = Self> {
///     type Output;
///     // Required method
///     fn bitor(self, rhs: &Rhs) -> Self::Output;
/// }
/// 
/// pub trait BitOr4<Rhs = Self> {
///     type Output;
///     // Required method
///     fn bitor(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// Unlike trait BitOr, the trait PartialEq makes the operators
/// `==` and `!=` take not `&Self` but `Self` as its first operand and not
/// `&Rhs` (or `&Self`) but `Rhs` (or `Self`) as its second operand but makes
/// the functions eq() and ne() take not `self` but `&self` as its first
/// argument and not `Rhs` but `&Rhs` as its second argument.
/// So, I think the third suggestion is possible.
/// The prototype of trait PartialEq is as follows:
/// 
/// ```text
/// pub trait PartialEq<Rhs = Self>
/// where
/// Rhs: ?Sized,
/// {
///     // Required method
///     fn eq(&self, other: &Rhs) -> bool;
/// 
///     // Provided method
///     fn ne(&self, other: &Rhs) -> bool { ... }
/// }
/// ```
impl<T, const N: usize> BitOr for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    // fn bitor(self, rhs: Self) -> Self
    /// Performs the bitwise OR (|) operation, and then returns the result.
    /// 
    /// # Arguments
    /// - `rhs` is the reference of another object that OR (|) operation is
    ///   performed with.
    /// - `rhs` is of `Self` type.
    /// 
    /// # Output
    /// It returns the result after applying the bitwise OR (|) operation.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    /// let c_biguint = a_biguint.clone() | b_biguint.clone();
    /// 
    /// println!("{} | {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(c_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_11111111_00000000_11111111_11111111_11111111_11111111_11111111_11111111_11111111_00000000_00000000_10110011_11111111_11111111_11111111_11111111_00111111_10000000_11111111_00000000_10111011_11001111_11111111_11111111_11110000_11111111_11111111_11111111_00000000");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::max();
    /// let c_biguint = a_biguint.clone() | b_biguint.clone();
    /// 
    /// println!("{} | {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(c_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::zero();
    /// let c_biguint = a_biguint.clone() | b_biguint.clone();
    /// 
    /// println!("{} | {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(c_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    /// let _c_biguint = a_biguint | b_biguint;
    /// // It cannot be compiled!
    /// println!("{} | {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), _c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator | swallowed (took the ownership of) a_biguint and b_biguint.
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::max();
    /// let _c_biguint = a_biguint | b_biguint;
    /// // It cannot be compiled!
    /// println!("{} | {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), _c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator | swallowed (took the ownership of) a_biguint and b_biguint.
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::zero();
    /// let _c_biguint = a_biguint | b_biguint;
    /// // It cannot be compiled!
    /// println!("{} | {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), _c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator | swallowed (took the ownership of) a_biguint and b_biguint.
    /// ```
    #[inline]
    fn bitor(self, rhs: Self) -> Self
    {
        self.or(&rhs)
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `|=` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after Or (|=)
/// operation. In order to prevent this, the operands should be cloned or
/// copied before Or (|=) operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait BitOrAssign<Rhs = Self> {
///     // Required method
///     fn bitor_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait BitOrAssign<Rhs = Self> {
///     // Required method
///     fn bitor_assign(&mut self, rhs: Rhs);
///   or
///     fn bitor_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait BitOrAssign2 is provided and the developer
/// implements none or only one of traits BitOrAssign and BitOrAssign2.
/// 
/// ```
/// pub trait BitOrAssign<Rhs = Self> {
///     // Required method
///     fn bitor_assign(&mut self, rhs: Rhs);
/// }
/// 
/// pub trait BitOrAssign2<Rhs = Self> {
///     // Required method
///     fn bitor_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// Unlike trait BitOrAssign, the trait PartialEq makes the operators
/// `==` and `!=` take not `&Self` but `Self` as its first operand and not
/// `&Rhs` (or `&Self`) but `Rhs` (or `Self`) as its second operand but makes
/// the functions eq() and ne() take not `self` but `&self` as its first
/// argument and not `Rhs` but `&Rhs` as its second argument.
/// So, I think the third suggestion is possible.
/// The prototype of trait PartialEq is as follows:
/// 
/// ```text
/// pub trait PartialEq<Rhs = Self>
/// where
/// Rhs: ?Sized,
/// {
///     // Required method
///     fn eq(&self, other: &Rhs) -> bool;
/// 
///     // Provided method
///     fn ne(&self, other: &Rhs) -> bool { ... }
/// }
/// ```
impl<T, const N: usize> BitOrAssign for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // fn bitor_assign(&mut self, rhs: Self)
    /// Performs the bitwise OR (|) operation,
    /// and then assigns the result to `self` back.
    /// 
    /// # Arguments
    /// - `rhs` is the reference to another object that the OR (|) operation
    ///   is performed with.
    /// - `rhs` is of `Self` type.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    /// a_biguint |= b_biguint.clone();
    /// println!("After a_biguint |= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_11111111_00000000_11111111_11111111_11111111_11111111_11111111_11111111_11111111_00000000_00000000_10110011_11111111_11111111_11111111_11111111_00111111_10000000_11111111_00000000_10111011_11001111_11111111_11111111_11110000_11111111_11111111_11111111_00000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let b_biguint = U256::max();
    /// a_biguint |= b_biguint.clone();
    /// println!("After a_biguint |= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let b_biguint = U256::zero();
    /// a_biguint |= b_biguint.clone();
    /// println!("After a_biguint |= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    /// a_biguint |= b_biguint;
    /// // It cannot be compiled!
    /// println!("After a_biguint |= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator |= swallowed (took the ownership of) b_biguint.
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// let b_biguint = U256::max();
    /// a_biguint |= b_biguint;
    /// // It cannot be compiled!
    /// println!("After a_biguint |= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator |= swallowed (took the ownership of) b_biguint.
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// let b_biguint = U256::zero();
    /// a_biguint |= b_biguint;
    /// // It cannot be compiled!
    /// println!("After a_biguint |= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator |= swallowed (took the ownership of) b_biguint.
    /// ```
    #[inline]
    fn bitor_assign(&mut self, rhs: Self)
    {
        self.or_assign(&rhs);
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `^` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after XOR (^)
/// operation. In order to prevent this, the operands should be cloned or
/// copied before XOR (^) operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait BitXor<Rhs = Self> {
///     type Output;
///     // Required method
///     fn bitxor(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait BitXor<Rhs = Self> {
///     type Output;
///     // Required method
///     fn bitxor(self, rhs: Rhs) -> Self::Output;
///   or
///     fn bitxor(&self, rhs: Rhs) -> Self::Output;
///   or
///     fn bitxor(self, rhs: &Rhs) -> Self::Output;
///   or
///     fn bitxor(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait BitXor2, BitXor3, and BitXor4 are provided and
/// the developer implements none or only one of traits BitXor, BitXor2,
/// BitXor3, and BitXor4.
/// 
/// ```
/// pub trait BitXor<Rhs = Self> {
///     type Output;
///     // Required method
///     fn bitxor(self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait BitXor2<Rhs = Self> {
///     type Output;
///     // Required method
///     fn bitxor(&self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait BitXor3<Rhs = Self> {
///     type Output;
///     // Required method
///     fn bitxor(self, rhs: &Rhs) -> Self::Output;
/// }
/// 
/// pub trait BitXor4<Rhs = Self> {
///     type Output;
///     // Required method
///     fn bitxor(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// Unlike trait BitXor, the trait PartialEq makes the operators
/// `==` and `!=` take not `&Self` but `Self` as its first operand and not
/// `&Rhs` (or `&Self`) but `Rhs` (or `Self`) as its second operand but makes
/// the functions eq() and ne() take not `self` but `&self` as its first
/// argument and not `Rhs` but `&Rhs` as its second argument.
/// So, I think the third suggestion is possible.
/// The prototype of trait PartialEq is as follows:
/// 
/// ```text
/// pub trait PartialEq<Rhs = Self>
/// where
/// Rhs: ?Sized,
/// {
///     // Required method
///     fn eq(&self, other: &Rhs) -> bool;
/// 
///     // Provided method
///     fn ne(&self, other: &Rhs) -> bool { ... }
/// }
/// ```
impl<T, const N: usize> BitXor for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    // fn bitxor(self, rhs: Self) -> Self
    /// Performs the bitwise XOR (^) operation,
    /// and then assigns the result to `self` back.
    /// 
    /// # Arguments
    /// - `rhs` is the reference to another object that the AND (&) operation
    ///   is performed with.
    /// - `rhs` is of `Self` type.
    /// 
    /// # Output
    /// It returns the result after applying the bitwise XOR (^) operation.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    /// let c_biguint = a_biguint.clone() ^ b_biguint.clone();
    /// 
    /// println!("{} ^ {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(c_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "1010101_00110011_00001111_11111111_00000000_11111111_00000000_11111111_11111111_00000000_11111111_11111111_00000000_00000000_10110011_01110000_11110000_01111100_00001111_00111111_10000000_11111111_00000000_00011001_01000011_11111111_01111100_11110000_11000000_01111111_11111111_00000000");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::max();
    /// let c_biguint = a_biguint.clone() ^ b_biguint.clone();
    /// 
    /// println!("{} ^ {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(c_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "1010101_00110011_00001111_00000000_11111111_00000000_00000000_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_01001100_01110000_11110000_01111100_00001111_11000000_01111111_00000000_11111111");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::zero();
    /// let c_biguint = a_biguint.clone() ^ b_biguint.clone();
    /// 
    /// println!("{} ^ {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(c_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    /// let _c_biguint = a_biguint ^ b_biguint;
    /// // It cannot be compiled!
    /// println!("{} ^ {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), _c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator ^ swallowed (took the ownership of) a_biguint and b_biguint.
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::max();
    /// let _c_biguint = a_biguint ^ b_biguint;
    /// // It cannot be compiled!
    /// println!("{} ^ {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), _c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator ^ swallowed (took the ownership of) a_biguint and b_biguint.
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::zero();
    /// let _c_biguint = a_biguint ^ b_biguint;
    /// // It cannot be compiled!
    /// println!("{} ^ {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), _c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator ^ swallowed (took the ownership of) a_biguint and b_biguint.
    /// ```
    #[inline]
    fn bitxor(self, rhs: Self) -> Self
    {
        self.xor(&rhs)
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `^=` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after XOR (^=)
/// operation. In order to prevent this, the operands should be cloned or
/// copied before XOR (^=) operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait BitXorAssign<Rhs = Self> {
///     // Required method
///     fn bitxor_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait BitXorAssign<Rhs = Self> {
///     // Required method
///     fn bitxor_assign(&mut self, rhs: Rhs);
///   or
///     fn bitxor_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait BitXorAssign2 is provided and the developer
/// implements none or only one of traits BitXorAssign and BitXorAssign2.
/// 
/// ```
/// pub trait BitXorAssign<Rhs = Self> {
///     // Required method
///     fn bitxor_assign(&mut self, rhs: Rhs);
/// }
/// 
/// pub trait BitXorAssign2<Rhs = Self> {
///     // Required method
///     fn bitxor_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// Unlike trait BitXorAssign, the trait PartialEq makes the operators
/// `==` and `!=` take not `&Self` but `Self` as its first operand and not
/// `&Rhs` (or `&Self`) but `Rhs` (or `Self`) as its second operand but makes
/// the functions eq() and ne() take not `self` but `&self` as its first
/// argument and not `Rhs` but `&Rhs` as its second argument.
/// So, I think the third suggestion is possible.
/// The prototype of trait PartialEq is as follows:
/// 
/// ```text
/// pub trait PartialEq<Rhs = Self>
/// where
/// Rhs: ?Sized,
/// {
///     // Required method
///     fn eq(&self, other: &Rhs) -> bool;
/// 
///     // Provided method
///     fn ne(&self, other: &Rhs) -> bool { ... }
/// }
/// ```
impl<T, const N: usize> BitXorAssign for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString 
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // fn bitxor_assign(&mut self, rhs: Self)
    /// Performs the bitwise XOR (^) operation,
    /// and then assigns the result to `self` back.
    /// 
    /// # Arguments
    /// - `rhs` is the reference to another object that the AND (&) operation
    ///   is performed with.
    /// - `rhs` is of `&Self` type.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    /// a_biguint ^= b_biguint.clone();
    /// println!("After a_biguint ^= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "1010101_00110011_00001111_11111111_00000000_11111111_00000000_11111111_11111111_00000000_11111111_11111111_00000000_00000000_10110011_01110000_11110000_01111100_00001111_00111111_10000000_11111111_00000000_00011001_01000011_11111111_01111100_11110000_11000000_01111111_11111111_00000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let b_biguint = U256::max();
    /// a_biguint ^= b_biguint.clone();
    /// println!("After a_biguint ^= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "1010101_00110011_00001111_00000000_11111111_00000000_00000000_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_01001100_01110000_11110000_01111100_00001111_11000000_01111111_00000000_11111111");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let b_biguint = U256::zero();
    /// a_biguint ^= b_biguint.clone();
    /// println!("After a_biguint ^= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    /// a_biguint ^= b_biguint;
    /// // It cannot be compiled!
    /// println!("After a_biguint ^= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator ^= swallowed (took the ownership of) b_biguint.
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// let b_biguint = U256::max();
    /// a_biguint ^= b_biguint;
    /// // It cannot be compiled!
    /// println!("After a_biguint ^= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator ^= swallowed (took the ownership of) b_biguint.
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// let b_biguint = U256::zero();
    /// a_biguint ^= b_biguint;
    /// // It cannot be compiled!
    /// println!("After a_biguint ^= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator ^= swallowed (took the ownership of) b_biguint.
    /// ```
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self)
    {
        self.xor_assign(&rhs);
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `!` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after NOT (!)
/// operation. In order to prevent this, the operands should be cloned or
/// copied before NOT (!) operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait Not {
///     type Output;
/// 
///     // Required method
///     fn not(&self) -> Self::Output;
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait Not {
///     type Output;
///     // Required method
///     fn not(self) -> Self::Output;
///   or
///     fn not(&self) -> Self::Output;
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait Not2 is provided and
/// the developer implements none or only one of traits Not, Not2.
/// 
/// ```
/// pub trait Not {
///     type Output;
///     // Required method
///     fn not(self) -> Self::Output;
/// }
/// 
/// pub trait Not2 {
///     type Output;
///     // Required method
///     fn not(&self) -> Self::Output;
/// }
/// ```
/// 
/// Unlike trait Not, the trait PartialEq makes the operators
/// `==` and `!=` take not `&Self` but `Self` as its first operand and not
/// `&Rhs` (or `&Self`) but `Rhs` (or `Self`) as its second operand but makes
/// the functions eq() and ne() take not `self` but `&self` as its first
/// argument and not `Rhs` but `&Rhs` as its second argument.
/// So, I think the third suggestion is possible.
/// The prototype of trait PartialEq is as follows:
/// 
/// ```text
/// pub trait PartialEq<Rhs = Self>
/// where
/// Rhs: ?Sized,
/// {
///     // Required method
///     fn eq(&self, other: &Rhs) -> bool;
/// 
///     // Provided method
///     fn ne(&self, other: &Rhs) -> bool { ... }
/// }
/// ```
impl<T, const N: usize> Not for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    // fn not(self) -> Self
    /// Performs the bitwise NOT (!) operation, and then returns the result.
    /// 
    /// # Output
    /// It returns the result after applying the bitwise NOT operation.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let res = !a_biguint.clone();
    /// println!("! {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "1010101_00110011_00001111_00000000_11111111_00000000_00000000_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_01001100_01110000_11110000_01111100_00001111_11000000_01111111_00000000_11111111");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::max();
    /// let res = !a_biguint.clone();
    /// println!("! {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::zero();
    /// let res = !a_biguint.clone();
    /// println!("! {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let _res = !a_biguint;
    /// // It cannot be compiled!
    /// println!("! {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), _res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator ! swallowed (took the ownership of) a_biguint.
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let _res = !a_biguint;
    /// // It cannot be compiled!
    /// println!("! {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), _res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator ! swallowed (took the ownership of) a_biguint.
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let _res = !a_biguint;
    /// // It cannot be compiled!
    /// println!("! {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), _res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator ! swallowed (took the ownership of) a_biguint.
    /// ```
    #[inline]
    fn not(self) -> Self
    {
        self.flip()
    }
}



/// Trait for comparisons using the equality operator.
/// - Implementing this trait for types provides the == and != operators
///   for those types.
/// - x.eq(y) can also be written x == y, and x.ne(y) can be written x != y.
/// - This trait allows for comparisons using the equality operator,
///   for types that do not have a full equivalence relation.
/// For more information, [read more](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html#).
impl<T, U, const N: usize> PartialEq<U> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd,
    U: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
        + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
        + Rem<Output=U> + RemAssign
        + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
        + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
        + BitXor<Output=U> + BitXorAssign + Not<Output=U>
        + PartialEq + PartialOrd
{
    // fn eq(&self, other: &U) -> bool
    /// Compares `self` and `other` to find whether `self` is equal to `other`.
    /// 
    /// # Arguments
    /// `rhs` is to be compared with `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `true` if `self` is equal to `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_uint = 100_u8;
    /// let res = a_biguint == b_uint;
    /// if res
    ///     { println!("{} == {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} != {}", a_biguint, b_uint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_uint = 200_u8;
    /// let res = a_biguint == b_uint;
    /// if res
    ///     { println!("{} == {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} != {}", a_biguint, b_uint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_uint = 100_u8;
    /// let res = a_biguint != b_uint;
    /// if res
    ///     { println!("{} != {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} == {}", a_biguint, b_uint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_uint = 200_u8;
    /// let res = a_biguint != b_uint;
    /// if res
    ///     { println!("{} != {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} == {}", a_biguint, b_uint); }
    /// assert_eq!(res, true);
    /// ```
    #[inline]
    fn eq(&self, other: &U) -> bool
    {
        self.eq_uint(*other)
    }
}



/// Trait for comparisons using the equality operator.
/// - Implementing this trait for types provides the == and != operators
///   for those types.
/// - x.eq(y) can also be written x == y, and x.ne(y) can be written x != y.
/// - This trait allows for comparisons using the equality operator,
///   for types that do not have a full equivalence relation.
/// For more information, [read more](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html#).
impl<T, const N: usize> PartialEq for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // fn eq(&self, other: &Self) -> bool
    /// Compare `self` with `other` to find whether `self` is equal to `other`.
    /// 
    /// # Arguments
    /// `rhs` is to be compared with `self`, and is of `&Self` type.
    /// 
    /// # Output
    /// It returns `true` if `self` is equal to `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_string(num_str).unwrap();
    /// let res = a_biguint == b_biguint;
    /// if res
    ///     { println!("{} = {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} != {}", a_biguint, b_biguint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_uint(100_u8);
    /// let res = a_biguint == b_biguint;
    /// if res
    ///     { println!("{} = {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} != {}", a_biguint, b_biguint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_string(num_str).unwrap();
    /// let res = a_biguint != b_biguint;
    /// if res
    ///     { println!("{} != {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} == {}", a_biguint, b_biguint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_uint(100_u8);
    /// let res = a_biguint != b_biguint;
    /// if res
    ///     { println!("{} != {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} == {}", a_biguint, b_biguint); }
    /// assert_eq!(res, true);
    /// ```
    #[inline]
    fn eq(&self, other: &Self) -> bool
    {
        self.eq(other)
    }
}



impl<T, U, const N: usize> PartialOrd<U> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd,
    U: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
        + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
        + Rem<Output=U> + RemAssign
        + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
        + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
        + BitXor<Output=U> + BitXorAssign + Not<Output=U>
        + PartialEq + PartialOrd
{
    // fn partial_cmp(&self, other: &U) -> Option<Ordering>
    /// __self < other -> bool__
    /// 
    /// Compares `self` and `other` to find whether `self` is less than `other`.
    /// 
    /// # Arguments
    /// `rhs` is to be compared with `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `true` if `self` is less than `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = UU32::from_uint(200_u8);
    /// let b_uint = 100_u8;
    /// let res = a_biguint < b_uint;
    /// if res
    ///     { println!("{} < {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} >= {}", a_biguint, b_uint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_uint = 200_u8;
    /// let res = a_biguint < b_uint;
    /// if res
    ///     { println!("{} < {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} >= {}", a_biguint, b_uint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_uint = 100_u8;
    /// let res = a_biguint < b_uint;
    /// if res
    ///     { println!("{} < {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} >= {}", a_biguint, b_uint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// 
    /// __self > other -> bool__
    /// 
    /// Compares `self` and `other` to find whether `self` is greater
    /// than `other`.
    /// 
    /// # Arguments
    /// `rhs` is to be compared with `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `true` if `self` is greater than `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = UU32::from_uint(200_u8);
    /// let b_uint = 100_u8;
    /// let res = a_biguint > b_uint;
    /// if res
    ///     { println!("{} > {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} <= {}", a_biguint, b_uint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_uint = 200_u8;
    /// let res = a_biguint > b_uint;
    /// if res
    ///     { println!("{} > {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} <= {}", a_biguint, b_uint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_uint = 100_u8;
    /// let res = a_biguint > b_uint;
    /// if res
    ///     { println!("{} > {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} <= {}", a_biguint, b_uint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// 
    /// __self <= other -> bool__
    /// 
    /// Compares `self` and `other` to find whether `self` is less than or
    /// equal to `other`.
    /// 
    /// # Arguments
    /// `rhs` is to be compared with `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `true` if `self` is less than or equal to `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = UU32::from_uint(200_u8);
    /// let b_uint = 100_u8;
    /// let res = a_biguint <= b_uint;
    /// if res
    ///     { println!("{} <= {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} > {}", a_biguint, b_uint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_uint = 200_u8;
    /// let res = a_biguint <= b_uint;
    /// if res
    ///     { println!("{} <= {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} > {}", a_biguint, b_uint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_uint = 100_u8;
    /// let res = a_biguint <= b_uint;
    /// if res
    ///     { println!("{} <= {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} > {}", a_biguint, b_uint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// 
    /// __self >= other -> bool__
    /// 
    /// Compares `self` and `other` to find whether `self` is greater than
    /// or equal to `other`.
    /// 
    /// # Arguments
    /// `rhs` is to be compared with `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `true` if `self` is greater than or equal to `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = UU32::from_uint(200_u8);
    /// let b_uint = 100_u8;
    /// let res = a_biguint >= b_uint;
    /// if res
    ///     { println!("{} >= {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} < {}", a_biguint, b_uint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_uint = 200_u8;
    /// let res = a_biguint >= b_uint;
    /// if res
    ///     { println!("{} >= {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} < {}", a_biguint, b_uint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_uint = 100_u8;
    /// let res = a_biguint >= b_uint;
    /// if res
    ///     { println!("{} >= {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} < {}", a_biguint, b_uint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// 
    /// __self == other -> bool__
    /// 
    /// Compares `self` and `other` to find whether `self` is equal to `other`.
    /// 
    /// # Arguments
    /// `rhs` is to be compared with `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `true` if `self` is equal to `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_uint = 100_u8;
    /// let res = a_biguint == b_uint;
    /// if res
    ///     { println!("{} == {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} != {}", a_biguint, b_uint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_uint = 200_u8;
    /// let res = a_biguint == b_uint;
    /// if res
    ///     { println!("{} == {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} != {}", a_biguint, b_uint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// 
    /// __self != other -> bool__
    /// 
    /// Compares `self` and `other` to find whether `self` is equal to `other`.
    /// 
    /// # Arguments
    /// `rhs` is to be compared with `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `true` if `self` is not equal to `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_uint = 100_u8;
    /// let res = a_biguint != b_uint;
    /// if res
    ///     { println!("{} != {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} == {}", a_biguint, b_uint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_uint = 200_u8;
    /// let res = a_biguint != b_uint;
    /// if res
    ///     { println!("{} != {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} == {}", a_biguint, b_uint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    fn partial_cmp(&self, other: &U) -> Option<Ordering>
    {
        self.partial_cmp_uint(*other)
    }
}



impl<T, const N: usize> PartialOrd for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    /// __self < other -> bool__
    /// 
    /// Compares `self` and `other` to find whether `self` is less than `other`.
    /// 
    /// # Arguments
    /// `rhs` is to be compared with `self`, and is of `Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `true` if `self` is less than `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_uint(100_u8);
    /// let res = a_biguint < b_biguint;
    /// if res
    ///     { println!("{} < {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} >= {}", a_biguint, b_biguint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_biguint = UU32::from_string(num_str).unwrap();
    /// let res = a_biguint < b_biguint;
    /// if res
    ///     { println!("{} < {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} >= {}", a_biguint, b_biguint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_string(num_str).unwrap();
    /// let res = a_biguint < b_biguint;
    /// if res
    ///     { println!("{} < {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} >= {}", a_biguint, b_biguint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// 
    /// __self > other -> bool__
    /// 
    /// Compares `self` and `other` to find whether `self` is greater
    /// than `other`.
    /// 
    /// # Arguments
    /// `rhs` is to be compared with `self`, and is of `Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `true` if `self` is greater than `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_uint(100_u8);
    /// let res = a_biguint > b_biguint;
    /// if res
    ///     { println!("{} > {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} <= {}", a_biguint, b_biguint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_biguint = UU32::from_string(num_str).unwrap();
    /// let res = a_biguint > b_biguint;
    /// if res
    ///     { println!("{} > {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} <= {}", a_biguint, b_biguint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_string(num_str).unwrap();
    /// let res = a_biguint > b_biguint;
    /// if res
    ///     { println!("{} > {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} <= {}", a_biguint, b_biguint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// 
    /// __self <= other -> bool__
    /// 
    /// Compares `self` and `other` to find whether `self` is less than or
    /// equal to `other`.
    /// 
    /// # Arguments
    /// `rhs` is to be compared with `self`, and is of `Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `true` if `self` is less than or equal to `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_uint(100_u8);
    /// let res = a_biguint <= b_biguint;
    /// if res
    ///     { println!("{} <= {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} > {}", a_biguint, b_biguint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_biguint = UU32::from_string(num_str).unwrap();
    /// let res = a_biguint <= b_biguint;
    /// if res
    ///     { println!("{} <= {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} > {}", a_biguint, b_biguint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_string(num_str).unwrap();
    /// let res = a_biguint <= b_biguint;
    /// if res
    ///     { println!("{} <= {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} > {}", a_biguint, b_biguint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// 
    /// __self >= other -> bool__
    /// Compares `self` and `other` to find whether `self` is greater than
    /// or equal to `other`.
    /// 
    /// # Arguments
    /// `rhs` is to be compared with `self`, and is of `Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `true` if `self` is greater than or equal to `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_uint(100_u8);
    /// let res = a_biguint >= b_biguint;
    /// if res
    ///     { println!("{} >= {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} < {}", a_biguint, b_biguint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_biguint = UU32::from_string(num_str).unwrap();
    /// let res = a_biguint >= b_biguint;
    /// if res
    ///     { println!("{} >= {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} < {}", a_biguint, b_biguint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_string(num_str).unwrap();
    /// let res = a_biguint >= b_biguint;
    /// if res
    ///     { println!("{} >= {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} < {}", a_biguint, b_biguint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// 
    /// __self == other -> bool__
    /// Compares `self` and `other` to find whether `self` is equal to `other`.
    /// 
    /// # Arguments
    /// `rhs` is to be compared with `self`, and is of `Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `true` if `self` is equal to `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_string(num_str).unwrap();
    /// let res = a_biguint == b_biguint;
    /// if res
    ///     { println!("{} = {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} != {}", a_biguint, b_biguint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_uint(100_u8);
    /// let res = a_biguint == b_biguint;
    /// if res
    ///     { println!("{} = {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} != {}", a_biguint, b_biguint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// 
    /// __self != other -> bool__
    /// Compares `self` and `other` to find whether `self` is equal to `other`.
    /// 
    /// # Arguments
    /// `rhs` is to be compared with `self`, and is of `Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `true` if `self` is not equal to `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_string(num_str).unwrap();
    /// let res = a_biguint != b_biguint;
    /// if res
    ///     { println!("{} != {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} == {}", a_biguint, b_biguint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_uint(100_u8);
    /// let res = a_biguint != b_biguint;
    /// if res
    ///     { println!("{} != {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} == {}", a_biguint, b_biguint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        self.partial_cmp(other)
    }
}



impl<T, U, const N: usize> From<U> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd,
    U: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
        + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
        + Rem<Output=U> + RemAssign
        + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
        + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
        + BitXor<Output=U> + BitXorAssign + Not<Output=U>
        + PartialEq + PartialOrd
{
    // fn from(val: U) -> Self
    /// Constructs a new `BigUInt<T, N>`-type object from a primitive unsigned
    /// integer such as `u8`, `u16`, `u32`, `u64`, `u128` and `usize`.
    /// 
    /// # Argument
    /// `val` is a primitive unsigned integer
    /// which will be converted into `BigUInt`.
    /// 
    /// # Features
    /// The method Self::from() is the same of the method Self::from_uint().
    /// 
    /// # Output
    /// A new `BigUInt<T, N>`-type object
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from(123456789123456789123456789123456789_u128);
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "123456789123456789123456789123456789");
    /// ```
    #[inline]
    fn from(val: U) -> Self
    {
        Self::from_uint(val)
    }
}



impl<T, const N: usize> From<[T; N]> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // fn from(val: [T; N]) -> Self
    /// Constructs a new `BigUInt<T, N>`-type object from an array of type `T`
    /// with `N` elements.
    /// 
    /// # Argument
    /// `val` is a primitive unsigned integer
    /// which will be converted into `BigUInt`.
    /// 
    /// # Features
    /// The method Self::from() is the same of the method Self::from_uint().
    /// 
    /// # Output
    /// A new `BigUInt<T, N>`-type object
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from([1, 2, 3, 4, 5, 6, 7, 8]);
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "215679573381144830513811895868694400695694534256768036697775454289921");
    /// ```
    fn from(val: [T; N]) -> Self
    {
        Self::from_array(val)
    }
}



impl<T, const N: usize> FromStr for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Err = NumberErr;

    // fn from_str(s: &str) -> Result<Self, Self::Err>
    /// Constructs a new `BigUInt<T, N>`-type object from a string with radix 10.
    /// 
    /// # Argument
    /// `s` is a text (string) slice, which `BigUInt<T, N>`-type object is
    /// constructed from the string slice `s`.
    /// 
    /// # Features
    /// The constructed object will be wrapped in `Ok(BigUInt<T, N>)` if it is
    /// successfully created.
    /// Otherwise, this method returns `Err(NumberErr::ParsingError)`.
    /// And, if you import (use) std::str::FromStr,
    /// you can automagically use `str::parse::<BigUInt>()` too.
    /// 
    /// # Output
    /// A new `BigUInt<T, N>`-type object constructed
    /// from a string with radix 10.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint_wrapped = U256::from_str("215679573381144830513811895868694400695694534256768036697775454289921");
    /// match a_biguint_wrapped
    /// {
    ///     Ok(a_biguint) => {
    ///             println!("a_biguint = {}", a_biguint);
    ///             assert_eq!(a_biguint.to_string(), "215679573381144830513811895868694400695694534256768036697775454289921");
    ///         },
    ///     Err(e) => { println!("Error: {}", e); }
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::NumberErr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint_wrapped = U256::from_str("@!#$%^&*()_+=-|-/?><`~");
    /// match a_biguint_wrapped
    /// {
    ///     Ok(a_biguint) => { println!("a_biguint = {}", a_biguint); },
    ///     Err(e) => {
    ///             println!("Error: {}", e);
    ///             assert_eq!(e, NumberErr::NotAlphaNumeric);
    ///         }
    /// }
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint_wrapped = "215679573381144830513811895868694400695694534256768036697775454289921".parse::<U256>();
    /// match a_biguint_wrapped
    /// {
    ///     Ok(a_biguint) => {
    ///             println!("a_biguint = {}", a_biguint);
    ///             assert_eq!(a_biguint.to_string(), "215679573381144830513811895868694400695694534256768036697775454289921");
    ///         },
    ///     Err(e) => { println!("Error: {}", e); }
    /// }
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::NumberErr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint_wrapped = "@!#$%^&*()_+=-|-/?><`~".parse::<U256>();
    /// match a_biguint_wrapped
    /// {
    ///     Ok(a_biguint) => { println!("a_biguint = {}", a_biguint); },
    ///     Err(e) => {
    ///             println!("Error: {}", e);
    ///             assert_eq!(e, NumberErr::NotAlphaNumeric);
    ///         }
    /// }
    /// ```
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        Self::from_string(s)
    }
}



/// Format trait for an empty format, {}.
/// - Implementing this trait for a type will automatically implement the
///   ToString trait for the type, allowing the usage of the .to_string()
///   method.
/// - Prefer implementing the Display trait for a type, rather than ToString.
/// - Display is similar to Debug, but Display is for user-facing output,
///   and so cannot be derived.
/// - For more information on the trait Display,
///   [read more](https://doc.rust-lang.org/std/fmt/trait.Display.html).
/// - For more information on formatters,
///   see [the module-level documentation](https://doc.rust-lang.org/std/fmt/index.html).
impl<T, const N: usize> Display for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
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
    /// # Errors
    /// This function should return Err if, and only if, the provided Formatter
    /// returns Err. String formatting is considered an infallible operation;
    /// this function only returns a Result because writing to the underlying
    /// stream might fail and it must provide a way to propagate the fact that
    /// an error has occurred back up the stack.
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
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:>80}", a_biguint);
    /// let txt = format!("{:>80}", a_biguint);
    /// assert_eq!(txt, "   69743176821145534028236692093846345739169743176821145534028236692093846345739");
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:>080}", a_biguint);
    /// let txt = format!("{:>080}", a_biguint);
    /// assert_eq!(txt, "00069743176821145534028236692093846345739169743176821145534028236692093846345739");
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:^80}", a_biguint);
    /// let txt = format!("{:^80}", a_biguint);
    /// assert_eq!(txt, " 69743176821145534028236692093846345739169743176821145534028236692093846345739  ");
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:^080}", a_biguint);
    /// let txt = format!("{:^080}", a_biguint);
    /// assert_eq!(txt, "00069743176821145534028236692093846345739169743176821145534028236692093846345739");
    /// ```
    /// 
    /// # Example 6
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:<80}", a_biguint);
    /// let txt = format!("{:<80}", a_biguint);
    /// assert_eq!(txt, "69743176821145534028236692093846345739169743176821145534028236692093846345739   ");
    /// ```
    /// 
    /// # Example 7
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:<080}", a_biguint);
    /// let txt = format!("{:<080}", a_biguint);
    /// assert_eq!(txt, "00069743176821145534028236692093846345739169743176821145534028236692093846345739");
    /// ```
    /// 
    /// # Example 8
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:!>80}", a_biguint);
    /// let txt = format!("{:!>80}", a_biguint);
    /// assert_eq!(txt, "!!!69743176821145534028236692093846345739169743176821145534028236692093846345739");
    /// ```
    /// 
    /// # Example 9
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:@>080}", a_biguint);
    /// let txt = format!("{:@>080}", a_biguint);
    /// assert_eq!(txt, "00069743176821145534028236692093846345739169743176821145534028236692093846345739");
    /// ```
    /// 
    /// # Example 10
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:#^80}", a_biguint);
    /// let txt = format!("{:#^80}", a_biguint);
    /// assert_eq!(txt, "#69743176821145534028236692093846345739169743176821145534028236692093846345739##");
    /// ```
    /// 
    /// # Example 11
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:$^080}", a_biguint);
    /// let txt = format!("{:$^080}", a_biguint);
    /// assert_eq!(txt, "00069743176821145534028236692093846345739169743176821145534028236692093846345739");
    /// ```
    /// 
    /// # Example 12
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:%<80}", a_biguint);
    /// let txt = format!("{:%<80}", a_biguint);
    /// assert_eq!(txt, "69743176821145534028236692093846345739169743176821145534028236692093846345739%%%");
    /// ```
    /// 
    /// # Example 13
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:^<080}", a_biguint);
    /// let txt = format!("{:^<080}", a_biguint);
    /// assert_eq!(txt, "00069743176821145534028236692093846345739169743176821145534028236692093846345739");
    /// ```
    /// 
    /// # Example 14
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
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>
    {
        fmt_with_radix!(self, f, 10, "");
    }
}


/// X formatting.
/// - The UpperHex trait should format its output as a number in hexadecimal,
///   with A through F in upper case.
/// - The alternate flag, #, adds a 0X in front of the output.
/// - For more information on the trait UpperHex,
///   [read more](https://doc.rust-lang.org/std/fmt/trait.UpperHex.html).
/// - For more information on formatters,
///   see [the module-level documentation](https://doc.rust-lang.org/std/fmt/index.html).
impl<T, const N: usize> UpperHex for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // fn fmt(&self, f: &mut Formatter) -> fmt::Result
    /// Formats the value using the given formatter.
    /// 
    /// # Arguments
    /// `f` is a buffer, this method must write the formatted string into it,
    /// and is of the type `&mut Formatter`.
    /// 
    /// # Errors
    /// This function should return Err if, and only if, the provided Formatter
    /// returns Err. String formatting is considered an infallible operation;
    /// this function only returns a Result because writing to the underlying
    /// stream might fail and it must provide a way to propagate the fact that
    /// an error has occurred back up the stack.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:X}", a_biguint);
    /// let txt = format!("{:X}", a_biguint);
    /// assert_eq!(txt, "9A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:80X}", a_biguint);
    /// let txt = format!("{:80X}", a_biguint);
    /// assert_eq!(txt, "9A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B                ");
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:080X}", a_biguint);
    /// let txt = format!("{:080X}", a_biguint);
    /// assert_eq!(txt, "00000000000000009A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:#X}", a_biguint);
    /// let txt = format!("{:#X}", a_biguint);
    /// assert_eq!(txt, "0X9A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:#80X}", a_biguint);
    /// let txt = format!("{:#80X}", a_biguint);
    /// assert_eq!(txt, "0X9A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B              ");
    /// ```
    /// 
    /// # Example 6
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:#080X}", a_biguint);
    /// let txt = format!("{:#080X}", a_biguint);
    /// assert_eq!(txt, "0X000000000000009A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");
    /// ```
    /// 
    /// # Example 7
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:>80X}", a_biguint);
    /// let txt = format!("{:>80X}", a_biguint);
    /// assert_eq!(txt, "                9A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");
    /// ```
    /// 
    /// # Example 8
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:>080X}", a_biguint);
    /// let txt = format!("{:>080X}", a_biguint);
    /// assert_eq!(txt, "00000000000000009A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");
    /// ```
    /// 
    /// # Example 9
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:^80X}", a_biguint);
    /// let txt = format!("{:^80X}", a_biguint);
    /// assert_eq!(txt, "        9A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B        ");
    /// ```
    /// 
    /// # Example 10
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:^080X}", a_biguint);
    /// let txt = format!("{:^080X}", a_biguint);
    /// assert_eq!(txt, "00000000000000009A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");
    /// ```
    /// 
    /// # Example 11
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:<80X}", a_biguint);
    /// let txt = format!("{:<80X}", a_biguint);
    /// assert_eq!(txt, "9A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B                ");
    /// ```
    /// 
    /// # Example 12
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:<080X}", a_biguint);
    /// let txt = format!("{:<080X}", a_biguint);
    /// assert_eq!(txt, "00000000000000009A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");
    /// ```
    /// 
    /// # Example 13
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:!>80X}", a_biguint);
    /// let txt = format!("{:!>80X}", a_biguint);
    /// assert_eq!(txt, "!!!!!!!!!!!!!!!!9A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");
    /// ```
    /// 
    /// # Example 14
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:@>080X}", a_biguint);
    /// let txt = format!("{:@>080X}", a_biguint);
    /// assert_eq!(txt, "00000000000000009A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");
    /// ```
    /// 
    /// # Example 15
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:#^80X}", a_biguint);
    /// let txt = format!("{:#^80X}", a_biguint);
    /// assert_eq!(txt, "########9A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B########");
    /// ```
    /// 
    /// # Example 16
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:$^080X}", a_biguint);
    /// let txt = format!("{:$^080X}", a_biguint);
    /// assert_eq!(txt, "00000000000000009A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");
    /// ```
    /// 
    /// # Example 17
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:%<80X}", a_biguint);
    /// let txt = format!("{:%<80X}", a_biguint);
    /// assert_eq!(txt, "9A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B%%%%%%%%%%%%%%%%");
    /// ```
    /// 
    /// # Example 18
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:*<080X}", a_biguint);
    /// let txt = format!("{:*<080X}", a_biguint);
    /// assert_eq!(txt, "00000000000000009A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");
    /// ```
    /// 
    /// # Example 19
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:>#80X}", a_biguint);
    /// let txt = format!("{:>#80X}", a_biguint);
    /// assert_eq!(txt, "              0X9A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");
    /// ```
    /// 
    /// # Example 20
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:>#080X}", a_biguint);
    /// let txt = format!("{:>#080X}", a_biguint);
    /// assert_eq!(txt, "0X000000000000009A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");
    /// ```
    /// 
    /// # Example 21
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:^#80X}", a_biguint);
    /// let txt = format!("{:^#80X}", a_biguint);
    /// assert_eq!(txt, "       0X9A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B       ");
    /// ```
    /// 
    /// # Example 22
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:^#080X}", a_biguint);
    /// let txt = format!("{:^#080X}", a_biguint);
    /// assert_eq!(txt, "0X000000000000009A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");
    /// ```
    /// 
    /// # Example 23
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:<#80X}", a_biguint);
    /// let txt = format!("{:<#80X}", a_biguint);
    /// assert_eq!(txt, "0X9A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B              ");
    /// ```
    /// 
    /// # Example 24
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:<#080X}", a_biguint);
    /// let txt = format!("{:<#080X}", a_biguint);
    /// assert_eq!(txt, "0X000000000000009A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");
    /// ```
    /// 
    /// # Example 25
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:!>#80X}", a_biguint);
    /// let txt = format!("{:!>#80X}", a_biguint);
    /// assert_eq!(txt, "!!!!!!!!!!!!!!0X9A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");
    /// ```
    /// 
    /// # Example 26
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:@>#080X}", a_biguint);
    /// let txt = format!("{:@>#080X}", a_biguint);
    /// assert_eq!(txt, "0X000000000000009A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");
    /// ```
    /// 
    /// # Example 27
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:#^#80X}", a_biguint);
    /// let txt = format!("{:#^#80X}", a_biguint);
    /// assert_eq!(txt, "#######0X9A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B#######");
    /// ```
    /// 
    /// # Example 28
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:$^#080X}", a_biguint);
    /// let txt = format!("{:$^#080X}", a_biguint);
    /// assert_eq!(txt, "0X000000000000009A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");
    /// ```
    /// 
    /// # Example 29
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:%<#80X}", a_biguint);
    /// let txt = format!("{:%<#80X}", a_biguint);
    /// assert_eq!(txt, "0X9A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B%%%%%%%%%%%%%%");
    /// ```
    /// 
    /// # Example 30
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:^<#080X}", a_biguint);
    /// let txt = format!("{:^<#080X}", a_biguint);
    /// assert_eq!(txt, "0X000000000000009A313D3C55B12353228024CDBE156C443888E78AD3E424DBC43036A7788AD80B");
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
    {
        fmt_with_radix!(self, f, 16, "0X");
    }
}



/// x formatting.
/// - The LowerHex trait should format its output as a number in hexadecimal,
///   with a through f in lower case.
/// - The alternate flag, #, adds a 0x in front of the output.
/// - For more information on the trait LowerHex,
///   [read more](https://doc.rust-lang.org/std/fmt/trait.LowerHex.html).
/// - For more information on formatters,
///   see [the module-level documentation](https://doc.rust-lang.org/std/fmt/index.html).
impl<T, const N: usize> LowerHex for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // fn fmt(&self, f: &mut Formatter) -> fmt::Result
    /// Formats the value using the given formatter.
    /// 
    /// # Arguments
    /// `f` is a buffer, this method must write the formatted string into it,
    /// and is of the type `&mut Formatter`.
    /// 
    /// # Errors
    /// This function should return Err if, and only if, the provided Formatter
    /// returns Err. String formatting is considered an infallible operation;
    /// this function only returns a Result because writing to the underlying
    /// stream might fail and it must provide a way to propagate the fact that
    /// an error has occurred back up the stack.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:x}", a_biguint);
    /// let txt = format!("{:x}", a_biguint);
    /// assert_eq!(txt, "9a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:80x}", a_biguint);
    /// let txt = format!("{:80x}", a_biguint);
    /// assert_eq!(txt, "9a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b                ");
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:080x}", a_biguint);
    /// let txt = format!("{:080x}", a_biguint);
    /// assert_eq!(txt, "00000000000000009a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:#x}", a_biguint);
    /// let txt = format!("{:#x}", a_biguint);
    /// assert_eq!(txt, "0x9a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:#80x}", a_biguint);
    /// let txt = format!("{:#80x}", a_biguint);
    /// assert_eq!(txt, "0x9a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b              ");
    /// ```
    /// 
    /// # Example 6
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:#080x}", a_biguint);
    /// let txt = format!("{:#080x}", a_biguint);
    /// assert_eq!(txt, "0x000000000000009a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");
    /// ```
    /// 
    /// # Example 7
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:>80x}", a_biguint);
    /// let txt = format!("{:>80x}", a_biguint);
    /// assert_eq!(txt, "                9a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");
    /// ```
    /// 
    /// # Example 8
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:>080x}", a_biguint);
    /// let txt = format!("{:>080x}", a_biguint);
    /// assert_eq!(txt, "00000000000000009a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");
    /// ```
    /// 
    /// # Example 9
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:^80x}", a_biguint);
    /// let txt = format!("{:^80x}", a_biguint);
    /// assert_eq!(txt, "        9a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b        ");
    /// ```
    /// 
    /// # Example 10
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:^080x}", a_biguint);
    /// let txt = format!("{:^080x}", a_biguint);
    /// assert_eq!(txt, "00000000000000009a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");
    /// ```
    /// 
    /// # Example 11
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:<80x}", a_biguint);
    /// let txt = format!("{:<80x}", a_biguint);
    /// assert_eq!(txt, "9a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b                ");
    /// ```
    /// 
    /// # Example 12
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:<080x}", a_biguint);
    /// let txt = format!("{:<080x}", a_biguint);
    /// assert_eq!(txt, "00000000000000009a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");
    /// ```
    /// 
    /// # Example 13
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:!>80x}", a_biguint);
    /// let txt = format!("{:!>80x}", a_biguint);
    /// assert_eq!(txt, "!!!!!!!!!!!!!!!!9a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");
    /// ```
    /// 
    /// # Example 14
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:@>080x}", a_biguint);
    /// let txt = format!("{:@>080x}", a_biguint);
    /// assert_eq!(txt, "00000000000000009a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");
    /// ```
    /// 
    /// # Example 15
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:#^80x}", a_biguint);
    /// let txt = format!("{:#^80x}", a_biguint);
    /// assert_eq!(txt, "########9a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b########");
    /// ```
    /// 
    /// # Example 16
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:$^080x}", a_biguint);
    /// let txt = format!("{:$^080x}", a_biguint);
    /// assert_eq!(txt, "00000000000000009a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");
    /// ```
    /// 
    /// # Example 17
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:%<80x}", a_biguint);
    /// let txt = format!("{:%<80x}", a_biguint);
    /// assert_eq!(txt, "9a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b%%%%%%%%%%%%%%%%");
    /// ```
    /// 
    /// # Example 18
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:*<080x}", a_biguint);
    /// let txt = format!("{:*<080x}", a_biguint);
    /// assert_eq!(txt, "00000000000000009a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");
    /// ```
    /// 
    /// # Example 19
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:>#80x}", a_biguint);
    /// let txt = format!("{:>#80x}", a_biguint);
    /// assert_eq!(txt, "              0x9a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");
    /// ```
    /// 
    /// # Example 20
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:>#080x}", a_biguint);
    /// let txt = format!("{:>#080x}", a_biguint);
    /// assert_eq!(txt, "0x000000000000009a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");
    /// ```
    /// 
    /// # Example 21
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:^#80x}", a_biguint);
    /// let txt = format!("{:^#80x}", a_biguint);
    /// assert_eq!(txt, "       0x9a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b       ");
    /// ```
    /// 
    /// # Example 22
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:^#080x}", a_biguint);
    /// let txt = format!("{:^#080x}", a_biguint);
    /// assert_eq!(txt, "0x000000000000009a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");
    /// ```
    /// 
    /// # Example 23
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:<#80x}", a_biguint);
    /// let txt = format!("{:<#80x}", a_biguint);
    /// assert_eq!(txt, "0x9a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b              ");
    /// ```
    /// 
    /// # Example 24
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:<#080x}", a_biguint);
    /// let txt = format!("{:<#080x}", a_biguint);
    /// assert_eq!(txt, "0x000000000000009a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");
    /// ```
    /// 
    /// # Example 25
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:!>#80x}", a_biguint);
    /// let txt = format!("{:!>#80x}", a_biguint);
    /// assert_eq!(txt, "!!!!!!!!!!!!!!0x9a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");
    /// ```
    /// 
    /// # Example 26
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:@>#080x}", a_biguint);
    /// let txt = format!("{:@>#080x}", a_biguint);
    /// assert_eq!(txt, "0x000000000000009a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");
    /// ```
    /// 
    /// # Example 27
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:#^#80x}", a_biguint);
    /// let txt = format!("{:#^#80x}", a_biguint);
    /// assert_eq!(txt, "#######0x9a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b#######");
    /// ```
    /// 
    /// # Example 28
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:$^#080x}", a_biguint);
    /// let txt = format!("{:$^#080x}", a_biguint);
    /// assert_eq!(txt, "0x000000000000009a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");
    /// ```
    /// 
    /// # Example 29
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:%<#80x}", a_biguint);
    /// let txt = format!("{:%<#80x}", a_biguint);
    /// assert_eq!(txt, "0x9a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b%%%%%%%%%%%%%%");
    /// ```
    /// 
    /// # Example 30
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:^<#080x}", a_biguint);
    /// let txt = format!("{:^<#080x}", a_biguint);
    /// assert_eq!(txt, "0x000000000000009a313d3c55b12353228024cdbe156c443888e78ad3e424dbc43036a7788ad80b");
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
    {
        fmt_with_radix!(self, f, 16, "0x", true);
    }
}



/// b formatting.
/// - The Binary trait should format its output as a number in binary with 0 and 1.
/// - The alternate flag, #, adds a 0b in front of the output.
/// - For more information on the trait LowerHex,
///   [read more](https://doc.rust-lang.org/std/fmt/trait.Binary.html).
/// - For more information on formatters,
///   see [the module-level documentation](https://doc.rust-lang.org/std/fmt/index.html).
impl<T, const N: usize> Binary for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // fn fmt(&self, f: &mut Formatter) -> fmt::Result
    /// Formats the value using the given formatter.
    /// 
    /// # Arguments
    /// `f` is a buffer, this method must write the formatted string into it,
    /// and is of the type `&mut Formatter`.
    /// 
    /// # Errors
    /// This function should return Err if, and only if, the provided Formatter
    /// returns Err. String formatting is considered an infallible operation;
    /// this function only returns a Result because writing to the underlying
    /// stream might fail and it must provide a way to propagate the fact that
    /// an error has occurred back up the stack.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:b}", a_biguint);
    /// let txt = format!("{:b}", a_biguint);
    /// assert_eq!(txt, "1001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:272b}", a_biguint);
    /// let txt = format!("{:272b}", a_biguint);
    /// assert_eq!(txt, "1001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011                ");
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:0272b}", a_biguint);
    /// let txt = format!("{:0272b}", a_biguint);
    /// assert_eq!(txt, "00000000000000001001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:#b}", a_biguint);
    /// let txt = format!("{:#b}", a_biguint);
    /// assert_eq!(txt, "0b1001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:#272b}", a_biguint);
    /// let txt = format!("{:#272b}", a_biguint);
    /// assert_eq!(txt, "0b1001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011              ");
    /// ```
    /// 
    /// # Example 6
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:#0272b}", a_biguint);
    /// let txt = format!("{:#0272b}", a_biguint);
    /// assert_eq!(txt, "0b000000000000001001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");
    /// ```
    /// 
    /// # Example 7
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:>272b}", a_biguint);
    /// let txt = format!("{:>272b}", a_biguint);
    /// assert_eq!(txt, "                1001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");
    /// ```
    /// 
    /// # Example 8
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:>0272b}", a_biguint);
    /// let txt = format!("{:>0272b}", a_biguint);
    /// assert_eq!(txt, "00000000000000001001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");
    /// ```
    /// 
    /// # Example 9
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:^272b}", a_biguint);
    /// let txt = format!("{:^272b}", a_biguint);
    /// assert_eq!(txt, "        1001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011        ");
    /// ```
    /// 
    /// # Example 10
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:^0272b}", a_biguint);
    /// let txt = format!("{:^0272b}", a_biguint);
    /// assert_eq!(txt, "00000000000000001001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");
    /// ```
    /// 
    /// # Example 11
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:<272b}", a_biguint);
    /// let txt = format!("{:<272b}", a_biguint);
    /// assert_eq!(txt, "1001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011                ");
    /// ```
    /// 
    /// # Example 12
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:<0272b}", a_biguint);
    /// let txt = format!("{:<0272b}", a_biguint);
    /// assert_eq!(txt, "00000000000000001001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");
    /// ```
    /// 
    /// # Example 13
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:!>272b}", a_biguint);
    /// let txt = format!("{:!>272b}", a_biguint);
    /// assert_eq!(txt, "!!!!!!!!!!!!!!!!1001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");
    /// ```
    /// 
    /// # Example 14
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:@>0272b}", a_biguint);
    /// let txt = format!("{:@>0272b}", a_biguint);
    /// assert_eq!(txt, "00000000000000001001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");
    /// ```
    /// 
    /// # Example 15
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:#^272b}", a_biguint);
    /// let txt = format!("{:#^272b}", a_biguint);
    /// assert_eq!(txt, "########1001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011########");
    /// ```
    /// 
    /// # Example 16
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:$^0272b}", a_biguint);
    /// let txt = format!("{:$^0272b}", a_biguint);
    /// assert_eq!(txt, "00000000000000001001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");
    /// ```
    /// 
    /// # Example 17
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:%<272b}", a_biguint);
    /// let txt = format!("{:%<272b}", a_biguint);
    /// assert_eq!(txt, "1001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011%%%%%%%%%%%%%%%%");
    /// ```
    /// 
    /// # Example 18
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:*<0272b}", a_biguint);
    /// let txt = format!("{:*<0272b}", a_biguint);
    /// assert_eq!(txt, "00000000000000001001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");
    /// ```
    /// 
    /// # Example 19
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:>#272b}", a_biguint);
    /// let txt = format!("{:>#272b}", a_biguint);
    /// assert_eq!(txt, "              0b1001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");
    /// ```
    /// 
    /// # Example 20
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:>#0272b}", a_biguint);
    /// let txt = format!("{:>#0272b}", a_biguint);
    /// assert_eq!(txt, "0b000000000000001001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");
    /// ```
    /// 
    /// # Example 21
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:^#272b}", a_biguint);
    /// let txt = format!("{:^#272b}", a_biguint);
    /// assert_eq!(txt, "       0b1001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011       ");
    /// ```
    /// 
    /// # Example 22
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:^#0272b}", a_biguint);
    /// let txt = format!("{:^#0272b}", a_biguint);
    /// assert_eq!(txt, "0b000000000000001001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");
    /// ```
    /// 
    /// # Example 23
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:<#272b}", a_biguint);
    /// let txt = format!("{:<#272b}", a_biguint);
    /// assert_eq!(txt, "0b1001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011              ");
    /// ```
    /// 
    /// # Example 24
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:<#0272b}", a_biguint);
    /// let txt = format!("{:<#0272b}", a_biguint);
    /// assert_eq!(txt, "0b000000000000001001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");
    /// ```
    /// 
    /// # Example 25
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:!>#272b}", a_biguint);
    /// let txt = format!("{:!>#272b}", a_biguint);
    /// assert_eq!(txt, "!!!!!!!!!!!!!!0b1001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");
    /// ```
    /// 
    /// # Example 26
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:@>#0272b}", a_biguint);
    /// let txt = format!("{:@>#0272b}", a_biguint);
    /// assert_eq!(txt, "0b000000000000001001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");
    /// ```
    /// 
    /// # Example 27
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:#^#272b}", a_biguint);
    /// let txt = format!("{:#^#272b}", a_biguint);
    /// assert_eq!(txt, "#######0b1001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011#######");
    /// ```
    /// 
    /// # Example 28
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:$^#0272b}", a_biguint);
    /// let txt = format!("{:$^#0272b}", a_biguint);
    /// assert_eq!(txt, "0b000000000000001001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");
    /// ```
    /// 
    /// # Example 29
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:%<#272b}", a_biguint);
    /// let txt = format!("{:%<#272b}", a_biguint);
    /// assert_eq!(txt, "0b1001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011%%%%%%%%%%%%%%");
    /// ```
    /// 
    /// # Example 30
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:^<#0272b}", a_biguint);
    /// let txt = format!("{:^<#0272b}", a_biguint);
    /// assert_eq!(txt, "0b000000000000001001101000110001001111010011110001010101101100010010001101010011001000101000000000100100110011011011111000010101011011000100010000111000100010001110011110001010110100111110010000100100110110111100010000110000001101101010011101111000100010101101100000001011");
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
    {
        fmt_with_radix!(self, f, 2, "0b");
    }
}



/// o formatting.
/// - The Octal trait should format its output as a number in base-8.
/// - The alternate flag, #, adds a 0o in front of the output.
/// - For more information on the trait LowerHex,
///   [read more](https://doc.rust-lang.org/std/fmt/trait.Octal.html).
/// - For more information on formatters,
///   see [the module-level documentation](https://doc.rust-lang.org/std/fmt/index.html).
impl<T, const N: usize> Octal for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // fn fmt(&self, f: &mut Formatter) -> fmt::Result
    /// Formats the value using the given formatter.
    /// 
    /// # Arguments
    /// `f` is a buffer, this method must write the formatted string into it,
    /// and is of the type `&mut Formatter`.
    /// 
    /// # Errors
    /// This function should return Err if, and only if, the provided Formatter
    /// returns Err. String formatting is considered an infallible operation;
    /// this function only returns a Result because writing to the underlying
    /// stream might fail and it must provide a way to propagate the fact that
    /// an error has occurred back up the stack.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:o}", a_biguint);
    /// let txt = format!("{:o}", a_biguint);
    /// assert_eq!(txt, "11506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:102o}", a_biguint);
    /// let txt = format!("{:102o}", a_biguint);
    /// assert_eq!(txt, "11506117236125542215231050004463337025330420704216361264762044667420601552357042554013                ");
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:0102o}", a_biguint);
    /// let txt = format!("{:0102o}", a_biguint);
    /// assert_eq!(txt, "000000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:#o}", a_biguint);
    /// let txt = format!("{:#o}", a_biguint);
    /// assert_eq!(txt, "0o11506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:#102o}", a_biguint);
    /// let txt = format!("{:#102o}", a_biguint);
    /// assert_eq!(txt, "0o11506117236125542215231050004463337025330420704216361264762044667420601552357042554013              ");
    /// ```
    /// 
    /// # Example 6
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:#0102o}", a_biguint);
    /// let txt = format!("{:#0102o}", a_biguint);
    /// assert_eq!(txt, "0o0000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 7
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:>102o}", a_biguint);
    /// let txt = format!("{:>102o}", a_biguint);
    /// assert_eq!(txt, "                11506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 8
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:>0102o}", a_biguint);
    /// let txt = format!("{:>0102o}", a_biguint);
    /// assert_eq!(txt, "000000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 9
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:^102o}", a_biguint);
    /// let txt = format!("{:^102o}", a_biguint);
    /// assert_eq!(txt, "        11506117236125542215231050004463337025330420704216361264762044667420601552357042554013        ");
    /// ```
    /// 
    /// # Example 10
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:^0102o}", a_biguint);
    /// let txt = format!("{:^0102o}", a_biguint);
    /// assert_eq!(txt, "000000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 11
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:<102o}", a_biguint);
    /// let txt = format!("{:<102o}", a_biguint);
    /// assert_eq!(txt, "11506117236125542215231050004463337025330420704216361264762044667420601552357042554013                ");
    /// ```
    /// 
    /// # Example 12
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:<0102o}", a_biguint);
    /// let txt = format!("{:<0102o}", a_biguint);
    /// assert_eq!(txt, "000000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 13
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:!>102o}", a_biguint);
    /// let txt = format!("{:!>102o}", a_biguint);
    /// assert_eq!(txt, "!!!!!!!!!!!!!!!!11506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 14
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:@>0102o}", a_biguint);
    /// let txt = format!("{:@>0102o}", a_biguint);
    /// assert_eq!(txt, "000000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 15
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:#^102o}", a_biguint);
    /// let txt = format!("{:#^102o}", a_biguint);
    /// assert_eq!(txt, "########11506117236125542215231050004463337025330420704216361264762044667420601552357042554013########");
    /// ```
    /// 
    /// # Example 16
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:$^0102o}", a_biguint);
    /// let txt = format!("{:$^0102o}", a_biguint);
    /// assert_eq!(txt, "000000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 17
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:%<102o}", a_biguint);
    /// let txt = format!("{:%<102o}", a_biguint);
    /// assert_eq!(txt, "11506117236125542215231050004463337025330420704216361264762044667420601552357042554013%%%%%%%%%%%%%%%%");
    /// ```
    /// 
    /// # Example 18
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:*<0102o}", a_biguint);
    /// let txt = format!("{:*<0102o}", a_biguint);
    /// assert_eq!(txt, "000000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 19
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:>#102o}", a_biguint);
    /// let txt = format!("{:>#102o}", a_biguint);
    /// assert_eq!(txt, "              0o11506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 20
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:>#0102o}", a_biguint);
    /// let txt = format!("{:>#0102o}", a_biguint);
    /// assert_eq!(txt, "0o0000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 21
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:^#102o}", a_biguint);
    /// let txt = format!("{:^#102o}", a_biguint);
    /// assert_eq!(txt, "       0o11506117236125542215231050004463337025330420704216361264762044667420601552357042554013       ");
    /// ```
    /// 
    /// # Example 22
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:^#0102o}", a_biguint);
    /// let txt = format!("{:^#0102o}", a_biguint);
    /// assert_eq!(txt, "0o0000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 23
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:<#102o}", a_biguint);
    /// let txt = format!("{:<#102o}", a_biguint);
    /// assert_eq!(txt, "0o11506117236125542215231050004463337025330420704216361264762044667420601552357042554013              ");
    /// ```
    /// 
    /// # Example 24
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:<#0102o}", a_biguint);
    /// let txt = format!("{:<#0102o}", a_biguint);
    /// assert_eq!(txt, "0o0000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 25
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:!>#102o}", a_biguint);
    /// let txt = format!("{:!>#102o}", a_biguint);
    /// assert_eq!(txt, "!!!!!!!!!!!!!!0o11506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 26
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:@>#0102o}", a_biguint);
    /// let txt = format!("{:@>#0102o}", a_biguint);
    /// assert_eq!(txt, "0o0000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 27
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:#^#102o}", a_biguint);
    /// let txt = format!("{:#^#102o}", a_biguint);
    /// assert_eq!(txt, "#######0o11506117236125542215231050004463337025330420704216361264762044667420601552357042554013#######");
    /// ```
    /// 
    /// # Example 28
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:$^#0102o}", a_biguint);
    /// let txt = format!("{:$^#0102o}", a_biguint);
    /// assert_eq!(txt, "0o0000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 29
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:%<#102o}", a_biguint);
    /// let txt = format!("{:%<#102o}", a_biguint);
    /// assert_eq!(txt, "0o11506117236125542215231050004463337025330420704216361264762044667420601552357042554013%%%%%%%%%%%%%%");
    /// ```
    /// 
    /// # Example 30
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:^<#0102o}", a_biguint);
    /// let txt = format!("{:^<#0102o}", a_biguint);
    /// assert_eq!(txt, "0o0000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
    {
        fmt_with_radix!(self, f, 8, "0o");
    }
}



/// E formatting.
/// - The UpperExp trait should format its output in scientific notation
///   with a upper-case E.
/// - For more information on the trait UpperExp,
///   [read more](https://doc.rust-lang.org/std/fmt/trait.UpperExp.html).
/// - For more information on formatters,
///   see [the module-level documentation](https://doc.rust-lang.org/std/fmt/index.html).
impl<T, const N: usize> UpperExp for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // fn fmt(&self, f: &mut Formatter) -> fmt::Result
    /// Formats the value using the given formatter.
    /// 
    /// # Arguments
    /// `f` is a buffer, this method must write the formatted string into it,
    /// and is of the type `&mut Formatter`.
    /// 
    /// # Errors
    /// This function should return Err if, and only if, the provided Formatter
    /// returns Err. String formatting is considered an infallible operation;
    /// this function only returns a Result because writing to the underlying
    /// stream might fail and it must provide a way to propagate the fact that
    /// an error has occurred back up the stack.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:E}", a_biguint);
    /// let txt = format!("{:E}", a_biguint);
    /// assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:100E}", a_biguint);
    /// let txt = format!("{:100E}", a_biguint);
    /// assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76                   ");
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:0100E}", a_biguint);
    /// let txt = format!("{:0100E}", a_biguint);
    /// assert_eq!(txt, "00000000000000000001.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:20.9E}", a_biguint);
    /// let txt = format!("{:20.9E}", a_biguint);
    /// assert_eq!(txt, "1.234567890E76      ");
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:020.9E}", a_biguint);
    /// let txt = format!("{:020.9E}", a_biguint);
    /// assert_eq!(txt, "0000001.234567890E76");
    /// ```
    /// 
    /// # Example 6
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:21.13E}", a_biguint);
    /// let txt = format!("{:21.13E}", a_biguint);
    /// assert_eq!(txt, "1.2345678901235E76   ");
    /// ```
    /// 
    /// # Example 7
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:021.13E}", a_biguint);
    /// let txt = format!("{:021.13E}", a_biguint);
    /// assert_eq!(txt, "0001.2345678901235E76");
    /// ```
    /// 
    /// # Example 8
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:<E}", a_biguint);
    /// let txt = format!("{:<E}", a_biguint);
    /// assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");
    /// ```
    /// 
    /// # Example 9
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:<100E}", a_biguint);
    /// let txt = format!("{:<100E}", a_biguint);
    /// assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76                   ");
    /// ```
    /// 
    /// # Example 10
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:<0100E}", a_biguint);
    /// let txt = format!("{:<0100E}", a_biguint);
    /// assert_eq!(txt, "00000000000000000001.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");
    /// ```
    /// 
    /// # Example 11
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:<20.9E}", a_biguint);
    /// let txt = format!("{:<20.9E}", a_biguint);
    /// assert_eq!(txt, "1.234567890E76      ");
    /// ```
    /// 
    /// # Example 12
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:<020.9E}", a_biguint);
    /// let txt = format!("{:<020.9E}", a_biguint);
    /// assert_eq!(txt, "0000001.234567890E76");
    /// ```
    /// 
    /// # Example 13
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:<21.13E}", a_biguint);
    /// let txt = format!("{:<21.13E}", a_biguint);
    /// assert_eq!(txt, "1.2345678901235E76   ");
    /// ```
    /// 
    /// # Example 14
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:<021.13E}", a_biguint);
    /// let txt = format!("{:<021.13E}", a_biguint);
    /// assert_eq!(txt, "0001.2345678901235E76");
    /// ```
    /// 
    /// # Example 15
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:!<E}", a_biguint);
    /// let txt = format!("{:<E}", a_biguint);
    /// assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");
    /// ```
    /// 
    /// # Example 16
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:@<100E}", a_biguint);
    /// let txt = format!("{:@<100E}", a_biguint);
    /// assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76@@@@@@@@@@@@@@@@@@@");
    /// ```
    /// 
    /// # Example 17
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:#<0100E}", a_biguint);
    /// let txt = format!("{:#<0100E}", a_biguint);
    /// assert_eq!(txt, "00000000000000000001.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");
    /// ```
    /// 
    /// # Example 18
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:$<20.9E}", a_biguint);
    /// let txt = format!("{:$<20.9E}", a_biguint);
    /// assert_eq!(txt, "1.234567890E76$$$$$$");
    /// ```
    /// 
    /// # Example 19
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:%<020.9E}", a_biguint);
    /// let txt = format!("{:%<020.9E}", a_biguint);
    /// assert_eq!(txt, "0000001.234567890E76");
    /// ```
    /// 
    /// # Example 20
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^<21.13E}", a_biguint);
    /// let txt = format!("{:^<21.13E}", a_biguint);
    /// assert_eq!(txt, "1.2345678901235E76^^^");
    /// ```
    /// 
    /// # Example 21
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:&<021.13E}", a_biguint);
    /// let txt = format!("{:&<021.13E}", a_biguint);
    /// assert_eq!(txt, "0001.2345678901235E76");
    /// ```
    /// 
    /// # Example 22
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:>E}", a_biguint);
    /// let txt = format!("{:>E}", a_biguint);
    /// assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");
    /// ```
    /// 
    /// # Example 23
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:>100E}", a_biguint);
    /// let txt = format!("{:>100E}", a_biguint);
    /// assert_eq!(txt, "                   1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");
    /// ```
    /// 
    /// # Example 24
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:>0100E}", a_biguint);
    /// let txt = format!("{:>0100E}", a_biguint);
    /// assert_eq!(txt, "00000000000000000001.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");
    /// ```
    /// 
    /// # Example 25
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:>20.9E}", a_biguint);
    /// let txt = format!("{:>20.9E}", a_biguint);
    /// assert_eq!(txt, "      1.234567890E76");
    /// ```
    /// 
    /// # Example 26
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:>020.9E}", a_biguint);
    /// let txt = format!("{:>020.9E}", a_biguint);
    /// assert_eq!(txt, "0000001.234567890E76");
    /// ```
    /// 
    /// # Example 27
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:>21.13E}", a_biguint);
    /// let txt = format!("{:>21.13E}", a_biguint);
    /// assert_eq!(txt, "   1.2345678901235E76");
    /// ```
    /// 
    /// # Example 28
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:>021.13E}", a_biguint);
    /// let txt = format!("{:>021.13E}", a_biguint);
    /// assert_eq!(txt, "0001.2345678901235E76");
    /// ```
    /// 
    /// # Example 29
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:!>E}", a_biguint);
    /// let txt = format!("{:>E}", a_biguint);
    /// assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");
    /// ```
    /// 
    /// # Example 30
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:@>100E}", a_biguint);
    /// let txt = format!("{:@>100E}", a_biguint);
    /// assert_eq!(txt, "@@@@@@@@@@@@@@@@@@@1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");
    /// ```
    /// 
    /// # Example 31
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:#>0100E}", a_biguint);
    /// let txt = format!("{:#>0100E}", a_biguint);
    /// assert_eq!(txt, "00000000000000000001.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");
    /// ```
    /// 
    /// # Example 32
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:$>20.9E}", a_biguint);
    /// let txt = format!("{:$>20.9E}", a_biguint);
    /// assert_eq!(txt, "$$$$$$1.234567890E76");
    /// ```
    /// 
    /// # Example 33
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:%>020.9E}", a_biguint);
    /// let txt = format!("{:%>020.9E}", a_biguint);
    /// assert_eq!(txt, "0000001.234567890E76");
    /// ```
    /// 
    /// # Example 34
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^>21.13E}", a_biguint);
    /// let txt = format!("{:^>21.13E}", a_biguint);
    /// assert_eq!(txt, "^^^1.2345678901235E76");
    /// ```
    /// 
    /// # Example 35
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:&>021.13E}", a_biguint);
    /// let txt = format!("{:&>021.13E}", a_biguint);
    /// assert_eq!(txt, "0001.2345678901235E76");
    /// ```
    /// 
    /// # Example 36
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^E}", a_biguint);
    /// let txt = format!("{:^E}", a_biguint);
    /// assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");
    /// ```
    /// 
    /// # Example 37
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^100E}", a_biguint);
    /// let txt = format!("{:^100E}", a_biguint);
    /// assert_eq!(txt, "         1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76          ");
    /// ```
    /// 
    /// # Example 38
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^0100E}", a_biguint);
    /// let txt = format!("{:^0100E}", a_biguint);
    /// assert_eq!(txt, "00000000000000000001.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");
    /// ```
    /// 
    /// # Example 39
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^20.9E}", a_biguint);
    /// let txt = format!("{:^20.9E}", a_biguint);
    /// assert_eq!(txt, "   1.234567890E76   ");
    /// ```
    /// 
    /// # Example 40
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^020.9E}", a_biguint);
    /// let txt = format!("{:^020.9E}", a_biguint);
    /// assert_eq!(txt, "0000001.234567890E76");
    /// ```
    /// 
    /// # Example 41
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^21.13E}", a_biguint);
    /// let txt = format!("{:^21.13E}", a_biguint);
    /// assert_eq!(txt, " 1.2345678901235E76  ");
    /// ```
    /// 
    /// # Example 42
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^021.13E}", a_biguint);
    /// let txt = format!("{:^021.13E}", a_biguint);
    /// assert_eq!(txt, "0001.2345678901235E76");
    /// ```
    /// 
    /// # Example 43
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:!^E}", a_biguint);
    /// let txt = format!("{:^E}", a_biguint);
    /// assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");
    /// ```
    /// 
    /// # Example 44
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:@^100E}", a_biguint);
    /// let txt = format!("{:@^100E}", a_biguint);
    /// assert_eq!(txt, "@@@@@@@@@1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76@@@@@@@@@@");
    /// ```
    /// 
    /// # Example 45
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:#^0100E}", a_biguint);
    /// let txt = format!("{:#^0100E}", a_biguint);
    /// assert_eq!(txt, "00000000000000000001.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");
    /// ```
    /// 
    /// # Example 46
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:$^20.9E}", a_biguint);
    /// let txt = format!("{:$^20.9E}", a_biguint);
    /// assert_eq!(txt, "$$$1.234567890E76$$$");
    /// ```
    /// 
    /// # Example 47
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:%^020.9E}", a_biguint);
    /// let txt = format!("{:%^020.9E}", a_biguint);
    /// assert_eq!(txt, "0000001.234567890E76");
    /// ```
    /// 
    /// # Example 48
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^^21.13E}", a_biguint);
    /// let txt = format!("{:^^21.13E}", a_biguint);
    /// assert_eq!(txt, "^1.2345678901235E76^^");
    /// ```
    /// 
    /// # Example 49
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:&^021.13E}", a_biguint);
    /// let txt = format!("{:&^021.13E}", a_biguint);
    /// assert_eq!(txt, "0001.2345678901235E76");
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
    {
        fmt_with_exponent!(self, f, 'E');
    }
}



/// e formatting.
/// - The LowerExp trait should format its output in scientific notation
///   with a lower-case e.
/// - For more information on the trait LowerHex,
///   [read more](https://doc.rust-lang.org/std/fmt/trait.LowerHex.html).
/// - For more information on formatters,
///   see [the module-level documentation](https://doc.rust-lang.org/std/fmt/index.html).
impl<T, const N: usize> LowerExp for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // fn fmt(&self, f: &mut Formatter) -> fmt::Result
    /// Formats the value using the given formatter.
    /// 
    /// # Arguments
    /// `f` is a buffer, this method must write the formatted string into it,
    /// and is of the type `&mut Formatter`.
    /// 
    /// # Errors
    /// This function should return Err if, and only if, the provided Formatter
    /// returns Err. String formatting is considered an infallible operation;
    /// this function only returns a Result because writing to the underlying
    /// stream might fail and it must provide a way to propagate the fact that
    /// an error has occurred back up the stack.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:e}", a_biguint);
    /// let txt = format!("{:e}", a_biguint);
    /// assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567e76");
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:100e}", a_biguint);
    /// let txt = format!("{:100e}", a_biguint);
    /// assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567e76                   ");
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:0100e}", a_biguint);
    /// let txt = format!("{:0100e}", a_biguint);
    /// assert_eq!(txt, "00000000000000000001.2345678901234567890123456789012345678901234567890123456789012345678901234567e76");
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:20.9e}", a_biguint);
    /// let txt = format!("{:20.9e}", a_biguint);
    /// assert_eq!(txt, "1.234567890e76      ");
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:020.9e}", a_biguint);
    /// let txt = format!("{:020.9e}", a_biguint);
    /// assert_eq!(txt, "0000001.234567890e76");
    /// ```
    /// 
    /// # Example 6
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:21.13e}", a_biguint);
    /// let txt = format!("{:21.13e}", a_biguint);
    /// assert_eq!(txt, "1.2345678901235e76   ");
    /// ```
    /// 
    /// # Example 7
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:021.13e}", a_biguint);
    /// let txt = format!("{:021.13e}", a_biguint);
    /// assert_eq!(txt, "0001.2345678901235e76");
    /// ```
    /// 
    /// # Example 8
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:<e}", a_biguint);
    /// let txt = format!("{:<e}", a_biguint);
    /// assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567e76");
    /// ```
    /// 
    /// # Example 9
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:<100e}", a_biguint);
    /// let txt = format!("{:<100e}", a_biguint);
    /// assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567e76                   ");
    /// ```
    /// 
    /// # Example 10
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:<0100e}", a_biguint);
    /// let txt = format!("{:<0100e}", a_biguint);
    /// assert_eq!(txt, "00000000000000000001.2345678901234567890123456789012345678901234567890123456789012345678901234567e76");
    /// ```
    /// 
    /// # Example 11
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:<20.9e}", a_biguint);
    /// let txt = format!("{:<20.9e}", a_biguint);
    /// assert_eq!(txt, "1.234567890e76      ");
    /// ```
    /// 
    /// # Example 12
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:<020.9e}", a_biguint);
    /// let txt = format!("{:<020.9e}", a_biguint);
    /// assert_eq!(txt, "0000001.234567890e76");
    /// ```
    /// 
    /// # Example 13
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:<21.13e}", a_biguint);
    /// let txt = format!("{:<21.13e}", a_biguint);
    /// assert_eq!(txt, "1.2345678901235e76   ");
    /// ```
    /// 
    /// # Example 14
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:<021.13e}", a_biguint);
    /// let txt = format!("{:<021.13e}", a_biguint);
    /// assert_eq!(txt, "0001.2345678901235e76");
    /// ```
    /// 
    /// # Example 15
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:!<e}", a_biguint);
    /// let txt = format!("{:<e}", a_biguint);
    /// assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567e76");
    /// ```
    /// 
    /// # Example 16
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:@<100e}", a_biguint);
    /// let txt = format!("{:@<100e}", a_biguint);
    /// assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567e76@@@@@@@@@@@@@@@@@@@");
    /// ```
    /// 
    /// # Example 17
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:#<0100e}", a_biguint);
    /// let txt = format!("{:#<0100e}", a_biguint);
    /// assert_eq!(txt, "00000000000000000001.2345678901234567890123456789012345678901234567890123456789012345678901234567e76");
    /// ```
    /// 
    /// # Example 18
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:$<20.9e}", a_biguint);
    /// let txt = format!("{:$<20.9e}", a_biguint);
    /// assert_eq!(txt, "1.234567890e76$$$$$$");
    /// ```
    /// 
    /// # Example 19
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:%<020.9e}", a_biguint);
    /// let txt = format!("{:%<020.9e}", a_biguint);
    /// assert_eq!(txt, "0000001.234567890e76");
    /// ```
    /// 
    /// # Example 20
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^<21.13e}", a_biguint);
    /// let txt = format!("{:^<21.13e}", a_biguint);
    /// assert_eq!(txt, "1.2345678901235e76^^^");
    /// ```
    /// 
    /// # Example 21
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:&<021.13e}", a_biguint);
    /// let txt = format!("{:&<021.13e}", a_biguint);
    /// assert_eq!(txt, "0001.2345678901235e76");
    /// ```
    /// 
    /// # Example 22
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:>e}", a_biguint);
    /// let txt = format!("{:>e}", a_biguint);
    /// assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567e76");
    /// ```
    /// 
    /// # Example 23
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:>100e}", a_biguint);
    /// let txt = format!("{:>100e}", a_biguint);
    /// assert_eq!(txt, "                   1.2345678901234567890123456789012345678901234567890123456789012345678901234567e76");
    /// ```
    /// 
    /// # Example 24
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:>0100e}", a_biguint);
    /// let txt = format!("{:>0100e}", a_biguint);
    /// assert_eq!(txt, "00000000000000000001.2345678901234567890123456789012345678901234567890123456789012345678901234567e76");
    /// ```
    /// 
    /// # Example 25
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:>20.9e}", a_biguint);
    /// let txt = format!("{:>20.9e}", a_biguint);
    /// assert_eq!(txt, "      1.234567890e76");
    /// ```
    /// 
    /// # Example 26
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:>020.9e}", a_biguint);
    /// let txt = format!("{:>020.9e}", a_biguint);
    /// assert_eq!(txt, "0000001.234567890e76");
    /// ```
    /// 
    /// # Example 27
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:>21.13e}", a_biguint);
    /// let txt = format!("{:>21.13e}", a_biguint);
    /// assert_eq!(txt, "   1.2345678901235e76");
    /// ```
    /// 
    /// # Example 28
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:>021.13e}", a_biguint);
    /// let txt = format!("{:>021.13e}", a_biguint);
    /// assert_eq!(txt, "0001.2345678901235e76");
    /// ```
    /// 
    /// # Example 29
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:!>e}", a_biguint);
    /// let txt = format!("{:>e}", a_biguint);
    /// assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567e76");
    /// ```
    /// 
    /// # Example 30
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:@>100e}", a_biguint);
    /// let txt = format!("{:@>100e}", a_biguint);
    /// assert_eq!(txt, "@@@@@@@@@@@@@@@@@@@1.2345678901234567890123456789012345678901234567890123456789012345678901234567e76");
    /// ```
    /// 
    /// # Example 31
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:#>0100e}", a_biguint);
    /// let txt = format!("{:#>0100e}", a_biguint);
    /// assert_eq!(txt, "00000000000000000001.2345678901234567890123456789012345678901234567890123456789012345678901234567e76");
    /// ```
    /// 
    /// # Example 32
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:$>20.9e}", a_biguint);
    /// let txt = format!("{:$>20.9e}", a_biguint);
    /// assert_eq!(txt, "$$$$$$1.234567890e76");
    /// ```
    /// 
    /// # Example 33
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:%>020.9e}", a_biguint);
    /// let txt = format!("{:%>020.9e}", a_biguint);
    /// assert_eq!(txt, "0000001.234567890e76");
    /// ```
    /// 
    /// # Example 34
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^>21.13e}", a_biguint);
    /// let txt = format!("{:^>21.13e}", a_biguint);
    /// assert_eq!(txt, "^^^1.2345678901235e76");
    /// ```
    /// 
    /// # Example 35
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:&>021.13e}", a_biguint);
    /// let txt = format!("{:&>021.13e}", a_biguint);
    /// assert_eq!(txt, "0001.2345678901235e76");
    /// ```
    /// 
    /// # Example 36
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^e}", a_biguint);
    /// let txt = format!("{:^e}", a_biguint);
    /// assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567e76");
    /// ```
    /// 
    /// # Example 37
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^100e}", a_biguint);
    /// let txt = format!("{:^100e}", a_biguint);
    /// assert_eq!(txt, "         1.2345678901234567890123456789012345678901234567890123456789012345678901234567e76          ");
    /// ```
    /// 
    /// # Example 38
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^0100e}", a_biguint);
    /// let txt = format!("{:^0100e}", a_biguint);
    /// assert_eq!(txt, "00000000000000000001.2345678901234567890123456789012345678901234567890123456789012345678901234567e76");
    /// ```
    /// 
    /// # Example 39
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^20.9e}", a_biguint);
    /// let txt = format!("{:^20.9e}", a_biguint);
    /// assert_eq!(txt, "   1.234567890e76   ");
    /// ```
    /// 
    /// # Example 40
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^020.9e}", a_biguint);
    /// let txt = format!("{:^020.9e}", a_biguint);
    /// assert_eq!(txt, "0000001.234567890e76");
    /// ```
    /// 
    /// # Example 41
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^21.13e}", a_biguint);
    /// let txt = format!("{:^21.13e}", a_biguint);
    /// assert_eq!(txt, " 1.2345678901235e76  ");
    /// ```
    /// 
    /// # Example 42
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^021.13e}", a_biguint);
    /// let txt = format!("{:^021.13e}", a_biguint);
    /// assert_eq!(txt, "0001.2345678901235e76");
    /// ```
    /// 
    /// # Example 43
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:!^e}", a_biguint);
    /// let txt = format!("{:^e}", a_biguint);
    /// assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567e76");
    /// ```
    /// 
    /// # Example 44
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:@^100e}", a_biguint);
    /// let txt = format!("{:@^100e}", a_biguint);
    /// assert_eq!(txt, "@@@@@@@@@1.2345678901234567890123456789012345678901234567890123456789012345678901234567e76@@@@@@@@@@");
    /// ```
    /// 
    /// # Example 45
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:#^0100e}", a_biguint);
    /// let txt = format!("{:#^0100e}", a_biguint);
    /// assert_eq!(txt, "00000000000000000001.2345678901234567890123456789012345678901234567890123456789012345678901234567e76");
    /// ```
    /// 
    /// # Example 46
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:$^20.9e}", a_biguint);
    /// let txt = format!("{:$^20.9e}", a_biguint);
    /// assert_eq!(txt, "$$$1.234567890e76$$$");
    /// ```
    /// 
    /// # Example 47
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:%^020.9e}", a_biguint);
    /// let txt = format!("{:%^020.9e}", a_biguint);
    /// assert_eq!(txt, "0000001.234567890e76");
    /// ```
    /// 
    /// # Example 48
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^^21.13e}", a_biguint);
    /// let txt = format!("{:^^21.13e}", a_biguint);
    /// assert_eq!(txt, "^1.2345678901235e76^^");
    /// ```
    /// 
    /// # Example 49
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:&^021.13e}", a_biguint);
    /// let txt = format!("{:&^021.13e}", a_biguint);
    /// assert_eq!(txt, "0001.2345678901235e76");
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
    {
        fmt_with_exponent!(self, f, 'e');
    }
}



/// p formatting.
/// - The Pointer trait should format its output as a memory location.
/// - This is commonly presented as hexadecimal.
/// - Printing of pointers is not a reliable way to discover how Rust programs
///   are implemented.
/// - The act of reading an address changes the program itself, and may change
///   how the data is represented in memory, and may affect which optimizations
///   are applied to the code.
/// - The printed pointer values are not guaranteed to be stable nor unique
///   identifiers of objects.
/// - Rust allows moving values to different memory locations, and may reuse
///   the same memory locations for different purposes.
/// - There is no guarantee that the printed value can be converted back to a pointer.
/// - For more information on the trait LowerHex,
///   [read more](https://doc.rust-lang.org/std/fmt/trait.Pointer.html).
/// - For more information on formatters,
///   see [the module-level documentation](https://doc.rust-lang.org/std/fmt/index.html).
impl<T, const N: usize> Pointer for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // fn fmt(&self, f: &mut Formatter) -> fmt::Result
    /// Formats the value using the given formatter.
    /// 
    /// # Arguments
    /// `f` is a buffer, this method must write the formatted string into it,
    /// and is of the type `&mut Formatter`.
    /// 
    /// # Errors
    /// This function should return Err if, and only if, the provided Formatter
    /// returns Err. String formatting is considered an infallible operation;
    /// this function only returns a Result because writing to the underlying
    /// stream might fail and it must provide a way to propagate the fact that
    /// an error has occurred back up the stack.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:p}", a_biguint);
    /// let txt = format!("{:p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "0x7ffcd958aab0"); // can be different everytime
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:20p}", a_biguint);
    /// let txt = format!("{:20p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "0x7ffcd958b0b0      "); // can be different everytime
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:020p}", a_biguint);
    /// let txt = format!("{:020p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "0x0000007ffcd958aae0"); // can be different everytime
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:<p}", a_biguint);
    /// let txt = format!("{:<p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "0x7ffcd958b0e0"); // can be different everytime
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:<20p}", a_biguint);
    /// let txt = format!("{:<20p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "0x7ffcd958ab10      "); // can be different everytime
    /// ```
    /// 
    /// # Example 6
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:<020p}", a_biguint);
    /// let txt = format!("{:<020p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "0x0000007ffcd958b1a0"); // can be different everytime
    /// ```
    /// 
    /// # Example 7
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:!<p}", a_biguint);
    /// let txt = format!("{:!<p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "0x7ffcd958ab40"); // can be different everytime
    /// ```
    /// 
    /// # Example 8
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:@<20p}", a_biguint);
    /// let txt = format!("{:@<20p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "0x7ffcd958b1d0@@@@@@"); // can be different everytime
    /// ```
    /// 
    /// # Example 9
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:#<020p}", a_biguint);
    /// let txt = format!("{:#<020p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "0x0000007ffcd958ab70"); // can be different everytime
    /// ```
    /// 
    /// # Example 10
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:>p}", a_biguint);
    /// let txt = format!("{:>p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "0x7ffcd958b200"); // can be different everytime
    /// ```
    /// 
    /// # Example 11
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:>20p}", a_biguint);
    /// let txt = format!("{:>20p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "      0x7ffcd958aba0"); // can be different everytime
    /// ```
    /// 
    /// # Example 12
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:>020p}", a_biguint);
    /// let txt = format!("{:>020p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "0x0000007ffcd958b230"); // can be different everytime
    /// ```
    /// 
    /// # Example 13
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:$>p}", a_biguint);
    /// let txt = format!("{:$>p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "0x7ffcd958abd0"); // can be different everytime
    /// ```
    /// 
    /// # Example 14
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:%>20p}", a_biguint);
    /// let txt = format!("{:%>20p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "%%%%%%0x7ffcd958b110"); // can be different everytime
    /// ```
    /// 
    /// # Example 15
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^>020p}", a_biguint);
    /// let txt = format!("{:^>020p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "0x0000007ffcd958a750"); // can be different everytime
    /// ```
    /// 
    /// # Example 16
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^p}", a_biguint);
    /// let txt = format!("{:^p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "0x7ffcd958a850"); // can be different everytime
    /// ```
    /// 
    /// # Example 17
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^20p}", a_biguint);
    /// let txt = format!("{:^20p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "   0x7ffcd958b140   "); // can be different everytime
    /// ```
    /// 
    /// # Example 18
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^020p}", a_biguint);
    /// let txt = format!("{:^020p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "0x0000007ffcd958b170"); // can be different everytime
    /// ```
    /// 
    /// # Example 19
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:&^p}", a_biguint);
    /// let txt = format!("{:&^p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "0x7ffcd958af40"); // can be different everytime
    /// ```
    /// 
    /// # Example 20
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:*^20p}", a_biguint);
    /// let txt = format!("{:*^20p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "***0x7ffcd958af70***"); // can be different everytime
    /// ```
    /// 
    /// # Example 21
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:_^020p}", a_biguint);
    /// let txt = format!("{:_^020p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "0x0000007ffcd958afa0"); // can be different everytime
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
    {
        let ptr = Self::from_uint(self as *const Self as usize);
        fmt_with_radix!(ptr, f);
    }
}


/*
impl<T, const N: usize> Add<&Self> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: &Self) -> Self
    {
        self.wrapping_add(rhs)
    }
}

impl<T, const N: usize> AddAssign<&Self> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /// Adds and assign the result to it.
    /// 
    #[inline]
    fn add_assign(&mut self, rhs: &Self)
    {
        self.wrapping_add_assign(rhs);
    }
}

impl<T, const N: usize> Sub<&Self> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: &Self) -> Self
    {
        self.wrapping_sub(rhs)
    }
}

impl<T, const N: usize> SubAssign<&Self> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /// Subtracts and assign the result to it.
    #[inline]
    fn sub_assign(&mut self, rhs: &Self)
    {
        self.wrapping_sub_assign(rhs);
    }
}

impl<T, const N: usize> Mul<&Self> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: &Self) -> Self
    {
        self.wrapping_mul(rhs)
    }
}

impl<T, const N: usize> MulAssign<&Self> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /// Multiplies and assign the result to it.
    #[inline] 
    fn mul_assign(&mut self, rhs: &Self)
    {
        self.wrapping_mul_assign(rhs);
    }
}

impl<T, const N: usize> Div<&Self> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: &Self) -> Self
    {
        self.wrapping_div(rhs)
    }
}

impl<T, const N: usize> DivAssign<&Self> for BigUInt<T, N>
where T: SmallUInt + Copy + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    #[inline]
    fn div_assign(&mut self, rhs: &Self)
    {
        self.wrapping_div_assign(rhs);
    }
}

impl<T, const N: usize> Rem<&Self> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    #[inline]
    fn rem(self, rhs: &Self) -> Self
    {
        self.wrapping_rem(rhs)
    }
}

impl<T, const N: usize> RemAssign<&Self> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    #[inline]
    fn rem_assign(&mut self, rhs: &Self)
    {
        self.wrapping_rem_assign(rhs);
    }
}

impl<T, const N: usize> BitAnd<&Self> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;
    //type Output = <Self as BitAnd<Self>>::Output;
    //type <BigUInt<T, N> as std::ops::BitAnd<&BigUInt<T, N>>>::Output = BigUInt<T, N>;

    /// Performs the & operation.
    /// [Read more](https://doc.rust-lang.org/core/ops/bit/trait.BitAnd.html#tymethod.bitand)
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::from_str_radix("11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000", 2).unwrap();
    /// let c = a & b;
    /// 
    /// println!("a = {}", a.to_string_with_radix(2));
    /// println!("b = {}", b.to_string_with_radix(2));
    /// println!("a & b = {}", c.to_string_with_radix(2));
    /// 
    /// assert_eq!(c, a & b);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::zero();
    /// let c = a & b;
    /// 
    /// println!("a = {}", a.to_string_with_radix(2));
    /// println!("b = {}", b.to_string_with_radix(2));
    /// println!("a & b = {}", c.to_string_with_radix(2));
    /// 
    /// assert_eq!(c, a & b);
    /// ```
    fn bitand(self, rhs: &Self) -> Self
    {
        self.and(rhs)
    }
}

impl<T, const N: usize> BitAndAssign<&Self> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /// Performs the &= operation.
    /// [Read more](https://doc.rust-lang.org/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::from_str_radix("11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000", 2).unwrap();
    /// a &= b;
    /// 
    /// println!("a = {}", a.to_string_with_radix(2));
    /// 
    /// assert_eq!(a, u256::from_str_radix("1111000000000000110000000000001110001000000100011010101000000000111100000000000011000000000000111000100000010001101010100000000011110000000000001100000000000011100010000001000110101010000000001111000000000000110000000000001110001000000100011010101000000000", 2).unwrap());
    /// ```
    ///
    /// # Example 2
    /// ```
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::zero();
    /// a &= b;
    /// 
    /// println!("a = {}", a.to_string_with_radix(2));
    /// 
    /// assert_eq!(a, u256::zero());
    /// ```
    #[inline]
    fn bitand_assign(&mut self, rhs: &Self)
    {
        self.and_assign(rhs);
    }
}

impl<T, const N: usize> BitOr<&Self> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;
    //type Output = <Self as BitOr<Self>>::Output;

    /// Performs the bitwise OR (|) operation.
    /// 
    /// # Output
    /// It returns the result after applying the bitwise OR (|) operation.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::number::HugeInteger;
    /// use cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::from_str_radix("11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000", 2).unwrap();
    /// let c = a | b;
    /// 
    /// println!("a = {}", a.to_string_with_radix(2));
    /// println!("b = {}", b.to_string_with_radix(2));
    /// println!("a | b = {}", c.to_string_with_radix(2));
    /// 
    /// assert_eq!(c, a | b);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::max();
    /// let c = a | b;
    /// 
    /// println!("a = {}", a.to_string_with_radix(2));
    /// println!("b = {}", b.to_string_with_radix(2));
    /// println!("a | b = {}", c.to_string_with_radix(2));
    /// 
    /// assert_eq!(c, u256::max());
    /// ```
    #[inline]
    fn bitor(self, rhs: &Self) -> Self
    {
        self.or(rhs)
    }
}

impl<T, const N: usize> BitOrAssign<&Self> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /// Performs the bitwise OR (|) operation.
    /// [Read more](https://doc.rust-lang.org/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)
    /// 
    /// # Output
    /// It returns the result after applying the bitwise OR (|) operation.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::from_str_radix("11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000", 2).unwrap();
    /// a |= b;
    /// 
    /// println!("a = {}", a.to_string_with_radix(2));
    /// 
    /// assert_eq!(a, u256::from_str_radix("1111111100001111111111000011111111101110011101111111111101010101111111110000111111111100001111111110111001110111111111110101010111111111000011111111110000111111111011100111011111111111010101011111111100001111111111000011111111101110011101111111111101010101", 2).unwrap());
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::max();
    /// a |= b;
    /// 
    /// println!("a = {}", a.to_string_with_radix(2));
    /// 
    /// assert_eq!(a, u256::max());
    /// ```
    #[inline]
    fn bitor_assign(&mut self, rhs: &Self)
    {
        self.or_assign(rhs);
    }
}

impl<T, const N: usize> BitXor<&Self> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;
    // type Output = <Self as BitXor<Self>>::Output;

    /// Performs the ^ operation.
    /// [Read more](https://doc.rust-lang.org/core/ops/bit/trait.BitXor.html#tymethod.bitxor)
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::from_str_radix("11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000", 2).unwrap();
    /// let c = a ^ b;
    /// println!("a = {}", a.to_string_with_radix(2));
    /// println!("b = {}", b.to_string_with_radix(2));
    /// println!("a ^ b = {}", c.to_string_with_radix(2));
    /// assert_eq!(c, a ^ b);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::max();
    /// let c = a ^ b;
    /// println!("a = {}", a.to_string_with_radix(2));
    /// println!("b = {}", b.to_string_with_radix(2));
    /// println!("a ^ b = {}", c.to_string_with_radix(2));
    /// assert_eq!(c, !a);
    /// ```    
    #[inline]
    fn bitxor(self, rhs: &Self) -> Self
    {
        self.xor(rhs)
    }
}

impl<T, const N: usize> BitXorAssign<&Self> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /// Performs the ^= operation.
    /// [Read more](https://doc.rust-lang.org/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let mut a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::from_str_radix("11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000", 2).unwrap();
    /// a ^= b;
    /// println!("a = {}", a.to_string_with_radix(2));
    /// assert_eq!(a, u256::from_str_radix("111100001111001111000011110001100110011001100101010101010101000011110000111100111100001111000110011001100110010101010101010100001111000011110011110000111100011001100110011001010101010101010000111100001111001111000011110001100110011001100101010101010101", 2).unwrap());
    /// ```
    /// You have to import (use) cryptocol::number::HugeInteger in order to use
    /// its method to_string_with_radix(). If you find headaching to remember
    /// what you should import, you can just import everything
    /// (cryptocol::number::*) as next example. It is not harmful.
    /// ```
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let mut a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::max();
    /// a ^= b;
    /// println!("a = {}", a.to_string_with_radix(2));
    /// assert_eq!(a, u256::from_str_radix("11111111000011111111000000110011110011000101010110101010000000001111111100001111111100000011001111001100010101011010101000000000111111110000111111110000001100111100110001010101101010100000000011111111000011111111000000110011110011000101010110101010", 2).unwrap());
    /// ```
    #[inline]
    fn bitxor_assign(&mut self, rhs: &Self)
    {
        self.xor_assign(rhs);
    }
}
*/