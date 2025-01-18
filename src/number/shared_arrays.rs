// Copyright 2023, 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module that contains unions of primitive signed/unsigned integral
//! data types used in a lot of modules of the crate Cryptocol.
//! __These unions are for segmentation.__

// #![warn(missing_docs)]
// #![warn(rustdoc::missing_doc_code_examples)]
#![allow(missing_docs)]
// #![allow(rustdoc::missing_doc_code_examples)]

use std::fmt::Display;
use std::mem::{ size_of, size_of_val };
use std::cmp::{ PartialEq, PartialOrd };
use std::ops::*;

use crate::number::SmallUInt;


/// union for transforming from an array of one type into another array of
/// anther type
/// 
/// The type S is the source type while the type D is the destination type.
/// The type S and D should have `SmallUInt`. In this crate, `u8`, `u16`,
/// `u32`, `u64`, `u128`, `i8`, `i16`, `i32`, `i64`, `i128`, `ShortUnion`,
/// `IntUnion`, `LongUnion`, `LongerUnion`, and `SizeUnion` have the trait
/// `SmallUInt`.
/// 
/// N is the number of elements of D type while M is the number of elements
/// of S.
/// 
/// Unlike `ShortUnion`, `IntUnion`, `LongUnion`, `LongerUnion`, and
/// `SizeUnion`, the fields of `SharedArrays` are all public. So,
/// `SharedArrays` is very flexible and gives you the full control of itself.
/// You can convert data array from one type into another type freely by
/// means of `SharedArrays` if the element of the data array has the trait
/// `SmallUInt`. However, you have to use unsafe {...}.
/// 
/// # Quick Start
/// You can freely convert from an array of primitive type or another union
/// type into an array of another primitive type or anther union type.
/// 
/// ## Example 1 for primitive data types for source and destination
/// ```
/// use cryptocol::number::{ SmallUInt, SharedArrays, IntUnion, LongUnion };
/// let a = SharedArrays::<u16, 4, u64, 2> { src: [123456789123456789_u64, 987654321987654321_u64] };
/// print!("source = [ ");
/// for i in 0..2
///     { print!("{} ", a.get_src_elem_(i)); }
/// println!("]");
/// print!("Destination = [ ");
/// for i in 0..4
///     { print!("{} ", a.get_des_elem_(i)); }
/// println!("]");
/// assert_eq!(a.get_src_elem_(0), 123456789123456789_u64);
/// assert_eq!(a.get_src_elem_(1), 987654321987654321_u64);
/// assert_eq!(a.get_des_elem_(0), 24341_u16);
/// assert_eq!(a.get_des_elem_(1), 44240_u16);
/// assert_eq!(a.get_des_elem_(2), 39755_u16);
/// assert_eq!(a.get_des_elem_(3), 438_u16);
/// ```
/// 
/// ## Example 2 for primitive data type for source and union data type for destination
/// ```
/// use cryptocol::number::{ SmallUInt, SharedArrays, IntUnion, LongUnion };
/// let mut b = SharedArrays::<IntUnion, 4, u64, 2>::new();
/// b.src = [123456789123456789_u64, 987654321987654321_u64];
/// print!("source = [ ");
/// for i in 0..2
///     { print!("{} ", b.get_src_elem_(i)); }
/// println!("]");
/// print!("Destination = [ ");
/// for i in 0..4
///     { print!("{} ", b.get_des_elem_(i)); }
/// println!("]");
/// assert_eq!(b.get_src_elem_(0), 123456789123456789_u64);
/// assert_eq!(b.get_src_elem_(1), 987654321987654321_u64);
/// assert_eq!(b.get_des_elem_(0).get(), 2899336981_u32);
/// assert_eq!(b.get_des_elem_(1).get(), 28744523_u32);
/// assert_eq!(b.get_des_elem_(2).get(), 2129924785_u32);
/// assert_eq!(b.get_des_elem_(3).get(), 229956191_u32);
/// ```
/// 
/// ## Example 3 for primitive data type for destination and union data type for source
/// ```
/// use cryptocol::number::{ SmallUInt, SharedArrays, IntUnion, LongUnion };
/// let c = SharedArrays::<u16, 4, LongUnion, 2>::from_src(&[123456789123456789_u64.into_longunion(), 987654321987654321_u64.into_longunion()]);
/// print!("source = [ ");
/// for i in 0..2
///     { print!("{} ", c.get_src_elem_(i)); }
/// println!("]");
/// print!("Destination = [ ");
/// for i in 0..4
///     { print!("{} ", c.get_des_elem_(i)); }
/// println!("]");
/// assert_eq!(c.get_src_elem_(0).get(), 123456789123456789_u64);
/// assert_eq!(c.get_src_elem_(1).get(), 987654321987654321_u64);
/// assert_eq!(c.get_des_elem_(0), 24341_u16);
/// assert_eq!(c.get_des_elem_(1), 44240_u16);
/// assert_eq!(c.get_des_elem_(2), 39755_u16);
/// assert_eq!(c.get_des_elem_(3), 438_u16);
/// ```
/// 
/// ## Example 4 for union data type for source and destination
/// ```
/// use cryptocol::number::{ SmallUInt, SharedArrays, IntUnion, LongUnion };
/// let d = SharedArrays::<IntUnion, 4, LongUnion, 2>::from_src(&[123456789123456789_u64.into_longunion(), 987654321987654321_u64.into_longunion()]);
/// print!("source = [ ");
/// for i in 0..2
///     { print!("{} ", d.get_src_elem_(i)); }
/// println!("]");
/// print!("Destination = [ ");
/// for i in 0..4
///     { print!("{} ", d.get_des_elem_(i)); }
/// println!("]");
/// assert_eq!(d.get_src_elem_(0).get(), 123456789123456789_u64);
/// assert_eq!(d.get_src_elem_(1).get(), 987654321987654321_u64);
/// assert_eq!(d.get_des_elem_(0).get(), 2899336981_u32);
/// assert_eq!(d.get_des_elem_(1).get(), 28744523_u32);
/// assert_eq!(d.get_des_elem_(2).get(), 2129924785_u32);
/// assert_eq!(d.get_des_elem_(3).get(), 229956191_u32);
/// ```
///  
/// # Big-endian issue
/// It is just experimental for Big Endian CPUs. So, you are not encouraged
/// to use it for serious purpose. Only use this crate for Big-endian CPUs
/// with your own full responsibility.
pub union SharedArrays<D, const N: usize, S, const M: usize>
where D: SmallUInt + Add<Output=D> + AddAssign + Sub<Output=D> + SubAssign
        + Mul<Output=D> + MulAssign + Div<Output=D> + DivAssign
        + Rem<Output=D> + RemAssign
        + Shl<Output=D> + ShlAssign + Shr<Output=D> + ShrAssign
        + BitAnd<Output=D> + BitAndAssign + BitOr<Output=D> + BitOrAssign
        + BitXor<Output=D> + BitXorAssign + Not<Output=D>
        + PartialEq + PartialOrd
        + Display + ToString,
      S: SmallUInt + Add<Output=S> + AddAssign + Sub<Output=S> + SubAssign
        + Mul<Output=S> + MulAssign + Div<Output=S> + DivAssign
        + Shl<Output=S> + ShlAssign + Shr<Output=S> + ShrAssign
        + Rem<Output=S> + RemAssign
        + BitAnd<Output=S> + BitAndAssign + BitOr<Output=S> + BitOrAssign
        + BitXor<Output=S> + BitXorAssign + Not<Output=S>
        + PartialEq + PartialOrd
        + Display + ToString
{
    pub des: [D; N],
    pub src: [S; M],
}

