// Copyright 2023, 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(missing_docs)]
#![allow(missing_doc_code_examples)]

/********** FOR BIG-ENDIANNESS ONLY **********/
use std::fmt::{ self, Display, Formatter, Debug };
use std::mem::{ size_of, transmute };
use std::cmp::{ PartialEq, PartialOrd, Ordering };
use std::convert::{ From, Into };
use std::str::FromStr;
use std::ops::*;

use super::uint::*;
use super::big_uint::BigUInt;
use super::NumberErr;


#[cfg(target_endian = "big")]
pub trait _BigUInt
{
    fn _is_one(&self) -> bool;
    fn _is_uint(&self, val: T) -> bool;
    fn _one() -> Self;
    fn _random_with_MSB_set() -> Self;
    fn _turn_check_bits(&mut self, bit_pos: usize);
    fn _get_num(&self, i: usize) -> Option<T>;
    fn _get_num_(&self, i: usize) -> T;
    fn _set_num(&mut self, i: usize, val: T) -> bool;
    fn _set_num_(&mut self, i: usize, val: T);
    fn _copy_within<R>(&mut self, src: R, dest: usize);
    fn _add_assign(&mut self, rhs: Self);
    fn _wrapping_sub_assign(&mut self, rhs: Self);
    fn _wrapping_mul_assign(&mut self, rhs: Self);
}

#[cfg(target_endian = "big")]
impl<T, const N: usize> _BigUInt for BigUInt<T, N>
{
    /// Checks whether `BigUInt` to be one and returns true if it is
    /// one, and returns false if it is not one.
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::number::BigUInt;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let a = u1024::one();
    /// if a.is_one()
    ///     { println!("a is One"); }
    /// else
    ///     { println!("a is Not One"); }
    /// assert!(a.is_one());
    /// ```
    fn _is_one(&self) -> bool
    {
        if self._get_num_(N-1) != T::one()
            { return false; }

        for i in 0..N-1
        {
            if self._get_num_(i) != T::zero()
                { return false; }
        }
        true
    }

    /// Check whether the `BigUInt`-type number is equal to `T`-type number.
    /// It will return `true`, if it is equal to the `T`-type number. Otherwise,
    /// it will return `false`.
    /// 
    /// # Counter Part Method
    /// This method is_uint() is virtually the same the method [eq_uint()](struct@BigUInt#method.eq_uint).
    /// However, you may want to use this method is_uint() rather than [eq_uint()](struct@BigUInt#method.eq_uint),
    /// if you know that this method is_uint() is a bit faster than [eq_uint()](struct@BigUInt#method.eq_uint),
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    /// 
    /// # Example
    /// ```
    /// define_utypes_with!(u128);
    /// let mut a = u1024::new();
    /// a.set_uint(25);
    /// if a.is_uint(25u128)   { println!("They are the same."); }
    /// else                   { println!("They are differnt."); }
    /// let b = 
    /// assert!(a.is_uint(25u128));
    /// ```
    pub fn _is_uint(&self, val: T) -> bool
    {
        if self._get_num_(N-1) != val
        {
            false
        }
        else
        {
            for i in 0..N-1
            {
                if self._get_num_(i) != T::zero()
                    { return false; }
            }
            true
        }
    }

    /// Constructs a new `BigUInt<T, N>` which has the value of one.
    /// It is virtually the same as [one()](struct@BigUInt#method.one)
    /// but it is considered to be slightly faster than one() for big-endian
    /// CPUs.
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// let one = u256::_one();
    /// println!("one = {}", one);
    /// assert_eq!(one, u256::from_uint(1_u64));
    /// ```
    fn _one() -> Self
    {
        let mut me = Self::new();
        me._set_num(N-1, T::one());
        me
    }

    /// Constucts a new `BigUInt<T, N>` which has the random value
    /// with MSB (Most Significant Bit) is set. The random number that this
    /// method random_with_MSB_set() returns is a random number whose range
    /// is from !(BigUInt::max() >> 1) up to BigUInt::max() inclusively.
    /// It is virtually the same as the method [random_with_MSB_set()](struct@BigUInt#method.random_with_MSB_set),
    /// but it is considered slightly faster than random_with_MSB_set().
    /// 
    /// # Cryptographical Security
    /// The random number generated by this method random_with_MSB_set() is
    /// cryptographically secure because it uses the crate [rand](https://docs.rs/rand/latest/rand/index.html)
    /// ([rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Counter Part Method
    /// - If you want to be sure to use pure random number whose range is from 0
    /// up to BigUInt::max() inclusively, you are highly encouraged to use the
    /// method [random()](struct@BigUInt#method.random) rather than this method
    /// random_with_MSB_set().
    /// - If you want to be sure to use (N * 8)-bit long random number for
    /// cryptographic purpose, you are highly recommended to use the method
    /// random_with_MSB_set() instead of this method [random()](struct@BigUInt#method.random)
    /// because this method random_with_MSB_set() __DOES__ guarrantee MSB (Most
    /// Significant Bit) is `1`. So, it is __0 (zero) %__ that this method
    /// random_with_MSB_set() will generate a random number shorter than
    /// (N * 8)-bit.
    ///
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with_u128;
    /// 
    /// define_utypes_with_u128!();
    /// println!("Random number = {}", u1024::random());
    /// println!("Random number = {}", u1024::_random_with_MSB_set());
    /// ```
    fn _random_with_MSB_set() -> Self
    {
        let mut r = Self::random();
        let highest = r._get_num_(0).unwrap();
        let msb = !(T::max() >> T::one());
        r._set_num_(0, highest | msb);
        r
    }
    
    /// Changes a `BigUInt<T, N>` to have the value zero and sets only
    /// the bit specified by the argument bit_pos to be 1.
    /// 
    /// # Bit Position
    /// The bit positon bit_pos is zero-based and should be counted from LSB
    /// (Least Significant Bit) reguardless endian. So, if the bit_pos is `0`,
    /// only LSB is set to be `1` and all the other bits will be set to `0`.
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    /// 
    /// # Counter Part Method
    /// This method _turn_check_bits() is considered to be slightly faster than
    /// the method [turn_check_bits()](struct@BigUInt#method.turn_check_bits).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with_u8;
    /// 
    /// define_utypes_with_u8!();
    /// let mut a = u256::random();
    /// a.turn_check_bits(12);
    /// println!("a = {}", a.to_string_with_radix_and_stride(2, 8));
    /// assert_eq!(a, u256::from_str_radix("10000_00000000", 2).unwrap());
    /// ```
    fn _turn_check_bits(&mut self, bit_pos: usize)
    {
        let TSIZE_BITS = self.length_in_bits();
        let chunk_num = N - 1 - bit_pos / TSIZE_BITS;
        let piece_num = bit_pos % TSIZE_BITS;
        let mut val = T::one();
        val <<= T::num(piece_num as u128);
        self.set_zero();
        self._set_num_(chunk_num, val);
    }

    /// Returns i-th element of its array of type `T` wrapped in Some
    /// if `i` < `N`. Otherwise, it returns `None`. 
    /// 
    /// # Argument i
    /// 0-th element contains MSB (Most Significant Bit), while (N-1)-th element
    /// contains LSB (Least Significant Bit). BigUInt` have an array of type `T`
    /// in order to present long-sized unsigned integer.
    /// 
    /// # Error
    /// If `i` >= `N`, it returns `None`.
    /// 
    /// # Counter Part Method
    /// When you are sure that `i` < `N`, you may want to use its counter part
    /// method [_get_num_()](struct@BigUInt#method._get_num_) for performance.
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with_u32;
    /// 
    /// define_utypes_with_u32!();
    /// let a = u256::from([0_u32, 10, 20, 30, 40, 50, 60, 70]);
    /// let e = a.get_num(3);
    /// match e
    /// {
    ///     Some(num) => {
    ///             println!("a.get_num(3).unwrap() = {}", num);
    ///             assert_eq!(num, 30);
    ///         },
    ///     None => { println!("There is no third element."); },
    /// }
    /// ```
    fn _get_num(&self, i: usize) -> Option<T>
    {
        if i < N
            { Some(self.get_number()[i]) }
        else
            { None }
    }