impl<D, const N: usize, S, const M: usize> SharedArrays<D, N, S, M>
where D: SmallUInt + Add<Output=D> + AddAssign + Sub<Output=D> + SubAssign
        + Mul<Output=D> + MulAssign + Div<Output=D> + DivAssign
        + Rem<Output=D> + RemAssign
        + Shl<Output=D> + ShlAssign + Shr<Output=D> + ShrAssign
        + BitAnd<Output=D> + BitAndAssign + BitOr<Output=D> + BitOrAssign
        + BitXor<Output=D> + BitXorAssign + Not<Output=D>
        + PartialEq + PartialOrd
        + Display + ToString,
      S: SmallUInt + Add<Output=S> + AddAssign + Sub<Output=S> + SubAssign
        + Mul<Output=S> + MulAssign + Div<Output=S> + DivAssign
        + Rem<Output=S> + RemAssign
        + Shl<Output=S> + ShlAssign + Shr<Output=S> + ShrAssign
        + BitAnd<Output=S> + BitAndAssign + BitOr<Output=S> + BitOrAssign
        + BitXor<Output=S> + BitXorAssign + Not<Output=S>
        + PartialEq + PartialOrd
        + Display + ToString
{
    /// Constructs a new `SharedArrays<D, N, S, M>`.
    /// 
    /// # Output
    /// A new object of `SharedArrays<D, N, S, M>`.
    /// 
    /// # Initialization
    /// All the fields of the constructed object will be
    /// initialized with `0`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::number::SharedArrays;    
    /// let a = SharedArrays::<u32, 4, u64, 2>::new();
    /// print!("source = [ ");
    /// for i in 0..2
    ///     { print!("{} ", a.get_src_elem_(i)); }
    /// println!("]");
    /// print!("Destination = [ ");
    /// for i in 0..4
    ///     { print!("{} ", a.get_des_elem_(i)); }
    /// println!("]");
    /// assert_eq!(a.get_src_elem_(0), 0_u64);
    /// assert_eq!(a.get_src_elem_(1), 0_u64);
    /// assert_eq!(a.get_des_elem_(0), 0_u32);
    /// assert_eq!(a.get_des_elem_(1), 0_u32);
    /// assert_eq!(a.get_des_elem_(2), 0_u32);
    /// assert_eq!(a.get_des_elem_(3), 0_u32);
    /// ```
    pub fn new() -> Self
    {
        if Self::size_of_des() >= Self::size_of_src()
            { Self { des: [D::zero(); N] } }
        else
            { Self { src: [S::zero(); M] } }
    }

    /// Constructs a new `SharedArrays<D, N, S, M>` from an array
    /// of `S` type values.
    /// 
    /// # Output
    /// A new object of `SharedArrays<D, N, S, M>`.
    /// 
    /// # Argument
    /// The field `src` will be initialized with the aregument `src`
    /// which is the array of `S` type values.
    /// 
    /// Example
    /// ```
    /// use cryptocol::number::SharedArrays;
    /// let a = SharedArrays::<u32, 4, u64, 2>::from_src(&[123456789123456789_u64, 987654321987654321_u64]);
    /// print!("source = [ ");
    /// for i in 0..2
    ///     { print!("{} ", a.get_src_elem_(i)); }
    /// println!("]");
    /// print!("Destination = [ ");
    /// for i in 0..4
    ///     { print!("{} ", a.get_des_elem_(i)); }
    /// println!("]");
    /// assert_eq!(a.get_src_elem_(0), 123456789123456789_u64);
    /// assert_eq!(a.get_src_elem_(1), 987654321987654321_u64);
    /// assert_eq!(a.get_des_elem_(0), 2899336981_u32);
    /// assert_eq!(a.get_des_elem_(1), 28744523_u32);
    /// assert_eq!(a.get_des_elem_(2), 2129924785_u32);
    /// assert_eq!(a.get_des_elem_(3), 229956191_u32);
    /// ```
    pub fn from_src(src: &[S; M]) -> Self
    {
        let mut me = SharedArrays::<D, N, S, M>::new();
        unsafe { me.src.copy_from_slice(src); }
        me
    }

    /// Returns an array of the values of source type.
    /// 
    /// # Output
    /// An array of the values of source type.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::number::SharedArrays;
    /// let a = SharedArrays::<u32, 4, u64, 2>::from_src(&[123456789123456789_u64, 987654321987654321_u64]);
    /// let b = a.get_src();
    /// print!("source = [ ");
    /// for i in 0..2
    ///     { print!("{} ", b[i]); }
    /// println!("]");
    /// assert_eq!(b[0], 123456789123456789_u64);
    /// assert_eq!(b[1], 987654321987654321_u64);
    /// ```
    #[inline] pub fn get_src(&self) -> &[S; M]  { unsafe { &self.src } }

    /// Returns the value of the source element that `i` indicates,
    /// wrapped by `Some` of the enum `Option` if `i` is less than `M`
    /// which is the index range. Otherwise, it returns `None`.
    /// 
    /// # Argument
    /// `i` is the index of the source element to get
    /// 
    /// # Output
    /// - If `i` is less than `M` which is the index range, the value of
    /// the source element that `i` indicates, wrapped by the enum `Some
    /// of `Option`.
    /// - Otherwise, `None`
    /// 
    /// # Example
    /// ```
    /// use cryptocol::number::SharedArrays;
    /// let a = SharedArrays::<u32, 4, u64, 2>::from_src(&[123456789123456789_u64, 987654321987654321_u64]);
    /// print!("source = [ ");
    /// for i in 0..2
    /// {
    ///     match a.get_src_elem(i)
    ///     {
    ///         Some(b) =>  { print!("{} ", b); }
    ///         None =>     { print!("None "); }
    ///     }
    /// }
    /// println!("]");
    /// assert_eq!(a.get_src_elem(0).unwrap(), 123456789123456789_u64);
    /// assert_eq!(a.get_src_elem(1).unwrap(), 987654321987654321_u64);
    /// ```
    #[inline] pub fn get_src_elem(&self, i: usize) -> Option<S>
    {
        if i < M
            { Some(self.get_src_elem_(i)) }
        else
            { None }
    }

    /// Returns an i-th element of the array of the values of source type
    /// if `i` is less than `M` which is the index range.
    /// Otherwise, it panics.
    /// 
    /// # Argument
    /// `i` is the index of the source element to get
    /// 
    /// # Output
    /// If `i` is less than `M` which is the index range, the value of
    /// the source element that `i` indicates.
    /// 
    /// # Panics
    /// If `i` is more than or equal to `M` or `i` is outside of the index
    /// range, it panics.
    /// 
    /// # Caution
    /// Use this method only when you are pretty sure that `i` is
    /// in the index range.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::number::SharedArrays;
    /// let a = SharedArrays::<u32, 4, u64, 2>::from_src(&[123456789123456789_u64, 987654321987654321_u64]);
    /// print!("source = [ ");
    /// for i in 0..2
    ///     { print!("{} ", a.get_src_elem_(i)); }
    /// println!("]");
    /// assert_eq!(a.get_src_elem_(0), 123456789123456789_u64);
    /// assert_eq!(a.get_src_elem_(1), 987654321987654321_u64);
    /// ```
    #[inline] pub fn get_src_elem_(&self, i: usize) -> S
    {
        #[cfg(target_endian = "little")]    unsafe { self.src[i] }
        #[cfg(target_endian = "big")]       unsafe { self.src[M-i-1] }
    }

    /// Returns an array of the values of destination type.
    /// 
    /// # Output
    /// An array of the values of destination type.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::number::SharedArrays;
    /// let a = SharedArrays::<u32, 4, u64, 2>::from_src(&[123456789123456789_u64, 987654321987654321_u64]);
    /// let b = a.get_des();
    /// print!("Destination = [ ");
    /// for i in 0..4
    ///     { print!("{} ", b[i]); }
    /// println!("]");
    /// assert_eq!(b[0], 2899336981_u32);
    /// assert_eq!(b[1], 28744523_u32);
    /// assert_eq!(b[2], 2129924785_u32);
    /// assert_eq!(b[3], 229956191_u32);
    /// ```
    #[inline] pub fn get_des(&self) -> &[D; N]  { unsafe { &self.des } }

    /// Returns the value of the destination element that `i` indicates,
    /// wrapped by `Some` of the enum `Option` if `i` is less than `M`
    /// which is the index range. Otherwise, it returns `None`.
    /// 
    /// # Argument
    /// `i` is the index of the destination element to get
    /// 
    /// # Output
    /// - If `i` is less than `M` which is the index range, the value of
    /// the destination element that `i` indicates, wrapped by the enum `Some
    /// of `Option`.
    /// - Otherwise, `None`
    /// 
    /// # Example
    /// ```
    /// use cryptocol::number::SharedArrays;
    /// let a = SharedArrays::<u32, 4, u64, 2>::from_src(&[123456789123456789_u64, 987654321987654321_u64]);
    /// print!("destination = [ ");
    /// for i in 0..4
    /// {
    ///     match a.get_des_elem(i)
    ///     {
    ///         Some(b) =>  { print!("{} ", b); }
    ///         None =>     { print!("None "); }
    ///     }
    /// }
    /// println!("]");
    /// assert_eq!(a.get_des_elem(0).unwrap(), 2899336981_u32);
    /// assert_eq!(a.get_des_elem(1).unwrap(), 28744523_u32);
    /// assert_eq!(a.get_des_elem(2).unwrap(), 2129924785_u32);
    /// assert_eq!(a.get_des_elem(3).unwrap(), 229956191_u32);
    /// ```
    #[inline] pub fn get_des_elem(&self, i: usize) -> Option<D>
    {
        if i < N
            { Some(self.get_des_elem_(i)) }
        else
            { None }
    }

    /// Returns an i-th element of the array of the values of destination type
    /// if `i` is less than `M` which is the index range.
    /// Otherwise, it panics.
    /// 
    /// # Argument
    /// `i` is the index of the destination element to get
    /// 
    /// # Output
    /// If `i` is less than `M` which is the index range, the value of
    /// the destination element that `i` indicates.
    /// 
    /// # Panics
    /// If `i` is more than or equal to `N` or `i` is outside of the index
    /// range, it panics.
    /// 
    /// # Caution
    /// Use this method only when you are pretty sure that `i` is
    /// in the index range.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::number::SharedArrays;
    /// let a = SharedArrays::<u32, 4, u64, 2>::from_src(&[123456789123456789_u64, 987654321987654321_u64]);
    /// print!("destination = [ ");
    /// for i in 0..4
    ///     { print!("{} ", a.get_des_elem_(i)); }
    /// println!("]");
    /// assert_eq!(a.get_des_elem_(0), 2899336981_u32);
    /// assert_eq!(a.get_des_elem_(1), 28744523_u32);
    /// assert_eq!(a.get_des_elem_(2), 2129924785_u32);
    /// assert_eq!(a.get_des_elem_(3), 229956191_u32);
    /// ```
    #[inline] pub fn get_des_elem_(&self, i: usize) -> D
    {
        #[cfg(target_endian = "little")]    unsafe { self.des[i] }
        #[cfg(target_endian = "big")]       unsafe { self.des[N-i-1] }
    }

    /// Puts an array of the values of destination type in the array `des`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::number::SharedArrays;
    /// let a = SharedArrays::<u32, 4, u64, 2>::from_src(&[123456789123456789_u64, 987654321987654321_u64]);
    /// let mut b = [0_u32; 4];
    /// a.put_des_in_array(&mut b);
    /// print!("destination = [ ");
    /// for i in 0..4
    ///     { print!("{} ", b[i]); }
    /// println!("]");
    /// assert_eq!(b[0], 2899336981_u32);
    /// assert_eq!(b[1], 28744523_u32);
    /// assert_eq!(b[2], 2129924785_u32);
    /// assert_eq!(b[3], 229956191_u32);
    /// ```
    #[inline] pub fn put_des_in_array(&self, des: &mut [D; N])  { unsafe { des.copy_from_slice(&self.des); } }

    /*
    #[cfg(target_endian = "big")]
    pub fn put_des_in_array(& self, des: &mut [D; N])
    {
        let des_size = Self::size_of_des();
        let src_size = Self::size_of_src();
        if src_size > des_size
            { self.shl_assign_src((src_size - des_size) * 8); }
        else
            { self.shr_assign_des((des_size - src_size) * 8); }
        des.copy_from_slice(&self.des);
    }

    #[cfg(target_endian = "big")]
    fn shl_assign_src(&mut self, rhs: usize)
    {
        #[allow(non_snake_case)]
        let DSIZE_BIT = size_of::<D>() * 8;
        let chunk_num = rhs as usize / DSIZE_BIT as usize;
        let piece_num = rhs as usize % DSIZE_BIT as usize;
        let zero = S::zero();
        if chunk_num > 0
        {
            self.src.copy_within(chunk_num..N, 0);
            for idx in N-chunk_num..N
                { self.src[idx] = zero; }
        }
        if piece_num == 0
            { return; }

        let mut num: S;
        let mut carry = zero;
        for idx in 0..N-chunk_num
        {
            num = (self.src[idx] << S::num(piece_num.into_u128())) | carry;
            carry = self.src[idx] >> S::num((DSIZE_BIT - piece_num).into_u128());
            self.src[idx] = num;
        }
    }

    #[cfg(target_endian = "big")]
    fn shr_assign_des(&mut self, rhs: usize)
    {
        #[allow(non_snake_case)]
        let DSIZE_BIT = size_of::<D>() * 8;
        let chunk_num = rhs as usize / DSIZE_BIT as usize;
        let piece_num = rhs as usize % DSIZE_BIT as usize;
        let zero = D::zero();
        if chunk_num > 0
        {
            unsafe { self.des.copy_within(0..N-chunk_num, chunk_num); }
            for idx in 0..chunk_num
                { unsafe { self.des[idx] = zero; } }
        }
        if piece_num == 0
            { return; }

        let mut num: D;
        let mut carry = D::zero();
        let mut idx = 0;
        loop
        {
            num = (self.get_des_elem_(idx) >> D::num(piece_num.into_u128())) | carry;
            carry = self.get_des_elem_(idx) << D::num((DSIZE_BIT - piece_num).into_u128());
            unsafe { self.des[idx] = num; }
            if idx == N - 1 - chunk_num
                { break; }
            idx += 1;
        }
    }
    */

    /// Returns the size of `src`.
    /// 
    /// # Feature
    /// It is `size_of::<S>() * M`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::number::SharedArrays;
    /// type Shared = SharedArrays::<u32, 5, u64, 3>;
    /// let a = Shared::from_src(&[123456789123456789_u64, 987654321987654321_u64, 13579246801357924680_u64]);
    /// println!("The size of src is {}.", Shared::size_of_src());
    /// assert_eq!(Shared::size_of_src(), 24);
    /// ```
    #[inline] pub fn size_of_src() -> usize   { size_of::<[S; M]>() }

    /// Returns the size of `des`.
    /// 
    /// # Output
    /// `size_of::<D>() * N`
    /// 
    /// # Example
    /// ```
    /// use cryptocol::number::SharedArrays;
    /// type Shared = SharedArrays::<u32, 5, u64, 3>;
    /// let a = Shared::from_src(&[123456789123456789_u64, 987654321987654321_u64, 13579246801357924680_u64]);
    /// println!("The size of des is {}.",  Shared::size_of_des());
    /// assert_eq!(Shared::size_of_des(), 20);
    /// ```
    #[inline] pub fn size_of_des() -> usize   { size_of::<[D; N]>() }

    /// Returns the size of `src`.
    /// 
    /// # Output
    /// `size_of::<S>() * M`
    /// 
    /// # Example
    /// ```
    /// use cryptocol::number::SharedArrays;
    /// type Shared = SharedArrays::<u32, 5, u64, 3>;
    /// let a = Shared::from_src(&[123456789123456789_u64, 987654321987654321_u64, 13579246801357924680_u64]);
    /// println!("The size of src is {}.", Shared::size_of_src());
    /// assert_eq!(Shared::size_of_src(), 24);
    /// ```
    #[inline] pub fn length_of_src(&self) -> usize   { size_of_val(unsafe { &self.src }) }

    /// Returns the size of `des`.
    /// 
    /// # Feature
    /// It is `size_of::<D>() * N`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::number::SharedArrays;
    /// type Shared = SharedArrays::<u32, 5, u64, 3>;
    /// let a = Shared::from_src(&[123456789123456789_u64, 987654321987654321_u64, 13579246801357924680_u64]);
    /// println!("The size of des is {}.",  Shared::size_of_des());
    /// assert_eq!(Shared::size_of_des(), 20);
    /// ```
    #[inline] pub fn length_of_des(&self) -> usize   { size_of_val(unsafe { &self.des }) }
}