    /// Returns i-th element of its array of type `T` if `i` < `N`.
    /// Otherwise, it will panic.
    /// 
    /// # Argument i
    /// 0-th element contains MSB (Most Significant Bit), while (N-1)-th element
    /// contains LSB (Least Significant Bit). BigUInt` have an array of type `T`
    /// in order to present long-sized unsigned integer.
    /// 
    /// # Panics
    /// It is performance-oriented and does not care for safety. So, 
    /// if `i` >= `N`, it will panic.
    /// 
    /// # Counter Part Method
    /// Use this method _get_num_() only when you are sure that `i` < `N`.
    /// Otherwise, use its counter part method [_get_num()](struct@BigUInt#method._get_num).
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with_u32;
    /// 
    /// define_utypes_with_u32!();
    /// let a = u256::from([0_u32, 10, 20, 30, 40, 50, 60, 70]);
    /// let e = a._get_num_(3);
    /// println!("a._get_num_(3) = {}", e);
    /// assert_eq!(e, 30);
    /// ```
    #[inline]
    fn _get_num_(&self, i: usize) -> T
    {
        self.get_number()[i]
    }

    /// Sets i-th element of its array of type `T`, and return `true`
    /// if `i` < `N`. Otherwise, it sets none of the elements of its
    /// array of type `T`, and returns `false`.
    ///  
    /// # Argument i
    /// 0-th element contains MSB (Most Significant Bit), while (N-1)-th element
    /// contains LSB (Least Significant Bit). BigUInt` have an array of type `T`
    /// in order to present long-sized unsigned integer.
    /// 
    /// # Error
    /// If `i` >= `N`, it will return `false`.
    /// 
    /// # Counter Part Method
    /// When you are sure that `i` < `N`, you may want to use its counter part
    /// method [_set_num_()](struct@BigUInt#method._set_num_) for performance.
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with_u32;
    /// 
    /// define_utypes_with_u32!();
    /// let mut a = u256::from([0_u32, 10, 20, 30, 40, 50, 60, 70]);
    /// let mut e = a.get_num(3);
    /// match e
    /// {
    ///     Some(num) => {
    ///             println!("a.get_num(3).unwrap() = {}", num);
    ///             assert_eq!(num, 30);
    ///         },
    ///     None => { println!("There is no third element."); },
    /// }
    /// let succ = a.set_num(3, 300);
    /// if succ
    /// {
    ///     let num = a.get_num_(3);
    ///     println!("a.get_num_(3) = {}", num);
    ///     assert_eq!(num, 300);
    /// }
    /// else
    /// {
    ///     println!("There is no third element.");
    /// }
    /// ```
    fn _set_num(&mut self, i: usize, val: T) -> bool
    {
        if i < N
        {
            self.get_number_mut()[i] = val;
            true
        }
        else
        {
            false
        }
    }

    /// Sets i-th element of its array of type `T` if `i` < `N`.
    /// Otherwise, it will panic.
    ///  
    /// # Argument i
    /// 0-th element contains MSB (Most Significant Bit), while (N-1)-th element
    /// contains LSB (Least Significant Bit). BigUInt` have an array of type `T`
    /// in order to present long-sized unsigned integer.
    /// 
    /// # Panics
    /// If `i` >= `N`, it will panic.
    /// 
    /// # Counter Part Method
    /// It is performance-oriented and does not care for safety.
    /// It is virtually the same as the method _set_num(). This method _set_num_()
    /// is considered to be slightly faster than the method _set_num().
    /// Use this method _set_num_() only when you are sure that `i` < `N`.
    /// Otherwise, use its counter part method [_set_num()](struct@BigUInt#method._set_num).
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with_u32;
    /// 
    /// define_utypes_with_u32!();
    /// let mut a = u256::from([0_u32, 10, 20, 30, 40, 50, 60, 70]);
    /// let mut num = a.get_num_(3);
    /// println!("a.get_num(3).unwrap() = {}", num);
    /// assert_eq!(num, 30);
    /// 
    /// a.set_num_(3, 300);
    /// num = a.get_num_(3);
    /// println!("a.get_num_(3) = {}", num);
    /// assert_eq!(num, 300);
    /// ```
    #[inline]
    fn _set_num_(&mut self, i: usize, val: T)
    {
        self.get_number_mut()[i] = val;
    }

    /// Copies elements from one part of the slice `T`-array of BigUInt to
    /// another part of itself, using a memmove.
    /// 
    /// # Arguments
    /// - src is the range within self.number to copy from. Regardless
    /// endianness, the index is counted from MSB (Most Significant Bit) to LSB
    /// (Least Significant Bit). So, index 0 is MSB and index N-1 is LSB.
    /// - dest is the starting index of the range within self.number to copy to,
    /// which will have the same length as src.
    /// - The two ranges may overlap.
    /// - The ends of the two ranges must be less than or equal to self.len().
    /// 
    /// # Panics
    /// This method copy_within() will panic if either range exceeds the end
    /// of the slice, or if the end of src is before the start.
    ///  
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt;
    /// define_utypes_with!(u16);
    /// let mut a = u256::new();
    /// a.set_number(&[0_u16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    /// println!("a = {:?}", a);
    /// a._copy_within(3..10, 6);
    /// println!("a = {:?}", a);
    /// assert_eq!(a.get_number(), &[0, 1, 2, 3, 4, 5, 3, 4, 5, 6, 7, 8, 9, 13, 14, 15]);
    /// ```
    #[inline]
    fn _copy_within<R>(&mut self, src: R, dest: usize)
    where R: RangeBounds<usize>
    {
        self.get_number_mut().copy_within(src, dest);
    }


    // pub fn wrapping_sub_assign(&mut self, rhs: Self)
    /// Computes `self` - `rhs`, wrapping around at the boundary of the type.
    /// 
    /// # Feature
    /// Wrapping (modular) subtraction.
    /// 
    /// # Example
    /// ```
    /// // Todo
    /// ```
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    #[cfg(target_endian = "big")]
    fn _wrapping_sub_assign(&mut self, rhs: Self)
    {
        let mut num: T;
        let mut i = N - 1;
        let mut	borrow = false;
        loop
        {
            (num, borrow) = self.number[i].borrowing_sub(rhs.number[i], borrow);
            self.set_num_(i, num);
            if i == 0
                { break; }
            i -= 1;
        }
        if carry
            { self.set_underflow(); }
/*
        let zero = T::zero();
        let mut	carry: T = zero;
        let mut midres: T;
        let mut c: bool;
        let mut cc: T;
        let mut i = N;
        loop
        {
            i -= 1;
            midres = self.number[i].wrapping_sub(rhs.number[i]);
            c = midres > self.number[i];
            cc = midres;
            midres = midres.wrapping_sub(carry);
            carry = if c || (midres > cc) { T::one() } else { zero };
            self.number[i] = midres;
            if i == 0
                { break; }
        }
        if carry != zero
            { self.set_underflow(); }
    */
    }

    /// Multiplies and assign the result to it.
    /// 
    #[cfg(target_endian = "big")]
    fn _wrapping_mul_assign(&mut self, rhs: Self)
    {
        if rhs.is_zero()
        {
            self.set_zero();
            return;
        }
        if self.is_zero()
            { return; }

        let zero = T::zero();
        let one = T::one();
        let adder = self.clone();
        let TSIZE_BIT = size_of::<T>() * 8;
        let mut multiply_first = |num: T| {
            let mut bit_check = one;
            bit_check <<= T::num((TSIZE_BIT - 1).into_u128());
            while (bit_check != zero) && (bit_check & num == zero)
                { bit_check >>= one; }

            self.set_zero();
            while bit_check != zero
            {
                *self <<= 1;
                if bit_check & num != zero
                    { *self += adder; }
                bit_check >>= one;
            }
        };

        let mut n = 0;
        while rhs.number[n] == zero
            { n += 1; }
        multiply_first(rhs.number[n]);
        n += 1;

        let mut multiply = |num: T| {
            if num == T::zero()
            {
                *self <<= TSIZE_BIT as i32;
                return;
            }
            let mut bit_check = one;
            bit_check <<= T::num((TSIZE_BIT - 1).into_u128());
            while bit_check != zero
            {
                *self <<= 1;
                if bit_check & num != zero
                    { *self += adder; }
                bit_check >>= one;
            }
        };
        while n < N
        {
            multiply(rhs.number[n]);
            n += 1;
        }
    }
} 



#[cfg(target_endian = "big")]
pub trait _From<T>: Sized
{
    // Required method
    fn _from(value: T) -> Self;
}

#[cfg(target_endian = "big")]
impl<T, const N: usize, S> _From<S> for BigUInt<T, N>
where T: SmallUInt + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd,
    S: SmallUInt + Clone + Display + Debug + ToString
        + Add<Output=S> + AddAssign + Sub<Output=S> + SubAssign
        + Mul<Output=S> + MulAssign + Div<Output=S> + DivAssign
        + Shl<Output=S> + ShlAssign + Shr<Output=S> + ShrAssign
        + BitAnd<Output=S> + BitAndAssign + BitOr<Output=S> + BitOrAssign
        + BitXor<Output=S> + BitXorAssign + Not<Output=S>
        + PartialEq + PartialOrd
{
    /// Constructs a new BigUInt<T, N> from an unsigned integer such as u8,
    /// u16, u32, u64, u128 and usize. This crate is so experimental for
    /// Big-endian CPUs that you are highy discouraged to use this crate
    /// for Big-endian CPUs for serious purpose. So, use this crate for
    /// Big-endian CPUs with your own full responsibility.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use cryptocol::number::BigUInt;
    /// let cc = BigUInt::<u16,32>::from(1004_u16);
    /// println!("cc = {}", cc);
    /// assert_eq!(cc.into_u32(), 1004);
    /// ```
    fn _from(val: S) -> Self
    {
        let TSIZE = size_of::<T>();
        let SSIZE = size_of::<S>();
        let mut me = Self::new();
        let mut share = Share::<T, S>::from_src(val);
        
        if TSIZE >= SSIZE
        {
            unsafe { me._set_num(N-1, share.des); }
        }
        else    // if TSIZE < SSIZE
        {
            let TSIZE_BIT = TSIZE * 8;
            let LEN = SSIZE/TSIZE;
            if LEN <= N
            {
                for i in 0..LEN
                {
                    unsafe { me._set_num(N - LEN + i, share.des); }
                    unsafe { share.src <<= S::num(TSIZE_BIT as u128); }
                }    
            }
            else    // if LEN > N
            {
                unsafe { share.src <<= S::num(((LEN - N) * TSIZE_BIT) as u128); }
                for i in 0..N
                {
                    unsafe { me._set_num(i, share.des); }
                    unsafe { share.src <<= S::num(TSIZE_BIT as u128); }
                } 
            }
        }
        return me;
    }
}



#[cfg(target_endian = "big")]
pub trait _ShlAssign<Rhs = Self>
{
    // Required method
    fn _shl_assign(&mut self, rhs: Rhs);
}

#[cfg(target_endian = "big")]
impl<T, const N: usize> _ShLAssign<i32> for BigUInt<T, N>
where T: SmallUInt + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /// Performs the <<= operation. It is so experimental for big endian CPUs
    /// that you are highy discouraged to use this crate for big endian CPUs
    /// for serious purpose. If overflow happens during the <<= operation,
    /// `OVERFLOW` flag is set  and the method is_overflow() will return true. 
    /// [Read more](https://doc.rust-lang.org/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::{u256, BigInteger};
    /// let mut a = u256::from_str("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
    /// a <<= 4;
    /// println!("a = {}\noverflow: {}", a, a.is_overflow());
    /// assert_eq!(a.is_overflow(), true);
    /// ```
    /// You have to import (use) cryptocol::number::u256 in order to use the
    /// type u256 and import cryptocol::number::BigInteger in order to use
    /// its method is_overflow(). If you find headaching to remember what you
    /// should import, you can just import everything (cryptocol::number::*)
    /// as next example. It is not harmful.
    /// ```
    /// use cryptocol::number::*;
    /// let mut a = u256::from_str("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
    /// a <<= 1;
    /// println!("a = {}\noverflow: {}", a, a.is_overflow());
    /// assert_eq!(a.is_overflow(), false);
    /// ```
    #[cfg(target_endian = "big")]
    fn _shl_assign(&mut self, rhs: i32)
    {
        if rhs < 0
        {
            *self >>= -rhs;
            return;
        }
        let TSIZE_IN_BITS = T::size_in_bits();
        let chunk_num = rhs as usize / TSIZE_IN_BITS as usize;
        let piece_num = rhs as usize % TSIZE_IN_BITS as usize;
        let zero = T::zero();
        self.flag = 0;
        if chunk_num > 0
        {
            for i in 0..N-chunk_num
            {
                if self._get_num_(i) > zero
                {
                    self.set_overflow();
                    break;
                }
            }
            self._copy_within(chunk_num..N, 0);
            for idx in N-chunk_num..N
                { self._get_num(idx) = zero; }
        }
        if piece_num == 0
            { return; }
        if (self._get_num(0) >> T::num((TSIZE_IN_BITS - piece_num).into_u128())) != zero
            { self.set_overflow(); }

        let mut num: T;
        let mut carry = zero;
        let mut idx = N - 1 - chunk_num;
        loop
        {
            num = (self.number[idx] << T::num(piece_num.into_u128())) | carry;
            carry = self.number[idx] >> T::num((TSIZE_IN_BITS - piece_num).into_u128());
            self._set_num(idx, num);
            if idx == 0
                { break; }
            idx -= 1;
        }
        if carry != zero
            { self.set_overflow(); }
    }
}



#[cfg(target_endian = "big")]
pub trait _ShrAssign<Rhs = Self>
{
    // Required method
    fn _shr_assign(&mut self, rhs: Rhs);
}

#[cfg(target_endian = "big")]
impl<T, const N: usize> _ShrAssign<i32> for BigUInt<T, N>
where T: SmallUInt + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /// Performs the >>= operation. It is so experimental for big endian CPUs
    /// that you are highy discouraged to use this crate for big endian CPUs
    /// for serious purpose. If underflow happens during the >>= operation,
    /// `UNDERFLOW` flag is set and the method is_underflow() will return true.
    /// Here, 'underflow' means that none-zero part is shifted out to the right.
    /// [Read more](https://doc.rust-lang.org/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::BigInteger;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let mut a = u256::from_str("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
    /// a >>= 2;
    /// println!("a = {}\nunderflow: {}", a, a.is_underflow());
    /// assert_eq!(a.is_underflow(), true);
    /// ```
    /// You have to import (use) cryptocol::number::BigInteger in order to use
    /// its method is_underflow(). If you find headaching to remember what you
    /// should import, you can just import everything (cryptocol::number::*)
    /// as next example. It is not harmful.
    /// ```
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let mut a = u256::from_str("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
    /// a >>= 1;
    /// println!("a = {}\nunderflow: {}", a, a.is_underflow());
    /// assert_eq!(a.is_underflow(), false);
    /// ```
    fn _shr_assign(&mut self, rhs: i32)
    {
        use std::slice::Chunks;

        if rhs < 0
        {
		    *self <<= -rhs;
            return;
        }
        let TSIZE_IN_BITS = T::size_in_bits();
        let chunk_num = rhs as usize / TSIZE_IN_BITS as usize;
        let piece_num = rhs as usize % TSIZE_IN_BITS as usize;
        let zero = T::zero();
        self.flag = 0;
        if chunk_num > 0
        {
            for i in N-chunk_num..N
            {
                if self._get_num(i) > zero
                {
                    self.set_underflow();
                    break;
                }
            }
            self._copy_within(0..N-chunk_num, chunk_num);
            for idx in 0..chunk_num
                { self._set_num_(idx, zero); }
        }
        if piece_num == 0
            { return; }
        if (self._get_num_(N-1) << T::num((TSIZE_IN_BITS - piece_num).into_u128())) != zero
            { self.set_underflow(); }

        let mut num: T;
        let mut carry = T::zero();
        let mut idx = 0;
        for idx in 0..N-chunk_num
        {
            num = (self._get_num_(idx) >> T::num(piece_num.into_u128())) | carry;
            carry = self._get_num_(idx) << T::num((TSIZE_IN_BITS - piece_num).into_u128());
            self._set_num_(idx, num);
        }
        if carry != zero
            { self.set_underflow(); }
    }

    fn _add_assign(&mut self, rhs: Self)
    {
        let mut i = N - 1;
        let mut	carry = false;
        loop
        {
            (self._get_num_(i), carry) = self._get_num_(i).carrying_add(rhs.number[i], carry);
            if i == 0
                { break; }
            i -= 1;
        }
        if carry
            { self.set_overflow(); }
    /*
        let zero = T::zero();
        let mut	carry: T = zero;
        let mut midres: T;
        let mut c: bool;

        let mut i = N - 1;
        loop
        {
            midres = self._get_num_(i).wrapping_add(rhs._get_num_(i));
            c = midres < self._get_num_(i);
            midres = midres.wrapping_add(carry);
            carry = if c || (midres < carry) { T::one() } else { zero };
            self._set_num_(i, midres);
            if i == 0
                { break; }
            i -= 1;
        }

        if carry != zero
            { self.set_overflow(); }
    */
    }
}
